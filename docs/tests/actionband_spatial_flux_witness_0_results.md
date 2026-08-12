# ACTIONBAND-SPATIAL-FLUX-WITNESS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5b)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-spatial-flux-witness-0`
- PR: **#1732**
- Base: `46347c81f17091ad6d3aa4bc51a9f43f2f3729f5`
- tested_code_sha: `d0c017e4bfcd93ad60fadc8b834fae992efb20fb`
- HD-RECEIPT: `c9f03eccc77a`
- ORIENT-RECEIPT: `98a916672d1a`
- orientation_rule_stamp: `3e2afa381d2aea10`
- Dispatch: board comment `5260559223`
- DA pre-dispatch: `5260442354` (A1 applied)

## Orientation

Fresh coding ORIENT on master `46347c81` before edits.

### ANCHOR-ACK (required)

| Anchor | content_hash |
|---|---|
| actionband-field-triad-authority | `56cf5cdf2d2cb0f67b847a0ab3eb42157fd51c2b7ab37253dd37af412beab77f` |
| actionband-native-authority-table | `541a03cb00a13ee073bbb7a854226054888b9528b31560f5024071e44be6eb97` |
| actionband-binding-laws | `d6a8b1b2d673be9f19f430afc333e7490e1b45dd32cbd55c10cc1d2e37b7f660` |
| field-sweep-preservation | `acc521a5a36121ef8b53fae1d2b2e1f4f1e57cf4ca717ffc4fe6d42204973201` |
| workshop-candidate-homing | `3e584f0ad1754a23a0a7fa841cc5f56ec8ffd4ed8fbc4bc7e0e2e2faf671e848` |
| stead-spatial-contract-core | `8585db4ac631f2fb390a6735409a6bc7d61760390a371ea5d6292328cf4accf3` |
| movement-front-adjudications | `5af6a29acb75573bdbd89a167aec73e3a3f6b342ffc980163e91cafa1e032efc` |

## Pre-clamp seam (mandatory first-act check)

**PASS — no production src edit required.**

The graduated ActionBand emission write evaluates EML `payload` first, then clamps conserved
`executable_payload` to the signed native `crossing.post_value` interval when
`auxiliary1 != NONE` (`action_band_execution.wgsl`). A dual non-conserved
`property_next` emission of the same `payload` is the witness-owned PRE-CLAMP
operand; conserved RF claim is POST-CLAMP. Workshop observes both via existing
proof readbacks (same pattern as graduated 7.5a).

## Scope ledger

| Path | Kind |
|---|---|
| `crates/simthing-workshop/src/actionband_spatial_flux_witness_0.rs` | workshop pure witness |
| `crates/simthing-workshop/src/lib.rs` | module export |
| `crates/simthing-workshop/tests/actionband_spatial_flux_witness_0.rs` | GPU + seam proofs |
| `docs/tests/actionband_spatial_flux_witness_0_results.md` | results |
| `scripts/ci/test_inventory.tsv` | ledger |
| `scripts/ci/inspect_justifications.tsv` | TEST-BUDGET (if needed) |

**Production engine `src` diff: none.**

## Proofs

| Obligation | Evidence |
|---|---|
| Capacity: fixed descent, vary Gu-Yang capacity | `capacity_witness_fixed_descent_varying_gu_yang_capacity` |
| Opposed demand: PRE-CLAMP opposite signs + POST-CLAMP stall | `opposed_demand_pre_clamp_signs_and_post_clamp_mutual_stall` |
| abs(flux) / sign mutants RED on pre-clamp | pure_unit + opposed GPU negative fixture |
| No-sink capacity-bearing lane | `no_sink_capacity_bearing_lane_without_costband` |
| Reapability / detachability | `reapability_production_has_zero_workshop_coupling` |
| Pre-clamp seam without production hook | `pre_clamp_seam_is_workshop_observable_without_production_src_edit` |

## Batteries

Focused 5 integration + 8 unit. Inherited 7.5a/7.5/7.4/7.3/7.2/7.1 at final head (relay).

Coding does not `/clearance`, merge, move pointer, or begin 7.5c.
