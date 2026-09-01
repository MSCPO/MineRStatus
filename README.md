# MineRStatus API

![License](https://img.shields.io/github/license/MSCPO/MineRStatus?style=for-the-badge) [![GitHub Release](https://img.shields.io/github/release/MSCPO/MineRStatus.svg?style=for-the-badge&logo=Qase&color=005AA4)](https://github.com/MSCPO/MineRStatus/releases/latest)<br>[![Contributors](https://img.shields.io/github/contributors-anon/MSCPO/MineRStatus.svg?style=flat-square&logo=github&color=005AA4)](https://github.com/MSCPO/MineRStatus/graphs/contributors) [![Forks](https://img.shields.io/github/forks/MSCPO/MineRStatus?style=flat-square&logo=github&logoColor=fff&color=005AA4)](https://github.com/MSCPO/MineRStatus/network/members) [![Stars](https://img.shields.io/github/stars/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4)](https://github.com/MSCPO/MineRStatus/stargazers) [![Issues Open](https://img.shields.io/github/issues/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/issues) [![Issues Closed](https://img.shields.io/github/issues-closed/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/issues?q=is%3Aissue+is%3Aclosed) [![GitHub Discussions](https://img.shields.io/github/discussions/MSCPO/MineRStatus?style=flat-square&logo=github&logoColor=fff&color=953B00&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/discussions)

<p align="center">
  <a href="https://vercel.com/new/clone?repository-url=https://github.com/MSCPO/MineRStatus">
    <img src="https://vercel.com/button" alt="Deploy with Vercel" />
  </a>
</p>

[简体中文](README_CN.md)

## Introduction

A lightweight and simple Minecraft server query API written in **Rust**, and a rewrite of [MSCPO/MineStatus](https://github.com/MSCPO/MineStatus). It queries Java and Bedrock Edition servers over their status protocols, auto-detects the server type, and returns player counts, version, MOTD and the server icon — with a built-in TTL response cache.

## Features

- Lightweight and fast (Rust / axum / async)
- Free to deploy on Vercel (serverless function)
- Auto-detects Java vs Bedrock, queries both in parallel
- Response caching with TTL and size limit
- OpenAPI / Swagger UI documentation
- Configuration via `config.toml` or `MINESTATUS_*` environment variables

## API

All endpoints take the query parameters `ip` (server address, required) and `cache` (use cached result, optional, defaults to `true`).

| Method | Path | Description |
|---|---|---|
| GET | `/` | Auto-detect Java / Bedrock and return status |
| GET | `/java` | Query a Java Edition server status |
| GET | `/bedrock` | Query a Bedrock Edition server status |
| GET | `/health` | Health check and version |
| GET | `/swagger-ui` | Swagger UI documentation |
| GET | `/api-docs/openapi.json` | OpenAPI specification |

> Note: `/java` and `/bedrock` also match their trailing-slash forms (`/java/`, `/bedrock/`).

### Java server icon

Append `/icon` to the `ip` value to get the server icon as a PNG image instead of JSON (returns `404` when the server is unreachable or has no icon):

```
GET /java/?ip=play.example.com/icon
```

### Example

```
GET /?ip=play.example.com
```

```json
{
  "online": true,
  "players": {
    "online": 3,
    "max": 20
  },
  "delay": 42.5,
  "version": "1.20.1",
  "motd": {
    "plain": "A Minecraft Server",
    "html": "<span style=\"color:#55FF55\">A Minecraft Server</span>",
    "minecraft": "§aA Minecraft Server",
    "ansi": "\u001b[92mA Minecraft Server"
  },
  "icon": "data:image/png;base64,iVBORw0KG..."
}
```

When the server is unreachable, the API responds with `200` and an error message:

```json
{ "error": "No server status detected. Is the server offline?" }
```

Missing or invalid query parameters return a FastAPI-style `422` error:

```json
{
  "detail": [
    {
      "type": "missing",
      "loc": ["query", "ip"],
      "msg": "Failed to deserialize query string: missing field `ip`",
      "input": null
    }
  ]
}
```

## Deployment

### Vercel (serverless)

Click the deploy button above, or set the `vercel` project manually (`vercel.json` rewrites all routes to `/api/src/main`).

The `vercel` cargo feature (enabled by default) builds the serverless-function entrypoint; the binary **must** be named `main` for Vercel's Rust builder.

### Local server

Requires a recent Rust toolchain (edition 2024). Run with the Vercel runtime disabled:

```
cargo run --no-default-features
```

The server listens on `http://0.0.0.0:3000` by default.

## Configuration

Priority: `config.toml` > environment variables (`MINESTATUS_*`) > defaults.

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

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [rust-mc-status](https://crates.io/crates/rust-mc-status)
- [Tokio](https://tokio.rs/)
- [MSCPO/MineStatus](https://github.com/MSCPO/MineStatus) — the original Python implementation this project mirrors