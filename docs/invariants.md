# SimThing Core — Structural Invariants

These rules are enforced by the type system and code structure, not by convention.
A violation is either a compile error or a test failure. If you find yourself
working around one of these, stop and reconsider the design.

---

## Property Layout

| Rule | Enforced by |
|---|---|
| `stride` is computed, never stored | `PropertyLayout::stride()` method; no `stride` field on the struct |
| Local index arithmetic has one home | `PropertyLayout::offset_of()` only — no raw index arithmetic anywhere else |
| Global column arithmetic has one home | `PropertyColumnRange::col_for_role(layout)` only |
| `PropertyValue::data` indices never hardcoded | All callers go through `offset_of` |
| Integration relationships are explicit | `SubFieldSpec::governed_by` — designer declares which sub-field governs which |
| Clamping is per-sub-field | `SubFieldSpec::clamp: ClampBehavior` — no property-level `valid_range` |
| Sub-field roles are named, not positional | `SubFieldRole::Named(String)` replaces `VectorComponent(usize)` |

## Registry

| Rule | Enforced by |
|---|---|
| Property definitions are registered once per session | `register()` panics on duplicate namespace+name |
| Columns are append-only within a session | `DimensionRegistry::total_columns` only increases |
| Tombstoned columns stay indexed | `active: Vec<bool>` marks inactive; no column removal |
| Column owners are recorded at registration | `column_owners: Vec<(SimPropertyId, usize)>` built during `register()` |

## Evaluation

| Rule | Enforced by |
|---|---|
| Transforms reference sub-fields by role, not column | `PropertyTransformDelta::sub_field_deltas: Vec<(SubFieldRole, TransformOp)>` |
| Column resolution happens in the CPU prep pass | `apply_to_data` takes `layout`; GPU receives only resolved indices |
| Transforms apply after velocity integration | Evaluation step order in `evaluate_node`; documented as intentional |
| Evaluator does not mutate the SimThing tree | `evaluate_node` takes `&SimThing`, returns snapshot; fission/fusion belong to day-boundary protocol |
| Determinism is bit-exact | Tests use `f32::to_bits()` comparison, not approximate equality |

## State Authority

| Rule | Enforced by |
|---|---|
| Within-day CPU shadow writes do not perform stale read-modify-write | `TransformPatcher` applies only `Set` immediately; `Add`/`Multiply` increment `unsafe_rmw_skipped` |
| Boundary lifecycle decisions read GPU-integrated values | `BoundaryProtocol::execute` reads `WorldGpuState::values` into `coord.shadow` before expiry/fission/structural work |
| CPU `TowardZero` expiry reads synchronized shadow | `resolve_property_expiry(root, registry, allocator, shadow, n_dims, ...)` resolves slot+column and reads `shadow` |
| Registry tombstoning is whole-tree scoped | CPU expiry collects removals first, then checks liveness from the root before `registry.tombstone(pid)` |
| Structural slot churn scrubs dense state | `AddChild` zeroes and projects initialized subtree properties; `Remove` zeroes rows before tombstoning slots |
| Fission secondary checks use the triggering property | `check_secondary(..., triggering_pid, ...)` reads Amount/Intensity from that property's shadow columns |

## SimProperty Identity

| Rule | Enforced by |
|---|---|
| `SimProperty` equality is on `namespace + name` only | Manual `PartialEq`, `Eq`, `Hash` impls that exclude all other fields |
| Metadata fields do not participate in key comparison | Verified by: two properties with same identity but different layouts compare equal |

## AccumulatorOp v2

| Rule | Enforced by |
|---|---|
| Exact operations never use soft-aggregate combine fns | Code review gate; `WeightedMean` / `Mean` may not appear in conservation-critical registration paths |
| `EvalEML` combine requires a whitelist entry | `EmlExpressionRegistry::assert_whitelisted(tree_id)` checked at registration |
| `SubtractFromSource` is the transfer mechanism for source-debit transactions | Discrete transfers and per-recipe consumption (via `SubtractFromAllInputs`) use `SubtractFromSource`-class semantics; allocator disbursements use `AddToTarget` on independent target slots with approximate-deterministic conservation per `docs/adr/resource_flow_substrate.md`. No two-overlay transfers anywhere |
| Emission records are produced for every GPU-resolved emission | `EmissionRecord { reg_idx, emit_count }` written to compact buffer; read back for delta log |
| Persistent GPU buffer is the residency model | `AccumulatorOpSession` is created at session open and closed at session close; no per-tick device creation |
| Timestamp queries are required for performance claims | Any PR claiming a performance win must include timestamped GPU pass measurements, not just wall-clock |
| Old pass code is never deleted without a green CI run at default-on flag | Sunset PR checklist; enforced before deletion |
| `design_v7.md` §4 is updated by each migration PR | PR template checklist item |
| `SoftAggregateGuard` on WeightedMean columns feeding thresholds | `assert_no_hard_trigger_on_soft_aggregate()` at registration |
| `simthing-sim` never knows recipe semantics | No recipe strings, costs, or economic types in `simthing-sim` |

