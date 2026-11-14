#!/usr/bin/env python3
"""Import the shared Nodalia volumetric CAD icons into Torno's masters.

The set is drawn once in nodalia-design (packages/icons/volumetric) and shared by every
Engineering app. Build it there (`node volumetric/build.mjs <dir>`), then run
`import_ndl.py <dir>`: each model/ and workspace/ master Torno already has is replaced by the
icon of the same name, with the shared roles mapped to this palette's source colours. small/
variants are removed, because the shared art is drawn to read at small sizes.
"""
import os, re, sys

# Shared role colour (nodalia-design volumetric/scene.mjs ROLES) -> palette.toml source.
ROLES = {
    '#26303d': '#34465b',  # ink -> ink
    '#28323f': 'currentColor',  # line -> line (drawn on the background)
    '#eceef2': '#e2e8ef',  # face-1 -> passive-1
    '#c3c7d1': '#b8c3cf',  # face-2 -> passive-2
    '#9499a8': '#8e9faf',  # face-3 -> passive-3
    '#f3e4ac': '#b9ddf8',  # accent-1 -> active-1
    '#d1b453': '#328bd3',  # accent-2 -> active-2
    '#8f7518': '#236ead',  # accent-3 -> active-3
    '#7a8193': '#6f879c',  # guide -> guide
    '#ffffff': '#e2e8ef',  # paper -> passive-1 (glyphs on badges; gap would cut holes)
}

def convert(svg):
    def sub(m):
        hexv = m.group(2).lower()
        if hexv not in ROLES:
            raise SystemExit('unknown colour %s' % hexv)
        return '%s"%s"' % (m.group(1), ROLES[hexv])
    return re.sub(r'((?:fill|stroke)=)"(#[0-9a-fA-F]{6})"', sub, svg)

def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    src = sys.argv[1]
    here = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'svg')
    missing = []
    for fam in ('model', 'workspace'):
        for f in sorted(os.listdir(os.path.join(here, fam))):
            name = f[:-4]
            shared = os.path.join(src, name.replace('_', '-') + '.svg')
            if not os.path.exists(shared):
                missing.append(name)
                continue
            with open(shared) as fh:
                svg = convert(fh.read())
            with open(os.path.join(here, fam, f), 'w') as fh:
                fh.write(svg)
    small = os.path.join(here, 'small')
    for f in os.listdir(small):
        os.remove(os.path.join(small, f))
    if missing:
        raise SystemExit('not in the shared set: ' + ', '.join(missing))

main()
