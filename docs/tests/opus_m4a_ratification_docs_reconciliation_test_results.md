# Opus M-4A Ratification — Docs Reconciliation — Test Results

**Date:** 2026-05-28
**Base HEAD:** `5e05dc30a6900d5963ae3d1dbbe9ec623f5298aa` (master; merge of PR #233 Opus oversight)
**Final commit SHA:** recorded post-merge (see worklog 2026-05-28 entry / PR for this branch)
**Branch:** `cursor-m4a-docs-reconciliation`

## Summary

Narrow **docs-only** reconciliation after Opus' M-4 / M-4A / first-slice oversight (PR #233)
landed. Removed stale pre-ratification wording in the M-4 design note and synchronized the
active mapping docs so they consistently state the ratified posture. **No production Rust,
WGSL, tests, or runtime behavior changed.**

## Docs-only confirmation

`git status --short` after edits shows only docs changes (plus unrelated, pre-existing
nondeterministic workshop bench-report `.txt` timing churn, which is **not** part of this
change and was left unstaged):

```
M docs/accumulator_op_v2_production_plan.md
M docs/worklog.md
M docs/workshop/mapping_atlas_batching_isolation_design_note.md
M docs/workshop/mapping_current_guidance.md
M docs/workshop/workshop_current_state.md
```

(The ADR, `design_v7_7.md`, `invariants.md`, and `todo.md` were already reconciled by PR #233
and required no further edits this round.)

## Files changed

| File | Change |
|---|---|
| `docs/workshop/mapping_atlas_batching_isolation_design_note.md` | §1 status table: "Blocked until human + Opus sign-off" → isolation sign-off done 2026-05-28, atlas implementation still gated on scenario + VRAM budget + §11 PR; added explicit isolation-policy sign-off row. §3 Rule reordered (algebraic preferred listed first) and framed as "exactly one of two admitted policies (+ deferred local-bounds)". "Tile gutter requirement" → "Tile isolation requirement" with the three-policy block. "Future packer obligation" rewritten to per-policy refusal rules. §4 "active evidence, not automatic ratification" → ratified-by-Opus wording. §8 stale "until human + Opus sign-off" line corrected. §11 `atlas_refuses_without_isolation_policy` row generalized to "exactly one admitted policy". |
| `docs/workshop/mapping_current_guidance.md` | Closing "gated after explicit Option A or Option B decision" → first-slice landed/accepted, Option 3 named next, Option 4 atlas not next. |
| `docs/workshop/workshop_current_state.md` | Header date 2026-05-28; "Next action" and decision-gate table updated to resolved gate (B done, A deferred, Option 3 next); removed stale Master HEAD line. |
| `docs/accumulator_op_v2_production_plan.md` | Phase M "next task" line and M-4 implementation PR block updated from "human + Opus sign-off / explicit Option A" to ratified-isolation + §11-gate-and-scenario-gated, Option 3 named next. |
| `docs/worklog.md` | Appended 2026-05-28 entry recording ratification (PR #233) + this reconciliation. Historical 2026-05-19 entries left intact (dated record). |

## Status wording fixed

- M-4 design-note `Implementation` row no longer reads "Blocked until human + Opus sign-off on
  this note"; it now states isolation-policy sign-off is complete and atlas implementation is
  gated on a named multi-theater scenario + approved VRAM budget + §11-gate-passing PR.
- Isolation requirement now cleanly separates the three policies: **AlgebraicTileLocalMask**
  (preferred candidate, homogeneous square, G=0, full-tile protocol-oracle parity),
  **PhysicalGutter** (fallback, G≥H), **LocalBoundsMetadata** (deferred).
- §11 remains the **binding acceptance gate**.
- "active evidence, not automatic ratification" replaced with ratified-by-Opus wording that
  still scopes ratification to the isolation design only (not Adopted, not implemented, not
  execution-authorized).
- Option 3 product-facing first-slice scenario fixture named as next step across active docs;
  Option 4 atlas packer explicitly not next.

## Stale wording scan

`rg` across the nine target files for ratification-sensitive phrases. Result: every active
**current-status** statement reflects the ratified posture. Remaining matches for
"pending Opus / human + Opus sign-off" and bare "Option A/Option B" occur only in:
- `docs/worklog.md` **dated 2026-05-19 historical entries** (honest record of past state; the
  new 2026-05-28 top entry carries current status);
- unrelated subsystems (capability tree, scripted events, resource flow, FMA decision) whose
  "Option A/B" is a different decision;
- dated `docs/tests/*` result logs and `docs/workshop/archive/*` (historical evidence, not
  active guidance).

No active doc describes the current status with stale wording.

## Guardrails preserved (stop-condition check — all NEGATIVE, i.e. none triggered)

| Stop condition | Present in edits? |
|---|---|
| atlas batching implemented | No |
| atlas Adopted (not Provisional) | No — remains Provisional/unimplemented |
| `request_atlas_batching` admitted before M-4 impl | No — still rejected at admission |
| M-4A ratification authorizes implementation by itself | No — explicitly isolation-policy only |
| mixed-size LocalBoundsMetadata approved | No — remains deferred |
| semantic WGSL allowed | No |
| simthing-sim may know RegionField | No — remains map-free |
| mapping execution default-on | No — `MappingExecutionProfile` default `Disabled` |
| active masks production-authorized | No |
| source_mask / behavioral source policy before M-5 | No |
| column-wide `source_col` zeroing allowed | No — banned |
| t44/corridor acceptance sufficient for atlas | No — full-tile protocol-oracle parity required |
| VRAM reporting optional | No — mandatory |

## Commands run

```text
git rev-parse master / HEAD     -> 5e05dc30a6900d5963ae3d1dbbe9ec623f5298aa
rustc --version                 -> rustc 1.95.0 (59807616e 2026-04-14)
cargo --version                 -> cargo 1.95.0 (f2d3ce0bd 2026-03-21)
git status --short              -> docs-only (+ unrelated bench .txt churn, unstaged)

cargo check --workspace
cargo test -p simthing-spec   --test region_field_spec_admission -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_runtime  -- --nocapture
cargo test -p simthing-gpu    --test accumulator_op_session_gpu_bridge -- --nocapture
```

**Substitution recorded:** This is a genuinely docs-only change (confirmed by `git status`).
Per the handoff's constrained-time provision, the full `cargo test --workspace` was replaced
with `cargo check --workspace` (green) plus the three mapping/admission suites the handoff
named as the minimum, plus the GPU bridge suite. No production code was touched, so a broader
run carries no additional signal for this change.

## Pass / fail table

| Check | Result | Detail |
|---|---|---|
| `cargo check --workspace` | **PASS** | Finished; only a pre-existing `unused_imports` warning in `simthing-driver` (not introduced here) |
| `region_field_spec_admission` | **PASS** | 10 passed; 0 failed — `request_atlas_batching` still rejected at admission |
| `phase_m_first_slice_runtime` | **PASS** | 28 passed; 0 failed — GPU-resident no-readback + R3 readiness intact |
| `accumulator_op_session_gpu_bridge` | **PASS** | 2 passed; 0 failed |

## Final verdict

**PASS** — Opus M-4A ratification docs reconciliation completed; active docs consistently
state that AlgebraicTileLocalMask G=0 is the preferred isolation candidate for homogeneous
square atlas batches, PhysicalGutter is fallback, LocalBoundsMetadata is deferred, atlas
remains Provisional/unimplemented, `request_atlas_batching` remains rejected, and the next
mapping step is Option 3 first-slice product scenario fixture.
