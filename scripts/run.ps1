# FlowRename 执行脚本 - Windows
# 使用方式: .\scripts\run.ps1

param(
    [switch]$Debug
)

# 获取脚本所在目录
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

Set-Location $ProjectRoot

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FlowRename 执行脚本 (Windows)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

if ($Debug) {
    Write-Host "[INFO] 启动调试模式..." -ForegroundColor Yellow
    npm run tauri dev -- --debug
} else {
    Write-Host "[INFO] 启动应用..." -ForegroundColor Yellow
    npm run tauri dev
}
