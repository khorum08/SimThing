# WRITE-DOOR-BAND-DELTA-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand-1 discharged)
- HD-RECEIPT: `bfd7fd9c217b`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Board dispatch: comment `5111853299`
- Remand: comment `5112136052`
- base_sha: `77ea7f12a933b5f0362afdaa4edf6970b4339ffc` (handoff); branch cut from current master including dispatch tip
- tested_code_sha: `b71f6f3c06fff0d7eea41824c7fcc7b8822c60b8`
- implementation_code_sha: `b71f6f3c06fff0d7eea41824c7fcc7b8822c60b8`
- final_head_sha: PR-body-bound (see draft PR `final_head_sha` after push; this file does not self-hash)
- clearance_pr_head: PR-body-bound (see draft PR `clearance_pr_head` after push; this file does not self-hash)
- coverage_basis: exact pre/post remaps; typed registry-filtered BandCrossingDelta; boundary/delta-log/replay transport; strengthened referees + census; fused GPU mint vs CPU oracle; adapter-pinned batteries
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: PR-body-bound after hosted clearance

## Remand-1 discharge (`5112136052`)

1. **Exact structural remaps** — pre-mutation `snapshot_anchored_loci` + post snapshot → `derive_exact_anchor_remaps`; retire uses pre slot/col (never `SlotIndex(0)` fallback); dimension/capacity growth can emit identity moves from pre→post; encode gate verifies endpoints + rejects duplicates.
2. **Typed band deltas / no public readback** — `BandCrossingDelta` binds `SimThingId` / `SimPropertyId` / `SubFieldRole` / `SlotIndex` / `ColumnIndex`; Anchored eligibility from canonical registry; public `readback_band_crossing_deltas` removed; sealed `apply_band_crossing_deltas_from_*` mint doors; CPU oracle remains proof-only.
3. **Boundary/replay transport** — `BoundaryDeltaEntry::BandCrossingDeltasApplied` + `AnchorRemapApplied`; `ReplayDriver` retains both for bit-exact compare.
4. **Stronger referees/census** — retire-from-nonzero, column-shift, wrong-endpoint + duplicate negatives, Unobserved without caller filter, multi-edge GPU parity, e2e replay; census forbids public band-delta readback and fabricated remap endpoints.

## Fences held

- No `ANCHOR-TABLE-SURFACE-0` table / consumer migration
- No FIELD-SWEEP 5.4–5.8
- No CPU decision branching on observed values; no second threshold authority
- No band edges in accumulation/falloff/conservation
- No new raw identity doors outside `wgsl_encode` / GPU round-trip rematerialize
- Draft PR only; no merge; no next-rung dispatch; no `/clearance` from coder lane
