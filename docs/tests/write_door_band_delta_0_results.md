# WRITE-DOOR-BAND-DELTA-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand-3 discharged locally; rides PR #1488 draft)
- HD-RECEIPT: `bfd7fd9c217b`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Board dispatch: comment `5111853299`
- Remand-1: comment `5112136052` (landing `5112304200`)
- Remand-2: comment `5117474928`
- Remand-3: comment `5117961061` (DA ruling `5117893595`)
- base_sha: `77ea7f12a933b5f0362afdaa4edf6970b4339ffc` (handoff)
- tested_code_sha / implementation_code_sha: bound after remedial commit (see PR Exact-head fields; this file does not self-hash tip)
- final_head_sha / clearance_pr_head: PR-body-bound only (this file does not self-hash)
- coverage_basis: plan_struct oracle-door arm cfg(test)-aware (filename exclusion class retired); plan_struct_typing_census exit 0; write-door census + focused 5.2 referees green; prior remands 1–2 retained
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## Remand-3 discharge (`5117961061`)

**Defect:** `scripts/ci/plan_struct_typing_census.sh` oracle-door arm false-positive on lawful `from_raw_for_oracle_or_rehearsal` inside `#[cfg(test)] mod tests` in `anchor_remap.rs` / `anchor_remap_encode.rs`.

**Fix (preferred option 1):** structural `cfg(test) mod tests` exclusion for the oracle-door arm; retired filename exclusions for `arena_allocation_plan.rs` / `child_share_eml.rs` / `emission_accumulator.rs` (those hits were test-module-only). Door definition + oracle/rehearsal module allowlist retained.

**Proof:**
- `bash scripts/ci/plan_struct_typing_census.sh` → process exit **0**
- `bash scripts/ci/write_door_band_delta_census.sh` PASS
- `cargo test -p simthing-core anchor_remap` 8/0
- `cargo test -p simthing-sim anchor_remap_encode` 4/0
- `cargo test -p simthing-driver --test write_door_band_delta_0` 10/0

Accepted 5.2 production code and fences intact. No Phase 10.1 scan-wiring addendum. No merge / no `ANCHOR-TABLE-SURFACE-0` / no `/clearance` from coder lane.

## Remand-2 (`5117474928`) — retained

Independent pre/post remap completeness; inventory compile_fail drift; real multi-edge GPU → BoundaryDeltaEntry → replay.

## Remand-1 (`5112136052`) — retained

Exact pre/post remaps; typed registry-filtered `BandCrossingDelta`; sealed apply mint; boundary/replay transport.
