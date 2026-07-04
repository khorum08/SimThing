# SimThing Core — Structural Invariants

These rules are enforced by the type system and code structure, not by convention.
A violation is either a compile error or a test failure. If you find yourself
working around one of these, stop and reconsider the design. The *why* behind these
rules lives in [`simthing_core_design.md`](simthing_core_design.md); this file is the gate.

---

## Specification Fidelity & Anti-Ceremony

> Binding, draconian. Constitutional home: constitution §0.6 (carry-forward spine). These rules
> gate **closure**, not just code. A change to this section is Tier-2.

| Rule | Enforced by |
|---|---|
| **Spec fidelity — PASS/CLOSED requires the specified structure** | A track claiming IMPLEMENTED/PASS/CLOSED against a written spec must either implement the spec's structure as written, or carry a **Deviation Record** at the top of its results doc — approved by design authority, enumerating each unimplemented element, the proxy, the reason, the consumer impact. Silent substitution is **VOID**; the track reopens. |
| **No silent tier collapse** | Collapsing a specified containment hierarchy (tiers / gridcells / surfaces / building-children) into a flat proxy never satisfies the spec. Tier collapse — including "parked / not yet wired" — is a Deviation requiring a recorded, approved Deviation Record, never an implicit pass. |
| **Scope Ledger on every closure** | Every CLOSE/ACCEPT carries a *Specified vs Implemented* ledger: each element marked `implemented` / `proxied` / `deferred` / `parked`, with evidence. No ledger, invalid closure. |
| **The specified consumer must actually run** | No PASS/CLOSED via documentation churn, status-table edits, report-only aggregation, or harness ceremony; the specified consumer must execute. Sibling of "Scenario Proof" and gating-policy §6, binding for closure. |
| **No project-management cosplay** | Progress is working, spec-faithful implementation under test — never the count of memos, packets, reviews, status rows, or reports. Documents *record* progress; they never *constitute* it. |
| **The Necessity Test — a test is residue, kept only if it catches what nothing higher catches** (added 2026-07-03; retires "one representative per boundary") | A test is admissible **only if it catches a regression that neither (1) the compiler / a type boundary, (2) a production admission hard-error on a live path, nor (3) an existing integration/canonical path already catches.** If deleting a test cannot break production and it is not a downstream dependency or required for canonical function, **delete it** — do not keep it as a per-boundary "representative." "One representative per boundary" was a **fossil premise** (tests = coverage) that the kernel admission substrate falsified; the floor is **zero** for any invariant a type or live admission hard-error enforces. Legitimate keeps: parser/format behavior no type absorbs, CPU-oracle/GPU parity, determinism/golden byte-exactness, doc-named invariant proofs, escaped-bug regressions, CI scanner known-bad fixtures. |
| **The CI doctrine-scan is the primary automated compliance mechanism** | Anti-ceremony governs *documents and process* — not the running enforcement layer. A guard that **actually runs and blocks** (the CI doctrine-scan + its self-test + triage) is real enforcement, never "harness ceremony," even though it emits a report. Ceremony is a document standing *in place of* running enforcement — and **asserting or fabricating a result you did not run is its worst form.** Do not cite §0.6.5, D8, or "no governance artifacts" against the screen or to skip running it. |

---

## Scenario Proof

| Rule | Enforced by |
|---|---|
| A scenario is proven only through a real reduction | A scenario/feature is not "done" until an opt-in test constructs real `SimThing` / `SimProperty` / `Overlay` state and advances it through `BoundaryProtocol` (or the accepted spec→`AccumulatorOp` lowering), asserting behavior on the resulting GPU/CPU-resolved values. A standalone CPU math module is an **oracle**, not a proof: it may stand in for the GPU side under bit-exact parity, but a scenario gate may **not** close on admission-rejection tests plus a `Vec`-based math loop with no engine reduction on either side of the parity. |

---

## Scenario / GameSession / Spatial Tree

