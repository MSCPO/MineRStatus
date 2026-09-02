# MineRStatus Wiki

欢迎来到 **MineRStatus** Wiki。MineRStatus 是一款用 **Rust** 编写的轻量级
Minecraft 服务器状态查询 API,是 [MSCPO/MineStatus](https://github.com/MSCPO/MineStatus)
的 Rust 重写版。

它通过状态协议查询 Java 和 Bedrock 服务器、自动探测服务器类型,并返回玩家
数量、版本、MOTD 与服务器图标,内置带 TTL 的响应缓存。

## 亮点

- 轻量快速(Rust / axum / 异步)
- 并行探测 Java 与 Bedrock,自动探测服务器类型
- 带 TTL 与容量限制的响应缓存
- OpenAPI / Swagger UI 文档
- 可部署到 Vercel(Serverless),也可自托管

## 快速开始

```bash
# 本地开发(禁用 Vercel 运行时)
cargo run --no-default-features
```

然后查询服务器:

```
GET /?ip=play.example.com
```

## 仓库

GitHub: [github.com/MSCPO/MineRStatus](https://github.com/MSCPO/MineRStatus)

[![Stars](https://img.shields.io/github/stars/MSCPO/MineRStatus?style=flat-square&logo=github&label=Stars)](https://github.com/MSCPO/MineRStatus/stargazers)
[![Forks](https://img.shields.io/github/forks/MSCPO/MineRStatus?style=flat-square&logo=github&label=Forks)](https://github.com/MSCPO/MineRStatus/network/members)
[![Watchers](https://img.shields.io/github/watchers/MSCPO/MineRStatus?style=flat-square&logo=github&label=Watchers)](https://github.com/MSCPO/MineRStatus/watchers)
[![Contributors](https://img.shields.io/github/contributors/MSCPO/MineRStatus?style=flat-square&logo=github&label=Contributors)](https://github.com/MSCPO/MineRStatus/graphs/contributors)
[![Issues](https://img.shields.io/github/issues/MSCPO/MineRStatus?style=flat-square&logo=github&label=Issues)](https://github.com/MSCPO/MineRStatus/issues)
[![License](https://img.shields.io/github/license/MSCPO/MineRStatus?style=flat-square&label=License)](LICENSE)

> Star / Fork / Watcher 数量通过 [shields.io](https://shields.io) 自动更新。
> Clone 数量只有通过 GitHub API 认证后才能获取,因此这里不展示。

## 目录

| 页面 | 说明 |
|---|---|
| [接口](endpoints.md) | 所有 HTTP 接口、参数与响应格式 |
| [部署](deployment.md) | Vercel Serverless 与本地自托管 |
| [配置](configuration.md) | `config.toml` 与环境变量 |
| [错误](errors.md) | 错误响应与状态码 |
| [贡献指南](contributing.md) | 如何贡献、开发环境与检查项 |