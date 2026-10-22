# M0 可行性记录

## 2026-10-21 后续：Claude 重置时间与 statusLine 桥接可行性

- 文档线索（未在本机实测数值）：Claude Code 官方状态栏文档列出 `rate_limits.five_hour/seven_day.used_percentage` 与 `resets_at`（Unix 秒），仅 Pro/Max、会话首个响应之后出现，过了重置时间会移除。生财有术 TokenRank 的 `opentoken` 客户端即通过 statusLine 采集“官方额度百分比”；其规则页称仅在用户未配置状态栏时写入。
- 本机实测：Claude Desktop 的 Code 标签页以 Claude Code 2.1.304 无界面模式运行（`--output-format stream-json --input-format stream-json`），加载 user/project/local 设置，附加设置中没有 statusLine。90 秒内每 0.2 秒检查一次（共 259 次），期间本会话持续产生消息，未见 `opentoken statusline` 子进程；opentoken 已配置状态栏，但从未生成 `statusline_buffer.jsonl`。结论：Code 标签页**很可能不调用** statusLine；采样可能漏掉极短进程，未做确定性探针测试（临时改 settings.json 的探针被自动权限检查拦截，文件哈希核对未改动）。
- 影响：仅使用 Claude Desktop 时，statusLine 桥接拿不到重置时间。
- 终端实测（同日，用户自行切换探针并发一条消息）：Claude Code 2.1.305 交互模式调用 statusLine 两次，首次无 `rate_limits`（尚无响应），第二次含 `five_hour` 与 `seven_day` 的 `used_percentage` 与 `resets_at`（Unix 秒），与当时 Desktop 历史的百分比接近；opentoken 同时首次生成 `statusline_buffer.jsonl`，佐证 Desktop Code 标签页此前从未调用。`~/.claude.json` 的 `oauthAccount.organizationUuid` 与 Desktop 历史 `org` 的哈希一致，可作为合并依据；该文件为 Apple 订阅账户。据此实现可选桥接（CHG-20261021-CLAUDE-STATUSLINE）。

## 2026-10-20 后续：Claude Desktop 订阅用量本机历史

已测能力：这台 Mac 的 Claude Desktop 2.42304.2 在应用支持目录写入版本 2 的 `plan-usage-history.json`，真实样本含 `fh` 和 `sd` 数值；本机 App 资源中的写入映射分别对应 `five_hour` 与 `seven_day` 利用率。只读解析测试和当前用户真实文件测试通过。该文件不含重置时间或聊天精确 Token，也不能核验当前登录账户，故仅作标时历史快照、关闭额度提醒，不替代官方实时账户接口。

限制与后续：此路径/格式是 Claude Desktop 私有实现，不是 Anthropic 承诺的稳定集成接口；未来版本可能变化。Windows 版本、本机账户切换、登出后文件行为和 TokenDaily 安装态 UI 尚未验收。用户无需安装 Claude Code 才能读取这个已有的本机额度采样。

## 2026-10-17 后续：Mac 悬浮窗接入

依据本机锁定的 Tauri/Tao/Wry 源码和 AppKit 接口实现。原生拖动采用本应用的 NSEvent local monitor；不要求产品取得辅助功能、录屏或全局输入权限。测试辅助程序使用这台开发机已有的 AX 权限执行人工交互等价操作，不属于产品运行依赖。

初验发现 Tao 的 cursor_position（主屏物理像素）和 monitor_from_point（逻辑桌面坐标）单位不同；Mac 拖动先转换逻辑坐标，前端命中检查统一由平台门面按窗口比例返回。单测覆盖 1/1.5/2 倍尺寸、Retina 指针、负坐标、工作区边界、近边不吸附和回拖。原生操作通过与未通过的最终结果见 TEST_REPORT 的 CHG-20261017-07。

透明窗通过 Mac 配置启用自定义 Cargo feature，再映射 Tauri macos-private-api；CLI 会移除与配置不一致的目标依赖特性，所以不把该标志直接写进 Mac 目标的重复 tauri 依赖。此功能仅用于当前本地桌面路线，不作为 App Store 可发行依据。依赖版本和锁文件未更换。

## 2026-10-17 后续：Mac 原生通知接口

