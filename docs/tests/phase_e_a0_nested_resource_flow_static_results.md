# Phase A-0 — Static Nested Resource Flow First Slice

**Date:** 2026-05-30  
**Base HEAD:** `3bda84077e346101f875ce0e787212b840d119fb` (post B-0-ACCEPT-0)  
**PR:** #358 (merged to `master`)  
**Class:** GpuVerifiedApproximate (leaf allocated_flow bit-exact in fixtures)  
**Verdict:** **PASS — implemented; pending Opus/design-authority review** (not accepted)

---

## B-0 / v7.8 state summary

- **Line B/T:** B-0 ACCEPTED; hard-currency ordering closed at narrow smoke level  
- **Line C/M:** C-0/C-1/C-2 ACCEPTED; map batching closed at designer surface  
- **Line A/E:** NamedScenarioAccepted; **A-0 implemented** (this report)  
- **L3 / FrontierV2-5 / ACT-EVENT-OBS-PIPE:** parked/rejected  

---

## Resource Flow ADR summary

[`docs/adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md): hard-currency stays on Phase T discrete transfer path; Resource Flow is continuous hierarchical allocation via existing AccumulatorOp OrderBand sweeps; opt-in only; no default-on.

---

## E-11B readiness summary

[`docs/reviews/e11b_nested_hierarchy_gpu_readiness_review.md`](../reviews/e11b_nested_hierarchy_gpu_readiness_review.md): depth-generic band layout (`3·D−1`), SlotRange upsweep, child_share disburse, governed integration — all on existing accumulator execute pipeline. [`docs/reviews/e11b_nested_dynamic_enrollment_readiness.md`](../reviews/e11b_nested_dynamic_enrollment_readiness.md): dynamic enrollment (E-11B-5) explicitly deferred.

A-0 packages existing E-11B substrate into the v7.8 Line A first-slice proof with authored nested materialization fixtures and session-path smoke.

---

## Pre-edit audit

| # | Question | Answer |
|---|---|---|
| 1 | Accepted Line A scenario | `NestedResourceFlowDepthFanout` (depth 4) — V7.8-MET-SCENARIO-ACCEPT-0 |
| 2 | Already present from flat-star E-11 | `plan_arena_allocation`, `run_arena_allocation_oracle`, `sync_resource_flow_accumulator`, OrderBand GPU encode |
| 3 | Static nested tests already | `e11b_nested_hierarchy_gpu`, `e11b_nested_materialization`, `e11b_nested_fission_gap` |
| 4 | Production path forces flat-star | `use_accumulator_resource_flow` default false; flat-star opt-in validation; no nested profile default-on |
| 5 | Materialization policy | `materialize_arena_participants` + `parent_subtree_root_id` builds SimThing subtree; `build_execution_plan` selects `build_nested_layout` when nested participants present |
| 6 | Per-parent contiguity | `HierarchyNode::verify_child_contiguity`; non-contiguous → `HierarchyError::NonContiguousChildren` |
| 7 | D=3/D=4 OrderBand budget | `ArenaBandLayout::for_depth(D)` → D=3: 8 bands, D=4: 11 bands |
| 8 | CPU oracle | `run_arena_allocation_oracle` (test-only) |
| 9 | GPU path | `plan_arena_allocation` → `sync_resource_flow_ops_from_cpu` → `run_resource_flow_bands` |
| 10 | Not dynamic enrollment | No E-11B-5, no Policy B, no selector rerun, no gap auto-promotion |
| 11 | Not default-on RF | `PipelineFlags::default().use_accumulator_resource_flow == false` preserved |
| 12 | Not hard-currency | Phase T transfer path separate; A-0 tests assert no economy registry |

---

## Files changed

| File | Change |
|---|---|
| `crates/simthing-driver/src/arena_hierarchy.rs` | `NestedHierarchyMaterializationReport` + builder |
| `crates/simthing-driver/src/lib.rs` | Export materialization report |
| `crates/simthing-driver/tests/support/e11_nested.rs` | **New** A-0 nested fixtures + GPU parity helpers |
| `crates/simthing-driver/tests/phase_e_a0_nested_resource_flow_static.rs` | **New** A-0 proof tests (19) |
| Docs | production track, design_v7_8 note, mapping, sead, worklog |

---

## Implementation summary

A-0 adds formal v7.8 evidence wiring atop landed E-11B substrate:

- Authored nested participants (`ExplicitParticipantSpec::nested` + `parent_subtree_root_id`) materialize static D=3/D=4 trees  
- `build_execution_plan` → `build_nested_layout` when nested topology detected  
- Deterministic materialization report via `nested_hierarchy_materialization_report`  
- GPU/CPU oracle parity through existing OrderBand resource-flow bands (no new WGSL)

---

## Nested materialization policy

Static only: all participants declared in `ResourceFlowSpec` before session install. Interior parents receive contiguous direct-child SlotRanges. Reserved-gap slots excluded from active SlotRanges. No slot compaction, no indirection lists.

---

## D=3 fixture summary

Handoff topology (miniature):

```text
FactionRoot → Planet_A/B → Factory_A1/A2/B1/B2
```

7 explicit participants, `max_depth = 3`, 8 OrderBands, 4 leaves.

---

## D=4 fixture summary

Proven E-11B depth-4 branch (one depth-4 chain + second shallow root):

```text
Root₀ → … → leaf (depth 4); Root₁ → child
```

7 explicit participants, `max_depth = 4`, 11 OrderBands.

---

## Contiguity proof

`a0_nested_children_contiguous_per_parent` + materialization report `all_parents_contiguous = true` for D=3/D=4.

---

## Reserved-gap exclusion proof

Gap pools reserved per interior parent; `nested_fission_gap_report` confirms gaps outside active child span; gap-claimed fission children excluded from allocation leaves.

---

## CPU oracle summary

`run_arena_allocation_oracle` with deterministic intrinsic/weight inputs; same upsweep/downsweep semantics as GPU planner.

---

## GPU path summary

Existing AccumulatorOp v2 execute pipeline: `plan_arena_allocation` → encoded ops → `WorldGpuState::run_resource_flow_bands(n_bands, dt)`.

---

## Parity / tolerance table

| Fixture | max_abs_error | l_inf | Classification |
|---|---|---|---|
| D=3 | 0.0 | 0.0 | GpuVerifiedApproximate (bit-exact leaves) |
| D=4 | 0.0 | 0.0 | GpuVerifiedApproximate (bit-exact leaves) |

---

## Replay result

`a0_replay_reproducibility`: two D=3 parity runs — identical `max_abs_error` and `l_inf` (0.0).

---

## Safety behavior matrix

| Case | Result |
|---|---|
| `use_accumulator_resource_flow` default | **false** (unchanged) |
| Nested RF execution | explicit opt-in only in tests |
| Hard-currency | not routed through RF |
| Dynamic enrollment | not implemented |
| Policy B / selector rerun / compaction | not present |
| simthing-sim awareness | none in lib.rs |

---

## Guardrail scans

Resource Flow remains opt-in; no hard-currency substitution; no new WGSL in A-0 diff; B-1 / Line C runtime / L3 / FrontierV2-5 / ACT-EVENT-OBS-PIPE not opened.

---

## Test results

```text
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture
→ 19 passed; 0 failed

cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
→ 11 passed; 1 failed (e11b_nested_no_new_wgsl — pre-existing: atlas_mask.wgsl from C-0 not in E-11B whitelist)

cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
→ 12 passed; 1 failed (same atlas_mask whitelist — pre-existing)

cargo test -p simthing-spec --test v7_8_met_consumer_scenarios -- --nocapture
→ 10 passed; 0 failed

cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture
→ 13 passed; 0 failed

cargo check --workspace
→ Finished (green)
```

---

## Transient cleanup

No scratch/tmp/log artifacts under `docs/tests/` requiring deletion.

---

## Final verdict

**PASS — A-0 landed the static nested Resource Flow first slice:** nested arena materialization for D=3/D=4, per-parent contiguous SlotRange enforcement, reserved-gap exclusion, and GPU/CPU oracle parity over existing AccumulatorOp OrderBand execution. Resource Flow remains opt-in/default-off; hard-currency remains on Phase T; no dynamic enrollment, Policy B, selector rerun, slot compaction, new WGSL, new AccumulatorRole, CPU fallback, simthing-sim awareness, B-1, Line C runtime, L3, FrontierV2-5, or ACT/EVENT/OBS/PIPE expansion was added. **A-0 is implemented and pending Opus/design-authority review.**
