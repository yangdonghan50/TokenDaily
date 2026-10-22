# TokenDaily 开发进度

## 2026-10-22：README 首图按附图突出品牌

- 用户反馈上一版中央留白偏大、Logo 不突出。按新附图将首图改为 1280×690：完整 Logo 宽度从 272 增至 620，放大两行主标题，产品面板移近并放大，取消外框与浅绿背景。左栏依次为品牌、主张、简短说明、工具来源和 Windows / macOS；图片下方改成开始使用、社区用量榜、开放共建三栏实际链接。
- 保留合成数据说明、双平台支持及现有安装状态，未新增产品能力。更新素材说明；离线 HTML、首屏与完整长图保存在本机忽略目录，并同步更新上一版 HTML 路径，方便刷新查看。
- 检查：渲染脚本与素材验证通过，17 个本地引用有效，SVG XML 可解析且无活动内容，`git diff --check` 通过。Tabbit 实测 1180px / 390px 图片加载、内部锚点、明暗切换、页面无横向溢出；首图六项文案均在画布范围内，三栏五个链接存在。首屏与长图目视检查通过，并由独立只读审阅复核与参考布局一致。
- 本轮仅改 README、首图、素材说明和进度记录。未运行应用构建或安装态测试；本地预览提交未推送公开仓库，原私库工作区保持不变。

## 2026-10-22：README 首屏收紧与双平台表达修正

- 根据用户反馈，去掉首页独立 Logo、标题、介绍与元信息逐层居中的布局。`hero.svg` 改为 1280×600 的一体横幅：左侧品牌、Windows / macOS 并列标识与产品主张，右侧沿用合成悬浮面板；保持白底、深青与薄荷绿风格。三张展示图的合成数据说明保留。
- README 主介绍明确支持 Windows 与 macOS，主入口改为“获取桌面端”。核心功能移除 Windows 专属更新宣传，补入用量趋势；下载、正式签名及官网更新通道的差异集中在快速开始，保留 Intel 已实机运行、Apple 芯片待实机验证的准确边界。产品支持与公开分发进度分别表达，未声称 Mac 已正式签名发布。
- 验证：本地渲染与素材检查脚本通过，17 个本地引用有效；SVG XML 解析通过，无脚本或外部活动内容，`git diff --check` 通过。Tabbit 在 1180px / 390px 检查三图加载、内部锚点、明暗切换及横向溢出，均通过；首屏与长图已目视检查。新版离线 HTML 与截图留在忽略目录。
- 只修改 README、首图、素材说明和本记录，未运行应用构建或系统安装测试。保存本地预览提交，尚未推送公开仓库；原私库工作目录保持不变。

## 2026-10-22：按用户参考版精简公开 README

- 参考用户提供的 HTML 与 ZIP，沿用白底、品牌渐变 Logo 和三张矢量展示图；仅提取明确需要的 README 与素材，未执行附件中的补丁或应用指令。参考正文由 192 行精简为 76 行，保留六项核心能力、四来源表、使用步骤和开发／贡献入口；详细规则改为链接。
- 替换 `assets/readme/` 中上一轮预览用的 PNG 与排版 HTML，采用 `hero.svg`、`activity.svg`、`community.svg`，同步素材说明。沿用普通来源标识，图中数值为示例；旧草稿可在上一提交查看。可选参榜、账号同步、系统模拟参与者、TRAE 不上传、API 等价估算及 Mac 实机／分发边界仍明确说明。
- 检查：`node .bundle-tmp/readme-design/render-condensed.mjs` 与 `node .bundle-tmp/readme-design/validate-condensed.mjs` 通过；18 个本地引用有效，三张 SVG 共 69,205 字节，XML 可解析，无脚本、外部引用或活动内容；`git diff --check` 通过。
- Tabbit 验证 1180px 桌面与 390px 窄屏，四张图片全部加载、内部锚点有效、页面无横向溢出，明暗切换正常；完整截图与首屏截图已目视检查。可离线 HTML 及截图保存于本机忽略目录，便于用户审阅。
- 本轮仅改 README、展示素材与进度文档，未运行应用构建、原生或安装态测试。修改保留在公开主线派生的本地预览分支，未推送公开仓库，也未引入私库历史；下一步等待用户审阅本版效果。

## 2026-10-22：公开 README 视觉与功能说明预览

- 用户要求依据原工程更新记录完善公开 README 与展示图，并先查看效果。工作分支 `shared/readme-showcase` 从公开 `main` 的 `ab15fcf` 建立；旧私库工作目录与历史保留。本阶段为本地可审阅草稿，未推送公开仓库，等待用户确认视觉方向。
- README 重排为产品首屏、九类功能、日历／趋势展示、四来源矩阵、三步开始、平台状态、数据边界、开发与许可。补齐用量趋势与额度提醒，修正普通悬浮球点击／贴边半球悬停、总历史与来源详情、游客参榜与账号同步的区别。Windows、Intel Mac 与 Apple Silicon 的已测／未验证状态分别说明。
- 新增 `assets/readme/`：两张展示图、三张真实组件演示截图、可编辑的本地排版 HTML 和素材说明。截图复用当前 `FloatingPanel`、`UsagePanel`，使用固定日期的合成额度、积分和用量；排名保持未参榜占位，费用为部分覆盖的示例，不读取本机真实凭据或日志，不请求厂商。沿用原创普通来源标识。
- 浏览器验证：Tabbit 实测两张画布 1200×820 / 1200×900，素材全部加载，文字和图片没有越出画布；已目视修正品牌背景与最初的截图底部裁切。README 本地排版在 1180px / 390px 视口检查通过，图片加载、内部锚点有效，页面没有横向溢出，窄屏表格可局部滚动。完整预览保存在本机忽略目录；这是 GitHub 风格的本地预览，尚未在公开页回验。
- 文件检查：`node .bundle-tmp/readme-design/validate-readme.mjs` 通过，README 的 17 个本地链接、排版页的 8 个素材引用均有效；5 张 PNG 共 617,555 字节。`git diff --check` 通过。生成、检查脚本与原始预览仅存本机忽略目录，公开候选只包含 README、素材与进度文档。
- 本轮仅改文档与展示素材，未改变应用逻辑，没有重新运行 Windows/macOS 构建或安装态测试。下一步按用户意见调整视觉，再将经确认的文档提交推送到公开主线；不携带私库历史。

> 2026-10-22 公开前脱敏说明：历史验收记录中的个人路径、昵称及可识别的真实账户数值已替换为占位符；测试数量、版本、技术结论及未验证项保留。原始私有 Git 历史未改写，不能据当前文件已脱敏就直接公开旧历史。审查见 [OPEN_SOURCE_REVIEW](OPEN_SOURCE_REVIEW.md)。

## 2026-10-22：公开仓库采用 TokenDaily 名称

- 按用户要求先将原私库改名为 `yangdonghan50/TokenDaily-Private`，再将公开库 `TokenDaily-Desktop` 改为 [TokenDaily](https://github.com/yangdonghan50/TokenDaily)。原私库 ID `1422056631`、公开库 ID `1429631056` 均保持不变，可见性分别为 private / public。
- GitHub API 回读确认：改名时私库 main 仍为 `734e9213e2e02bcead08d399839312d1d0bf086a`，公开 main 仍为 `98c4dcc622aee3cab8e605125d54ab31bb6eb762`；私密漏洞报告仍为 `enabled=true`，Apache-2.0 识别正常。改名没有交换、重建或改写两库历史。
- 本机 origin 已更新为 TokenDaily-Private，public 及隔离发布仓库的 origin 已更新为 TokenDaily。README、SECURITY、交接文档、开源审查和双机工作流同步规范地址；下方原发布记录保留当时名称。
- 检查：文档检查脚本核对 275 个文件、52 个本地 Markdown 链接，无断链或忽略规则异常；`git diff --check` 通过，LICENSE 校验和不变。GitHub 回验按固定仓库 ID 检查名称、可见性、主线 SHA 与漏洞报告设置；原始结果保存在本机忽略目录，未纳入源码。
- Mac 旧私库工作目录尚未在本机操作：下一次 fetch/pull/push 前须将 origin 改为 `https://github.com/yangdonghan50/TokenDaily-Private.git`，步骤见 [DEVELOPMENT_WORKFLOW](DEVELOPMENT_WORKFLOW.md)。原 TokenDaily 地址现属于公开库，旧私有分支不能向该地址推送。
- 此次仅修改仓库设置和六份文档；文档更新在私库任务分支保留，并沿原公开 main 追加独立文档提交。公开根仍为 `ca0a0a1d7b3e25b63346b3fd9357c31d4b44b44d`，不合入旧私有历史。源码逻辑与平台构建均未改动，不重复前端/Rust/安装态测试。

## 2026-10-22：创建公开仓库与普通来源标识

- 用户明确批准：干净快照新建公开库，原私库保留；四个厂商 Logo 替换为普通标识，启用私密漏洞报告。名称检查后选用 `yangdonghan50/TokenDaily-Desktop`；原 `TokenDaily` 私库只读确认仍为 private。
- 四个来源改用原创 CX / CL / WB / TR 中性字母 SVG；移除原 Codex/Claude PNG，替换 WorkBuddy/TRAE SVG 内容，更新 `ProviderLogo.tsx`、README 示意图、素材与第三方声明。名称、尺寸和控件行为沿用；示意数值为合成数据。未修改账户、采集、计算或上传规则。
- 更新 README、SECURITY、交接文档、功能对照和双机工作流。新公开主线从独立根提交开始；日后任务基于公开 main，原私有分支、标签与历史不能合入或整体推送。公开源码仍为 Apache-2.0，免费桌面与网页情报订阅独立。
- `npm test` 101/101、TypeScript/Vite 生产构建及 `git diff --check` 通过。Tabbit 本机合成预览加载实际 ProviderLogo 组件，16 个图标在 21/24/32/33px 全部加载且尺寸正确，截图目视无裁切；保持 `aria-hidden` 装饰图语义。沙盒内预览端口不可达，改由本机 loopback 临时服务完成检查。没有运行 Windows/macOS 原生安装态；Rust 未改动，本轮未重复其上一阶段 146 项检查。
- GitHub 官方 API 已创建新的公开空库；私密漏洞报告启用后 GET 回读 `enabled=true`，SECURITY 指向新库入口。未创建公开 Issue 或测试漏洞报告。原私库可见性仍为 private。
- 发布完成：[TokenDaily-Desktop](https://github.com/yangdonghan50/TokenDaily-Desktop) 的独立初始提交为 `ca0a0a1d7b3e25b63346b3fd9357c31d4b44b44d`，文件树 `3764b38376ffa3dedcf67cee881c07b11017cd71`。从只导入本次 bundle 的隔离 Git 仓库推送，首次远端匿名回验为 1 个无父提交的根、275 个文件、唯一 main、0 标签；原四个厂商图形的 Git blob 均不在新树中。GitHub 识别许可证为 Apache-2.0；公开源码约 2.10 MB 的最终 Gitleaks 复扫未检出密钥。私密漏洞报告 GET 回读 `enabled=true`，原私库仍为 private。
- 首次公开回验后仅追加本次发布文档记录，父提交限定上述公开根，不合入原私有任务分支。Windows 原目录的 origin 保留，新增 public remote 用于明确区分新库；本地原始审查报告、候选/发布 bundle 与 ZIP 保存在忽略目录。没有新的安装包发布、官网部署或生产签名。

## 2026-10-22：Apache-2.0 开源准备与公开前审查

- 用户确认桌面端采用 Apache-2.0，免费共建；网页 AI 工具情报、优惠与更新提醒订阅独立运营。公开仓库必须等待本轮交付后的用户确认。基于 `734e9213` 从最新 `origin/main` 建 `shared/open-source-preparation`，开始时工作区干净；现有仓库通过只读 GitHub API 确认为 private，未修改可见性。
- 新增完整 Apache LICENSE、NOTICE、CONTRIBUTING、SECURITY、Issue/PR 模板与 `docs/OPEN_SOURCE_REVIEW.md`；README 增加参与入口。npm/Cargo 许可元数据与 Tauri 安装资源同步，保留第三方原许可和 TokenTracker MIT 声明；未新增产品功能、依赖或网站收费实现。
- 基线全部可达 Git 历史为 113 提交、1,284 blob，非浅克隆。官方 Gitleaks 8.30.1 发布 ZIP 的校验和验证通过；历史差异扫描（工具报告 103 个有差异的提交）及全部历史 blob 扫描（约 47.59 MB）均未检出密钥。补查发现旧文档和历史含个人路径、昵称、真实账户数值；当前文件已脱敏，测试夹具改合成值、示意图仍明确标记示例。已删除历史界面图亦含旧数值，故不能直接公开原有历史。原始报告、旧文件备份和工具均保存在忽略的本地审查目录，不提交。
- 补充 `.gitignore` 保护受保护凭据、Claude 桥接备份/捕获及桌面分发物；保留 lockfile、平台配置及图标的跟踪。当前合成邮箱/URL 反例、更新公钥、公共接口与 TRAE 应用格式常量已复核；未发现已跟踪的真实凭据、JSONL 或数据库。历史维护脚本的通用迁移目录保留，贡献指南明确不应为日常开发运行它们。
- 验证：`npm test` 101/101；`npm run build`；`cargo fmt --manifest-path src-tauri/Cargo.toml --check`；`cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 146 通过/10 忽略；`cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` 均通过。Rust 有既有未使用代码及增量缓存硬链接回退警告。未运行真实来源忽略测试、Windows 安装态或 Mac 构建/实机。
- `npm run tauri -- build --no-sign --bundles nsis -- --locked --offline` 最终通过。首次 NSIS 工具下载因 DNS 失败，使用当前进程的既有本机代理下载官方工具后重试成功。未调用生产私钥、未安装或发布；LICENSE、NOTICE、THIRD_PARTY_NOTICES 暂存资源与源文件逐字节一致，NSIS 脚本包含对应文件。统一验收条目为 CHG-20261022-OPEN-SOURCE，Mac 仍未验证。
- 全部 48 个本地 Markdown 链接有效；忽略规则样例和所需源码资源检查通过，`git ls-files -ci --exclude-standard` 为空，`git diff --check` 通过。最终新增材料纳入当前树复扫，再提交并按既有授权普通推送到原私库任务分支；不合并 main。当前文件树还生成单提交、无旧历史的本地公开候选 bundle/ZIP，提交及校验信息见忽略目录 `candidate-manifest.json`；候选不推送或公开，原始历史不改写、不强推。
- 公开前剩余条件：四个厂商 Logo 的再分发依据尚未确认，需核实或替换；用户确认项目品牌及自带荣誉图标分发权；确定干净快照的公开仓库名称/迁移方式，并在公开时启用、验证私密漏洞报告通道。不能把本次扫描解释为完整安全认证或第三方授权证明；详细边界见 OPEN_SOURCE_REVIEW。下一步为所有者审阅并明确批准公开操作。

## 2026-10-22：README 按最新主线更新

- 按用户要求更新仓库首页内容，保留已确认的图文结构。补充 0.1.28 当日多工具排名与详情周榜的区别、主动收起与关闭的区别、组件管理、增量刷新和 Mac Esc 的使用条件；修正 Claude Desktop 额度记录现已支持双端的旧说明，并说明 Mac 可选状态栏桥接和积分重启恢复。
- 平台状态依据现有记录更新：Intel Mac 0.1.28 日榜／收起及此前 Claude 分项验收、通用 App 已交叉构建；Apple Silicon 实机、Mac 正式分发与官网更新通道仍未完成。未将源码版本写为官网已发布版本。`assets/readme-app-preview.svg` 同步今日排名、Claude Code 名称与收起图标，继续明确所有数值为示例。
- 本轮仅修改 README、说明用 SVG 和进度文档；16 个本地链接／图片引用均存在，SVG XML 解析、源码版本 0.1.28 一致性、示意数据标记及 `git diff --check` 均通过。不改应用源码、不重复运行应用测试或安装验收。按已有授权提交并推送。

## 2026-10-22：主线核对与已合并分支清理

- 用户要求全部改动合入主分支、提交并清理无用分支；按已有授权由助手在检查后普通 push。fetch 后确认 `main` 与 `origin/main` 均为 `1f366b4`：Mac 端已通过 `6d15ada` 合入 Windows 0.1.28 功能，并补充安装态记录与 Mac Esc 修复。本轮无需再次合并，无代码冲突。
- Windows 合并源码复查：`npm test` 101/101；`npm run build` 通过；加载 `scripts/use-local-dev-env.ps1` 后，`cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 146 通过、10 忽略；`cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` 通过。编译有 5 条未使用代码警告，无编译错误。忽略项为需单独运行的真实环境诊断，本轮未运行。
- 核验三个本地任务分支均已完全包含于 main，删除 `shared/floating-daily-rank-collapse`、`codex/account-sync-repair`、`codex/github-home-review`。account-sync-repair 工作树原为干净状态，先转为 detached HEAD 再删除分支；所有额外工作目录、旧提交快照与忽略的备份／构建产物均保留。
- 本轮只复核集成源码和 Git 状态，不重新打包、安装或发布官网版本。Windows 现有安装态记录对应 `f223c0d`；Mac 实机结果仍以此前分项记录为准，未完成项不改为同步完成。提交前检查差异和暂存范围，收尾时推送 main 并删除已完全合并的远端同名功能分支。

## 2026-10-22：Mac 悬浮窗普通浏览时 Esc 收起

- 原因：Mac 悬浮窗为不激活面板（`focus: false`），Esc 送往应用当前键盘窗口（通常是设置主窗口），面板根节点的 `onKeyDown` 收不到。
- 修复（仅 macOS 原生侧）：既有本地 NSEvent 监听在无拖动时，若悬浮窗可见、已展开（大于 56 点悬浮球）且指针在窗口内，向悬浮窗发送 `floating-escape` 并吞掉该键；其余情况照常放行。前端收到后在面板内获焦元素（无则面板根节点）重放标准 Esc 键盘事件，沿用“先取消拖动、再退出编辑、最后收起”的顺序。新增 `escape_collapses`（Rust）与 `nativeEscapeTarget`（前端）测试，先确认失败再实现；前端 101/101、Rust 162 通过/13 忽略，TypeScript/Vite 构建通过。
- 限制：仅当 TokenDaily 为活动应用时有效（点击面板通常会使其活动）；仅悬停展开而其他应用持有键盘时，Esc 仍归该应用。Windows 行为未改。
- 安装：Intel `--bundles app` 构建、ad-hoc 签名并严格验签，旧版备份于忽略目录 `.bundle-tmp/mac-validation/before-esc-fix-20261022-141353/`，替换并启动；新签名再次触发钥匙串授权，由用户处理。全屏控制申请连续 4 次未获响应，改由用户按清单自行验收：用户确认指针在面板上按 Esc 可正常收起。用户未反馈“指针在面板外按 Esc 不收起”“∨ 收起”“编辑模式下 Esc 先退出编辑”三项，记为未验证。

## 2026-10-22：Windows 0.1.28 合入 Mac 主线

- 用户要求把 Windows 刚推送的改动加入 Mac 这边。`origin/shared/floating-daily-rank-collapse`（日榜与主动收起，0.1.28，3 个提交）基于 `acffc34`，不含此后的 13 个 Mac 提交，直接比较会显得删除钥匙串、statusLine 桥接与通用版支持；按三方合并并入 `main`，代码自动合并，仅 PROGRESS/FEATURE_PARITY 顶部新增条目冲突，两侧记录均保留。
- 检查：`npm test` 100/100、`npm run build`；`bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 161 通过/13 忽略；cargo fmt 整理了 Windows 侧 `leaderboard.rs` 的排版。
- Mac 0.1.28 安装态（用户授权）：Intel `--bundles app` 构建、ad-hoc 签名并严格验签；旧 0.1.27 备份于忽略目录 `.bundle-tmp/mac-validation/before-0.1.28-20261022-135550/`；正常退出后替换并启动，单进程，版本 0.1.28。新签名再次触发 `Claude Code-credentials` 钥匙串授权，由用户本人处理。
- computer use 验收（经用户批准临时全屏控制）：悬浮窗顶部显示“今日排名 #18”，设置页排行详情仍为周榜 #17，二者独立；右上角新增 ∨。展开后保持指针在面板上点击 ∨，面板立即收回到原悬浮球位置，通过。
- 未通过：普通浏览时按 Esc 不收起（指针在面板上、先点面板空白处后再按 Esc 均无效）。Mac 悬浮窗为不激活面板（`focus: false`），Esc 监听挂在面板根节点，未获键盘焦点时按键送往前台主窗口；需单独修复并实测，不能标记双端同步。
- 其他观察：①展开时窗口自屏幕边缘内移，指针未移动会被当作离开而立即收回，移动指针进入后正常；②模拟点击面板顶部空白（拖动区）后底部出现“暂时无法拖动窗口，请重试”，可能与合成点击有关，未用真实拖动复核；③钥匙串授权关闭后 Claude 卡片显示“Claude Desktop · 上次记录、重置时间未知”，即本次未取得 Code 订阅额度，原因（拒绝或待下次刷新）未查。

## 2026-10-22：macOS 通用版（Intel + Apple 芯片）构建

- 用户无 Apple 芯片设备，要求先在本机交叉构建通用版并让更新入口支持通用包。`bash scripts/mac-dev.sh rustup target add aarch64-apple-darwin` 仅写入工程内忽略的隔离工具链。
- 更新：`tauri.macos.conf.json` 端点改为 `darwin-{{arch}}.json`（Intel 仍请求原 `darwin-x86_64.json`）；`platform/macos/app_update.rs` 下载白名单按运行架构判断，Intel 接受 `x64`/`universal`，Apple 芯片只接受 `universal`。测试先写并确认编译失败，实现后 `cargo test --locked --offline --features macos-transparent --lib` 159 通过/13 忽略，fmt 已应用。
- 构建：`bash scripts/mac-dev.sh npm run tauri -- build --target universal-apple-darwin --no-sign --bundles app -- --locked` 约 4.5 分钟生成 37.5 MiB 通用 App（`src-tauri/target/universal-apple-darwin/release/bundle/macos/`）；`lipo` 显示 x86_64 与 arm64，两段 minos 11.0；ad-hoc 签名覆盖两段，严格验签通过；Intel 段以 `arch -x86_64` 运行状态栏辅助模式正常退出，arm64 段本机无法运行。
- 未做：Apple 芯片实机验收；替换安装版；官网通用包与 `darwin-aarch64` 清单发布；GitHub Actions Apple 芯片 CI。

## 2026-10-21：macOS TRAE 官方积分窗口残留

- 现象：0.1.27 安装版启动 15 分钟有 3 个、16 分钟后 4 个不可见 1024×760 窗口（CoreGraphics 窗口列表按 pid 统计尺寸与是否在屏，不读标题或像素），每次约 5 分钟的后台积分刷新增加 1 个；此前 0.1.24 运行约 1.5 小时约 60 个。只有 `trae_credits.rs` 创建 1024×760 窗口。
- 原因：后台刷新每次 `destroy()` 旧的隐藏官方窗口再新建；macOS 上被销毁 WebView 的原生窗口未被释放（tao 只在窗口对象释放时关闭 NSWindow，WebView 层仍持有引用），因此逐次累积。未改动 Tauri/wry。
- 修复（仅 macOS，`REUSE_WINDOW`）：窗口存在时复用同一窗口及其注入脚本的 nonce——已在仪表盘则 `reload()`，否则 `navigate()` 回仪表盘（同一 `#usage` 地址只换片段不会重载）；手动登录也复用并显示窗口。窗口不存在、导航失败或清除授权后才重建并换新 nonce。回调仍要求 nonce 与当前尝试一致、在截止时间内且来自官方源。Windows 行为不变。新增 2 项测试；Rust 156 通过/13 忽略，fmt 已应用。
- 安装态验证（用户授权）：修复版 `--bundles app` 构建、ad-hoc 签名并严格验签；旧安装版备份于忽略目录 `.bundle-tmp/mac-validation/before-trae-window-fix-20261021-203849/`，正常退出后替换并于 20:39 启动。22 分钟逐分钟记录：不可见 1024×760 窗口始终为 1；`trae-last-balance.json` 于 20:39:09（新建窗口）及 20:44:14、20:49:10、20:54:12、20:59:11（复用窗口）各更新一次，4 个后台刷新周期均成功；WebContent 进程恒为 5，常驻内存 49–67 MB 无增长（修复前同期 3→4 个窗口）。computer use 核对设置页 TRAE 积分 [余额已脱敏]，“21:02 读取 · 官方账户数据”。
- 旁注：同次查看 Claude 曾短暂显示“上次成功的数据”，数秒后再看为“额度已读取”；未见钥匙串弹窗，未定位原因。
- 未验证：手动“重新登录官方账户”复用窗口的显示流程；Windows 未测（该平台行为未改）。

## 2026-10-21：合并 Windows 0.1.27 主线与 Mac Claude 分支（CHG-20261021-MERGE）

- 本地 `main`（8 个 Mac 提交 + statusLine 桥接 `58159ff`）与 `origin/main`（Windows 0.1.25–0.1.27 共 13 个提交）自 `011293a` 分叉。按用户要求兼顾两端，合并冲突 8 个文件。
- 代码取舍：采用 Windows 侧的共用 `choose_source` / `parse_desktop_history` / `source_fingerprint` 与 provider 刷新流程；Mac 专属解析器删除，`platform/macos/claude_desktop.rs` 改为只读字节并实现原先的 Mac 存根；保留 Mac 钥匙串读取与 statusLine 桥接，桥接只在 Desktop 来源（`desktop:` 指纹）时叠加，改用共用标识 `claude:desktop:*` 与窗口键。前端三处 `claude-desktop:` 前缀改为 `claude:desktop:`；设置页说明改为“Code 订阅登录优先、Desktop 记录回退、macOS 可选桥接”；Desktop 成功提示改普通说明样式；保留 Mac 的快照预览和“查看 Claude 额度与本机用量”入口。文档冲突两侧记录全部保留，删去已过时的“macOS Desktop 与钥匙串未实现”。
- 检查：`npm test` 98/98、`npm run build`；`bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 154 通过/13 忽略；cargo fmt 已应用（顺带整理 Windows 侧未格式化的 `leaderboard.rs`、`windows/mod.rs` 等排版）；真实 Desktop 记录只读测试 1/1。
- 未做：未在 Windows 编译测试合并结果；未推送。
- Mac 0.1.27 安装态（用户授权）：`bash scripts/mac-dev.sh npm run tauri -- build --no-sign --bundles app -- --locked` 生成 Intel App，ad-hoc 签名并严格验签；旧 0.1.24 测试版完整备份于忽略目录 `.bundle-tmp/mac-validation/before-0.1.27-merge-20261021-201748/`；正常退出后替换并启动，单进程，版本 0.1.27。启动后 macOS 弹出 `Claude Code-credentials` 钥匙串授权，由用户本人允许（未代点）。computer use 核对：设置页“额度已读取”，状态栏桥接保持关闭；Claude 详情来源“Claude Code 订阅接口 · 只读”、覆盖范围为已核验订阅账户，5 小时剩余 [比例已脱敏]、预计 [重置时间已脱敏] 重置，7 天剩余 [比例已脱敏]、预计 [重置时间已脱敏] 重置（与此前状态栏采样的重置时间一致）；悬浮窗 Claude 卡片同显 69%/34 分钟后重置与 88%/4 天 21 小时 34 分钟后重置。详情页本机今日 Token [用量已脱敏] 万（20:26），独立脚本稍后为 [用量已脱敏] 万，差额为期间新增日志。
- 旁注：新版启动约 10 分钟后已有 2 个不可见 1024×760 离屏窗口，积分窗口残留问题仍在（另开排查会话中途停止，未产出修复）。

## 2026-10-21：Claude Code 状态栏桥接（重置时间）

- 用户参考 TokenRank 的做法，批准可选 statusLine 桥接：由用户在设置中开启，先备份 settings.json，原样转发给现有 opentoken 状态栏，关闭时还原。终端实测确认 Claude Code 交互模式提供 5 小时/7 天重置时间，Desktop Code 标签页不提供（见 FEASIBILITY）。
- 实现：`src-tauri/src/claude_statusline.rs`、`src-tauri/src/main.rs`（`--claude-statusline` 启动前分流）、`providers.rs`（`get/set_claude_statusline`、刷新时合并、清除数据先还原）、平台层 `claude_account_file`、`src/ui/ClaudeStatuslineSettings.tsx` 与 `App.tsx` 重置时间显示；`serde_json` 启用 `raw_value`，锁文件不变。Windows 设置项显示不支持。
- 验证：`npm test` 96/96、`npm run build`；`bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 150 通过/12 忽略（新增 12 项：设置文件逐字节还原、只还原 statusLine、被他人改动不覆盖、缺文件/缺键、非法文件拒绝、输入白名单、组织哈希、合并新旧与跨账号/过期窗口、路径转义）；cargo fmt 已应用。调试版二进制临时目录冒烟：转发输出与退出码 3 保持，采样文件 0600 且不含会话/路径，约 40 ms。
- 安装（同日，用户授权）：`bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 生成 Intel App（DMG 步骤因 Finder AppleEvent 超时失败，未生成 DMG，无残留挂载）；按既有本地验收方式 ad-hoc 签名，`codesign --verify --strict` 通过。旧安装版完整备份到忽略目录 `.bundle-tmp/mac-validation/before-claude-statusline-20261021-153951/`，摘要一致；正常退出旧版后替换 `/Applications/TokenDaily.app` 并启动，单进程运行，安装版二进制摘要与构建版一致。`~/.claude/settings.json` 摘要未变，桥接未开启。
- 安装态开启（用户在设置中自行打开，并在终端 Claude Code 发一条消息）：`~/.claude/settings.json` 与备份的差异只有 statusLine 两行，hooks、theme、键顺序与 0644 权限不变；备份摘要与开启前原文件一致，桥接记录保留原 opentoken statusLine 原值。16:10 采样文件 0600，只含采样时间、组织哈希与两窗口；5 小时已用 [比例已脱敏]、21:00 重置，7 天已用 [比例已脱敏]、10-26 18:00 重置，组织哈希与 `~/.claude.json` 及 Desktop 历史一致。opentoken 状态栏缓冲同时更新，说明转发生效。Desktop 最新采样 15:34 仍为上一窗口的 36%，按规则采用较新的状态栏数值。
- 显示：用户目视确认悬浮窗 Claude 卡片显示重置时间且数值正确。
- 关闭还原：经用户授权用 computer use 在设置页关闭开关。`~/.claude/settings.json` SHA-256 恢复为开启前的 `3b2b023c…e724`（逐字节一致），statusLine 回到 opentoken，权限 0644；TokenDaily 数据目录内备份、桥接记录、采样文件均已删除，无临时文件残留。
- 旁注：同次检查发现安装版有约 60 个不可见的 1024×760 离屏窗口（与 TRAE/WorkBuddy 官方积分窗口尺寸一致），进程常驻内存约 88 MB、WebContent 进程 5 个；疑似积分刷新重建窗口后原生窗口外壳未释放，未在本次处理。
- 未验证：Windows 未实现；“statusLine 被他人修改时不覆盖”仅单测覆盖。测试时误以 `--version` 调用了一次已安装的 TokenDaily（已在运行，单实例检查后约 0.24 秒退出）。

## 2026-10-21：Mac Claude Code 真实日志补验

- 用户要求添加 Claude 用量查看与额度监测。核对后两项已实现：Claude Code 本机 Token（`claude_usage.rs`）与订阅额度（Code OAuth，Mac 另有 Claude Desktop 本机历史回退）。本机已安装 0.1.24 的 Claude 来源开关为开启。
- 本机首次出现真实 Claude Code 日志（来自 Claude Desktop 的 Code 标签页）。新增默认忽略测试 `live_claude_code_logs`；Rust 采集器今日合计与独立 Python 同口径计算一致（[用量已脱敏]，日志仍在增长，数值只作当次核对）。该登录方式无 Claude Code 钥匙串条目/凭据文件，Code OAuth 额度不可用；Desktop 历史回退真实样本只读测试通过（最新采样约 12 分钟前，含 5 小时/7 天）。
- 验证：`bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 139 通过 / 12 忽略；fmt 检查通过；两项真实环境测试显式运行通过。前端未改动、未重跑；安装版界面未重新验收；Windows 未测。
- 已知限制：Desktop 历史无重置时间且身份未核验，额度提醒保持暂停；Windows 未实现 Desktop 历史读取。
- 同日追加：参考 TokenRank（opentoken 通过 Claude Code statusLine 取官方额度与重置时间）。用户批准可选 statusLine 桥接，前提是先验证 Desktop Code 标签页是否调用 statusLine。实测 Code 标签页为无界面 stream-json 模式，90 秒/259 次进程采样未见 statusLine 调用，opentoken 状态栏缓冲从未生成，判断为很可能不调用；临时探针需改 `~/.claude/settings.json`，被自动权限检查拦截，文件未改（SHA-256 前后一致，备份在会话临时目录）。桥接代码未实现，详情见 FEASIBILITY。

## 2026-10-20：Claude Desktop 快照提示与 Token 来源核对

- 用户给出的所谓错误原文是“Claude Desktop 本机历史快照；账户未核验，重置时间未知，提醒暂停。”；这是成功读取后的范围提示，旧版却用橙色警告样式展示，并且设置页未列出额度窗口。连接选择已保存，本机历史样本含 5 小时和 7 天数值，真实文件只读测试通过。已改为成功时不设置错误消息，并在 Claude 设置页直接展示两个窗口、采样时间、账户未核验与重置时间未知；另保留详情入口。读取失败时仍区分文件不存在/权限问题并显示后端具体原因。
- 验证：`npm test` 96/96、`npm run build`、Mac `cargo test --locked --lib claude_desktop` 3 通过 1 忽略，真实样本只读测试显式运行 1/1；Rust fmt 与差异检查通过。Mac App 构建、本地临时签名、严格验签通过；旧安装版完整备份于忽略目录 `.bundle-tmp/mac-validation/TokenDaily-before-claude-ui-20261020.app`，新版已原位安装且单进程启动。用户目视确认设置页的 5 小时和 7 天剩余百分比显示正常，橙色误报警已消失；安装版二进制与构建版 SHA-256 一致。此次仍未 push。
- 用户提供的生财有术 TokenRank 页面公开样例将来源标为 **Claude Code**，未见 Claude Desktop 聊天精确 Token 的读取说明。TokenDaily 已用 Claude Code 本机 JSONL 中的输入/输出/缓存字段统计精确 Token；本机 Claude Desktop 只发现订阅额度百分比历史，不能把它换算为 Token。网站榜单不是 Claude Desktop 聊天 Token 的已验证数据源；不安装其采集器、不上传 TokenDaily 数据。参考：[生财有术 TokenRank 示例](https://scys.com/tokenrank/u/122881)。

## 2026-10-22：GitHub 代理诊断与任务收尾自动 push

- 用户将 Git 偏好改为助手完成检查和提交后自动 push 对应分支。Windows 系统代理端口为 127.0.0.1:7897；Git 此前没有显式代理。实际分别以禁用代理和显式 HTTP 代理执行 `git ls-remote origin refs/heads/main` 均成功，因此未把历史失败断言为端口错误。为当前仓库的 GitHub URL 保存已验证代理于 `.git/config`；未改全局 Git、系统代理、凭据或共享 Codex 配置。
- 经该配置执行 `git fetch origin` 成功，发现远端 main 已有 13 个后续 Mac 提交（至 `fc1f545`）。本次保留主线与当前任务分支，未自动合入未在 Windows 回归的 Mac 改动。`git push --dry-run` 验证写权限成功，随后 `git push -u origin HEAD:refs/heads/shared/floating-daily-rank-collapse` 实际成功推送 `7455697`，对应已验收的 0.1.28 日榜和主动收起功能；远端 main 未改写。
- 将当前授权与任务收尾步骤写入交接和协作说明，后续代理异常时先检查 VPN 是否开启及其本机端口。此次只改流程文档与本机 Git 网络配置，应用测试沿用前一条的 100 项前端／133 项 Rust 与 Windows 安装态结果；提交前复查 Git 差异与暂存清单。

## 2026-10-22：悬浮窗日榜与主动收起（CHG-20261022-01）

- 任务分支 `shared/floating-daily-rank-collapse`。先建立统一验收并补失败测试：悬浮窗旧请求仅返回周榜；新日榜需来自已认证公开身份、`period=day&tool=all`、北京时间当日的起止日期，且返回本人 ID 一致。新增只读 `daily-status`，复用已有日榜 API，不改变上传、公开参榜、详情窗口周榜或官网部署。
- 新增右上角主动收起动作，清除编辑、焦点、快捷键保持和残留拖动状态；主动收起跳过鼠标仍在面板内的自动收缩保护，已有贴边侧保留。等待原生指针真正离开再允许贴边悬停展开，避免立即弹回；控制器已经收起但原生尺寸失败时仍可主动重新调整。普通浏览 Esc 同样收起，组件编辑／拖动先由组件取消该状态；原 × 按钮继续负责关闭悬浮窗开关。
- 检查：新增测试在旧实现先失败，随后 `npm test` 100/100、`npm run build` 通过；E 盘工具链执行 `cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 133 通过／8 忽略，`cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` 通过。Tabbit 稳定 CLI 未安装，已有浏览器连接本机测试端口超时，随后改用已有本机 WebView 调试工具执行真实安装态检查。Mac 未编译或实机验证，功能清单不标同步完成。
- 构建及安装：公共版本统一为 0.1.28，沿用既有 DPAPI 更新密钥签名；NSIS 包 11,424,596 字节，SHA-256 `9335cbd4645196270b5a25edf90bc6f22bed2caee8500c93f2dd3e7e530bba4b`，经官网同一 `updateManifest` 校验逻辑验证包内容、公钥、签名与可信版本绑定。旧 0.1.27 exe／卸载器备份于忽略目录 `.bundle-tmp/pre-0.1.28-install`；以 `/S /UPDATE /P /R /D=<installation-directory>` 原位安装退出码 0，安装版文件版本 0.1.28。四份原设置文件摘要未变。
- 安装态：真实日榜返回 `period=day`、北京时间起止均为 2026-10-22、本人 #[名次已脱敏]；详情只读状态仍为 `period=week`、#[名次已脱敏]，证明没有把周榜改名。300×540 逻辑像素（375×675 截图）四个头部按钮完整可见，初次几何断言因 0.00002 像素浮点误差失败，按 0.5 像素容差与截图重新核对通过。主动收起覆盖保持展开、长按编辑、重新展开与普通 Esc；回到 56×56 球，悬浮窗开关仍开启。普通点击展开后真实鼠标在外会自动收起；用户的偶发卡住未复现，拖动／贴边和混合 DPI 本轮未实机重测。完成后关闭临时本机调试端口并正常重启安装版；未发布官网、未 push，源码保留任务分支供 Mac 后续验收。

## 2026-10-21：全部本地改动汇总到 main（用户自行 push）

- 检查主工作区、全部本地分支及四个附加 worktree，均无未提交改动。README 改稿与 0.1.26 Claude 修复已在主线；将唯一新增的 `codex/account-sync-repair` 提交 `a08bec6` 快进合入本地 `main`，公共源码版本统一为 0.1.27。旧悬浮球、日历与快捷键工作树的功能已由后续主线提交纳入，不将旧快照再次合入以免回退。保留现有工作树和分支。
- 同步更新 README 的源码版本、Claude 额度来源与双设备验收边界，更新交接入口指向用户随后推送的 `main`。按本次用户明确要求集成尚待 Mac 验证的共享修复，功能清单仍标“未完成”。本轮不发布、不上传安装包、不 push；Windows 安装态沿用原任务的已记录结果，本轮不重复安装，Mac 未编译或实机验证。
- 集成检查：`npm test` 98/98、`npm run build` 通过；加载 `scripts/use-local-dev-env.ps1` 后执行 `cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib`，131 通过／8 按既有配置忽略；`cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` 通过。编译只报既有 `can_submit` 未使用与增量缓存无法硬链接而回退复制的警告。Node/Tauri/Cargo 共六处版本均为 0.1.27；README 17 个链接／图片引用中的本地路径全部存在；`git diff --check` 通过。构建目录、工具链与忽略工作树保持在 Git 外。

## 2026-10-21：官网设备连接与历史额度阻塞修复（CHG-20261021-02）

- 任务分支 `codex/account-sync-repair`。用户报告 Windows/Mac 使用同一邮箱但账号总量偏少。只读核对发现：Windows 0.1.26 的受保护试用身份、官网关联与 12,583 个上传检查点均在；旧运行进程曾显示为新游客，重启后原“测试账户”恢复。初始空/失败状态此前用 `!state?.loggedIn` 渲染为游客，确切的原进程缓存来源未能复现，不能断言凭据曾丢失。Mac 截图没有官网连接入口，仅有“不支持跨设备登录”的游客提示，版本未知；生产账号仅连接一台 Windows，Mac 尚未向它上传。
- 先补共用状态与 Rust 排序/恢复失败测试，再改账号加载门控、失败重试、前台会话复核与后台更新事件。明确重新检查会重读安全存储，而非继续使用初次空结果。新用户可直接从应用发起官网连接，内部创建本机身份后在浏览器批准，不必先公开参榜，不向前端传递凭据。
- 历史补传先处理最旧记录且与当天记录共用预算，曾因每日历史预算触顶停在 13:44。账号上传现在按最新优先、每批最多 200 条、当天与历史分批，逐批保存确认；新增向后兼容的 `last_current_success` 记录部分成功，不提前推进整段历史游标。账号文案区分永久云端历史与游客两周补报限制。官网独立仓库提交 `fd71dbf` 提供历史 60,000/live 15,000 估算写入预算的原子预约、状态提示与跨标签会话复核；Worker `28a048d2-6b61-4302-ba85-331e5150b876` 已部署，无迁移或预算清零。
- 检查：`npm test` 98/98；`npm run build` 通过；使用已有 E 盘工具链运行 Rust `cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 131 通过／8 忽略，`cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` 通过，仅既有 `can_submit` 警告。Git 差异检查通过。官网 56 项单元测试、隔离 D1 业务/成本和真实认证测试通过。
- 安装：版本升至 0.1.27，使用既有受保护更新密钥签名。最终 NSIS 包 4,323,308 字节，SHA-256 `5bcbccc69efd24a9d52b65429de7a4fe16208dc364ecc2902445aed3137086c3`，经官网相同 `updateManifest` 逻辑验证公钥、文件签名及可信版本绑定。旧 exe/卸载器保留于忽略目录 `.bundle-tmp/pre-0.1.27-install`，加密身份密文另存本机临时恢复目录；用 `/S /UPDATE /P /R /D=<installation-directory>` 原位安装（退出码 0），保留原身份与设置。
- 真实安装态：辅助功能读取得到“测试账户”、已连接官网账号、#[名次已脱敏]、[用量已脱敏] 本周 Token 和北京时间 15:31 最近成功；修复前为 [用量已脱敏]／13:44。生产只读累计从 [用量已脱敏] 增至 [用量已脱敏]，未减少已有记录；历史预算 60,000 已满，live 预算 1,956，证明当天上传不再被历史预算阻断。连接设备数仍为 1，不能宣称 Mac 用量已经合入。
- Mac：未编译、安装或实测。共享文件与公共版本变更保留在任务分支，尚未集成到主线；接手步骤与相同周期/工具核对规则见 `docs/ACCOUNT_SYNC_REPAIR_HANDOFF.md`。官网公开安装包仍为 0.1.23，未发布桌面更新、未 push。后续由用户推送任务分支，Mac 更新后批准设备连接，完成真实双设备汇总、重复事件与重启验证，再更新共仓清单。

## 2026-10-21：Claude 额度改为 Code 订阅登录优先（CHG-20261021-01）

- 用户反馈安装版不显示 Claude 5 小时／7 天重置时间。排查：真实只读诊断显示 Desktop `plan-usage-history.json` 只含已用百分比，无重置时间；带 `resets_at` 的 OAuth 来源此前仅在没有 Desktop 样本时使用，且本机无 `~/.claude/.credentials.json`。Claude Code 日志与 Desktop 日志、网页存储中均无结构化重置时间字段（只统计出现次数，未输出内容）。
- 按用户确认的规则，`claude.rs` 新增 `choose_source`：有效 Code 订阅登录优先，缺失／无权限／过期时回退 Desktop，二者不合并；`source_fingerprint` 与 `read_once` 使用同一选择，保持既有登录切换检测。Desktop 提示补充“登录 Claude Code 订阅账户后可显示重置时间”。先写 3 项失败测试再实现；另在 `claude_usage.rs` 新增默认忽略的真实数据只读诊断 `live_claude_sources`（不输出内容或路径）。
- 检查：`cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 129 通过／8 忽略；`cargo check --locked --offline --all-targets` 通过（既有 `can_submit` 未使用警告）；`npm test` 97/97、`npm run build`、`git diff --check` 通过。真实诊断：今日 Claude Code Token 可读，额度回退 Desktop 记录且重置时间未知，符合无登录预期。
- Mac 未编译验收。
- 用户在独立终端完成 `claude auth login`（`claude auth status` 为 loggedIn: true；凭据含 `user:profile`，约 8 小时后过期，组织与 Desktop 记录相同；只检查权限与过期时间，未读取或输出令牌）。真实只读诊断 `live_claude_sources`：额度来源 verified=true，5 小时剩余 [比例已脱敏]、重置北京时间 10/21 15:49:59，7 天剩余 [比例已脱敏]、重置 10/26 17:59:59；今日 Claude Code Token 可读。只请求 OAuth profile/usage，未调用模型。
- 版本升至 0.1.26（package.json、package-lock.json、tauri.conf.json、Cargo.toml、Cargo.lock）。`scripts/build-windows-release.ps1` 在 Windows PowerShell 5.1 找不到 `System.Security.Cryptography.ProtectedData` 程序集，补充回退加载 `System.Security`。在工具内用 `*>` 重定向运行时，5.1 把 Tauri 的 stderr 信息行当作终止错误，脚本提前退出；但已启动的构建子进程继续完成了签名构建。确认无残留构建进程后，核对产物：`tokendaily.exe` 版本 0.1.26 且含本次新提示文案；安装包 11,333,392 字节，SHA-256 `7656BF627E11F9770DFF4641FD628B12693B69C9C40055A890A77B4A94F1C8C7`；独立 Minisign 校验（ED 预哈希）文件签名与可信注释均有效，密钥 ID 与 `tauri.windows.conf.json` 内置公钥一致。
- 安装：旧 0.1.25 exe/卸载器备份于忽略目录 `.bundle-tmp/pre-0.1.26-install`。首次 `/S /UPDATE /P /R` 未带 `/D`，且安装前注册表没有 TokenDaily 卸载项，安装器装到默认的 `%LOCALAPPDATA%TokenDaily` 并改写注册表；随即停止该副本，以 `/S /UPDATE /P /R /D=D:SoftwareTokenDaily` 重装（退出码 0），注册表 DisplayVersion 0.1.26、InstallLocation 与安装目录键均回到 D 盘，快捷方式原本即指向 D 盘。误装副本（14:15–14:17 新建）未硬删，移至忽略目录 `.bundle-tmp/stray-c-install-0.1.26`。D 盘安装版与构建版仅差 NSIS 3 字节包类型标记；安装器拉起的 D 盘 0.1.26 单实例运行，设置与来源开关文件保留。
- 用户目视确认安装版悬浮窗显示 Claude 5 小时与 7 天重置时间（本会话无桌面截图能力，由用户确认）。未验：OAuth 来源的真实低额度提醒、令牌过期回退的真实时序未验。未提交、未 push、未发布官网。以后在本机构建需用独立终端运行脚本，或不重定向 stderr。

## 2026-10-20：GitHub README 展示改稿获用户确认

- 用户确认采用 GitHub README 排版预览。将精简版首页写入 `README.md`，新增 `assets/readme-app-preview.svg` 作为标明示例数据的界面结构图；核心能力、四个来源的数据边界、Windows 下载入口及 Mac 尚未公开分发的状态保留在首页，开发与参榜细节折叠展示。
- 仅修改文档和示意资源，不改变应用行为或发布状态；Mac 安装包没有上传，官网目前只公开 Windows 下载。检查：`git diff --check` 通过；README 的 17 个链接／图片引用中全部本地相对路径存在；示意图经 XML 解析为有效 SVG。未运行应用测试（无应用代码改动），本轮不 push，待用户自行推送。

## 2026-10-20：README 按本地 main 更新事实说明

- 保留评审前的 README 结构，不合入尚未验收的交互首页改稿。按 `main` 当前提交与功能清单校正 0.1.25 Windows 安装态、设置主窗口、四来源今日汇总、真实额度双窗口、积分旧值恢复、多工具／多设备榜单代码和 Mac 分发边界。同步修正 README 链接指向的 Claude 与排行榜阶段说明；真实 Claude 客户端页面对照、Codex Plus 双窗口和多设备连接端到端仍标未验收。本轮只改文档，不改变应用行为、不发布或 push。

## 2026-10-20：GitHub 首页改稿等待用户验收

- 用户明确尚未验收 `4e12159` 的 README 改稿。完整改稿、交互 HTML、示意图保留于本地 `codex/github-home-review` 分支，并另存忽略目录 `.bundle-tmp/github-home-review` 供直接浏览；本地 `main` 恢复评审前 README 结构，只校正现有 0.1.25 与 Claude 来源事实。预览及其素材从 `main` 撤下，未 push 或发布；用户确认后再依据反馈决定是否合入。

## 2026-10-20：Windows 0.1.25 原生与安装态回归

- 找到仓库已有的 E 盘 `.local-toolchain`，通过 `scripts/use-local-dev-env.ps1` 运行 Rust，不新增 C 盘工具链。`cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 126 项通过、7 项按既有配置忽略；`cargo check --locked --offline --all-targets --manifest-path src-tauri/Cargo.toml` 通过。`npm test` 97 项及 `npm run build` 已通过。`cargo fmt --check` 因仓库已有多文件格式差异未通过，本轮不批量改写无关源码。
- 版本升至 0.1.25，使用既有受保护更新密钥生成签名 Windows NSIS 包与 `.sig`。安装包 11,257,714 字节，SHA-256 `9EE2DE9C245C5716DB3DB9DD3C14BDD8D9864C818B74666B86955F1225B09A03`；使用官网同一 Minisign 校验逻辑验证包内容、公钥和签名版本绑定。旧安装程序备份于忽略目录 `.bundle-tmp/pre-0.1.25-install`，以 NSIS `/S /UPDATE /P /R` 原位更新 `<installation-directory>`；注册表与可执行文件版本均为 0.1.25。安装版与构建版仅相差 NSIS 写入的 3 字节包类型标记。安装器启动的常驻进程使 `Start-Process -Wait` 持续等待，因此未取得安装器独立退出码；改以注册信息、文件版本、字节差异和已安装程序启动核对完成状态。
- 实际安装态悬浮窗显示 Claude Desktop 本机历史样本对应的 5 小时剩余 [比例已脱敏]、7 天剩余 [比例已脱敏]，两项重置时间均显示未知；最新本机样本只读换算结果一致。Codex 当前来源只返回 7 天窗口，界面没有补造 5 小时值。375×675 物理像素（125% 缩放，对应 300×540）截图目视无遮挡；鼠标移出后收回 70×70 悬浮球。设置文件仍在、修改时间早于升级，原有 Claude 来源开关和悬浮窗开关仍开启；实际 Claude 客户端用量页数值尚未人工对照，Codex Plus 双窗口也因来源未返回短期窗而未做实值验收。
- 官网未更新或发布，Git 未 push。Mac Desktop 额度来源未实现且本轮未在 Mac 编译/验收；真实多设备账号连接、长时驻留、异常通知和系统权限路径仍需后续专项验证。

## 2026-10-20：多额度窗口同屏展示

- 悬浮窗每个 AI 仍用一条余额组件；Codex 和 Claude 如实际返回通用短期／长期两个额度窗口，将两个剩余比例、进度及各自重置状态并排显示。其他模型专属或额外额度池显示“另 N 项”，点击进入现有详情。无第二窗口时不补造一个；WorkBuddy/TRAE 积分仍按积分显示。订阅套餐名不作为额度数据源。
- 针对 Codex 同池双窗口、Claude 分键的 5 小时／7 天窗口、额外模型池及未知值加入前端测试。`npm test` 97/97、`npm run build` 通过。Windows 0.1.25 安装态视觉与真实 Claude 双窗口验证见上节；Codex 当前真实来源只返回 7 天窗口，Plus 双窗口仅有合成测试。Mac 实机未验证。


## 2026-10-20：Claude Desktop 订阅额度读取（开发中）

- Windows 接入 Claude Desktop 自写的短期额度历史文件，只读最新样本，显示 5 小时／7 天及存在的模型窗口剩余比例；来源时间超过 10 分钟标旧，超过 24 小时不展示。无样本时尝试现有 Claude Code OAuth 来源。Desktop 文件没有重置时间或精确聊天 Token，界面显示未知；未核验当前账号，通知暂停。
- 已加入版本、无效百分比、账号切换、样本过期和重置时间缺失的 Rust 合成测试。最初因未加载项目 E 盘工具链而暂缓原生验证；Windows 0.1.25 的 Rust 完整测试、编译、签名安装态及本机文件换算核对已在上节完成。真实 Claude 客户端用量页数值尚未人工对照。
- Mac Desktop 路径适配尚未实现，Mac 原生构建及安装态未验收。后续需在 Mac 实机实现与验证对应来源，并在 Windows 对照客户端用量页、复测样本过期与账号切换。


## 2026-10-20：多设备账号用量与全工具榜（开发中）

- 按用户确认的规则实现：游客继续以一台设备参榜；已验证官网账号可连接多台设备，默认补传可读取的历史，按工具与稳定事件键去重，私人历史保留至账号删除。总榜汇总 Codex、Claude Code、WorkBuddy，三种分榜各自授荣誉；本期零用量不参榜。TRAE 因跨设备去重未完成，仍仅本机展示。一次参榜选择覆盖已接入工具的计数，不重复逐来源询问。
- 桌面改动涉及三个来源的历史事件导出、账号连接/自动与手动同步、悬浮总榜展示及状态说明；Mac 官网 URL 白名单允许账号页一次性连接片段。官网独立仓库新增 D1 迁移、官网会话批准连接、账号历史及设备管理 API、公开榜单合并与筛选、双语账号和榜单界面。上传内容仅工具、计数、时间、脱敏事件键、随机设备标识。
- 本机桌面最初 `npm test` 96/96、`npm run build` 通过；官网 `npm test`（46 项）、`npm run release:check`、隔离 D1 及生产 D1 迁移通过。官网 Worker `6e5310cf-5670-499f-996e-de8a7a989e61` 已部署，旧版 0.1.23 的 Codex 游客参榜继续可用。随后使用已有 E 盘 Rust 工具链完成 0.1.25 的 126 项 Rust 测试、全目标编译与 Windows 原位安装，详情见文首回归记录。真实多设备连接和 Mac 实机仍未验证；0.1.25 安装包未更新到官网。


## 2026-10-20：Mac 改动合入本机 main，等待用户 push

- 用户要求将全部已完成改动提交到主分支，并由用户自行 push。本机 `main` 从 `4bf28f1` 合并 `mac/in-app-updates`，生成合并提交 `0fe47d1`；合并后代码树与 Mac 分支一致、工作区无未提交文件。此次未 push、未部署或发布。
- Mac 0.1.24 的前端 96 项、Rust 134 项、Intel App/DMG 与安装态验证见下节。合并本身未改代码，不重复运行测试。Windows 对此合并源码尚未回归，跨平台功能仍按 `FEATURE_PARITY.md` 标记未同步完成。
- 本轮远端拉取未能完成，`origin/main` 仍是本机记录的 `4bf28f1`；用户 push 前应先核对远端是否出现新提交，以免覆盖他人更新。

## 2026-10-20：本地 main 0.1.24 合入 Mac

- 合入本地 `main` 的 `4bf28f1`：主窗口改为设置页、悬浮来源 Logo/详情入口、WorkBuddy/TRAE 最后成功积分缓存与失败后自动恢复。保留 Mac 原生悬浮窗、快捷键、通知、钥匙串 WorkBuddy 凭据、独立 TRAE 网页数据存储和参榜。Mac 积分缓存仅保存规范化余额，文件限定当前用户读取；凭据仍按各自原生存储。新增 Mac 设置窗口尺寸 760×620。
- 验证：`npm test` 96/96、`npm run build`、`bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 134/134（10 项真实环境测试默认忽略）、Rust fmt 与 Git 差异检查通过；Mac 缓存恢复/权限合成测试通过。`bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 生成 Intel x86_64 0.1.24 App/DMG，`hdiutil verify` 成功。仅对构建 App 执行本地 ad-hoc 签名并严格校验；DMG 未包含该事后签名，也没有 Developer ID 签名或公证。
- 安装态：旧 0.1.23 完整 App 与 ZIP 已备份到忽略目录 `.bundle-tmp/mac-validation/before-0.1.24-20261020-160512/`，ZIP 完整性与旧二进制摘要核对通过；正常退出旧版后替换 `/Applications/TokenDaily.app`。安装版版本 0.1.24、严格签名及二进制摘要与构建版一致；启动后仅一个 TokenDaily 进程，主窗口 760×620，AX 可见“设置、数据来源、额度提醒、悬浮窗、通用”。本次新签名触发 WorkBuddy 钥匙串系统确认，用户在系统弹窗选择“始终允许”后，WorkBuddy/TRAE 两份最近成功积分文件均以 0600 权限写入。
- 同版正常退出、重启后没有再次出现 WorkBuddy 钥匙串确认，仍为单实例；两份缓存均再次更新，设置页 WorkBuddy/TRAE 积分卡在 AX 中均显示实时数值而非“上次记录”（未读取/记录具体余额）。悬浮窗通过菜单保持展开，AX 包含顶部今日 Token、费用、排行、四个来源、日历和设置入口；点击 WorkBuddy 来源后主窗口进入“WorkBuddy 详情”。测试后恢复原 Codex 设置视图和已开启的收起悬浮球。官网 Mac 更新清单尚未发布，悬浮更新按钮明确报检查失败；没有测试正式更新或视觉像素布局。
- 边界：`origin/main` 读取返回 HTTP 403，因此本轮同步来源是**本机 main**，不能声称已取得远端最新提交。Windows 对本次合并源码未重新测试；真实断网恢复、官方授权异常、启动瞬间“上次余额”短暂呈现、悬浮 Logo 像素/其他详情入口、Mac 正式签名/公证与 Apple Silicon 仍待验。没有发起模型推理、主动上传用量或发布官网更新；已安装版原有参榜自动同步设置未改变。

## 2026-10-20：暂缓 Mac 付费分发条件

- 用户明确决定在市场验证、确认有真实用户之前，不加入 Apple Developer Program，也不购买其他 Mac 正式分发服务。现阶段保留已完成的 Mac 更新器源码和隔离两版本测试；不申请 Developer ID、不执行公证、不开放官网 Mac 更新清单或宣称公开版一键升级可用。此决定作出时 `/Applications/TokenDaily.app` 是更新按钮禁用的 0.1.23；现已按上方记录换为本地 0.1.24。
- 后续可继续无证书的本机/隔离测试，包括安装位置权限与数据保留，但测试结论须标明临时签名、系统提示和钥匙串确认等限制。公开分发、正式签名、公证和官网升级通道待用户依据市场验证结果另行决定。此次仅更新产品决策文档，没有修改程序、安装版或官网；无需运行代码测试。

## 2026-10-19：Mac 更新器隔离两版本安装态验收（`mac/in-app-updates`）

- 按用户“先在本机验证”的决定，创建独立标识 `app.tokendaily.updater-test` 的 0.1.23/0.1.24 临时 App、只监听 `127.0.0.1` 的静态更新服务和临时 Tauri 测试密钥。测试构建临时覆盖公钥/端点并仅在隔离源码中放行 loopback HTTP、自动触发同一 Mac `check`/`install` 流程；日常版 `/Applications/TokenDaily.app`、其数据目录与官网均未修改。临时源码在验收后恢复为提交中的正式 HTTPS 白名单，私钥已删除，测试服务和测试应用进程已停止。
- 故障路径：服务端保留更新清单和签名，但篡改下载包一个字节。测试版发现 0.1.24 后返回 `InstallError`，本地 App 仍为 0.1.23，没有执行替换。恢复正确包后在 `/private/tmp` 以 Tauri 官方打包器的 `createUpdaterArtifacts=true`、本地 ad-hoc App 签名和测试更新密钥生成 `.app.tar.gz`/`.sig`；测试版由 0.1.23 下载、验证、原位替换、自动重启到 0.1.24，第二次检查为 `Current`。更新后的 `Info.plist` 为 0.1.24，`codesign --verify --deep --strict` 通过，日常安装版进程仍运行。
- 打包与路径发现：最初使用系统 `tar` 手工打包时，macOS 扩展属性产生 `._icon.icns`，虽成功重启到新版但严格代码签名失败；改用 Tauri 官方更新包后通过，故新增仅发布时使用的 `tauri.macos.updater.conf.json`。从 Desktop 工作区内运行的隔离 App 在更新器 `renamex_np` 上停住，退出该测试进程后改在 `/private/tmp` 完成全链路；这不能推断 `/Applications` 的管理员授权路径已通过。临时测试证据在忽略目录 `.bundle-tmp/mac-update-e2e/`，不作为可发布包。
- 剩余：官网 Mac 清单仍 404，正式更新公钥对应私钥在 Windows 发布机，本机没有 Developer ID Application 证书/公证；当前 `/Applications` 仍为旧 0.1.23、更新按钮禁用。待具备正式签名、公证和官网发布条件后，手动引导安装一次新版，再在真实 `/Applications` 路径验收下一次应用内升级、系统授权、设置/钥匙串保留及 Apple Silicon。没有推送、上传、部署或更改日常安装版。

## 2026-10-19：Mac 应用内更新本地验证（`mac/in-app-updates`）

- 用户报告安装版悬浮按钮提示“此平台暂不支持应用内更新”，并确认没有 Developer ID Application 证书，先做本地可验证版本。基于已安装 0.1.23 的对应源码分支，Mac 加入官方 Tauri updater 2.14；固定官网 Intel 清单、同域同版本 `.app.tar.gz`、禁重定向与版本绑定签名。沿用共用检查状态、手动节流、进度与重试。Mac 安装成功后调用应用重启；检查无有效清单时明确报错，当前官网清单实测 HTTP 404。
- 测试先行：新增 Mac 下载 URL 严格校验用例，初次运行因实现尚无校验函数而失败；实现后 `bash scripts/mac-dev.sh cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --features macos-transparent --lib` 130 通过、10 忽略。`npm test` 93/93、`npm run build`、Rust fmt、差异检查通过。`bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 生成 Intel App/DMG；`hdiutil verify` 成功，二进制为 x86_64。构建目录 App/DMG 没有 Developer ID 签名或公证。
- 本地更新包验证：将构建 App 复制到忽略目录并按已有本地授权 ad-hoc 签名，严格签名校验通过；打成 `.app.tar.gz`，用临时测试用 Tauri 密钥签名并绑定 0.1.23。使用与 updater 相同的 `minisign-verify` 实现验签和检查受签名保护的版本字段：正确包通过，篡改包内容及伪报 0.1.24 均失败。测试私钥已删除；临时公钥与正式配置中的 Windows 公钥不同，因此本步骤不等于正式更新通道的安装态验收。产物只在忽略的 `.bundle-tmp/mac-update-local-smoke/`，没有发布或上传。
- 阻塞与下一步：官网 `darwin-x86_64` 清单目前 404；Mac 正式包需要 Developer ID Application 签名、公证、使用正式更新密钥的 `.app.tar.gz.sig`，并在官网增加 Mac 清单。当前 `/Applications/TokenDaily.app` 仍为此前的 0.1.23 旧构建，按钮仍禁用；本节记录的初次验证尚未覆盖 `/Applications` 真实路径及其设置/钥匙串回归，隔离两版本升级结果见上方补录。Apple Silicon 或 Windows 对本分支共用配置的编译也未验证。取得正式发布条件后，先手动安装一次支持更新的 Mac 正式版，再以两相邻版本做安装态升级与设置/钥匙串回归。未推送、发布或公证。

## 2026-10-19：本地 main 0.1.23 同步至 Mac

- 用户要求同步主分支更新。远端 Git fetch 返回 403，连接器查询返回 404，故本轮只以已核对的本地 `main` `e97aa2d` 为基线，不声称已取得远端最新提交。从 `mac/main-0.1.22-sync` 创建 `mac/main-0.1.23-sync`，合入官网 Windows 0.1.22 发布记录和 0.1.23 签名应用内更新。解决 README、交接/架构及三个 Rust 入口冲突，保留 Mac Codex 来源、TRAE 积分、通知、快捷键、参榜与透明浮窗；Mac 更新适配明确返回 unsupported，不注册 Windows 更新插件。
- 共用验证：`npm test` 93/93，`npm run build` 通过；Intel Mac `cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml --lib` 129 通过、10 项按条件忽略，`cargo fmt --check` 与差异检查通过。离线 Rust 测试首次因本机尚无新引入的 `tauri-plugin-updater` 缓存而停止，联网获取固定锁文件依赖后同一测试通过。
- `bash scripts/mac-dev.sh npm run tauri -- build` 生成 0.1.23 Intel App/DMG，`hdiutil verify` 通过；构建目录 App 按已有授权本地临时签名，严格校验通过。旧 `/Applications/TokenDaily.app` 0.1.22 已 ZIP 完整备份并验证于忽略目录 `.bundle-tmp/mac-validation/before-main-0.1.23-20261019/`，原 App 也保留在该目录。新 App 安装后版本、签名和二进制摘要与构建产物一致；五个既有设置文件安装前后摘要一致。
- 安装版已启动，Codex 选择仍为原设置，真实本机 Codex 今日 Token 可读，悬浮球维持 56×56。因本地临时签名变化，macOS 再次弹出 TokenDaily 专属 WorkBuddy 钥匙串确认；用户在系统弹窗选择“始终允许”后提示消失，未读取或代填本机密码。WorkBuddy [余额已脱敏] 与 TRAE [余额已脱敏] 的真实官方积分在主面板和浮窗恢复；300×540 浮窗 AX 树含顶部三项、四来源卡片与 20 周日历，新更新按钮为 disabled，文案“此平台暂不支持应用内更新”。未做新增按钮像素目视验收。
- 正常退出并从 `/Applications` 重启同一版后，WorkBuddy 钥匙串不再提示，两项官方积分自动恢复，无须重新登录。应用内公开参榜身份“测试账户”、当时 #[名次已脱敏]、最近同步成功和每 30 分钟自动同步仍在；没有为验收手动触发上传。测试时临时选择总览后，已恢复原 Codex 视图与 56×56 球体，`providers.json` 摘要与验收前相同；五个设置文件摘要在安装前后及重启后相同。DMG 在构建时产生，签名是在构建后针对 App 完成，不能称 DMG 为正式签名分发包。
- 未对这台 Mac 下载安装远端 Windows 包；Mac 原位更新尚未实现。Windows 对本轮合并后的同一源码未在此机重测，真实 TRAE Token 仍因本机数据库 0 条轮次未验。未推送、发布或公证。

## 2026-10-19：Mac TRAE 官方积分安装态验收补录

- 用户在 TokenDaily 独立 TRAE 官方窗口自行完成登录，未向开发流程提供密码或验证码。安装版随后从官网白名单余额响应显示真实 TRAE 积分 [余额已脱敏]，主面板与悬浮面板均能读取；本地 Token 仍显示未知，因为这台 Mac 的 TRAE CN 数据库目前没有聊天轮次，不能以账号积分推算用量。
- 正常退出并从 `/Applications/TokenDaily.app` 重启同一构建后，TRAE 积分自动恢复，无须重新登录；WorkBuddy 真实积分也自动恢复，未再次出现 TokenDaily 专属钥匙串确认。应用内公开参榜身份“测试账户”、本周 #[名次已脱敏] 与每 30 分钟自动同步状态仍在。展开悬浮窗的原生窗口为 300×540，AX 树含顶部三项、WorkBuddy/TRAE 卡片和 20 周日历，TRAE 积分在 AX 中可见；没有完成新增卡片的像素目视验收。
- 验收后将主面板恢复为原先的 Codex 视图，`providers.json` 的 SHA-256 与验收前完全一致；悬浮窗恢复 56×56 球体。保留 TRAE 本地连接及用户的官方登录授权。没有实际执行“断开并清除授权”，以免要求用户重新登录；清除路径仅有源码/合成测试覆盖。
- 剩余：待这台 Mac 产生真实 TRAE Token 记录后核对精确用量、去重与总历史；Windows 对本次共享源码重测，以及 Apple Silicon、正式签名升级和悬浮新增卡片像素验收。未推送、发布或公证。

## 2026-10-19：Mac TRAE 本地连接与独立官方积分窗口

- 用户确认继续开发 Mac TRAE。沿用共享数据库读取/去重/北京时间归日规则，Mac 允许保存本地连接选择并在启动后恢复；连接前不读取 TRAE 数据，空库仍为未知。积分授权沿用只接收官网余额白名单字段的独立窗口；macOS 14+ 使用固定的专属 WKWebsiteDataStore，低版本明确禁用积分授权，避免回退到默认网页数据存储。断开授权只清理 TokenDaily 的专属存储，不修改 TRAE 客户端登录。
- 源码验证：Mac Rust 126 通过、10 忽略，`npm test` 91/91、TypeScript/Vite 构建、Rust fmt/diff、Intel App/DMG 构建与 DMG 校验通过；构建目录 App 已按既有授权做本地临时签名并通过严格校验。新增测试覆盖 Mac 连接选择恢复但不伪造用量、Mac 积分恢复只进入加载状态而不恢复旧余额。
- 本机 TRAE CN 3.4.4 数据库可只读查询，但目前 `chat_turn` 为 0；未做有真实轮次时的统计验收。新版安装态的 TRAE 连接、官方网页登录/余额、断开清除和重启恢复尚待检查；不能把源码构建写成安装态功能通过。Windows 对本次共享文件变更尚未编译回归。
- 安装进度：用户在应用内自行完成参榜，Mac 页面显示公开昵称、名次、已同步本周 Token 与 30 分钟自动同步；没有代填昵称或代点上传。用户确认可重启后，先将旧安装版完整 ZIP 校验备份到忽略目录，再退出应用、替换 `/Applications/TokenDaily.app`，严格签名及新旧构建二进制摘要匹配通过，原 settings/providers/workbuddy 文件摘要未变。新签名首次启动触发系统 WorkBuddy 钥匙串确认，随后由用户在系统弹窗处理。
- 用户在系统弹窗完成“始终允许”后，WorkBuddy 钥匙串确认消失；已安装 App 保持运行。榜单试用身份从钥匙串恢复，页面显示“测试账户”、本周 #[名次已脱敏]、最近同步成功及每 30 分钟自动同步，不再出现确认参榜入口。切换到 TRAE 设置后，本地连接按钮可用并成功保存连接；积分官方登录按钮随连接启用，打开独立 TRAE 官方登录窗口，AX 确认呈现真实官方登录页。当时等待用户自行登录；其后结果见本页顶部补录，未读取验证码/密码。

## 2026-10-19：本地 main 0.1.22 向 Mac 合并

- 用户要求同步主线更新。以 Mac `cdc6601` 创建 `mac/main-0.1.22-sync`，合入已核对的本地 main `e15a4a8`；远端最新状态本轮未能取得，因此不声称已包含远端未获取的提交。合并保留 Mac 原生快捷键、参榜钥匙串、通知、透明浮窗与自定义 Codex 来源。
- 共用更新包含积分两位小数展示、跨期 0 Token 正常名次、官网公开身份定位、七枚荣誉图标，以及 TRAE 来源代码。Mac TRAE 仍明确 `unsupported`。复查发现本机已安装 TRAE CN 3.4.4 且有本地数据库；只读解密和聚合查询成功，但 `chat_turn` 为 0 条，当前没有可验收的 Token 用量。官网 URL 仅放行公开 UUID 定位片段，Mac 本地参榜令牌不会进入浏览器链接。
- 结果：本地合并提交 `ba4be64`；`npm test` 91/91，`npm run build` 通过；Mac `cargo test --locked --offline --features macos-transparent --lib` 124 通过/9 忽略，`cargo fmt --check` 通过。Rust 锁文件已按两侧依赖重解。Intel 0.1.22 App/DMG 构建、DMG 校验、本地临时签名严格校验通过；旧 0.1.17 已完整 ZIP 备份并替换 `/Applications/TokenDaily.app`。
- 安装态：真实 Codex 只读数据、手动来源、通知设置、快捷键入口、Mac 参榜试用身份恢复与本机上传预览可用；预览不上传，未填写昵称或公开参榜。悬浮窗 300×540，顶部三项、四来源卡片和 20 周日历在 AX 树可见；WorkBuddy 官方积分在主面板与悬浮窗一致且显示两位小数，本机 WorkBuddy 日志不可读时仍显示 Token 未知。签名更新后的首次钥匙串允许未持久生效；用户再次选择系统“始终允许”后，同一安装版退出重启无再次确认，真实积分自动恢复。三份原设置/来源摘要在验收操作恢复后均不变，重复 `open` 仅保留一个实例。
- 限制：用户目视反馈 TRAE 未连接、排行未上榜，与当前 Mac TRAE `unsupported`、试用身份尚未公开相符；本轮未获得独立的像素无裁切确认。TRAE 客户端和数据库存在，但当前 0 条轮次记录，不能验证真实 Token 统计；原生监测及官网积分仍未实现。榜单真实公开上传、安装版临时快捷键重新录入、Apple Silicon、多屏与正式签名升级未验。合并源码尚未在 Windows 编译/安装回归；未推送、合回 main、发布或公证。
- TRAE 只读复查：显式 `live_trae_database` 返回“未发现 TRAE 有效 Token 记录”；新增忽略的 `live_trae_database_record_shape` 通过且只查询聚合数量，确认轮次/上下文/Token 用量对象均为 0，不读取或输出会话内容。复查后 Mac Rust 全量测试 124 通过、10 忽略，`cargo fmt --check` 和 `git diff --check` 通过。这说明当前来源为空，不能据此判断未来有记录时 Mac 采集器已通过实测。
- 快捷键原生专项：在当前 0.1.22 源码上显式运行 `native_conflict_keeps_original_and_drop_releases` 通过，覆盖 Carbon 注册冲突时保留原绑定和释放；这不代替 0.1.22 安装版的录入、重启和物理按键验收。
- 下一步：按用户是否选择公开参榜和是否继续 Mac TRAE 接入处理。若参榜，需要用户核对预览、填写公开昵称并明确同意；若开发 TRAE，需要先完成 Mac 原生实现和合成/真实来源分层验收。
## 2026-10-20 CHG-20261020-02 积分查询失败后的自动恢复

- 完成：WorkBuddy/TRAE 单次官方授权异常保留已知余额并标旧，自动隔一分钟复核；连续确认失效才清除余额并提示重新登录。WorkBuddy 本地授权明确到期立即按失效处理，确认失效时移除 TokenDaily 自有授权文件。网络、服务或页面失败仍每分钟重试；用户手动刷新最少间隔 10 秒。TRAE 显式登录窗口收到未登录响应时继续等待登录和后续有效余额，不将其误判为已失效。浮窗、总览、来源详情和设置的刷新入口覆盖已连接的 WorkBuddy/TRAE 积分；授权中/未授权不发无效查询。共享积分状态字段与余额口径不变。
- 测试：前端 `npm.cmd test` 93 项通过，`npm.cmd run build` 在并行界面修改出现前通过；Rust `cargo test --offline --manifest-path src-tauri/Cargo.toml --lib` 97 项通过、6 项忽略。覆盖一次/连续授权错误、网络故障打断确认链、自动与手动节流、未授权刷新门禁。版本文件已准备为 0.1.24，现有签名目录在本机可见；签名构建首次被并行界面的 TypeScript 必填参数变更阻断，尚无新签名包、安装态测试或发布。Mac 未编译/实测。
- 后续：签名构建与安装态自动恢复已在下方补测；可见旧值文案、真实授权失效及并行界面任务仍需分别验收。官网发布与远端推送尚未进行。

### 0.1.24 Windows 签名安装态补测（2026-10-20）

- 使用原有 `<本机签名目录>` 密钥和 `scripts/build-windows-release.ps1` 生成 0.1.24 NSIS 安装包及同版 `.exe.sig`。首次沙箱构建因 NSIS 下载 DNS 失败，正常网络环境重跑通过；未重建或输出私钥。并行界面样式修改后重新签名打包，本轮**实际安装并验收的包**为 10,549,254 字节，SHA-256 `9C971706E9664FFB594D273962A9140E34D5C70BAFB7C9B5852AF7BE2132CC7F`，签名文件 444 字节。原 0.1.23 程序与卸载器备份在忽略目录 `.bundle-tmp/release-0.1.24-backup`，第一次 0.1.24 构建备份在 `.bundle-tmp/release-0.1.24-first-build-backup`。本轮 NSIS `/S /UPDATE /P` 原位安装退出 0，实际安装文件版本为 0.1.24；安装版再次联网启动，两份真实官方积分缓存均更新。
- 安装版正常联网启动后，两个官方积分来源都成功创建各自 DPAPI 保护的最后成功文件，未读取/输出真实余额或凭据。仅对 TokenDaily 测试进程设置不可达代理（Windows 全局网络未断开），WebView2 两个进程确认带代理标志；在超过 TRAE 页面超时后应用仍运行，两份缓存保留且时间不变。停止该测试进程、以正常环境重新启动后，两个缓存均被新查询覆盖，证明真实积分读取与启动恢复后查询可用。测试后当前运行进程无代理覆盖。
- 同进程恢复补测：只给已安装的 TokenDaily 测试进程指定当时不可达的 `127.0.0.1` 代理，旧缓存保留；随后启动只转发指定官方 HTTPS 域名的临时本机代理，**进程不重启、用户未点击刷新**，WorkBuddy 缓存先自动更新。TRAE 起初未恢复，测试代理记录仅域名并发现其官方页面还需 `lf-cdn.trae.com.cn` 和 `lf-c-flwb.bytetos.com` 静态资源；放行后 TRAE 也在同一进程自动更新。统计域名仍未由测试代理转发。测试后关闭并删除代理及域名记录，TokenDaily 无代理正常重启，两个官方缓存再次更新；Windows 全局网络未改动。此试验验证自动恢复链路，不代表真实断网时每个网络栈错误完全相同。
- 真实断网补测：经用户许可，仅断开当前 WLAN，并在独立隐藏看门狗中设置 70 秒后自动重连兜底；离线启动同一已安装 0.1.24 程序，持续约 32 秒期间程序存活，两份 DPAPI 保护的最后成功缓存均未被清除或改写。恢复 WLAN 后，**程序不重启、用户未手动刷新**，WorkBuddy 与 TRAE 的两份缓存均因重新查询而更新。结束时原 WLAN 已连接、默认路由存在，DNS 与公网 HTTPS 443 连通；只检查缓存文件时间，不读取/输出余额或凭据。测试用脚本在忽略目录 `.bundle-tmp/`，不纳入提交。
- 仍未验收：离线时展开悬浮窗的“上次积分余额”可见文案、真实官方连续授权失效、Mac 实机。并行界面改动也在该工作区内，本轮验收包包含当时可构建的界面状态；该界面任务应独立验收。12:27 并行构建再次覆盖同名安装包与签名，当前磁盘上的包 SHA-256 为 `2970B876B13402DC710BEA7B03D88583B21208F0E697F39CE50C8C6E26CBCE2A`，**不是本轮已安装验收的包**，不得沿用上述安装态结论发布它。未更新官网、未推送 Git。现有 0.1.23 用户不会通过应用更新发现本地 0.1.24，直到另行发布签名包与官网清单。

## 2026-10-20 CHG-20261020-01 两个积分来源只保存最后一次成功查询结果

- 完成：WorkBuddy/TRAE 各用一个 DPAPI 保护文件保存规范化余额、积分包和官方查询时间；成功后覆盖同一文件，不保存完整官方响应、Cookie、令牌或历史序列。启动时仅在对应授权/独立网页配置存在且记录有效时显示“上次积分余额”，使用日期和时间标旧；联网成功后覆盖为实时结果。WorkBuddy 记录绑定当前授权令牌的哈希作用域；TRAE 重新登录、明确失效或断开时清除旧值。两端异常未来时间、损坏与过大记录不作为余额。存档最大 64 KiB/来源；本机 Token 仍不推算积分。
- 改动：`src-tauri/src/credit_cache.rs`、两个积分后端、`src/domain/credits.ts`、相关测试、`.gitignore`、交接与来源/同步文档。沿用原有 CreditState/Balance 字段，不改变 IPC 命令或在线查询口径。现有安装版不会自动获得此功能，需要后续安装新构建。
- 测试：`npm.cmd test` 92/92；`npm.cmd run build` 通过；`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过；`cargo test --offline --manifest-path src-tauri/Cargo.toml` 95 通过、6 忽略；Windows Release 编译成功。`npm.cmd run tauri -- build` 首次在沙箱内因 NSIS 下载 DNS 失败，允许联网重试后 NSIS 包生成，但因未设置 `TAURI_SIGNING_PRIVATE_KEY` 最终返回错误，未形成可发布的签名更新产物。
- 验收边界：仅合成缓存数据参与测试，未读取或记录真实余额。尚未安装本构建、验证实机断网重启/网络恢复和清除授权联动；macOS 积分接入仍禁用且未编译/实测。未部署、推送或签名发布。

## 2026-10-20 Windows 重启后积分与排行榜显示空值排查

- 现象：重启后 WorkBuddy、TRAE 在悬浮窗显示“积分待授权”，本周排行显示“—”；设置页的本机监测连接仍开启。
- 已验证：两个积分连接开关和排行榜身份文件仍在；WorkBuddy 授权与参榜身份可在当前 Windows 用户下解密，WorkBuddy 保存的授权尚未到记录的到期时间。安装版重新启动后，两个官方积分均返回真实余额，排行榜远端也返回有效名次；调试结束已关闭本机诊断端口并恢复正常启动。此次未读取或记录令牌、Cookie、原始日志和余额数值。
- 启动环境线索：系统事件显示 10:46 左右重启，10:47 有 DNS 查询超时；故障截图生成于 10:57。该事件说明启动阶段曾有网络异常，但没有故障当时三个请求的响应记录，不能认定它就是各请求失败的唯一原因。
- 代码确认：本地“已连接”只表示 Token 日志监测开关，并非两个独立官方积分授权的实时状态。积分余额及远端名次均未作为重启后可直接显示的缓存保存，启动后要重新查询；查询失败就显示未知。WorkBuddy 收到授权失效响应会清除**进程内**凭据并停止重试，但磁盘上的加密授权仍保留，故重启可恢复；TRAE 收到 `auth-required` 后自动轮询排除该状态；浮窗“刷新”目前不调用这两个积分刷新命令。排行榜远端查询失败时展示“—”，本地身份仍可保留。
- 结论边界：本次确认账户/身份没有丢失，也没有证据表明官方授权实际过期。最可能是启动时的临时请求失败与现有恢复机制不足叠加；缺少故障瞬间的脱敏错误状态，尚不能确定三个请求各自的精确 HTTP 错误。后续修复宜先区分网络错误与明确授权失效，提供可见错误及积分重试入口，再做断网启动/恢复回归。
- 检查：安装版真实状态只读诊断；Windows System 事件只读核对；源码路径检查。未修改产品代码，未运行构建或自动测试；macOS 未验证。

## 2026-10-18 CHG-20261018-09 浮窗费用、周榜、持续编辑提示

- 完成：编辑动画改为持续轻抖，拖动/退出停止，系统减少动画时静止；新增小字等效 API 预估费用，右上角显示已参榜用户的 TokenDaily 周榜，未参榜隐藏。
- 数据：新增 api_cost 共享规则，读取 Codex 模型与缓存读/写/输出字段，Claude 模型及分时长缓存创建；复用既有去重，费用不进入排行榜上传。官方 Standard/全球路由价格核验于 2026-10-18；未知型号/字段待估价，部分覆盖显式标记，WorkBuddy 不推测计价。详见 API_COST.md。
- 缓存：Codex 本机索引升级 v3，旧缓存安全重建，恢复/跨日测试覆盖；排行榜展开时按需读取，成功结果缓存五分钟，过期/失败不冒充最新排名；刷新与参榜变更使缓存失效。中途关闭未完成读取不误占成功缓存时限。
- 检查：`npm test` 74/74；`npm run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 72 通过、4 忽略；`npm run tauri -- build` 生成 0.1.11 NSIS。
- Windows 安装：覆盖更新至 `<installation-directory>`，旧 exe/uninstall 已备份 `.bundle-tmp/release-0.1.11-backup`，未卸载或清除数据。原生 QA 已验证真实费用带“部分”标记与周榜名次、持续 >2 秒抖动、减少动画静止、拖动停止、退出编辑停止；528px 常态高度、140 格/20 周日历与详情 13 周无溢出。现有三来源连接、WorkBuddy 授权、参榜身份/开关、组件顺序/隐藏偏好与安装前一致。
- 限制：只覆盖已列入本地核验表的型号/别名；当前真实金额是部分估算，不是完整账单。macOS 编译/实机未验证。未部署官网、未提交或 push Git。检查脚本与截图留于忽略目录；临时回环调试验收结束后关闭。

更新：2026-10-15。范围依据根目录三份交接文档；首版是本地自用原型。所有演示数字均与真实数据隔离。

## M0 · 环境与数据验证

- 已完成：检查空工程与 Git 状态（原目录无仓库）；记录 Windows、Node、Codex CLI、WebView2 和 Rust 环境。
- 已完成：用只读 stdio 探测验证本机 ChatGPT 登录能返回 1 个真实额度窗口的字段结构。探测脚本只输出脱敏的字段可用性。
- 已完成：在 `docs/FEASIBILITY.md` 记录已测能力、官方文档线索与发布风险。
- 已完成：经用户授权安装项目隔离的 Rust 1.99.0 和本机 MSVC/Windows SDK，Tauri `cargo check`、单元测试与开发态原生窗口已运行。
- 未完成：与官方 UI 对比真实数值；工作区归属、权限变化与长时间并存尚未验证。VS 安装器提示需重启，当前会话编译已可用。

## M1 · 极简桌面骨架

- 已完成：React/TypeScript 紧凑面板、详情、设置，以及 6 种清楚标记的开发演示状态。生产默认无演示余额。浏览器已检查默认断连及 6 种状态。
- 已完成：Tauri 托盘、单实例、关闭隐藏、手动刷新、设置入口的源码；原生开发窗口已启动。托盘点击和菜单的完整人工验收待安装版。

## M2–M4

- M2 真实桌面适配器、5 分钟刷新与失败退避、本地脱敏快照已实现。Rust 显式真实只读测试通过；缓存重启后标旧。与官方 UI 同时对照尚未完成。
- M3 低额度/确认恢复规则、持久化去重、阈值与开关、原生通知调用已实现并有单元测试。安装态手动测试通知由用户确认可见；通知点击跳转和真实低额度/恢复事件尚未端到端验收。
- M4 `README.md`、`docs/PLAN.md`、`docs/TEST_REPORT.md` 与 NSIS 当前用户安装包已完成；本机安装成功。安装版只读连接后，概览确认脱敏身份、1 个真实额度窗口和重置时间，无演示标记；未输出真实余额。关闭窗口仍驻留；重复启动仍为单实例并重新显示。开机启动往返测试后恢复关闭；清除本地设置/快照/提醒账本成功，Codex 登录仍可用，随后重新连接成功。用户确认安装态测试通知可见。托盘菜单、显式退出、通知点击、缩放与非管理员账户测试待做。

## 测试与命令

| 命令 | 结果 |
|---|---|
| `node scripts/probe-codex.mjs` | 沙箱内访问拒绝；受控沙箱外只读探测成功。只记录结构，不记录真实余额 |
| `npm test` | 12/12 通过：百分比、缺失/非法字段、窗口去重、时间、身份隔离、低额度和恢复规则 |
| `cargo test --offline --manifest-path src-tauri/Cargo.toml --lib` | 9 通过、1 个需本机登录的真实契约测试默认忽略；新增首次连接失败状态回归测试 |
| `cargo test --offline --manifest-path src-tauri/Cargo.toml --lib live_read_only_contract -- --ignored` | 受控沙箱外通过；只断言脱敏身份、能力状态与窗口存在 |
| `npm install --cache .npm-cache --no-audit --no-fund` | 已完成，`package-lock.json` 已锁定依赖 |
| `npm run build` | 通过；包含 TypeScript 检查和 Vite 生产前端构建 |
| `cargo check --manifest-path src-tauri/Cargo.toml` | 通过，包含提醒与开机启动插件 |
| `npm run tauri dev` | 原生窗口已启动；开发态不作为系统通知验收 |
| `npm run tauri build` | 最终 Release 与 2.46 MiB NSIS 包构建通过。首次遇到 NSIS 临时目录跨盘错误，第二次官方 NSIS 下载超时；校验官方 NSIS 缓存及辅助 DLL 后通过 |
| `TokenDaily_0.1.0_x64-setup.exe /S` | 当前用户静默安装退出码 0；HKCU 卸载项和 `%LOCALAPPDATA%/TokenDaily` 安装目录存在 |
| 安装版 UIA/Win32 手工检查 | 真实额度结构、关闭隐藏、单实例重新打开、开机启动往返、清除本地数据/保留 Codex 登录通过；用户确认测试通知实际可见 |
| 安装版进程树采样 | 主进程 5 秒 CPU 增量 0；7 个进程专用内存约 201.5 MiB。仅为本机空闲快照 |

## 技术决策与数据边界

- 采用 Tauri 2 + React/TypeScript + Rust。设置、规范化快照和提醒账本规模很小，使用应用数据目录中的 JSON，暂不引入 SQLite。
- 额度按来源的 `usedPercent` 转换为剩余百分比；不从 Token 数量推算额度，不固定窗口种类。
- 原始账户响应和凭据不进入项目文件；探测只输出结构。真实桌面适配器在用户连接后读取实际额度；开发演示参数下的虚构数字明确标记且不会进入生产默认界面。
- 开机启动默认关闭，开发态禁止启用，避免把未安装的调试路径注册为启动程序。
- 官方 App Server 命令仍被标注实验性；真实路径适合目前本地原型，正式发布门槛待进一步验证。

## 阻塞与下一步

1. 在安装态补测托盘菜单/显式退出、通知点击跳转及真实低额度/恢复事件；测不同缩放、非管理员账户及长时间后台运行，未测不得标通过。
2. 在相同身份和时间下与官方 Codex 界面核对真实额度；确认工作区与 Token 日期范围前不显示未经证实的指标。
3. 重启 Windows 后确认 VS 工具链仍可用；若未来正式发布，重新评估 App Server 授权与接口稳定性。

## 磁盘位置检查与开发环境调整 · 2026-10-15

- 用户要求：C 盘空间有限，后续工程文件尽量放在其他盘。本工程继续固定在 `<desktop-project>`；当前目录仍无 Git 仓库，保留已有文件。
- 本机只读测量（文件逻辑大小，非实际释放空间；并非全盘扫描）：E 盘 `src-tauri/target` 约 8.30 GiB、`.local-toolchain` 约 4.44 GiB、`node_modules` 约 0.09 GiB；源码、`dist` 与安装包也在工程目录。`vswhere -all -products '*' -property installationPath` 确认 Build Tools 主安装位置在项目 `.local-toolchain/vs-buildtools`。
- C 盘仍有：`C:\Program Files (x86)\Windows Kits` 约 1.68 GiB，`C:\ProgramData\Microsoft\VisualStudio\Packages` 约 1.25 GiB，`C:\ProgramData\Package Cache` 约 0.47 GiB，`C:\Program Files (x86)\Microsoft Visual Studio` 约 0.14 GiB。它们可能包含共享或此前已有组件，不能全部归因于本工程。Windows SDK 注册表路径确认为 C 盘。
- 已安装 TokenDaily 位于 `%LOCALAPPDATA%\TokenDaily`（约 0.01 GiB），本地应用/WebView2 数据 `%LOCALAPPDATA%\app.tokendaily.local` 约 35.97 MiB；Roaming 下同名设置目录小于 0.01 MiB；Tauri 工具缓存 `%LOCALAPPDATA%\tauri` 约 0.01 GiB。未读取账户数据内容。npm 用户缓存目录部分拒绝访问，未将其测量当作完整结果。检查时 C 盘剩余约 19.80 GiB。
- 新增 `.npmrc` 与 `scripts/use-local-dev-env.ps1`，更新 `README.md`。开发前运行 `. .\scripts\use-local-dev-env.ps1`，当前终端及其后续子进程的 Cargo/Rustup、编译输出、npm 缓存和 TEMP/TMP/TMPDIR 均指向 E 盘项目目录；不改系统环境或共享 Codex 配置。
- 已验证：加载脚本前 `npm config get cache` 已返回 E 盘 `.npm-cache`；加载后 7 项环境变量均在 E 盘，`cargo --version`、`rustc --version` 正常；`npm run build` 的 TypeScript 检查与 Vite 构建通过。
- 本次没有迁移或删除 C 盘系统组件、缓存或已安装应用，没有重跑业务单元测试、Rust 完整编译、NSIS 打包或 Windows 安装验收；没有新增真实或演示额度数据。剩余 C 盘内容保持现状，若要释放空间，需另行通过对应安装器评估迁移/清理。

### 后续：VS 下载缓存迁移

- 用户继续要求将 C 盘相关内容迁到其他盘。按 [Microsoft CachePath 文档](https://learn.microsoft.com/en-us/visualstudio/install/configure-policies-for-enterprise-deployments?view=visualstudio) 准备并执行 `scripts/move-vs-cache.ps1`：仅迁移 VS 的 `C:\ProgramData\Microsoft\VisualStudio\Packages` 到 `<VS-download-cache>`，不动 Windows SDK、`C:\ProgramData\Package Cache` 或已安装 TokenDaily。
- 脚本检查安装器未运行、目标路径无重解析点，复制后逐文件比较 SHA256/长度/相对路径并复核源目录未变化。目标 ACL 为 SYSTEM/Administrators 完全控制、Everyone 读取执行；通过 HKLM 的 `CachePath` 指向 E 盘。清理前检查 VS 实例仍完整可用。
- 第一轮复制及 SHA256 比对通过，CachePath 已切到 E 盘；随后 Windows PowerShell 5.1 的默认编码损坏 `vswhere -utf8` JSON，导致实例验证异常退出，C 盘源副本完整保留。已修复为显式 UTF-8 解码，并加入受控 `-Resume`、源清单复核和验证失败时回滚注册表的逻辑。
- 中间状态：继续迁移一度等待 Windows UAC 确认；批准后因 PowerShell 5.1 的 `Get-PSDrive` 可用空间字段为空提前退出，C 盘源副本仍保留。后续排查同时发现 PS 5.1 将 JSON 数组作为单个管道对象返回，修正为直接赋值，并使用 `System.IO.DriveInfo.AvailableFreeSpace` 读取空间。增加错误日志，修订后的 PS 5.1 语法、823 条清单解析与空间读取检查通过。
- 已验证：修订后的 PowerShell 脚本语法检查通过；加载本地环境后 `cargo check --offline --manifest-path src-tauri/Cargo.toml` 通过。未重跑业务单元测试、打包或安装态验收；本次不涉及额度数据。
- **最终完成（2026-10-15 19:32）**：管理员 `move-vs-cache.ps1 -Resume` 退出码 0；823 个文件、1,339,321,417 字节逐文件 SHA256/长度/相对路径复核通过。C 盘原目录已删除，HKLM CachePath 确认为 `<VS-download-cache>`，目标 ACL 符合前述权限要求。操作前后 C 盘空闲空间增加 1,343,696,896 字节（约 1.25 GiB；瞬时差值可能包含系统同时写入的变化），复核时剩余约 20.91 GiB。
- 迁移后独立验证：`vswhere` 返回本工程 E 盘 Build Tools 实例，`isComplete=true`、`isLaunchable=true`；`cargo check --offline --manifest-path src-tauri/Cargo.toml` 通过。安装器更新/修复操作和 Windows 安装态验收未重跑。SDK、独立的 `C:\ProgramData\Package Cache`、VS 安装器和已安装 TokenDaily 均未移动。
- 清单、前置状态、执行日志及最终 `result.json` 位于忽略目录 `.local-toolchain/cache-migration/`。本次修改限于迁移脚本、README 和进度记录，保留其他并行产品改动；当前仍无 Git 仓库。E 盘新缓存是机器级 VS 安装缓存，不能随项目构建产物一起清理。

## 每日 Token 与用量趋势 · 2026-10-15

- 用户新增要求：每日 Token 消耗、用量趋势。原交接文档暂缓复杂趋势图，本轮明确授权后增加详情中的轻量图表；首页继续以剩余额度和重置时间为主。未加入截图中的等效 API 费用，因为来源未提供账单金额，也不属于本次明确要求。
- M0 数据核验：`account/usage/read` 真实只读探测返回 67 个日期唯一的日桶，ISO 日期与非负 Token 数量均有效。响应没有时区或覆盖范围字段，所以产品标注“来源日记录”；不声称本地今日、Codex 全量或实际费用。
- M2 数据实现：Rust 适配器在同一只读连接中查询额度和日用量；用量有单独的 5 秒等待上限，失败不会丢弃已获取的额度。两种指标分别记录成功时间和过期状态；切换身份不沿用旧用量。只缓存规范化日桶、脱敏身份和状态，不保存原始响应或凭据。
- M1/M4 界面：详情新增每日 Token、最近 30 天趋势、13 周日历；缺失日期与明确返回的零值分开。首页只有单行来源日摘要。开发演示的用量明确标记；生产版忽略 `?demo=` 参数。
- 已验证：`npm test` 14/14；`cargo test --offline --manifest-path src-tauri/Cargo.toml --lib` 14 通过、1 个真实契约测试默认忽略；显式真实契约测试通过；`npm run build` 与 `cargo check --offline` 通过。420 像素浏览器演示界面视觉检查通过，点击日历零用量格显示 0 且无横向溢出。
- 已验证：此轮 Release/NSIS 构建成功；新包大小 2,774,181 字节，SHA-256 `885A94A3D7B586A93945D2340A49D69AEA081223A71E784A0B3016B843714AAC`。本机当前用户覆盖安装退出码 0，安装版显示真实额度和来源日用量；缓存包含 67 个规范化日桶，额度与用量身份一致、分别为可用状态，字段白名单检查通过。生产前端打开 `?demo=normal` 仍为未连接状态，未显示虚构数字。
- 待解决：官方日期时区、产品覆盖范围与历史保留范围仍未知；不将来源日桶称作本机今日或 Codex 全量，不与本机日志合并。长期后台运行、缩放、独立标准用户及此前未验收的托盘/通知点击事项仍见上文。
- 其他检查：`cargo fmt --check` 最终通过；当前目录仍不是 Git 仓库。安装版进一步切换日历的 Windows 无障碍检查遇到自动权限审批超时，未据此标记安装版日历点击通过；浏览器演示交互已验收。

## Windows SDK 迁移 · 2026-10-15

- 用户明确授权将本工程安装的 Windows SDK 从 C 盘迁到其他盘。目标为机器级 `<Windows-SDK-directory>`，不将 SDK 随项目临时文件清理。
- 现有 SDK 版本为 `10.0.26100.7705`（卸载注册表 DisplayVersion 为 `10.1.26100.7705`）；`Include` 只有 `10.0.26100.0`。32/64 位 `KitsRoot10` 均指向 C 盘。完整 `Windows Kits` 目录另有 `8.1` 子目录，不把该目录视为本次目标直接删除。
- 从本机 VS 的 `_package.json` 读取精确版本清单：229 个文件、529,683,969 字节。复用已缓存文件，并从微软 `download.visualstudio.microsoft.com` 补齐缺失文件；全部长度/SHA256 通过，`winsdksetup.exe` 的 Microsoft Corporation Authenticode 签名有效。离线文件和清单保留在 `.local-toolchain/sdk-migration/`，先准备完再卸载。
- 实测安装程序 `/help` 支持 `/installpath`、`/features`、`/uninstall`、`/quiet`、`/norestart`、`/log`。采用 VS 包清单中原有的 10 个组件选项；备份当前选项和 Windows Kits 的 32/64 位注册表。使用原版 SDK 安装器卸载重装，不手工剪切 SDK 文件或伪造 KitsRoot10。
- 新增 `scripts/prepare-sdk-migration.ps1` 和 `scripts/move-windows-sdk.ps1`；前者准备离线包，后者检查环境、签名和所有 payload，再卸载旧 SDK 并安装到 E 盘。失败时尝试用离线包恢复到原位置；不自动重启。脚本语法检查已通过。
- 首次尝试：旧 SDK 卸载退出码 0；新路径安装被安装器以 1001 拒绝，日志报告仍检测到旧路径上的安装。脚本自动恢复到 C 盘原版本成功（回滚卸载/安装均为 0，无需重启），原始日志保留。
- 原因核验：同批安装的 `Windows SDK AddOn` 10.1.0.0 被 SDK 卸载逻辑保留；VS 的开发用 `Universal CRT Redistributable` 10.0.26624 MSI 还拥有 `KitsRoot10` 注册表组件。只读检查 MSI 表确认其文件位于 Windows Kits 的 Redist/Catalogs，非 Windows 系统目录里的运行库。两个组件安装日期均为 2026-10-15，AddOn 的唯一依赖为本 SDK，CRT 的依赖为本 VS 实例。
- 修订迁移流程：SDK 卸载后，在 AddOn 没有其他依赖时通过 MSI 卸载 AddOn 和上述开发用 CRT 包，再安装 SDK 到 E 盘并恢复同版本 CRT 包及其原 VS 依赖关系；不直接改写 KitsRoot10。CRT 的 4 个离线文件和签名另行校验，失败仍尝试恢复原位置。第二轮安装日志按 `attempt-*` 子目录隔离；不将空闲 msiexec 服务误判为正在安装，改查活跃安装事务。
- 进一步核验：MSI 日志显示开发用 UCRT 包的 `WixDependencyCheck` 因 VS 依赖而提前结束，但进程仍返回 0；所以前两次所谓 CRT 卸载并未实际移除它。增加卸载后注册状态和共享组件客户检查，确认唯一剩余路径组件客户为该 CRT 包。第三次检查拒绝清理仍有引用的登记，自动回滚又被同一检查阻断，出现过 `RecoveryRequired` 状态。
- 查阅 [WiX 原始实现](https://raw.githubusercontent.com/wixtoolset/wix3/develop/src/ext/DependencyExtension/ca/wixdepca.cpp)，确认可以用 `IGNOREDEPENDENCIES` 指定依赖标识；准备仅针对本 VS 标识临时重装并恢复相同版本/依赖关系的方案。但**自动审批拒绝了执行**：SDK 当时尚未恢复，临时移除 VS 依赖包可能进一步破坏共享编译环境，超出常规迁移授权。被拒绝的命令没有执行；未使用其他途径绕过。
- **等待额外授权时的状态：SDK 未迁移，已安全恢复到原 C 盘。** 改为直接运行已校验的原版 SDK 安装器，在 C 盘恢复原组件，退出码 0，无需重启（2026-10-15 20:05）。AddOn 10.1.0.0、开发用 CRT 10.0.26624 仍在，32/64 位 KitsRoot10 正确；12 个原有 Installed Options 全部保留。后续成功结果见下文。
- 恢复后验证：C++ `windows.h` 编译、kernel32 链接、调用 `GetCurrentProcessId` 的程序运行全部通过；`cargo rustc --offline --manifest-path src-tauri/Cargo.toml --bin tokendaily -- -C debuginfo=1` 实际重新编译/链接 TokenDaily 通过。未运行业务单元测试或 Windows 安装态验收，未修改产品代码/额度数据。
- 检查时 C 盘剩余约 20.67 GiB。之前已迁移的 VS 下载缓存仍指向 E 盘；本次 SDK 未释放空间。SDK 离线恢复文件和执行日志保留在 E 盘 `.local-toolchain/sdk-migration/`；`result.json` 已更新为 `RestoredOriginal` 且 `MigrationCompleted=false`，早期失败日志保留在各 attempt 目录。
- 迁移脚本默认会在卸载前检查 VS 依赖并停止；临时重装依赖包的方案通过 `-AllowDependentPackageReinstall` 显式开关隔离，尚未获单独批准、未运行。修正回滚流程，不再让仅用于迁移的所有权检查阻止恢复原路径。后续需用户明确决定是否授权临时重装这项 VS 依赖的开发用 CRT 包，再继续换盘。脚本最终语法检查通过。
- 用户随后明确回复“同意”，授权临时移除、重装这项 VS 依赖的开发用 UCRT 包并继续迁移。已在原 SDK 恢复且编译验证通过的状态下，以 `-AllowDependentPackageReinstall` 执行。首轮依赖卸载成功，但 COM 在组件完全移除后抛异常；自动恢复成功。改用 [MsiEnumClientsW](https://learn.microsoft.com/en-us/windows/win32/api/msi/nf-msi-msienumclientsw) 区分“不存在/无引用”与真正错误，新增 `scripts/sdk-installer-state.ps1`；存在、不存在、非法参数三种只读检查通过，PS 5.1 下存在/不存在检查也通过。
- **最终迁移成功（2026-10-15 20:17）**：`attempt-20261015-201401` 的 SDK 卸载、AddOn 卸载、开发用 CRT 卸载、SDK 安装、CRT 恢复均退出码 0，无需重启。确认旧路径组件无 MSI 客户后才清理残留路径值，再由微软安装器登记新路径；32/64 位 `KitsRoot10` 均为 `<Windows-SDK-directory>\`，原 VS 依赖标识已恢复。
- 版本和组件保持：SDK `10.0.26100.7705`、AddOn `10.1.0.0`、开发用 CRT `10.0.26624` 均存在；原有 12 个 Installed Options 全部保留。E 盘开发用 CRT DLL 存在，不涉及 Windows 系统目录中的运行库。
- 空间复核：旧 SDK 文件逻辑大小 1,796,253,453 字节（约 1.67 GiB），原 C 盘 `Windows Kits\10` 目录内剩余文件为 0 字节。本轮 C 盘空闲空间增加 1,861,992,448 字节（约 1.73 GiB，含系统同期临时文件变化），最终检查剩余约 22.13 GiB。之前迁移的 VS 缓存仍在 E 盘。
- 最终验证：MSVC 开发环境自动选择 E 盘 SDK；C++ `windows.h` 编译、kernel32 链接及 Windows API 程序运行通过；`cargo rustc --offline --manifest-path src-tauri/Cargo.toml --bin tokendaily -- -C debuginfo=1 -C link-arg=/INCREMENTAL:NO` 实际重新编译/链接通过。未重跑业务单元测试、NSIS 打包或安装态验收，本次无产品代码/额度数据变更。
- 交付文件：迁移脚本与 MSI 状态辅助脚本、README、FEASIBILITY 和本进度记录；日志、恢复包、最终 `result.json` 与 `verification.json` 在 `.local-toolchain/sdk-migration/`。保留 E 盘离线包用于恢复；SDK 和机器级 VS 缓存不要作为项目临时文件清理。此项迁移已无阻塞，未来升级 VS/SDK 后重新核对路径。

## 2026-10-15 用户追加：可开关悬浮窗

- 范围变更：用户明确要求“通过悬浮窗来实现，设置中可设置开关”，覆盖此前交接基线中的“不做悬浮窗”。其余产品、数据和隐私边界保持原约定。
- 实现：新增 300×250 逻辑像素的独立 Tauri 悬浮窗、无系统标题栏、置顶、不占任务栏，顶部可拖动，↗ 打开主面板，× 关闭并保存开关。主面板设置新增“悬浮窗 / 显示悬浮窗”；旧设置默认关闭，用户开启后重启恢复。
- 数据：两个窗口共用 Rust 状态及事件；悬浮窗不新增轮询或读取 Codex 请求。手动刷新、连接/断开、清除本地数据和提醒阈值变化会广播同步。延用原有未知、过期、低额度、重置等待确认规则，不创建虚构窗口或额度。窗口可滚动承载更多来源窗口。
- 持久化：新增 floating_enabled 及切换串行化，保存失败报告错误并回退窗口状态；清除本地数据同步关闭悬浮窗，清除命令在异步执行上下文运行，避免窗口切换期间阻塞 UI。窗口位置只在本次运行中保留，未增加跨重启位置恢复。
- 文件：src/ui/FloatingPanel.tsx、App.tsx、styles.css、main.tsx；src-tauri/src/lib.rs、tauri.conf.json、capabilities/floating.json；README、PROGRESS、TEST_REPORT。
- 先写测试：旧设置兼容/开关跨加载保存、保存失败上报，初次执行因新字段和错误返回尚未实现而失败；实现后通过。最终 npm test 为 14/14；cargo test --offline --manifest-path src-tauri/Cargo.toml 为 16 通过、1 个真实接口专用测试按默认忽略；本次未单独运行该忽略测试。npm run build、cargo fmt、npm run tauri build 均通过。
- 浏览器验证：300×250 下单窗口、双窗口、低额度、字段未知、过期、未连接六场景均无横向溢出，提示正确；420×620 设置开关可切换。演示显式标记，仅开发环境有效。
- Windows 安装态验证：安装器退出 0；真实 WebView 确认读取共享额度、开/关设置落盘、× 后主窗口开关同步、原生窗口关闭消息同步关闭开关。原生鼠标拖动移动 80×60 物理像素；WS_EX_TOPMOST 确认置顶；关闭主面板后悬浮窗仍可见，↗ 能重新打开主面板；进程重启恢复已开启状态。当前系统缩放下实际内容为 300×251 CSS 像素，无横向溢出，页脚完整；桌面截图确认无可见系统标题栏（Windows 底层仍有 caption 样式位，不能仅据该样式位判断可见边框）。
- 验证脚本/截图在被忽略的 .bundle-tmp/ 内。仅测试时临时启用本机 WebView 调试端口，交付前正常重启关闭调试。工程和构建缓存继续留在 E 盘；应用沿用原安装位置。
- 未测限制：多显示器切换、不同 DPI 间拖动及长期后台资源占用；本次没有触发真实低额度/恢复通知，不宣称完整回归全部历史待验收项。无功能阻塞。
- 最终交付复核：最终 NSIS 包再次安装退出 0，正常模式启动后悬浮窗可见且置顶，保存开关为 true；本机临时调试端口 9223 已关闭。已为用户开启悬浮窗，可在设置中随时关闭。

## 2026-10-15 用户追加：悬浮球与悬停展开

- 按用户要求，将默认常显面板改为 48 像素圆形悬浮球（原生透明窗口 56×56 逻辑像素）；鼠标悬停 180ms 后展开 300×250 额度面板，移出 500ms 后收起，返回面板会取消收起。窗口实际缩小，不以大面积透明窗口挡住桌面。
- 球体及面板顶部均可拖动，拖动时暂停展开/收起计时。保留置顶、设置开关与持久化；重启恢复球体。键盘聚焦可展开，Escape 可收起。
- 展开按显示器工作区限制位置，支持负坐标和缩放换算；收起恢复球体锚点。Windows 原生缩放过程中可能产生额外移出事件，收起前核对实际鼠标位置以避免误收起。仅悬浮窗获得调整大小和位置权限。
- 文件：新增 src/domain/floating.ts、src/ui/useFloatingHover.ts、tests/floating.test.ts；修改 FloatingPanel.tsx、App.tsx 设置说明、styles.css、main.tsx、tauri.conf.json、capabilities/floating.json、README 与测试/进度记录。未改变额度读取或提醒规则。
- 测试先行：新增测试先因实现模块不存在而失败；实现后 npm test 18/18 通过，覆盖悬停延迟、快速移入移出、返回取消收起、拖动抑制、键盘焦点、重置/销毁取消计时、屏幕边缘/负坐标/过小工作区。类型检查、Vite 生产构建、Tauri release 和 NSIS 打包通过。Rust 业务代码未变，本轮未重跑 Rust 单元测试。
- 浏览器：默认球体 48×48，命中窗口 56×56，透明背景；展开无横向溢出，返回面板取消收起通过。开发演示仍明确标记，安装版继续使用本机 Codex 真实来源。
- 安装态：更新安装成功；Windows 原生鼠标验收完整通过：初始球体、悬停展开、持续悬停无闪烁、移入面板保持、移开宽限期、自动收起、右下角向内展开及收起恢复锚点、球体拖动后正常收起。当前缩放实测球体原生窗口 70×70 物理像素，展开 375×313；右下角面板限制在工作区 1920×1020 内。临时测试脚本位于 .bundle-tmp/verify-ball-native.ps1。
- 已知限制：多屏跨 DPI 的真实拖动尚未实测；只测试了当前 Windows 缩放及当前屏幕工作区。球体位置依旧不跨重启保存。
- 补充检查：安装版真实 Tab 键聚焦展开、Escape 收起通过（程序式 focus 不等同键盘聚焦）。正常模式首次创建时曾被 Windows 最小宽度扩为 170×70 物理像素，已增加显式最小尺寸和启动尺寸校正；最终重装复核结果见下。
- 最终安装复核通过：安装器退出 0，正常模式首次启动即为 70×70 物理像素，无透明宽度残留；原生完整悬停/收起/边缘/拖动验收再次全部通过。悬浮球保持开启，临时调试端口已关闭。

## 2026-10-15 修复：拖动后偶发无法收起

- 用户反馈拖动悬浮窗后偶发无法收回。检查发现原实现完全依赖 DOM 移开事件，原生拖动后事件缺失就无法启动收起；此外原生鼠标位置检查取消收起时，控制器已提前记为收起，短时间再次移出可使实际窗口和控制器不一致。原生焦点恢复也可能保留键盘展开状态。
- 修复：新增仅悬浮窗可调用的 get_floating_pointer_state，在展开或拖动时以 150ms 间隔核对窗口内外、可见性及主鼠标键当前状态；真实松开后结束拖动，移出后安排收起。收起静止时不发送原生状态查询，不增加 Codex 请求。Windows 使用 GetAsyncKeyState 的当前按下位并尊重主副鼠标键互换，不保存鼠标坐标。
- 控制器连续收到同一状态不会反复延后 500ms 计时；原生切换成功、失败、被取消时同步实际展开状态。拖动清除键盘保留状态，原生焦点恢复不会伪装成 Tab 键盘导航；Tab 展开与 Escape 收起保留。
- 文件：src/domain/floating.ts、src/ui/useFloatingHover.ts、src/ui/FloatingPanel.tsx、src-tauri/src/lib.rs、tests/floating.test.ts、PROGRESS、TEST_REPORT。额度与提醒规则未改变。
- 测试先行：补充丢失移开事件、按住拖动不收起、重复采样不延后计时、键盘状态清理、移回取消收起、原生收起被取消后的同步回归测试。新增用例先失败，实现后 npm test 22/22 通过；cargo test --offline --manifest-path src-tauri/Cargo.toml 为 16 通过、1 默认忽略；cargo fmt、TypeScript/Vite 生产构建、Tauri release 和 NSIS 打包通过。
- 安装版专项：临时屏蔽 mouseout/mouseleave/pointerout/pointerleave，三轮展开面板原生拖动后仍能收起，含屏幕边缘；收起后 1 秒的原生鼠标查询增量为 0。真实 Tab 展开 / Escape 收起通过。临时测试脚本在 .bundle-tmp/verify-drag-collapse.ps1，其余最终验收见下。
- 限制：仍未验收多显示器跨 DPI、触摸输入及长时间后台运行；本次未执行默认忽略的真实接口测试或触发额度通知。
- 最终正常事件验收：连续六轮展开面板原生拖动全部通过，覆盖长按、松开即移出、停留后移出、右下角展开后拖动。正常重启清除了临时事件屏蔽和调试注入；悬浮球可见、置顶、尺寸 70×70 物理像素，调试端口已关闭。修复版已更新本机，保持用户悬浮窗开启设置。

## 2026-10-15 用户追加：悬浮窗用量 / 额度双模式

- 用户确认先改悬浮窗。默认用量模式，顶部按钮直接切换；额度模式突出剩余百分比与重置时间，用量模式突出来源日 Token 数字。主面板首页布局保持不变，设置增加悬浮窗模式选择；两窗口同步，重启恢复选择，清除本地数据重置为用量模式。
- 保持原生 56×56 球体 / 300×250 展开尺寸、拖动区域、悬停和收起控制器。两种模式均展示额度与 Token；多窗口在额度区域滚动，底部保留低额度/受限文字，避免滚动隐藏风险。球体状态点也反映用量过期和额度风险。
- 数据：继续使用共享状态，不新增 Codex 请求、模型调用或遥测。Token 保留来源日期，不宣称今日全量或各工具分项；万/亿缩写明确单位，悬停与无障碍名称提供完整数量。缺失为未知，显式 0 保留；用量独立判断新鲜度，额度旧值旁标记“上次剩余”。
- 持久化：增加枚举 floating_mode 和 get/set 命令，旧设置缺省 usage；设置写入串行化，成功落盘后才更新内存与广播。失败保留原模式并显示重试信息，非法模式拒绝，不改动连接/提醒/悬浮开关。
- 文件：新增 src/domain/floatingUsage.ts、src/ui/useFloatingMode.ts、tests/floating-usage.test.ts；修改 FloatingPanel.tsx、App.tsx、styles.css、src-tauri/src/lib.rs；更新 Handoff、README、PROGRESS、TEST_REPORT。
- 测试先行：来源日期/零与缺失、用量独立新鲜度、单位边界以及旧设置默认/双模式持久化测试先因实现缺失失败，实现后通过。npm test 25/25；cargo test --offline --manifest-path src-tauri/Cargo.toml 为 17 通过 / 1 默认忽略；cargo fmt、TypeScript/Vite、Tauri release/NSIS 打包通过。所有构建与缓存留在 E 盘。
- 浏览器：300×250 下六种场景 × 两种模式共 12 组，单/双额度、低额度、字段缺失、过期、未连接均无横向溢出，底部状态完整；显式零、大数量/万亿单位边界和最大安全整数显示检查通过；两种模式截图已目视检查。
- Windows 安装版：NSIS 更新退出 0。真实共享数据展示、默认用量、顶部双向切换、设置页同步、落盘与重启恢复 quota 通过。临时将应用自己的 settings.json 设为只读，实测保存失败后仍为原 quota 模式且提示“模式未保存，请重试”；finally 已恢复原属性。非法模式命令拒绝通过。未修改 Codex 文件。
- 原生交互：额度模式和用量模式各完整通过一轮拖动、长按保持、松开与移出收起；用量轮包含右下角展开。真实 Tab 展开 / Escape 收起通过。连续测试曾出现悬停未展开或未移动的失败，增加窗口激活和鼠标到位等待后仍有间歇命中失败，不能宣称连续六轮回归通过；未把这些结果归因为产品缺陷或用户操作。悬停控制器本轮未修改，其自动化单元回归均通过。
- 已知限制：未验证多屏跨 DPI、触摸与长期运行；未触发真实通知或执行默认忽略的真实接口专项；为保留数据未在安装版执行清除数据。当前 Token 日期时区及产品覆盖范围仍待来源确认。
- 最终交付：正常重启后 mode=usage、floating_enabled=true，悬浮窗可见且置顶；settings.json 非只读，临时 9223 调试端口已关闭。已安装供本机使用，无发布或上传。

## 2026-10-16 悬浮窗顶部切换与手动刷新

- 按用户确认的方案实现：用量/额度切换移至顶部栏，品牌区域保持拖动，右侧保留打开主面板和关闭。删除原独立切换行；底部增加“↻ 刷新”，额度与 Token 各自显示最近成功时间、未知、待更新或失败信息。风险提示独立保留，不被更新时间覆盖。
- 手动刷新复用原 Codex 只读流程，同时尝试获取额度和 Token；不新增模型调用、遥测或自动轮询。正常约 5 分钟一次的自动刷新及失败退避保持原规则，展开/切换模式不触发读取。
- 刷新状态通过 Rust refresh-status 事件和 get_refreshing 初始查询在两个窗口同步；按钮读取中禁用，前端同步 ref 防止同一帧重复点击，后端保留原子并发保护。手动命令返回的旧缓存不再覆盖共享事件送达的新结果。通信异常同时标记两个指标失败并保留各自旧值。
- 文件：src/ui/FloatingPanel.tsx、App.tsx、styles.css、src-tauri/src/lib.rs；新增 src/domain/refresh.ts、tests/refresh.test.ts；更新 README、PROGRESS、TEST_REPORT。
- 测试先行：新增独立指标更新时间、部分失败、缓存未刷新、未知/未连接/缺失/未来时间测试，先因模块未实现失败，实现后 npm test 27/27。cargo test --offline --manifest-path src-tauri/Cargo.toml：17 通过 / 1 默认忽略。cargo fmt、TypeScript/Vite、Tauri release/NSIS 构建通过，工程和缓存仍在 E 盘。
- 浏览器：300×250 下 9 场景 × 2 模式（单/双窗口、低额度、字段未知、过期、未连接、仅额度失败、仅 Token 失败、刷新中）共 18 组；顶部按钮位置、底部完整、无内容横向溢出通过。刷新中/未连接按钮禁用且不调用回调；部分失败两行分别展示。演示图片已目视检查。
- 安装态：NSIS 更新退出 0。真实共享数据刷新后额度、Token 成功时间均前进；主窗与悬浮窗同步显示刷新中；同一帧连续三次点击只观察到一组 true/false 刷新事件。结束后按钮恢复。两种模式的顶部切换、底部信息与真实截图目视检查通过。
- 交互：Tab 展开 / Escape 收起通过；单独执行的一轮原生右下角展开、品牌区域拖动、长按保持、移出收起通过。前一轮连续脚本在长按阶段测到小球宽度而失败，未确认是桌面指针干扰还是产品问题，不宣称连续多轮通过。悬停/拖动控制器本轮没有修改。
- 限制：部分失败在浏览器与现有 Rust 合并测试覆盖，未人为破坏真实 Codex 连接制造失败；未执行默认忽略的接口专项、多屏跨 DPI、触摸、长期运行或真实通知专项。
- 最终交付：正常重启后悬浮窗可见且置顶，模式 usage、开关 true；临时调试端口 9223 已关闭，开发服务器已停止。本机更新完成，未发布或上传。

## 2026-10-16 来源日期落后于本地日期的排查

- 用户指出本地已为 10/16，悬浮窗仍显示来源日 10/15，同时底部显示“刚刚更新”。本次只排查，没有修改或重新安装产品。
- 09:30（Asia/Shanghai）检查应用缓存：09:29 仍成功读取，最新来源日期为 2026-10-15。随后独立启动只读 App Server，调用 account/read（refreshToken:false）、account/rateLimits/read、account/usage/read；09:30:55 返回的最新日期仍为 10/15，与应用一致，未返回 10/16 日桶。未记录原始响应、身份或凭据。
- 当前流程：读取账户日桶 → 日期排序 → 展示最后一个有记录的日期；读取成功会更新本地 collectedAt/lastSuccess 时间，即使日桶日期与数值未变化。界面“刚刚更新”实际代表刚成功读取，并非统计已经覆盖本地今天，存在语义缺陷。
- 官方 App Server 文档 Token usage 段说明日桶 startDate/tokens，但未说明该字段时区或实时性保证：https://learn.chatgpt.com/docs/app-server#7-token-usage-chatgpt 。本次响应也无时区/覆盖范围字段，不能断言固定延迟一天或 UTC 切日，更不能改日期把历史数值当作今日。
- 后续修正方向：区分“最近读取时间”和“统计所属日期”，明确最新来源日及当日记录尚未返回；若要求本地自然日的及时用量，必须另行核验来源能力与统计口径，不能仅缩短轮询。只读排查脚本位于忽略目录 .bundle-tmp/probe-usage-date.mjs；没有更改自动刷新规则。

## 2026-10-16 修正来源日与读取时间混淆

- 根据用户授权修改显示语义，不更换数据来源、不改写历史日期或计数，不调整刷新频率。用量标题显示实际来源日，如“10/15 Token 记录”；当前本地日期尚无同名来源记录时，两种模式都显示“尚未取得 10/16 日记录”。不将“已取得同名日期”解释成今日全量/实时统计。
- 读取状态统一为“刚刚读取 / N 分钟前读取”，读取失败与保留上次值仍分别显示。主窗口用量摘要与详情同步显示来源日期、缺失提示和最近读取时间。
- 测试先行：新增 tests/usage-date.test.ts，先因缺少日期展示函数失败；更新读取文案测试先失败。实现后 npm test 31/31，TypeScript/Vite 通过。覆盖本地零点、不同本地时区、跨年、来源日期超前、当天零值、缺失和旧数据；日期比较不重解释来源时区。
- 浏览器 300×250 下 6 种日期/状态 × 2 种模式共 12 组通过：标题正确、缺失提示与底部完整，无用量区域横向溢出。新日期记录到达时自动清除缺失提示；不补零。
- 文件：src/domain/usage.ts、refresh.ts，FloatingPanel.tsx、App.tsx、UsagePanel.tsx、styles.css；tests/usage-date.test.ts、refresh.test.ts。Rust 业务代码未改，本轮不重复运行 Rust 单元测试。安装态结果见下。
- 安装交付：Tauri release/NSIS 构建通过，安装器退出 0。真实手动刷新后额度与 Token 读取时间均前进，但来源最新日期仍为 10/15；两种悬浮模式正确保留“10/15 Token 记录 / 尚未取得 10/16 日记录”，底部为“刚刚读取”。主窗口摘要及详情分别展示来源日、最近读取时间，通过安装态断言和截图检查。
- 原刷新回归：同帧三次点击只有一组刷新事件，两个窗口忙碌同步，完成后按钮恢复。拖动/悬停控制器未改，本轮未重复原生拖动专项或 Rust 单元测试；多屏 DPI、长期运行仍未测。
- 最终正常重启，悬浮窗保持开启且置顶，用量模式保留；临时调试端口和开发服务器已关闭。来源数据延迟/时区问题本身尚未解决，本次修正的是准确展示日期覆盖与读取新鲜度，不宣称新增实时今日统计能力。

## 2026-10-16 实时用量方向调研

- 已核对用户指定的生财 TokenRank 和 AI 破局统计规则/安装说明：两者的 Codex 数据均来自本机日志，站方说明默认约 30 分钟同步，并非以账户日汇总接口保证实时。
- 已只读抽查本机最近 6 个会话文件，发现本地 10/16 的 token_count，最新写入距检查不足 1 秒；同时发现累计值重复与缓存包含关系，证明本机有及时来源，也证明需要去重和增量统计。
- 建议将本机今日用量作为用量模式主来源，账户额度保持独立接口；以“日志事件落盘后 5 秒内可见”为待实现验收目标。具体范围、证据与待测边界见 FEASIBILITY 最新小节。
- 本轮没有改动或安装 TokenDaily，没有运行产品构建/测试，没有安装、运行或接入竞品采集器；浏览器与本机只读验证完成。实时采集尚待实现，不把可行性验证当作已交付。

## 2026-10-16 本机今日 Token 与北京时间切日

- 用户明确要求：今日用量改读本机 Codex 日志，固定北京时间 00:00 切日，新一天无使用显示 0。已更新交接基线，覆盖旧设备时区建议。
- 新增 Rust local_usage 采集器：使用有效 CODEX_HOME（未设置时回退用户目录 .codex）的 sessions 与 archived_sessions；首次流式读取日志，之后每 2 秒枚举元数据并只读新增字节。只解析用量和回合标识，不修改日志、配置或登录，不上传。当天计数/事件哈希仅在内存，未往 C 盘复制日志或新增磁盘索引。
- 按 token_count 的 last_token_usage 输入加输出计数（输入已含缓存，推理属于输出子项），total_token_usage 只用于抑制重复快照；处理计数器重置，排除上下文填充值；事件时间/回合/计数指纹防止复制会话、归档移动重复相加。日期取事件时间的 UTC+8，自然日变化清零，长会话继续累计当日新事件。
- 真实验证发现 Windows 文件修改日期可停留在昨日，而事件内已有今天记录；因此取消按文件日期过滤，并增加回归测试。首次读取全部现存文件，之后靠文件长度/元数据及偏移跟进；未写完的最后一行等待后续追加，异常保留部分/上次数据并明确标旧，未知不变成零。
- 后端独立 local-usage-updated/get_local_usage，账号接口和本机统计互不覆盖；账号错误不停止本机采集。手动刷新唤醒日志检查并保留原账号刷新流程。断开连接停止采集并清空本机内存状态，重连重建；清除本工具数据不删除 Codex 日志。
- 首页/悬浮窗使用本机今日数字并注明含缓存、北京时间；详情分为本机今日与账号来源日统计。前端一秒检查日期，健康采集器跨零点显示 0；休眠/失败后的旧日期不冒充今日。原账号日统计只在详情保留，绝不加到今日数字。
- 文件：src-tauri/src/local_usage.rs、local_usage_tests.rs、lib.rs、Cargo.toml/Cargo.lock；src/domain/localUsage.ts、tests/local-usage.test.ts；App.tsx、FloatingPanel.tsx、UsagePanel.tsx；README 与交接基线。
- 验证：npm test 34/34；cargo test --offline --manifest-path src-tauri/Cargo.toml 最终 24 通过、2 默认忽略；cargo fmt、TypeScript/Vite、Tauri release/NSIS 成功。只读 live_local_logs 另外显式执行通过：499 个日志文件，约 1.88 GB；优化后 debug 首扫 17.44 秒，紧接增量扫描 36ms（测试样本，不代表所有机器或长期 SLA）。曾因同时运行真实测试导致 Windows 测试可执行文件被占用而链接失败，待其结束后串行重跑已全部通过。
- 已安装：NSIS 退出 0。安装版读到真实 2026-10-16 本机计数，验收时为 [用量已脱敏] Tokens；两种模式、首页、详情来源分离、手动刷新、本机自动采集及真实额度接口检查通过。WebView 时区临时改为洛杉矶后，统计日期仍为北京日期，随后已恢复。截图 local-usage.png / local-quota.png 在 .bundle-tmp，已目视检查，无数字/底部溢出。
- 限制：按有效日志目录统计本机现存记录，不承诺当前账号/所有设备完整用量；删掉的日志不能在重启后恢复。继承事件依赖原时间与回合信息去重；极旧格式缺失回合时使用时间与计数指纹。日志超过单行 16 MiB 或格式不支持会标为不完整。自动增长依赖 Codex 先写入有效事件，不能统计尚未落盘的生成中 Token。未修改系统时间来等待真实零点，切日使用固定时钟测试；未跑通知、跨屏 DPI 或长时间休眠专项。
- 补充安装态观察：未点击刷新、未发起测试推理的 25 秒观察中，本机计数从 [用量已脱敏] 自动增至 [用量已脱敏]；采样时最近成功读取距当前约 179ms。证明自然使用会自动更新，但这不是日志落盘到 UI 的全链路 SLA 测量。
- 交付收尾：正常启动已安装应用，悬浮窗开关 true、模式 usage 保留；临时 WebView 9223 调试端口确认不再监听。未发布或上传。

## 2026-10-16 日志采集性能优化

- 用户批准减少日志轮询开销并优化启动扫描。新增 log_watch.rs：Windows ReadDirectoryChangesW + 内核事件等待，500ms 合并连续变化；完整目录补查由 2 秒改为 30 秒。手动刷新立即唤醒；北京零点独立唤醒，不等 30 秒。连接关闭时取消并等待未完成目录请求，释放本工具监听句柄。
- 实测 Windows 缓存中的持续追加可能暂不触发目录通知（FindFirstChangeNotification 与 ReadDirectoryChangesW 都观察到），因此没有仅依赖通知：每 2 秒最多检查最近 8 个日志的长度，不遍历目录、不读取正文；长度变化才触发增量采集。刷新到磁盘后的通知测试通过。目录监控失效仍有 30 秒补查；未声称每条生成中 Token 都即时可见。
- 新增归一化统计索引：日期、计数、读取偏移及路径/文件头/回合/事件哈希，不保存对话正文、原始路径或凭据。启动校验来源、版本、校验和和文件头，正常仅继续读取新增内容。索引损坏、来源不匹配时安全重建；跨日复用读取位置、清零当日累计。读取不完整不覆盖有效索引。仅有变动时至多每 30 秒保存一次。
- 安装态发现 Rust fs::rename 在本机应用目录同目录操作也返回 Win32 17（跨卷），临时索引无法提交；独立 Rust 探针复现，.NET Move 成功，具体环境原因未断言。因此最终采用 a.json / b.json 两个校验槽交替写入并 sync_all，按校验通过的最大序号恢复，不依赖重命名。单份最大 8MiB，实际约 280KiB；清除数据同时删除两槽及遗留临时文件。
- 前端本机统计的过期阈值调整到 90 秒，避免 30 秒空闲补查间隔被误判为故障。补充健康空闲、北京零点、索引恢复/损坏回退/来源隔离/不保存正文/增量追加/读数去重、原生通知及手动唤醒测试。
- 已测开发版本同批真实日志：503 个文件；全量扫描 17,437ms，第二次扫描 43ms，索引恢复 174ms（解析日志正文 0 字节，但仍读取少量文件头校验），单槽索引 284,925 字节；100 次活跃文件长度检查共 62ms。该测量针对日志读取阶段，不是应用完整启动时间；最终双槽会多读一份小索引，待安装验证。
- 安装版监听调度原型已验证两模式、首页/详情、手动刷新、真实额度以及北京时区口径。自然产生的一条用量事件到统计读取约 301ms，非全局 SLA。30.1 秒、8 个进程（含 WebView）的短采样 CPU 总计 1.8125 秒：约单核 6.02%、整机 0.376%；进程工作集相加 499.9MiB（含共享页重复，不是独占物理内存）。这批资源数据在最终双槽保存修改前取得，不能当作长期或最坏情况保证。
- 文件：local_usage.rs、local_usage_tests.rs、log_watch.rs、lib.rs、Cargo.toml/Cargo.lock、localUsage.ts、local-usage.test.ts、App.tsx、FloatingPanel.tsx、README。35 项前端测试通过；Rust 最近一次完整运行 27 通过/2 默认忽略，真实本机日志测试另行显式通过。最终重建、安装与双槽重启恢复结果见下方补充。

### 最终安装验证补充

- 最终 Rust 完整测试 27 通过、2 默认忽略；前端 35/35；fmt、TypeScript/Vite、Tauri release/NSIS 均通过，最终安装包约 3.46MiB。真实日志专项已显式通过。Windows 通知测试使用已刷新到磁盘的写入；未刷盘写入延迟由活跃文件长度检查覆盖，不假称通知一定及时。
- 双槽索引已在实际安装目录写入，验收时 a/b 各约 306KB，两份合计约 0.59MiB；C 盘没有复制原始日志。旧 .tmp 失败产物已定点清理。重启前两槽均存在，重启后用量仍为正确北京日期、真实数据继续增长，双模式/主页/详情/手动刷新/账号额度均通过。重启验收脚本曾过早连接尚未注入 Tauri 桥的 WebView，已改为等待 URL、桥与 React 界面就绪，再跑通过。
- 断开时目录请求取消并等待完成；退出进程时系统释放其句柄，清除本工具数据会删除 a/b 索引及旧临时文件；没有删除或修改 Codex 会话。异常中断写入可回退另一槽的逻辑已用损坏最新槽测试验证，未做断电破坏测试。
- 最终版本资源采样：30.09 秒、8 个进程（TokenDaily + WebView 等子进程），CPU 总计 1.1094 秒，单核等效 3.69%、按本机逻辑处理器数折算整机约 0.23%；工作集相加 548.0MiB，含共享页重复，不等于独占物理内存。采样含自然使用，不能当作长期上限或与旧版的严格对照实验。
- 交付完成：正常重启后应用在运行，floating_enabled=true、mode=usage，临时 WebView 9223 端口确认未监听。未发布或上传。仍未做真实断电、跨多设备、长时间休眠与实际午夜驻留专项。

## 2026-10-16 悬浮窗贴边隐藏

- 用户新增功能：设置中独立“贴边隐藏”开关，旧设置默认关闭，保存后两窗口同步，重启保留；保存失败不修改运行态，清除本工具数据恢复默认。
- 左右工作区边缘 16 逻辑像素范围内吸附，鼠标移开约 500ms 收成 10×56 逻辑像素的窄边，悬停约 180ms 展开，点击也可展开，Escape 收回。拖离恢复普通球体；关闭开关立即恢复可见球体/面板。顶部与底部不吸附，位置和吸附侧不跨重启保存。
- 使用原生窗口缩窄与工作区定位，窄边完全保留在当前屏幕内。串行处理尺寸/位置变化，原生拖动中不收起；尺寸查询完成后再次检查拖动状态，避免异步查询期间开始拖动导致尺寸突变。隐藏或已稳定的球体不增加持续原生鼠标查询。
- 文件：src/domain/floating.ts、src/ui/useFloatingHover.ts、useFloatingEdge.ts、FloatingPanel.tsx、App.tsx、styles.css；src-tauri/src/lib.rs、tauri.conf.json；tests/floating.test.ts；README 与交接文档。
- 验证命令：npm test 37/37；cargo fmt；cargo test --offline --manifest-path src-tauri/Cargo.toml：28 通过、2 默认忽略（本轮不重跑真实日志/账号接口专项）；npm run tauri build 成功，最终 NSIS 约 3.58MiB。项目与编译缓存仍在 E 盘。
- 初轮安装态：125% DPI、1920×1020 工作区，左右分别 3 次悬停展开/离开隐藏通过；左右关闭开关恢复 70px 球体通过；真实鼠标从右侧面板拖离边缘后收球通过。窄边实测宽 13px，展开宽 375px。设置与窄边截图已目视检查。
- 初轮自动化有回拖球体未产生位移、另一次悬停未展开；后续读取到指针已在测试目标之外，尚不能将两次失败全部归因于应用或桌面输入干扰。增加前台稳定等待与失败指针坐标记录，最终安装态重跑结果待补充。
- 重装最终版后，原已启用的 floating_edge=true 被正确读回。未改变日志采集和额度接口；界面继续使用真实来源，不添加演示数据。多显示器跨 DPI、任务栏动态变动与长时间休眠未做实机专项。

### 贴边隐藏最终交付检查

- 最终包已安装（NSIS 退出 0）；重装后通过运行态接口确认原 floating_edge=true 保留，验证重启持久化。
- 最终版原生鼠标脚本在左侧悬停检查时记录：窄边位于 (0,360)，预设鼠标目标 (6,390)，断言时实际指针 (332,660)。该次失败受到指针离开目标影响，不能宣称整套连续拖动脚本全部通过。初轮已通过的左右各 3 轮悬停/收回、开关恢复与面板拖离记录仍有效；球体回拖专项未完成稳定验收。
- 最终版使用已安装 WebView 界面激活窄边并 Escape 收回，连续 3 轮通过；关闭开关恢复原生 70×70px 球体，主设置页 checkbox=false 同步正确、mode=usage 保留。该步骤是界面事件验收，不冒充真实鼠标拖动。
- 最终开关恢复默认关闭，用户可在“设置 → 悬浮窗 → 贴边隐藏”启用。无发布/上传；没有新增演示数据或改变原始日志。下一步为用户正常使用反馈；有双屏设备时补做跨屏 DPI/任务栏变动专项。
- 收尾确认：正常启动的安装版在运行；floating_enabled=true、floating_edge=false、floating_mode=usage；临时 WebView 9223 端口已关闭。

## 2026-10-16 品牌 Logo（第一版）

- 用户确认第一版 TokenDaily Logo。早期确认图保留为 `assets/brand-logo-v1.png`；主窗口现直接使用用户提供的完整透明 SVG 字标，确保名称、渐变和图形比例与确认稿一致。
- 修正：此前 `assets/icon.svg` 是近似重绘，与正式标记不一致，已被用户提供的原始透明 SVG 完整替换。原始透明版与白底版分别保留为 `assets/TokenDaily_logo_gradient.svg`、`assets/TokenDaily_logo_gradient_white.svg`；悬浮球、悬浮窗、托盘、窗口与安装包使用透明版，不再使用近似路径。
- 已重新生成 Tauri Windows 图标（ICO、32/64/128/256 PNG 与 AppX 图标），并目视核对生成的图标保留圆环缺口、独立圆点与中间方块；`npm.cmd run tauri -- icon assets/icon.svg` 通过。
- 验证：`npm.cmd test` 37/37 通过；加载本地开发环境后 `npm.cmd run tauri -- build` 通过，更新后的 NSIS 安装包为 `src-tauri/target/release/bundle/nsis/TokenDaily_0.1.0_x64-setup.exe`（4,932,646 字节）。未重新安装、发布或上传。

### Windows 内嵌图标刷新

- 用户发现窗口/任务栏仍显示旧图标。根因是 Tauri 生成的 Windows 资源只监听 `tauri.conf.json`，不监听 `icons/icon.ico`；此前即使替换 SVG 并重新打包，仍会复用上午生成的旧 `resource.lib`。
- 修复 `src-tauri/build.rs`：显式声明 `cargo:rerun-if-changed=icons/icon.ico`。重新构建后 `resource.rc` 的生成时间更新为 14:53；直接从新 EXE 与 D 盘已安装 EXE 提取的内嵌图标均为正式的粗环、缺口、独立圆点与中间方块标记。
- 已将更新后的 NSIS 安装包原地覆盖至 `<installation-directory>`，安装后 `tokendaily.exe` 时间为 14:54:36、大小 13,993,472 字节，并触发 Windows 外壳图标刷新。`npm.cmd test` 37/37 通过；Tauri 前端生产构建与 Windows/NSIS 构建通过。透明正式 SVG 的 SHA-256 为 `898EC1EE5BECEC79174FEDFA671E07902185ACA199E4D2E9195B3292900DA428`；未修改额度、本机日志、设置或用户数据，未发布或上传。

### 品牌区完整显示与图标缓存

- 主窗口字标来自用户确认的留白 PNG；原裁切窗口将图形上沿贴在容器边缘。已把顶部栏/字标容器从 38px 调整到 42px，并重新定位原图，使正式标识四周保留可见安全边距，不重绘或替换确认稿。
- 已重新打包并原地覆盖 `<installation-directory>`；当前 EXE 时间为 15:10:34、大小 13,997,056 字节。无需卸载。安装后执行 Windows 图标缓存清理和显示刷新；内嵌图标仍直接取自已核验的正式 ICO。
- 验证：Tauri/NSIS 生产构建通过，`npm.cmd test` 37/37 通过。未修改额度、本机日志、设置或用户数据，未发布或上传。

### 完整字标 SVG

- 用户补充正式完整字标的透明/白底 SVG，分别归档为 `assets/TokenDaily_full_logo_gradient.svg` 与 `assets/TokenDaily_full_logo_gradient_white.svg`。主界面改用透明版；白底版不在浅色 UI 中叠加使用。
- 删除 PNG 裁切定位：字标容器按 150×42 逻辑像素容纳 SVG 并保持其原始比例，避免顶部和右侧（`y`）截断。紧凑的托盘/悬浮窗/EXE 图标继续使用仅图形版，保证小尺寸可辨。
- 新 NSIS 包于 15:28 生成并已原地覆盖 `<installation-directory>`；已安装 EXE 时间为 15:28:00、大小 14,087,680 字节。`npm.cmd test` 37/37 通过。未做发布或上传。

### 应用、安装器与卸载器图标统一

- 用户反馈资源管理器仍显示旧 Logo。本轮直接提取已安装 EXE 的 16/32/48/256px 图标，与正式 `src-tauri/icons/icon.ico` 比较，四种尺寸均一致；修改前 Windows Shell 图标查询也返回新版，因此未把截图问题断言为 EXE 内嵌资源错误，具体旧显示可能与资源管理器视图缓存有关。
- 发现 NSIS 的 installerIcon / uninstallerIcon 尚未配置，卸载器使用默认图标。修改 `src-tauri/tauri.conf.json`，两者明确指向正式 `icons/icon.ico`；沿用已有 build.rs 图标变更监听。
- 验证：`npm.cmd test` 37/37；加载 `scripts/use-local-dev-env.ps1` 后执行 `npm.cmd run tauri -- build`，TypeScript、Vite、Windows release 与 NSIS 打包通过。生成的 installer.nsi 已包含两个正式图标路径。未修改业务代码，本轮未重跑 Rust 单元测试或额度/通知专项。
- 安装包原地覆盖 `<installation-directory>`，安装器退出 0；新应用 EXE 时间 15:39:56、卸载器时间 15:40:32。已重启应用，并发送逐文件 Shell 更新、图标关联刷新与 ie4uinit -show；未删除全局图标缓存文件或重启 Explorer。
- 安装后应用与卸载器四种尺寸的提取图像哈希均与正式 ICO 对应尺寸相同，Windows Shell 返回的图标已目视核对为新版。临时验证脚本与 PNG 位于 `.bundle-tmp/inspect-icons.ps1`、`.bundle-tmp/refresh-app-icons.ps1` 及同目录。尝试读取现有 Explorer 窗口时访问被拒，未取得用户当前资源管理器窗口的截图，不宣称该视图已目视验收；若已打开窗口仍保留旧图，可刷新或重开文件夹。
- 本轮只调整打包图标，沿用原有真实数据来源，无新增演示数据、发布或上传。根目录未初始化 Git，无法提供 Git diff；保留既有文件与修改。可继续运行 `<installation-directory>\tokendaily.exe`。

### 悬浮球 Logo 占比调整

- 按用户提供的豆包悬浮标参考，提高收起状态的 Logo 占比。修改 `src/ui/styles.css` 的悬浮球图标尺寸，由 29×29 改为 44×44 逻辑像素；正式 SVG 自带留白后，可见图形约占 48px 球体直径的 75%（原约 50%）。保持球体大小、正式图形和其他位置的 Logo 不变。
- 加载 `scripts/use-local-dev-env.ps1` 后运行 `npm.cmd run tauri -- build` 通过，含 TypeScript、Vite、Windows release 和 NSIS。纯 CSS 调整，本轮未新增或重跑业务单元测试；未改变真实数据来源或添加演示数据。
- 已原地覆盖安装 `<installation-directory>`（NSIS 退出 0）。安装版 WebView 实测球体 48×48、图标 44×44、devicePixelRatio=1.25；截图 `.bundle-tmp/ball-logo-installed.png` 已目视检查，完整显示且留有边距。此前直接桌面截屏捕获到下方内容，未将该图片用于视觉验收。
- 验收后正常重启安装版，临时 9223 调试监听已关闭。运行路径仍为 `<installation-directory>\tokendaily.exe`；未发布或上传。拖动、贴边隐藏与其他 DPI 本轮未专项复测。

### 悬浮球点击展开与贴边半 Logo

- 用户确认交互：普通悬浮球只点击展开，鼠标离开约 500ms 收起；贴边隐藏时保留半个 Logo，悬停约 180ms 展开，离开约 500ms 收回。普通球悬停不再展开；左右边缘开关仍持久化。
- `src/domain/floating.ts` 增加点击/拖动判定（5 CSS 像素容差）、显式点击激活与贴边状态，半 Logo 窗口宽 28、高 56 逻辑像素。`useFloatingHover.ts` 先区分点击与拖动再调用原生拖动，松手不误激活；半 Logo 拖动前恢复完整球体并约束到工作区；保留原生鼠标补查和单个可取消计时器。
- `FloatingPanel.tsx` 与 `styles.css` 用原有 48px 球体/44px 正式图标裁出一半；窗口本身完全在工作区内，避免越过屏幕或进入相邻显示器。设置说明、README、Handoff 和 tauri.conf.json 最小宽度同步更新。未改变额度、Token、通知或数据来源，没有新增演示数据。
- 测试先行：新增点击开关、未贴边悬停不展开、拖动释放不展开、切换贴边取消等待、按压微移/其他指针/取消/丢失移动事件、半 Logo 尺寸用例；新增导出尚未实现时先失败。最终 `npm.cmd test` 42/42；`npm.cmd run build` 与加载本地环境后的 `npm.cmd run tauri -- build` 通过，含 TypeScript、Vite、Windows release、NSIS（约 4.89MiB）。Rust 业务代码未变，本轮未重跑 Rust 单元测试或真实通知/账号接口专项。
- 已覆盖安装 `<installation-directory>`（安装器退出 0）。125% DPI、1920×1020 工作区下原生鼠标专项完整通过：普通球持续悬停保持 70px、点击展开 375px、离开恢复 70px；原生拖动位置变化且松手不展开；拖到右边收成 35px，连续两轮悬停展开/离开收回；直接拖半 Logo 离开边缘恢复 70px；拖到左边收成 35px，悬停展开/离开收回；关闭贴边开关恢复普通球且悬停仍不展开。
- 首轮点击断言失败，补查 WebView 收到的原生指针事件后单独点击成功；脚本改为点击前再次将鼠标定位到球体，再完整运行通过。没有断言首轮失败的唯一原因。临时验收脚本 `.bundle-tmp/verify-click-dock.ps1`，左右截图 `half-logo-left.png` / `half-logo-right.png` 已目视核对。
- 验收恢复原有设置：floating_enabled=true、floating_edge=true、floating_mode=usage。正常重启后安装版在运行，临时 9223 调试端口已关闭。运行路径仍为 `<installation-directory>\tokendaily.exe`。未验收多屏跨 DPI、触摸输入及长时间休眠；未发布/上传。

### 吸附仅由拖动到边缘触发

- 按用户要求移除“窗口距边缘 16 逻辑像素以内”的吸附判定。现在仅在一次实际拖动结束时，鼠标指针位于左右工作区边界并松手，才产生新吸附；靠近边缘停放、点击展开/收起、开启贴边开关不会新建吸附。已吸附的半 Logo 交互保留。
- 文件：`src/domain/floating.ts` 增加显式拖动意图及边界判定；`src/ui/useFloatingHover.ts` 取消位置检查触发吸附，改为消费拖动结束事件；`src-tauri/src/floating_drag.rs` 在悬浮窗所属 UI 线程记录原生拖动结束瞬间的指针、工作区与移动/取消状态，窗口销毁时释放回调。不安装全局鼠标钩子、不持久化坐标；其余采集路径不变。`lib.rs` 安装窗口回调，`Cargo.toml` 仅启用现有 windows-sys 依赖所需的 API 特性。README、设置说明及 Handoff 同步更新。
- 测试先行：替换旧距离阈值用例，新增真实边界/附近不吸附、负坐标屏幕、工作区外/任务栏、不处于显式拖动、重复事件、取消/未移动/关闭开关测试；实现前缺失导出失败。`npm.cmd test` 43/43；`npm.cmd run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`（28 通过/2 默认忽略）；加载本地环境后的 `npm.cmd run tauri -- build` 全部通过，NSIS 约 4.99MiB。
- 已原地安装（退出 0）。125% DPI、1920×1020 工作区实测：鼠标在 x=1885 / x=35 松手，仍为 70px 完整球且离开后位置不变；在 x=1919 / x=0 松手，收成 35px 半 Logo；拖离恢复完整球；附近关闭/开启贴边开关不吸附。拖动结束事件记录的实际落点与上述值一致。临时脚本 `.bundle-tmp/verify-edge-trigger.ps1`。
- 较长的旧交互脚本两次在点击检查时记录鼠标离开目标，未将这两次作为通过；首次缩短脚本的右侧附近实际落点也受到干扰，已重跑并核对所有原生结束坐标后通过。本轮未宣称完整旧点击/悬停交互脚本重新通过；这些交互的既有单元测试通过。
- 原有 floating_enabled=true、floating_edge=true、floating_mode=usage 保留；安装版正常重启，临时 9223 调试端口关闭。未改真实额度/本机 Token 来源、未添加演示数据、未发布或上传。多屏跨 DPI、任务栏移到左右侧、触摸、原生 Escape 取消拖动专项未实测；本轮未执行默认忽略的真实接口/日志专项。
- 原生实现参考：[WM_EXITSIZEMOVE](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove)、[SetWindowSubclass](https://learn.microsoft.com/en-us/windows/win32/api/commctrl/nf-commctrl-setwindowsubclass)。限定为本工具浮窗的同线程回调，沿用默认窗口消息处理。

### 拖动中先吸附，松手结束拖动

- 用户纠正时机：应按住拖到边缘即吸附，不能等松手。`src-tauri/src/floating_drag.rs` 现在在原生 WM_MOVING 中直接调整拖动矩形；鼠标仍按住且真正触及左右工作区边界，窗口立即贴齐，继续往回拖时不再约束到边缘。只靠近、非拖动及关闭贴边设置均不吸附。松手结束拖动，鼠标离开后沿用半 Logo 隐藏。
- 保留最后一次按住时的吸附位置和工作区；鼠标松手后立即移开不再丢失已建立的吸附。松手后排队的移动消息不会覆盖该状态，拖动中退回则立即清除。`src/domain/floating.ts` 及 `tests/floating.test.ts` 增加 snapped 状态校验，防止松手之后把鼠标移到边缘才凭空吸附。设置文案、README、Handoff 同步修正。
- 测试先行：新增 3 个 Rust 用例覆盖按住触边、附近/回拖/关闭开关/工作区外不吸附、负坐标与尺寸保持，先因实现缺失失败；新增前端未建立实时吸附就不能提交隐藏的用例先失败。最终 `cargo fmt --manifest-path src-tauri/Cargo.toml`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`（31 通过/2 默认忽略）、`npm.cmd test`（43/43）与 `npm.cmd run tauri -- build` 全部通过；NSIS 约 5.05MiB。
- 初轮未进入拖动或按下状态中断，不计通过；调整激活等待后已验证右侧未松手贴齐，但发现立即松手移开后未隐藏，随后补充上述吸附状态保留修复。最终安装版完整脚本 `.bundle-tmp/verify-live-snap.ps1` 通过：附近按住不吸附；右侧鼠标保持按下时窗口范围 1850..1920，往回拖解除；左侧按住时范围 0..70，往回拖解除；左右再次触边后松手立即移开均收成 35px 半 Logo。测试同时通过 GetAsyncKeyState 确认断言时主鼠标键仍按下，不把松手后的状态当作提前吸附。
- 最终 NSIS 原地覆盖 `<installation-directory>`（退出 0）。floating_enabled=true、floating_edge=true、floating_mode=usage 保留；正常重启后安装版运行中，临时 9223 调试端口关闭。未修改真实数据来源、没有新增演示数据、未发布/上传。多屏跨 DPI、侧边任务栏、触摸及真实 Escape 取消专项未验收。
- 原生实现参考：[WM_MOVING](https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-moving)。在本工具窗口的拖动消息中修改位置，保持原生拖动与松手流程。

### 球体触边判定修正（已完成）

- 用户再次反馈未吸附，沟通确认期望是“球体碰到边缘”，而此前实现及测试要求“鼠标指针到最外侧像素”，验收条件与用户操作不一致。核对当前运行路径与设置：D 盘安装版、floating_enabled=true、floating_edge=true；EXE 与构建产物的代码节一致，仅 Tauri 打包类型标记存在 3 字节差异，不是旧版本残留。
- 原生改按球体可见边界与工作区边界相交触发；排除球体外围透明留白，按物理窗口尺寸换算。前端直接接收原生 snapSide，不再用鼠标位置否决已发生的球体吸附。设置说明、README、Handoff 已同步。
- 测试先行：新增/修正的 Rust 球体触边测试在旧实现下 3 项失败；前端接收边侧的新测试也先失败。实现后前端 43/43，Rust 32 通过/2 默认忽略，fmt、TypeScript/Vite、Tauri release/NSIS 均通过，安装器退出 0。
- 当前 `.bundle-tmp/verify-ball-contact.ps1` 专项明确要求鼠标 x=1905 / x=20 而非 1919 / 0，并检查按下与实际指针位置。两次原生验收实际位置偏离目标（1909,432 与 967,661），未计为通过；已向用户协调短暂不操作鼠标后再验收。最终结果待补充。
- 用户同意暂停鼠标操作后，原生事件确认：右侧按住触边和回拖成立，但再次触边松手后 snapSide=null，未隐藏；不是此次指针干扰。补充固定拖动抓取点的回归测试（先因实现缺失失败），在整次拖动中以起始窗口位置与鼠标位移重建候选矩形，防止 Windows 调整过的矩形反过来清除触边判定。前端 43/43、Rust 33 通过/2 默认忽略，正在重新打包验收。
- 最终构建/NSIS 成功并覆盖安装，退出 0。新安装后的首轮在起拖时主鼠标键已松开，未计为通过；随后的完整专项通过：右侧指针 x=1905（最外侧为 1919）、左侧 x=20（最外侧为 0）时，按钮仍按下，球已分别贴齐 1850..1920 / 0..70；沿边上下移动保持吸附，往回拖解除，再次触边并松手立即移开后，两侧均收为 35px 半 Logo。原生结束事件分别为 snapSide=right / left，与实际触边侧一致。测试环境：125% DPI、1920×1020 工作区；脚本 `.bundle-tmp/verify-ball-contact.ps1`。
- 最终命令：`cargo fmt --manifest-path src-tauri/Cargo.toml`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`（33 通过/2 默认忽略）、`npm.cmd test`（43/43）、`npm.cmd run tauri -- build`（包含 TypeScript/Vite，NSIS 5.10MiB）。修改重点为 `src-tauri/src/floating_drag.rs`；前序球体判定改动见 `src/domain/floating.ts`、`tests/floating.test.ts` 与设置/交接文档。
- 原设置 floating_enabled=true、floating_edge=true、floating_mode=usage 已核对保留；<installation-directory>\tokendaily.exe 已正常重启，9223 调试端口关闭。此次不改变数据来源或新增演示数据，未发布/上传。多屏跨 DPI、侧边任务栏、触摸及真实 Escape 取消仍未专项验收；本轮未重跑完整点击/悬停脚本及默认忽略的账号/日志测试。下一步为用户日常使用反馈。

### 按住触边即半隐藏（已完成）

- 用户进一步明确：碰边时先吸附并显示半个 Logo，给出已经进入隐藏状态的反馈，然后才松手；不能等松手或移开鼠标才隐藏。
- `floating_drag.rs` 在实时吸附侧变化时发出窗口局部事件；`useFloatingHover.ts` 在原生拖动期间立即裁出半个 Logo，回拖立即恢复完整球，拖动时保持原生抓取面尺寸以免改变抓取点。松手后收窄实际窗口，保持半隐藏，离开后再次悬停才展开。
- `floating.ts` 增加松手后悬停保护与活动拖动预览；先新增测试并确认缺少实现时失败，现前端 44/44、Rust 33 通过/2 默认忽略。设置说明、README、Handoff 同步新时序；构建与安装态验收正在进行。
- 最终命令通过：加载 `scripts/use-local-dev-env.ps1` 后运行 `cargo fmt --manifest-path src-tauri/Cargo.toml`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`（33 通过/2 默认忽略）、`npm.cmd test`（44/44）与 `npm.cmd run tauri -- build`（含 TypeScript/Vite，NSIS 5.18MiB）。设置文案最后修改后重新构建并覆盖安装，退出 0。
- Windows 安装态专项 `.bundle-tmp/verify-held-half.ps1` 完整通过：125% DPI、1920×1020 工作区，右侧指针 x=1905 / 左侧 x=20 时仍按下鼠标，DOM 可见球体宽度已经从 48 裁为 24 CSS 像素；截图前后均检查 GetAsyncKeyState，确认截图发生在松手之前。沿边移动保持，回拖时完整球立即恢复，再碰边、松手且鼠标停留 700ms 后保持半隐藏；移开后仍保持。鼠标离开后再次悬停展开 375px，离开后恢复 35px，通过补充原生检查。
- 已目视核对 `.bundle-tmp/held-half-right.png` 与 `held-half-left.png`，均为按住拖动时的半 Logo。拖动期间原生窗口保持 70px 抓取面，Logo 在屏幕边缘真实裁切为一半；松手后实际窗口收为 35px，全部留在工作区内。
- 核对保留 floating_enabled=true、floating_edge=true、floating_mode=usage；安装版已正常重启，9223 临时调试端口关闭。数据来源不变，没有新增演示数据，未发布/上传。多屏跨 DPI、侧边任务栏、触摸及真实 Escape 取消未专项验收；默认忽略的真实账号/日志测试未运行。下一步：用户日常使用反馈。
# macOS 技术方案评估 · 2026-10-16

- 用户要求依据现有功能确认 macOS 路线，已审查真实源码、Cargo.lock、打包配置和官方 Tauri/Apple/OpenAI/notify 文档。建议同一仓库、同一 Tauri + React/TypeScript + Rust 工程，增加 macOS AppKit/日志监听/路径适配；先做真实数据接入与拖动中半隐藏原型，再推进完整移植。无需立即创建 SwiftUI 独立工程。
- 确认当前不能直接作为 Mac 工程交付：log_watch.rs 无条件 Win32 导入；CLI 查找和默认主目录采用 Windows 路径；macOS 缺少拖动事件实现，非 Windows 主键状态恒 false；包配置只有 NSIS。详细功能映射、建议目录、原型验收与官方来源已记入 docs/FEASIBILITY.md。
- 发布建议为先签名/公证 DMG；商店 Sandbox 与当前外部日志/CLI 接入需单独评估。已核对 Tauri 2.12.1+ 的透明窗口开关变更，未套用旧版开关说明或声称可通过商店审核。
- 本轮仅修改 FEASIBILITY.md 和 PROGRESS.md，没有移动源码、初始化 Git、增加依赖或开始 Mac 实现；未运行 Mac 构建/实机测试，文档变更未重复跑 Windows 测试。没有新增真实/演示数据，未发布/上传。下一步为用户确认开发安排后，在 Mac 上验证最小原型。

## Windows Git 基线与 macOS 工程骨架 · 2026-10-16

- 已在现有 `<desktop-project>` 初始化本地 Git `main`，初始提交 `aa0daa9` 保存拆分前 Windows 原型（119 个源码、资源与文档文件）；检查过待跟踪清单和常见密钥文本模式。`node_modules`、构建输出、本地工具链与临时数据被 `.gitignore` 排除。未配置远端、推送或发布。
- 新增 `docs/ARCHITECTURE.md`、`.gitattributes` 和 AGENTS.md 共仓规则：共用数据口径、界面与状态；原生路径、监听、窗口行为与打包配置按系统隔离。将现有 Win32 日志监听和悬浮拖动源码移至 `src-tauri/src/platform/windows/`，Codex CLI 与默认日志路径查找也转入平台模块。`windows-sys` 改为 Windows 目标依赖。
- 新增 `src-tauri/src/platform/macos/` 的 CLI/日志路径与定时唤醒骨架；当前 Mac 只保留 30 秒目录补查和 2 秒活跃文件检查，原生目录变化通知与悬浮球未实现。悬浮球启用命令会拒绝非 Windows 平台，设置页禁用该能力。Tauri 公共配置与 Windows NSIS、macOS App/DMG 配置分文件；Mac 文件仅代表构建意图，未经过 Mac 构建验证。
- Windows 验证：`npm.cmd test` 44/44；`npm.cmd run build` 通过；加载 `scripts/use-local-dev-env.ps1` 后 `cargo test --offline --manifest-path src-tauri/Cargo.toml` 33 通过、2 个真实来源测试默认忽略；`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过；`npm.cmd run tauri -- build` 通过，生成 NSIS 5.22 MiB。此处是编译、单元测试与打包检查，未重装本轮安装包，也未重复 Windows 安装态交互验收。
- 数据来源与生产演示边界未改变：Windows 真实额度仍走本机 Codex App Server、本机 Token 仍读日志；本轮未读写真实账户值，没有新增演示余额。Mac 的真实登录、额度、日志、通知和悬浮行为均未实测，不标记可交付。
- 下一步：在 Mac 开发机用同一仓库与锁文件运行构建/测试，先核验 Finder 启动后的只读 Codex CLI 和有效 `CODEX_HOME`；再实现原生目录通知与悬浮球，完成菜单栏、通知、登录启动和安装态验收。每次改共用契约都要两端回归。

## Windows 托盘图标占比调整 · 2026-10-16

### 补充：完整 Logo 最大化与原地更新

- 用户反馈前次 81% → 89% 的变化不明显，要求完整显示的最大尺寸。新增 `scripts/generate-tray-icon.mjs`，从正式品牌 SVG 的圆环几何边界生成 `src-tauri/icons/tray/64x64.png`，只保留每侧 0.5 SVG 单位的抗锯齿余量，图形占比约 99.66%。Windows 托盘通过 `tauri::include_image!` 使用该专用资源；其他平台继续使用应用默认图标。`build.rs` 监听专用资源变化。
- 此次未继续修改共享 `assets/icon.svg`。纠正前次记录：`FloatingPanel.tsx` 实际引用该共享 SVG，前次裁切也影响了悬浮球，之前写“悬浮球不变”不准确；本次专用图标不会进一步改变悬浮球。
- 验证：`cargo fmt --check`、前端 44/44、Rust 33 通过/2 个真实来源测试默认忽略、TypeScript/Vite 与 Tauri release/NSIS 构建通过。已原地更新 D 盘安装版并启动，安装 EXE 与新构建 SHA-256 相同；旧版备份 `<installation-directory>\tokendaily.exe.before-max-tray-20261016-202658.bak`。
- 已实际打开 Windows 隐藏托盘并目视比较 `.bundle-tmp/tray-max-before.png` 和 `tray-max-after.png`：当前 125% 缩放下，Logo 可见范围由 18 × 18 提升为 20 × 20 物理像素，圆环与独立圆点完整，大小接近同面板 Codex 图标。系统决定托盘图标槽位大小；更大的点击区域无法靠增加图片分辨率填满。其他 DPI 与 macOS 未实测。

- 按用户提供的系统托盘截图，调整应用图标的画布裁切：`assets/icon.svg` 的视图从 369 × 369 缩至居中的 335 × 335，使有效标记由约 81% 提升至约 89%，并保留约 5% 安全边距，避免 Windows 小尺寸栅格化时贴边或发糊。
- 已用 `npm.cmd run tauri -- icon assets/icon.svg` 重新生成 `src-tauri/icons/` 下的 ICO、PNG 等平台资源；`npm.cmd run tauri -- build` 通过，产出 NSIS 安装包 `src-tauri/target/release/bundle/nsis/TokenDaily_0.1.0_x64-setup.exe`（5.26 MiB）。从新 EXE 提取的 32px 图标有效图形为 29 × 29px，确认放大后的资源已嵌入。
- 已按用户要求原地升级 `<installation-directory>\tokendaily.exe`：先退出运行实例，再将旧 EXE 备份为 `<installation-directory>\tokendaily.exe.before-tray-icon-update.bak`，替换为新构建版本。替换后 SHA-256 与构建产物一致，并已重新启动新进程；没有运行安装器，用户本地设置未改动。该调整不改变主界面、悬浮球使用的品牌 SVG，也不改变数据采集或设置。

## 双机并行开发规则 · 2026-10-17

- 新增 `docs/DEVELOPMENT_WORKFLOW.md`：一条 `main` 主线、按任务分 `win/`、`mac/`、`shared/` 短期分支；同一分支同时只有一台电脑写入。明确共用/平台文件边界、共享契约变更、双端验证、冲突处理、PR 和跨电脑交接方式。
- `AGENTS.md` 增加双机规则入口，`docs/ARCHITECTURE.md` 链接流程并移除过时的“尚无远端”描述。现有 `origin` 指向 `yangdonghan50/TokenDaily`；本轮没有修改远端设置、推送或合并。
- 开始时发现工作区已有用户的托盘图标与相关代码、文档修改；这些改动已保留，未暂存、切换分支或提交。此次仅修改文档，未触及额度来源、演示数据或运行代码。
- 验证：`git diff --check -- AGENTS.md docs/ARCHITECTURE.md docs/PROGRESS.md` 通过；Windows/Mac 构建及安装态测试未因本次文档修改重跑。下一步是在 Mac 实机按流程完成首次构建与数据来源验证，再依据结果补充自动检查。

## Windows/macOS 功能同步清单 · 2026-10-17

- 用户要求一份文档持续对照两端的功能和改动。新增 `docs/FEATURE_PARITY.md` 作为唯一功能清单：每项统一目标与验收条件下分别维护 Windows、macOS 状态、提交和测试依据；仅两端都完成才标记同步完成。`docs/PROGRESS.md` 继续保留详细时间线。
- 首次梳理 8 项现有功能基线，均如实标记 Mac 实机未验证；另登记正在修改的常驻图标为 `CHG-20261017-01`，Windows 当前未提交、Mac 待适配，不宣称两端同步完成。功能单端先合入的例外需记录原因与补齐目标。
- `AGENTS.md`、`docs/DEVELOPMENT_WORKFLOW.md` 和 `README.md` 已加入清单入口；同时修正 `AGENTS.md` 仍写“不做悬浮窗”的旧表述，按交接文档后续批准的悬浮球需求执行。没有修改运行代码、真实数据来源或演示数据；保留所有原有用户修改，未暂存、提交或推送。
- 验证：文档差异与空白检查；本次纯文档调整未重跑 Windows/Mac 构建或安装态测试。下一步：Mac 首次实机验证时逐项更新本清单，之后每项产品改动先登记再两端实现与验收。

## 整理本地 Git 提交 · 2026-10-17

- 按用户要求整理当前 `main` 工作区并仅在本地提交，由用户最后推送。本次属于启用双机任务分支流程前，对原有未提交 Windows 图标工作的收尾；后续新任务按 `docs/DEVELOPMENT_WORKFLOW.md` 使用短期分支。
- 图标源码、生成脚本、Tauri 资源与 Windows 托盘专用图标已单独提交为 `9d17327`（`feat: refine app icons and Windows tray mark`）。协作流程与功能同步文档单独提交，未混入图标代码提交。没有加入凭据、原始日志、本地构建产物或工具链。
- 提交前复测：`npm.cmd test` 44/44；`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过；`cargo test --offline --manifest-path src-tauri/Cargo.toml` 33 通过/2 个真实来源测试默认忽略；`npm.cmd run build` 通过；`git diff --cached --check` 通过。本轮未重新运行 NSIS 打包或 Windows 安装态验收，Mac 仍未实机验证。
- 本次没有修改额度或 Token 数据来源、没有加入演示余额；远端 `origin` 配置未变，未执行 `git push`。下一步由用户决定何时推送，并在 Mac 首次开发时按功能清单逐项补齐。

## Git 忽略规则与双机提交检查 · 2026-10-17

- 审计当前 Git 跟踪清单及工作区：`node_modules/`、`dist/`、`.npm-cache/`、`src-tauri/target/`、`src-tauri/gen/`、`.local-toolchain/`、`.bundle-tmp/` 已被原规则忽略；未发现已跟踪的凭据、原始日志或本机缓存。本机 Codex 设置和额度缓存由 Tauri 写入系统应用数据目录，不在仓库里。
- 扩充 `.gitignore`，覆盖 Windows/macOS 系统杂项、编辑器状态、测试输出、环境文件、签名材料和误放进工程的 TokenDaily 本地运行数据；`package-lock.json`、`Cargo.lock`、两端配置、源码与图标仍保持可跟踪。`.gitattributes` 补充 `.gitignore`、shell 与 YAML 的 LF 换行约定。
- `AGENTS.md` 和 `docs/DEVELOPMENT_WORKFLOW.md` 新增双机提交检查：共享忽略规则只放共同类型；单机专用项放本机 `.git/info/exclude` 或全局 Git 忽略文件；提交前检查暂存清单，已跟踪敏感文件不能靠 `.gitignore` 解决。
- 验证：`git check-ignore` 对 33 个应忽略路径、10 个应保留路径均符合预期；当前所有已跟踪文件均未被新规则误忽略；`git check-attr` 确认新增 LF 规则；`git diff --check` 通过。本次只改 Git/文档规则，未运行应用测试或 Mac 实机检查。无真实/演示数据变化、无远端推送。下一步：Mac 首次检出后检查本机忽略目录及签名材料路径。

## Mac 首次构建与只读来源验证 · 2026-10-17

- 分支：`mac/initial-validation`，起点 `9f0fc4c`。开始时工作区干净，本地 HEAD 与缓存的 origin/main 一致；`git fetch origin` 返回 GitHub HTTP 403，无法确认远端最新状态，未修改远端或凭据。此分支暂不合入 main。
- 范围：验证现有 Mac 构建与真实只读来源，记录平台缺口；不加入悬浮球实现、不修改共享数据规则、不发布或上传。
- 环境：Intel x86_64、macOS 15.7.7（24G720）、Xcode 26.3（17C529）、Node 24.19.0、npm 10.9.8。当前 PATH 无 Cargo；准备将官方 Rust 工具链隔离到已忽略的 `.local-toolchain/`，不修改 shell 配置或系统工具链。
- 已执行 `npm test`：44/44 通过。前端依赖安装、Rust 测试和桌面构建进行中，完成后补充结果。
- `node scripts/probe-codex.mjs`：只读 stdio 初始化和三项账户查询完成，返回 ChatGPT 身份、1 个额度池/1 个有效 primary 窗口、有效百分比及重置字段、69 个日期唯一且格式/计数合法的日桶；响应未提供日桶时区或覆盖范围。未打印或保存原始响应、身份及具体用量。
- 此次终端发现 Codex CLI 0.163.0-alpha.10.2，来自现有 ChatGPT 应用内部路径；没有新增硬编码路径。尚不能据此证明 Finder 启动可发现独立 CLI，或证明与官方 UI 同时刻数值一致。
- `npm ci --no-audit --no-fund`、`npm run build`（TypeScript/Vite）及安装后 `npm test`（44/44）通过，锁文件未改。新增 `scripts/mac-dev.sh`，可选择本检出的隔离 Rust 工具链运行命令，或沿用系统工具链；脚本本身不安装软件，`bash -n` 与 Node 命令透传通过。
- 使用 tabbit 在 Mac 浏览器检查 420×620 的 6 个演示状态，无横向溢出；单窗口截图已目视查看，演示标签明确。360×620 非演示未连接页及设置页无横向溢出，悬浮开关禁用。发现 360px 演示页的 demo-banner 向两侧各溢出 5px，为开发预览样式问题，未改共享 UI。浏览器检查不代替 WKWebView/安装态验收，临时页面已关闭；截图保存在被忽略的 `.bundle-tmp/mac-validation/browser-demo-single.png`。
- 隔离 Rust 1.99.0/Cargo 1.99.0 安装完成，`bash scripts/mac-dev.sh npm run tauri -- info` 的环境依赖检查通过。`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml` 在本 Mac 上 27 通过、2 默认忽略。仅有 Mac `Signal::Changed` 尚未构造的 dead_code 警告，对应原生目录监听待实现，不隐藏警告或宣称已支持事件监听。
- 显式执行 `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml live_ -- --ignored`：真实账号只读契约与本机日志两项均通过，确认 Rust 实际适配器可读取额度/账号日桶，以及本机日志首次采集、增量、索引恢复；测试成功输出未包含真实数量或原始日志。仍未完成官方 UI 同时刻对照或真实午夜驻留。
- `bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 成功，生成 Intel x86_64 的 `src-tauri/target/release/bundle/macos/TokenDaily.app`（12.21 MiB）及 `bundle/dmg/TokenDaily_0.1.0_x64.dmg`（3.82 MiB）；`hdiutil verify` 校验通过。未进行 Developer ID 签名、公证、复制到 Applications 安装、发布或上传。两个锁文件保持原样。
- 从构建目录经 LaunchServices 启动 App。已有辅助功能权限允许读取本应用 WKWebView 无障碍树并操作按钮；没有屏幕录制权限，未获取原生截图，不将 AX 检查当作像素级视觉验收。首次未连接状态没有演示余额；设置页的悬浮窗/贴边/模式均禁用并显示适配中。
- 实际复现 CLI 发现缺口：以 `/usr/bin:/bin:/usr/sbin:/sbin` PATH、无 TOKENDAILY_CODEX_EXE/CODEX_HOME 启动，核实子进程环境后点击连接，主页显示“未找到本机 Codex CLI”和更新失败；不会伪造额度。这是受控最小环境启动测试，不冒充用户从 Finder 双击的完整验收。
- 正常退出后，仅给下一次启动传入终端实际发现的 TOKENDAILY_CODEX_EXE（不写入产品、用户配置或 shell 配置），App 真实额度、本机今日 Token 与账号日统计显示成功，无演示标记；详情保留工作区待核验和账号日桶时区/范围说明。点击刷新后缓存的额度成功时间推进，额度与账号日统计均 available。关闭窗口后进程保留且 AX 中窗口消失；重复启动恢复原窗口，进程仍为同一 PID、实例数为 1。
- 收尾通过应用自身“断开连接”和“Quit TokenDaily”正常停止采集并退出；没有清除用户 Codex 数据、修改授权或启用登录启动。浏览器任务和临时 Vite 服务已关闭。构建产物、工具链、辅助验证脚本与截图均在既有忽略目录中保留，便于本机后续复用。
- 结论：现有技术栈在本 Intel Mac 的编译、真实来源和基础原生运行可行，无须为本轮改写应用。下一优先项是正常 Mac 启动下的稳定 CLI 发现/选择机制；随后再做原生目录通知、菜单栏图标与悬浮球。未验收：Finder 双击的真实来源闭环、系统通知送达/点击、登录启动、菜单栏图标视觉、多屏/Retina/Dock/Spaces、实际午夜/长时运行、Apple Silicon、Windows 本轮回归。原有工作区身份隔离与提醒失败处理问题保留为共享逻辑后续任务。

## 2026-10-17 Mac Codex 来源发现与手动指定（实施中）

- 分支 `mac/codex-discovery`，基线 `4d63bce`。开始时工作区干净；再次 fetch origin 403，继续本地验证分支，不更新 main 或推送。
- 先登记 CHG-20261017-03；本任务修改共享调度、读取器、设置 UI 和 Mac 路径发现。新增连接来源设置默认空，旧设置兼容；缓存按来源隔离。Windows 原发现实现不变，共享改动待 Windows 回归后才能合入主线。
- 计划：先写路径/持久化/来源隔离测试；实现自动查找与手动绝对路径配置；随后运行共用测试、Mac 构建、重启和真实只读刷新。原生悬浮球、通知可靠性专项及发布不在本轮范围。

### 实施结果

- 完成源码：新增 `connection.rs` 和 `ConnectionSettings.tsx`；更新 Mac 路径查找、平台接口、`codex.rs`、共享 `lib.rs`、本机日志测试入口、`App.tsx` 和样式。新增显式程序/数据目录持久化，常见路径候选，路径校验，额度与日志同目录，来源切换与旧请求隔离；缓存加入来源指纹，旧缓存重新读取。
- 测试先行：路径测试先因未实现函数失败；最终前端 44/44、Rust 35 通过/2 默认忽略，显式真实专项 2/2，TypeScript/Vite、cargo fmt、git diff 检查通过。具体命令见 TEST_REPORT 本条。
- 未签名 Intel App/DMG 构建成功（12.28/3.83 MiB），DMG 校验通过。Finder Apple Event 打开构建 App，未注入 CLI/home 环境变量；UI 保存已有 CLI 后，真实额度、账号日桶、本机今日数据与手动刷新通过，退出重开可继续真实读取。
- 故障验证：非法路径拒绝、空测试 home 显示需授权和日志未知、含空格 CLI 链接成功、删除自建链接后重启明确失败且不恢复旧额度、只读设置文件保存失败保留原内容、请求中断连后迟到结果不重新发布。未修改原 CLI、登录凭据、Codex 配置或原始日志。
- 验证资源在被忽略的 `.bundle-tmp/mac-discovery-validation/`，没有真实响应或原始日志；仅本工具测试前设置备份、空测试 home 和时间戳。最终 TokenDaily 断连并正常退出，已保存可用 CLI 路径、home 保持默认。
- 更新 README、ARCHITECTURE、FEATURE_PARITY、FEASIBILITY、TEST_REPORT 和本记录。依赖、两份锁文件、公共平台配置保持不变。
- 限制：手动来源采用绝对路径输入；标准目录由文件系统夹具测试，本机未安装独立 Homebrew/npm CLI；Finder 由 Apple Event 打开，不是人工鼠标双击。未做 /Applications 安装、原生截图、Apple Silicon、签名/公证、Windows 编译/安装验收。原生监听、通知可靠性、工作区身份隔离和悬浮球不在本轮交付。
- 下一步：在 Windows 回归本次共享来源/调度/缓存变化后才能合入主线；随后按既定顺序处理身份与通知可靠性，再推进 Mac 原生能力。远端 fetch 403 仍待解决，未推送或发布。

## 2026-10-17 共享身份与提醒可靠性（实施中）

- 基线 5691ee5，开始工作区干净；新分支 shared/identity-alert-reliability，先登记 CHG-20261017-04。保留前轮未合并改动，当前不能执行 Windows 回归。
- 直接读取本机 CLI 生成的实验协议 schema：GetAccountResponse 顶层 workspaceRouting 提供 chatgptAccountId/backendOrigin，account 内没有这些字段；不读凭据猜工作区。schema 保存在忽略目录 .bundle-tmp/identity-schema。
- 核对锁定的 tauri-plugin-notification 2.7.0 源码：桌面 show() 只准备并后台派发，permission_state() 固定 Granted，不能把它当系统授权/送达证据。实现将区分提交与送达，并对可观测失败处理；不承诺外部通知与本地持久化的绝对 exactly-once。

### 实施结果与交接

- 真实探测补查顶层 workspaceRouting：本机返回有效 chatgptAccountId/backendOrigin，纠正前轮只检查 account 内字段的局限。新增 source + email + workspace + origin 哈希身份 v2，新增 identityVerified；旧缓存默认未核验且不恢复，未核验的新读取不继承历史、不发送额度提醒。
- account/rateLimits/read 后和 account/usage/read 后分别增加只读 account/read(refreshToken:false) 一致性核对；观察到身份变化丢弃本轮账户数据。额度和账号日桶共享同一身份，日志仍保持本机目录口径。
- 提醒运行逻辑集中在 Rust alerts.rs：保留不同已见身份的去重历史；观测状态与提交状态分离，失败不吞掉恢复证据；可观测失败至少 60 秒后仅随新数据重试。移除只被测试引用的 src/domain/notifications.ts、tests/notifications.test.ts，并把有效场景补到 Rust 运行模块测试。
- 本地通知记录改为 alerts.a.json / alerts.b.json（开发态另用 alerts-dev 前缀），序号 + 校验和 + sync_all；发送前持久预留，成功后记录已提交。崩溃/结果不确定时保守去重；两个槽都坏时停止发送。清除本工具数据同步删除新槽。设置页通过新增命令/事件显示提交、重试、记录异常和身份未核验状态。
- 已通过：前端 39/39、Rust 45 通过/2 默认忽略、显式真实来源 2/2、fmt、diff check、TypeScript/Vite、未签名 Intel App/DMG（12.36/3.87 MiB）与 DMG 校验。命令及测试演进详见 TEST_REPORT。
- 原生实机：Finder 打开、已保存来源真实连接、v2 核验状态、详情工作区说明与账号统计范围提示、设置提醒状态通过 AX 检查；结束时断连并正常退出。没有模拟真实额度或实际切换用户工作区，未发送人工测试通知；系统送达不在本轮通过项中。
- 仍有限制：通知插件后台错误不可观测，submitted 不等于 delivered；系统权限/免打扰/送达和点击需后续原生适配及安装态验收。本次只修复可观测提交和本地记录可靠性，未声称保证必达。旧身份迁移可能在新已核验工作区第一次低额度时产生一次新提醒。
- 共享影响文件：codex.rs、alerts.rs、lib.rs、App.tsx、探测脚本与上述删除的重复模块/测试；README、ARCHITECTURE、FEASIBILITY、FEATURE_PARITY、TEST_REPORT 与本记录已更新。没有修改依赖、锁文件、平台配置或凭据。
- 阻塞与下一步：fetch origin 再次 403；本地分支包含前轮 Mac 来源配置提交，未合主线/推送/发布。Windows 必须对两轮共享变更回归；Mac 下一项为原生能力，优先补菜单栏、目录事件与可观察的系统通知提交/授权，再做悬浮球、多屏和安装态。Mac/Windows 都未做的验收继续标未完成。

## 2026-10-17 Mac 原生日志目录监听

- 分支 `mac/native-log-watch`，基线 `1e9ad03`；开始工作区干净，先登记 CHG-20261017-05。`git fetch origin` 再次 403，继续现有本地提交链，没有更新主线、推送、签名或发布。
- 测试先行：新增原生目录事件用例，在旧 Mac 定时骨架上得到 Timeout（预期 Changed）而失败；随后实现并补齐 5 个测试。使用临时目录和合成事件，没有写真实 Codex 日志。
- 新增 Mac 目标依赖 notify 8.2/FSEvents，更新共享 Cargo.lock；其中其他目标传递包也进入统一锁文件，没有升级既有包版本。package-lock.json、平台配置、图标和 Windows 原生实现保持原样。
- Mac `log_watch.rs` 用非递归 home + 递归两个实际日志根目录；过滤无关变化、合并事件为有界唤醒位、手动唤醒优先。新建/替换日志目录按路径与 inode 重绑，周期检查无变化则复用流；停止/切来源失效旧回调，Drop 释放原生线程。日志根符号链接不跟随。
- 共用 `platform/log_watch.rs` 增加规范化降级说明，`lib.rs` 复用已有 UsageState.message；不新增序列化字段、不覆盖日志读取错误、不改变计数可用性。可观测监听错误时保留 2 秒活跃文件检查及 30 秒补查，下次周期尝试恢复；事件仍使用已有 500 ms 合并。
- 已通过：npm test 39/39、Rust 50 通过/2 真实来源默认忽略、fmt、TypeScript/Vite；5 个新增测试覆盖创建/追加/归档/替换/目录重建、忽略无关文件、停止/重绑、丢失事件重扫信号、合成错误恢复、迟到回调、突发合并、手动唤醒、符号链接隔离及 Drop 释放。事件接入实际 Collector 后 100→150，归档后仍 150。
- 原生 App 复核：Finder Apple Event 打开构建目录 App 可显示未连接首页；再次执行二进制唤回窗口，经已有 AX 权限点击连接，真实额度/身份核验/本机 Token 可用，无演示或监听降级提示。之后通过应用按钮断连并正常退出。本轮未显式重跑两个 ignored 真实来源专项，不把 App 复核写成专项测试通过。
- 打包首次编译成功，但 DMG 临时映像卸载报资源忙；确认是本次构建的 rw.1509 映像后普通 detach 成功，没有动系统模拟器映像。退出本工具后重试同一构建命令；最终结果见 TEST_REPORT。
- 更新 README、ARCHITECTURE、FEASIBILITY、FEATURE_PARITY、TEST_REPORT 与本记录。原始日志、工具链和构建产物仍在 Git 之外。
- 限制：本地 Intel 临时目录实测不代表网络盘、睡眠恢复、长时驻留、Apple Silicon 或 Windows 通过；notify 的部分底层启动失败不暴露、子路径处理假定 UTF-8，因此不保证任意系统事件都可观察。定时补查继续作为数据兜底。本轮未做 /Applications 安装、签名公证、系统通知送达、菜单栏视觉或悬浮球验收。
- 下一步：优先补 Mac 系统通知授权/提交可观察性与安装态验收；菜单栏视觉、悬浮球及多屏另做。Windows 需回归前两轮共享身份/来源变更和本次共享降级提示；远端权限未恢复前不宣称双机同步完成。
- 最终打包重试成功：未签名 Intel App 12.48 MiB、DMG 3.92 MiB，hdiutil verify 通过；当前应用断连并退出。开发运行命令 `bash scripts/mac-dev.sh npm run tauri -- dev`，或打开构建目录 TokenDaily.app。提交前检查 Git 差异与忽略项，未加入本机数据或产物。

## 2026-10-17 Mac 原生系统通知（代码与本机通知闭环已验，安装态待续）

- 分支 `mac/native-notifications`，基线 `dd5fe6f`；开始工作区干净，先登记 CHG-20261017-06。远端 fetch 本次 20 秒超时停止，未更新主线或推送。
- 已核对 Apple UserNotifications 文档、本机 SDK 头文件及锁定的 Tauri 插件源码。现插件的权限返回不能代表系统授权；Mac 改为原生 UNUserNotificationCenter 查询/请求权限、addNotificationRequest 完成回调与 delegate 点击。通知请求只使用 Alert，不申请声音、徽标、临时授权或时间敏感权限。
- 授权只由设置按钮主动发起；后台检查和额度读取不自动申请。原生提交明确拒绝进入既有新数据重试；等待完成回调超时为 uncertain，保留持久预留，重启不重发。测试内容明确标注，不修改真实额度或提醒账本。
- 已添加后台单工作线程合并刷新、最新来源/身份/快照复核及清除账本互斥，避免等待系统回执时占用面板状态锁。尚在系统处理中的已提交通知不能撤回为“未发送”。
- 新增命令 get_notification_permission、request_notification_permission、check_test_notification；send_test_notification 返回 accepted/queued。设置区区分系统权限、提交接受、通知中心记录、未观察到和点击回调；缺失记录不当作失败，通知中心记录不等于横幅已展示或已阅读。
- 首轮失败证据：新增 uncertain 测试先因未实现 SubmissionError 编译失败；切换为系统点击标识常量时遇到 NSString AsRef 类型歧义，已改为明确解引用。后续前端 42/42、Rust 57 通过/2 真实来源默认忽略；完整构建与实机验证待下节记录。
- 依赖只把锁文件中已有的 objc2、block2、objc2-foundation、objc2-user-notifications 声明为 Mac 目标直接依赖并启用所需特性，没有升级既有依赖版本。Windows 仍沿用通知插件，授权查询返回明确不可观测，不推断已授权。

### 实机初验与交接状态

- 构建目录 App 经 Finder Apple Event 打开；设置页曾显示尚未申请系统权限，四个通知操作入口可用；后续查明该值不可靠，见下节更正。“检查测试通知”返回本次没有测试记录。未点击申请授权、未发送测试通知；检查结束正常退出，保持 Codex 断连。
- 用户随后明确回复“继续验收，允许通知权限”。已执行申请，但系统拒绝未签名进程，未出现授权框，未发送测试提醒。签名需按 AGENTS.md 单独授权，真实送达/点击仍待验证。即使后续系统接受请求，通知中心匹配和横幅/用户阅读仍是不同证据。
- 最低系统声明改为 macOS 11.0，依据本机 SDK 的 Banner/List API 可用版本；原先 Tauri 默认 10.13 不适用于新接口。只是构建要求，目前未在 macOS 11 或 Apple Silicon 测试。
- 平台文件最终归位：Windows 插件封装移至 platform/windows/notification.rs，共用接口仅转发；Mac 所有 Objective-C 类型留在 platform/macos/notification.rs。共用调度/账本/IPC/UI 与最低版本配置为本轮影响范围，Windows 编译与安装态需要另一台电脑回归。
- 新增/修改文件包括两端通知模块、共享 notification.rs 与 alerts/lib、NotificationSettings.tsx、前端通知状态模块/测试、样式、Mac 目标依赖与统一锁文件、Mac 配置及文档。依赖版本集合检查完全不变；锁文件仅新增直接依赖和所需特性边。
- 下一步：本地签名获得单独授权后，在此分支继续真实权限请求、测试通知提交、通知中心匹配和点击回调；/Applications 安装、公证仍不自动执行。未经验证的 Windows/Apple Silicon 与正式安装态继续保持未完成。

### 用户授权后的实机阻塞与修正

- 已点击申请通知权限；系统回执为失败，无系统授权弹窗。只读 `codesign -dv` 确认 App 未签名，系统通知服务拒绝本应用的查询和授权请求。未添加日志中提及的 Apple 私有 entitlement。
- 更正前面的初验结论：系统返回的 not-determined 是身份未识别时的默认值，不是可靠权限证据。新增 Security `SecCodeCopySelf` / `SecCodeCheckValidity` 只读运行身份检查；未签名或校验失败时显示不可用，不访问通知中心或发送通知。
- 签名错误状态测试先因缺少实现失败，实现后 Rust 全量 58 通过、2 默认忽略。真实额度采集保持断连，测试使用合成状态；未修改账户、原始日志或通知设置。

最终未签名构建复验：App 12.73 MiB、DMG 3.97 MiB；`hdiutil verify` 通过。Finder 启动后设置页通过 AX 确认显示“当前应用未签名，无法可靠读取或申请系统通知权限”，申请按钮不出现，未伪报已授权。前端 42/42、Rust 58/2 ignored、fmt/diff、TypeScript/Vite 及 App/DMG 构建均通过。已另行询问仅对构建产物做本地 ad-hoc 签名，未执行签名或安装；真实系统通知提交/送达/点击继续待验。

### 本地临时签名与真实通知验收完成

- 用户明确回复“允许本地临时签名并继续验收”。在 `5b44e72` 代码对应的构建目录 App 上执行 `/usr/bin/codesign --force --sign - src-tauri/target/release/bundle/macos/TokenDaily.app`，随后 `codesign --verify --strict --verbose=2` 通过；只读详情为 Signature=adhoc、TeamIdentifier=not set。不使用开发者证书，不上传、不安装。
- Finder 打开签名 App 后，原生权限返回已关闭，故没有首次授权按钮。通过系统设置的 TokenDaily 专属页面开启“允许通知”；刷新显示系统允许，未修改其他应用或全局通知选项。首次授权弹窗路径没有实测，不能以手动开启替代该项。
- 发送一条明确标注的“TokenDaily 测试提醒”，系统完成回调返回 accepted；“检查测试通知”返回 in-center。打开系统通知中心，通过 AX 定位该测试条目，先展开分组、再点击实际通知，检查返回 clicked。系统窗口列表确认 TokenDaily 主面板（420×621）在屏幕上，且应用处于前台。
- 隐藏恢复边界：曾执行菜单 Close Window，但点击前屏幕窗口列表仍有主面板，因此本次只证明点击回调与面板前台激活，不声称隐藏后重新打开已验。退出后通知冷启动、横幅像素显示和专注模式组合也未验。
- 退出并从 Finder 重开后，通知权限仍为允许，Codex 保持断连；最后正常退出 App 和本轮打开的系统设置，保留用户允许的通知开关。仅测试通知为测试内容，没有伪造真实额度、触发真实额度事件或修改提醒账本。
- 本轮只更新验收文档和本地构建签名，没有改产品源码，因此未重复单元测试/构建；沿用同一代码已通过的前端 42/42、Rust 58 通过/2 默认忽略及 App/DMG 构建。原 DMG 校验有效，但内部仍是未签名 App，不作为此次签名验收产物。
- 下一步：Windows 回归共享契约后再合主线；Mac 补菜单栏视觉与原生悬浮球，并在另行授权的安装态补首装权限/隐藏与冷启动等用例。Apple Silicon、Windows 本轮实机、发行签名、公证均未完成；没有推送或发布。

## 2026-10-17 Mac 通知隐藏恢复与退出后启动补验

- 分支仍为 `mac/native-notifications`，本轮起点 `f27a305`，源码/签名 App 对应 `5b44e72`；开始工作区干净。沿用用户已批准的本地临时签名和通知验收，未扩大到安装、发行签名、公证或上传。`git fetch origin` 再次 20 秒超时，未修改远端或合入主线。
- 短计划：用已有签名产物分别验证隐藏面板和完全退出两条路径，失败则修复，成功则更新同一 CHG-20261017-06 记录。最终无需修改产品代码。
- 隐藏恢复：发送第一条明确标注的测试提醒，accepted 后检查通知中心。用 AX 获取主窗口真实关闭按钮执行 AXPress，屏幕上本应用 layer=0 主窗口数为 0，进程仍在。点击系统通知分组只展开不恢复；点击实际通知后，同一进程主窗口数为 1（420×621），处于前台，应用检查返回 clicked。此前菜单 Close Window 未隐藏的验证缺口已由原生关闭按钮测试补齐。
- 退出后启动：发送第二条测试提醒，确认 accepted/in-center；正常 Quit 后确认无本应用进程。系统通知分组展开时仍无进程，点击实际条目后出现新进程，主窗口数为 1 并处于前台。此项证明本机 Launch Services 能从该构建路径启动 App；不外推为 App 移动/重装后的行为。
- 冷启动后的“检查测试通知”返回本次运行无记录，符合测试记录仅存于当前进程的设计；没有把上一进程记录伪装成已持久化。此项冷启动证据来自系统进程/窗口变化，不声称新进程保存了上一进程测试 ID 或 clicked 状态。
- 命令与检查：签名 `codesign --verify --strict --verbose=2` 仍通过；使用既有本机 AX 辅助脚本和 CoreGraphics 屏幕窗口列表读取本工具窗口，不申请录屏权限。辅助脚本仅留在忽略的 `.bundle-tmp/mac-validation/`，没有凭据/原始日志进 Git。文档执行 `git diff --check`。本轮产品代码和依赖未变，未重新运行单测/构建，沿用 5b44e72 的前端 42/42、Rust 58 通过/2 默认忽略。
- 真实/测试边界：两条均是测试内容，没有连接 Codex、没有伪造余额或触发真实额度通知，不改变共享 Codex 配置与提醒账本。收尾保持用户允许的通知权限，正常退出 TokenDaily。
- 下一步：Mac 菜单栏交互/视觉与原生悬浮球；后续安装态补首次授权弹窗、安装路径与移除，再测专注模式、实际低额度/恢复、长时运行和多屏。Windows 仍需回归共享改动，Apple Silicon 未验。原 DMG 仍包含未签名 App，没有推送或发布。

## 2026-10-17 Mac 悬浮球与菜单栏（本机单屏交互完成）

- 从干净的 0f4145b 建立 mac/floating-panel；先登记 CHG-20261017-07。复用前端交互和真实数据契约，在 Mac 平台补本地拖动事件、当前鼠标按键及菜单栏入口，先测几何再实机。最近一次远端读取超时，继续本地主线链，不推送/发布。
- 新增 platform/floating.rs 门面、macos/floating.rs 本地 NSEvent 监听及 floating_geometry.rs；共享 lib.rs 与 useFloatingHover.ts 经过门面开始拖动、检查按键及归一化坐标。Windows 保留原 Win32 拖动实现，仍需回归新入口。
- 几何测试先因缺少实现失败；一个负屏夹具原本尚差 2px 才接触，修正后通过。实机发现 Tao 指针使用主屏物理像素，而屏幕查找使用逻辑坐标；新增 Retina 回归后统一 Mac 拖动的逻辑坐标，窗口命中按当前窗口比例归一化。
- 监听仅在主动拖动期间移动窗口并处理 Escape，无全局钩子、录屏或输入记录；鼠标序列继续传给 WebKit，避免连续拖动遗留按键状态。Mac 配置添加 acceptFirstMouse，并启用自定义 macos-transparent 特性映射 Tauri 的透明能力；已比对窗口数组，除首次点击选项外与公共定义一致。依赖版本和锁文件未改变。
- 本机 Intel macOS 15.7.7、单屏 1680×1050 逻辑分辨率、2× Retina：普通球 56×56、点击展开 300×250、移开收起；左右按住触边预览、回拖解除、松手保持 28×56、离开后重新悬停展开、半隐藏拖离及 Escape 还原均通过。球触边时指针无需到屏幕边缘。
- 菜单栏使用已有品牌 alpha 模板；左键切换主面板、右键列出四个入口、设置进入设置页、退出后进程消失均通过。没有执行真实账号刷新；像素级图标和透明边缘未截图验收。
- 开启、贴边及额度模式重启后保留，位置和吸附状态不保存；最后恢复原先的悬浮关闭、贴边关闭、用量模式。Codex 全程断连，没有伪造余额、修改凭据、原始日志或提醒账本。
- 发现电脑上已有 /Applications/TokenDaily.app，Finder 曾打开旧版；未修改它。后续直接启动构建目录可执行文件，并核对 NSRunningApplication.bundleURL，旧版结果不计入当前验收。
- 最终前端 42/42；cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml 为 62 通过/2 默认忽略；fmt、TypeScript/Vite、diff、App/DMG 构建通过。App 12.83 MiB、DMG 3.99 MiB，hdiutil verify 通过；按已有授权仅对构建 App 本地 ad-hoc 签名并通过严格校验，DMG 内仍未签名。未安装、公证、发行签名或发布。
- 运行：bash scripts/mac-dev.sh npm run tauri -- dev；或运行构建目录 TokenDaily.app，在设置开启“显示悬浮窗”。两份同标识 App 并存时先退出另一份并核验进程路径。
- 远端 fetch 本轮最终返回 HTTP 403，未合并或推送。下一步补 Windows 共用入口回归、Mac 多屏混合 DPI、Spaces、Dock 变化、视觉和安装态；Apple Silicon、最低系统版本和长期运行未验，不标两端同步完成。

## 2026-10-17 Mac 单屏补验与跨平台回归交接

- 继续 `mac/floating-panel`，起点 `918ff99`、工作区干净；读取交接/架构/双机流程，短计划为核验运行路径、补实际环境边界与窗口恢复、记录未验项目。fetch origin 再次 HTTP 403，未合主线/推送。
- 仅一块 Intel Retina 屏幕，逻辑工作区 x=0/y=25/1680×966。构建 App 严格签名校验通过；原生拖动顶部/底部不越工作区，底部展开向上让位、收起还原；关闭主面板后浮窗仍可展开并重开主面板。窗口坐标和完整证据见 TEST_REPORT.md。
- 本应用全屏往返已验：主窗 AXFullScreen 0→1→0，悬浮球/展开面板保持在屏幕窗口列表中，退出恢复原主窗尺寸。此项不等于其他应用全屏或多桌面切换验收。没有改变 Dock/Spaces 设置或强制电脑睡眠。
- 用户对透明背景/无白框与菜单栏图标目视回复“显示正常”。没有录屏权限，不新增权限请求，不把 AX 当作像素检查。
- 一轮底部物理点击因未稳定观察到展开且指针发生变化不计通过；后续 AXPress 验证了展开布局。没有隐藏失败记录或将 AX 操作宣称物理点击通过。
- 开始时原安装版在运行且二进制不同。正常退出后用精确构建路径验收，结束恢复 `/Applications/TokenDaily.app` 运行；原悬浮/贴边开启、用量模式、断连状态及右侧半隐藏位置 (1652,357) 保持。未覆盖安装文件、未发送通知、未连接 Codex 或变更真实数据。
- 本轮仅修改 TEST_REPORT.md、FEATURE_PARITY.md、PROGRESS.md，并新增 REGRESSION_HANDOFF.md（Windows 命令、共享影响范围、逐项实机条件、Mac 剩余项及安装边界）。产品源码/依赖/配置未变，不重复单测和打包；沿用最近前端 42/42、Rust 62 通过/2 默认忽略，文档通过 diff check。
- 下一步：按 REGRESSION_HANDOFF.md 在 Windows 对同一代码回归；远端权限需恢复且推送需单独确认。Mac 安装态需先授权备份/替换已有安装版；多屏、Dock 换边/自动隐藏、其他应用全屏/多桌面、睡眠唤醒、长期运行和 Apple Silicon 仍待验。当前不宣称两端同步完成或 Mac 全面可交付。

## 2026-10-17 安装态验收准备

- 继续 `mac/floating-panel`，起点 `f383c10`，开始工作区干净。候选仍为产品代码 `918ff99` 的构建 App；严格 ad-hoc 签名校验通过，bundle ID 为 app.tokendaily.local、版本 0.1.0。
- 已用 ditto 将现有 `/Applications/TokenDaily.app` 归档至忽略目录 `.bundle-tmp/mac-validation/install-backup-20261017-131544/TokenDaily-installed.zip`；unzip 完整性检查通过，归档内主二进制 SHA-256 与原安装版一致。manifest.json 另记原版、候选及归档的 SHA-256；未复制凭据、原始日志或用户数据到仓库。
- 原安装版二进制与候选不同，尚未替换或停止原应用。已向用户请求允许备份后替换安装路径并验收启动/单实例/测试通知；原因是此前明确授权只允许构建目录临时签名且不安装。待明确答复后执行；失败恢复原版，保留原设置，不公证/上传。
- Windows 只做源码预检：门面按 cfg 转发、Win32 函数保留在 Windows 模块，拖动入口继续调用 Tauri start_dragging，运行库会派发到窗口线程。当前仅安装 x86_64-apple-darwin Rust target，无 PowerShell/Wine/Windows 编译运行环境；没有把源码阅读计为 Windows 编译或实机通过。
- Windows 依赖元数据预检：补正 manifest-path 后，`cargo metadata --manifest-path src-tauri/Cargo.toml --offline --locked --filter-platform x86_64-pc-windows-msvc --format-version 1` 因缓存缺少 jiff-tzdb 失败；联网重试只下载了部分锁定版本依赖，后续无进展，已终止本轮子进程。未生成完整元数据，不计依赖检查通过；锁文件未改变。本轮未改产品源码、不重复单测/构建，文档 diff check 通过。

## 2026-10-17 用户授权后的 Mac 安装态基础验收完成

- 用户明确回复“允许替换并继续验收”，沿用 `mac/floating-panel`，起点 `3475c7b`。先复核签名、备份与二进制校验，再正常退出旧版；候选通过隐藏临时目录复制和校验后替换到 /Applications/TokenDaily.app。旧 ZIP 及 original-bundle 留在本机忽略备份目录，用户设置与数据未清除。
- 产品代码仍为 `918ff99`，使用既有本地 ad-hoc 签名，不另签名或公证。安装版严格签名和二进制哈希匹配通过；open 启动的 bundleURL、PID 和屏幕主窗均核验，重复执行二进制仍仅一个实例。
- 三条人工测试提醒：第一条 accepted/点击回调发生早，不计受控隐藏恢复；第二条先验 accepted/in-center，再关闭至主窗数 0，点击实际通知后同进程主窗恢复且 clicked；第三条先验 accepted/in-center，再正常退出至无进程，点击实际通知后新进程从 /Applications 路径启动并显示主面板。分组展开与实际通知点击分别核对。
- 重启后既有通知权限、悬浮/贴边开启和用量模式保留；新进程测试检查为本次无记录，符合当前实现。安装版普通鼠标点击展开/移开收起及右侧半隐藏通过；恢复原 (1652,357) 位置，结束保留新版安装 App 运行，Codex 保持断连。
- 本轮仅更新 FEATURE_PARITY.md、REGRESSION_HANDOFF.md、TEST_REPORT.md 和本记录；文档 diff check 通过，未重复单元测试/构建（同一源码最近为前端 42/42、Rust 62 通过/2 默认忽略）。未连接账户、发起推理、改共享配置或真实提醒账本；没有推送、上传或发布，原 DMG 仍未签名。
- 后续优先在 Windows 对同一代码回归。Mac 本轮是既有安装替换与基础通知闭环，不含首装授权/Gatekeeper/卸载、登录启动、真实额度事件、专注模式、睡眠/多屏/其他应用全屏和长期驻留；这些继续标未验，不宣称全面可交付。

## 2026-10-17 Windows 离线交接包

- 用户表示 Windows 暂不方便接手，明确要求先准备交接包。本轮从干净的 `a53d725` 继续 `mac/floating-panel`，只做本地打包和交接文档，不上传/推送、不操作 Windows 安装或 Mac 已安装应用。
- 发现 START_HERE.md 仍描述最初三份需求资料和“不做悬浮窗”，已改为当前工程接手说明、Windows 回归提示词和已有工具链保留规则。REGRESSION_HANDOFF.md 增加 Git bundle 离线接手入口，不再把恢复 GitHub 访问作为本地回归的唯一前置条件。
- 完整可达历史检查覆盖 300 个历史 blob：未发现禁止交接的凭据/运行数据/工具链路径或常见私钥、访问密钥格式。仅检查 Git 已提交对象，不读取本机账户、Codex 日志或凭据；模式检查不是对任意秘密的绝对保证。
- 以 `a53d725` 试包并在临时空仓库验证：bundle verify、fetch、建立 win/regression-20261017 分支、精确提交/树比对、干净检出和 git fsck 均通过，145 个跟踪文件包含两份锁文件、Windows 配置和图标。ZIP 完整性与解压后各文件 SHA-256 通过；临时验证仓库已清理。
- 最终包会纳入本次接手文档提交，位于忽略目录 `.bundle-tmp/windows-handoff/TokenDaily-Windows-<提交前8位>.zip`；精确快照、文件树和校验结果以包内 manifest.json 为准，包外提供同名 .zip.sha256。包内仅有 Git bundle、README-WINDOWS.md、manifest.json、SHA256SUMS.txt；不用旧试包接手。
- Windows README 包含校验、保留现有改动、查看分叉、导入指定 ref、建立独立 win/ 分支和后续验证步骤。完整 Git 历史可离线导入，依赖安装仍可能联网；不含工具链、已安装 App、安装器、签名材料或用户数据。
- 本轮未改产品源码，不重复单元测试/构建；diff check 通过。Windows 实机与编译仍未验，待用户方便时把最终 ZIP 和校验文件转移到 Windows，再按包外说明执行；不宣称双端同步完成。
## Claude 接入与双来源总览 · 2026-10-17 · Windows · shared/claude-monitoring

- 开始时工作区干净，读取交接/架构/双机流程和功能清单；从与 origin/main 一致的 9f0fc4c 建任务分支。按用户确认接入 Claude Code Pro/Max 与同账户聊天端共享额度；后续明确默认同时显示两来源额度和本机 Token 合计，保留单独查看。统一验收登记 CHG-20261017-02。
- Rust 新增 claude.rs（只读 OAuth/profile/usage、认证错误及身份校验）、claude_usage.rs（北京日期、增量 JSONL、子代理/复制/流式去重）、providers.rs（独立连接、事件、刷新和提醒账本）；平台模块仅负责目录/凭据来源。Cargo 新增 reqwest 原生 TLS；固定 Anthropic HTTPS 主机、无重定向，无模型调用。Claude 不修改用户设置、状态栏、登录或日志，不使用中转 Key 读取订阅额度。
- 前端新增来源选择、双来源总览和汇总规则；主面板/悬浮窗选择同步持久化。合计保留分项，缺失、过期及跨日错误不会冒充完整今日总量。每来源额度、新鲜度与错误分别展示；聊天端精确 Token 明确未接入。浮窗总览展开 300×340、单来源 300×280，保留用量/额度切换与球体/贴边交互。
- Claude 日志初次/重连/切日读取一次，2 秒检查近期文件元数据、30 秒发现新文件/补查冷文件；无变化不重读正文或重算全部消息，索引只在内存。缺失目录显示未知，可读空目录显示 0，错误保留同日旧值并标旧。每日总计包括输入、输出、缓存读取和缓存创建，不重复加缓存子字段。
- 测试先于实现：先加入 Claude 解析/去重测试，运行确认缺少实现失败；补充双来源汇总测试同样先确认缺少模块失败，再实现。最终 npm test 48/48；cargo test --offline --manifest-path src-tauri/Cargo.toml 41 通过/2 个旧真实来源测试默认忽略；cargo fmt --manifest-path src-tauri/Cargo.toml -- --check、npm run build、npm run tauri -- build --config src-tauri/tauri.windows.conf.json 及 git diff --check 通过。构建/缓存/临时文件保留工程盘。
- 实机验证：本机未发现 Claude 订阅 OAuth 文件，projects 目录没有 JSONL；已验证连接显示 auth-required 且无假额度，本机空日志今日 0。Codex 保持真实读取。主面板和悬浮窗总览、来源按钮同步、断开后清空 Claude 状态/合计标注不完整、再次连接、底部按钮可见均通过。合成额度事件显式标注“合成数据 · 仅布局验收”，验证 5 小时/7 天窗口与两种悬浮模式，断言后端仍无 Claude 快照，测试结束重新加载真实状态。
- 本地升级：备份旧 exe 至被忽略的 .bundle-tmp/tokendaily-before-claude-*.exe，原位替换 <installation-directory>\tokendaily.exe 并验证 SHA-256 与 Release 一致，无需重新安装。已在该安装路径重跑上述两组原生验证。验证脚本/截图留在 .bundle-tmp，不提交、不上传；随后关闭调试端口并正常启动。
- 文档同步 README、docs/CLAUDE.md、FEASIBILITY、交接和 FEATURE_PARITY；共享 .gitignore 增加 Claude 凭据与运行数据保护。未上传凭据/日志/源码，没有推送、合并或发布。
- 限制/下一步：需在本机 Claude Code 登录与聊天端相同的官方 Pro/Max 账户，核验真实订阅额度及重置时间；非空真实日志、海量历史、限流/跨版本接口及 Claude 通知送达未完成实机验收。账户端点不是稳定公共 API。Mac 仅路径/凭据文件回退，钥匙串尚未实现，Mac 编译/实机全部未验证；本任务保持分支，不声明两端同步完成。

## 纠正 Claude 聊天端与 Code 数据范围 · 2026-10-17

- 用户确认实际使用网页/桌面聊天，截图显示桌面 Free 账户已有对话。此前接入的是 Code 日志与 Code 订阅授权，不能覆盖该入口；没有 Code 凭据不能推断聊天端未登录。只读检查发现本机 projects 无 JSONL，未读取或输出聊天内容、凭据。
- 修正 `claude_usage.rs`：没有任何有效用量记录时显示未知；来源已有有效历史记录且今日无用量时仍显示 0；删除全部来源日志恢复未知。`combinedUsage.ts` 分项明确写 Claude Code，缺失来源使合计不完整。主面板/悬浮窗及托盘的 Claude 认证缺失提示改为“额度未接入”，Windows 来源说明明确聊天端未接入、不代表未登录。交接、CLAUDE、FEASIBILITY、FEATURE_PARITY 同步边界。
- 先添加测试并确认空日志/仅用户记录测试失败，再修正实现。`cargo test --offline --manifest-path src-tauri/Cargo.toml` 42 通过、2 个真实来源测试默认忽略；`npm test` 48/48；`npm run tauri -- build --config src-tauri/tauri.windows.conf.json`（含前端构建、Release、NSIS）通过。
- 备份旧程序到忽略目录 `.bundle-tmp/tokendaily-before-claude-chat-fix-*.exe`，原位更新 `<installation-directory>\tokendaily.exe`，SHA-256 与构建产物一致。安装态脚本通过：空日志 unknown 且无快照、不推断聊天登录、双来源选择同步、断开隔离、不完整合计、悬浮底栏可见；查看主面板截图确认展示正确。显式标记的合成 UI 事件验证两种浮窗模式和两额度窗口，后端无合成快照，结束后恢复真实状态。无需重新安装。
- 数据限制：Codex 保留真实数据；当前 Claude 数值未知，没有伪造余额或 Token。网页/桌面聊天的直接额度和精确 Token 读取仍未实现；下一步需单独验证适用于聊天入口的数据来源，而非要求用户为这次口径错误升级套餐。Mac 编译及实机未验证；未推送、合并或发布。

## WorkBuddy 本机用量 · 2026-10-17 · Windows · shared/workbuddy-monitoring

- 用户暂停 Claude 额度调试并要求增加 WorkBuddy。开始时工作区干净，读取交接/架构/双机流程，先登记 CHG-20261017-03。远端 fetch 因连接重置失败；本地保留既有多来源功能，从 6413a9e 建新任务分支，没有合并 main 或推送。
- 新增 `workbuddy_usage.rs`：只读 projects JSONL 的规范化 usage，含工具调用；输入包含缓存、输出包含推理，按 sessionId 与记录 ID 哈希去重。2 秒近期文件元数据检查、30 秒目录补查；增量读取完整行、文件截断重建、缓存合计、北京跨日及错误状态隔离，不重复扫描无变化正文。根目录跟随用户迁移到 D 盘的目录联接，不扫描旧备份。
- 新增 `workbuddy.rs` 独立服务/命令/事件、自己的连接设置、断开/重连代际校验；Windows/macOS 路径由 platform 提供。扩展 provider、combinedUsage、App、OverviewPanel、FloatingPanel 和来源按钮，主面板/悬浮窗/设置支持 WorkBuddy 单独查看及三来源合计，未知来源标注不完整。总览展开 300×380、单来源 300×280；有效额度先展示，未接入提示随后展示，WorkBuddy 底栏名称不换行。其他拖动/贴边交互不变。
- 积分限制：本机已定位登录文件，accessToken 为加密对象。开发只检查其结构，没有输出/导出凭据；应用运行版不读取或解密凭据，不改 WorkBuddy 配置、不联网、不请求推理。积分余额明确显示未接入、不触发积分提醒，Token 不换算积分。
- 先添加前端来源隔离/三来源合计测试和 Rust 用量测试，确认失败后实现。`npm test` 50/50；`cargo test --offline --manifest-path src-tauri/Cargo.toml` 47 通过（当时 2 个既有真实来源测试默认忽略）；另运行 `cargo test --offline --manifest-path src-tauri/Cargo.toml workbuddy_usage -- --include-ignored --nocapture` 5/5，通过新增真实只读测试。真实样本 5 个日志约 1.31MB，首次 24.9204ms、增量 369.2µs，重复读取正文为 0，今日用量 0。当前默认测试新增 1 个需显式选择的 WorkBuddy 真实来源测试。
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`、`npm run build`、`npm run tauri -- build --config src-tauri/tauri.windows.conf.json`（Release/NSIS）、`git diff --check` 通过。构建、参考源码、验收脚本和截图留在工程盘忽略目录。
- 已备份旧 exe 至 `.bundle-tmp/tokendaily-before-workbuddy-*.exe`，原位更新 `<installation-directory>\tokendaily.exe`，校验 SHA-256，不运行安装器。安装态验证通过：三来源与三分项、主面板和浮窗来源同步、WorkBuddy 北京当日真实 0、积分快照为空、单来源详情无 Codex 口径、总览/单来源 × 用量/额度四布局底栏可见、断开清空且不影响 Codex/Claude。目视发现初版三来源浮窗底栏名称换行后扩大空间并修正顺序。
- 同步 README、交接、WORKBUDDY、FEASIBILITY、FEATURE_PARITY 与 Git 忽略规则。真实数据只有日志计数与既有 Codex 来源；没有模拟余额。Mac 编译/实机、海量日志、新增真实模型回复的落盘时延及积分查询仍未验证。后续若继续积分功能，先验证官方网页登录授权和服务端返回，不能宣称当前已完成全部 WorkBuddy 指标。
- 布局修正后重新构建 NSIS（6.12 MiB）并再次原位更新，重复安装态检查全部通过，额外确认 WorkBuddy 连接设置在重启后保留。最终目视确认 Codex 有效额度在总览两模式中可见、三来源和刷新底栏不溢出；关闭临时 9223 调试端口，正常启动安装版，WorkBuddy 采集保持连接。未重新运行安装器，也没有触发任何模型任务。


## WorkBuddy 积分余额 · 2026-10-17 · Windows · shared/workbuddy-monitoring

- 用户要求继续尝试积分余额；开始读取交接、架构、双机流程与 WorkBuddy 文档，检查干净状态，fetch 成功，延续当前任务分支，先登记 CHG-20261017-04。Claude 额度调试保持暂停。
- 新增 workbuddy_credits.rs 独立授权/查询服务与 credits IPC/事件，既有百分比 QuotaWindow 不变。仅官方 HTTPS 主机、不跟随重定向；授权 state 五分钟有效，已确认 11217 是等待登录。用户完成登录后首次请求过期，重发请求并选择网页唯一已有个人账号后成功，未要求重复输入密码。没有修改 WorkBuddy 客户端登录/配置。
- Windows platform/secrets.rs 使用当前用户 DPAPI 加密自身 accessToken/域名/脱敏身份/到期时间；不保存密码/refreshToken，不读取或解密 WorkBuddy 客户端凭据，不读取 Cookie。macOS 原生能力明确未实现并禁用。授权失效清空余额、请求重新登录；断开/取消使用代际隔离，清除授权失败明确反馈，旧请求不能复活授权。
- 查询 summary，仅 404 fallback 到 resource；读取明确 remaining 字段，六位定点小数相加，精确字段优先，未知/错误不转零，过期/重复/冲突/不完整与超限格式保守处理。实际接口返回分类汇总，TotalCount 是分类内项目数，名称缺失时标为“积分汇总（N 项）”。不会把 Token 换算积分或把有效期当重置。
- WorkBuddyCredits、credits.ts、App、OverviewPanel、FloatingPanel 与托盘展示独立余额、来源、新鲜度和授权操作。默认五分钟刷新，手动成功后十秒、失败后六十秒节流；独立于两秒本机日志扫描。积分不参与既有百分比提醒。修正浮窗来源标签的旧宽度覆盖，详情不再显示矛盾的“无有效额度”提示。
- 先写新增数据/时间/状态测试再实现；npm test 52/52，cargo test --offline --manifest-path src-tauri/Cargo.toml 55 通过/3 项真实来源默认忽略；包括系统凭据往返/损坏、精确余额、未知/零、过期、重复冲突、不完整列表、官方域名、刷新节流、授权失效、分类汇总。cargo fmt --manifest-path src-tauri/Cargo.toml -- --check、前端构建与 Tauri Release/NSIS 通过。
- 真实验收：官方网页登录成功、读取积分并与官网基础积分+平台奖励积分核对一致；重启后无需再次登录即可查询；手动刷新与十秒节流通过。安装态四布局（总览/WorkBuddy × 用量/额度）真实余额一致，底部按钮可见，横向不溢出。合成事件另验证小数/未知/零/旧数据，显著标记且未写后端缓存，结束恢复真实状态。截图与脚本只保留 .bundle-tmp，不提交。
- 用户此前授权原位升级：旧 exe 备份 .bundle-tmp/tokendaily-before-credits.exe，D 盘安装路径替换并核对 SHA-256，无需运行安装器。官方网页核验使用 Tabbit 技能；关闭临时套餐核验页，保留登录成功页。
- 数据边界：当前真实汇总没有每包有效期，显示未提供；官网逐包到期日期未硬编码或推断。自动令牌续期、全部套餐/企业/超百项分页、积分提醒、Mac 编译/实机未完成；仅规则测试覆盖五分钟调度，未专门等整周期做自动更新实测。没有模型调用、购买、领积分、上传原始日志/凭据、推送、合并或发布。
- 最终产物再次原位替换后，真实余额四布局、授权重启恢复、手动刷新节流重复通过；安装 exe 与 Release SHA-256 一致。目视确认 WorkBuddy 来源/时间不重叠、单来源用“积分”标签。已关闭 9223 临时调试端口，正常启动安装版，保留已授权的真实积分与本机监测。

## 邮箱账号与单设备自愿参榜 · 2026-10-17 · Windows/Web · shared/leaderboard-v1

- 用户批准此前讨论的排行方案，指定 `<web-project>`、`tokendaily.net` 与 Cloudflare。两个工程开始均干净；桌面从已有 WorkBuddy 成果 `aa932b0` 建任务分支，网页从当前 main 建同名分支；未合并、推送或发布。先登记 CHG-20261017-05 与 `docs/LEADERBOARD.md`，覆盖旧不做排行/账号的范围；保留本地监测无需账号、默认不上传。
- 桌面新增 `leaderboard.rs` 和 `Leaderboard.tsx`：邮箱密码注册/登录、验证邮件/找回入口、Windows DPAPI 独立会话、随机设备 ID、公开昵称、本周数据预览、明确勾选同意、30 分钟调度、手动同步、暂停/恢复、退榜删除与密码注销。账号不等于 Codex 登录；TokenDaily 凭据不进 JS。清除本地数据也停止上传并移除自有会话，但不擅自删除云端记录。Mac 参榜显式禁用。
- 复用 `local_usage.rs` 同一归一化/去重逻辑，增加有明确毫秒起止的最小事件导出，不影响原日计数/检查点。仅 Codex 开放；不把 Claude/WorkBuddy 的未知覆盖混入排名。初次本周导入，后续最多补报本周+上周；重绑清空旧统计并从服务器绑定时刻起计。
- 网页新增 Better Auth 1.7.10、Worker、D1 迁移与本地/生产配置模板；公开周/日榜、工具筛选、并列排名、默认头像、自己本周、分享卡、账号管理。服务端字段白名单、64 KiB 请求、批次/幂等/设备 epoch 校验、限流、异常隔离、级联删除及定期清理。邮件接 Resend 适配器，尚未提供/配置真实发件密钥；没有发送外部邮件。
- 回归：桌面 `npm test`（52 项）与 `npm run build`，`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib`（59 通过、4 个真实来源测试默认忽略），cargo fmt、git diff --check。网页 `npm run check`、`npm test`（9 项）、`npm run test:d1`（真实本地 workerd D1：迁移/验证邮箱/登录/Bearer/200 事件重放/级联删除）、Wrangler deploy --dry-run 打包；均为本地，不执行生产部署。
- 显式真实只读测试：`cargo test --manifest-path src-tauri/Cargo.toml live_rank_export_matches_today -- --ignored --nocapture` 通过，扫描 5537 条范围内事件，与相同截止时刻的今日统计一致，没有上传。首轮直接比较前后两次活动日志扫描时失败；定位为持续落盘造成采样时刻不同，测试改为双方统一事件截止时刻后通过。不能将此次检查解读为所有竞品统计口径一致。
- 前端设计沿用现有 Logo/青绿色与浅色布局；Tabbit 检查 Web 空榜/日周切换/未配置邮件 503 提示、390px 手机和 420/360px 桌面布局。合成原生 IPC 夹具验证昵称+同意缺一不可（夹具仅在忽略的测试脚本，未混入应用）；没有用真实账号上传。手机摘要窄列问题已修正。自动关页被安全检查拒绝后只释放控制、保留测试标签页；已停止本次本地预览服务器。
- 发布阻塞：真实生产 D1 尚未建立/迁移、邮件服务/域名验证/密钥未配置、真实收件未验证；未打包或替换当前安装版、未更新官网下载包。实际 30 分钟等待/断网重启的原生端到端和 Windows 安装态待完成；Mac 未编译/实测。当前代码不是“已经上线的新版”。下一步按 Web `docs/LEADERBOARD-OPERATIONS.md` 经授权配置服务、实测后再更新安装包并部署。

## 首版改为免验证邮箱密码注册 · 2026-10-17 · Windows/Web · shared/leaderboard-v1

- 按用户最新要求覆盖上一节的强制邮件验证方案：邮箱作为账号 + 密码，注册后直接登录，不要求验证码或验证邮件。先登记 CHG-20261017-06；同步交接与统一验收基线。`src/ui/Leaderboard.tsx` 去掉重发入口和验证提示；`src-tauri/src/leaderboard.rs` 去掉 resend，注册与登录统一清理旧账号同步状态，上传仍默认关闭。
- Web `server/auth.mjs`/`worker.mjs` 不再依赖邮件配置允许注册登录，成员接口要求有效会话而非已验证邮箱；不伪造 emailVerified 标记。注册不写参榜绑定，仍须独立确认。邮件仅用于可选密码找回，未配置时明确 503；密码哈希、签名会话、限流、注销保护保留。部署检查与说明去掉强制发件配置，不需要数据库迁移。
- 测试先写后实现：新增回归先失败 2 项，修改后网页 `npm test` 9/9；覆盖自动登录 Cookie/签名 Bearer、未验证状态、注册不发信/不参榜、重复注册不能替换密码、错误密码、未登录上传、找回未配置、重置撤销会话、注销。`npm run test:d1` 无邮件配置下真实本地 workerd D1 的注册/登录/参榜/200 条重放/删除通过。`npm run check`、JS 语法检查、Wrangler `deploy --dry-run` 通过，未部署。
- Windows `npm test` 52/52、`npm run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`、`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib`（59 通过、4 忽略）与两仓 `git diff --check` 通过。运行 Rust 前加载 `scripts/use-local-dev-env.ps1`；Web 本地运行命令见运维文档。
- 仅隔离测试账号/合成统计，没有发送外部邮件或上传真实日志。未重新做浏览器目视、Windows 安装态或 Mac 实机验收；未替换安装程序、未发布/推送。基础发布仍需真实 D1、AUTH_SECRET、生产部署与安装态验收；邮件配置不再是注册登录发布阻塞，但未配置前不能自助找回密码。未验证邮箱可能填错/被抢注，不把邮箱或昵称当身份认证。

## 0.1.1 公开测试版发布准备与安装升级 · 2026-10-17 · Windows · shared/leaderboard-v1

- 用户授权“部署、上线、替换”，并确认公开测试版暂不展示反馈邮箱/社交主页。统一版本更新为 0.1.1（package/lock、Cargo/lock、Tauri、设置页），不修改其他来源或账号上传默认值。
- Windows `npm test` 52/52、Rust `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 59 通过/4 忽略、`npm run tauri -- build` 前端/Release/NSIS 成功。安装包 SHA-256 `464ecdceef909ebc93920099a91dead4317ad48cebffe5a35d5a0e14074e531d`，6912104 字节。
- 先备份旧 exe/uninstaller 到忽略目录 `.bundle-tmp/release-0.1.1-backup/`，验证目标为 `<installation-directory>` 后仅停止该安装进程，运行安装器 `/S /UPDATE /D=<installation-directory>`，退出码 0，安装注册版本与文件版本均 0.1.1。保留设置及凭据。安装 exe 哈希 `cf823faf52fd9416cc96860528ec95954b7859fab923f62b7ee8f51aec8a33bd`；与原始 Release exe 仅有 3 字节差异，位于打包类型标记（NSIS 的 NSS 对比未打包 UNK），不是旧程序。
- 实际安装态 WebView 检查账号页面无需验证码、无重发入口、未登录/上传默认关闭、横向不溢出；截图已目视检查，Logo 完整。真实 WorkBuddy 授权保留且余额状态 available，未输出余额或凭据。临时本机 9223 端口验收后关闭，正常重启 D 盘程序。未做正式服务注册/上传端到端或 Mac 验收，不宣称账号可用。
- Web 已准备同版下载包且 release:check、9 项测试、真实本地 D1 测试通过。Cloudflare 经用户确认 OAuth 补 D1 权限，创建空库、完成首次建表、通过 stdin 写入新 AUTH_SECRET。未上传新版 Web Worker/资产，线上仍是旧静态网站。原线上版本与数据库恢复点见 Web 进度。
- 当前阻塞：Tabbit 只读核实 Workers Free 为当前套餐，10ms CPU/请求；当前认证库合成密码本地 CPU 测量约 78ms（不是线上计时）。用户询问为何不适合稳定上线，已说明这是当前实现的超限风险而非免费方案必然不可用；未购买/升级。待用户决定继续免费适配验证或自行升级后上线。已安装桌面账号功能须等服务部署，不自动注册、不上传真实用量。

## 0.1.2 免注册试用上线 · 2026-10-17 · Windows/Web · shared/leaderboard-v1

- 用户同意免费验证阶段改为免注册试用；没有实现明文密码。先登记 CHG-20261017-08，更新交接和 LEADERBOARD 基线。256 位随机凭证只在 Rust/Windows DPAPI 保存，服务端只存摘要；创建先保存，失败重试复用同一身份，创建不参榜。React 不接触原始凭证。
- 桌面用量榜改为开始试用→本周预览→昵称+同意→确认上传；保留暂停/恢复/退出统计，删除身份二次确认且不收密码。无跨设备登录/重绑/找回；清除本地数据前警示凭证不可恢复。90 天未访问过期，服务端清理。网页版公开榜单/分享，旧密码 API 关闭。
- `npm test` 52/52；Rust `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 60 通过/4 默认忽略；新增 pending 身份在重启后保持、未启用上传、凭证不明文测试。首轮 Claude 两项测试失败，定位为 Windows 同时间戳使并行临时目录碰撞；仅在测试 fixture 增加进程内原子序号后全部通过，未改变 Claude 产品逻辑。`cargo fmt`、前端构建、Release/NSIS 和 diff 检查通过。
- 0.1.2 安装包 6975794 字节，SHA-256 `1fe786f43d2b756693fb69452600663cbdbc460f959f4bcfbc72660440cfe68b`。升级前只读检查实际安装文件版本为 0.1.0（与上节已安装 0.1.1 的历史结果不同，未推断原因），备份当时 exe/uninstaller 到忽略目录 `.bundle-tmp/release-0.1.2-backup/`，未覆盖旧备份。明确目标后运行 `/S /UPDATE /D=<installation-directory>`，退出码 0，文件版本 0.1.2；安装 exe SHA-256 `95df57bbdff345fa57833297bbce50c91f1fd77f2dfc17e4835a4b8c9b839106`。
- 安装态真实验收：免注册界面/默认未上传/Logo 完整/无横向溢出；创建身份无 enrollment，重启凭证恢复，本地真实日志可预览但不确认参榜，删除试用身份后未登录且上传关闭。WorkBuddy 授权保留，积分状态 available；不输出凭证或余额。验收后删除本次测试身份、正常重启 D 盘安装版，9223 临时端口已关闭。
- Web 12 项测试、真实本地 workerd D1（200 合成事件幂等）通过；生产新增 0002 后部署至 tokendaily.net，版本 `f55e3097-984a-483c-aba3-de25dee1a3c3`。生产无用量临时身份验证 start/retry/status/enroll/空上传/pause/resume/leave/delete/撤销，旧密码和重绑接口 404，未鉴权上传 401。下载包远端哈希与本机完全一致。最终生产试用身份、绑定、事件计数均 0；没上传真实用量，也没有写正数模拟排名。
- frontend-design 保留原品牌与极简布局；Tabbit 检查线上日/周切换、工具筛选、真实空榜、390px 手机和桌面无溢出、首页 0.1.2 下载链接。截图已目视；释放控制并保留官网页。首轮 Node 直连验收超时，改用既有本机代理后 API/下载全通过，没有修改系统网络。
- 免费套餐未变、没有邮件服务费用或付费升级。只做低流量生产冒烟，不承诺负载容量；没有生产 CPU profiling、备份还原、实际 Cron 触发或等待完整 30 分钟自动上报；Mac 未构建/实测且参榜禁用。分享卡新周期文案/按钮未另做真实有数据浏览器导出验收。后续在明确用户同意后观察真实参榜同步与免费配额。代码只提交本地任务分支，不推送/合并。

## 悬浮窗余额长条与排序 · 2026-10-17 · Windows · shared/floating-balance-strips

- 用户要求无滚动条、每个 AI 一条、最多四来源、拖动上下排序。开始读取交接与架构/流程，先登记 CHG-20261017-06，应用 frontend-design 技能沿用品牌样式。主目录当时有另一任务的排行未提交修改，因此从 aa932b0 建独立工作树 `.bundle-tmp/floating-balance-strips`，没有覆盖、暂存或合并排行工作；期间该任务自行提交为 5351faf。fetch 成功，没有推送、发布或合入 main。
- 新增 `floatingBalances.ts` 展示归一化/最多四来源/排序规则与测试，`FloatingBalances.tsx`、独立 CSS 实现紧凑长条、拖动手柄、键盘方向键和 Escape 取消。指针捕获放稳定容器，避免换位后 DOM 重排丢失捕获；松手保存，取消/失焦不提交中间顺序。localStorage 仅保存来源 ID 顺序，不含凭据/日志；失败明确提示，清除本地数据时也移除偏好。
- `FloatingPanel.tsx` 移除滚动内容区与重复底栏状态；每条显示独立新鲜度，百分比和积分保持各自单位，未知为破折号。多额度窗口选最紧张的已知窗口，其余数量/未知显式标记并可打开主面板。`App.tsx` 传入各来源的真实状态。`floating.ts`、`useFloatingHover.ts` 按行数设定尺寸：宽 300，一行 272、三行 396、四行 458；悬浮球/贴边尺寸与原生逻辑不变。
- 先添加数据/状态/排序/尺寸测试；`npm test` 56/56，`npm run tauri -- build`（含 TypeScript/Vite、Windows Release 和 NSIS）通过，`git diff --check` 通过。只改前端和共享 TS；本轮未重复运行 Rust 单测。工具链源自根目录 scripts/use-local-dev-env.ps1，CARGO_TARGET_DIR 独立指向工作树 src-tauri/target，构建与测试夹具均留在 E 盘。
- 浏览器夹具明确标记“演示·非真实数据”，验证四行在 300×458、125% 缩放的用量/额度模式均无滚动，底栏完整；真实浏览器拖动、键盘上移和刷新后顺序恢复通过。未把第四来源或合成数据接入应用后端。
- 安装态验证：D 盘安装程序原位更新，旧 exe 备份根目录 `.bundle-tmp/tokendaily-before-floating-strips.exe`，Release/安装版 SHA-256 一致。125% 缩放，总览/WorkBuddy/Codex/Claude × 用量/额度八种布局均无滚动且底栏可见；真实 Codex 额度、WorkBuddy 积分和 Claude 未接入状态分开展示。CDP WebView 指针事件验证 WorkBuddy 移到首位、持久化与窗口位置不变，Escape 取消不提交；将系统鼠标移出后确实恢复悬浮球，重新启动后恢复排列。测试后恢复默认 Codex/Claude/WorkBuddy 顺序与用量总览。
- 验证中发现测试时序问题：后台 WebView 模式事件需等待完成再断言；立即强制结束并重启可能在旧 WebView 未释放存储时读到旧状态，等待进程资源释放后重启验证通过。当前顺序采用 WebView 本地偏好，不承诺断电/强杀瞬间写入耐久性。最终关闭临时调试端口并恢复正常启动。
- 限制/交接：仅当前三个已接入来源，第四条仅验证布局容量；Claude 额度仍未接入。本轮未做 macOS 编译或原生实测，其悬浮窗仍禁用；未改额度/Token 刷新、通知、原生接口或凭据。源码在本独立任务分支，后续排行任务打包前应集成本分支的悬浮窗改动，避免安装版回退；不在本轮擅自合并另一任务或更新官网下载包。

## 悬浮窗固定总览 · 2026-10-17 · Windows · shared/floating-balance-strips

- 用户要求进一步精简悬浮窗：移除“用量 / 额度”及“总览 / Codex / Claude / WorkBuddy”选择，固定展示今日 Token 合计和各来源余额；各来源详情留在主面板。保留详情、关闭、刷新和排序手柄。
- 删除前端模式偏好 hook 与设置项，浮窗不再读取或展示额度模式；保留 Rust 端旧模式设置兼容已有本地配置，但 UI 固定按用量布局。浮窗派生为总览，主面板切换来源不会改变浮窗的数据范围。删除无用模式样式和单来源浮窗传参。
- `npm test` 56/56、`npm run build`、`npm run tauri -- build`（Windows Release / NSIS）通过，`git diff --check` 通过。安装态原位更新 D 盘程序并核验 SHA-256。125% 缩放下悬浮窗 300×396，三来源和底栏完整、无滚动，两组选择控件均为零；将主面板来源切到 Codex 后重新展开，仍为三来源总览，随后恢复原来源选择。调试截图和脚本均在忽略的 `.bundle-tmp`，结束后关闭调试端口并正常启动。
- Mac 未编译或实测，悬浮窗仍禁用；未推送、合并、发布或覆盖主目录排行任务。

## 全分支整合到 main · 2026-10-17 · Windows · codex/integrate-all-branches

- 用户明确要求将各分支改动合入主分支，本次按该要求进行本地集成，不推送、部署、替换安装版或删除分支。两个仓库及悬浮窗独立工作树开始均干净；直连 fetch 失败/阻塞，使用命令级既有本机代理后两仓 fetch 成功，未修改 Git 全局代理配置。
- 桌面原 main 为 9f0fc4c，Claude 6413a9e 与 WorkBuddy aa932b0 均已包含于排行 b73549e；实际需汇合排行和悬浮窗 ed12f12 两条历史。从排行建立临时集成分支并合并悬浮窗，只有 PROGRESS 追加区冲突，完整保留双方记录。检查 App 自动合并，免注册入口、0.1.2、凭证清理警示与浮窗固定总览/排序清理均保留；悬浮窗专用源码与原分支一致。重复功能编号 -06 的悬浮窗条目改为 -09，并保留历史编号说明。
- 集成代码 `npm test` 56/56、`npm run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`、`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 60 通过/4 默认忽略；暂存差异与冲突标记检查通过。网页 12 项测试、check 构建及真实隔离本地 D1 通过。只使用测试数据，没有调用生产写入接口。
- 验证后将集成提交快进到本地 main，并对全部本地/远端分支做祖先覆盖检查。配套 TokenDaily-Web 也将已验证的排行发布分支快进至本地 main。原功能分支和独立工作树保留，便于追溯；远端 main 不变。
- 此次仅源码整合：没有新的 Windows 安装态/浏览器验收或安装包发布，当前运行应用及官网下载仍是此前发布产物，不代表已经包含合并后的悬浮窗。Mac 仍未编译/实测，各功能清单维持未同步完成；用户此次显式要求本地主分支集成不等于 Mac 可交付。后续发布须从 main 重新打包并单独验收。

## 主线集成版 0.1.3 安装与部署 · 2026-10-17 · codex/release-0.1.3

- 用户授权安装完整最新主线并重新部署。两工程 fetch 后确认全部本地/远端分支已被各自 main 覆盖，桌面以 ed4eb4c、网页以 27ab659 为基线创建短期发布分支。更新版本、发布说明与下载元数据，不新增产品范围、不推送 Git；发布记录提交后快进回本地 main。
- `npm test` 56/56；加载本机工具链后 `cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 60 通过/4 默认忽略；`npm run tauri -- build` 的 TypeScript/Vite、Rust Release 和 NSIS 全部通过。Web `npm test` 12/12、隔离本地 `npm run test:d1`、`npm run release:check` 通过，部署预检当前 API 测试 11/11。
- 安装包 `src-tauri/target/release/bundle/nsis/TokenDaily_0.1.3_x64-setup.exe` 为 7043115 字节，SHA-256 `0e0d3015fe3fbcac7f199db35d5e3dc8e8b41121cb8e2f91258405d3728018ef`。核实目标路径后备份旧 exe/uninstall 到忽略的 `.bundle-tmp/release-0.1.3-backup/`，原位升级 `<installation-directory>`，安装退出码 0；exe 文件版本 0.1.3、SHA-256 `d1be2497ce526c25e9f6bdcac43ee547d3aa5271cb61ace46c8a9903cd50fdd6`。无须卸载、不清理用户数据。
- 安装态通过临时本机 WebView 调试核对升级前后非敏感设置快照：来源、悬浮窗与贴边开关、参榜开关/身份有无及 WorkBuddy 授权有无均保留。验证设置版本、免注册排行入口、无横向溢出；浮窗固定三来源且无两组旧选择控件、无滚动、底栏完整，主面板切换 Codex 不改变浮窗范围，键盘排序可持久化。最后恢复来源、顺序和开关；未创建试用身份、未参榜、未上传真实用量。
- 首次浮窗截图命中收起动画，调整验收脚本让真实鼠标留在窗内后重验，逻辑尺寸 300×396、125% 缩放截图 375×495，与原生窗口尺寸一致。已目视完整截图 `.bundle-tmp/release-0.1.3-floating.png` 与排行入口截图，均不发布为官网素材。验收后恢复鼠标位置，停止调试实例并正常启动安装版。本次未重新测试鼠标拖动排序（键盘排序已测）。
- Cloudflare 无待应用 D1 迁移，保留数据库、秘密和免费套餐；`npm run deploy` 发布版本 `adfd27c3-c320-4ad2-851c-fbc6a817b6c7`，前一版本 `f55e3097-984a-483c-aba3-de25dee1a3c3` 可供回滚。官网中英文页 200 且包含 0.1.3/新下载地址/摘要，公开日周榜 API 200，旧密码接口 404；线上完整下载 7043115 字节且哈希与本机构建一致，无生产写入验收数据。
- 官网既有截图标明早期界面示意，未将用户真实统计当宣传素材；本次网页做 HTTP/内容/下载核验，未重做浏览器视觉验收。Mac 编译/实机、完整 30 分钟自动同步、通知送达、长期负载及生产 Cron/备份恢复演练仍未验证，不称双端完成或无限免费保证。

## 本机总 Token 历史日历 · 2026-10-18 · Windows · shared/total-token-calendar

- 用户确认撤下旧 Codex 账户日统计日历：主面板总览新增“总 Token 历史”入口，历史页仅合并可读取的 Codex、Claude Code、WorkBuddy 本机日志，固定按北京时间归日；详情不再展示账号日统计时间或旧账户趋势。
- Rust 新增按需 IPC：打开历史页才扫描最近 13 周日志，不改变现有两秒增量采集。三个来源分别沿用实时采集的记录解析/去重语义；跨来源同日相加，读取失败的来源不补零并在界面标注不完整，空白旧日期不伪造记录。完成扫描的当天可明确显示 0。
- 先添加并通过 Rust 历史去重、北京跨日、缺失日及溢出测试；`npm test` 56/56、`npm run build`、`cargo fmt --check`、`cargo test --offline --manifest-path src-tauri/Cargo.toml`（64 通过、4 个真实来源只读测试默认忽略）通过，Release/NSIS 也构建成功。
- 已备份旧程序到本工作树 `.bundle-tmp/tokendaily-before-total-history.exe`，将 D 盘 `TokenDaily` 原位升级；安装版与 Release 都是 0.1.3，文件大小一致。通过临时本机 WebView 调试实际打开“总 Token 历史”：读取真实本机日志后显示 12 个记录日、最新 10 月 18 日，空白日期未显示为零。验收后恢复正常启动；未输出原始日志、凭据或统计明细。Mac 未编译或实测。

## 已合并分支清理 · 2026-10-17

- 用户要求清理无用分支，最终 push 由用户操作；本次只做本地清理与提交。两仓 fetch 成功，远端均只有 main，无待整合远端提交；工作区干净。
- 逐个验证为 main 祖先后使用 `git branch -d` 删除：`codex/integrate-all-branches`（ed4eb4c）、`codex/release-0.1.3`（12070c5）、`shared/claude-monitoring`（6413a9e）、`shared/floating-balance-strips`（ed12f12）、`shared/leaderboard-v1`（b73549e）、`shared/workbuddy-monitoring`（aa932b0）。提交历史均保留在 main，可按上述提交恢复分支。
- 悬浮窗独立工作树干净，切到原提交 ed12f12 的 detached HEAD 后删除分支；目录、忽略文件和构建资源保留，未删除文件。通过临时记录分支提交本节后快进 main 并删除该临时分支。
- 本轮无产品代码改动，未重跑应用测试或重新部署；执行祖先关系、工作区、暂存差异与 `git diff --check` 检查。两个工程仅保留本地 main，远端不变，等待用户分别 `git push origin main`。

## 主线 0.1.3 同步到 Mac · 2026-10-17 · mac/main-0.1.3-sync

- 用户要求把主分支更新同步到 Mac。以现有 Mac `ee3e1c9` 合入本地 main/origin/main `1b06e57`；本轮远端 fetch 超时，未声称核对更新的远端提交。保留两侧历史和用户设置，逐块解决 App/lib/platform/锁文件与文档冲突。未推送、发布或合回 main。
- 同步 Codex/Claude Code/WorkBuddy 来源、总览、固定浮窗与排序、免注册排行界面及版本 0.1.3。保留 Mac 来源配置、身份隔离、双槽提醒去重、原生通知/监听/拖动和透明窗口。Mac WorkBuddy 积分授权与参榜因安全存储未实现继续禁用；Claude 仅接入已有文件凭据路径，Keychain/真实订阅未验证。
- Claude 新模块与 Mac 提醒重构存在 API 冲突：先增加当前身份/快照/代次测试，再接入共用 Delivery 与原生提交回调；不持有 provider 状态锁等待系统，状态与清理使用同一账本。补充独立 Claude 提醒状态事件与设置页读取。主线 leaderboard 改用平台 codex_home 门面以兼容生产编译。
- `npm test` 54/54；`npm run build` 通过；`bash scripts/mac-dev.sh cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml` 87 通过/4 默认忽略。`bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked` 产出 App/DMG；DMG 校验、cargo fmt 检查及差异检查通过。本轮前端测试数与 Windows 旧主线不同，因 Mac 原有通知测试重构，不照抄 Windows 数量。
- 沿用用户先前本地临时签名/安装授权，先备份旧 0.1.0 并验证 ZIP，再 ad-hoc 签名候选 App、验证签名、运行测试后复制到安装路径。候选与安装 executable SHA-256 均为 `b33e56375acd22690432c3201052718e3f0a2114d74fd7d9f1c8f1b98670de89`。旧 ZIP 和原 bundle 在忽略目录 `.bundle-tmp/mac-validation/before-main-sync-20261017-194729/`；DMG 内仍是未签名 App，未公证。
- 候选版验证三来源、固定 300×396、全部按钮在窗口范围内、主面板选择 WorkBuddy 不改变浮窗三来源、键盘排序及正常退出/重启保留；恢复默认顺序。安装版 PID/bundleURL 确认 /Applications/TokenDaily.app，0.1.3、重复启动单实例、浮窗/贴边开关与通知权限保留；鼠标拖动排序通过后恢复默认，球拖到右侧 28×56，悬停展开 300×396，恢复原右侧位置。
- 安装版发送一条明确测试通知：通知中心查询匹配；关闭主窗后点击实际通知，原 PID 面板恢复且收到 clicked 回调。没有进行首装授权、退出后冷启动或真实低额度/恢复事件的新一轮验收。Codex 由原有连接自动取得真实账户额度和本机 Token；Claude/WorkBuddy 未连接显示未知/统计不完整，未制造演示数、发起推理、试用或参榜。
- 新布局仅通过 AX/窗口几何检查，目视确认待用户回复；未验证 Windows 整合代码、Apple Silicon、多屏/长期驻留。旧离线 Windows 交接包已过时，接手必须使用本次合并提交；后续按 REGRESSION_HANDOFF 回归，不标双端同步完成。

## Mac 0.1.3 目视验收补记 · 2026-10-17

- 用户对代码 `64ddf56` 对应的安装版回复“显示正常”，确认新版主面板、右侧悬浮总览的三来源长条和底部刷新按钮完整、透明边缘正常。更新 FEATURE_PARITY 与 TEST_REPORT，关闭本轮待用户目视项；其他平台与未实现能力的限制保持不变。
- 本次仅更新验收文档，未修改程序、重新构建或重跑产品测试；执行 `git diff --check`。保存本地文档提交，不推送或合回主线。

## Mac WorkBuddy 积分修复 · 2026-10-17 · mac/workbuddy-credits

- 用户反馈积分无法正常显示。检查安装版确认为“当前平台未支持”，根因为 shared Credits 的 Windows 限制和 Mac 安全存储/打开登录页面的显式未实现，并非余额为零。基于 19149c1 新建短期分支，先登记统一验收条件并补存储恢复状态测试，确认缺失函数失败后实施。
- 新增 WorkBuddy 专用平台 load/save/remove；Mac 使用独立且不云同步的钥匙串条目，Windows 保留现有 DPAPI 文件。恢复区分缺失、拒绝、损坏和有效授权，错误不会静默变成未授权。Mac 官方登录页仅允许指定 HTTPS 域名，由独立参数调用系统默认浏览器；不读取客户端凭据。共享 UI 文案改为系统安全存储，排行榜仍禁用。
- `npm test` 54/54、`npm run build` 通过；`bash scripts/mac-dev.sh cargo test --offline --features macos-transparent --manifest-path src-tauri/Cargo.toml` 90 通过/5 默认忽略。显式执行 `cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml live_workbuddy_keychain_roundtrip -- --ignored` 1/1，通过合成专属条目的保存、读取、更新、删除、重复删除和无凭据文件检查，测试条目已删除。未读取用户其他钥匙串内容。Cargo.lock 仅新增根包到已存在 security-framework 3.7.0 的依赖边，无版本变化。
- 构建与安装态真实授权正在验收；修复前已备份 0.1.3 至忽略的 `.bundle-tmp/mac-validation/before-workbuddy-fix-20261017-200742/`，ZIP 完整性通过。后续真实结果追加本节，不以合成钥匙串测试替代真实登录和余额。
- 第一轮修复构建的 App/DMG 通过，ad-hoc 签名校验后替换安装版。官方 state 请求与打开默认浏览器成功；用户明确回复已完成官方登录，应用读取到真实官方余额，主面板/总览/悬浮窗一致。正常退出重启，无须重新登录即从钥匙串恢复并重新查询；手动刷新成功。没有读取或输出令牌、客户端登录数据，没有发起推理或参榜；不把真实金额/账户标识写入仓库文档。安装态发现隐私说明还有一处旧 Windows 文案，已改为系统安全存储并重新打包。
- 最终文案修正构建、DMG 校验与 ad-hoc 签名校验通过；安装二进制 SHA-256 `2c4ba7e140bf34a1a1877f773703afe6a8365dda2cd521caf1302a9cc109cec0`。原版和第一轮修复 bundle 均保留在上述备份目录。最终安装版因临时签名变化，启动读取自身条目时出现 SecurityAgent 确认，明确显示 TokenDaily 访问 `app.tokendaily.local.workbuddy-credits`；已请用户在系统弹窗输入登录钥匙串密码并允许，不读取或代填密码，不修改 ACL 绕过确认。当前等待用户处理，最终安装版恢复待补验；第一轮同签名重启与真实积分已通过。Windows 本轮未编译/安装，不标双端完成。
- 用户随后回复已在系统弹窗允许。最终安装版 PID/bundleURL 确认为 /Applications/TokenDaily.app，主窗恢复，官方积分重新查询成功；WorkBuddy 主面板与右侧悬浮总览数值一致，设置中两处系统安全存储文案已生效，授权仍在。保留 WorkBuddy 来源选择并恢复右侧贴边位置。代码提交 `2bc94d4`；本次仅追加验收文档，`git diff --check` 通过，不重跑无代码变化的测试。未推送、合回主线或发布；DMG 内仍是未签名 App。

## WorkBuddy 安装后免重复授权核验 · 2026-10-17

- 用户要求验证能否一键安装并读取本地积分。检查现有分支和干净工作区，以已安装 WorkBuddy 5.8.3 做只读调查；未修改 TokenDaily 程序、宿主设置或登录状态。
- 用 `plutil`/进程清单/`lsof` 核对版本及本机监听；Python 仅输出 auth JSON 字段类型，确认 accessToken/refreshToken 加密，不输出值或解密。只检查随包程序的探测/账户/服务路由代码，临时程序索引和摘录放在已忽略的 `.bundle-tmp/mac-validation/`，不纳入交接。
- 使用不带凭据、禁用代理和重定向的只读 HTTP 请求：本机 `/workbuddy/probe` 200，仅应用版本信息；当次两个 daemon 端口的 `/healthz` 分别 404 和 200（后者为 MCP 状态）；官方 `/openapi/v2/credit` 401。未调用调试执行、模型推理、兑换或购买接口。
- 官方文档确认有个人积分只读 scope，但需 OAuth 用户授权，当前换取 token 文档还要求不可放入客户端的 clientSecret。没有验证新的已授权积分返回，未将其接入产品。已检查的本地接口不能提供免授权积分，结论不外推所有版本或未检查接口。
- 更新 WORKBUDDY/FEASIBILITY 的证据和历史状态说明。下一步建议自动发现、首次连接后自动查询，以及非阻塞安全存储恢复；正式发行签名连续性另行处理，不能承诺永不弹窗。本轮无功能改动，不重跑产品测试或 Windows 验收；执行文档差异及忽略检查，仅保存本地提交，不推送或发布。

## Mac WorkBuddy 非阻塞授权恢复 · 2026-10-17 · mac/workbuddy-credits

- 用户要求优化启动凭据读取。根因是 `Credits::new` 在 Tauri setup 同步读取 TokenDaily 自己的 WorkBuddy 钥匙串条目；系统确认会阻塞应用初始化。改为初始状态不访问安全存储，只有 WorkBuddy 已连接后才启动一次后台恢复，先发布“正在恢复授权”。恢复出有效授权后自动读取积分；缺失、拒绝或损坏仍按原有明确状态显示。
- 为避免钥匙串被拒绝/锁定时每三秒重复出现确认，后台调度只尝试一次；用户手动刷新才明确重试。断开会删除 TokenDaily 自己的授权并使迟到恢复代次无效；显式重新登录也会取消旧恢复结果。未读取 WorkBuddy 客户端文件、浏览器 Cookie 或其他钥匙串条目。
- 前端将“恢复中”与“积分待授权”区分展示。新增 Rust/前端状态测试；前端 `npm test` 55/55、`npm run build` 通过，Mac `cargo fmt --check` 与 `cargo test --locked --features macos-transparent` 92 通过/5 默认忽略。`tauri build --no-sign -- --locked` 的 Release App/DMG 构建和 `hdiutil verify` 通过，DMG SHA-256 为 `09bcef7d10b9b837c1baa3c4c62aabbf289aa1d56bafc6deb5f7a7d36778cddf`。未替换 /Applications 安装版、未触发用户真实钥匙串确认，也未做 Windows 回归。

## 统一 Token 简写与 0.1.4 交付 · 2026-10-18 · codex/compact-token-display

- 用户要求桌面/网页所有 Token 展示沿用悬浮窗简写，随后明确确认更新本机并部署官网，Git push 仍由用户执行。新增 tokenFormat.ts 与 TokenCount 组件；浮窗总数、来源分项、主面板、详情、账号趋势/日历、个人榜单、上传总量及逐日预览共用规则。不到一万为整数，万/亿截断两位小数，完整数字保留于 title/无障碍标签；未知不变成 0，积分/比例/人数/记录数不简写。
- 先写新增测试并观察旧实现缺少格式导出而失败，再统一实现。修正旧浮点计算顺序在边界可能少显示 0.01 的问题；覆盖 0、9999、10000、10100、万亿边界、627306→62.73万、741969661→7.41亿、非法值和精确值。仅展示层改动，不修改真实采集、上传、名次计算或数据契约。
- `npm test` 58/58；`npm run build` 与 `npm run tauri -- build`（TypeScript/Vite/Rust Release/NSIS）通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 60 通过/4 默认忽略；Rust 格式及 Git 差异检查通过。Web 20/20，包含实际中英文表格/分享 canvas 消费者回归，原始整数和参与人数保留；发布预检 API 15/15 与隔离 D1 读取成本门禁通过。
- Tabbit 本地验证桌面详情的简写/完整提示/大字号，网页中英文日周榜、390px 手机布局和分享卡绘制；数据为标注“虚构”的本地拦截，不写入生产。官网主面板和浮窗演示图使用当前组件重新生成，保留演示标记，不发布用户真实统计截图。
- 安装包 TokenDaily_0.1.4_x64-setup.exe 为 7158286 字节，SHA-256 `a3d679705ce657042beb106072184e8b0b7fcbf1d0765ea477ee4533615b46aa`。旧 exe/uninstall 备份在忽略的 `.bundle-tmp/release-0.1.4-backup/`；核实精确目标后原位升级 <installation-directory>，文件版本 0.1.4。升级前后脱敏状态快照一致，保留 Codex 来源、悬浮/贴边开关、现有参榜身份/已开启上传以及 WorkBuddy 授权。未手动触发同步或修改用户上传设置，既有自动同步继续按原同意状态运行。
- 安装态榜单显示“7.54亿”、浮窗合计和分项显示万单位，三来源完整无横向溢出；首次核验因浮窗 WebView 尚未出现在早期目标列表而失败，改为等待目标创建后重验通过。截图已目视，仅留本地忽略目录。验收后正常启动应用，关闭临时 9223 调试端口，停止本轮三个预览服务。
- Web 部署版本 `580f5da3-f23c-49a1-8357-4430ea6c31e6`，保留已有 D1 读取优化、数据库、密钥、Cron 和免费套餐，不执行迁移。正式网页榜单已目视简写；只读 HTTP 核验中英文版本/下载元数据、两个脚本与演示图哈希、日/周 API 200 且仍返回整数；完整线上下载摘要与本机构建一致。首轮网络超时后重试通过，未创建模拟生产排名。
- Mac 未编译/实机验收；本次没有重做提醒或完整自动同步周期验收。发布修改与记录本地提交、快进 main，未推送 Git；原安装目录、授权与统计数据保留。

## 悬浮卡片整块拖动与 0.1.5 本机交付 · 2026-10-18 · codex/floating-card-drag

- 用户要求整块组件可排序、默认边界融入面板，悬停时提示可拖动，并明确确认直接更新本机。先登记 CHG-20261018-03；frontend-design 指导沿用原品牌、尺寸和布局，仅调整透明边界、悬停/聚焦手柄与拖动态，不重做界面。
- 新增起拖条件测试先失败再实现；PointerDown 从手柄移至卡片，保留按钮排除、4px 阈值、指针捕获、取消、键盘排序和本机保存。浏览器追加无手柄焦点的 Escape 测试发现原局部事件无法取消，改为仅在拖动中捕获 Escape，并验证从正文起拖也可取消。未修改额度/积分/实时用量与上传逻辑。
- 安装前检测到另一工作树刚安装历史汇总版 0.1.3，暂停覆盖并询问。用户确认“合并验证后一起更新”，因此合入 shared/total-token-calendar 的 1eb6260；解决交接文档和 UsagePanel 冲突，同时保留历史本机口径、缺失提示、TokenCount 简写及完整数值标签。历史条目由重复的 -01 改为 CHG-20261018-02。另一工作树和其分支不改动。
- 合并后 `npm test` 59/59；`npm run build`、`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 64 通过/4 默认忽略、`cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`、`npm run tauri -- build` 和 Git 差异检查通过。最终 Escape 修复后再次运行前端测试和完整 Release/NSIS 构建。
- Tabbit 在标注演示的本地夹具测试名称/数值/进度条/空白区起拖、单击/小抖动/右键不改序、更多按钮不触发拖动、Escape/失焦取消、键盘上移与刷新持久化，正常/悬停布局无跳动；减少动态效果与真实 Tab 焦点轮廓检查通过。首次在模拟 blur 后直接 focus 的断言因浮窗收起失败，改为重新展开并使用实际键盘导航后通过；未因此修改产品焦点规则。截图已目视，演示页和服务已关闭。
- 最终安装包 `TokenDaily_0.1.5_x64-setup.exe` 7294686 字节，SHA-256 `71db0ce41fd6682aeddd2559f8cd28a3b5cda7ab00fdd9097b2a1f76419401ad`。备份现有 exe/uninstall 到忽略目录 `.bundle-tmp/release-0.1.5-backup/` 后，以 `/S /UPDATE /D=<installation-directory>` 原位升级，退出码 0。安装 exe 文件版本 0.1.5，SHA-256 `b52fc5f4e766eb7aef4e36042edf760ecb175801caa513b1ffb000a3f776bd18`。初次与裸 Release 比较哈希不等，逐字节核实仅 3 字节 Tauri 包类型 `UNK`→`NSS` 的差别，按 NSIS 标记归一后完全一致；没有用裸 exe 覆盖安装版。
- Windows 实际安装态：125% 缩放下整卡拖动/持久化、Escape 取消、减少动态效果、聚焦边界、原生窗口不随排序移动通过；默认/悬停与总 Token 日历截图已目视。读取真实本机历史，简写/完整数值提示和缺失日期保持，未混入演示数据。原来源 all、浮窗/贴边开关、参榜身份及既有上传开关、WorkBuddy 授权和 Codex/WorkBuddy/Claude 顺序均保留；验收调整的顺序已恢复，并在重启后核对。首次验收因系统鼠标在展开前后位置变化触发自动收起，校正真实窗口坐标后重验通过。
- 最终恢复正常安装版运行，9223 临时调试端口已关闭。未手动触发参榜同步；原用户已启用的自动同步继续按既有授权运行。不部署官网、不更换线上下载包、不推送 Git；本地完成提交并快进 main。Mac 未编译/实测，原生浮窗仍禁用；提醒送达及完整自动同步周期未重新验收。下一步若要官网下载同版，需另行确认部署。

## 悬浮组件编辑与 0.1.6 本机交付 · 2026-10-18 · codex/floating-component-editor

- 用户确认先讨论的方案后明确开始实现并更新本机：去掉左侧手柄、扩大名称与辅助文字展示，600ms 原地长按编辑，移动优先排序。frontend-design 沿用青绿/浅底、原数字字形和紧凑长条，仅在编辑时短暂轻抖约 480ms，减少动态效果时无抖动；正常态不显示强边界。未更换品牌或资源。
- 先登记 CHG-20261018-04 和交接范围，写假时钟测试观察缺少实现而失败，再实现独立长按状态机、组件显示过滤和编辑展开保持。新隐藏键只存 Codex/Claude/WorkBuddy ID，原排序键兼容；隐藏卡片不改变采集、授权、提醒、总量或上传。隐藏组件的原排序槽位保留，单卡可编辑，空态可重新添加，添加列表不重复、不自动连接或授权。设置提供管理入口，两个窗口共享显示偏好，存储失败不改变界面结果，清理本地数据同步清除显示/排序偏好。
- `npm test` 64/64；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 64 通过/4 默认忽略；`npm run build`、`npm run tauri -- build` 的 TypeScript/Vite/Windows Release/NSIS、`git diff --check` 通过。本次没有修改 Rust 采集实现，未重做真实通知送达、完整自动同步周期或 Mac 编译/实测。
- Tabbit 本机演示页（明确标记合成数据）真实输入验证：移动取消长按、整块排序、原地长按、单卡长按、全移除后重载为空且可恢复、添加禁重复/未连接提示/连接引导、数值合计不随隐藏改变、键盘方向键/回车/Escape、减少动态效果、更多按钮隔离、失焦取消定时器。通过注入本地测试存储失败验证保留原卡片并显示未保存；第二个测试页验证跨窗口显示同步。默认/编辑/选择器截图已目视，测试页与 4324 预览服务已关闭，不修改官网工程。
- 安装前快照仅记录来源/开关/授权有无/来源连接布尔值/组件偏好，不包含凭据。原 0.1.5 exe/uninstall 备份到忽略目录 `.bundle-tmp/release-0.1.6-backup/`；以 `/S /UPDATE /D=<installation-directory>` 原位升级。安装包 7415478 字节，SHA-256 `8f757dbe2b2f4670f1f6bb7fa271139422451e0f0c567390813c29129b90dc7b`；安装 exe 0.1.6，SHA-256 `ef24293d0ac6cb2524368d6a26993e413f0bb33a7b8243cf678254e0a3b0a03a`，与裸构建仅差正常的 3 字节 NSIS 类型标记。
- 安装态（125% 缩放）通过 WebView 指针输入验证长按编辑、鼠标离开仍展开、移除后各来源采集连接与授权不变、主设置和浮窗偏好同步、WebView 重载后隐藏仍在、三/二/零卡片与恢复全部、Escape 先取消排序后退出编辑，均无横向溢出或底部裁切。首次失败源于测试辅助 PowerShell 的 DPI 虚拟化：其鼠标坐标被放大 1.25 倍，实际在窗口外；对测试进程启用 DPI 感知后，Tauri 与辅助进程坐标一致。未因此修改产品原生指针逻辑。两卡高度 334 逻辑像素在 125% 下取整为 WebView 335，验收采用 1px 容差。没有将 OS 输入注入失败当成功，最终依据是安装版 WebView 输入及实际原生窗口尺寸验证。
- 真实安装态截图只存本地忽略目录，不作为宣传素材。所有验收增删/排序已恢复原始 Codex/WorkBuddy/Claude 顺序和全部显示；重启后来源 all、浮窗/贴边、三来源连接、WorkBuddy 授权及原参榜身份/上传开关与升级前一致。恢复正常运行，关闭 9223 调试端口；未手动触发同步，原有自动同步按用户既有同意继续。
- 本地提交并集成 main；不部署官网、不替换线上 0.1.4 下载包、不推送 Git。Mac 浮窗仍禁用且未实机验证，不称双端交付；后续若要官网与本机同版需另行确认发布。

## 2026-10-18：悬浮 Token 日历（CHG-20261018-05，0.1.7）

- 完成：日历作为独立可编辑组件，默认追加到既有额度顺序后；复用拖动/长按/移除/添加，设置页可开关。13 周本机总 Token，按北京时间；日期选择、万/亿简写、缺失与零区分、未来日期禁用。只保留日历或全部隐藏均能恢复；尺寸随可见组件调整。
- 读取：复用 `get_local_total_history`，不更改 Rust 采集契约。仅展开且日历可见触发；五分钟内重复展开复用缓存、跨北京时间日期失效、合并并发请求，异常保留旧记录并标旧，30 秒错误重试缓存，可手动刷新。界面标快照时间，区别顶部持续更新的今日数；不上传日志或增加授权。
- 文件：`src/domain/floatingCalendar.ts`（缓存/日期）、`src/ui/useFloatingHistory.ts`、`FloatingCalendar.tsx`；`floatingComponents.ts`/`floating.ts`、`FloatingBalances.tsx`/`FloatingPanel.tsx`/`useFloatingHover.ts`/`floatingBalances.css`/`App.tsx` 接入编辑与布局；版本文件、README、交接/平台验收同步。
- 测试先行：新增 `tests/floatingCalendar.test.ts`，先确认缺少实现失败，再通过迁移、尺寸、北京时间、缺失/零/未来/过期、缓存合并/失效/重试测试。
- 命令：加载 `scripts/use-local-dev-env.ps1`；`npm test` 68 通过；`npm run build` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 64 通过、4 默认忽略；`npm run tauri -- build` 生成 0.1.7 NSIS。
- Windows 安装态：原位更新 `<installation-directory>/tokendaily.exe`，旧 0.1.6 程序/卸载器保存在忽略目录 `.bundle-tmp/release-0.1.7-backup`。真实 WebView 输入验证日历在额度后、91 格、选日不触发编辑、日历标题长按、拖动到首位重载保留、移除/再添、仅日历及空态恢复。截图检查完整/编辑/仅日历/添加面板无裁切。设置日历开关跨窗口同步。运行时历史调用计数：收起 0 次，首次展开 1 次，重复展开不增加，手动刷新增加 1 次，隐藏再显示不增加。测试后恢复用户显示顺序及开关，连接/WorkBuddy 授权保持。
- 验收工具修正：添加面板检查最初早于原生异步缩放完成，等待完成后通过；Tauri 调用函数不可直接覆盖，改用只计数、不暂停的调试断点核查历史读取次数。未因此改动产品采集逻辑。
- 限制：日历不是实时统计流，快照内今日数可暂时落后顶部；空白是未记录而非零。macOS 原生悬浮仍禁用，未编译/实机验证。本轮未部署官网、未推送 Git。
- 最终安装包 SHA-256：`63d37db17be3abedb7abb9f2f2fd3833244f7752a68ed8b06d5966b5d9dac32e`；安装程序版本核对为 0.1.7。验收后关闭临时 WebView 调试并正常重启。

## 2026-10-18：方格日历与悬停提示（CHG-20261018-06，0.1.8）

- 用户要求悬浮窗和详情日历参考 GitHub 方格，取消点击选择日期。新增共用 `UsageHeatmap.tsx` / `usageHeatmap.css` 与 `usageHeatmap.ts`，两处固定正方形、月份/星期标签、青绿色阶；格子为无点击动作的信息元素，不再有选中态。悬停显示日期与万/亿用量，移开关闭，键盘聚焦可等效读取。鼠标点击聚焦不固定提示。
- `FloatingCalendar.tsx` 去除重复的日期/数值摘要；`UsagePanel.tsx` 日历模式固定展示最近记录，不受趋势图选择影响，趋势图原交互保留。`floatingBalances.css` / `floating.ts` 为方格调整卡片及原生高度，保留整卡拖动、长按编辑和显示偏好。frontend-design 用于保持品牌配色与紧凑排版，未引入外部素材。
- 验证：`npm test` 70 通过，新增提示文案（缺失/零/简写/未来）及跨年月份定位测试；`npm run build` 和 `npm run tauri -- build` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 64 通过、4 默认忽略；`git diff --check` 通过。
- Windows 原位安装 0.1.8 至 `<installation-directory>`，旧 0.1.7 程序/卸载器备份至忽略目录 `.bundle-tmp/release-0.1.8-backup`。安装包 SHA-256 `b897e7d271846c00e353fe1b62c49343845438e4b2e89d6966d43cfb7b84ce08`。真实安装态测量浮窗 12×12、详情 14×14 逻辑像素；截图目视悬停提示可见、方格无拉伸、编辑无裁切。鼠标/WebView 输入验证点击不选日/不改数值、移开提示消失、键盘提示、长按编辑仍有效。
- 验收中系统鼠标与 CDP 坐标不一致曾影响悬停截图和长按；使用 DPI 感知实际鼠标定位并记录指针事件后复验通过，未修改产品原生拖动逻辑。恢复测试前组件顺序 Codex/Claude/WorkBuddy/日历及全部显示，连接、WorkBuddy 授权、原参榜身份/开关与升级前一致；不改变数据读取口径或缓存。
- 保留上一轮尚未提交的改动；本轮不提交/推送、不部署官网。macOS 未编译/实机验证，原生浮窗仍禁用。验收后关闭临时 9223 调试接口，恢复正常运行。

## 2026-10-18：启动靠右与紧凑日历（CHG-20261018-07，0.1.9）

- 新增 Windows `platform/windows/floating_placement.rs`：在 setup 显示悬浮球之前按主屏工作区右侧居中定位，56px 球体和 16px 边距按 DPI 转物理像素；回退当前显示器。`lib.rs` 仅调用平台初始化，窗口配置取消 floating 居中；主面板居中、运行中拖动/贴边不变。
- 日历卡片 188→134 逻辑像素，格子 12→10px，间隔 3→2px；去除下方读取快照、口径说明及独立刷新。日期/用量仍悬停读取，缺失不补零；读取失败/过期/部分数据在标题旁显示小提示，详情通过标题悬停可读，不伪造正常。frontend-design 指导沿用品牌配色和压缩比例；详情日历不变。
- 底部统一刷新：可见日历强制重新读取，同时调用既有来源刷新；隐藏日历不读取历史。忙状态包括可见日历，只有日历而无已连接来源时也可刷新。新增 `floatingRefresh.ts` 与测试，先确认缺少实现失败，再实现并验证。
- 检查：`npm test` 71 通过；`npm run build`/`npm run tauri -- build` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 65 通过、4 默认忽略；`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 通过。新增 Rust 几何测试覆盖 125% DPI、任务栏工作区偏移、负坐标和极小工作区；只在实际主屏做原生验收，未做真实多屏热插拔。
- 安装：原位更新 `<installation-directory>/tokendaily.exe` 至 0.1.9，旧 0.1.8 程序/卸载器备份于忽略目录 `.bundle-tmp/release-0.1.9-backup`。两次进程启动实测：工作区 1920×1020、缩放 1.25，球体 70×70，位置 (1830,475)，右边距 20 物理像素、垂直居中。正常/编辑布局无裁切；截图确认标题+月份+方格，无底部解释和独立按钮。
- 安装态统一刷新计数：底部一次点击触发 `refresh_quota`、`refresh_claude`、`refresh_workbuddy`、`get_local_total_history`；日历移除后再次点击前三项刷新，但不读取历史。完成后恢复 Codex/WorkBuddy/Claude/日历顺序与全部显示，三来源连接、WorkBuddy 授权、参榜身份/既有开关均保持；关闭验收调试接口，正常运行。
- 限制/下一步：macOS 未编译/实机验证，原生浮窗仍禁用。本次只更新本机，未部署官网；保留所有未提交修改，不提交、不推送 Git。

## 2026-10-18：无标题宽幅日历（CHG-20261018-08，0.1.10）

- 用户要求利用左右留白展示更长月份并移除标题。浮窗从 13 周扩为 20 周/140 格，保留 10px 方格、2px 间距；缩窄星期栏，图宽 252px，卡片从 134 降为 112px。移除可见标题，保留组件无障碍名称和异常小提示；编辑/添加列表仍称 Token 日历。frontend-design 指导保留配色，利用横向留白并减少高度；不增加下方说明或独立刷新。
- `UsageHeatmap.tsx` / `usageHeatmap.css` 按实际周数设列数和无障碍标签；`floatingCalendar.ts` 默认 20 周；`FloatingCalendar.tsx`、`floatingBalances.css`、`floating.ts`、`FloatingPanel.tsx` 调整布局和元数据。`total_history.rs` 本机读取回溯从 91 扩至 140 天，覆盖 20 周网格最早一天的完整记录；详情仍使用原 13 周网格，不扩展页面宽度。身份/来源契约、去重、实时采集和上传规则不变。
- 测试先行：前端先验证旧 91 格/旧高度失败；Rust 新边界测试先确认 13 周读取不足，再扩范围。`npm test` 72 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 66 通过、4 默认忽略；`cargo fmt --manifest-path src-tauri/Cargo.toml --check`、TypeScript/Vite/Windows Release/NSIS 通过。
- 安装：0.1.10 原位更新 `<installation-directory>/tokendaily.exe`；旧 0.1.9 程序/卸载器备份在忽略目录 `.bundle-tmp/release-0.1.10-backup`。安装态测量 20 列/140 格、每格 10×10px、热力图宽 252px/卡片约 272px、高 112px。真实记录覆盖显示 6–10 月，缺失仍为空；悬停缺失格显示未记录而非零。正常/编辑截图目视无裁切，键盘仍可进入编辑，详情验证仍 91 格。
- 首次悬停验收未获得提示，重跑等待刷新稳定、重新读取格子位置后通过；未据此修改产品悬停逻辑。检查并恢复原 Codex/WorkBuddy/Claude/日历顺序、显示偏好及连接/授权，关闭临时调试接口并正常重启。
- 限制：扩展为固定最近 20 周，不是无限历史翻页；仅本机可读取日志，缺失不补零。Mac 未编译/实机验证，原生浮窗仍禁用。保留已有未提交改动，本次未提交/推送、未部署官网。

## 2026-10-18：悬浮窗全局快捷键（CHG-20261018-10，shared/floating-hotkey，待合并验收）

- 用户要求在设置中用键盘录入单键/组合键，一次展开、再次收缩。先登记统一验收并写前端测试，确认缺少模块失败后实现。主目录有未提交的 0.1.10 日历代码，创建 `.bundle-tmp/floating-hotkey` 独立工作树，基线快照 e3d6726；未覆盖主目录文件。
- `floatingHotkey.ts` / `FloatingHotkeySettings.tsx` 实现捕获、显示、保存、取消、清除；默认未绑定，说明单键占用正常输入、F12 保留。`floating.ts` / `useFloatingHover.ts` 接入切换与展开保持，鼠标在外不收回，再按强制收缩；原有编辑保持、拖动和贴边状态继续复用。`App.tsx` 接入设置。后台 `floating_hotkey.rs` 管理状态、独立配置与 30 秒录入租约，`platform/windows/hotkey.rs` 使用 RegisterHotKey + MOD_NOREPEAT 的阻塞消息线程，不采集键盘文本或安装键盘钩子。关闭浮窗、清除数据和退出释放注册；Mac 明确不支持。
- 前端 `npm test` 74 通过、`npm run build` 通过；`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib` 70 通过/5 默认忽略；额外显式运行 Windows 原生冲突/释放测试通过。检查覆盖单键/组合键编码、系统保留键、鼠标在外保持、编辑中收缩、贴边防立即重开、拖动中忽略、冲突/写盘失败回滚、停用/恢复及旧录入租约不干扰新录入。
- 早期 0.1.11 曾构建 NSIS 并原位安装，旧 exe/uninstall 保存在本工作树忽略目录 `.bundle-tmp/before-hotkey`。Windows 实际按键 F8 长按只展开一次，再按收缩通过；设置录入单键及组合键的前端捕获通过。首次保存发现 C 盘应用目录 fs::rename 失败；尝试原生替换后首次保存成功，但组合键替换仍未持久化。最终改为双槽序号写入 + sync_all，坏槽回退，清除时先保存空值再按旧到新删除。不依赖重命名；在 E 盘测试和 C:/Users/example/AppData/Roaming/app.tokendaily.local 内的独立临时测试子目录分别验证反复保存、损坏回退和清除通过，测试目录已清理。
- 验证期间发现另一任务修改主目录并重启安装进程，用户明确回复“是，等另一边完成后合并更新”。之后不再覆盖安装版或重启应用；只继续独立代码检查。最终双槽修复尚未构建安装包/安装态验收；组合键原生切换、重启恢复、录入超时、Esc 和贴边真实窗口验收仍待合并后完成。清理本轮两个临时快捷键配置，恢复原未绑定状态；原显示浮窗与贴边均 true，恢复/开关测试结果保持 true，未修改其他偏好或授权。
- 下一步：等并行费用/周榜任务完成，逐块合并共享 `App.tsx` / `lib.rs` / 版本和文档；保留对方新功能，统一新版本号后运行合并测试、构建和原位升级。使用独立产物目录或协调构建，避免不同任务同时覆盖根目录 Release 安装包。暂时版本号 0.1.11 与对方重叠，不能直接拿本轮旧安装包再次覆盖。无官网部署/远端推送；Mac 未编译或实机验收。

## 2026-10-18：快捷键合并与 0.1.12 安装（真实键盘待反馈）

- 用户回复“另一边完成了”，恢复合并与原位更新。将本地 shared/floating-hotkey 的 323896b 相对 e3d6726 的功能增量合入主目录现有未提交修改；保留等效 API 费用、周榜、持续编辑提示、用量明细和日历。没有清理或覆盖其他来源文件，没有提交/推送主目录改动。
- `App.tsx` 增加设置组件，原生注册/状态服务/窗口事件与另一任务无冲突；公共版本统一 0.1.12。首次批量更新版本误改同号 winapi-util 锁项，Cargo 拒绝后立即恢复原 0.1.11 依赖，未更新任何第三方依赖；随后离线锁定构建通过。
- 检查：`npm test` 76 通过；`cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` 76 通过/5 默认忽略；额外显式执行原生冲突/注销测试通过；`cargo fmt --check`、`git diff --check`、TypeScript/Vite、Windows Release/NSIS 通过。
- 旧安装 exe/uninstall 保存在 `.bundle-tmp/release-0.1.12-backup`，新包 `/S /UPDATE /D=<installation-directory>` 原位安装成功。安装包 SHA-256 `b83186ad57003ff2f01afc82431ab21e46461114e8c6c09ff26f586fcba616d5`，安装 exe SHA-256 `e211287f847ddd547af42b9fabca2871d4fe4010afdbdda38c101757d9b337b5`，文件版本 0.1.12。
- 安装态：F8 及 Ctrl+Shift+K 键盘录入、双槽重复保存、取消录入恢复注册、浮窗停用释放与重开注册、程序重启恢复、30 秒录入超时恢复通过；WebView 输入 Esc 后窗口 300×528 收为 56×56。设置和展开浮窗截图已目视，费用/周榜/140 格日历保留，无裁切；这些数据来自原本已连接来源，未写入演示数据或改变参榜同意设置。
- 真实 Windows 按键注入未触发事件；进一步检测注入后的 Ctrl/Shift/K GetAsyncKeyState 全为零。因此不把模拟输入结果当成用户键盘失败，也不声明真实全局切换/长按/贴边联动已验收。通过异步问题请用户实际按 Ctrl+Shift+K，尚未收到答复。临时组合键保留供用户测试，待其确认后按原计划恢复为未绑定（或按用户指定保留）；原浮窗/贴边开关均 true。收尾恢复正常启动，关闭 9223 调试接口。Mac 未编译/实机验证；无官网部署/远端推送。

## 2026-10-18：托盘右键入口与 0.1.13（CHG-20261018-11）

- 用户要求从 Windows 托盘图标右键打开悬浮窗、设置快捷键。`lib.rs` 新增两个菜单项及其命令，原“打开”明确为“打开主面板”；刷新、设置、退出和左键行为保留。关闭浮窗时展开菜单会开启并保存开关，已展开时重复选择不切换为收起。Mac 两个入口禁用。
- `floating.ts` / `useFloatingHover.ts` 增加幂等展开与保持，复用已有快捷键收缩；`App.tsx` / `FloatingHotkeySettings.tsx` 定位设置并在配置读取完成后聚焦录入按钮，不自动启动捕获。安装验收发现启动期间事件可能早于界面监听，增加本机内存请求标记，前端先注册监听再领取请求；不写统计或账户数据。
- 先写幂等展开测试并确认旧实现缺少方法失败，再实现。最终 `npm test` 77 通过；`cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib` 76 通过/5 忽略；TypeScript/Vite/Release/NSIS、cargo fmt 与 git diff --check 通过。版本替换时锁文件文本曾格式错误，已修正为 tokendaily 0.1.13，无第三方依赖更新。
- 原生菜单验收用已安装 tray-icon 的回调消息打开 Windows 菜单，读出“展开悬浮窗 / 设置快捷键… / 打开主面板 / 刷新 / 设置 / 退出 TokenDaily”，菜单命令路由展开及设置可用。自动菜单关闭消息一度影响收缩/焦点，未将该轮重复展开结果作通过依据；最终安装版直接调用菜单同一处理命令，验证首次设置焦点、不自动录入、重复展开仍 300×528、关闭后自动开启。设置截图已目视，无裁切。物理鼠标右键点击和贴边快捷键联动未重新验收；此前全局物理键盘反馈仍未收到。
- 原位更新 <installation-directory> 至 0.1.13，0.1.12 exe/uninstall 备份位于 `.bundle-tmp/release-0.1.13-backup`。最终安装包 SHA-256 `b26f53ca850d1daa760d43dd539eb4e2833e0084fc214d47dd8ed57234d4837f`。现有 Ctrl+Shift+K 保留，未改贴边偏好、授权、统计或上传设置；真实来源继续运行，无演示数据。最终关闭 9223 调试接口，恢复正常启动。
- 保留根目录已有未提交修改，本轮未提交、推送或部署官网。Mac 未编译/实机验证。

## 2026-10-18：悬浮顶部三列指标与 0.1.14（CHG-20261018-12）

- 用户要求顶部为“用量｜预估费用｜排行”。先登记统一验收，应用 frontend-design 指导沿用青绿色与细线分隔。`FloatingPanel.tsx` 与新增 `floatingSummary.css` 实现三列数字，分别注明今日 Token、美元覆盖状态、本周 Codex 排名；保留来源状态、余额和日历，不改变采集、估价或参榜上传逻辑。
- `floatingSummary.ts` 增加独立金额/单位状态和排名/范围字段。未知使用破折号，已确认零金额才显示 $0.00，部分与上次数据明确标记；费用完整悬停说明继续强调等效 API 预估、非账单。未取得名次显示暂无排名，不误称未参榜。新增测试先确认旧实现缺少字段失败，再验证正常、未知、零值、部分、过期和待核查排名。
- 检查：`npm test` 78 项通过；`cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib` 76 项通过、5 项默认忽略；`npm run tauri -- build` 的 TypeScript/Vite/Windows Release/NSIS 通过；`git diff --check` 通过。本次没有第三方依赖变更。
- 原位升级 `<installation-directory>/tokendaily.exe` 至 0.1.14，安装器返回 0，旧 0.1.13 程序及卸载器备份在 `.bundle-tmp/release-0.1.14-backup`。安装包 SHA-256 `4cd3e58d6745e0970c8525b302adb235fb345ee098441d11ae120feb0dde8a00`。
- Windows 安装态实际浮窗 300×528，三列内容宽约 100/97/73px，真实用量、费用和排名均完整，无横向或纵向溢出；截图已目视，余额与 140 格日历保留。浏览器仅对布局注入明确标识的虚构长数字，9999.99、$12,345.67、#99,999 均完整显示，未写入应用数据。浏览器环境无 Tauri，既有排名事件监听报缺少原生接口，因此不作为数据功能验收；数据检查以 Windows 安装版为准。
- 收尾关闭浏览器预览和 9223 调试接口，正常重启安装版。保留现有 Ctrl+Shift+K、贴边偏好及账户授权，未改变统计/上传设置；此前物理快捷键反馈仍待用户确认，本轮不重复宣称通过。保留所有已有未提交修改，不提交、不推送、不部署官网。Mac 未编译或实机验证。

## 2026-10-18：顶部可配置组件与 0.1.15（CHG-20261018-13）

- 用户要求顶部去掉小字、改名“今日 Token”，并选择顶部维持一行、三项各自隐藏/左右排序、隐藏后均分宽度。应用既有 frontend-design 指导延用色彩和数字字体，取消常驻小字与来源拆分，完整数据范围/部分/旧值说明移至悬停及无障碍名称，底部统计状态保留；旧值数字使用灰色。窗口随顶部隐藏和下方组件数量调整高度，不改变数据来源或上传。
- 新增 `floatingMetrics.ts` / `FloatingMetrics.tsx`：独立顶部排序键，本机保存，长按编辑、整块左右拖动、左右键排序、Delete 移除。与现有组件共用隐藏偏好及长按判定，扩展 `floatingComponents.ts` 支持三项新 ID；旧隐藏/排序不丢失。`FloatingBalances.tsx` 的添加选择器增加顶部三按钮；`App.tsx` 设置管理增加三开关，清除偏好同时清除顶部顺序。顶部与下方各自排序，不混排。
- `FloatingPanel.tsx` / `floatingSummary.css` 去掉小字与拆分行，顶部只保留标题、数字及数量单位。`floating.ts` / `useFloatingHover.ts` 缩减基准高度 50px，全隐藏再缩短 64px，编辑保留恢复操作空间。全部七项隐藏时仍提供添加组件入口。
- 测试先确认缺少新模块失败，再实现迁移/隐藏/恢复/独立排序和尺寸测试；`npm test` 80 项通过，TypeScript/Vite/Windows Release/NSIS 构建通过，`git diff --check` 通过。本轮仅共享前端交互和版本变化，未重跑 Rust 单元测试；上一轮为 76 通过/5 忽略。一次辅助 Python 命令因本机不可用退出、未改文件，改用项目 Node 完成接线，之后测试/构建通过。
- 原位安装 0.1.15，安装器退出 0，旧程序/卸载器备份在 `.bundle-tmp/release-0.1.15-backup`。安装包 SHA-256 `2334f844c7435bf9ff49e47fec735662c34cd5c0ffe169346290d81fc8a1e5a6`。安装态通过 WebView 指针事件测试左右拖动及 720ms 长按、键盘排序、Escape 取消拖动且保留编辑、再次 Escape 退出编辑；逐项移除后两项/单项等宽，全部隐藏可恢复，重新加载保留排序。三个顶部全隐藏、七项全部隐藏及添加菜单均无溢出；正常约 300×479、编辑约 300×543、全隐藏 300×172（当前 DPI 取整）。截图已目视，140 格日历保留。
- 验收只临时调整展示偏好，结束恢复原三个存储键，未修改账户、采集、参榜、快捷键或贴边配置。数据为已有真实来源，未注入演示值；排名/日历初次载入时未知保留未知。关闭临时调试接口后恢复正常启动。无 Git 提交/推送或官网部署，已有未提交修改保留；Mac 未编译/实机验证。

## 2026-10-18：编辑留白修复、统计范围与身份入口（0.1.16，CHG-20261018-14）

- 用户报告退出编辑后顶部/日历下方留白，要求移除底部“Token 统计不完整”并增加身份入口；确认暂用游客/设备参榜身份，已有昵称显示首字头像，不实施正式账号体系。
- `floatingLayout.ts` / `useFloatingHover.ts` 用版本号记录未完成尺寸请求；拖动期间跳过或被新状态替代的尺寸请求在指针释放后补做。`FloatingPanel.tsx` 点击空白退出编辑；两组组件在退出时取消按压/拖动状态。保留真实窗口拖动和快捷键逻辑，避免仅靠 CSS 把内容顶上去掩盖窗口高度。
- `UsageState` 增加向后兼容的 `sourceUnavailable` 默认 false 字段；Claude Code / WorkBuddy 明确没有有效日志或目录不存在且尚未观察到用量时标识未接入。未知初始状态与权限/损坏错误不被当作未接入；采集器在进程内记忆曾读取的有效来源，后续日志丢失/跨日失败仍提示异常。`combinedUsage.ts`、总览与浮窗按启用状态和此标识过滤范围，费用采用相同来源范围，全部未接入保持未知而非零。修正过滤后详情入口的原来源索引映射，组件隐藏仍不改变统计。
- `floatingIdentity.ts` / `useFloatingRank.ts` / `FloatingPanel.tsx` 复用只读身份状态，底部文字替换为人物图标或昵称首字符。点击用本机待处理导航标记与主窗口事件打开现有参榜页；仅导航，不调用 start/enroll 或修改同步开关。昵称头像是本地文字头像，并非远端账号照片；无正式跨设备账号登录。缺失/旧值说明仍在顶部悬停与详情中。
- 测试先行：新增前端测试在缺少模块/范围过滤时失败，Rust 未接入/损坏日志断言先因缺少契约字段失败，再实现。`npm test` 83 项通过；完整 `cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib` 76 项通过、5 项忽略；补充已观察来源丢失断言后 `... --lib usage::tests` 26 项通过、3 项忽略；cargo fmt、TypeScript/Vite/Windows Release/NSIS 与 git diff --check 通过。
- 原位安装 <installation-directory> 0.1.16，安装器退出 0；旧 0.1.15 程序/卸载器备份 `.bundle-tmp/release-0.1.16-backup`。安装包 SHA-256 `4750dc8e6be22edcdc748be2a8dde5efd792f7eaf15814cf19c876a0da219a89`。
- Windows WebView 指针验收：六次进入编辑后点击底部空白（含按住再松开），均恢复约 300×479；对 set_size 增加临时 180ms 延迟，再快速退出编辑，最终仍正常高度。正常面板无横/纵溢出，140 格日历保留，截图已目视。真实 Claude Code 状态 unknown/sourceUnavailable=true，顶部用量说明不再因该来源未读而报不完整。当前参榜昵称首字头像正确，点击打开身份页，操作前后 loggedIn/enabled 不变。
- 本轮未注销真实设备身份来验收游客页面；游客/身份未知/昵称头像分支由共用单元测试覆盖。恢复原展示偏好、移除临时 IPC 延迟、关闭调试端口并正常启动。保留所有既有未提交修改，无 Git 提交/推送和官网部署。Mac 未编译/实机验证；契约影响记录在 ARCHITECTURE 与 FEATURE_PARITY。

## 2026-10-18：本地提交交接（codex/release-0.1.16）

- 用户授权将改动提交 Git，明确由用户执行最终 push。本地任务分支基于 main 257e024，统一收录此前累积至 0.1.16 的 66 个源码、测试、锁文件和文档文件；不推送、不部署，不将 Windows 验收视作 Mac 同步完成。
- 提交前核对工作区/暂存区、版本与锁文件，依赖版本未变化；git diff --check 通过，新增文件均属于代码/测试/说明文档。扫描未发现常见私钥/API 密钥内容，确认 .bundle-tmp、工具链、npm 缓存、dist 和 src-tauri/target 受忽略规则保护，无原始日志、凭据、安装包或验收截图进入提交。
- 沿用前述同一代码版本的验证结果：前端 83 项通过，Rust 76 项通过/5 忽略，后续来源断言 26 项通过/3 忽略，Windows 0.1.16 构建与安装态通过。本轮只更新交接记录，不重复构建。Mac 未验证；后续由用户审阅并推送该任务分支。

## 2026-10-18：用户授权合入 main 与本地分支清理

- 按用户后续指示，main 从 257e024 快进到 0e3db76，0.1.16 功能提交已合入；无冲突、无代码改动，沿用上述测试与 Windows 安装态结果，Mac 未验证。
- 删除已合并的 codex/compact-token-display、codex/floating-card-drag、codex/floating-component-editor、codex/release-0.1.16、shared/total-token-calendar。
- shared/floating-hotkey 的原生实现和纯规则已原样进入主线，UI/测试包含后续增强；其旧提交历史并未形成主线祖先，先用本地标签 archive/floating-hotkey-323896b 保留，再删除分支名。两个占用分支的临时工作树均先确认干净并切换 detached HEAD，保留目录与缓存，不递归删除文件。
- 当前本地仅保留 main 分支。未推送、未删除远端分支，用户最终可执行 git push origin main；本地归档标签无需随主分支推送。本轮仅 Git 整理及记录更新，不重新运行构建或改变已安装应用。

## 主线 0.1.16 同步到 Mac · 2026-10-18 · mac/main-0.1.16-sync

- 按用户要求，以 Mac 6fa9e12 合入本地 main bea5435；保留双方提交和文档历史，实际解决六个文件的内容冲突。远端 fetch 此前返回 403，本次同步的是已核对的本地主线，不声称已核对更新的远端。未推送、发布、部署或合回 main。
- 接入统一 Token 简写、总 Token 历史、等效 API 费用、顶部三项指标、组件编辑/隐藏/排序、20 周日历与游客身份入口。保留 Mac 通知、身份核验、来源设置、FSEvents、透明浮窗和 WorkBuddy 专属钥匙串及后台恢复；参榜/全局快捷键保持不可用。
- 主线初始悬浮位置的 Tauri 工作区算法提取到平台公共模块，Windows/macOS 共用测试；Mac 窗口配置同步 center=false。主线历史扫描原本忽略 Mac 自定义 Codex 目录，本轮改为沿用 connection 设置；目录无效时显示缺失而非回退。浮窗积分复用 creditPresentation，新增测试先复现“恢复中”误写成“待授权”，再修正。
- 验证：npm test 83/83；npm run build；cargo fmt --check；bash scripts/mac-dev.sh cargo test --locked --features macos-transparent --manifest-path src-tauri/Cargo.toml：109 通过、5 默认忽略。新增历史目录约束测试。首次新增 IPC 使用借用 State 与异步返回的组合导致编译失败，改为 AppHandle 获取设置后测试通过。
- bash scripts/mac-dev.sh npm run tauri -- build --no-sign -- --locked 生成 Intel App/DMG，hdiutil verify 通过。DMG SHA-256：4830e041d48dd3cd31b5d2add33ec958adcf5ae6557312ed741607a6e7a5b89f。两份锁文件仅产品版本从 0.1.3 到 0.1.16，未升级依赖。
- 沿用用户已有本地临时签名/安装授权。旧版 ZIP 已完整校验，原 App 保留在忽略目录 .bundle-tmp/mac-validation/before-main-0.1.16-20261018-220650/。候选 ad-hoc 签名严格校验后复制到 /Applications 临时路径，再替换；构建与安装主二进制 SHA-256 一致：eb9fc8d7ae78e63ca2821267b4cf716ea3a35699e217a8240ded0677f3350118。DMG 内仍为未签名 App，未公证。
- 安装版 0.1.16 已启动，设置/providers/WorkBuddy 配置文件摘要升级前后完全一致。初始球体 56×56，位置 (1608,482)，距工作区右边 16 逻辑像素；首次展开为 300×478。WorkBuddy 钥匙串确认期间主窗与浮窗仍可操作，主面板显示“正在恢复授权”。系统因临时签名变化再次要求用户允许访问，已请用户在系统弹窗处理；未读取/代填密码，未绕过钥匙串。
- 待补安装态交互、用户目视及钥匙串允许后的积分恢复记录。Windows 整合提交、Apple Silicon、多屏/Spaces、正式签名升级与长期运行未验证；不称双端同步完成。

### 0.1.16 安装态补充（同轮）

- AX 检查顶部今日 Token/预估费用/排行、20 周日历无障碍入口和 140 个日期格均存在。进入组件管理后窗口 300×542，13 个可操作按钮均在窗口矩形内；移除费用→添加组件→恢复费用成功，完成后恢复 300×478。此处使用实际 AX 按钮，未把它称作鼠标长按/拖动或像素视觉验收。
- 总览的总 Token 历史实际读取本机日志，展示真实记录日、缺失日期和统计不完整提示；未使用演示数据或账户日桶。操作后恢复 WorkBuddy 来源和全部原显示组件。设置页显示 0.1.16、系统安全存储及 Mac 全局快捷键不可用提示。
- 当前仍等待用户在系统弹窗允许钥匙串访问及目视答复。没有取消其授权、触碰密码输入或反复触发恢复；为保留当前确认弹窗，本轮不执行退出/重启。余额恢复、重启、通知生命周期及真实拖动手势未重新验证。代码和安装已同步，不能将这些待验项写成通过。

## Mac 0.1.16 授权与目视补验 · 2026-10-18

- 用户确认已在系统弹窗允许访问，并目视确认顶部三项指标、余额条、日历完整及透明边缘正常。随后 AX 读取确认真实积分余额已恢复，恢复中/上次积分提示消失；本机 WorkBuddy 日志缺失仍独立显示未知，不与积分状态混淆。
- 使用应用自身 Quit TokenDaily 菜单正常退出，确认进程消失后从 /Applications/TokenDaily.app 启动。版本仍为 0.1.16，主二进制 SHA-256 仍为 eb9fc8d7ae78e63ca2821267b4cf716ea3a35699e217a8240ded0677f3350118；主窗和右侧球正常建立。
- 同一构建重启后再次观察到针对 app.tokendaily.local.workbuddy-credits 的系统确认，应用显示正在恢复授权且界面可操作。不能将一次“允许”当作持续授权或把本次重启记作免确认通过；已请用户在系统弹窗选择“始终允许”后再复验，也保留仅本次授权的选择。未读取/代填密码、修改钥匙串 ACL 或重新登录。
- 本轮仅安装态核验和文档记录，没有修改运行代码或重建产物；沿用同一构建的 83 项前端/109 项 Rust 测试，不重复运行。后续签名变化可能再次确认，正式发行签名连续性尚未验收。

### 0.1.16 持续授权后的重启复验 · 2026-10-18

- 用户确认已在系统弹窗选择“始终允许”。使用应用自身 Quit TokenDaily 正常退出，确认无进程，再打开 /Applications/TokenDaily.app；新进程单实例，0.1.16 与主二进制 SHA-256 保持不变。
- 重启后自动恢复真实积分，恢复中/旧积分提示消失；检查系统窗口未见 TokenDaily 专属钥匙串确认。分别读取主面板和展开浮窗的积分，数值一致。没有重新登录、重新签名或修改凭据存储；不记录账户和真实金额。
- 该项补验通过，取代上节“等待始终允许后复验”的待办；仅覆盖当前已安装构建，不保证后续临时签名变化或正式发行升级免确认。应用保持运行，用户已确认视觉正常。本轮只补充验证记录，没有代码变更或重新构建，未重复单元测试；无推送/发布。

## Mac 三项体验修复 · 0.1.17 · CHG-20261018-MAC-PARITY

- 分支 mac/floating-hotkey-leaderboard，基于干净的 9cf63ea；用户截图确认数值被省略、快捷键和参榜被平台门面禁用。先登记统一验收条件后实施，保留已安装设置。
- MetricNumber/metricFit 使用真实字体宽度缩放三列数字，Mac 热键实现位于 platform/macos/hotkey.rs；共享服务派发到主线程以避免锁等待。Command/Option 标签、菜单和设置入口启用。
- 参榜新增平台安全存储门面与独立 Mac 钥匙串，原 Windows DPAPI 格式兼容。后台延迟恢复，缺失/拒绝/损坏分开处理，失败禁止创建替代身份；显式重试。预览与定时同步沿用当前 Codex 来源，来源改变要求重新预览。版本及锁文件同步 0.1.17，未升级依赖。
- 测试：npm test 85/85；Rust 113 通过、8 默认忽略；cargo fmt --check 与 TypeScript/Vite 通过。显式执行原生 Carbon 冲突/旧绑定保留/清除/退出释放、独立钥匙串往返，各 1/1；TOKENDAILY_RANK_LOCAL=http://127.0.0.1:8787 的 local_trial_consent_restart_sync_pause_delete 测试通过，覆盖预览不上传、同意后上传、重启/去重、暂停/恢复/删除。全程模拟数据与本机服务，清理自建钥匙串条目，不访问真实参榜身份。
- Tabbit Chromium 与本机 WKWebView 使用生产 MetricNumber/CSS 验证一至三列、1,538.76 万及极大值；数字实际字形宽度未超可用宽度，三列截图已检查。临时夹具/工具/截图均在忽略目录，未注入生产数据。首次集成夹具缺少累计输入输出字段，按真实日志格式补齐后通过。
- App/DMG 构建与安装态验证进行中；Windows 回归、Apple Silicon 和正式签名升级未验证，不标记双端同步完成。真实公开参榜需用户在产品内预览并确认，本轮不代为报名。

### 0.1.17 安装进展

- App/DMG 构建与 hdiutil 校验通过。沿用既有本地签名/安装授权，旧 App 与已校验 ZIP 保留 .bundle-tmp/mac-validation/before-mac-parity-0.1.17；新 App ad-hoc 严格验签，备份后替换 /Applications。已启动 0.1.17，构建/安装二进制 SHA-256 一致（5a436da216efc5a4aaa98d7e500a3f8ddd4641db239d4667f9aef618eb74d498），三个设置文件摘要相同。
- 安装态 AX 确认快捷键录入/榜单开始试用/网页版按钮已启用、未实现提示消失。试用身份未创建，真实用量未上传。
- 系统因临时签名变化再次询问 WorkBuddy 专属钥匙串访问，已请求用户处理。系统弹窗仍在时合成按键未被录入，已取消录入恢复未绑定；不将此次尝试记作键盘通过。暂不退出重启，以保留用户待处理弹窗。剩余为实际录入/全局切换/长按与重启，及当前构建 WorkBuddy 恢复确认。

### 0.1.17 钥匙串允许后的安装态补验 · 2026-10-18

- 用户确认系统弹窗选择“始终允许”。检测 TokenDaily 专属 WorkBuddy 确认消失，随后正常退出并从 /Applications 重启；0.1.17 主二进制摘要仍为 5a436da216efc5a4aaa98d7e500a3f8ddd4641db239d4667f9aef618eb74d498，单实例。未再次出现钥匙串确认，真实积分自动恢复，主面板与浮窗数值一致；没有重建、重新签名或重新登录。
- 安装态录入临时 Shift + ⌘ + K 并保存，设置显示正确且无注册错误。先前自动注入未录入的原因是 AX 点击未让主窗口成为键盘焦点；使用 AXRaise/Main/Focused 后，同一已安装应用正确接收系统按键，无代码修补。
- TokenDaily 处于后台、独立验收空窗口占据前台时，通过 CGEvent 系统按键路径验证 56×56→300×478→56×56；指针在外仍保持展开。另发送一次按下加七次自动重复按下再抬起，仍只展开一次，再按正常收起。这是系统合成按键验收，不称人工物理键盘验收。
- 独立 Carbon 独占注册探针确认：正常绑定为占用，开始录入可注册，取消后重新占用；关闭浮窗可注册，重新开启占用；应用退出可注册，重启恢复占用。重启后设置保留绑定且按键再次可展开。
- 验收结束在设置清除临时绑定，原生探针确认已释放、持久化最新槽为 null，恢复原来未绑定状态和 Codex 来源选择。settings/providers/workbuddy 三份原设置摘要均一致；本地快捷键双槽为本轮新增的空偏好。参榜身份未创建、真实用量未上传。
- 本轮仅安装态补验及文档，沿用 81180d2 对应的 85 前端/113 Rust/3 显式专项测试，不重复构建。替代上节键盘与重启待办；Windows 回归、Apple Silicon、人工物理键盘、多屏/Spaces、正式签名升级和真实生产参榜仍未验证。

## 2026-10-19：换日/换周后空榜调查

- 用户报告昨日有排名、今日消失。只读核对线上日榜/周榜 API，均 HTTP 200；北京时间今天为周一，两个窗口都是 2026-10-19 至 2026-10-19，当前无本周期事件，因此 total=0。服务端通过 enrollment 与本周期 usage_event 内连接生成榜单，无新周期事件的已有参榜者不会出现在结果中。
- 生产 D1 仅执行聚合 SELECT：10/13–10/18 历史仍在，10/18 已接收 209,611,397 Token；参榜身份 1、active=1、held=0，最近成功同步为 10/19 09:49:22（北京时间）。两项查询分别读取 8,780 和 1 行，均 0 写。没有删除、重建或手工补写数据，没有重新参榜或上传用量。
- 结论：此截图不是历史记录丢失或参榜资格取消；周一重置与日界同时生效，当前 UI“0 位参与者/还没有公开记录”不能清楚区分未参榜与本期未产生用量，容易造成掉榜误解。桌面排名同样取本周，未返回名次时显示破折号。
- 已向用户说明事实，并询问新周期展示偏好：保留参榜者为“0/待上榜”，或继续只列有用量的人但明确切榜说明。展示方案待确认，未改变产品规则、生产配置或源码；本轮未运行测试（只读调查与记录），未提交/推送 Git 或部署。

## 2026-10-19：新周期保留身份，0 Token 正常排名（最终 0.1.18）

- 用户先选择保留参榜者为“待上榜”，随后明确修改为“消耗是 0 也正常排名”。最终规则为普通 RANK，同分并列，全部为 0 时并列第 1；已有正数时 0 排在后面。0 指服务端本期已接收量，不保证本机无使用；旧日/周数据不删除，不重新报名。
- 官网在相邻 TokenDaily-Web 工程修正原本把无本期事件的参榜者排除的内连接：先按时间范围聚合，再与已同意参榜的身份左连接；未收到记录为 0，参加正常名次。暂停仍保留身份，隔离不显示，退出/删除移除；双语列表和零用量名次分享卡可用。无需迁移或手工修改数据库。
- 桌面 `floatingSummary.ts`、`Leaderboard.tsx` 保持零用量的实际数字名次，保留 nullable 名次的兼容读取但未知仍显示破折号，身份页注明已同步。新增零用量实际数字名次（合成用例为并列第 1）、隔离、旧服务空名次兼容测试。版本经过中间 0.1.17，最终为 0.1.18；README、LEADERBOARD、交接和功能账本按最终规则统一。
- 测试先确认旧代码不满足新周期保留身份及待上榜分支，再按用户最终规则更新回归。桌面 npm test 84 项通过；TypeScript/Vite/Windows Release/NSIS 通过；本轮没有修改 Rust 业务逻辑，未重跑 Rust 单元测试。官网 npm test 25 项、最终部署门禁 20 项、隔离 D1/真实 OAuth 库和成本测试、build/check 通过。新周期无事件查询在 12,514 条保留历史下只读 13 行、0 写；正数周榜 6,257 条事件约 12,525 行读，保持现有预算。
- 官网先发布过本轮初版 6bdb4569-e8a8-4155-9dcb-78e86acfae39，用户修改后最终 Worker 为 a24e8a19-87ab-44e7-86a3-dd23c5a3b8f2。官网资产/认证、D1、密钥、Cron 和免费套餐沿用；下载仍为官网此前已验收的 0.1.14。只读复核昨日记录仍为 [用量已脱敏] Tokens（1,945 行读/0 写），日/周 API 均 HTTP 200；本轮客户端自动同步后真实账号收到 [用量已脱敏] Tokens、排名 #[名次已脱敏]，未代替用户手动上传。
- 最终原位安装 <installation-directory> 0.1.18，安装器返回 0；旧 0.1.17 程序/卸载器保存在 .bundle-tmp/release-0.1.18-backup，更早 0.1.16 备份在 release-0.1.17-backup。最终安装包 SHA-256 `827356067001daf28468cade582143a5ceba600987c1aa3280f21e965de4002e`。安装态真实身份仍有效、上传开关不变；悬浮窗/身份页显示 #[名次已脱敏]，无 #null，300×479 无溢出、140 格保留，桌面截图已目视。
- Tabbit 合成数据覆盖中英文 1280/390/320，零值在有正数者后显示第 2、分享按钮可用，日周切换保留身份；正式页面新提示与 #[名次已脱敏] 已核对。Tabbit 截图超时，当前 CLI 不支持提示的前台选项，因此不宣称网页截图目视通过。真实生产已有正数，未制造零值参榜者或更改公开数据；零值/并列通过共用、隔离数据库和浏览器合成测试验证。
- 收尾保留用户原有身份、快捷键、组件偏好，关闭本机预览和临时 9223 调试接口。Mac 未编译/实机验证；不执行 GitHub push。

## 2026-10-19：TRAE 个人版与本机来源可行性

- 用户要求接入 TRAE 用量/额度，随后询问能否通过本机日志读取。先核对安装版本与数据结构，没有修改其登录、配置或原文件，没有为了验证发送模型请求。
- 官网已登录个人用量页可返回真实积分包和会话精确 Token，但它是账号口径。CLI 导出令牌在积分端点被拒绝，没有继续尝试其他凭据或解密客户端登录信息；也未将网站凭据保存到 TokenDaily。
- 初查普通日志未发现精确计数，本地 database.db 不能按明文 SQLite 打开。随后参考公开一手代码，使用公开应用级数据库密钥完成页 HMAC/格式验证；只读数据库+已提交 WAL，内存解密并查询必要计数和时间，前后文件状态一致，无明文数据库文件落盘。
- 真实样本有 18 条含用量记录；今日输入 [输入量已脱敏]、输出 [输出量已脱敏]，总计 [总量已脱敏]，与官网对应记录一致；缓存读取 [缓存量已脱敏] 为独立字段，不能再加一次。本次读取约 9.68MB，不代表已完成增量采集或性能验收。详细来源边界、同步历史归属和生产接入验收列在 TRAE.md 与 CHG-20261019-02。
- 试读命令：加载 scripts/use-local-dev-env.ps1 后运行 Git 忽略目录中的 trae-local-probe.cjs、trae-db-probe.cjs；一次第一页认证和两次一致快照查询通过。使用隔离临时目录中的 sql.js（忽略安装脚本），没有变更项目依赖或执行第三方采集器。原始数据及临时脚本均不提交。
- 此轮仅调查、只读试验和文档，无产品源码/版本/安装包变更；未运行前端/Rust测试，未做新的 Windows 安装态验收，Mac 未验证。用户关于本机读取的可行性已有实测依据，监控接入尚待实现；不推送 Git，不部署。

## 2026-10-19：TRAE 本地 Token 与官方积分接入（0.1.19，CHG-20261019-02）

- 用户批准接入后新增独立 TRAE CN 来源；本地数据库 Token 纳入今日合计与按需总历史，官方积分独立授权/刷新，不混用官网账号用量，不扩大 Codex 参榜上传范围。支持主面板、详情、设置、托盘提示与悬浮长条；修正四来源上限把独立日历截断的问题，旧顺序/隐藏偏好保留。
- 新增 trae_usage.rs、trae.rs、trae_credits.rs 和测试；平台模块提供数据路径，Mac 入口禁用。Rust 内存 SQLite 只投影计数、时间和哈希去重标识；认证 SQLCipher 页与 WAL，拒绝损坏、冲突和不支持格式，不写明文数据库或修改原文件。数据库/WAL 各 64 MiB 上限、最多 200,000 条；未变化时不读正文、不重算聚合。两秒只查文件信息，变更最多每三十秒重读，手动至少十秒。
- 数据转换/范围及四卡+日历测试先写，前端先因未支持 TRAE 命令与第五个日历元素失败，再实现。Rust 覆盖缓存/推理不重复、非零累计与零回退、去重/修正、北京时间切日、溢出、损坏、页认证、WAL 未提交帧和余额精度/有效期。最后补出 Windows 新日文件被独占、元数据可读而正文不可读的失败用例，再增加新日/错误恢复时的只读打开验证，避免错误补零。
- TokenDaily 自有官方网页配置接收固定 api.trae.cn 积分响应的白名单字段；Cookie/令牌不进入 Rust/React，不读取或解密客户端登录，不扫描进程内存。随机票据、来源/过期校验与独立清除；默认五分钟、手动十秒、失败六十秒。用户本人完成官方登录，真实余额 [原始余额已脱敏]（3 个积分包），有效期与重置分离；重启恢复与真实手动刷新、连续点击节流通过。首次验收捕获异步关窗/重建竞争，改用销毁并等待管理器移除，重新打包安装后复测通过。
- 本机真实今日 Token [用量已脱敏]，与官网对应记录一致；Release 手动全量刷新含 IPC 65.43ms，立即重复 7.31ms；Debug 首次读取约 941ms。仅当前约 4.36 MiB 数据库样本，不是海量/长期性能承诺，未为验收发起模型请求。TRAE 恢复/同步记录可能来自其他设备，详情和悬停注明本地记录；没有模型计价字段，TRAE 费用不估算，费用合计保持部分覆盖。
- npm test 86 项通过，TypeScript/Vite、最终 Windows Release/NSIS 通过；完整 cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib 85 项通过/6 忽略；真实 live_trae_database 显式通过。共享格式检查、差异检查均执行。测试使用合成数据，真实账户/数据库验收另记，不把合成响应当成功登录。
- Windows 原位安装 <installation-directory> 0.1.19，旧 0.1.18 程序/卸载器备份于 .bundle-tmp/release-0.1.19-backup。安装态总览/详情显示 [用量已脱敏]  TRAE Token，浮窗同时展示四 AI 卡片和 140 格日历，300×540 无横纵溢出；退出编辑恢复正常高度；真实 [余额已脱敏] 积分完整可见，截图目视通过。第三方格式参考的 MIT 声明随包安装。
- 未验证完整五分钟自动刷新等待、真实多请求累计、同步历史的设备归属、其他 TRAE 版本/国际版、超大数据文件和长期运行。Mac 未编译/实测，不标同步完成；保留任务分支，无远端推送或网站部署。最终安装包摘要、调试清理和提交 SHA 在收尾记录补充。

### 0.1.19 收尾

- 最终 NSIS SHA-256：ADE82604D36ACCA5488E5BC0364B68B59CFDC4C8BCBD04498427E48821756B10，原位安装退出 0，已安装版本 0.1.19；第三方声明文件已安装。最终程序与构建二进制仅相差 Tauri 打包写入的 NSS/UNK 三字节 bundle-type 标记，其他字节一致，确认运行的是最终构建。
- 最终安装态再次核对 Token [用量已脱敏]、积分 [原始余额已脱敏]、状态 available；重启恢复、真实手动刷新和连续刷新节流复测通过。键盘将新 TRAE 卡片排到日历上方，原有 AI 卡片相对顺序保留；隐藏 TRAE 不停止采集或清除授权，恢复原隐藏偏好。
- 还原主面板总览，保留既有参榜身份、上传开关与快捷键，关闭验收连接，退出调试实例后正常启动；本机 9223 调试端口已确认关闭。未为了验收清除真实授权，未改 TRAE 客户端或发送模型请求。Mac 未验证，功能保留任务分支；未推送或部署。
- 收尾检测到并行的悬浮排名荣誉图标改动（CHG-20261019-03），其源码/样式/测试未纳入本次安装包和提交。对重叠的 FloatingPanel 与 FEATURE_PARITY 只暂存 TRAE 部分，工作区保留另一任务内容；另在忽略目录导出仅本任务的快照，86 项前端测试与构建再次通过，JS/CSS 哈希与安装版一致。

- 本次 TRAE 实现提交：d57fadd28d4c3bf16d85b0e0a2055b983b721c79（codex/trae-cn-monitor）；Windows 0.1.19 的实现与验收对应此源码提交。后续仅补充本行与功能账本 SHA，无产品代码变化。并行排名荣誉任务保持未提交，不推送、不合并到主分支。

## 2026-10-19 悬浮排名荣誉图标（源码完成）

- 根据官网任务明确请求，悬浮窗排名加入冠/银/铜与星耀/精英/进阶/新锐图标。沿用本周 Codex 范围，使用服务端完整榜单 rankEnd 与 honorTotal；前三不显示百分位，0/未知/待核查不授奖。旧接口缺字段仍显示名次但不推测徽章。暂停保留上传记录荣誉。
- 新增 domain/leaderboardHonors.ts、ui/RankHonorIcon.tsx，图形与官网固定 SVG 一致；图标与名次同一行，荣誉名称通过原有悬停与无障碍标题提供。不增加 API 调用、自动报名、上传或原生依赖；保留此工作区 TRAE 改动。
- npm run build 与 npm test（88项）通过；新增完整同分边界、50%截止、缺字段/0值/待核查与暂停测试。Tabbit 使用实际 FloatingMetrics 与 RankHonorIcon 合成预览，在284px内容内七类图标及普通/零值名次无横向溢出；不是原生安装态验收。
- 未打包或替换已安装0.1.19；官网仍下载已验收0.1.14。Mac未实机验证。官网模拟混排生成器已开发但生产未开启，披露方式待用户回复；悬浮窗仍读取服务端实际名次。未提交整个桌面工作区或push，避免混入并行TRAE任务。

## 2026-10-19：显示统一两位小数（0.1.20，CHG-20261019-04）

- 用户要求数据展示保留两位小数。新增前端 formatCreditAmount 和共享 Rust credit_display，仅按十进制字符串四舍五入，不把显示字符串写回余额。浮窗、主面板、设置与逐包详情统一两位，托盘提示相同；原本已经两位的费用/Token 缩写沿用，整数 Token/名次不补 .00。
- 先修改测试，旧模块因缺少新格式化导出失败，再实现。覆盖精确半值 1.005、跨整数进位、小额、零/未知、整数补零、大于 JS 安全整数的余额与原始值不变。前端 87 项、Rust credit_display 1 项通过；TypeScript/Vite/Windows Release/NSIS 通过。此轮只运行与显示改动相关的 Rust 测试，未重跑完整 Rust 测试；Mac 未编译/实测。
- 从 HEAD 导出独立构建快照，只叠加本任务文件，排除正在并行修改的排名荣誉前端及后端。Windows 原位安装 <installation-directory> 0.1.20，退出码 0；旧 0.1.19 备份于 .bundle-tmp/release-0.1.20-backup。安装包 SHA-256 31883031CF9EE618FC568513078E9BADE12E57B878CD93C021B009CE0215536D。
- 安装态确认总余额显示 [余额已脱敏]，逐包 [逐包余额已脱敏]，后端原始余额仍为 [原始余额已脱敏]；浮窗 300×540 无溢出，截图目视通过。独立登录恢复正常；只读核对参榜身份 loggedIn/enabled/enrollment 均保持，不修改上传设置。验证完还原总览并正常启动，关闭 9223 调试入口。
- 实现提交 ab5d91b14f249a4e372e88cdc52455c3e447026a，仅本地提交，未推送或合入主分支；并行排名荣誉的源码、样式、测试及文档原样保留。

## 2026-10-19 官网本人定位与悬浮荣誉发布（0.1.21，CHG-20261019-05）

- 根据用户选择，桌面“在官网查看我的排名”携带已经认证的本人公开UUID片段；校验loggedIn、参榜用户与me一致、归属本机设备、未待核查及UUID格式。未满足时只打开普通榜单，不携带令牌或设备标识。官网消费片段并记住公开ID，本人行始终置顶，可清除；与网站账号登录独立。
- 已包含CHG-20261019-03的悬浮荣誉实现，沿用本周Codex与完整同分/分母规则；未知与0值不推测奖章。沿用原有status调用与刷新，不改账号、额度来源、参榜上传开关。
- 保留另一任务提交ab5d91b的0.1.20积分两位小数实现。在.bundle-tmp/honors-release-0.1.21创建隔离构建快照，源文件摘要确认构建期间无变动；待0.1.20任务完成安装验收并关闭调试后，提升工程版本并原位安装到<installation-directory>。
- 89项共享前端、完整Rust87通过/6默认忽略、格式检查、TypeScript/Vite、Windows Release/NSIS通过。新增公开链接测试覆盖身份不符、未参榜、待核查及非法UUID；未读取或打印凭据。
- 0.1.21安装包SHA256：2961665A2959EE325FD227D55C1F510A9A80B6A932761E145F48E5802F715036；旧程序/卸载器保存在.bundle-tmp/release-0.1.21-backup。安装态真实悬浮窗显示绿色精英勋章与#[名次已脱敏]、积分两位小数，300×540无溢出；截图.bundle-tmp/honors-0.1.21-installed.png已目视检查。首次UI等待遇到异步加载未完成，待status返回后重测真实按钮成功。
- 实际点击原生“在官网查看我的排名”打开正式网页，Tabbit确认首行“我”、荣誉在昵称后、无百分位文本、浏览器记忆及片段清理通过。还原总览，退出调试实例后正常启动，已验证运行版本0.1.21及9223无监听。
- 官网配套Worker已发布，公开下载仍为0.1.14。Mac未编译或实机验证，参榜入口保持禁用。本轮只本地提交，不push，不变更付费套餐、身份或上传设置。
## 2026-10-19 v3荣誉图标与排名后勋章（0.1.22，CHG-20261019-06）

- 用户提供TokenDaily-Honor-Icons文件夹，要求更新图标并先比较悬浮前后位置。以真实尺寸/样式生成两版交互预览，用户明确选择B：名次在前、勋章在后。导入7个原始SVG至assets/honors，RankHonorIcon采用静态本地导入，22px正方形，不引入远程资源或改变授奖规则。
- 保留前三/5/10/25/50、同分/0值/未知行为和原有无障碍荣誉标题；仅更新图形与排列，不修改原生逻辑、账号、额度来源或上传。官网昵称后、说明和分享卡同步采用新图标。
- 89项共享测试、TypeScript/Vite构建与Windows Release/NSIS通过；本轮没有Rust业务逻辑变动，未重复完整Rust测试。Windows原位安装<installation-directory> 0.1.22，退出码0，旧程序/卸载器备份于.bundle-tmp/release-0.1.22-backup。
- 安装包SHA256：E9ACEC4CC3884F6E334B484B4692DCC45368DA1DFC562F54AB2AE81B69A470D4。真实安装态确认当前#[名次已脱敏]在前、精英钻石在后；实际img解码成功，路径与原SVG一致，22×22、300×540无溢出；截图.bundle-tmp/honors-0.1.22-installed.png已目视验证。当前真实参榜身份保留；已退出调试实例并正常启动，9223关闭。
- 官网Worker 0edd746c-b8e6-4c18-87d1-70116cbab3cf已发布，公开下载仍为0.1.14。Mac未实机验证。本地提交，不push，不修改任何付费或账号授权设置。

## 2026-10-19 本地汇总主分支与分支清理（Windows）

- 按用户明确要求，将 codex/trae-cn-monitor 上已提交的 TRAE 监测、积分两位小数、官网本人定位及悬浮荣誉图标改动统一汇总至 main，版本保持 0.1.22。起始 main 为 8bb5728，任务分支源码为 dafe5a2；保留各功能原始提交，不压缩或改写历史。
- 已执行 git fetch origin，远端没有新增分叉；工作区干净，git diff --check main..HEAD 通过。核对提交文件及 .gitignore：凭据、原始日志、数据库、构建产物和本机工具链均未加入本轮提交。
- 汇总源码重新执行 npm test（89 项通过）、npm run build（TypeScript/Vite 通过）、cargo fmt --manifest-path src-tauri/Cargo.toml --check（通过）、cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib（87 项通过、6 项默认忽略）。本轮不重复打包或安装，Windows 0.1.22 安装态依据上一条记录；Mac 未编译或实机验证，不标记跨平台同步完成。
- 此次按用户授权直接本地集成，采用 fast-forward 合入 main，并仅删除已被 main 完整包含的本地 codex/trae-cn-monitor 分支；旧 detached worktree 与缓存保留。本轮不推送、不删除远端分支、不部署，远端推送由用户执行。

## 2026-10-19 官网下载更新为 0.1.22

- 用户明确要求“新版软件应用更新到官网”，在相邻 TokenDaily-Web 工程更新中英下载配置、更新说明、TRAE CN 支持表和来源边界，使用现有已验收安装包，不重新构建或替换本机应用。README 同步修正当前版本和官网下载状态；Mac 仍未验证。
- 安装包 TokenDaily_0.1.22_x64-setup.exe 为 9,949,782 字节，SHA-256 E9ACEC4CC3884F6E334B484B4692DCC45368DA1DFC562F54AB2AE81B69A470D4。官网完整下载后摘要和大小与此包完全一致，HTTP 200、application/octet-stream、attachment；正式地址 https://tokendaily.net/downloads/TokenDaily_0.1.22_x64-setup.exe 。
- 官网 npm test 35 项、8 页面/发布信息检查、32 项发布门禁、隔离本地 D1 成本和 OAuth 生命周期通过。Tabbit 核对中英本地 1280/768/390/320 与正式 1280/320 的版本、双下载入口、TRAE 支持和校验值，无页面横向溢出。本轮未重新进行 Windows 安装态验收，沿用上述 0.1.22 记录；桌面业务代码不变。
- Cloudflare Worker 9fcd8945-a736-4871-a649-80ef1d68405c，发布前版本 75cfebca-1c78-40fc-a7b8-567a507d592b；仅新增/更新安装包和中英首页共 3 个静态资产。榜单、账号及认证平台接口 HTTP 200，API 源码、生产 D1、Secret、Cron、模拟参与者与套餐不变。HTML 只有 Cloudflare 现有 beacon 注入差异，未修改相关账户配置；原 0.1.14 配图明确标演示版本，旧安装包仍可访问。
- 预览服务已停止，浏览器保留正式下载区域。源码和文档仅本地提交，远端 push 由用户执行。

## 2026-10-19 悬浮窗签名更新与真实原位升级（0.1.23）

- 按用户要求在右上角主面板按钮前增加更新入口，常态可检查，发现新版有下载图标/提示点，悬停展示版本；点击开始下载、验签、原位安装和重启。进度与失败重试通过规范化状态呈现，重复点击及检查/下载并发由共享状态机阻止。启动约五秒与每六小时检查，手动最少一分钟，网络检查 20 秒/下载五分钟超时，不改变日志扫描、第三方凭据、用量或上传范围。
- Windows 使用固定版本 tauri-plugin-updater 2.14.0，启用 requireSignedVersion；固定官网 HTTPS 静态清单与同版 exe 地址，拒绝其他域名、协议、端口、凭据、查询、片段及重定向。插件在内存下载并验签后启动 NSIS /UPDATE /P /R，只有安装器启动成功才退出；签名校验同时绑定应用版本，不能用旧包配新版本号。共享状态/IPC 与 Windows/macOS 适配分离，Mac 明确 unsupported 且未编译/实机验证。
- 新增签名初始化/发布脚本。签名材料位于本机 <本机签名目录>，目录仅当前用户/SYSTEM；私钥由当前用户 DPAPI 保护，不保留明文文件，不写入 Git、官网、命令参数或输出。构建临时放入子进程环境并收尾恢复。公钥可以跟踪，密钥不得重建覆盖；换机恢复限制见 UPDATES.md。
- 测试先增加新模块用例并确认缺模块失败，再实现。桌面 npm test 91 项、完整 cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib 91 项通过/6 默认忽略，cargo fmt --check、TypeScript/Vite 和签名 Release/NSIS 通过；覆盖单任务、节流、检查失败不冒充最新、保留候选重试、进度/溢出、非法下载地址及前端状态。官网 37 项测试及 34 项部署门禁、本地隔离 D1 成本/OAuth 通过；密码学回归包含同大小内容修改并重算 SHA 后仍拒绝、篡改可信注释、错误公钥/版本/大小和外部地址。官网构建也用实际最终包与 CLI 生成签名完成验签。
- 先在忽略目录 .bundle-tmp/app-update-bootstrap-0.1.23-beta.1 导出隔离快照，构建/安装仅本机验收版，不公开分发。旧 0.1.22 程序/卸载器备份在 .bundle-tmp/release-0.1.23-backup。上线前缺少更新清单时实际显示 checkError，仍可使用和重试；上线后手动检查真实返回 available / 0.1.23。实际点击悬浮按钮，由原生插件自行从正式官网下载、验签、启动安装器并自动重启，最终文件版本与 IPC 均为 0.1.23，状态 current；没有用手工覆盖正式 exe 代替更新验收。
- 设置、来源偏好、两个快捷键槽及 WorkBuddy 授权存档 SHA 与验收前一致。参榜存档随正常同步改变，另通过真实 status 确认 loggedIn/enabled/remote.enrollment 均有效且周榜 #[名次已脱敏]；Codex 仍连接，TRAE/WorkBuddy 积分 available。新版提示与当前态均为 300×540、无横纵溢出，截图 .bundle-tmp/update-available-installed.png 与 update-current-installed.png 已目视。验收后退出调试实例、正常启动 <installation-directory>/tokendaily.exe，9223 端口关闭。
- 官网新增构建时生成的 /updates/windows-x86_64.json，更新中英下载/说明/FAQ 和 no-cache 响应头；只有 Windows x64 条目。正式安装包 10,264,802 字节，SHA-256 543E3D3966995BB546F138650E4C1C2AB441A3E991E16555AAD0AF912D4544E5。实际完整下载和清单均 HTTP 200、与本地完全一致；HTML 仅保留 Cloudflare 既有 beacon 注入差异，榜单/账号/认证平台 HTTP 200。
- Worker 发布为 6bc6e216-98b5-4f20-bcea-b66eaf5f1e81，前版本 9fcd8945-a736-4871-a649-80ef1d68405c；上传两个首页、更新清单、exe 与 exe.sig 共五个静态资产。API/D1/Secret/Cron/模拟参与者及免费套餐不变，无数据库迁移、手动真实用量上传或 Git push。旧版本首次需要官网安装一次，本机已完成升级。未实测六小时长等待、下载中断、磁盘不足、断电与管理员拒绝；Mac 未验证，不标同步完成。

## 2026-10-20 设置主窗口与悬浮组件详情入口（CHG-20261020-03）

- 参考用户提供的 TokenDaily-Settings-V2.html 视觉结构：主窗口默认只显示设置，分为数据来源、额度提醒、悬浮窗和通用四类。原有连接、授权、提醒、快捷键、组件管理、开机启动和本地数据操作继续调用当前实现，参考文件中的示例值没有进入产品。主窗口调整为 760×620，托盘“打开主面板”改称“打开设置”。
- 悬浮窗右上角改为打开设置；四个 AI 余额条显示从本机对应安装应用资源提取的 Logo。点击来源卡片、日历、顶部今日 Token/预估费用/排行会打开对应真实详情，长按编辑、整卡拖动、键盘方向排序保持可用。左下角身份入口保留现有参榜页。
- 验证：`npm.cmd run build`、`npm.cmd test`（94 项通过）、`cargo fmt --manifest-path src-tauri/Cargo.toml --check` 与 `cargo test --offline --manifest-path src-tauri/Cargo.toml`（98 项通过、6 项按项目既有设置忽略）；使用既有本机签名材料完成 0.1.24 Windows NSIS 与 updater 签名构建。已原位安装在 `<installation-directory>`；从已安装 WebView 验证设置页 760×620、四个工具 Logo、余额条鼠标点击、键盘打开来源详情/总 Token 历史/排行榜，以及齿轮返回设置；两张实际窗口截图已目视核对。
- 最终源码重新签名构建并再次原位安装，安装器退出码 0；最终已安装版重复通过 WebView 组件点击验收。验收后关闭临时 WebView 调试端口 9223，已正常重启 0.1.24。现有用户设置及来源授权未主动更改。
- 同一工作区另有 0.1.24 积分缓存/刷新改动；没有覆盖其已有修改。本轮未发布、部署或推送。Mac 未编译或实机验证。

## 2026-10-20 0.1.24 设置界面与积分功能合并安装核对

- 积分功能任务完成后核对同一工作区：应用源码最后修改于 12:30，最终签名 NSIS 生成于 13:08，已安装版本为 0.1.24。已安装程序与最终 release 程序大小相同，仅有打包器写入的 3 字节 bundle-type 标记差异；此前原位安装已包含两项功能，不重复安装同一产物。
- 合并工作区重新运行 `npm.cmd test`（94 项通过）、`npm.cmd run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib`（98 项通过、6 项按项目设置忽略）和 `git diff --check`，均通过。已安装设置页与悬浮组件真实点击验收沿用同一最终构建的上一条记录；本轮无新的应用源码修改。
- 工作区仍保留两项任务的未提交修改，未替用户提交、推送或发布。Mac 未编译或实机验证。

## 2026-10-20 本地合并 0.1.24 设置与积分改动

- 按用户要求将设置主窗口、悬浮组件详情入口、四个工具 Logo，以及 WorkBuddy/TRAE 最后成功积分缓存与刷新恢复改动一起提交并快进合入本地 `main`。合并前 `main` 与现有 `origin/main` 引用同为 `e97aa2d`；尝试 `git fetch origin main`，但当前网络无法连接 GitHub，因此远端是否有新提交尚未核实，用户稍后推送前需先拉取核对。
- 合并前已执行 `npm.cmd test`（94 项通过）、`npm.cmd run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`cargo test --offline --locked --manifest-path src-tauri/Cargo.toml --lib`（98 项通过、6 项忽略）和 `git diff --check`。Windows 0.1.24 最终安装态与源码一致，Mac 未验证；本轮不发布或推送。

## 2026-10-20 Mac Claude Code 本机监测（CHG-20261020-MAC-CLAUDE）

- 用户明确选择优先接入 Claude Code 本机 Token 与订阅额度。共用 Claude Code JSONL 采集、去重和北京时间归日规则沿用；Mac 新增从当前系统登录用户的默认 `Claude Code-credentials` 钥匙串条目只读凭据，条目不存在时回退 `.claude/.credentials.json`。钥匙串拒绝/异常时不回退旧文件，不复制凭据到 TokenDaily 存储，不读 Claude Desktop 聊天或浏览器 Cookie。
- 共用额度刷新在请求前与请求后各读取一次凭据；原先额外的请求前重复读取已移除，保持请求期间账户切换核对和失败时旧身份隔离。若 Claude Code 重写钥匙串条目，macOS 仍可能再次弹出系统确认；本次不能保证免弹窗。
- `bash scripts/mac-dev.sh cargo test --locked --offline --features macos-transparent --manifest-path src-tauri/Cargo.toml --lib --quiet`：136 通过、10 项既有真实环境测试忽略。新增 Mac 合成测试覆盖钥匙串优先、拒绝时不回退旧文件、条目缺失回退和完全缺失仍未知。`npm test` 96 项通过，`npm run build` 与 `cargo fmt` 通过。最终 `bash scripts/mac-dev.sh npm run tauri -- build --no-sign --bundles app -- --locked` 通过，生成未签名 18.85 MiB App；未替换 `/Applications` 安装版。
- 本机只安装 Claude Desktop，尚无 Claude Code CLI 登录、默认钥匙串条目或有效 Code 日志；因此真实 Token、Pro/Max 额度接口、系统钥匙串确认与安装态界面仍待 Claude Code 登录后验收。Free 版桌面聊天精确 Token/百分比没有可靠来源，不混入本机汇总。Windows 本轮未复测，未发布或推送。

## 2026-10-20 Claude Desktop 聊天与 Claude Code 来源澄清（CHG-20261020-CLAUDE-SCOPE）

- 用户确认近期登录并产生使用记录的位置是 Claude Desktop 聊天窗口。只读核对本机：运行中的 `/Applications/TokenDaily.app` 二进制仍是上一版，构建版未安装；TokenDaily `providers.json` 的 Claude 来源开关为 `false`；`~/.claude/projects`、默认 Claude Code 钥匙串条目及 Claude Code CLI 均未发现。未读取聊天正文、Cookie 或凭据。
- 官方 Claude 帮助说明 Free 版会话限额约五小时重置且消息容量可变；Pro/Max 等付费方案可在设置的用量页看进度条。没有找到可核验的桌面聊天精确 Token / Free 剩余额度来源，不能将登录或聊天记录换算为 Token 或百分比。
- 为防止“Claude”泛称误导，将共享来源展示名及选择器改为“Claude Code”，详情和设置连接前注明桌面聊天未接入，悬浮日历说明补足桌面聊天排除范围；连接键、采集算法、上传范围和用户现有设置不变。`npm test` 96 通过、`npm run build` 通过，Mac Rust 136 通过/10 项真实环境测试忽略；最终未签名 Mac App 18.85 MiB 构建通过。Windows 本轮未运行安装态验收，Mac 现有安装版未替换。

## 2026-10-20 Mac Claude Desktop 本机额度历史（CHG-20261020-MAC-CLAUDE-DESKTOP）

- 用户希望像 Codex 一样直接读取已有的 Claude Desktop，不安装 Claude Code。只读核对本机 Claude Desktop 2.42304.2 的应用支持目录，发现 `plan-usage-history.json` 的版本 2 真实样本含 `fh`、`sd`；查看已安装 App 自身的字段映射，确认对应 `five_hour`、`seven_day` 利用率。未读取聊天正文、Cookie 或 OAuth 缓存，也没有发起推理或私有网络请求。
- 新增 Mac 专属有界解析和最新样本选择。允许数值 0，缺失不补 0；旧于 10 分钟标旧，超过 24 小时拒绝；最新样本无数值时不回退上一身份，组织 ID 只作摘要，不能证明当前账户仍一致。Mac 在既有 Claude Code 额度查询失败后使用此本地历史；快照无重置时间、`identity_verified=false`，共用通知门禁暂停提醒。Claude Code 本机 Token 日志仍独立，Desktop 聊天精确 Token 未接入。共享 UI 来源改为“Claude”，在详情区分 Code 接口和 Desktop 本机历史，并明确账户未核验。
- 测试先写，最初 Rust 因缺失解析函数编译失败，实现后 `bash scripts/mac-dev.sh cargo test --locked --manifest-path src-tauri/Cargo.toml --lib claude_desktop` 3 项通过；显式运行真实本机样本忽略测试 1 项通过。`npm test` 96 项通过，`npm run build` 通过，Mac Rust 全量 139 项通过/11 项默认忽略；`bash scripts/mac-dev.sh npm run tauri -- build --no-sign --bundles app -- --locked` 通过，未签名 App 18.86 MiB。格式检查初次提示新文件排版，已用 cargo fmt 修正。
- 最终 UI 文案调整后 `npm test` 96 项、`npm run build`、Mac 专项 3 项/真实本机 1 项、`cargo fmt --check` 与再次未签名 App 构建均通过。现有 TokenDaily 的 Claude 开关仍为关闭，只有用户在应用里连接 Claude 后才读取。尚未安装或替换 `/Applications/TokenDaily.app`，未做 Mac 安装态界面与真实账户切换/登出验收；Windows 本轮未构建或实测。Claude Desktop 本机格式非公开稳定接口，版本变化须显示未知。没有发布、远端推送或改动用户现有设置。

## 2026-10-20 Mac Claude Desktop 安装态后续

- 依据用户此前允许的本机临时签名和替换验收范围，将原 `/Applications/TokenDaily.app` 完整备份在忽略目录 `.bundle-tmp/mac-validation/TokenDaily-before-claude-desktop-20261020.app`，另保留移出的旧版；对本轮 18.86 MiB 构建版执行 ad-hoc 签名、严格验签，替换后校验二进制 SHA-256 与构建版一致，启动新进程成功。无开发者证书、无公证、未上传。
- 当前 `providers.json` 仍保留用户原有 `claude_enabled=false`，没有自动连接或改变其他来源授权。已请求用户在应用里连接 Claude 并目视确认 5 小时/7 天快照；在回复之前不把安装态数值显示或钥匙串状态写作已通过。真实 Claude Desktop 文件解析测试通过，精确聊天 Token 仍未知；Windows 仍未验证。

## 2026-10-20 Mac 合并后 Windows 安装态回归（`main` 011293a）

- 先核对主线工作区干净；合并 Mac 后的源码通过 `npm.cmd test`（96 项）、`npm.cmd run build`、`cargo fmt --manifest-path src-tauri/Cargo.toml --check`、`cargo test --locked --manifest-path src-tauri/Cargo.toml --lib`（123 项通过、7 项忽略）和 `cargo check --locked --manifest-path src-tauri/Cargo.toml --all-targets`。首次离线 Rust 测试仅因本机缺少 Mac 专属 `notify` 包缓存未启动；联网解析依赖后完整测试通过。Windows 编译有一条未使用 `Permission::can_submit` 的非阻塞警告。
- 使用既有本机签名材料重建 0.1.24 NSIS 与 `.exe.sig`；安装包 10,857,440 字节，SHA-256 `4CB75BA2886A4840B2FB5C55C306C1AA2519630BBBAB7C465D76E91E8A2D5AEB`。旧 exe/卸载器备份于忽略目录 `.bundle-tmp/pre-mac-merge-win-011293a`；NSIS `/UPDATE /P /R` 退出码 0，原位安装到 `<installation-directory>`。新 exe 为 27,407,360 字节，与构建 exe 仅相差 Tauri NSIS 的 3 字节 bundle-type 标记；设置文件安装前后 SHA-256 一致。
- 实际安装版 WebView 验证 760×620 设置页、300×540 悬浮窗无横向溢出，四个 AI Logo 加载，余额条鼠标/键盘打开来源详情，日历和顶部今日 Token 打开总历史，顶部预估费用/排行打开相应详情，齿轮返回设置。只读 IPC 验证新 `identityVerified`、Codex 来源状态、通知权限、WorkBuddy/TRAE 积分状态及悬浮指针契约；Windows 手动 Codex 来源设置仍明确不支持。额度提醒页新通知控件可见，未发送测试通知。实际截图已目视核对，仅保存在忽略目录。
- 验收结束已关闭临时 WebView 调试端口 9223，并正常重启安装版；应用运行中，用户设置摘要未变。未重测真实拖动、贴边、系统通知实际送达、离线旧值文案、授权失效、长时间驻留或 Mac 实机。未修改登录/授权或用量记录，未发布、部署或推送。

## 2026-10-20 GitHub 仓库首页预览与 README 整理（`win/github-home-preview`）

- 按用户指定先制作 `docs/previews/github-readme.html` 可交互预览：悬浮总览/设置切换，指标、来源卡与日历详情，Windows/macOS 说明切换及预览开关。全部展示数字明确标为示例，不读取真实账户；浏览器实测详情、切换和窄屏无横向溢出。
- 基于该预览制作无真实账户数据的 `docs/images/overview-preview.png`，并重排 README 为产品价值、界面示意、数据口径、平台状态、下载、本地开发与参与说明；移除首页过时的 0.1.23 源码版本陈述，官网版本仍以官网为准。没有改变桌面应用代码或数据契约。
- 开放源码仍需用户决定许可证，并另行完成全仓公开前的敏感信息与文档审查、贡献规则及发布范围确认。粗查未发现已跟踪的常见密钥/数据库扩展名；现有 `docs/FEASIBILITY.md`、`docs/PROGRESS.md` 仍含本机路径，且尚无 `LICENSE`、`CONTRIBUTING.md`、`SECURITY.md`，公开前须逐项处理。本轮没有发布、部署或推送，也不宣称 Mac 正式可下载。
- 验证：浏览器检查桌面及窄屏布局、详情/设置/平台切换、Escape 关闭、示例日历、所有页面图片与锚点；`node --check` 通过内联交互脚本语法检查，README 14 个本地链接均存在，`git diff --check` 通过。GitHub 远端页面本轮不可读取，改稿以仓库当前 README 与本地源码/验收文档为准；未运行桌面应用测试，因为仅改仓库介绍材料。
