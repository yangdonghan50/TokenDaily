# TokenDaily 跨平台工程架构

2026-10-20 本地 `main` 0.1.24 集成中：共享设置主窗口及积分重试逻辑，Mac 原生 WorkBuddy/TRAE 授权继续使用钥匙串和独立 WebKit 数据存储；新增的最后成功积分仅是规范化余额，Mac 限定本机用户可读。以下旧记录按当时版本理解，当前验收以 FEATURE_PARITY/PROGRESS 顶部为准。

2026-10-19 Mac 0.1.23 整合：共享应用更新状态机、悬浮入口和 IPC 合入同一工程，Windows 的插件、签名校验和安装留在 `platform/windows/app_update.rs`；Mac `platform/macos/app_update.rs` 返回明确的 unsupported，不启动更新检查或安装。Mac TRAE 本地连接及独立官方积分已安装态验证；数据库目前 0 条聊天轮次，真实 Token 待验。Mac Codex 自定义目录、独立钥匙串参榜与 WorkBuddy 积分、通知、原生快捷键和透明悬浮窗继续保留。更新边界见 UPDATES.md，安装态状态见 FEATURE_PARITY.md。

最新整合（2026-10-18，CHG-20261018-MAC-SYNC）：0.1.16 共享组件与 Mac 原生模块共存。初始悬浮位置移到 `platform/floating_placement.rs`，仅调用跨平台 Tauri 工作区 API，不含 Win32/AppKit 类型；原生拖动仍走既有平台门面。Mac 窗口数组同步 center=false，保留 acceptFirstMouse。菜单“展开悬浮窗”在 Mac 开放，全局快捷键与参榜在该基线尚不可用，后由 CHG-20261018-MAC-PARITY 补齐。历史扫描沿用 `connection::Settings::home()`，尊重 Mac 自定义 Codex 目录；找不到指定目录时不回退到其他来源。安装实测状态见 PROGRESS/TEST_REPORT。

状态：2026-10-17，Windows 保持当前功能；Intel Mac 已完成首次编译、真实只读来源与基础原生运行验证。Mac 已补齐常见路径查找和手动来源设置，并验证 Finder 打开及重启后真实读取；原生目录事件已实现并通过 Intel Mac 临时目录实测，悬浮球已完成 Mac 原生适配，本机验收见 CHG-20261017-07；详情见 PROGRESS.md。

## 仓库决策

一个 Git 仓库、一个 Tauri 2 + React/TypeScript + Rust 应用、一个主分支。按功能使用短期分支并合回主分支；不建立长期 `-win`、`-mac` 代码副本或平台分支。Windows 和 macOS 分别构建安装包，分别做系统验收。首个本地基线提交 `aa0daa9` 保存了拆分前的 Windows 原型。双机分支、交接和合并流程见 [DEVELOPMENT_WORKFLOW.md](DEVELOPMENT_WORKFLOW.md)。

## 依赖方向

```text
src/ui + src/domain                  共同界面与纯规则
           ↓ invoke / 规范化状态
src-tauri/src/lib.rs                  应用状态、调度、命令与事件
           ├── connection.rs / codex.rs / local_usage.rs / alerts.rs / usage.rs
           │   共同协议、计数、时间、状态与提醒规则
           └── platform/{windows,macos}
               可执行文件与数据路径、日志唤醒、窗口系统行为
```

`src-tauri/src/platform/log_watch.rs` 给调度层固定 `Wake / Watch / Signal`、补查和合并时序接口。Windows 实现在 `platform/windows/`，继续使用原有 Win32 目录通知与窗口拖动代码。macOS 路径解析和 FSEvents 监听在 `platform/macos/`；使用 Mac 目标依赖 notify 8.2，非递归监听来源根目录，递归监听实际存在的 sessions/archived_sessions。回调只保留合并唤醒位，不读取内容或保存路径；无关事件被过滤。日志根目录不跟随符号链接，目录 inode 变化时重绑，断连/切来源以代次隔离迟到事件并释放原生线程。30 秒目录补查、2 秒活跃文件检查与 500 ms 事件合并仍由共用调度负责；相同目录的周期 reset 复用原生流。可观测监听失败通过已有 UsageState.message 提示定时补查，不覆盖采集错误、不把有效数据改成未知/零；Windows 原有监听及错误报告保持不变。macOS 悬浮球通过本地 NSEvent 监听处理主动拖动，前端继续共用展开/收起与贴边规则；支持状态与系统实测分别维护，不把实现等同于完整多屏/安装态验收。

