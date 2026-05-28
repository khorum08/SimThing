# ADR: Sparse RegionCell Mapping

**Status:** **Approved (architecture, 2026-05-28)** — Phase M natives unlocked; first scenario-level slice named (§"First slice"). Still authorizes **no production mapping runtime** (first-slice session wiring is separately gated, after Phase M natives — see §"What 'no production mapping runtime' actually means").
**Date:** 2026-05-28
**Authors:** Opus 4.8 (design authority synthesis), grounded in the 2026-05-19 mapping/SEAD sandbox probes and the unified Cursor/Opus mapping guidance
**Extends:** `docs/design_v7.md` (V7.5 constitution + AccumulatorOp v2), `docs/design_v7_6.md` (StructuredFieldStencilOp promotion + guardrail relaxation), `docs/adr/resource_flow_substrate.md` (the opt-in / bounded / semantic-free substrate template this ADR follows)
**Companion (next):** `docs/design_v7_7.md` (mapping + SEAD surfacing), `docs/accumulator_op_v2_production_plan.md` (Phase M natives)

**Evidence cited (all `docs/tests/`):**
- `mapping_optimization_toolkit_sandbox_test_results.md`
- `mapping_optimization_remedial_sandbox_test_results.md`
- `mapping_atlas_algebraic_mask_sandbox_test_results.md` (M-4A — **ratified 2026-05-28: algebraic tile-local mask G=0 is the preferred isolation candidate for homogeneous square batches; physical gutter is fallback. Atlas remains Provisional/unimplemented.** See `docs/reviews/m4_m4a_first_slice_oversight_opus_review.md`.)
- `mapping_optimization_remedial_candidate_notes.md` (`docs/workshop/archive/mapping/`)
- `v7_6_structured_field_stencil_guardrail_hardening_test_results.md`
- `v7_6_structured_field_stencil_parked_state_test_results.md`
- SEAD series: `sead_field_intelligence_sandbox_test_results.md`, `sead_operator_toolkit_sandbox_test_results.md`, `sead_strategic_horizon_sandbox_test_results.md`, `sead_tensor_stencil_wgsl_sandbox_test_results.md`, `sead_tensor_stencil_refinement_sandbox_test_results.md`

---

## Context

V7.6 promoted `StructuredFieldStencilOp` to a live, hardened, opt-in generic GPU
primitive and then **parked**: the recorded next work item is explicitly "the
Mapping ADR that defines RegionCell fields, source policy, active-mask
halo/frontier semantics, cadence tiers, and column-aware parent bindings"
(`v7_6_structured_field_stencil_parked_state_test_results.md`). This ADR is that
document. It makes the architectural decision and classifies every optimization;
it is not a runtime implementation handoff.

Two questions sat open after V7.6:

1. **What is the map, architecturally?** The V7.5 constitution already answers
   the political-map question — factions, regions, and alliances are overlays on
   the spatial tree, not nodes in it (`design_v6.md` §2). What was unspecified is
   how *dense local spatial fields* (threat, suppression pressure, supply
   reach, fog, contamination) are represented and evolved without violating the
   constitution.

2. **Which optimizations from the sandbox probes are safe to adopt, and at what
   layer?** The toolkit and remedial probes plus the five SEAD probes produced a
   large, consistent body of evidence about atlas batching, cadence tiers, dirty
   skipping, active-mask halos, and source policy. That evidence needs to be
   converted into binding adoption/deferral decisions with the caveats attached.

The SEAD probes established the load-bearing negative result that shapes the
whole architecture: **dense, lateral, long-horizon diffusion over the full grid
is over budget.** Per-edge AccumulatorOp propagation projects to ~3236 ms/tick at
30k cells (`sead_operator_toolkit_sandbox_test_results.md` Test 8). Hierarchy-first
strategic awareness — `SlotRange` Sum cell→faction then `EvalEML` urgency at the
parent — is ~15× cheaper (1.45 ms vs 21.09 ms for faction awareness, same probe
Test 5). The architecture must therefore be **local-and-bounded for dense fields,
hierarchical-and-cheap for strategic awareness**, never dense-and-global.

