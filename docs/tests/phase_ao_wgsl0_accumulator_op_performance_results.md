# AO-WGSL-0 — Generic AccumulatorOp WGSL Performance Path Results

## Base HEAD

`4214116` (origin/master — WGSL-GUARD-R1) + AO-WGSL-0 implementation commit.

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-gpu/src/shaders/accumulator_op.wgsl` | Refactored band gating; added `execute_orderband_bands` entry (one band per dispatch) |
| `crates/simthing-gpu/src/shaders/accumulator_op_generic.wgsl` | **New** — AO-WGSL-0 gate contract marker (kernel lives in `accumulator_op.wgsl`) |
| `crates/simthing-gpu/src/accumulator_op/wgsl_path.rs` | **New** — plan-shape classification and fast-path eligibility |
| `crates/simthing-gpu/src/accumulator_op/session.rs` | `orderband_fast_pipeline`, `encode_orderband_fast_into` |
| `crates/simthing-gpu/src/accumulator_op/runtime.rs` | Cache uploaded ops for compatibility checks |
| `crates/simthing-gpu/src/accumulator_op/mod.rs` | Wire `wgsl_path` module |
| `crates/simthing-gpu/src/world_state.rs` | `run_resource_flow_bands_with_fast_path` |
| `crates/simthing-gpu/src/lib.rs` | Export AO-WGSL-0 symbols |
| `crates/simthing-sim/src/boundary.rs` | `PipelineFlags::use_accumulator_wgsl_fast_path` (default **false**) |
| `crates/simthing-driver/tests/phase_ao_wgsl0_accumulator_op_performance.rs` | **New** — parity, guard, benchmark tests |
| `crates/simthing-driver/tests/phase_e_a0_nested_resource_flow_static.rs` | Remove stale `assert_no_new_wgsl` (WGSL-GUARD-R1 follow-up) |
| `crates/simthing-spec/tests/c2_atlas_admission_relaxation.rs` | Fix `report.admitted` → `report.accepted` |
| `docs/design_v7_8_production_track.md` | Generic GPU performance gates row |
| `docs/design_v7_8.md` | Compact AO-WGSL-0 doctrine note |
| `docs/workshop/mapping_current_guidance.md` | Status row |
| `docs/workshop/sead_self_ai_track.md` | Posture note |
| `docs/worklog.md` | Append entry |

## Why this gate is authorized after WGSL-GUARD-0/R1

WGSL-GUARD-0/R1 removed the stale global filename-based WGSL ban and restored the v7.8 doctrine: **generic GPU-resident WGSL is admissible through named production gates**; **semantic/raw WGSL from designer/spec admission remains rejected**. AO-WGSL-0 is an explicitly named generic AccumulatorOp performance gate. Shader source is static project code under `simthing-gpu/src/shaders/`; no scenario-authored WGSL, no ClauseSpec/RON shader generation.

## AO-WGSL-0 design decision

**Option 3 — feature-gated fast path selected when the AO plan shape is compatible; fallback remains the existing accepted path.**

- `PipelineFlags::use_accumulator_wgsl_fast_path` (default off) selects the fast path at runtime.
- `classify_ao_wgsl0_plan` / `ao_wgsl0_fast_path_compatible` gate eligibility before dispatch.
- The fast path uses WGSL entry `execute_orderband_bands` with **one band per dispatch** inside a single compute pass (preserves global OrderBand ordering; does not fuse bands in-shader).
- Unsupported shapes (threshold, affine intent, crossing formula, product, last-by-priority, empty plan) fall back to legacy per-band `encode_orderband_with_eml_into`.

## Supported AO plan shapes

| Shape | Fixture | Classification |
|---|---|---|
| Flat-star E-11 allocation | `ao_wgsl0_generic_kernel_matches_existing_ao_for_flat_star` | `FlatStarOrderBand` |
| A-0 D=3 nested Resource Flow | `ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d3_nested_resource_flow` | `ResourceFlowOrderBand` |
| A-0 D=4 nested Resource Flow | `ao_wgsl0_generic_kernel_matches_existing_ao_for_a0_d4_nested_resource_flow` | `ResourceFlowOrderBand` |
| B-0 transfer OrderBand | `ao_wgsl0_generic_kernel_matches_existing_ao_for_b0_transfer_orderband_if_supported` | `TransferOrderBand` |

## Unsupported / fallback plan shapes

| Reason | Behavior |
|---|---|
| `ThresholdOps` | `ao_wgsl0_fast_path_compatible` → false; legacy path only |
| `AffineIntentOps`, `CrossingFormulaOps`, `ProductOps`, `LastByPriorityOps` | Fallback |
| `EmptyPlan` | Fallback |
| Unsupported gate/combine kinds | Fallback |

Verified in `ao_wgsl0_unsupported_plan_falls_back_or_rejects_without_semantics_change`.

## Shader summary

- **Authoritative kernel:** `accumulator_op.wgsl` — entry `execute_orderband_bands` (shared helpers with `execute_ops`).
- **Contract marker:** `accumulator_op_generic.wgsl` — documents AO-WGSL-0 gate; no separate compiled kernel.
- **Allowed concepts:** slot/dimension/band indices, role ids, combine/gate/consume mode ids, buffers, strides, clamps, weights, masks, reductions.
- **Forbidden concepts:** faction, planet, star, map, AI intent, economy meaning, ClauseThing/ClauseScript/scenario semantics.

## Proof shader is generic / semantic-free

Static scan of `accumulator_op.wgsl` and `accumulator_op_generic.wgsl` — no forbidden semantic tokens (`faction`, `planet`, `ClauseThing`, `ClauseScript`, `scenario`). Test `ao_wgsl0_no_designer_authored_wgsl` asserts this at compile time.

## CPU/oracle parity table

| Fixture | Classification | max_abs_error | l_inf | conservation residual | Notes |
|---|---|---:|---:|---:|---|
| A-0 D=3 nested RF | GpuVerifiedApproximate | 0.0 | 0.0 | 0.0 (leaf bit-exact) | CPU oracle vs fast path |
| A-0 D=4 nested RF | GpuVerifiedApproximate | — | — | — | legacy vs fast buffer bit-exact |
| Flat-star | GpuVerifiedApproximate | — | — | — | legacy vs fast buffer equality |

## Existing-path parity table

| Fixture | Classification | Parity | Notes |
|---|---|---|---|
| Flat-star | GpuVerifiedApproximate | legacy == fast (full buffer) | |
| A-0 D=3 | GpuVerifiedApproximate | legacy == fast (leaf bits) | max_abs_error = 0 |
| A-0 D=4 | GpuVerifiedApproximate | legacy == fast (full buffer) | |
| B-0 transfer OrderBand | GpuVerifiedExact | legacy == fast (bit-exact) | treasury 3.0, sinks 3.0/4.0; conservation 0.0 |

## Replay result

`ao_wgsl0_replay_reproducibility`: two consecutive D=3 fast-path runs — identical `max_abs_error` and `l_inf` (0.0).

## Performance table

Bounded D=3 nested RF fixture (`ao_wgsl0_benchmark_report_smoke`). **Timings include queue sync in harness; noisy on small fixtures — not production-grade claims.**

| Metric | Legacy path | WGSL fast path |
|---|---:|---:|
| cold_time_us | 504 | 429 |
| warm_mean_us | 543 | 428 |
| warm_min_us | 458 | 370 |
| warm_max_us | 628 | 519 |
| speedup_ratio (mean) | — | **1.27** |
| dispatch_count | per-band legacy encodes | single pass, sequential band dispatches |
| buffer_sizes | D=3 nested arena (7 participants, 16 cap) | same |

GPU adapter: platform default wgpu adapter (not pinned in fixture).

## Semantic-WGSL guard confirmation

- `DesignerAdmissionRequest::SemanticWgsl` → `report.accepted == false` (`ao_wgsl0_semantic_wgsl_still_rejected_at_designer_layer`, `designer_admission_rejects_raw_wgsl_source` in c2_atlas).
- AO-WGSL-0 shader is static project code, not scenario-authored.
- No global filename whitelist restored.

## No downstream posture widening summary

- Resource Flow remains opt-in (`use_accumulator_resource_flow` default false).
- AO-WGSL-0 fast path default off.
- Hard-currency routing unchanged (transfer path separate).
- A-0, B-0, C-2 semantics unchanged.
- A-0 remains **pending Opus review** — not accepted.
- AO-WGSL-0 remains **pending Opus review** — not accepted.
- E-11B-5, Policy B, selector rerun, slot compaction, B-1, Line C runtime, L3, FrontierV2-5, ACT/EVENT/OBS/PIPE — not opened.

## Test results

```bash
cargo test -p simthing-driver --test phase_ao_wgsl0_accumulator_op_performance -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture
→ 19 passed; 0 failed

cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
→ 11 passed; 0 failed

cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test phase_t_b0_d2a_hard_currency_ordering -- --nocapture
→ 11 passed; 0 failed

cargo test -p simthing-spec --test c2_atlas_admission_relaxation -- --nocapture
→ 15 passed; 0 failed

cargo check --workspace
→ ok
```

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `AO-WGSL-0\|accumulator_op_generic\|SemanticWgsl` in crates/docs | Named gate + designer rejection | PASS — gate symbols in gpu/sim/driver/spec; SemanticWgsl rejection in designer_admission |
| `accepted_wgsl_baseline\|ACCEPTED_WGSL\|no_new_wgsl` in crates | No active filename ban | PASS — only historical test *names* (`*_no_new_wgsl`); no `accepted_wgsl_baseline.rs` module |
| `accepted_wgsl_baseline\|no_new_wgsl` in docs | Historical only | PASS — A-0-R1 / WGSL-GUARD reports only |
| Forbidden semantics in shaders/accumulator_op/tests | None except negative test refs | PASS |
| E-11B-5 / Policy B / default-on RF in report+track | Parked/rejected only | PASS |
| B-1 / C-runtime / M-6A in track | Parked/deferred only | PASS |
| ClauseThing/L3/FrontierV2-5/ACT in crates/docs | Parked/rejected only | PASS |
| ArenaSpec/ResourceFlowSpec in simthing-sim | No new semantic awareness | PASS — only `use_accumulator_wgsl_fast_path` flag comment in boundary.rs |

## Transient cleanup result

No `target/`, `*.replay.ldjson`, `.claude/worktrees/`, or scratch logs committed. Benchmark output captured in this report only.

## Final verdict

**PASS — AO-WGSL-0 landed a generic, semantic-free AccumulatorOp WGSL performance option under a named production gate.** It proves parity against CPU/oracle and existing accepted AO execution for flat-star, A-0 D=3/D=4, and B-0 transfer OrderBand shapes; reports bounded performance data; keeps designer-authored semantic/raw WGSL rejected at admission; does not restore stale filename bans; and does not alter A-0/B-0/C-2 semantics, Resource Flow defaults, hard-currency routing, simthing-sim awareness, L3, FrontierV2-5, or ACT/EVENT/OBS/PIPE posture. **Pending Opus review.**
