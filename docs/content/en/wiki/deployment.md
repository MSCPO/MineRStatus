# Deployment

## Vercel (serverless)

Click the one-click deploy button in the repository README, or create a Vercel
project from the repository manually. `vercel.json` rewrites all routes to the
serverless function entrypoint:

```json
{
  "rewrites": [
    { "source": "/(.*)", "destination": "/api/src/main" }
  ]
}
```

The `vercel` cargo feature (enabled by default) builds the serverless-function
entrypoint. Vercel's Rust builder hardcodes the entrypoint at `src/main.rs`
and runs `cargo build --bin main`, so the binary **must** be named `main`.

## Local server

Requires a recent Rust toolchain (edition 2024). Run with the Vercel runtime
disabled:

```bash
cargo run --no-default-features
```

The server listens on `http://0.0.0.0:3000` by default (see
[Configuration](configuration.md) to change host/port).

### Build a release binary

```bash
cargo build --release --no-default-features
./target/release/main
```

## Notes

- The `vercel` feature links the upstream `vercel_runtime` crate, which does
  not compile outside Vercel's own build environment. Use
  `--no-default-features` for local builds — this is also what CI uses.
- When querying servers that are far away from the deployment region (for
  example Chinese servers from a US region), connections can be slow or
  intermittently dropped. If you self-host, pick a region close to the servers
  you query; on Vercel you can raise `MINESTATUS_TIMEOUT` if the default 8 s
  is too tight, but keep it below the function execution limit.