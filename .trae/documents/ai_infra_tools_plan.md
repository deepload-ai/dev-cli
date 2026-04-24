# 基础设施扩展计划：AI 环境与工具 (AI Infrastructure Extension Plan)

## 1. 摘要 (Summary)
根据用户的需求，AI Agent 在执行任务时经常需要处理多媒体（如 `ffmpeg`）、文档解析（RAG）、网页自动化（Playwright/Puppeteer）、系统诊断以及数据处理。本计划将这些依赖作为新的组件加入到当前的安装工具中，方便用户按需勾选，完善 AI Agent 的底层基础设施。

## 2. 现状分析 (Current State Analysis)
目前系统主要安装了基础的网络工具（`curl`, `git`, `netcat` 等）以及一些基本的 AI CLI 工具（`bat`, `fd`, `btop`）。但对于 AI 经常涉及的高级任务（如视音频处理、截图与自动化测试、深层系统诊断），缺乏预置的环境依赖，导致在部署相关 Agent（如 Hermes）时频繁报错缺少 `ffmpeg` 等库。

## 3. 提议的变更 (Proposed Changes)

为了保持 `Base` 的轻量化，我们将在 `src/tui.rs` 中新增 4 个专用的基础设施组件，并在 `src/installers/` 目录下提供对应的安装逻辑：

### 3.1. 新增组件列表 (src/tui.rs)
在 `Component` 枚举中新增以下选项，并在 `name()` 和 `all()` 方法中进行映射：
1. **`AIMedia`** (AI Media & Docs): 安装 `ffmpeg`, `imagemagick`, `poppler-utils` (用于 `pdftotext`), `tesseract-ocr`。
2. **`WebAuto`** (Web Automation Deps): 安装 Puppeteer/Playwright 依赖，如 `xvfb`, `libnss3`, `libgbm-dev`, `libasound2`。
3. **`SysDiag`** (System Diagnostics): 安装 `lsof`, `strace`, `dnsutils`, `iproute2`, `net-tools`。
4. **`DataTools`** (Data & Search): 安装 `yq`, `fzf`。

### 3.2. 实现安装逻辑 (src/installers/)
为保持代码整洁，我们可以：
- 在 `src/installers/cli_tools.rs` 中新增 `install_sys_diag()` 和 `install_data_tools()`。
- 创建新文件 `src/installers/ai_env.rs` (并在 `mod.rs` 中暴露)，实现 `install_ai_media()` 和 `install_web_auto()`。
- 使用现有的 `apt::install` 和 `cmd::run_sudo_cmd` 来执行安装。
- 为 `yq` 提供通过 GitHub Release 下载或通过官方 PPA/wget 安装的逻辑（因为 Ubuntu 默认源中的 yq 版本可能较旧或者名称冲突）。

### 3.3. 主程序集成 (src/main.rs)
- 在 `Commands::Install` 的 `match comp` 块中，调用上述新增的安装函数。
- 在 `Commands::List` 的 `match comp` 块中，提供对应命令的版本检测逻辑：
  - `AIMedia` -> 检测 `ffmpeg`
  - `WebAuto` -> 检测 `xvfb-run`
  - `SysDiag` -> 检测 `lsof`
  - `DataTools` -> 检测 `yq`

## 4. 假设与决策 (Assumptions & Decisions)
- **按需安装决策**: 将 `ffmpeg` 等重量级依赖拆分为独立的 `AIMedia` 组件，避免对仅需要纯代码环境的用户造成负担。
- **系统包管理器**: 主要依赖 `apt`。如果遇到像 `yq` 这样在 apt 中表现不佳的包，将使用官方 wget/curl 脚本安装。
- **版本号获取**: 继续复用 `crate::core::version::get_generic_version`。

## 5. 验证步骤 (Verification)
1. 运行 `cargo run -- list`，确认新组件出现在列表中，且状态显示正确。
2. 运行 `cargo run -- install`，在 TUI 界面中可以多选这些新组件并成功完成安装。
3. 检查对应的可执行文件 (`ffmpeg`, `xvfb-run`, `lsof`, `yq`) 是否可用。