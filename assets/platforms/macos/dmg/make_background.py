#!/usr/bin/env python3
"""Render the disk image window background (background.tiff) from background.svg.

background.svg is the Nodalia installer window drawn by `tools/dmg-background/draw_background.py`
in nodalia-design: two islands on the window colour, the left one in Torno's gold with a turned
part, the right one white under the icons (Finder draws their names in black). Its text is
outlined Sora glyph paths, so rendering it needs no fonts. To change the art, change that script,
regenerate the SVG with `draw_background.py torno background.svg`, and run this.

The SVG is drawn for layout.json (660 x 400 pt content, a 128 pt app icon at (326, 205) and
Applications at (574, 205)), which settings.py also reads, so the art and the icons agree.

Authoring tool only (assets/AGENTS.md): run it after changing the SVG, commit the TIFF. Packaging
never runs it. Needs Python 3.11+, Pillow 12+ and resvg on PATH:

    python3 -m venv target/dmg-art && target/dmg-art/bin/pip install pillow
    target/dmg-art/bin/python assets/platforms/macos/dmg/make_background.py [--preview DIR]

The TIFF holds a 1x (72 dpi) and a 2x (144 dpi) representation.
"""
import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from PIL import Image

HERE = Path(__file__).resolve().parent
LAYOUT = json.loads((HERE / "layout.json").read_text())
W, H = LAYOUT["window"]
PALETTE = 32  # flat inks plus antialiasing; a palette keeps the file small


def render(svg, scale, out):
    subprocess.run(["resvg", "--skip-system-fonts", "-w", str(W * scale), str(svg), str(out)], check=True)
    with Image.open(out) as im:
        if im.size != (W * scale, H * scale):
            sys.exit(f"error: {svg.name} is {im.size}, layout.json wants {W}x{H} pt")
        return im.convert("RGB")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--preview", help="also write the 1x and 2x PNGs to this directory")
    ap.add_argument("--out", default=str(HERE / "background.tiff"))
    args = ap.parse_args()
    if not shutil.which("resvg"):
        sys.exit("error: resvg not found (brew install resvg / cargo install resvg --locked)")
    with tempfile.TemporaryDirectory() as tmp:
        one, two = (render(HERE / "background.svg", s, Path(tmp) / f"{s}x.png") for s in (1, 2))
    if args.preview:
        out = Path(args.preview)
        out.mkdir(parents=True, exist_ok=True)
        one.save(out / "background.png")
        two.save(out / "background@2x.png")
    pages = [im.quantize(PALETTE, method=Image.Quantize.FASTOCTREE, dither=Image.Dither.NONE) for im in (one, two)]
    pages[1].encoderinfo = {"dpi": (144, 144)}
    pages[0].save(args.out, "TIFF", save_all=True, append_images=[pages[1]],
                  compression="tiff_deflate", dpi=(72, 72))
    with Image.open(args.out) as im:
        im.seek(1)
        if im.info.get("dpi") != (144, 144):
            sys.exit("error: the 2x page isn't 144 dpi (needs Pillow 12 or later)")


if __name__ == "__main__":
    main()
