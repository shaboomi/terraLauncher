<#
.SYNOPSIS
  Build the terraLauncher MSI from an already built terralauncher.exe.

.DESCRIPTION
  Stages terralauncher.exe with its licence files and runs WiX v5 on terralauncher.wxs, writing
  terralauncher-<version>-windows-x64.msi to -OutDir. The version comes from Cargo.toml.

  Needs WiX v5 and its UI and Util extensions:
    dotnet tool install --global wix --version 5.0.2
    wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2

.EXAMPLE
  cargo build --release -p artcraft-launcher
  pwsh packaging/windows/build-msi.ps1
#>
param(
  [string] $Exe,
  [string] $OutDir,
  [string] $Wix = 'wix'
)
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
if (-not $Exe) { $Exe = Join-Path $Root 'target\release\terralauncher.exe' }
if (-not $OutDir) { $OutDir = Join-Path $Root 'dist' }

$Version = $null
$inPkg = $false
foreach ($line in Get-Content (Join-Path $Root 'Cargo.toml')) {
  if ($line -match '^\s*\[') { $inPkg = ($line.Trim() -eq '[workspace.package]'); continue }
  if ($inPkg -and $line -match '^\s*version\s*=\s*"([^"]+)"') { $Version = $Matches[1]; break }
}
if (-not $Version) { throw 'could not read [workspace.package] version from Cargo.toml' }
# MSI versions are numeric (major.minor.build); a pre-release tag is dropped there.
$MsiVersion = ($Version -split '-')[0]

if (-not (Test-Path -LiteralPath $Exe)) { throw "no launcher build at $Exe" }
$Stage = Join-Path ([IO.Path]::GetTempPath()) "terralauncher-msi-$PID"
Remove-Item -Recurse -Force $Stage -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $Stage, $OutDir | Out-Null
try {
  Copy-Item -LiteralPath $Exe (Join-Path $Stage 'terralauncher.exe')
  foreach ($f in 'NOTICE', 'LICENSE-MIT', 'LICENSE-APACHE') { Copy-Item -LiteralPath (Join-Path $Root $f) $Stage }

  $Msi = Join-Path $OutDir "terralauncher-$Version-windows-x64.msi"
  & $Wix build (Join-Path $PSScriptRoot 'terralauncher.wxs') -arch x64 `
    -ext WixToolset.UI.wixext -ext WixToolset.Util.wixext `
    -culture en-US -loc (Join-Path $PSScriptRoot 'terralauncher.en-us.wxl') `
    -d "Version=$MsiVersion" -d "DisplayVersion=$Version" -d "BinDir=$Stage" `
    -d "IconPath=$(Join-Path $Root 'assets\app-icon\terralauncher.ico')" `
    -o $Msi
  if ($LASTEXITCODE -ne 0) { throw "wix build failed with exit code $LASTEXITCODE" }
  # wix writes debug symbols (.wixpdb) beside the MSI; they are not a release file.
  Remove-Item -Force -ErrorAction SilentlyContinue ([IO.Path]::ChangeExtension($Msi, '.wixpdb'))
  Get-Item $Msi | Format-Table Name, Length
} finally {
  Remove-Item -Recurse -Force $Stage -ErrorAction SilentlyContinue
}
