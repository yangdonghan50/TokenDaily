# Windows 与 macOS 双机协作规则

## 2026-10-22 公开仓库起点

桌面共建仓库为 [TokenDaily-Desktop](https://github.com/yangdonghan50/TokenDaily-Desktop)，公开 `main` 从脱敏后的独立根提交开始。原私库留存旧历史；旧私库的分支、标签及本地候选引用不能整体推送或合并到公开仓库，也不使用 `--mirror`、`--all`、`--tags` 迁移。历史进度文档引用的部分内部提交仅用于追溯原验收，不属于公开历史。

两台开发机开始新的公开任务时，从公开仓库的 `main` 建短期分支。新检出可执行 `git clone https://github.com/yangdonghan50/TokenDaily-Desktop.git`；旧工作目录有未完成修改时先保留在原私有分支，再将需要的文件差异单独审查、扫描后应用到公开分支。不要把两套无关 Git 历史合并。后续桌面产品只在公开主线演进，私库保留为历史档案。

本次 Windows 发布工作目录仍保留原私库 `origin`；单独命名的 `public` remote 指向新公开仓库，首次只推送已验证无父提交的指定快照到 `main`。普通新克隆的 `origin` 则自然指向公开仓库。运行下方日常命令前先核对 `git remote -v` 和当前分支的基线，不直接把旧目录的自动推送约定用于公开库。

状态：2026-10-17 制定。适用于一台 Windows 电脑和一台 Mac 电脑在同一 TokenDaily 仓库内并行开发。Intel Mac 已完成首次编译、真实来源与基础运行检查，完整原生能力与安装态仍待验收；功能边界与目录分层以 [ARCHITECTURE.md](ARCHITECTURE.md) 为准。

## 1. 分支和任务归属

2026-10-22 用户明确授权本仓库自动推送：助手完成必要检查、核对提交文件并提交后，在任务收尾普通 push 当前任务分支到既有 `origin`，首次推送设置上游。此约定替代此前由用户自行 push 的选择；主线按用户明确的合并／集成要求另行处理，双端未完成仍如实记录。遇到认证失败或远端分歧时保留本地提交并诊断，不能为完成推送改写远端历史。Windows 本机 VPN 代理保存在仓库 `.git/config`，不写入共享源码或向 Mac 复制端口设置。

- `main` 是唯一集成主线，保持可构建；不建立长期 `win` / `mac` 分支，也不复制整个应用。每项工作从最新 `main` 建短期分支：`win/<任务>`、`mac/<任务>` 或 `shared/<任务>`。
- 一项任务、一个分支、同一时间一台电脑负责写入。两台电脑可以并行开发不同任务，但不要同时改写或推送同一个任务分支。需要换电脑接手时，先在原电脑提交、推送并停止写入，再在另一台电脑拉取该分支继续。
- `main` 只合并已检查的任务分支；不在两台电脑上直接向 `main` 提交，不强制推送 `main`。使用 GitHub Pull Request（草稿也可以）记录改动与验收，再合并。合并后删除已完成的短期分支。
- 不为尚未可运行的 Mac 功能提前占用一个长期分支。Mac 的首个实机验证、原生监听、悬浮球、打包等可以各自使用独立任务分支。

## 2. 每台电脑开始工作

先执行 `git status --short --branch`。仅在工作区干净时更新本地 `main` 并从它创建分支：

```bash
git fetch origin
git switch main
git pull --ff-only origin main
git switch -c win/example-task    # Mac 任务改用 mac/example-task；共用契约改用 shared/example-task
```

若已有未提交修改，先确认归属并在原任务分支完成，或使用独立 worktree；不要用 `reset --hard`、`clean` 或随意 `stash` 清掉另一项工作。若本地 `main` 与远端已分叉，暂停自动合并，检查各自提交后再处理。继续现有任务分支时先 `git fetch origin`，有意识地合入最新 `origin/main`；已由两台电脑共用或已供他人审阅的分支不要改写历史。

## 3. 文件边界和共用改动

功能目标与两端进度统一记在 [FEATURE_PARITY.md](FEATURE_PARITY.md)。跨平台新功能或行为改动先在该文件建条目，再分别开发；每台电脑只更新同一条目中自己的平台状态和验证依据，避免维护两份独立功能清单。历史执行细节仍追加到 `docs/PROGRESS.md`。

| 改动位置 | 默认归属与要求 |
|---|---|
| `src/domain/`、`src/ui/`、`tests/`，以及 `src-tauri/src/` 的共用调度、数据和提醒模块 | 共用。两端沿用同一数据契约和规则；改动前在任务说明中列出受影响字段、调用方与测试。 |
| `src-tauri/src/platform/windows/`、`src-tauri/tauri.windows.conf.json` | Windows 专属。只暴露规范化结果或明确错误给共用层。 |
| `src-tauri/src/platform/macos/`、`src-tauri/tauri.macos.conf.json` | macOS 专属。同样遵守共用契约；未实现能力明确返回失败或在界面禁用。 |
| `src-tauri/src/lib.rs`、`Cargo.toml` / `Cargo.lock`、`package.json` / 锁文件、公共 `tauri.conf.json`、公共品牌资源 | 双端高冲突文件。改前先在 PR 或任务记录中声明，并让同一时段只有一个任务负责修改相同区域；另一端基于合并后的版本继续。 |
| `src-tauri/icons/` 等生成资源 | 以源文件和生成脚本为准，由一个任务统一重生成并提交需要跟踪的产物；不要两端分别手改同一批二进制文件。 |
| `docs/PROGRESS.md` | 各任务只追加带日期、平台和分支的记录，不重排旧记录。并行追加产生冲突时保留双方内容，再按实际合并顺序整理。 |

需要修改共享状态字段、持久化格式、通知判定、额度口径或 Rust ↔ TypeScript 命令时，先在任务分支说明新旧契约和兼容方式，再修改共用测试与两端调用。单个平台暂不能支持时显示未知或禁用；不能用演示数据或空实现掩盖。公共版本号与依赖锁文件只保留一份，不为 Windows/Mac 各维护一份。

## 4. 验证和合并

- 纯平台改动：在目标系统运行该系统的编译、测试和相关安装态检查；同时运行受影响的共用测试。Windows 命令和本地工具链准备见 `ARCHITECTURE.md`。Mac 首次准备运行 `npm ci`、`npm test`、`npm run build`、`cargo test --locked --manifest-path src-tauri/Cargo.toml`，原生或打包改动再运行 `npm run tauri -- build`，并记录实机结果。
- 共用逻辑或契约改动：先补充/更新共用测试，再在 Windows 和 Mac 各运行受影响的测试与构建。Mac 尚未具备可用构建环境时，已有功能的必要维护可以继续 Windows 开发，但 PR、`docs/PROGRESS.md` 和功能清单必须标明“Mac 未验证”；新增跨平台行为遵守下条的双端完成条件。Mac 进入并行开发后，共用改动须两端验证后再合入 `main`。
- 新增跨平台功能或用户可见行为变化，以 `FEATURE_PARITY.md` 的同一条目安排两端实现。分别开发后，在 `shared/<任务>` 集成分支合并 Windows 与 Mac 的改动；两台电脑可以检出同一集成提交测试，但仍只有一台负责写入。两端对同一集成代码完成验收后，才标记“同步完成”并通过一个 PR 合入 `main`。当前 Mac 迁移缺口和必要的单端先行例外按清单记录，不把一端完成写成跨平台完成。
- 合并前更新任务分支到最新 `origin/main`，检查 `git status`、提交差异、凭据/原始日志与生成文件清单。冲突逐块核对双方意图；共享源码、锁文件或图标冲突不得整文件选择 `ours` / `theirs`。解决后重跑受影响的测试。
- PR 至少写清任务与平台、共享文件/契约变化、执行过的命令及结果、未验证的系统或安装态、真实/演示数据边界。Windows 通过不代表 Mac 通过，反之亦然。发布、远端推送、签名和公证由执行者按授权处理；自动化助手仍遵守项目 `AGENTS.md` 的单独授权要求。

## 5. 跨电脑交接

每次结束工作，把代码与 `docs/PROGRESS.md` 的记录一起提交到任务分支。需要另一台电脑接手时，由人确认推送任务分支，并留下分支名、最新提交 SHA、共享文件清单、已跑测试、未完成事项。接手电脑先抓取远端，核对 SHA 与干净工作区后继续。个人凭据、Codex 原始日志、本地数据库、构建目录和工具链不通过 Git 交接。

## 6. 两端共用的 Git 忽略与提交检查

- 根目录 `.gitignore` 同时覆盖 Windows/macOS 的依赖、构建产物、系统杂项、编辑器缓存，以及可能误放进工程的凭据、签名文件和本地运行数据。两台电脑使用同一份规则；某台机器独有的目录或个人偏好写入其 `.git/info/exclude` 或全局 Git 忽略文件，避免每台机器反复修改共享规则。忽略规则不能保护已经被 Git 跟踪的文件。
- `package-lock.json`、`src-tauri/Cargo.lock`、两端平台源码与 `tauri.*.conf.json`、所需图标和生成脚本须保持跟踪；构建目录 `node_modules/`、`dist/`、`src-tauri/target/`、`src-tauri/gen/` 和本地工具链不交接。Mac 构建时使用同一锁文件，不因平台差异另建一套锁文件。
- 每次提交前运行 `git status --short --untracked-files=all` 和 `git diff --cached --name-status`，逐项核对暂存文件；不确定某文件为何被忽略时运行 `git check-ignore -v -- <路径>`。不要用 `git add -f` 绕过凭据或本地数据规则。若发现敏感文件已被跟踪，先停止提交并检查 Git 历史与清理范围；单改 `.gitignore` 不会从已存在的提交移除它。

当前仅制定流程，没有设置 GitHub 分支保护或自动 CI。Intel Mac 首次实机构建已通过；后续根据实际耗时与仓库权限决定是否加自动检查，在此之前以 PR 记录和双机实测作为合并依据。
