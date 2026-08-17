# ACTIONBAND-OVERLAY-ACTUATION-0 (7.8) evidence

Status: **PROBATION / proof-present / DA-review-pending**. Coding does not invoke `/clearance`, merge, move the pointer, close the binding row, retire `OVERLAY-PEER-AUTHORITY`, or begin 7.8a/7.9/8.x.

## Receipts and scope

- Authoritative implementation base: `ebbae0617d5630cd9ab61a0beb67badbeb8586dc`
- Tested code checkpoint: `a7953f8c9d9dd3cbd6628fe207ec73d97ec4c983`
- ORIENT-RECEIPT: `4c9f068db285`
- orientation_rule_stamp: `ea1f69606c6929ea`
- HD-RECEIPT: `cf41c4ec2316`
- Orchestrator handoff: Board comment `5299283073`; bounded-history remand `5303319505`; actual-boundary remand `5311121155`

Every rendered REQUIRED-ANCHOR was queried and read before its governed path was edited. Load-bearing ACKs include `actionband-binding-laws` `d6a8b1b2d673`, `actionband-crossing-surface` `623db585f145`, `actionband-determinism-lifecycle` `6306c484732c`, `actionband-field-triad-authority` `56cf5cdf2d2c`, `actionband-native-authority-table` `541a03cb00a1`, `core-gpu-residency` `eea1356db087`, `core-overlays` `f95c9376ee06`, `field-sweep-preservation` `acc521a5a361`, `movement-front-adjudications` (queried/read), `overlay-germ` (queried/read), and `structural-execution-convergence` (queried/read). The generated reach log carries the complete query set.

No CI implementation, workflow, scan, allowlist, binding-condition, orientation, ladder, or pointer file was edited. `OVERLAY-PEER-AUTHORITY` remains live.

## Production shape and call graph

There is one session-frozen `CrossingConsequenceBinding` enum with exactly three variants:

1. `ResidentNextWrite` retains the admitted GPU binding plus logical `SimPropertyId` and `SubFieldRole`; no row, slot, owner capability, or buffer handle is representable.
2. `RoutedOverlayDelivery` retains only logical target plus the authored `Overlay`; source generation is taken from the sealed dispatch at execution, and no deadline field exists. It emits `GenerationStamped<RoutedOverlayProduct>`, whose deny-unknown-fields wire shape is the sole routed-product mapper into the existing boundary request.
3. `StructuralAuthorization` retains only one closed existing structural `BoundaryRequest` (`AddChild | Remove | Reparent`); overlay lifecycle verbs and `AddDimension` are rejected.

The exact production trace is:

`Phase-5 threshold emission` → generation-bearing sealed `BandCrossingDelta` → `ActionBandExecutionPlan::crossings_from_sealed` → opaque `ActionBandCrossingBatch` plus semantic consumption key → consuming `CrossingConsequenceSession::bind_dispatch(self, ...)` binds exactly one dispatcher to exactly one facility boundary → `CrossingConsequenceDispatch::dispatch_and_apply(&mut self, ...)` derives one scalar world/facility offset from the first sealed batch and compares stamps with `offset + execution.facility_generation()` → `ActionBandGpuSession::dispatch_resident_next` → admitted facility-local Current/Next plane → the existing `FacilityPlaneGenerationBoundary` advances ActionBand state and resident consequence planes together → the dedupe window observes that same authority and clears prior identities before dispatch returns → `CrossingConsequenceBinding::submit_boundary` submits either the stamped routed product or the frozen structural request through the ordinary feeder boundary. The immediate post-boundary state is `(generation=1, keys=0)`; replay of the cloned generation-0 batch REDs `CrossingGenerationMismatch { expected: 1, actual: 0 }`. The scalar offset keeps a newly bound facility generation 0 compatible with inherited real world-generation crossings such as 75/76/77 without creating a second clock or history.

