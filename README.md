# Coolapk Desktop｜个人维护分支

<p align="center">
  <strong>基于 Tauri 2、Vue 3 与 Rust 的非官方酷安客户端</strong>
</p>

<p align="center">
  <a href="https://github.com/martian53058/coolapk-desktop/actions/workflows/build.yml"><img src="https://github.com/martian53058/coolapk-desktop/actions/workflows/build.yml/badge.svg?branch=main" alt="构建状态"></a>
  <a href="https://github.com/martian53058/coolapk-desktop/releases"><img src="https://img.shields.io/github/v/release/martian53058/coolapk-desktop" alt="最新发布版本"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
  <img src="https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri" alt="Tauri 2">
</p>

> [!IMPORTANT]
> **这是个人维护的 fork，不是酷安官方客户端。** 本仓库是在既有开源项目基础上继续维护的独立分支，主要用于个人使用、兼容性修复、稳定性改进和构建验证。本仓库的代码改动、版本与发布计划仅代表本仓库。
>
> “酷安 / Coolapk”名称、Logo、商标以及服务端内容归各自权利人所有。本项目与酷安官方及深圳酷安网络科技有限公司不存在隶属、授权或合作关系。

## 项目说明

我会优先处理自己实际使用中遇到的问题，并在必要时调整前端、Tauri/Rust 桌面端及构建流程。每次版本的具体改动以本仓库的提交记录和 Release Notes 为准；不要默认本 fork 与上游保持同步。

- **本仓库：** [martian53058/coolapk-desktop](https://github.com/martian53058/coolapk-desktop)
- **发布下载：** [本仓库 Releases](https://github.com/martian53058/coolapk-desktop/releases)
- **安全报告：** 请先阅读 [SECURITY.md](SECURITY.md)
- **许可证与来源说明：** [LICENSE](LICENSE) · [NOTICE.md](NOTICE.md)

## 下载

请从[本仓库的 Releases 页面](https://github.com/martian53058/coolapk-desktop/releases)下载。每个版本实际提供哪些文件，以对应 Release 页面列出的附件为准。

| 平台 | 可用的构建类型 | 说明 |
| --- | --- | --- |
| Windows x64 | 安装版、单文件便携版 | 适用于常见的 Intel / AMD 64 位 Windows 设备 |
| Windows ARM64 | 安装版、单文件便携版 | 适用于 Windows on ARM |
| macOS | DMG / APP | 提供 Apple 芯片与 Intel 架构构建 |
| Linux x64 | AppImage、DEB、RPM | 以该版本 Release 实际附件为准 |
| iOS ARM64 | 未签名 IPA | 需要自行完成签名后才能安装 |
| Android ARM64 | APK / AAB | Release 签名构建依赖仓库的 Android 签名 Secrets；未配置时可能不发布 |

Windows 便携版适合不想执行传统安装流程的场景。首次运行前，请确认系统满足对应平台的 Tauri/WebView 运行要求。对于未签名的 iOS IPA，请勿将其误认为可直接安装或可直接提交 App Store 的正式发行包。

## 功能概览

当前代码包含以下主要能力，具体可用性可能随酷安服务端接口、账号权限和客户端版本变化：

- 浏览首页信息流、分类频道、动态详情与评论。
- 搜索动态、用户、话题、应用及游戏等内容。
- 使用点赞、收藏、关注等互动功能。
- 查看浏览记录、管理收藏，并使用主题和布局相关设置。
- 支持账号登录及多账号切换；部分场景可通过手动导入 Cookie 登录。
- 在客户端内使用私信等互动功能。
- 使用 Windows、macOS、Linux 桌面端，并保留 Android / iOS 移动端构建路径。

部分能力依赖酷安服务端，接口、登录流程或风控策略变化可能导致功能暂时不可用。本 fork 不保证能够绕过服务端限制，也不承诺所有平台的每个功能完全一致。

## 隐私与凭据

- 登录 Cookie、Token、私信和诊断信息都应视为敏感数据。
- 不要将真实凭据、个人私信、完整日志或账户数据提交到公开仓库。
- 分享诊断文件前，请先检查文件内容并确认接收方可信；任何可访问的分享链接都可能暴露其中的信息。
- 本客户端会与酷安相关服务通信。实际网络请求和本地数据行为以当前版本代码为准；如需确认，请检查相应版本的源代码，而不要只依赖本说明。

如怀疑凭据泄露，请立即在相关服务侧撤销或更新凭据，并避免继续传播原始日志。

## 本地开发

### 环境要求

- Node.js 22 或更高版本
- Rust stable 工具链
- 对应操作系统所需的 [Tauri 2 构建依赖](https://v2.tauri.app/start/prerequisites/)
- Android 构建需要额外的 JDK、Android SDK 与 NDK；iOS 构建需要 macOS、Xcode 和 CocoaPods

### 获取代码并运行

~~~bash
git clone https://github.com/martian53058/coolapk-desktop.git
cd coolapk-desktop
npm ci
npm run tauri -- dev
~~~

### 常用检查

~~~bash
npm test
npm run typecheck
npm run build

cd src-tauri
cargo check
cargo test --lib
~~~

### 本地构建示例

~~~bash
# Windows：NSIS 安装包
npm run tauri build -- --bundles nsis

# Windows：单文件便携版（在 PowerShell 中执行）
./scripts/prepare-windows-portable.ps1 -Architecture x64

# Linux：系统安装包和 AppImage
npm run tauri build -- --bundles deb,rpm,appimage

# macOS
npm run tauri build -- --bundles app,dmg
~~~

Android 和 iOS 的完整构建步骤会受 SDK、签名配置与操作系统限制影响。可从仓库中的构建脚本和 [build.yml](.github/workflows/build.yml) 查看当前实现；请勿把签名文件、密码或 Token 写进源码、提交记录或公开日志。

## 自动构建与发布

本仓库使用 GitHub Actions 运行前端测试、类型检查、构建检查以及 Windows / Linux / macOS Rust 检查。正式版本的跨平台打包流程由 [build.yml](.github/workflows/build.yml) 管理；Android 正式包是否生成取决于所需签名 Secrets 是否配置，iOS 构建产物为未签名 IPA。

如果你是本仓库维护者，发布前应确认版本号、构建结果、Release 附件及更新说明彼此一致。

## 许可证与上游来源

本仓库沿用 **MIT License**。请阅读 [LICENSE](LICENSE) 了解许可文本，并查看 [NOTICE.md](NOTICE.md) 了解代码来源和第三方素材说明。原有版权声明与许可条款继续保留；本 fork 的新增贡献仅在实际适用范围内单独标注。

## 安全问题与反馈

如果发现可能影响本客户端账户、隐私、设备或更新完整性的安全问题，请先阅读 [SECURITY.md](SECURITY.md)，不要公开发布漏洞细节、可利用样本或真实账号数据。

一般功能异常请附上版本、操作系统、复现步骤和已脱敏的必要日志。请勿提交 Cookie、Token、授权码、私信内容、个人文件或其他敏感信息。个人维护者无法承诺固定响应时间，也不提供漏洞赏金计划。

## 代码来源

本仓库基于以下开源项目进行维护：

**[原项目代码仓库](https://github.com/daimiaopeng/coolapk-desktop)**

此链接仅用于说明代码来源与许可关系，不代表推广、背书或官方关联。上游原有版权声明及 MIT 许可条款已保留，详见 [LICENSE](LICENSE) 和 [NOTICE.md](NOTICE.md)。