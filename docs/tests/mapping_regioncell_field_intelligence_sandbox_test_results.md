# Sparse RegionCell field-intelligence sandbox — test results

**Date/time:** 2026-05-27 21:05:07 -05:00  
**Base HEAD (before commit):** `2bba71129b45bbf2498c507060a4669a76691b25`  
**Final commit SHA:** `9af12a9`  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — E-11B nested hierarchy GPU parity tests passed (12/12). Sandbox pressure projection is CPU-side EML-equivalent math; no new GPU EML path required.

---

## Commands

| Command | Result |
|---------|--------|
| `git status --short` | PASS — new sandbox test + docs only (unrelated workshop report churn ignored) |
| `git rev-parse HEAD` | PASS — `2bba711` (pre-commit) |
| `rustc --version` | PASS |
| `cargo --version` | PASS |
| `cargo test -p simthing-driver --test mapping_regioncell_field_intelligence_sandbox -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |

---

## Sandbox test excerpts

```
running 10 tests
test regioncell_sandbox_static_10x10_materializes ... ok
test regioncell_sandbox_durable_fields_are_narrow ... ok
test regioncell_sandbox_attack_pressure_changes_with_aggression_weight ... ok
test regioncell_sandbox_defense_pressure_changes_with_caution_or_stability_weight ... ok
test regioncell_sandbox_hotspot_crosses_attack_threshold ... ok
test regioncell_sandbox_same_cell_state_different_personality_different_heatmap ... ok
test regioncell_sandbox_no_cpu_ai_decision_loop ... ok
test regioncell_sandbox_no_mapping_runtime_primitives ... ok
test regioncell_sandbox_no_new_wgsl ... ok
test regioncell_sandbox_global_resource_flow_flag_default_false ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

**Acceptance behavior (hotspot cell 55):**

| Personality | `attack_pressure` | `defense_pressure` | `attack_intent` (threshold ≥ 1.0) |
|-------------|-------------------|--------------------|-----------------------------------|
| defensive (`aggression=0.2`) | ~0.0 | dominates attack | **false** |
| aggressive (`aggression=0.9`) | ~1.36 | below attack | **true** |

**Materialization:** FactionRoot + 100 RegionCell participants (10×10) via `ExplicitParticipantSpec::nested` + `parent_subtree_root_id`; `build_nested_layout` max_depth=2, 100 contiguous child slots.

**Durable fields (sandbox only):** `presence`, `threat`, `opportunity`, `supply`, `control` — attack/defense/expansion pressure derived via EML-equivalent formulas, not stored.

---

## Posture preserved

- No mapping runtime, location schema, Scatter/Gather, wavefront, dynamic nested enrollment, Policy B, D-2a, WGSL, new AccumulatorRole variants, CPU fallback, or slot compaction.
- FlatStarResourceFlow remains bounded production posture.
- Static deep hierarchy authoring via `parent_subtree_root_id` remains available.
- `simthing-sim` unchanged (arena-ignorant, spec-free).
- `PipelineFlags::default().use_accumulator_resource_flow` remains **false**.
- Presence of `ResourceFlowSpec` alone does not enable GPU execution.

---

## Final verdict

**PASS** — Static 10×10 RegionCell hierarchy materializes; narrow durable fields; personality-weighted EML-equivalent pressure projections produce different heatmaps and hotspot threshold crossing without CPU AI decision loops or mapping runtime primitives.
