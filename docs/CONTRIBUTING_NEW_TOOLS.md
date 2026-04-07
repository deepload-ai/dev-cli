# 添加新工具指南 (Contributing New Tools)

为了保持 DevEnv CLI 的高质量和 AI 代理的完美兼容性，所有新工具的添加都必须遵循本指南和 TDD（测试驱动开发）流程。

## 1. 核心设计原则

在添加任何新工具之前，请务必自问以下几个问题：
- **AI 是否需要它？** 这个工具是否能提升 AI 代理在分析代码、编译构建或部署时的效率？
- **是否能静默安装？** 它的安装过程是否可以完全跳过人类的交互输入（如 `yes |` 或 `-y`）？
- **全局可达性**：安装后，AI 代理在非交互式 Shell（没有 `~/.bashrc` 的情况）下，能否直接找到它的可执行文件？
- **权限安全**：如果它是包管理器（如 `npm`, `pip`, `cargo`），它后续安装第三方包时，是否会因为尝试写入系统目录而报 `EACCES` 权限错误？

## 2. 添加新工具的标准 TDD 流程

我们强制要求使用 **Red-Green-Refactor (红-绿-重构)** 的 TDD 流程来添加新特性。

### Step 1: 编写测试用例 (RED)
在 `tests/installers_test.rs` 中，为您要添加的工具编写一个存在性测试：

```rust
#[test]
fn test_new_tool_installer_exists() -> Result<()> {
    // 这在编译时会报错（Red），因为函数还未实现
    let _ = devenv_cli::installers::cli_tools::install_new_tool();
    Ok(())
}
```

### Step 2: 更新组件列表
在 `src/tui.rs` 中，将新工具添加到交互式菜单中：
1. 在 `pub enum Component` 中增加一个枚举值（例如 `NewTool`）。
2. 在 `name(&self)` 方法中为其提供展示名称。
3. 在 `all()` 方法的数组中加入该枚举。

### Step 3: 实现安装逻辑 (GREEN)
在合适的模块（如 `src/installers/cli_tools.rs` 或 `src/installers/lang.rs`）中实现安装函数：

```rust
pub fn install_new_tool() -> Result<()> {
    // 1. 幂等性检查：如果已安装，直接跳过
    if cmd::command_exists("new_tool_bin") {
        println!("🟢 New Tool is already installed. Skipping.");
        return Ok(());
    }

    println!("🛠️ Installing New Tool...");
    
    // 2. 静默安装逻辑
    // 示例：通过 APT 安装
    apt::install(&["new-tool-package"])?;
    
    // 或者通过 Curl 脚本安装到用户目录，并软链接到全局
    // let home = std::env::var("HOME")?;
    // let src = format!("{}/.new_tool/bin/new_tool_bin", home);
    // cmd::run_sudo_cmd("ln", &["-sf", &src, "/usr/local/bin/new_tool_bin"])?;
    
    Ok(())
}
```

### Step 4: 注册到主流程
在 `src/main.rs` 的 `match comp` 路由块中，调用刚刚实现的安装函数：
```rust
tui::Component::NewTool => installers::cli_tools::install_new_tool()?,
```

### Step 5: 更新卸载逻辑
如果新工具产生了大量缓存或系统文件，必须在 `src/uninstaller.rs` 中添加对应的清理逻辑（包括 `apt remove`、软链接删除、缓存目录 `rm -rf` 等）。

### Step 6: 验证与空跑 (REFACTOR)
1. 运行 `cargo test` 确保所有单元测试通过（绿灯）。
2. 运行空跑模式检查生成的命令是否安全、正确：
   ```bash
   DEVENV_DRY_RUN=1 cargo run -- install --auto
   ```
3. 检查输出日志中是否包含了正确的 `sudo` 提权、软链接创建以及幂等性跳过。

## 3. 常见陷阱与避坑指南

- **不要假设环境变量存在**：永远不要依赖 `$PATH` 中除了 `/usr/bin` 和 `/usr/local/bin` 之外的路径。如果工具安装在 `~/.xxx/bin`，**必须**使用 `sudo ln -sf` 将其硬链接或软链接到 `/usr/local/bin/`。
- **不要交互**：使用 `DEBIAN_FRONTEND=noninteractive` 环境变量来包裹所有 `apt-get` 命令。
- **权限降级**：如果安装的是 SDK 或包管理器，确保它的全局包安装路径被配置在 `~/` 目录下（如 `~/.npm-global`, `~/.local`），避免 AI 代理在后续使用时因无权写入 `/usr/lib` 而崩溃。