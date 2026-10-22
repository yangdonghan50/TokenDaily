# WorkBuddy 接入边界

已接入本机 Token 用量；Windows 官方网页登录积分已做历史验收。Mac 在 CHG-20261017-MAC-WB 补齐官方登录与本机钥匙串存储，实际验收结果见 PROGRESS。主面板可选择来源，悬浮窗固定总览；设置中可独立连接和断开，不会退出或注销 WorkBuddy。

## 数据与刷新

- Windows 读取用户目录 `.workbuddy/projects/**/*.jsonl`。根目录可以是用户迁移到 D 盘的目录联接；不扫描旧备份，不跟随内部子目录链接。Mac 独立解析 HOME 下对应目录，尚未实机验证。
- 读取 `message`（assistant）与 `function_call` 的 `providerData.usage.inputTokens/outputTokens`，缺少规范化 usage 时读取 rawUsage 的 prompt_tokens/completion_tokens。输入已含缓存，输出已含推理，合计为两者之和，不重复加缓存、推理或积分。
- 按 sessionId 与记录 uuid/messageId/id 的哈希去重，重复更新取最大计数；不同记录即使 traceId 相同仍分别计入。索引只保留在内存，不持久化会话正文、路径清单或原始日志。
- 按记录的毫秒时间戳换算北京时间日期。历史来源已有有效记录而今天无记录时为 0；从未发现有效记录显示未知。错误保留同日旧数并标旧，跨日不沿用昨日数值。
- 每 2 秒检查近期文件的大小和修改时间，只解析新增完整行；每 30 秒发现新文件和补查冷文件，手动刷新立即补查。首次启动/重新连接/切日重建索引，关闭来源后停止扫描。日志未落盘前不能实时显示。
- 总览包含各本机来源的分项，无法取得的来源使合计标注不完整。统计不区分 WorkBuddy 账户，不代表跨设备或账号账单；积分与 Token 独立。

## 积分授权与查询（2026-10-17）

在设置中连接 WorkBuddy，再点击「登录官方账户查询积分」，在默认浏览器完成官方登录。登录窗口五分钟有效，TokenDaily 自动等待结果；可取消或重新开始。建议选择与客户端相同的账户，界面保留授权账户的哈希标识，本机 Token 仍不区分账户。登录可能授权插件访问能力，并非独立的只读 scope；TokenDaily 实际只调用授权、账户标识与积分读取接口，不执行任务、领取奖励或购买操作。

- 官方流程为 `/v2/plugin/auth/state?platform=workbuddy` → 官方网页登录 → `/v2/plugin/auth/token` → `/v2/plugin/login/account`。已实测请求 state 成功、未登录等待码为 11217；它不是积分查询失败。禁止重定向，只允许 codebuddy.cn/workbuddy.cn 的指定 HTTPS 域名；授权地址、令牌不传给 React、不写日志。
- 使用独立授权，不读取 `workbuddy-desktop.info` 或浏览器 Cookie。只保存 accessToken、服务域名、脱敏账号标识和有效期。Windows 沿用当前用户 DPAPI 加密的 `workbuddy-credits.protected`；Mac 使用 TokenDaily 专属服务 `app.tokendaily.local.workbuddy-credits` 的本机钥匙串条目，明确关闭同步，按应用数据路径哈希及 Debug/Release 隔离，不写凭据文件。不保存密码或原始账号资料。系统拒绝读取和内容损坏显示错误，不冒充无授权；清除只删自身对应条目。临时签名变化后系统可能要求重新允许钥匙串访问。
- 积分接口为 `/billing/meter/get-user-resource-summary`，仅 HTTP 404 回退 `/v2/billing/meter/get-user-resource`；产品代码 `p_tcaca`。默认五分钟查询，自动失败后每分钟重试；手动刷新至少间隔十秒。独立线程，不随两秒日志采集联网；无推理请求。
- 只读取明确的剩余字段，优先 Precise 小数，内部定点六位相加；不以容量减用量猜余额。不完整列表、未知格式、无时区的日期、超出精度或溢出显示错误，不转为零。正常空列表才为 0，过期包排除，重复资源去重、冲突拒绝汇总；详情列出有效积分包。返回超过百项时当前保守拒绝汇总，分页支持待真实数据验证。
- 有效期只取明确时间戳/RFC3339，多个结束时间取较早值；到期后的旧余额标记待更新，不当作重置。网络或服务失败保留同账户上次成功余额并标旧；单次官方授权异常先复核，连续两次异常或本地凭据明确到期才清空旧值并要求重新登录。2026-10-20 起仅保存一份规范化余额、积分包及查询时间，新结果覆盖旧结果；Windows 用 DPAPI 加密，Mac 的非凭据缓存只允许当前用户读取（0600），授权凭据仍仅在钥匙串。重启时只有同一授权令牌且未到本地有效期才显示“上次积分余额”，再向官方更新。不保存原始响应，也不以本机 Token 推算积分。
- 清除积分授权只清理 TokenDaily 自有凭据和缓存；断开 WorkBuddy 同时停用日志与积分；清理失败会报错。代际校验防止取消、断开或重连后的旧响应重新写入授权。

