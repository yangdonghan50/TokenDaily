# TokenDaily 0.1.0 本机测试记录

## 0.1.26 Windows Claude 额度来源优先级 · 2026-10-21

Windows 11 Home 10.0.26300 x64，本机 E 盘工具链。前端 `npm test` 97/97、`npm run build` 通过；Rust `cargo test --locked --offline --lib` 129 通过、8 忽略，`cargo check --all-targets` 通过（既有 `can_submit` 警告）。真实只读诊断：无 Claude Code 登录时回退 Desktop 记录且重置时间未知；登录后 OAuth 来源 verified=true，返回 5 小时与 7 天重置时间。签名 NSIS 包 SHA-256 `7656BF62…C8C7`，Minisign 校验通过；以 `/D=D:SoftwareTokenDaily` 原位安装，注册表与文件版本 0.1.26，安装版与构建版仅差 3 字节包类型标记。用户目视确认悬浮窗显示两项重置时间；首轮误装到默认目录的副本已移至 `.bundle-tmp`。

## 0.1.23 → 0.1.24 Mac 隔离应用内升级 · 2026-10-19

独立标识 `app.tokendaily.updater-test`，Intel macOS 15.7.7。仅临时测试构建使用本机 `127.0.0.1` 静态清单、测试公私钥与自动触发；正式源码已恢复 HTTPS 官网白名单，临时私钥已删除。错误签名场景：保留清单和签名，篡改下载包后，状态为 `Available → InstallError`，磁盘上的旧 App 仍是 0.1.23。正确签名场景：Tauri 官方生成 `.app.tar.gz`/`.sig`，从 `/private/tmp` 独立安装的 0.1.23 下载、验签、原位替换并自动重启到 0.1.24；重启后状态为 `Current`，`Info.plist` 和运行进程版本吻合，`codesign --verify --deep --strict` 成功。正常 `/Applications/TokenDaily.app` 进程在整个测试期间保持运行，没有替换或重新授权。

手工系统 `tar` 产生扩展属性副文件 `._icon.icns`，在更新后造成代码签名封存资源校验失败；改用 Tauri 官方更新包后消除。从 Desktop 工作区启动的隔离 App 在 macOS `renamex_np` 停住；`/private/tmp` 完成上述成功路径。真实 `/Applications` 管理员授权、当前用户设置/钥匙串保留、Apple Silicon、官网 HTTPS 及 Developer ID/公证包仍未验收。测试包使用临时密钥，与正式 Mac 配置公钥不同，不能作为公开更新包。

## 0.1.23 Mac 应用内更新本地验证 · 2026-10-19

`mac/in-app-updates`，Intel macOS 15.7.7。前端 `npm test` 93/93、`npm run build` 通过；Mac Rust `cargo test --locked --offline --features macos-transparent --lib` 130 通过、10 忽略，新增官网 Mac 同版本 HTTPS 下载地址白名单用例；Rust fmt 与差异检查通过。`tauri build --no-sign -- --locked` 生成 Intel App/DMG，DMG 校验成功。复制 App 的 ad-hoc 签名严格校验通过；本地临时测试密钥签署 `.app.tar.gz` 后，以相同公钥验证包签名和签名版本成功，篡改包及错误版本均被拒绝。临时私钥已删除，不用于正式客户端。

官网 `https://tokendaily.net/updates/darwin-x86_64.json` 实测 HTTP 404。当前 `/Applications` 中仍是之前的 0.1.23 禁用构建；本节初次测试未执行安装态升级，随后的隔离两版本升级见上方补录。真实 `/Applications` 授权/设置保留验收、Developer ID 签名、公证与 Apple Silicon 测试仍未完成，不能宣称官网应用内更新已上线。

## 0.1.23 本地 main 向 Mac 同步 · 2026-10-19

- 分支 `mac/main-0.1.23-sync` 合入本地 main `e97aa2d`；远端 fetch 403，不能证明远端无更新。共享前端 `npm test` 93/93、`npm run build`、Mac Rust `cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml --lib` 129 通过/10 忽略、`cargo fmt --check`、差异检查通过。首次离线 Rust 测试仅因 updater crate 本地缓存缺失停止，联网安装锁定依赖后测试通过。
- Intel App/DMG Release 0.1.23 构建、`hdiutil verify`、构建 App 的本地临时签名与严格校验通过。旧 0.1.22 ZIP 完整备份并验证，旧 App 另行保留；替换 `/Applications` 后安装版版本为 0.1.23，签名校验通过，二进制 SHA-256 与构建 App 一致。五个设置文件安装前后摘要一致。
- 首次启动真实 Codex 今日 Token 可读、原 Codex 来源仍选中，悬浮球 56×56。系统因本地临时签名变化请求 WorkBuddy 专属钥匙串访问，用户在系统弹窗选择“始终允许”后提示消失；本轮未读取或代填密码。WorkBuddy [余额已脱敏]、TRAE [余额已脱敏] 在主面板和悬浮窗可见。300×540 浮窗 AX 树含顶部三项、四来源与 20 周日历，更新按钮 disabled，明确显示“此平台暂不支持应用内更新”；新增按钮像素无裁切未目视确认。
- 正常退出并重启同一安装版后，不再弹 WorkBuddy 钥匙串确认，两项积分自动恢复。参榜身份“测试账户”、当时 #[名次已脱敏]、最近同步成功及每 30 分钟自动同步仍在；未手动上传。恢复原 Codex 视图后 `providers.json` 与验收前摘要一致，球体回到 56×56；五个配置文件安装前后与重启后摘要一致。Mac 原位更新明确 unsupported，不使用 Windows 更新插件；本轮不下载或安装官网 Windows 包。签名发生在 DMG 打包之后，DMG 不作为正式签名分发验证。

## 0.1.22 Mac TRAE 接入源码 · 2026-10-19

- 安装态补验：用户在独立 TRAE 官方窗口自行登录；真实积分 [余额已脱敏] 在主面板和悬浮窗显示。正常退出并重启 `/Applications/TokenDaily.app` 后，无须重新登录，积分自动恢复；WorkBuddy 积分也恢复且无新的专属钥匙串确认。公开参榜身份“测试账户”、本周 #[名次已脱敏] 与 30 分钟自动同步状态仍在。300×540 悬浮窗 AX 树含顶部三项、四来源卡片和 20 周日历；未完成新增卡片的像素目视验收。主面板恢复原 Codex 视图，`providers.json` 摘要与验收前一致，悬浮窗已收起。未执行真实断开/清除，以免注销用户刚完成的授权。
- 真实 TRAE CN 数据库当前无聊天轮次，Token 用量维持未知；实际 Token 解析、去重与总历史仍待有记录时验证。用户未提供密码或验证码，本轮未读取网页登录凭据。
- Mac TRAE 本地连接及 macOS 14+ 独立 WebKit 官方积分窗口已实现。`bash scripts/mac-dev.sh cargo test --locked --offline --features macos-transparent --manifest-path src-tauri/Cargo.toml --lib`：126 通过、10 忽略；`npm test`：91/91；`cargo fmt --check`、`git diff --check`、TypeScript/Vite 与 Intel App/DMG 构建通过，`hdiutil verify` 通过。构建 App 的本地临时签名和严格校验通过。
- 新测试确认 Mac 连接设置可恢复但不会伪造用量，积分授权状态恢复不会携带旧余额。真实 TRAE 数据库目前 0 条轮次；官方登录、积分与重启已实测，真实授权清除未实测。Windows 对本次变更未编译或安装验收。
- 用户在原安装版应用端完成公开参榜；AX 显示昵称“测试账户”、本周 #[名次已脱敏]、[用量已脱敏] Tokens、最近同步成功及 30 分钟自动同步。随后用户确认可重启，旧安装版完整备份并校验，构建版已替换到 `/Applications/TokenDaily.app` 且严格签名与二进制摘要核对通过；三份既有设置/来源文件摘要未变。新签名启动后曾触发系统 WorkBuddy 钥匙串确认，由用户在系统弹窗处理。
- 用户在系统 WorkBuddy 弹窗完成“始终允许”后，提示消失；重启后的 Mac 榜单显示原身份“测试账户”、#[名次已脱敏]、最近同步成功和 30 分钟自动同步。本地 TRAE 连接设置可启用并写入独立 `trae.json`；独立官方积分窗口已在安装版打开，AX 确认是真实 TRAE 官方登录页。用户的官方凭据仅在该窗口自行输入；登录后的余额与再次重启结果见上方补验。

## 0.1.22 Mac 主线同步 · 2026-10-19