实现依据：读取本机 macOS SDK 的 UserNotifications 头文件和锁定 objc2 绑定，使用系统权限查询、显式授权、提交完成回调及 delegate。Apple 的[授权说明](https://developer.apple.com/documentation/usernotifications/asking-permission-to-use-notifications)要求按实际设置决定发送；产品不在后台自动请求权限。SDK 标明当前使用的 Banner/List 展示选项需要 macOS 11，因此构建最低版本由 Tauri 默认的 10.13 明确为 11.0。

已测：构建目录 App 在 Intel macOS 15.7.7 正常启动。用户明确同意授权后点击申请，但系统拒绝请求且没有弹窗。只读 codesign 检查确认 App 完全未签名；本机通知服务记录 getNotificationSettings / requestAuthorization not allowed。此前查询返回的 not-determined 是服务拒绝后的默认值，撤回“权限读取已验证”的判断。新增 Security 框架运行身份校验，签名失败时明确不可用，避免采信该默认值。单测覆盖签名失败、未知/拒绝归一化、拒绝与回执超时、重启去重及过时快照。

后续已获本地 ad-hoc 签名单独授权，对构建目录 App 签名并通过 codesign 严格校验。签名后系统返回 denied；按用户授权在系统设置中仅打开 TokenDaily 通知，再查询为允许。单条测试请求获得 accepted，原生查询和系统 AX 均找到测试通知；点击实际条目后应用记录 clicked 且面板处于前台，重启后仍为允许。通知中心记录不能证明横幅出现或用户阅读。后续实测原生关闭按钮隐藏主面板（屏幕主窗口数从 1 到 0、原进程保留），点击通知恢复同一进程主面板并产生 clicked；完全退出后点击另一条测试通知启动新进程并展示主面板。首装授权弹窗、安装态及发行签名仍待验；已有 DMG 仍含未签名 App。

## 2026-10-17 后续：Mac 原生日志事件

已测：notify 8.2 的 FSEvents 后端在 Intel macOS 15.7.7 的本地临时目录能通知创建、追加、归档移动、替换和目录重建；实际唤醒共用 Collector 后计数正确，归档不重复。停止/重绑定、回调释放、突发合并、代次隔离与错误恢复有测试。事件测试使用合成日志，不是向真实 Codex 日志写入测试内容。

依赖线索与限制：[notify 官方文档](https://docs.rs/notify/8.2.0/notify/) 说明网络盘、文件归属及大量事件可能导致事件不可用或丢失；不能由本机测试推断这些场景可靠。已读取锁定后端源码：部分底层启动失败未上报、路径处理假定 UTF-8；产品拒绝非 UTF-8 来源根路径并保留定时补查，但不能声称可观察全部系统失败或支持任意非法编码子文件名。未验证网络盘、睡眠恢复、长期驻留或 Apple Silicon。

## 2026-10-17 后续：工作区字段与通知边界

本机 CLI 0.163.0-alpha.10.2 的 `generate-json-schema --experimental` 显示工作区位于 GetAccountResponse 顶层 workspaceRouting，其 chatgptAccountId 与 backendOrigin 为字符串。更新脱敏探测后，真实只读响应确认这两个字段均存在；此前只检查 account 内部字段，不能据此断言来源不提供工作区。新实现只保存哈希和核验状态，没有读取或复制凭据。不同工作区切换由合成协议序列测试覆盖，未切换用户实际工作区或与官方 UI 做人工对照。

锁文件中的 tauri-plugin-notification 2.7.0 desktop.rs 显示 show() 在后台派发前返回、permission_state() 固定 Granted，后台错误不暴露给当前调用者。新状态机可处理同步提交错误、存储错误及崩溃去重，但不能保证系统送达或发现所有后台失败；真实权限/免打扰/点击与送达仍须分别在安装态验证。此为本机依赖源码证据，不是未执行的文档推测。

## 2026-10-17 后续：Mac 来源选择实测

`mac/codex-discovery` 已实现标准路径候选查找及手动绝对路径设置。通过 Finder 的 `open POSIX file` Apple Event 打开构建目录 App（未注入 TOKENDAILY_CODEX_EXE/CODEX_HOME），手动保存本机现有 CLI 后，真实额度、账号日桶、本机日志、手动刷新和退出重开通过。不是人工鼠标双击，也不是 /Applications 安装验收。

本机没有独立标准位置 CLI，自动发现覆盖来自文件系统夹具测试；实际运行使用用户本机已存在的 CLI，路径只进入 TokenDaily 本地设置，源码不硬编码 App 内部路径。空测试数据目录返回需授权/日志未知；含空格的 CLI 符号链接可执行，删除该测试链接后重启明确报错且不恢复旧额度；设置只读时保存失败且原内容保留。详细命令与限制见 TEST_REPORT、PROGRESS。

下方“首次实测”的 CLI 缺口是本条之前的历史状态。当时仍未完成 Windows 回归、完整安装态、签名公证、Apple Silicon、菜单栏视觉、通知、登录启动、原生目录事件与悬浮球；目录事件后续结果见上方记录。

## 2026-10-17 Intel Mac 首次实测

分支 `mac/initial-validation`，基线 `9f0fc4c`。以下为本轮实测；下方 2026-10-16 的方案评估保留历史语境。

| 项目 | 本轮结果 |
|---|---|
| 系统/开发环境 | Intel x86_64、macOS 15.7.7（24G720）、Xcode 26.3、Node 24.19.0、npm 10.9.8 |
| 前端 | 锁文件安装依赖、44 项规则测试、TypeScript 检查、Vite 生产构建通过 |
| 终端只读接口 | 现有 CLI 0.163.0-alpha.10.2；stdio 初始化以及 account/read（refreshToken:false）、account/rateLimits/read、account/usage/read 均成功 |
| 返回结构 | ChatGPT 身份含邮箱字段；1 个额度池/1 个 primary 窗口，归一化百分比与重置字段有效；69 个有效且日期唯一的账号日桶；未打印真实值或身份 |
| 身份/日桶边界 | 探测未发现 account.id 或 chatgptAccountId；未提供日桶时区/覆盖范围。不据此声称完成工作区隔离或账号全设备统计核验 |
| CLI 发现边界 | 当前终端 PATH 指向现有 ChatGPT 应用内部 CLI；未安装独立 CLI、未硬编码私有路径。受控最小 PATH 的 LaunchServices 启动已确认找不到 CLI；只为下一次启动传入已发现路径即可真实连接。Finder 双击完整闭环待补 |
| 本机日志 | 未显式设置 CODEX_HOME，默认 .codex 的 sessions/archived_sessions 均存在；Rust live_local_logs 专项通过首次采集、增量和索引恢复 |
| Rust | 工程内隔离 Rust/Cargo 1.99.0；格式检查通过，27 项常规测试通过、2 项真实来源默认忽略；显式 live_ 专项 2/2 通过（真实额度/账号日桶及本机日志） |
| 桌面构建 | 未签名 Intel App（12.21 MiB）/DMG（3.82 MiB）构建成功，DMG 校验通过，锁文件未改；未签名公证或正式安装 |
| 原生运行 | 构建目录 App 启动、真实数据展示、刷新、关闭驻留、单实例恢复窗口、断开与正常退出通过 AX/缓存状态核对；没有原生截图验收 |
| 浏览器展示 | 420px 六种演示状态无横向溢出，360px 非演示未连接/设置页无溢出；开发演示横幅在 360px 宽时溢出 5px。此为浏览器前端检查，不是 WKWebView 验收 |

Rust 使用[官方手动安装方式](https://rust-lang.github.io/rustup/installation/other.html)，下载 rustup-init 二进制并核对官方 SHA-256，指定工程内 CARGO_HOME/RUSTUP_HOME、minimal profile 和 `--no-modify-path`。不修改系统工具链或 shell 配置。`scripts/mac-dev.sh` 只为后续命令选择工具链，不执行安装。

远端读取返回 GitHub HTTP 403；本地 HEAD 与缓存的 origin/main 相同，但未能确认远端最新状态。没有推送、合并或修改凭据，未用演示数据替代真实数据。签名、公证、Windows 回归和 Mac 完整安装态尚未执行。当前接入优先缺口是 Mac 正常启动下的稳定 CLI 发现/选择机制。原生悬浮球/目录事件、通知、登录启动、菜单栏图标视觉、多屏和长期运行仍待实现或验收。

> 2026-10-16 后续实施更新：本地 Git 已建立 Windows 基线 `aa0daa9`；Windows 原生监听与拖动代码已收进 `src-tauri/src/platform/windows/`，新增 macOS 路径与定时唤醒骨架、平台配置分文件。以下早期“待实施/无 Git”描述保留为当时评估记录；当前实现状态与未验证项以 `docs/ARCHITECTURE.md`、`docs/PROGRESS.md` 为准。macOS 未实机编译或验收。

## 2026-10-16 macOS 技术方案评估（方案建议，尚未实机验证）

结论：按现有产品功能，建议继续 Tauri 2 + React/TypeScript + Rust，使用一个 TokenDaily 仓库、一个跨平台应用工程，平台相关实现按 windows/macos 模块隔离。无需先复制两个完整应用，也无需先用 SwiftUI 重写主界面。macOS 使用 AppKit 原生适配补足悬浮窗、拖动与系统行为；可通过 Rust 的 objc2 / objc2-app-kit 绑定访问。该结论是代码审查和官方能力文档支持的工程判断，不等于 macOS 构建、数据接入或安装态验收已经通过。用户当前要求确认方案，本轮未开始移植、初始化 Git 或改动运行程序。

### 现有代码证据与适配边界

| 功能 | 共用内容 | macOS 必须补充或验收 |
|---|---|---|
| 主面板、详情、用量/额度切换、设置 | React/TypeScript、品牌素材、数据状态与布局 | WKWebView 下的透明背景、字体、键盘与缩放 |
| 额度和重置时间 | codex.rs 的 JSONL 请求/响应、额度归一化、错误处理 | codex_executable 当前只查 codex.exe/LOCALAPPDATA；Mac 要查可执行的 codex，显式路径优先，并处理 Finder 启动时 PATH 与终端不同 |
| 本机今日 Token | local_usage.rs 的增量解析、去重、北京时间切日与缓存 | codex_home 当前默认 USERPROFILE；Mac 要按用户主目录与有效 CODEX_HOME 定位。Finder 不会自动继承用户终端配置，定制路径须可确认。真实 Mac 日志与账号读取须验证 |
| 日志变化监听 | 调度、增量读取和低频补查语义 | log_watch.rs 无条件使用 std::os::windows 与 Win32 HANDLE，是直接编译阻塞。建议 Windows 保留现有实现，macOS 通过 notify 的 FSEvents 后端提供统一的变化/唤醒/停止接口；通知不是可靠事件账本，保留补查 |
| 菜单栏常驻、通知、登录启动、单实例 | 已有 Tauri 插件和业务流程 | Mac 菜单栏模板图标、关闭/退出语义、通知授权、登录项、Dock 显示策略与实际安装路径 |
| 悬浮球、按住触边半隐藏 | 点击/悬停状态机、Logo 裁切、显示内容和交互契约 | floating_drag.rs 只有 Windows 实现，lib.rs 的非 Windows primary_down 恒为 false；必须实现 AppKit 拖动、按键状态和屏幕工作区适配，不能认为现有 cfg 已完成跨平台 |
| 安装和发布 | 产品名、版本和绝大多数公共配置 | tauri.conf.json 当前仅 NSIS，图标仅 PNG/ICO；需独立 macOS 打包配置、ICNS、Mac 构建和签名/公证 |

Cargo.lock 当前锁定 tauri 2.12.4 / wry 0.57.0。windows-sys 应移入 Windows 目标专用依赖；Win32 模块和测试使用条件编译，业务测试两端共用。源文件分层即可，不必立即建立多 crate 工作区或两个 apps 子工程。

### 建议的工程布局（待实施）

```text
TokenDaily/
  src/                            # 共用 React 界面与显示逻辑
  src-tauri/
    src/
      codex.rs, local_usage.rs, alerts.rs, usage.rs
      platform/
        mod.rs                    # 路径、监听、浮窗系统能力接口
        windows/                  # 保留现有 Win32 实现
        macos/                    # AppKit 与 FSEvents 适配
    tauri.conf.json               # 公共配置
    tauri.windows.conf.json       # NSIS / ICO
    tauri.macos.conf.json          # APP / DMG / ICNS
  assets/
  docs/
```

仓库/产品仍叫 TokenDaily；平台名称放在适配目录、配置和安装包文件名中。两端共同主分支、按功能开临时分支，不长期拆成互不合并的 Windows/macOS 分支。上述目录仅为建议，本轮未移动现有文件。

### 悬浮球的实现与原型验收门槛

- 共用产品契约：普通球仅点击展开、离开收起；拖动可见球体触及左右工作区边界时，按住期间即吸附并显示半 Logo；回拖恢复完整球；松手保持半隐藏，离开后重新悬停展开。靠近边缘静置不吸附。
- macOS 原生适配以 AppKit 为基础，优先使用 Tauri 自有窗口配合原生接口；如失焦、层级或全屏行为不符合要求，再在浮窗适配层使用 NSPanel，避免为一个浮窗重写整款应用。绑定库是桥接路线，不是已验证的即插即用方案。
- 使用当前显示器的可用区域处理菜单栏和 Dock，并明确逻辑点/Retina 像素与 AppKit 坐标转换。以工作区边缘延续现有 Windows 行为，不默认覆盖 Dock。
- 原型验收必须包含：鼠标未松手的半隐藏画面、回拖、松手保持、重新悬停、跨显示器缩放、Dock 左/右/底部及自动隐藏、普通桌面/Spaces/全屏应用、失焦不抢输入、休眠唤醒与退出清理。全屏/跨 Spaces 的最终支持范围取决于实测，不承诺无条件覆盖所有窗口。
- 设计只观察本工具窗口的交互，不默认引入全局输入钩子、录屏或辅助功能权限。是否有权限要求须以所选 API 和 Mac 实机为准，不能把无权限弹窗当作已测结论。

### 数据接入、版本与发布建议

- 官方 App Server 文档提供 stdio JSONL 和 account/rateLimits/read、account/usage/read，当前协议适配可共用；Mac 上的 CLI 版本、有效登录身份、额度字段和日志位置必须重新做只读验证。仍不发送模型推理、不修改共享配置、不登出用户 Codex、不保存凭据。
- 首轮采用已安装且已登录的 Codex CLI 作为可验证前提；只有桌面 App 而没有独立 CLI 的安装形式需要单独检查，不硬编码或承诺 Mac App 内部私有可执行路径稳定。
- 推荐首先交付经 Developer ID 签名和公证的 DMG，优先在 Apple Silicon Mac 上建立完整验收；Intel 与最低 macOS 版本按实际设备和依赖测试确定，不凭能生成安装包宣称支持。构建使用 Mac/Xcode 工具链；Windows 检查不能替代 Mac 安装态验收。
- Mac App Store 不作为第一轮目标。其 App Sandbox 要求意味着当前读取外部 Codex 日志、启动外部 CLI 的接入方式需要独立设计和验收；改用 SwiftUI 也不会消除这个限制。此处是分发路线建议，不是用户已授权发布或购买开发者资格。
- 透明窗口文档有版本差异：旧配置页仍描述 macOSPrivateApi 开关；官方 2.12.1 发布说明以及本地 CLI schema 明确该开关已不再必需。不能沿用旧教程直接判断当前版本需要打开该开关，也不能从开关取消推断应用已满足商店审核；仍需审计所用 API 和实际发行包。

### 实施顺序与当前验证状态

1. 先把当前 Windows 版本纳入 Git 并提交基线；当前目录尚无 .git（初始化待用户后续指令）。
2. 做最小平台拆分，在 Mac 上先验证真实只读额度/本机日志和完整悬浮球交互；共用测试在两端运行，Windows 保留回归。
3. 原型通过后适配菜单栏、通知、登录启动、图标和配置；安装态验证 Finder 启动及 PATH、授权、重启、关闭/退出。
4. 最后才做签名、公证与发布准备。若窗口原型不达标，先替换 macOS 浮窗实现；只有原生 UI 的明确需求或验证结果支持时，才考虑独立 SwiftUI 前端，同时保留 Rust 数据核心。

本轮只完成代码审查与官方文档核对，未运行 macOS 编译、测试、真实账号探测、签名或安装；也未新增运行依赖。无新的真实/演示数据产生。仅编辑本可行性记录和 PROGRESS.md，故未重复运行 Windows 业务测试。

参考（2026-10-16 核对）：

- [Tauri 平台配置](https://v2.tauri.app/reference/config/)、[系统托盘](https://v2.tauri.app/learn/system-tray/)、[通知](https://v2.tauri.app/plugin/notification/)、[登录启动](https://v2.tauri.app/plugin/autostart/)。
- [OpenAI Codex App Server](https://learn.chatgpt.com/docs/app-server)：stdio 与账户/额度/用量协议，仅作为 Mac 接入线索，不能代替 Mac 实测。
- [Apple NSPanel](https://developer.apple.com/documentation/appkit/nspanel)、[NSScreen.visibleFrame](https://developer.apple.com/documentation/appkit/nsscreen/visibleframe)、[NSEvent.pressedMouseButtons](https://developer.apple.com/documentation/appkit/nsevent/pressedmousebuttons)、[fullScreenAuxiliary](https://developer.apple.com/documentation/appkit/nswindow/collectionbehavior-swift.struct/fullscreenauxiliary)。
- [notify 的 macOS FSEvents 后端](https://docs.rs/notify/latest/notify/)。
- [Tauri macOS 应用打包与 GUI PATH 差异](https://v2.tauri.app/distribute/macos-application-bundle/)、[签名与公证](https://v2.tauri.app/distribute/sign/macos/)、[App Store 的 Sandbox 要求](https://v2.tauri.app/distribute/app-store/)。
- [Tauri 2.12.1 透明窗口开关变更](https://v2.tauri.app/release/@tauri-apps/api/v2.12.1/)、[对应维护者 PR](https://github.com/tauri-apps/tauri/pull/16166)。

## 工具链磁盘位置更新 · 2026-10-15

本机实测：Windows SDK `10.0.26100.7705` 已通过安装器迁至 `<Windows-SDK-directory>`，32/64 位注册路径一致；SDK AddOn 和 VS 开发用 UCRT 分发包按原版本恢复，原 12 项 SDK 组件选择保留。MSVC 自动选中 E 盘 SDK，C++ 编译/链接/运行及 TokenDaily 的 Cargo 实际重新链接均通过，无需重启。MSVC/Rust 在工程 `.local-toolchain/`，VS 下载缓存在 `<VS-download-cache>`。VS 安装器、部分共享安装缓存仍在 C 盘。SDK 换盘不代表重新完成了应用安装态验收，后续 VS/SDK 更新的路径保持行为尚未验证。过程与恢复记录见 `PROGRESS.md`。

更新：2026-10-15。这里区分本机实测与官方文档线索；原始账户响应、凭据和个人标识均未写入项目。

## 本机实测

| 项目 | 结果 |
|---|---|
| 工程起点 | 根目录只有 `AGENTS.md`、`TokenDaily-Codex-Handoff.md`、`START_HERE.md`；没有 Git 仓库及现成代码 |
| 操作系统 | Windows NT 10.0.26200（注册表显示 25H2，内部版本 26200.9550）；当前用户安装态已部分验收，独立标准用户与不同缩放未验收 |
| 前端工具 | Node 24.11.1、npm 11.6.2、Git 2.52.0.windows.1 可用 |
| 桌面工具 | Codex CLI `0.158.0-alpha.2.1` 可用；WebView2 已安装；原环境缺少 Rust/Cargo 和 MSVC，后按用户授权安装到项目本地 `.local-toolchain/` |
| Rust 构建 | Rustup 1.29.1、Rust 1.99.0、本地 Cargo 与 Microsoft C++ Build Tools/Windows SDK 已可编译 Tauri；VS 安装器返回 3010，提示需重启，当前会话的 `cargo check`、`cargo test` 已通过 |
| 只读探测 | `node scripts/probe-codex.mjs` 通过 `codex app-server --listen stdio://` 初始化，并只调用 `account/read`（`refreshToken:false`）、`account/rateLimits/read`、`account/usage/read` |
| 授权类型 | 返回 `chatgpt`，存在可核对的身份字段；没有执行登录、登出或令牌刷新 |
| 额度结构 | 返回 1 个额度池，其中只有 `primary` 窗口；`usedPercent` 与 `resetsAt` 是有效数值；未存储或输出真实百分比和时间 |
| Token 结构 | 返回日桶和汇总结构；日期时区、覆盖范围尚未核对，不能直接显示“今日 Codex” |
| Token 日桶复核 | 2026-10-15 新增只读探测：当前账户返回 67 个日期唯一的日桶；`startDate` 均为有效 ISO 日期格式，`tokens` 均为非负安全整数。响应未提供时区或覆盖范围字段；探测未打印账户身份、具体日期或 Token 数值 |
| Rust 用量适配器 | 显式真实只读契约测试通过：同一次本机 App Server 连接取得额度与非空日桶。用量请求单独设 5 秒等待上限，超时不丢弃已经取得的额度结果 |
| 沙箱边界 | 沙箱内 App Server 启动访问被拒；只读探测经受控沙箱外执行成功。未修改共享配置 |
| Rust 真实适配器 | 显式运行忽略的 `live_read_only_contract` 测试通过：当前账户返回脱敏身份及至少一个额度窗口；正常单元测试不依赖登录 |
| 原生桌面 | `npm run tauri dev` 编译并启动原生窗口；NSIS 当前用户安装成功，安装版窗口显示真实额度、脱敏身份和重置时间；关闭隐藏及重复启动单实例已实测 |
| Windows 通知 | 安装版手动测试通知请求返回成功，用户确认在 Windows 弹窗或通知中心看到了“TokenDaily 测试提醒”；点击跳转仍未验收 |

这些结果证明当前机器存在可用的只读额度路径，**尚未证明**所有账户、所有 Codex 版本或正式安装包都可用。探测没有发起模型推理、重置券消费、购买或任务控制请求。是否完全不计入服务侧计量，需要官方进一步明确；本次未发起任何推理请求。

## 官方文档线索与风险

- [Codex App Server](https://learn.chatgpt.com/docs/app-server) 说明 JSONL stdio 协议和上述账户/额度/用量方法。`rateLimits` 是单池兼容视图，`rateLimitsByLimitId` 是多池视图；`resetsAt` 为 Unix 秒。实现必须按池 ID 去重，而不能固定 `primary` 的窗口长度。
- 同一文档说明 App Server 自带认证可继续用于本地或开源应用，但不允许商业或托管服务；用户已选择**先做本地自用原型**。未来用途变化时要重新决定授权方案。
- [Codex MCP 迁移文档](https://learn.chatgpt.com/docs/mcp-server) 指明 App Server 命令仍是实验性且不支持生产工作负载。该路径可用于原型验证，正式发布可靠性待评估。
- [Tauri 托盘](https://v2.tauri.app/learn/system-tray/) 与 [单实例](https://v2.tauri.app/plugin/single-instance/) 文档提供候选桌面能力；当前尚无 Rust 构建和 Windows 安装态实测。
- [Tauri 通知](https://v2.tauri.app/plugin/notification/) 明确 Windows 通知只对安装后的应用正常工作，因此开发窗口发送成功不能代替 M4 验收。

## 未测与下一步

1. 与 Codex 官方界面在同一身份、同一时间核对额度及重置时间；探测脚本刻意不输出真实数字，目前无法完成对照。
2. 确认 `account/read` 是否能稳定获得工作区标识；如果不能，不能把额度称为某个工作区专属。
3. 通过真实适配器验证并存、断网、授权过期、部分更新、超时和低频采集开销。
4. 补齐安装态托盘菜单、通知点击跳转、缩放与非管理员账户验收；当前已验证本机当前用户安装、真实面板、关闭隐藏、单实例、开机启动、清除本地数据及测试通知实际可见。
5. 核对官方日用量的日期时区、服务覆盖范围与历史保留范围；当前只展示“来源日统计”，不承诺本地自然日或全 Codex 用量。

## 2026-10-16 日期新鲜度复核

本地 2026-10-16 09:30:55（UTC+08:00）独立只读探测仍返回最新日桶日期 2026-10-15，应用缓存一致。本地成功采集时间不代表统计日期已推进。官方 account/usage/read 文档与本次响应未明确统计时区或更新延迟；目前不能保证实时本地今日 Token，用量主展示需区分读取新鲜度和日期覆盖。不能将未返回的今日推断为 0，也不能直接把前一天的数值改成今日。

## 2026-10-16 竞品采集机制与本机近实时可行性

- 用户强调实时用量是产品核心，并指定 https://scys.com/tokenrank/ 与 https://aipoju.com/token-ranking 作参考。本次读取网页规则、接入说明并只读核验本机日志，未安装竞品采集器、生成接入凭证或上传本机用量；未改产品采集代码。
- 生财官方规则 https://scys.com/tokenrank/rules ：Codex 等工具采用本机会话日志统计；同设备以最近上报替换而非重复累加，官方账户口径与本地口径不同。opentoken 定时任务每 30 分钟读取/上报。此为产品页面描述，未对其二进制或私有实现做源码审计。
- AI 破局排行榜“统计规则”及 https://aipoju.com/token-ranking/install ：Codex 按已采集的会话/小时记录去重，以北京时间汇总；每 30 分钟重算/覆盖快照，多设备汇总，不代表完整账户。仅接收统计和哈希标识，不接收对话等内容。这些是站方说明，不能宣称已测秒级榜单更新。
- 本机实测：CODEX_HOME 为 <CODEX_HOME>；sessions 共发现 497 个 jsonl，仅抽查最近修改的 6 个文件。2026-10-16 09:46:30（UTC+8）检查到 36 条属于本地 10/16 的 token_count 事件（抽样原始条数，不是去重后的日总量），最新事件时间 09:46:30.131，距检查约 0.64 秒。另一些记录存在累计值重复，不能累加所有 total_token_usage。未输出或保存对话、身份、项目路径；诊断脚本在 .bundle-tmp/probe-local-token-events.mjs。
- 实测字段：total_token_usage/last_token_usage 下有 input_tokens、cached_input_tokens、cache_write_input_tokens、output_tokens、reasoning_output_tokens、total_tokens。样例 total_tokens = input_tokens + output_tokens，不能把 cached_input_tokens 再叠加到 total_tokens。样本不足以证明所有版本、fork/续接/压缩情况下都可直接相减，实施前需补齐契约和去重测试。
- 调整方向：用量主展示改为“本机今日 Codex / 按本地时区 / 含缓存”，账户额度继续原只读接口，账户来源日历史单独保留。两类来源不相加、不以低数值回退高数值、不把未采到当作零，不推断全账户覆盖。
- 建议验收目标（尚未实现/实测）：源日志写出有效用量事件后 5 秒内面板反映；按文件偏移增量读取，发现新会话并低频补扫；静止时不重复解析全部历史。重启/重复事件/fork/累计值回退/跨午夜/半行写入/文件轮转/目录不可读均需测试。模型尚未写出 usage 时不能承诺逐 Token 实时。
- 官方事件说明 https://learn.chatgpt.com/docs/app-server#turn-events 表示 thread/tokenUsage/updated 属于活动线程；不能未经验证假设单独启动的 app-server 可以全局监听其他 Codex 进程的所有线程，因此本机日志路径更贴合当前独立助手。

## 2026-10-16 本机日志实现验证（按用户指定北京时间）

- 已实现，替代此前仅调研状态。有效 CODEX_HOME 的 sessions 与 archived_sessions 为同一来源，其他旧目录不合并。逐事件采用北京时间日期，不使用目录日期或文件修改日期。
- 重要实测：当前设备至少两份日志的 LastWriteTimeUtc 在 10/15，但内部最后 token_count 时间分别到 10/16 01:47:41Z、01:20:17Z。仅按 mtime 筛今日会漏算，已取消，相关测试刻意将文件 mtime 设为 Unix 纪元后 1 秒仍能计入今天事件。
- 官方协议源码 TokenUsageInfo::append_last_usage 将 last 加到累计 total；fill_to_context_window 可生成 total 非零、输入/输出为零的估计。实现使用输入加输出，跳过这种估计，缓存/推理字段不重复叠加。源码仅作为语义依据，本地真实契约另有测试：https://raw.githubusercontent.com/openai/codex/main/codex-rs/protocol/src/protocol.rs 。读取日 2026-10-16，未来 Codex 格式变更仍需复核。
- 真实只读测试覆盖 499 个文件（含归档），约 1.88GB；优化后 debug 首次全量 17.44 秒、第二次增量 36ms；安装版能读出今天 [用量已脱敏] Tokens（验收时快照，会继续增长），账号日桶仍停在 10/15，证明两种来源必须独立展示。
- 新模块不发起网络请求或模型推理，不修改共享配置/登录/日志，不持久化原始日志。2 秒是检查间隔，不是模型流式输出到落盘的时延保证。首次启动与新增大日志需扫描；暂无跨版本、长期休眠、海量多目录测试保证。

## 2026-10-16 Windows 通知与启动索引

- 官方 ReadDirectoryChangesW 文档说明目录变更异步订阅、缓冲区溢出需重新枚举，以及文件大小/修改通知可能依赖缓存刷新：https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-readdirectorychangesw 。本机测试证实未关闭的普通 WriteFile 写入并不总能立即收到通知；FlushFileBuffers 后通知测试通过。不能将“订阅成功”当作实时到账保证。
- 实现使用通知合并 + 8 个近期日志的轻量长度检查 + 30 秒目录补查。文件内容仍增量读；手动、北京零点和目录重建独立处理。原生请求取消后等待完成再释放缓冲区，避免关闭时访问已释放内存。
- 索引恢复前核验 checksum、来源哈希、版本、文件头；只存归一化数值及哈希。本机 C 盘应用目录 fs::rename 返回错误码 17，已选双槽校验回退方案而不是假定原子重命名成功；E 盘单位测试不能替代 C 盘安装态验证。

## 2026-10-16 贴边隐藏的原生窗口验证

- 在本机 Windows 125% DPI 下，将悬浮窗最小逻辑宽度降为 10 后，setSize 可得到 13×70 物理像素窄边，展开可恢复 375×313 面板，禁用后恢复 70×70 球体。隐藏使用缩窄而非移到屏幕外；左右工作区边界定位有安装态通过记录。
- 负坐标工作区、缩放阈值、面板/球体/窄边定位有纯函数测试；多显示器实际跨 DPI 与任务栏布局改变尚未实测。鼠标自动验收会被桌面输入干扰，失败时需对照真实指针坐标，不能仅凭脚本失败断言界面坏掉或凭部分通过宣称全部覆盖。

## 2026-10-17 Claude 数据来源与双来源展示

- 文档线索：Anthropic 官方 Pro/Max 使用说明确认同一账户的 Claude Code 与聊天端共享限额（https://support.claude.com/en/articles/11145838-use-claude-code-with-your-pro-or-max-plan）。Claude Code 官方 statusline 文档提供额度字段，但用户已有状态栏，本任务不覆盖该配置。
- 实现线索：CodexBar 的 Claude 来源说明与 OAuthUsageFetcher 使用 api.anthropic.com/api/oauth/usage、profile，Bearer + anthropic-beta，读取 .claude/.credentials.json 中的订阅 OAuth。此为内部账户端点线索，不构成官方稳定 API 保证。参考 https://github.com/steipete/CodexBar/blob/main/docs/claude.md 和 https://github.com/steipete/CodexBar/blob/main/Sources/CodexBarCore/Providers/Claude/ClaudeOAuth/ClaudeOAuthUsageFetcher.swift 。
- 已测（当日纠正后）：本机没有 Code 订阅登录文件、Claude projects 无 JSONL，但用户截图确认 Claude 桌面聊天已有对话且显示 Free。Windows 安装位置运行时额度显示“额度未接入”，明确不代表聊天端未登录；无有效 Code 用量记录显示未知、合计标注不完整。此前将空目录显示为 0 的判断已修正。双来源隔离及 UI 同步通过。合成日志/响应用于日期、去重、部分行、非法计数及额度字段测试，明确不当作真实 Claude 请求成功。
- 未测：Claude 真实额度 happy path、真实非空日志时延、实际订阅账户切换、网络限流、长期运行与通知送达。网页/桌面聊天精确 Token 无可靠已核验来源，界面明确未接入。Mac 钥匙串未实现、Mac 未实机验证。详见 CLAUDE.md。

## 2026-10-17 WorkBuddy 来源核验

- 用户要求暂停 Claude 额度调试，增加 WorkBuddy。本机 `.workbuddy` 为指向 D 盘的目录联接，projects 下 5 个 JSONL 合计约 1.31MB。只检查字段与计数，没有输出对话正文。实际记录包含 function_call、毫秒 timestamp、sessionId/id、providerData.usage 与 rawUsage；规范化 inputTokens 已包含缓存，outputTokens 已包含推理，totalTokens 等于两者相加。
- 真实只读测试 `cargo test --offline --manifest-path src-tauri/Cargo.toml live_workbuddy_logs -- --ignored --nocapture` 通过：首次 24.9204ms，第二次 369.2µs，第二次读取正文 0 字节；当日 Token 为 [用量已脱敏]，历史来源有有效记录。仅当前小样本性能，海量/长期运行未验证。未为了验收发起模型请求。
- 公开一手实现线索：[WorkBuddy Switch](https://github.com/koi128bit/WorkBuddy-Switch) 的 desktop/usage.mjs 描述 projects 记录、按记录 ID 去重；desktop/quota.mjs 与 settings.mjs 提供只读积分查询及登录路径线索。只读查看公开代码，不安装或运行第三方采集器。
- 本机已定位 CodeBuddyExtension/Data/Public/auth/workbuddy-desktop.info，accessToken 为加密对象，仅输出字段结构/加密状态。未解密、未导出或上传凭据。运行实现不读取凭据、完全不联网；积分查询未接入，需后续独立验证官方网页登录授权及真实积分返回。不能用 Token 消耗推算积分余量，不能将积分到期当作额度重置。
- Mac 仅增加 HOME 下数据目录适配，共享规则可复用但未在 Mac 编译/实机验收。详细口径见 WORKBUDDY.md。


## WorkBuddy 积分授权实测 · 2026-10-17

- 新增独立官方网页登录授权：state 成功、未登录等待码 11217、选择已有个人账户后登录成功，TokenDaily 收到令牌与账户标识。用户本人完成登录；未读取客户端加密凭据或浏览器 Cookie。账号明文/令牌不进入 React、日志或 Git。
- 真实 /billing/meter/get-user-resource-summary 返回 data.Packages 分类汇总，CycleRemainCapacity 为明确的剩余积分，TotalCount 为分类内项目数。与官网“套餐与积分”的基础积分和平台奖励积分逐项核对，合计一致。主面板、总览、悬浮窗均使用真实余额。
- 汇总无逐包有效期/名称，详情以积分汇总 N 项标注，有效期未知；没有把官网显示的套餐周期时间硬编码成重置。逐包资源列表/分页只保留保守 fallback，未对全部付费套餐、企业、特殊积分或超百项账户验证。
- Windows 当前用户 DPAPI 往返/损坏拒绝测试通过，应用重启后能从自己的加密授权恢复并重新查询；手动连续点击被节流。五分钟调度由共用规则测试覆盖，本轮没有专门等完整五分钟验证自动刷新送达；过期重新登录，不自动刷新共享授权。
- 公开线索来自官方套餐页面与 WorkBuddy Switch 登录/积分实现；这是客户端接口，不是稳定性承诺。Mac 安全存储/官方授权禁用，未编译或实机验证。

## Mac WorkBuddy 低打扰连接核验 · 2026-10-17

- 后续 Mac 专用钥匙串和独立官方登录已实现并完成安装态余额/重启恢复验收，上节的 Mac 未实现为历史状态；详见 PROGRESS 的 Mac WorkBuddy 积分修复记录。
- 当前 Mac WorkBuddy 5.8.3 登录文件的 accessToken/refreshToken 均为加密封装，仅检查字段类型。实际本机 `/workbuddy/probe` 返回 200，只提供应用/版本/平台；另外探测到的 MCP 健康接口不返回积分。随包账户模块通过认证后的远端账单接口读取积分，已检查路径未发现受支持的本地免授权积分 API。
- [官方英文 Open API 文档](https://open.workbuddy.cn/en/docs/openapi) 提供个人积分只读 scope `user.credit.readable` 和 `/openapi/v2/credit`；实际无凭据 GET 为 401。授权后的新接口、总量/用量口径和桌面公共客户端授权方式尚未验证。当前文档的 clientSecret 不可嵌入客户端；未注册新应用、增加服务端或改动现有认证。
- 可以实现自动发现和减少重复连接步骤，但当前证据不足以承诺安装后零授权读积分。正式签名连续性及非阻塞钥匙串恢复是独立待办；未以解密宿主凭据或修改其安全设置实现免密。完整证据与范围见 WORKBUDDY 的 Mac 5.8.3 核验表。

## 2026-10-18 本机总 Token 历史

- 用户确认历史日历不再使用 Codex 账户的 `dailyUsageBuckets`：它的时区、延迟和设备覆盖范围都不能用来替代本机自然日。历史页只读扫描 Codex、Claude Code 和 WorkBuddy 本机日志，以每条事件时间按北京时间分组。
- 为避免干扰日常运行，完整历史扫描只在用户打开“总 Token 历史”时执行；实时采集仍使用现有增量读取、目录通知和低频补查，不会增加两秒轮询的读盘范围。
- 去重沿用各来源实时采集的标识与“字段取最大值”规则；跨来源同日期相加。可读源当天没有事件时可显示 0，旧日期没有可读记录保持空白。某个来源目录不可读时保留其他来源数据并标注统计不完整。
- 已有单元测试覆盖 Claude Code/WorkBuddy 的历史去重与北京跨日、合并缺失日和溢出拒绝。尚未在真实 Windows 安装态测量 13 周扫描时间，未在 macOS 编译或实机读取；不宣称三个来源在所有机器均可用。

## 2026-10-19 TRAE 本机读取核验

- 用户进一步要求优先本机读取。TRAE CN 3.4.4 普通日志未发现可靠 Token 字段，但已实测其 SQLCipher 兼容本地数据库可只读查询；使用公开应用级密钥验证页 HMAC，不读取登录凭据或扫描进程内存。
- 一致快照包含已提交 WAL，在内存中解密并用 SQLite 仅查询时间/计数；18 条有效记录，北京时间今日一条为输入 [输入量已脱敏]、输出 [输出量已脱敏]、合计 [总量已脱敏]，与官方账号页面对应记录一致。没有明文数据库落盘或修改 TRAE 文件。
- 这是可行性验证，不是已接入功能：多请求累计字段、同步历史的设备归属、增量性能、运行时状态/界面/安装态均待验证；macOS 未验证。官网积分可独立查询，但不是 Token 的本机来源，授权仍待实现。详见 TRAE.md。

## 2026-10-19 TRAE Windows 运行时实测补充

- 本地数据库路径已转为产品只读采集器；内存页认证+已提交 WAL，SQLite 白名单字段投影、去重/修正和北京时间聚合。真实今日 [用量已脱敏] Tokens，Release 完整手动刷新含 IPC 约 65.43ms，重复请求约 7.31ms；未变化跳过正文与聚合。64 MiB 单文件上限、200,000 记录上限，超出显示未知，不伪造零。没有做海量或长期验收。
- 用户完成独立官方窗口登录后，收到真实 [余额已脱敏] 积分/3 包；重启及手动刷新恢复成功，连续点击被节流。没有导出 Cookie/令牌、解密客户端凭据或上传日志/对话。已接入四卡+独立日历；实际安装态见 PROGRESS。
- 缓存、推理不重复累加；真实样本累计字段均为 0，多请求累计规则仅有公开实现线索与合成测试。恢复/同步会话的设备归属没有证实，界面称“本地记录”并说明范围。TRAE 的 API 费用暂不估算。Mac 未验证并禁用入口，国际版未接入。
