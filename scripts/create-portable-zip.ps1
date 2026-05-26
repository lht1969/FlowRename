$exe = "D:\projects\rename_master\target\release\adrename.exe"
$zipDir = "D:\projects\rename_master\target\release\bundle\portable"
$outputZip = Join-Path $zipDir "AdRename_1.0.0_x64_portable.zip"

if (-not (Test-Path $zipDir)) {
    New-Item -ItemType Directory -Path $zipDir -Force | Out-Null
}

$staging = Join-Path $env:TEMP "AdRename_portable_staging"
if (Test-Path $staging) {
    Remove-Item $staging -Recurse -Force
}
New-Item -ItemType Directory -Path $staging -Force | Out-Null

Copy-Item $exe $staging

$readmeContent = @"
AdRename v1.0.0 - Portable Edition
====================================

This is the portable version of AdRename.
No installation required - just run adrename.exe.

System Requirements:
- Windows 10 or later
- WebView2 Runtime (usually pre-installed on Windows 11)

Usage:
1. Double-click adrename.exe to launch
2. Select a directory containing files to rename
3. Configure renaming methods
4. Preview and execute

Undo History:
- Undo history is stored in %APPDATA%\AdRename\undo_history.json
- History persists across application restarts
"@

Set-Content -Path (Join-Path $staging "README.txt") -Value $readmeContent -Encoding UTF8

if (Test-Path $outputZip) {
    Remove-Item $outputZip -Force
}

Compress-Archive -Path (Join-Path $staging "*") -DestinationPath $outputZip

Remove-Item $staging -Recurse -Force

$size = [math]::Round((Get-Item $outputZip).Length / 1MB, 2)
Write-Host "Portable ZIP created: $outputZip ($size MB)"