| 保持共用 | 必须按平台适配 |
|---|---|
| Codex App Server 的只读协议与额度归一化 | Codex CLI 的发现、Finder/终端环境差异 |
| 日志事件解析、去重、北京时间自然日、索引格式 | `CODEX_HOME` 默认路径、目录变化监听 |
| 新鲜度、身份隔离、提醒判定与持久化去重 | 原生悬浮球拖动、屏幕工作区、菜单栏/托盘细节 |
| React 页面与数据状态契约 | 系统通知授权、登录启动、图标与安装包 |

共享 Rust 模块不得直接导入 Win32/AppKit 类型；平台模块只向共享层暴露路径、唤醒、窗口动作或明确失败。若未来出现真正不同的界面需求，可在同仓库增加平台 UI 入口，仍先复用数据契约和规则。当前不预建第二套应用、插件系统或多 crate 工作区。

## 连接来源与缓存

`connection.rs` 管理共享来源配置和子进程输入，平台模块处理 CLI 查找和子进程 PATH。Mac 开放 `get_connection` / `set_connection` 对应的设置 UI；Windows 返回 `supported: false`，沿用已有自动发现入口，新增共享调度需在 Windows 回归。

- Mac 程序优先级：保存的显式路径、`TOKENDAILY_CODEX_EXE`、PATH、常见安装目录。显式路径无效不会回退；只接受绝对路径的可执行文件，不运行用户 shell 初始化脚本，不自动遍历私有 App bundle。
- Mac 数据目录优先级：保存的显式目录、进程 `CODEX_HOME`、用户主目录 `.codex`。要求目录存在；额度子进程显式传入同一有效目录，日志采集随来源重新绑定。Windows 也经过目录校验并显式传给自有子进程；这项共享行为仍需 Windows 实测。
- 来源修改仅在断连且没有未完成额度请求时允许；持久化成功后才更新内存并使采集轮次失效。断连/清除也使旧结果失效，避免迟到结果重新发布。
- 本地设置新增默认空的 `connection`。额度缓存改为 `{sourceKey,state}`，指纹来自程序路径与有效数据目录；来源不匹配或旧无指纹缓存不恢复。有效且有工作区核验信息的缓存仍标记为旧数据，等新身份读取；缺少核验信息的旧缓存放弃恢复。本机索引继续使用已有目录校验。
- 配置路径只在本工具设置保存，不修改 Codex 配置或系统环境。Mac 仅为自有 CLI 子进程补充标准 PATH 和所选程序目录，支持同目录的 npm/Node 包装入口；不同安装方式仍须各自实测。

## 配置与 Git 约定

2026-10-18 共享用量契约新增 `UsageState.sourceUnavailable`（Rust 默认 false，旧快照兼容；TS 可选）：只表示采集层已确认尚无可统计日志来源，不代表用量为零或额度未登录。关闭的来源及明确未接入日志来源不参与今日汇总/费用范围；未知初始化、损坏或权限错误不能冒充未接入。Claude Code / WorkBuddy 采集器在进程内记忆已观察到有效用量，后续日志丢失或跨日失败仍属于统计异常。Windows 已测试，Mac 未编译/实机验证。

