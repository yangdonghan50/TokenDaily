# 跨平台回归交接

2026-10-17 更新：当前接手目标为 `mac/main-0.1.3-sync` 本节所在合并提交（父提交 Mac `ee3e1c9` / main `1b06e57`）。交接前记录 `git rev-parse HEAD`，重新打包或经用户授权推送。下文旧 `mac/floating-panel`、918ff99 及旧离线包仅供追溯，不能作为当前回归版本。本清单是待执行步骤，不是通过记录。

新增回归范围：三来源/固定总览/排序、Claude 原生提醒状态与持久去重、WorkBuddy 积分与排行榜的 Windows 安全存储兼容性。Mac 已安装 0.1.3，本轮前端 54、Rust 87 通过/4 忽略；不代表 Windows 验收通过。旧模式保留后端兼容，UI 固定总览，不再验模式切换。

## 接手条件

- 当前远端 fetch 返回 HTTP 403，代码仅在本机提交，未推送/合并。可用经本地校验的 Git bundle 离线交接，按包外 README-WINDOWS.md 导入并比对 manifest 中的精确提交；不依赖 GitHub 推送。恢复远端后仍需单独确认推送，不要使用旧远端代码冒充本轮回归。
- 接手先核对 `git status --short --branch` 和 `git log -1 --oneline`，确认精确匹配交接 manifest 且包含 `918ff99`。Windows 新建短期 win/ 回归分支，保留 Mac 分支。若本机有未提交修改或另一路新提交，先检查差异，不自动覆盖/合并；双机协作遵守 DEVELOPMENT_WORKFLOW.md。
- 回归不仅涉及悬浮窗：此本地提交链还包含来源配置、身份 v2、提醒持久去重、后台通知调度及目录监听降级提示。功能范围见 FEATURE_PARITY.md 中 CHG-20261017-03 至 -07 及共享身份提醒条目。
- 共享调用重点：`src/ui/useFloatingHover.ts`、`src-tauri/src/lib.rs`、`platform/floating.rs` 与 `platform/windows/floating_drag.rs`；同时复核 `connection.rs`、`codex.rs`、`alerts.rs`、`notification.rs`、通知设置 UI 和统一 Cargo.lock。不复制 Mac 原生实现到 Windows。

## Windows 命令

在已准备好本项目工具链的 PowerShell 中逐条执行；任一步失败先保留输出、修复原因，再继续。

```powershell
. .\scripts\use-local-dev-env.ps1
npm.cmd ci
npm.cmd test
npm.cmd run build
cargo fmt --manifest-path src-tauri/Cargo.toml --check
cargo test --locked --manifest-path src-tauri/Cargo.toml
npm.cmd run tauri -- build --no-sign -- --locked
git diff --check
```

Windows 不启用 `macos-transparent`。记录真实测试数量，不照抄 Mac 的 62 项：平台专属测试不同。默认忽略的真实账户/日志专项仍是未运行；按用途单独执行时不能触发模型推理来制造数据。

## Windows 实机与安装态

| 项目 | 通过条件 | 当前状态 |
|---|---|---|
| 原生拖动入口 | 普通球悬停不展开；点击展开；移开收起；拖动不被误判点击；Escape 取消可恢复 | 本轮未验 |
| 左右贴边 | 可见球边缘触工作区时按住即预览；回拖解除；松手半隐藏；离开再悬停展开；半隐藏可直接拖离 | 本轮未验 |
| 非触边操作 | 靠近边缘、开启贴边开关、展开/收起均不新建吸附；关闭贴边恢复完整球 | 本轮未验 |
| 屏幕和任务栏 | 顶底及侧边任务栏不遮球；展开不出工作区；多屏/混合缩放/负坐标分别记录 | 本轮未验 |
| 托盘和持久设置 | 四个菜单操作、主窗关闭后常驻、重启保留开关和模式；退出只清理本工具资源 | 本轮未验 |
| 来源和身份 | 只读连接正确来源；有效目录一致；旧缓存不伪报新鲜；身份未核验不得触发额度提醒；断连迟到结果不恢复连接 | 本轮未验 |
| Token 与刷新 | 今日使用本机日志、北京时间切日，读取失败非零；额度失败不污染 Token 新鲜度；断连不注销 Codex | 本轮未验 |
| 通知 | 安装路径下测试请求、实际通知和点击分别记录；Windows queued 不等于送达/已授权；重启保留去重 | 本轮未验 |
| 安装和退出 | 记录 Windows/架构/缩放、安装器路径及版本；已授权安装后验证启动、单实例、隐藏恢复、重启和退出 | 本轮未验 |

实际低额度/恢复需自然出现的新数据支持；合成单测、人工测试提醒不能替代真实事件验收。禁止消耗重置券、主动推理或自动续跑任务。

## Mac 剩余项

- 已完成：Intel 单屏交互、固定底部 Dock 工作区边界、本应用全屏往返、主窗口隐藏恢复；透明边缘和菜单栏图标由用户目视确认。
- 待补：其他应用全屏、多桌面切换、多屏混合 DPI、Dock 换边/自动隐藏、睡眠唤醒、长期驻留、Apple Silicon 和最低支持系统。
- 用户已授权备份并替换安装版，`918ff99` 对应 App 已安装于 `/Applications/TokenDaily.app`。安装路径启动、单实例、设置保留、正常退出、测试通知中心记录/隐藏恢复/退出后启动通过，见 TEST_REPORT.md。旧版归档及原 bundle 保留在忽略的本机备份目录；没有修改用户设置和 Codex 数据。首次授权、移除、真实事件、全新下载 Gatekeeper 流程仍未验。
- 本次安装复用已授权的 ad-hoc 签名构建 App，没有重新签名；现有 DMG 中仍是未签名 App。正式签名、公证、上传和发布需另行授权。

完成后把环境、代码 SHA、逐项结果和失败复现追加到 TEST_REPORT.md/PROGRESS.md，并更新同一 FEATURE_PARITY 条目。两端对同一代码通过前不合主线、不标同步完成。