主面板、总览、详情、固定悬浮总览和托盘采用独立积分状态，既有百分比额度与提醒不受影响。积分不触发百分比低额度/恢复提醒；是否参榜始终由用户独立决定。

## 前一轮验证记录与接口限制

此前 Windows 本机 `CodeBuddyExtension/Data/Public/auth/workbuddy-desktop.info` 的 accessToken 为加密对象。仅检查结构，没有输出凭据或解密客户端登录。TokenDaily 运行代码不读取该凭据文件；日志采集器完全不联网，独立的积分授权服务才会请求官方接口。Mac 5.8.3 的独立核验见下节。

公开线索：官方 [套餐与用量](https://www.workbuddy.cn/docs/workbuddy/Usage) 提供积分入口；社区 [WorkBuddy Switch](https://github.com/koi128bit/WorkBuddy-Switch) 的登录与积分代码用于核验官方流程和字段线索，TokenDaily 独立实现更保守的未知、精度和有效期处理。这些是客户端接口线索，不保证稳定；已在本机验证官方授权成功、积分汇总读取、与官网套餐基础/奖励积分核对、重启授权恢复和手动节流。真实汇总为 Packages 分类项，包含 CycleRemainCapacity 和 TotalCount；分类下可以有多个积分包，详情标为“积分汇总（N 项）”，不冒充单个积分包。汇总不返回每包有效期，显示未知，逐包明细尚未接入。

没有可靠积分结果前，界面显示「积分待授权」或具体错误，不显示合成数值。既有全局提醒设置只作用于已接通的百分比额度来源。

## 验证

单元测试覆盖含缓存口径、重复记录/同 trace 的独立记录、半行、截断、无变化不重读、北京零点、未知与错误、非法/溢出计数及旧请求不能覆盖断开后状态。真实只读测试需显式运行：

```powershell
. ./scripts/use-local-dev-env.ps1
cargo test --offline --manifest-path src-tauri/Cargo.toml live_workbuddy_logs -- --ignored --nocapture
```

2026-10-17 Windows 本机初次读取 5 个日志约 25ms，第二次约 0.37ms，未重读正文；今日无日志用量，返回真实的今日 0。仅代表本机当前约 1.31MB 的样本，不能外推海量日志性能。新增真实模型回复的落盘时延尚未验证。Mac 后续编译、官方积分登录与安装态已完成本机验收，本机 Token 日志仍未验证；详见 PROGRESS 与 FEATURE_PARITY。

## Mac 5.8.3 免重复登录可行性核验 · 2026-10-17

用户希望安装后一次点击即可读取积分。本轮只做读取验证，未改变登录状态、解密凭据、调用任务或更换已安装的 TokenDaily。

| 路径 | 实测或静态证据 | 能否支持零授权读取积分 |
| --- | --- | --- |
| 本地客户端发现 | `/Applications/WorkBuddy.app` 为 5.8.3；`GET http://127.0.0.1:18488/workbuddy/probe` 返回 200，仅含 ok/app/version/platform。随包 `LocalProbeServer` 实现也只有该探测路由 | 能发现运行中的客户端，不能取得积分或登录态 |
| 客户端登录文件 | 仅检查 auth 文件字段结构；accessToken、refreshToken 都是 `$wbEncrypted`/envelope 对象，未输出值、读取密钥或解密 | 当前版本不能作为普通明文配置直接复用 |
| 其他本机服务 | 当次 daemon 监听端口中，一个 `/healthz` 返回 200 的 MCP 桥接状态，另一个返回 404；已检查的桥接代码未发现公开积分查询路由 | 未验证到可用的只读积分入口；不把内部服务或调试执行入口当正式集成接口 |
| 客户端积分实现 | 随包账户模块通过已认证的网络客户端调用 `/billing/meter/get-user-resource-summary` | 支持“账户积分通过认证后联网查询”，不是证明本机有可直接读取的余额文件 |
| 官方 Open API | [英文官方文档](https://open.workbuddy.cn/en/docs/openapi) 列出 `GET /openapi/v2/credit`、Bearer token 和 `user.credit.readable`；实际无凭据 GET 返回 HTTP 401 | 有公开只读接口，但需要独立用户授权；本轮未做授权后的响应验证 |

官方 Open API 当前文档要求注册第三方应用，并在换取/刷新 token 时使用 clientSecret，明确禁止将 secret 放入客户端。因此不能直接把 secret 内置到 TokenDaily；是否有适合纯桌面应用的公开客户端/PKCE 流程仍需官方确认，不能为此擅自增加凭据中转服务。该接口返回总容量与已用量，口径迁移还需独立验证，不替换当前剩余字段计算。

结论限于此台 Mac 的 WorkBuddy 5.8.3 和上述已检查路径：尚无已验证、受支持的零授权积分读取方案，不能外推所有版本均不可能。近期体验方向为自动发现、首次官方连接、授权有效期间自动查询；现有独立授权已验证同一构建重启恢复。临时签名变更后的钥匙串确认与 WorkBuddy 网页授权是两件事；稳定发行签名身份、将安全存储读取移出阻塞启动路径及拒绝/失效时的恢复交互仍待实施与验收，不承诺所有系统状态下永不弹窗。
