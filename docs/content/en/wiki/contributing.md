# Contributing

Thanks for considering contributing to **MineRStatus**! This page covers the
development setup, project layout and the checks that must pass before a pull
request is merged.

## Repository

- GitHub: <https://github.com/MSCPO/MineRStatus>
- Issue tracker: <https://github.com/MSCPO/MineRStatus/issues>
- Releases: <https://github.com/MSCPO/MineRStatus/releases>

## Development Setup

Requirements: a recent Rust toolchain (edition 2024).

```bash
git clone git@github.com:MSCPO/MineRStatus.git
cd MineRStatus

# run the local server (disables the Vercel runtime)
cargo run --no-default-features
```

The server listens on `http://0.0.0.0:3000` by default.

## Project Layout

```
src/
  app.rs     HTTP routes and handlers (axum)
  status.rs  Status querying, response cache and response models
  config.rs  Configuration loading (TOML > env > defaults)
  main.rs    Binary entrypoint (Vercel function / local server)
docs/        Static documentation site (see "Docs" below)
```

## How to Contribute

1. Fork the repository.
2. Create a feature branch: `git checkout -b feat/my-change`.
3. Make your changes and add tests where it makes sense.
4. Run the checks below.
5. Open a pull request against `main`.

## Checks Before Submitting

CI runs these automatically (`.github/workflows/ci.yaml`), so passing them
locally makes review faster:

```bash
cargo fmt --all -- --check
cargo clippy --no-default-features --all-targets -- -D warnings
cargo test --no-default-features
```

Notes:

- Tests/clippy use `--no-default-features` because the `vercel` feature links
  the upstream `vercel_runtime` crate, which only compiles inside Vercel's own
  build environment.
- Keep the HTTP response format stable — other projects depend on it.

## Docs

The docs site is static and stored in `docs/`:

- Content is organized per language: `docs/content/<lang>/`
- Wiki pages: `docs/content/<lang>/wiki/*.md`
- UI strings: `docs/content/<lang>/language.json` (missing keys fall back to
  English)
- The pages and UI text are bundled into JS so they render even when opened
  directly from disk (`file://`).

After editing any wiki page or `language.json`, rebuild the bundles:

```bash
node docs/build.js
```

## Release Process

Pushing to `main` triggers `.github/workflows/release.yaml`, which runs the
tests, builds a release binary and publishes a GitHub Release tagged with the
version from `Cargo.toml`. You do not need to tag releases manually.

## Code Style

- Rust 2024 edition, formatted with `rustfmt`.
- Follow the existing patterns; keep the API response format consistent.
- Add comments only to explain *why*, not *what*.