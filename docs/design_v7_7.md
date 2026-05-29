# SimThing — Design v7.7 Amendment

> **Status:** Active amendment (2026-05-28). Surfaces the **Mapping (Sparse RegionCell)**
> architecture and the **SEAD field-intelligence** surfacing. Supersedes nothing in the
> V7.5 constitutional core or the V7.6 amendment; it **extends** them with the architectural
> decision recorded in [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md).
>
> **Companion:** [`design_v7.md`](design_v7.md) (V7.5 base) · [`design_v7_6.md`](design_v7_6.md)
> (StructuredFieldStencilOp) · [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md)
> (the decision) · [`accumulator_op_v2_production_plan.md`](accumulator_op_v2_production_plan.md)
> (Phase M natives)

---

## 1. What changed in v7.7

V7.7 preserves the V7.5 constitutional core and the V7.6 guardrail posture in full:

- SimThing remains the single substrate abstraction.
- `simthing-sim` remains semantic-free — and now also map-free, RegionCell-free,
  atlas-free, cadence-free, halo-free.
- Resource Flow remains opt-in and not global default-on.
- GPU primitives remain deterministic, bounded, replayable, data-parallel.
- No CPU-side AI planner or semantic sidecar may become the real game — and the
  mapping AI explicitly does **not** introduce one (§3).