## Resource Flow Substrate

Added by `docs/adr/resource_flow_substrate.md`. These rules govern the
continuous-flow arena substrate that builds on AccumulatorOp v2.

| Rule | Enforced by |
|---|---|
| Arena participation is explicit | `simthing-spec` rejects implicit/wildcard admission without declared upper bound at session build; property possession alone never admits to an arena |
| Arena caps are declared and enforced | Every `GpuArenaDescriptor` carries `max_participants`, `max_coupling_fanout`, `max_orderband_depth`; spec compiler fails the build if computed expansion exceeds any declared cap |
| Coupling cycles must contain a delay-bearing edge | Spec compiler walks the coupling graph; any cycle whose edges are all `CouplingDelay::Algebraic` fails the build |
| Hierarchical conservation is approximate-deterministic | For every intermediate allocator, `|Σ disbursed − budget| ≤ O(ε × n_children)`; residual integrates into the parent's `Balance` via existing `governed_by`; error is deterministic and replay is bit-exact |
| Balance is the sole carryforward ledger for resource flow | Leaf residual, allocator rounding residual, and zero-weight surplus all integrate into `Balance` via existing `governed_by` machinery; no separate per-arena budget state may exist in the runtime |
| Allocation policy is expressed through overlays, not policy enums | Allocator kernel reads weight columns; weight columns default to Demand-proportional and are overlay-modifiable via existing Add/Multiply/Set OrderBands; no new policy enum in `ArenaSpec` or the kernel |
| `simthing-sim` never sees `ArenaRegistry` | The driver compiles registry → flat `AccumulatorOp` registrations before upload; `simthing-sim` sees only `AccumulatorOp` structs and remains arena-ignorant |
| Fission inheritance is declared per arena | Each arena declares its `FissionPolicy` from `{Inherit, Reevaluate, Reject}` (default `Reevaluate`); the boundary protocol applies the policy at fission time via incremental subtree-scoped re-evaluation |
| `AccumulatorRole` is compile-time metadata only | Roles compile away into combine/gate/consume choices before reaching the GPU; `simthing-sim` never branches on `AccumulatorRole` at runtime |
| ArenaRegistry refresh is subtree-incremental | Boundary structural mutations refresh only the affected subtree's selector evaluations, not the global registry; expansion report updates correspondingly |
| Zero `weight_sum` integrates to parent Balance | When all child weights at an intermediate allocator are zero, every `child_share` evaluates to 0 via EML `SELECT`; the undisbursed budget integrates into the parent's `Balance` via the standard `governed_by` path |

---

## Mapping (Sparse RegionCell)

Added by `docs/adr/mapping_sparse_regioncell.md`. These rules govern dense local
spatial fields. They are designer-facing guardrails first: they reject dangerous
scenarios at import/session-build so blowout never reaches the simulation.

