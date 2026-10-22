# 应用内更新

2026-10-20 产品决定：市场验证前不购买 Apple Developer Program 会员资格或其他 Mac 正式分发付费服务。Mac 更新器继续保留为本机/隔离验证能力；不开放官网 Mac 更新清单，不承诺无证书版本能为普通用户提供顺畅的一键安装与升级。待有真实用户后再决定是否投入 Developer ID 签名、公证与公开发布；此决定不改变已上线 Windows 更新通道。

统一验收条目：Windows 为 CHG-20261019-07，macOS 为 CHG-20261019-08。Windows 从 0.1.23 起提供悬浮窗右上角更新入口。Mac 已在独立临时应用上完成 0.1.23 → 0.1.24 的下载、验签、原位替换与自动重启；2026-10-20 日常 `/Applications` 安装版已手动替换为本地 ad-hoc 签名的 0.1.24，包含 Mac 检查代码，但官网 Mac 清单仍未开放，检查会报不可用，尚不能从官网一键升级。截至本轮，官网 Mac 更新清单返回 404，也没有 Developer ID Application 证书及公证权限。隔离本地验收不代表正式发布或日常安装版升级验收。

## 用户行为

- 启动约五秒后检查，随后每六小时检查一次。手动检查最少间隔一分钟；不扫描日志、不发起模型请求、不上传身份或用量。
- 未发现新版时按钮可以手动检查。发现新版时按钮突出显示，悬停提供版本及下载安装说明；点击即表示开始下载、验签、原位安装并重启。
- 下载中显示进度并阻止重复操作；下载/签名校验失败时应用继续运行，可以重试。Windows 在安装启动成功后才退出；NSIS 使用 `/UPDATE /P /R`，沿用已有安装目录及应用数据。Mac 在插件确认替换成功后重启；写入受保护的安装位置可能触发系统管理员授权。
- 旧版 0.1.22 及之前没有更新功能，首次需安装官网新版。之后由应用内更新，不会通过网页直接执行本机程序。

## 实现边界

共享 `app_update.rs` 管理状态、节流和单任务门禁，前端 `appUpdate.ts` 只呈现状态。Windows 与 Mac 分别在 `platform/windows/app_update.rs`、`platform/macos/app_update.rs` 使用官方 `tauri-plugin-updater` 2.14；IPC 只允许 main/floating 窗口，不接受来自前端的版本、网址或安装参数。

更新清单固定 `https://tokendaily.net/updates/windows-x86_64.json`；安装包必须是同一官网的 `/downloads/TokenDaily_<版本>_x64-setup.exe`，拒绝其他域名、协议、凭据、端口、查询或片段，HTTP 重定向禁用。网络检查超时 20 秒，下载超时五分钟；使用系统代理。下载进度事件最多约每 250ms 发送，不参与高频采集调度。

`tauri.windows.conf.json` 固定公开校验密钥并启用 `requireSignedVersion`。Tauri 对安装包和签名中的应用版本同时验证；官网构建使用 Node 内置 Ed25519/Blake2b 验证完整 Minisign 格式、签名注释和版本绑定，再生成清单。只改 JSON 版本号或重用旧包签名不能通过发布检查。Windows 代码签名证书与该更新签名是不同机制，本次没有购买代码签名证书。

Mac 配置固定同一公开校验密钥、`requireSignedVersion` 与 `https://tokendaily.net/updates/darwin-{{arch}}.json`：运行中的 Intel 部分请求 `darwin-x86_64.json`（与旧版相同），Apple 芯片部分请求 `darwin-aarch64.json`。下载包只接受同一官网、同版本的 `/downloads/TokenDaily_<版本>_<名称>.app.tar.gz`：Intel 接受 `x64` 或 `universal`，Apple 芯片只接受 `universal`（避免被降级为需 Rosetta 的 Intel 包）；拒绝其他域名、协议、端口、凭据、查询及重定向。更新包必须用与配置公钥匹配的 Tauri 更新私钥签名；这个私钥目前由 Windows 发布机本机保护，Mac 不持有。更新签名验证包来源及版本，不代替 Apple Developer ID 代码签名与公证；后两者需要在正式面向其他用户分发前完成。

## 发布机密钥与构建

首次在专用空目录运行 `scripts/initialize-update-signing.ps1 -SigningDirectory <专用目录>`。目录限当前 Windows 用户与 SYSTEM；私钥生成后立即由当前用户 DPAPI 保护，仅保留 `updater.key.protected` 与公开 `updater.key.pub`，不进入 Git 或官网。已有目录不覆盖，不应为后续版本重新生成密钥。保存密钥及 Windows 用户恢复材料；换电脑或更换密钥需单独规划，不能把本机保护文件当成可跨设备通用密钥。

```powershell
./scripts/build-windows-release.ps1 -SigningDirectory <专用目录>
```

脚本只在子进程环境中传递解密后的私钥，结束恢复环境，不把私钥写入构建日志、命令参数或明文文件。NSIS 产物与 `.exe.sig` 一并生成，签名自动绑定实际版本。普通前端/测试构建不需要密钥；正式 Windows 打包需要签名材料。

发布到既有 TokenDaily-Web：复制已验收的 exe 和 exe.sig 至忽略的 `public/downloads/`，更新 `site.config.mjs` 中的版本、日期、路径、SHA-256、大小和 updaterPublicKey（公钥保持一致），执行 `npm run release:check`、`npm test` 和 `npm run deploy`。网站构建生成清单并设置 no-cache；发布脚本包含签名回归检查。发布后重新下载包、核对摘要，并验证安装版更新及重启。不得公开私钥、原始日志或本机配置，不自动 push Git 或更改付费套餐。

Mac 发布还需在一台受控构建机上生成已 Developer ID 签名、公证的 App。发布构建显式叠加 `src-tauri/tauri.macos.updater.conf.json`，由 Tauri 官方打包器生成 `.app.tar.gz` 与 `.sig`；构建环境必须安全提供与 Mac 配置公钥匹配的更新私钥，不能用本地测试密钥。系统 `tar` 手工打包在本机曾产生 AppleDouble 资源文件，虽通过更新签名验证，却使更新后 App 的代码签名封存失效；不要用它替代官方更新包。网站增加对应的 Mac 通用下载包（`_universal.app.tar.gz`）与 `darwin-x86_64`、`darwin-aarch64` 两个清单（均指向同一通用包）、校验包摘要/Minisign 签名及版本绑定，然后先让当前不支持更新的 Mac 安装版手动升级到首个支持更新的正式版本。之后用两个相邻正式版本在 Intel Mac `/Applications` 安装态验证发现、下载、验签、替换、重启、授权与设置保留；Apple Silicon 另行验收。上述正式发布步骤尚未执行。

仅本地验收使用独立应用标识、测试签名密钥和 `127.0.0.1` 清单；错误签名保持旧版，正确的 Tauri 官方包从 0.1.23 自动升级重启到 0.1.24，更新后严格代码签名校验通过。测试专用的 HTTP 放行及自动触发源码已移除，测试私钥已删除。Desktop 工作区路径曾在 macOS `renamex_np` 阶段停住，`/private/tmp` 路径通过；这不证明 `/Applications` 的权限/管理员授权已通过。详情见 PROGRESS 与 TEST_REPORT。

实际构建/安装验证、安装包摘要及 Worker 版本见 PROGRESS。本机六小时长时间等待、断电和系统权限异常等未实测项须单独标明。

参考：[Tauri 官方 Updater](https://v2.tauri.app/plugin/updater/)。本次也核对了本机固定版本的插件与 CLI 实现。
