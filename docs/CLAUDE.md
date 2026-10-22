# Claude 接入与数据范围

2026-10-20，Windows 0.1.25 已完成 Claude Desktop 本机额度记录适配、Rust 测试和安装态核对；真实客户端用量页数值对照仍未完成。Mac Desktop 来源未实现。

2026-10-21 更新：Mac 上 Claude Desktop 的 Code 标签页会写入 `~/.claude/projects` 日志，真实今日 Token 已与独立脚本核对一致（`live_claude_code_logs`，单个小目录）。这种登录不提供 Claude Code 钥匙串条目/凭据文件，额度改用 Desktop 本机历史（5 小时/7 天、重置时间未知、提醒暂停）。下文“Code projects 为空”为 10-20 当时状态；大历史目录首扫耗时仍未验。重置时间的官方来源只有 Claude Code statusLine 的 `rate_limits.*.resets_at`（Pro/Max），但 Desktop Code 标签页以无界面模式运行，实测很可能不调用 statusLine，见 FEASIBILITY 2026-10-21。终端交互模式已实测提供重置时间；Mac 设置中可选开启状态栏桥接（默认关闭，会修改并在关闭时还原 `~/.claude/settings.json` 的 statusLine），这是“不改状态栏”原则经用户明确批准的例外，见 FEATURE_PARITY CHG-20261021-CLAUDE-STATUSLINE。


## 使用

在设置中分别管理 Codex、Claude 来源，连接开关独立。主窗口默认只显示设置；悬浮窗固定展示已选择的来源卡片，点击 Claude 卡片打开详情。今日 Token 合计只包含已接入的本机记录，不包含 Codex 账号日统计或 Claude 网页／桌面聊天 Token。

2026-10-21 起（CHG-20261021-01），本机有有效的 Claude Code 订阅登录时优先使用其 OAuth 额度，显示接口返回的重置时间，且不混入 Desktop 记录；没有登录或登录已过期时，Windows 回退只读 Claude Desktop 自行写入的 `%APPDATA%\Claude\plan-usage-history.json`，跟随用户现有的 AppData 目录位置或联接；当前电脑的 Claude 用户数据实际位于 D 盘。该文件的最新样本含 5 小时、7 天等窗口的**已使用百分比**，TokenDaily 换算为剩余百分比。要显示重置时间，可在终端运行 `claude` 并执行 `/login` 登录 Pro / Max 订阅账户；TokenDaily 不刷新该凭据，令牌过期后回退 Desktop 记录，直到再次使用 Claude Code。缺少 Code 凭据不代表用户未登录聊天端，也不代表没有使用。TokenDaily 不读取 Desktop 的加密登录凭据或浏览器 Cookie，不自动登录、续期或改共享配置。使用 API Key / 中转服务不等于存在 Anthropic 订阅额度。

官方说明确认同一 Pro / Max 账户下 Code 与 Claude 共享使用限额：[Use Claude Code with your Pro or Max plan](https://support.claude.com/en/articles/11145838-use-claude-code-with-your-pro-or-max-plan)。Desktop 历史文件**没有精确网页／桌面聊天 Token、重置时间或经 TokenDaily 独立验证的当前账户身份**。因此这些数据保持未知，本机记录来源不触发额度通知。Token 合计继续只包含接入的本机日志。

## 刷新与口径

- 本机目录：`CLAUDE_CONFIG_DIR`，否则平台用户目录下 `.claude/projects`。递归包含子代理日志；不跟随目录符号链接。缺失目录、空目录、没有有效 Token 记录或损坏记录均不能显示假零。只有读取过该目录内的有效用量记录且当前日期无使用时，今日显示 0；源日志全部删除后重新显示未知。分项明确命名 Claude Code，不冒充 Claude 聊天。
- 以记录时间转换为北京时间日期；输入、输出、缓存读取、缓存创建相加，不重复加 `cache_creation` 内的子字段。请求 ID + 消息 ID 的哈希去重，同一流式响应采用各计数最大观测值；未完成的半行等待写完。
- 启动、重新连接和切日扫描一次；日内保存内存偏移，只读新增内容；2 秒检查近期文件变化，30 秒发现新日志及补查冷文件。没有持久化原始日志、路径、对话或凭据。大历史目录首次扫描耗时尚未用真实 Claude 日志验证。
- 额度约 5 分钟自动检查，失败退避；手动成功检查至少间隔 10 秒，失败后 60 秒内不重复检查。Desktop 样本以文件内时间为准，超过 10 分钟标旧，超过 24 小时不展示；最新样本空白或账户切换时不沿用旧账户比例。Code OAuth 失败仅在凭据未变时保留旧额度并标旧；认证失效或登录切换清空旧身份结果。仅账户核验通过的 Code 来源可发送额度通知。Claude 最近额度只在内存保存，重启重新读取。
- 合计仅是同日可用来源的合计/已知小计；其中一来源缺失、断开或过期时提示不完整。不把额度百分比相加，不用 Token 换算订阅剩余额度。

## 来源与验证限制

额度适配使用 Anthropic 的账户 OAuth usage/profile 端点，固定 HTTPS 主机、禁止重定向、限制响应大小；凭据只发送给 Anthropic 做身份认证，不调用模型。该端点不是已承诺稳定的公共用量 API，实现线索参考 [CodexBar 的 Claude 来源说明](https://github.com/steipete/CodexBar/blob/main/docs/claude.md)，字段变更或权限限制会显示错误，不估算余额。

Claude Desktop 的历史文件同样是客户端内部格式，并非 Anthropic 对第三方承诺的稳定接口。适配器只接受当前核实的版本 2、已知字段与 0–100 的百分比，文件上限 256 KiB；字段变化时显示未知，不回退到旧账号的历史样本。

macOS 先向系统钥匙串只读查询 `Claude Code-credentials`（账户为当前系统登录名）。只有条目不存在时，才回退 `~/.claude/.credentials.json`；拒绝、损坏或其他钥匙串错误不会用可能过期的文件冒充当前身份。TokenDaily 不枚举其他钥匙串条目、不复制凭据到自己的存储，也不会修改 Claude 登录。系统可能要求用户授权访问，而且 Claude Code 更新其钥匙串条目后可能再次要求确认；用户可只在系统弹窗处理。当前仅覆盖默认服务名，自定义配置目录或 Claude Code 后续更改钥匙串服务名时会显示未知。没有 Claude Code 登录/有效日志的 Mac 无法完成真实数值验收；Claude Desktop Free 聊天依旧不在统计范围。

用户已升级 Pro / Max，主要使用桌面／网页。Windows 0.1.25 的 Rust 126 项测试通过（7 项按既有配置忽略），原位安装版从本机最新 Desktop 样本显示 5 小时剩余 [比例已脱敏]、7 天剩余 [比例已脱敏]，与只读换算一致；两项重置时间仍为未知。尚未与 Claude 客户端用量页人工对照，账号切换和样本过期仅有合成测试；实际 Code 写入节奏与额度通知也待有效来源验证。

2026-10-21 双端整合：两端共用 `claude.rs` 的来源选择与 Desktop 记录解析。优先级为有效 Claude Code 订阅登录（Windows 读凭据文件，macOS 读钥匙串、条目不存在才读文件）→ Claude Desktop 本机记录（Windows `%APPDATA%\\Claude`，macOS `~/Library/Application Support/Claude`，平台层只读字节）。Code 与 Desktop 不合并；仅 macOS 可选的 statusLine 桥接会在使用 Desktop 记录时，把同一组织哈希的状态栏重置时间附加上去，Windows 显示不支持。
