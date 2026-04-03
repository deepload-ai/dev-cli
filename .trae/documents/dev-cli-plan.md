# DevEnv CLI Implementation Plan

## 1. 探索与准备 (Phase 1)
- **目标**: 初始化 Rust 项目，配置所需的依赖包（如 `dialoguer` 用于交互菜单, `clap` 用于参数解析, `tokio` 用于异步任务等）。
- **当前状态**: 项目目录仅包含 `README.md` 和 `LICENSE`，属于全新初始化状态。
- **行动**:
  - 运行 `cargo init` 创建 Rust 项目结构。
  - 在 `Cargo.toml` 中添加所需的依赖库。

## 2. 核心模块划分 (Proposed Changes)
项目将按职责划分为以下几个核心模块：

1. **CLI 框架解析与交互 (cli.rs, tui.rs)**
  - 使用 `clap` 解析命令行参数（支持 `install`, `update`, `uninstall` 等子命令）。
  - 使用 `dialoguer` 实现交互式复选框菜单，供用户进行自定义环境选择，或直接执行“完全自动”一键安装。
2. **权限与环境上下文 (context.rs, sudo.rs)**
  - 检查当前执行用户，避免以 root 直接运行。
  - 提前获取 `sudo` 权限缓存，确保在执行 `apt` 等系统命令时无缝衔接。
3. **安装器与配置器模块 (installers/)**
  - **AptInstaller**: 封装 `apt-get update` 和 `apt-get install -y`，统一处理 `DEBIAN_FRONTEND=noninteractive`。
  - **NodeInstaller**: 导入 NodeSource GPG key，配置 APT 源，全局安装 Node.js，执行 `npm i -g pnpm`。
  - **PythonInstaller**: 通过 APT 全局安装 `python3`, `python3-pip`, `python3-venv`。
  - **RustInstaller**: 下载并执行 `rustup-init.sh`，为了 AI 兼容性，将 `~/.cargo/bin/*` 软链接至 `/usr/local/bin/`。
  - **BunInstaller**: 执行官方安装脚本，将二进制文件链接至 `/usr/local/bin/bun`。
  - **DockerInstaller**: 配置 Docker 官方 APT 源，安装 Docker CE，执行 `sudo usermod -aG docker $USER`。
  - **ToolsInstaller**: 统一处理 `gh`, `jq`, `ripgrep`, `sentry-cli` 的自动化脚本和官方 APT 源配置。
4. **系统守护与自动更新模块 (systemd.rs)**
  - 动态生成并写入 `~/.config/systemd/user/devenv-update.service` 和 `.timer` 单元文件。
  - 执行 `systemctl --user daemon-reload` 和 `systemctl --user enable --now devenv-update.timer` 实现每天的静默自动更新。
5. **卸载与深度清理模块 (uninstaller.rs)**
  - 提供交互式卸载选项。清理相关的 APT 包、移除 `/usr/local/bin/` 中的软链接、停用并删除 systemd timer。
  - 根据用户的选择，决定是否执行 `rm -rf ~/.npm ~/.cargo ~/.bun` 等深度数据清理操作。

## 3. 假设与决策 (Assumptions & Decisions)
- **操作系统假设**: 假设宿主机操作系统为 Ubuntu 20.04 或更新版本（原生支持 `apt` 和 `systemd`）。
- **兼容性决策**: 决定采用全局安装和软链接策略，放弃 `nvm` 等环境管理器。这是为了最大限度地提高对 AI 代理的兼容性（AI 代理经常在非登录非交互式 Shell 中运行，依赖 `/usr/local/bin/` 和 `/usr/bin/`）。
- **静默更新决策**: 静默更新任务将在后台执行，遇到由于网络原因或锁冲突导致的失败时，不会阻塞或打扰用户，依赖下一次定时任务重试。

## 4. 验证与测试步骤 (Verification)
1. 在 Ubuntu 环境中编译并运行 `cargo run -- install`，验证 TUI 菜单是否正常渲染。
2. 模拟普通用户执行，验证 `sudo` 提权是否按预期在开始时提示一次密码，后续不再阻塞。
3. 验证软链接和全局安装是否生效（如运行 `which node` 和 `which cargo` 是否在 `/usr/bin/` 或 `/usr/local/bin/` 中）。
4. 运行 `systemctl --user list-timers` 验证自动更新任务是否成功注册并处于激活状态。
5. 运行 `cargo run -- uninstall` 验证卸载和深度清理逻辑。
