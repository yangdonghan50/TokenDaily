# 今日等效 API 预估费用

核验日期：2026-10-18。展示于悬浮窗 Token 总量下方，USD；是本机北京时间今日日志换算，不是订阅费用、余额、积分或实际账单。默认采用官方 Standard / 全球路由 Token 单价，不含 Fast/Batch/地区倍率、工具费和税费。

## 已核验价格

单位：美元 / 百万 Tokens；输入与输出分开。输入中的缓存读取/写入仅计一次；输出已含推理，不重复相加。

| 模型 | 普通输入 | 缓存读 | 缓存写 | 输出 |
|---|---:|---:|---:|---:|
| gpt-6.1-sol | 2 | 0.1 | 2.5 | 10 |
| gpt-6-sol | 2 | 0.2 | 2.5 | 10 |
| gpt-6-astra | 10 | 1 | 12.5 | 50 |
| gpt-6-luna | 0.1 | 0.01 | 0.125 | 0.5 |
| gpt-5.6-sol / gpt-5.6 | 4 | 0.4 | 5 | 20 |
| claude-opus-5-5 | 4 | 0.2 | 5（5 分钟）/ 8（1 小时） | 20 |
| claude-sonnet-5-5 | 2 | 0.1 | 2.5 / 4 | 10 |
| claude-opus-5 / 4-8 / 4-7 / 4-6 | 5 | 0.5 | 6.25 / 10 | 25 |

OpenAI 上述模型单次请求输入超过 272,000 时，整次请求输入/缓存费率 ×2、输出 ×1.5，不按每日合计判断长上下文。GPT-5.6 Sol 促销价格至少持续至 2026-11-21；到期前需重新核验。所列 Claude 模型长上下文无额外倍率。

来源：[OpenAI 价格](https://developers.openai.com/api/docs/pricing)、[6.1 Sol](https://developers.openai.com/api/docs/models/gpt-6.1-sol)、[6 Sol](https://developers.openai.com/api/docs/models/gpt-6-sol)、[Astra](https://developers.openai.com/api/docs/models/gpt-6-astra)、[Luna](https://developers.openai.com/api/docs/models/gpt-6-luna)、[5.6 Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol)、[Claude 价格及缓存规则](https://platform.claude.com/docs/en/about-claude/pricing)。未列入本地核验表的型号/别名不模糊匹配、不猜测价格。

## 覆盖与缺失

- Codex：读取 turn_context 的模型与 last_token_usage 的输入、cached_input_tokens、cache_write_input_tokens、输出，沿用原去重键；不使用累计总量算费用。
- 若缓存读缺失，输入全部待估价；读存在但写字段缺失，无法区分普通输入/缓存写入的那部分待估价。可确定的缓存读和输出仍可估价。
- Claude Code：沿用流式消息最大计数及跨文件去重；解析模型、缓存读、5 分钟/1 小时缓存创建。写入时长缺失只排除写入部分；模型不一致或未知不估价。
- WorkBuddy：现有契约未验证可靠模型与缓存明细，本次不套用 Codex/Claude 价格；非零用量计入待估价。真实零用量可以为零费用，缺失来源不是零。
- 全部无法估价显示“待估价”；有可靠部分显示“≈$金额 · 部分”，悬停说明覆盖 Token 数及未读取来源；小额非零显示“<$0.01”，旧数据标“上次”。只代表已覆盖部分，不能作为完整账单或严格账单下界。
- 不调用模型 API，不上传模型名/费用/日志；排行榜上传字段未变。不会自动参榜。
- Codex 本机计数索引升级至 v3，旧 v2 不复用偏移，首轮重新读取日志生成计数与费用。后续改价格表应同步升级索引版本，防止新旧价混算；保留原日志不作修改。

## 排名独立口径

浮窗右上角使用 TokenDaily 现有会员状态接口的本周排名，只在已建立参榜记录时显示；仅创建试用身份不展示，暂停上传仍保留排名。未知名次/待核查明确标记，网络失败不展示旧名次。展开时按需读取，成功缓存五分钟，跨北京时间日期失效；参榜操作通过应用内事件失效缓存，右下角刷新同时重新读取。不会抓取 scys 私有榜单或将其名次当作 TokenDaily 名次。
