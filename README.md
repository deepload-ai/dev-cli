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

### 1. 基础系统与编译构建
提供服务器最基础的网络、系统工具以及 C/C++ 扩展编译环境。

*(注：系统会在安装任何包前自动执行 `apt-get update` 以确保软件源为最新。)*

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **基础网络与系统** | 确保网络探测与文件解压畅通无阻 | `apt-get install curl git wget zip unzip tar netcat-openbsd psmisc` |
| **Build Essential** | 原生扩展编译基础套件 | `apt-get install build-essential gcc make` |
| **CMake & Ninja** | 高级 C/C++ 构建系统 | `apt-get install cmake ninja-build` |
| **SQLite3** | 轻量级本地数据存储 | `apt-get install sqlite3 libsqlite3-dev` |

### 2. 核心 CLI 工具
提供现代化、极速的搜索与分析能力，让 AI 代理能更高效地阅读代码库。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **jq** | 轻量级命令行 JSON 处理器 | `apt-get install jq` |
| **ripgrep (rg)** | 极速正则搜索工具，替代 grep | `apt-get install ripgrep` |
| **AI 分析工具** | 带高亮的查看、极速查找及系统资源查看 | `apt-get install bat fd-find tree btop` |
| **系统诊断工具** | 用于进程与网络问题排查 | `apt-get install lsof strace dnsutils net-tools iproute2` |
| **数据与搜索工具** | YAML 处理与模糊查找 | `add-apt-repository ppa:rmescandon/yq && apt-get install yq fzf` |
| **GitHub CLI (gh)** | 命令行管理 PR 与 Issue | `apt-get install gh` |

### 3. AI 环境依赖
为 AI 代理提供处理多媒体、文档以及运行网页自动化的底层动态库。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **多媒体与文档依赖** | 用于处理图像、PDF 及 OCR | `apt-get install ffmpeg imagemagick poppler-utils tesseract-ocr` |
| **Web 自动化依赖** | 供 Playwright / Puppeteer 在无头模式运行 | `apt-get install xvfb libnss3 libatk1.0-0 libx11-xcb1` |

### 4. 编程语言与运行时
完全放弃 nvm/pyenv，针对 AI 代理优化了**全局安装**或**软链接映射**，彻底解决非交互式 Shell 找不到环境或安装包时报错权限不足的问题。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **Node.js & pnpm** | Node.js 22.x LTS (自动配置 npm 目录至 `~/.npm-global`) | `curl -fsSL https://deb.nodesource.com/setup_22.x \| bash -`<br>`apt-get install nodejs`<br>`mkdir -p ~/.npm-global`<br>`npm config set prefix '~/.npm-global'`<br>`echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc`<br>`source ~/.bashrc`<br>`npm install -g pnpm` |
| **Bun** | 极速 JavaScript 运行时 | `curl -fsSL https://bun.sh/install \| bash` |
| **Python 3** | Python 及虚拟环境 (默认配置 pip 使用 `--user`) | `apt-get install python3 python3-pip python3-venv`<br>`pip config set global.user true`<br>`echo 'export PATH=~/.local/bin:$PATH' >> ~/.bashrc`<br>`source ~/.bashrc` |
| **Rust** | Rust 语言编译器与包管理器 | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| **Go** | Golang 编译器 | `apt-get install golang-go` |
| **Java** | OpenJDK 17 LTS | `apt-get install openjdk-17-jdk openjdk-17-jre` |

### 5. 容器与重型系统
提供标准的容器化运行环境，并自动配置免密码执行权限。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **Docker Engine** | Docker 引擎及 Docker Compose (自动加入 docker 用户组) | `apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin`<br>`usermod -aG docker $USER`<br>`newgrp docker` |

### 6. 移动端 SDK
安卓及 Flutter 跨平台移动端开发工具链。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **Android SDK** | 包含 cmdline-tools 与 adb，自动同意所有 License | `wget https://dl.google.com/android/repository/commandlinetools-linux-*.zip`<br>`mkdir -p ~/android-sdk/cmdline-tools`<br>`unzip cmdline-tools...`<br>`echo 'export ANDROID_HOME=~/android-sdk' >> ~/.bashrc`<br>`yes \| sdkmanager --licenses` |
| **Flutter SDK** | Google 跨平台 UI 框架 | `git clone https://github.com/flutter/flutter.git -b stable ~/flutter`<br>`echo 'export PATH=~/flutter/bin:$PATH' >> ~/.bashrc`<br>`source ~/.bashrc`<br>`flutter precache` |

### 7. 应用级工具
监控及错误追踪等上层开发工具。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **Sentry CLI** | Sentry 错误追踪平台的命令行工具 | `curl -sL https://sentry.io/get-cli/ \| bash` |

### 8. AI 编程代理 (AI Coding Agents)
全自动检查并安装/更新当前最主流的基于终端的 AI 编程助手。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **Claude Code** | Anthropic 官方命令行 AI 代理 | `npm install -g @anthropic-ai/claude-code@latest` |
| **OpenCode** | 开源的高性能命令行 AI 代理 | `curl -fsSL https://opencode.ai/install \| bash` |
| **Codex** | OpenAI 驱动的代码生成 CLI | `npm install -g @openai/codex@latest` |

### 9. 代理技能插件 (AI Agent Skills)
为已安装的 AI 代理注入“通用技能”、“长期记忆”与“知识图谱”，让你的代理表现得像一个资深工程师团队。系统会根据步骤 8 检测到的代理，自动为它们注册适用的插件。

| 工具/环境 | 简介 | 安装方式/命令 |
| :--- | :--- | :--- |
| **everything-claude-code** | 代理性能优化及记忆系统 | `git clone ... && npm install && ./install.sh --profile full` |
| **claude-mem** | Claude 与 OpenCode 的持久化记忆插件 | `npx -y claude-mem install` |
| **openclaw** | CMEM 记忆网关 | `curl -fsSL https://install.cmem.ai/openclaw.sh \| bash` |
| **rtk** | 实时知识注入工具 | `curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh \| sh` |
| **pua** | 增强提示词技能 | `npx -y skills add tanweai/pua --skill pua` |
| **gstack** | 包含多角色的虚拟工程团队技能 | `git clone ... ~/gstack && ./setup` |
| **ui-ux-pro-max-skill** | UI/UX 前端代码增强技能 | `npm install -g uipro-cli && uipro init` |
| **oh-my-claudecode** | 多代理编排工具 | `npm install -g oh-my-claude-sisyphus@latest && omc setup` |
| **graphify** | 代码库知识图谱生成查询工具 (包含 Trae 适配) | `pipx install graphifyy`<br>`graphify install` |

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