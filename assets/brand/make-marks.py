"""Builds the square icon tiles in this folder (tG, tL, tD, tP) from the line drawings.

Sources: design/tG.svg, design/tL.svg, design/tD.svg (single stroked paths on an A4 page) and
assets/brand/terrapen.svg (terraPen's own mark, for tP). Each tile is a rounded square with the
drawing centred at one stroke weight, so the four read as a set; each has its own colours.

Run from the repository root: python assets/brand/make-marks.py
"""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets" / "brand"
TILE = 400.0
# terraPen's yellow-green and its dark ink.
YELLOW = "#E5F15C"
DARK = "#171717"
# The drawing's longer side, and its stroke, as parts of the tile.
EXTENT = 0.70
STROKE = 0.013

# name: (source, tile colour, line colour)
SOURCES = {
    # White on black, like terraForge's icon.
    "tG": (ROOT / "design" / "tG.svg", "#0D0D0D", "#FFFFFF"),
    # The green of "Lens" in the card title (`ink_dark` of terraLens in core's catalog.rs).
    "tL": (ROOT / "design" / "tL.svg", "#6FC77C", DARK),
    "tD": (ROOT / "design" / "tD.svg", YELLOW, DARK),
    "tP": (OUT / "terrapen.svg", YELLOW, DARK),
}


def path_data(svg: str) -> str:
    return re.search(r'<path[^>]*\sd="([^"]+)"', svg, re.S).group(1)


def points(d: str):
    """Every coordinate pair the path names, following relative commands, for its bounds."""
    tokens = re.findall(r"[MmLlHhVvCcSsQqTtZz]|-?\d*\.?\d+(?:e-?\d+)?", d)
    out, x, y, start, cmd, i = [], 0.0, 0.0, (0.0, 0.0), "", 0
    arity = {"m": 2, "l": 2, "h": 1, "v": 1, "c": 6, "s": 4, "q": 4, "t": 2}
    while i < len(tokens):
        if re.fullmatch(r"[A-Za-z]", tokens[i]):
            cmd = tokens[i]
            i += 1
            if cmd in "Zz":
                x, y = start
            continue
        n = arity[cmd.lower()]
        nums = [float(t) for t in tokens[i : i + n]]
        i += n
        rel = cmd.islower()
        kind = cmd.lower()
        if kind == "h":
            x = x + nums[0] if rel else nums[0]
        elif kind == "v":
            y = y + nums[0] if rel else nums[0]
        else:
            pairs = [(nums[j], nums[j + 1]) for j in range(0, n, 2)]
            if rel:
                pairs = [(x + px, y + py) for px, py in pairs]
            out.extend(pairs)
            x, y = pairs[-1]
            if kind == "m":
                start = (x, y)
                cmd = "l" if rel else "L"
        out.append((x, y))
    return out


for name, (source, FILL, INK) in SOURCES.items():
    d = " ".join(path_data(source.read_text(encoding="utf-8")).split())
    pts = points(d)
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    w, h = max(xs) - min(xs), max(ys) - min(ys)
    scale = TILE * EXTENT / max(w, h)
    tx = (TILE - w * scale) / 2 - min(xs) * scale
    ty = (TILE - h * scale) / 2 - min(ys) * scale
    stroke = TILE * STROKE / scale
    svg = f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 400">
  <rect width="400" height="400" rx="88" fill="{FILL}"/>
  <path transform="translate({tx:.3f},{ty:.3f}) scale({scale:.5f})" d="{d}" fill="none" stroke="{INK}" stroke-width="{stroke:.3f}" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
"""
    (OUT / f"{name}.svg").write_text(svg, encoding="utf-8", newline="\n")
    print(f"{name}.svg  drawing {w:.0f} x {h:.0f}, scale {scale:.4f}")
