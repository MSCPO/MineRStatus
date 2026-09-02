# 贡献指南

感谢你考虑为 **MineRStatus** 贡献代码!本页介绍开发环境、项目结构,以及
Pull Request 合并前必须通过的检查项。

## 仓库

- GitHub: <https://github.com/MSCPO/MineRStatus>
- Issue 追踪: <https://github.com/MSCPO/MineRStatus/issues>
- Release: <https://github.com/MSCPO/MineRStatus/releases>

## 开发环境

需要较新的 Rust 工具链(edition 2024)。

```bash
git clone git@github.com:MSCPO/MineRStatus.git
cd MineRStatus

# 运行本地服务器(禁用 Vercel 运行时)
cargo run --no-default-features
```

默认监听 `http://0.0.0.0:3000`。

## 项目结构

```
src/
  app.rs     HTTP 路由与处理器(axum)
  status.rs  状态查询、响应缓存与响应模型
  config.rs  配置加载(TOML > 环境变量 > 默认值)
  main.rs    二进制入口(Vercel 函数 / 本地服务器)
docs/        静态文档站点(见下方"文档")
```

## 如何贡献

1. Fork 本仓库。
2. 创建功能分支:`git checkout -b feat/my-change`。
3. 做出修改,并在合理的地方补充测试。
4. 运行下面的检查项。
5. 向 `main` 分支发起 Pull Request。

## 提交前检查

CI 会自动运行这些检查(`.github/workflows/ci.yaml`),因此本地先通过可以加快
审查:

```bash
cargo fmt --all -- --check
cargo clippy --no-default-features --all-targets -- -D warnings
cargo test --no-default-features
```

说明:

- 测试与 clippy 使用 `--no-default-features`,因为 `vercel` feature 会链接
  上游 `vercel_runtime` crate,该 crate 只能在 Vercel 自身的构建环境中编译。
- 请保持 HTTP 响应格式稳定 —— 其他项目依赖它。

## 文档

文档站点是静态的,存放在 `docs/` 中:

- 内容按语言组织:`docs/content/<lang>/`
- Wiki 页面:`docs/content/<lang>/wiki/*.md`
- UI 文案:`docs/content/<lang>/language.json`(缺失字段回退英文)
- 页面与 UI 文案会被打包进 JS,以保证即使直接以 `file://` 打开也能渲染。

编辑任何 wiki 页面或 `language.json` 后,重新构建 bundle:

```bash
node docs/build.js
```

## 发布流程

推送到 `main` 会触发 `.github/workflows/release.yaml`:运行测试、构建发布版
二进制,并用 `Cargo.toml` 中的版本号打 tag 发布 GitHub Release。无需手动打
tag。

## 代码风格

- Rust 2024 edition,使用 `rustfmt` 格式化。
- 遵循现有模式,保持 API 响应格式一致。
- 注释只解释"为什么",不解释"是什么"。