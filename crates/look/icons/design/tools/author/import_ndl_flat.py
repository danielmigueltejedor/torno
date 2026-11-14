#!/usr/bin/env python3
"""Draw Torno's flat interface masters from Nodalia Design's icon set.

NDL draws its icons on a 24 x 24 grid with free SVG path syntax; Torno's masters are 64 x 64
and allow only absolute M L C Q A Z. Export the bold bodies from nodalia-design
(packages/icons) as JSON {"stroke": w, "icons": {"<torno id>": {"ndl": name, "body": [svg
elements]}}} and run `import_ndl_flat.py <that.json>`: each flat master is redrawn in currentColor.
NDL's soft face fills (fill-opacity) are left out, because a flat master is one solid colour.
"""
import json, math, os, re, sys

K = 64 / 24
NUM = r'-?(?:\d+\.?\d*|\.\d+)(?:e-?\d+)?'

def f(v):
    s = ('%.2f' % v).rstrip('0').rstrip('.')
    return '0' if s in ('-0', '') else s

def tokens(d):
    for m in re.finditer(r'([MmLlHhVvCcSsQqTtAaZz])|(%s)' % NUM, d):
        yield m.group(1) or float(m.group(2))

def path(d):
    """Absolute M L C Q A Z, scaled to the 64 grid."""
    toks = list(tokens(d)); i = 0; out = []
    x = y = sx = sy = 0.0; cmd = None; last_c = last_q = None
    def num():
        nonlocal i
        v = toks[i]; i += 1
        return v
    def flag():
        # Arc flags may be written run together ("01"); the tokenizer reads them as one number.
        nonlocal i
        v = toks[i]
        s = str(int(v)) if isinstance(v, float) and v == int(v) else None
        if s is not None and len(s) > 1:
            toks[i] = float(s[1:]); return int(s[0])
        i += 1
        return int(v)
    P = lambda px, py: '%s %s' % (f(px * K), f(py * K))
    while i < len(toks):
        t = toks[i]
        if isinstance(t, str):
            cmd = t; i += 1
            if cmd in 'Zz':
                out.append('Z'); x, y = sx, sy; last_c = last_q = None
                continue
        elif cmd is None:
            raise SystemExit('path data starts with a number: ' + d)
        rel = cmd.islower(); c = cmd.upper(); ox, oy = (x, y) if rel else (0.0, 0.0)
        nc = nq = None
        if c == 'M':
            x, y = ox + num(), oy + num(); sx, sy = x, y
            out.append('M' + P(x, y)); cmd = 'l' if rel else 'L'
        elif c == 'L':
            x, y = ox + num(), oy + num(); out.append('L' + P(x, y))
        elif c == 'H':
            x = ox + num(); out.append('L' + P(x, y))
        elif c == 'V':
            y = (y if rel else 0.0) + num(); out.append('L' + P(x, y))
        elif c in 'CS':
            if c == 'C':
                x1, y1 = ox + num(), oy + num()
            else:
                x1, y1 = (2 * x - last_c[0], 2 * y - last_c[1]) if last_c else (x, y)
            x2, y2 = ox + num(), oy + num(); x, y = ox + num(), oy + num()
            out.append('C%s %s %s' % (P(x1, y1), P(x2, y2), P(x, y))); nc = (x2, y2)
        elif c in 'QT':
            if c == 'Q':
                x1, y1 = ox + num(), oy + num()
            else:
                x1, y1 = (2 * x - last_q[0], 2 * y - last_q[1]) if last_q else (x, y)
            x, y = ox + num(), oy + num()
            out.append('Q%s %s' % (P(x1, y1), P(x, y))); nq = (x1, y1)
        elif c == 'A':
            rx, ry, rot = num(), num(), num(); la, sw = flag(), flag()
            x, y = ox + num(), oy + num()
            out.append('A%s %s %s %d %d %s' % (f(rx * K), f(ry * K), f(rot), la, sw, P(x, y)))
        else:
            raise SystemExit('unsupported path command ' + cmd)
        last_c, last_q = nc, nq
    return ''.join(out)

def attrs(el):
    return dict(re.findall(r'([a-z-]+)="([^"]*)"', el))

def element(el, stroke):
    tag = re.match(r'<(\w+)', el).group(1); a = attrs(el)
    filled = a.get('fill') == 'currentColor' and 'fill-opacity' not in a
    paint = 'fill="%s"' % ('currentColor' if filled else 'none')
    if a.get('stroke') != 'none':
        paint += ' stroke="currentColor" stroke-width="%s" stroke-linejoin="round" stroke-linecap="round"' % f(stroke * K)
    elif not filled:
        return None
    if tag == 'circle':
        return '<circle cx="%s" cy="%s" r="%s" %s/>' % (f(float(a['cx']) * K), f(float(a['cy']) * K), f(float(a['r']) * K), paint)
    if tag == 'rect':
        x, y, w, h = (float(a[k]) for k in ('x', 'y', 'width', 'height'))
        r = min(float(a.get('rx', 0)), w / 2, h / 2)
        if r:
            d = ('M%g %gH%gA%g %g 0 0 1 %g %gV%gA%g %g 0 0 1 %g %gH%gA%g %g 0 0 1 %g %gV%gA%g %g 0 0 1 %g %gZ'
                 % (x + r, y, x + w - r, r, r, x + w, y + r, y + h - r, r, r, x + w - r, y + h, x + r, r, r, x, y + h - r, y + r, r, r, x + r, y))
        else:
            d = 'M%g %gH%gV%gH%gZ' % (x, y, x + w, y + h, x)
        return '<path d="%s" %s/>' % (path(d), paint)
    if tag == 'path':
        return '<path d="%s" %s/>' % (path(a['d']), paint)
    raise SystemExit('unsupported element <%s>' % tag)

def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    data = json.load(open(sys.argv[1]))
    flat = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'svg', 'flat')
    for tid, icon in sorted(data['icons'].items()):
        target = os.path.join(flat, tid + '.svg')
        if not os.path.exists(target):
            raise SystemExit('Torno has no flat master %s' % tid)
        title = tid.replace('_', ' ').capitalize()
        body = [e for e in (element(el, data['stroke']) for el in icon['body']) if e]
        with open(target, 'w') as fh:
            fh.write('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">\n  <title>%s</title>\n' % title)
            for e in body:
                fh.write('  %s\n' % e)
            fh.write('</svg>\n')

main()
