# SCOPE-MEMO — Symbolic Scope Model for ClauseThing Hydration

> **Status: DESIGN MEMO / READY FOR REVIEW (authored 2026-06-11, frontier design agent per track
> §5 gating; acceptance is a design-authority ruling recorded in the track ledger).** This is the
> SCOPE-MEMO rung of [`../design_0_0_8_1_clausething_production_track.md`](../design_0_0_8_1_clausething_production_track.md)
> (§5): a **design memo, not an implementation**. No production code changes ride with it, and no
> implementation may start from it before acceptance. Authority: the 0.0.8.1 constitution §0/§2,
> [`../simthing_core_design.md`](../simthing_core_design.md), [`../invariants.md`](../invariants.md),
> the production track §2/§5/§6/§7/§9/§11, [`ClauseThing_Spec.md`](ClauseThing_Spec.md), and the
> CT-0d / CT-1a evidence reports. The ClauseScript textbook
> ([`ClauseThing.md`](ClauseThing.md) §4) is consulted as **reference under the track §6.1
> secondary-source provenance caveat** — scope-language shape only, never engine semantics.

**Named consumers (what this memo unblocks):**

1. **CT-1b and later triggered-modifier / recalc rungs** where scope references become
   load-bearing (cross-scope `potential` triggers, owner-scoped modifiers).
2. **CT-1c capability-tree hydration** where payload activation and prerequisite contexts may
   require scoped references beyond the install owner.
3. **The future `EmitEvent` payload-context rung** (track §6 backlog item "EmitEvent payload
   context", T2, gated on this memo).
4. **The retirement of `ScopeRef::Slot(u32)` from authored surfaces** — the authoring-hostile
   variant named in track §2.1 and §6.

**Evidence base actually used (only where it changes a decision):** CT-0d's lab aggregate scan
(90 scope names, 25 output-scope classes, 356 supported-relation pairs;
[`../tests/ct_0d_impl_results.md`](../tests/ct_0d_impl_results.md)) drives §2.4's decision that
the domain-scope step table is **registered designer data, not a Rust enum** — 90 names with 356
typing relations cannot be a hard-coded vocabulary, and full coverage is consumer-pulled per the
track §6 doctrine. CT-1a's per-owner install machinery (`InstallTargetSpec`, per-instance
`current_slot`, slot-churn refresh; [`../adr/scripted_event_scope_model.md`](../adr/scripted_event_scope_model.md))
drives §2.3's decision that `Root`/`This` resolve through the **existing instance-binding
pattern**, not a new mechanism. No other aggregate counts are cited; none are decoration.

---

## 1. Symbolic scope atoms

The authoring-time atom vocabulary is the CT-0d vocabulary
([`crates/simthing-clausething/src/scope.rs`](../../crates/simthing-clausething/src/scope.rs)
`ScopeAtomKind`), promoted unchanged in meaning. One authored scope reference is a non-empty
ordered chain of atoms:

| Atom | ClauseScript surface | Meaning (authoring-time, symbolic) |
|---|---|---|
| `This` | `this` | The current scope at the point of evaluation, after lexical scope changes. |
| `Root` | `root` | The entity the compiled artifact is evaluated against: the **install owner** of the trigger/effect/event instance, or — inside a chained event — the chain-origin scope carried in the event context (§3). |
| `From { depth: n }` | `from`, `fromfrom`, … | The n-th prior **event evaluation context** in an event chain. Only meaningful where an event context exists (§3); `depth` is statically bounded (§7). |
| `Prev { depth: n }` | `prev`, `prevprev`, … | The scope n lexical scope-changes back. **Compile-time only**: hydration resolves every `Prev` against the lexical scope stack and rewrites the chain to an absolute form (§2.2); `Prev` never survives into compiled artifacts. |
| `Step { name }` | `owner`, `planet`, `capital_scope`, …, and dot-path segments | A **registered domain scope step** (§2.4). Subsumes CT-0d's `Domain`; dot-notation segments and block scope keys produce identical atoms (textbook §4.4: dot chains are semantically equivalent to nested scope changes). |
| `EventTarget { name }` | `event_target:NAME` | A named entry in the event context's bounded target map (§3.3). `name` must be statically enumerable at admission. |
| *(rejected)* | anything else | Unknown atoms are **hard hydration errors with spanned diagnostics** — never silently passed through, never guessed (no proxy-but-pass). |

