# AS-CLOSEOUT-0 Final Admission Substrate Closeout

## Status

DONE — DA-APPROVED (executive DA, 2026-06-29). 0.0.8.4 Admission Substrate track CLOSED. Rungs 1–8B
DA-APPROVED; consolidation completed at DA review (10 sub-rung slice docs expunged — see below).

## PR / branch / merge

- Branch: `codex/as-closeout-0`
- PR: https://github.com/khorum08/SimThing/pull/977
- Merge: `89b3b54ccb` (master)

## Track summary

0.0.8.4 Admission Substrate promoted nine prose/guard-enforced invariants to type boundaries across rungs 1–8B. Each rung retired the enforcement surface it replaced. The track's success metric is **net-negative enforcement surface**: fewer guards, shorter prose, shorter scans. No source behavior was changed; CPU-oracle parity was preserved throughout.

**Build ladder rungs 1–8B are DONE / DA-APPROVED** (per DA graduation logs in `docs/design_0_0_8_4_admission_substrate.md`).

## Final ladder state

| Rung | ID | Primary boundary delivered | State |
|---|---|---|---|
| 1 | `AS-COLUMN-ACCESS-0` | `PropertyValue.data` encapsulated; `raw_lane()` only escape | **DONE — DA-APPROVED** |
| 2 | `AS-CHANNEL-NEWTYPES-0` | `OwnerRef` / `ResourceKey` / `ScopeId` / `ParentLocationId` newtypes; production adoption | **DONE — DA-APPROVED** |
| 3 | `AS-KIND-OUT-OF-TICK-0` | Kind-free tick view; production kind-reads resolved to marker columns | **DONE — DA-APPROVED** |
| 4 | `AS-SIM-SEMANTIC-FREE-0` | `simthing-sim` public surface cannot name semantic kinds | **DONE — DA-APPROVED** |
| 5 | `AS-INDEX-NEWTYPES-0` | `SlotIndex` / `ColumnIndex` private-field newtypes; old alias deleted | **DONE — DA-APPROVED** |
| 6 | `AS-STRUCTURAL-COORD-0` | `StructuralCoord` private-field newtype; render floats cannot enter structural paths | **DONE — DA-APPROVED** |
| 7 | `AS-TICK-FABRIC-BOUNDARY-0` | `SimulationFabric` seals hot tick; session-loop direct dispatch removed | **DONE — DA-APPROVED** |
| 8 | `AS-PACKED-UPLOAD-BOUNDARY-0` | Structural GPU upload sealed behind `PackedUpload` | **DONE — DA-APPROVED** |
| 8B | `AS-8B` | Session upload paths sealed behind packed packet types | **DONE — DA-APPROVED** |
| F | `AS-CLOSEOUT-0` | Scope Ledger / consolidation | **PROBATION** |

## Scope Ledger