- 环境：Intel macOS 15.7.7，本地分支 `mac/main-0.1.22-sync`，整合 Mac `cdc6601` 与本地 main `e15a4a8`。没有核对成功远端最新提交；不代表 Windows 或 Apple Silicon 验收。
- 检查：`npm test` 91/91；`npm run build` 通过；Mac `cargo test --locked --offline --features macos-transparent --lib` 124 通过、9 个按条件忽略；`cargo fmt --check`、`git diff --check` 通过。新增官网公开身份 UUID 的 Mac 官方 HTTPS 白名单测试通过。未运行需真实生产参榜的专项测试，未上传本机用量。
- 构建：`bash scripts/mac-dev.sh npm run tauri -- build` 生成 0.1.22 Intel App/DMG，`hdiutil verify` 通过，第三方声明文件在 App 资源内。App 已按先前授权本地临时签名，`codesign --verify --deep --strict` 通过；不代表 Apple Developer 正式签名、公证或 Gatekeeper 全新下载验收。
- 安装：旧 0.1.17 先 ZIP 校验备份于忽略目录 `.bundle-tmp/mac-validation/before-main-0.1.22-20261019`，然后替换 `/Applications/TokenDaily.app`。新版启动且原 settings/providers/workbuddy 文件摘要未变；真实 Codex 只读数据、手动来源、通知设置和快捷键入口在安装态可见。TRAE 页显示当前平台不支持，连接和官方积分授权均禁用。复查确认本机有 TRAE CN 3.4.4 及数据库；数据库可只读解密，但 `chat_turn` 表为空，不能宣称真实用量监测完成。
- 安装态复验：首次钥匙串确认后真实 WorkBuddy 官方积分在主面板/悬浮窗一致并显示两位小数；WorkBuddy 日志未读到时 Token 保持未知。第一次正常退出重启又弹确认，用户再次在系统弹窗选择“始终允许”；第二次退出重启未再弹确认，积分自动恢复，无需重新登录。三份原 settings/providers/workbuddy 文件在临时来源切换恢复后摘要一致；重复启动仅一个进程。
- 悬浮与参榜：原生窗口 300×540，AX 树含顶部三项、四来源卡片与 20 周日历；真实总 Token 和 WorkBuddy 积分均可见。用户目视反馈 TRAE 未连接、排行未上榜，符合 Mac TRAE 未实现及试用身份未公开状态；没有独立确认新版像素无裁切。Mac 参榜试用身份从钥匙串重启恢复，本机本周上传预览可生成并显示去重记录与日期；没有填写昵称、勾选同意或上传。官网公开 UUID 定位行为仅由单测验证，未以真实公开身份实测。
- TRAE 来源检查：显式 `live_trae_database` 未通过，原因是“未发现 TRAE 有效 Token 记录”；`live_trae_database_record_shape` 通过，只读聚合显示 0 条轮次、0 个 Token 用量对象，不读取/输出会话内容。加入诊断测试后 Mac Rust 全量测试 124 通过、10 忽略，格式和 diff 检查通过。该结果不能证明有记录时 Mac 采集器正确。
- 当前 0.1.22 源码显式运行 `native_conflict_keeps_original_and_drop_releases` 通过，验证原生快捷键冲突保留和释放；新版安装态全流程仍未重测。
- 限制：TRAE Mac 原生监测/授权、生产参榜上传、新版安装态快捷键全流程、Apple Silicon、多屏及正式签名升级均未验证。未付费、部署或上传源码。用户是否选择公开参榜或继续开发 Mac TRAE 接入待确认。

日期：2026-10-15。测试机：Windows 10.0.26200（25H2，内部版本 26200.9550），Node 24.11.1，npm 11.6.2，Rust 1.99.0，Codex CLI 0.158.0-alpha.2.1。安装包为本地原型，未签名，未公开发布。

## 自动检查

| 检查 | 结果 |
|---|---|
| `npm test` | 14/14 通过：额度转换、缺失/非法值、窗口去重、时间与身份、提醒状态、缺失日期与零用量区分、日历对齐 |
| `npm run build` | TypeScript 检查与 Vite 生产构建通过 |
| `cargo test --offline --manifest-path src-tauri/Cargo.toml --lib` | 14 通过、1 个真实登录契约测试默认忽略；新增日桶解析、身份切换与额度/用量分别失败的状态测试 |
| 显式 `live_read_only_contract` | 受控沙箱外通过；验证当前本机登录、额度与非空日桶，不输出真实数值 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通知、开机启动和托盘后端编译通过 |
| `npm run tauri build` | Release 程序与 NSIS 安装包构建通过 |

首次 NSIS 打包因跨盘临时文件移动失败，随后一次官方工具下载超时；已校验并整理官方 NSIS 缓存，最终构建通过。使用项目内 `.bundle-tmp/` 作为构建临时目录。

## 安装版实测

| 项目 | 结果 |
|---|---|
| 当前用户静默安装 | 通过；安装器退出码 0，HKCU 卸载项和 `%LOCALAPPDATA%/TokenDaily` 存在 |
| 启动与真实额度 | 通过；安装版显示脱敏身份、一个真实额度窗口及重置时间，未显示演示标记；具体数值未记录 |
| 每日 Token 与趋势 | 通过；更新安装版缓存显示额度和日用量均可用、身份一致，保存 67 个规范化来源日桶；Windows 无障碍树可定位首页用量摘要、详情中的趋势和日历控件。浏览器演示界面在 420 像素宽检查，点击明确零用量日期显示 0，无横向溢出；没有输出真实数值 |
| 演示隔离 | 生产前端以 `?demo=normal` 打开仍为未连接状态，不显示虚构额度或用量；开发模式演示有醒目标记 |
| 关闭窗口 | 通过；窗口隐藏，TokenDaily 进程仍运行 |
| 再次启动 | 通过；仍只有一个 TokenDaily 进程，原窗口重新显示 |
| 开机启动 | 通过；默认关闭，安装版开关临时打开后恢复关闭 |
| 清除本地数据 | 通过；设置、快照和提醒账本文件消失；清除后只读 Codex 探测仍返回登录与额度结构，再次连接成功 |
| 本地隐私字段 | 已检查保存文件结构：无凭据、Cookie、原始对话或请求正文；当前快照只含脱敏身份、窗口、来源日 Token 数量、状态和时间字段 |
| 手动测试通知 | 应用侧发送调用成功，用户确认在 Windows 弹窗或通知中心看到了通知；点击跳转**未验收** |
| 托盘菜单/显式退出 | 托盘构建和单实例行为已运行；Windows 无障碍接口未定位到隐藏图标菜单，菜单项和显式退出**未人工验收** |
| 空闲资源快照 | 上一安装包的测量：主进程 5 秒 CPU 增量 0 秒，约 28.5 MiB 工作集；完整进程树 7 个进程，工作集总和约 445.3 MiB、专用内存约 201.5 MiB。工作集总和可能重复计算共享页；新增日用量版本尚未重测资源 |

本机当前安装包：`src-tauri/target/release/bundle/nsis/TokenDaily_0.1.0_x64-setup.exe`，大小 2,774,181 字节，SHA-256 `885A94A3D7B586A93945D2340A49D69AEA081223A71E784A0B3016B843714AAC`。2026-10-15 覆盖安装退出码 0，重新启动后真实额度及来源日用量结构正常。NSIS 默认当前用户安装，无管理员权限要求，但尚未在独立标准用户账户验证。

## 待验收

1. 与 Codex 官方界面在同一账户、同一时间比较真实百分比和重置时间；当前只验证来源结构和安装版展示。
2. 通知点击返回面板，以及低额度/恢复两种真实事件的端到端送达；安装态手动测试通知可见，自动规则已由单元测试覆盖。
3. 托盘图标点击、右键菜单四项与正常退出；不同 Windows 缩放比例和多屏位置。
4. 独立非管理员账户、断网/授权过期与长时间后台并存资源开销。VS Build Tools 安装器提示需重启，当前会话编译和安装已通过；重启后工具链可用性待复查。
5. 官方日桶的日期时区、产品覆盖范围及历史保留范围尚未确认；界面按来源日期展示，不承诺本机今日或 Codex 全量。安装版无障碍树确认“日历”控件存在，进一步点击它的检查因自动权限审批超时而未完成；浏览器演示环境的日历点击和零用量选择已通过。

演示数字仅存在于开发参数 `?demo=...` 中；上述安装版额度与来源日 Token 均来自当前本机 Codex 只读接口。用量日期和覆盖范围尚未核实，界面保持来源口径说明。

## 2026-10-15 悬浮窗增量验收

- 新增设置兼容与持久化错误测试，Rust 16 通过 / 1 默认忽略；前端 14/14。类型检查、生产构建及 NSIS 打包通过。
- 300×250 演示覆盖单/双窗口、低额度、未知、过期、未连接；420×620 设置开关可用，无横向溢出。
- Windows 安装版：设置开启/关闭、关闭按钮同步开关、原生关闭、落盘与重启恢复、鼠标拖动、置顶、主窗口关闭后继续显示及重新打开均通过。实际来源为本机 Codex 只读数据，未使用演示值。
- 当前缩放下原生窗口截图检查通过；多屏和跨 DPI 拖动、长期资源占用未验收。清除本地数据的关闭逻辑已实现，本次为保留用户数据未执行清除操作。

## 2026-10-15 悬浮球增量验收

- npm test 18/18；TypeScript、Vite 生产构建、Tauri release / NSIS 打包通过。Rust 业务代码未改，本轮未重跑 Rust 单元测试。
- 已实测原生透明球体、悬停展开/持续保持、面板内移动、移出延迟收起、边缘内移/锚点恢复、球体拖动。当前缩放原生球体窗口 70×70、面板 375×313 物理像素。
- 浏览器测试了重新移入取消收起、48×48 球体与透明背景、展开无横向溢出。计时、拖动抑制、负坐标边界均有新增单元测试。
- 尚未实测多屏和跨 DPI；原有真实提醒、来源日期口径等待验收项仍保留。
- 最终安装版启动尺寸校正通过，正常模式首次创建窗口即为 70×70 物理像素；原生完整交互再次通过，调试端口关闭。真实 Tab 聚焦展开与 Escape 收起已验证。

