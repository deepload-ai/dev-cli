# AI 编程助手及技能一键安装支持计划

## 1. 简介 (Summary)
为了进一步提升开发环境配置的自动化程度，计划新增对当前主流 AI 编程助手（Claude Code、Codex、OpenCode）的一键安装与配置支持。不仅会自动通过官方推荐方式安装这些 CLI 工具，还会一并安装社区中最受欢迎的增强技能（Skills/Plugins），包括 `everything-claude-code`、`oh-my-claudecode`、`gstack`、`ui-ux-pro-max-skill`，以及适合 OpenCode 的 `oh-my-opencode` 和 `superpower`。

## 2. 当前状态分析 (Current State Analysis)
目前 `dev-cli-1` 已经实现了基础系统工具、核心 CLI 工具（jq, rg 等）以及各种语言运行时（Node.js, Python, Rust 等）的安装，并通过 `tui.rs` 进行了清晰的分层（1~6 梯队）。目前 AI 相关的工具仅有 `bat, fd, tree, btop` 被归类在 `Component::AITools` 中。我们需要引入一个全新的梯队专门用于安装“AI Coding Agents”。

## 3. 提出的更改 (Proposed Changes)

### 3.1 增加新的组件枚举 (`src/tui.rs`)
在 `Component` 枚举中新增：
- `Component::ClaudeCode`
- `Component::Codex`
- `Component::OpenCode`

将它们归类为新的梯队（例如 `7. AI Coding Agents`），并在 `name()` 和 `all()` 方法中进行相应的映射和排序。

### 3.2 创建新的安装器模块 (`src/installers/ai_agents.rs`)
新建 `src/installers/ai_agents.rs` 文件，实现以下三个函数：

1. **`install_claude_code()`**
   - 依赖检查：检查 `npm` 和 `git` 是否已安装（底层依赖 `NodeJs` 和 `Base`）。
   - 核心安装：执行 `npm install -g @anthropic-ai/claude-code`。
   - 技能安装：
     - `npm install -g ecc-universal` (对应 everything-claude-code)
     - `npm install -g oh-my-claude-sisyphus@latest` (对应 oh-my-claudecode)
     - 自动 clone `gstack` 仓库到 `~/.claude/skills/gstack`
     - 自动 clone `ui-ux-pro-max-skill` 仓库到 `~/.claude/skills/ui-ux-pro-max-skill`

2. **`install_codex()`**
   - 依赖检查：检查 `npm` 和 `git` 是否已安装。
   - 核心安装：执行 `npm install -g @openai/codex`。
   - 技能安装：与 Claude Code 共享上述四大必备技能（如果适用，或者将其安装到 Codex 的对应技能目录 `~/.codex/skills/`）。

3. **`install_opencode()`**
   - 核心安装：通过官方推荐的 bash 脚本安装 `curl -fsSL https://opencode.ai/install | bash`。
   - 技能安装：
     - `npm install -g oh-my-opencode@latest`
     - 为 `superpower` 配置插件：自动在 `~/.config/opencode/opencode.json` 中注入 `"plugin": ["superpowers@git+https://github.com/obra/superpowers.git"]`，以便 OpenCode 启动时自动加载超级技能。

### 3.3 更新主程序路由 (`src/main.rs`)
- 在 `main.rs` 中引入 `mod ai_agents;`（并在 `installers/mod.rs` 中导出）。
- 在 `match comp` 路由逻辑中，将 `Component::ClaudeCode`、`Component::Codex`、`Component::OpenCode` 映射到 `installers::ai_agents::*` 中的对应函数。
- 确保在安装这些 AI Agent 时，底层依赖（如 Node.js）如果缺失能有适当的提示或前置处理。

## 4. 假设与决策 (Assumptions & Decisions)
- **API Key 配置**：根据用户要求，参考 `zcf` 开源方案的零配置理念自动完成工具和 workflow(skills) 的拉取与组装，但不涉及 API Key 的设置（由用户后续自行通过 `cc switch` 或环境变量配置）。
- **安装方式确认**：已确认 Codex 采用 `npm install -g @openai/codex`，OpenCode 采用官方 bash 脚本，而 GitHub 上的技能仓库（gstack 等）会直接被 clone 到对应的用户技能目录下。
- **目录结构假设**：假设 Claude Code 的本地技能目录为 `~/.claude/skills`，Codex 的为 `~/.codex/skills`。

## 5. 验证步骤 (Verification steps)
- 运行 `cargo check` 确保无编译错误。
- 运行 CLI，检查 TUI 菜单中是否正确显示了新增的 AI 编程助手选项。
- （可选）在沙盒或本地进行 Dry Run 模式测试，验证所有的 `npm install`、`curl` 和 `git clone` 命令拼接与执行逻辑是否符合预期。