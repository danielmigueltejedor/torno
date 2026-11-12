# Agent skills

Skills for contributing to Torno and modelling through its [MCP server](../docs/mcp.md). Each folder is
one skill in the [Agent Skills](https://agentskills.io) format: a `SKILL.md` with a name and a
description that tell the agent when to load it, and an optional supporting reference file.

The modelling documents are built into Torno: its MCP server's `skill_reference` tool returns them, so an
agent without file access (such as the one in Torno's own chat) can still read a skill's reference.
The contributor skills (everything except modeling, editing, checking, mechanisms and sheet metal) are for repository work and are not embedded in that tool. `AGENTS.md` routes to them; `cargo xtask skills` fails when its table and this folder disagree or a file, `cargo xtask` command or skill they name is gone.

| Skill | Loads when the agent |
|---|---|
| [`torno-contributing`](torno-contributing/SKILL.md) | helps someone set up a fork, choose and claim an issue, and submit a contribution for review; the rules every PR follows |
| [`torno-ground-rules`](torno-ground-rules/SKILL.md) | adds a dependency, asset or wording: independence, licences, attribution, the product name |
| [`torno-crash-safety`](torno-crash-safety/SKILL.md) | writes non-test Rust, handles hostile input or investigates a crash report |
| [`torno-commands`](torno-commands/SKILL.md) | adds a command or decides which crate code belongs in |
| [`torno-layers`](torno-layers/SKILL.md) | decides whether code goes in Torno or solidcore (kernel, ops), or works on kernel or geometry code |
| [`torno-ui`](torno-ui/SKILL.md) | adds a dialog, panel, camera input, icon or colour |
| [`torno-ui-text`](torno-ui-text/SKILL.md) | adds a label, message or translation |
| [`torno-file-formats`](torno-file-formats/SKILL.md) | changes what a design saves, or touches import and export |
| [`torno-preferences`](torno-preferences/SKILL.md) | adds a setting or changes a default |
| [`torno-tests`](torno-tests/SKILL.md) | places or runs tests |
| [`torno-performance`](torno-performance/SKILL.md) | works on slow code, the window thread or the wasm build |
| [`torno-packaging`](torno-packaging/SKILL.md) | prepares a release or changes packages |
| [`torno-agent-tools`](torno-agent-tools/SKILL.md) | runs the app, drives the control channel or changes MCP tools |
| [`torno-modeling`](torno-modeling/SKILL.md) | builds a new part (enclosure, bracket, plate, flange, …) |
| [`torno-sheetmetal`](torno-sheetmetal/SKILL.md) | builds or changes a sheet metal part: base, edge and contour flanges, segments, hem, fold, unfold, flat pattern, DXF and bend table, and turns a request in words into flange parameters |
| [`torno-editing`](torno-editing/SKILL.md) | changes, fixes or extends an existing design |
| [`torno-checking`](torno-checking/SKILL.md) | builds, changes or ports a design, next to the two above: parameter ranges, every allowed value, exact STEP comparison, regression tests, designs kept in git |
| [`torno-mechanisms`](torno-mechanisms/SKILL.md) | builds, drives or animates an assembly: components, joints, motion links, closed loops such as actuators, contact sets, motion studies |
| [`torno-reporting`](torno-reporting/SKILL.md) | hears that something went wrong (a crash, a failed command, a wrong shape or number, a hang): reproduces it in a fresh design, shrinks it to a minimal script and writes the bug report |

The modelling skills teach the agent to:
- plan the part as numbers
- build it in a few `batch` calls with parametric positions
- verify every stage with `measure` against values it computed itself
- skip the large replies it doesn't need, such as the unfiltered command catalog and full topology dumps

They also list the traps that otherwise cost failed calls. Examples are the sketch axes of the XZ plane, edge picks that snap to the nearest edge, and cuts that miss but still report success.

The modelling skills are written for Torno 0.8 and later; `torno-mechanisms` for `main` after 0.11.1 (the next release), which counts the degrees of freedom of closed loops; `torno-reporting` for 0.18 and later; `torno-sheetmetal` for the build with `segments` on an edge flange (the `feat/sheet-flange-segments` branch; `unfold_method` on a rule needs PR #9911).

## Install

The modelling skills, for Claude Code in all projects:

```sh
git clone --depth 1 https://github.com/getnodalia/torno
cp -r torno/skills/torno-{modeling,editing,checking,mechanisms,sheetmetal} ~/.claude/skills/
```

Windows (PowerShell):

```powershell
git clone --depth 1 https://github.com/getnodalia/torno
foreach ($s in 'modeling','editing','checking','mechanisms','sheetmetal') { Copy-Item -Recurse torno\skills\torno-$s $HOME\.claude\skills\ }
```

- **One project only:** copy the folders into that project's `.claude/skills/` instead.
- **Other agents:** for agents that read the Agent Skills format, copy the folders into the skills directory their documentation names.
- **Updating:** copy the folders again after you update Torno.
- **Contributing:** the contributor skills need no install. Tell your assistant: "I want to contribute to Torno. Read https://github.com/tangency-labs/solidcraft/blob/main/skills/solidcraft-contributing/SKILL.md and help me get started."

## Maintaining

- The worked example in `torno-modeling/reference.md` runs in CI. `skill_reference_example_builds_as_documented` in `crates/mcp/src/tests.rs` builds it, checks the documented volume, changes a parameter and checks it again.
- The sketch-plane table is covered by `sketch_plane_axes_match_the_skill_reference` in the same file.
- The worked example in `torno-mechanisms/reference.md` runs in CI too: `mechanisms_skill_example_moves_as_documented` builds it, drives the hinge and checks the actuator stroke, the linked panel and the stop against their analytic values.
- The worked example in `torno-sheetmetal/reference.md` runs in CI: `sheetmetal_skill_example_builds_as_documented` builds the segmented flange, checks the volume against the hand formula and checks the bend table's lengths, angles and flat centre lines.
- `skills_name_only_real_tools_and_commands` checks that every tool and command the skills call exists.
- `torno-checking/scripts/tests` checks that skill's helpers, its traps and conventions, and the facts behind the corrections in `torno-modeling`: `PYTHONDONTWRITEBYTECODE=1 uv run --no-project --with pytest --with build123d pytest -p no:cacheprovider skills/torno-checking/scripts/tests`. The tests run the `torno-cli` on `PATH`, not the workspace build, and CI does not run them.
- When a command, a parameter name or a behaviour that these files describe changes, update them in the same PR.
- Facts that describe current limitations, such as edge picks without a distance limit, should go once the limitation is fixed.
