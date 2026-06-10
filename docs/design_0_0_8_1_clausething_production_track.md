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

| Rung | Gate | Scope | Exit criteria |
|---|---|---|---|
| **CT-0a** | T2 (track entry) | Crate skeleton + vendored jomini text path + license accounting (§4 items 1–3) | Workspace builds; vendored tests green; `THIRD_PARTY_LICENSES.md` present |
| **CT-0b** | T2 | **Lossless raw model**: tape → typed raw structs preserving ordered duplication, quoting, headers, operators, mixed containers (textbook §2.7 weeds) | Round-trip corpus → JSON golden files → re-emit; byte-faithful where jomini's writer is; §2.7 weed suite green |
| **CT-0c** | T1 | Macro/expansion passes in textbook §3.7 order: `@vars` → `inline_script` + `[[param]]`/`$PARAM$` → `@[ ]` inline math; `value:` left symbolic | Worked plague example (textbook §3.8) expands to golden output; expansion-order pitfall tests |
| **CT-0d** | T1 | Scope-chain extraction + validation against `scopes.log` (lab fixture, §7); **corpus frequency report** (scope usage histogram — SCOPE-MEMO's evidence) | Malformed chains rejected with spanned diagnostics; frequency report artifact produced |
| **CT-1a** | T2 (entity-hydration contract) | Literal `modifier` blocks + flat properties on one template → overlays through the existing install path | **Bit-identity**: ClauseScript-authored entity ≡ hand-authored RON (RON-diff clean; installed tree bit-exact; CPU-oracle parity on overlay application) |
| **SCOPE-MEMO** | T2 design gate | Symbolic scope model design memo from CT-0d evidence: `ScopeRef` successor, `EmitEvent` payload context, rejection classes. **No code until accepted.** | Memo accepted by design authority; widening tickets cut |
| **CT-1b** | T1 (measurement) | Recalc stress test: large `triggered_modifier` corpus → `Suspended`/`Threshold`/column counts + tick cost vs RON baseline | Measured report; "every-tick is a net simplification" confirmed or the assumption retired |
| **CT-1c** | T1 | One capability tree (small tradition set) → `capability_tree_v1` pattern: prereq DAG → threshold ordering, payload activation | First "designer writes Clausewitz, SimThing runs it" proof; parity green |
| **CT-2a** | T1 | Literal `produces`/`upkeep` → `IntrinsicFlow` registrations (opt-in, default-off) | Micro-economy fixture green |
| **CT-2c** | T1 | `value:` amounts + `economic_category` inheritance → reduction OrderBands; `category_map` defaults + hard-error diagnostics | The Daily Economy Fixture, authored in ClauseScript, matches the RON original |
| **CT-3b + CT-4a** | T1 over accepted substrate | **Headline vertical:** ClauseScript-authored `RegionFieldSpec` + `ai_will_do` → the accepted field → reduction → `field_urgency` EML → threshold commitment path, default-off | The entire accepted Phase-M vertical, authored in Clausewitz, `simthing-sim` map-free and ClauseScript-blind |

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
  surface in EffectSpec/TriggerSpec.
- **`EmitEvent` payload context** (T2, via SCOPE-MEMO): `from`/`root` chain support.
- **List-registry + `category_map` tables** (T1): the duplication policy's list registry and the
  category default table as first-class, documented spec inputs.

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
`simthing_core_design.md` §9 litmus tests.

## 10. Read order (low-context agents start here)

1. This document (§2 ruling, §5 your rung, §11 status).
2. [`clausething/ClauseThing_Spec.md`](clausething/ClauseThing_Spec.md) §4 (correspondence) + §5 (tiers) + §8 (limits).
3. `simthing_core_design.md` (always in full) + `invariants.md`.
4. `capability_tree_v1.md` (T1 rungs), `adr/resource_flow_substrate.md` (T2 rungs),
   `adr/mapping_sparse_regioncell.md` (T3/T4 rungs).
5. `crates/simthing-spec/src/spec/` — the actual structs you emit (read before mapping anything).
6. Lab-only (never committed): `C:\Users\mvorm\Clauser\ClauseThing.md` (language ground truth),
   `Clauser/Paradox/script_documentation/*.log`, `Clauser/jomini/` (vendor source).

## 11. Status ledger

| Rung | Status | Report |
|---|---|---|
| CT-0a vendor + skeleton + licenses | **OPEN — next** | — |
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