Member access via dot paths is represented as the same chain: `root.owner.capital_scope` is
`[Root, Step("owner"), Step("capital_scope")]`. A terminal dot segment that is a *property or
trigger name* rather than a scope step is split off by hydration using the registered scope-step
table first, then the property registry; a name registered as **both** is an ambiguity rejection
(§7.3), never a silent precedence pick.

## 2. Symbolic scope chain representation

### 2.1 The authored data model (the `ScopeRef` successor)

Proposed `simthing-spec` authoring struct (name final at ticket implementation; shape binding):

```rust
/// Authored, symbolic, compile-time scope reference. Replaces ScopeRef on
/// authored surfaces. Contains no slot index, no column index, no runtime id.
pub struct ScopeChainRef {
    pub atoms: Vec<ScopeAtomSpec>,   // non-empty; source order
}

pub enum ScopeAtomSpec {
    This,
    Root,
    From { depth: u8 },              // 1 = from, 2 = fromfrom, …
    Step { name: String },           // registered domain scope step
    EventTarget { name: String },    // statically enumerated named target
}
```

Notes on the shape:

- **No `Prev` variant.** `prev` is resolved lexically during hydration (§2.2) and is therefore a
  ClauseThing-internal atom only. RON authors never needed it (RON has no lexical scope stack);
  admitting it to the spec surface would be speculative widening.
- **No `Slot`, no `u32`, no runtime id anywhere.** `ScopeRef::Slot(u32)` is retired from authored
  surfaces when this lands (track §6). `ScopeRef::Current` migrates as `ScopeChainRef { atoms:
  [This] }`; existing RON authoring stays expressible verbatim.
- **Consuming surfaces** widen mechanically at the ticket (§5): `TriggerSpec::Threshold.target`,
  `EffectSpec::{Remove, ActivateOverlay, SuspendOverlay}.target`, `ScriptExpr::Read.scope`.

### 2.2 Compile-time symbolic vs runtime-resolved (the binding distinction)

Three strictly separated representations; data flows left to right and never back:

```
ClauseThing lexical form          simthing-spec authored form        compiled/runtime form
(this/prev/dot paths, scope    →  ScopeChainRef (symbolic atoms,  →  resolved binding (slot /
 stack tracked at hydration;       serde, designer-inspectable)       instance binding / boundary
 Prev rewritten to absolute)                                          resolution plan; NOT serde-
                                                                      authored, driver-internal)
```

- **Hydration (ClauseThing, Stage 5):** tracks the lexical scope stack over the expanded raw
  model; rewrites `Prev{n}` to the absolute chain it denotes; emits `ScopeChainRef`. A `prev`
  that walks past the bottom of the lexical stack is a spanned hydration rejection.
- **Admission (`simthing-spec`):** type-checks the chain against the registered scope-step table
  (§2.4) — every `Step` must exist, every step's Supported set must contain the preceding atom's
  output class, `From`/`EventTarget` atoms are admitted only on surfaces that carry an event
  context (§3.4). Anything else is a hard admission error with the CT-0b span.
- **Install/boundary (`simthing-driver`):** compiles the admitted chain to a resolved binding —
  exactly the pattern `ScriptedEventInstance.current_slot` uses today (per-instance binding,
  refreshed on slot churn). The runtime representation is compiled symbolic data consumed by the
  existing event/boundary substrate; it is **never** authored, serialized as authoring, or
  exposed to designers.

### 2.3 Resolution classes (when each atom binds)

| Atom | Binds at | Mechanism (existing substrate only) |
|---|---|---|
| `This`, `Root` (unchained) | install | the per-instance owner binding (`current_slot` pattern, slot-churn refresh per the O4 ADR) |
| `Root` (inside event chain) | boundary | read from the event context payload (§3) |
| `Step` | boundary | the step's registered **resolution binding** (§2.4): a spatial-ancestor walk over the installed tree or a relation-column read from the CPU shadow of GPU-resolved values. Reading a GPU-resolved owner column to resolve an effect target is boundary *consumption*, not recomputation — same class as `slot_to_thing` today |
| `From` | boundary | indexed read of the event context's from-stack (§3.2) |
| `EventTarget` | boundary | keyed read of the event context's bounded named-target map (§3.3) |

