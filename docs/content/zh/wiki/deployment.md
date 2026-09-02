# 部署

## Vercel(Serverless)

点击仓库 README 中的一键部署按钮,或在 Vercel 中手动从仓库创建项目。
`vercel.json` 将所有路由重写到 Serverless 函数入口:

```json
{
  "rewrites": [
    { "source": "/(.*)", "destination": "/api/src/main" }
  ]
}
```

`vercel` cargo feature(默认开启)用于构建 Serverless 函数入口。Vercel 的
Rust 构建器硬编码入口为 `src/main.rs` 并运行 `cargo build --bin main`,
因此二进制 **必须** 命名为 `main`。

## 本地运行

需要较新的 Rust 工具链(edition 2024)。以禁用 Vercel 运行时的模式运行:

```bash
cargo run --no-default-features
```

默认监听 `http://0.0.0.0:3000`(修改 host/port 见
[配置](configuration.md))。

### 构建发布版二进制

```bash
cargo build --release --no-default-features
./target/release/main
```

## 说明

- `vercel` feature 会链接上游 `vercel_runtime` crate,该 crate 在 Vercel 自身
  的构建环境之外无法编译。本地构建请使用 `--no-default-features` —— CI 也是
  这样运行的。
- 当查询距离部署区域较远的服务器时(例如从美区部署查询中国服务器),连接
  可能会较慢或被间歇性丢弃。如果自托管,请选择离目标服务器较近的区域;在
  Vercel 上,若默认的 8 秒超时不够,可通过 `MINESTATUS_TIMEOUT` 调大,但
  不要超过函数执行时长上限。
- **Fluid compute**:Vercel 的 Fluid compute(新项目默认开启)会根据负载与
  可用性动态放置函数实例,可能让实际执行区域与出口 IP 偏离你配置的区域,
  而部分服务器会封锁这些 IP 段。为让出口可预测,请在 `vercel.json` 固定
  区域(`"regions": ["sin1"]`,可覆盖 Fluid 默认值),并在项目设置的
  Functions 中关闭 Fluid compute,然后重新部署。