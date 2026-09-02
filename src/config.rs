//! Configuration loader.
//!
//! Priority: config.toml > environment variables (`MINESTATUS_*`) > defaults.

use std::env;
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
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub cache: CacheConfig,
    pub query: QueryConfig,
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

    result
}
