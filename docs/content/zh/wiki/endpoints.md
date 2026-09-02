# 接口

所有接口都接受相同的查询参数:

| 参数 | 类型 | 必填 | 说明 |
|---|---|---|---|
| `ip` | string | **是** | 要查询的 Minecraft 服务器地址/IP |
| `cache` | boolean | 否 | 是否使用缓存。默认 `true`;设为 `false` 强制刷新查询 |

## 查询服务器状态(自动探测)

```
GET /
```

并行尝试 Java 与 Bedrock,返回第一个成功的结果。

```
GET /?ip=play.example.com
```

当 Java 地址**未指定端口**时,会自动解析 `_minecraft._tcp.<host>`
SRV 记录(与官方客户端一致)后再连接;指定端口则跳过 SRV:

```
GET /java/?ip=play.example.com:25565
```

在 `ip` 值后追加 `/icon` 可返回 Java 服务器图标的 PNG 图片而不是 JSON
(服务器图标仅 Java 有;服务器不可达或无图标时返回 `404`):

```
GET /?ip=play.example.com/icon
```

## 查询 Java 版服务器

```
GET /java
```

同时匹配带斜杠的形式 `/java/`。

在 `ip` 值后追加 `/icon` 可返回服务器图标的 PNG 图片而不是 JSON
(服务器不可达或无图标时返回 `404`):

```
GET /java/?ip=play.example.com/icon
```

## 查询 Bedrock 版服务器

```
GET /bedrock
```

同时匹配带斜杠的形式 `/bedrock/`。

## 健康检查

```
GET /health
```

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

## 文档

```
GET /swagger-ui              # Swagger UI
GET /api-docs/openapi.json   # OpenAPI 3.1 规范
```

## 响应格式

在线服务器返回 `200`,响应体如下:

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
    "minecraft": "\u00a7aA Minecraft Server",
    "ansi": "\u001b[92mA Minecraft Server"
  },
  "icon": "data:image/png;base64,iVBORw0KG..."
}
```

| 字段 | 说明 |
|---|---|
| `online` | 服务器是否有响应 |
| `players.online` / `players.max` | 当前 / 最大玩家数 |
| `delay` | 测量的延迟(毫秒) |
| `version` | 服务器软件/版本字符串 |
| `motd` | 以 `plain`、`html`、`minecraft`、`ansi` 四种编码表示的服务器公告(MOTD) |
| `icon` | Base64 服务器图标(仅 Java;无图标时省略该字段) |