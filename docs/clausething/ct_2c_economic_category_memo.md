# CT-2c-DESIGN-1 — Economic Category Mapping Memo

> **Status: DESIGN MEMO / READY FOR REVIEW (2026-06-11).** This is a design/admission-shape memo,
> not an implementation. No production code rides with it, and CT-2c implementation may not start
> before design-authority acceptance. Authority: the 0.0.8.1 constitution,
> [`../simthing_core_design.md`](../simthing_core_design.md), [`../invariants.md`](../invariants.md),
> [`../design_0_0_8_1.md`](../design_0_0_8_1.md) §0/§2, the production track
> [`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md)
> §2/§3.1/§5/§6/§6.1/§7/§9, [`../adr/resource_flow_substrate.md`](../adr/resource_flow_substrate.md),
> [`ClauseThing_Spec.md`](ClauseThing_Spec.md), [`scope_memo.md`](scope_memo.md), the CT-2a report
> ([`../tests/ct_2a_impl_results.md`](../tests/ct_2a_impl_results.md)), and
> CLAUSETHING-MOVABLE-SEMMAP-0 (track §3.1). The ClauseScript textbook is consulted as reference
> only, under the track §6.1 secondary-source provenance caveat.

**Named consumer (what this memo unblocks):** **CT-2c implementation** — category-based
resource-flow hydration: `category_map` defaults + hard errors, the economic modifier-key grammar
decoder, `_mult`/`_add` inheritance asymmetry, `value:` literal-amount lowering, and the ladder's
exit proof (the Daily Economy Fixture authored in ClauseScript, canonically matching the existing
RON original). CT-2a (literal `produces`/`upkeep` → `IntrinsicFlow`) is the accepted floor this
memo builds on; category hydration is the next risk boundary.

---

## 1. What an "economic category" means in SimThing terms

**Decision: an economic category is hydration/admission metadata that compiles away entirely. It
is never a runtime participant, never a runtime taxonomy, never an engine.**

Concretely, a category is a named entry in a **registered category table** (the `category_map`
of `ClauseThing_Spec.md` §5 Tier 1) that resolves, at hydration time, to exactly three things:

1. **A default application level** — `(SimThingKind, tree depth)` — stating where in the recursive
   SimThing tree entries tagged with this category attach by default. Per the track §6 granularity
   rule (errata #1, verified against the primary source), the category is a **soft tag**: a
   lower-granularity category legally applies at a higher level with broadcast-down through the
   existing ancestor-stack overlay sweep. Hydration validates **direction only** (no cascade-up).
2. **A resource binding set** — which `(category × resource)` pairs map to which flow property
   keys and arena names. This is the table the modifier-key decoder (§3) resolves against.
3. **A reduction grouping** — which property columns a `_mult` subtree sweep covers (§4), i.e.
   which existing reduction OrderBands carry the category's aggregate.

This is the same posture the Resource Flow ADR pins for `AccumulatorRole`: *"compile-time spec
metadata, not a runtime participant taxonomy. … It must not become runtime semantic branching."*
A category that survived into the runtime as a dispatch tag would be a semantic economy subsystem
— the named forbidden interpretation. After hydration + admission, the runtime sees ordinary
properties, overlays, arenas, and OrderBands; `grep` for the category name in any runtime artifact
must come up empty (it may persist only in authoring-side ids/display strings).

**Classification (closure Q3):** category handling is **authoring sugar + admitted metadata** —
sugar at the ClauseScript surface (generated keys, category blocks), metadata in the ClauseThing
category table that steers hydration. It is *not* macro expansion in the CT-0c sense (no text
rewriting; the table is structured data with diagnostics), and categorically *not* runtime
semantics.

**The category table is ClauseThing-side data, not a `simthing-spec` struct.** `simthing-spec`
already expresses everything the lowering needs (properties, sub-fields with `accumulator_spec`,
overlays, arenas, EML trees, `ResourceEconomySpec`). Keeping the table in
`simthing-clausething` preserves the firewall: the spec layer never learns what a "category" is.

## 2. Supported CT-2c source shape

All fixtures are **original SimThing-authored ClauseScript** (track §7). No Paradox corpus text,
no `script_documentation` content.

### 2.1 The category fixture (smallest synthetic shape)

```
simthing_ct2c_categories = {
    category_map = {
        settlement = { kind = Cohort  depth = 2 }
        polity     = { kind = Faction depth = 1 }
    }
    resource = { id = "simthing_food" namespace = "simthing" name = "food" }
    resource = { id = "simthing_energy" namespace = "simthing" name = "energy" }

    unit_template = {
        id = "farmer"
        category = settlement
        resources = {
            produces = { settlement_food_produces_add = 6 }
            upkeep   = { settlement_energy_upkeep_add = 1 }
        }
    }

    modifier = {
        id = "irrigation"
        settlement_food_produces_mult = 0.10
    }
}
```

**Accepted fields (exhaustive — anything else is a spanned hard error):**

- `category_map { NAME = { kind depth } }` — explicit table entries. A small built-in default
  table (the `ClauseThing_Spec.md` §5 Tier 1 "conventional defaults") covers common names;
  `category_map` overrides per project; an unmapped category is a **hard error with a suggested
  mapping**, never a silent guess.
- `resource { id namespace name [display_name] }` — explicit resource registration. The resource
  set is closed and statically enumerated by the fixture; the key decoder resolves against it.
- `unit_template { id category resources { produces upkeep } }` — entries are **generated
  modifier keys with literal numeric amounts** (§3), or the CT-2a literal form
  (`property = ns::name` + `rate`), which remains valid unchanged.
- `modifier { id <generated_key> = <literal> }` — category-derived standalone modifiers,
  lowering to overlays per §4.
- `value:`-style amounts: **literal numerics and `@var` constant-folded literals only** at CT-2c
  (CT-0c already folds `@vars`). Full scripted-value formula trees (`value: { base add mult }`)
  lower to `EvalEML` `ExactDeterministic` ≤32 nodes **only if** the Daily Economy parity target
  (§2.2) requires a formula; otherwise the EML lowering is deferred to the rung that names it.
  No recursion beyond the declared depth cap; reject beyond (track §6.1 Class B).

**Rejected fields (spanned hard errors at hydration):**

- trigger-gated `produces` sub-blocks (`produces { trigger = {...} ... }`) — that is sub-ladder
  2b (`ClauseThing_Spec.md` §5 Tier 2), deferred until a rung names it; the diagnostic says so;
- `economic_category` **inheritance chains** beyond one parent hop (deep category trees deferred;
  the v1 table is flat with an optional single `parent =` for the `_mult` sweep grouping);
- shipsize-family keys (§3), iterator/selector forms, dynamic identifiers, scope chains beyond
  same-owner (SCOPE-MEMO §8 posture continues), category wildcards, `cost` keys without the
  discrete `ResourceEconomySpec` context to receive them;
- any unknown key form (§3 — no silent parsing).

### 2.2 The Daily Economy parity target (the ladder's exit criterion)

The track §5 CT-2c exit is: *"The Daily Economy Fixture, authored in ClauseScript, matches the
RON original."* The RON original is
`crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron` — a `GameModeSpec`
with five plain properties and a `ResourceEconomySpec` (`opt_in_mode: TransferOnly`, two
transfers, one recipe). The ClauseScript authoring of it therefore needs a small **discrete
dialect**: `transfer { id source target amount order_band }` and
`recipe { id input{…} target throttle… }` blocks with explicit `namespace::name` references and
literal amounts. This is mechanical hydration to the **existing** `ResourceEconomySpec` structs —
no category machinery is even required for the parity fixture itself; categories are proven on
the §2.1 fixture. Both proofs ride in CT-2c implementation.

## 3. Modifier-key grammar

**Decision: CT-2c accepts exactly one grammar family — the economic family — resolved by
longest-match against the registered category and resource sets. Everything else hard-errors with
a span and a suggested path.**

```
key := (category) "_" (resource) "_" (produces|upkeep|cost) "_" (add|mult)
```

- **Resolution is longest-match against registered sets, never naive `_` splitting** (track §6,
  binding): `pop_category_bio_trophy_unity_upkeep_add` resolves category
  `pop_category_bio_trophy` and resource `unity` only because both are registered; an
  unregistered segmentation is unresolvable and hard-errors listing the near-miss candidates.
- **A tie (two valid segmentations) is an ambiguity rejection**, mirroring SCOPE-MEMO §7.3 —
  never a silent precedence pick.

**Accepted examples** (against §2.1's registered sets):

| Key | Resolution |
|---|---|
| `settlement_food_produces_add` | category `settlement`, resource `food`, axis `produces`, op `add` |
| `settlement_food_produces_mult` | same, op `mult` → subtree sweep (§4) |
| `settlement_energy_upkeep_add` | category `settlement`, resource `energy`, axis `upkeep` |
| `polity_energy_cost_add` | accepted **only** inside a discrete `ResourceEconomySpec` context (adoption/build cost); otherwise hard error "cost requires a discrete-economy context" |

**Rejected examples:**

| Key | Diagnostic |
|---|---|
| `shipsize_corvette_build_speed_mult` | "shipsize grammar family is not admitted (no consumer names ships); see track §6" |
| `settlement_food_produces` | "missing op suffix `_add`/`_mult`" |
| `settlement_crystal_produces_add` | "resource `crystal` is not registered" + registered list |
| `village_food_produces_add` | "category `village` is unmapped" + suggested `category_map` entry |
| `triggered_produces_modifier` forms | "gated family generation is deferred (sub-ladder 2b)" |

**Bridge or durable API?** The decoder is the first slice of the **durable classifier engine**
the track §6 names (*"a family of grammars, not a dictionary"*). The economic family lands at
CT-2c as a permanent, registered-set-driven parser inside `simthing-clausething`; the shipsize
family (~69% of `modifiers.log` keys) waits for a consumer that names ships. The API shape is
durable; the family coverage is consumer-pulled. Per the track §6.1 provenance caveat, the
grammar is accepted at implementation only after it round-trips the lab `modifiers.log` economic
subset in the **ignored, lab-only** verification posture (CLAUSER_LAB_DIR opt-in, never CI, no
corpus text committed) — and that verification is evidence, not decoration.

## 4. Lowering target (existing substrate only)

**No new `AccumulatorRole`. No new GPU primitive. No WGSL. No `simthing-spec` widening (§8 of
the closure). Everything lowers to structs that exist today.**

| Authored form | Lowered artifact |
|---|---|
| `(cat, res)` pair first used by any `produces`/`upkeep` entry | One flow property per pair, **CT-2a's exact triple shape**: `Named("flow")` → `IntrinsicFlow`, `Named("allocated")` → `AllocatedFlow{arena}`, `Named("weight")` → `AllocatorWeight{arena}`; arena name = `"{category}_{resource}"` (deterministic, collision-checked) |
| `…_produces_add = N` on a template | Positive literal intrinsic-flow contribution (CT-2a sidecar-rate pattern: authored rate metadata + seeded fixture inputs; same `HydratedResourceFlowPack`-style carrier) |
| `…_upkeep_add = N` | Negative/consumption-rate contribution, same path |
| `…_produces_mult = F` / `…_upkeep_mult = F` | **Overlay** `TransformOp::Multiply(1+F)` targeting the flow sub-field, installed at the category's application level with **subtree sweep** through the existing ancestor-stack / reduction-OrderBand mechanics — the `_mult`-sweeps-the-category-subtree half of the track §6 inheritance asymmetry |
| `…_add` standalone modifier | **Overlay** `TransformOp::Add` applied **leaf-only** — the `_add`-is-leaf-only half of the asymmetry; no cascade |
| `…_cost_…` in a discrete context | Existing `ResourceEconomySpec` transfer/recipe entries (§2.2 dialect) |
| `category_map` entry | Hydration-side application-level + grouping resolution; **emits nothing** |
| Arena admission | Explicit `ArenaSpec` per emitted arena: bounded `max_participants` / `max_coupling_fanout` / `max_orderband_depth`, `FissionPolicySpec` declared, `explicit_participants` or bounded `EnrollmentSelectorSpec::InstallTarget`, `opt_in_mode` authored per CT-2a |

The CT-2a flow-property triple is preserved verbatim; no consumer at this rung names `Balance`
columns, so the triple does not widen to a quad (if a balance ledger is later needed, that is the
existing `AccumulatorRole::Balance` — registered metadata, still no widening).

**Inheritance asymmetry is reduction semantics, not new machinery.** `_mult` subtree sweep =
overlay Multiply masked down an ancestor stack the engine already walks; the category aggregate a
player observes = the existing reduce-up Sum over the grouped columns. The universal loop —
accumulate → reduce up → mask down → disburse down → threshold — is untouched; categories only
*decide where authoring attaches* to it.

**Movement-front note (binding context).** Category-derived flow aggregates reduce up into
exactly the per-parent columns that downstream field/heatmap consumers (the accepted Phase-M
movement-front substrate, CT-3b+4a's future vertical) read. CT-2c emits **no** `RegionFieldSpec`,
no stencil, no mapping structure — but the design must not foreclose that consumer: aggregates
stay ordinary reduced columns, which is precisely what the field substrate consumes. When a
movable participant reparents (§5), its contribution moves with it through subtree-incremental
arena refresh — so location-local heatmaps see flows change because *the SimThing moved*, never
because a category engine re-routed anything.

## 5. Movable semantic concepts under category economy (track §3.1, binding)

Category economy attaches to movables **through the SimThing they already are** — never through
a per-noun subsystem:

- **Leaders/characters** are mobile SimThings. A leader template tagged
  `category = court` with `…_upkeep_add` entries hydrates upkeep as **intrinsic-flow obligations
  on the leader's own property columns**, participating in the **current parent's** explicit
  Resource Flow arena when admitted. Bonuses/penalties (`…_mult`/`…_add` keys on the template)
  are **overlays reduced to the parent and disbursed down** through existing mask/modifier
  mechanics. Assignment is **reparenting or bounded assignment-slot parentage**. No leader
  engine, no global character registry, no CPU assignment planner, no out-of-band bonus applier.
- **Fleets, armies, ships, monsters, agents:** identical. Fleet upkeep is a property /
  resource-flow obligation on the fleet SimThing in its admitted arena — not a fleet-economy
  subsystem. A monster's drain on a region is an admitted arena participation, not a combat
  entity engine.
- **Pop cohorts** are recursive SimThings or cohort SimThing groups under a location/container.
  A `pop_category_*` category names their default attachment depth via `category_map`; there is
  no pop engine and no implicit population scan — cohort arena membership is explicit admission.
- **Movement** between gridcell/location/container nodes is **reparenting or an admitted
  movement-front transfer**. When a movable participant reparents, arena membership follows via
  the existing declared `FissionPolicy` + **subtree-incremental ArenaRegistry refresh**
  (Resource Flow ADR invariant) — re-evaluating admission for the changed subtree only. Category
  metadata plays no role at move time; it compiled away at hydration.

## 6. Bounded participation

- **Static enumerability:** the number of emitted arenas = |used (category × resource) pairs|,
  closed at hydration because both sets are explicitly registered (§2.1). No category may mint
  arenas open-endedly; an authored fixture that emits more than its declared `max_arenas`
  (fixture-level cap, default small) hard-errors at hydration with the expansion list.
- **Explicit participants only:** templates never auto-enroll. Participation is
  `explicit_participants` or a bounded `EnrollmentSelectorSpec::InstallTarget` resolved at
  install — the existing E-2B path. **Property possession never admits** (Resource Flow ADR
  guardrail, restated as binding here: holding a `settlement_food` column does not place a
  SimThing in the `settlement_food` arena).
- **Caps:** every emitted `ArenaSpec` declares `max_participants`, `max_coupling_fanout`,
  `max_orderband_depth`; `simthing-spec` admission fails the build on any computed-expansion
  excess, exactly as `e10_resource_flow_admission` proves today. No unbounded wildcards; a
  `wildcard_admission` without `max_expansion` is already rejected by the existing compiler.
- **No presentation-only expansion:** a category that binds zero entries emits nothing (no empty
  arenas, no placeholder properties).

## 7. Opt-in / default-off (unchanged from CT-2a)

- Continuous flow: GPU execution requires `ResourceFlowOptInMode::FlatStarOptIn` (or the accepted
  execution-profile path) authored explicitly; `ResourceFlowSpec` **presence alone stays
  inactive** — re-proven in CT-2c tests by the CT-2a `Disabled`-mutation pattern.
- Discrete economy: `ResourceEconomyOptInMode` (`TransferOnly` in the parity fixture) — same
  posture, already enforced.
- The global pipeline default stays disabled. No runtime/default schedule wiring of any kind
  rides with CT-2c.

## 8. Admission & diagnostics ladder

| Failure | Layer | Diagnostic |
|---|---|---|
| Unknown key form / grammar family | ClauseThing hydration | spanned hard error + suggested mapping (track §6 doctrine: the diagnostic stream is the backlog's priority queue) |
| Unmapped category | hydration | spanned + suggested `category_map` entry |
| Unregistered resource | hydration | spanned + registered list |
| Ambiguous longest-match segmentation | hydration | spanned ambiguity rejection (both parses shown) |
| Cascade-up granularity violation | hydration | spanned (direction check per errata #1) |
| Trigger-gated produces (2b), shipsize keys, deep category chains, iterators, dynamic ids | hydration | spanned "deferred" with the ticket/rung named |
| Arena cap excess, implicit participation, unbounded wildcard, coupling cycle | `simthing-spec` admission | existing `SpecError`s — the firewall is unchanged |
| EML whitelist / node-cap violations (if formula lowering is pulled in) | spec admission | existing EML registry errors |

Spans flow from the CT-0b raw model through `HydrateError::new_spanned`, exactly as CT-1a/CT-2a
already do. Nothing is warned-and-continued; nothing parses silently.

## 9. Existing substrate reuse (implementation must reuse, not duplicate)

1. **Continuous-path oracle:** `simthing_driver::run_arena_allocation_oracle` + the flat-star
   harness (`crates/simthing-driver/tests/support/e11_flat_star.rs`) — the same accepted
   GPU/CPU parity guard CT-2a consumed. No new economy oracle.
2. **CT-2a fixture pattern:** `crates/simthing-clausething/tests/ct_2a_intrinsic_flow.rs`
   (canonical-JSON RON parity, opt-in posture test, bounded-participation test, GPU vs oracle).
   CT-2c adds **one** sibling test target (`ct_2c_category_economy.rs`) following the identical
   pattern — not a permanent expensive battery; the GPU leg skips without a GPU like every other
   RF test.
3. **Discrete-path parity target:** `crates/simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron`
   + `deserialize_game_mode_ron` (already exercised by `daily_economy_session.rs` support).
   Hydration-level canonical equality against this fixture requires **no driver changes** at all.
4. **Admission proof:** `cargo test -p simthing-spec --test e10_resource_flow_admission` —
   reused as-is for cap/implicit-participation enforcement; CT-2c adds no spec tests because it
   widens no spec surface.

## 10. Implementation handoff — CT-2c-IMPL-0 (opens only on memo acceptance)

- **simthing-spec widening required: none.** All lowering targets exist. (If implementation
  discovers a missing literal field, it stops per the standing stop conditions and the widening
  gets its own ticket naming CT-2c as consumer.)
- **New AccumulatorRole: none.**
- **Driver tests:** reuse only. A driver-level session run is needed only for the GPU
  micro-economy leg, which lives in the ClauseThing test crate exactly as CT-2a's did
  (dev-dependency on `simthing-driver`/`simthing-gpu` already present). No driver
  production/runtime edits.
- **Allowed files:**
  - `crates/simthing-clausething/src/**` (category table, key decoder, category hydration —
    sibling module to `hydrate_resource_flow.rs`)
  - `crates/simthing-clausething/tests/**` (+ fixtures: `ct2c_categories.clause`,
    `ct2c_categories_baseline.ron`, `ct2c_daily_economy.clause`)
  - `docs/tests/ct_2c_impl_results.md`
  - `docs/design_0_0_8_1_clausething_production_track.md` (ledger row only)
- **Exact targeted tests:**
  - `cargo test -p simthing-clausething --test ct_2c_category_economy`
  - `cargo test -p simthing-clausething`
  - `cargo test -p simthing-spec --test e10_resource_flow_admission`
  - `cargo fmt --all -- --check`
  - **Never** `cargo test --workspace`.
- **Exit proofs:** (a) §2.1 category fixture → canonical RON baseline equality + bounded install
  + GPU micro-economy vs `run_arena_allocation_oracle`; (b) §2.2 ClauseScript Daily Economy →
  canonical equality vs `daily_economy_banking_scenario.ron`; (c) hard-error suite for §3's
  rejected keys and §2.1's rejected fields.
- **Frontier gating:** the track §5 marks CT-2c **[FRONTIER]**; implementation stays with a
  frontier-tier agent (the inheritance asymmetry and longest-match decoder are exactly the
  silent-fidelity-loss surface the gate exists for).

## Exact GPU sqrt rule (standing doctrine; not exercised here)

Any GPU-resident sqrt/magnitude/distance/gradient-norm/movement-front-norm/threshold path
claiming exactness must route through `m_jit_sqrt_f_exact` (hash-pinned, artifact-backed
Candidate F); native WGSL `sqrt` is `ApproximateJitOnly`/diagnostic and cannot close
exact-authority tests; a red Candidate F guard is a constitutional alarm. **CT-2c requires no
sqrt or magnitude anywhere** — category economy is rates, overlays, and reductions. If
sqrt/magnitude appears during implementation, stop and escalate.

## Closure answers

1. **Named consumer:** CT-2c implementation (category hydration + Daily Economy ClauseScript parity).
2. **Category meaning:** registered hydration/admission metadata resolving to application level,
   resource bindings, and reduction grouping; compiles away (§1).
3. **Classification:** authoring sugar + admitted metadata; never macro text-rewriting, never
   runtime semantics (§1).
4. **Accepted source shape:** §2.1 (category fixture) + §2.2 (discrete dialect for the parity target).
5. **Accepted grammar:** the economic family `(cat)_(res)_(produces|upkeep|cost)_(add|mult)`,
   longest-match against registered sets (§3).
6. **Rejected forms:** shipsize family, unknown/ambiguous segmentations, unmapped
   categories/resources, gated 2b forms, deep chains, iterators, dynamic ids (§3, §8).
7. **Emitted structures:** CT-2a flow-property triple + `ArenaSpec`/`ResourceFlowSpec`; overlays
   for `_mult`/`_add`; existing `ResourceEconomySpec` for the discrete dialect (§4).
8. **simthing-spec widening:** none required.
9. **New AccumulatorRole:** none.
10. **Opt-in/default-off:** preserved; explicit `ResourceFlowOptInMode` / `ResourceEconomyOptInMode` (§7).
11. **Spec presence alone:** stays inactive; re-proven by the CT-2a Disabled-mutation test pattern (§7).
12. **Unbounded wildcards:** none; existing rejection paths reused (§6).
13. **Bounding:** static (cat × res) enumeration, fixture arena cap, per-arena declared caps,
    explicit/bounded enrollment only (§6).
14. **Movables:** leaders/characters/fleets/pops/monsters remain ordinary SimThings per track
    §3.1; upkeep = property/flow obligations, bonuses = overlays, assignment = reparenting/bounded
    slots, movement = reparenting/admitted transfer + subtree-incremental arena refresh (§5).
15. **simthing-sim:** untouched, arena-ignorant, ClauseThing-blind; categories never reach it (§1, §4).
16. **GPU/WGSL:** no changes; no new primitives.
17. **CPU fallback economy:** none; CPU appears only as the existing bit-exact oracle (§9).
18. **sqrt/magnitude:** not required; standing rule restated above.
19. **Reused tests/oracles:** `run_arena_allocation_oracle`, flat-star harness, CT-2a fixture
    pattern, `daily_economy_banking_scenario.ron`, `e10_resource_flow_admission` (§9).
20. **Superseded artifacts:** none created; single durable memo retained.
21. **Visibility artifacts:** this memo only (under `docs/clausething/`); no docs/tests report needed.
22. **`cargo test --workspace`:** not run (docs-only rung).
23. **Ledger:** CT-2c row → DESIGN MEMO / READY FOR REVIEW, honestly (implementation not started).
