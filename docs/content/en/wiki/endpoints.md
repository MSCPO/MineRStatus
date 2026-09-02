# Endpoints

All endpoints accept the same query parameters:

| Parameter | Type | Required | Description |
|---|---|---|---|
| `ip` | string | **yes** | Minecraft server address/IP to query |
| `cache` | boolean | no | Use cached result. Defaults to `true`; set `false` to force a fresh query |

## Query server status (auto-detect)

```
GET /
```

Tries Java and Bedrock in parallel and returns the first successful result.

```
GET /?ip=play.example.com
```

When a Java address has **no explicit port**, the `_minecraft._tcp.<host>`
SRV record is resolved automatically (like the official client) before
connecting; provide an explicit port to skip SRV:

```
GET /java/?ip=play.example.com:25565
```

Appending `/icon` to the `ip` value returns the Java server icon as a PNG
image instead of JSON (servers icons exist for Java only; `404` when the
server is unreachable or has no icon):

```
GET /?ip=play.example.com/icon
```

For Java addresses without an explicit port, the `_minecraft._tcp.<host>`
SRV record is resolved automatically (like the official client) and the
server is queried at the SRV target; an explicit port skips SRV.

## Query a Java Edition server

```
GET /java
```

Also matches the trailing-slash form `/java/`.

Appending `/icon` to the `ip` value returns the server icon as a PNG image
instead of JSON (`404` when the server is unreachable or has no icon):

```
GET /java/?ip=play.example.com/icon
```

## Query a Bedrock Edition server

```
GET /bedrock
```

Also matches the trailing-slash form `/bedrock/`.

## Health check

```
GET /health
```

```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

## Documentation

```
GET /swagger-ui        # Swagger UI
GET /api-docs/openapi.json   # OpenAPI 3.1 specification
```

## Response Format

An online server returns `200` with:

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

| Field | Description |
|---|---|
| `online` | Whether the server responded |
| `players.online` / `players.max` | Current / max player counts |
| `delay` | Measured latency in milliseconds |
| `version` | Server software/version string |
| `motd` | Message of the Day in `plain`, `html`, `minecraft` and `ansi` encodings |
| `icon` | Base64 server icon (Java only; omitted when absent) |