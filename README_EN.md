<div align="center">

# DevEnv CLI

**One-click setup of the ultimate Ubuntu development environment for your AI coding agents (Claude Code, Cursor, OpenClaw, etc.)**

[![OS](https://img.shields.io/badge/OS-Ubuntu_20.04+-orange.svg)](https://ubuntu.com/)
[![Arch](https://img.shields.io/badge/Arch-x86__64_|_ARM64-blue.svg)](https://en.wikipedia.org/wiki/X86-64)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Built_with-Rust-red.svg)](https://www.rust-lang.org/)

[English](./README_EN.md) · [简体中文](./README.md)

[Quick Start](#-quick-start) · [Supported Environments](#-supported-environments--tools) · [Design Philosophy](#-why-devenv-cli)

</div>

---

## 🤔 Why DevEnv CLI?

AI coding assistants (Agents) are changing how we write code, but getting them to run smoothly on a brand new server is often fraught with obstacles:

❌ **"Command not found"** → AI agents typically run in Non-interactive shells, meaning environment variables in `~/.bashrc` are never loaded, leading to missing commands like `node`, `cargo`, or `adb`.  
❌ **"Execution hangs"** → Many traditional install scripts prompt for passwords or configurations (like `apt-get` timezone selection), permanently blocking the AI agent's execution flow.  
❌ **"Lack of analysis tools"** → Relying solely on `cat` and `find` is too inefficient. AI agents need modern tools like `bat`, `fd`, `ripgrep`, and `jq` to quickly understand codebases.  
❌ **"Permission chaos"** → Some tools require global `sudo` installation, while others must be installed as a regular user (otherwise causing permission pollution), making management extremely tedious.

**DevEnv CLI fills all these pitfalls:**

> "Help me install DevEnv CLI: `curl -fsSL https://raw.githubusercontent.com/deepload-ai/dev-cli/main/install.sh | bash`"
> 
> Then tell your Agent: "Help me run `devenv-cli install --auto`"

In the time it takes to drink a cup of coffee, your Ubuntu server transforms into a super workstation that is **perfectly compatible with AI agents, incredibly rich in toolchains, and automatically updates silently every day**.

---

## 🌟 Core Features

- 🤖 **Optimized for AI Agents**: All binary tools are smartly symlinked to `/usr/local/bin/` or global system paths, ensuring they work out-of-the-box in any Shell environment.
- 🛡️ **Secure Permission Isolation**: The CLI is launched by a regular user, internally managing the lifecycle of `sudo` privilege escalation automatically, avoiding Root environment pollution.
- 🔄 **Idempotent Installation**: Running multiple times will not cause duplicate installations; existing tools are automatically skipped.
- 🕒 **Silent Daemon Updates**: Automatically configures a `systemd user timer` to silently update all components in the background daily (`apt upgrade`, `npm update`, `rustup update`, etc.), never interrupting your workflow.
- 🖥️ **Elegant TUI Interface**: Besides the fully automatic mode, it offers a beautiful terminal checkbox menu powered by Rust's `dialoguer` to customize your environment on demand.

---

## 🚀 Quick Start

### 1. One-Click Download & Install
Execute directly in your Ubuntu terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/deepload-ai/dev-cli/main/install.sh | bash
```

### 2. Start Environment Configuration

**▶ Method 1: Let the AI Agent install fully automatically (Recommended)**
Directly let your AI assistant (or yourself) execute:
```bash
devenv-cli install --auto
```
*The tool will automatically and non-blockingly install all preset optimal environments.*

**▶ Method 2: Manual Custom Interactive Installation**
If you want to pick and choose the components to install yourself:
```bash
devenv-cli install
```
*Use keyboard `↑` `↓` to move, `Space` to check, and `Enter` to confirm.*

---

## 📦 Supported Environments & Tools

Whether you are doing frontend, backend, or mobile cross-platform development, everything is here.

| Category | Included Components | AI Agent Optimization Points |
| :--- | :--- | :--- |
| **Base & Network** | `curl`, `git`, `wget`, `zip`, `unzip`, `tar`, `nc`, `psmisc` | Ensures network probing and file decompression are unimpeded |
| **Compilation & Core** | `build-essential` (gcc/make), `cmake`, `ninja-build`, `sqlite3` | Foundation for compiling native extensions and local data storage |
| **Modern Programming Languages** | `Node.js` (LTS) + `pnpm`<br>`Python3` + `pip` + `venv`<br>`Rust` (`cargo`, `rustup`)<br>`Go` (golang)<br>`Bun` | Abandons nvm/pyenv, adopting **global installation** or **symlink mapping**.<br>To fix `EACCES` errors:<br>1. Auto-configures `npm` global prefix to `~/.npm-global`<br>2. Auto-configures `pip` to use `--user` space isolation by default<br>Completely solves AI permission denied errors when installing global packages. |
| **Mobile & Cross-Platform** | `Java` (OpenJDK 17 LTS)<br>`Android SDK` (cmdline-tools)<br>`Flutter SDK` | Automatically accepts Google Licenses, globally configures `JAVA_HOME` and `ANDROID_HOME`, and maps `adb` and `flutter` globally |
| **Deployment & Containers** | `Docker`, `Docker Compose` | Automatically adds the current user to the `docker` group, enabling **sudo-free execution** of containers |
| **AI Exclusive Analysis Libs** | `bat` (highlighted cat)<br>`fd` (blazing fast find)<br>`ripgrep` (blazing fast regex search)<br>`jq` (JSON parsing)<br>`tree`, `btop`, `gh`, `sentry-cli` | Gives AI agents code scanning and structural understanding capabilities far beyond built-in system tools |

---

## 🛠️ Advanced Usage

**Manually trigger global update**
```bash
devenv-cli update
```
*(Note: Daily background silent updates are configured by default during installation, so manual execution is usually unnecessary.)*

**One-click uninstall & data cleanup**
```bash
devenv-cli uninstall
```
*(Provides interactive options, allowing you to choose whether to completely clean all environments, or keep user data like `~/.npm`, `~/.cargo`, and Docker caches for future use.)*

---

## 💻 Source Compilation & Contributing

This project is developed entirely in Rust and built following strict TDD (Test-Driven Development) processes.

If you want to compile from source:

```bash
# 1. Ensure Rust environment is installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Clone the code and compile
git clone https://github.com/deepload-ai/dev-cli.git
cd dev-cli
cargo build --release

# 3. Run tests & CLI
cargo test
./target/release/devenv-cli install
```

### 🧪 Debug Mode (Dry Run)
Want to know exactly what `sudo apt-get` or `curl` commands the CLI will execute in the background, without it actually modifying your system? Just add the environment variable:

```bash
DEVENV_DRY_RUN=1 devenv-cli install --auto
```
*The CLI will enter dry-run mode, precisely printing all intended underlying Shell commands to the console.*

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=deepload-ai/dev-cli&type=Date)](https://star-history.com/#deepload-ai/dev-cli&Date)

<div align="center">
Made with ❤️ for AI Agents.
</div>