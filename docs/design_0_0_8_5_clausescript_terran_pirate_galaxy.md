# 0.0.8.5 — ClauseScript Terran-Pirate Galaxy Production Track

> **Status: OPEN / DA-OPENED (production track, Phase 0; execution opened 2026-07-01 at `TP-TRACK-OPEN-0`).** Resequenced
> 0.0.8.4 → 0.0.8.5 (2026-06-28, owner-directed). Prerequisite ladder landed in the tree:
> [`0.0.8.4 Admission Substrate`](design_0_0_8_4_admission_substrate.md) (`AS-CLOSEOUT-0` CLOSED),
> [`0.0.8.4.5 simthing-kernel`](design_0_0_8_4_5_simthing_kernel.md) (`KERNEL-CLOSEOUT-0` CLOSED),
> [`0.0.8.4.6 CI scaffolding`](design_0_0_8_4_6_ci_scaffolding.md) (Track A + Track C DA-CLOSED).
> Implementation begins at `TP-RF-CAPACITY-AMENDMENT-0`. Owner-authorized maximal scope (2026-06-28). Sits
> *beneath* [`simthing_core_design.md`](simthing_core_design.md) (permanent paradigm, incl. **§1.2 the
> admission substrate**) and *beneath* the active constitution [`design_0_0_8_3.md`](design_0_0_8_3.md).
> This document is the authoritative 0.0.8.5 production design track and PR ladder. It opens the **first full-scenario native ClauseScript
> ingestion vertical**: a single authored `.clause` file that transpiles entirely to canonical
> `SimThingScenarioSpec` and runs live, STEAD-driven, through indefinite ticks in SimThing Studio.
>
> **This is a TEST SCENARIO track (§0A, binding).** Everything it mints — tests, binaries, `.clause`
> artifacts, decoder rules, heuristics — is contained within the scenario test envelope
> (`birth_track = 0.0.8.5-terran-pirate`), **expirable and NOT canonical**. Nothing here becomes SimThing
> corpus doctrine except by a **separate, specific DA admission review conducted AFTER the scenario is
> completed and test-run.** No in-track canonization; surface for admission, never self-grant.
>
> Detailed per-rung evidence lands in `docs/tests/*_results.md` and
> [`docs/tests/current_evidence_index.md`](tests/current_evidence_index.md).

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable, load-bearing — hold all of these in context every rung):**