| Rule | Enforced by |
|---|---|
| No production mapping runtime without first-slice gating | The compute exists (StructuredFieldStencilOp kernel/WGSL/ping-pong/oracle live; Layers 2–3 are existing AccumulatorOp paths); "runtime" = wiring a RegionCell field into a production session pass graph at session open. That step is gated to the named first slice, after Phase M natives; until then fields are test/opt-in-driven only |
| RegionCell is an authored mapping-role, not a core kind | RegionCell is a spec/RON-authored mapping-role/profile on a SimThing backed by a slot range + field columns; `simthing-core` gains no new `SimThingKind` variant |
| Guardrail placement is two-layered | RON/Designer/spec owns expressive policy + rejects unsafe authoring at import; runtime enforces hard safety unconditionally (horizon execution caps, source-cap clamp, finite/column validation, ping-pong correctness, bounded field/perception clamps). Authoring is never trusted to have been safe |
| `simthing-sim` is map-free | No RegionCell, atlas, gutter, cadence, halo, or field-formula concept appears in `simthing-sim`; it sees only flat columns and opaque `AccumulatorOp` registrations |
| Dense local fields use the generic primitive only | RegionCell fields are evolved by `StructuredFieldStencilOp` over flat buffers/dimensions/columns/kernel weights; no semantic or map-specific WGSL is admitted |
| Strategic awareness is hierarchy + parent EML, never dense global diffusion | Cell→parent `SlotRange` Sum reduction then `EvalEML` on a later order band; widening the stencil horizon for empire-scale awareness is rejected (over budget by evidence) |
| Band ordering: reduce before interpret | Sum reduction band precedes parent `EvalEML` band within a tick; cross-cell propagation advances by later-band cascade, not same-band chaining |
| Default stencil operators are stability-bounded | `normalized_stencil` / `source_capped_normalized` only; `raw_additive` (blows up) and `clamped_additive` (saturates, loses gradient) are not production operators |
| Horizon is capped | Default tactical H ≤ 8; extended H ≤ 16 only with `allow_extended_horizon` plus a source-cap/decay stability contract; `run_ping_pong`/`dispatch_ping_pong` reject steps above `config.horizon` |
| Ping-pong required for H > 1 | Proven GPU=CPU bit-exact; single-buffer multi-hop is a data race and is rejected |
| Source policy is caller-managed (v1) | `CallerManagedOneShotSeedThenZero`; the primitive never auto-identifies or auto-clears sources; column-wide `source_col` zeroing is **banned** (corrupts propagated state) |
| Active masks require a halo | `ActiveOnlyExperimentalNoHalo` is never production-authorized; only H-hop / per-hop halo with CPU-oracle parity is admitted |
| Atlas batching requires isolation policy + VRAM reporting + protocol oracle | Still **Provisional/unimplemented** — `request_atlas_batching` is rejected at admission until a gate-passing M-4 PR. Isolation policy **ratified (2026-05-28):** for homogeneous square batches, **algebraic tile-local mask G=0 is the preferred isolation candidate** (1.0× VRAM, full-tile protocol-oracle parity); **physical gutter G≥H (per-tile seed clearing) is the fallback** (6.76× on 10×10 H=8); local-bounds tile-rect metadata is the deferred long-term policy. VRAM-multiplier reporting mandatory; **production acceptance requires full-tile parity vs an exact per-tile-protocol CPU oracle, not corridor-t44 alone**; column-wide `source_col` zeroing banned |
| Behavioral source policy needs source identity | No shader-step source masking until a generic `source_mask` or separate seed buffer lands; deferred by ADR |
| Velocity needs an explicit previous-value column | EML has no previous-buffer read opcode; trajectory pressures use an explicit previous column with a copy band scheduled before the threat-update band |
| No CPU AI map planner | Strategic commitments emerge as `Threshold` + `EmitEvent` crossings over parent EML pressure columns; no CPU-side map traversal decides anything |
| Perception write-boundary: perceived/deception never write back to true fields | Data flows true → perceived only; perceived/last-known/confidence/deception are bounded (`bounded_field_update`/`field_decay`) per-observer columns; the **only** path that updates authoritative state is an explicit gameplay event via the event/`BoundaryRequest` path; any op writing a perceived/deception column into a true column is rejected at spec admission |
| Field formula-class admission is a designer-layer policy | `field_pressure`/`field_urgency`/`field_decay`/`bounded_field_update` are admitted at the RON/Designer/spec layer (C-8 `register_formula` is runtime-sufficient); legacy whitelist rejection was wrong-layer, not runtime safety |
| Mapping is opt-in, bounded, default-off | Map-spec presence is structure; execution requires explicit scenario-class/profile opt-in (Resource Flow precedent); every field declares hard caps (grid bounds, horizon cap, source cap) |
| Economy→mapping influence is fixture-orchestration-only | A resolved `ResourceEconomySpec` boundary result may influence a SEAD commitment **only** by selecting authored, admitted EML weight profiles in `tests/support` fixture orchestration (the CPU reads resolved storage and *selects*, it never *computes* urgency or *emits* the commitment — urgency and the event stay GPU-resident via field→reduction→`field_urgency` EvalEML→Threshold+EmitEvent). A production economy→mapping runtime bridge is **not** authorized without a separate gated decision; guardrails for this coupling live at the designer/importer/scenario-admission layer, not the sim/boundary layer (accepted 2026-05-29 — `docs/reviews/phase_m_product_fixture_chain_acceptance_opus_review.md`) |

