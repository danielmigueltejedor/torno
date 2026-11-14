# Torno brand kit

Torno's mark is its app icon: a portrait of Henry Maudslay over Torno's gold, in a rounded tile.
The source picture is `assets/icons/app/shared/source/torno-portrait-512.rgba`, and every size is
made from it, so the desktop, the packages, the web app and these files always show the same image.

## Files

| File | Use it for |
|---|---|
| `assets/icons/app/linux/exports/hicolor/512x512/apps/dev.danielmiguelt.Torno.png` | The icon on its tile, transparent around it: avatars, slides, the web. |
| `assets/icons/app/linux/exports/hicolor/*/apps/dev.danielmiguelt.Torno.png` | The same icon at smaller sizes. |
| `web/source/og.png` | The link preview card for the docs site. |
| `assets/styles/web/loading-logo.svg` | The web app's loading screen. |

The name is set in IBM Plex Sans Bold (SIL Open Font License 1.1, `crates/look/fonts/ibm-plex-sans/`).

## Don'ts

- Don't stretch, squash, rotate, skew or crop the icon. Scale it evenly.
- Don't add outlines, glows or other effects, and don't put it on a busy background.

## Name and licence

The icon and the card are MIT OR Apache-2.0; the source portrait is in the public domain. The name
Torno identifies the project: if you publish a modified build, give it its own name and icon so
people don't mistake it for Torno.
