# Production Transfer / Emission Registration Ownership — Opus Design Review

**Status:** Design-review gate (Opus). No implementation in this PR.
**Authors:** Opus (design authority), under SimThing v7.5 constitution.
**Date:** 2026-05-27.
**Companion docs:** [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md) · [`../adr_accumulator_op_v2.md`](../adr_accumulator_op_v2.md) · [`../adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md) · [`../workshop/c8_eml_transfer_intensity_design.md`](../workshop/c8_eml_transfer_intensity_design.md) · [`../workshop/workshop_current_state.md`](../workshop/workshop_current_state.md).

> **Constitutional posture (v7.5).** The runtime substrate is `AccumulatorOp`.
> Transfer/emission/recipe **policy** belongs in `simthing-spec` (authoring)
> and `simthing-driver` (compilation). `simthing-sim` receives flat
> `AccumulatorOp` registrations only. No new WGSL. No new `AccumulatorOp`
> primitive. No CPU production fallback peer. No semantic registry in
> `simthing-sim`. No weakening of exact transfer conservation. No folding of
> hard-currency transfers into continuous Resource Flow.
>
> This memo audits the current substrate, locates the ownership gap, and
> defines the registration source of truth that closes it without violating
> any of the above. The narrowed Cursor handoff at §7 stays inside those lines.

---

## 1. Current-state audit

### 1.1 Substrate (landed)

The GPU-side substrate for production transfer and emission is complete and
sunset-protected:

| Layer | Type / module | Purpose | Status |
|-------|---------------|---------|--------|
| `simthing-core` | `AccumulatorOp` / `CombineFn` / `GateSpec` / `ConsumeMode` / `SourceSpec` | Flat substrate primitive | Landed (A-2) |
| `simthing-core` | `EmlExpressionRegistry`, `EmlConsumerKind`, `EmlExecutionClass` | EML admission policy | Landed (A-3, C-8a) |
| `simthing-core::accumulator_op_builder` | `emit_on_threshold`, `resource_transfer_discrete`, `conjunctive_recipe` builders + `*Registration` carriers (`EmitOnThresholdRegistration`, `DiscreteTransferRegistration`, `ConjunctiveRecipeRegistration`) | Author-side flat-op builders + boundary-refresh helpers | Landed (E-1, E-2A, E-3, E-3R) |
| `simthing-gpu::transfer_accumulator` | `TransferRegistration`, `plan_transfer_ops`, `encode_transfer_plan`, `*_registration_to_transfer` bridges | C-8c exact transfer planner; `ExactDeterministic` only; same-band consumed-input contention rejected | Landed (C-8c / C-8c remedial) |
| `simthing-gpu::emission_accumulator` | `EmissionRegistration`, `EmissionFormula`, `plan_emission_ops`, `encode_emission_plan`, `EmissionOpPlanSignature` | C-8d emission planner (`IdentityFloor` / `Constant` / `EvalEML`); `ExactDeterministic` baseline; `max_emit` explicitly rejected until shader clamp | Landed (C-8d / C-8d remedial) |
| `simthing-gpu::world_state` | `ensure_transfer_accumulator`, `ensure_emission_accumulator`, `upload_accumulator_transfer_ops`, `set_transfer_dispatch`, `set_emission_dispatch` | Session attachment / op upload / dispatch toggles | Landed |
| `simthing-sim::boundary` | `PipelineFlags::use_accumulator_transfer` / `use_accumulator_emission` (default **false**), `sync_accumulator_{transfer,emission}_session` | Boundary-level flag gating, no semantic awareness | Landed |

### 1.2 Ownership gap (the warning this review answers)

`simthing-spec` authors arenas, couplings, fission policy, admission caps
([`spec/resource_flow.rs::ResourceFlowSpec`](../../crates/simthing-spec/src/spec/resource_flow.rs)),
and properties, **but does not author transfer, emission, or recipe content.**
`simthing-driver` materializes `ArenaRegistry` from a `ResourceFlowSpec`
([`resource_flow_compile.rs`](../../crates/simthing-driver/src/resource_flow_compile.rs))
and owns the flat-star allocation compilation path (`arena_allocation_plan`,
`arena_allocation_sync`), but **emits no transfer / emission / recipe
registrations** — there is no `compile_transfer_*` / `compile_emission_*`
analogue.

Today, transfer and emission registrations enter the GPU only through
**test code constructing the `simthing-core` builders directly** and handing
the resulting `*Registration` values to `simthing-gpu` bridge functions
(`discrete_transfer_registrations_to_transfer`,
`conjunctive_recipe_registrations_to_transfer`,
`plan_emission_ops`). The production session has no authored origin and no
boundary-refresh path. That is the gap.

The C-8 line in `accumulator_op_v2_production_plan.md` records this directly:

> **Open after C-8:** production transfer/emission registration ownership
> (spec/builder integration); shared-input cross-pool contention (D-1);
> Soft/Fast EML classes remain future-gated.

E-1 / E-2A / E-3 / E-3R built the **builder surface**; E-4 was the placeholder
for "RON fixture format and session integration." The work in this memo is
the precondition for a usefully-narrowed E-4: before we author a RON shape,
we must decide where the source of truth lives, what crosses crate
boundaries, and how the lifecycle is sequenced.

### 1.3 What the substrate already guarantees (do not re-derive)

These properties are landed and out of scope for the registration ownership
question — they constrain it, they do not have to be re-litigated:

- **Exact source-debit conservation** for discrete transfers
  ([`transfer_accumulator.rs`](../../crates/simthing-gpu/src/transfer_accumulator.rs)
  — `SubtractFromSource` / `SubtractFromAllInputs`; clamped at WGSL by
  the C-8c defensive debit).
- **Same-band consumed-input contention rejected at plan time** (C-8c
  remedial / `TransferPlanError::ContendedConsumedInput`). Same-target
  contention remains allowed via atomic target adds.
- **EML admission policy:** `EmlExpressionRegistry::assert_consumer_admissible`
  enforces `EmlConsumerKind` × `EmlExecutionClass` matrix; C-8 baseline admits
  `ExactDeterministic` only.
- **Stable `reg_idx`** for emission records (`combine_b` slot; C-8d remedial
  `EmissionOpPlanSignature` keys on stable identifiers).
- **No `max_emit` enforcement** until a GPU clamp is designed
  (`EmissionPlanError::MaxEmitUnsupported`); registration ownership must
  not silently re-introduce a CPU-side cap.
- **Generation-based op-plan invalidation** for both transfer and emission
  (input-list / op signatures). Refreshes are cheap when content is unchanged.

---

## 2. Ownership model

### 2.1 Source of truth: spec authors, driver compiles, sim receives ops

The single coherent answer that preserves v7.5:

```
    AUTHORING                COMPILATION                EXECUTION
    ─────────                ───────────                ─────────
    simthing-spec   ──▶  simthing-driver  ──▶  simthing-sim / simthing-gpu
    (RON content)        (session compile)       (flat AccumulatorOp ops)
       │                       │                          │
       │ ResourceTransferSpec  │ uses E-2A / E-3 / E-1    │ ensure_* sessions,
       │ ResourceRecipeSpec    │ builders in              │ plan_*_ops,
       │ ResourceEmissionSpec  │ simthing-core, then      │ encode_*_plan,
       │ EmitOnThresholdSpec   │ bridges in simthing-gpu  │ dispatch
       │ (+ EML formula refs)  │ to TransferRegistration  │
       │                       │ / EmissionRegistration   │
```

Concretely the assignment is:

| Question | Answer | Notes |
|----------|--------|-------|
| Authoring source-of-truth | **`simthing-spec`** | New RON-backed types in `spec/resource_flow.rs` (or a sibling `spec/resource_transfer.rs` / `spec/resource_emission.rs`). |
| EML formula authoring | **`simthing-spec`** | Formulas reference `EmlTreeId` declared in a spec-level EML formula table; the driver hands the resolved `EmlExpressionRegistry` content to the GPU. EML stays `ExactDeterministic` for C-8 production consumers. |
| Compilation | **`simthing-driver`** | Per-session compile step alongside `compile_and_materialize_resource_flow`, producing `Vec<DiscreteTransferRegistration>`, `Vec<ConjunctiveRecipeRegistration>`, `Vec<EmissionRegistration>` (the last typed as `simthing_gpu::EmissionRegistration` via a thin re-export, or as a driver-local mirror — see §3.2). |
| Lifecycle / refresh | **`simthing-driver`** session-owned registries (`TransferRegistry`, `EmissionRegistry`) mirroring the `ArenaRegistry` pattern. | Subtree-incremental refresh on structural mutation; never global rebuild. |
| Feeder / boundary | **No semantic ownership.** Boundary-layer `BoundaryProtocol` syncs sessions, takes flag-off rejection paths, but does not author or compile registrations. | The existing `PipelineFlags::use_accumulator_{transfer,emission}` gate stays where it is. |
| Legacy economic systems | **None.** Legacy intensity (S-2), reduction (S-4), velocity (S-5), threshold (S-6), intent (S-1), overlay (S-3) are deleted. There is no pre-AccumulatorOp economic peer to keep alive. | The C-8 line is the only production transfer/emission path. |

### 2.2 Why this assignment and not the alternatives

- **Not `simthing-sim`.** A semantic registry in `simthing-sim` would
  re-introduce policy in the kernel-adjacent crate, which v7.5 forbids. The
  E-8 invariant — "`AccumulatorRole` is compile-time spec metadata only" —
  applies identically to transfer/emission origin: by the time ops reach
  `simthing-sim`, they are flat `AccumulatorOp` registrations with no semantic
  tag. **`simthing-sim` will gain zero `match` arms over transfer / emission
  variants under this proposal.**
- **Not `simthing-feeder`.** The feeder owns intake (player/AI intents);
  it does not own production economic content. Transfer/emission/recipe are
  spec content, not runtime intent.
- **Not `simthing-gpu` or `simthing-core` alone.** `simthing-core` owns the
  flat-op shape and builder surface. `simthing-gpu` owns the GPU-side planner
  / encoder. Neither owns *authored content*; both are downstream consumers
  of the authored, compiled registration list.
- **Not "implicit from property possession".** E-10 already rejects implicit
  Resource Flow participation; the same draconian discipline applies here.
  A property declaring it can be transferred does not, by itself, register a
  transfer. Explicit authoring only — the same policy that protects arena
  expansion budgets.

### 2.3 What this preserves about v7.5

- Runtime substrate is `AccumulatorOp` (no new WGSL, no new primitive).
- Policy lives in spec/driver, never in kernel branches.
- `simthing-sim` is spec-free (no `simthing-spec` import; no semantic tagging
  on the ops it dispatches).
- `ExactDeterministic` admission matrix is the only path for production
  transfer and (today) emission consumers.
- Conservation invariants are GPU-enforced; the spec/driver layer cannot
  weaken them — it can only choose to register an op or not.
- Replay/reload is deterministic because compilation is a pure function of
  spec + session state.

---

## 3. Crate / module boundary diagram (prose)

The pipeline runs strictly downstream — each crate only knows the layer
below it as a destination, never the layer above as a caller.

### 3.1 Authoring (`simthing-spec`)

New module `spec/resource_economy.rs` (name negotiable; see §8) declares:

- `ResourceTransferSpec { source: PropertyKey, target: PropertyKey, amount: f32, … }`
  — discrete transfer authoring.
- `ResourceRecipeSpec { inputs: Vec<RecipeInputSpec>, target: PropertyKey, throttle_hint_max_per_tick: u32, … }`
  — conjunctive recipe authoring; the `throttle_hint_*` rename from E-3R
  must be preserved verbatim (no `max_per_tick` field; no implied GPU cap).
- `ResourceEmissionSpec { source: PropertyKey, formula: EmissionFormulaSpec, … }`
  with `EmissionFormulaSpec::{IdentityFloor, Constant(f32), EvalEml { tree_key }}`.
- `EmitOnThresholdSpec { source: PropertyKey, threshold: f32, direction, event_kind, buffer }`
  — explicit re-author of E-1 entry points so spec content owns the
  identity, not test code.
- EML formula references resolve via a spec-level formula table keyed by a
  designer-stable name; the compile pass translates names → `EmlTreeId`.

All four spec types live in or alongside `spec/resource_flow.rs` because they
share the same admission discipline (explicit participation, draconian caps,
expansion report). They are **not** subsumed by `ResourceFlowSpec` — discrete
transfer and emission are hard-currency / event semantics, not continuous
Resource Flow Balance semantics. (Conflating them is an explicit stop
condition; see §6.)

The compile pass in `simthing-spec/src/compile/` validates:

- Property references resolve, with the expected sub-field role.
- EML formula references resolve to a registered `ExactDeterministic`
  tree (admission via `EmlConsumerKind::{Transfer, Emission}`).
- `unit_cost > 0`, `amount` finite and `>= 0`, etc. — mirroring the
  validation already inside the `simthing-core` builders.
- No same-band consumed-input contention at the spec level (catches the
  error earlier than the C-8c planner, with a friendlier diagnostic).
- The expansion report is extended with transfer/emission/recipe counts so
  E-10 admission stays the single budget oracle. Per-arena budgets remain
  the existing caps; transfer/emission/recipe counts are reported on
  `ResourceFlowExpansionReport` (or a sibling) for visibility, not as a new
  hard cap.

### 3.2 Compilation (`simthing-driver`)

New module `resource_economy_compile.rs` (sibling of
`resource_flow_compile.rs`) exposes:

```text
pub fn compile_resource_economy(
    spec:     &CompiledResourceEconomy,         // produced by simthing-spec
    registry: &DimensionRegistry,
    arena:    &ArenaRegistry,                   // for cross-validation only
) -> Result<ResourceEconomyRegistrations, ResourceEconomyCompileError>;

pub struct ResourceEconomyRegistrations {
    pub discrete_transfers: Vec<simthing_core::DiscreteTransferRegistration>,
    pub recipes:            Vec<simthing_core::ConjunctiveRecipeRegistration>,
    pub emissions:          Vec<simthing_gpu::EmissionRegistration>,
    pub emit_on_threshold:  Vec<simthing_core::EmitOnThresholdRegistration>,
    pub generation:         u64,
}
```

The driver session state grows a `resource_economy: ResourceEconomyRegistrations`
field. At session open and at boundary refresh, the driver:

1. Calls `rebuild_discrete_transfer_ops` / `rebuild_conjunctive_recipe_ops` /
   `plan_emission_ops` / `rebuild_emit_on_threshold_ops` from existing
   bridges. **No new builder surface is required.**
2. Uploads via existing `WorldGpuState::upload_accumulator_transfer_ops` /
   emission analog / threshold op upload.
3. Bumps `resource_economy.generation` only when the compiled output
   changes (signature-keyed, mirroring `IntensityEmlOpPlanSignature`).

The driver remains the only crate that knows transfer/recipe/emission are
*authored content* rather than substrate primitives.

### 3.3 Execution (`simthing-sim` + `simthing-gpu`)

`simthing-sim` keeps the existing `PipelineFlags::use_accumulator_transfer` /
`use_accumulator_emission` gates. When enabled, boundary sync calls the
existing `ensure_transfer_accumulator` / `ensure_emission_accumulator` paths.
When disabled, sync clears sessions exactly as today.

**`simthing-sim` does not import `simthing-spec` and does not match on any
spec variant.** Verified by an explicit anti-import test, mirroring the
E-8 / E-11 invariants:

```rust
#[test]
fn simthing_sim_does_not_import_simthing_spec() { /* see §9 */ }
```

`simthing-gpu` is unchanged: the C-8c / C-8d planners already take typed
registrations and produce GPU ops. No new WGSL, no new combine functions,
no new buffer layouts.

---

## 4. Registration lifecycle

The lifecycle mirrors `ArenaRegistry` and `IntensityEmlOpPlanSignature`. It
has eight phases. Each phase has a unique owner.

| Phase | Owner | Trigger | Output | Determinism gate |
|-------|-------|---------|--------|------------------|
| **Authored** | `simthing-spec` (designer RON) | Edit-time | `ResourceTransferSpec` / `ResourceRecipeSpec` / `ResourceEmissionSpec` / `EmitOnThresholdSpec` | RON serde roundtrip stable. |
| **Compiled (spec)** | `simthing-spec::compile` | `compile_resource_flow_admission` companion call at session build | `CompiledResourceEconomy` (resolved property ids, resolved EML tree ids, validated unit costs) | Same `SpecError` taxonomy as E-10; bad spec rejected, not warned. |
| **Validated (driver)** | `simthing-driver::resource_economy_compile` | Session open / structural mutation | Cross-checks vs `ArenaRegistry`, dimension registry, allocator headroom | New `ResourceEconomyCompileError` variants for cross-crate mismatches (unknown slot, role mismatch). |
| **Materialized** | `simthing-driver` | Inside compile call | `ResourceEconomyRegistrations` (flat `*Registration` Vecs) | `generation: u64` bumped iff signature changed. |
| **Uploaded** | `simthing-gpu` via existing `world_state` entry points | First boundary sync after generation bump, or session open | `TransferPlan` / `EmissionPlan` encoded; ops uploaded; input-list table refreshed | Op-plan signature equality skips reupload. |
| **Refreshed (subtree)** | `simthing-driver` | Structural mutation (fission / fusion / install) | Re-compile **only the affected subtree's** transfer/emission/recipe set; merge into existing `ResourceEconomyRegistrations`; bump `generation` if shape changes | Naïve global rebuild **forbidden**, same prohibition as E-9 `ArenaRegistry::refresh_for_structural_mutation`. Boundary-time bloat is the exact failure mode this rule prevents. |
| **Removed** | `simthing-driver` | Subtree removal / unenrollment | Compiled registration set shrinks; `generation` bumped; GPU upload re-runs with smaller vectors | Stale ops cleared via existing `clear_transfer` / `clear_emission` paths. |
| **Replayed** | `simthing-driver::spec_replay` extension | Replay restore | Re-runs the spec→compile→materialize pipeline on the restored spec + session state | Replay is bit-exact because compilation is a pure function of inputs. Compact emission records (`EmissionRecordGpu { reg_idx, emit_count }`) restore the same `reg_idx` because authoring identity → stable `reg_idx` mapping is part of the compile pass and recorded in the snapshot. |

Two cross-cutting invariants:

1. **Stable `reg_idx`.** The driver assigns `reg_idx` deterministically from
   the authoring identity (a stable spec-level key), not from registration
   order. This is what makes emission compact logs replayable across
   structural mutations.
2. **Boundary refresh is subtree-scoped.** The same Approach B pattern E-9
   already enforces: walk only the mutated subtree's participants; never the
   global registry. The expansion report records what was re-evaluated so
   ballooning shows up as a visible regression, not a silent slowdown.

---

## 5. Conservation / replay invariants

These constraints define what the registration ownership model **must not
break**. Each maps to an existing landed invariant or an explicit stop
condition.

### 5.1 Discrete transfer

- **Exact source-debit.** `SubtractFromSource` / `SubtractFromAllInputs` are
  the only consume modes admitted for transfer. The spec-level
  authoring types must not expose a `ConsumeMode` field; the mapping is
  fixed by the builder.
- **Shared-input same-band contention remains rejected.** The driver
  compile pass should detect this *before* the GPU planner does, but the
  planner's rejection (`TransferPlanError::ContendedConsumedInput`)
  remains the authoritative gate. The diagnostic must point back at spec
  identity, not GPU op indices.
- **No probabilistic / continuous-flow substitute.** Transfer is exact and
  discrete. The spec authoring type must not accept a `rate` field, a
  probability, or a Resource Flow Balance role. (Stop condition; see §6.)
- **No CPU production fallback peer.** Sunset complete; do not re-introduce.
- **No Resource Flow Balance semantics for hard-currency transfer.**
  Hard-currency transfers go through `DiscreteTransferRegistration` →
  `SubtractFromSource`; they never flow through arena allocation. The spec
  authoring types must keep these separate.

### 5.2 Conjunctive recipe

- **Exact per-recipe conservation.** `MinAcrossInputs` + `SubtractFromAllInputs`
  with `ScaleSpec::Identity`. Lifted N>4 cap (E-3) is preserved.
- **`throttle_hint_max_per_tick` is metadata only.** E-3R is non-negotiable:
  the spec field is named exactly `throttle_hint_max_per_tick`, and it does
  not produce a GPU cap. Any future cap must be GPU-resident and affect
  *both* the target credit and the input debit so per-recipe conservation
  survives. The driver must not silently translate a spec-level hint into a
  CPU-side gate.

### 5.3 Emission

- **Emission is event-shaped, not balance-shaped.** It produces compact
  emission records (`EmissionRecordGpu { reg_idx, emit_count }`); it does
  not credit/debit source values *by itself*. Source decrement happens via
  a paired transfer if the spec authors it; emission alone is observation,
  not conservation work. This matches the C-8d landed shape.
- **Capping model (the open question this memo closes).** Emission is
  **unconstrained creation, capped by EML admission + threshold gates**.
  In v7.5 terms:
  - `IdentityFloor`: emits one record per slot where source ≥ 0 (the
    existing semantics).
  - `Constant`: emits the constant per tick per registration.
  - `EvalEML`: emits the value computed by the `ExactDeterministic`
    formula, with admission gated by the `EmlExpressionRegistry`.
  Recipe-output capping is **not** an emission concern — it is a recipe's
  job to debit inputs to enforce a per-tick yield. Scheduled-event output
  capping is **not** modeled in this registration layer; scheduled events
  are an authoring-time pattern that compiles to transfer + emission ops,
  not a new registration kind.
  `max_emit` remains rejected (`EmissionPlanError::MaxEmitUnsupported`)
  until a GPU clamp ships; the spec authoring type must not expose
  `max_emit` until then.
- **Exactness × EML admission.** The driver compile pass rejects any
  emission formula whose tree is not registered as
  `ExactDeterministic` for `EmlConsumerKind::Emission`. The spec layer is
  not allowed to claim a Soft/Fast class for emission until a separate ADR
  amendment opens the matrix; today this is documented as future-gated.

### 5.4 Replay

- **Bit-exact for `ExactDeterministic`.** Re-running the compile pipeline
  against the snapshot spec + restored allocator state must produce
  identical `*Registration` vectors. Stable authoring → stable `reg_idx` →
  stable emission records.
- **Compact emission records replay deterministically.** The C-8d
  `EmissionRecordGpu { reg_idx, emit_count }` shape is unchanged; the
  registration layer's contribution is making `reg_idx` semantically
  meaningful (tied to authoring identity).
- **Structural mutation replay.** Fission/fusion records on the replay log
  pair with subtree-refresh compile calls so the registration set at any
  replay tick equals the registration set the original run had at that tick.

---

## 6. Stop conditions (explicit halts)

Per the handoff brief, Opus stops and reports rather than recommending
implementation if the design requires any of the following. Each is checked
against the proposal in §2–§5.

| Stop condition | Triggered by this proposal? | Notes |
|----------------|-----------------------------|-------|
| New WGSL | **No.** | Substrate is unchanged; only spec/driver layers grow. |
| New `AccumulatorOp` primitive | **No.** | Existing combine/source/consume variants suffice. |
| `simthing-sim` semantic ownership of transfer/emission | **No.** | Anti-import test enforces; no spec types reach `simthing-sim`. |
| CPU production fallback peer | **No.** | All paths route through `AccumulatorOp`; flag-off rejects rather than falls back, mirroring the S-series sunset pattern. |
| Weakening exact transfer conservation | **No.** | `SubtractFromSource` / `SubtractFromAllInputs` are the only admitted consume modes; same-band consumed-input contention remains rejected. |
| Making Resource Flow default-on | **No.** | `use_accumulator_resource_flow` stays default false. This memo concerns transfer/emission/recipe, which are separate registration families; their flags can independently default-on once integration tests are green, but **this memo does not change the defaults** — that decision is the next gate after Cursor's implementation lands. |
| Folding hard-currency transfers into continuous Resource Flow | **No.** | Discrete transfer is authored and compiled separately from `ResourceFlowSpec`; the spec types are sibling, not subsumed. |

**No stop condition is triggered.** Cursor may proceed with the §7 handoff
once this memo is reviewed and merged.

---

## 7. Implementation handoff for Cursor

The narrowed Cursor scope, in dependency order. Each numbered item is a
single PR. **Do not bundle.**

### 7.1 PR T-1 — `simthing-spec` authoring types (Codex 5.5)

- Add `ResourceTransferSpec`, `ResourceRecipeSpec`, `ResourceEmissionSpec`,
  `EmitOnThresholdSpec` in `crates/simthing-spec/src/spec/` (file name TBD;
  recommend `resource_economy.rs` to keep `resource_flow.rs` arena-only).
- Add `RecipeInputSpec`, `EmissionFormulaSpec`, `EmitBufferSpec` enums
  mirroring the `simthing-core` shape (do not re-export `simthing-core`
  enums from `simthing-spec`; mirror to keep the dependency arrow correct).
- Serde + RON roundtrip tests for every variant.
- **No compile pass yet.** Authoring types only.

### 7.2 PR T-2 — `simthing-spec` compile pass (Composer 2.5)

- `compile/resource_economy.rs` resolves property keys → ids, EML formula
  names → `EmlTreeId`, validates unit costs / amounts / contention, and
  produces `CompiledResourceEconomy`.
- Extend `ResourceFlowExpansionReport` (or add a sibling) with
  transfer/recipe/emission/threshold counts.
- Rejection-fixture suite mirroring E-10: bad cell, unknown property,
  Soft/Fast EML for emission, etc.

### 7.3 PR T-3 — `simthing-driver` materialization (Composer 2.5)

- `crates/simthing-driver/src/resource_economy_compile.rs`:
  `compile_resource_economy` → `ResourceEconomyRegistrations`.
- `SpecSessionState` grows a `resource_economy` field; install /
  `react_to_fission_clones` invoke subtree refresh.
- Stable `reg_idx` assignment from authoring identity (documented; tests).

### 7.4 PR T-4 — Session integration + boundary refresh (Composer 2.5)

- `BoundaryProtocol::execute_with_boundary_hook` (already exists) becomes
  the integration point: the driver-level handler uploads the compiled
  registrations through existing `sync_accumulator_transfer_session` /
  `sync_accumulator_emission_session` paths.
- Generation-keyed skip on unchanged spec.
- Flag-off + populated spec rejection path (no silent fallback).

### 7.5 PR T-5 — Boundary refresh / replay tests (Composer 2.5)

- Multi-tick scenario fixture: discrete transfer + conjunctive recipe +
  emission registered from a RON spec; replay-bit-exact under fission and
  fusion.
- Subtree-refresh test: large multi-arena fixture, single mutated subtree,
  assert only that subtree's registrations are recompiled (use the
  generation counter / a structured trace, not wall time).
- Anti-import test: `simthing-sim` must not import `simthing-spec`.

### 7.6 PR T-6 — Docs sync (Codex 5.5)

- Update `accumulator_op_v2_production_plan.md` (E-4 expanded into the T-1
  through T-5 ladder; close the "Open after C-8" line).
- Update `design_v7.md` §5 / §6 with the registration ownership story.
- Update `workshop_current_state.md` (drop "production transfer/emission
  registration ownership" from open warnings).
- Update `todo.md` ledger.

**Out of scope for this Cursor handoff:**

- Flipping `use_accumulator_transfer` / `use_accumulator_emission` to
  default-on. That is a separate gate, after T-5 burn-in.
- E-11B nested hierarchy GPU.
- E-2B `resource_flow_participant` — still blocked on enrollment
  compilation.
- Any new GPU primitive, WGSL, or CPU production peer.
- `max_emit` enforcement (still rejected at the planner).

---

## 8. Docs update requirements

This memo lands alongside the following docs-only edits, all required for
the gate to close:

1. **`docs/accumulator_op_v2_production_plan.md`.**
   - Mark the "Open after C-8: production transfer/emission registration
     ownership" line as now resolved by this design memo, with a forward
     pointer to the T-1…T-6 ladder.
   - Add a §"Phase T — production transfer/emission registration ownership"
     subsection (or fold into Phase E after E-11) describing the ladder.
2. **`docs/todo.md`.**
   - Promote "Opus — production transfer/emission registration ownership"
     from "Next recommended gates" to a landed memo entry pointing at this
     file.
   - Add a "Next: Cursor T-1…T-6 implementation, gated on memo merge" line.
3. **`docs/worklog.md`.**
   - Add a 2026-05-27 entry recording the memo landing and the gate
     decision (E-11B deferred; transfer/emission registration ownership next).
4. **`docs/workshop/workshop_current_state.md`.**
   - Update §1 Executive summary's "Next gates" line to reflect that the
     Opus transfer/emission registration ownership memo has landed and the
     next gate is the Cursor T-ladder.
   - Update §2 "Open migration work" row to point at this memo.
   - Update §1's "Open design gates" line accordingly.

These four updates are landed in the same commit as this memo (this PR is
docs-only and contains no implementation).

---

## 9. Tests required for acceptance (future Cursor work)

These are the tests Cursor's T-1…T-5 implementation must add; this memo
specifies them so the Cursor handoff stays narrow.

### 9.1 Authoring & compile

- `crates/simthing-spec/tests/resource_economy_roundtrip.rs`: RON serde
  roundtrip for every authoring type.
- `crates/simthing-spec/tests/resource_economy_compile_rejections.rs`: bad
  property reference, unknown EML formula, Soft/Fast emission, negative
  amount, zero unit cost, same-band contention, `throttle_hint_max_per_tick`
  zero — each must reject with a specific `SpecError`.
- `crates/simthing-spec/tests/resource_economy_expansion_report.rs`:
  counts match expected for a well-formed multi-arena fixture.

### 9.2 Driver materialization

- `crates/simthing-driver/tests/resource_economy_compile.rs`: a multi-arena
  RON fixture compiles to expected `*Registration` vectors (golden compare).
- `crates/simthing-driver/tests/resource_economy_stable_reg_idx.rs`:
  identical authored spec re-compiled in different process invocations
  produces identical `reg_idx` assignments.
- `crates/simthing-driver/tests/resource_economy_subtree_refresh.rs`:
  structural mutation re-compiles only the affected subtree (asserted via
  generation counter / structured trace, not wall time).

### 9.3 Session integration & replay

- `crates/simthing-driver/tests/resource_economy_session_open.rs`: RON →
  session open → flag-on → registrations uploaded → 100-tick run with
  conservation assertion.
- `crates/simthing-driver/tests/resource_economy_replay.rs`: 100-tick
  record, replay from snapshot + spec → final values bit-exact for
  `ExactDeterministic` paths; emission record sequence identical.
- `crates/simthing-driver/tests/resource_economy_flag_off_rejects.rs`:
  populated spec with `use_accumulator_transfer = false` rejects on
  boundary sync (no silent fallback).

### 9.4 Boundary discipline

- `crates/simthing-sim/tests/sim_does_not_import_simthing_spec.rs`:
  source-grep assertion (or `cargo metadata`-based check) that
  `simthing-sim` has no `simthing-spec` dependency. This mirrors the E-11
  arena-ignorance test.

### 9.5 Existing regression coverage

- `cargo test -p simthing-spec transfer emission -- --nocapture`
- `cargo test -p simthing-driver transfer emission -- --nocapture`
- `cargo test -p simthing-gpu accumulator_op -- --nocapture`
- `cargo test -p simthing-driver e11_resource_flow_soak -- --nocapture`
- `cargo check --workspace`
- `cargo test --workspace`

All of the above must remain green; transfer/emission flags **stay default
false until T-5 burn-in is itself green**.

---

## 10. What remains blocked after this memo

| Item | Status after memo | Why |
|------|-------------------|-----|
| **E-2B `resource_flow_participant`** | **Still blocked.** | Enrollment compilation (E-9/E-11 compile-side enrollment) is not in scope here; this memo only addresses transfer/emission/recipe/threshold registration ownership. |
| **Default-on Resource Flow** | **Still default false.** | `use_accumulator_resource_flow` is unrelated to transfer/emission ownership. Its default-on gate remains "burn-in clean + E-2B unblocked." |
| **E-11B nested hierarchy GPU** | **Still deferred.** | Flat-star is the production execution; nested is a separate substrate question. |
| **D-1 discrete-transaction contention memo** | **Still pending.** | Untouched by this memo; D-1 evaluates whether discrete *boundary* transactions need a GPU allocator. C-8c continues to reject same-band consumed-input contention. |
| **Soft / Fast EML for transfer or emission** | **Still future-gated.** | Production admits `ExactDeterministic` only; opening the matrix needs a separate ADR. |
| **`max_emit` enforcement** | **Still rejected.** | Requires a GPU clamp that hasn't been designed; the spec authoring type omits the field. |

---

## 11. Bottom line

The substrate is complete. The remaining warning is purely an ownership /
authoring question, and the answer that satisfies v7.5 is unambiguous:

- **`simthing-spec` authors** transfer / recipe / emission / threshold
  registrations as first-class RON content, separately from
  `ResourceFlowSpec`.
- **`simthing-driver` compiles** authored content into the existing
  `simthing-core` builder shapes and the existing `simthing-gpu`
  registration shapes, with stable `reg_idx` and subtree-scoped refresh.
- **`simthing-sim` and `simthing-gpu` are unchanged in spirit:** they
  receive flat `AccumulatorOp` registrations and dispatch them; they grow
  no `match` arms over economic content.

No stop conditions are triggered. The Cursor T-1…T-6 ladder in §7 is the
narrow implementation handoff; the memo lands docs-only, with the four doc
updates listed in §8 in the same commit.
