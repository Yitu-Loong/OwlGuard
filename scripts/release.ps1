$ErrorActionPreference = "Stop"

$VERSION = "0.1.0"
$BINARY_NAME = "owlguard-mcp"
$PROJECT_ROOT = Split-Path -Parent $MyInvocation.MyCommand.Path
$DIST_DIR = Join-Path $PROJECT_ROOT "dist"

if (Test-Path $DIST_DIR) {
    Remove-Item $DIST_DIR -Recurse -Force
}
New-Item -ItemType Directory -Path $DIST_DIR | Out-Null

Write-Host "编译 Release 版本..."
Push-Location $PROJECT_ROOT
cargo build --release
Pop-Location

$exePath = Join-Path $PROJECT_ROOT "target\release\$BINARY_NAME.exe"
if (-not (Test-Path $exePath)) {
    Write-Error "编译产物未找到: $exePath"
    exit 1
}

$rulesDir = Join-Path $PROJECT_ROOT "rules"

Write-Host "创建 Windows 发布包..."
$winDir = Join-Path $DIST_DIR "$BINARY_NAME-$VERSION-windows-amd64"
New-Item -ItemType Directory -Path $winDir | Out-Null
Copy-Item $exePath $winDir
Copy-Item $rulesDir "$winDir\rules" -Recurse
Copy-Item (Join-Path $PROJECT_ROOT "README.md") $winDir
Copy-Item (Join-Path $PROJECT_ROOT "LICENSE") $winDir

Compress-Archive -Path "$winDir\*" -DestinationPath (Join-Path $DIST_DIR "$BINARY_NAME-$VERSION-windows-amd64.zip") -Force
Remove-Item $winDir -Recurse -Force

Write-Host "发布包已创建:"
Get-ChildItem $DIST_DIR | ForEach-Object { Write-Host "  $($_.Name) ($([math]::Round($_.Length / 1MB, 2)) MB)" }

Write-Host ""
Write-Host "下一步操作:"
Write-Host "1. 在 GitHub 创建 Release v$VERSION"
Write-Host "2. 上传 dist/ 目录中的压缩包"
Write-Host "3. npm publish --access public (在 npm/ 目录下)"
