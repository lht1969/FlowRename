# AdRename 构建脚本 - Windows
# 使用方式: .\scripts\build.ps1 [dev|build|release]

param(
    [ValidateSet("dev", "build", "release")]
    [string]$Mode = "build"
)

# 获取脚本所在目录
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

Set-Location $ProjectRoot

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  AdRename 构建脚本 (Windows)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

switch ($Mode) {
    "dev" {
        Write-Host "[INFO] 启动开发模式..." -ForegroundColor Yellow
        npm run tauri dev
    }
    "build" {
        Write-Host "[INFO] 执行开发构建..." -ForegroundColor Yellow
        npm run tauri build -- --bundles msi
    }
    "release" {
        Write-Host "[INFO] 执行发布构建..." -ForegroundColor Yellow
        npm run tauri build
    }
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] 构建失败!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[SUCCESS] 构建完成!" -ForegroundColor Green
Write-Host "构建产物位于: $ProjectRoot\src-tauri\target\release\bundle" -ForegroundColor Gray