---

## Decision

**Adopt sparse RegionCell mapping: a dense local 2D field is represented as a
bounded set of RegionCells laid out in flat GPU buffers and evolved by the
existing V7.6 `StructuredFieldStencilOp` primitive, under a three-layer model.
The simulation crate gains no map, location, faction, or AI semantics. Every
guardrail that protects the simulation is placed at the RON/Designer/spec
admission layer — dangerous scenarios are rejected before they reach the GPU,
not clamped after.**

**Guardrail placement is two-layered, not designer-only.** The RON/Designer/spec
layer owns *expressive* policy — which fields, formula classes, operators,
horizons, cadences, and source policies a scenario may author, and rejection of
unsafe *authoring* at import/session-build. The **runtime still enforces hard
safety invariants** that must hold regardless of what was authored: horizon
execution caps (`ExecutionHorizonExceedsConfig`), source-cap clamping in
`source_capped_normalized`, finite-coefficient and column-range validation,
ping-pong correctness, and bounded clamping of field/perception values. A
designer-layer mistake or a malformed spec can never produce an unbounded or
out-of-range GPU field, because the primitive's own validation and clamps are
the last line and run unconditionally. Designer admission is the *first* line
(reject early, with good diagnostics); runtime enforcement is the *last* line
(never trust authoring to have been safe).

This decision authorizes the *architecture* and a small set of generic native
primitives (see §7 and the Phase M production-plan amendment). It does **not**
authorize a production mapping runtime, does **not** wire any existing production
pass graph to mapping behavior, and does **not** change any default.

**What "no production mapping runtime" actually means.** It does *not* mean a
large body of hidden compute work remains. The compute already exists: the
`StructuredFieldStencilOp` kernel, its WGSL, ping-pong buffers, CPU oracle, and
horizon/source-cap validation are live in `simthing-gpu`, and Layers 2–3 are
existing AccumulatorOp `SlotRange` Sum + `EvalEML` paths the SEAD probes already
drove on the GPU. "Runtime" here means exactly one remaining step: **wiring a
RegionCell field into a real production session's pass graph at session open so it
runs every tick as part of a shipping scenario** — the ADR's first slice. Every
sandbox probe ran the toolkit directly from a test and recorded "no production
pass graph wiring"; the gate is that a field stays test/opt-in-driven until the
first slice is scheduled. The **Phase M generic natives are unlocked now**; the
**production runtime (first-slice session wiring) remains separately gated** and
begins only after the natives are green (§"First slice").

### Constitutional placement (V7.6 posture preserved)

This ADR preserves the V7.6 constitutional core verbatim and adds nothing that
weakens it:

- **No semantic or map-specific WGSL.** RegionCell fields are evolved by the
  generic `StructuredFieldStencilOp` kernel, which operates on flat buffers,
  dimensions, columns, and kernel weights only. Generality is proven:
  every SEAD/stencil probe records `mapping_semantics_embedded = NO`,
  `simthing_sim_awareness = NO`, `general_tensor = YES`
  (`sead_tensor_stencil_wgsl_sandbox_test_results.md` Test 7,
  `sead_tensor_stencil_refinement_sandbox_test_results.md` Test 9). No new
  semantic kernel is admitted by this ADR.
- **Deterministic bounded-field EML admitted at the RON/Designer layer.** The
  field formula classes (`field_pressure`, `field_urgency`, `field_decay`,
  `bounded_field_update`) are admissible at runtime through the C-8
  `register_formula` path today; the legacy whitelist rejection was a
  wrong-layer admission decision, not a runtime safety property
  (`sead_tensor_stencil_refinement_sandbox_test_results.md` Test 6 finding A;
  `sead_field_intelligence_sandbox_test_results.md` Test 10 finding B). Formula-
  class admission therefore lives at the designer/spec policy layer, exactly as
  V7.6 §1.2 specifies, and is enforced at import/session-build, never mid-tick.
