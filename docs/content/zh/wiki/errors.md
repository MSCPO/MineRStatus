# 错误

## 服务器不可达(状态查询)

服务器无法连接时,状态接口仍返回 **HTTP 200**,但响应体为错误信息:

```json
{
  "error": "Failed to connect to Java server at play.example.com: ..."
}
```

短暂的**超时与连接失败会进行重试**后才返回错误 —— 位于区域防火墙或
反 DDoS 保护之后的服务器可能会间歇性丢弃连接,快速重试通常能够通过。
DNS 解析失败不会重试。整个尝试序列受总预算(`query.max_total`,默认 9 秒)
限制,以保证在到达任何无服务器执行时长上限之前返回 JSON 错误。

自动探测接口在两种协议都失败时:

```json
{
  "error": "No server status detected. Is the server offline?"
}
```

## 查询参数非法

缺少或非法的查询参数返回 **HTTP 422 Unprocessable Entity**,响应体为
FastAPI 风格的校验错误:

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

## 图标接口

在 `/java` 的 `ip` 值后追加 `/icon` 可返回服务器图标的 PNG。当服务器不可达、
没有图标或图标数据非法时,接口返回 **HTTP 404** 与 JSON 错误:

```json
{
  "error": "Server has no icon"
}
```

## 汇总

| 场景 | 状态码 | 响应体 |
|---|---|---|
| 服务器不可达(JSON 查询) | `200` | `{"error": "..."}` |
| 缺少 `ip` | `422` | FastAPI 风格 `detail` 数组 |
| 图标:服务器不可达 / 无图标 / 数据非法 | `404` | `{"error": "..."}` |
| 未知路由 | `404` | 纯文本 |