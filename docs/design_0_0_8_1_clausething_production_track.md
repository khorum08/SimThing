# SimThing 0.0.8.1 — ClauseThing Production Track (`CT-`)

> **Status: OPEN (2026-06-10). This document is the product-authorization record and the live
> ledger for the ClauseThing front-end (formerly "L3, parked").** Consumer:
> **the Stellaris/Clausewitz-engine grand-strategy audience (players and modders)** — recorded in
> the horizon charter's `CLAUSETHING-IMPORTER-0` product note and `simthing_lewm_corpora_case.md` §4.
> Sequencing ruling: **parser-first** (§2). Crate: **`simthing-clausething`** (§3).
> Strategy/correspondence reference: [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md).
> Governance: [`design_0_0_8_1.md`](design_0_0_8_1.md) §5; paradigm: `simthing_core_design.md`.
> One rung in flight at a time; every rung = one PR + one test report + one status row (§11).

---

## 1. Mission & authorization

ClauseScript becomes SimThing's **native modder-facing authoring language**, co-equal with RON:
the `simthing-clausething` crate parses ClauseScript, resolves scopes and macros, and emits the
same `simthing-spec` authoring structs RON produces — serialized through the crate's own `ron.rs`
so the transpilation is literally *ClauseScript → RON*. The existing admission/compile/install
path is unchanged; `simthing-sim` never learns ClauseScript exists. No GPU primitive, no WGSL, no
`AccumulatorRole`, no default change.

This track is the product authorization the 0.0.8.0 ledger has been parked against. Opening it
consumes the accepted CLAUSE-SPEC (L0/L1/L2) admission substrate as its named consumer.

## 2. The sequencing determination (binding)

**Ruling: build the parser first; widen `simthing-spec` only as the parser pulls it.** Adjudicated
2026-06-10 from full reads of the ClauseScript textbook, the spec, and the `simthing-spec` source.

