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

## 目录

| 页面 | 说明 |
|---|---|
| [接口](endpoints.md) | 所有 HTTP 接口、参数与响应格式 |
| [部署](deployment.md) | Vercel Serverless 与本地自托管 |
| [配置](configuration.md) | `config.toml` 与环境变量 |
| [错误](errors.md) | 错误响应与状态码 |