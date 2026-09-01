//! Minecraft status querying and response models.
//!
//! Thin layer over [`rust_mc_status`] that mirrors the MineStatus Python API:
//! Java and Bedrock Edition queries, auto-detection, and cached reads.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use rust_mc_status::{
    BedrockServerStatus, JavaServerStatus, McClient, StatusExt, strip_formatting,
};
use serde::Serialize;
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::config::Config;

/// Shared application state: an uncached client plus a response cache that
/// stores the full [`StatusResponse`] (including the measured delay), so a
/// cache hit returns the latency recorded at the time of the original ping.
#[derive(Clone)]
pub struct AppState {
    /// Client without a built-in response cache; caching is handled by
    /// [`Cache`] so that cached responses keep their original delay.
    pub client: McClient,
    /// Application-level TTL response cache.
    pub cache: Cache,
}

impl AppState {
    pub fn new(config: &Config) -> Self {
        Self {
            client: McClient::builder().timeout(config.query.timeout).build(),
            cache: Cache::new(config.cache.ttl, config.cache.max_size),
        }
    }
}

/// Simple TTL + capacity response cache keyed by `protocol:host`.
///
/// Unlike `rust_mc_status`'s built-in cache (which resets `latency` to `0.0`
/// on a hit), this stores the complete [`StatusResponse`] so cached replies
/// return the delay measured at the time the entry was created.
#[derive(Clone)]
pub struct Cache {
    inner: Arc<RwLock<HashMap<String, CachedEntry>>>,
    ttl: Duration,
    max_size: usize,
}

struct CachedEntry {
    response: StatusResponse,
    stored_at: SystemTime,
}

impl Cache {
    pub fn new(ttl: Duration, max_size: usize) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            max_size: max_size.max(1),
        }
    }

    async fn get(&self, key: &str) -> Option<StatusResponse> {
        if self.ttl.is_zero() {
            return None;
        }
        let cache = self.inner.read().await;
        let entry = cache.get(key)?;
        if entry.stored_at.elapsed().unwrap_or(Duration::MAX) >= self.ttl {
            return None;
        }
        Some(entry.response.clone())
    }

    async fn insert(&self, key: String, response: StatusResponse) {
        if self.ttl.is_zero() {
            return;
        }
        let mut cache = self.inner.write().await;
        if cache.len() >= self.max_size {
            // Evict the oldest entry to stay within capacity.
            if let Some(oldest) = cache
                .iter()
                .min_by_key(|(_, e)| e.stored_at)
                .map(|(k, _)| k.clone())
            {
                cache.remove(&oldest);
            }
        }
        cache.insert(
            key,
            CachedEntry {
                response,
                stored_at: SystemTime::now(),
            },
        );
    }
}

/// Error produced by a failed status query (kept as a plain message).
#[derive(Debug)]
pub struct QueryError(pub String);

