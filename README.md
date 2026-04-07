<div align="center">

# DevEnv CLI

**一键为你的 AI 编程代理（Claude Code, Cursor, OpenClaw 等）配置最佳 Ubuntu 开发环境**

[![OS](https://img.shields.io/badge/OS-Ubuntu_20.04+-orange.svg)]()
[![Arch](https://img.shields.io/badge/Arch-x86__64_|_ARM64-blue.svg)]()
[![License](https://img.shields.io/badge/License-MIT-green.svg)]()
[![Rust](https://img.shields.io/badge/Built_with-Rust-red.svg)]()

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

## 📦 支持的环境与工具

无论你是做前端、后端、还是移动端跨平台开发，这里应有尽有。

| 分类 | 包含的组件 | AI 代理优化点 |
| :--- | :--- | :--- |
| **基础与网络** | `curl`, `git`, `wget`, `zip`, `unzip`, `tar`, `nc`, `psmisc` | 确保网络探测与文件解压畅通无阻 |
| **编译与底层** | `build-essential` (gcc/make), `cmake`, `ninja-build`, `sqlite3` | 原生扩展编译与本地数据存储的基础 |
| **现代编程语言** | `Node.js` (LTS) + `pnpm`<br>`Python3` + `pip` + `venv`<br>`Rust` (`cargo`, `rustup`)<br>`Go` (golang)<br>`Bun` | 放弃 nvm/pyenv，采用**全局安装**或**软链接映射**，彻底解决 AI 找不到语言环境的问题 |
| **移动与跨平台** | `Java` (OpenJDK 17 LTS)<br>`Android SDK` (cmdline-tools)<br>`Flutter SDK` | 自动同意 Google Licenses，全局配置 `JAVA_HOME` 和 `ANDROID_HOME`，`adb` 和 `flutter` 映射至全局 |
| **部署与容器** | `Docker`, `Docker Compose` | 自动将当前用户加入 `docker` 用户组，实现**免 sudo 运行**容器 |
| **AI 专属分析库** | `bat` (高亮 cat)<br>`fd` (极速查找)<br>`ripgrep` (极速正则搜索)<br>`jq` (JSON 解析)<br>`tree`, `btop`, `gh`, `sentry-cli` | 让 AI 代理具备远超系统自带工具的代码扫描与结构理解能力 |

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
<div align="center">
Made with ❤️ for AI Agents.
</div>