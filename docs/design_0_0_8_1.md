# SimThing — Design 0.0.8.1 Constitution (Consumer-Pulled Phase, Recursive-Rehearsal Horizon Reached)

> **Status: ACTIVE constitution (synthesized 2026-06-07, design authority Opus).** 0.0.8.1 is the
> single, self-contained design doc for the consumer-pulled phase. It **synthesizes and supersedes** the
> entire prior design lineage — `design_v4.md`, `design_v5.md`, `design_v6.md`, `design_v6.5.md`,
> `design_v7.md` (v7/v7.6 core), `design_v7_7.md`, `design_v7_8.md`, the v7.8/v0.0.7.9 production tracks,
> and `design_0_0_8_0.md` — which are now **archived** under `archive/superseded_design/` as historical
> record only. Future agents should read **this doc** plus the binding homes below; the archived files
> are not required reading and must not be cited as active authority.
>
> **Binding homes are unchanged.** The *structural* invariants live in [`invariants.md`](invariants.md)
> (binding; any change is Tier-2). The *gating* mechanics live in
> [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md) (Tier-1/
> Tier-2 + §6 proven-capability stop rule). The live production status ledger is
> [`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md)
> (kept under its established name to preserve consumer wiring). This doc **rules**; it does not
> re-derive the binding rules — it points at them.

---

## 0. Transient constitution — carry-forward doctrine (MUST propagate to every future version)

> **This section is transient by design and is the cross-version spine.** It holds doctrine that
> outlives any single constitution version. **Every future constitution version MUST copy §0 forward
> verbatim**, amending only by *addition* — never silent removal or weakening. The version-specific
> track, parked inventory, and operating mechanics below §0 are not carried forward automatically; §0
> is. If a future version omits §0, that version is defective. (Carried verbatim from 0.0.8.0.)

### 0.0 Purpose — the unitary vision (why §0 is transient)
Maximal SimThing conformance is in the transient constitution for **one** reason: it is the mechanism
by which **conflict, opportunity, ambition, and extraction collapse into a single generic,
GPU-resident SimThing.** Each is the *same* mechanism wearing a different label —
- **conflict** → combat (`HP/Damage` arena), disruption (decaying accumulator);
- **opportunity** → desirability fields and gradients (where to go);
- **ambition** → faction drives, expansion, fight-or-flight (threshold-gated value decisions);
- **extraction** → resource extraction, raiding, the production/energy economy —

and all of them reduce to: **accumulation/flow, reduced up and masked down the one recursive tree,
resolved by threshold crossings on the resulting field.** There is no combat engine, no economy engine,
no AI engine — there is one *accumulate → reduce → mask → threshold* loop that resolves all of them in
the same GPU pass.

The payoff, and the entire point, is that **resolution lives as GPU automata in a FIELD_POLICY model.**
Decisions — engage/withdraw, move, raid, expand, allocate — are not computed by a CPU planner; they
**emerge as GPU-resident threshold crossings over the resolved, masked field**, exactly as combat,
movement, and engage/withdraw fall out of a single pass (§0.2, §0.3). The moment any behavior is modeled
as a privileged *structural* special-case — the rejected **D=3 ownership-node** is the canonical
example: ownership smuggled back in as a bespoke tree shape instead of a decaying owner overlay — it
leaves the generic substrate, can no longer be resolved as uniform GPU automata, and the unitary vision
breaks. That is why conformance is non-negotiable and carries forward across every version: it is not a
style preference, it is the **precondition** for the whole simulation being one GPU-resident FIELD_POLICY
automaton rather than a federation of bespoke subsystems.

### 0.1 Maximal SimThing conformance (the founding premise)
**Everything is a SimThing.** There are no privileged engine-side special cases for game concepts:
gamesession, factions, worldstate, starmap, star systems, planets, grid cells, fleets, and cohorts are
all SimThings in one recursive `{properties, overlays, children}` tree. New behavior is modeled by
adding SimThings, properties, overlays, and `AccumulatorOp` registrations — **never** by a bespoke
subsystem sitting outside the tree. When a design seems to need special-case logic, the correct move is
almost always to express it as *more SimThing*. (This is the antidote to the math-in-a-vacuum drift that
`invariants.md` "Scenario Proof" now also guards: behavior is proved through the tree, not beside it.)

### 0.2 Allocation is always recursive (overrides the flat-star carve-out)
Resource flow is **one** mechanism: reduction **up** the tree (each parent reduces its children's flow
into a surplus or deficit and passes it to its parent) and disbursement **down** the tree (the
gamesession root sends flow down; factions hold the stockpiled values, resolve deficits from the
stockpile, and the resolved values sweep back up to the root and down again). **"Flat-star" vs "nested"
is not a structural fork:** local balance is simply where a masked flow nets to zero at a *leaf level*
of the one recursive hierarchy — same machinery, different settling depth. This **explicitly overrides**
the "combat is flat-star within the cell, nesting lives only above" carve-out in
`docs/workshop/mobility_and_transfer_allocation.md` §3.2: a cell-local arena is the leaf-most level of
the recursive hierarchy, not a different mechanism.
> *Implementation note (not a weakening of the doctrine):* the flat D=2 (`FlatStarResourceFlow`) slice
> was the original proven production posture; the **recursive galaxy → system → planet → surface ladder
> is now proven** end-to-end on GPU (RUNTIME-0080-RR-0…RR-4, §4A) against a recursive CPU oracle.
> §0.2 is the **target doctrine** and is now demonstrated through the RR ladder; it remains *not* a
> claim that arbitrary depth is wired into the default `SimSession` path. **This note is bounded by §0.6
> (Specification Fidelity):** it permits *honest* parking of depth not yet built — it is **not** a
> licence to close a track against a spec that *calls for* depth while shipping a flat proxy. If a
> governing spec specifies a tier and the implementation collapses it, that is a **Deviation** requiring
> a recorded, approved Deviation Record per §0.6 — never a silent pass.

### 0.3 All conflict is resource flow
**Every adversarial interaction is expressed as resource-flow dynamics** — accumulation, reduction, and
threshold crossings over SimThing participants — never as bespoke conflict logic. Binding named instances:
- **Combat** is an `HP/Damage` resource-flow arena: fleet/ship cohorts are participants; damage is a
  `SubtractFromSource` transfer; HP recovery is `governed_by` integration; a cohort crossing zero HP
  fires `Threshold` + `EmitEvent` → boundary removal.
- **Disruption** is a resource-flow arena whose value accumulates and decays as a location SimThing's
  `disruption` property (the BoundedFeedback recurrence); patrols and pirates are participants
  (suppress / emit); the disruption vector reduces up to the starmap, where it accumulates as the heatmap.

Diplomacy, trade, and any future adversarial system follow the same law. This **supersedes** the
"Combat / Diplomacy / Trade as a Flow arena — out of scope" deferral in
`docs/adr/resource_flow_substrate.md` §"Out of scope": those are now **in scope, as arenas**, by this
directive. The substrate stays semantic-free — these names live at the spec/driver layer and compile
away to generic `AccumulatorOp` registrations; `simthing-sim` never learns the word "combat."

### 0.4 Substrate consequence — endgame scale is never prohibited
Large-scale concurrency (e.g. one cell hosting a very large fleet count at endgame) is **never** solved
by prohibiting scale. The participant cap is on **concurrent** participants (bounded by the global
cohort population), **not** cumulative and **not** cells × capacity. Slots recycle through the REENROLL
free-list — deregister marks a slot inactive (consistent with the no-compaction rule); a new enrollment
reuses a free slot. Pool growth, when the global population itself rises, happens at a **boundary**,
never per tick — the load-bearing "no per-tick device creation" invariant is preserved. Pulling REENROLL
into a production path is the named-consumer gate that opens it.

### 0.5 Track harness discipline — the base every production PR track carries
The §0 drift (math-in-a-vacuum, kind-as-behavior, structural special-cases like the rejected D=3
ownership-node) was a **context-harness failure, not a doctrine failure**: low-context implementation
agents (Codex/Cursor/Grok) had no tight harness, so they re-derived architecture from conventional
priors and drifted. The fix is a **fixed, small, citable base harness on every production PR track** —
and a discipline that keeps it from bloating back into a doc-treadmill no agent reads.

- **Rule 1 — every track opens with a fixed-size harness header citing the high-signal set.** Cite the
  **4–5 most relevant links** (up to **5–6** when an extra genuinely improves outcomes), never more. The
  header always includes (a) this constitution **§0** and (b) the track's **one canonical design file**;
  the rest are the highest-signal design/code surfaces (e.g. the binding invariants, the
  design-of-record for the mechanism, the parked code being pulled). **Every link must be load-bearing** —
  if a reader wouldn't open it on a typical rung, it does not belong in the header; demote it into the
  canonical design file. Plus a **one-screen** "established decisions / do-not-re-derive" checklist. The
  discipline is **high-signal density, not link count**: keep the header to one screen and prune anything
  a low-context agent wouldn't actually use.
  - **Two layers — fixed base + rung-local.** The 4–6 links above are the **fixed base harness**: the
    same set on *every* handoff — capped, durable, the anchor against drift. A handoff MAY add a short
    **rung-local** citation list — the artifacts *that* rung directly consumes (the immediately-upstream
    rung's test report / status row, a bespoke note, the prior result it builds on). Rung-local rules:
    (a) only what this rung directly consumes (a reader opens it *for this task*, not generally);
    (b) keep it to **≤ 3**; (c) it is **ephemeral** — it does not carry to the next handoff and **never
    accretes into the fixed base**. The base controls drift; the rung-local layer carries task detail.
    If a rung-local link proves durable across rungs, **promote it into the canonical design file** —
    do not grow the base.
- **Rule 2 — every rung handoff cites the harness and self-checks the diff against the base principles
  below**, stating in one line that the change holds them. A handoff that cannot cite the harness is rejected.
- **Rule 3 — link out, never inline.** Detail lives in the canonical design file and the linked code;
  the header points, it does not restate. Restating is what overburdened the old tracks.

**The base SimThing principles — the harness checklist, every track, every rung:**
1. **Everything is a SimThing** (§0.1). New behavior = SimThings + properties + overlays + `AccumulatorOp`
   registrations — never a subsystem outside the tree, never a runtime `match kind`.
2. **All conflict/opportunity/ambition/extraction is resource flow** (§0.0, §0.3):
   `accumulate → reduce → mask → threshold`. No combat / economy / AI engine.
3. **Allocation is recursive; settling depth is emergent** (§0.2). Reduce-up / disburse-down through the
   one tree. No per-relation depth assignment, no flat-star special case.
4. **Decisions are GPU-resident threshold crossings — FIELD_POLICY, not a CPU planner** (§0.0, §2.6):
   `Threshold` + `EmitEvent` → `BoundaryRequest`.
5. **`simthing-sim` is semantic-free; exact claims carry CPU-oracle bit-exact parity** (§2.6). Semantics
   compile away to flat `AccumulatorOp` / overlay / threshold registrations.
6. **Proven only through a real reduction** (`invariants.md` "Scenario Proof"); **opt-in / default-off**,
   no default wiring without a gate. A CPU math module is an oracle, never the proof.

If a change cannot be expressed within 1–6, that is the signal to **escalate to design authority** — not
to add a special case. The checklist is six lines on purpose: a low-context agent will hold six lines and
drift past sixty.

### 0.6 Specification Fidelity — no silent flattening (the anti-drift law)
> **Added 2026-06-07 by design authority on direct product mandate (draconian), after a specified
> recursive structure — galaxy → star-system subgrid → planet → planet-surface, with pop cohorts and
> factory/building children on surface cells — was silently flattened into a single galactic-tier proxy
> and then closed as a PASS rehearsal. This MUST NOT recur. Like the rest of §0, this carries forward
> verbatim, amended only by addition.**

**What gets specified gets implemented — or the deviation is recorded in the open and approved.** An
implementation may **not** silently substitute a flattened, collapsed, or proxy structure for a
specified recursive/structured design and then claim it IMPLEMENTED / PASS / CLOSED. The bindings:

1. **No silent tier collapse.** When a spec defines a containment hierarchy (parent → child tiers /
   gridcells / surfaces / building-children), an implementation may not collapse those tiers into a flat
   proxy and claim the spec is met. Modelling 13 star-systems as flat galactic cells when the spec calls
   for 13 × 10×10 system subgrids each with a 10×10 planet surface and pop/factory children is a
   **collapse**, not an implementation.
2. **Deviation Record or it did not pass.** Any gap between the governing spec and what was built is a
   **Deviation** that MUST be written at the top of the track's results doc, enumerating every specified
   element not implemented, the proxy used in its place, the reason, and the consumer impact — and MUST be
   explicitly approved by design authority. A PASS / CLOSED that lacks a required Deviation Record for a
   real gap is **constitutionally VOID** and the track is reopened.
3. **Scope Ledger on every closure.** Every CLOSE / ACCEPT ruling MUST carry a *Specified vs Implemented*
   ledger: each spec element marked `implemented` / `proxied` / `deferred` / `parked`, with evidence. A
   closure without a complete ledger is invalid.
4. **"Parked / not-yet-wired" is a Deviation, never a free pass.** The §0.2 implementation note ("the
   proven slice today is D=2; deeper recursion is parked, pulled by a named scenario") is **not** a
   licence to close a track against a richer spec while shipping a flat proxy. Parking a specified tier is
   itself a Deviation that must be recorded and approved per (2); it may never be left implicit.
5. **Hygiene theater is void.** Progress is **working, spec-faithful code under test** — never the count
   of memos, packets, reviews, status rows, reports, or harness ceremony. Documents *record* progress;
   they never *constitute* it. A PASS / CLOSED asserted via documentation churn, report-only aggregation
   of predecessor artifacts, or harness ceremony in place of the specified consumer actually running is
   void (sibling of `invariants.md` "Scenario Proof" and the gating policy §6 stop rule, now binding for
   closure). "Project-management cosplay" — activity that produces governance artifacts instead of the
   specified feature — is the failure mode this law exists to kill.

Binding enforcement lives in `invariants.md` → "Specification Fidelity & Anti-Ceremony". This §0.6 is the
doctrine; the invariant table is the gate.

---

## 1. Design lineage v4 → 0.0.8.1 (consolidated; archived sources)

This doc folds the whole versioned design history into one place. The predecessors are archived under
`archive/superseded_design/` and are **historical record only**:

- **v4 / v5 / v6 / v6.5** (`archive/superseded_design/design_v4.md` … `design_v6.5.md`) — the early
  SimThing tree, GPU pipeline, and economy-substrate formation. Superseded wholesale by v7.
- **v7 / v7.6** (`design_v7.md`) — the v7 constitution: the recursive `{properties, overlays, children}`
  SimThing type, the GPU pipeline that replaced v6 §10, the economic substrate, EML integration, and the
  performance model. This is the architectural floor every later version inherits.
- **v7.7** (`design_v7_7.md`) — the closed constitutional baseline amendment.
- **v7.8** (`design_v7_8.md`) + its production track (`design_v7_8_production_track.md`) — bounded-posture
  expansion; origin of the §2.6 non-negotiable rigor list (CPU-oracle parity, semantic-free sim,
  opt-in/default-off, artifact-backed exact authority, no CPU planner).
- **0.0.7.9** (`design_v7_9_mobility_transfer_allocation_production_track.md`) — the mobility/transfer
  allocation ladder (ALLOC, REENROLL, IDROUTE, ECON, OWNER + RUNTIME-0/1A/1B + semantic-free GPU kernel
  substrate), all proven, opt-in, default-off, and **parked**. Origin of the anti-loop discipline (§2.5).
- **0.0.8.0** (`design_0_0_8_0.md`) — the consumer-pulled redirection: stop building substrate ahead of
  consumers; build the named scenario that pulls one parked substrate into production. Author of §0.6
  Specification Fidelity after the silent-flattening incident. Its §0 and operating doctrine are carried
  forward here verbatim (§0, §2).

**Nothing in this section is binding** beyond pointing at where each idea came from; the binding doctrine
is §0, the operating mechanics are §2, and the structural invariants live in `invariants.md`.

### 1.1 What the consumer-pulled phase is — and the lesson it encodes
By the end of 0.0.7.9 the project had built a **large, correct, proven, and entirely parked** substrate,
consumed by nothing in production, and building it generated repeated **hygiene loops**. The governing
stance since 0.0.8.0: **stop building substrate ahead of consumers; build the consumer that names what to
wire.** Every parked substrate is gated on a *named product scenario*. This is not a new architecture; it
is a **redirection of effort** from substrate-ahead-of-need to consumer-pulled integration, under the
same two-track fastlane and the same non-negotiables.

---

## 2. Operating doctrine (consolidated — read before any 0.0.8.x work; carried verbatim from 0.0.8.0)

### 2.1 Guardrails live at the designer-facing barrier
Two-layered (`invariants.md` "Guardrail placement is two-layered"): the **RON/designer/spec layer owns
expressive policy and rejects unsafe *authoring* at import**; the **runtime enforces hard safety
unconditionally** as the last line. As work moves toward the designer surface, guardrails **relocate to
spec admission** — they do not disappear and they do not stay as hard-coded runtime/fixture
special-cases. This is the optimal home: reject early with good diagnostics; the runtime catches
anything that slips.

**Demotion (2026-06-02, design authority): admission rejection is a guardrail, not a proof.** A scenario
demonstrates correctness by running its behavior through a **real SimThing reduction** (`invariants.md`
"Scenario Proof"), never by accumulating rejection assertions. Reject-bad-authoring tests are a thin net
around a scenario that is *already* proven through the engine — they are not its primary evidence, and a
scenario must never re-litigate the standing prohibition list as a matrix of per-module local flags.

### 2.2 Two-track fastlane (Tier-1 / Tier-2)
Classify every change first (`phase_m_gating_and_doc_policy.md` §1).
- **Tier-1 fast lane** — within an accepted design, generic/semantic-free, opt-in/default-off,
  CPU-oracle-parity-backed, reversible. Ships as **one implementation PR + one test report + one
  status-row update.** No separate design-review/acceptance/parking/R-series ceremony.
- **Tier-2 gated** — touches a binding invariant, introduces default-on / default `SimSession`
  pass-graph wiring, is new architecture / an open design question, or is on the standing prohibition
  list. Keeps the full design-review → acceptance → implementation cadence. Closure/acceptance memos
  are **design-authority + product only**.

### 2.3 The WGSL ban is on *semantic* WGSL only
There is no blanket WGSL ban. Banned: **semantic WGSL** (gameplay/map/faction/AI concepts in shader
text). **Generic, semantic-free shader extensions are admissible** when (a) they carry no
map/faction/AI semantics, (b) they are paired with **CPU-oracle parity**, and (c) their meaning is
pinned entirely at the designer/spec admission layer (the shader sees only floats/indices). New generic
WGSL is a Tier-2 gate, not a prohibition (precedents: AO-WGSL-0, the artifact-backed exact `sqrt`, and
the 0.0.7.9 semantic-free mobility column kernel).

### 2.4 EML gadgets / formula classes are admissible at the designer layer
Gadgets compile to a postfix subgraph over the **existing** `EvalEML` opcode set — **no new WGSL, no
per-gadget kernel, no new opcode.** Formula classes and gadgets are admitted at the RON/designer/spec
layer with mandatory CPU-oracle parity and a bounded-feedback contract for recurrent gadgets.

The "no new opcode" rule binds the **gadget authoring layer** (gadgets are macros over the fixed
interpreter vocabulary). It is **not** a blanket prohibition on extending the generic interpreter itself.
Extending the substrate vocabulary — a new **generic, semantic-free** `EvalEML` opcode, an
`AccumulatorOp` combine function, or a generic kernel — follows §2.3 exactly: it is a **Tier-2 gate, not
a prohibition**, admissible when (a) it carries no map/faction/AI/scenario semantics (the interpreter
sees only floats/indices; admission rejects semantic names), (b) it is paired with **CPU-oracle bit-exact
parity**, (c) its meaning is pinned entirely at the spec/designer admission layer, and (d) it is
reusable by any SimThing rather than encoding one scenario's rules. A new opcode that hard-codes a
specific scenario's behaviour (e.g. an "R6C economy" op) is **semantic** and stays banned; behaviour must
remain expressed as **data** (EML programs / column params / op registrations) over a generic instruction
set. Rung/handoff stop-lines that read "no new op / no new WGSL" are **scheduling hygiene**, not
constitutional bans, and may be narrowed by design authority to the §2.3/§2.4 gate when an honest
result depends on a generic substrate extension. This never relaxes the *semantic*-freeness guarantee,
the CPU-oracle parity requirement, or any anti-faking/measurement discipline a rung imposes.

### 2.5 Anti-loop discipline (the 0.0.7.9 lessons, now constitutional)
Three rules, each a single principle covering its whole class:
- **No opening-review treadmill** (0.0.7.9 §2.1). Once a class of work is classified Tier-1 fast-lane,
  it ships directly. Do not spawn a per-slice opening-review PR for CPU/default-off/test-fixture work
  that re-asks an already-settled principle.
- **Proven-capability stop rule** (`phase_m_gating_and_doc_policy.md` §6). Once a capability is
  reasonably proven (substrate floor green + one representative scale/parity run), **further variants
  that only recombine the proven primitive are not authorized** (more soaks/replays/permutations,
  Nth scale runs, accounting-over-accounting). The only authorized next moves are: **(1) close the
  dangling path**, or **(2) escalate one paragraph toward the next distinct gate** (and stop if it is
  Tier-2). *Proven once, not proven N times.*
- **One principle per class — no per-slice accretion.** A constraint is stated once; a change that adds
  an Nth restatement of an existing principle is rejected as redundant. Active docs carry a **compact
  status table**; per-slice narrative is a one-line worklog entry, never an accreting litany.
- **No per-scenario `Gate`/`Surface`/`ForbiddenRequests` boilerplate (retired 2026-06-02, design
  authority).** The convention of cloning a `Gate`/`Surface`/`ForbiddenRequests` triple per scenario —
  re-declaring the standing prohibition list as struct fields and asserting each rejection per module —
  is **retired**. The standing prohibitions live once (gating policy §2 + `invariants.md`); a scenario
  does not re-encode them as local flags. Opt-in/default-off is expressed at the accepted
  spec-admission layer, and a scenario's evidence is its **reduction** (`invariants.md` "Scenario
  Proof"), not a forbidden-flag matrix. This was the literal self-feeding boilerplate that let scenario
  tracks ship as math-in-a-vacuum.

### 2.6 Non-negotiable rigor (relaxation never touches these)
Verbatim from v7.8 §2.5 — binding regardless of any relaxation above:
- **CPU-oracle bit-exact parity** where a kernel claims `ExactDeterministic`; honest classification
  (`ApproximateJitOnly` / `ReplayAccepted` / `GpuVerified`) otherwise — no overclaiming.
- **`simthing-sim` stays semantic-/arena-/map-/Gadget-/Personality-free.** All semantics compile away
  at the spec/driver layer to flat `AccumulatorOp` / overlay / threshold registrations.
- **Opt-in / default-off.** No default `SimSession` wiring, scheduler, cache, or production bridge
  without its own named gate.
- **Exact authority is artifact-backed and proof-gated** (the `sqrt` Candidate F precedent).
- **No CPU planner / CPU urgency / CPU commitment emission.** AI is a SimThing: decisions are
  GPU-resident threshold crossings; the structural path is `Threshold` + `EmitEvent` → `BoundaryRequest`.

---

## 3. Parked capability inventory (proven, default-off, awaiting a consumer)

Everything below is **complete or accepted at its first slice, opt-in/default-off, reversible, and
parked.** None is "in progress." Each is ready to be pulled into production by a named scenario.

| Capability | State | Pulls into production when… |
|---|---|---|
| **0.0.7.9 mobility/transfer substrate** (ALLOC, REENROLL, IDROUTE, ECON, OWNER + RUNTIME-0/1A/1B + semantic-free GPU kernel substrate) | COMPLETE + PARKED. [`design_v7_9_mobility_transfer_allocation_production_track.md`](archive/superseded_design/design_v7_9_mobility_transfer_allocation_production_track.md) (archived) | a named scenario needs mobility/ownership/economy in the default `SimSession` path |
| **Recursive galaxy→system→planet→surface ladder** (RUNTIME-0080-RR-0…RR-4) | **PROVEN + CLOSED** as integrated recursive 100-tick GPU rehearsal vs recursive CPU oracle (§4A). Opt-in/default-off. | a named scenario wires recursive economy into the default `SimSession` path, or extends it (multi-faction / richer emergence) |
| **Line A — nested Resource Flow (A-0)** | ACCEPTED, static nested first slice; flat posture superseded by the proven RR ladder. | a named economy needs depth>2 nested fanout in the default path |
| **Line B — discrete hard-currency ordering (B-0)** | ACCEPTED, narrow smoke; no B-1. | a named multi-transaction hard-currency workload |
| **Line C — atlas / multi-theater mapping (C-0/C-1/C-2)** | ACCEPTED, **map batching CLOSED at the designer surface.** | a named multi-theater scenario opens the atlas production runtime gate (M-4A; §4) |
| **simthing-spec → CLAUSE-SPEC (L0/L1/L2)** | ACCEPTED designer-admission substrate. | it is the *engine* of the next track (§5) |
| **ClauseThing / ClauseScript (L3)** | PARKED pending product authorization. | product authorizes the front-end (§5) |
| **Deferred-by-design** | E-11B-5, atlas production runtime (M-4A), B-1, Hybrid-Strata/faction-index ECON scaling, FrontierV2-5, ACT/EVENT/OBS/PIPE | each behind its own named scenario; none is an open question (§4) |

---

## 4. Closed questions (no dangling open design questions remain)

**E-11B / E-11B-5 (nested Resource Flow dynamic enrollment) — CLOSED.** A-0 proved static nested
materialization; dynamic enrollment is folded into the proven-and-parked 0.0.7.9 REENROLL bilateral
arena re-enrollment substrate (deregister-from-origin + register-into-destination, atomic-or-reject, no
compaction, deterministic). Nested dynamic enrollment, if ever named, is a scenario that pulls the parked
re-enrollment + nested-RF substrate, not new architecture.

**M-4 / atlas (multi-theater mapping) — designer surface CLOSED; production runtime is a parked gate.**
C-0/C-1/C-2 closed map batching at the designer surface (`request_atlas_batching` admits bounded
algebraic-G=0, homogeneous-square, protocol-oracle-backed specs within the active `V78AtlasVramBudget`).
The atlas production runtime / sparse-residency scheduler (M-4A territory) is a parked gate behind a named
multi-theater scenario. There is no open M-4 *question*; there is a parked gate awaiting a consumer.

**Net: no dangling open questions remain on the M/E/T or mobility tracks.** All are accepted at a first
slice, closed at the designer surface, or parked behind a named scenario.

### 4A. Recursive rehearsal horizon — REACHED (RUNTIME-0080-RR ladder CLOSED)

The recursive rehearsal track opened to discharge §0.6 after the flat-proxy incident. It is **closed**;
the recursive rehearsal horizon is **reached**.

```text
RR-0 recursive world + 100-tick recursive CPU oracle
 → RR-1 nested sparse residency (galaxy 20×20 resident; system/surface materialize on descend)
 → RR-2 planet-surface pop→factory labor economy on GPU (bit-exact vs RR-0)
 → RR-3 recursive GPU reduce-up / disburse-down (surface→planet→system→galaxy→stockpile→starport)
 → RR-4 integrated recursive 100-tick GPU rehearsal (one persistent GPU session; per-tick + final
        parity vs RR-0 recursive CPU oracle; horizon reached)
```

**RUNTIME-0080-RR-4 — ACCEPTED / CLOSED (design authority, 2026-06-07).** RR-4 runs the integrated
recursive consumer for 100 ticks on one persistent `AccumulatorOpSession`, carries stockpile/starport
state forward on the GPU between ticks, and proves per-tick and final-state bit-exact parity against the
RR-0 recursive CPU oracle (Terran stockpile 400, Pirate 100; 200 tick-parity rows). It consumes RR-0
world/oracle, RR-1 nested residency, RR-2 GPU surface economy, and RR-3 recursive transfers as
mechanisms — no flat surface→faction shortcut in the PASS path, no tier collapse, no report-only close.
Closure carries a complete Scope Ledger (rows 1–30 implemented; 31–33 non-claims) with no Deviation
Record required. Evidence: [`tests/runtime_0080_rr_4_results.md`](tests/runtime_0080_rr_4_results.md);
status ledger in the production track.

**Non-claims at closure (carried from the RR-4 ruling):** no generalized multi-faction economy beyond
the Terran/Pirate fixture; no default `SimSession` / session wiring; no standalone M-4A parallel-theater
track; no `invariants.md` edit; no RUNTIME-0080-0 reopen; no pinned-number change. These are the
**future-consumer surface**, not blockers.

**Open findings (non-blocking, recorded for the next consumer):** RR-4's inline cross-owner / cross-tier
negative controls are weaker than RR-3's (an isolated single tick compared to a mid-run accumulated
oracle value); the substantive cross-owner/cross-tier proofs live in the consumed RR-3 ladder. Most RR-4
tests assert report flags that are themselves computed from genuine per-tick GPU-vs-oracle bit
comparisons; a future hardening pass could assert directly over the tick-parity rows.

---

## 5. Production track — the consumer edge (SCENARIO-FIRST)

**Governing stance:** the production track is consumer-pulled scenario authoring, not more substrate. Its
job is to produce **named product scenarios** through the accepted simthing-spec / CLAUSE-SPEC admission
layer (advancing toward the ClauseThing/ClauseScript aspiration, L3) and let each scenario **pull exactly
one parked substrate** into a real production path. A substrate's production-path gate opens **only
because a named scenario consumes it** — never speculatively.

**Scenario gate contract (Tier-2):** a named product-scenario **demonstrates its behavior through a real
SimThing reduction** (`invariants.md` "Scenario Proof") — an opt-in test that constructs real
`SimThing` / `SimProperty` / `Overlay` state and advances it through `BoundaryProtocol` (or the accepted
spec→`AccumulatorOp` lowering), pulling **exactly one** parked substrate and asserting behavior on the
resolved GPU/CPU values. A standalone CPU math module is an **oracle** for the parity check, **never** a
substitute for the reduction. Standing stop conditions (reject the scenario if it requires any):
owner-entity as spatial parent; capture-as-reparenting; semantic/raw WGSL; default-on without a gate;
hard-currency through Resource Flow; a CPU planner/urgency/commitment; or reopening a closed ladder
(atlas runtime, E-11B-5, B-1, ClauseThing/L3 front-end, FrontierV2-5, ACT/EVENT/OBS/PIPE) without its
own product authorization.

**Live status ledger:** the per-rung status, exit criteria, and worked harness examples live in
[`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md)
(kept under its established name to preserve consumer wiring). This §5 is the doctrine; that doc is the ledger.

