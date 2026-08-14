# GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 (7.7) evidence

Status: **COMPLETE — DA-GRADUATED / merged #1761 @ `3fbdf56f`** (pre-dispatch `5290429128`, substrate STOP `5293929841`, census-pin `5298015175`, graduation ruling on Board #1332). The `OverrideReceived` promotion blocker is discharged by admission rejection.

## Receipts and authority

- ORIENT-RECEIPT: `8de008acfbdd` (`orientation_rule_stamp: 874354e66bc3ac81`)
- HD-RECEIPT: `5c6218625bc9`
- DA pre-dispatch authority: Board comment `5290429128`, A1-A4 unchanged
- DA restart authority: Board comment `5296404970`
- Orchestrator handoff: Board comment `5296440515`
- Implementation base: `275b6d8d755417fc3d2adf7102578b94efe5dd20`; historical replay base remains `9593ca2e658cfa063797e8d74a8cf6b29505a7b0`.

All rendered REQUIRED-ANCHORS were read and acknowledged before edits: `accumulator-exact-vs-soft-semantics`, `accumulator-op-v2-invariants`, `actionband-8x-sequencing`, `actionband-axis-budget`, `actionband-binding-laws`, `actionband-constitutional-placement`, `actionband-crossing-surface`, `actionband-determinism-lifecycle`, `actionband-eml-payload-purity`, `actionband-executive`, `actionband-fenced-questions`, `actionband-field-triad-authority`, `actionband-gpu-physical-model`, `actionband-native-authority-table`, `actionband-performance-model`, `actionband-target-forms`, `actionband-vendorization-direction`, `admission-ladder-necessity-test`, `candidate-f-exhaustive-proof-method`, `core-gpu-residency`, `core-overlays`, `core-property-value-model`, `core-rf-arenas`, `eml-admission-shapes`, `eml-extension-ladder`, `eml-integration-plan`, `eml-triad-integration`, `evaluation-identity-invariants`, `exact-numeric-candidate-f`, `field-policy-time-decisions`, `field-sweep-preservation`, `founding-ontology-invariants`, `intrinsic-constrained-clearing`, `movement-front-adjudications`, `one-tree-owners-never-spatial`, `orientation-harness-core`, `overlay-closure-thesis`, `overlay-designer-closure`, `overlay-germ`, `overlay-promoted-laws`, `overlay-scale-laws`, `rf-arena-allocation-invariants`, `rf-arena-substrate`, `scanner-selftest-delta-gate`, `seal-residue-cross-crate`, `simthing-0087-binding-laws`, `simthing-0087-pillars`, `stead-events-are-rf`, `stead-rejected-shapes`, `stead-shared-surface-ledger`, `stead-spatial-contract-core`, `stemthing-binding-laws`, `stemthing-lane-not-leg`, `stemthing-slot-identity-ruling`, `structural-execution-convergence`, and `workshop-candidate-homing`.

## Real production dispatch trace

`BoundaryProtocol::initial_gpu_sync/execute` → `sync_gpu_buffers` → `append_overlay_lifecycle_registrations` appends property/deadline predicates to the ordinary Phase-5 `ThresholdRegistration` packet → `WorldGpuState::upload_accumulator_threshold_ops` → `AccumulatorOpSession` binding 13 (facility-local Next plane) → canonical `accumulator_op.wgsl::maybe_emit_threshold` calls the sole existing `threshold_crossed` comparator → the successful crossing atomically projects its admitted bit into `OverlayLifecycleStateGpu` → the shared facility boundary advances Current/Next → `BoundaryProtocol::execute` reads compact completed rows and `apply_gpu_overlay_lifecycle` performs logical `(SimThingId, OverlayId)` removal and authored expire writeback. Routed `BoundaryRequest::AttachOverlay` carries `source_generation`; destination structural admission preserves that provenance, rebases `AfterTicks` against the destination generation, and rejects overflow before tree attachment. `resolve_overlay_lifecycle()` remains callable only as the CPU oracle and has no production caller.

