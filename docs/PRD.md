# DevEnv CLI - Product Requirements Document (PRD)

## 1. 项目背景与目标

随着 AI 编程助手（如 Claude Code, Cursor, OpenClaw 等）的普及，开发者越来越多地让 AI 代理在远程服务器上自主执行构建、分析和部署任务。
然而，在初始化一台全新的 Ubuntu 服务器时，开发者常常面临以下痛点：
- **环境隔离导致命令丢失**：AI 代理通常运行在受限的、非交互式 Shell (Non-interactive shell) 中，不会加载 `~/.bashrc`，导致找不到 `node`, `cargo`, `adb` 等通过常规方法安装在用户目录的命令。
- **全局安装权限冲突 (EACCES)**：如果 AI 代理直接执行 `npm install -g` 或 `pip install`，由于缺少 `sudo` 权限，会导致安装失败。
- **交互阻塞**：传统的 `apt-get` 或 `curl | sh` 安装脚本经常弹出时区选择或 License 确认等交互界面，导致 AI 代理的执行流永久阻塞。
- **分析工具匮乏**：Ubuntu 自带的 `cat`、`find` 在处理大型代码库时效率低下，AI 代理需要更现代化的工具（如 `ripgrep`, `bat`, `fd`, `jq`）来快速理解上下文。

**DevEnv CLI** 旨在解决这些痛点。它是一个由 Rust 编写的、无需依赖的独立二进制 CLI 工具，只需一键运行，即可将全新的 Ubuntu 服务器配置为一个对 AI 代理完美兼容、工具链丰富的“超级工作站”。

## 2. 目标用户与适用场景

- **目标用户**：使用 AI 编程代理的开发者、DevOps 工程师、以及需要快速初始化标准化开发环境的团队。
- **适用场景**：
  - 操作系统：**Ubuntu 20.04+ (重点优化并测试通过 Ubuntu 24.04 LTS)**。
  - 用户角色：AI 工具安装在普通用户目录 (`~/`) 下，且以**非 Root 的普通用户身份**运行。

## 3. 核心功能需求

### 3.1 自动化与一键式安装
- 提供一个引导脚本（如 `install.sh`），能根据当前系统架构（x86_64 或 ARM64）自动从 GitHub Releases 下载对应的预编译二进制文件。
- CLI 支持 `--auto` 参数，允许 AI 代理无需任何人工干预，一键无脑安装所有推荐工具。

### 3.2 智能交互式 TUI
- 对于人类开发者，CLI 需提供基于终端的图形用户界面（TUI），展示结构化的复选框菜单。
- 用户可以通过上下方向键移动，空格键选中/取消选中，回车键确认安装列表。

### 3.3 AI 兼容性优化 (核心竞争力)
- **命令全局化**：所有的开发工具、SDK 和二进制文件，必须被映射或安装到全局路径（如 `/usr/local/bin/`），确保任何非交互式 Shell 都能直接调用。
- **安全权限下放**：
  - **Node.js**: 配置 `npm` 全局安装路径至 `~/.npm-global`，并将其加入全局 PATH，避免全局 `npm install` 时报 `EACCES` 错误。
  - **Python**: 自动配置 `pip.conf` 设置 `user = true`，使 `pip install` 默认在用户态运行，并配置 `~/.local/bin`。
- **无交互执行**：所有的安装过程（如 `apt-get`, SDK 许可接受）必须带有静默标志（如 `-y`, `DEBIAN_FRONTEND=noninteractive`, `yes | ...`）。

### 3.4 幂等性与增量执行
- 每次安装前需检查目标工具是否已经存在。如果已存在，则直接跳过，不会产生重复操作或破坏现有配置。

### 3.5 自动静默更新守护
- CLI 需具备注册后台定时任务的能力（如 `systemd user timer`）。
- 每天自动在后台静默执行一次 `devenv-cli update`，更新 `apt` 软件包、`npm` 全局包、`rustup`、`bun` 等工具，且不干扰前台操作。

### 3.6 一键卸载与清理
- 支持交互式的卸载流程。
- 提供“保留用户数据”选项，让用户选择是否在卸载工具链时保留 `~/.npm`, `~/.cargo` 和 Docker 镜像等大体积缓存。

## 4. 工具链矩阵需求

系统必须原生支持安装以下分类的工具：

1. **基础系统与网络**：`curl`, `git`, `wget`, `zip`, `unzip`, `tar`, `nc` (netcat), `psmisc` (killall, fuser)。
2. **编译与底层**：`build-essential`, `cmake`, `ninja-build`, `sqlite3`。
3. **现代编程语言**：
   - Node.js (LTS) + pnpm
   - Python3 + pip + venv
   - Rust (rustup, cargo)
   - Go (golang)
   - Bun
4. **移动与跨平台 SDK**：
   - Java (OpenJDK 17 LTS)
   - Android SDK (cmdline-tools)
   - Flutter SDK
5. **容器部署**：Docker CE & Docker Compose。
6. **AI 分析工具**：`bat`, `fd-find`, `ripgrep`, `jq`, `tree`, `btop`, `gh` (GitHub CLI), `sentry-cli`。

## 5. 非功能性需求
- **语言选择**：Rust。因其能编译为单一的二进制文件，无需依赖 Python 或 Node.js 运行时，完美解决“先有鸡还是先有蛋”的问题。
- **性能**：安装过程中的文件下载应尽量使用系统官方源或稳定的 CDN 加速链接。
- **文档**：提供中英文双语的 `README.md`，并在其中详细说明 AI 代理的权限解决原理。
