# Torno upstream

| Field | Value |
|---|---|
| Upstream | `tangency-labs/solidcraft` |
| Last observed | `87400c80ef817acde59ad5984b9ccd6209fcca96` |
| Last evaluated | `87400c80ef817acde59ad5984b9ccd6209fcca96` |
| Last integrated | `87400c80ef817acde59ad5984b9ccd6209fcca96` imported with full history on 2026-11-10 |
| Skipped | none in the baseline import |
| Adapted | app icon: `assets/icons/app/shared/source/app_icon.rs` resamples a portrait instead of drawing upstream's mark; every SolidCraft name renamed to Torno (`.github/rebrand.toml`) |
| Conflicting | none recorded |

Torno shares history with upstream, but never merges `upstream/main` directly: upstream still
calls the app SolidCraft, and its crates, paths, app id, environment variables and docs say so.
Upstream's name and logo are covered by `LICENSE-BRAND`; Torno uses its own name and icon.

## How upstream arrives renamed

`scripts/rebrand.py` renames upstream's names to Torno's, in file contents and in paths, from the
rules in `.github/rebrand.toml` (literal replacements in order; spans to keep such as links to
upstream's issues, the `.solidcraft` design extension and file format tags; files to skip such as
`NOTICE` and the licences; and lines that keep Torno's own value such as the version). A line
marked `rebrand: keep` is left alone, for compatibility code that has to name the old product
(docs/compatibility.md).

The branch `upstream-torno` mirrors upstream with Torno's names. Each of its commits holds the
rebranded tree of one upstream commit, has the previous mirror commit and that upstream commit as
parents, and names it in an `Upstream-Commit:` trailer. The weekly sync
(`.github/workflows/upstream-sync.yml`) adds a mirror commit for upstream's new head and merges
that, so upstream's changes arrive already speaking Torno, and a conflict is a real one. Files in
`.github/upstream-keep` keep Torno's version.

The first mirror commit is upstream `87400c80` renamed; it was merged with `-s ours` when the
whole repository was renamed, so only later upstream changes come through it.

`python3 scripts/rebrand.py check` lists anything still using an upstream name.