- **`simthing-sim` remains semantic-free.** RegionCell, atlas, gutter, cadence,
  halo, and field-formula concepts exist only in RON, the spec/driver layer, and
  generic GPU toolkit code. The simulation crate sees flat columns and opaque
  `AccumulatorOp` registrations, identically to how it already sees Resource Flow
  arenas without knowing what an arena is.
- **Opt-in, bounded, default-off.** Following the Resource Flow precedent
  (`resource_flow_limited_scenario_class_posture.md`): presence of a map spec is
  *structure*; execution requires an explicit scenario-class/profile opt-in.
  No global default-on. Every field declares hard caps (grid bounds, horizon cap,
  source cap). Unsafe content is rejected at import, not survived at runtime.

### The map, restated constitutionally

```
The spatial tree is the physical map.                    (unchanged, design_v6.md §2)
The political map is overlays on that tree.              (unchanged)
A dense local field is a bounded RegionCell grid in      (this ADR)
  flat GPU buffers, evolved by StructuredFieldStencilOp.
Strategic awareness is hierarchical reduction + parent   (this ADR)
  EvalEML, never dense lateral diffusion.
Perception is a set of filter fields over true fields.   (this ADR, §5)
```

A RegionCell is not a new `SimThingKind` and not a special entity. It is an
**authored mapping-role / profile on a SimThing** (declared in RON at the
spec/Designer layer) backed by a slot range in the dense matrices whose columns
carry field values, addressed positionally by `(width, height, col)` exactly as
`StructuredFieldStencilOp` already requires. `simthing-core` gains **no** new
`SimThingKind` variant; the simulation crate sees only flat columns and opaque
`AccumulatorOp` registrations. Mapping *consumes* the primitive; it is not the
primitive, and it is not runtime authorized here.

---

## The three-layer model

The negative result (dense global diffusion is over budget) forces a separation
of concerns into three layers, each using the mechanism naturally suited to it.
This is the load-bearing structure of the decision.

### Layer 1 — `StructuredFieldStencilOp` for dense local fields

Bounded, tactical, local 2D propagation (threat gradient, suppression pressure,
supply reach, contamination spread) is evolved by the V7.6 primitive over a
RegionCell grid. Constraints are inherited directly from V7.6 §3.1 and the
refinement probe's recommended production constraints:

- Default operators `normalized_stencil` / `source_capped_normalized`. Plain
  `raw_additive` blows up (H=24 max 5.8e7,
  `sead_operator_toolkit_sandbox_test_results.md` Test 1); `clamped_additive`
  saturates and loses gradient; `decayed_normalized` is stable but too weak for
  tactical horizons. `source_capped_normalized` bounds late-horizon growth while
  preserving the H≤8 gradient (`sead_tensor_stencil_refinement_sandbox_test_results.md`
  Test 1, Test 4).
- Default tactical horizon **H ≤ 8**; extended **H ≤ 16** only with
  `allow_extended_horizon` plus a source-cap/decay stability contract. Directional
  signal first reaches a distance-4 corridor at H=8 in every probe that measured
  it (`sead_strategic_horizon_sandbox_test_results.md` Test 1,
  `sead_tensor_stencil_wgsl_sandbox_test_results.md` Test 2).
- **Ping-pong buffers required for H > 1**, proven GPU=CPU bit-exact
  (`sead_tensor_stencil_refinement_sandbox_test_results.md` Test 2, max_error 0.0).
- Source policy is caller-managed one-shot seed then zero (see §6).

### Layer 2 — Hierarchy reductions for strategic awareness

Empire/faction-scale awareness is **not** computed by widening the stencil
horizon. It is computed by `SlotRange` Sum reduction from cells up into parent
threat/resource columns, which is ~15× cheaper than lateral H=8 diffusion for
the same faction-awareness signal (`sead_operator_toolkit_sandbox_test_results.md`
Test 5: hierarchy 1.45 ms vs lateral 21.09 ms). The hybrid split — local stencil
for the tactical gradient, hierarchy reduction for strategic awareness — is the
recommended cost shape and is non-blowing-up
(`sead_operator_toolkit_sandbox_test_results.md` Test 6).