## 2026-10-15 拖动后无法收起回归

- 前端 22/22，Rust 16 通过 / 1 默认忽略，生产构建与 NSIS 打包通过。
- 新增缺失鼠标移开事件、拖动期间保持展开、连续状态采样计时、键盘焦点恢复及取消原生收起后的状态同步测试。
- 安装版临时屏蔽所有 DOM 鼠标/指针移出事件，三轮展开面板拖动后仍自动收起；收起静止时原生鼠标查询数为 0。Tab 展开与 Escape 收起通过。
- 最终六轮正常原生拖动回归全部通过，覆盖长按、立即移出、停留后移出与屏幕边缘；已正常重启清理调试状态，初始小球 70×70 物理像素、置顶可见。

## 2026-10-15 悬浮窗双模式增量验收

- 测试先行：npm test 25/25；Rust 17 通过 / 1 默认忽略；格式化、类型检查、生产构建、release/NSIS 通过。
- 浏览器 300×250，六场景 × 两模式 12 组，横向无溢出，底部状态可见；大数字/零、来源日、过期标记检查通过。两种布局目视检查通过。
- 安装版更新成功，真实数据共享、模式切换、设置页同步、持久化与重启恢复通过。实际只读设置文件故障验证保存失败保留原模式且显示错误；属性已恢复。非法枚举请求被拒绝。
- 原生额度模式、用量模式各完成一轮拖动/长按/移出收起；后者含屏幕右下角。Tab 展开和 Escape 收起通过。连续脚本另有间歇未命中/未移动失败，故不宣称多轮连续验收通过；详情见 PROGRESS。
- 未测：多屏跨 DPI、触摸、长期运行、清除本地数据实操；默认忽略的真实接口测试及通知未重跑。Token 来源日与范围限制保持。

## 2026-10-16 顶栏切换与刷新

- npm test 27/27，Rust 17 通过 / 1 默认忽略，格式化、类型检查、生产构建与 NSIS 打包通过。
- 浏览器 18 组（9 状态 × 2 模式）检查顶部切换、底部状态与刷新按钮；部分失败独立提示，刷新中及未连接禁用按钮，布局无横向溢出。
- 安装版真实点击：额度和 Token 成功读取时间均更新，两个窗口同步刷新中，三次连续点击只产生一次读取，完成后恢复按钮。两种模式截图检查通过。
- 键盘展开/收起和单轮原生边缘拖动/长按/移出收起通过；连续脚本曾失败，不作为连续通过证据。多屏跨 DPI、长时间运行及人为真实来源故障未测。

## 2026-10-16 来源日与读取语义修正

- npm test 31/31，TypeScript/Vite、Tauri release/NSIS 通过。新增日期测试覆盖本地零点、时区差异、跨年、来源超前、当天零值与缺失。Rust 代码未改，未重跑 Rust 单元测试。
- 浏览器 12 组日期/模式组合：日期标题、缺失提示、底部信息均完整，无用量区域横向溢出。
- 安装版真实手动读取成功后，10/15 来源记录仍保留，10/16 缺失提示在两种模式下保持，显示“刚刚读取”；主窗口摘要和详情来源日/读取时间一致。连续点击去重与共享忙碌状态回归通过。
- 未验证来源会在何时提供 10/16 记录；没有模拟为真实今日数据，没有修改计数/日期。原生拖动专项本轮未重跑。最终正常重启，调试已关闭。

## 2026-10-16 本机今日统计验收

- JS/TS：34/34；Rust：24 通过、2 默认忽略（账号 live contract 和本机日志 live probe）。本机日志 probe 本轮另行显式执行通过；账号专项未另跑，安装版真实账号刷新成功。
- 新测试：北京零点/跨年、无使用零值、旧日期失败不伪造零、健康跨零点归零、输入缓存/输出推理避免重复、累计快照重复、计数重置、复制事件、不同回合区分、合成上下文估计排除、部分行、增量追加、归档移动、重启重建、老 mtime 的今日事件、目录不可读、畸形事件、长会话跨日新响应。
- TypeScript/Vite、cargo fmt、Tauri release/NSIS 打包成功；NSIS 本机安装退出 0。
- .bundle-tmp/verify-local-native.mjs：安装版今日真实数值、用量/额度两模式、无横向溢出、底部可见、主页与详情、本机/账号统计分离、切换洛杉矶时区仍采用北京日、手动按钮忙碌与读数时间推进全部通过。截图已查看。后台额度 available，1 个有效窗口，账号日统计最新日期 2026-10-15 与本机今日 2026-10-16 分离。
- 不以模拟时钟测试代替真实零点驻留验收，尚未等待实际午夜；无新通知/拖动控制器/多屏/长时间休眠专项。新增功能未上传或发布。
- 补充安装态观察：未点击刷新、未发起测试推理的 25 秒观察中，本机计数从 [用量已脱敏] 自动增至 [用量已脱敏]；采样时最近成功读取距当前约 179ms。证明自然使用会自动更新，但这不是日志落盘到 UI 的全链路 SLA 测量。
- 交付收尾：正常启动已安装应用，悬浮窗开关 true、模式 usage 保留；临时 WebView 9223 调试端口确认不再监听。未发布或上传。

### 最终安装验证补充

- 最终 Rust 完整测试 27 通过、2 默认忽略；前端 35/35；fmt、TypeScript/Vite、Tauri release/NSIS 均通过，最终安装包约 3.46MiB。真实日志专项已显式通过。Windows 通知测试使用已刷新到磁盘的写入；未刷盘写入延迟由活跃文件长度检查覆盖，不假称通知一定及时。
- 双槽索引已在实际安装目录写入，验收时 a/b 各约 306KB，两份合计约 0.59MiB；C 盘没有复制原始日志。旧 .tmp 失败产物已定点清理。重启前两槽均存在，重启后用量仍为正确北京日期、真实数据继续增长，双模式/主页/详情/手动刷新/账号额度均通过。重启验收脚本曾过早连接尚未注入 Tauri 桥的 WebView，已改为等待 URL、桥与 React 界面就绪，再跑通过。
- 断开时目录请求取消并等待完成；退出进程时系统释放其句柄，清除本工具数据会删除 a/b 索引及旧临时文件；没有删除或修改 Codex 会话。异常中断写入可回退另一槽的逻辑已用损坏最新槽测试验证，未做断电破坏测试。
- 最终版本资源采样：30.09 秒、8 个进程（TokenDaily + WebView 等子进程），CPU 总计 1.1094 秒，单核等效 3.69%、按本机逻辑处理器数折算整机约 0.23%；工作集相加 548.0MiB，含共享页重复，不等于独占物理内存。采样含自然使用，不能当作长期上限或与旧版的严格对照实验。
- 交付完成：正常重启后应用在运行，floating_enabled=true、mode=usage，临时 WebView 9223 端口确认未监听。未发布或上传。仍未做真实断电、跨多设备、长时间休眠与实际午夜驻留专项。

## 2026-10-16 贴边隐藏

- npm test：37 通过；cargo test --offline --manifest-path src-tauri/Cargo.toml：28 通过、2 默认忽略；cargo fmt、TypeScript/Vite、Tauri release/NSIS 成功，已安装最终包。
- 新增测试覆盖：旧设置默认关闭、开关持久化不改变其他设置、左右吸附阈值按缩放计算、顶部不吸附、负坐标工作区、缩窄/展开/恢复球体均在工作区内。既有 hover/拖动失去 mouseup/leave 恢复测试通过。
- 安装态初轮左右各 3 次悬停展开/离开收回、开关关闭恢复、展开面板拖离边缘通过。最终版开关重启保留、界面激活/Escape 3 轮、关闭开关恢复 70×70 球体与设置页同步通过。设置/窄边截图位于 .bundle-tmp/edge-settings.png、edge-handle.png，已目视检查。
- 整套原生鼠标连续脚本未完整通过：有回拖球体未移动；末轮记录指针已从目标 (6,390) 移到 (332,660)，影响悬停断言。没有将界面事件测试当作真实拖动测试；未宣称所有桌面输入环境与多屏 DPI 验收通过。完整记录见 PROGRESS.md。

## 2026-10-16 点击展开 / 半 Logo 贴边交互

- 新增交互测试先因实现缺失失败；实现后 `npm.cmd test` 42/42 通过。TypeScript/Vite、Tauri release 与 NSIS 构建通过，原地安装退出 0。Rust 业务代码未改，本轮未重跑 Rust 单元测试、账号/日志接口与真实通知专项。
- 125% DPI 的 Windows 安装版原生鼠标专项完整通过：普通球悬停不展开、单击展开、移开收起；球体拖动移动且松手不展开；左右边缘分别以 35×70 物理像素窗口显示半 Logo；右侧两轮/左侧一轮悬停展开与离开收回；半 Logo 直接拖离恢复球体；关闭贴边隐藏后悬停不展开。
- 首轮点击未通过，核对原生输入事件后点击成功。调整脚本在点击前再次定位鼠标后，完整脚本通过；未把最初失败武断归因为用户或产品。脚本 `.bundle-tmp/verify-click-dock.ps1`，截图 `.bundle-tmp/half-logo-left.png` 与 `half-logo-right.png` 已目视检查。
- 原有开启/贴边/用量模式设置已恢复，安装版正常重启，临时 9223 调试端口关闭。多显示器跨 DPI、触摸与长期休眠未测。

