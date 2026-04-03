# DevEnv CLI 需求规格说明书 (SPEC)

## 1. 目标与背景
为 Ubuntu 最新版本设计并开发一款自动化的开发环境安装与配置 CLI 工具。该工具的核心目标是简化新环境的搭建流程，特别优化以兼容各类 AI 编程助手（如 Claude Code、Codex 等），确保 AI 代理在非交互式 Shell 中能够无缝调用相关命令（如 `node`, `python3`, `cargo`）。

## 2. 核心架构设计
- **开发语言**: Rust (编译为独立无依赖的单一二进制文件)。
- **架构支持**: 提供 `x86_64` (amd64) 和 `arm64` (aarch64) 的预编译二进制支持。
- **分发方式**: 提供类似 `curl -sSf https://[domain]/install.sh | sh` 的单行引导脚本，自动检测架构，下载预编译二进制包至 `/usr/local/bin/`。
- **权限隔离**: CLI 由普通用户启动。执行过程中若需系统级权限（如 APT 安装、Docker 守护进程配置），通过内部动态调用 `sudo` 并缓存凭证，避免污染普通用户目录权限。
- **UI/UX交互**: 采用 Rust 的 `dialoguer` 等库提供终端交互式 TUI 菜单。支持：
  - **完全自动**: 一键无脑安装所有预设环境。
  - **自定义选择**: 呈现复选框列表，用户可通过方向键和空格自定义需要安装的模块和工具。

## 3. 环境与工具支持列表
所有工具均默认安装官方最新版本，并确保其可执行文件位于全局 PATH (`/usr/local/bin/` 或 `/usr/bin/`)：
1. **基础系统与网络工具**:
  - `curl`, `git`, `wget`, `gnupg`, `ca-certificates`
  - `unzip`, `zip`, `tar` (文件解压缩工具)
  - `psmisc` (包含 `killall`, `fuser`，用于进程管理)
  - `netcat-openbsd` (轻量级网络与端口探测，AI 调试 API 常用)
2. **编译与构建工具**: 
  - `build-essential` (包含 gcc, make 等)
  - `cmake`, `ninja-build` (C/C++ 及底层依赖构建工具)
  - `sqlite3` (轻量级数据库客户端)
3. **编程语言**:
  - **Node.js**: 放弃 `nvm` 等版本管理器以确保 AI 代理兼容性。通过 NodeSource APT 源全局安装最新的 LTS 或 Current 版本，自带 `npm`，并通过 `npm install -g pnpm` 安装 `pnpm`。
  - **Python**: 通过 Ubuntu 官方源或 Deadsnakes PPA 全局安装 `python3` 及 `python3-pip`, `python3-venv`。
  - **Rust**: 通过官方 `rustup` 安装至用户目录（`~/.cargo/bin`），并将其核心可执行文件（如 `cargo`, `rustc`, `rustup`）软链接至 `/usr/local/bin/` 供全局和 AI 代理识别。
  - **Bun**: 通过官方 `curl | bash` 脚本安装，同样将 `~/.bun/bin/bun` 软链接至 `/usr/local/bin/`。
  - **Go (golang)**: 通过官方压缩包或 PPA 全局安装最新的 Go 语言环境。
4. **部署工具**: 
  - **Docker & Docker Compose**: 通过官方 Docker APT 源安装。自动将当前用户加入 `docker` 用户组，实现免 sudo 运行。
5. **AI 代码分析与 CLI 工具**:
  - **gh (GitHub CLI)**: 通过官方 APT 源安装。
  - **jq**: 通过官方 APT 源安装，处理 JSON 数据。
  - **ripgrep**: 通过官方 APT 源安装 (包名为 `ripgrep`，命令为 `rg`)，高效代码搜索。
  - **fd (fd-find)**: 更快的文件查找工具 (包名为 `fd-find`，命令通常软链为 `fd`)。
  - **bat**: 语法高亮的文件查看工具，帮助 AI 更好地理解代码上下文 (包名为 `bat`，命令通常软链为 `bat`)。
  - **tree**: 目录结构可视化。
  - **btop / htop**: 系统资源监控。
  - **sentry-cli**: 通过官方引导脚本安装至 `/usr/local/bin/`。

## 4. 自动化与守护特性
- **静默自动更新**: 
  - 通过注册 `systemd user timer`（或 system timer），每天后台执行一次检查和更新任务（执行类似 `apt-get update && apt-get upgrade -y`, `rustup update`, `npm update -g`, `bun upgrade` 等操作）。
  - 更新过程完全静默，不弹窗、不打扰用户，日志统一输出到 `systemd journal`，可通过 `journalctl --user -u devenv-update` 追溯。
- **智能配置**:
  - 自动处理各种交互式确认（例如在执行 apt 时自动附加 `-y` 标志并设置 `DEBIAN_FRONTEND=noninteractive`）。

## 5. 卸载与清理机制
- **灵活卸载**: CLI 提供 `uninstall` 子命令。支持卸载单个选定工具，或一键卸载全部工具及 CLI 自身。
- **数据保留选项**: 卸载时，通过交互式菜单询问用户是否保留缓存和全局包数据（如 `~/.npm`, `~/.cargo/registry`, `~/.bun/install/cache`, Docker 镜像等）。支持在保留数据后，未来再次运行清理命令进行彻底的深度清理。