---

## Boundary resolution (tick / boundary / day)

Accepted as binding doctrine 2026-05-29, **naming preference set by product 2026-05-29** — see
`docs/reviews/phase_m_boundary_resolution_and_example_economy_acceptance_opus_review.md`. SimThing
exposes deterministic **tick / boundary / day** resolution. The line we hold is on *semantics*, not
on vocabulary: a `day` is one host/spec interpretation of the boundary counter, but `tick`,
`boundary`, `day`, `day_index`, and `ticks_per_day` are the **preferred, endorsed names** because
they are legible.

| Rule | Enforced by / meaning |
|---|---|
| **Prefer legible tick / boundary / day naming** | `tick` = deterministic substrate advancement; `boundary` = synchronization point for resolved summaries/events/metadata; `day_index` = monotonic boundary counter (one host/spec interpretation is a calendar day); `ticks_per_day` = ticks-per-boundary cadence (`DispatchCoordinator`, `boundary_reached = tick_in_day >= ticks_per_day`). These names are **preferred for legibility**. Do **not** churn them toward abstract/illegible alternatives (e.g. "boundary-index", "ticks-per-boundary-unit"); product set this preference 2026-05-29 |
| Avoid Clausewitz/calendar **semantics** (not the names) | The guardrail is on *semantics*: no calendar arithmetic, no `Calendar`/month/year/season type, no leap/date math, no sim-side pause flag. The legible `tick`/`boundary`/`day`/`day_index`/`ticks_per_day`/"day boundary" naming is **endorsed** — `day_index` is a monotonic boundary counter, not a calendar. Calendar/turn/frame/season meaning lives at the host/spec/boundary-handler layer |
| No `DailyResolutionBoundary` | No runtime primitive that bakes calendar semantics into the boundary; absence is regression-guarded by source-scan tests. A future generic boundary-output packet is admissible only if it stays an abstract, read-only carrier of already-resolved values and never grows calendar fields / CPU recomputation / date arithmetic |
| Pause/speed are host-layer | The sim never advances autonomously (no internal wall-clock scheduler); it advances only when the host requests a tick. Pausing at a boundary is a coherent save/snapshot point. No sim pause flag |
| CPU boundary consumes, never recomputes | At the boundary the CPU consumes resolved summaries/events/metadata only; it must not recompute economy/threat/urgency, must not emit commitments via CPU planner logic (commitments are GPU `Threshold` + `EmitEvent` crossings), and must not scan dense RegionCell grids by default |
| Discrete banking ≠ continuous flow | Discrete boundary banking uses the opt-in `ResourceEconomySpec` substrate (conservation-exact transfers/recipes, threshold emit; storage persists in GPU values across boundaries). Resource Flow E-11 is the continuous/high-frequency substrate, separately opt-in and **default-off** (`use_accumulator_resource_flow` false) — not the default discrete-banking answer |
| Daily fixture stays an example | Daily Economy Fixture V1 is an example/product fixture only and does not make daily cadence canonical; other hosts may read the same boundary as a turn, frame, season, orbital step, market close, or learning epoch |

---

## EML Gadget Library

Added by `docs/workshop/eml_gadget_library_design_note.md`; EML-GADGET-1 (Tier-1) accepted
2026-05-29 — see `docs/reviews/phase_m_eml_gadget_tier1_acceptance_opus_review.md`. Gadgets are a
**designer-facing authoring layer**, not a new runtime.

