# TokenDaily｜继续开发与跨电脑交接

本仓库已包含 Tauri 2 + React/TypeScript + Rust 应用，不是最初的三份需求资料包。不要重新从 M0 搭建，也不要用本文替代产品交接文档中的最新用户要求。

## 开始前

先读取 `AGENTS.md`、`TokenDaily-Codex-Handoff.md`、`docs/PROGRESS.md`，检查 Git 状态并保留用户修改。平台工作另读 `docs/ARCHITECTURE.md`、`docs/DEVELOPMENT_WORKFLOW.md` 与 `docs/FEATURE_PARITY.md`。

当前范围包括可开关悬浮球：普通球点击展开、离开收起，贴边半隐藏后悬停展开。今日 Token 使用本机日志、按北京时间切日；额度仍取只读账号接口，两者不混算。首页额度优先；主线已按用户更新加入免注册单设备自愿参榜，默认不上传。Mac 已补齐 WorkBuddy 官方积分登录与本机钥匙串存储，参榜仍禁用；实际验证见 FEATURE_PARITY。不做自动任务控制或主动推理刷新。

## 当前最重要的下一步

整合主线 0.1.3 与 Mac 原生适配，在 Mac 验收后再做 Windows 同版本回归。Mac 已完成 Intel 单屏与经授权的 /Applications 安装路径基础验收，不代表 Windows 通过，也不代表完整发行验收通过。准确测试记录、未验范围和运行命令见 `docs/REGRESSION_HANDOFF.md` 与 `docs/TEST_REPORT.md`。

远端暂时不可用时，可使用本地 Git bundle 交接。包外 `README-WINDOWS.md` 写明精确提交、SHA-256 和导入命令，`manifest.json` 为本次快照依据。离线指源码传递不依赖 GitHub；安装 npm/Cargo 依赖仍可能需要网络。包内不提供工具链、安装器、账户数据或签名材料。

## 给 Windows Codex 的接手提示词

```text
继续 TokenDaily 的 Windows 回归，不新增产品功能。

先读 AGENTS.md、TokenDaily-Codex-Handoff.md、docs/ARCHITECTURE.md、docs/DEVELOPMENT_WORKFLOW.md、docs/FEATURE_PARITY.md、docs/REGRESSION_HANDOFF.md 和最新 PROGRESS/TEST_REPORT，核对离线包 manifest 的提交号与干净工作区。

在 Windows 的短期 win/ 分支工作，不改写 Mac 任务分支。按交接清单运行真实测试和构建，回归共享来源配置、身份隔离、提醒去重/提交、悬浮拖动入口、托盘和安装态。先记录失败证据，再修复；数据/时间/状态/通知改动先写测试。保留本机现有修改、凭据、设置、Codex 日志和工具链。

安装前检查现有安装版及授权范围；不要将 Mac 的替换授权视为 Windows 安装授权。不要改共享 Codex 配置、发起推理、消费重置券或自动续跑任务。更新同一 FEATURE_PARITY 条目、PROGRESS 和 TEST_REPORT；未执行的项目明确写未验。未经另外授权不推送、合并、签名、公证或发布。
```

## 本机开发入口

- Mac：`bash scripts/mac-dev.sh npm run tauri -- dev`，开发态不用于系统通知验收。
- Windows：先按 `docs/ARCHITECTURE.md` 准备本机工具链，在 PowerShell 中加载 `scripts/use-local-dev-env.ps1`，再执行回归清单。离线包不包含 `.local-toolchain/`；已有工具链不能被新克隆覆盖。
- Git bundle 可导入现有干净仓库或克隆至新目录；不要把压缩包解压覆盖源码，不要用 reset/clean 清掉另一台电脑的修改。