| Rung | Delivered boundary | Primary enforcement form | Evidence | Guard / prose retired | Remaining honest residue |
|---|---|---|---|---|---|
| AS-1 | `PropertyValue.data` private; role/layout-mediated access only | `data` field inaccessible; `raw_lane()` greppable escape | `as_column_access_0_results.md` / #949 `ea31b2e1ab` | `data[N]` source scan as primary gate | Serialization byte-lanes via `raw_lane()` by design |
| AS-2 | `OwnerRef` / `ResourceKey` / `ScopeId` / `ParentLocationId(u32)` newtypes; production channel sites adopted | `compile_fail`: passing `ResourceKey` where `OwnerRef` expected; `as_channel_newtypes_production_adoption` | `as_channel_newtypes_0_results.md`, `as_channel_newtypes_0r_results.md` / #951 `3a6df374f8`, #966 `2089c0126b` | Arg-order / mis-binding runtime checks; raw-string transposition grep | None — channel identity is type-enforced |
| AS-3 | Kind-free tick view; `FissionCloneSourceView` + `ResolvedFissionChildBlueprint` resolve clone eligibility to marker columns | `compile_fail` on `.kind` in tick view; `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | `as_kind_out_of_tick_closeout_results.md` / #952–957 | `is_capability_container(&child.kind, …)` production branch deleted; per-slice prose restatements | Test/fixture `SimThingKind` construction; `Faction` legacy-serialization variant; CPU-boundary driver closures |
| AS-4 | `simthing-sim` public surface cannot name semantic `SimThingKind` variants | `compile_fail` on kind import at sim boundary; `as_sim_semantic_free_public_surface_audit` | `as_sim_semantic_free_closeout_results.md` / #958–960 | Broad semantic-free source scans narrowed to declared-public-module audit + compile_fail seams | WGSL shader text; replay/delta `SimThing` CPU-boundary payloads; `pub(crate)` internal seams (acknowledged follow-on territory) |
| AS-5 | `SlotIndex(u32)` + `ColumnIndex(usize)` private-field newtypes; old `type ColumnIndex = usize` alias deleted; owner-vs-spatial-parent `compile_fail` | `owner_ref_rejects_spatial_parent` compile_fail; `accumulator_op_gpu_encoding_preserved_after_index_newtypes` | `as_index_newtypes_0_results.md` / #969 `e39b85ba59`, #970 `ffae0eb5f6` | Index-mixing lint; "owner is never a spatial parent" prose detector | Raw `u32` at WGSL/GPU packing boundary via explicit `.raw()` / `.raw_u32()` (by design) |
| AS-6 | `StructuralCoord { col, row }` private-field newtype; render `f32` coords cannot enter structural paths | `compile_fail` on `StructuralCoord` from render float; N4-atlas path enforces | `as_structural_coord_0_results.md` / #961 `f6e82ea023` | Public-field `StructuralGridCoordinate` literal seam deleted | None — seam is type-enforced |
| AS-7 | `SimulationFabric` seals hot tick; all GPU tick dispatch via `run_simulation_fabric_*`; session-loop direct dispatch removed | `compile_fail` on fabric field access outside boundary; mapping hot path cannot reach boundary effects | `as_tick_fabric_boundary_0a_results.md`, `0b`, `0c` / #962–964 | Direct session-loop GPU dispatch replaced; planning state structurally inaccessible inside tick | No-CPU-planner boundary-time planning (behavior-over-time; types cannot reach it) |
| AS-8 | `upload_structural_rows_to_gpu` accepts only `&PackedUpload`; free frame+slice bundle removed | `compile_fail` on free row args; `compile_fail` on field-literal `PackedUpload`; byte parity tests | `as_packed_upload_boundary_0_results.md` / #973 `35afe6e5e7` | Free-row upload function signature removed | None — structural seam type-enforced |
| AS-8B | `AccumulatorOpSession` upload consumes only packed packet types; 8 free-slice methods removed | 6 `compile_fail` proofs; 83+ lib tests; encode/sidecar parity | `as_8b_session_upload_packed_packets_results.md` / #975 `734f84d9d7` | Free-slice session upload methods (`upload_ops`, `upload_gpu_ops`, `upload_threshold_ops`, `upload_intent_ops`, etc.) | WGSL shader text (Rust type system cannot inspect WGSL); `WorldAccumulatorRuntime` envelope still packs semantic slices CPU-side above the sealed session seam (correct per §4) |

## Invariant promotion ledger

Each row records an invariant that moved from prose / guard-scan / runtime-check up to type boundary.

| Invariant | Before (rung opened) | After (rung closed) |
|---|---|---|
| No hardcoded `data[N]` column access | Prose detector; optional role-keyed accessor | `PropertyValue.data` inaccessible; only `raw_lane()` greppable escape |
| RF channel identity cannot be transposed | Runtime arg-order check; bare `Option<String>` fields | `OwnerRef` / `ResourceKey` / `ScopeId` / `ParentLocationId` distinct newtypes; transposition uncompilable |
| Tick/runtime view carries no `kind` | Production `match kind` / `.kind` reads in sim; grep gate | Kind-free view types; kind access in tick path is uncompilable |
| `simthing-sim` public surface is semantic-free | Scattered semantic-free source scans | Public surface compile_fail; narrowed audit covers only declared public modules |
| `SlotIndex` and `ColumnIndex` are not interchangeable | `type ColumnIndex = usize` alias; bare integer indexing | Private-field newtypes; old alias deleted; `owner_ref_rejects_spatial_parent` compile_fail |
| Render coords cannot enter structural paths | Public-field `StructuralGridCoordinate`; convention only | Private-field `StructuralCoord`; render float conversion is explicit and named |
| Hot tick path is isolated from planning state | Session-loop direct dispatch; planning state accessible | `SimulationFabric` type; field access outside boundary is uncompilable |
| Structural GPU upload consumes validated packed data | Free `(frame, locations, links)` argument bundle | `PackedUpload` with count validation at pack time; free bundle argument is uncompilable |
| Session GPU upload consumes packed packets | `&[AccumulatorOp]` / `&[ThresholdRegistration]` / `&[IntentDelta]` / `&[AccumulatorOpGpu]` at session boundary | `PackedAccumulatorUpload` / `PackedThresholdUpload` / `PackedIntentUpload`; free slice upload is uncompilable |

## Net-negative enforcement ledger

Enforcement surface retired or narrowed across the track:

| Retired / narrowed | Replaced by | Rung |
|---|---|---|
| `data[N]` source scan as primary gate | `PropertyValue.data` inaccessibility | AS-1 |
| `StructuralGridCoordinate` public-field literal seam | Private-field `StructuralCoord` | AS-6 |
| Direct session-loop GPU dispatch | `SimulationFabric` type | AS-7 |
| `is_capability_container(&child.kind, …)` production branch | Marker property columns | AS-3 |
| Per-slice "no kind in tick" prose restatements as primary enforcement | Production audit + type boundary | AS-3 |
| Broad scattered semantic-free source scans | Narrowed to declared-public-module audit + compile_fail | AS-4 |
| Arg-order / mis-binding validation for RF channels | Channel newtypes; transposition uncompilable | AS-2 |
| Raw-string transposition grep for channel identity | Same | AS-2 |
| Old `type ColumnIndex = usize` alias (**deleted**) | Private-field `ColumnIndex(usize)` newtype | AS-5 |
| Free `(frame, locations, links)` upload signature (**removed**) | `PackedUpload` | AS-8 |
| Free-slice session upload methods (**removed**: `upload_ops`, `upload_gpu_ops`, `upload_threshold_ops`, `upload_intent_ops`, `upload_ops_with_eml`, `upload_ops_resolving_input_lists`, `append_threshold_ops`, `write_op_buffer`) | `upload_packed_ops`, `upload_packed_threshold_ops`, `upload_packed_intent_ops`, `append_packed_threshold_ops`, `write_packed_op_buffer` | AS-8B |

**Net result:** the removals listed above are permanent deletions or renames; the type boundary is the only enforcement path remaining. Each rung closed with net-negative enforcement surface.

## Residue ledger

Honest residue: what remains outside the type system's reach by nature, not neglect.

| Residue | Why it stays | Mitigation / admission |
|---|---|---|
| **WGSL shader text** | Rust type system cannot inspect shader strings; the upload seam is sealed but the shader itself is a string until compilation | CPU-oracle parity tests; semantic-free shader scan; byte-exact parity proven at every GPU rung |
| **Live ontological conformance** | "Is this still one accumulate→reduce→threshold loop?" is a behavioral judgment about the running system, not a type fact | DA review per release; the constitution §2 remains the specification |
| **No-flattening / recursive structure** | Specified vs implemented recursive tree structure; types cannot prove a tier was not silently collapsed | Constitutional review + DA judgment (design_0_0_8_3.md §0.6) |
| **No-CPU-planner boundary-time** | Planning *across* ticks at boundary-time is behavior-over-time; types enforce the hot-path slice (AS-7) but cannot see multi-tick temporal ordering | AS-7 seals the hot path; boundary-time planning reviewed constitutionally |
| **`WorldAccumulatorRuntime` envelope** | Runtime envelope still accepts semantic slices CPU-side; it packs before calling the sealed session API — correct per §4 ("compile away before upload") | Session seam is sealed; runtime envelope is the pack site, not an upload bypass |
| **AS-9 write / emission / participation seals** | Upload seal is complete (AS-8/8B); the "one authoritative path" directive at write / emission / participation level is a distinct future optional track | Named as AS-9+ in handoff-spine; not a gap in AS-F |

**The upload trust domain is sealed.** GPU upload residue is shader-text only after AS-8B. AS-9 (if opened) would address write-back / emission output / participation seals — a separate concern from the upload boundary.

## Documents consolidated

| Sub-rung / slice evidence doc | Consolidated into |
|---|---|
| `as_kind_out_of_tick_0a_results.md` through `0e_results.md` | `as_kind_out_of_tick_closeout_results.md` (pre-existing closeout ledger); cited from this doc |
| `as_sim_semantic_free_0a_results.md` through `0c_results.md` | `as_sim_semantic_free_closeout_results.md` (pre-existing closeout ledger); cited from this doc |
| `as_tick_fabric_boundary_0a_results.md` through `0c_results.md` | Summary in this doc's Scope Ledger |
| `as_index_newtypes_0_results.md` (0A + 0B) | Summary in this doc's Scope Ledger; original doc updated with DA-APPROVED status |
| `as_packed_upload_boundary_0_results.md` | Summary in this doc's Scope Ledger; original doc updated with DA-APPROVED status |
| `as_8b_session_upload_packed_packets_results.md` | Summary in this doc's Scope Ledger; original doc updated with DA-APPROVED status |

Sub-rung slice docs (AS-3 `0a`–`0e`, AS-4 `0a`–`0c`, AS-7 `0b`–`0c` — 10 files) are **expunged** at DA
closeout; provenance is retained in the evidence index + this ledger + the live `.rs` tests (the repo's
standard expunge pattern). Each split rung keeps exactly **one** ledger (AS-3/AS-4 their `*_closeout`;
AS-7 its `0a`). This is the net-negative consolidation AS-F exists to deliver.

## Known non-closeout follow-ons

These are not failures of AS-F; they are future optional work recorded so they are not re-derived.

- **AS-9: write-seal / emission-seal / participation-seal** — if a future track seals the write-back / emission output boundary with packed types, it uses the same AS-8/8B template. Not required by the constitution; an optional quality-of-enforcement improvement.
- **AS-9 intensity / velocity / reduction session envelopes** — `WorldAccumulatorRuntime` runtime envelope still packs CPU-side before calling the sealed session; a future rung could extend packed packets to the runtime envelope itself if needed.
- **Overlay-lifecycle typestate** — active vs Suspended; tabled in §5A of the design doc.
- **Arena settlement-phase typestate** — tabled in §5A.
- **AS-F doc consolidation** — DONE at DA closeout: 10 sub-rung slice docs **expunged** (net-negative); each split rung now has exactly one ledger; this doc is the track ledger.

## Conformance

- AS-F is PROBATION, not DA-APPROVED.
- Rungs 1–8B remain DONE — DA-APPROVED.
- No source code changes in this PR.
- No AS-9 work.
- No 0.0.8.5 work.
- Changed files: docs only (`docs/design_0_0_8_4_admission_substrate.md`, `docs/tests/current_evidence_index.md`, `docs/tests/as_closeout_0_results.md`, and status updates to AS-3/AS-4/AS-5/AS-8/AS-8B results docs).

## Hold point

DA final review required before AS-F → DONE — DA-APPROVED. After that, 0.0.8.5 ClauseScript Terran-Pirate galaxy track may proceed.
