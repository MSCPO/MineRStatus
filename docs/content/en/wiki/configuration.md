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

## Precedence Example

To run on port `8080` for one session without touching `config.toml`:

```bash
MINESTATUS_PORT=8080 cargo run --no-default-features
```

Environment variables override `config.toml` values, and `config.toml`
overrides the built-in defaults.