# Draws terraLauncher's placeholder mark (the same shapes as terralauncher.svg) at every size
# the app needs, and packs the Windows .ico. Run from anywhere: pwsh assets/app-icon/make-icons.ps1
Add-Type -AssemblyName System.Drawing
$here = $PSScriptRoot
$id = 'xyz.terrapen.terralauncher'

function New-Mark([int]$size, [bool]$template) {
    $bmp = New-Object System.Drawing.Bitmap $size, $size, ([System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.SmoothingMode = 'AntiAlias'
    $g.PixelOffsetMode = 'HighQuality'
    $g.Clear([System.Drawing.Color]::Transparent)
    $s = [single]$size
    $ink = if ($template) { [System.Drawing.Color]::Black } else { [System.Drawing.Color]::White }
    if (-not $template) {
        # Rounded square, radius 22 %.
        $r = [single]($s * 0.22); $d = $r * 2
        $path = New-Object System.Drawing.Drawing2D.GraphicsPath
        $path.AddArc(0, 0, $d, $d, 180, 90); $path.AddArc($s - $d, 0, $d, $d, 270, 90)
        $path.AddArc($s - $d, $s - $d, $d, $d, 0, 90); $path.AddArc(0, $s - $d, $d, $d, 90, 90)
        $path.CloseFigure()
        $fill = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 30, 107, 88))
        $g.FillPath($fill, $path); $fill.Dispose(); $path.Dispose()
    }
    # The "t": a stem that curves into a foot, and a crossbar.
    $pen = New-Object System.Drawing.Pen $ink, ([single]($s * 0.11))
    $pen.StartCap = 'Round'; $pen.EndCap = 'Round'; $pen.LineJoin = 'Round'
    $stem = New-Object System.Drawing.Drawing2D.GraphicsPath
    $stem.AddLine([single]($s * 0.42), [single]($s * 0.20), [single]($s * 0.42), [single]($s * 0.62))
    $stem.AddBezier([single]($s * 0.42), [single]($s * 0.62), [single]($s * 0.42), [single]($s * 0.76), [single]($s * 0.50), [single]($s * 0.80), [single]($s * 0.60), [single]($s * 0.80))
    $g.DrawPath($pen, $stem); $stem.Dispose()
    $g.DrawLine($pen, [single]($s * 0.27), [single]($s * 0.40), [single]($s * 0.60), [single]($s * 0.40))
    $pen.Dispose()
    # The pen tip: a dot where the line ends.
    $dotColor = if ($template) { [System.Drawing.Color]::Black } else { [System.Drawing.Color]::FromArgb(255, 240, 160, 72) }
    $dot = New-Object System.Drawing.SolidBrush $dotColor
    $dr = [single]($s * 0.065)
    $g.FillEllipse($dot, [single]($s * 0.76) - $dr, [single]($s * 0.80) - $dr, $dr * 2, $dr * 2)
    $dot.Dispose(); $g.Dispose()
    $bmp
}

function Save-Png($bmp, [string]$path) {
    New-Item -ItemType Directory -Force (Split-Path $path) | Out-Null
    $bmp.Save($path, [System.Drawing.Imaging.ImageFormat]::Png); $bmp.Dispose()
}

foreach ($n in 16, 24, 32, 48, 64, 128, 256, 512) {
    Save-Png (New-Mark $n $false) (Join-Path $here "hicolor/${n}x${n}/apps/$id.png")
}
Save-Png (New-Mark 1024 $false) (Join-Path $here 'terralauncher-1024.png')
Save-Png (New-Mark 64 $false) (Join-Path $here 'tray-64.png')
Save-Png (New-Mark 44 $true) (Join-Path $here 'tray-template-44.png')

# The .ico: PNG-compressed entries, which Windows Vista and later read.
$sizes = 16, 20, 24, 32, 40, 48, 64, 128, 256
$images = foreach ($n in $sizes) {
    $ms = New-Object System.IO.MemoryStream
    $b = New-Mark $n $false; $b.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png); $b.Dispose()
    , $ms.ToArray()
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
"wrote icons to $here"