- **No mapping/location runtime is authorized by this amendment.** V7.7 surfaces
  the *architecture*; the **Phase M generic natives are unlocked**, but production
  runtime — wiring a RegionCell field into a real session's pass graph at session
  open (the ADR's first slice) — remains separately gated and begins only after
  the natives land. This is not a hidden body of compute work: the kernel, WGSL,
  ping-pong, and Layers 2–3 paths already exist; runtime is the session-wiring
  step, not new GPU math (see ADR §"What 'no production mapping runtime' actually
  means").
- No semantic or map-specific WGSL.

V7.7 adds exactly two surfaced concepts, both already evidenced and parked:

1. **Mapping (Sparse RegionCell)** — the architectural decision for how dense
   local spatial fields are represented and evolved (§2). Decided in
   `adr/mapping_sparse_regioncell.md`.
2. **SEAD field-intelligence surfacing** — the three-layer awareness model
   (dense-local field → hierarchy reduction → parent EvalEML) and its
   commitments-as-threshold-crossings discipline (§3).

Neither adds a new GPU primitive, a new WGSL kernel, a new `AccumulatorRole`, or a
default change. Both compile onto the V7.6 `StructuredFieldStencilOp` primitive and
the existing AccumulatorOp v2 reduction/EML/threshold substrate.

**Governance amendment (promoted 2026-05-29):** V7.7 also makes **gating & documentation
governance** constitutional (§5) — a two-tier model (fast lane vs gated) plus documentation
discipline that keeps drift protection binding while removing the per-slice ceremony that was
stalling implementation. It adds no primitive and relaxes no enforcement; it governs *process*, not
substrate.

---

## 2. Mapping (Sparse RegionCell)

The full decision, classifications, and caveats live in
[`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md). This section
surfaces the constitutional placement only.

### 2.1 What the map is

```
The spatial tree is the physical map.            (unchanged — design_v6.md §2)
The political map is overlays on that tree.      (unchanged)
A dense local field is a bounded RegionCell grid (v7.7)
  in flat GPU buffers, evolved by StructuredFieldStencilOp.
Strategic awareness is hierarchy reduction       (v7.7)
  + parent EvalEML, never dense lateral diffusion.
Perception is filter fields over true fields.    (v7.7)
```

A RegionCell is a **spec/RON-authored mapping-role / profile on a SimThing**,
backed by a slot range in the dense matrices whose columns carry field values,
addressed positionally by `(width, height, col)` — the layout
`StructuredFieldStencilOp` already requires. It is **not** a new `SimThingKind`
and **not** a special entity; `simthing-core` gains no new kind variant. Mapping
consumes the V7.6 primitive; it is not the primitive.

### 2.2 The three-layer model

| Layer | Mechanism | Why |
|---|---|---|
| 1 — Dense local field | `StructuredFieldStencilOp` over a RegionCell grid (`source_capped_normalized`, H≤8 tactical, ping-pong for H>1) | Tactical gradients (threat, suppression, supply, contamination) are local and bounded. |
| 2 — Strategic awareness | `SlotRange` Sum reduction cell→parent threat/resource columns | ~15× cheaper than lateral H=8 diffusion for faction awareness (`sead_operator_toolkit_sandbox_test_results.md` Test 5: 1.45 ms vs 21.09 ms). |
| 3 — Parent interpretation | Column-aware `EvalEML` on a **later order band** than the Sum | Personality-weighted urgency (aggression/risk → urgency ratio 4.44, `sead_tensor_stencil_refinement_sandbox_test_results.md` Test 5). |

The load-bearing negative result from the SEAD probes: **dense, lateral,
long-horizon diffusion over the full grid is over budget** (~3236 ms/tick at 30k
cells per-edge, `sead_operator_toolkit_sandbox_test_results.md` Test 8). The
architecture is therefore local-and-bounded for fields, hierarchical-and-cheap
for awareness — never dense-and-global.

### 2.3 Perception as filters

Fog-of-war, stale intel, confidence, espionage, and deception are filter fields
over the authoritative true field, each a bounded-field EML class
(`bounded_field_update` / `field_decay`) admitted at the designer layer. No new
primitive; no new WGSL. **Write-boundary (hard invariant):** data flows true →
perceived only; perceived/deception fields never write back into the true
authoritative field except through an explicit gameplay event (espionage success,
discovery, sensor ping) routed via the event/`BoundaryRequest` path. Any op
writing a perceived/deception column into a true column is rejected at spec
admission. Deception is what an observer *believes*, never a corruption of ground
truth.

### 2.4 Optimization doctrine (surfaced)

Classifications are ratified in the ADR. Summary:

- **Adopted (now):** cadence tiers (RON/Designer), dirty macro-region skipping
  (driver/scheduler), caller-managed one-shot-seed-then-zero source policy (v1
  runtime API).
- **Provisional:** atlas batching (still unimplemented; `request_atlas_batching` rejected
  at admission). Isolation policy **ratified (Opus 2026-05-28):** for homogeneous square
  batches, **algebraic tile-local mask G=0 is the preferred isolation candidate** (1.0×
  VRAM, full-tile protocol-oracle parity); **physical gutter G≥H is the fallback** (**6.76×
  VRAM on 10×10 H=8**, VRAM-multiplier reporting mandatory); local-bounds metadata remains
  the deferred long-term policy. Ratifying the isolation policy does **not** authorize
  implementation — a gate-passing M-4 PR is still required (`docs/reviews/m4_m4a_first_slice_oversight_opus_review.md`).
  Also provisional: active frontier + H-hop/per-hop halo (CPU-oracle parity required;
  active-only **banned**).
- **Deferred:** behavioral source policy (needs explicit `source_mask`/seed
  buffer; column-wide source zeroing **banned**).

### 2.5 Constitutional guarantees (unchanged from V7.6)

- No semantic/map-specific WGSL — generality proven across all probes
  (`general_tensor = YES`, `mapping_semantics_embedded = NO`).
- Deterministic bounded-field EML admitted at the RON/Designer/spec layer; the
  legacy whitelist rejection was wrong-layer admission, not runtime safety.
- **Guardrail placement is two-layered:** RON/Designer/spec owns expressive
  policy and rejects unsafe authoring at import; the runtime still enforces hard
  safety unconditionally (horizon execution caps, source-cap clamp, column/finite
  validation, ping-pong correctness, bounded field/perception clamps). Authoring
  is never trusted to have been safe.
- `simthing-sim` never sees RegionCell/atlas/gutter/cadence/halo/field-formula
  concepts.
- Opt-in, bounded, default-off — spec presence is structure; execution requires
  explicit scenario-class/profile opt-in (Resource Flow precedent).

---

## 3. SEAD field-intelligence surfacing

SEAD (the field-intelligence awareness probes) is the proving ground for the
three-layer model and the source of its discipline. V7.7 surfaces the awareness
model as the canonical consumer of mapping.

### 3.1 AI-as-SimThing: no CPU planner

Strategic commitments emerge as **threshold crossings over parent EML pressure
fields** (Layer-3 urgency/pressure columns), using the existing `Threshold` gate
+ `EmitEvent` substrate — identical to every other structural commitment in
SimThing. There is no CPU map planner. The AI reads already-computed
risk/opportunity fields and acts when a pressure crosses a named threshold. Its
state is float columns; its decisions are threshold crossings; its tunables are
designer EML formulas. This keeps the AI a SimThing and adds no decision machinery.

### 3.2 Substrate facts established by the probes

| Fact | Evidence |
|---|---|
| Propagation advances by later-band cascade, not same-band chaining | `sead_field_intelligence_sandbox_test_results.md` Test 0 (`later-band-cascade`) |
| GPU `EvalEML` urgency is bit-exact and personality-sensitive | field-intel Test 1; refinement Test 5 (ratio 4.44) |
| Decay without erasure via `ScaleTarget` | field-intel Test 4 (monotone decay, tick20_max 0.12) |
| Hierarchy reduction ≈15× cheaper than lateral diffusion | operator-toolkit Test 5 |
| Ping-pong H>1 is GPU=CPU bit-exact | refinement Test 2 (max_error 0.0) |
| Directional signal first reaches distance-4 corridor at H=8 | strategic-horizon Test 1; tensor-stencil-wgsl Test 2 |

### 3.3 Velocity / trajectory caveat

EML has **no previous-buffer read opcode** (`previous_values = NO`, field-intel
Test 0). Trajectory/velocity pressures require an **explicit previous-value
column** with a copy band scheduled before the threat-update band:
`EvalEML` SUB + `ResetTarget` copy, ~14.3% overhead
(`sead_strategic_horizon_sandbox_test_results.md` Test 4–5). Velocity-based
commitments are admissible only through this explicit-column pattern.

### 3.4 Honest limits

Long-range gradient quality past H≈8–16 and AI *decision* quality are gameplay-
evaluation questions the sandboxes explicitly cannot settle. PF-based boundary
skip is observability-only until its epsilon/tick threshold is tuned (skip
simulation barely failed epsilon: max_error 0.0106 > 0.01,
`sead_strategic_horizon_sandbox_test_results.md` Test 7). V7.7 surfaces technical
viability and constitutional safety, not balance.

---

## 4. Greenfield admission criteria (unchanged from V7.6 §4)

New GPU/EML work is allowed when it is a generic primitive (not scenario
semantics), opt-in (no production default changes), does not impair Resource
Flow / E-11B / Phase T / `simthing-sim` behavior, and is covered by regression
tests with documented admission constraints. Mapping and SEAD surfacing meet
these: they add no primitive, change no default, and are covered by the cited
sandbox evidence.

---

## 5. Gating & documentation governance (V7.7 constitutional — promoted 2026-05-29)

The admission criteria in §4 say *what* may be built. This section is constitutional on *how much
process and documentation a change carries* — so drift discipline stays binding without the
ceremony treadmill that stalls implementation and burns effort. The operational detail lives in
[`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md); the binding
principles are here.

**Two lanes, classified before work starts:**

- **Tier-1 (fast lane).** A change that is *within an already-accepted design*, *generic substrate*
  (no semantic WGSL, no `simthing-sim` map/Gadget/Personality awareness), *opt-in / default-off*,
  *CPU-oracle-parity-backed where it touches compute*, and *reversible* ships in **one PR + one test
  report + one compact status-table update**. No separate design-review memo, no acceptance memo, no
  parking packet, no consolidated review packet, and no R-series hygiene pass unless a real defect is
  found. Any Opus/design review is post-hoc and non-blocking.
- **Tier-2 (gated).** A change that touches a **binding invariant** (`docs/invariants.md`),
  introduces **default-on behavior** or default `SimSession` pass-graph wiring, is **new architecture
  / irreversible** or carries a **genuinely open design question**, or is on the **standing
  prohibition list**, keeps the full cadence: design review → Opus/product acceptance →
  implementation. This is where the real safety lives; it is not shortcut.

**Documentation discipline (constitutional):**

- Standing posture ("no semantic WGSL / no default wiring / `simthing-sim` map-free / defaults
  unchanged") is **asserted once** per PR in its test report — not duplicated across the production
  plan, guidance, state, design notes, todo, and worklog. The binding rules live once in
  `docs/invariants.md`.
- Active guidance/state docs carry a **compact status table**; per-slice narrative history lives in
  the append-only `docs/worklog.md`. Verbose per-slice blocks are collapsed when a file is touched,
  never grown.
- No packet proliferation; no reflexive R-series. **Anti-loop stop rule:** an agent about to write a
  *third* meta-document for one slice is in the ceremony loop and must ship the code instead.

**What is explicitly retained (drift protection is not relaxed):** `invariants.md` stays binding and
**any change to it is Tier-2**; test reports with oracle parity and a single posture attestation stay
mandatory (that is the real, cheap safety); the standing prohibition list (§6) is unchanged. The
trade is precise — remove **redundant narration and redundant gates**, never **enforcement**.

---

## 6. Explicit non-goals (V7.7)

- **No mapping/location runtime from this amendment.** Surfacing ≠ runtime.
- No production pass graph wired to mapping behavior by default.
- No `ActiveOnlyExperimentalNoHalo` production authorization.
- No atlas batching without explicit gutter/isolation policy **and** VRAM-
  multiplier reporting.
- No behavioral source policy without explicit source identity.
- No CPU AI map planner / semantic sidecar.
- No semantic WGSL. No new `AccumulatorRole`. No Resource Flow default-on.
- No `simthing-sim` map awareness.

---

## 7. Parked state (2026-05-28)

The Mapping ADR is **drafted and accepted at the architecture level**. The next
work item is the **Phase M natives** (the few generic primitives mapping needs —
see the production-plan amendment) followed by the **first scenario-level slice**
named in the ADR (single-faction tactical suppression field, single ≤32×32 grid,
adopted-only optimizations). No mapping runtime is implemented. No production
pass graph is wired. Resource Flow defaults remain unchanged. `simthing-sim`
remains semantic-free.

### 7.1 Confirmed current posture

```text
Mapping (Sparse RegionCell) is decided at the architecture level in
adr/mapping_sparse_regioncell.md. No mapping runtime is authorized.

The map is: physical spatial tree (unchanged) + political overlays (unchanged)
+ bounded RegionCell fields evolved by the V7.6 StructuredFieldStencilOp
(new) + hierarchy reduction and parent EvalEML for strategic awareness (new)
+ perception filter fields over true fields (new).

Three-layer model:
  L1 StructuredFieldStencilOp — dense local fields, source_capped_normalized,
     H<=8 tactical, ping-pong for H>1, caller-managed one-shot-seed-then-zero.
  L2 SlotRange Sum reduction — cell to parent columns, ~15x cheaper than
     lateral diffusion for strategic awareness.
  L3 EvalEML on a later order band — parent personality-weighted interpretation.

AI is a SimThing: commitments emerge as threshold crossings over parent EML
pressure fields. No CPU map planner.

Optimization doctrine:
  adopted     — cadence tiers, dirty macro-region skipping, caller-managed source.
  provisional — atlas batching (unimplemented; isolation ratified 2026-05-28:
                algebraic tile-local mask G=0 preferred for homogeneous square
                batches at 1.0x VRAM, physical gutter G>=H fallback at 6.76x,
                reporting mandatory, local-bounds long-term; implementation still
                gated on a §11-gate-passing PR), active+H-hop halo (active-only banned).
  deferred    — behavioral source policy (needs source identity; column zero banned).

Velocity needs an explicit previous-value column (EML has no previous read).
StructuredFieldStencilOp remains live, opt-in, hardened, inert by default.
simthing-sim remains semantic-free.
```

**Evidence preserved:** the cited `docs/tests/` mapping and SEAD probe results
(see ADR header) and `docs/workshop/archive/mapping/mapping_optimization_remedial_candidate_notes.md`.

---

## 8. Read order (v7.7 addition)

Insert after `design_v7_6.md` in the V7 read order:

1. `docs/invariants.md` — including the new Mapping rows.
2. `docs/adr/mapping_sparse_regioncell.md` — the decision.
3. `docs/design_v7_7.md` — this amendment (surfacing + §5 gating governance).
4. `docs/workshop/phase_m_gating_and_doc_policy.md` — operational gating/doc policy (constitutional per §5); read before picking up any slice.
5. The cited `docs/tests/` mapping + SEAD evidence before changing any
   classification.