1. [`design_0_0_8_3.md`](design_0_0_8_3.md) **§0** — the transient constitution (carry-forward doctrine, anti-flattening §0.6, exact-magnitude §0.7, STEAD §0.8, closed-lowering-layer gates §A).
2. [`simthing_core_design.md`](simthing_core_design.md) — the permanent paradigm (the one tree §2, RF arenas §5, overlays §6, the Movement-Front automaton §7, decisions-as-thresholds §8, drift detectors §9).
3. **This file** — the 0.0.8.5 canonical design file.
4. [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4 (the deep correspondence table) + §8 (sequencing hard problems) — the ClauseScript→SimThing isomorphism and its known gaps.
5. [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md) — the modifier-key decoder, the `_mult`/`_add` inheritance asymmetry, and the movables doctrine (fleets/cohorts/leaders are ordinary SimThings).
6. [`stead_spatial_contract.md`](stead_spatial_contract.md) — the 8 STEAD invariants (mandatory for any MapGen / Location-grid / Movement-Front / Gu-Yang / PALMA / RF-over-gridcell work).
7. [`ci_screening_surface.md`](ci_screening_surface.md) — the live CI **carrot+stick** layer (0.0.8.4.6, CLOSED): the screening logic, the **agent onboarding procedure (§7)**, and the per-track addendum standards (§8). Every PR of this track is gated by the doctrine-scan; **follow §7 every rung.**

A rung handoff may add ≤3 rung-local links it directly consumes; rung-local links never accrete into this base.

**Established decisions — do NOT re-derive (one screen):**

- **Everything is a SimThing.** Owners are GameSession siblings, never spatial parents. Ownership = owner-columns + permanent identity overlays. Capture = column flip, never reparenting. (core §2, §6)
- **All conflict/opportunity/ambition/diplomacy is resource flow:** `accumulate → reduce up → settle locally → mask/disburse down → threshold crossings fire decisions`. No combat engine, no economy engine, no diplomacy engine, no AI engine. (core §1, §5; constitution §0.3)
- **Combat is HP/Damage economics** (constitution §0.3, core §5.3): damage = `SubtractFromSource` transfer; HP recovery = `governed_by`; death = a zero-crossing `Threshold` → `EmitEvent` → `BoundaryRequest`. **Owner bonuses disburse down as overlays on the HP/Damage columns.**
- **Diplomacy is influence/distrust economics** (core §2.1, §5.3): trust/influence is an ordinary flowing quantity reduced up and disbursed down; a stance change is a registered threshold crossing on the reduced trust/distrust column. No diplomacy subsystem.
- **Hull designs / ship classes are capability trees** (ClauseThing_Spec §4): one `Custom(...)` SimThing; progress = sub-fields; unlock = `Suspended`→`Permanent` overlay; cost = flow drain. Never a runtime `match kind`.
- **Decisions are GPU-resident threshold crossings — FIELD_POLICY, never a CPU planner** (core §8). The AI reads the front and acts when a personality-weighted pressure column crosses a named threshold. The CPU only consumes structural events at boundaries.
- **The map is the Movement-Front automaton** (core §7): bounded-horizon falloff (P1), one shared stencil (P2), attractor/threshold projection (P3). Gu-Yang `SaturatingFlux` for borders/chokes; PALMA for reach/impedance. The front *is* the route; fleet movement is gradient-following reparenting, never a route solver.
- **`simthing-sim` and WGSL are semantic-free.** ClauseScript never crosses into the sim crate; every modifier/category/trigger compiles away at hydration/admission. After ingestion, `grep` for "terran"/"pirate"/category names in any runtime artifact comes up empty (authoring ids/display strings only). (ClauseThing_Spec §7)
- **Exact magnitude gates route through Candidate F** (constitution §0.7). Native `sqrt`/`length`/`distance` are `ApproximateDiagnostic` only.
- **Closed lowering layers are closed** (constitution §A): a producer/front-end change makes zero edits to closed `crates/simthing-clausething/src/` lowerers *except* under a DA-authorized amendment. **This track carries two such owner-authorized amendments — see §1.**
- **No silent tier collapse; Deviation Record + Scope Ledger on every closure** (constitution §0.6). Parking a specified tier is a recorded, approved Deviation — never an implicit free pass.
- **The CI carrot+stick layer (0.0.8.4.6, CLOSED) is live and applies to every PR here.** *Stick* — the GitHub `Doctrine Scan` gate screens every PR (clean **RELIABLE** = DA-equivalent; `FAIL` = HOLD; `INSPECT` → §1A triage); it now **mechanizes several of this track's own invariants** (the `SEMANTIC-WORDS`/`SIM-KIND-READ` scans enforce the "grep for terran/pirate comes up empty" rule above; the sealed-kernel scans enforce the one-authoritative-path rule). *Carrot* — **follow the onboarding procedure** ([`ci_screening_surface.md`](ci_screening_surface.md) §7): read the **sanctioned-surface digest** (`docs/sanctioned_surface.md`) *before* grepping the kernel surface, and run the **inner-loop self-scan** (`cargo check -p` + `doctrine_scan.sh`) during edits — the digest is your pre-computed grep answer. If a Terran-Pirate-specific anti-pattern chronically fires (triage log evidence), this track may author an **opt-in per-track CI addendum** (§8) — additive-only, never widening the global floor. Architectural insight a seal blocks routes through the **breakthrough valve** (gated + invited): surface a baseline-backed proposal, never self-grant.

---

## 0A. Scenario test envelope — this is a TEST SCENARIO track, nothing it mints is canonical (binding, DA 2026-07-04)

> **0.0.8.5 is a test *scenario* track, not a corpus-defining track.** Its purpose is to *prove a
> capability* — that a complete native ClauseScript file ingests, transpiles, and runs live under STEAD —
> **through one scenario**. It is the engine's *first consumer*, never its definition (core §1: "the
> substrate, not the game, is the product"). This section is the binding fence that keeps a scenario from
> silently minting permanent doctrine.

**The envelope.** Every test, test binary, fixture, `.clause` authored artifact, decoder rule, tuning
constant, and heuristic that arises anywhere under this track is **contained within the Terran-Pirate
scenario test envelope.** In the test-lifecycle ledger (0.0.8.4.6, live) every such row carries
`birth_track = 0.0.8.5-terran-pirate`. By construction of the Necessity Test and the birth-track expiry
tripwire, these are **scenario-scoped and expirable by default** — subject to deletion at track close unless
each independently earns a durable class or a reviewed `downstream-utility:` lease. They are **NOT
canonical, NOT inviolate, and NOT doctrine.**

**Nothing surfaced here is corpus by default — admission is separate, specific, and post-completion.** A
concept, code path, test, decoder heuristic, or tuning that appears worth abstracting *into* the SimThing
corpus — a `simthing-kernel` type or EML opcode (core §1.2/§1.2.1), a core-design paradigm change, an
`invariants.md`/STEAD contract line, a permanent-residue test class, or a doctrine-scan rule — **is admitted
to the corpus only by a dedicated, explicit DA review and admission that (a) happens AFTER the scenario is
completed and test-run, and (b) names the specific artifact and the rung by which it climbs the admission
ladder.** During the track:

- **No in-track canonization.** A TP rung may *use* an artifact within the scenario, but it may **not**
  declare any TP-born concept/test/heuristic canonical, inviolate, or corpus doctrine. A handoff or PR that
  writes a TP-surfaced heuristic into `simthing_core_design.md`, the constitution, `invariants.md`, the
  kernel seals, or the durable-test classes **is out of scope and rejected at review** — it must instead be
  *surfaced for the post-completion admission review*.
- **Surface, never self-grant** (breakthrough valve, core §1.2.1 / constitution §0.9.6). A genuine
  structural win the scenario reveals rides risk-free alongside the conformant rung and is **surfaced** to
  the DA/Owner for admission; it never self-canonizes mid-track. The admission is the owner-gated door, not
  a TP rung's prerogative.
- **The closeout produces the admission candidate list, not the admission.** `TP-DA-CLOSEOUT-0` (Workplan
  Closure Track — Owner-triggered only, not an automatic Phase 8 production pointer)
  additionally enumerates the concepts/code/tests/heuristics the scenario surfaced as *candidates for corpus
  abstraction* — a list, with each item's proposed admission rung — handed to a **separate post-completion
  admission review**. Closing the scenario does not admit any of them; it only nominates.

**Why this fence exists.** A scenario is high-entropy, exploratory, and tuned to make *one* thing work; its
tests and heuristics are proof-of-a-capability, not proof-of-a-paradigm. Absorbing them into the corpus
unreviewed is exactly the fossil-premise / test-propagation failure the test-lifecycle regime was built to
end. The corpus stays small, typed, and inviolate; the scenario stays contained, expirable, and — only after
it works and is reviewed — a *source* of admission candidates, never an author of doctrine.

### 0A.1 The physical fence — scenario candidate code lives in the `simthing-workshop` leaf, not the sealed core (binding, DA 2026-07-04)

The `birth_track` ledger above is *lifecycle bookkeeping*. Two distinct properties are at work here, and
only one is structural — **do not conflate them**:

- **Containment** (code *in* workshop cannot leak *up* into the sealed core) is **structural, compile-time**:
  the crate dependency arrow, enforced by `cargo`. Solid.
- **Homing** (new scenario code must be *written into* workshop, not into a sealed crate) is **NOT** enforced
  by the arrow — the arrow contains workshop, it places no fence around `simthing-clausething`/`spec`. Homing
  is enforced by **classification-at-review** plus a delta-scoped tripwire (`SPEC-LOWERER-KIND-READ` today,
  kind-branching only; a broader net-new-engine-symbol tripwire is queued). Until that lands, an engine-crate
  addition in a scenario PR is **classify-before-merge**, never silently landed.

- **Home.** Every candidate *service / struct / function / heuristic* a proofing scenario needs beyond its
  authored `.clause` data lives in **`simthing-workshop`**, never in a sealed engine crate.
- **The Homing Boundary — one-line classifier: "would this code exist if this scenario didn't?"** If **no**,
  it is scenario candidate code → `simthing-workshop`. If **yes** — a genuinely generic, semantic-free
  ClauseScript language/lowering surface *any* scenario would want (e.g. extending a generic decoder family
  with a new generic form, as `TP-SHIPSIZE-DECODER-0` did) — a sealed engine crate is fine. **Not** allowed in
  a sealed crate: any scenario-specific service/struct/fn/heuristic — an HP/Damage resolver, fleet-contact
  logic, owner-bonus combat helper, zero-HP removal, an RF-child-depth workaround, or Terran/Pirate/Fleet/Cohort
  semantic branching. *"Generic lowering, as prior TP rungs did it"* is **not** a licence: prior rungs predate
  this doctrine; the classifier, not precedent, governs.
- **Substrate widening (future utility) — DA/Owner-authorized only; agents surface, never self-grant.** A
  genuinely generic capability a scenario *surfaces* — a reusable, semantic-free API a crate's use case needs —
  is admissible in an engine crate as future utility (this is the "yes" branch; the test is *reusability*, not
  scenario-neediness). **But this route flows only top-down from DA/Owner approval.** An agent may **propose /
  appeal** it to the orchestrator when it is genuinely the most performant/logical path — *surface, never
  self-grant* (breakthrough valve, core §1.2.1 / constitution §0.9.6). "Future utility" is a **request the DA
  adjudicates**, never a verdict an agent issues to itself. An agent that self-classifies its code as "generic
  widening" and lands it in an engine crate on its own authority is **drift, rejected at review** — because
  "downstream value" is nearly unfalsifiable and self-serving (any scenario helper can be narrated as generic),
  so the burden of proof sits on the appeal and the **default is deny → workshop-home it** (consumer-side). The
  DA's adjudication question: *genuinely reusable and gameplay-concept-free, or scenario code in a generic
  costume?*
- **Why it contains.** `simthing-workshop` is a **verified leaf**: nothing in the tree depends on it (it
  only depends *inward*). Game-semantic candidate code placed there therefore **cannot leak upward into the
  sealed core by linkage** — the seal law (`simthing-kernel` authority; no inbound arrows to the sealed
  core) makes the containment a compile-time fact, not a promise. Workshop is also already outside every
  fence-scan target, so it needs no carve-out to be the sanctioned game-semantic zone. This is the guarantee
  a dedicated per-scenario crate would give; the workshop leaf already gives it, so no new crate is minted.
- **Elevation re-fences.** Moving code `simthing-workshop` → an engine crate is a reviewed, admission-gated
  act: the outbound diff must be generic-namespaced and game-semantic-free, and passes the engine-crate fence
  scans (which now cover `simthing-spec` + lowerers — see `design_0_0_8_4_6` / `CI-SCAN-SPEC-KIND-COVERAGE-0`).
  The fence isn't removed by living in workshop; it is relocated to workshop's *exit*.
- **Default-delete at closeout, no registry.** Scenario candidate code is expirable by default — deleted at
  track close via the existing lifecycle expiry sweep (orchestrator closeout duty, `ci_screening_surface §11`)
  unless explicitly kept. A candidate worth keeping is *moved by explicit decision* into standing
  `simthing-workshop` code; its value is **decided, not tracked**. There is deliberately **no registry, no
  `dsu` counter, and no re-justification tier** for kept candidates (considered and rejected as redundant with
  default-delete).
- **Consequence for the fleet-RF case.** The arbitrary-depth child-RF reduce-up the trial run needs is built
  **in `simthing-workshop`**, consumed there by the trial run (no starvation, no shim, no mid-track mutation
  of `simthing-spec`), and screened at closeout: semantic-free + generic-namespaced → elevate to
  `simthing-spec`; kind-matched → deleted. The Fleet/Cohort drift that reached review would have had no
  business in `simthing-spec` in the first place.

> **Deferred elaboration (not yet in force).** A per-production `simthing-workshop/src/testthing/<production>/`
> sub-taxonomy (unit-deletable subtree, a scan carve-out, and a mechanical `--track-closeout` emptiness gate)
> is the natural next step *when* workshop begins to fill and needs per-expedition sub-organization. It is
> **deferred** — until then the simple rule above (candidate code in the `simthing-workshop` leaf,
> default-delete at closeout) is the whole mechanism.

---

## 1. Owner authorization & mandate (recorded verbatim by instruction)

The project Owner opened this track (2026-06-28) with **maximal authorization to expand the
ingest/authoring capability of SimThing Studio** toward the horizon goal of *full ClauseScript ↔
SimThing runtime fluency*, and ultimately ingesting and transpiling Paradox's full Stellaris base
configuration. The 0.0.8.5 scenario is **Objective A**: prove the capability of ingesting a complete,
native ClauseScript file (authored in-repo) and running it as a live SimThing simulation.

Two normally-gated expansions are **owner-authorized as in-scope for this track**:

1. **Closed-lowerer RF capacity amendment.** Raise/scale the RF participant & slot caps and the GPU
   slot/emission capacity so a galaxy-scale tree (1500 systems, ~250 owned, fleets, cohorts, factories)
   can admit / install / run. (Today: arena `max_participants` defaults of 8/16 in
   `simthing-driver/src/arena_registry.rs`; `SCENARIO_STRUCTURAL_INTEGER_MAX = 16_777_216` in
   `simthing-spec/src/spec/scenario.rs`; GPU slot/emission capacity fixed at session attach.) This is
   the **DA-authorized closed-lowerer capacity amendment** the 0.0.8.3 constitution §A named as the
   outstanding gate before galaxy-scale packs can install. It is opened here.

2. **The shipsize / `triggered_produces_modifier` modifier family.** ClauseThing_Spec §8 and the CT-2c
   memo §3 deferred this family (~69% of the `modifiers.log` key space) *"until a consumer names ships."*
   **This scenario names ships and fleets — it is exactly that consumer.** The family is pulled into
   scope here.

> **Owner stress-test instruction (recorded verbatim by the Owner's request, 2026-06-28):** *The Owner
> directed Claude to use the particularly hairy part of the ClauseScript modifier language — the
> `(category)_(resource)_(produces|upkeep|cost)_(add|mult)` underscore modifier-key chains, the
> `pop_category_*` / `_mod_pop_*`-style chains, `shipsize_*`/`ship_*` weapon/hull keys,
> `triggered_modifier { potential ... }`, `complex_trigger_modifier`, and `value:`/`ai_will_do` scripted
> modifier blocks — **liberally** when authoring this scenario, specifically to stress-test the intake
> stack and force the deep reduction to `EvalEML` opcode chains that proved difficult without it.* The
> authored `.clause` is therefore deliberately adversarial against the decoder, not minimal. The
> modifier catalogue the author must exercise is **Appendix A**.

---

## 1A. DA adjudication of the orchestrator review + orchestration discipline (binding, 2026-06-28)

Codex 5.5 Max (orchestration agent) reviewed this track against the repo and filed a caveat list. As
executive Design Authority I adjudicate it below, then issue binding orchestration directives. **The
through-line: Codex's engineering-precision points are largely sound and are folded in; Codex's
*instincts* — treating RF-arena semantics as subsystem-sized work, accreting proof chains, inventing
resolution mechanisms — are the exact drift the constitution forbids, and are fenced off here.**

### 1A.1 Rulings on the eight caveats

1. **Combined-document grammar — UPHELD, with a simplicity sledgehammer.** The base
   (`static_galaxy_scenario` neutral-AST) and overlay (`scenario`-container) are two parse paths today.
   The resolution is **one grammar, never a third path**: the scenario-container front-end accepts the
   MapGenerator base via the **existing neutral-AST parser** as an embedded/`include` base block; base
   system ids are namespaced and become the overlay's location-targets; duplicate ids hard-error; the
   producer owns base provenance, the overlay owns runtime. Absorbed into `TP-BASE-EMBED-0` (sharpened).
2. **1500 vs 1000-star evidence confidence — REDUCED (already handled).** 1500-star *placement/topology*
   is guarded (`topology_stead.rs`); *admit/install/GPU* at 1500 is **unproven** and is exactly what
   `TP-SCALE-ENVELOPE-0` exists to establish. No rung overclaims; the summary's confidence is corrected
   here, the ladder already had the right rung.
3. **RF capacity amendment underspecified — UPHELD, but bounded.** `TP-RF-CAPACITY-AMENDMENT-0` must emit
   **one concise capacity-budget ledger** (SimThing count, property columns, RF arenas + per-arena
   participants/coupling-fanout/orderband-depth, emissions, GPU slots, field buffers, atlas theater size,
   readback policy, explicit "no per-tick allocation" assertion). **One table — not a proof battery**
   (directive D4). Raising `max_participants` alone is insufficient; all three `GpuArenaDescriptor` caps
   plus slot/emission capacity scale together, budget-driven.
4. **Atlas/theater execution — UPHELD as a scope *tightening* (smaller, not bigger).** The live run is
   **one deterministically-selected bounded theater = the contested Terran/Pirate border sub-volume**, not
   the whole galaxy. Theater field state is **runtime cache**, never ScenarioSpec writeback, unless a
   structural commitment fires through `BoundaryRequest`. Halo/gutter/stitching and full-galaxy atlas
   tiling are recorded Deviations/future — **do not build an atlas scheduler for this track.** Absorbed
   into Phase 6 + `TP-LIVE-RUN-0`.
5. **Fleet homing — RULED (DA decision; no new tier).** Studio doctrine: star-system gridcells carry
   **10×10 local grids**; **orbital bodies/starbases are local-grid cells**; inert local-grid cells
   **already carry receiver grids**; gameplay children attach at a **1×1 surface gridcell**. Therefore a
   fleet berths at the **1×1 surface gridcell of a star-system local-grid cell** — a planet's surface when
   garrisoning, or an orbital/space local-grid cell's surface when patrolling — and **movement = reparent
   to an adjacent system's local-grid cell surface**. This honors "fleets are children of the surface
   gridcell" verbatim while keeping mobility = reparenting. **No "fleet berth" new structure is minted.**
   Absorbed into §3 (ruling) + `TP-FLEET-MOVEMENT-0`.
6. **Trigger-to-column vocabulary — REDUCED (already resolved doctrine; Owner was correct).**
   ClauseThing_Spec §8: `complex_trigger_modifier` / bool→number forms **only compile when the trigger
   reads a column; otherwise rejected at admission.** The "explicit trigger-to-column table with hard
   errors" Codex requests **is the existing admission behavior** — minting a separate ceremony table is
   exactly the hygiene the Owner forbids. The scenario uses only column-backed triggers (`is_at_war` → a
   war-state flag column; `has_border_threat` → the reduced threat-front pressure column), declared
   **inline at point of use**; `from`/`root` dynamic chains are out (authored same-owner/same-scope,
   already deferred). No new artifact.
7. **Semantic-free scan — UPHELD (narrow it).** The scan targets **runtime / GPU / `simthing-sim`
   artifacts**, not ScenarioSpec authoring strings. Stable ids, provenance, display names, and Stellaris
   star names (PR #936) legitimately persist on `Location` SimThings. Rule = **"no semantic tokens below
   the spec boundary,"** not "no name strings in ScenarioSpec JSON." Absorbed into `TP-FULL-TRANSPILE-0`.
8. **Corpus hygiene — UPHELD (one line).** The authored `.clause` is **original SimThing-authored
   ClauseScript**; no Paradox content is committed; any `modifiers.log` round-trip uses the **ignored,
   lab-only `CLAUSER_LAB_DIR` posture with provenance hashes** (CT-2c §3). Standing discipline restated.

### 1A.2 Orchestration directives (binding on the orchestrator and every implementation handoff)

These exist because the orchestrator *admitted* it would have built combat/diplomacy as subsystems and
underweighted the anti-flattening and ClauseScript→ScenarioSpec boundaries. They are the §0 drift
detectors aimed at the demonstrated failure modes. A handoff that violates one is **rejected at review,
not implemented.**

- **D1 — More SimThing, never a subsystem.** Combat, diplomacy, economy, raiding, fleet movement,
  suppression are RF arenas + overlays + EML + thresholds. If a handoff proposes a combat engine, a
  diplomacy module, a pathfinding/route service, a CPU planner/urgency loop, or any "system beside the
  tree," it is rejected (core §9 detectors 1–4). The deferred labels in Studio are *consumer-pulled RF
  rungs*, not conceptual blockers.
- **D2 — Reach for the existing substrate, in this order, before inventing anything:** (1) an RF
  allocator arena; (2) an overlay on a weight / HP / Damage / flow column; (3) an **EML gadget tree over
  the fixed `EvalEML` interpreter**; (4) a **JIT EML→WGSL** straight-line shader (default-off, pinned,
  CPU-oracle parity). A new opcode / kernel / `AccumulatorRole` is **Tier-2, last resort, bit-exact
  parity required.** Inventing a new resolution mechanism while (1)–(4) suffice is the canonical drift
  this track exists to prevent (core §4.1 ladder).
- **D3 — No hygiene looping.** Reject docs-only, comparison-only, report-aggregation, or status-row rungs
  unless they directly enable a §2 acceptance element. No "project-management cosplay" (constitution
  §0.6.5). The diagnostic stream is the backlog — not a deliverable to be polished.
- **D4 — Proof is minimal and load-bearing.** **One** targeted test (or one tiny sibling test) per rung;
  GPU leg skips without a GPU; CPU-oracle parity to the bit. **No long proof chains, no sprawling test
  batteries, no accreting evidence artifacts.** `cargo test --workspace` is never run. A rung's evidence
  is one `*_results.md` with a Scope Ledger — token- and disk-cheap by mandate.
- **D5 — Anti-flattening *and* anti-ceremony.** Specified recursive structure (the surface tier, the
  recursive RF settle) must be **real** (no silent collapse, constitution §0.6) **and** proven by the
  **smallest non-vacuous reduction** — never a proof factory. Both halves bind simultaneously.
- **D6 — The ClauseScript→ScenarioSpec boundary is one-way and total.** After hydration,
  `simthing-sim` / WGSL / runtime never see ClauseScript, categories, modifiers, or scenario semantics
  (ClauseThing_Spec §7). Authoring-side ids / provenance / display names persist legitimately (ruling 7).
- **D7 — Handoffs are short and cite the harness.** Detail lives in code + this canonical design file;
  the header points, it never restates (constitution §0.5 Rule 3). Long handoffs are themselves a drift.
- **D8 — Enforcement is admission *behavior*, not a governance *artifact* (the noun-for-verb fence).**
  When a rung must "ensure / validate / govern" that authored input is well-formed — triggers resolve to
  columns, modifier keys resolve by longest-match, ids don't collide, caps aren't exceeded — the
  deliverable is a **spanful hard error in the decoder / admission layer with a suggested path**, never a
  new registry, preflight table, trigger-to-column governance doc, validation ledger, or ceremony rung
  that *restates* admission doctrine already in force. The binding review question is **"does this authored
  form lower to a concrete column / overlay / arena, with a spanned hard error if not?"** — **never "where
  is the table?"** (Carve-out: a bounded *one-table design output* like the `TP-RF-CAPACITY-AMENDMENT-0`
  budget ledger is an amendment's artifact, not governance ceremony; the test is whether the document
  *duplicates behavior the admission layer already performs* — if so, it is the drift this fence kills.)
  This is the specific reflex the orchestrator demonstrated and admitted; it recurs unless named.

**Every rung handoff under this track MUST use the base template
[`handoff_template.md`](handoff_template.md),** which operationalizes D1–D8 and carries the §H
anti-kabuki rules + the context spine. A handoff that omits the spine, pads the reading list, batteries
type-/admission-guaranteed conditions, restates the scope diff as bespoke guards, triple-updates docs, or
hand-authors the implementation inline is **rejected at review** (template §H).

---

## 2. Objective & acceptance

**Deliverable:** a single native ClauseScript file —
`crates/simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause` — that:

1. **Describes a 1500-star disc galaxy consistent with the Studio galaxy generator.** The base lattice
   (disc shape, structural `(col,row)` placements, hyperlane topology, Stellaris-namespace star display
   names) is produced by `simthing-mapgenerator` (the closed producer) and embedded/referenced as the
   authored base; the ClauseScript adds the runtime overlay on top. The base **is byte-consistent with
   what Studio would generate** for the same seed/shape (proven by regenerating it).
2. **Transpiles fully to canonical `SimThingScenarioSpec` JSON.** After ingestion, *only* SimThing-Spec /
   ClauseThing scaffolding remains — no ClauseScript modifier/category/trigger concept survives into the
   runtime authority or any GPU artifact.
3. **Injects the Terran-Pirate runtime scenario at galaxy scale:**
   - **Terrans own 200 star systems** in a contiguous disc volume. Owning a star system = owning all child
     planet systems under it. Each owned system has **≥1 planet**, and each such planet has **≥1 factory
     building + ≥1 Terran population cohort** under its surface gridcell — which is what bestows ownership
     (owner-column + identity overlay flip on the system subtree).
   - **Pirates own 50 star systems** in a volume adjacent to the Terran volume, each with a planet carrying
     a factory + cohort.
   - **The remaining ~1250 systems are light-payload neutrals** (per Owner decision): each a gridcell
     Location with a minimal planet + 1×1 surface gridcell, **no** cohort/factory/owner — raidable/
     colonizable targets that give the STEAD fronts something to contest.
   - **Terran fleets:** **200 ships in 10 fleets**, distributed by the **60-40 rule** — 60% (6 fleets /
     ~120 ships) securing the border connections to Pirate systems; 40% (4 fleets / ~80 ships) in the
     interior and other borders.
   - **Pirate fleets:** **400 ships in 10 fleets**, distributed by the **80-20 rule** — 80% (8 fleets /
     ~320 ships) poised to raid/disrupt Terran space; 20% (2 fleets / ~80 ships) protecting Pirate space.
4. **Runs live through an indefinite number of ticks** in SimThing Studio, with **all decision-making
   made entirely by STEAD** — each faction perceiving threats and needs organically as Movement-Front
   pressure columns, and committing (attack / reinforce / raid / withdraw / fortify) only when a
   personality-weighted pressure crosses a registered threshold. No CPU planner anywhere.

**Acceptance (the Scope Ledger this track closes against, constitution §0.6):** every element of (1)–(4)
marked `implemented` / `proxied` / `deferred` / `parked` with evidence, plus a non-vacuous multi-tick
run on a real adapter where the contested Terran/Pirate border front measurably evolves and at least one
faction commitment fires from a threshold crossing (not a scripted event).

---

## 3. The transpilation pipeline (what gets built)

```
terran_pirate_galaxy.clause                     ← single authored native ClauseScript file
   │   (A) base: 1500-star disc static_galaxy_scenario   (simthing-mapgenerator, seed-pinned)
   │   (B) overlay: scenario-container ClauseScript       (owners, planets, factories, cohorts,
   │                                                        fleets, ships, combat/diplomacy economics,
   │                                                        front authoring, ai_will_do commitments)
   ▼
ClauseThing hydrate  (widened front-end; closed lowerers touched only under §1 amendments)
   │   • neutral-AST base → gridcell lattice hierarchy (galaxy → star → planet → 1×1 surface)
   │   • Owner SimThings as GameSession siblings  +  owner-columns  +  identity overlays
   │   • surface-tier gameplay children: factory buildings, population cohorts
   │   • shipsize/triggered-modifier decoder family (NEW §1.2): fleets, ships, hull capability trees
   │   • RF arenas:  HP/Damage (combat) · influence/distrust (diplomacy) · economy · suppression · disruption
   │   • Movement-Front L1/L2/L3:  Gu-Yang SaturatingFlux borders/chokes · PALMA reach · ai_will_do urgency
   ▼
SimThingScenarioSpec JSON   ← canonical save/load authority; only SimThing-Spec scaffolding remains
   │   + RF capacity amendment (§1.1): galaxy-scale participant/slot/emission caps
   ▼
Studio load → driver compile → SimSession resident GPU tick (indefinite)
   │   accumulate → reduce up → settle → disburse down → threshold crossings → BoundaryRequests
   ▼
Live STEAD simulation:  fronts propagate, borders settle/shift, fleets follow gradients (reparent),
                        combat resolves as HP/Damage, faction commitments fire from thresholds.
```

**Fleet homing — DA ruling (§1A.1 #5, binding; no new tier).** Immobile gameplay children (factories,
cohorts, buildings) home under the **planet 1×1 surface gridcell** (the mandated tier). **A fleet berths
at the 1×1 surface gridcell of a star-system local-grid cell** — a planet's surface when garrisoning, or
an **existing orbital/space local-grid cell's receiver-grid surface** when patrolling (star-system gridcells
already carry 10×10 local grids; orbital bodies are local-grid cells; inert cells already carry receiver
grids — Studio doctrine). **Movement = reparent the fleet to an adjacent system's local-grid cell surface**
down the desirability/threat/reach gradient (core §7.2, "the front is the route"). This honors "fleets are
children of the surface gridcell" verbatim while keeping mobility = reparenting; **no new structure is
minted.**

---

## 4. The PR ladder

Each rung opens with the §0 harness header, self-checks its diff against the six base principles
(constitution §0.5 Rule 2), and lands a `docs/tests/*_results.md` report carrying a Scope Ledger.
Tier gates per constitution §0.5 Rule 7 and ClauseThing_Spec §5/§7. **`cargo test --workspace` is never
run**; each rung names its exact targeted tests.

### Phase 0 — Track opening & the capacity amendment (clears the galaxy-scale gate first)

**Track open (`TP-TRACK-OPEN-0`, 2026-07-01) - DONE / DA-OPENED.** Execution is open; the 0.0.8.4 prerequisite ladder is verified CLOSED in the tree. The Tier-2 capacity amendment `TP-RF-CAPACITY-AMENDMENT-0` is **COMPLETE - DA/Owner-cleared** (2026-07-01, independent re-review of PR #1071 against the merged tree), `TP-SCALE-ENVELOPE-0` / 0R / 0R2 is **COMPLETE - DA-equivalent orchestrator-cleared 2026-07-01**, and `TP-BASE-DISC-GEN-0` is **COMPLETE - DA-equivalent orchestrator-cleared 2026-07-01**. The next active rung is `TP-BASE-EMBED-0` (1.1). Evidence: [`docs/tests/tp_track_open_0_results.md`](tests/tp_track_open_0_results.md).

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 0.0 | `TP-TRACK-OPEN-0` | This document + evidence-index row + harness. Docs only. | **DONE — DA-OPENED** (2026-07-01): doc lands; ledger row honest (impl not started). | — |
| 0.1 | `TP-RF-CAPACITY-AMENDMENT-0` | **COMPLETE — DA/Owner-cleared** (2026-07-01, PR #1071 `4a7b8d028d`; independent DA re-review against the merged tree). §1.1 amendment: all three `GpuArenaDescriptor` caps (`max_participants`, coupling-fanout, orderband-depth) plus GPU slot & emission capacity scale from a checked-`u128` RF capacity budget resolver that hard-errors (`SpecError::ResourceFlowCapacityBudget`) on zero/`SCENARIO_STRUCTURAL_INTEGER_MAX`-overflow surfaces. Budget is resolved, stored, and reserved on the real install/session-attach path (`n_slots` from `budget.gpu_slots`; emission via `ensure_threshold_accumulator`). No new `AccumulatorRole`, no semantic WGSL, no runtime `match kind`, no per-tick allocation; pool growth at boundaries only (constitution §0.4). One concise capacity-budget ledger landed. Evidence: [`docs/tests/tp_rf_capacity_amendment_0_results.md`](tests/tp_rf_capacity_amendment_0_results.md). | DA-verified against the tree: `e10_resource_flow_admission` 19/19 (incl. checked-totals + cap-scaling); galaxy-scale driver test `tp_rf_capacity_budget_installs_250_owned_systems_plus_fleet_load` ran on a real GPU adapter (no skip) — asserts `n_slots >= 2048` and budget-derived caps 704/8/16; `doctrine_scan.sh` PASS 0/0; `gen_digest --check` PASS; GitHub Doctrine Scan run 28537448382 success. | **Tier-2** (closed lowerer + binding caps) |
| 0.2 | `TP-SCALE-ENVELOPE-0` | **DONE — DA-APPROVED (2026-07-02, executive DA deep review)** after 0R2. The generate -> lattice -> RF-budget -> link -> `install_atomic` legs are real and proven (3000 participants / 2 arenas / 7505 slots through the accepted RF capacity budget). Original HOLD history is preserved in the evidence doc: the first terminal exit proof was false-green because a real-adapter `SimSession::open_from_spec` panic in `initial_gpu_sync -> upload_reduction_topology` was caught and swallowed before `assert!(session.mapping.is_none())`. Evidence: [`docs/tests/tp_scale_envelope_0_results.md`](tests/tp_scale_envelope_0_results.md). | 0R repaired the false-green reduction-topology failure; 0R2 repaired the velocity upload scale failure and proved CPU-oracle parity plus real-adapter `mapping.is_none()`. | Tier-1 over 0.1 |
| 0.2r | `TP-SCALE-ENVELOPE-0R` | **DONE — DA-APPROVED (2026-07-02, executive DA deep review)**. Removed catch_unwind false-green path; fixed `rebuild_for_slots` `column_rules` sizing when slot growth coincides with registry expansion. | Scale test no longer swallows panics; `upload_reduction_topology` no longer overruns 24-byte `column_rules`. | Tier-2 (`seal-residue-risk`) |
| 0.2r2 | `TP-SCALE-ENVELOPE-0R2` | **DONE — DA-APPROVED (2026-07-02, executive DA deep review)**. Compact C-7 velocity upload from `n_slots * governed_pairs` materialized ops to one pair op expanded across slots by GPU dispatch; CPU-oracle velocity parity passes for dt=1.0 and fractional dt; preserve RF/install/session proof without Phase 1 content. | Local real-adapter scale test reaches `SimSession::open_from_spec` success and asserts `session.mapping.is_none()`; compact upload remains under the 2 MiB scale target while the old expanded shape would exceed 10 GiB. | Tier-2 (`seal-residue-risk`) |

### Phase 1 - Base galaxy production (mostly reuse; Studio-consistency proof)

**Phase 1 is now unblocked.** `TP-SCALE-ENVELOPE-0` / 0R / 0R2 and `TP-BASE-DISC-GEN-0` are COMPLETE - DA-equivalent orchestrator-cleared 2026-07-01. The next active rung is `TP-BASE-EMBED-0`, the scenario-container embedding rung.

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 1.0 | `TP-BASE-DISC-GEN-0` | **DONE — DA-APPROVED (2026-07-02, executive DA deep review)**. Emits the canonical **disc, 1500-star** base `static_galaxy_scenario` (seed `770421`) with deterministic SimThing-authored Stellaris-style star-name corpus assignment. Seed, generator params, profile id, corpus source, and assignment mode are captured in scenario metadata. Evidence: [`docs/tests/tp_base_disc_gen_0_results.md`](tests/tp_base_disc_gen_0_results.md). | Byte-identical regeneration from recorded seed/params PASS; `map_quality_status = PASS`; names assigned deterministically by system id; Studio Generate path produces the same canonical bytes after TP canonicalization; live Doctrine Scan passed on PR head `29e374d3`. | Tier-1 |
| 1.1 | `TP-BASE-EMBED-0` | **DONE — DA-APPROVED (2026-07-02, executive DA review)**. Scenario-container front end accepts the canonical MapGenerator base as an embedded `static_galaxy_scenario` block parsed through the existing neutral-AST/raw parser path. The embedded base preserves producer provenance, structural grid placements, deterministic system ids under namespace transform, star display names, and `map_quality_status = PASS`; overlay runtime metadata remains separate. Evidence: [`docs/tests/tp_base_embed_0_results.md`](tests/tp_base_embed_0_results.md). | Combined `.clause` parses; embedded base lattice round-trips identical to rung 1.0; base ids are namespaced into overlay location-targets; duplicate namespace ids hard-error with a span; producer provenance remains distinct from overlay runtime ownership; live Doctrine Scan passed on PR head `0c484813`. | Tier-2 (combined-document grammar) |

### Phase 2 — Ownership: owners, planets, factories, cohorts (the scenario-container widening)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 2.0 | `TP-OWNER-SIBLINGS-0` | **DONE — DA-APPROVED (2026-07-02, executive DA targeted review)**. ClauseScript authoring of **Owner SimThings as GameSession siblings** (Terran, Pirate) with stockpile/policy/personality/capability metadata hooks. Lowers to `Scenario → GameSession → {Owner, Owner, GalaxyMap}`. **No owner is a spatial parent.** Evidence: [`docs/tests/tp_owner_siblings_0_results.md`](tests/tp_owner_siblings_0_results.md). | Hydrated tree has Terran/Pirate owners as direct GameSession children; GalaxyMap remains a GameSession sibling; canonical scenario roundtrip preserves owner metadata distinct from spatial parentage; embedded base placements remain unchanged; duplicate owner ids and unsupported owner fields hard-error with spans; live Doctrine Scan passed on PR head `19bbe37a`. | Tier-2 (first owner-as-sibling clause authoring) |
| 2.1 | `TP-OWNERSHIP-COLUMNS-0` | **DONE — DA-APPROVED (2026-07-02, executive DA full review)**. Ownership = owner-column + permanent identity overlay over the canonical embedded TP base. Exactly 200 Terran systems, 50 Pirate systems, and 1250 neutral systems are proven; Terran/Pirate volumes are integer-only Chebyshev selections with deterministic tie-break; owner refs resolve to GameSession sibling owners; systems remain GalaxyMap children; capture-as-column-flip changes only the owner ref. Evidence: [`docs/tests/tp_ownership_columns_0_results.md`](tests/tp_ownership_columns_0_results.md). | Combined embedded base + owner siblings + ownership volumes parses; exactly 200 Terran + 50 Pirate + 1250 neutral systems; Terran/Pirate Chebyshev contiguity and adjacency proven; owner refs resolve to sibling owners; systems remain GalaxyMap children; capture-as-column-flip changes only owner ref. | Tier-1 over 2.0 |
| 2.2 | `TP-PLANET-SURFACE-PAYLOAD-0` | **DONE — DA-APPROVED (2026-07-02, executive DA deep review post-merge; merge-hold breach recorded, accepted on merits, not precedent)**. Planet/surface payload authoring over owned/neutral systems via `planet_surface_payload` blocks. Each owned system: **≥1 planet gridcell → mandated 1×1 surface gridcell → ≥1 factory building + ≥1 cohort**. Light-payload neutrals: planet + surface, no children. Factory/cohort economy authored with **Appendix A modifier chains** (`pop_category_*` factory output, upkeep) through existing CT-2c decoder surfaces. Evidence: [`docs/tests/tp_planet_surface_payload_0_results.md`](tests/tp_planet_surface_payload_0_results.md). | Combined embedded base + owner siblings + ownership columns + planet/surface payload parses; all 250 owned systems have planet + 1×1 surface + factory + cohort; all 1250 neutral systems have planet + 1×1 surface only; surface tier non-vacuous; TP-OWNERSHIP-COLUMNS-0 owner refs/counts unchanged; RF reduce-up path admits owned surface participants; modifier chains decode through existing surfaces. | Tier-1 over 2.1 + 2c decoder |

> **DA graduation log (executive DA, 2026-07-02).** Rungs **0.2/0.2r/0.2r2 `TP-SCALE-ENVELOPE`**, **1.0
> `TP-BASE-DISC-GEN-0`**, **1.1 `TP-BASE-EMBED-0`**, **2.0 `TP-OWNER-SIBLINGS-0`** → **DONE / DA-APPROVED**;
> **2.1 `TP-OWNERSHIP-COLUMNS-0`** → **DONE / DA-APPROVED** (2026-07-02, executive DA full review; PR #1078 merged).
> Review depth routed per `ci_screening_surface.md` §5 from each rung's declared risk class — the triage log
> carries **zero** TP INSPECTs, so depth came from structural risk, never from the log alone.
> **Deep (0.2r/0.2r2 — `seal-residue-risk` kernel/WGSL door logic; Track B does not exist, so the DA ran the
> falsification battery itself on the owner's adapter):** verified in the tree — `catch_unwind` false-green
> path deleted from `tp_scale_envelope.rs`; `rebuild_for_slots` reallocates `column_rules` on slot-growth;
> compact C-7 velocity real (`source_count = n_slots` in `compact_pair_to_gpu_op`, WGSL
> `EXECUTE_MODE_COMPACT_VELOCITY` invocation expansion `packed_op_idx = op_idx % op_count`, chunked encode
> under a checked-u32 bound) — and independently re-run: velocity CPU-oracle parity 2/2 (dt=1.0 + fractional),
> compact-upload pin PASS, real-adapter terminal `mapping.is_none()` PASS (8.0s).
> **Deep (1.0 — data-deliverable):** fixture SHA-256 `aab8b0d2…` + 889808 bytes match the recorded values in
> the tree; byte-identical regeneration / Studio-Generate parity / seeded-name determinism re-run 3/3 PASS.
> **Gate-wiring (1.1):** duplicate-id span guard + round-trip identity + provenance separation re-run 5/5 PASS.
> **Targeted (2.0):** owners-as-GameSession-children / GalaxyMap-sibling / hard-error / roundtrip suite re-run
> 7/7 PASS. **Full (2.1):** 9/9 re-run PASS; `chebyshev_distance` verified integer-only
> (`u32::abs_diff().max()`) with deterministic `(distance,row,col,id)` tie-break and a spanned selection-failure
> hard-error — no Euclidean/float authority; capture-as-column-flip verified; live Doctrine Scan green on
> PR #1078 (run 28563325202).
> **One binding follow-on note (not a HOLD):** `HydratedScenarioPack` now carries the canonical
> `authority_root` **beside** the legacy scenario-container `root`. Both are projections of the same hydration
> sources today, but a dual representation is a divergence seam: a named rung — at latest
> `TP-FULL-TRANSPILE-0` — must converge consumers onto `authority_root` or record a Deviation deriving/retiring
> the legacy root. **Orchestrator practice note:** the per-rung `Graduation routing` blocks (risk class +
> falsification check) are what let this review run at routed depth instead of re-deriving findings from
> nothing — keep filing them exactly as done.

> **DA graduation log (executive DA, 2026-07-02, second entry — post-merge deep review of 2.2).**
> **2.2 `TP-PLANET-SURFACE-PAYLOAD-0` → DONE / DA-APPROVED.** Deep review per its declared risk class
> (surface-tier non-vacuity — the first §0.6 tier-collapse-exposed rung): the 12-test suite + the
> ownership-columns 9 re-run green by the DA; the surface tier is proven non-vacuous through the
> **pre-existing** `simthing-spec` tier-evaluation surfaces (`evaluate_planet_child_locations` /
> `_rf_admission` / `_rf_reduce_up` — the PR #851 lineage, reuse not reinvention); factory =
> `SimThingKind::Custom("Infrastructure")` (no new kind variant); neutral-payload factory/cohort
> hard-errors spanned; 200/50/1250 ownership counts preserved; live Doctrine Scan green (run 28564092572).
> **Process breach recorded:** PR #1079 merged while its own results doc read *"PROBATION —
> orchestrator/DA review required before merge"* — a §0.9.5 merge-hold breach. The evidence doc was
> truthful about its state and the substance verifies sound, so the breach is **accepted on its merits
> per the #1042 precedent — and, like #1042, it is not precedent**: no PROBATION/authority/gate rung
> merges before DA/Owner clearance. Orchestration carries this forward.

### Phase 3 — Fleets, ships & the shipsize modifier family (the hairy-modifier stress core)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 3.0 | `TP-SHIPSIZE-DECODER-0` | **§1.2 amendment.** Extend the CT-2c longest-match modifier decoder with the **shipsize/`ship_*` family** (`shipsize_corvette_hull_add`, `ship_weapon_damage_mult`, `ship_fire_rate_mult`, `ships_upkeep_mult`, `country_naval_cap_add`, …) and **`triggered_modifier`/`complex_trigger_modifier`** gated forms. All lower to overlays (`Add` leaf-only / `Multiply` subtree-sweep) + `EvalEML` `ExactDeterministic` ≤32-node trees. **Ship classes lower to capability-tree `Custom(...)` SimThings.** | **DONE — DA-APPROVED (2026-07-04, deep review + DA fix + lab waiver)** — DA-verified: EML ≤32-node bound proven (`MAX_SHIP_EML_NODES`) with CPU-oracle inline parity; ambiguity rejection (`duplicate_registered_class_is_ambiguous`); lowers to existing substrate (overlays + gated-rate EML, **no new AccumulatorRole / opcode / GPU path**); semantic-free below spec (doctrine-scan PASS); scope clean (clausething src + test + `.clause` only). **DA fix applied:** the two new tests were mis-tagged `birth_track=TP-SHIPSIZE-DECODER-0` (rung id) and a rung-as-track was registered — a §0A violation; corrected to `birth_track=0.0.8.5-terran-pirate` (the scenario envelope) and the rung-track removed, so all TP tests expire together at scenario completion, never per-rung. **Lab waiver (DA):** the `#[ignore]` `CLAUSER_LAB_DIR` `modifiers.log` round-trip is the *horizon* generalization check (real-Stellaris fluency), not this scenario's acceptance; the committed authored-form table proof + CPU-oracle parity is sufficient. Per §0A this decoder is envelope-scoped/expirable — full-Stellaris generalization is a post-completion admission concern, not this rung's gate. Evidence: `docs/tests/tp_shipsize_decoder_0_results.md`. Next: `TP-FLEETS-SHIPS-0`. | **Tier-2** (new grammar family + the frontier silent-fidelity surface) |
| 3.1 | `TP-FLEETS-SHIPS-0` | Author **fleets as mobile star-system-grid occupants** and **ships as cohort-style children** with HP/Damage/upkeep columns. Place 10 Terran fleets (200 ships) by the 60-40 rule and 10 Pirate fleets (400 ships) by the 80-20 rule, homed at the relevant border/interior/raid-posture systems. | **DONE — DA-APPROVED (2026-07-04, deep review after 0R)** — DA-verified against the tree: Fleet/Cohort `planet_child_rf` special-case **removed** (no net-new `SimThingKind::Fleet/Cohort` reads in `simthing-spec` vs master); fleets/ships are ordinary SimThings with owner/resource/upkeep RF metadata; fleet-nested RF reduce-up **parked §0.6 Deviation** (surfaced candidate: generalize child-RF admission to arbitrary depth, semantic-free/kind-agnostic — NOT a Fleet/Cohort traversal); counts/60-40/80-20/parentage/owner-refs/HP-Damage-upkeep proven (`tp_fleets_ships_0` 2/2); `birth_track=0.0.8.5-terran-pirate`; scope clean; gates green. **DA note:** drift (less-conformant kind-gate vs generic recursion) + §0A/§A boundary breach, not a broken type seal (spec kind-reads legit per AS-3); the scanner gap that let it reach review is closed by companion `CI-SCAN-SPEC-KIND-COVERAGE-0`. Evidence: [`tp_fleets_ships_0_results.md`](tests/tp_fleets_ships_0_results.md). Next: Phase 4 `TP-COMBAT-ARENA-0`. | Tier-1 over 3.0 |

### Phase 4 — Combat as HP/Damage economics

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 4.0 | `TP-COMBAT-ARENA-0` | Author the **HP/Damage RF arena**: damage = `SubtractFromSource` transfer between opposing ships co-located in a system arena; HP recovery = `governed_by`; **owner combat bonuses (`ship_weapon_damage_mult`, etc.) disburse down as overlays on the Damage columns**; ship death = zero-HP `Threshold` → `EmitEvent` → `BoundaryRequest` removal (slot recycles, constitution §0.4). | **DA-GRADUATED / merged [#1145](https://github.com/khorum08/SimThing/pull/1145) @ `a54695ec`** — one-time Homing Boundary exception (combat hydrator in `simthing-clausething`); GPU==CPU bit-exact (owner-local proof `tested_code_sha=72dc4355`, [`tp_combat_arena_0_results.md`](tests/tp_combat_arena_0_results.md)); zero-HP → `BoundaryRequest::Remove`; no combat subsystem. **Next blocked:** Phase 5 until DA clearance (cleared). | Tier-1 over the accepted HP/Damage doctrine |

### Phase 5 — Diplomacy as influence/distrust economics

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 5.0 | `TP-DIPLOMACY-FLOW-0` | Author **influence/trust/distrust as RF lanes** (core §2.1, §5.3): each owner emits influence/distrust into touched assets; reduces up to the owner; a stance/hostility change is a **registered threshold crossing on the reduced trust/distrust column** (`AggregateAlertRegistration`-class) → `EmitEvent`. Terran↔Pirate baseline hostility seeded as an authored distrust intensity. No diplomacy subsystem. | **DA-GRADUATED / merged [#1150](https://github.com/khorum08/SimThing/pull/1150) @ `9aa66c39`** — workshop-homed Mechanism B; distrust threshold → hostility `EmitEvent`; GPU==CPU oracle (4/4); [`tp_diplomacy_flow_0_results.md`](tests/tp_diplomacy_flow_0_results.md). **Next:** Phase 6. | Tier-1 over RF substrate |

### Phase 6 — STEAD fronts & fleet movement (the Movement-Front automaton, live)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 6.0 | `TP-FRONTS-AUTHORING-0` | Author the **Movement-Front L1/L2/L3** over the galaxy lattice: **threat front** (pirate raid pressure), **suppression front** (terran patrol/fleet presence), **disruption front** (raids feeding it). L1 = Gu-Yang `SaturatingFlux` `RegionFieldSpec` seeded by RF arena pressure (`ArenaPressureBindingSpec`); L2 = reduce up; L3 = `ai_will_do` urgency. Bounded horizon (P1), one shared stencil (P2), attractor/threshold (P3). | **DA-GRADUATED / merged [#1151](https://github.com/khorum08/SimThing/pull/1151) @ `9f56794a`** — workshop-homed Mechanism B; on-device arena scatter GPU==CPU; settling contour + L3 urgency (5/5); [`tp_fronts_authoring_0_results.md`](tests/tp_fronts_authoring_0_results.md). **Next:** Phase 6.1. | Tier-1 over landed Movement-Front (atlas/bounded-theater scheduling for the vast lattice) |
| 6.1 | `TP-PALMA-REACH-0` | **PALMA reach/impedance** over the fronts: impedance `W` composed from choke/threat fields → resident `D`. The reach field is what fleet movement gradients consume. No route object, no predecessor. | **DA-GRADUATED / merged [#1152](https://github.com/khorum08/SimThing/pull/1152) @ `335f55c0`** — workshop-homed W compose + resident D + gradient probe; GPU==CPU oracle (`tested_code_sha=905fb35a`); [`tp_palma_reach_0_results.md`](tests/tp_palma_reach_0_results.md). **Next:** Phase 6.2. | Tier-1 over seated PALMA |
| 6.2 | `TP-FLEET-MOVEMENT-0` | **Fleet movement = gradient-following reparenting.** A fleet steers proportionally down/up the local desirability/threat/reach gradient between star-system gridcells; movement is column updates + arena re-enrollment, **not** a route solver. Velocity uses an explicit previous-value column + copy band (EML has no previous-buffer read). | **DA-GRADUATED / merged [#1154](https://github.com/khorum08/SimThing/pull/1154) @ `7d44037e`** — 7×7 / horizon-3 theater; horizon truncation engages; ≥3 cells/≥3 ticks D-gradient reparent; arena re-enrollment; GPU==CPU on larger theater (`tested_code_sha=5b03bfb1`); [`tp_fleet_movement_0_results.md`](tests/tp_fleet_movement_0_results.md). **Next:** Phase 7. | Tier-2 (first live fleet movement; confirms the §3 fleet-homing decision) |

### Phase 7 — STEAD decisions: ai_will_do commitments (FIELD_POLICY, the capstone of "decisions by STEAD")

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 7.0 | `TP-COMMITMENTS-0` | Author each faction's **personality `ai_will_do`/`ai_weight` EML weight profiles** over its reduced L3 pressure columns (aggression/risk-tolerance sub-fields × pressure → urgency), firing commitments (**attack / reinforce / raid / withdraw / fortify**) as `Threshold` + `EmitEvent` → `BoundaryRequest` crossings. Pirate 80-20 and Terran 60-40 postures are **initial placement**, not ongoing scripted decisions; every subsequent decision is a threshold crossing. **No CPU planner, ever** (drift detector §9.4). | **ORCHESTRATOR-GRADUATED / merged [#1155](https://github.com/khorum08/SimThing/pull/1155) @ `6547b90c`** — §5A orchestrator merge-clear under #1153 precedented-class ruling; `tested_code_sha=ca19e956571d451d97d20c37f6cf0627f40b0fa9`; citable proof [`tp_commitments_0_results.md`](tests/tp_commitments_0_results.md); Terran reinforce + Pirate raid commitments fire from L3 pressure crossings; `ai_will_do`/`ai_weight` urgency path proven; owner-local GPU proof PASS (5/5); `BoundaryRequest` at boundary only; no CPU planner / no CPU commitment emission; no engine edits / no widening / no new opcode-WGSL-AccumulatorRole. **Next:** Phase 8 `TP-FULL-TRANSPILE-0` unblocked. | Tier-1 over the accepted FIELD_POLICY path |

**Design-ladder anti-drift (binding):** update each rung's exit-proof/status cell before declaring the rung ready — results docs and PR bodies do not substitute for the design ladder. If a rung is self-cleared or DA-cleared and the design row cannot contain the final merge SHA before merge, the orchestrator must land an immediate docs-only status-stamp PR before opening the next rung (remedial example: `TP-COMMITMENTS-0-DESIGN-STATUS-STAMP` after #1155).

### Phase 8 — Full transpile, live run, Studio ingest readiness

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| 8.0 | `TP-FULL-TRANSPILE-0` | Ingest the **complete single `.clause`** end-to-end → `SimThingScenarioSpec` JSON. **Semantic-free scan targets runtime/GPU/`simthing-sim` artifacts only** (§1A.1 #7) — authoring ids, provenance, display names, and star names legitimately persist on `Location` SimThings. Canonical save/load roundtrip stable digest. | **DA-GRADUATED / merged [#1215](https://github.com/khorum08/SimThing/pull/1215) @ `f8eb3815`** (head `3a59b9c5af`) — single native `terran_pirate_galaxy.clause` parses/hydrates; authority_root → canonical ScenarioSpec; byte-stable JSON roundtrip; GameSession→{Owner×2,GalaxyMap}; 1500 systems; 200/50 ownership; fleets/ships/combat/MF/PALMA/commitment hydrate. **DA acceptance (Option A):** STEAD lattice/feedstock proven on embedded base + `grid_metadata`; full placement/link re-bind onto authority tree nodes is **TP-LIVE-RUN-0 residue**, not a transpile blocker. Evidence: [`tp_full_transpile_0_results.md`](tests/tp_full_transpile_0_results.md). **Next:** Phase 8.1. | Tier-1 |
| 8.1 | `TP-LIVE-RUN-0` | **Indefinite-tick live run** over **one deterministically-selected bounded theater = the contested Terran/Pirate border sub-volume** (§1A.1 #4), in Studio or headless driver with Studio load proven. Theater field state is **runtime cache, not ScenarioSpec writeback** unless a structural commitment fires via `BoundaryRequest`. **No atlas scheduler is built here**; full-galaxy tiling is a recorded Deviation. | **DA-GRADUATED / merged [#1217](https://github.com/khorum08/SimThing/pull/1217) @ `f42b4d28d7`** (head `3a3eba9b7f`, 0R2) — full `terran_pirate_galaxy.clause` → 7×7 rebind theater; multi-tick real-adapter front shift; STEAD L3 reinforce → hard `BoundaryRequest::AttachOverlay`; RF combat: cross-opponent weapon→hull-deficit/DTK band fill + non-vacuous workshop emission-band `destroyed_ships`/`num_ships` settlement; GPU==CPU transfer oracle; no CPU planner. **DA acceptance (Option A):** workshop-homed emission settlement accepted for scenario envelope; generic GPU `destroyed_ships` emission, non-vacuous combat overlay effect, and casualty→next `ArenaPressureBinding` coupling are **closeout Deviation / substrate opportunities**, not live-run blockers. Evidence: [`tp_live_run_0_results.md`](tests/tp_live_run_0_results.md). **Next:** Phase 8.2 Studio ingest readiness. | Tier-1 |
| 8.2 | `TP-STUDIO-INGEST-READINESS-0` | Evaluate the approved `terran_pirate_galaxy.clause` file and current transpile artifacts for SimThing Studio ingest/transpile/load/save readiness. Produce a report identifying every missing surface, format bridge, UI/driver boundary, save/load pathway, and test required for Studio to ingest the ClauseScript file, transpile it, load it, save it, and reload it without semantic drift. **No implementation.** | **DA-GRADUATED / evaluation complete** — gap ledger in [`tp_studio_ingest_readiness_0_results.md`](tests/tp_studio_ingest_readiness_0_results.md): parse/hydrate/project available but not Studio-wired; JSON ScenarioSpec path proven in mapeditor; placement rebind / picker / session STEAD gaps named. **Next:** workshop candidate then API admission. | Tier-1 (docs/evaluation only) |
| 8.3 | `TP-STUDIO-CLAUSE-INGEST-0` | Workshop-homed TP candidate: `.clause` → parse/hydrate → ScenarioSpec candidate → production authority serde save/reload. **Not** a production mapeditor API. | **DA-GRADUATED / merged [#1222](https://github.com/khorum08/SimThing/pull/1222) @ `bcbc2f4389`** (head `ee0480c438`, 0R) — workshop `tp_studio_clause_ingest`; zero mapeditor production surface; FULL-TRANSPILE projection shape; canonical digest parity; malformed status path. Evidence: [`tp_studio_clause_ingest_0_results.md`](tests/tp_studio_clause_ingest_0_results.md). **Next:** `TP-STUDIO-CLAUSE-API-ADMISSION-0` (explicit DA/Owner) before any picker. | Tier-1 (workshop candidate) |
| 8.4 | `TP-STUDIO-CLAUSE-API-ADMISSION-0` | **Owner/DA-gated.** Admit a generic, non-TP-default `simthing-mapeditor` ClauseScript ingest API only if reusable and semantic-free (elevation of workshop candidate or reimplementation). Default deny → keep workshop. | **OPEN — next production pointer (admission decision).** No UI picker until this graduates. | Tier-2 (substrate/API admission) |

### Workplan Closure Track — Owner-triggered only

This section is **not** part of the production Phase 8 ladder. It is run only when the Owner/User explicitly declares the 0.0.8.5 Terran-Pirate workplan complete and requests closure.

**Closeout is forbidden as an automatic next rung.** It becomes active only after explicit Owner/User instruction that the workplan is complete. Until then, production continues through the Phase 8 table (and any Owner-authorized Phase 8 extensions). Orientation / next-rung pointers must never treat closeout as the standing production pointer.

| Closure Rung | ID | Scope | Exit proof |
|---|---|---|---|
| C.0 | `TP-DA-CLOSEOUT-0` | Workplan closeout only: Scope Ledger over every §2 acceptance element; Deviation Records for anything proxied/deferred (e.g. galaxy-scale dense Movement-Front execution deferring to atlas scheduling is a bounded-theater Deviation, not a failure; generic GPU `destroyed_ships` emission opportunity; casualty→next `ArenaPressureBinding` coupling; non-vacuous combat overlay; workshop emission-band residue); DA review. **Per §0A: also emit the corpus-abstraction *candidate list*** — each TP-surfaced concept/code/test/heuristic worth admitting to the SimThing corpus, with its proposed admission rung — for the **separate post-completion admission review**. Closeout *nominates*; it does **not** admit. TP-born tests remain `birth_track=0.0.8.5-terran-pirate` (expirable) until each is individually promoted or leased. Closure through `track_closeout.sh` when Owner-triggered. | **OWNER-TRIGGERED / NOT ACTIVE.** Exit proof: closeout manifest + closeout report + `track_closeout.sh --apply` result + DA/Owner sign-off. |

---

## 5. Honest deferrals & Deviations (declared up front, constitution §0.6)

- **Dense galaxy-scale Movement-Front execution defers to atlas/bounded-theater scheduling.** A vast
  layout is admitted even where a dense execution profile defers — *layout admitted, execution requires
  atlas/tile scheduling*, never "the map is too large" (STEAD invariant 5). The live border run (8.1)
  proves the contested sub-volume as one or more bounded theaters; full-galaxy simultaneous dense fronts
  are an atlas Deviation, recorded, not silently flattened.
- **`from`/`root` dynamic scope chains** (ClauseThing_Spec §8 SCOPE-MEMO gate) are bounded to the CPU
  boundary or rejected; this scenario is authored to need only same-owner/same-scope triggers so no rung
  blocks on the event-payload substrate extension. Any construct that needs richer context is a recorded
  Deviation, not a silent CPU planner.
- **Balance / economic depth** beyond what drives the contested front is out of scope; this track proves
  the *mechanism and the ingestion*, not faction balance (ClauseThing_Spec §8 closing note).
- **Hull-design capability-tree depth** lands as the *shape* (ship class = `Custom(...)` SimThing); deep
  multi-tier tech trees are a follow-on consumer, not this track.

---

## 6. Appendix A — the hairy-modifier stress catalogue (Owner-mandated, §1)

The authored `.clause` must exercise these forms liberally (the Owner's explicit stress-test instruction).
Each lowers through the extended decoder to existing substrate only (overlays + `EvalEML` ≤32-node trees +
RF arenas), CPU-oracle bit-exact. Listing is the decoder's priority queue, not decoration.

**"Liberally" = coverage, not volume (binding bound on the mandate).** The stress-test is satisfied by
forcing the decoder's **real edges** — at least one instance of each admitted form below — **not** by a
giant artificial corpus whose size is itself the claimed achievement (that is its own hygiene, rejected by
D3/D4). The required coverage set: longest-match category/resource parsing (e.g.
`pop_category_bio_trophy_unity_upkeep_add`); **ambiguous-key rejection**; `_add` leaf-only **vs** `_mult`
subtree sweep; `cost`-context restriction; shipsize/`ship_*` combat & upkeep overlays; `triggered_modifier`
activation/dissolution; `complex_trigger_modifier` bool→number over a **column-backed** trigger;
`value:` formula → bounded `EvalEML`; `ai_will_do`/`ai_weight` urgency over a resolved pressure column.
Every form lowers through existing substrate or is a spanned hard error — there is no third outcome.

- **Underscore modifier-key chains** `(category)_(resource)_(produces|upkeep|cost)_(add|mult)`, longest-match
  against registered sets: e.g. `pop_category_worker_minerals_produces_mult = 0.10`,
  `settlement_food_produces_add = 6`, `pop_factory_minerals_produces_mult`,
  `polity_energy_cost_add` (discrete-economy context only). `_add` lowers leaf-only; `_mult` sweeps the
  category subtree (the inheritance asymmetry, CT-2c §4).
- **Shipsize / `ship_*` family** (§1.2, newly admitted): `shipsize_corvette_hull_add`,
  `ship_weapon_damage_mult`, `ship_fire_rate_mult`, `ships_upkeep_mult`, `country_naval_cap_add` — combat
  economics + naval capacity as overlays on HP/Damage/upkeep columns.
- **`triggered_modifier { potential = { ... } <keys> }`** → `Suspended`→`Permanent`/`Transient` overlay +
  threshold/dissolve: e.g. `triggered_modifier { potential = { is_at_war = yes } ship_weapon_damage_mult = 0.15 }`.
- **`complex_trigger_modifier` (bool→number)** → `EvalEML` `SELECT`/threshold-count — admitted only where
  the underlying trigger reads a column (otherwise hard-error, ClauseThing_Spec §8).
- **`value:` scripted values** (base + math + modifier) → `EvalEML` formula trees, `ExactDeterministic`,
  ≤32 nodes, `@var` constant-folded.
- **`ai_will_do` / `ai_weight` blocks** with nested `modifier { factor … <trigger> }`: e.g.
  `ai_will_do = { weight = { base = 1 modifier = { factor = 5 is_at_war = yes } modifier = { factor = 3 has_border_threat = yes } } }`
  → urgency EML over a pressure column → `Threshold` crossing = commitment (Phase 7).
- **Diplomacy keys** as influence/distrust economics: `trust_growth_mult`, `opinion_add`, distrust `_mult`
  on the influence lane (Phase 5).

Any unknown/ambiguous form is a **spanned hard error with a suggested path** — never a silent guess
(CT-2c §8). The diagnostic stream is the backlog's priority queue toward full Stellaris fluency.

---

## 7. References

- Permanent paradigm: [`simthing_core_design.md`](simthing_core_design.md) (§2 the one tree, §5 RF arenas, §6 overlays, §7 Movement-Front automaton, §8 decisions, §9 drift detectors).
- Active constitution: [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 (anti-flattening §0.6, exact-magnitude §0.7, STEAD §0.8), §A (closed-lowering gates + the named RF capacity amendment).
- ClauseScript↔SimThing isomorphism: [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4, §5 (tier ladder), §7 (guardrails), §8 (hard problems); decoder + movables: [`clausething/ct_2c_economic_category_memo.md`](clausething/ct_2c_economic_category_memo.md).
- MapGen producer + grammar: [`clausething/MapGeneratorCLI.md`](clausething/MapGeneratorCLI.md), [`clausething/MapGenThing.md`](clausething/MapGenThing.md); STEAD contract: [`stead_spatial_contract.md`](stead_spatial_contract.md).
- Studio authority + prior Terran-Pirate skeleton: [`design_0_0_8_3_studio_production.md`](design_0_0_8_3_studio_production.md), [`tests/terran_pirate_scenario_skeleton_0_results.md`](tests/terran_pirate_scenario_skeleton_0_results.md).
- Movement-Front operators: Gu & Yang (arXiv:2509.20797, `SaturatingFlux`), PALMA (arXiv:2601.17028), Wei/STEAD (arXiv:2602.01651), EML (arXiv:2603.21852).
