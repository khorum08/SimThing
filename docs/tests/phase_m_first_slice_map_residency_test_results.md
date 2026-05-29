# Phase M Map Residency V1 — Test Results

Date: 2026-05-29

## Base

- Base HEAD: `e9ce11125459e88be29d59dac227e761f564fe64` (Queue-Write Scale Hardening V1)
- Final commit SHA: recorded at merge

## Files Changed

- `crates/simthing-driver/src/first_slice_mapping_runtime.rs` — `FirstSliceResidencyStatus`, `FirstSliceResidencyReport`, `build_residency_report`
- `crates/simthing-driver/src/lib.rs` — exports
- `crates/simthing-driver/tests/phase_m_first_slice_map_residency.rs` — 7-test suite
- Docs: production plan, mapping guidance, workshop state, todo, worklog, ADR landing note

## Residency Status Design

| Status | When | Summary status |
|---|---|---|
| `DisabledUnavailable` | `MappingExecutionProfile::Disabled` | `InvalidOrUnavailable` |
| `ColdSkipped` | Enabled, skipped, no prior GPU parent summary | `ZeroInitial` |
| `HotExecutedThisTick` | Dense field executed this tick | `FreshThisTick` |
| `ResidentCached` | Skipped after prior execution | `Cached { age_ticks }` |

**Report fields:** `summary_visible_to_parent`, `dense_field_executed`, `parent_summary_retained_on_gpu`, `cached_commitment_scan_supported` (false in V1).

No new RON field — V1 derives residency from existing SummaryValidity GPU-parent-summary state.

## Commands Run

| Command | Result |
|---|---|
| `git rev-parse HEAD` | PASS; `e9ce11125459e88be29d59dac227e761f564fe64` |
| `rustc --version` | PASS; `rustc 1.95.0 (59807616e 2026-04-14)` |
| `cargo --version` | PASS; `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| `cargo test -p simthing-driver --test phase_m_first_slice_map_residency -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_queue_write_hardening -- --nocapture` | PASS; 4/4 |
| `cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --nocapture` | PASS; 9/9 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_product_fixture -- --nocapture` | PASS; 7/7 |
| `cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture` | PASS; 28/28 |
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | PASS; 11/11 |
| `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture` | PASS; 3/3 |
| `cargo check --workspace` | PASS |
| `cargo test --workspace -j 1` | PASS (see full log) |

Full log: [`phase_m_first_slice_map_residency_full.log`](phase_m_first_slice_map_residency_full.log)

## Pass/Fail Table

| Test | Result |
|---|---|
| 1 — disabled profile unavailable | PASS |
| 2 — cold skipped before execution | PASS |
| 3 — hot executed tick | PASS |
| 4 — resident cached tick | PASS |
| 5 — dirty refresh from cached | PASS |
| 6 — deterministic replay | PASS |
| 7 — posture preservation | PASS |

## Cold / Hot / Resident-Cached / Refresh Sequence

1. OnEvent, no seed → `ColdSkipped` / `ZeroInitial`
2. Seeded execute → `HotExecutedThisTick` / `FreshThisTick`, bulk fill 1, dispatches 9
3. Clean skip → `ResidentCached` / `Cached age 1`, 0 dispatches, no commitment scan
4. New seed → `HotExecutedThisTick` / `FreshThisTick`, bulk fill resumes

## Hot-Path GPU-Residency Summary

- `reduction_stencil_readbacks == 0` preserved
- ResidentCached retains GPU parent summary; no CPU threat/urgency rederivation
- Bulk fill hardening intact on hot ticks

## Cached Event Behavior

- Cached ticks: no threshold events; `summary_used_for_commitment_scan = false`; `cached_commitment_scan_supported = false`

## Posture Summary

Phase M Map Residency V1 landed.
It adds first-slice residency status/reporting over the accepted GPU-resident path: HotExecutedThisTick, ResidentCached, ColdSkipped, and DisabledUnavailable.
Residency status is metadata only. CPU does not recompute threat/urgency, emit commitment events, or mutate true field values for cached/skipped maps.
ResidentCached preserves visibility of prior GPU parent summaries through metadata while cached commitment scans remain deferred in V1.
No SummaryValidity behavior changed.
No default SimSession wiring was introduced.
No atlas batching landed.
No M-4A atlas masking landed.
No active mask, perception/fog, behavioral source policy, or source_mask landed.
No semantic WGSL landed.
simthing-sim remains map-free.
Defaults unchanged.

## Remaining Caveat

Queue-write child resource scale caveat addressed for first-slice by generic bulk fill. Parent scalar writes remain O(1). Multi-field/atlas scaling remains separately gated. Cached commitment threshold scan over retained GPU parent urgency remains deferred in V1.

## Final Verdict

PASS — Phase M Map Residency V1 landed; first-slice RegionFields now report bounded residency state while preserving GPU-resident summaries, default-off execution, simthing-sim map-freedom, and no atlas, semantic WGSL, source_mask, perception, default SimSession wiring, or CPU-side gameplay planning.
