# Errors

## Server unreachable (status queries)

When a server cannot be reached, the status endpoints still return **HTTP 200**
with an error body:

```json
{
  "error": "Failed to connect to Java server at play.example.com: ..."
}
```

For the auto-detect endpoint, when both protocols fail:

```json
{
  "error": "No server status detected. Is the server offline?"
}
```

## Invalid query parameters

Missing or invalid query parameters return **HTTP 422 Unprocessable Entity**
with a FastAPI-style validation body:

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

## Icon endpoint

Appending `/icon` to the `ip` value of `/java` returns the server icon as a
PNG. When the server is unreachable, has no icon, or the icon data is invalid,
the endpoint returns **HTTP 404** with a JSON error body:

```json
{
  "error": "Server has no icon"
}
```

## Summary

| Situation | Status | Body |
|---|---|---|
| Server unreachable (JSON query) | `200` | `{"error": "..."}` |
| Missing `ip` | `422` | FastAPI-style `detail` array |
| Icon: server unreachable / no icon / bad data | `404` | `{"error": "..."}` |
| Unknown route | `404` | Plain text |