A chain whose every step cannot be assigned one of these classes at admission **cannot be
statically bounded and is rejected** — that is the rejection path the memo mandate requires. There
is no fallback interpreter, no deferred-to-runtime "we'll see", no CPU lazy evaluation.

### 2.4 The registered scope-step table (CT-0d-evidence-driven)

The lab scan found **90 scope names with 25 output classes and 356 supported-relation pairs**.
That settles the design question: the domain-step vocabulary is **registered designer data** (a
spec input, sibling of the `category_map` table in the track §6 list-registry item), not an enum:

```rust
pub struct ScopeStepSpec {
    pub name: String,                 // "owner", "capital_scope", …
    pub supported: Vec<String>,       // input scope classes (scopes.log Supported)
    pub output: String,               // output scope class   (scopes.log Output)
    pub binding: ScopeStepBinding,
}

pub enum ScopeStepBinding {
    SpatialAncestor { kind: String },              // walk the installed spatial tree
    RelationColumn { property: PropertyKey, role: SubFieldRole }, // read an owner/relation column
}
```

Scope classes are strings against the registered class set, exactly as `PropertyKey` is symbolic.
A `Step` whose name has no registered `ScopeStepSpec` is an admission rejection **with a suggested
mapping** — the diagnostic stream is the backlog priority queue (track §6 doctrine). Coverage is
consumer-pulled: rungs register the steps their fixtures need; nobody transcribes 90 entries
speculatively. Repo fixtures stay original (track §7); the synthetic `ScopeTable` in CT-0d tests
is the pattern.

### 2.5 Deterministic serialization

`ScopeChainRef` serializes through serde exactly like every other authoring struct (RON-native,
PascalCase enum tags per the existing `spec/` convention): an ordered sequence of tagged atoms,
no maps, no implicit defaults inside the chain. Example, today vs successor:

```ron
// today                       // successor (same trigger, symbolic owner scope)
target: Current,               target: (atoms: [This]),
                               target: (atoms: [Root, Step(name: "owner")]),
```

Golden tests serialize through `simthing-clausething`'s existing deterministic JSON projection;
chains carry their CT-0b `RawSpan` (token index) end-to-end for diagnostics, with spans excluded
from canonical-equality fingerprints exactly as CT-1a's `InstalledTreeFingerprint` already
normalizes ids.

## 3. Event payload context

### 3.1 What exists today, and what the gap is

The runtime event path is GPU `Threshold` + `EmitEvent` → compact `EmissionRecord` (event id +
slot) → `ScriptedEventTriggerEvent` → `ScriptedEventBoundaryHandler` resolves effect targets and
pushes `BoundaryRequest`s. The payload is deliberately minimal: *which registration fired, on
which slot*. ClauseScript event chains additionally expect `root`, `from`/`fromfrom`, and named
`event_target:` references — the textbook calls context passing "the primary mechanism" of
event-driven script, and the track Class-C register names it the single largest fidelity risk.

The design rule: **the GPU payload does not widen.** All chain context is constructed and consumed
at the boundary, where events are already dispatched. Nothing below the spec/driver layer learns
any of this exists.

### 3.2 The context model

```rust
/// Boundary-layer event-chain context. Constructed by the boundary handler
/// when an event fires; carried to follow-on events emitted by its effects.
/// Never serialized as authoring; replay state per the replay ADR taxonomy.
pub struct EventScopeContext {
    pub root: SimThingId,                       // chain-origin scope
    pub from_stack: Vec<SimThingId>,            // [from, fromfrom, …]; len ≤ declared cap
    pub named_targets: BTreeMap<String, SimThingId>, // keys fixed at admission (§3.3)
}
```

- **Explicit fields vs derived:** `root` and `from_stack` are **explicit payload fields**, set by
  the boundary handler from event execution state at emit time — `root` propagates unchanged from
  the chain origin; the emitting event's evaluation scope is pushed onto the receiver's
  `from_stack` (so the emitter's `from` becomes the receiver's `fromfrom` by an ordinary
  shift-push, the receiver's `from` is the emitter — **no implicit CPU stack magic, no ambient
  state**: the stack is a value in the payload, bounded, inspectable, replay-serializable).