## 2026-10-16 仅拖动至边界才吸附

- `npm.cmd test` 43/43；TypeScript/Vite、cargo fmt、Rust 测试 28 通过/2 默认忽略、Tauri release/NSIS 构建通过；安装退出 0。
- 新测试先失败后通过，覆盖鼠标精确左右边界、附近位置不吸附、负坐标工作区、任务栏范围、显式拖动意图、重复结束、取消/未移动/关闭开关。
- Windows 125% DPI、1920×1020 工作区原生拖动专项通过：右侧 x=1885 和左侧 x=35 松手保持完整球及原位；x=1919 / x=0 松手吸附为半 Logo；拖离恢复；附近切换设置不吸附。确认实际原生拖动结束坐标与目标一致。脚本 `.bundle-tmp/verify-edge-trigger.ps1`。
- 旧整套脚本点击环节受鼠标离开目标影响未完成；首轮专项右侧附近落点也有干扰，已重跑并检查真实落点。未将失败样本计入通过结果。
- 正常重启并关闭临时调试；原设置保留。多屏跨 DPI、侧边任务栏、触摸及原生 Escape 取消拖动未专项验收；真实接口/日志忽略测试本轮未执行。

## 2026-10-16 按住触边即时吸附

- 测试先行后实现：新增 3 个 Rust 拖动矩形测试与前端 snapped 状态校验。最终前端 43/43，Rust 31 通过/2 默认忽略，fmt、TypeScript/Vite、Tauri release/NSIS 通过。
- 最终 Windows 安装版（125% DPI、1920×1020 工作区）完整原生鼠标专项通过：附近未松手不吸附；右侧按住时窗口已贴齐 1850..1920；左侧按住时窗口已贴齐 0..70；两侧继续往回拖均解除吸附；再次触边后松手立即移开均隐藏为 35px 半 Logo。使用 GetAsyncKeyState 断言当时仍按住鼠标，脚本 `.bundle-tmp/verify-live-snap.ps1`。
- 初轮未进入拖动/按下中断不计通过；后续发现快速松手移开丢失隐藏，补充最后一次实时吸附状态保留与松手后移动消息保护，再打包安装后完整通过。
- 最终安装退出 0，原设置保留；已正常重启，临时调试端口关闭。多屏跨 DPI、侧边任务栏、触摸、真实 Escape 取消未验收；本轮未运行默认忽略的账号/日志接口专项。

## 2026-10-16 球体碰边即时吸附（最终验收）

- 验收条件改为可见球体碰左右工作区边界，鼠标无需到最外侧。原生 snapSide 直接交给前端；拖动抓取点在整次拖动中保持固定，避免 Windows 贴齐后的矩形影响后续触边判断。
- 测试先行：球体触边旧实现失败，固定抓取点回归测试先因实现缺失失败；最终前端 43/43、Rust 33 通过/2 默认忽略，fmt、TypeScript/Vite、Tauri release/NSIS 通过，安装退出 0。
- 实机发现并修复：早先版本再次碰边后结束事件 snapSide=null，未收成半 Logo；固定抓取点后原生专项完整通过。125% DPI、1920×1020 工作区下，右侧鼠标 x=1905、左侧 x=20（屏幕最外侧为 1919 / 0），按住时分别贴齐 1850..1920 / 0..70；垂直移动保持、回拖解除、再次触边后松手立即移开均收成 35px 半 Logo，结束事件分别 right / left。脚本 `.bundle-tmp/verify-ball-contact.ps1` 检查真实按键状态和指针位置。之前的指针偏离、首轮起拖按键中断均未计作通过。
- 已正常重启安装版并关闭临时调试端口，原设置保留。多屏跨 DPI、侧边任务栏、触摸、真实 Escape 取消未验收；此次未重跑完整点击/悬停专项及默认忽略的账号/日志测试。真实数据来源不变，无新增演示数据。

## 2026-10-16 拖动未松手即显示半 Logo

- 用户明确更新时序：拖动触边即吸附并半隐藏，再松手。新增实时吸附事件与半 Logo 裁切；松手保护防止立即悬停展开，回拖恢复完整球。
- 新测试先因缺少实现失败；最终前端 44/44、Rust 33 通过/2 默认忽略，fmt、TypeScript/Vite、Tauri release/NSIS 通过。最终安装器退出 0，安装版正常重启，调试端口关闭。
- 原生脚本 `.bundle-tmp/verify-held-half.ps1` 完整通过：附近不吸附；左右球体碰边、鼠标未到最外侧时仍按下，DOM 球体实际可见宽为 24/48 CSS 像素；截图前后按键保持按下；沿边移动保持半 Logo，回拖恢复完整球，再次触边并松手后鼠标停留 700ms 仍半隐藏，移开仍保持。按住时左右截图已目视核对。
- 补查真实鼠标：离开后重新悬停展开为 375px 面板，移开恢复 35px 半 Logo。测试环境为 125% DPI、1920×1020 工作区。拖动期间原生抓取面保持原尺寸，视觉已半隐藏；松手后收窄实际窗口。原设置和数据来源保留。
- 未验收多屏跨 DPI、侧边任务栏、触摸、真实 Escape 取消；默认忽略的真实账号/日志测试未运行。

## 2026-10-17 Intel Mac 首轮验证

基线 `9f0fc4c`，分支 `mac/initial-validation`；本轮没有修改应用运行源码或两份锁文件。系统为 Intel macOS 15.7.7（24G720），Xcode 26.3，工程内 Rust/Cargo 1.99.0，Node 24.19.0。

| 检查 | 结果 |
|---|---|
| `npm ci --no-audit --no-fund`、`npm test`、`npm run build` | 安装与构建成功，44 项规则测试通过 |
| `bash scripts/mac-dev.sh cargo fmt --manifest-path src-tauri/Cargo.toml --check` | 通过 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml` | 27 通过、2 个真实来源默认忽略；Mac 目录事件尚未实现对应的 `Changed` dead_code 警告保留 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml live_ -- --ignored` | 2/2 通过：真实额度/账号日桶，本机日志首次读取、增量及索引恢复 |
| `node scripts/probe-codex.mjs` | stdio 初始化与三项只读接口完成，返回有效额度和合法账号日桶；没有输出原始响应/身份 |
| `bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` | 发布模式 Intel App/DMG 生成成功；12.21 MiB / 3.82 MiB |
| `hdiutil verify src-tauri/target/release/bundle/dmg/TokenDaily_0.1.0_x64.dmg` | 校验成功 |
| Mac 浏览器 | 420px 六种演示状态正常；360px 非演示主页/设置无溢出。开发演示横幅有 5px 溢出，已记录未改 |
| 原生 App（AX 树及交互） | 启动、未连接状态、设置、悬浮功能禁用、真实数据展示、详情来源区分、手动刷新、关闭驻留、重复启动恢复且仅单实例、断开及正常退出通过 |
| CLI 发现反例 | 用最小 PATH 经 LaunchServices 启动，实际显示“未找到本机 Codex CLI”；只为一次启动提供现有 CLI 路径后读取成功。不是 Finder 双击的完整验收 |

原生真实数据来自本机已有登录和日志，没有使用演示数据或发送模型推理。原生检查没有屏幕录制权限，不能宣称完成截图/像素级视觉验收。App 从构建目录运行，未安装到 Applications，未签名、公证或发布；结束时已停止本工具采集并退出。

仍待：正常 Finder 启动的稳定 CLI 发现与来源闭环、菜单栏图标视觉/交互、系统通知送达/点击、登录启动、清除本工具数据、多屏/Retina/Dock/Spaces、真实午夜与长期运行、Apple Silicon，以及 Windows 本轮回归。额度与官方 UI 的同身份同时刻对照、工作区身份隔离、账号日桶时区/范围未因本轮测试得到确认。

## 2026-10-17 Mac 来源发现与手动设置

分支 `mac/codex-discovery`，基线 `4d63bce`，同一 Intel macOS 15.7.7；CHG-20261017-03。新增共享来源配置、请求轮次隔离、来源缓存包装和设置 UI；Mac 查找规则独立放在平台目录。依赖与锁文件未改。

