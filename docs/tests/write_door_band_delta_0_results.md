# WRITE-DOOR-BAND-DELTA-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand-2 discharged locally; rides PR #1488 draft)
- HD-RECEIPT: `bfd7fd9c217b`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Board dispatch: comment `5111853299`
- Remand-1: comment `5112136052` (landing `5112304200`)
- Remand-2: comment `5117474928`
- base_sha: `77ea7f12a933b5f0362afdaa4edf6970b4339ffc` (handoff)
- tested_code_sha / implementation_code_sha: `5dd354a5e72491219ae7c97d5056738e584cd9b1`
- final_head_sha / clearance_pr_head: PR-body-bound only (this file does not self-hash)
- coverage_basis: independent pre/post remap completeness; omitted-retire/move negatives; real multi-edge GPU mint → BoundaryDeltaEntry → replay; inventory drift closed; census independent-required-keys fence; adapter-pinned batteries
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## Remand-2 discharge (`5117474928`)

1. **TEST-INVENTORY-DRIFT** — ledgered `compile_fail_line_29` / `compile_fail_line_48` for `sealed/band_crossing_delta.rs`; drift check PASS (unledgered=0, stale=0).
2. **Independent fail-closed remap completeness** — `expected_anchored_remap_keys(pre, post, include_stable_identity)` seeds required keys; `validate_exact_anchor_remap_endpoints` rejects missing/extra/duplicate vs that set; production `gate_structural_gpu_encode_exact` uses the same; omitted-retire + omitted-move negatives in core + sim.
3. **Real multi-edge GPU transport** — `gpu_multi_edge_band_delta_boundary_replay_transport`: one Anchored cell, two ordered rising edges in one GPU threshold pass; sealed mint from GPU emissions; BoundaryDeltaEntry JSON → ReplayDriver bit-exact retention (count, reg_idx order, typed identities).
4. **PR body** — plain Markdown rebound on tip (no escaped-control corruption).

## Remand-1 (accepted; retained)

Exact pre/post remaps (no `SlotIndex(0)` fallback); typed registry-filtered `BandCrossingDelta`; public `readback_band_crossing_deltas` removed; sealed `apply_band_crossing_deltas_from_*`; boundary/replay transport; strengthened referees/census.

## Fences held

- No `ANCHOR-TABLE-SURFACE-0` table / consumer migration
- No FIELD-SWEEP 5.4–5.8
- No CPU decision branching on observed values; no second threshold authority
- No band edges in accumulation/falloff/conservation
- No new raw identity doors outside `wgsl_encode` / GPU round-trip rematerialize
- Draft PR only; no merge; no 5.3 dispatch; no `/clearance` from coder lane