| Rule | Enforced by / meaning |
|---|---|
| Gadgets are spec-layer node-template macros | A gadget compiles to a postfix subgraph over the **existing** `EvalEML` opcode set — **no new WGSL, no per-gadget GPU kernel, no new opcode.** Registry/compiler live in `simthing-spec`; `simthing-gpu` stays the one generic interpreter; `simthing-sim` has no `Gadget`/`Personality` type |
| Mandatory CPU-oracle parity per gadget | No gadget (any tier) is admitted without a CPU oracle + parity test; compiled postfix evaluation must match within tolerance (bit-exact for `ExactDeterministic`). SoftStep is `ExactDeterministic` because it is the **algebraic** sigmoid `0.5+0.5·u/(1+\|u\|)` — never `exp`/logistic/transcendental |
| Composition is PerGadgetOnly; preview ≠ runtime | Per-gadget templates and a single-gadget `InlineFlattenPreview` may be executable previews; multi-gadget stacks are `PerGadgetOnly` (chained OrderBand runtime scheduling deferred). The flatten preview is **not** a runtime execution path — no driver/gpu/sim code consumes it. True inline multi-gadget flattening requires separately-proven intermediate column wiring |
| Node cap is per executable tree | `MAX_EML_TREE_NODES` applies to each executable gadget/single-tree; a `PerGadgetOnly` multi-gadget stack may exceed that informational total (diagnostic only), since each gadget is its own tree |
| Admission validates at the designer layer | Unknown/deferred kinds rejected; column bounds; finite/`>0` params; non-empty + matched input/weight counts. Tier-2 temporal gadgets (velocity/EMA/acceleration/hysteresis/decay) are admitted per landed EML-GADGET-2 implementation slices — the **design is accepted as a gate** (Opus/product 2026-05-29, [`reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md`](reviews/phase_m_eml_gadget_tier2_design_acceptance_opus_review.md)); the ladder is 2A (snapshot/copy fixture proof) → 2B (VelocityMonitor + Decay/EMA) → 2C (BoundedFeedback) → 2D (Hysteresis) → 2E (explicit velocity-column Acceleration); position-history acceleration and dense per-cell temporal memory remain separately gated |
| Temporal memory is explicit-column state | No implicit previous-value read in EML; temporal gadgets require authored `current_col`/`previous_col`/`state_col`/`output_col` and a snapshot/copy band (`Identity`+`ResetTarget` at an earlier `OrderBand`) before the update band; no hidden runtime allocation; defaults to Layer-3 parent/personality scope — dense per-cell temporal memory separately gated (VRAM budget + named scenario + product gate) |
| Bounded-feedback admission contract | Any self-referential/recurrent temporal gadget must declare, at import/admission: finite decay with **V1 default `0 ≤ decay < 1`** (negative-but-`<1` decay needs explicit opt-in), an **explicit output clamp** (required when the output feeds a hard threshold/`EmitEvent`) **or** an *admission-checkable* analytically-bounded formula, finite gain, and no positive unbounded recurrence — otherwise admission rejects. Tier-2 analog of source-cap / `bounded_field_update`. Plus stateful-sequence (not single-step) CPU-oracle parity |

---

## The Proof Test

`custom_layout_ethics_axis` in `property.rs` is the invariant proof for the
generalization. It constructs a non-standard layout — signed ethics axis with a
drift governor and a width-3 bonus vector — and verifies:

1. `stride()` = sum of sub-field widths (1 + 1 + 3 = 5)
2. `offset_of` returns correct local indices for each role
3. `width_of` returns 3 for the Named("ethics_bonus") vector sub-field
4. `default_data()` produces correct defaults including the 1.0 neutral bonus values
5. Integration advances the governed sub-field using the governing sub-field's value
6. Non-governed sub-fields are untouched by integration

If this test passes, the full generalization works. If a future change breaks it,
something about the sub-field layout invariants has been violated.

---

## What These Invariants Buy

**Correctness by construction.** A designer editing a `SubFieldSpec` cannot accidentally
produce inconsistent column arithmetic — there is only one place column arithmetic
lives, and it reads the layout at runtime.

**Refactoring safety.** Renaming a sub-field from `Named("vec_0")` to `Named("grievance_inertia")`
requires updating the `SubFieldSpec` role and any overlay `sub_field_deltas` that
reference it by name. The compiler will catch the latter via exhaustive match if
`SubFieldRole` is a closed enum for your game's named fields — or the test suite
will catch it via the observability query tests if `Named` remains open.

**GPU/CPU parity.** The CPU preparation pass and the CPU reference evaluator both go
through `offset_of` and `col_for_role`. If the GPU output diverges from the CPU
oracle in Week 2 tests, the bug is in shader arithmetic or buffer layout — not in
a disagreement about what column a sub-field occupies, because both sides read the
same registry.

**Designer safety.** The one-edit guarantee holds as long as all callers use
`offset_of` and `col_for_role`. Any direct `data[N]` access outside of
`PropertyLayout` methods is a violation of the invariants above and should be
treated as a bug regardless of whether it produces correct output today.
