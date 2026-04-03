# DevEnv CLI

DevEnv CLI 是一款专为 **Ubuntu 20.04+ (x86_64 / ARM64)** 打造的自动化开发环境配置工具。
其核心目标是解决每次初始化新服务器时的繁琐配置，并且**特别针对各类 AI 编程助手（如 Claude Code、Cursor、OpenClaw 等）进行了全局环境变量的兼容性优化**。

## 🌟 核心特性

- **开箱即用**: 仅需一行命令，自动检测系统架构并下载预编译好的二进制文件。
- **AI 代理友好**: 所有安装的语言环境和工具（如 Node.js, Python, Rust, Go 等）均被软链接或配置到 `/usr/local/bin/` 或系统级全局路径中，确保 AI 代理在非交互式 Shell 下能无缝调用。
- **TUI 交互菜单**: 提供漂亮的终端交互界面，您可以自由勾选需要安装的组件。
- **丰富的工具链支持**:
  - 🌐 **基础与网络**: `curl`, `git`, `wget`, `zip`, `unzip`, `nc`, `psmisc`
  - 🛠️ **构建与底层**: `build-essential`, `cmake`, `ninja-build`, `sqlite3`
  - 💻 **编程语言**: `Node.js (pnpm)`, `Python3 (pip, venv)`, `Rust (cargo)`, `Go (golang)`, `Bun`
  - 🐳 **部署工具**: `Docker` & `Docker Compose` (自动配置免 sudo 运行)
  - 🤖 **AI 高效分析工具**: `bat` (高亮 cat), `fd` (极速查找), `ripgrep`, `jq`, `tree`, `btop`, `gh`, `sentry-cli`
- **后台自动更新**: 通过注册 `systemd user timer` 每天在后台静默升级所有已安装组件到最新版本。
- **智能权限隔离**: 工具本身由普通用户运行，仅在必要时动态请求并缓存 `sudo` 权限，不污染 Root 环境变量。

---

## 🚀 安装与使用

### 1. 一键安装 DevEnv CLI
在您的 Ubuntu 服务器终端中执行以下命令（*当前为演示脚本，实际部署后替换为真实的 GitHub Release URL*）：

```bash
curl -fsSL https://raw.githubusercontent.com/your-repo/dev-cli-1/main/install.sh | bash
```

### 2. 启动环境配置

**方式一：自定义交互式安装（推荐）**
```bash
devenv-cli install
```
*运行后会弹出一个复选框菜单，通过 `上下方向键` 移动，`空格键` 勾选，`回车键` 确认。*

**方式二：无脑全自动安装**
```bash
devenv-cli install --auto
```
*自动安装内置的所有推荐工具，期间完全无需人工干预。*

---

## 🔄 其他命令

**手动触发全局更新**
```bash
devenv-cli update
```
*注：工具安装时已默认配置每天的后台静默更新，通常不需要手动执行。*

**一键卸载与清理**
```bash
devenv-cli uninstall
```
*提供交互式选项，可选择是否保留 `~/.npm`, `~/.cargo` 和 Docker 缓存等用户数据。*

---

## 🔧 开发与编译

本项目使用 Rust 开发。如果您想自行修改和编译：

```bash
# 1. 确保安装了 Rust 环境
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 克隆代码并编译
git clone https://github.com/your-repo/dev-cli-1.git
cd dev-cli-1
cargo build --release

# 3. 运行
./target/release/devenv-cli
```

## 🧪 调试模式
如果您想在不修改真实系统的情况下预览 CLI 会执行哪些底层命令，可以开启 `DEVENV_DRY_RUN=1` 环境变量进行空跑测试：

```bash
DEVENV_DRY_RUN=1 devenv-cli install --auto
```