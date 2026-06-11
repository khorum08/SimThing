# CT-3b+4a-ORIENT-0 — Resource Flow Movement-Front Heatmap Design Memo

> **Status: ACCEPTED WITH AMENDMENTS (executive design authority, 2026-06-11).**
> Amendments binding over the body text: (1) §12's listing of `ai_will_do` Layer-3 compilation
> as "optional" is **reversed** — the CT-4a half of the rung (`ai_will_do` → EML
> urgency/pressure over **reduced parent columns** → `Threshold` + `EmitEvent` commitment) is
> **mandatory in the implementation exit**, alongside the heatmap; the reorientation adds the
> Resource Flow pressure feed, it does not drop the FIELD_POLICY leg. (2) The orientation
> ruling's re-scope of the §5 ladder row is **ratified as a recorded amendment** in the
> production track (an implementing agent may not silently re-scope a rung). (3) The gated-rate
> EML band (**CT-RF-EML-RATE-0**, cut at the CT-2c-REMEDIAL-3 review) is added to §11's allowed
> GPU-resident computations and is expected to land with this implementation rung.
> This is a guardrail + design-orientation memo. No production code, no WGSL, no tests;
> implementation may start only against the amended text. Authority: the 0.0.8.1 constitution,
> [`../simthing_core_design.md`](../simthing_core_design.md), [`../invariants.md`](../invariants.md),
> [`../design_0_0_8_1.md`](../design_0_0_8_1.md) §0/§2, the production track
> [`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md)
> §3.1/§5/§6/§6.1/§7/§9/§11, [`../adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md),
> RF-BASE-INTRINSIC-0, CT-2c-REMEDIAL-2
> ([`../tests/ct_2c_impl_results.md`](../tests/ct_2c_impl_results.md)), and CLAUSETHING-MOVABLE-SEMMAP-0
> (track §3.1). Historical dress-rehearsal disruption heatmap work (0.0.8.0 R1) is precedent for
> *shape* only — this memo reorients the **ClauseThing headline vertical** around Resource Flow arena
> pressure as the primary feed, not occupant hand-seeding or a standalone movement engine.

**Named consumer (what this memo unblocks):** **CT-3b + CT-4a implementation** — the first
ClauseThing-authored headline vertical that proves a **GPU-resident movement-front /
suppression-disruption heatmap** over admitted gridcell/location/container topology, fed by
Resource Flow arena pressure and existing AccumulatorOp / Movement-Front substrate — not by
ClauseScript runtime interpretation, category dispatch, noun engines, CPU planners, or
presentation-only maps. CT-2c (category economy → `BaseFlowObligationSpec` → install-seeded
`IntrinsicFlow`) is the accepted floor this vertical builds on.

**Orientation ruling:** The production track §5 row previously described CT-3b+4a as
`RegionFieldSpec` + `ai_will_do` → `field_urgency` EML in isolation. **ORIENT-0 reorients that
vertical:** the heatmap is a **field/arena-derived pressure map** whose *sources* are admitted
SimThing Resource Flow outputs (intrinsic produce/upkeep, reduced aggregates, overlay-modified
weights, coupling edges) and whose *propagation* reuses the accepted sparse RegionCell /
`StructuredFieldStencilOp` machinery — not a new semantic movement, combat, or economy engine.

**JIT EML / WGSL posture (binding amendment):** This memo must **not** preserve a blanket JIT EML or
WGSL ban. It distinguishes **forbidden semantic GPU code** from **allowed and expected
GPU-resident accumulator/arena/heatmap arithmetic**. Spec/admission/install defines what exists;
JIT EML / WGSL may compute over those admitted bounded buffers — it must never decide the semantic
meaning of the buffers.

---

## 1. What a movement-front heatmap is in SimThing terms

**Decision:** A movement-front heatmap is a **GPU-resident scalar (or small vector) pressure field
over an admitted spatial lattice** — gridcell SimThings, location/container nodes, or their
parent-reduced summaries — whose values represent **local operational pressure** (threat,
disruption, suppression, supply reach, desirability, or scenario-named analogues) and whose
spatial contours are **propagating fronts**, not presentation overlays.