The former private `SimSession::submit_commitment_effects` AttachOverlay mapper is deleted. Its genuine journal-watermark boundary pacing remains, but it now stores an admitted `CrossingConsequenceBinding` and calls the same shared stamped-product boundary door with the sealed mapping-event generation.

Thus a crossing at t writes resident Next or authorizes a boundary consequence consumed no earlier than t+1. There is no comparator, listener, CPU crossing evaluator, direct tree mutation, second local swap authority, append-only crossing ledger, or session-lifetime replay history.

## Three-arm witness and routed epoch

`one_real_gpu_door_executes_all_three_consequence_arms` starts from a real Phase-5 GPU crossing and proves:

- resident `PropertyNext(Set)` runs the admitted bounded-feedback EML over resident previous `1.5` plus the real STEAD registration's output column value `2.0`, yielding bit-exact `1.75` only through the facility-local plane; that real non-empty dispatch advances the existing production facility boundary and returns with `(generation=1, keys=0)` immediately, without an empty dispatch trigger; replay of the cloned generation-0 batch then REDs `CrossingGenerationMismatch { expected: 1, actual: 0 }` without consulting a retained identity;
- routed delivery starts at a distinct origin below a policy host, crosses the LCA to a distinct target, emits one ordinary `AttachOverlay` stamped with source generation 1, arrives through ordinary receive ingress at destination generation 7, and evaluates to policy-filtered `0.3` rather than the direct-target mutant's `0.6`; activation at 7 rebases authored duration 4 to deadline 11, never source-relative 5;
- structural authorization emits and applies one ordinary `Reparent`, with no GPU state-plane destination.

The same routed overlay carries six logical parameter transforms and compiles through the existing generic FieldSweep door as STEAD source/falloff, PALMA W/terminal value, and Gu-Yang conductance/capacity. These compiler forms change only authored map/post EML data and resident column bindings. Adjacency, canonical fold order, conservative symmetry, conductance certificate, and χ remain minted by the existing `apply_field_sweep_registration` admission path.

## 7.5c oracle-first migration

The 7.5c real-GPU vendor was already graduated before this rung. It now executes its three native Next lanes plus `Reparent` through the canonical consequence door, while its prior low-level dispatch is retained only inside the proof as the migration oracle.

The compact structural destination ABI remains row 0 and the numeric plan fingerprint remains `1584755073879803108`.

Bit-identical native-lane parity digests from the capacity N/N+1/N+2 series:

- generation 75, capacity 1.0: `a2ef29c97cb77c9a`
- generation 76, capacity 0.25: `bc8630ec226f7240`
- generation 77, capacity 1.0: `a2ef29c97cb77c9a`

Each run also applies the door-produced `Reparent` through the ordinary structural boundary. The existing 7.3 subordinate-activation reference was not changed.

## Eight production falsifiers

