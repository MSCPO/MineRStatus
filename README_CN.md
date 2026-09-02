# MineRStatus API

![License](https://img.shields.io/github/license/MSCPO/MineRStatus?style=for-the-badge) [![GitHub Release](https://img.shields.io/github/release/MSCPO/MineRStatus.svg?style=for-the-badge&logo=Qase&color=005AA4)](https://github.com/MSCPO/MineRStatus/releases/latest)<br>[![Contributors](https://img.shields.io/github/contributors-anon/MSCPO/MineRStatus.svg?style=flat-square&logo=github&color=005AA4)](https://github.com/MSCPO/MineRStatus/graphs/contributors) [![Forks](https://img.shields.io/github/forks/MSCPO/MineRStatus?style=flat-square&logo=github&logoColor=fff&color=005AA4)](https://github.com/MSCPO/MineRStatus/network/members) [![Stars](https://img.shields.io/github/stars/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4)](https://github.com/MSCPO/MineRStatus/stargazers) [![Issues Open](https://img.shields.io/github/issues/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/issues) [![Issues Closed](https://img.shields.io/github/issues-closed/MSCPO/MineRStatus.svg?style=flat-square&logo=github&logoColor=fff&color=005AA4&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/issues?q=is%3Aissue+is%3Aclosed) [![GitHub Discussions](https://img.shields.io/github/discussions/MSCPO/MineRStatus?style=flat-square&logo=github&logoColor=fff&color=953B00&cacheSeconds=300)](https://github.com/MSCPO/MineRStatus/discussions)

<p align="center">
  <a href="https://vercel.com/new/clone?repository-url=https://github.com/MSCPO/MineRStatus">
    <img src="https://vercel.com/button" alt="Deploy with Vercel" />
  </a>
</p>

[English](README.md)

## 简介

一款用 **Rust** 编写的轻量级 Minecraft 服务器查询 API，是 [MSCPO/MineStatus](https://github.com/MSCPO/MineStatus) 的 Rust 重写版。它通过状态协议查询 Java 和 Bedrock 服务器、自动探测服务器类型，并返回玩家数量、版本、MOTD 和服务器图标，内置 TTL 响应缓存。

## 特性

- 轻量快速（Rust / axum / 异步）
- 可免费部署到 Vercel（Serverless 函数）
- 自动探测 Java / Bedrock，并行查询
- 带 TTL 和容量限制的响应缓存
- OpenAPI / Swagger UI 文档
- 通过 `config.toml` 或 `MINESTATUS_*` 环境变量配置

## 文档

仓库自带静态文档站点，位于 `docs/` 目录：

- `docs/index.html` — 首页
- `docs/wiki.html` — Wiki（markdown，客户端渲染）
- `docs/api-test.html` — 在线 API 测试页

内容按语言放在 `docs/content/<语言代码>/` 下：

- `language.json` — 站点的 UI 文案（缺失的字段会回退到 `en`）
- `wiki/*.md` — 该语言的 Wiki 页面

语言根据浏览器自动检测（`?lang=` 可覆盖；导航中的语言选择器会记住选择）。
Wiki 的 markdown 与 UI 文案会被打包进 `*.js` 文件，以保证即使直接以
`file://` 打开页面（浏览器会禁用 `fetch()`）也能正常渲染。编辑任何内容后
需重新构建：

```
node docs/build.js
```

## API

所有端点均接受查询参数 `ip`（服务器地址，必填）和 `cache`（是否使用缓存，可选，默认 `true`）。

| 方法 | 路径 | 说明 |
|---|---|---|
| GET | `/` | 自动探测 Java / Bedrock 并返回状态 |
| GET | `/java` | 查询 Java 版服务器状态 |
| GET | `/bedrock` | 查询 Bedrock 版服务器状态 |
| GET | `/health` | 健康检查与版本 |
| GET | `/api-docs` | Swagger UI 文档 |
| GET | `/api-docs/openapi.json` | OpenAPI 规范 |

> 注意：`/java` 和 `/bedrock` 同时匹配带斜杠的形式（`/java/`、`/bedrock/`）。

### Java 服务器图标

在 `ip` 末尾追加 `/icon`，可返回服务器图标的 PNG 图片而不是 JSON（服务器不可达或无图标时返回 `404`）：

```
GET /java/?ip=play.example.com/icon
```

### 示例

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

服务器不可达时，API 返回 `200` 及错误信息：

```json
{ "error": "No server status detected. Is the server offline?" }
```

查询参数缺失或非法时，返回 FastAPI 风格的 `422` 错误：

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

## 部署

### Vercel（Serverless）

点击上方部署按钮，或在 Vercel 中手动配置项目（`vercel.json` 将所有路由重写到 `/api/src/main`）。

`vercel` cargo feature（默认开启）用于构建 Serverless 函数入口；二进制 **必须** 命名为 `main`，因为 Vercel 的 Rust 构建器硬编码了该入口。

### 本地运行

需要较新的 Rust 工具链（edition 2024）。以禁用 Vercel 运行时的模式运行：

```
cargo run --no-default-features
```

默认监听 `http://0.0.0.0:3000`。

## 配置

优先级：`config.toml` > 环境变量（`MINESTATUS_*`）> 默认值。

```toml
[server]
host = "0.0.0.0"   # 环境变量：MINESTATUS_HOST
port = 3000        # 环境变量：MINESTATUS_PORT

[cache]
ttl = 600          # 环境变量：MINESTATUS_CACHE_TTL  （秒）
max_size = 100     # 环境变量：MINESTATUS_CACHE_MAX_SIZE

[query]
timeout = 8        # 环境变量：MINESTATUS_TIMEOUT    （秒）
```

## 鸣谢

- [Axum](https://github.com/tokio-rs/axum)
- [rust-mc-status](https://crates.io/crates/rust-mc-status)
- [Tokio](https://tokio.rs/)
- [MSCPO/MineStatus](https://github.com/MSCPO/MineStatus) —— 本项目所镜像的原始 Python 实现