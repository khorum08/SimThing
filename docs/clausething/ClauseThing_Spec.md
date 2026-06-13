# ClauseThing — Clausewitz Scripting as SimThing's Designer-Facing Language

> **Status: AUTHORIZED — the `CT-` production track is OPEN (2026-06-10).** This document is the
> strategy and correspondence reference for the track; the **live ladder, gates, and status ledger**
> are in [`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md).
> This repo copy is **canonical**. The original in `C:\Users\mvorm\Clauser\` is the lab copy, kept
> beside the materials that intentionally stay out of the repo: the `jomini` reference clone and
> the Paradox `script_documentation/` dumps. The ClauseScript textbook now lives beside this file
> at [`ClauseThing.md`](ClauseThing.md) (moved into the repo 2026-06-10 for implementation-agent
> access; subject to the track §6.1 secondary-source provenance caveat).
>
> *Provenance:* written 2026-05-29 against the v7.7 governance era. Governance citations in the body
> (`design_v7_7.md` §5 etc.) are **historical** — current governance is `design_0_0_8_1.md` §5 plus
> `simthing_core_design.md`; the legacy field-AI acronym has been normalized to
> **Movement-Front / FIELD_POLICY** per the repo guard test. The correspondence table (§4), the
> tiers (§5), the worked examples (§6), and the limits (§8) remain the live technical content.
>
> **Source ingested:** `Clauser/ClauseThing.md` (the ClauseScript architectural textbook),
> `Clauser/Paradox/02_Dynamic_Modding_Scripting_Format.md`, the generated
> `script_documentation/{effects,triggers,scopes,modifiers,localizations}.log`,
> and the `jomini` parser (`Clauser/jomini/`).
>
> **SimThing companions:** `design_0_0_8_1.md` (constitution), `simthing_core_design.md` (permanent
> paradigm), `invariants.md`, `adr/mapping_sparse_regioncell.md`, `adr/resource_flow_substrate.md`,
> `adr_accumulator_op_v2.md`, `capability_tree_v1.md`.

---

## 1. Thesis

**ClauseScript and SimThing are the same architecture approached from opposite ends,
and the gap between them is exactly the layer SimThing already has: `simthing-spec`.**

ClauseScript is, in its own textbook's words, *"a declarative definition language
with embedded imperative effects and a sophisticated scoping and parameterization
system. Its primary output is definitions (templates and types), not direct
executable commands. A separate ingestion and hydration layer is always required to
turn these definitions into runtime participants."*

SimThing is the inverse statement: a deterministic, GPU-resident runtime of flat
columns and opaque `AccumulatorOp` registrations, fronted by a spec/driver layer
(`simthing-spec` → `simthing-driver`) whose entire job is to **hydrate authored
definitions into runtime artifacts** while keeping `simthing-sim` semantic-free.

The proposal is constitutionally clean and aspirationally broad:

> **ClauseScript becomes an alternate front-end to `simthing-spec`. A new
> ingestion module (working name `clausething`) parses ClauseScript with `jomini`,
> resolves scopes and references, and emits the *same* `simthing-spec` authoring
> structs that RON authoring produces today. From that point the existing
> compile/admission/install path is unchanged. `simthing-sim` never learns that
> ClauseScript exists.**

This adds **no GPU primitive, no WGSL, no `AccumulatorRole`, no default change, and
no `simthing-sim` awareness**. It is a designer-layer authoring surface — the same
admission firewall the Resource Flow ADR (`resource_flow_substrate.md`) and the
Mapping ADR (`mapping_sparse_regioncell.md`) already establish: *semantics live at
the RON/Designer/spec layer; the runtime sees only flat columns and generic
registrations.* ClauseScript is simply a second dialect spoken at that layer.

### Why bother

- **Density and expressiveness.** ClauseScript is purpose-built for "extremely high
  authoring density" of structured game data + logic. SimThing's RON is correct but
  verbose; a grand-strategy designer authoring economies, capability trees, and AI
  weights wants ClauseScript's idioms (`modifier`, `triggered_modifier`, `resources`,
  `value:`, `ai_will_do`, `possible`).
- **A 20-year-proven design vocabulary.** Paradox's `economic_category` + modifier
  generation, capability trees, `ai_budget`, and on-action model are battle-tested
  solutions to exactly the problems SimThing's economic and mapping ADRs solve from
  first principles. We can adopt the *vocabulary* without adopting the *engine*.
- **A migration corpus.** The world's largest body of grand-strategy content is
  authored in this language. A faithful front-end makes that corpus a reference and
  a test oracle, not a competitor.
- **A named consumer.** The Stellaris/Clausewitz-engine grand-strategy audience
  (players and modders) — recorded in the horizon charter's `CLAUSETHING-IMPORTER-0`
  product note (2026-06-10). This is the consumer-pulled standing the track opens on.

### Aspiration

The goal is **full ClauseScript as a first-class, co-equal authoring surface for
SimThing** — not a subset, not a convenience shim, not a "second" option sitting
behind RON. The aspiration is that a designer who knows ClauseScript should be
able to express anything in SimThing that a designer who knows RON can express,
and that the world's existing body of Clausewitz-authored grand-strategy content
becomes a reference corpus and a test oracle rather than a distant competitor.

The tiered ladder (§5) is a **sequencing discipline**, not a scope ceiling. Each
tier is a shippable proof of the mapping; later tiers expand the vocabulary until
the language is fully covered. The runtime constraints (§7) are binding, but they
constrain *how* ClauseScript compiles, not *how much* of it we admit.

**What does not translate and never will:** Presentation-only constructs
(`custom_tooltip`, `defined_text`, portrait/name lists, localization keys) are
dropped before runtime — they have no SimThing equivalent by design. Binary save
format is out of scope — ClauseThing ingests authoring `.txt`, not save games.
These are the only permanent exclusions. Everything else is a sequencing question.

---

## 2. Where it lives (architecture)

```
                AUTHORING (designer layer)
   ┌─────────────────────┐        ┌─────────────────────┐
   │   RON authoring      │        │  ClauseScript (.txt) │
   │  (existing)          │        │  authoring (new)     │
   └──────────┬───────────┘        └──────────┬───────────┘
              │                                │
              │                     ┌──────────▼───────────┐
              │                     │  clausething          │  NEW, isolates jomini
              │                     │  parse → raw model →  │  + scope/ref resolution
              │                     │  simthing-spec structs│
              │                     └──────────┬───────────┘
              │                                │
              ▼                                ▼
   ┌──────────────────────────────────────────────────────┐
   │  simthing-spec  (UNCHANGED contract)                   │
   │  authoring structs → admission/firewall → compile →    │
   │  flat AccumulatorOp regs, overlays, registry, arenas,  │
   │  RegionFieldSpec, thresholds, ScenarioSpec             │
   └──────────────────────────┬─────────────────────────────┘
                              ▼
   ┌──────────────────────────────────────────────────────┐
   │  simthing-driver → simthing-sim → simthing-gpu         │
   │  deterministic, replayable, CPU-oracle-parity runtime  │
   │  (NEVER sees RON, ClauseScript, scopes, modifiers,     │
   │   categories, on_actions, or "tradition")              │
   └────────────────────────────────────────────────────────┘
```

Two placement options for the front-end; the track adopts **(A)**, named
**`simthing-clausething`**, for dependency hygiene:

- **(A) Separate crate `simthing-clausething` (adopted).** Depends on the vendored
  `jomini` text path and on `simthing-spec`'s public authoring structs only. Keeps
  the parser machinery, the Windows-1252/UTF-8 decoding, and the tape/AST machinery
  out of `simthing-spec`. Emits authoring structs; `simthing-spec` stays the single
  admission/firewall owner.
- **(B) A `clausewitz` module inside `simthing-spec`.** Simpler wiring, but pulls
  the parser into the admission crate. Rejected.

Either way the **firewall is the existing `simthing-spec` admission layer**, not the
parser. ClauseScript is rejected for unsafe *authoring* at import (bad scope,
unbounded fanout, non-deterministic constructs, out-of-class EML); the runtime still
enforces hard safety unconditionally (horizon caps, source-cap clamp, finite/column
validation, bounded fields) exactly as the Mapping ADR requires.

---

## 3. The ingestion pipeline (jomini-grounded)

ClauseThing follows the five-stage pipeline the textbook §12 (and jomini) prescribe.
The first four stages are generic and game-agnostic; **Stage 5 is the only
SimThing-specific stage** and is where all the tiering lives.

| Stage | What | Tooling |
|---|---|---|
| 1. Lexing | bytes (Win-1252/UTF-8) → tokens, exact scalar preservation | vendored `jomini` `TextTape` |
| 2. Structural parse | tokens → tape/tree: blocks, mixed containers, headers (`rgb {}`), operators (`= < > <= >= ==`) | vendored `jomini` tape + reader |
| 3. Scope & symbol resolution | assign scope to each statement; resolve `this/root/from/prev` + domain scopes against `scopes.log` Supported/Output rules; resolve dot-paths; expand `[[param]]`, `$PARAM$`, `@[ ]`, `inline_script`, `value:` | ClauseThing |
| 4. Raw definition extraction | typed, language-neutral raw structs (`RawEntity`, `RawResources`, `RawModifier`, `RawCapability`, `RawField`), **duplication preserved as ordered sequences** | ClauseThing |
| 5. Hydration → SimThing artifacts | raw → `simthing-spec` authoring structs (the tiered mapping, §5–6) | ClauseThing → `simthing-spec` |

Stage-3/4 invariants we inherit verbatim from the textbook §13 and must honor:

- **Definition vs. instance separation is explicit** — ClauseScript describes
  templates; instances are the hydrated SimThing tree. (SimThing already enforces
  this: spec → `install_atomic` → tree.)
- **Ordered duplication is preserved until a deliberate policy is applied** — last-
  wins vs collection vs override is a *compile-time policy choice per key*, never a
  parser default. `jomini`'s `duplicated`/`take_last`/`alias` attributes model this
  directly. **Adopted default policy** (a documented decision, not a per-project
  TODO): scalar keys are **last-wins**; keys declared in the spec's *list registry*
  (e.g. `produces`, `modifier`, `prerequisites`) **collect** in source order; and an
  explicit `@override`/`@append` marker (sugar, §6) lets an author opt a single key
  into override-or-merge against an earlier definition. Mod load-order merging
  beyond a single compilation unit is **out of scope for v1** (one authoring tree,
  one policy table) and is itself a later Tier-2 question.
- **Scope is part of semantics, not syntax** — a representation that discards scope
  is lossy. Scope compiles to tree-position + slot/column resolution.
- **Triggers and effects stay distinct categories** — triggers → threshold/EML
  gates; effects → overlays / `BoundaryRequest` / `EmitEvent`.
- **Modifiers are first-class** — they compile to overlays, independent of the
  object that hosts them.

`jomini` also gives us, for free: a JSON projection of any parsed document (debug /
golden-file testing), a write API (round-trip / canonicalization), and a fuzzed,
>1 GB/s parser. Binary save format support exists in `jomini` but is permanently
out of scope for ClauseThing — we ingest authoring `.txt`, not save games (§10).

### 3.1 Scenario containers (0.0.8.2 PR2/PR3)

The closeout ladder adds a narrow scenario-container importer as a composing front end, not as a
new runtime concept. The admitted shape uses the existing jomini header-block idiom:

```clause
scenario = sample_scenario {
    metadata = {
        display_name = "Sample Scenario"
        description = "Import container only."
    }
    location = alpha {
        name = "Alpha"
        properties = {
            property = {
                id = "alpha_pressure"
                namespace = "simthing"
                name = "alpha_pressure"
            }
        }
        overlays = {
            modifier = {
                id = "alpha_pressure_bonus"
                targets_property = "simthing::alpha_pressure"
                amount_add = 1
            }
        }
        children = {
            child = alpha_cohort {
                kind = Cohort
            }
        }
    }
    location = beta {
        name = "Beta"
    }
    link = {
        from = alpha
        to = beta
    }
}
```

`hydrate_scenario` lowers this into existing generic surfaces:

- `GameModeSpec` carries flattened `PropertySpec` and `OverlaySpec` declarations.
- A real root `SimThingKind::World` contains authored `Location` children.
- ClauseThing retains a `HydratedScenarioNode` tree so authored ids, properties, overlays, and
  children survive before driver admission/registry compilation.
- Standalone overlays install through existing `InstallTargetSpec::ScenarioListed` ids.
- PR3 top-level `link` declarations lower to `HydratedScenarioGridMetadata`: bounded square-grid
  cell placements keyed by scenario install-target id plus canonical N4-adjacent links. This is
  admission/import metadata shaped for RegionField pressure placement, not a runtime topology
  object.

PR3 admits links only between top-level scenario locations; endpoints must exist, self-links are
rejected, duplicate/reversed links canonicalize deterministically, fanout is capped at the N4
degree, and row-major cell placement must make every explicit link an N4 neighbor. There is still
no `route`, `path`, `edge`, `predecessor`, `waypoint`, `movement_order`, `destination`, `border`,
`frontline`, pathfinding, arbitrary graph, PALMA, FIELD_POLICY-unification, GPU, Bevy, or
editor grammar in the scenario container beyond PR4's one accepted `field_operator` block. Non-grid
topology is deferred to a future topology-spec rung. `simthing-sim` remains unaware of
ClauseThing.

### 3.2 Scenario-contained field operators (0.0.8.2 PR4)

PR4 admits one top-level scenario `field_operator = id { ... }` block using the existing BH-3
authoring shape:

```clause
field_operator = alpha_choke_flux {
    display_name = "Alpha Choke Flux"
    grid_size = 10
    source_col = 0
    target_col = 0
    n_dims = 6
    saturating_flux = {
        u_sat = 1.0
        chi = 0.25
        choke_output_col = 2
    }
}
```

`hydrate_scenario` composes this through `hydrate_field_operator_property` into generic
`RegionFieldSpec` on the scenario `GameModeSpec`, optional W/stress compose slots on
`HydratedScenarioPack`, and `MappingExecutionProfile::Disabled`. SaturatingFlux remains generic
field math — not a border/frontline/movement/pathfinding service. PALMA W/D feedstock,
FIELD_POLICY threshold unification, driver install closure, GPU kernels, and runtime semantics are
still out of scope for PR4.

### 3.3 Scenario-contained PALMA W/D feedstock (0.0.8.2 PR5)

PR5 admits one top-level scenario `palma_feedstock = id { ... }` block bound to an existing
scenario `field_operator` id:

```clause
palma_feedstock = alpha_wd {
    w_source = alpha_choke_flux
    w_output_col = 3
    d_output_col = 4
    enabled = false
}
```

`hydrate_scenario` lowers this into `HydratedScenarioPalmaFeedstock` on `HydratedScenarioPack`:
generic W/D column bindings plus grid/`n_dims` context copied from the referenced field operator.
This is inert authoring feedstock for later driver/admission consumption via existing
`MinPlusStencilConfig` / `GpuInterleavedW` bridges — not a pathfinding service, route planner,
movement engine, or runtime PALMA service. `enabled = true` is rejected; scenario execution remains
default-off. FIELD_POLICY threshold unification, canonical sample closure, GPU kernels, and
`simthing-sim` awareness remain out of scope for PR5.

### 3.4 Scenario-contained FIELD_POLICY threshold / commitment (0.0.8.2 PR6)

PR6 admits one top-level scenario `commitment = id { ... }` block bound to an existing scenario
`field_operator` id:

```clause
commitment = stabilize_alpha {
    threshold = 0.75
    event_kind = 7
    field_urgency = {
        source = alpha_choke_flux
        column = 2
        weight = 1.0
    }
    effect = {
        attach_overlay = alpha_pressure_bonus
        target = alpha
    }
}
```

`hydrate_scenario` lowers this through the existing CT-3b+4a commitment path into generic
`FirstSliceCommitmentSpec`, optional `CommitmentEffectSpec`, `RegionFieldFormulaBindingSpec`
(`field_urgency`), and reduction feedstock on the referenced scenario `RegionFieldSpec`. Effect
blocks may also use the CT-3b+4a `target` / `targets_property` / `amount_add|amount_mult` shape.
This is threshold feedstock only — GPU-resident threshold crossings, not CPU planner logic,
movement, pathfinding, routes, predecessor tables, borders, frontlines, driver install closure,
canonical sample closure, GPU exercise, or `simthing-sim` awareness. `enabled = true` is rejected;
scenario execution remains default-off.

### 3.5 Canonical closeout sample (0.0.8.2 PR7)

PR7 adds one canonical sample at `docs/clausething/examples/ct_bh3_closeout_sample.clause`
(mirrored in `crates/simthing-clausething/tests/fixtures/ct_bh3_closeout_sample.clause`). The sample
exercises PR2–PR6 together: scenario metadata, three locations with properties/overlays/children,
two bounded N4 grid links, one SaturatingFlux `field_operator`, one PALMA `palma_feedstock`, and
one FIELD_POLICY `commitment`. Import tests prove parse → lower coherence into a single
`HydratedScenarioPack` with default-off posture preserved. This is authoring/lowering proof only —
no driver admit/install, GPU exercise, movement, pathfinding, routes, predecessors, borders,
frontlines, or `simthing-sim` awareness.

**PR8 driver closure (0.0.8.2 — DA APPROVED):** focused driver tests in
`crates/simthing-driver/tests/ct_bh3_closeout_sample_driver.rs` prove the same canonical sample
admits/installs through existing generic driver surfaces and exercises GPU-resident SaturatingFlux +
commitment + PALMA paths with compact evidence only. Default-off posture is preserved; no new GPU
kernels or `simthing-sim` semantics. Windows-safe command:
`cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver`.

**PR9 closeout battery (0.0.8.2):** PR9 consolidated fast guardrails without new runtime semantics.
Primary commands:

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
```

Per-PR2–PR6 result reports are **ARCHIVE** under `docs/archive/superseded_tests/`. PR7/PR8/PR9
evidence is **CURRENT_EVIDENCE**, folded into
[`docs/tests/clausething_closeout_results.md`](../tests/clausething_closeout_results.md).

**PR10 closeout (0.0.8.2 — DA SIGN-OFF PENDING):** the ClauseThing/BH/PALMA closeout track is
**CLOSED / PARKED** for authoring/import/runtime-feedstock closure — not playable-game, editor, or
corpus/export closure. Final report:
[`docs/tests/clausething_closeout_results.md`](../tests/clausething_closeout_results.md). Primary
guardrail commands remain:

```text
cargo test -p simthing-clausething --test ct_scenario_container
cargo test -p simthing-driver --test ct_bh3_closeout_sample_driver
```

**Awaiting Design Authority sign-off before track close is final.**

---

## 4. The deep correspondence (why the mapping is natural)

Every load-bearing ClauseScript construct has a direct SimThing counterpart that
**already exists**. This table is the spine of the whole proposal.

| ClauseScript construct | SimThing artifact it compiles to | Existing SimThing reference |
|---|---|---|
| Definition file / block | spec authoring struct → SimThing template | `install_clone_then_commit.md` |
| Block-as-object / array / mixed | keyed property block / positional list | jomini reader |
| `this/root/from/prev`, domain scopes, dot-paths | tree navigation + event context (`root`/`from` = `EmitEvent` payload) | `state-authority.md` |
| Trigger (boolean) | `Threshold` gate over a property column | Pass 7 / `ThresholdRegistration` |
| `AND/OR/NOT/NAND/NOR/calc_true_if` | boolean algebra in `EvalEML` (`SELECT`/`CMP`) + threshold combination | EvalEML |
| Effect (mutation) | overlay `PropertyTransformDelta` / `BoundaryRequest` / `EmitEvent` | `overlay.rs`, `work.rs` |
| `hidden_effect` / `custom_tooltip` | presentation-only metadata, dropped before runtime | — |
| Static `modifier` (`_add`/`_mult` + category) | overlay `TransformOp::{Add,Multiply}` on a sub-field; **category → tree level / `SimThingKind`** | `overlay_prep.rs`, ancestor stack |
| `triggered_modifier { potential modifier }` | `Suspended`/`Transient` overlay, `ActivateOverlay` on threshold, `DissolveCondition` on exit | `overlay_lifecycle.rs` |
| `economic_category` hierarchy + generated keys | `DimensionRegistry` columns + reduction `OrderBand`s; `_mult` propagates to children = reduction sweep | `reduction.rs`, C-5/C-6 |
| `resources { produces upkeep cost category }` | Resource Flow arena: `IntrinsicFlow` / `AllocatedFlow` / `Balance` | `resource_flow_substrate.md` |
| `overlord_resources` / trade / subject diversion | cross-arena **coupling edge** with declared `CouplingDelay` | Resource Flow ADR §commit 3 |
| `value:` script value (base + math + modifier) | `EvalEML` formula tree (`ExactDeterministic`, ≤32 nodes) | `accumulator_op.wgsl`, C-8 |
| `complex_trigger_modifier` (bool→number) | `EvalEML` `SELECT`/threshold-count | C-8 |
| `@scripted_variable`, inline `@[ ]` | compile-time constant folding (spec layer) | — |
| Runtime variables (`set_variable`) | `Named` sub-field columns + overlays | `property.rs` |
| Capability tree (tradition/edict/tech) | one `Custom(...)` SimThing; progress = sub-fields; unlock = `Suspended` overlay; `possible` = threshold gate; `cost`/`upkeep` = flow drain | **`capability_tree_v1.md` (1:1)** |
| `on_action` periodic pulse (`on_monthly_pulse`) | **cadence tier** (EveryTick/4/10/60) + boundary handler | Mapping ADR §opt doctrine; boundary doctrine |
| `on_action` event hook | `Threshold` + `EmitEvent` | Pass 7 / delta log |
| Stars + hyperlanes | physical **spatial tree** + adjacency/coupling topology | `simthing_core_design.md` §2 |
| Dense province / local field | **RegionCell** grid + `StructuredFieldStencilOp` (L1) | Mapping ADR L1 |
| `setup_scenario`, `solar_system_initializers` | `ScenarioSpec` + `AddChild` boundary requests / projection | `FirstSliceScenarioSpec` |
| Fog / stale intel / espionage / deception | **perception filter fields** (true→perceived write-boundary) | Mapping ADR §perception |
| `ai_will_do` / `ai_weight` | `EvalEML` pressure/urgency → `Threshold` crossing = commitment | **FIELD_POLICY / AI-as-SimThing** (Mapping ADR; `simthing_core_design.md` §8) |
| `ai_budget` (economic personality) | overlay-modifiable **allocator weight** columns | Resource Flow ADR §commit 4 |
| `diplomatic_stances` / personalities | authored `EvalEML` **weight profiles** (CPU selects, GPU computes) | economy→FIELD_POLICY fixture |

The textbook's own conclusion — that ClauseScript's economy is *"an emergent
directed graph of category + modifier + scope + budget"* (§10.9) — describes, almost
exactly, SimThing's *overlay + reduction + arena + allocator* graph. The two models
are isomorphic at the level that matters; ClauseThing is the isomorphism made
explicit.

---

## 5. The tiered strategy

Tiering runs along **two axes**. We commit to both and reconcile them: a **domain
ladder** (entities → flows → mapping → AI) where each rung is a shippable vertical
slice, and within each rung a **fidelity ladder** (literal → conditional →
expression) controlling how much of the language's metaprogramming we admit.

Each tier is classified against the gating policy (`design_0_0_8_1.md` §5). The
pattern mirrors Phase M exactly: the *first* slice of a tier is Tier-2 (new
architecture / open question); incremental fidelity within an accepted tier is
Tier-1 fast-lane (one PR + one test report + one status row), provided it stays
generic, opt-in, CPU-oracle-parity-backed, and reversible.

### Tier 0 — Parser + raw model (foundation)

- **Scope:** `jomini`-based lex/parse → tape → raw, typed, scope-resolved definition
  model. Mixed containers, headers, operators, comments, duplication-as-ordered-
  sequence. `[[param]]` / `$PARAM$` / `@[ ]` / `inline_script` expansion. Scope
  validation against `scopes.log`. JSON golden-file projection for tests.
- **Produces:** raw structs only. **No SimThing artifact yet.**
- **Gate:** **Tier-2** (new front-end architecture). After acceptance, parser
  feature additions are Tier-1.
- **Exit proof:** round-trip a corpus of authored `.txt` to JSON and back; reject
  malformed scope chains with good diagnostics.

### Tier 1 — Entities & overlays (the "definitions" core)

- **Scope:** blocks → SimThing templates; keyed scalars → properties / sub-fields;
  static `modifier` blocks → overlays (`Add`/`Multiply`/`Set`); `triggered_modifier`
  → `Suspended`/`Transient` overlay + threshold/dissolve; capability trees
  (traditions/edicts/tech) → the existing `capability_tree_v1` pattern verbatim;
  `possible`/`potential`/`allow` triggers → threshold gates.
- **Fidelity sub-ladder:**
  - **1a** literal modifiers + flat properties → overlays (`T1`).
  - **1b** `triggered_modifier` + `possible` → suspended overlays + threshold gates (`T1`).
  - **1c** capability-tree wiring (prereq DAG → threshold ordering; payload → activation) (`T1`).
- **Maps to:** `overlay.rs`, `overlay_lifecycle.rs`, `capability_tree_v1.md`,
  `ThresholdRegistration`.
- **Gate:** first slice **Tier-2** (establishes the entity-hydration contract); 1a–1c
  thereafter **Tier-1**.
- **Honest constraint (the biggest practical gap):** the **modifier `category` →
  tree-level mapping** has no automatic meaning in SimThing and must be resolved
  explicitly (a `category → (SimThingKind, tree depth, recalc cadence)` table). To
  keep this from being an ergonomic regression versus Paradox's implicit
  planet/country/pop granularity, the front-end ships **conventional defaults + sugar**,
  not a bare requirement: a small built-in default table maps the common
  `country`/`planet`/`pop`-style category names to sensible kinds/depths, a single
  `category_map { ... }` authoring block overrides per project, and an unmapped
  category is a hard admission error with a suggested mapping — never a silent
  guess. We do **not** reverse-engineer Stellaris category semantics; the designer
  confirms or overrides the defaults.

### Tier 2 — Flows & economy

- **Scope:** `resources { produces upkeep cost }` → Resource Flow arenas
  (`IntrinsicFlow`/`AllocatedFlow`/`Balance`); `economic_category` parent hierarchy →
  registry columns + reduction `OrderBand`s + arena descriptors with declared caps;
  `value:` script values → `EvalEML` (`ExactDeterministic`, ≤32 nodes); `@vars` +
  `@[ ]` → constant folding; **discrete** adoption costs/upkeep → `ResourceEconomySpec`
  discrete banking; **continuous** flow → E-11 hierarchical allocation (opt-in,
  default-off); `overlord_resources`/trade → coupling edges with `CouplingDelay`.
- **Fidelity sub-ladder:**
  - **2a** literal `produces`/`upkeep` → `IntrinsicFlow` registrations (`T1`).
  - **2b** `trigger`-gated sub-blocks → `EvalEML` `SELECT` / threshold gate (`T1`).
  - **2c** `value:` amounts + economic-category inheritance → reduction OrderBands (`T1`).
  - **2d** coupling (`overlord_resources`/trade) → cross-arena edges (`T2` — touches cycle/delay admission).
- **Maps to:** `resource_flow_substrate.md` (all four commitments), `accumulator_op_v2_production_plan.md` (E-7…E-11), `adr_accumulator_op_v2.md`.
- **Gate:** first slice + 2d **Tier-2**; 2a–2c **Tier-1**.
- **Honest constraints:** conservation is **approximate-deterministic** for
  continuous flow (O(ε·n), residual into `Balance`) — exactly the ADR's contract;
  hard-currency exactness uses the discrete path. EML must stay in
  `ExactDeterministic`; richer classes are per-PR opt-in only.

### Tier 3 — Mapping & space

- **Scope:** stars/systems → spatial tree nodes; hyperlanes → adjacency / coupling
  topology; `setup_scenario` → `ScenarioSpec`; `solar_system_initializers` →
  `AddChild`/projection; **dense local fields** (threat/suppression/supply/
  contamination) → RegionCell + `StructuredFieldStencilOp` (L1) → `SlotRange` Sum
  reduction (L2) → parent `field_urgency` `EvalEML` (L3); perception/fog → perception
  filter fields with the true→perceived write-boundary; periodic `on_action` →
  cadence tiers.
- **Fidelity sub-ladder:**
  - **3a** static topology (systems + hyperlanes) → spatial tree + adjacency (`T1`).
  - **3b** single-grid dense field → first-slice mapping runtime (the **landed**
    `SparseRegionFieldV1` path; ClauseScript just authors the `RegionFieldSpec`) (`T1`).
  - **3c** perception filter fields (`T2` — admission contract).
  - **3d** atlas / multi-theater (`T2` — gated per the standing prohibition list).
- **Maps to:** `adr/mapping_sparse_regioncell.md`, the landed
  `FirstSliceScenarioSpec` / `RegionFieldSpec` / `MappingExecutionProfile`.
- **Gate:** 3a/3b **Tier-1** (authoring over accepted runtime, default-off); 3c/3d **Tier-2/deferred** per the standing prohibition list.
- **Honest constraint:** no dense lateral long-horizon diffusion as strategic
  awareness — the load-bearing negative result holds (`simthing_core_design.md`
  §7.1 P1); ClauseScript authoring cannot override the three-layer model.
  `MappingExecutionProfile` default stays `Disabled`.

### Tier 4 — AI commitments & governance

- **Scope:** `ai_will_do`/`ai_weight` → `EvalEML` pressure/urgency formula → `Threshold`
  + `EmitEvent` = commitment (**AI-as-SimThing, no CPU planner**); `ai_budget` →
  overlay-modifiable allocator weight columns; `diplomatic_stances`/personalities →
  authored EML weight profiles (the CPU may *select* a profile, as in the accepted
  economy→FIELD_POLICY fixture, but never computes urgency or emits the commitment).
- **Fidelity sub-ladder:**
  - **4a** `ai_weight` → EML urgency → threshold commitment (`T1`, over the accepted FIELD_POLICY path).
  - **4b** `ai_budget` → allocator weight overlays (`T1`).
  - **4c** velocity/trajectory pressures → explicit previous-value column pattern (`T2`).
- **Maps to:** Mapping ADR §AI-as-SimThing, FIELD_POLICY probes, Resource Flow §commit 4,
  `simthing_core_design.md` §8.
- **Gate:** 4a/4b **Tier-1** over the accepted substrate; 4c **Tier-2** (explicit-column contract).
- **Honest constraint:** EML has **no previous-buffer read** — trajectory needs an
  explicit previous-value column + copy band (~14.3% overhead). No CPU map planner,
  ever; `ai_will_do` is data the GPU reads, not a CPU decision loop.

### Tier reconciliation

The domain ladder is a **dependency order**: T1 entities are the substrate T2 flows
attach to; T2 economic categories are the columns T3 mapping fields and T4 AI weights
read; T4 commitments fire over T2/T3 outputs. A vertical product slice picks one rung
of each as needed — exactly how the landed Phase M economy→FIELD_POLICY fixture
already spans T2 (discrete economy) → T4 (commitment) without a production bridge.

---

## 6. Worked micro-examples (the mapping made concrete)

**A tradition (T1).** ClauseScript:

```script
tr_adaptability_recycling = {
    possible = { has_tradition = tr_adaptability_adopt }
    modifier = { planet_structures_volatile_motes_upkeep_mult = -0.15 }
    triggered_modifier {
        potential = { is_wilderness_empire = no }
        pop_housing_usage_mult = -0.10
    }
    ai_weight = { factor = 10000 }
}
```

→ on the owner's `Custom("tradition_tree")` SimThing: a progress sub-field gated by a
threshold compiled from `has_tradition = tr_adaptability_adopt`; a `Suspended` overlay
carrying `Multiply(-0.15)` on the `volatile_motes::upkeep` column, flipped to
`Permanent` by `ActivateOverlay` when the prereq threshold fires; a second overlay
that is `Suspended` until the `is_wilderness_empire == no` threshold holds and
`Transient`/dissolved when it fails; `ai_weight` → an `EvalEML` term feeding the
adoption-pressure column. (`capability_tree_v1.md` is the exact existing pattern.)

**A producer job (T2).** ClauseScript:

```script
resources = {
    category = planet_technician
    produces = { energy = 6 }
    produces { trigger = { owner = { is_robot_empire = yes } } energy = 2 }
}
```

→ an `IntrinsicFlow` registration into the `planet_technician → energy` arena column
(base 6), plus a second contribution gated by an `EvalEML SELECT` compiled from the
`is_robot_empire` trigger (read as a `Named` flag column). The `category` resolves to
the arena/column range; `_mult` modifiers authored elsewhere (T1 overlays) propagate
to it through the reduction OrderBand — no enumeration of jobs required, exactly as
the economic-category engine intends.

**An AI commitment (T4).** `ai_will_do { weight = { base 1; modifier { factor 5
is_at_war = yes } } }` → an `EvalEML` urgency formula over a pressure column; when it
crosses an authored `Threshold`, `EmitEvent` fires the commitment. The AI is a
SimThing: float columns in, threshold crossing out, designer EML in the middle.

---

## 7. Constitutional guardrails (binding on any implementation)

These are restatements of existing invariants applied to the front-end; none is new
enforcement, and none is relaxed.

1. **`simthing-sim` never sees ClauseScript.** No scope, modifier, category,
   `on_action`, `resources`, `value:`, or "tradition" concept crosses into the
   simulation crate. All compilation is in `clausething` / `simthing-spec` /
   `simthing-driver`. (Mirrors the Resource Flow `ArenaRegistry` firewall and the
   Mapping `simthing-sim` map-free rule.)
2. **No semantic WGSL, no new primitive, no new `AccumulatorRole`.** ClauseScript
   compiles onto the *existing* overlay / `AccumulatorOp` / `EvalEML` /
   `StructuredFieldStencilOp` / `Threshold` substrate. If a construct cannot be
   expressed on existing primitives, it is rejected at admission, not given a kernel.
3. **Opt-in, default-off.** Authoring in ClauseScript changes no defaults;
   `MappingExecutionProfile` stays `Disabled`, Resource Flow stays opt-in. Presence of
   a script is structure; execution requires explicit profile/scenario opt-in.
4. **CPU-oracle bit-exact parity (I8).** Every compiled `EvalEML` formula and every
   overlay op must match the CPU oracle to the float bit. Clausewitz math semantics
   we *do* model (mult-is-additive-in-effect stacking, clamps, `percentage = yes` as
   tooltip-only) must be modeled exactly or rejected.
5. **Determinism / replay-safety.** `random_list`, `random`, and any RNG construct
   compile to a **seeded deterministic stream** (replay bit-exact) or are **rejected
   at admission**. Non-deterministic authoring never reaches the runtime.
6. **Admission is the firewall, runtime is the last line.** `simthing-spec` rejects
   unsafe authoring (unbounded fanout, all-`Algebraic` coupling cycles, out-of-class
   EML, bad scope transitions, perceived→true writes) at import; the runtime still
   clamps/validates unconditionally.
7. **Gating discipline.** Tier-1 fast-lane only for within-accepted-design, generic,
   opt-in, parity-backed, reversible feature additions; everything touching a binding
   invariant, a default, new architecture, or an open question is Tier-2. **The
   front-end's introduction is Tier-2.**

---

## 8. Current limits & sequencing hard problems

The textbook is candid that ClauseScript is *"recklessly flexible."* These are the
real sequencing challenges — things that do not translate cleanly *yet*, with notes
on how they eventually close:

- **The Stellaris standard library is built incrementally, not declared out of scope.**
  The grammar is the foundation; the standard vocabulary of named effects (`set_owner`,
  `create_fleet`, …), triggers, and the 3.8 MB of generated modifier keys are real
  work to map, but the aspiration is that they get mapped. Each effect/trigger that
  matters gets a SimThing compilation target (column op, overlay, BoundaryRequest,
  EmitEvent) as the tiers advance. An unknown effect/trigger is a hard admission error
  *with a diagnostic and a suggested path* — not a permanent "unsupported," but a
  signal of what to build next. The gap between "grammar accepted" and "full standard
  library covered" is the long tail of the project.
- **Modifier category → simulation level requires an explicit authored table.**
  Mitigated by built-in defaults + a `category_map` override + hard errors on unmapped
  categories (§5 Tier 1), but the table is still real authoring work and the most
  likely source of early friction. Treat the default table's quality as a first-class
  deliverable, not an afterthought.
- **Recalculation model differs — *probably* a simplification, but unmeasured.**
  ClauseScript caches and invalidates modifiers on events; SimThing re-evaluates every
  column every tick on GPU. This removes ClauseScript's invalidation complexity, but
  "cheap country-level modifier" cost intuitions do not carry over — everything is a
  column, and frequency mismatches are handled by cadence tiers, not lazy evaluation.
  The claim that this is a net simplification is an **assumption that must be
  stress-tested early** (CT-1b): a corpus with many `triggered_modifier` blocks →
  many `Suspended`/`Threshold` registrations could move column/threshold counts in
  ways that matter. Measure before relying on it.
- **Dynamic scope chains (`from`, event context) are an open design question, not a
  free mapping.** `root`/`from` context passing compiles to `EmitEvent` payloads +
  boundary handlers; chains that reference state not held in a column cannot be
  GPU-resident and are rejected or bounded to the CPU boundary (never a per-tick CPU
  planner). **But the current event substrate may not carry the rich context Paradox
  designers expect**, so extending the `EmitEvent` payload is a likely **Tier-2
  substrate change that must be scoped and accepted *before* any tier leans on
  `from`/`root` chaining.** T1a–T1c are deliberately chosen to need only same-scope
  triggers so the front-end can land without blocking on this. The track formalizes
  this as the **SCOPE-MEMO gate** (see the track doc §5).
- **`complex_trigger_modifier` and arbitrary bool→number extraction** only compile
  when the underlying trigger reads a column; otherwise rejected.
- **Duplication / mod load-order semantics** are load-bearing and must be pinned by
  explicit policy at compile time. v1 ships the documented default policy in §3
  (last-wins scalars / collection for list-registry keys / explicit `@override`
  marker); cross-unit mod merging is deferred (jomini gives the mechanism, the spec
  owns the policy). Surprising-merge behavior is a real designer-facing risk — the
  policy must be in the authoring docs, not just the compiler.
- **Binary save format, localization rendering, and presentation hooks** are out of
  scope (§10).
- **Long-range gradient quality and AI *decision* quality** remain gameplay-
  evaluation questions, exactly as the Mapping ADR states; ClauseThing establishes an
  authoring path and constitutional safety, not balance.

---

## 9. First slice & PR ladder — **superseded by the live track**

The original proposal ladder in this section was promoted, refined, and **opened**
as the `CT-` track on 2026-06-10. The live ladder, rung definitions, exit criteria,
and status ledger are in
[`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md).
This section is retained as provenance only; do not work from it.

---

## 10. Permanent exclusions & runtime constraints

These are the things that are genuinely out of scope — not sequencing gaps, but
architectural boundaries that hold regardless of how far ClauseScript coverage
advances.

**Permanent exclusions (will never be in scope):**
- **Binary save-format reading.** `jomini` can do it; ClauseThing will not. Save
  games are engine state, not authoring. This boundary is load-bearing.
- **Presentation-only constructs.** `custom_tooltip`, `defined_text`, localization
  keys, portrait/name lists, bracket localisations — dropped before runtime. SimThing
  has no display layer to route them to.
- **Stellaris engine-specific behavior that has no SimThing equivalent.** Some effects
  and triggers are pure Stellaris engine calls with no meaningful compilation target
  (e.g. galaxy rendering, save/load hooks, UI animations). These are rejected with a
  diagnostic. The test for "no equivalent": if there is no column, overlay,
  BoundaryRequest, EmitEvent, or threshold that captures the semantics, it stays out.

**Runtime constraints (how ClauseScript compiles, not how much we admit):**
- **No semantic WGSL, no new primitive, no new `AccumulatorRole`.** ClauseScript
  compiles onto *existing* substrate. Constructs that require a new primitive require
  that primitive to be designed and accepted first — then the ClauseScript mapping
  follows. This is sequencing, not exclusion.
- **No CPU AI planner or semantic sidecar.** `ai_will_do` is data the GPU reads.
  Commitments are threshold crossings. This is architectural, not a limit on
  expressiveness.
- **No `simthing-sim` awareness.** The simulation crate never sees ClauseScript
  concepts. All compilation terminates at the spec/driver layer.
- **No relaxation of `invariants.md`.** Every change to a binding invariant is Tier-2
  regardless of whether it was motivated by a ClauseScript mapping need.
- **Admitting a new construct is a gated decision.** Tier-1 if within accepted design
  and generic; Tier-2 otherwise. Expansion is expected and welcome — it just goes
  through the gate.

**RON stays valid and co-equal.** ClauseScript is not a replacement — it is a
parallel first-class surface. Designers choose the language that fits their
background and workflow. Both compile to the same `simthing-spec` contract.

---

## 11. Read order for agents picking this up

1. [`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md) — the live track: ladder, gates, status.
2. `design_0_0_8_1.md` §5 — gating governance; `simthing_core_design.md` — the paradigm (always in context).
3. `invariants.md` — binding constraints (the firewall is here).
4. `adr/resource_flow_substrate.md` + `adr_accumulator_op_v2.md` — the economic substrate ClauseScript flows compile onto.
5. `adr/mapping_sparse_regioncell.md` + `simthing_core_design.md` §7–8 — the mapping/FIELD_POLICY substrate ClauseScript fields and AI compile onto.
6. `capability_tree_v1.md` — the exact pattern ClauseScript traditions/edicts/tech reuse.
7. This document §4 (correspondence) and §5 (tiers).
8. `C:\Users\mvorm\Clauser\ClauseThing.md` + `Clauser/Paradox/script_documentation/*.log` — the language ground truth, before changing any compile mapping. **Lab-only material; never committed to this repo** (the logs are generated from proprietary game data).
9. `Clauser/jomini/README.md` + `src/` — the parser the front-end vendors (text path only).

---

*ClauseThing is the aspiration that ClauseScript becomes as natural a way to author
SimThing as RON — a full first-class surface, not a subset. The tiered ladder is a
sequencing discipline, not a ceiling. The runtime stays flat, deterministic,
GPU-resident, replayable, and semantic-free — and never learns that it is, in part,
speaking Clausewitz.*
