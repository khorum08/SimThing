# Phase M SummaryValidity V1-R1 — Runtime Status Layer Hygiene Test Results

**Date:** 2026-05-29
**Base HEAD:** `0e603ee119f30ce286dc356429495d18b0282bd8` (Phase M SummaryValidity V1 merge)
**Agent:** Grok (completing Cursor remedial handoff)

## Summary

This PR performs the narrow hygiene fix identified in the prior audit: moving the runtime summary status enum out of `simthing-spec` into the driver runtime reporting layer.

## Files Changed (core)

- `crates/simthing-driver/src/first_slice_mapping_runtime.rs`
- `crates/simthing-driver/src/lib.rs`
- `crates/simthing-driver/tests/phase_m_first_slice_summary_validity.rs`

## Boundary Fix Summary

- Renamed runtime enum `RegionFieldSummaryStatus` → `FirstSliceSummaryStatus` for proper scoping.
- Moved ownership fully into `simthing-driver`.
- Removed all references and re-exports of the runtime status enum from `simthing-spec`.
- `RegionFieldSummaryPolicySpec` and `CompiledRegionFieldSummaryPolicy` remain in spec (designer + admission layer) — correct.
- All SummaryValidity V1 behavior preserved exactly.

## Commands Run

```bash
cargo test -p simthing-driver --test phase_m_first_slice_summary_validity -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec -- --quiet
cargo check --workspace
```

## Pass/Fail Table

| Test Suite                                      | Result     | Notes |
|------------------------------------------------|------------|-------|
| `phase_m_first_slice_summary_validity`         | **PASS** (11/11) | All SummaryValidity V1 + hygiene tests pass |
| `phase_m_first_slice_scenario_spec`            | **PASS** (9/9)   | Prior FirstSliceScenarioSpec work unaffected |
| `cargo check --workspace`                      | **PASS**         | Clean (pre-existing warnings only) |

## Boundary Confirmation

- `simthing-spec` no longer defines `RegionFieldSummaryStatus` / `FirstSliceSummaryStatus`
- `simthing-spec` no longer re-exports any runtime summary status
- `FirstSliceSummaryStatus` lives in `simthing-driver/src/first_slice_mapping_runtime.rs`
- `RegionFieldSummaryPolicySpec` remains in spec (designer-facing)
- `CompiledRegionFieldSummaryPolicy` remains in the compile/admission path

## Posture Summary

All required posture preserved:

- V7.7 Mapping ADR approved posture maintained.
- Phase M first-slice vertical proof acceptance maintained.
- `MappingExecutionProfile` default remains `Disabled`.
- Spec presence alone does not execute mapping.
- `simthing-sim` remains map-free.
- No default SimSession pass-graph wiring.
- No atlas batching / M-4A / active mask / perception / source_mask / semantic WGSL introduced.
- No CPU-side AI planner or gameplay recomputation introduced.

## Known Caveat

Queue-write scale caveat (per-slot resource/weight writes on the 10x10 bridge) remains unresolved, as documented in prior Phase M work.

## Final Verdict

**PASS** — Phase M SummaryValidity V1-R1 layer hygiene landed.

Runtime summary status has been moved from `simthing-spec` into the driver/runtime reporting layer (`FirstSliceSummaryStatus`), while designer-facing summary policy remains correctly in the spec admission layer.

All SummaryValidity V1 behavior is preserved. All required prior first-slice tests continue to pass. All V7.7 / Mapping ADR / FIELD_POLICY guardrails respected.

No scope expansion occurred.