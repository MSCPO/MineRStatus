//! Configuration loader.
//!
//! Priority: config.toml > environment variables (`MINESTATUS_*`) > defaults.

use std::env;
use std::net::IpAddr;
use std::path::Path;
use std::time::Duration;

const CONFIG_FILE: &str = "config.toml";

/// Environment-variable prefix for every supported setting.
const ENV_PREFIX: &str = "MINESTATUS_";

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub ttl: Duration,
    pub max_size: usize,
}

#[derive(Debug, Clone)]
pub struct QueryConfig {
    pub timeout: Duration,
    pub max_total: Duration,
}

/// Custom DNS servers for SRV / A-record resolution (primary first).
///
/// Each entry may be an IPv4 or IPv6 address; DNS queries go to port 53.
/// When empty, the system DNS is used instead.
#[derive(Debug, Clone)]
pub struct DnsConfig {
    pub servers: Vec<IpAddr>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub query: QueryConfig,
    pub dns: DnsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            cache: CacheConfig {
                ttl: Duration::from_secs(600),
                max_size: 100,
            },
            query: QueryConfig {
                timeout: Duration::from_secs(8),
                max_total: Duration::from_secs(9),
            },
            dns: DnsConfig {
                servers: Vec::new(),
            },
        }
    }
}

/// Read a string value from the `MINESTATUS_<KEY>` environment variable.
fn env_str(key: &str) -> Option<String> {
    env::var(format!("{ENV_PREFIX}{key}")).ok()
}

/// Parse an integer from a TOML table or environment variable.
///
/// TOML wins over the environment, which wins over the default.
/// `0` is a legitimate value and is never treated as "missing".
fn int_or<const DEFAULT: i64>(root: &toml::Value, section: &str, name: &str) -> i64 {
    if let Some(table) = root.get(section).and_then(toml::Value::as_table)
        && let Some(n) = table.get(name).and_then(toml::Value::as_integer)
    {
        return n;
    }
    if let Some(raw) = env_str(name)
        && let Ok(n) = raw.parse::<i64>()
    {
        return n;
    }
    DEFAULT
}

fn string_or(root: &toml::Value, section: &str, name: &str, default: &str) -> String {
    if let Some(table) = root.get(section).and_then(toml::Value::as_table)
        && let Some(s) = table.get(name).and_then(toml::Value::as_str)
    {
        return s.to_string();
    }
    env_str(name).unwrap_or_else(|| default.to_string())
}

/// Read a custom DNS server (IPv4 or IPv6) from `[dns] <key>` or the
/// `MINESTATUS_<ENV_NAME>` environment variable. Empty / invalid -> `None`.
fn dns_ip(root: &toml::Value, key: &str, env_name: &str) -> Option<IpAddr> {
    let from_toml = root
        .get("dns")
        .and_then(toml::Value::as_table)
        .and_then(|t| t.get(key))
        .and_then(toml::Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string);
    let raw = from_toml
        .or_else(|| env::var(format!("MINESTATUS_{env_name}")).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    raw.and_then(|s| s.parse::<IpAddr>().ok())
}

pub fn load() -> Config {
    let mut result = Config::default();

    let root = Path::new(CONFIG_FILE)
        .exists()
        .then(|| {
            std::fs::read_to_string(CONFIG_FILE)
                .ok()
                .and_then(|text| toml::from_str::<toml::Value>(&text).ok())
        })
        .flatten()
        .unwrap_or(toml::Value::Table(toml::map::Map::new()));

    // [server]
    result.server.host = string_or(&root, "server", "host", &result.server.host);
    let port = int_or::<3000>(&root, "server", "port");
    result.server.port = u16::try_from(port).unwrap_or(3000);

    // [cache]
    let ttl = int_or::<600>(&root, "cache", "ttl");
    result.cache.ttl = Duration::from_secs(u64::try_from(ttl).unwrap_or(600));
    let size = int_or::<100>(&root, "cache", "max_size");
    result.cache.max_size = usize::try_from(size).unwrap_or(100);

    // [query]
    let timeout = int_or::<8>(&root, "query", "timeout");
    result.query.timeout = Duration::from_secs(u64::try_from(timeout).unwrap_or(8));

    let max_total = int_or::<9>(&root, "query", "max_total");
    result.query.max_total = Duration::from_secs(u64::try_from(max_total).unwrap_or(9));

    // [dns] — up to three custom nameservers (primary + two secondary).
    let mut dns_servers = Vec::new();
    for (key, env_name) in [
        ("primary", "DNS_PRIMARY"),
        ("secondary1", "DNS_SECONDARY1"),
        ("secondary2", "DNS_SECONDARY2"),
    ] {
        if let Some(ip) = dns_ip(&root, key, env_name) {
            dns_servers.push(ip);
        }
    }
    result.dns.servers = dns_servers;

    result
}