- `src-tauri/tauri.conf.json` 存公共产品名、版本、窗口、前端路径和权限；`tauri.windows.conf.json` 存 NSIS/ICO；`tauri.macos.conf.json` 存 App/DMG/ICNS。Tauri 会自动合并对应平台配置。配置数组是**整体替换**，覆盖 `app.windows` 时必须重复保留的窗口定义。[Tauri 平台配置](https://v2.tauri.app/develop/configuration-files/)
- `Cargo.toml` 的 `windows-sys` 仅是 Windows 目标依赖。Mac 新增原生依赖时也使用目标依赖和 `cfg(target_os = "macos")`，不要把系统库引入共用模块。
- `.gitattributes` 固定源码、配置与文档为 LF，减少跨系统换行冲突；`.gitignore` 排除依赖、构建输出、本地工具链和临时文件。提交前查看 `git status` 与 `git diff --cached`，确认没有凭据、原始日志或私有账号响应。
- Windows 与 macOS 使用同一产品版本和数据契约；不同平台的安装、通知授权与窗口行为分别记录实测结果。不得把一端测试结果写成另一端通过。

## 修改规则与验收门槛

1. 修改百分比、时间、去重、身份或通知规则，先更新共用测试，再改实现；两端都运行共用测试。修改规范化状态字段时同时检查 Rust 序列化、TypeScript 类型、两端 UI 和旧缓存兼容。
2. 修改平台模块时运行对应系统的编译与测试。Windows 开发机运行 `npm.cmd test`、`npm.cmd run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`；改 Tauri 配置或原生窗口后再运行 `npm.cmd run tauri -- build`。本机先加载 `scripts/use-local-dev-env.ps1`。
3. Mac 开发机使用相同仓库和锁文件，先运行 `npm ci`、`npm test`、`npm run build`、`cargo test --locked --manifest-path src-tauri/Cargo.toml`、`npm run tauri -- build`；具体工具链、签名条件和命令以 Mac 实机结果修订。Windows 的编译不能证明 Mac 构建成功。
4. Mac 的第一道验收是 Finder 启动后只读发现已安装 Codex CLI、识别正确身份和额度、核对有效 `CODEX_HOME` 与日志日期。缺失来源要显示未知；不发送模型推理、不复制登录凭据、不修改用户 Codex 配置。
5. Mac 原生目录通知和悬浮球需要独立实现与实机验收。悬浮球须覆盖按住触边半隐藏、回拖、松手、重新悬停、Retina/多屏、Dock 与 Spaces；通知须覆盖授权和安装态送达。完成这些检查前，不能把 macOS 标为可交付。

本文件是开发规则，不代表 Mac 已完成安装态验收或发布。首次本地构建及实际验证见 `docs/PROGRESS.md`；真实数据接入边界见 `docs/FEASIBILITY.md`。


## 身份与提醒提交边界（CHG-20261017-04）

- `DesktopState.identityVerified` 默认为 false，指邮箱和有效 workspaceRouting 字段在本次只读读取中可核对，不表示已与官方 UI 人工比对。身份 v2 哈希纳入来源、邮箱、chatgptAccountId、backendOrigin；本机日志不按该键冒充账号统计。
- 协议读取在额度后和账号日统计后增加 `account/read(refreshToken:false)` 复核。检测到身份变化清空本轮账户结果；身份未核验时不继承旧账户历史、不恢复旧缓存、不触发通知。网络失败仍可保留已核验身份的旧快照并标旧。
- `alerts.rs` 是唯一运行中的提醒规则，单测直接覆盖它；移除仅被测试调用的 TypeScript 重复算法。账本保存各已见身份的窗口状态，切回同一身份不会重放同一风险。
- 通知分为观测、持久预留、提交、确认记录。可观测提交失败保留低额度重试与恢复证据，至少间隔 60 秒且必须有新数据；外部提交与本地文件无法成为原子事务，不确定结果按已预留保守去重。
- 账本双槽带 SHA-256/序号，写入后 sync_all；读写异常可见，两个槽均损坏时阻止发送。旧 JSON 只在没有新槽时读取；旧身份不会被猜测绑定到新工作区。
- `get_notification_status` / `notification-status` 给设置页返回 idle、submitted、retry、storage-error、uncertain、disabled、identity-unverified。Windows 通知插件仍异步派发；submitted 不能报告 delivered。插件 permission_state 固定 Granted，产品不使用它推断系统授权。Mac 后续实现见下节。


## Mac 原生通知（CHG-20261017-06）

- `notification.rs` 保存共用权限/失败契约、最新快照校验和有界工作队列。`platform/notification.rs` 只导出接口，两端实现在 `platform/macos/notification.rs` 与 `platform/windows/notification.rs`；Windows 保持插件调用语义，Mac 使用 UserNotifications。
- Mac 查询 authorizationStatus、alertSetting、notificationCenterSetting；未知值不映射为已授权。授权只响应设置页明确点击，仅申请 Alert；不会用 provisional 自动授权，也不启用声音、徽标或时间敏感打扰。未打包的开发/测试二进制返回不可用，不调用需要 bundle 的通知中心。
- Mac 等待 addNotificationRequest 完成回调（最多 5 秒）；成功仅为 accepted，明确拒绝为 Rejected，缺失回调为 Uncertain。共享账本遇到 Uncertain 保留持久预留并在内存消费同一提醒，避免迟到成功后重复。读取权限失败发生于提交前，可作为未发送处理。
- 额度通知在独立单工作线程处理，合并连续刷新为最新来源代次；持久化和提交前复核连接、身份及快照版本。系统权限查询后、真正提交前再次检查；已交给系统的请求无法通过断连保证撤回。清除数据先失效来源，再持有同一账本锁删除，避免后台重新写回已清理记录。
- get_notification_permission / request_notification_permission 返回 supported、authorization、alerts、center；send_test_notification 返回 accepted（Mac）或 queued（Windows）；check_test_notification 返回 none、in-center、not-observed、clicked、unsupported。测试标识只在本次运行内存中保留，不混入真实额度去重账本。
- 原生 delegate 在主线程初始化并保留（系统 delegate 为弱引用），点击仅处理系统默认激活动作，派回主线程打开面板；关闭通知不当作点击。前台展示请求 Banner/List，仍服从系统设置。Mac 配置显式最低 11.0，与使用的现代展示接口一致；最低版本声明不等于所有 macOS 11+ 都已验收。

Mac 通知身份前置检查：使用 Security 框架验证当前运行进程签名；失败时拒绝查询/授权/提交，避免将通知服务拒绝身份后返回的默认设置视为真实权限。该校验不代替用户授权、通知显示验收或发行公证。


## Mac 悬浮窗（CHG-20261017-07）

- `platform/floating.rs` 是窗口安装、开始拖动、当前主按键与坐标归一化的门面；Win32 类型已从共享 lib.rs 的按键检查中移出。`start_floating_drag` 仅接受 floating 窗口，Windows 委托既有系统拖动，Mac 使用已安装的本应用 NSEvent 本地监听。不安装全局输入钩子、不申请录屏/辅助功能；产品不记录输入或指针轨迹。
- Mac 原生监听只在前端明确开始拖动后修改本窗口，按原始抓取点计算位移；每次移动查询工作区，立即预览触边，退回清除；鼠标抬起发布规范化结束事件，Escape 取消并还原位置。WebKit 继续收到鼠标序列，避免连续拖动留下按键状态。闲置时不移动窗口。
- 底层 Tao 的 Mac cursor_position 使用主屏比例，而 monitor_from_point 使用逻辑坐标。Mac 原生拖动统一用逻辑桌面坐标；前端“鼠标是否在窗口内”通过共用命令读取按当前窗口比例归一化后的结果，避免混合单位。负坐标/多比例由单测覆盖，多屏混合 DPI 仍须实机。
- `macos-transparent` 是显式 Cargo feature，映射 Tauri 的 macos-private-api 透明窗口能力；仅 macOS 配置的 build.features 启用，Windows 配置不启用。避免 CLI 按根依赖修改目标依赖特性；锁定依赖版本不变。底层透明 WebView 使用私有 KVC，这不构成 Mac App Store 发行方案。
- Mac 配置覆盖 app.windows 数组以设置 floating.acceptFirstMouse=true，必须同步保留主窗口和其他公共窗口字段。菜单栏使用已有品牌图标的 alpha 模板，由系统着色；颜色外观及多屏/Spaces 不能仅依据代码配置标通过。
本文件是开发规则，不代表 Mac 版本已构建、安装或发布。当前实际验证见 `docs/PROGRESS.md`；真实数据接入边界见 `docs/FEASIBILITY.md`。

## Mac Claude Code 凭据（CHG-20261020-MAC-CLAUDE）

本机 Token 扫描和订阅额度解析继续共用。Mac 平台仅从当前登录用户的 `Claude Code-credentials` 钥匙串条目读取凭据；确认为不存在才回退 Claude Code 凭据文件。拒绝访问不触发文件回退，避免使用旧令牌冒充当前账户。平台返回原始凭据只给内存中的共用解析器，前端只接收规范化额度或错误。本机没有 Code 凭据和有效日志时显示未知；桌面聊天不在此来源内。钥匙串弹窗的频率取决于 Claude Code 写入方式与系统访问控制，不能保证永久免确认。

## Mac Claude Desktop 本机额度历史（CHG-20261020-MAC-CLAUDE-DESKTOP）

`platform/macos/claude_desktop.rs` 只读当前用户的 `~/Library/Application Support/Claude/plan-usage-history.json`。本机 Claude Desktop 2.42304.2 的应用代码将 `five_hour`/`seven_day` 利用率采样写为 `fh`/`sd`；该文件是私有格式，未来版本可能变化。读取只接收受限大小的版本 2 文件、最新样本的 0–100 数值，超过 10 分钟标旧、超过 24 小时拒绝；不回溯到较早账户样本。快照携带样本时间和组织 ID 摘要，不读取浏览器存储、Cookie、OAuth 缓存或聊天正文。文件不能证明当前仍登录同一账户，因此 `identity_verified=false`，共享通知规则拒绝发送额度提醒。它不含重置时间、精确聊天 Token 或账户套餐信息；Code 日志 Token 扫描仍为独立来源。Mac 仅在 Code 额度获取失败时使用本机快照，Windows 行为不变。

## 2026-10-17 可选排行榜边界

`leaderboard.rs` 负责共享统计导出/同意状态/同步调度/服务契约，复用 local_usage 的规范化事件，不修改 Codex/Claude/WorkBuddy 凭据。只向固定 TokenDaily HTTPS 发送最小计数；Debug 仅允许指定 loopback。参榜状态经 platform/leaderboard_credentials 门面保存：Windows 延续原 DPAPI 文件格式；macOS 使用独立、不同步的 app.tokendaily.local.leaderboard 钥匙串条目，按数据路径与 Debug/Release 隔离。构造不读取凭据，后台一次恢复；仅不存在视为新用户，拒绝/损坏会阻止新身份覆盖，并允许显式重试。参榜预览和同步沿用当前 connection.home()，确认时再次检查来源与预览一致。React 不接触 session token，平台类型没有进入共享契约。

Web API/D1/auth 位于相邻 TokenDaily-Web 仓库；接口及安装顺序记录于该仓库 `docs/LEADERBOARD-OPERATIONS.md`。两仓当前任务分支同名但不表示跨仓自动发布；上线需要独立验收和授权。

## 0.1.3 主线与 Mac 整合（CHG-20261017-MAC-SYNC）

- 短期 `mac/main-0.1.3-sync` 合入 main `1b06e57` 与 Mac `ee3e1c9`，保留原生浮窗、目录监听、通知与来源设置。WorkBuddy 积分授权、参榜的 Mac 安全存储未实现，UI 与命令继续明确拒绝。
- Claude 提醒复用 `alerts::Delivery` 双槽持久化和平台通知结果，不再绕过原生回调直接调用插件。成功读取身份及额度并校验凭据一致后标记 identity_verified；提交前复核来源代次、身份、快照和连接状态。通知等待不持有 provider 状态锁，清理先失效来源、再删除账本。独立 `get_claude_notification_status` / `claude-notification-status` 向共用设置页报告结果。
- 固定三来源总览沿用主线动态浮窗高度与排序，Mac 仍通过原生拖动入口移动窗口；旧 floatingMode 只保留后端兼容，前端不再提供模式选择。共用和 Mac 窗口配置除 acceptFirstMouse 外保持一致。锁文件合并仅采用两侧已有版本。

## Mac WorkBuddy 积分存储（CHG-20261017-MAC-WB）

- `platform/workbuddy_credentials.rs` 为 load/save/remove 门面。Windows 保留 DPAPI 文件格式；Mac 使用 `platform/macos/workbuddy_credentials.rs`，通过已锁定的 security-framework 3.7.0 调用钥匙串 API，指定不云同步。条目按数据目录哈希和构建类型隔离，只有缺失状态映射 None，权限失败返回明确错误。合成实机测试使用独立临时路径派生条目并清理。
- WorkBuddy 共享授权、积分解析、刷新与代际隔离继续共用；启动恢复不再吞掉存储错误。Mac 官方 URL 通过指定域名 HTTPS 校验后使用 `/usr/bin/open` 的独立参数打开，不经 shell、不记录授权 URL 或令牌。积分使用独立官方登录，不读取 WorkBuddy 客户端凭据。
- 通用 protect_secret/unprotect_secret 的 Mac 实现仍拒绝调用，排行榜继续禁用；此修复仅启用 WorkBuddy 积分。用户数据、钥匙串条目和签名产物均不进入 Git。

## Mac 快捷键与指标适配（0.1.17，CHG-20261018-MAC-PARITY）

- 原生快捷键放 platform/macos/hotkey.rs，以 Carbon RegisterEventHotKey 独占注册，不安装全键盘监听、不要求辅助功能权限。公共 Shortcut 保留原键值契约，平台转换物理按键、Command/Option 修饰键。新绑定成功后才释放旧绑定；失败保留原绑定，按下/抬起状态阻止长按连发，退出释放。
- Carbon 调用和 Service 锁在 Mac 主线程统一执行，避免主线程读取状态与后台等待原生注册形成死锁。录入租期、取消/超时、关闭释放和持久化继续复用共享规则；Windows 调用仍由其专用驱动完成。
- MetricNumber 按实际字体测量与可用宽度缩小数字，ResizeObserver 与字体就绪重新测量；单位单独占位，保留精确提示。只改变展示，不改 Token/费用/排名值。

## 2026-10-19 TRAE CN 来源边界

- 新增 trae_usage/trae/trae_credits：UsageState/CreditState 沿用现有字段，TRAE 计数进入已连接来源的本地合计与历史，积分独立；不改变账号百分比额度契约，不修改参榜导出范围。TRAE 本地数据库可能含恢复/同步会话，范围不能等同于严格设备独占；React 详情与悬停均说明。
- 路径由 platform 返回；当前 Windows 已实测，Mac 连接与官方授权明确禁用。数据库只读、认证页与提交 WAL，在内存 SQLite 中查询必要计数，原始上下文不出查询层；损坏/锁定/未知格式不补零，大小受上限保护。
- TRAE 官方窗口使用 TokenDaily 自有网页数据目录；远程页面不在任何 IPC capability 中，不授予原生命令。初始化脚本只观察固定官方余额端点的白名单数字字段；无凭据头、Cookie、浏览器存储或会话正文读取。回调只由自有窗口导航拦截，随机票据/来源/期限校验；系统网页引擎维护独立登录。Mac 不调用这条路径。
- 四个 AI 卡片与 calendar 分开限额：排序持久化允许四来源+一个日历，保留旧偏好；原生高度与前端同步支持第五行，不增加滚动条。依赖新增 Rust AES/CBC/HMAC/PBKDF2、内存 SQLite 和缓冲清零组件，锁文件统一跟踪。第三方格式参考许可随安装包提供。