Concretely:

| It **is** | It **is not** |
|---|---|
| A field column (or small column bundle) on admitted gridcell/location SimThings, evolved by bounded stencil / cascade over admitted adjacency | Presentation-only UI heatmap state |
| Derived from Resource Flow arena outputs + overlay-modified sub-fields + explicit masks | An AI planner or pathfinding engine |
| Reduced up the SimThing tree on declared OrderBands (Layer 2 in `simthing_core_design.md` §7.2) | A combat engine, economy engine, fleet engine, or leader engine |
| Consumed by threshold crossings → `BoundaryRequest` / `EmitEvent` (Layer 3 interpretation) | Semantic movement policy executed inside the heatmap kernel |
| Inspectable as a deterministic artifact for parity proof | A side-channel test map manually written outside install/admission |

The heatmap **is world state** in the same sense as any other resolved property column: it participates
in `accumulate → reduce up → mask down → disburse down → threshold`. Visualization may *read* it;
visualization does not *author* it.

**Relationship to Anchor A (Movement-Front):** The heatmap is the Layer-1 face of the Movement-Front
automaton (`simthing_core_design.md` §7): locally coupled cells, one shared stencil rule (P2),
horizon-capped propagation (P1), bounded operators + threshold projection (P3). The **sources** that
seed cell columns are no longer hand-authored test constants as the primary proof path — they are
**Resource Flow arena pressure** from mobile and stationary SimThings participating in admitted arenas
at their current parent/location.

---

## 2. What “suppression/disruption front” means generically

**Decision:** Use **suppression/disruption front** as the generic name. Literal historical SEAD
domain semantics are **not** imported unless an authored scenario names them.

Generic meaning:

1. **Local pressure** on a spatial node expresses how much nearby movement/operation capacity is
   suppressed or disrupted (positive disruption, negative suppression, or paired columns — scenario
   authored).
2. **Pressure attenuates** through **admitted topology only**: stencil neighborhoods, declared
   coupling edges, OrderBand cascade depth — never unbounded world scan or hidden neighbor discovery.
3. **Participants remain SimThings** — fleets, patrols, pirates, factories, cohorts contribute
   through properties, intrinsic-flow obligations, overlays, and arena reduction outputs at their
   **current parent/location**, not through a global mover registry.
4. **Effects are Resource Flow / overlay / field outputs** — bonuses and penalties are overlays;
   base produce/upkeep are `BaseFlowObligationSpec` / `IntrinsicFlow`; propagation is stencil +
   reduction, not a semantic “combat resolution” pass.

A suppression/disruption front is therefore **the moving contour where opposing or complementary
pressure columns meet** after bounded propagation — the same “traveling wave / attractor” reading as
`simthing_core_design.md` §7.1 P3, fed from arena pressure rather than from a dedicated subsystem.

---

## 3. Admitted inputs (exhaustive for v1 design)

The heatmap may consume **only** data that passes the existing admission/install firewall:

| Input class | Source | Role in pressure |
|---|---|---|
| **Parent/location/gridcell topology** | Authored SimThing tree + mapping-role slot ranges | Defines *where* cells exist and *which* adjacency stencil applies |
| **Mobile SimThing participation** | Explicit arena enrollment + install targets | Who contributes pressure at which node this tick |
| **Base intrinsic-flow obligations** | `BaseFlowObligationSpec` → install-seeded `IntrinsicFlow` (RF-BASE-INTRINSIC-0; CT-2c consumer proof) | Base produce/upkeep rates at enrolled participants |
| **Overlays / modifiers** | `OverlaySpec` OrderBands on flow/field sub-fields | Perturb weights, demand, suppression/disruption columns without collapsing provenance |
| **Resource Flow arena outputs** | Reduction sweep (`IntrinsicFlow` aggregate up), allocation sweep (down), `Balance`, coupling edges | Local arena pressure magnitudes and signed net flow |
| **Explicit masks** | Mask-down / disburse-down authored bands | Gate which pressure reaches which subtree or column |
| **Threshold specs** | `Threshold` + event kinds on parent or cell columns | Fire when pressure crosses — not movement policy inside stencil |
| **Bounded movement/assignment constraints** | Assignment-slot parentage, reparenting admission, movement-front transfer specs | Cap *who* may enroll where — not a CPU path planner |