- **Depth cap:** `from_stack` length is capped by a declared admission constant. The cap value is
  set at ticket time from the deepest transitive form the consuming rung's fixtures actually use
  (the language documents `fromfrom`-class forms; CT-0d's scan covered scope *names*, not chain
  *usage depth*, so the cap is a declared admission bound, honestly not a corpus-derived one). A
  `From{depth}` atom exceeding the cap is an admission rejection.
- **Boundary/event compatibility:** a follow-on event emitted by an effect is ordinary boundary
  work (the effect emits, the handler dispatches within the boundary's existing
  priority/cooldown protocol). Context handoff adds **data** to that path, not a new execution
  phase, not mid-tick mutation, not a sequential-effect-observability change (that remains the
  separate effect-ordering-contract gate, track §6 — explicitly out of scope here).

### 3.3 Named event targets

`save_event_target_as = name` (and successors) are admissible **only as statically enumerable
names**: admission collects every saved-target name declared in the compiled event family and
fixes the `named_targets` key set per chain. The track §6 dynamic-identifiers ruling applies
verbatim — a runtime-constructed target *name* (`save_event_target_as = flag_@root`-class) is
rejected at admission. The target *value* (which SimThing the name binds to) is runtime data,
written at boundary time by the saving event and read by `EventTarget` atoms downstream. Unbounded
runtime *creation of names* is rejected; bounded runtime *binding of pre-declared names* is the
design.

### 3.4 Where event-context atoms are legal

`From` and `EventTarget` atoms are admitted **only** on surfaces evaluated with an
`EventScopeContext` in hand: event effect targets, event predicate triggers, and follow-on emit
specs. They are **rejected in GPU threshold trigger targets** (`TriggerSpec::Threshold`): a
threshold registration needs a concrete slot at registration time, and event-chain context does
not exist until events fire. This is a sharp, easily-checked admission rule and the load-bearing
guard that keeps scope chains from leaking runtime semantics into the GPU substrate. (`Step`
chains in threshold targets are legal: they resolve to a slot at install/boundary and re-register
through the existing `sync_spec_threshold_registrations` churn path when a relation flips.)

## 4. Scope validation

Four stages; every diagnostic carries the CT-0b/CT-0d `RawSpan` (token index) so designer errors
point at source tokens:

1. **Now, syntactic (CT-0d, shipped):** atom grammar, malformed chains (empty dot segments, bad
   `event_target:` segments), unknown-domain detection against a scope table, deterministic
   source-order extraction. Nothing in this memo invalidates CT-0d output; the memo consumes it.
2. **Hydration-time, lexical (ClauseThing, at the consuming rung):** `Prev` resolution against
   the lexical scope stack (reject on underflow); `This` normalization; dot-path
   scope-vs-property terminal split (reject on ambiguity); rejection of context atoms outside
   event surfaces.
3. **Admission-time, semantic (`simthing-spec`, at the consuming rung):** Supported/Output
   transition typing over the registered step table; `From` depth cap; `EventTarget` name
   enumeration; threshold-target restrictions (§3.4); unknown step → suggested-mapping error.
4. **Install/boundary, structural (`simthing-driver`):** unresolvable binding at install
   (missing slot, missing relation column) is an install error or an `OwnerRemoved`-class
   boundary diagnostic — the existing patterns, unchanged.

Forms rejected **immediately** (stage 1–2, no deferral): malformed chains, `prev` underflow,
ambiguous terminals, dynamic name construction. Forms deferred to **later semantic validation**
(stage 3, because they need the registered table or event-family data that only exists at the
consuming rung): transition typing, depth caps, target-name enumeration, per-surface legality.

## 5. `simthing-spec` widening tickets (cut on acceptance; no speculative widening)

Three tickets, each with a named consumer rung; nothing else widens. Relation to
`EffectSpec`/`TriggerSpec` widening: these tickets change only the **target/scope field type** on
those enums; vocabulary widening (more effect/trigger variants) remains the separate track §6
items and is not pulled by this memo.

