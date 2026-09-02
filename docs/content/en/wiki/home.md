# MineRStatus Wiki

Welcome to the **MineRStatus** wiki. MineRStatus is a lightweight Minecraft
server status query API written in **Rust**, and a rewrite of
[MSCPO/MineStatus](https://github.com/MSCPO/MineStatus).

It queries Java and Bedrock Edition servers over their status protocols,
auto-detects the server type, and returns player counts, version, MOTD and the
server icon — with a built-in TTL response cache.

## Highlights

- Fast and lightweight (Rust / axum / async)
- Auto-detects Java vs Bedrock by probing both in parallel
- Response caching with TTL and capacity limit
- OpenAPI / Swagger UI documentation
- Deployable to Vercel as a serverless function, or self-hosted

## Quick Start

```bash
# local development (disables the Vercel runtime)
cargo run --no-default-features
```

Then query a server:

```
GET /?ip=play.example.com
```

## Contents

| Page | Description |
|---|---|
| [Endpoints](endpoints.md) | All HTTP endpoints, parameters and response format |
| [Deployment](deployment.md) | Vercel serverless and local self-hosting |
| [Configuration](configuration.md) | `config.toml` and environment variables |
| [Errors](errors.md) | Error responses and status codes |