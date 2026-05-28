# FlowRename 环境安装脚本 - Windows
# 使用方式: .\scripts\install-deps.ps1

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FlowRename 环境安装 (Windows)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Rust 是否已安装
Write-Host "[1/4] 检查 Rust 环境..." -ForegroundColor Yellow
if (Get-Command rustc -ErrorAction SilentlyContinue) {
    $rustVersion = rustc --version
    Write-Host "  [OK] Rust 已安装: $rustVersion" -ForegroundColor Green
} else {
    Write-Host "  [ERROR] Rust 未安装!" -ForegroundColor Red
    Write-Host "  请访问 https://rustup.rs 下载并安装 Rust" -ForegroundColor Gray
    exit 1
}

# 检查 Node.js 是否已安装
Write-Host "[2/4] 检查 Node.js 环境..." -ForegroundColor Yellow
if (Get-Command node -ErrorAction SilentlyContinue) {
    $nodeVersion = node --version
    $npmVersion = npm --version
    Write-Host "  [OK] Node.js 已安装: v$nodeVersion (npm: $npmVersion)" -ForegroundColor Green
} else {
    Write-Host "  [ERROR] Node.js 未安装!" -ForegroundColor Red
    Write-Host "  请访问 https://nodejs.org 下载并安装 Node.js (推荐 LTS 版本)" -ForegroundColor Gray
    exit 1
}

# 安装 Rust 目标平台
Write-Host "[3/4] 检查 Rust 目标平台..." -ForegroundColor Yellow
rustup target list --installed | ForEach-Object {
    if ($_ -eq "x86_64-pc-windows-msvc") {
        Write-Host "  [OK] Windows 目标平台已安装" -ForegroundColor Green
    }
}
rustup target add x86_64-pc-windows-msvc
Write-Host "  [OK] Windows 目标平台配置完成" -ForegroundColor Green

# 安装项目依赖
Write-Host "[4/4] 安装项目依赖..." -ForegroundColor Yellow
npm install

if ($LASTEXITCODE -ne 0) {
    Write-Host "  [ERROR] npm install 失败!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  环境配置完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "接下来可以执行以下命令:" -ForegroundColor Cyan
Write-Host "  开发模式: .\scripts\build.ps1 dev" -ForegroundColor Gray
Write-Host "  编译应用: .\scripts\build.ps1 build" -ForegroundColor Gray
Write-Host "  发布版本: .\scripts\build.ps1 release" -ForegroundColor Gray
