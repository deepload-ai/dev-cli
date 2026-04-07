# DevEnv CLI - Technical Design Document (TDD)

## 1. 系统架构概览

DevEnv CLI 采用基于 **Rust** 的模块化架构，核心思想是将用户交互、权限管理、依赖安装和系统守护进程完全解耦。

### 1.1 目录结构
```text
src/
├── main.rs            # 程序入口与子命令路由
├── cli.rs             # clap 命令行参数解析
├── tui.rs             # dialoguer 交互式终端 UI (复选框)
├── systemd.rs         # Systemd User Timer 配置与守护进程管理
├── uninstaller.rs     # 统一卸载逻辑与用户数据清理
├── core/              # 核心底层工具
│   ├── mod.rs
│   ├── cmd.rs         # 封装 std::process::Command，支持 DRY_RUN 和 env
│   └── sudo.rs        # 处理 sudo 提权生命周期
└── installers/        # 各类工具的具体安装器
    ├── mod.rs
    ├── apt.rs         # 封装 apt-get，统一非交互环境变量
    ├── base.rs        # 基础与编译工具 (curl, build-essential)
    ├── cli_tools.rs   # AI 分析工具 (bat, fd, rg, jq, cmake, etc.)
    ├── docker.rs      # Docker 安装与免 sudo 配置
    └── lang.rs        # 编程语言及 SDK (Node, Python, Rust, Java, Android, Flutter)
```

## 2. 核心模块设计

### 2.1 CLI 与 TUI (User Interface)
- 依赖：`clap` (用于命令行参数)，`dialoguer` (用于终端复选框)。
- 工作流：
  1. `main.rs` 解析子命令 (`install`, `update`, `uninstall`)。
  2. 若用户传入 `--auto`，跳过 TUI 渲染，直接全选 `Component::all()` 列表。
  3. 否则，调用 `tui::select_components()` 展示交互式菜单。

### 2.2 Sudo 权限生命周期管理
AI 代理在运行过程中最大的痛点是交互式密码输入阻塞。
- 策略：在 `main.rs` 执行任何安装器之前，优先调用 `core::sudo::ensure_sudo()`。
- 实现：执行 `sudo -v` 探测并缓存提权状态。若失败（或无免密 sudo），直接报错退出，避免程序进入不可逆的半安装状态。
- **DRY_RUN 支持**：若环境变量 `DEVENV_DRY_RUN=1`，所有权限检查会被安全绕过，仅打印意图。

### 2.3 核心命令执行器 (cmd.rs)
对 `std::process::Command` 进行高度封装：
- `run_cmd`: 执行普通命令。
- `run_sudo_cmd`: 前缀加 `sudo` 执行。
- `run_cmd_with_env`: 挂载指定环境变量（如 `DEBIAN_FRONTEND=noninteractive`）。
- **DRY_RUN 拦截**：在实际调用 `.status()` 前拦截，并将拼接好的完整 Shell 字符串输出到控制台。

### 2.4 安装器设计模式 (Installers)
每个安装器必须是**幂等**的，且对 AI 代理高度友好：
1. **存在性检查 (Idempotency)**：使用 `cmd::command_exists("tool")`。若目标二进制已存在，打印跳过日志并返回 `Ok(())`。
2. **静默执行 (Silent Execution)**：所有包管理器必须使用静默标志（`apt -y`, `curl -sSf`, `yes | sdkmanager --licenses`）。
3. **全局路径映射 (Global Accessibility)**：
   - 对于安装在 `~/` 的工具（Rust, Bun, Flutter, Android SDK），强制使用 `sudo ln -sf` 将其入口可执行文件链接到 `/usr/local/bin/`。
4. **权限安全下放 (Permission Downgrade)**：
   - **Node.js**: 运行 `npm config set prefix ~/.npm-global`，避免全局安装时触发 `EACCES`。
   - **Python**: 创建 `~/.config/pip/pip.conf`，写入 `[global] user = true`。

### 2.5 自动更新系统 (systemd.rs)
- 生成 Systemd 单元文件：
  - `~/.config/systemd/user/devenv-update.service`：执行 `devenv-cli update`。
  - `~/.config/systemd/user/devenv-update.timer`：配置 `OnCalendar=daily`。
- 激活：通过 `systemctl --user enable --now devenv-update.timer` 实现无 Root 守护。

## 3. 测试与验证策略
- **测试框架**：利用 Rust 内置的 `#[test]` 模块。
- **TDD (测试驱动开发)**：
  - 在编写新工具的安装器前，先在 `tests/installers_test.rs` 中编写存在性测试。
  - 确保编译通过。
  - 依赖 `DEVENV_DRY_RUN=1` 进行输出断言，确保实际执行的命令符合预期。