1. **`simthing-spec` needs widening, not rebuilding.** The crate's architecture is the correct
   transpiler target (symbolic `PropertyKey`, serde/RON-native authoring structs, admission
   firewall, `#[non_exhaustive]` expression enums). The gaps are width, and they are named in the
   standing backlog (§6) — led by `ScopeRef` (two variants, one of them the authoring-hostile
   `Slot(u32)`) and `EffectSpec` (three variants vs `effects.log`'s hundreds).
2. **Stages 1–4 of the pipeline touch zero spec surface.** Lex → tape → macro/scope expansion →
   lossless raw model are spec-independent; no later spec evolution can invalidate them. CT-0 is
   fully de-risked work.
3. **Speculative spec enrichment is the named anti-pattern.** The textbook's own verdict — the
   language is "recklessly flexible," "no parser can ever be proven complete," only "continuous
   exposure to real content" works — applies equally to the lowering vocabulary. The spec gaps
   cannot even be *specified correctly* without the corpus-frequency evidence only the parser
   produces (which of ~40 scopes are load-bearing, what context actually flows through events).
4. **"Flawless" attaches to the raw model.** Flawless transpilation = **lossless Stage-4 raw
   model** (ordered duplication, quoting, headers, operator choice, unexpanded scope chains all
   preserved) so no future spec widening ever forces a re-parse — plus the **bit-identity
   instrument**: a ClauseScript-authored entity must produce RON (and an installed tree)
   bit-identical to its hand-authored RON equivalent (CT-1a exit).
5. **One design-first exception:** the symbolic scope model (the `ScopeRef` successor + `EmitEvent`
   payload context) is a genuine Tier-2 substrate question. It is gated as **SCOPE-MEMO** (§5),
   written *after* CT-1a from corpus evidence, *before* any rung leans on cross-scope references.
   T1 rungs deliberately need only same-scope triggers (`ScopeRef::Current` suffices).

## 3. Architecture & crate

- **Crate: `crates/simthing-clausething`** (the spec's option A, renamed to the project's name).
  Depends on the vendored jomini text path (§4) and `simthing-spec`'s public authoring structs.
  Nothing else depends on it; `simthing-spec` remains the single admission/firewall owner.
- **Pipeline:** the five stages in `clausething/ClauseThing_Spec.md` §3. Stages 1–2 vendored
  jomini; Stage 3 (scope/macro expansion, textbook §3.7 ordering) and Stage 4 (lossless raw model)
  are this crate's core; Stage 5 (hydration) emits `simthing-spec` structs and is where all
  tiering lives.
- **Output contract:** emitted structs serialize through `simthing-spec::ron` for golden tests and
  for designers — the transpiler's artifact is inspectable, diffable RON.
- **Duplication policy** (adopted, documented): scalar keys last-wins; list-registry keys
  (`produces`, `modifier`, `prerequisites`, …) collect in source order; explicit
  `@override`/`@append` author opt-in; cross-unit mod load-order merging out of scope for v1.

## 4. Jomini internalization & license accounting

**Decision: vendor the text path; exclude the rest.** Vendored into
`crates/simthing-clausething/src/jomini/` (module, not a separate crate): `TextTape` lexer, tape
parser, text reader/deserializer, scalar handling, Windows-1252/UTF-8 decoding, and the text
writer (round-trip/canonicalization). **Excluded:** binary-format modules, save-game envelope
handling, melting — permanently out of scope (`ClauseThing_Spec.md` §10), and trimming them keeps
the vendored surface auditable.

**Rationale for vendoring over a crates.io dependency:** (a) ClauseThing needs only the text path —
a trimmed vendor is a smaller audited surface than the full crate; (b) the tokenizer may need
SimThing-dialect divergence (e.g. `@override` sugar, diagnostic spans) that upstream would not
take; (c) supply-chain determinism for a repo whose runtime contract is bit-exactness.

**License accounting (MIT — verified against `Clauser/jomini/LICENSE.txt`):**
1. `crates/simthing-clausething/src/jomini/LICENSE` — the verbatim MIT text with the upstream
   copyright line, retained per the license's own condition.
2. A header comment on every vendored file: source repo (`github.com/rakaly/jomini`), the vendored
   commit/version, the MIT notice pointer, and a `MODIFIED:` line on any file we diverge.
3. `THIRD_PARTY_LICENSES.md` at repo root: one entry per vendored dependency (jomini is the
   first), with name, origin, version/commit, license, and vendored path.
4. Jomini's own test fixtures may be vendored with the source (same MIT terms). **Paradox game
   content may not** — see §7.

## 5. The `CT-` ladder

One rung in flight at a time. First slice of each tier is **Tier-2**; subsequent fidelity rungs
are Tier-1 fast-lane (generic, opt-in, parity-backed, reversible).

**Agent gating:** rungs marked **[FRONTIER]** are implemented only by frontier-tier agents —
**Claude Opus (max effort) or Claude Fable 5 (high effort)** — because they carry the
conditional-compilation semantics (expansion ordering, threshold/overlay lifecycle, gated effect
groups, `SELECT` formula lowering) where silent fidelity loss is likeliest and bit-identity is the
only detector. Unmarked mechanical rungs may go to Codex/Cursor-class agents. A frontier flag
never downgrades.

| Rung | Gate | Scope | Exit criteria |
|---|---|---|---|
| **CT-0a** | T2 (track entry) | Crate skeleton + vendored jomini text path + license accounting (§4 items 1–3) | Workspace builds; vendored tests green; `THIRD_PARTY_LICENSES.md` present |
| **CT-0b** | T2 | **Lossless raw model**: tape → typed raw structs preserving ordered duplication, quoting, headers, operators, mixed containers (textbook §2.7 weeds) | Round-trip corpus → JSON golden files → re-emit; byte-faithful where jomini's writer is; §2.7 weed suite green |
| **CT-0c** | T1 **[FRONTIER]** | Macro/expansion passes in textbook §3.7 order: `@vars` → `inline_script` + `[[param]]`/`$PARAM$` → `@[ ]` inline math; `value:` left symbolic | Worked plague example (textbook §3.8) expands to golden output; expansion-order pitfall tests |
| **CT-0d** | T1 | Scope-chain extraction + validation against `scopes.log` (lab fixture, §7); **corpus frequency report** (scope usage histogram — SCOPE-MEMO's evidence) | Malformed chains rejected with spanned diagnostics; frequency report artifact produced |
| **CT-1a** | T2 (entity-hydration contract) | Literal `modifier` blocks + flat properties on one template → overlays through the existing install path | **Bit-identity**: ClauseScript-authored entity ≡ hand-authored RON (RON-diff clean; installed tree bit-exact; CPU-oracle parity on overlay application) |
| **SCOPE-MEMO** | T2 design gate (design authority — Opus) | Symbolic scope model design memo from CT-0d evidence: `ScopeRef` successor, `EmitEvent` payload context, transitive forms (`fromfrom`, `prev` stacks, dot-paths), `event_target:` references, rejection classes. **No code until accepted.** | Memo accepted by design authority; widening tickets cut |
| **CT-1b** | T1 (measurement) **[FRONTIER]** | Recalc stress test: large `triggered_modifier` corpus → `Suspended`/`Threshold`/column counts + tick cost vs RON baseline | Measured report; "every-tick is a net simplification" confirmed or the assumption retired |
| **CT-1c** | T1 **[FRONTIER]** | One capability tree (small tradition set) → `capability_tree_v1` pattern: prereq DAG → threshold ordering, payload activation | First "designer writes Clausewitz, SimThing runs it" proof; parity green |
| **CT-2a** | T1 | Literal `produces`/`upkeep` → `IntrinsicFlow` registrations (opt-in, default-off) | Micro-economy fixture green |
| **CT-2c** | T1 **[FRONTIER]** | `value:` amounts + `economic_category` inheritance → reduction OrderBands; `category_map` defaults + hard-error diagnostics; generated-key grammar + inheritance asymmetry per §6 | The Daily Economy Fixture, authored in ClauseScript, matches the RON original |
| **CT-3b + CT-4a** | T1 over accepted substrate **[FRONTIER]** | **Headline vertical:** ClauseScript-authored `RegionFieldSpec` + `ai_will_do` → the accepted field → reduction → `field_urgency` EML → threshold commitment path, default-off | The entire accepted Phase-M vertical, authored in Clausewitz, `simthing-sim` map-free and ClauseScript-blind |

**Stays Tier-2 / not opened by this track:** 2d coupling edges, 3c perception fields, 3d atlas,
4c velocity columns, cross-unit mod merging, and every item in §6 until a rung names it.

## 6. Standing spec-widening backlog (consumer-pulled — opens only when a rung hits it)

Recorded so gaps are tickets, not surprises. Each is a `simthing-spec` change with its own gate:

- **`ScopeRef` successor** (T2, via SCOPE-MEMO): symbolic scope algebra replacing
  `{Current, Slot(u32)}`; `Slot(u32)` is authoring-hostile (runtime index at authoring time) and
  is retired from authored surfaces when the successor lands.
- **`EffectSpec` widening** (T1 per batch, architecturally-mapped only): from three variants
  toward the mapped subset of `effects.log` (`add_resource` → flow delta, `add_modifier` → overlay
  attach, `set_variable` → Named-column write, …). Unknown effect = hard admission error + spanned
  diagnostic + suggested mapping; the diagnostic stream is the backlog's priority queue.
- **`TriggerSpec` relational forms** (T1/T2 by case): count/any/every-style predicates over
  enrollment selections; flag triggers.
- **Iterator/selector machinery** (T2): `every_`/`any_`/`random_` lists → enrollment-selector
  specs; `random_*` compiles to a seeded deterministic stream or is rejected (guardrail 5).
- **Named runtime variables** (T1): `set_variable`/`check_variable` → `Named` sub-field column
  surface in EffectSpec/TriggerSpec, including the **read-modify-write family**
  (`change_variable`, `subtract_variable`, §8.2), which binds to the effect-ordering contract.
- **Dynamic identifiers** (T2, likely reject-with-bounds): `set_flag = @root`-style
  runtime-constructed flag/variable *names* (§3.6) collide with admission-time column
  registration — names must be statically enumerable; dynamic construction is rejected at
  admission or bounded to a pre-declared per-scope name family.
- **`EmitEvent` payload context** (T2, via SCOPE-MEMO): `from`/`root` chain support.
- **List-registry + `category_map` tables** (T1): the duplication policy's list registry and the
  category default table as first-class, documented spec inputs.
- **Modifier-key grammar — the classifier engine** (T1, **frontier-gated**, lands with CT-2c):
  generated keys decompose compositionally — **a family of grammars, not a dictionary** —
  **verified against `modifiers.log` (41,016 keys, 2026-06-10):** the `shipsize_*` grammar covers
  ~69% (28,179 keys), the economic grammar (`(category)_(resource)_(produces|upkeep|cost)_(add|mult)`)
  ~16% (6,426), hand-defined residue ~15%. **CT-2c implements the economic decoder**; ship grammars
  wait for a consumer that names ships. Implementation note: category segments are
  underscore-ambiguous (`pop_category_bio_trophy_unity_upkeep_add`) → **longest-match resolution
  against the registered category and resource sets**, never naive splitting.
  `triggered_produces_modifier` compiles as gated family generation. Two binding semantics ride
  with it: the **inheritance asymmetry** (`_mult` sweeps the category subtree via reduction
  OrderBands; `_add` applies leaf-only — Paradox's own anti-cascade rule, natively matching
  sweep-vs-leaf reduction semantics) and the **granularity rule — CORRECTED against the primary
  source (errata #1)**: the `modifiers.log` header states categories are a *soft tag* suggesting
  intended level, and lower-granularity modifiers legally apply at higher levels with
  **broadcast-down** (a country-applied pop modifier hits all the country's pops). Lowering:
  category = default application level + legal cascade-down via ancestor-stack overlay sweep
  (native); admission validates **direction only** (no cascade-up), not strict level equality.
- **Timed modifiers & `has_modifier` reads** (T1/T2): `add_modifier = { days = N }` → overlay
  expiry in ticks via the time-model mapping (small `simthing-spec` widening if overlay lifecycles
  lack tick expiry); `remove_modifier` → `SuspendOverlay`; `has_modifier = X` **as a trigger**
  needs overlay-active state readable — recompile the gating predicate inline, or expose Named
  active-flag columns (design choice taken at this gate **by design authority**; implementation
  thereafter is mechanical and Codex-eligible).
- **Control-flow lowering** (T1/T2 split, **frontier-gated**): `if`/`else_if`/`else` and `switch` → `SELECT` chains /
  gated effect groups (T1 once the entity contract exists); **`break`** (sequential short-circuit
  inside an effect block, `effects.log`-verified) → a taken-flag gating all subsequent groups —
  binds to the effect-ordering contract. **`while` — verified faithful:** the source engine's own
  semantics are already bounded (`while = { limit = {...} }` *"until set iteration count is
  reached"*, plus `while = { count = [N|Variable] }`), so bounded-compile-with-declared-cap is
  fidelity, not restriction; `count = Variable` compiles to runtime-gated iterations under a
  static max.
- **Effect-ordering contract** (T2, **frontier-gated**): the textbook §6.3 makes statement order significant inside an
  effect block (later statements observe earlier side effects) — vs SimThing's batch boundary
  application. Ordered or staged `BoundaryRequest` application semantics, designed once, before
  any effect-chain rung.
- **Multi-polity higher-order structures** (deferred, no rung): federations, galactic-community
  resolutions, agreement terms (§11.4) compile to owner-entities + arenas + coupling edges when a
  consumer names them — deferred, not rejected.

### 6.1 Transpilation hardness register (the textbook's difficulty analysis, dispositioned)

The ClauseScript textbook grades constructs by transpilation difficulty in its per-section
"Transpiler Implications." This register compiles every graded item and pins each to a track
disposition so no hardness verdict stays buried in body text. **Maintenance rule: a rung that
discovers an ungraded construct updates this register in the same PR.**

**Key caveat — secondary-source provenance (binding on every rung that uses this register or the
§6 modifier items).** The ClauseScript textbook is an AI-authored synthesis of community
reverse-engineering of a **closed-source engine**. Its semantic claims — the generated-key
grammar, the `_mult`-propagates / `_add`-does-not inheritance asymmetry, mult-additive-in-effect
stacking, `modifier_category` granularity rules, expansion ordering — are **provisional until
verified against the lab primary sources** (`script_documentation/*.log`,
`99_README_ECONOMIC_CATEGORIES.txt`, real `common/` files). The house rule applies: *source is
ground truth; documentation drifts* — and a synthesized textbook can also confabulate.
Concretely: CT-2c's key-grammar decoder is accepted only after it **round-trips `modifiers.log`
itself** (every generated key in the log must parse; unparseable keys are either hand-defined or
evidence the grammar is wrong); the inheritance asymmetry and granularity rules are checked
against the README before being hard-coded; any textbook claim that fails verification is
corrected in this register **and** noted as errata in the textbook copy. Where verification is
impossible (closed-engine corners), model-exactly-or-reject (guardrail 4) governs. And where the
textbook itself reports source semantics as **implementation-defined** (script-value evaluation
order and caching, §3.5/§8.3), fidelity-in-principle is unachievable and is **not chased**:
ClauseThing defines its own deterministic order at admission and documents the divergence.

**Measured vocabulary (primary-source scan of the lab logs, 2026-06-10):** 90 scopes (the
textbook implied roughly a dozen domain scopes — errata #3); 1,015 effects and 1,041 triggers,
of which **~28% of effects and ~20% of triggers are iterator forms**
(`every_/any_/count_/ordered_/random_*`) — the iterator/selector mechanism plus the scope model
covers a quarter of the vocabulary in one stroke, leaving a non-iterator effect residue of ~700
for the diagnostic queue; 41,016 modifier keys split per the grammar-family fractions in §6.
**Textbook-omitted construct classes found in the logs:** the DLC-era staged-progress systems —
*situations, archaeology sites, first contact, astral rifts* — are structurally a progress
sub-field + stage thresholds + approach overlays, i.e. the capability/event pattern with no new
substrate; their effect/trigger vocabulary lands in the standard-library queue like everything
else.

**Class A — mechanical (lowers cleanly from the parsed tree; the "fairly easy" majority).**
Blocks/properties/templates; static `modifier` → overlays; capability trees — including the
one-time-vs-ongoing distinction (adoption `BoundaryRequest` vs `Permanent` overlay) and prereq
DAGs; literal `resources`; static topology; `@vars` constant folding; `ai_weight` formulas;
same-scope triggers; **modifier source tracking** (§7.7's "where did this bonus come from" is
native — overlays are first-class with identity); **decisions** (§13.4 — events with
actor-initiated triggers, no new substrate); **leaders/governors** (§13.3/§10.6 — SimThings with
subtree-scoped trait overlays; ancestor-stack propagation is native); the §10.9 feedback-cycle
requirement ("previous-tick snapshots") — which is the engine's native ping-pong discipline,
independently derived by the textbook. *Disposition: the existing rungs, CT-1a → CT-3b+4a.*

**Class B — complicated (real engineering, clear compilation target).**
`triggered_modifier` lifecycles (CT-1b measures the cost assumption); category → tree-level table
(CT-2c); macro expansion ordering incl. recursive `inline_script` (CT-0c, textbook §3.7);
duplication/override policy (adopted); economic-category inheritance sweeps (CT-2c);
`if`/`else_if`/`else` and `switch` (control-flow backlog item); `limit`-filtered iterators (folds
into the iterator/selector backlog item); dot-notation scope paths + Supported/Output validation
against `scopes.log` (CT-0d); `prev` resolution via a compile-time scope stack (CT-0d);
content-unlock cascades from capabilities (registration fan-out, CT-1c onward); **modifier-key
grammar decomposition + inheritance asymmetry + granularity guard** (CT-2c via the §6 classifier-
engine item); timed-modifier expiry mapping (§6); controller-vs-owner dual relation columns
(§10.6 — occupation redirects compile as a second relation column, never a flag);
stockpile-banded `ai_budget` entries → threshold-gated `Suspended` weight overlays with native
allocator normalization (§10.8, CT-4b); `value:` recursion → compile-time expansion under a
declared depth cap against the 32-node class, reject beyond (§8.3); on_action multi-registration
→ list-registry collect in deterministic source order (§9.3).

**Class C — difficult (substrate extension or design gate required).**
- `root`/`from`/`fromfrom` event-context chains and **`event_target:` named runtime references** →
  SCOPE-MEMO + the `EmitEvent`-payload backlog item. The textbook calls these "the primary
  mechanism for context passing"; they are the single largest fidelity risk and are deliberately
  quarantined off the T1 path.
- **Sequential effect observability** (textbook §6.3) vs batch boundary application → the
  effect-ordering-contract backlog item (T2).
- Multi-claimant producer attribution / transfer slicing (`overlord_resources`, textbook §10.10)
  → 2d coupling (T2).
- Modifier stacking-rule exactness ("not always simple addition or multiplication," §7.7) →
  model-exactly-or-reject under I8 parity (guardrail 4).
- `random`/`random_list` → seeded deterministic stream or reject (guardrail 5).
- **`while`** → bounded-iteration compile with a declared cap, or reject; unbounded-as-authored is
  Class D. Decided at the control-flow gate.
- Runtime hyperlane mutation + hyperlane-distance/pathfinding queries (§14.3) → boundary topology
  edits + the CPU-side min-plus distance machinery; mappable, gated with T3.
- Velocity/trajectory pressures → explicit previous-value columns (4c, known constraint).
- Cross-faction comparative reads (`relative_power`, `is_threatened_by`, opinion modifiers,
  §17.4) → post-reduction faction aggregates + **pairwise relation columns**; perceived variants
  ride 3c perception filters. Columnar and cheap, but unscoped until a rung names it.

**Class D — architecturally rejected or deferred-by-design (and correctly so).**
Unbounded `while`; engine-call effects with no state equivalent (rendering, UI, save/load hooks);
save-game ingestion; presentation-only constructs; unseeded RNG; CPU-planner AI hooks; and the
deferred multi-polity governance layer (§11.4 — deferred without a rung, not rejected: it has a
clean compilation story when a consumer names it). Every Class-D rejection is a **hard admission
error with a spanned diagnostic** and, where one exists, a suggested path — the diagnostic stream
is the backlog's priority queue (§6 doctrine).

## 7. Fixture & licensing boundary (binding)

- **Paradox-generated material never enters this repo**: `script_documentation/*.log`, game
  `common/` files, and any Stellaris-derived content stay in `C:\Users\mvorm\Clauser\` as
  **lab-only** references. Tests that consult them are local/ignored-by-default
  (`CLAUSER_LAB_DIR` env opt-in) and never gate CI.
- **Repo fixtures are original**: ClauseScript written for SimThing by us, exercising the same
  grammar shapes (the §2.7 weed suite is authored from the *patterns*, not copied content).
- **Jomini's own MIT fixtures** may be vendored with the source under §4 accounting.

## 8. Deviation record — export-aware metadata deferred

The horizon charter (`workshop/field_world_model_horizon.md` §1.2) ruled exporter-first so the
importer would carry schema fields proven by `FIELD-MOVIE-DATASET-0`. Product opened the importer
first (this track). **Deviation, recorded:** the export-aware metadata hooks (episode/regime
tagging on authored scenarios) are **deferred, not dropped** — `RawEntity`/scenario structs carry
a documented, inert `export_meta` placeholder from CT-0b onward, to be populated when the exporter
exists. No rung in this track may claim corpus-export capability.

## 9. Discipline (restated, binding)

Per-PR shape: one crate change + one test report + one status row here. CPU-oracle bit-exact
parity on everything that computes. Opt-in, default-off, reversible. Determinism: no RNG construct
survives admission unseeded. The repo guard's banned legacy acronym applies — **Movement-Front /
FIELD_POLICY** naming only. Worklog entry per landed rung. Stop-and-escalate on any conflict with
`simthing_core_design.md` §9 litmus tests. Agent gating per §5: frontier-flagged rungs and frontier-gated backlog items go to
Claude Opus (max effort) or Claude Fable 5 (high effort) only; the flag never downgrades. The
ClauseScript textbook ([`clausething/ClauseThing.md`](clausething/ClauseThing.md)) is **reference,
not specification**: it describes a foreign engine, and implementing its engine model (lazy
invalidation, sequential imperative effects, CPU-side evaluation) instead of compiling onto
SimThing's substrate is the documented drift pattern — its preamble and the §6.1 provenance
caveat are binding; scope comes from the §5 ladder only, never from the book.

## 10. Read order (low-context agents start here)

1. This document (§2 ruling, §5 your rung, §11 status).
2. [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4 (correspondence) + §5 (tiers) + §8 (limits).
3. `simthing_core_design.md` (always in full) + `invariants.md`.
4. `capability_tree_v1.md` (T1 rungs), `adr/resource_flow_substrate.md` (T2 rungs),
   `adr/mapping_sparse_regioncell.md` (T3/T4 rungs).
5. `crates/simthing-spec/src/spec/` — the actual structs you emit (read before mapping anything).
6. [`clausething/ClauseThing.md`](clausething/ClauseThing.md) — the ClauseScript textbook
   (language ground truth, **subject to the §6.1 provenance caveat**; moved into the repo
   2026-06-10 for implementation-agent access). Lab-only (never committed):
   `C:\Users\mvorm\Clauser\Paradox\script_documentation\*.log`, `Clauser/jomini/` (vendor source).

## 11. Status ledger

| Rung | Status | Report |
|---|---|---|
| CT-0a vendor + skeleton + licenses | **IMPLEMENTED / PASS** (remedial verify closed 2026-06-10; unrelated pre-existing GPU fingerprint failure on this host documented in report) | [`docs/tests/ct_0a_impl_results.md`](tests/ct_0a_impl_results.md) |
| CT-0b lossless raw model | NOT STARTED | — |
| CT-0c expansion passes | NOT STARTED | — |
| CT-0d scope extraction + frequency report | NOT STARTED | — |
| CT-1a bit-identical entity | NOT STARTED | — |
| SCOPE-MEMO design gate | NOT STARTED (blocked on CT-0d/CT-1a) | — |
| CT-1b recalc stress measurement | NOT STARTED | — |
| CT-1c capability tree | NOT STARTED | — |
| CT-2a intrinsic flows | NOT STARTED | — |
| CT-2c category economy | NOT STARTED | — |
| CT-3b+4a headline vertical | NOT STARTED | — |

*Opened 2026-06-10 by product decision (Mike) on the design authority's parser-first
determination. The parser is the consumer that pulls the spec.*
