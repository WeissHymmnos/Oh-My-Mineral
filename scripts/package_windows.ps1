# Build and package Mineral GUI for Windows x86_64
$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$Dist = Join-Path $Root "dist"
$PkgDir = Join-Path $Dist "Mineral-0.1.0-win64"
$ZipOut = Join-Path $Dist "Mineral-0.1.0-win64.zip"

Set-Location $Root

Write-Host "==> Building Mineral GUI (--release) for Windows..."
cargo build -p mineral-gui-qt --release

if (Test-Path $PkgDir) { Remove-Item -Recurse -Force $PkgDir }
New-Item -ItemType Directory -Force -Path $PkgDir | Out-Null

Copy-Item "target\release\mineral-gui-qt.exe" (Join-Path $PkgDir "mineral-gui-qt.exe")
if (Test-Path "target\release\mineral-gui-bridge.exe") {
    Copy-Item "target\release\mineral-gui-bridge.exe" (Join-Path $PkgDir "mineral-gui-bridge.exe")
}
if (Test-Path "$env:USERPROFILE\.cargo\bin\mineral.exe") {
    Copy-Item "$env:USERPROFILE\.cargo\bin\mineral.exe" (Join-Path $PkgDir "mineral.exe")
}
Copy-Item "assets\logo.ico" (Join-Path $PkgDir "mineral-gui.ico")
Copy-Item "README.md" (Join-Path $PkgDir "README.md")

if (Get-Command windeployqt -ErrorAction SilentlyContinue) {
    Write-Host "==> Bundling Qt 6 runtime DLLs with windeployqt..."
    windeployqt --release (Join-Path $PkgDir "mineral-gui-qt.exe")
}

if (Test-Path $ZipOut) { Remove-Item -Force $ZipOut }
Compress-Archive -Path "$PkgDir\*" -DestinationPath $ZipOut
Write-Host "==> Done! Output: $ZipOut"