Property targets use the registered resident values/previous-values buffers. `AfterTicks` is lowered to `deadline_generation = activation + duration`, carried by the existing owning-generation uniform operand, with checked overflow and exact-f32 comparator admission. No countdown write, wall clock, foreign absolute deadline, local EML registry/cache, lifecycle event dispatcher, or durable physical-row identity was added. `OverrideReceived` is rejected by core delivery, dispatch-mint, and spec compilation admission doors because no compatible germ replacement identity exists.

## Referees and mutants

- Real GPU/oracle parity: the property-plus-deadline conjunction dissolves identically to the retained CPU oracle. A production binding mutant aliases the deadline bit to the property bit and leaves the lifecycle pending (RED); a resident-property-source bypass mutant likewise leaves the property bit clear (RED).
- Single crossing surface: lifecycle projection is reachable only after the existing `maybe_emit_threshold` comparator succeeds; there is no lifecycle comparator/evaluator or post-crossing dispatcher.
- Mid-session semantic mint: the real session admission rejects a new template shape after the admitted set freezes. Row capacity is fixed by session construction; no dynamic manager exists.
- Deadline overflow: `u32::MAX + 1` returns `DeadlineOverflow`; no saturation/sentinel arm exists.
- Routed epoch: the real `AttachOverlay` boundary transports duration with source provenance generation 900, records destination activation generation 7, and lowers to deadline 11; foreign absolute/global generation 904 is RED. The same production door rejects destination `u32::MAX + 1` before attachment.
- Durable row capture: physical rows occur only in the upload-time projection plan. Structural consumption uses the logical identity sidecar and regenerates rows.
- Overlay-local EML/cache: no new EML registry, program table, evaluator, cache, or cross-instance CSE surface exists; lifecycle uses canonical threshold packet data.

## Replay and one history

The exact artifact `gpu_overlay_lifecycle_preextract_9593ca2e.ldjson` was emitted by `ReplayWriter` after the real pre-extraction CPU lifecycle route ran in an isolated detached worktree at `9593ca2e658cfa063797e8d74a8cf6b29505a7b0`. Post-extraction `ReplayReader`/`ReplayDriver` consumes those unchanged bytes and removes the recorded overlay.

- SHA-256: `b8039da1be5cddacc21563f994bdba24d3f0066f574404c252800389b239da85`
- One-history: the recording contains the existing `BoundaryDeltaEntry::OverlayDissolved` frame only; no `OverlayHistory` type/log/ring was introduced.

## Sparse carry before compaction

Real adapter measurement (one sample each, dispatch+submit wall time, no compaction):

| Rows | Current→Next bytes | ns |
|---:|---:|---:|
| 1 | 16 | 622,900 |
| 64 | 1,024 | 613,200 |
| 256 | 4,096 | 478,600 |

These are evidence samples, not a performance claim; command submission noise dominates at these cardinalities.

## Test budget

- `real_phase5_crossings_project_conjunctive_lifecycle_state` — admitted as one seal-proof integration test because it is the only real-adapter witness for property/deadline projection, conjunction, facility carry, frozen template/cap admission, and forced-skew/overflow mutants.
- `gpu_production_decision_is_bit_identical_to_retained_cpu_oracle` — admitted as one oracle-parity integration test because it compares the real GPU decision to the retained oracle and consumes the immutable historical replay artifact.

Both are permanent residue under their respective seal-proof/oracle-parity classes; no helper-only `#[test]` inventory was added.

## Validation

- `cargo check -p simthing-driver`: PASS
- `cargo test -p simthing-core -p simthing-spec -p simthing-sim`: PASS
- `cargo test -p simthing-kernel --test gpu_overlay_lifecycle_extraction_0 -- --nocapture`: PASS on a real adapter
- `cargo test -p simthing-sim --test gpu_overlay_lifecycle_oracle_parity_0`: PASS on a real adapter
- `TEST-INVENTORY-DRIFT-CHECK`: PASS (`rows=1260`, `discovered=1260`, `unledgered=0`, `stale=0`)
- Census and exact-head doctrine/agent scans are recorded after clean-tree harvest in the relay.
