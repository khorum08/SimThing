# STEMTHING-B-FLOW-MARKET-GERM-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2a
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `a2ea1b6edaca415113beb7a42543dd1c38440b5a`
- Branch: `codex/stemthing-b-flow-market-germ-0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- HD-RECEIPT: `e2a67ae4502e`
- Handoff: Board comment `5405301542`
- Expected route: `DA-RESERVE(gate-wiring)`
- Pointer movement: none
- Structural certificate: owed at graduation

## Pre-edit grammar map

| §12 / A1 link | Exact landed attachment |
|---|---|
| arena enrollment and conserved claims | `OwnerChannelScopeKey`, `RuntimeOwnerSiloDemandBucket`, `reduce_owner_channel_rf` |
| specialization authoring | `SpecializationProfile` → `SpecializationFlowMarketSpec` → `admit_specialization_flow_market` |
| strict offering / sealed price vector / Draw | `ConservedOfferingSpec`, `OfferingPriceVectorSpec`, `DrawEnvelopeTemplateSpec`; serde unknown-field rejection and admission-time reference/trigger/bound checks |
| inherited effective clearing weight | recursive `resolve_effective_clearing_weights` over `SimThing` using sparse `TransformOp` overrides |
| EML valuation and constrained clear | `AuthoredClearingProgram`, stamp-derived `clear_stamped_owner_channels`, `ConstrainedGrant` |
| largest remainder and tie authority | landed `constrained_clearing.rs`; residual rank then exact-tie rotation by canonical `SimThingId` under `ClearingRemainderAuthority { granter, generation }` |
| CostBand | existing scalar `cost_band_quantize` through the admitted offering unit cost |
| field sweep and Triad | existing `compile_palma_n4_field_sweep`, `compile_gu_yang_n4_field_sweeps`, Phase-5 `EmitOnThresholdRegistration`, ActionBand GPU execution |
| detached subtree seam and 6.1 record | `AsyncOwnerChannelRfSeam`, `IntegrationSchedule`, `replay_async_owner_channel_rf_seam` |
| authoritative F2 corpus | existing `ReplayFrame { BandCrossingDeltasApplied, shadow_values }`, `ReplayWriter`, `ReplayReader`, `ReplayDriver` |
| boundary grant lifecycle | cleared-only `MarketGrantRecord`; detachment retain, cleared renewal, revocation/release, exact fission partition, exact fusion transfer, death/dissolution/explicit termination release |

No mapped link was absent. No new engine manager, allocator, clearing engine,
field mechanism, telemetry, or history was required.

## Signal matrix

| Signal | Observed result |
|---|---|
| same germ, two markets | residency-slot supply `2` clears `2`; compute-quanta supply `5` clears `5` through one RF + EML + constrained-clear path |
| sealed Draw | strict offering mismatch and inactive lifecycle trigger RED; quantity bounds and trigger references are admission checked; Draw emits claim data and grants nothing |
| price versus clearing weight | compute offering uses scalar unit cost `1.5`; inherited EML weight resolves `2.0` for one child and `1.0` for its sibling |
| CostBand | compute value `5.0` at unit cost `1.5` yields `N=3, R=0.5` |
| work-conserving rotation | equal residual tie selects different canonical recipients at granter generations 12 and 13; allocation sum remains supply exact |
| detached execution | provisioned child reduces its own RF generation after detachment; grant identity and quantity remain unchanged |
| lifecycle conservation | cleared + renewed quantity `8`; revocation releases `1`; fission partitions remaining `7`; fusion transfers `7`; explicit termination releases `7`; `1 + 7 = 8` |
| death and dissolution | each releases the entire active fused grant; topology detachment alone releases zero |
| stamped renewal/revocation | two downward standing views and one upward detached RF product replay bit-exactly from the one `IntegrationSchedule` |
| non-residency F1 GPU | actual compute grant `4`; PALMA potential bits `3f800000`; Gu-Yang/STEAD crossing bits `3e48f5c2`; native-clamped ActionBand RF response bits `3e48f5c2` |
| ordinary response and F2 | ordinary overlay delivered; existing replay frame restores the exact crossing, field shadow, and overlay from two delta rows |

## Standing falsifiers

| Test | Biting failure |
|---|---|
| `two_markets_draw_clear_costband_and_rotate_exact_ties` | split market germs, permissive Draw references, missing inherited weight, non-work-conserving clear, scalar CostBand drift, or fixed exact-tie winner |
| `detached_grant_lifecycle_conserves_and_replays_on_the_existing_stamped_seam` | detachment release, non-cleared renewal, inexact release/partition/transfer, seam conservation loss, or second/ambient replay ordering |
| `germ_absence_census_and_lifecycle_mutants_red` | manager/allocator/ledger/history/telemetry/field/parallel-clear reach, duplicate generation-clear door, over-revocation, lossy partition, or duplicate fission successor |
| `non_residency_market_executes_rf_costband_full_triad_action_and_existing_replay` | synthetic second lane, dead PALMA/Gu-Yang/STEAD/ActionBand leg, CPU-invented field outcome, missing ordinary overlay, or replay mismatch |

Focused command:

```text
cargo test -p simthing-driver --test stemthing_b_flow_market_germ_0 -- --nocapture
```

Local result: `4 passed; 0 failed`; the GPU leg executed and emitted the signal
row above. Exact tested commit and workflow run IDs are carried in the PR and
Board return after the evidence commit.

## Fences retained

- No 11.2b residency placement, 11.2c allocator retirement, 11.2d vendor door,
  11.3+, 11.4, 12.x, Vector CostBand, or ClauseThing work.
- No second resource manager, global grant authority, replay recorder,
  telemetry plane, field solver, or ActionBand-local clearing path.
- Grant records are pure cleared products with exact transitions; they do not
  own scheduling, capacity, allocation, or replay.
- Existing pre-§12 clearing functions remain compatibility entries; the 11.2a
  market door binds the real granter generation through the new canonical
  generation-aware entry point.
