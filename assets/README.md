# Application resources

`assets/` is the design resource home for Torno.
[ATTRIBUTION.md](../ATTRIBUTION.md) is the file inventory and provenance record.
Keep one editable master per resource and place shipped derivatives in `exports/`.

```text
assets/
  icons/
    app/
      shared/source/                 # app icon picture, its resampler and the executable ICO encoder
      macos/exports/                 # icon.icns
      windows/exports/               # executable DIB ICO and installer PNG ICO
      linux/exports/hicolor/         # Linux/FreeBSD package images, 16–512 px
    ui/design/svg/                   # authoritative original UI icon masters
    ui/design/tools/                 # authoring bases, checks and included review app
    ui/source/                       # Rust viewport glyphs, markers and navigation
  branding/web/source/og.png          # shared social preview card
  styles/web/loading.css             # web loading/canvas styles
  appearances/library.rs             # built-in visual presets
  materials/library.rs               # physical defaults and colours
  fonts/biz-udp-gothic/              # unmodified font and OFL licence
  locales/{en,de,ja,ko}.json          # shared offline catalogs
  platforms/{macos,linux,windows}/    # bundle, freedesktop and WiX templates
  platforms/macos/dmg/                # disk image window: layout, dmgbuild settings, background art
  archive/icons/macos-v1/
    source/icon.vectorcraft          # earlier editable work, retained
                                     # redundant historical exports are retired
```

## Editing and regeneration

[UI artwork](icons/ui/README.md) identifies the drawing masters and their consumers.
Read the mandatory [UI style guide](icons/ui/STYLE_GUIDE.md) before editing an icon.
Edit SVG geometry and its command/UI assignments together; inspect both themes
and small sizes. Temporary media and generated reports belong in ignored target/. Palettes and model presets are also editable Rust data.

The app icon is the portrait in `icons/app/shared/source/` (`app_icon.rs` resamples it). Keep any
native design work and linked inputs under that resource's `source/work/`, with an attribution
row; retain the editable document alongside its exports.

Regenerate all exports of the shared Windows/Linux/FreeBSD mark together:

```sh
cargo xtask app-icons
```

macOS uses `icons/app/macos/exports/icon.icns`, made from the same portrait. There is no Icon
Composer project, so the bundle carries no `Assets.car`.

Review and commit source changes and required exports together. Temporary renders,
tool caches and package staging belong under ignored `target/` or `dist/`.
Normal builds use the committed exports and require no design applications.
The CLI still supports arbitrary PNG/ICO exports with `torno-cli icon`.

## Platform bindings

| Platform | Resources and output paths |
| --- | --- |
| macOS | `icon.icns`, installed in the bundle as `Torno.icns` |
| Windows executable | `torno-executable.ico`: original DIB/mask representations at 16/24/32/48/64/128/256 px |
| Windows installer | `torno.ico`: PNG representations at 16/20/24/32/40/48/64/128/256 px; `TORNO_ICONS` override retained |
| Linux, Nix, deb/rpm, tar, Flatpak, Snap | Copy canonical hicolor images; icon name stays `dev.danielmiguelt.Torno` |
| AppImage | Uses the Linux 256 px image and `.DirIcon` alias |
| FreeBSD | Shares Linux hicolor images and freedesktop metadata |
| Web | HTML/Trunk and `manifest.webmanifest` stay in `apps/torno-web/`; copies Japanese font/licence to `fonts/`, uses the collected CSS, and copies the Linux 32/256/512 px images to `icons/` for the favicon and installed app |
| Docs and link previews | `xtask book` copies the shared card to the existing `/torno/og.png` public path |

## Code and provenance

Resources remain compiled by their owning layer through existing APIs:
`ui-egui::icons::paint_icon` and `Tokens` at L5, model presets in L2 `doc`, and sketch
fonts at L1. The sketch crate owns its built-in Liberation Sans font and OFL
licence under `crates/sketch/assets/fonts/liberation-sans/`, so Cargo source
packages and vendored builds include both files. State, projection, interaction, commands and GPU bindings stay in
their current modules. Embedded data uses `CARGO_MANIFEST_DIR`; application code
needs no runtime resource registry or repository lookup.

`cargo xtask assets` requires an exact attribution row for every collected file
except Markdown documentation, and continues checking file assets elsewhere in
the repository. This includes native work, Rust masters, catalogs, shaders and
platform templates. Packaging selects explicit current files and fails when
required inputs are absent. Archived artwork has no production consumers.

Dependency-owned egui fonts and vendored renderer shaders stay with their
libraries; their licences remain in `THIRD-PARTY-LICENSES.txt` and `ATTRIBUTION.md`.
System fonts, optional packs, user fonts/canvases/decals/materials and native OS
controls retain external ownership. Root licence/notice texts stay canonical.