**Forbidden inputs:** ClauseScript runtime values; category dispatch tags at runtime; noun-specific
engines; CPU assignment planners; presentation-only heatmap seeds as primary proof; side-channel
`HydratedCategoryEconomyPack.contributions`-style metadata; unbounded wildcard enrollment; hidden
fanout beyond declared arena caps.

---

## 4. Outputs and downstream consumers

**Primary outputs (GPU-resident, parity-backed when implemented):**

1. **Cell-level pressure columns** on gridcell/location SimThings (e.g. `disruption`,
   `suppression`, scenario-named analogues).
2. **Parent-reduced summaries** after OrderBand reduce-up (strategic awareness via hierarchy, not
   wider stencil — P1).
3. **Optional gradient columns** (`RegionFieldOperatorSpec::Gradient`) when a consumer needs local
   slope — e.g. FIELD_POLICY-style sit-vs-step *later*, not in ORIENT-0.
4. **Threshold crossings** → `BoundaryRequest` / `EmitEvent` when authored thresholds on reduced
   or cell columns fire.
5. **Inspectable deterministic artifact** (heatmap table, top hotspots) for CPU oracle parity —
   same discipline as dress-rehearsal R1 shape, but fed from RF arena pressure in the implementation
   rung.

**Suitable uses:** visualization heatmap; movement-front pressure signal; suppression/disruption
gradient for threshold-gated decisions; AI weighting overlays over **reduced** parent columns (Layer
3 EML) — **not** direct execution of semantic movement policy inside the heatmap kernel.

---

## 5. How Resource Flow arena outputs feed local pressure

**Decision:** Arena pressure is **not** a parallel economy subsystem. It is the existing Resource Flow
substrate (`adr/resource_flow_substrate.md`) read at spatial nodes after the universal loop's
accumulate/reduce phases.

Conceptual pipeline per tick (opt-in session only):

```
Admitted participants at location L
  → IntrinsicFlow (from BaseFlowObligationSpec at install + any admitted overlay deltas)
  → reduce up OrderBands within arena tree
  → optional coupling edges (declared delay form) between arenas
  → map selected sub-field(s) to cell seed column(s) via authored binding
  → StructuredFieldStencilOp / RegionField propagation (bounded horizon)
  → cell pressure heatmap
  → reduce up to parent summary columns
  → threshold / BoundaryRequest
```

**Binding rules:**

- **One arena registry per session** — many named arenas, column ranges, not kernel variants.
- **Dual role preserved:** intermediate nodes contribute `IntrinsicFlow` upward and receive
  `allocated_flow` downward; heatmap seeding reads the **resolved column state** after the authored
  band ordering, never a shadow economy ledger.
- **Coupling fanout and OrderBand depth** remain capped; admission rejects excess (`e10_resource_flow_admission`).
- **Opt-in/default-off unchanged:** `ResourceFlowSpec` presence alone stays inactive; mapping/field
  execution profiles stay explicit (`MappingExecutionProfile`, `ResourceFlowOptInMode`).

The ClauseThing headline vertical will **hydrate** scenario-authored bindings (which arena sub-field
feeds which cell column) into `GameModeSpec` — same transpilation firewall as CT-2c; no runtime
ClauseScript.

---

## 6. How `BaseFlowObligationSpec` obligations feed pressure

**Decision:** Base produce/upkeep obligations are the **intrinsic floor** of arena pressure at a
participant's enrolled location. They do **not** become overlay modifiers and must not be collapsed
into presentation heatmap constants.

Path (accepted in CT-2c-REMEDIAL-2):

1. ClauseScript literal `produces` / `upkeep` `_add` keys → `BaseFlowObligationSpec` on
   `ResourceFlowSpec.base_obligations`.
2. Install consumes obligations → participant `AccumulatorRole::IntrinsicFlow` sub-field
   (`seed_base_flow_obligations()` in driver).