### Layer 3 — `EvalEML` for parent/faction interpretation

Once cell fields are reduced into parent columns, the parent's
personality-weighted interpretation (aggression, risk tolerance → urgency) is a
column-aware `EvalEML` on a **later order band** than the Sum reduction. This is
substrate-real and personality-sensitive when the parent columns are populated
and the EML runs after the Sum band: urgency 571 at aggression 0.2 vs 2535 at
aggression 0.9, ratio 4.44 (`sead_tensor_stencil_refinement_sandbox_test_results.md`
Test 5). Where the parent slot lacks personality columns, urgency is correctly 0
(`sead_tensor_stencil_wgsl_sandbox_test_results.md` Test 5) — the binding, not the
kernel, is the requirement.

**Band ordering invariant:** reduction (Sum) precedes interpretation (EvalEML)
within a tick. Propagation across cells advances by later-band cascade, not
same-band chaining (`sead_field_intelligence_sandbox_test_results.md` Test 0,
`propagation_staging = later-band-cascade`).

---

## AI-as-SimThing: commitments emerge from EML pressures, not a CPU planner

The constitution forbids a CPU-side AI planner or semantic sidecar becoming the
real game (`design_v7_6.md` §1). Mapping must not smuggle one in. Therefore:

- There is **no CPU map planner**. The AI does not traverse the grid on the CPU
  to "decide" anything.
- Strategic commitments (commit to a front, reinforce, withdraw, suppress)
  **emerge as threshold crossings over parent EML pressure fields** — the Layer-3
  urgency/pressure columns — using the existing `Threshold` gate + `EmitEvent`
  substrate, identically to how every other structural commitment in SimThing
  fires. The AI "reads already-computed risk/opportunity fields" rather than
  re-deriving them (`eml_integration_guidance.md` §5.2), and acts when a pressure
  crosses a named threshold.
- This keeps the AI a SimThing: its visible state is float columns; its decisions
  are threshold crossings over those columns; its tunables are designer EML
  formulas. No new decision machinery is introduced.

**Temporal velocity caveat.** EML has no previous-buffer read opcode
(`sead_field_intelligence_sandbox_test_results.md` Test 0,
`previous_values = NO`). Trajectory/velocity pressures must use an **explicit
previous-value column** with a copy band scheduled before the threat-update band;
this works on GPU via `EvalEML` SUB + `ResetTarget` copy at ~14.3% overhead
(`sead_strategic_horizon_sandbox_test_results.md` Test 4, Test 5). Velocity-based
commitments are admissible only through this explicit-column pattern.

---

## Perception fields: filters over true fields

Fog-of-war, espionage, stale intel, confidence, and deception are **not** separate
simulations. Each is a filter field layered over the true field, expressed with
the same RegionCell + stencil + EML machinery:

| Perception filter | Expression |
|---|---|
| Fog-of-war | Per-observer visibility mask field; observed value = true value × visibility, else last-known. |
| Stale intel | Last-observed value column + an age column decayed by `field_decay`; confidence falls as age rises. |
| Confidence | Bounded field in [0,1] driven by observation recency and source reliability; an `EvalEML` `bounded_field_update`. |
| Espionage | Observation events that refresh last-known/confidence on the observer's perception field for specific cells. |
| Deception | An adversary-authored bias field added into the observer's *perceived* field, never into the true field. |

The true field is authoritative and shared; each faction's perception is its own
column range filtered from it. Because every filter is a bounded-field EML class
admitted at the designer layer, perception adds no new runtime primitive and no
new WGSL. Confidence and deception must be clamped/bounded (`bounded_field_update`)
so a perception field can never inject unbounded values back toward the true field.

