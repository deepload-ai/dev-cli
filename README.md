<div align="center">

# DevEnv CLI

**一键为你的 AI 编程代理（Claude Code, Cursor, OpenClaw 等）配置最佳 Ubuntu 开发环境**

[![OS](https://img.shields.io/badge/OS-Ubuntu_20.04+-orange.svg)](https://ubuntu.com/)
[![Arch](https://img.shields.io/badge/Arch-x86__64_|_ARM64-blue.svg)](https://en.wikipedia.org/wiki/X86-64)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Built_with-Rust-red.svg)](https://www.rust-lang.org/)

[English](./README_EN.md) · [简体中文](./README.md)

[快速开始](#-快速上手) · [支持的环境列表](#-支持的环境与工具) · [设计理念](#-为什么需要-devenv-cli)

</div>

---

## 🤔 为什么需要 DevEnv CLI？

AI 编程助手（Agent）正在改变我们写代码的方式，但在全新的服务器上，让 AI 代理真正跑起来却常常遇到阻碍：

❌ **“找不到命令”** → AI 代理通常在非交互式 Shell (Non-interactive shell) 中运行，`~/.bashrc` 中的环境变量根本不会被加载，导致它找不到 `node`, `cargo` 或 `adb`。  
❌ **“执行卡死”** → 很多传统安装脚本会弹窗要求输入密码或选择配置（如 `apt-get` 的时区选择），导致 AI 代理的执行流永久阻塞。  
❌ **“缺乏分析工具”** → 仅靠 `cat` 和 `find` 效率太低，AI 代理需要 `bat`, `fd`, `ripgrep`, `jq` 等现代化工具来快速理解代码库。  
❌ **“权限混乱”** → 有的工具需要 `sudo` 全局安装，有的必须在普通用户下安装（否则会有权限污染），管理极其繁琐。

**DevEnv CLI 把这些坑全部填平了：**

> “帮我安装 DevEnv CLI：`curl -fsSL https://raw.githubusercontent.com/deepload-ai/dev-cli/main/install.sh | bash`”
> 
> 然后告诉你的 Agent：“帮我执行 `devenv-cli install --auto`”

喝杯咖啡的功夫，你的 Ubuntu 服务器就变成了一个**对 AI 代理完美兼容、工具链极其丰富、且每天会自动静默更新**的超级工作站。

---

## ⚠️ 前提条件与适用场景

本项目在 **Ubuntu 24.04 LTS** 环境下经过严格测试验证。

**核心适用场景**：
当你的 AI 编程代理（如 Claude Code, Codex, OpenClaw 等）安装在普通用户的家目录（`~/`）下，且以**非 Root 的普通用户身份**运行时，本工具能发挥最大价值。

**原理解析**：
AI 代理通常运行在受限的非交互式 Shell 中，且没有权限执行 `sudo`。如果它们需要调用 `node`, `python`, `cargo` 或 `adb`，往往会因为找不到环境变量或遇到 `EACCES` 权限拒绝而崩溃。DevEnv CLI 专门针对这一痛点，在安装时预先将所有必需的二进制文件映射到了全局可访问的 `/usr/local/bin`，同时将包管理器的写入权限安全地交还给 `~/` 目录，确保 AI 代理不仅能**无缝调用**任何命令，还能**安全地安装**第三方依赖。

---

## 🌟 核心特性

- 🤖 **专为 AI 代理优化**: 所有的二进制工具均被智能软链接至 `/usr/local/bin/` 或系统全局路径，保证任何 Shell 环境下都能开箱即用。
- 🛡️ **安全的权限隔离**: CLI 由普通用户启动，内部自动管理 `sudo` 提权生命周期，避免污染 Root 环境。
- 🔄 **自动幂等安装**: 多次运行不会重复安装，已存在的工具会自动跳过。
- 🕒 **静默守护更新**: 自动配置 `systemd user timer`，每天后台静默更新所有组件（`apt upgrade`, `npm update`, `rustup update` 等），绝不打扰你的开发流。
- 🖥️ **优雅的 TUI 界面**: 除了全自动模式，还提供基于 Rust `dialoguer` 的精美终端复选框，按需定制你的环境。

---

## 🚀 快速上手

### 1. 一键下载与安装
在你的 Ubuntu 终端中直接执行：

```bash
curl -fsSL https://raw.githubusercontent.com/deepload-ai/dev-cli/main/install.sh | bash
```

### 2. 启动环境配置

**▶ 方式一：让 AI 代理全自动安装（推荐）**
直接让你的 AI 助手（或你自己）执行：
```bash
devenv-cli install --auto
```
*工具会自动无阻塞地安装所有预设的最佳环境。*

**▶ 方式二：人工自定义交互安装**
如果你想自己挑选需要安装的组件：
```bash
devenv-cli install
```
*通过键盘 `↑` `↓` 移动，`Space` 勾选，`Enter` 确认。*

---

## 📦 支持的环境与工具 (安装清单)

无论你是做前端、后端、还是移动端跨平台开发，这里应有尽有。工具会严格按照以下归类和顺序依次进行安装：

| 顺序 | 分类 (Category) | 安装项及简介 | 安装内容 (命令/包名) |
| :--- | :--- | :--- | :--- |
| 1 | **基础系统与网络**<br>(Base system & Net) | **基础工具**: 确保网络探测与文件解压畅通无阻 | `curl`, `git`, `wget`, `zip`, `unzip`, `tar`, `nc`, `psmisc` |
| 1 | **编译构建**<br>(C/C++ build tools) | **编译工具链**: 原生扩展编译与本地数据存储 | `gcc`, `make`, `cmake`, `ninja-build`, `sqlite3` |
| 2 | **核心 CLI 工具**<br>(Core CLI utilities) | **现代化命令行**: 极速搜索、诊断与数据处理 | `jq`, `ripgrep (rg)`, `bat`, `fd`, `tree`, `btop`, `lsof`, `strace`, `dnsutils`, `yq`, `fzf`, `gh` |
| 3 | **AI 环境依赖**<br>(AI Dependencies) | **多媒体与网页自动化**: 供 AI 代理调用的底层依赖 | `ffmpeg`, `imagemagick`, `poppler-utils`, `tesseract-ocr`, `xvfb`, `libnss3` |
| 4 | **编程语言与运行时**<br>(Languages & Runtimes) | **全局优化的开发语言**: 避免 AI 权限受限报错 | **Node.js** (LTS, `npm`, `pnpm`), **Bun**, **Python 3** (`pip`, `venv`), **Rust** (`rustup`, `cargo`), **Go**, **Java** (OpenJDK 17) |
| 5 | **容器与重型系统**<br>(Heavy systems) | **Docker SDK**: 免 sudo 运行容器 | `docker`, `docker-compose` |
| 6 | **移动端 SDK**<br>(Mobile SDKs) | **移动开发工具链**: 自动同意协议及配置环境变量 | `Android SDK` (`cmdline-tools`, `adb`), `Flutter SDK` |
| 7 | **应用级工具**<br>(App level tools) | **监控及其他平台工具** | `sentry-cli` |
| 8 | **AI 编程代理**<br>(AI Coding Agents) | **主流 AI 编程 CLI**: 会自动检查并更新到最新版 | **Claude Code** (`@anthropic-ai/claude-code`)<br>**OpenCode** (`opencode.ai/install`)<br>**Codex** (`@openai/codex`) |
| 9 | **代理技能插件**<br>(AI Agent Skills) | **通用的代理增强插件**: 为上述代理注入技能与记忆 | **everything-claude-code** (`./install.sh`)<br>**claude-mem** (`npx claude-mem install`)<br>**openclaw** (`install.cmem.ai/openclaw.sh`)<br>**rtk** (`rtk-ai/rtk/install.sh`)<br>**pua** (`npx skills add tanweai/pua`)<br>**gstack** (`./setup`)<br>**ui-ux-pro-max-skill** (`uipro init`)<br>**oh-my-claudecode** (`omc setup`)<br>**graphify** (`graphify install`) |

*(注：系统也会自动为当前 IDE Trae 注册 graphify 插件)*

---

## 🛠️ 高级用法

**手动触发全局更新**
```bash
devenv-cli update
```
*(注：工具安装时已默认配置每天的后台静默更新，通常不需要手动执行。)*

**一键卸载与数据清理**
```bash
devenv-cli uninstall
```
*(提供交互式选项，可选择彻底清理所有环境，或者保留 `~/.npm`, `~/.cargo`, Docker 缓存等用户数据以备后用。)*

---

## 💻 源码编译与参与贡献

本项目完全使用 Rust 开发，并遵循严格的 TDD（测试驱动开发）流程构建。

如果你想从源码编译：

```bash
# 1. 确保安装了 Rust 环境
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. 克隆代码并编译
git clone https://github.com/deepload-ai/dev-cli.git
cd dev-cli
cargo build --release

# 3. 运行测试与 CLI
cargo test
./target/release/devenv-cli install
```

### 🧪 调试模式 (Dry Run)
想知道 CLI 在后台到底会执行哪些 `sudo apt-get` 或 `curl` 命令，而不希望它真的修改系统？只需加上环境变量：

```bash
DEVENV_DRY_RUN=1 devenv-cli install --auto
```
*此时 CLI 将进入空跑模式，在控制台精确打印出所有意图执行的底层 Shell 命令。*

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=deepload-ai/dev-cli&type=Date)](https://star-history.com/#deepload-ai/dev-cli&Date)

<div align="center">
Made with ❤️ for AI Agents.
</div>