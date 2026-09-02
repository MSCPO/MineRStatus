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

## Repository

GitHub: [github.com/MSCPO/MineRStatus](https://github.com/MSCPO/MineRStatus)

[![Stars](https://img.shields.io/github/stars/MSCPO/MineRStatus?style=flat-square&logo=github&label=Stars)](https://github.com/MSCPO/MineRStatus/stargazers)
[![Forks](https://img.shields.io/github/forks/MSCPO/MineRStatus?style=flat-square&logo=github&label=Forks)](https://github.com/MSCPO/MineRStatus/network/members)
[![Watchers](https://img.shields.io/github/watchers/MSCPO/MineRStatus?style=flat-square&logo=github&label=Watchers)](https://github.com/MSCPO/MineRStatus/watchers)
[![Contributors](https://img.shields.io/github/contributors/MSCPO/MineRStatus?style=flat-square&logo=github&label=Contributors)](https://github.com/MSCPO/MineRStatus/graphs/contributors)
[![Issues](https://img.shields.io/github/issues/MSCPO/MineRStatus?style=flat-square&logo=github&label=Issues)](https://github.com/MSCPO/MineRStatus/issues)
[![License](https://img.shields.io/github/license/MSCPO/MineRStatus?style=flat-square&label=License)](LICENSE)

> Star / fork / watcher counts update automatically via
> [shields.io](https://shields.io). Clone counts are only exposed by the
> GitHub API with authentication, so they are not shown here.

## Contents

| Page | Description |
|---|---|
| [Endpoints](endpoints.md) | All HTTP endpoints, parameters and response format |
| [Deployment](deployment.md) | Vercel serverless and local self-hosting |
| [Configuration](configuration.md) | `config.toml` and environment variables |
| [Errors](errors.md) | Error responses and status codes |
| [Contributing](contributing.md) | How to contribute, dev setup and checks |