// ---------------------------------------------------------------------------
// Response models (mirror MineStatus Python API)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct MotdResponse {
    pub plain: String,
    pub html: String,
    pub minecraft: String,
    pub ansi: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PlayersResponse {
    pub online: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct StatusResponse {
    pub online: bool,
    pub players: PlayersResponse,
    pub delay: f64,
    pub version: String,
    pub motd: MotdResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

// ---------------------------------------------------------------------------
// Query functions
// ---------------------------------------------------------------------------

pub async fn query_java(
    state: &AppState,
    host: &str,
    use_cache: bool,
) -> Result<StatusResponse, QueryError> {
    let key = format!("java:{host}");
    if use_cache {
        if let Some(resp) = state.cache.get(&key).await {
            return Ok(resp);
        }
    }
    match state.client.java(host).await {
        Ok(status) => {
            let resp = build_java_response(&status);
            if use_cache {
                state.cache.insert(key, resp.clone()).await;
            }
            Ok(resp)
        }
        Err(err) => Err(QueryError(format!(
            "Failed to connect to Java server at {host}: {err}"
        ))),
    }
}

pub async fn query_bedrock(
    state: &AppState,
    host: &str,
    use_cache: bool,
) -> Result<StatusResponse, QueryError> {
    let key = format!("bedrock:{host}");
    if use_cache {
        if let Some(resp) = state.cache.get(&key).await {
            return Ok(resp);
        }
    }
    match state.client.bedrock(host).await {
        Ok(status) => {
            let resp = build_bedrock_response(&status);
            if use_cache {
                state.cache.insert(key, resp.clone()).await;
            }
            Ok(resp)
        }
        Err(err) => Err(QueryError(format!(
            "Failed to connect to Bedrock server at {host}: {err}"
        ))),
    }
}

/// Probe Java and Bedrock in parallel; return the first successful result.
///
/// If one probe fails before the other finishes, its future is dropped
/// (cancelled) and we keep waiting on the remaining one. When caching is
/// enabled, the winning protocol's response is stored under its own key so a
/// later request (including a fresh auto-detect) can hit the cache.
pub async fn query_unclassified(
    state: &AppState,
    host: &str,
    use_cache: bool,
) -> Result<StatusResponse, QueryError> {
    if use_cache {
        for proto in ["java", "bedrock"] {
            let key = format!("{proto}:{host}");
            if let Some(resp) = state.cache.get(&key).await {
                return Ok(resp);
            }
        }
    }

    let mut java = state.client.java(host).into_future();
    let mut bedrock = state.client.bedrock(host).into_future();
    let mut java_done = false;
    let mut bedrock_done = false;

    loop {
        tokio::select! {
            r = &mut java, if !java_done => {
                java_done = true;
                if let Ok(status) = r {
                    let resp = build_java_response(&status);
                    if use_cache {
                        state.cache.insert(format!("java:{host}"), resp.clone()).await;
                    }
                    return Ok(resp);
                }
            }
            r = &mut bedrock, if !bedrock_done => {
                bedrock_done = true;
                if let Ok(status) = r {
                    let resp = build_bedrock_response(&status);
                    if use_cache {
                        state.cache.insert(format!("bedrock:{host}"), resp.clone()).await;
                    }
                    return Ok(resp);
                }
            }
        }

        if java_done && bedrock_done {
            return Err(QueryError(
                "No server status detected. Is the server offline?".to_string(),
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Response builders
// ---------------------------------------------------------------------------

fn build_java_response(status: &JavaServerStatus) -> StatusResponse {
    StatusResponse {
        online: status.is_online(),
        players: PlayersResponse {
            online: status.players_online(),
            max: status.players_max(),
        },
        delay: status.latency_ms(),
        version: status.version().to_string(),
        motd: build_motd(status.motd()),
        icon: status.favicon().map(ToOwned::to_owned),
    }
}

fn build_bedrock_response(status: &BedrockServerStatus) -> StatusResponse {
    StatusResponse {
        online: status.is_online(),
        players: PlayersResponse {
            online: status.players_online() as i64,
            max: status.players_max() as i64,
        },
        delay: status.latency_ms(),
        version: status.version().to_string(),
        motd: build_motd(status.motd()),
        icon: None,
    }
}

fn build_motd(raw: &str) -> MotdResponse {
    MotdResponse {
        plain: strip_formatting(raw),
        html: mc_to_html(raw),
        minecraft: raw.to_string(),
        ansi: mc_to_ansi(raw),
    }
}

// ---------------------------------------------------------------------------
// Minecraft formatting-code converters
// ---------------------------------------------------------------------------

/// Maps a Minecraft color code character (lowercase) to its hex colour.
fn mc_color_hex(code: char) -> Option<&'static str> {
    match code {
        '0' => Some("#000000"),
        '1' => Some("#0000AA"),
        '2' => Some("#00AA00"),
        '3' => Some("#00AAAA"),
        '4' => Some("#AA0000"),
        '5' => Some("#AA00AA"),
        '6' => Some("#FFAA00"),
        '7' => Some("#AAAAAA"),
        '8' => Some("#555555"),
        '9' => Some("#5555FF"),
        'a' => Some("#55FF55"),
        'b' => Some("#55FFFF"),
        'c' => Some("#FF5555"),
        'd' => Some("#FF55FF"),
        'e' => Some("#FFFF55"),
        'f' => Some("#FFFFFF"),
        _ => None,
    }
}

/// Convert a MOTD with `§`-codes into inline-styled HTML.
pub fn mc_to_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut open = false;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\u{a7}' {
            out.push(c);
            continue;
        }
        let Some(code) = chars.next() else { break };
        let lower = code.to_ascii_lowercase();

        if lower == 'r' {
            if open {
                out.push_str("</span>");
                open = false;
            }
            continue;
        }

        let style = if let Some(hex) = mc_color_hex(lower) {
            format!("color:{hex}")
        } else {
            match lower {
                'l' => "font-weight:bold".to_string(),
                'o' => "font-style:italic".to_string(),
                'n' => "text-decoration:underline".to_string(),
                'm' => "text-decoration:line-through".to_string(),
                _ => continue,
            }
        };

        if open {
            out.push_str("</span>");
        }
        out.push_str(&format!("<span style=\"{style}\">"));
        open = true;
    }

    if open {
        out.push_str("</span>");
    }
    out
}

/// Convert a MOTD with `§`-codes into a terminal ANSI escape sequence.
pub fn mc_to_ansi(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '\u{a7}' {
            out.push(c);
            continue;
        }
        let Some(code) = chars.next() else { break };
        let lower = code.to_ascii_lowercase();

        let ansi = match lower {
            'r' => "\x1b[0m",
            '0' => "\x1b[30m",
            '1' => "\x1b[34m",
            '2' => "\x1b[32m",
            '3' => "\x1b[36m",
            '4' => "\x1b[31m",
            '5' => "\x1b[35m",
            '6' => "\x1b[33m",
            '7' => "\x1b[37m",
            '8' => "\x1b[90m",
            '9' => "\x1b[94m",
            'a' => "\x1b[92m",
            'b' => "\x1b[96m",
            'c' => "\x1b[91m",
            'd' => "\x1b[95m",
            'e' => "\x1b[93m",
            'f' => "\x1b[97m",
            'l' => "\x1b[1m",
            'o' => "\x1b[3m",
            'n' => "\x1b[4m",
            'm' => "\x1b[9m",
            _ => continue,
        };
        out.push_str(ansi);
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_converts_colors_and_bold() {
        assert_eq!(
            mc_to_html("§aGreen §lBold"),
            "<span style=\"color:#55FF55\">Green </span><span style=\"font-weight:bold\">Bold</span>"
        );
    }

    #[test]
    fn html_reset_closes_open_span() {
        assert_eq!(
            mc_to_html("§cRed§r plain"),
            "<span style=\"color:#FF5555\">Red</span> plain"
        );
    }

    #[test]
    fn ansi_converts_colors() {
        assert_eq!(mc_to_ansi("§aGreen"), "\x1b[92mGreen");
    }

    #[test]
    fn ansi_reset_uses_reset_code() {
        assert_eq!(mc_to_ansi("§fHi§r"), "\x1b[97mHi\x1b[0m");
    }

    #[test]
    fn plain_motd_strips_codes() {
        assert_eq!(strip_formatting("§aHypixel §c[1.8/1.21]"), "Hypixel [1.8/1.21]");
    }

    fn sample_response() -> StatusResponse {
        StatusResponse {
            online: true,
            players: PlayersResponse { online: 3, max: 20 },
            delay: 42.5,
            version: "1.20.1".to_string(),
            motd: MotdResponse {
                plain: "hi".to_string(),
                html: "hi".to_string(),
                minecraft: "hi".to_string(),
                ansi: "hi".to_string(),
            },
            icon: None,
        }
    }

    #[tokio::test]
    async fn cache_returns_stored_delay_on_hit() {
        let cache = Cache::new(Duration::from_secs(60), 4);
        let key = "java:example.com".to_string();
        cache.insert(key.clone(), sample_response()).await;
        let hit = cache.get(&key).await.expect("should be a cache hit");
        // The cached reply must keep the original delay, not reset it to 0.0.
        assert_eq!(hit.delay, 42.5);
    }

    #[tokio::test]
    async fn cache_expires_after_ttl() {
        let cache = Cache::new(Duration::from_millis(10), 4);
        let key = "java:example.net".to_string();
        cache.insert(key.clone(), sample_response()).await;
        assert!(cache.get(&key).await.is_some());
        tokio::time::sleep(Duration::from_millis(30)).await;
        assert!(cache.get(&key).await.is_none());
    }

    #[tokio::test]
    async fn cache_is_noop_when_ttl_zero() {
        let cache = Cache::new(Duration::ZERO, 4);
        cache.insert("java:x".to_string(), sample_response()).await;
        assert!(cache.get("java:x").await.is_none());
    }

    #[tokio::test]
    async fn cache_evicts_oldest_over_capacity() {
        let cache = Cache::new(Duration::from_secs(60), 2);
        cache.insert("java:a".to_string(), sample_response()).await;
        tokio::time::sleep(Duration::from_millis(5)).await;
        cache.insert("java:b".to_string(), sample_response()).await;
        cache.insert("java:c".to_string(), sample_response()).await;
        // "a" was the oldest and should have been evicted to stay at capacity 2.
        assert!(cache.get("java:a").await.is_none());
        assert!(cache.get("java:b").await.is_some());
        assert!(cache.get("java:c").await.is_some());
    }
}