| Ticket | Contents | Named consumer (when it opens) |
|---|---|---|
| **SPEC-SCOPE-1 — `ScopeChainRef` successor** (T2, frontier) | `ScopeChainRef`/`ScopeAtomSpec` (§2.1); mechanical migration of `TriggerSpec`/`EffectSpec`/`ScriptExpr` target fields (`Current` ≡ `[This]`); retire `Slot(u32)` from authored surfaces; admission checks §4.3 minus table-dependent ones | **CT-1c** — first rung whose fixtures may need an owner-scoped reference in capability payload activation/prereqs; CT-1b pulls it earlier **only if** its measurement corpus needs cross-scope `potential` (not expected; §8) |
| **SPEC-SCOPE-2 — scope-step table** (T1 once SPEC-SCOPE-1 lands) | `ScopeStepSpec`/`ScopeStepBinding` registration (§2.4); Supported/Output transition typing; install/boundary resolution of `Step` chains via spatial-ancestor walk + relation-column read; threshold re-registration on relation churn | the first rung that registers a real step — expected **CT-1b follow-on or CT-1c**, whichever first authors `owner`-class navigation |
| **SPEC-SCOPE-3 — `EventScopeContext` payload** (T2) | the §3 context struct; boundary construction/handoff; `save_event_target_as` static name enumeration; `From`/`EventTarget` admission rules incl. the §3.4 threshold restriction; replay classification of context state | the **future `EmitEvent` payload-context rung** (track §6) — opens when ClauseScript event chains are first hydrated, post-CT-1c; nothing earlier touches it |

## 6. Runtime / GPU implications (explicit bounds)

**This design makes no new GPU kernel, WGSL, or runtime-planner claim.** Specifically:

- The runtime behavior path remains exactly:
  **resolved field → mask/modifier → threshold crossing → event → `BoundaryRequest`.**
- No WGSL changes; no new `AccumulatorOp`/`ConsumeMode`/opcode; the GPU `EmitEvent` payload
  (compact `EmissionRecord`) does not widen.
- All runtime target resolution is **compiled symbolic data consumed by the existing
  event/boundary substrate** — install-time instance bindings, boundary-time relation-column
  reads of GPU-resolved values, and boundary-time event-context lookups. There is no ClauseScript
  evaluation in `simthing-sim`, no CPU-side ClauseScript evaluator anywhere, and no CPU planner:
  the boundary consumes resolved values to *address* effect targets; it never traverses state to
  *decide* anything.
- `simthing-sim` remains ClauseThing-blind and scope-blind: it continues to see flat columns,
  threshold registrations, and `BoundaryRequest`s. ClauseScript must not and does not become a
  runtime language: chains are admission-time data that compile away to bindings.

## 7. Rejection classes (all: hard error + spanned diagnostic + suggested path where one exists)

1. **Dynamic unbounded names** — runtime-constructed scope/target/flag *names*
   (`@root`-interpolated identifiers). Names must be statically enumerable (track §6 ruling).
2. **Unbounded runtime-created event targets** — saved-target names outside the
   admission-enumerated set (§3.3).
3. **Ambiguous chains** — `prev` underflow; dot-terminal registered as both scope step and
   property; unregistered `Step` names; Supported/Output type mismatches; `From` beyond the
   declared depth cap.
4. **Forms requiring CPU-engine lazy evaluation or sequential imperative behavior** — iterator
   scopes (`every_*`/`any_*`/`random_*` as navigation) are not scope atoms and are rejected here;
   they belong to the separate iterator/selector backlog item (track §6) with its own
   seeded-determinism contract. Likewise any chain whose meaning depends on within-block effect
   ordering (the effect-ordering-contract gate, not this memo).
5. **Save-game ingestion and presentation-only constructs** — permanently out of scope
   (spec §10); scope forms reaching into save-state or UI context are rejected with no suggested
   path.
6. **Event-context atoms in GPU threshold targets** (§3.4) — rejected with the suggested path of
   a predicate trigger or a boundary-evaluated event chain.

## 8. Future rung sequencing

