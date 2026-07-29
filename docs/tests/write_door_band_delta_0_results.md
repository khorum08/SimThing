# WRITE-DOOR-BAND-DELTA-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `bfd7fd9c217b`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Board dispatch: comment `5111853299`
- base_sha: `77ea7f12a933b5f0362afdaa4edf6970b4339ffc` (handoff); branch cut from current master including dispatch tip
- tested_code_sha: `8d2ada93db6313ce6acb1656e486d5c93db128e6`
- implementation_code_sha: `8d2ada93db6313ce6acb1656e486d5c93db128e6`
- final_head_sha: PR-body-bound (see draft PR `final_head_sha` after push; this file does not self-hash)
- clearance_pr_head: PR-body-bound (see draft PR `clearance_pr_head` after push; this file does not self-hash)
- coverage_basis: sealed band-delta + remap encode units; driver oracle/remap referees; fused GPU mint vs CPU oracle (`c1_threshold_gpu_matches_cpu_oracle`); write-door census; adapter-pinned kernel/sim/full driver; RF-1 + replay
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: PR-body-bound after hosted clearance

## What landed

1. Sealed `BandCrossingDelta` join-mint from fused-pass `ThresholdEmission` + registration sidecar (`band_crossing_deltas_from_fused_emissions` / `AccumulatorOpSession::readback_band_crossing_deltas`), scoped by optional Anchored column filter.
2. Typed `AnchorRemapSection` / `AnchorLocusRemap` with fail-closed `validate_anchor_remap_for_encode` gated immediately before boundary `sync_gpu_buffers`; stable-slot reparent = empty `remap_not_required` witness; `BoundaryDeltaEntry::AnchorRemapApplied` evidence-only for replay.
3. Permanent referees for rising/falling/exact-edge/no-crossing/multi-edge, remap-less negatives with op context, slot-churn completeness, reparent witness, remap serde/replay bit-exactness.
4. Census `scripts/ci/write_door_band_delta_census.sh` — sealed mint confinement, zero unexplained CPU invent symbols, boundary remap gate, no fission/tree_mutation `sync_gpu_buffers` bypass, fused readback mint door, structural inventory paths.
5. Ladder 5.2 **PROBATION**; Active open rung → `ANCHOR-TABLE-SURFACE-0`.

## Permanent referees

| Referee | Regression caught |
|---|---|
| kernel `band_crossing_delta` units + GPU `c1_threshold_gpu_matches_cpu_oracle` | Forgeable/missing deltas; GPU≠CPU oracle; Unobserved leak |
| core/sim remap encode units | Remap-less encode; broken reparent witness |
| driver `write_door_band_delta_0` | Crossing matrix + structural gate + replay transport |
| `write_door_band_delta_census.sh` | CPU-rescan invent / remap-free relocation / mint-door drift |

## Census appendix (load-bearing map)

- Anchored write impact binds to AccumulatorOp fused threshold scan → sealed BandCrossingDelta at readback (no parallel ladder).
- Structural encode inventory: `fission.rs`, `tree_mutation.rs`, `boundary.rs`, `gpu_sync.rs`; GPU sync after boundary remap gate only.
- CPU post-hoc invent symbols fenced to oracle/rehearsal/era residues by census arm.

## Fences held

- No `ANCHOR-TABLE-SURFACE-0` table / consumer migration
- No FIELD-SWEEP 5.4–5.8
- No CPU decision branching on observed values; no second threshold authority
- No band edges in accumulation/falloff/conservation
- No new raw `SlotIndex`/`ColumnIndex`/`RoleOffset` doors outside `wgsl_encode`
- Draft PR only; no merge; no next-rung dispatch until exit-proof
