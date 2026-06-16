# FlowRename ﺗﺗﺛ۷ﺛﺧﺎﺝ - Windows
# ﮌﺗﺽﺣﺓﺛﮌﺛ: .\scripts\build.ps1 [dev|build|release]

param(
    [ValidateSet("dev", "build", "release")]
    [string]$Mode = "build"
)

# ﭨﮦﺫ۰ﺛﺧﺎﺝﺯﻱﺿﻌﺥﺟﺡﺙ
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

Set-Location $ProjectRoot

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  FlowRename ﺗﺗﺛ۷ﺛﺧﺎﺝ (Windows)" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

switch ($Mode) {
    "dev" {
        Write-Host "[INFO] ﺩﮪﭘﺁﺟ۹ﺓ۱ﺥ۲ﮌﺛ..." -ForegroundColor Yellow
        npm run tauri dev
    }
    "build" {
        Write-Host "[INFO] 执行开发构建..." -ForegroundColor Yellow
        npm run tauri build
    }
    "release" {
        Write-Host "[INFO] ﻅﺑﺷﺷﺓ۱ﺎﺙﺗﺗﺛ۷..." -ForegroundColor Yellow
        npm run tauri build
    }
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] ﺗﺗﺛ۷ﮌ۶ﺍﻎ!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "[SUCCESS] ﺗﺗﺛ۷ﺱﻡﺏﺭ!" -ForegroundColor Green
Write-Host "ﺗﺗﺛ۷ﺎﻲﺳﺅﺳﭨﺽﻌ: $ProjectRoot\src-tauri\target\release\bundle" -ForegroundColor Gray