| 检查 | 结果 |
|---|---|
| 测试先行 | 新路径测试首次执行因尚未实现 resolver/search_directories 编译失败；实现后通过 |
| `npm test` | 44/44 通过 |
| `npm run build` | TypeScript / Vite 通过 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml` | 35 通过、2 默认忽略。覆盖显式优先/失效不回退、权限、相对路径、空格、符号链接、标准候选、目录校验、子进程环境、旧设置兼容、来源指纹和过期请求 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml live_ -- --ignored` | 2/2 通过，真实账号契约与本机日志；未输出原始日志、身份和余额 |
| `bash scripts/mac-dev.sh cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`git diff --check` | 通过 |
| `bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` | App 12.28 MiB、DMG 3.83 MiB；`hdiutil verify` 通过 |
| Finder 启动 | 由 AppleScript 给 Finder 发送 `open POSIX file`，打开构建目录 App。确认子进程环境没有 TOKENDAILY_CODEX_EXE/CODEX_HOME 注入；初始未发现 CLI，手动指定并保存现有 CLI 后真实额度、账号日统计和本机今日用量正常，无演示标记 |
| 手动刷新/重启 | 刷新后成功时间推进；退出后再次由 Finder 打开，沿用已保存路径并重新读取真实数据 |
| 来源隔离 | 改为项目忽略目录中的空测试 home：旧额度缓存删除，账号显示需授权，日志显示读取失败/暂无数据；恢复默认 home 后真实数据恢复 |
| 路径失效 | 含空格的测试 CLI 符号链接可运行；退出后只删除该自建链接，再启动明确报程序无效，不回退、不显示旧额度。原 CLI 未移动/修改 |
| 输入/保存失败 | 不存在程序和目录被拒绝；临时把本工具 settings.json 改只读，保存失败消息可见、原内容不变、没有错误成功提示；随后恢复原权限 |
| 读取中断连 | 手动刷新后立即断连，确认仍忙时连接按钮禁用；读取结束后仍为断连，迟到结果没有重新发布 |

原生检查使用既有 AX 权限与键盘事件，只操作 TokenDaily；未开屏幕录制，未做原生截图/视觉验收。最初仅 AXSetValue 不触发 React 输入事件，不计作有效输入检查；改用聚焦和键盘事件后上述输入检查通过。最终保留已验证 CLI 的本地路径配置、默认 home，恢复断连并正常退出。测试空 home 不含复制的凭据；没有模型推理、登出、共享配置修改或原始日志上传。

标准 CLI 安装位置来自隔离文件系统夹具，当前机器没有独立安装，不能把该测试称为 Homebrew/npm 安装态验收。手动指定目前是粘贴绝对路径，没有原生文件选择器。Windows 共享回归、Apple Silicon、/Applications 安装、人工鼠标双击、签名公证未运行；原生监听、通知送达及悬浮球仍待后续。保留现有 Mac `Signal::Changed` 未使用警告。

## 2026-10-17 共享身份与提醒提交可靠性

分支 `shared/identity-alert-reliability`，基线 `5691ee5`，CHG-20261017-04。运行环境仍为 Intel macOS 15.7.7；没有 Windows/Apple Silicon 验收。

| 检查 | 结果 |
|---|---|
| 测试先行 | 新身份与提交故障用例先因未实现 identify/Delivery 编译失败；实现后通过。旧身份继承测试随后因新增核验门槛失败，修正夹具为明确已核验，并补未核验不继承断言 |
| `npm test` | 39/39 通过。删除了未被产品调用的 TypeScript 提醒算法及其 5 个重复测试；相应用例由 Rust 实际运行模块覆盖，不再把前端通过当作后端提醒正确的证据 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml` | 45 通过、2 个真实来源默认忽略；包括工作区/来源隔离、协议读取期间切换、旧缓存降级、失败重试/冷却、恢复证据、局部恢复、周期/身份切回、预留失败、提交后持久化失败、双槽损坏恢复 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml live_ -- --ignored` | 2/2 通过；真实读取断言工作区已核验、脱敏身份、额度和账号日桶，以及本机日志首次/增量/恢复 |
| `node scripts/probe-codex.mjs` | 真实响应顶层 workspaceRouting 的 ID 与 origin 均存在；只输出字段存在性和结构，无原始标识/凭据 |
| `cargo fmt --check`（通过 mac-dev.sh、指定 manifest）、`git diff --check` | 通过 |
| `npm run build` / `bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` | TypeScript/Vite、Intel App 12.36 MiB、DMG 3.87 MiB 通过；`hdiutil verify` 通过 |
| 原生 AX 复核 | Finder 打开构建 App，沿用已保存来源连接；真实额度可用、identityVerified=true、额度与账号日桶同一 v2 身份。详情显示该快照的工作区核验与账号日统计范围限制；设置显示提醒提交状态，无“已送达”承诺。结束时断连并正常退出 |

单测中的身份、低额度、恢复与存储故障均为隔离夹具，不作为真实额度或系统通知发送。没有切换用户实际账号/工作区、推理请求、登录/注销、复制凭据或修改 Codex 配置。新增前后核对只能检测接口可观察到的身份变化，不能声称多次接口读取为服务端原子事务。

关键边界：当前 Tauri 通知插件在后台异步提交；同步返回错误会重试，但后台系统失败、用户关闭通知、免打扰及实际展示无法从 show() 得知。成功状态严格命名为 submitted；未验证实际系统送达/点击。发生崩溃或成功提交后的记录保存失败时，可能漏一条但不自动重放不确定提醒，不能同时保证必达和绝对不重复。损坏或不可读账本启动时暂停通知，修复后重启或清除本工具数据才能重新初始化；写盘故障可在后续有效刷新时恢复。

旧邮箱身份不能可靠映射到工作区；新身份首次有效低额度可能提醒一次，旧无核验信息的快照不恢复。Windows 共享回归、正式安装态、官方 UI 同时对照、原生通知授权/送达、签名公证仍未做。现有 Mac `Signal::Changed` 未使用警告保留。

## 2026-10-17 Mac 原生日志目录监听

分支 `mac/native-log-watch`，基线 `1e9ad03`，CHG-20261017-05；Intel macOS 15.7.7。

| 检查 | 结果 |
|---|---|
| 先写原生事件测试 | 旧实现失败：创建日志后返回 Timeout，预期 Changed；实现后通过 |
| `npm test` / `npm run build` | 39/39；TypeScript/Vite 通过 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml` | 50 通过、2 默认忽略；新增 5 个 Mac watcher 测试，旧 Mac Changed 未使用警告消失 |
| 原生文件系统事件 | 本地临时目录：缺失日志根后创建、深层新文件、追加、归档移动、文件原子替换、根目录移走再创建、停止/重绑；不相关 settings 文件和旧来源写入不唤醒 |
| 状态及故障夹具 | 1000 次回调合并；手动唤醒优先；Rescan 信号；合成后端错误可见；无效目录回退及恢复；迟到错误/事件不能污染新代次；周期重置复用流、Drop 后回调 Arc 释放 |
| Collector 集成 | 原生事件唤醒后运行共用 Collector：合成计数 100→150→归档后仍 150；不更改解析或去重算法 |
| 路径边界 | 日志根符号链接不订阅外部目录，外部写入不唤醒；没有复制真实日志或凭据 |
| `bash scripts/mac-dev.sh cargo fmt --manifest-path src-tauri/Cargo.toml --check` / `git diff --check` | 通过 |
| 原生 App | 构建目录 App 经 Finder Apple Event 打开未连接首页；再次执行二进制唤回后 AX 连接，真实额度 available、identityVerified=true、本机 Token 显示，无演示/监听降级说明；随后断连、正常退出 |

本轮两个 `live_ --ignored` 专项没有重跑；前述真实数据是 App 基础复核。没有在原生 UI 人为注入监听失败；降级文案沿用既有 UsageState.message 展示，状态故障由 Rust 夹具覆盖。FSEvents 测试未计时承诺端到端毫秒延迟，500 ms 是共用调度合并窗口。

打包命令 `bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 首次在 DMG 卸载时失败（资源忙，App 编译完成）。识别并普通卸载本次临时映像、退出应用后重试；最终产物校验另记下行。未在签名、安装态、Windows、Apple Silicon、网络盘、睡眠恢复或长期运行中验证；不外推为完整 Mac 可交付。

最终重试成功：Intel App 12.48 MiB、DMG 3.92 MiB；`hdiutil verify src-tauri/target/release/bundle/dmg/TokenDaily_0.1.0_x64.dmg` 校验通过。构建目录 App 为本地未签名验证产物，未安装到 /Applications 或发布。运行开发版：`bash scripts/mac-dev.sh npm run tauri -- dev`；也可打开 `src-tauri/target/release/bundle/macos/TokenDaily.app`。

## 2026-10-17 Mac 系统通知授权与提交反馈

分支 `mac/native-notifications`，基线 `dd5fe6f`，CHG-20261017-06。Intel macOS 15.7.7；macOS 最低构建声明调整为 11.0（不是对全部 11+ 系统的验收结论）。

| 检查 | 结果 |
|---|---|
| 测试先行 | uncertain 账本测试先因未实现类型失败；实现后通过；系统 NSString 常量比较曾出现类型歧义，改用明确解引用后编译通过 |
| `npm test` | 42/42：新增 3 项验证未知/拒绝/静默/通道关闭，以及接受、通知中心记录、缺失和点击证据的文案边界 |
| `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml` | 58 通过、2 真实来源默认忽略；新增 8 项（含签名失败保护）覆盖 uncertain 后重启去重、授权及显示通道、原生枚举、未打包二进制保护、回调拒绝/超时、单工作线程合并、过时/失败/断连快照不得发送 |
| fmt / diff | `bash scripts/mac-dev.sh cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`git diff --check` 通过 |
| TypeScript/Vite | `npm run build` 通过 |
| 原生权限实测 | 初版显示“尚未申请”，用户批准后申请失败且无弹窗。通知服务拒绝未签名进程，之前 not-determined 是默认值，不能算有效权限读取。新增签名校验保护 |
| 测试记录初始状态 | 点击“检查测试通知”返回本次运行没有可检查记录；未发送任何测试通知，未改变真实额度账本 |

`get_notification_permission` 使用真实 UserNotifications 查询；权限枚举与失败/超时发送测试则为隔离单测，不能视为实际系统发送。未运行本轮两个 ignored 账号/日志专项；没有修改 Codex 凭据、原始日志或共享配置。

