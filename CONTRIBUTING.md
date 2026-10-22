# 参与 TokenDaily 共建

感谢你帮助改进 TokenDaily。桌面端免费提供，原始源码和文档采用 [Apache-2.0](LICENSE)；官网托管服务及 AI 工具情报、优惠与更新提醒订阅独立运营，不包含在本仓库授权中。贡献不要求购买订阅。

## 从哪里开始

- 使用方式和真实支持范围见 [README](README.md)。数据来源和平台边界见 [架构](docs/ARCHITECTURE.md)，Windows/macOS 的逐项实现与验收见 [功能对照](docs/FEATURE_PARITY.md)。
- 产品范围以 [交接文档](TokenDaily-Codex-Handoff.md) 顶部最新决定为准，旧阶段记录不代表当前限制。较大的功能、来源接入或架构调整先开 Issue 讨论目标、数据来源和验收条件。
- 适合初次贡献的工作：文案和文档修正、合成样例与解析测试、已支持工具的安装路径兼容、可复现的平台问题。先描述具体问题，再提出实现。
- 涉及漏洞、凭据、越权或更新签名的问题请按 [安全说明](SECURITY.md) 私密报告。

## 本地开发

需要 Node.js 24 或兼容的更新版本、Rust stable，以及 [Tauri 对应系统的构建依赖](https://v2.tauri.app/start/prerequisites/)。Windows 需要 C++ Build Tools、Windows SDK 和 WebView2；macOS 需要 Xcode Command Line Tools。请先准备系统环境，项目脚本不会替你全局安装这些依赖。

```bash
npm ci
npm test
npm run build
npm run tauri dev
```

普通开发与测试不需要发布签名私钥。未连接来源时界面显示未连接；开发服务器可用 `?demo=normal` 查看明确标记的合成数据。生产界面不启用演示参数。

若当前检出目录已有隔离工具链，Windows 可在同一个 PowerShell 加载 `. ./scripts/use-local-dev-env.ps1`；macOS 可用 `bash scripts/mac-dev.sh <command>`。没有该本机工具链时使用系统 Rust 即可，不必执行 Windows 隔离环境脚本。`scripts/*migration*.ps1`、`move-*.ps1` 等是历史维护脚本，不属于构建步骤，不要为开发本应用而运行它们。

## 分支与代码边界

从最新 `main` 建短期 `win/<task>`、`mac/<task>` 或 `shared/<task>` 分支。每个 PR 聚焦一个问题，保留维护者和其他贡献者的修改，不改写他人分支历史。双机协作细则见 [DEVELOPMENT_WORKFLOW](docs/DEVELOPMENT_WORKFLOW.md)。其中维护者的自动推送约定只适用于其已授权环境，不授予贡献者主线或生产发布权限。

- React、TypeScript 数据规则、Rust 数据契约及采集、时间、刷新、提醒规则共用。原生能力放在 `src-tauri/src/platform/windows/` 或 `macos/`，平台配置放在对应 `tauri.*.conf.json`。
- 修改状态、日期归属、累计/增量转换、去重、身份隔离和提醒前，先补能复现问题的测试。未知、失败、过期不等于零；倒计时结束不代表额度已恢复。
- 改共享契约时检查 Rust 序列化、TypeScript 调用、旧缓存兼容及两端状态。新功能在 `docs/FEATURE_PARITY.md` 的同一条目写统一验收条件，再分别填写平台结果。
- 新来源需说明官方或本机数据依据、授权方式、字段口径、刷新频率和失败状态。只读最少必要信息；不通过模型推理、购买或消费重置券来验证，不增加默认遥测。
- 测试使用合成、脱敏样例，不提交真实日志、数据库、凭据、用户目录、昵称、真实用量或账户截图。仅某台电脑需要的忽略项放入该机 `.git/info/exclude`。
- `package-lock.json`、`src-tauri/Cargo.lock`、平台配置与所需图标继续跟踪；新增依赖需说明必要性及许可证。引用他人代码要保留来源、版权和许可。

## 提交前验证

按变更运行必要检查，记录实际结果。仅改文档时检查链接、示例命令与差异即可；不要把没运行的应用测试标为通过。

```bash
npm test
npm run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --locked --manifest-path src-tauri/Cargo.toml --lib
cargo check --locked --manifest-path src-tauri/Cargo.toml --all-targets
git diff --check
git status --short --untracked-files=all
git diff --cached --name-status
```

macOS 的 Rust 测试和编译检查同时添加 `--features macos-transparent`；Windows 不启用该平台特性。原生或窗口配置改动还需在对应系统运行构建与相关交互检查。真实来源测试默认忽略，只能在你自己已授权的本机环境显式运行，不能要求他人提交登录材料。

PR 请说明问题与最终行为、涉及的平台、执行过的命令、真实/演示数据边界和未验证项。改产品行为时更新 `docs/PROGRESS.md` 与功能对照。Windows 通过不代表 Mac 通过；未在相应实机运行就写“未验证”。维护者负责合并、官方构建、签名、更新通道与发布。

## 贡献许可与交流

提交贡献时，请确认你有权提交这些内容，并同意按本仓库 Apache-2.0 条款纳入，第三方材料另附原许可。作者保留其版权；当前不要求版权转让。许可的具体条件以 [LICENSE](LICENSE) 为准，现有第三方声明见 [THIRD_PARTY_NOTICES](THIRD_PARTY_NOTICES.txt)。

欢迎有依据的不同意见。讨论聚焦问题、复现步骤和方案，尊重他人；不发布个人资料、骚扰信息、广告或无关推广。AI 辅助提交由提交者检查和负责，避免未经验证的大段改动。
