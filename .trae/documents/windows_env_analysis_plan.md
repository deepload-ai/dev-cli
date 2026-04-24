# Windows 下 AI 基础环境安装分析与计划 (Windows AI Environment Analysis Plan)

## 1. 摘要 (Summary)
在 Windows 环境下使用 AI IDE（如 Trae）或者 Agent 执行任务时，**强烈建议提前安装好基础环境和工具**。虽然 AI 具备执行终端命令的能力，但依赖 AI 在运行时自行下载安装环境极易失败。Windows 系统特有的 UAC 权限控制、环境变量刷新机制以及交互式安装包等特性，会严重阻断 AI 的自动化工作流。

## 2. 现状分析 (Current State Analysis)
在 Windows 环境下，依赖 AI（如 Trae）自己去安装基础环境（如 `ffmpeg`, `Node.js`, `Python` 等），会面临以下几个致命痛点：

1. **UAC（用户账户控制）提权拦截**
   大多数底层工具（特别是写入 `C:\Program Files` 的工具）需要管理员权限。AI 在终端执行安装命令时，系统会弹出 UAC 提权确认框。终端会被阻塞，而 AI 无法模拟鼠标点击“是”，导致安装流程死锁。
2. **环境变量无法即时生效**
   Windows 的进程环境继承机制决定了：新安装工具修改了全局或用户 `PATH` 之后，当前正在运行的 Trae IDE 或终端窗口无法立即读取到最新的 `PATH`。AI 刚装完工具，下一条命令去调用它时依然会报“找不到命令”。必须重启终端甚至重启 Trae 才能生效，这会彻底打断 AI 的连续上下文。
3. **缺乏统一的静默安装标准**
   不像 Linux 的 `apt-get install -y`，Windows 的很多软件是 `exe` 或 `msi` 格式。虽然它们可能有 `/S` 或 `/quiet` 静默参数，但标准不一。如果 AI 猜错了静默参数，就会弹出一个 GUI 安装向导，导致终端挂起等待。
4. **包管理器（winget/scoop）的初始化交互**
   即便使用 Windows 自带的 `winget`，初次使用时往往会提示“是否同意源协议 (Y/N)”。AI 如果没有针对性地处理这些交互式标准输入，命令就会永远卡住。

## 3. 提议的变更与建议 (Proposed Changes)
基于以上分析，为了在 Windows 上获得与 Linux 一样流畅的 AI 编码体验，我们不应该让 AI 自己去“摸黑”安装，而是应该**提供提前准备好的自动化配置方案**。

针对我们的 `devenv-cli` 工具，提议如下两个方向供后续实施：

### 方案 A：新增 Windows 专属的一键配置脚本（轻量级）
- **What**: 在项目中增加一个 `scripts/windows_setup.ps1` PowerShell 脚本。
- **Why**: 集中处理权限和环境问题。用户只需右键“以管理员身份运行”一次即可。
- **How**:
  脚本内部使用 `winget` 或 `scoop` 批量静默安装核心依赖。
  ```powershell
  # 示例：通过 winget 自动同意协议并静默安装
  winget install -e --id Gyan.FFmpeg --accept-package-agreements --accept-source-agreements
  winget install -e --id MikeFarah.yq
  ```

### 方案 B：将现有的 Rust CLI 扩展为跨平台工具（深度集成）
- **What**: 修改我们的 `devenv-cli`，使其在 Windows 下也能运行。
- **Why**: 提供统一的跨平台体验（Linux 用 `apt`，Windows 用 `winget`/`scoop`）。
- **How**:
  - 在 `src/installers/` 中新增 `windows.rs`。
  - 使用条件编译 `#[cfg(target_os = "windows")]` 将原有的 `apt` 调用替换为对 `winget` 或 `scoop` 的调用。
  - 处理环境变量的热刷新（通过发送 `HWND_BROADCAST` 消息通知 Windows 刷新环境变量）。

## 4. 假设与决策 (Assumptions & Decisions)
- **核心决策**：无论采取哪种方案，**基础工具的安装行为必须由用户在任务开始前（或者 AI 运行的最初阶段，并伴随一次终端重启）完成**，绝不能穿插在 AI 业务逻辑执行的中间。
- **推荐包管理器**：对于 Windows 开发者环境，推荐优先使用 `Scoop`（默认安装在用户目录，无需 UAC 管理员权限，适合 CLI 工具）或者 `Winget`（Windows 11 自带）。

## 5. 验证步骤 (Verification steps)
如果决定推进上述任何一个 Windows 支持方案，验证步骤应包括：
1. 在纯净的 Windows 沙盒环境中，运行一键安装脚本或跨平台版 CLI。
2. 验证安装全程不弹出任何需要人工点击的 GUI 窗口或 UAC 弹窗（或仅在启动时弹一次）。
3. 验证安装后，新开一个终端能够直接识别 `ffmpeg`、`yq`、`node` 等命令。