用户已明确允许通知权限和测试，已点击申请；macOS 拒绝未签名应用的请求，无授权弹窗，未发送通知。已用只读 codesign 与系统日志确认身份问题，未添加任何 Apple 私有 entitlement。实际授权、真实提交回调、通知中心匹配与实际点击回调需签名后继续验收；这不是“Mac 通知已完整通过”。通知中心匹配只能证明系统列表中存在本次记录，不能证明横幅出现或已被阅读。

运行方式：`bash scripts/mac-dev.sh npm run tauri -- dev` 用于开发；系统权限与测试按钮在开发模式禁用，需打开经授权签名的 `src-tauri/target/release/bundle/macos/TokenDaily.app`，未签名构建明确提示不可用。未做 /Applications 安装、签名/公证、Apple Silicon、Windows 编译/安装态、专注模式组合或最低 macOS 11 实机验证。

收尾：Mac 通知类名使用 TokenDailyNotificationDelegate，避免通用 Objective-C 类名冲突；共享命令错误文案改为平台中性的“打包后的 TokenDaily”。修改后原生通知专项 3/3、fmt/diff 检查通过，再次构建最终 App/DMG。通知按钮采用两列布局，AX 可访问标签已验证；本轮没有原生截图，不能将 AX 读取当作像素级视觉验收。

后续修正：新增 Security 框架运行身份检查，先写签名失败归一化测试并确认失败，随后全量 Rust 58 通过/2 忽略。签名校验成功也仅是前置条件，不代表通知授权或送达。

最终未签名构建复验：App 12.73 MiB、DMG 3.97 MiB；`hdiutil verify` 通过。Finder 启动后设置页通过 AX 确认显示“当前应用未签名，无法可靠读取或申请系统通知权限”，申请按钮不出现，未伪报已授权。前端 42/42、Rust 58/2 ignored、fmt/diff、TypeScript/Vite 及 App/DMG 构建均通过。已另行询问仅对构建产物做本地 ad-hoc 签名，未执行签名或安装；真实系统通知提交/送达/点击继续待验。

## 2026-10-17 获授权后的 Mac 通知闭环实测

本节接续并覆盖上一节“签名待确认”的状态。代码为 `5b44e72`，Intel macOS 15.7.7，构建目录 App。用户已单独授权本地临时签名与通知验收。

| 检查 | 实际结果 |
|---|---|
| 本地签名 | `codesign --force --sign - src-tauri/target/release/bundle/macos/TokenDaily.app` 完成；`codesign --verify --strict --verbose=2` 通过；Signature=adhoc，无 TeamIdentifier |
| 权限读取/开启 | 签名后先得到 denied；系统设置中 TokenDaily 的“允许通知”从 0 开启，应用刷新为允许；未改其他应用/全局通知设置 |
| 系统提交回执 | 仅发送一条标注测试的提醒，真实系统完成回调返回 accepted |
| 通知中心 | 应用检查返回 in-center；系统 AX 树识别 TokenDaily 测试标题和正文 |
| 点击 | 展开系统分组后点击实际测试条目，应用检查返回 clicked；主面板在屏幕上且应用 isActive=true |
| 权限持久性 | 正常退出、Finder 重开后仍显示系统允许；不自动申请或再次发送 |
| 收尾 | Codex 保持断连，App 正常退出，保留获授权通知权限；没有改变真实额度/提醒账本 |

边界：菜单 Close Window 未使主面板离开屏幕，因此未将本次点击验收写成“隐藏后恢复”；只验证了真实点击回调与前台激活。首次授权弹窗、退出后点击冷启动、横幅像素显示/专注模式组合、/Applications 安装、Apple Silicon、Windows 和 macOS 11 仍未验证。AX 读取不是截图视觉验收。

这轮没有产品源码变更，未重复单测/构建；沿用同一代码的前端 42/42、Rust 58 通过/2 ignored、fmt、TypeScript/Vite 与 App/DMG 构建结果。已签名的是 `src-tauri/target/release/bundle/macos/TokenDaily.app`，原 `TokenDaily_0.1.0_x64.dmg` 仍含未签名 App；没有重新制作 DMG、公证或发布。

## 2026-10-17 通知点击的隐藏恢复与冷启动补验

本节补齐前节明确保留的两项窗口状态验收。Intel macOS 15.7.7，源码 `5b44e72`、文档起点 `f27a305`，同一构建目录的 ad-hoc 签名 App；未改代码、未重新打包。

| 场景 | 点击前的实证 | 点击后的实证 | 结果 |
|---|---|---|---|
| 隐藏主面板 | 第一条测试请求 accepted；真实窗口关闭按钮 AXPress 成功；CoreGraphics 中本应用 layer=0 主窗口数 0，进程保留 | 同一进程主窗口数 1、420×621、isActive=true；检查测试通知返回 clicked | 本机通过 |
| 完全退出 | 第二条测试请求 accepted/in-center；Quit 后 NSRunningApplication 查询无 TokenDaily 进程 | 从系统通知中心点击实际条目后出现新 PID；主窗口数 1、420×621、isActive=true | 本机通过 |
| 冷启动后测试状态 | 上一个进程已退出 | 检查显示本次运行无可检查测试通知，不继承旧 ID；通知权限仍允许，Codex 仍断连 | 符合当前进程内诊断设计 |
| 签名复查 | 已有本地临时签名 | `codesign --verify --strict --verbose=2` 成功 | 通过 |

通知中心里先点击分组只会展开，随后点击实际测试条目才触发恢复/启动；两步分别确认，未用手动启动代替通知点击。只读取和操作本工具的测试条目。本轮共发送两条明确标注的测试提醒，未操作真实额度、账户、原始日志或提醒账本。收尾正常退出 App，保留获授权通知权限。

未改产品源码，因此本轮未重复单测或构建；先前同一源码的前端 42/42、Rust 58 通过/2 ignored 结果仍为最近测试。仍未验证首次授权弹窗、安装/移动路径、发行签名/公证、Apple Silicon、Windows、横幅像素显示/专注模式组合及真实额度触发。冷启动检查只证明系统从本机构建目录启动并显示 App，不代表测试记录跨进程持久化。原 DMG 仍未签名。

## 2026-10-17 CHG-20261017-07 Mac 悬浮球与菜单栏

分支 mac/floating-panel，起点 0f4145b。Intel macOS 15.7.7、单屏 1680×1050 逻辑坐标、2× Retina；构建目录 App，核对实际 bundleURL，不以已安装旧版替代。

| 检查 | 结果 |
|---|---|
| 测试先行 | 几何与 Retina 测试先缺少函数而失败，再实现；修正尚未接触边界的负屏夹具 |
| npm test | 42/42，共用点击/拖动/触边/悬停与数据规则 |
| cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml | 62 通过、2 真实来源默认忽略；新增 4 项几何/Retina 回归 |
| fmt / diff / TypeScript / Vite | 全部通过；Mac 窗口覆盖数组除 acceptFirstMouse 外与公共定义完全一致 |
| Tauri build --no-sign -- --locked | App 12.83 MiB、DMG 3.99 MiB；hdiutil verify 通过 |
| 本地 App 签名 | 按已有授权 ad-hoc 签名；codesign --verify --strict 通过；DMG 内仍未签名 |
| 普通球 | 56×56；悬停保持收起，点击为 300×250，移开收起 |
| 左右触边 | 按住且指针未到屏幕边界时预览半 Logo；回拖解除；松手为 28×56 且保持，离开再悬停展开 |
| 拖离/取消 | 半隐藏直接拖离恢复完整球；Escape 返回拖动起点 |
| 设置/重启 | 用量/额度切换、开关及贴边设置重启保留；位置和吸附状态不保存，重启为完整球 |
| 菜单栏 | 左键主面板显示/隐藏；右键四个入口；设置显示实际设置页，退出后进程消失 |
| 收尾 | 恢复悬浮关闭、贴边关闭、用量模式；Codex 保持断连，正常退出 |

辅助程序使用开发机已有 AX/输入权限；产品的本地监听不需要这些权限，不安装全局钩子。脚本/构建/签名材料未入 Git。本轮显示断连的未知数据，不是演示余额；未运行账号/日志 ignored 专项，不改变真实提醒账本。已安装的 /Applications/TokenDaily.app 保持原样。

未验证：透明边缘/图标的截图像素外观、多屏混合缩放、Spaces/全屏应用、Dock 换边/自动隐藏、最低 macOS 11、Apple Silicon、Windows 本轮编译/安装、长时驻留、正式安装/发行签名/公证。现有辅助访问证据不等于视觉通过，不宣称两端同步完成。

### 2026-10-17 单屏边界、窗口恢复与本应用全屏补验

同一产品代码 `918ff99`，构建目录 App 的现有 ad-hoc 签名严格校验通过。实际运行 bundleURL 核对为本仓库构建路径。单屏 1680×1050、2×；AppKit visibleFrame 转为桌面坐标后为 x=0、y=25、width=1680、height=966，底边 y=991。

