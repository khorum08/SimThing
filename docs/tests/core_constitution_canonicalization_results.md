# CORE-CONSTITUTION-CANONICALIZATION-0 Results

## Status

PROBATION / design-doc consolidation — canonicalized Scenario/GameSession spatial tree, RF channel doctrine,
Gu-Yang/PALMA core summary, and simthing-tools typeface ADR; archived old typeface ladder/proposal.

No runtime code changes. All `.rs` files unchanged. Docs only.

## PR / branch / merge

- Branch: `core-constitution-canonicalization-0`
- PR: pending
- Merge SHA: pending

---

## Core design changes (`docs/simthing_core_design.md`)

### §2 rewritten

Section header changed from "The one tree, and the Session root" to
"The one tree: Scenario wrapper, GameSession root, owners, and spatial containment".

New content:
- Canonical Scenario → GameSession → Owners + GalaxyMap → gridcells → surface gridcell → gameplay children tree.
- Three laws stated explicitly: physical containment only; owners never spatial parents; movement is the only spatial reparenting.
- §2.1 opening clarifies that the owner-entity is a GameSession child beneath the Scenario wrapper.

### §5 expanded with RF channel doctrine

Section renamed "Resource flow arenas and channels — one mechanism for everything".

New subsections:
- §5.1 Channel identity: `(parent_location_id, owner_ref, resource_key, scope_id)` key defined.
- §5.2 Local settlement before bubbling: pseudocode showing settle-per-parent-Location before upward reporting.
- §5.3 What counts as RF: all combat/economy/logistics/diplomacy/AI are RF lanes; names compile away.

### §7.2.1 added

New subsection "Production front operators: borders and pathfinding" immediately before the existing
"Production operators" paragraph. Low-context summary: Gu-Yang/SaturatingFlux for borders/chokes;
PALMA for reach/impedance/pathfinding. Key line: "The front is the route."

### simthing-tools crate summary updated

Replaced `docs/design_typeface_ladder.md` + `docs/design_simthing_typeface_track_proposal.md` references
with `docs/simthing_tools_typeface_adr.md` (root ADR) + archive path.

---

## Invariants changes (`docs/invariants.md`)

### New section: Scenario / GameSession / Spatial Tree

Eight rules added:
- Scenario is the save/load authority wrapper.
- Scenario has exactly one direct GameSession child.
- GameSession is the runtime session root beneath Scenario.
- Owner SimThings are direct GameSession children.
- Owners are never spatial parents.
- GalaxyMap / WorldStateMap is a direct GameSession child and the spatial root.
- Planet gridcells contain a 1×1 surface gridcell before gameplay children.
- Movement is the only spatial reparenting.

### RF channel identity and settlement (new sub-table)

Five rules added to Resource Flow Substrate:
- RF channel identity is owner/resource/scope metadata, not containment.
- Local settlement precedes upward bubbling.
- RF channels do not cross owners by default.
- RF overlay/property maps are authoritative channel tags.
- Scope IDs distinguish local/system/strategic arenas without new engines.

### Front propagation operators (new sub-table in Mapping)

Three rules added:
- Gu-Yang/SaturatingFlux is the production front-propagation operator.
- PALMA is the production reach/impedance utility.
- A rendered path/border is presentation only.

---

## Constitution changes (`docs/design_0_0_8_3.md`)

### Typeface/simthing-tools reference updated

Replaced old ladder/proposal pointers with `docs/simthing_tools_typeface_adr.md` (root ADR).
Added archive paths for ladder and proposal.

### New subsection: Terminology correction — Scenario wrapper and GameSession root

Added after "Terminology correction — owner, not faction":
- Scenario is the save/load authority wrapper; GameSession is the runtime root beneath it.
- RF membership is channel identity, not ownership containment.
- Local owner/resource/scope channels settle inside each parent Location before bubbling upward.
- `simthing_core_design.md` §5 is the canonical home for this doctrine.

---

## TypeFace ADR consolidation

Created `docs/simthing_tools_typeface_adr.md`:
- Status: ACCEPTED / CLOSED / DA-APPROVED
- Authority boundary section
- Complete public API seams (from lib.rs at closure time)
- Closed-track outcomes table (LR0–LR9 + closeout PRs)
- GPU-residency guarantees
- Supported use-cases and non-goals/deferred
- Evidence pointers and closure commits
- Decision rule for future work

---

## Archived docs

| Old path | New path |
|---|---|
| `docs/design_typeface_ladder.md` | `docs/archive/typeface_track_2026_06/design_typeface_ladder.md` |
| `docs/design_simthing_typeface_track_proposal.md` | `docs/archive/typeface_track_2026_06/design_simthing_typeface_track_proposal.md` |

Moved via `git mv`. Archive README updated to point to `docs/simthing_tools_typeface_adr.md`.

---

## Reference/link checks

Live references to old paths updated in:
- `docs/simthing_core_design.md`
- `docs/design_0_0_8_3.md`
- `docs/invariants.md`
- `docs/tests/current_evidence_index.md`
- `docs/workshop/studio_production_log.md`

Remaining references to old paths:
- `docs/tests/typeface_lr*.md` — historical results docs; left as-is (historical record)
- `docs/tests/typeface_cleanup_docs_archive_results.md` — historical record of PR #900; left as-is
- `docs/archive/typeface_track_2026_06/*.md` — internal archive references; expected and appropriate

---

## Validation

| Check | Result |
|---|---|
| `cargo check -p simthing-spec` | pending |
| `cargo check -p simthing-driver` | pending |
| `cargo check -p simthing-tools` | pending |
| `cargo check -p simthing-mapeditor` | pending |
| `cargo test -p simthing-tools --test semantic_free_guard` | pending |
| No `.rs` files changed | PASS — `git diff --name-only` shows docs only |
| Reference guard: no active-doc pointer to old ladder path | PASS (updated) |
| `docs/simthing_tools_typeface_adr.md` created | PASS |
| Old ladder/proposal archived via `git mv` | PASS |

---

## Remaining debts

- `cargo check` / `semantic_free_guard` results (run after this doc is written)
- `STUDIO-TYPEFACE-STARTUP-FIX-0` (separate local WIP for Studio blank screen; uncommitted)
- Production icon art / default font choice (non-blocking, documented in ADR)
- Windowed Studio smoke (non-blocking)

---

## DA recommendation

PROBATION — docs-only consolidation, no DA escalation required. Design authority may promote to
ACCEPTED / closed after validating that the canonical trees and channel doctrine match the live code
and evidence index. No runtime scope.
