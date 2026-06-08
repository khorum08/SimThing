# Phase M First-Slice Summary Validity V1 — Test Results

Date: 2026-05-28

## Base

- Base HEAD: `47b3685b4b19ac3f74afb81b35b25b76e95a2545` (Opus/product vertical-proof acceptance on master)
- Final commit SHA: recorded at merge (branch `phase-m-summary-validity-v1`)

## Files Changed

- `crates/simthing-spec/src/spec/region_field.rs` — `RegionFieldSummaryPolicySpec`, `RegionFieldSummaryStatus`, `RegionFieldSpec.summary_policy`
- `crates/simthing-spec/src/compile/region_field_admission.rs` — summary policy admission/compile
- `crates/simthing-spec/src/lib.rs`, `spec/mod.rs`, `compile/mod.rs` — exports
- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — summary metadata + `FirstSliceSummaryReport`
- `crates/simthing-driver/src/lib.rs` — export
- `crates/simthing-driver/tests/phase_m_first_slice_summary_validity.rs` — 8-test suite
- `crates/simthing-driver/tests/fixtures/first_slice_product_summary_validity_scenario.ron`
- `crates/simthing-spec/tests/region_field_spec_admission.rs` — struct literal fix
- `crates/simthing-driver/tests/phase_m_first_slice_runtime.rs` — struct literal fix
- Docs: production plan, mapping guidance, workshop state, todo, worklog, ADR landing note

## Summary Policy / Status Design

| Layer | Type | Role |
|---|---|---|
| Spec | `RegionFieldSummaryPolicySpec::CachedUntilDirtyWithZeroInitial` (default) | Designer-facing; admits through `RegionFieldSpec.summary_policy` |
| Compiled | `CompiledRegionFieldSummaryPolicy` | Carried on `CompiledRegionFieldPreview` |
| Runtime status | `RegionFieldSummaryStatus` | Metadata only: `FreshThisTick`, `Cached { age_ticks }`, `ZeroInitial`, `InvalidOrUnavailable` |
| Report | `FirstSliceSummaryReport` | `policy`, `status`, `age_ticks`, `has_gpu_parent_summary`, `last_fresh_tick`, `summary_used_for_commitment_scan` |

**Lifecycle (EveryN cadence fixture):**

1. Seeded execute → `FreshThisTick`, age 0, GPU parent summary updated
2. Clean skip → `Cached`, age increments, no stencil/reduction/EML rerun
3. New seed → dirty execute → `FreshThisTick`, age resets

**Cached commitment scan:** deferred — runs only when dense path executes (`scheduled && eml_executed`); cached ticks emit no threshold event and report `summary_used_for_commitment_scan = false`.

## Commands Run

| Command | Result |
|---|---|
| `git status --short` | PASS |
| `git rev-parse HEAD` | PASS; `47b3685b4b19ac3f74afb81b35b25b76e95a2545` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture` | PASS; 8/8 |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 2/2 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

No Rust test assertion failures. No GPU/device loss or cargo artifact format errors observed.

Full log: [`phase_m_first_slice_summary_validity_full.log`](phase_m_first_slice_summary_validity_full.log)

## Pass/Fail Table

| Test | Result |
|---|---|
| 1 — summary policy RON/admission | PASS |
| 2 — zero-initial skip before execution | PASS |
| 3 — fresh summary after executed tick | PASS |
| 4 — cached summary on skipped clean tick | PASS |
| 5 — dirty seed invalidates cached and refreshes | PASS |
| 6 — cached summary does not CPU-emit event | PASS |
| 7 — deterministic replay | PASS |
| 8 — posture preservation | PASS |

## Fresh / Cached / Zero-Initial Sequence

- **Zero-initial:** `OnEvent` cadence, no seed → `ZeroInitial`, 0 dispatches, no fake reduction/EML values
- **Fresh:** seeded `EveryN(100)` execute → 9 dispatches, `FreshThisTick`, `has_gpu_parent_summary = true`
- **Cached:** two clean ticks → `Cached` ages 1 and 2, 0 stencil dispatches, hot path `field_values/reduction_parent_value/eml_output == None`
- **Refresh:** new seed → execute, `FreshThisTick`, age 0

## GPU-Resident Hot-Path Summary

- Skipped clean ticks retain GPU parent summary buffers; no CPU threat/urgency rederivation
- `reduction_stencil_readbacks == 0` on hot path across fresh and cached ticks
- Summary status is metadata only on CPU

## Event Behavior on Cached Summary

- Cached ticks: no threshold event; `summary_used_for_commitment_scan = false`
- Commitment scan deferred on cached summary (GPU-substrate scan on retained parent urgency not wired for skip path in V1)

## Posture Summary

Phase M SummaryValidity V1 landed.
It adds a bounded first-slice summary validity policy/status so a clean or skipped RegionField can report whether its strategic parent summary is fresh, cached, zero-initial, or unavailable without rerunning dense field propagation or rederiving gameplay state on CPU.
The hot path remains GPU-resident; cached summaries retain GPU-resident parent summary values and report metadata only.
No CPU-side AI planner was introduced.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception, map residency system, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Known Caveat

Queue-write scale caveat remains unresolved. Before any multi-field, multi-map, atlas, or broader production scaling, replace per-slot resource/weight queue writes with a measured GPU-resident mechanism such as a preinitialized resource column, generic fill helper, or GPU fill kernel.

Cached commitment threshold scan over retained GPU parent urgency is explicitly deferred in V1.

## Final Verdict

PASS — Phase M SummaryValidity V1 landed; clean/skipped first-slice RegionFields now expose bounded summary validity metadata while preserving GPU-resident parent summaries, default-off execution, simthing-sim map-freedom, and no atlas, semantic WGSL, source_mask, perception, map residency system, or CPU-side AI planning.