| 检查 | 证据与结果 |
|---|---|
| 底部拖动 | 原生按住拖到 y=1040，松手后球为 (812,935)、56×56，底边 991，位于 Dock 上方 |
| 底部展开/收起 | AXPress 展开为 (812,741)、300×250，底边仍为 991；鼠标移开后回到 (812,935)、56×56 |
| 顶部拖动 | 原生按住拖到 y=5，松手后球为 (812,25)、56×56，未覆盖菜单栏 |
| 关闭主窗 | 真实关闭按钮使屏幕 layer=0 主窗数为 0，进程保留；悬浮球能经 AXPress 展开并通过“打开主面板”恢复主窗 |
| 本应用全屏 | 主窗 AXFullScreen 从 0→1、尺寸 1680×1050；CoreGraphics 屏幕列表仍有 layer=5 的 56×56 悬浮球，AXPress 后为 300×250 |
| 退出全屏 | AXFullScreen 回到 0，主窗恢复 (630,198)、420×621；悬浮窗移开收回 56×56，无残留全屏状态 |
| 视觉 | 用户针对本轮构建的透明边缘/无白框和菜单栏图标回复“显示正常”；这是人工目视确认，没有截图或自动像素采样 |

普通鼠标点击底部球的一轮未得到稳定展开观测，其中一次按下/松开之间指针实际位置变化；该轮不计通过，也不归因于产品缺陷。后续用 AXPress 独立验证边界布局，未冒充新的物理点击专项；普通物理点击最近通过记录仍是上一节。

开始时已有 `/Applications/TokenDaily.app` 在运行，其二进制 SHA-256 与本轮构建不同。先正常退出，再启动并核验构建路径；结束退出构建 App 并恢复原安装版，核验 bundleURL。原悬浮开启、贴边开启、用量模式、Codex 断连保持，右侧半隐藏位置恢复为 (1652,357)、28×56；未覆盖已安装文件。

没有改变产品代码、依赖、系统 Dock/Spaces 配置或通知设置，没有连接账户或发送通知。本轮不重复单元测试/构建；沿用 `918ff99` 对应前端 42/42、Rust 62 通过/2 默认忽略的最近结果。文档运行 `git diff --check`。辅助程序仍在忽略目录。

本应用全屏通过不代表其他应用全屏或不同桌面间跟随通过；多屏、Dock 换边/自动隐藏、睡眠唤醒、长期运行与 Windows/安装态仍未验证。后续步骤见 [REGRESSION_HANDOFF.md](REGRESSION_HANDOFF.md)。

## 2026-10-17 授权替换安装版与安装路径通知验收

用户明确授权“允许替换并继续验收”。产品代码仍为 `918ff99`，本轮起点 `3475c7b`；Intel macOS 15.7.7、单屏 2×。使用已经验收并 ad-hoc 签名的构建 App，未重新构建或签名；不是从现有未签名 DMG 安装。

- 安装前复核候选、原版及旧版归档 SHA-256；旧归档 unzip 完整性和归档内主二进制校验已通过。原安装版正常退出至无进程。ditto 先复制候选到 /Applications 隐藏临时目录，严格签名与主二进制校验通过后替换目标。
- 原 bundle 移至忽略目录 `.bundle-tmp/mac-validation/install-backup-20261017-131544/original-bundle`（无 .app 扩展名），同目录保留 `TokenDaily-installed.zip` 与校验 manifest。备份未删；没有覆盖或清除用户设置/缓存/通知账本。
- 最终 `/Applications/TokenDaily.app` 严格签名校验通过，主二进制 SHA-256 为 `97cc06924286abbfce7f6324a46d3f23a2e622fe32c702e772c843841b433732`，与已验候选一致。

| 项目 | 实测证据 | 结果 |
|---|---|---|
| 安装路径启动 | `open /Applications/TokenDaily.app`；实际 bundleURL 为该路径，PID 12253，主窗 420×621 在屏幕上且激活 | 通过（Launch Services 打开，非人工 Finder 双击） |
| 单实例 | 再直接执行安装版二进制，第二次调用正常退出；运行实例数仍为 1，PID 仍 12253 | 通过 |
| 设置与权限 | 悬浮开启、贴边开启、用量模式保持；系统权限显示允许，Codex 仍断连 | 通过，非首次授权测试 |
| 第一条测试提醒 | accepted；后续检查得到 clicked，但回调已先于受控隐藏验证发生 | 仅证明安装版提交/点击回调，不算隐藏恢复证据 |
| 第二条测试提醒 | accepted、in-center；真实关闭按钮后主窗数 0；点击通知分组后仍为 0；再点实际条目后同 PID 主窗数 1、激活，检查返回 clicked | 隐藏恢复通过 |
| 第三条测试提醒 | accepted、in-center；正常退出后无进程；点分组仍无进程；点实际条目后新 PID 12447，bundleURL 确认为 /Applications/TokenDaily.app，主窗在屏幕上且激活 | 安装路径冷启动通过 |
| 冷启动后状态 | 系统权限仍允许、开关/模式保留；检查提示本次运行无可检查测试通知 | 符合测试 ID 仅存当前进程的设计 |
| 安装版浮窗 | 原生鼠标点击为 300×250，移开为 56×56；拖至右边后为 28×56，停于原使用位置 (1652,357) | 本次基础交互通过 |

共发送三条明确标注的测试提醒；没有连接账户、发起推理、伪造额度或触发真实额度通知。权限沿用此前用户允许的状态，没有改专注模式或其他系统通知设置。结束时保留新版安装 App 正常运行，原悬浮/贴边开关、用量模式和断连状态不变。

本轮只更新文档，未重复单测/构建，最近源码验证仍为前端 42/42、Rust 62 通过/2 默认忽略。`git diff --check` 通过；备份、辅助程序和产物在 Git 之外。未推送/公证/上传/发布。

仍未验：首装通知授权弹窗、全新下载的 Gatekeeper/公证流程、卸载/移除、登录启动、真实低额度/恢复事件、专注模式组合、多屏/Dock 换边/其他应用全屏与多桌面/睡眠唤醒/长期运行、Apple Silicon、Windows 本轮回归。安装路径通过不表示完整发行或双端交付通过；原 DMG 仍含未签名 App。

## 2026-10-17 Git bundle 交接流程验证

本机以 `a53d725` 试包验证交接流程：完整分支 bundle 在临时空仓库执行 `git bundle verify`、`git fetch --no-tags <bundle> refs/heads/mac/floating-panel`、`git switch -c win/regression-20261017 FETCH_HEAD`、提交号/文件树比对及 `git fsck --full --no-reflogs`，全部通过；检出干净，145 个跟踪文件中的两份锁文件、Windows 配置及 icon.ico 均在。包可不依赖 GitHub 导入。

ZIP CRC 与逐文件 SHA-256 校验通过。最终包包含接手文档更新后的快照，按同一流程重新生成验证；manifest.json 写入该快照提交、文件树、bundle SHA-256 和各验证结果，ZIP 同级另有整体 .sha256 文件。产物仅保存在忽略的本地 windows-handoff 目录，未上传。该验证在 macOS 执行，不等于 Windows PowerShell/编译/安装或实机验证；未重复产品单测和构建。

## 2026-10-17 Mac 0.1.3 主线整合与安装态

代码：本节所在 `mac/main-0.1.3-sync` 合并提交（父提交 ee3e1c9 / 1b06e57）。Intel macOS 15.7.7，2× Retina 单屏，既有工具链。

| 检查 | 结果 |
|---|---|
| 前端测试 / TypeScript / Vite | 54/54；生产构建通过 |
| Rust locked + macos-transparent | 87 通过、4 默认忽略；真实来源专项未运行 |
| Release App / DMG | 构建通过；hdiutil 校验通过；App ad-hoc 签名严格验证通过 |
| 三来源总览 | Codex 真实数据；Claude/WorkBuddy 未连接显示未知，统计不完整；未造模拟余额 |
| 悬浮布局 | 300×396，三来源手柄及刷新按钮在窗口内；无旧模式/来源选择；主面板来源切换不改变浮窗 |
| 排序 | 键盘上下移动并重启保留；安装版鼠标把 WorkBuddy 移至首位，窗口位置不变；恢复默认顺序 |
| 原生浮窗 | 安装版球体拖至右侧为 28×56，悬停展开 300×396；恢复原右侧位置；开关均保留 |
| 未实现能力 | WorkBuddy 官方登录和开始免注册试用按钮禁用，显示 Mac 安全存储未实现 |
| 安装与单实例 | /Applications 0.1.3，重复 open 仅同一 PID；候选/安装二进制哈希一致，旧版有 ZIP 和原 bundle 备份 |
| 通知 | 沿用允许权限；本次测试在通知中心查到，隐藏主窗后点击恢复原进程前台面板并收到 clicked |

安装二进制 SHA-256：`b33e56375acd22690432c3201052718e3f0a2114d74fd7d9f1c8f1b98670de89`，代码提交 `64ddf56`。测试只读现有 Codex 数据，没有更改共享配置、发起推理或参榜上传。其余来源真实凭据/数据未验；用户随后目视确认新版主面板和右侧悬浮总览显示正常，三来源长条、底部刷新按钮完整，透明边缘正常。此结论来自用户确认，未进行截图像素采样。Windows 合并代码、Apple Silicon、多屏/长期运行、首次授权与真实额度通知未验；DMG 内未签名，非公开发行物。

## 2026-10-17 Mac WorkBuddy 积分修复

分支 `mac/workbuddy-credits`，Intel macOS 15.7.7。根因是 Mac 服务显式 unsupported，既有积分解析无需更改。

| 检查 | 结果 |
|---|---|
| 前端 | 54/54，TypeScript/Vite 通过 |
| Rust | 90 通过/5 默认忽略；覆盖恢复状态、权限错误与缺失区分、官方 URL 限制 |
| 本机钥匙串专项 | 显式运行 1/1；合成隔离条目保存/读取/更新/删除/重复删除通过，无凭据文件，条目已清理 |
| 官方授权 | 安装版请求成功并打开默认浏览器；用户确认登录完成；真实积分成功返回 |
| 显示与刷新 | 主面板、总览、固定悬浮窗的积分一致；手动刷新有新读取时间 |
| 持久化 | 正常退出重启，沿用钥匙串授权自动重新查询成功 |

