---
name: torno-packaging
description: "Use when preparing or shipping a Torno release, changing installers or packages (AppImage, MSI, dmg, Flatpak, Snap, WinGet, Nix, FreeBSD, web), release asset names, signing or the Pages deployment."
---

# Packaging and releases

A pull request that bumps the version and adds `docs/release-notes/vX.Y.Z.md` releases that version when it merges; any other push releases nothing. The full procedure is `docs/releasing.md`; follow it when asked to prepare or ship a release.

## Preparing
1. Choose one version for the whole batch from the actual changes (patch, minor, major, `-rc.N`). Do not infer it from commit prefixes. Moving to `1.0` is the owner's decision.
2. Update `[workspace.package] version` in `Cargo.toml`, then `cargo update --workspace --offline`; only our own crates' `version =` lines in `Cargo.lock` change.
3. Fold the pending design-format changes with `scripts/fold-format-pending.sh` (`skills/torno-file-formats/SKILL.md`).
4. Write `docs/release-notes/vX.Y.Z.md` (format in that folder's README; `scripts/check-release-notes.sh vX.Y.Z` checks it).
5. Run `cargo xtask ci` and the packaging checks in `.github/workflows/packaging-lint.yml`. Land the version bump and notes on `main` in one pull request; never use `[skip ci]`.

## Building and publishing
- Merging is the release: `release-auto.yml` has Foundation's `release-tag` tag the merge and draft the release with the notes, `release.yml` builds, checks and attaches every package, and `release-publish` publishes it. Never push a tag or a release branch by hand.
- A failed platform leaves a draft: land the fix, then run **Release from main** with `retag-draft`. Never overwrite a published version, tag or asset; use a new version.
- After publication, check the release: its tag resolves to the merge commit, `sha256sum --check SHA256SUMS.txt` passes, and the built CLI runs (`--version`, an example model). Packages are unsigned unless the build summary says otherwise; do not claim they are signed.
- Nightlies are prereleases built by the same workflow on a schedule; they never become latest.

## Names that must stay stable
- Assets are `torno-<version>-...` and are also attached without the version (`torno-windows-x64.msi`). The README links and the in-app update check (`crates/ui-egui/src/update.rs`) pick them by name; renaming one breaks both.
- The old product name survives only in the shims in `docs/compatibility.md`. Do not rewrite published assets, checksums, tags or historical notes.
- Documentation, the browser app and the Flatpak repository publish under project Pages `/torno/`. Add no old-name Pages aliases.

## Where things live
- `packaging/` holds the per-platform scripts (`packaging/linux`, `packaging/windows`, `packaging/macos`, `packaging/freebsd`, `packaging/web`); `packaging/tests` checks them.
- Nix: `flake.nix` and `docs/nix.md`. Building: `docs/building.md`. Downloads: `docs/downloads.md`.
- Build with `CARGO_TARGET_DIR=<repo>/target`. Windows from Linux: `cargo xwin build --release --target x86_64-pc-windows-msvc -p torno`.
