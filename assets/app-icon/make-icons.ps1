# Renders terraLauncher's icon (terralauncher.svg) and menu-bar template (tray-template.svg) at
# every size the app needs, and packs the Windows .ico. Needs Inkscape 1.x on PATH.
# Run from anywhere: pwsh assets/app-icon/make-icons.ps1
$ErrorActionPreference = 'Stop'
$here = $PSScriptRoot
$id = 'xyz.terrapen.terralauncher'
$icon = Join-Path $here 'terralauncher.svg'
$tmp = Join-Path ([System.IO.Path]::GetTempPath()) "terralauncher-icons-$PID"
New-Item -ItemType Directory -Force $tmp | Out-Null

function Export-Png([string]$svg, [int]$size, [string]$path) {
    New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
    & inkscape $svg --export-type=png "--export-filename=$path" -w $size -h $size 2>&1 | Out-Null
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $path)) { throw "inkscape failed for $path" }
}

foreach ($n in 16, 24, 32, 48, 64, 128, 256, 512) {
    Export-Png $icon $n (Join-Path $here "hicolor/${n}x${n}/apps/$id.png")
}
Export-Png $icon 1024 (Join-Path $here 'terralauncher-1024.png')
Export-Png $icon 64 (Join-Path $here 'tray-64.png')
Export-Png (Join-Path $here 'tray-template.svg') 44 (Join-Path $here 'tray-template-44.png')

# The .ico: PNG-compressed entries, which Windows Vista and later read.
$sizes = 16, 20, 24, 32, 40, 48, 64, 128, 256
$images = foreach ($n in $sizes) {
    $p = Join-Path $tmp "$n.png"
    Export-Png $icon $n $p
    , [System.IO.File]::ReadAllBytes($p)
}
$out = New-Object System.IO.MemoryStream
$w = New-Object System.IO.BinaryWriter $out
$w.Write([uint16]0); $w.Write([uint16]1); $w.Write([uint16]$sizes.Count)
$offset = 6 + 16 * $sizes.Count
for ($i = 0; $i -lt $sizes.Count; $i++) {
    $n = $sizes[$i]; $dim = if ($n -ge 256) { 0 } else { $n }
    $w.Write([byte]$dim); $w.Write([byte]$dim); $w.Write([byte]0); $w.Write([byte]0)
    $w.Write([uint16]1); $w.Write([uint16]32)
    $w.Write([uint32]$images[$i].Length); $w.Write([uint32]$offset)
    $offset += $images[$i].Length
}
foreach ($img in $images) { $w.Write($img) }
$w.Flush()
[System.IO.File]::WriteAllBytes((Join-Path $here 'terralauncher.ico'), $out.ToArray())
Remove-Item -Recurse -Force $tmp
"wrote icons to $here"