3. Resource Flow reduction aggregates intrinsic contributions up the arena participant tree.
4. Authored **arena-to-cell binding** (implementation rung spec) projects the reduced value (or
   leaf participant column before reduce, if scenario requires local-only pressure) onto the
   gridcell seed column for that location.
5. Stencil propagation spreads seeded pressure to neighbors per admitted horizon.

**Provenance:** Overlays may multiply or add to **downstream** columns or weight sub-fields; they
must not replace the install-seeded intrinsic floor. Diagnostic mirrors (e.g. CT-2c
`contributions`) remain authoring-side only.

---

## 7. How overlays/modifiers perturb pressure

**Decision:** Modifiers remain **overlays** on flow/field sub-fields — the Resource Flow ADR's
“default-overlay allocation policy” and CT-2c `_mult` / `_add` inheritance asymmetry carry forward.

| Modifier kind | Effect on pressure |
|---|---|
| `_mult` on produce/upkeep family keys | OrderBand sweep over category subtree → scales intrinsic or weight columns before reduce |
| `_add` leaf overlays | Leaf-only delta on flow sub-fields |
| Gated / triggered overlays (CT-1b pattern) | Suspend/activate; no separate economy engine |
| Personality / AI weight overlays (Layer 3) | EML gadget trees over **parent-reduced** pressure — band ordering: reduce before interpret |

Overlays **never** introduce a runtime category tag or noun-specific dispatch. After hydration,
only property columns and overlay stacks exist.

---

## 8. Locality, parentage, and mobile SimThings

**Decision (track §3.1 binding):** Fleets, armies, leaders, monsters, agents, pops/cohorts remain
**ordinary mobile SimThings**. The heatmap reads pressure from **where they are parented now**, not
from a global registry.

| Mechanism | Heatmap effect |
|---|---|
| **Parent/location** | Determines which gridcell/location arena slot receives enrollment and seed binding |
| **Reparenting** | Changes enrollment locality; arena registry refresh is **incremental on changed subtree** (E-9 discipline) — not global rescan |
| **Admitted movement-front transfer** | Moves SimThing nodes; pressure sources move with them |
| **Bounded assignment slots** | Admission constraint on valid parents — not CPU planner output |

**No global mover registry. No CPU pathfinding in this vertical.** Movement *consumption* of gradient
(FIELD_POLICY-style) is a **later** threshold-gated rung over an already-valid heatmap; ORIENT-0 does
not authorize pathfinding implementation.

---

## 9. Topology, adjacency, and coupling surface

**Decision:** Every neighbor relationship used by propagation must be **authored and admitted**:

1. **Grid mapping profile** — `RegionFieldSpec` / sparse RegionCell layout: square grid, horizon cap,
   `alpha_self` / `gamma_neighbor`, operator (`Normalized`, `SourceCappedNormalized`, `Gradient`).
2. **Slot-range adjacency** — gridcell children addressed as `(width, height, col)` for stencil index
   arithmetic (`simthing_core_design.md` §7).
3. **Resource Flow coupling edges** — arena-to-arena with declared delay form; no all-algebraic cycles.
4. **OrderBand depth** — explicit per-arena `max_orderband_depth`; cascade across bands for light-cone
   (P1), never unbounded same-band chaining.
5. **Arena caps** — `max_participants`, `max_coupling_fanout`; expansion report at build time.

**No hidden neighbor discovery.** Stencil width and horizon are spec surfaces, not runtime magic.

---

## 10. Bounded propagation enforcement

| Guard | Enforcement layer |
|---|---|
| Implicit participation | Rejected at admission |
| Unbounded wildcards | Rejected unless `max_expansion` declared |
| Horizon > cap | Rejected at RegionField admission |
| Coupling cycle without delay edge | Rejected at compile |
| Hidden fanout | Rejected via expansion report |
| Dense global diffusion as strategic awareness | **Permanently rejected** (P1); use reduce-up hierarchy |
| Presentation-only seed as primary proof | **Forbidden** for CT-3b+4a exit |

Propagation is **finite-speed** by construction: horizon cap H, band-separated cascade, bounded
feedback/decay gadgets where recurrence is authored.

---