**Perception write-boundary (hard invariant).** Perceived, last-known, confidence,
and deception fields **never write back into the true authoritative field**. Data
flows true → perceived only. The single sanctioned exception is an **explicit
gameplay event** (e.g. an espionage success, a discovery, a sensor ping) routed
through the normal event/`BoundaryRequest` path, which may update authoritative
state deliberately and auditably. There is no implicit or stencil-level path by
which a perception/deception column feeds the true field: the reduction and
stencil bindings are directional (true-col → perceived-col), and any op that would
write a perceived/deception column into a true column is rejected at spec
admission. This keeps deception a property of *what an observer believes*, never
a corruption of ground truth.

---

## Optimization doctrine

Each optimization is classified **adopted**, **provisional**, or **deferred**,
with the layer it lives at and the caveat that must travel with it. Classifications
are the sandbox probes' own ADR-adoption tables, ratified here.

| Optimization | Classification | Layer | Caveat that must travel with it |
|---|---|---|---|
| **Cadence tiers** | **Adopted** | RON/Designer policy | Author a cadence tier per field class (EveryTick/4/10/60/Event); scheduler skips non-due fields. Deterministic, replay-safe (1580 dispatches avoided / 120 ticks, deterministic replay = YES, toolkit Test 3). Field-type quality depends on cap/decay authoring, not on the tier mechanism. |
| **Dirty macro-region skipping** | **Adopted** | driver/scheduler | Skip clean macro-regions before command-buffer construction. 62.5% skip ratio, **false_skips = 0** (toolkit Test 5). Conservative false-schedules are acceptable; false-skips are not. Skip ratio collapses at long horizons (96% dirty at H=16, operator-toolkit Test 2) — pair with hierarchy-first refresh rather than long-horizon dense diffusion. |
| **Caller-managed source policy** | **Adopted (v1)** | runtime substrate API | `CallerManagedOneShotSeedThenZero` is the v1 default: caller seeds once, clears the source column after the initial hop, runs configured horizon. Uncleared sources pump to cap (growth_ratio 2.13, remedial Test 4). The primitive does not identify or clear sources automatically. |
| **Atlas batching** | **Provisional** | driver/scheduler + runtime substrate | Pack scheduled tiles into one atlas, one dispatch per atlas (N=64 speedup 59.6×, toolkit Test 2). **Gutter ≥ H is the short-term-safe isolation policy but expensive: 10×10 H=8 carries a 6.76× VRAM multiplier (576% overhead, remedial Test 2).** Per-tile seed clearing is mandatory. **Production acceptance requires bit-exact parity against an exact per-tile-protocol CPU oracle** (a CPU model that replays the same per-tile seed-clear + gutter + boundary protocol the GPU atlas uses), **not** corridor-t44 agreement alone: t44 ≤0.016 was the sandbox *tactical-signal* metric, but full-tile L∞ stayed ~409 vs a naive standalone oracle because the standalone oracle did not model the atlas protocol. The acceptance oracle must model that protocol so full-tile parity is meaningful. Local-bounds tile-rect metadata is the long-term preferred isolation policy (deferred pending API design, remedial Test 3). **Ratified (M-4A, Opus 2026-05-28):** for homogeneous square batches, **algebraic tile-local mask at G=0 is the preferred isolation candidate** (full-tile protocol-oracle parity, 1.0× VRAM vs 6.76× for gutter); **physical gutter is the fallback** (required whenever masking is not configured/admitted or the layout is not homogeneous-square). Ratification governs *which isolation design an M-4 implementation PR pursues first*; it does **not** mark atlas Adopted and does **not** authorize implementation — atlas stays Provisional/unimplemented and `request_atlas_batching` stays rejected at admission until an M-4 PR satisfies the ratified acceptance gate. See `mapping_atlas_algebraic_mask_sandbox_test_results.md`, [`mapping_atlas_batching_isolation_design_note.md`](../workshop/mapping_atlas_batching_isolation_design_note.md), and `docs/reviews/m4_m4a_first_slice_oversight_opus_review.md`. |

