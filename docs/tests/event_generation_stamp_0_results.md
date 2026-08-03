# EVENT-GENERATION-STAMP-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- ORIENT-RECEIPT: `4a101ed6652d`
- orientation_rule_stamp: `abd646955a48aa4a`
- HD-RECEIPT: `22c8f88826dd`
- Dispatch: Board comment `5165742228` (DA authorization `5165693534`)
- base_sha (handoff): `49bc1d4a`
- dispatch master at open: `bbd593cde693`
- implementation_code_sha: `2334e0c12226dec3227b334c4d406194a08f9115`
- tested_code_sha: `2334e0c12226dec3227b334c4d406194a08f9115`
- tip (docs bind): `fbc8c6341e6e134cb8e6ef85f5a9fc59d43c96fa`
- coverage_basis: PASS - tip after tested head is docs-only results binding; no Rust source/test/registry change
- draft PR: #1596
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.1 ONLY** (no 6.1b / 6.2 / 6.2b / 6.3)

## ANCHOR-ACK (required)

| anchor_id | content_hash (prefix) |
|---|---|
| stead-events-are-rf | `525388344ef2` |
| stead-rejected-shapes | `3752549ff106` |
| core-rf-arenas | `d171614211e9` |
| core-overlays | `54df7604a49d` |
| field-policy-time-decisions | `993c7d0560e8` |
| simthing-0087-binding-laws | `c519e37f97d6` |
| simthing-0087-pillars | `61487cba1f9e` |
| stead-shared-surface-ledger | `87eaa1e7bb9c` |
| founding-ontology-invariants | `46802793fba7` |
| seal-residue-cross-crate | `49ee7c4ba6f4` |
| admission-ladder-necessity-test | `4bedf826f6f7` |
| core-gpu-residency | `b2f1be11daf5` |
| core-property-value-model | `04338b307bf8` |
| exact-numeric-candidate-f | `6938a2efadb5` |
| field-sweep-preservation | `acc521a5a361` |
| eml-extension-ladder | `7755bc72ffbe` |
| orientation-harness-core | `8a365d1c0864` |
| rf-arena-substrate | `17b5f1e5c2ba` |
| scanner-selftest-delta-gate | `34fb2662baae` |
| stead-spatial-contract-core | `b4a112cd02e8` |
| structural-execution-convergence | `6b4cedec482b` |
| workshop-candidate-homing | `3e584f0ad175` |

## Contract delivered

| Contract | Mechanism |
|---|---|
| Two carriers, one generation authority | Sealed `EmissionRecord` / `ThresholdEmission` / `ThresholdEvent` carry `generation` at the seal boundary (`with_generation`); GPU POD **not** widened. Reduce-up products stamp via `stamp_reduce_up_product` → `StampedReduceUpProduct` |
| Unstamped integrate hard-errors | `integrate_unstamped_product_forbidden` → `IntegrateError::UnstampedProduct`; only `GenerationStamped<T>` integrates |
| Async is ordinary | Parent gen N+3 integrating child gen N is one call, no wait token, no degraded path |
| Determinism by recording | `IntegrationSchedule` records `(parent_gen, child_gen, product_key)`; empty schedule + non-empty products → `MissingSchedule` |
| Staleness visible | `GenerationStamp::staleness_from_child` — attributable from stamps alone (no 6.3 derived STEAD field) |
| Ring egress + backpressure | `StampedEventRing` + `BackpressurePolicy::{OverwriteOldest,Throttle,CoalescePerKey}`; observer drain does not feed sim |
| Definable Horizon on dispatch | `dispatch_until_dissolved` / `UntilDissolvedWith`; unit `UntilDissolved` rejected for dispatch mint; session commitment path admits with `AtSessionEnd` |
| 6.0b preservation | Stamp rides existing carriers; `AttachOverlay` still routes through `deliver_routed_overlay`; no second transport |

## Rejected shapes held (stead-rejected-shapes)

No opt-in-upward sensitivity, no sink taxonomy, no second clock/barrier/sequence authority, no parent-wait for lagging child, no permanence variant.

## Focused proof

```text
simthing-core event_generation_stamp_0: 10 passed; 0 failed
simthing-spec event_generation_stamp_reduce_up_0: 3 passed; 0 failed
simthing-kernel sealed::emission generation_stamp_tests: 1 passed; 0 failed
```

## Local verification

- `cargo check -p simthing-core -p simthing-spec -p simthing-kernel -p simthing-sim -p simthing-driver --all-targets`
- Focused tests above
- CI does **not** run `cargo test` — green hosted checks are not crate health

## Posture

**PROBATION / proof-present / DA-review-pending** under `DA-RESERVE(gate-wiring)`.

Coder does **not** invoke `/clearance` or merge. Final exact-head clearance is orchestrator-owned.
No 6.1b / 6.2 / 6.2b / 6.3 work is included.