## 11. CPU/oracle vs GPU (implementation rung — not built in ORIENT-0)

**CPU/oracle remains authoritative for proof gates:**

- Install/admission expansion reports
- Resource Flow allocation oracle (`run_arena_allocation_oracle` pattern from CT-2c)
- Cell-by-cell stencil recurrence oracle for heatmap evolution (dress-rehearsal R1 precedent)
- Bit-exact parity before WGSL/JIT paths close proof gates

**GPU/JIT EML/WGSL candidates (future implementation rung, after bounded buffers + CPU oracle named):**

| Computation | JIT EML / WGSL role |
|---|---|
| Arena column projection / binding to cell seeds | Allowed — arithmetic over admitted buffers |
| Accumulator reduction/allocation passes | Allowed — existing E-11 kernels |
| Stencil propagation / normalized neighbor mix | Allowed — `StructuredFieldStencilOp` family |
| Local pressure recurrence (bounded feedback) | Allowed — EML gadget or WGSL over declared columns |
| Suppression/disruption heatmap combine | Allowed — multi-column arithmetic only |
| Threshold-prep buffers | Allowed |
| Gradient magnitude for **exact** parity gates | **`m_jit_sqrt_f_exact` only** — native WGSL `sqrt` is approximate/diagnostic |

**Forbidden JIT EML / WGSL:** ClauseScript interpretation; category dispatch; semantic entity
classification; participant discovery; hidden enrollment; CPU-planner replacement; leader/fleet/pop/
monster special engines; runtime economy/combat/movement policy; presentation-only heatmap pretending
to be simulation.

---

## 12. ClauseThing hydration shape (implementation preview — not ORIENT-0 scope)

When implementation opens (post memo acceptance), ClauseScript will hydrate **scenario bindings**, not
runtime interpreters:

- Spatial scenario blocks → gridcell/location SimThing templates + mapping roles
- Unit/fleet/cohort templates → mobile SimThings with `BaseFlowObligationSpec` + overlays (CT-2c path)
- Arena declarations → `ResourceFlowSpec` + explicit enrollment
- **New hydration surface (ticketed at implementation):** `arena_pressure_binding` or equivalent
  mapping from `(arena, sub_field)` → `(grid_profile, cell_column)` — must land as spec-visible
  authoring data, consumer-pulled; **no widening in ORIENT-0**.

Optional ClauseScript mirrors of historical shapes (`ai_will_do`, region field blocks) compile to
**threshold + overlay + EML** on **reduced** parent columns — Layer 3 — not to semantic GPU code.

---

## 13. AccumulatorRole, spec widening, and sim layers

| Question | ORIENT-0 answer |
|---|---|
| New `AccumulatorRole`? | **None required** for the oriented design — reuse `IntrinsicFlow`, existing flow/field sub-fields, stencil field columns, overlays |
| `simthing-spec` widening? | **Deferred** to implementation tickets naming CT-3b+4a as consumer (arena-to-cell binding surface at minimum) — **none in this memo PR** |
| `simthing-sim` changes? | **Must remain untouched** — arena-ignorant; flat registrations only |
| `simthing-gpu` / WGSL in this PR? | **None** — design only |
| Resource Flow opt-in/default-off? | **Preserved** — explicit profiles; presence alone inactive |
| CPU fallback economy/movement? | **Forbidden** |

---

## 14. Exact GPU sqrt rule (standing doctrine)

Any GPU-resident sqrt, magnitude, distance, gradient norm, movement-front norm, or threshold path
that claims exactness, feeds exact-authoritative state, participates in bit-exact parity, or closes a
proof gate **must** route through `m_jit_sqrt_f_exact` (hash-pinned, artifact-backed Candidate F).
Native WGSL `sqrt` is `ApproximateJitOnly` / diagnostic only and cannot close exact-authority tests.

**ORIENT-0:** Heatmap seeding from Resource Flow rates does **not** require sqrt. **Future**
FIELD_POLICY-style gradient magnitude consumption (0.0.8.0 R4 precedent) **will** require exact
classification at implementation — native sqrt cannot close that gate.

---

## 15. Implementation handoff — opens only on memo acceptance