| **Active frontier + halo** | **Provisional** | driver/scheduler + runtime substrate | H-hop / per-hop halo only, with CPU-oracle parity (halo_H8 max_error 0.0026 vs full grid, toolkit Test 7; 0.003 on safe atlas, remedial Test 6). Speedup is sparsity-dependent and modest on small grids (~1.5–1.9×); meaningful at ≥25% active on larger grids (refinement Test 7). |
| **Behavioral source policy** | **Deferred** | runtime substrate API design | Requires explicit source identity (`source_mask` or separate seed buffer). CPU models of a seed buffer and a source-mask match the caller-managed baseline and are semantic-free (remedial Test 5 options A/B viable). **Column-wide `source_col` zeroing is banned** — it corrupts propagated state held in that column (option C rejected). No behavioral-policy WGSL until source identity lands. |

### Ratified amendments

| Topic | Evidence | Change | Status |
|---|---|---|---|
| Atlas isolation (homogeneous square) | M-4A sandbox (`mapping_atlas_algebraic_mask_sandbox_test_results.md`) | Algebraic tile-local mask G=0 is the **preferred isolation candidate**; physical G≥H is **fallback** | **Ratified (Opus, 2026-05-28)** under human delegation — `docs/reviews/m4_m4a_first_slice_oversight_opus_review.md`. Atlas remains **Provisional/unimplemented**; this ratifies the *isolation policy*, not implementation. |

The acceptance gate a future M-4 atlas implementation PR must satisfy is **binding** and
recorded in [`mapping_atlas_batching_isolation_design_note.md`](../workshop/mapping_atlas_batching_isolation_design_note.md) §11
and in the oversight memo. Mixed-size `LocalBoundsMetadata` **remains deferred**.

### Hard prohibitions (the stop conditions, restated as doctrine)

- `ActiveOnlyExperimentalNoHalo` is **never** production-authorized. Active-only
  masks fail correctness in every probe (max_error 235 at active-only, toolkit
  Test 7). Halo is mandatory for any active-mask use.
- Atlas batching is **never** authorized without an explicit gutter/isolation
  policy **and** VRAM-multiplier reporting.
- Behavioral source policy is **never** approved without an explicit source
  identity buffer.
- Long-horizon dense diffusion as a strategic-awareness mechanism is **rejected**;
  strategic awareness is hierarchy + parent EML (the three-layer model).

---

## Debug / observability surfaces (required)

A mapping debug surface must expose, at minimum: dispatch count; atlas occupancy;
VRAM multiplier; dirty ratio; active-mask ratio; field max and L1 norm; source
policy in effect; horizon (configured and executed); parent reduction outputs; and
EML outputs (per-formula last value, NaN/clamp counters). VRAM-multiplier reporting
for atlas batching is mandatory, not optional — atlas adoption is conditioned on it.

---

## Consequences

**Positive.** The map is expressible today on generic, hardened, opt-in toolkit
code with no new semantic WGSL and no constitutional erosion. The three-layer
model keeps the expensive operation (dense diffusion) bounded and local while
strategic awareness stays cheap. Perception, deception, and AI commitments reuse
existing primitives. The combined optimization stack projects to ~5.1 ms/tick at
30k cells with safe gutter + dirty + halo (remedial Test 8), versus ~3236 ms for
naive per-edge AccumulatorOp.

**Costs accepted.** Atlas batching's gutter-≥-H VRAM tax (6.76× on small tiles)
is accepted as a short-term cost with local-bounds metadata as the named
long-term remedy. Behavioral source policy and local-bounds isolation are
deferred to explicit API-design gates. Long-range gradient quality and AI
decision quality remain gameplay-evaluation questions the sandboxes explicitly
cannot settle; this ADR establishes technical viability and constitutional
safety only.

**Invariants added.** See the new "Mapping (Sparse RegionCell)" rows appended to
`docs/invariants.md`.

---

## Stop conditions (this ADR must not be read as authorizing any of these)

- It does **not** authorize a production mapping runtime by implication.
- It does **not** claim active masks are production-safe without halo.
- It does **not** approve behavioral source policy without source identity.
- It does **not** omit VRAM-multiplier reporting for atlas batching.
- It does **not** route existing production pass graphs through mapping behavior
  by default.

---

## Completion criteria (met by this document)