未执行真实授权清除以免让用户再次登录；删除能力仅由隔离合成条目验证。未与官方套餐页面人工逐项对照，不把 API 结果称为账单审计。WorkBuddy 本机日志仍未读到，Token 显示未知，与账户积分分离。未运行 Windows 本轮构建/安装、Apple Silicon、钥匙串锁定/拒绝访问真实弹窗及长期运行；没有参榜或推理调用。最终安装产物摘要见 PROGRESS。

最终文案构建 SHA-256 `2c4ba7e140bf34a1a1877f773703afe6a8365dda2cd521caf1302a9cc109cec0` 已安装，代码 `2bc94d4`。跨临时签名升级实测出现系统钥匙串访问确认；仅查看提示文本，未读取或代填密码。用户随后在系统弹窗允许，最终安装版恢复主窗并成功查询官方积分，主面板与悬浮窗数值一致，存储文案正确；安装 bundleURL 为 /Applications/TokenDaily.app。此处验证的是用户允许后的跨签名恢复，不承诺未来临时签名升级无系统确认。

## 2026-10-17 Mac WorkBuddy 非阻塞授权恢复

| 检查 | 结果 |
|---|---|
| 启动状态 | 服务构造不读钥匙串；已连接后后台恢复，先显示“正在恢复授权” |
| 失败与重试 | 自动恢复单次执行；手动刷新才重试；拒绝/锁定不映射为未授权 |
| Rust | 92 通过、5 默认忽略；新增恢复中状态及自动单次/手动重试策略测试 |
| 前端 | 55/55；恢复中与待授权的展示区分测试通过；TypeScript/Vite 通过 |
| App / DMG | `tauri build --no-sign -- --locked`、DMG 校验通过；SHA-256 `09bcef7d10b9b837c1baa3c4c62aabbf289aa1d56bafc6deb5f7a7d36778cddf` |
| 未做验收 | 本轮未替换 /Applications、未触发真实钥匙串弹窗、未在 Windows 回归；正式签名升级体验仍待单独验收 |

## 2026-10-18 Mac 0.1.16 主线整合

- 代码基线：Mac 6fa9e12 + main bea5435，分支 mac/main-0.1.16-sync。83 项前端和 109 项 Rust 测试通过，5 项环境专项默认忽略；TypeScript/Vite、Rust 格式、Intel Release/App/DMG 构建及 DMG 校验通过。
- 安装版 0.1.16：ad-hoc 严格验证、候选/安装哈希一致；旧版完整备份。三个设置文件摘要不变；初始球体 56×56，右边距 16，首次展开 300×478。
- 在系统等待 WorkBuddy 钥匙串访问确认期间，主面板和浮窗可操作，恢复状态可见。此项验证的是非阻塞恢复，不表示临时签名升级免确认；用户允许后的余额及重启待复核。
- Windows 整合代码、Apple Silicon、多屏/Spaces、正式签名/Gatekeeper 和完整通知生命周期未重新验收。实际数据与合成测试分开记录；未上传日志、自动参榜或调用模型推理。

安装态补充：顶部三项指标、浮窗 140 日期格通过 AX 读取；组件管理 300×542，13 个按钮均在窗口内，移除/重新添加费用成功，完成恢复 300×478。主面板历史使用真实本机日志，缺失日仍未知，部分来源错误明确提示；恢复原 WorkBuddy 来源和组件显示。设置页版本/安全存储/快捷键禁用文案通过。目视、用户钥匙串允许后的真实余额与重启等待确认；没有为了重启关闭用户尚未处理的系统弹窗。

### 0.1.16 用户确认后的补验

用户确认界面完整、透明边缘正常，并已允许钥匙串访问；AX 确认积分余额恢复且无恢复中/旧积分提示。应用菜单正常退出后从安装路径重启，同一版本及二进制摘要不变，主窗和球体正常。系统再次要求 WorkBuddy 专属钥匙串确认，恢复中状态可见，不能记作免确认恢复通过；等待用户选择“始终允许”后复验。无需重新登录，未触碰密码或凭据内容。

持续授权复验：用户在系统弹窗选择“始终允许”后，正常退出并重启同一 0.1.16 安装二进制。新进程自动查询到积分，恢复中/旧积分状态消失，系统未出现本工具钥匙串确认；分别读取主面板与展开浮窗，积分一致。当前构建的重启恢复通过；未来签名变化的行为未验证，不承诺免确认。验收不保存凭据或真实金额，运行代码未改，不重复单元测试。

## Mac 0.1.17：数值、快捷键、参榜（CHG-20261018-MAC-PARITY）

- 环境：Intel macOS 15.7.7，Retina 2×，分支 mac/floating-hotkey-leaderboard，基线 9cf63ea。未运行 Windows/Apple Silicon，不称跨平台完成。
- 自动检查：npm test 85/85、npm run build、cargo fmt --check；bash scripts/mac-dev.sh cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml：113 通过/8 默认忽略。默认忽略项中，本轮显式执行 Carbon 冲突/保留旧绑定/释放、独立 Mac 参榜钥匙串往返、模拟服务全流程，共 3 项通过。
- 本机模拟端点测试命令：TOKENDAILY_RANK_LOCAL=http://127.0.0.1:8787 bash scripts/mac-dev.sh cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml local_trial_consent_restart_sync_pause_delete -- --ignored --test-threads=1。只发送自建 123 Token 夹具到 loopback；预览无上传，明确确认后上传，重建服务恢复身份和去重，暂停/恢复/删除通过。无真实参榜、无对话/日志上报；合成钥匙串和临时目录均删除。
- 数值布局：Tabbit Chromium 和本机 WKWebView 加载生产 MetricNumber/CSS，1/2/3 列、1,538.76 万、90,071,992.55 亿、长费用和排名均未发生字形裁切。WebKit 三列普通样例字体约 19.61px，字形宽约 69.27px，小于容器宽；移至单列/双列恢复 27px。浏览器截图已目视；该项不代替安装窗整体验收。
- bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked 生成 0.1.17 Intel App/DMG，hdiutil verify 通过。DMG SHA-256 c99ef7b2ad1af9acfeb24564e65af6936b79e6b796ea45f0f082f1fa10d30267，DMG 内未签名；安装 App 沿用授权执行 ad-hoc 签名并通过严格校验。
- 旧 0.1.16 App/校验通过的 ZIP 保留在 .bundle-tmp/mac-validation/before-mac-parity-0.1.17。正常退出后替换 /Applications；构建与安装二进制 SHA-256 一致：5a436da216efc5a4aaa98d7e500a3f8ddd4641db239d4667f9aef618eb74d498。settings/providers/workbuddy 配置摘要升级前后相同。
- AX 安装态已见快捷键录入按钮、试用按钮及网页版榜单按钮可用，Mac 未实现提示已消失，确认公开文案保留。未点击真实试用/参榜，当前无公开名次仍显示未知。
- 本次临时签名变化再次触发 WorkBuddy 专属钥匙串确认，已请用户在系统弹窗处理；没有读取/代填密码或更改访问控制。键盘录入、真实全局切换及重启尚待补验。

### 0.1.17 钥匙串允许后的安装态补验 · 2026-10-18

- 用户确认系统弹窗选择“始终允许”。检测 TokenDaily 专属 WorkBuddy 确认消失，随后正常退出并从 /Applications 重启；0.1.17 主二进制摘要仍为 5a436da216efc5a4aaa98d7e500a3f8ddd4641db239d4667f9aef618eb74d498，单实例。未再次出现钥匙串确认，真实积分自动恢复，主面板与浮窗数值一致；没有重建、重新签名或重新登录。
- 安装态录入临时 Shift + ⌘ + K 并保存，设置显示正确且无注册错误。先前自动注入未录入的原因是 AX 点击未让主窗口成为键盘焦点；使用 AXRaise/Main/Focused 后，同一已安装应用正确接收系统按键，无代码修补。
- TokenDaily 处于后台、独立验收空窗口占据前台时，通过 CGEvent 系统按键路径验证 56×56→300×478→56×56；指针在外仍保持展开。另发送一次按下加七次自动重复按下再抬起，仍只展开一次，再按正常收起。这是系统合成按键验收，不称人工物理键盘验收。
- 独立 Carbon 独占注册探针确认：正常绑定为占用，开始录入可注册，取消后重新占用；关闭浮窗可注册，重新开启占用；应用退出可注册，重启恢复占用。重启后设置保留绑定且按键再次可展开。
- 验收结束在设置清除临时绑定，原生探针确认已释放、持久化最新槽为 null，恢复原来未绑定状态和 Codex 来源选择。settings/providers/workbuddy 三份原设置摘要均一致；本地快捷键双槽为本轮新增的空偏好。参榜身份未创建、真实用量未上传。
- 本轮仅安装态补验及文档，沿用 81180d2 对应的 85 前端/113 Rust/3 显式专项测试，不重复构建。替代上节键盘与重启待办；Windows 回归、Apple Silicon、人工物理键盘、多屏/Spaces、正式签名升级和真实生产参榜仍未验证。