- **Implementation PR after acceptance:** none immediately — acceptance cuts the §5 tickets into
  the track §6 backlog as specified items; each lands consumer-pulled. The next ladder rung,
  **CT-1b, proceeds now with same-scope-only support** (`ScopeRef::Current` suffices; track §2.5
  chose T1 rungs to need exactly this). CT-1b does not wait for any ticket; if its corpus turns
  out to need cross-scope `potential` triggers, it pulls SPEC-SCOPE-1/2 at that moment and the
  ledger says so.
- **CT-1c** does **not** need the scope successor for its v1 tradition-set fixture: capability
  trees install per-owner through `InstallTargetSpec`, and v1 payload activation targets the
  install owner (≡ `[This]`/`[Root]`). It needs SPEC-SCOPE-1 (+2) only when a payload or
  prerequisite references another entity (`owner.capital_scope`-class); that is the expected
  first pull.
- **Deferred:** SPEC-SCOPE-3 until the event-chain rung; iterator scopes to the
  iterator/selector item; effect-ordering to its T2 gate; cross-unit mod merging unchanged;
  multi-polity structures unchanged. `fromfrom`-class depth beyond the declared cap stays
  rejected until a consumer demonstrates need.

## 9. Closure answers

1. **Named consumer:** CT-1b+ triggered-modifier scope refs, CT-1c capability payload contexts,
   the future `EmitEvent` payload rung, and `ScopeRef::Slot(u32)` retirement (header).
2. **Accepted atoms:** `This`, `Root`, `From{depth}`, `Step{name}` (registered), `EventTarget{name}`
   (enumerated); `Prev` compile-time only (§1).
3. **`root`/`from`/`fromfrom`/`prev`:** `Root` = install owner or chain origin; `From{n}` = indexed
   read of an explicit bounded from-stack in the boundary payload; `fromfrom` = `From{2}` via
   shift-push, no implicit stack; `prev` = lexical compile-time rewrite, rejected on underflow
   (§1–§3).
4. **Dot paths:** ordered atom chains identical to nested scope changes; terminal
   scope-vs-property split with ambiguity rejection (§1).
5. **`event_target:`:** `EventTarget{name}` atoms over an admission-enumerated, boundary-bound
   named-target map (§3.3).
6. **`ScopeRef` replacement:** `ScopeChainRef { atoms }` on authored surfaces; `Current` ≡ `[This]`;
   `Slot(u32)` retired from authoring, slots remain driver-internal compiled bindings (§2).
7. **`EmitEvent` payload:** GPU payload unchanged; `EventScopeContext { root, from_stack,
   named_targets }` constructed/consumed at the boundary (§3).
8. **Rejected immediately:** malformed chains, `prev` underflow, ambiguous terminals, dynamic
   names (§4, §7).
9. **Deferred to semantic validation:** transition typing, depth caps, target-name enumeration,
   per-surface legality — at the consuming rung, where the tables exist (§4).
10. **Widening tickets:** SPEC-SCOPE-1 (CT-1c, or CT-1b on demonstrated need), SPEC-SCOPE-2
    (first rung registering a real step), SPEC-SCOPE-3 (the event-chain rung) (§5).
11. **No runtime slots in authored surfaces:** correct — the successor contains no slot, and
    `Slot(u32)` is retired (§2.1).
12. **`simthing-sim` ClauseThing-blind:** yes — flat columns, registrations, `BoundaryRequest`s
    only (§6).
13. **No GPU/WGSL changes:** none claimed, none required (§6).
14. **Runtime path preserved:** field → mask/modifier → threshold → event → `BoundaryRequest`,
    verbatim (§6).
15. **Superseded artifacts:** none existed; no drafts retained.
16. **Visibility artifacts:** none needed — this memo plus the PR body answer closure; no
    `docs/tests` report created.
17. **`cargo test --workspace`:** not run (docs-only rung; no Rust tests run at all).
18. **Ledger honesty:** SCOPE-MEMO row set to DESIGN MEMO / READY FOR REVIEW with a pointer here;
    CT-1b/CT-1c remain NOT STARTED; tickets are *proposed pending acceptance*, not cut.

**Non-goals, restated:** no production code, no `simthing-sim`/`simthing-gpu`/WGSL/driver-runtime
edits, no `simthing-spec` struct edits in this PR, no tests, no lab scans, no CT-1b/CT-1c/CT-2x
work, no Paradox/lab corpus material.