1. **Certificate envelope:** χ `1.25` on the new overlay-parameterized Gu-Yang surface fails the existing conductance-certificate admission; runtime input EML is clamped and cannot mutate adjacency/order/symmetry/χ.
2. **Direct FOREIGN resident write:** a binding minted from an otherwise identical sibling `ActionBandNativeLaneAdmission` fails the actual `compile_crossing_consequence_session` door with `ForeignResidentLaneAdmission`; the foreign consequence cannot compile or dispatch.
3. **Second post-crossing dispatcher / stale generation:** two batches are deliberately minted from the same cloned sealed delta before binding. A same-generation rival dispatcher is type/control-flow unrepresentable: `CrossingConsequenceSession` is non-`Clone`, `bind_dispatch(self, ...)` consumes it, the resulting sole dispatcher requires exclusive `&mut self`, and every successful non-empty depth-1 dispatch advances its `FacilityPlaneGenerationBoundary` before returning. The first batch therefore lands at the real boundary with zero retained keys; the second can execute only afterward and REDs `CrossingGenerationMismatch { expected: 1, actual: 0 }`. Production `src` contains no append-only/session-lifetime crossing ledger, and no empty dispatch is used to manufacture cleanup.
4. **Unbounded positive feedback:** the positive ActionBand program is the existing bounded-feedback gadget reading the admitted STEAD output column. Mutating that same admission to `decay=1` and infinite bounds REDs specifically at bounded-feedback admission; generation pacing cannot legalize it.
5. **Overlay-local EML:** `RoutedOverlayDelivery` has exactly private `{ target, overlay }`; compile accepts only the canonical shared `&EmlExpressionRegistry`. No program table/evaluator/cache field or constructor exists.
6. **Durable-row capture:** `ResidentNextWrite` has exactly `{ gpu_binding, property_id, role }`; the physical column is a compile binding re-derived from `DimensionRegistry`, and no row/slot/buffer survives as semantic identity.
7. **NEW-arm foreign absolute deadline:** the real `GenerationStamped<RoutedOverlayProduct>` production carrier serializes only target + authored overlay under `deny_unknown_fields`. Injecting `foreign_absolute_deadline` into that product REDs deserialization before boundary ingress; the forced-skew witness independently lands at destination-relative 11.
8. **StructuralAuthorization SAME-FACILITY GPU write:** `StructuralAuthorization` contains only a private `BoundaryRequest` and admits only `AddChild | Remove | Reparent`; it has no destination binding, column, plane, value, or write method. Overlay lifecycle verbs and `AddDimension` both fail the real admission door.

These are production admission/type boundaries or real-GPU witnesses, not inert `refuse_*` helpers.

## CostBand audit and census

The audited production quotient path remains `BoundaryProtocol` → `ThresholdRegistry::resolve_cost_band_draws_for_deltas` → `resolve_cost_band_draw_from_delta` → `cost_band_quantize`. It consumes sealed crossing evidence for boundary reporting and does not feed ActionBand actuation. No CPU `floor(V/C)` authority leak exists on the new actuation path, so no CostBand rewrite or oracle removal was made.

The census row `OVL-COMMIT-ATTACH` is now `GENUINELY-STRUCTURAL / keep`: its private semantic mapper has migrated to the shared consequence ABI, while journal watermark/pacing remains a real boundary responsibility. The canonical consequence boundary mapping remains `AN-ACTION-CONSEQUENCE / GENUINELY-STRUCTURAL / keep`. Final reconciliation is `routes=77 discovery=73 residue=75 unclassified=0 open=0`, `CENSUS-CHECK-VERDICT: PASS`.

## Test budget and validation

The focused budget is exactly two permanent seal proofs:

- `one_real_gpu_door_executes_all_three_consequence_arms`: the sole positive real-GPU three-arm witness, including exact same-generation rival-dispatch unconstructibility, immediate real-boundary dedupe clearing, stale-generation rejection, bounded field-output feedback, nontrivial routed filter traversal, destination pacing, and Field-Triad binding.
- `forbidden_overlay_and_state_plane_shapes_are_rejected_by_the_real_door`: the independent negative proof for foreign native-lane consequence compilation, foreign deadline wire injection, closed structural vocabulary, certificate envelope, and the same feedback admission mutated unbounded.

Validation at the tested code checkpoint:

- focused 7.8: 2 passed, 0 failed, 0 ignored;
- inherited 7.3 recursive composition: 5 passed;
- inherited 7.5c full vendor: 4 passed, bit-identical migration digests above;
- inherited 7.7 kernel lifecycle extraction: 1 passed;
- inherited 7.7 sim/oracle parity: 1 passed;
- FieldSweep N4 parity: 3 passed;
- `cargo check -p simthing-driver`: PASS;
- `TEST-INVENTORY-DRIFT-CHECK`: PASS (`rows=1262 discovered=1262 unledgered=0 stale=0`).

Exact-head agent scan, doctrine scan, hosted checks, and relay-lint are recorded in the orchestration relay after the evidence commit and PR body exist. Coding deliberately does not invoke `/clearance`; exact-head clearance remains orchestrator-owned per the Board amendment.