- [x] Classifies all key mapping primitives and optimizations (three-layer model
      + optimization doctrine table).
- [x] Locks or explicitly defers the open questions (atlas isolation: provisional
      with gutter≥H now / local-bounds later; behavioral source policy: deferred
      pending source identity; active mask: provisional, halo-mandatory,
      active-only banned).
- [x] Names the first scenario-level mapping slice **only after architecture
      approval** — deferred to §"First slice" below, gated on approval.
- [x] Preserves the V7.6 constitutional posture (no semantic WGSL; designer-layer
      EML admission; `simthing-sim` semantic-free; opt-in/bounded/default-off).
- [x] Includes test-backed performance/VRAM caveats throughout.

### First slice (named — ADR approved 2026-05-28; begins after Phase M natives land)

With the architecture approved, the first scenario-level mapping slice is **named
and ready to schedule** once the Phase M generic natives (M-1–M-3) are green. It is
a **single-faction tactical suppression field on one bounded theater**: one
RegionCell grid (≤ 32×32), `source_capped_normalized` at H≤8, one-shot
seed-then-zero source policy, `EveryTick` cadence, dirty macro-region skipping,
**no atlas** (single grid), **no active mask**, Sum reduction into one parent
threat column, one `field_urgency` EvalEML on the parent. This slice exercises
Layers 1–3 end-to-end with only adopted optimizations and zero
provisional/deferred features. Atlas batching and active halo enter only at a
later slice once a multi-theater scenario is named and the VRAM budget is
approved.

**Update (2026-05-19):** Phase **M-first-slice** runtime landed in `simthing-driver`
(`FirstSliceMappingSession`) behind explicit `MappingExecutionProfile::SparseRegionFieldV1`
opt-in. Single-grid edge-boundary parity and designer-facing RegionField budget preview
landed. **Not** wired into default `SimSession` pass graph; `MappingExecutionProfile`
default remains `Disabled`. Atlas batching (M-4) remains provisional and unimplemented.
Classification unchanged.

**Update (2026-05-28):** First-slice runtime hardened through **R1** (no-readback GPU-state
ownership), **R2** (GPU-resident Layer 1→2→3 bridge: `StructuredFieldStencilOp` →
`AccumulatorOpSession` → `SlotRange` Sum → `EvalEML`, `reduction_stencil_readbacks=0` on the
hot path), and **R3** (readiness/observability parking). **Opus accepted the R3 runtime as a
stable base** (`docs/reviews/m4_m4a_first_slice_oversight_opus_review.md`; 28/28
first-slice tests green, independently re-run on GPU). Default remains `Disabled`;
`simthing-sim` remains map-free; no atlas/active-mask/perception/`source_mask` landed.
Named next step: a product-facing first-slice scenario fixture (single grid, no atlas) —
**not** atlas implementation. Known scale caveat: the bridge uses per-slot queue writes for
resource/parent-weight columns (fine and reported at 10×10; must be redesigned and measured
before atlas/multi-field scale).

**Update (2026-05-28, product fixture):** The product-facing first-slice scenario fixture
landed as an opt-in test/RON fixture over the accepted runtime: one grid,
source_capped_normalized, H<=8, caller-managed seed-only clear, dirty scheduling,
SlotRange Sum reduction, and parent field_urgency EvalEML. It proves default-off behavior,
explicit `SparseRegionFieldV1` opt-in, hot-path `reduction_stencil_readbacks=0`, finite
field propagation, and personality/weight-sensitive urgency. This is a landing note only:
no atlas, M-4A masking, active mask, perception, map residency, source identity, semantic
WGSL, default session wiring, or `simthing-sim` map awareness is authorized or landed.

---

## Read order for agents touching mapping

1. `docs/invariants.md` — including the new Mapping rows.
2. This ADR.
3. `docs/design_v7_6.md` — `StructuredFieldStencilOp` constraints (§3.1).
4. `docs/design_v7_7.md` — mapping + SEAD surfacing.
5. The cited `docs/tests/` evidence before changing any classification.
