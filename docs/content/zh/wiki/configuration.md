# 配置

优先级:`config.toml` > 环境变量(`MINESTATUS_*`)> 默认值。

默认的 `config.toml` 位于仓库根目录:

```toml
[server]
host = "0.0.0.0"   # 环境变量:MINESTATUS_HOST
port = 3000        # 环境变量:MINESTATUS_PORT

[cache]
ttl = 600          # 环境变量:MINESTATUS_CACHE_TTL  (秒)
max_size = 100     # 环境变量:MINESTATUS_CACHE_MAX_SIZE

[query]
timeout = 8        # 环境变量:MINESTATUS_TIMEOUT    (秒)
```

## 配置项

### `[server]`

| 键 | 默认值 | 环境变量 | 说明 |
|---|---|---|---|
| `host` | `0.0.0.0` | `MINESTATUS_HOST` | 绑定地址 |
| `port` | `3000` | `MINESTATUS_PORT` | 监听端口 |

### `[cache]`

| 键 | 默认值 | 环境变量 | 说明 |
|---|---|---|---|
| `ttl` | `600` | `MINESTATUS_CACHE_TTL` | 响应缓存 TTL(秒) |
| `max_size` | `100` | `MINESTATUS_CACHE_MAX_SIZE` | 缓存条目数上限 |

缓存存储完整的响应(包括测量的延迟),因此缓存命中会返回条目创建时记录的
延迟。设置 `ttl = 0` 可完全禁用缓存。

### `[query]`

| 键 | 默认值 | 环境变量 | 说明 |
|---|---|---|---|
| `timeout` | `8` | `MINESTATUS_TIMEOUT` | DNS 解析与连接超时(秒) |

## 优先级示例

不修改 `config.toml`,仅本次会话改用 `8080` 端口:

```bash
MINESTATUS_PORT=8080 cargo run --no-default-features
```

环境变量会覆盖 `config.toml`,`config.toml` 会覆盖内置默认值。