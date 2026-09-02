# Configuration

Priority: `config.toml` > environment variables (`MINESTATUS_*`) > defaults.

The default `config.toml` lives at the repository root:

```toml
[server]
host = "0.0.0.0"   # Env: MINESTATUS_HOST
port = 3000        # Env: MINESTATUS_PORT

[cache]
ttl = 600          # Env: MINESTATUS_CACHE_TTL  (seconds)
max_size = 100     # Env: MINESTATUS_CACHE_MAX_SIZE

[query]
timeout = 8        # Env: MINESTATUS_TIMEOUT    (seconds)
```

## Sections

### `[server]`

| Key | Default | Env var | Description |
|---|---|---|---|
| `host` | `0.0.0.0` | `MINESTATUS_HOST` | Bind address |
| `port` | `3000` | `MINESTATUS_PORT` | Listen port |

### `[cache]`

| Key | Default | Env var | Description |
|---|---|---|---|
| `ttl` | `600` | `MINESTATUS_CACHE_TTL` | Response cache time-to-live in seconds |
| `max_size` | `100` | `MINESTATUS_CACHE_MAX_SIZE` | Maximum number of cached entries |

The cache stores the full response (including the measured delay), so a cache
hit returns the latency recorded when the entry was created. Setting `ttl = 0`
disables caching entirely.

### `[query]`

| Key | Default | Env var | Description |
|---|---|---|---|
| `timeout` | `8` | `MINESTATUS_TIMEOUT` | Timeout for DNS resolution and connection, in seconds |
| `max_total` | `9` | `MINESTATUS_MAX_TOTAL` | Total budget for one query including retries, in seconds. Keep it below the platform function execution limit (Vercel Hobby: 10 s) so a failing query returns its JSON error instead of being killed |

### `[dns]`

Up to three custom DNS servers (IPv4 or IPv6) used for SRV / A-record
resolution. Leave them empty to use the system DNS.

| Key | Default | Env var | Description |
|---|---|---|---|
| `primary` | *(empty)* | `MINESTATUS_DNS_PRIMARY` | Primary DNS server |
| `secondary1` | *(empty)* | `MINESTATUS_DNS_SECONDARY1` | First secondary DNS server |
| `secondary2` | *(empty)* | `MINESTATUS_DNS_SECONDARY2` | Second secondary DNS server |

```toml
[dns]
primary = "223.5.5.5"
secondary1 = "119.29.29.29"
secondary2 = "2400:3200::1"   # IPv6 is supported
```

When custom servers are configured, both SRV and A-record queries go through
them; otherwise the system DNS is used (with a DNS-over-HTTPS fallback for
SRV).

## Precedence Example

To run on port `8080` for one session without touching `config.toml`:

```bash
MINESTATUS_PORT=8080 cargo run --no-default-features
```

Environment variables override `config.toml` values, and `config.toml`
overrides the built-in defaults.