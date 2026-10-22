# TokenDaily 桌面端公开前审查

日期：2026-10-22。已按所有者授权建立 [TokenDaily-Desktop](https://github.com/yangdonghan50/TokenDaily-Desktop) 公开仓库，初始快照不含旧历史，原私库保留；四个厂商 Logo 已替换为普通字母标识，私密漏洞报告已启用并回读确认。首次匿名回验为 275 个文件、一个无父提交的根、唯一 main 与零标签，之后仅追加发布记录。执行与验收详情见 [PROGRESS](PROGRESS.md)。本次没有发布新的安装包或部署服务。

## 授权范围与交付内容

- 桌面端原始源码和文档使用根目录 [LICENSE](../LICENSE) 中完整的 Apache License 2.0；[NOTICE](../NOTICE) 记录项目声明。npm、Cargo 清单标记为 `Apache-2.0`，安装资源包含 LICENSE、NOTICE 与第三方声明。
- 桌面本机监测免费，贡献不要求购买订阅。网页端 AI 工具情报、优惠与更新提醒订阅独立运营；本仓库许可不提供托管服务、订阅内容或第三方品牌的使用权。Apache-2.0 允许他人在遵守许可的前提下使用、修改和商业分发桌面源码，不能据此承诺收费网站拥有排他的同类产品经营权。
- [贡献指南](../CONTRIBUTING.md)、[安全说明](../SECURITY.md)、Issue/PR 模板已补齐；漏洞报告使用新公开仓库的私密入口。
- 采集、额度计算、账号上传、签名信任及提醒规则保持原实现。源码中的个人路径与余额用例改为合成样例，来源图标改用原创普通字母 SVG。示意图仍明确为示例数据。

## 检查范围与方法

审查基线为 `734e9213e2e02bcead08d399839312d1d0bf086a`，工作分支为 `shared/open-source-preparation`。扫描开始时工作区干净，已 fetch 既有 origin。仓库不是浅克隆。

| 检查 | 范围与结果 |
| --- | --- |
| Git 图 | 基线全部本地可达引用共 113 个提交、1,284 个不同 blob；含本地分支、远端跟踪分支、标签及本地工具引用 |
| 历史差异密钥扫描 | Gitleaks 8.30.1，`--all --full-history`；工具报告 103 个有差异的提交，未检出密钥 |
| 历史完整内容扫描 | 将全部 1,284 个可达 blob 导出到忽略目录再扫描，约 47.59 MB；未检出密钥，补充覆盖仅扫描差异可能遗漏的内容 |
| 当前文件密钥扫描 | 复制已跟踪文件及未忽略的新文件后扫描；最终审查材料也纳入复扫，未检出密钥 |
| 补充隐私检查 | 复核个人路径、邮箱、真实账户数值、敏感扩展名、提交作者与图片；发现及处理见下节 |
| 受忽略但已跟踪的文件 | `git ls-files -ci --exclude-standard` 无结果；当前忽略规则没有掩盖已跟踪文件 |
| 仓库状态 | 使用既有认证只读查询 GitHub API，确认 origin 在 2026-10-22 仍为 private；未修改可见性 |

使用官方 [Gitleaks 8.30.1 发布包](https://github.com/gitleaks/gitleaks/releases/tag/v8.30.1)，Windows x64 ZIP 的 SHA-256 已与同次官方下载的校验清单核对：`D29144DEFF3A68AA93CED33DDDF84B7FDC26070ADD4AA0F4513094C8332AFC4E`。没有把源码或疑似凭据提交给在线扫描服务，也没有调用供应商接口验证疑似密钥。

核心命令如下；完整原始报告、原文件备份及导出的历史对象仅保存在本地忽略目录 `.bundle-tmp/open-source-audit/`，不纳入公开材料：

```powershell
gitleaks git --log-opts="--all --full-history" --redact=100 --no-banner --no-color --ignore-gitleaks-allow --report-format=json --report-path=<ignored-report-path> .
gitleaks dir --redact=100 --no-banner --no-color --ignore-gitleaks-allow --report-format=json --report-path=<ignored-report-path> <exported-reachable-blobs>
gitleaks dir --redact=100 --no-banner --no-color --ignore-gitleaks-allow --report-format=json --report-path=<ignored-report-path> <current-tracked-and-new-files>
git ls-files -ci --exclude-standard
git diff --check
```

占位符需要替换为本机忽略目录路径；这不是可直接粘贴执行的路径配置。

## 发现与处理

### 当前文件已脱敏

- 进度、可行性、测试报告、功能对照及部分来源文档中，个人项目/工具/签名目录、个人昵称和真实账户用量/余额/排名替换为占位符。保留测试数量、技术结论、日期和已知限制；没有用合成数值冒充重新实测结果。
- Claude statusLine 测试中的个人 macOS 用户目录改为 `/Users/example`。积分格式化用例改用合成值 `1234.56789 → 1,234.57`；README 示意图余额改为合成值。
- 荣誉图标的来源记录去掉本机下载目录，保留用户提供的资产包名称。
- 补充 `.gitignore`：受保护凭据、Claude 桥接捕获/备份及桌面构建分发物。保留 lockfile、平台配置和应用图标的跟踪；扫描器、报告及工具链仍在已有忽略目录。

### 已复核的非密钥匹配

- 更新器公钥、文件 SHA-256、公共接口地址与客户端标识不是发布私钥或用户授权信息，仍需保留正常校验逻辑。
- TRAE 数据库模块的应用格式常量不是个人账号凭据；它仍属于第三方私有格式兼容实现，格式变化时必须报错或显示未知。保留 TokenTracker 的 MIT 归属及许可原文。
- 当前邮箱匹配来自合成解析样例和拒绝 URL 内嵌凭据的反例；未确认存在真实个人邮箱。提交作者使用 GitHub noreply 地址。
- 历史维护脚本中的 `E:\DevCache`、`E:\DevTools` 为工具迁移默认目录，未作为个人秘密删除；贡献指南明确这些不是开发初始化步骤。

### 原私有历史不能直接公开

旧提交仍包含脱敏前的个人路径、昵称、真实用量/余额/排名，以及已删除的旧界面图。完整历史密钥扫描未检出密钥，**不代表这些隐私信息适合公开**。删除当前文件中的文字不会删除旧提交内容。

原始私有仓库和全部旧提交保留。所有者已授权以审查后的当前文件树建立**没有父提交的公开初始快照**，在新仓库 `yangdonghan50/TokenDaily-Desktop` 中作为起点；现有私库留作内部档案。公开时仅推送这个根提交到新库 main，不迁移旧分支、标签、候选引用、PR 或附件。

发布快照通过 `git commit-tree` 创建单个无父提交，导出独立 bundle/ZIP 并再次扫描。发布提交、文件树、文件数和校验和写入本地忽略目录内的 `publication-manifest.json`；旧候选快照仅作内部审查留档，不公开。远端回验检查初始提交无父提交、文件树一致及唯一 main 分支。未来正常公开提交以这个根提交为祖先，不能合入旧私有历史。

## 第三方许可与资产边界

- 根 LICENSE 使用 [Apache 官方原文](https://www.apache.org/licenses/LICENSE-2.0.txt)，未自行改写条款；11,358 字节，SHA-256 为 `CFC7749B96F63BD31C3C42B5C471BF756814053E847C10F3EB003417BC523D30`。原有 TokenTracker MIT 声明保留。
- npm lockfile 81 项依赖的许可证字段已盘点；Windows Cargo 解析图 310 个包均有 `license` 或 `license_file` 声明。存在 MIT、Apache、BSD、Unicode 及 MPL 等不同条款；不能把这些依赖统称为 Apache-2.0。此处是元数据盘点，不是对全部依赖及二进制分发义务的法律认证。
- 原先从本机客户端提取的四个厂商 Logo 已从发布文件树移除，替换为原创 CX、CL、WB、TR 字母牌；四个 SVG 使用统一中性样式，不复用厂商图形。[第三方声明](../THIRD_PARTY_NOTICES.txt) 与 [素材说明](../assets/providers/README.txt) 已更新。新仓库的初始提交不携带旧 Logo 历史。
- 按所有者本次发布授权保留 TokenDaily 品牌图及用户提供的荣誉图标。Apache-2.0 不授予项目或厂商商标权。

## 验证结果与限制

| 验证 | 本次结果 |
| --- | --- |
| `npm test` | 101/101 通过 |
| `npm run build` | TypeScript / Vite 生产构建通过 |
| `cargo fmt --manifest-path src-tauri/Cargo.toml --check` | 通过 |
| `cargo test --locked --offline --manifest-path src-tauri/Cargo.toml --lib` | 146 通过，10 项真实环境测试忽略 |
| `cargo check --locked --offline --manifest-path src-tauri/Cargo.toml --all-targets` | Windows 通过；有既有未使用代码与增量缓存硬链接回退警告 |
| Windows 未签名 NSIS 构建及许可资源 | `npm run tauri -- build --no-sign --bundles nsis -- --locked --offline` 通过；NSIS 脚本包含三个许可文件，打包暂存资源与原文逐字节一致；未安装，不作正式更新包分发 |
| 本轮 Windows 安装态与 macOS 实机 | 未执行；不宣称本次完成双端交付 |

公开实施补充检查：普通标识替换后再次通过 101 项前端测试和 TypeScript/Vite 构建；真实 ProviderLogo 的合成浏览器预览确认 21/24/32/33px 下 16 张图片均加载、尺寸正确且可读。最终发布导出树约 2.10 MB，Gitleaks 复扫未检出密钥；独立初始提交为 `ca0a0a1d7b3e25b63346b3fd9357c31d4b44b44d`。公开端使用匿名 API 检查文件树、提交关系、分支与标签，并确认 GitHub 识别 Apache-2.0。本轮未改 Rust，也未重复原生安装态验收。

扫描没有发现不等于证明不存在秘密。范围限于本地取得的可达 Git 历史及审查后的当前文件；忽略目录中的运行数据、仓库外凭据、本机配置没有被纳入公开材料。未审查 GitHub 旧 Issue/PR 附件、Actions 日志/制品、远端不可见分支、不可达对象及 reflog 内容。候选快照不包含这些历史材料；未来若直接公开原仓库，需另做完整远端与历史清理，不能沿用本结论。

## 本次公开范围

- 所有者已批准干净快照、新公开仓库、普通来源标识及私密漏洞报告渠道。
- 目标为 `yangdonghan50/TokenDaily-Desktop`，通过 GitHub 官方 API 创建空库并开启漏洞报告，再推送已扫描的独立根提交；不切换原私库可见性。
- 文档链接指向新公开仓库。公开后只读核验仓库可见性、main 文件树、提交父关系、分支/标签范围与漏洞报告设置；结果记录在 PROGRESS。
- 本次没有官网部署、新的二进制发布或签名；旧私库与所有本机原始报告保持私有。

后续桌面开发从公开 main 开始；双机迁移步骤见 [DEVELOPMENT_WORKFLOW](DEVELOPMENT_WORKFLOW.md)。
