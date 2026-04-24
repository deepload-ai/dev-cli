<#
.SYNOPSIS
Windows 一键 AI 基础设施环境安装脚本 (AI Infrastructure Setup for Windows)

.DESCRIPTION
此脚本用于在 Windows 环境下静默安装 AI Agent（如 Trae/Claude Code）运行所需的基础工具。
主要通过 winget 进行批量静默安装，解决 UAC 弹窗和环境变量延迟生效的问题。

.NOTES
建议：以管理员身份运行此脚本。
#>

Write-Host "=========================================================" -ForegroundColor Cyan
Write-Host "  DevEnv CLI - Windows AI Infrastructure Setup Script    " -ForegroundColor Cyan
Write-Host "=========================================================" -ForegroundColor Cyan
Write-Host ""

# 检查是否以管理员身份运行
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $isAdmin) {
    Write-Host "⚠️  警告: 您没有以管理员身份运行此脚本。" -ForegroundColor Yellow
    Write-Host "某些工具（如 Git, Node.js 等）可能无法正确安装到系统目录并修改全局 PATH。" -ForegroundColor Yellow
    Write-Host "建议您关闭当前窗口，右键选择『以管理员身份运行』重新执行本脚本。" -ForegroundColor Yellow
    Write-Host "3秒后将继续尝试以普通用户权限安装..." -ForegroundColor Yellow
    Start-Sleep -Seconds 3
}

# 检查 winget 是否可用
if (-not (Get-Command "winget" -ErrorAction SilentlyContinue)) {
    Write-Host "❌ 错误: 系统中未找到 winget 工具。" -ForegroundColor Red
    Write-Host "请前往 Microsoft Store 安装『应用安装程序』(App Installer)。" -ForegroundColor Red
    Exit
}

# 定义要安装的包 (Winget ID 格式)
$packages = @(
    # 1. Base Tools
    @{ Id="Git.Git"; Name="Git" },
    @{ Id="GnuWin32.CoreUtils"; Name="CoreUtils (curl, wget, zip 等)" },
    @{ Id="mcmilk.jq"; Name="jq" },
    @{ Id="BurntSushi.ripgrep.MSVC"; Name="ripgrep (rg)" },
    
    # 2. Languages & Runtimes
    @{ Id="OpenJS.NodeJS.LTS"; Name="Node.js (LTS)" },
    @{ Id="Python.Python.3.12"; Name="Python 3.12" },
    
    # 3. AI Environments & Dependencies
    @{ Id="Gyan.FFmpeg"; Name="FFmpeg" },
    @{ Id="ImageMagick.ImageMagick"; Name="ImageMagick" },
    @{ Id="UB-Mannheim.TesseractOCR"; Name="Tesseract OCR" },
    @{ Id="MikeFarah.yq"; Name="yq" }
)

Write-Host "⏳ 开始安装预设的 AI 基础环境工具..." -ForegroundColor Cyan

foreach ($pkg in $packages) {
    Write-Host "-> 正在安装 $($pkg.Name) ($($pkg.Id))..."
    # 使用 --accept-package-agreements 和 --accept-source-agreements 实现静默、无交互安装
    $args = @(
        "install",
        "--id", $pkg.Id,
        "--exact",
        "--silent",
        "--accept-package-agreements",
        "--accept-source-agreements"
    )
    
    # 捕获输出和错误
    $process = Start-Process -FilePath "winget" -ArgumentList $args -NoNewWindow -Wait -PassThru
    
    if ($process.ExitCode -eq 0 -or $process.ExitCode -eq 2316632065) {
        # 2316632065 是 winget 的特殊退出码，表示包已是最新版本 (Package already installed)
        Write-Host "   ✅ $($pkg.Name) 已成功安装或已是最新版" -ForegroundColor Green
    } else {
        Write-Host "   ❌ $($pkg.Name) 安装失败 (Exit Code: $($process.ExitCode))" -ForegroundColor Red
    }
}

Write-Host ""
Write-Host "🎉 所有可用工具安装处理完毕！" -ForegroundColor Green
Write-Host "⚠️  注意: 许多工具安装后修改了环境变量 (PATH)。" -ForegroundColor Yellow
Write-Host "⚠️  要让这些工具在 Trae 或终端中生效，您必须 **彻底关闭并重新启动** 当前的终端窗口或 IDE！" -ForegroundColor Yellow
Write-Host "=========================================================" -ForegroundColor Cyan