- **Allowed files (implementation rung):** ClauseThing hydration, fixtures, driver proof tests,
  focused spec widening tickets — per separate implementation handoff; **not** this PR.
- **Forbidden in implementation without new gate:** simthing-sim semantic awareness; movement/combat/
  economy engines; global registries; CPU planners; presentation-only proof; WGSL before CPU oracle
  named; Paradox/lab corpus in repo fixtures.
- **Targeted tests (implementation):** mirror CT-2c discipline — never `cargo test --workspace` unless
  design authority explicitly approves.
- **Exit proof (implementation):** ClauseScript-authored headline scenario → canonical RON → install →
  RF arena pressure → GPU heatmap vs CPU oracle; no manual side-channel injection; opt-in/default-off
  re-proven; bounded participation re-proven.

---

## Closure answers

1. **Named consumer:** CT-3b + CT-4a headline vertical — Resource Flow-fed movement-front /
   suppression-disruption heatmap over admitted spatial topology.
2. **Movement-front heatmap:** GPU-resident pressure field over admitted gridcell/location lattice,
   propagated by bounded stencil/cascade, derived from arena outputs — not presentation, planner, or
   combat engine (§1).
3. **Suppression/disruption front:** Generic local pressure that suppresses/disrupts neighboring
   operational capacity, attenuating through admitted topology; participants remain SimThings (§2).
4. **Admitted data feeds:** Parentage/topology, mobile enrollment, `BaseFlowObligationSpec`,
   overlays, RF arena outputs, masks, thresholds, bounded assignment — §3.
5. **RF arena → pressure:** Reduce intrinsic/allocation columns at enrolled nodes → authored binding
   to cell seed columns → stencil propagation → reduce-up summaries (§5).
6. **Base obligations → pressure:** Install-seeded `IntrinsicFlow` from `BaseFlowObligationSpec` →
   arena reduction → cell binding (§6); CT-2c path is the floor.
7. **Overlays → pressure:** OrderBand overlays on flow/field sub-fields; provenance preserved (§7).
8. **Locality/parentage:** Enrollment follows current parent/location; incremental subtree refresh on
   reparenting (§8).
9. **Mobile SimThings:** Ordinary SimThings; parent determines arena locality (§8).
10. **Reparenting / movement:** Changes participation via reparenting or admitted transfer; no global
    registry (§8).
11. **Topology surface:** RegionField/grid profile, slot-range adjacency, RF coupling edges,
    OrderBand depth, arena caps — all spec/admission (§9).
12. **Bounded propagation:** Horizon cap, band cascade, admission caps, no hidden fanout (§10).
13. **CPU/oracle:** Authoritative for proof gates until GPU parity closes them (§11).
14. **WGSL/JIT allowed later:** Bounded arena projection, reduction, stencil propagation, heatmap
    arithmetic, threshold-prep — not semantics (§11).
15. **WGSL/JIT forbidden:** ClauseScript interpretation, dispatch, discovery, planners, noun engines
    (§11).
16. **Avoid noun engines?** Yes — track §3.1 binding preserved (§8).
17. **Avoid global registries / CPU planners?** Yes (§8, §10).
18. **Resource Flow opt-in/default-off?** Yes — unchanged (§5, §13).
19. **New AccumulatorRole?** No (§13).
20. **Sqrt/magnitude required?** Not for RF seeding; future gradient magnitude for exact gates must use
    `m_jit_sqrt_f_exact` (§14).
21. **Unneeded artifacts deleted?** Yes — no scratch reports created for ORIENT-0.
22. **Visibility artifacts:** This memo only under `docs/clausething/`; no duplicate `docs/tests` report.
23. **`cargo test --workspace` avoided?** Yes — docs-only rung; no tests run.
24. **Production ledger updated honestly?** CT-3b+4a → DESIGN MEMO / READY FOR REVIEW; not
    IMPLEMENTED/PASS.
25. **Branch/merge:** Recorded in the orientation PR body after push.

---

*ORIENT-0 completes the guardrail pass. Implementation of CT-3b+4a awaits design-authority acceptance
of this memo.*