### 5.1 Candidate next consumers after the RR closure (none auto-opened)
With the recursive rehearsal horizon reached, the next named consumer is a product/design-authority
choice. Candidates (open exactly one):
- **`SCENARIO-0080-3`** — richer emergence on the recursive GPU runtime (recommended default).
- **`ECON-0080-MULTIFACTION`** — multi-faction economy generality beyond the Terran/Pirate fixture.
- **`SESSION-0080-RUNTIME`** — default/session integration if the project wants runtime packaging.
- **`M4A-NESTED-SCALE`** — multi-atlas / nested sparse-residency scale, only if a specific consumer needs it.
- **`MOVEMENT-FRONT-0080-RECURSIVE`** — recursive movement / disruption-front (suppression) behavior over the nested runtime.

**L3 ClauseThing/ClauseScript** remains the long-horizon aspiration this track advances toward; its
parser/front-end and any production `SimSession` wiring stay Tier-2 and product-authorized.

---

## 6. Pointers
- Binding structural rules: [`invariants.md`](invariants.md)
- Gating mechanics + anti-loop + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Live production status ledger: [`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
- Workshop architecture record: [`workshop/mobility_and_transfer_allocation.md`](workshop/mobility_and_transfer_allocation.md)
- Recursive rehearsal closure evidence: [`tests/runtime_0080_rr_4_results.md`](tests/runtime_0080_rr_4_results.md)
- Archived design lineage (historical only): `archive/superseded_design/`