| Rule | Enforced by |
|---|---|
| Scenario is the save/load authority wrapper | Canonical `SimThingScenarioSpec` files use a `Scenario` root; save/load systems prove the Scenario wrapper |
| Scenario has exactly one direct GameSession child | Scenario validation rejects zero or multiple GameSession children (`SCENARIO-GAMESESSION-CHILD-0`, PR #779) |
| GameSession is the runtime session root beneath Scenario | Runtime/Studio/session tests load and reason from the GameSession subtree; save/load wraps the Scenario around it |
| Owner SimThings are direct GameSession children | Scenario fixtures and structural guards place owner entities as GameSession siblings (`SESSION-OWNER-ENTITIES-0`, PR #780) |
| Owners are never spatial parents | Ownership changes update owner refs/properties/columns/overlays; no spatial reparenting; the D=3 ownership-node is the canonical rejected design |
| GalaxyMap / WorldStateMap is a direct GameSession child and the spatial root | Scenario fixtures and Save/Load validation assert the spatial root under GameSession (`SESSION-GALAXYMAP-WORLDSTATE-0`, PR #781) |
| Planet gridcells contain a 1×1 surface gridcell before gameplay children | Scenario Runtime / Save-Load final review; admission errors on missing/duplicate/off-(0,0)/direct-gameplay (`SCENARIO-PLANET-SURFACE-GRIDCELL-TIER-0`, PR #851) |
| Movement is the only spatial reparenting | BoundaryProtocol structural movement is separate from owner/channel/overlay updates |

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
| `SoftAggregateGuard` on WeightedMean columns feeding thresholds | `assert_no_hard_trigger_on_soft_aggregate()` at registration |
| `simthing-sim` never knows recipe semantics | No recipe strings, costs, or economic types in `simthing-sim` |

## Resource Flow Substrate

Added by `docs/adr/resource_flow_substrate.md`. These rules govern the
continuous-flow arena substrate that builds on AccumulatorOp v2.

### RF channel identity and settlement

| Rule | Enforced by |
|---|---|
| RF channel identity is owner/resource/scope metadata, not containment | RF rows carry `owner_ref`/`resource_key`/`scope_id`; owner SimThings remain GameSession siblings; no spatial reparenting for channel membership |
| Local settlement precedes upward bubbling | Recursive RF arena tests settle per parent Location before net output moves to parent (`LOADED-SCENARIO-RECURSIVE-RF-RUNTIME-0`, PR #838) |
| RF channels do not cross owners by default | Grouping key includes `owner_ref`; any cross-owner transfer must be explicit source-debit / overlay policy |
| RF overlay/property maps are the authoritative channel tags | Lowering resolves RF metadata to dense row/table surfaces; runtime groups by resolved channel columns |
| Scope IDs distinguish local/system/strategic arenas without new engines | Scope participates in row/table keys; no bespoke combat/economy/logistics subsystems |

### RF arena and allocation

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

Added by `docs/adr/mapping_sparse_regioncell.md`. These rules govern dense local spatial fields;
they are designer-facing guardrails first, runtime-enforced last. Design notes and evidence:
the ADR and `docs/workshop/m5_gradient_extraction_design_note.md`.

### Front propagation operators

| Rule | Enforced by |
|---|---|
| Gu-Yang/SaturatingFlux is the production front-propagation operator | Borders/chokepoints emerge from bounded conservative front fields, not authored border services; choke readout is one resident scalar column in the same dispatch |
| PALMA is the production reach/impedance utility | Pathfinding consumes gradient fields (`D = W + min(N4 D)` field over the front); no CPU route planner, no predecessor, no simulation-authority path object |
| A rendered path/border is presentation only | UI polylines/labels derive from fields and never become simulation authority; writing a perceived/field-derived path into a true spatial-authority column is rejected at spec admission |

### Spatial field guardrails

| Rule | Enforced by |
|---|---|
| No production mapping runtime without first-slice gating | The compute exists (stencil kernel/WGSL/ping-pong/oracle live; Layers 2–3 are existing AccumulatorOp paths); "runtime" = wiring a RegionCell field into a production session pass graph at session open. Gated to the named first slice; until then fields are test/opt-in-driven only |
| A gridcell/RegionCell *is* a `Location` SimThing — spatial identity is intrinsic | A gridcell is a `Location`-kind SimThing occupying a grid coordinate `(row, col)` in its parent's grid, backed by RegionField columns. "Mapping-role" names the field-column *backing*, not a separable or optional identity; there is no non-spatial `Location`. `simthing-core` gains no new `SimThingKind` variant (`Location` is the kind). See `simthing_core_design.md` §7 |
| Arena→cell pressure is an on-device projection, never a side-channel | `ArenaPressureBindingSpec` projects a resolved RF arena participant flow `(arena, sub_field) → (target_id, row, col)` onto the cell seed, GPU-resident with no readback; the RegionField cell is its own column range (`source_col`/`target_col`), distinct from and seeded by the arena flow column. A hand-written side-channel seed map is never a scenario's primary evidence |
| Guardrail placement is two-layered | RON/Designer/spec rejects unsafe authoring at import; runtime enforces hard safety unconditionally (horizon execution caps, source-cap clamp, finite/column validation, ping-pong correctness, bounded field/perception clamps). Admission rejection is a guardrail, never a scenario's primary evidence (see "Scenario Proof") |
| `simthing-sim` is map-free | No RegionCell, atlas, gutter, cadence, halo, or field-formula concept in `simthing-sim`; it sees only flat columns and opaque `AccumulatorOp` registrations |
| Dense local fields use the generic primitive only | RegionCell fields are evolved by `StructuredFieldStencilOp` over flat buffers/dimensions/columns/kernel weights; no *semantic* WGSL. Generic, semantic-free shader extensions with CPU-oracle parity and meaning pinned at admission are admissible; gameplay concepts never enter WGSL |
| No within-frame field self-feedback; gradient fields are strict sinks | `source_col != output_col`/`target_col` at single-spec admission. A gradient/derivative `output_col` may not be the `source_col` of any same-frame diffusion/stencil field; derivatives are consumed only downstream (reduction/EML/threshold). Feedback closes across ticks, never within a tick's algebraic cycle. Frame-level: `validate_region_field_frame_gradient_sinks` |
| Strategic awareness is hierarchy + parent EML, never dense global diffusion | Cell→parent `SlotRange` Sum then parent `EvalEML` on a later band; widening the stencil horizon for empire-scale awareness is rejected (over budget by evidence) |
| Band ordering: reduce before interpret | Sum reduction band precedes parent `EvalEML` band within a tick; cross-cell propagation advances by later-band cascade, not same-band chaining |
| Default stencil operators are stability-bounded | `normalized_stencil` / `source_capped_normalized` only; `raw_additive` (blows up) and `clamped_additive` (saturates, loses gradient) are not production operators |
| Horizon is capped | Default tactical H ≤ 8; extended H ≤ 16 only with `allow_extended_horizon` plus a source-cap/decay stability contract; `run_ping_pong`/`dispatch_ping_pong` reject steps above `config.horizon` |
| Ping-pong required for H > 1 | Proven GPU=CPU bit-exact; single-buffer multi-hop is a data race and is rejected |
| Source policy is caller-managed (v1) | `CallerManagedOneShotSeedThenZero`; the primitive never auto-identifies or auto-clears sources; column-wide `source_col` zeroing is **banned** (corrupts propagated state) |
| Active masks require a halo | `ActiveOnlyExperimentalNoHalo` is never production-authorized; only H-hop / per-hop halo with CPU-oracle parity is admitted |
| Behavioral source policy needs source identity | No shader-step source masking until a generic `source_mask` or separate seed buffer lands; deferred by ADR |
| Velocity needs an explicit previous-value column | EML has no previous-buffer read opcode; trajectory pressures use an explicit previous column with a copy band scheduled before the update band |
| No CPU AI map planner | Strategic commitments emerge as `Threshold` + `EmitEvent` crossings over parent EML pressure columns; no CPU-side map traversal decides anything |
| Perception write-boundary: perceived/deception never write back to true fields | Data flows true → perceived only; perceived/last-known/confidence/deception are bounded per-observer columns. The **only** path updating authoritative state is an explicit gameplay event via the event/`BoundaryRequest` path; any op writing a perceived/deception column into a true column is rejected at spec admission |
| Field formula-class admission is a designer-layer policy | `field_pressure`/`field_urgency`/`field_decay`/`bounded_field_update` are admitted at the RON/Designer/spec layer (`register_formula` is runtime-sufficient) |
| Mapping is opt-in, bounded, default-off | Map-spec presence is structure; execution requires explicit scenario-class/profile opt-in; every field declares hard caps (grid bounds, horizon cap, source cap) |

---

## Boundary resolution (tick / boundary / day)

Binding doctrine; naming preference set by product. The line is on *semantics*, not vocabulary.

| Rule | Enforced by / meaning |
|---|---|
| Prefer legible tick / boundary / day naming; guard the semantics | `tick` = deterministic substrate advancement; `boundary` = synchronization point for resolved summaries/events/metadata; `day_index` = monotonic boundary counter; `ticks_per_day` = ticks-per-boundary cadence (`boundary_reached = tick_in_day >= ticks_per_day`). These names are product-endorsed — do not churn them toward abstract alternatives. The guardrail is semantic: no calendar arithmetic, no `Calendar`/month/year/season type, no leap/date math, no sim-side pause flag. Calendar/turn/frame/season meaning lives at the host/spec/boundary-handler layer; the Daily Economy Fixture is an example, not a canonical cadence |
| No `DailyResolutionBoundary` | No runtime primitive that bakes calendar semantics into the boundary; absence is regression-guarded by source-scan tests. A generic boundary-output packet is admissible only as an abstract, read-only carrier of already-resolved values — never calendar fields / CPU recomputation / date arithmetic |
| Pause/speed are host-layer | The sim never advances autonomously; it advances only when the host requests a tick. Pausing at a boundary is a coherent save/snapshot point. No sim pause flag |
| CPU boundary consumes, never recomputes | At the boundary the CPU consumes resolved summaries/events/metadata only; it must not recompute economy/threat/urgency, must not emit commitments via CPU planner logic, and must not scan dense RegionCell grids by default. Discrete boundary banking uses the opt-in `ResourceEconomySpec` substrate; Resource Flow is the continuous substrate, separately opt-in and default-off — not the default discrete-banking answer |

---

## JIT Kernel Registry (Phase M-JIT, closed at PROD-0)

Designer/spec-admission guardrails for the default-off production kernel registry shell.
Artifact identities, hashes, domains, candidate genealogy, and exhaustive proofs are pinned at
descriptor admission; see `workshop/sqrt_candidates.md` and the descriptor surface — they are
deliberately not restated here.

| Rule | Meaning |
|---|---|
| No semantic WGSL | JIT kernels are semantic-free straight-line shaders; gameplay/map/faction concepts never enter shader text — admission rejects semantic names |
| `ProductionCandidatePreview` is default-off | `production_wiring=false`; no default SimSession wiring; shell invoked only from explicit test/fixture paths until a separate gate authorizes production wiring; registry admits validated entries only (`validate_production_candidate_preview_entry`) |
| Exact authority requires a pinned fixed-point chain + the artifact-backed proven sqrt | A value is exact-authoritative **only** when its whole arithmetic chain is pinned fixed-point (Q16.16 / `ExactQ16WeightedSum` class) and any sqrt/mag stage is the exhaustively-proven, hash-pinned Candidate F artifact admitted through descriptor admission (exact mag additionally requires an exact pre-sqrt mag2, e.g. `ExactFixedPointDxDy`). Native/raw `sqrt`, raw f32 dx/dy, `mag2`, and any f32 weighting/scoring/threshold operand stay `ApproximateJitOnly`/`ApproximateDiagnostic`. Authority binds to artifact identity: any artifact change requires renewed exhaustive proof; hash mismatch is rejected. **Candidate F is permanently enshrined (STEAD-CONTRACT-0):** bit-exact Euclidean/sqrt decision-gate ops are routed *through* it as first-class, never avoided or demoted to dodge sqrt; it governs gate *magnitude* only and never licenses treating gridcell positions as inert (constitution §0.7; `stead_spatial_contract.md` inv. 8) |
| Approximate outputs cannot feed exact inputs | Rejected at admission and execution gates (`validate_exact_inputs` / `validate_exact_kernel_inputs`) |
| GPU atomic event compaction = exact count + unordered membership, never ordering | Compacted/bucketed/reduced GPU event outputs may claim exact count, per-code count, unordered membership, and order-invariant summaries **only** under declared capacity/overflow (and empty-bucket) contracts; ordering requires separate proof. FIELD_POLICY numeric records (proposals/consumers/admission/economic fixtures) are exact-authoritative only under declared fixed-integer rule + capacity contracts and authorize no semantic action interpretation |
| FIELD_POLICY/JIT closure posture | No CPU-side AI planner, urgency traversal, or commitment emission (strategic commitments are GPU `Threshold`+`EmitEvent` crossings). No scheduler/cache/default `SimSession` wiring. No production economy→mapping bridge. `simthing-sim` stays map/Gadget/Personality/Memory-semantic-free. FIELD_POLICY field-agent proposals route only through accepted substrates (Resource Flow allocator; `Threshold`+`EmitEvent`→`BoundaryRequest`; the SimThing's own columns); the ladder is closed at Proposal Pipeline V1. ClauseThing stays proposal-only |

---

## EML Gadget Library

Gadgets are a **designer-facing authoring layer**, not a new runtime
(`docs/workshop/eml_gadget_library_design_note.md`).

| Rule | Meaning |
|---|---|
| Gadgets are spec-layer node-template macros | A gadget compiles to a postfix subgraph over the **existing** `EvalEML` opcode set — no new WGSL, no per-gadget GPU kernel, no new opcode *for the gadget layer*. Registry/compiler live in `simthing-spec`; `simthing-gpu` stays the one generic interpreter; `simthing-sim` has no `Gadget`/`Personality` type |
| Extending the generic substrate vocabulary is a Tier-2 gate, not a prohibition | A new **generic, semantic-free** `EvalEML` opcode / `AccumulatorOp` combine function / generic kernel is admissible as a Tier-2 gate when (a) it carries no map/faction/AI/scenario semantics (admission rejects semantic names; the kernel sees only floats/indices), (b) it carries CPU-oracle bit-exact parity, (c) meaning is pinned at spec/designer admission, and (d) it is reusable by any SimThing — behavior stays **data**, never baked into the kernel. A scenario-specific op is semantic and stays banned. Rung "no new op/WGSL" lines are scheduling hygiene, narrowable by design authority to this gate; semantic-freeness + parity + anti-faking discipline is never relaxed |
| Mandatory CPU-oracle parity per gadget | No gadget (any tier) is admitted without a CPU oracle + parity test; bit-exact for `ExactDeterministic`. SoftStep is `ExactDeterministic` because it is the algebraic sigmoid `0.5+0.5·u/(1+\|u\|)` — never `exp`/logistic/transcendental |
| Composition is PerGadgetOnly; preview ≠ runtime | Per-gadget templates and a single-gadget `InlineFlattenPreview` may be executable previews; multi-gadget stacks are `PerGadgetOnly`. The flatten preview is not a runtime execution path; true inline multi-gadget flattening requires separately-proven intermediate column wiring |
| Node cap is per executable tree | `MAX_EML_TREE_NODES` applies to each executable gadget/single-tree; a `PerGadgetOnly` stack total is diagnostic only |
| Admission validates at the designer layer; temporal memory is explicit-column state | Unknown/deferred kinds rejected; column bounds; finite/`>0` params; matched input/weight counts. Temporal gadgets (velocity / EMA / decay / bounded-feedback / hysteresis / acceleration) are admitted per the landed EML-GADGET-2 slices: no implicit previous-value read in EML — authored `current_col`/`previous_col`/`state_col`/`output_col` plus a snapshot/copy band at an earlier `OrderBand`; no hidden runtime allocation; defaults to parent/personality scope. Dense per-cell temporal memory and position-history acceleration are separately gated |
| Bounded-feedback admission contract | Any self-referential/recurrent temporal gadget declares at import: finite decay with default `0 ≤ decay < 1` (negative-but-`<1` needs explicit opt-in), an **explicit output clamp** (required when feeding a hard threshold/`EmitEvent`) **or** an admission-checkable analytically-bounded formula, finite gain, and no positive unbounded recurrence — otherwise admission rejects. Plus stateful-sequence (not single-step) CPU-oracle parity |

---

## The Proof Test

`custom_layout_ethics_axis` in `property.rs` is the invariant proof for the layout
generalization: it constructs a non-standard layout (signed ethics axis, drift governor,
width-3 bonus vector) and verifies stride, `offset_of`/`width_of`, defaults, governed
integration, and non-governed isolation. If it passes, the full generalization works; if a
change breaks it, a sub-field layout invariant above has been violated.
