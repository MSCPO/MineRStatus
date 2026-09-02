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
| `max_total` | `9` | `MINESTATUS_MAX_TOTAL` | 单次查询含重试的总预算(秒)。请保持在平台函数执行时长上限之下(Vercel Hobby 为 10 秒),否则查询失败时会被运行时杀掉而不是返回 JSON 错误 |

### `[dns]`

最多三个自定义 DNS 服务器(支持 IPv4 或 IPv6),用于 SRV / A 记录解析。
留空则使用系统 DNS。

| 键 | 默认值 | 环境变量 | 说明 |
|---|---|---|---|
| `primary` | *(空)* | `MINESTATUS_DNS_PRIMARY` | 主 DNS 服务器 |
| `secondary1` | *(空)* | `MINESTATUS_DNS_SECONDARY1` | 第一个副 DNS 服务器 |
| `secondary2` | *(空)* | `MINESTATUS_DNS_SECONDARY2` | 第二个副 DNS 服务器 |

```toml
[dns]
primary = "223.5.5.5"
secondary1 = "119.29.29.29"
secondary2 = "2400:3200::1"   # 支持 IPv6
```

配置了自定义服务器时,SRV 与 A 记录查询都会走这些服务器;否则使用系统
DNS(SRV 另有 DNS-over-HTTPS 兜底)。

## 优先级示例

不修改 `config.toml`,仅本次会话改用 `8080` 端口:

```bash
MINESTATUS_PORT=8080 cargo run --no-default-features
```

环境变量会覆盖 `config.toml`,`config.toml` 会覆盖内置默认值。