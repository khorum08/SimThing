# SimThing — Design 0.0.8.1 Constitution (Consumer-Pulled Phase, Recursive-Rehearsal Horizon Reached)

> **Status: ACTIVE constitution (synthesized 2026-06-07, design authority Opus; editorially
> compressed 2026-06-09 under DOCS-TRIM-0, product-mandated — all binding doctrine preserved,
> history relocated to reports/archive).** 0.0.8.1 is the single, self-contained design doc for the
> consumer-pulled phase. It **synthesizes and supersedes** the entire prior design lineage (§1),
> archived under `archive/superseded_design/` as historical record only. Future agents read **this
> doc** plus the binding homes below; archived files must not be cited as active authority.
>
> **Binding homes are unchanged.** Structural invariants: [`invariants.md`](invariants.md)
> (binding; any change is Tier-2). Gating mechanics:
> [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
> (Tier-1/Tier-2 + §6 proven-capability stop rule). Live production status ledger:
> [`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md).
> The permanent paradigm reference beneath this constitution:
> [`simthing_core_design.md`](simthing_core_design.md). This doc **rules**; it does not re-derive
> the binding rules — it points at them.

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
> *Status note:* the recursive ladder is proven end-to-end on GPU against a recursive CPU oracle
> (RUNTIME-0080-RR-0…RR-4, §4A); §0.2 is demonstrated doctrine, not yet default `SimSession`
> wiring. Bounded by §0.6: parking specified depth is honest only as a recorded, approved
> Deviation — never a silent flat proxy.

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
agents had no tight harness, so they re-derived architecture from conventional priors and drifted. The
fix is a **fixed, small, citable base harness on every production PR track**:

- **Rule 1 — every track opens with a fixed-size harness header citing the high-signal set.**
  Fixed base = 4–6 durable, load-bearing links (always: this constitution **§0**, the track's
  **one canonical design file**, and [`simthing_core_design.md`](simthing_core_design.md)). A
  handoff may add ≤3 rung-local links it directly consumes; rung-local links are ephemeral and
  never accrete into the base — promote durable ones into the canonical design file. Plus a
  **one-screen** "established decisions / do-not-re-derive" checklist. High-signal density, not
  link count.
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
> recursive structure was silently flattened to a flat galactic-tier proxy and closed as a PASS
> rehearsal. Like the rest of §0, this carries forward verbatim, amended only by addition.**

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
4. **"Parked / not-yet-wired" is a Deviation, never a free pass.** The §0.2 status note is **not** a
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

### 0.7 Exact numeric authority for decision gates

Decision-critical gradient magnitude, movement-front magnitude, threshold magnitude, and similar
parity-sensitive scalar gates must use artifact-backed exact primitives. Raw f32 magnitude, WGSL `sqrt`,
`length`, `distance`, `normalize`, `hypot`, or Rust native sqrt-like operations are
`ApproximateDiagnostic` only and may not gate commitments.

The current exact-authoritative chain is:

fixed-point `dx/dy`
→ exact pre-sqrt mag2 (`m_jit_mag2_fixed_exact` / `ExactFixedPointDxDy`)
→ Candidate F sqrt (`m_jit_mag_f_from_exact_mag2`, artifact hash `59ab4b2892e3c690`, LF-canonical
re-pin 2026-06-11, `SQRT-REPIN-0`)
→ exact Euclidean magnitude
→ threshold.

Any GPU-resident sqrt, magnitude, distance, gradient norm, movement-front norm, threshold path, or
parity-sensitive exact path must route through Candidate F or another explicitly admitted
artifact-backed exact primitive. Native sqrt-like paths may exist only as diagnostics.

Historical R4 ledger detail remains in
[`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md);
§0.7 is the carry-forward constitutional rule.

---

## 1. Design lineage v4 → 0.0.8.1 (non-binding archaeology; sources archived)

| Version | Archived file (`archive/superseded_design/`) | One-line contribution |
|---|---|---|
| v4–v6.5 | `design_v4.md` … `design_v6.5.md` | early SimThing tree, GPU pipeline, economy-substrate formation |
| v7 / v7.6 | `design_v7.md` | the architectural floor: recursive `{properties, overlays, children}` + AccumulatorOp pipeline + EML |
| v7.7 | `design_v7_7.md` | closed constitutional baseline amendment |
| v7.8 | `design_v7_8.md` + production track | bounded-posture expansion; origin of the §2.6 non-negotiables |
| 0.0.7.9 | `design_v7_9_mobility_transfer_allocation_production_track.md` | mobility/transfer substrate, all proven + parked; origin of anti-loop (§2.5) |
| 0.0.8.0 | `design_0_0_8_0.md` | consumer-pulled redirection; author of §0.6 after the silent-flattening incident |

**Nothing in this section is binding.**

### 1.1 The consumer-pulled lesson
By the end of 0.0.7.9 the project had built a large, correct, proven, **entirely parked** substrate
consumed by nothing in production, and building it generated repeated hygiene loops. Governing stance
since 0.0.8.0: **stop building substrate ahead of consumers; build the named scenario that pulls one
parked substrate into production** — same two-track fastlane, same non-negotiables.

---

## 2. Operating doctrine (consolidated — read before any 0.0.8.x work; carried verbatim from 0.0.8.0)

### 2.1 Guardrails live at the designer-facing barrier
Two-layered (`invariants.md` "Guardrail placement is two-layered"): the **RON/designer/spec layer owns
expressive policy and rejects unsafe *authoring* at import**; the **runtime enforces hard safety
unconditionally** as the last line. As work moves toward the designer surface, guardrails **relocate to
spec admission** — they do not disappear and they do not stay as hard-coded runtime/fixture
special-cases. Reject early with good diagnostics; the runtime catches anything that slips.

**Demotion (2026-06-02): admission rejection is a guardrail, not a proof.** A scenario demonstrates
correctness by running its behavior through a **real SimThing reduction** (`invariants.md` "Scenario
Proof"), never by accumulating rejection assertions. Reject-bad-authoring tests are a thin net around a
scenario already proven through the engine — never its primary evidence — and a scenario must never
re-litigate the standing prohibition list as a matrix of per-module local flags.

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

The "no new opcode" rule binds the **gadget authoring layer** only. Extending the generic interpreter
itself (a new semantic-free `EvalEML` opcode / `AccumulatorOp` combine function / generic kernel) is a
**Tier-2 gate, not a prohibition**, under the §2.3 conditions; the binding text lives in
`invariants.md` → EML Gadget Library. Rung/handoff "no new op / no new WGSL" stop-lines are scheduling
hygiene, narrowable by design authority to that gate — never relaxing semantic-freeness, parity, or
any anti-faking discipline a rung imposes.

### 2.5 Anti-loop discipline (the 0.0.7.9 lessons, now constitutional)
- **No opening-review treadmill.** Once a class of work is classified Tier-1 fast-lane, it ships
  directly. Do not spawn a per-slice opening-review PR for CPU/default-off/test-fixture work that
  re-asks an already-settled principle.
- **Proven-capability stop rule** (`phase_m_gating_and_doc_policy.md` §6). Once a capability is
  reasonably proven (substrate floor green + one representative scale/parity run), **further variants
  that only recombine the proven primitive are not authorized**. The only authorized next moves:
  **(1) close the dangling path**, or **(2) escalate one paragraph toward the next distinct gate**
  (and stop if it is Tier-2). *Proven once, not proven N times.*
- **One principle per class — no per-slice accretion.** A constraint is stated once; an Nth restatement
  is rejected as redundant. Active docs carry a **compact status table**; per-slice narrative is a
  one-line worklog entry, never an accreting litany.
- **No per-scenario `Gate`/`Surface`/`ForbiddenRequests` boilerplate (retired 2026-06-02).** Standing
  prohibitions live once (gating policy §2 + `invariants.md`); a scenario's evidence is its
  **reduction**, never a forbidden-flag matrix.

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
| **ClauseThing / ClauseScript (L3)** | **AUTHORIZED 2026-06-10 — the `CT-` track is OPEN** ([`design_0_0_8_1_clausething_production_track.md`](design_0_0_8_1_clausething_production_track.md)). | the open front-end track (§5) |
| **Deferred-by-design** | E-11B-5, atlas production runtime (M-4A), B-1, Hybrid-Strata/faction-index ECON scaling, FrontierV2-5, ACT/EVENT/OBS/PIPE | each behind its own named scenario; none is an open question (§4) |

---

## 4. Closed questions (no dangling open design questions remain)

**E-11B / E-11B-5 (nested Resource Flow dynamic enrollment) — CLOSED.** A-0 proved static nested
materialization; dynamic enrollment folds into the proven-and-parked 0.0.7.9 REENROLL bilateral arena
re-enrollment substrate. If ever named, it is a scenario pulling parked substrate, not new architecture.

**M-4 / atlas — designer surface CLOSED; production runtime is a parked gate.** C-0/C-1/C-2 closed map
batching at the designer surface (bounded algebraic-G=0, homogeneous-square, protocol-oracle-backed,
within the active `V78AtlasVramBudget`). The atlas production runtime / sparse-residency scheduler
(M-4A) is a parked gate behind a named multi-theater scenario — a gate awaiting a consumer, not an open
question.

**Net: no dangling open questions remain** on the M/E/T or mobility tracks.

### 4A. Recursive rehearsal horizon — REACHED (RUNTIME-0080-RR ladder CLOSED)

```text
RR-0 recursive world + 100-tick recursive CPU oracle
 → RR-1 nested sparse residency (galaxy 20×20 resident; system/surface materialize on descend)
 → RR-2 planet-surface pop→factory labor economy on GPU (bit-exact vs RR-0)
 → RR-3 recursive GPU reduce-up / disburse-down (surface→planet→system→galaxy→stockpile→starport)
 → RR-4 integrated recursive 100-tick GPU rehearsal (one persistent GPU session; per-tick + final
        parity vs RR-0 recursive CPU oracle; horizon reached)
```

**RUNTIME-0080-RR-4 — ACCEPTED / CLOSED (design authority, 2026-06-07):** integrated recursive
100-tick GPU rehearsal on one persistent `AccumulatorOpSession`, per-tick and final-state bit-exact
against the RR-0 recursive CPU oracle; no flat shortcut in the PASS path, no tier collapse, complete
Scope Ledger, no Deviation Record required. Non-claims at closure and open findings (negative-control
hardening candidates) live in the closure report:
[`tests/runtime_0080_rr_4_results.md`](tests/runtime_0080_rr_4_results.md).

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
(atlas runtime, E-11B-5, B-1, FrontierV2-5, ACT/EVENT/OBS/PIPE) without its
own product authorization. (ClauseThing/L3 left this list on 2026-06-10 — it now holds product
authorization via the open `CT-` track.)

**Live status ledger:**
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

**L3 ClauseThing/ClauseScript** received product authorization on 2026-06-10: the **`CT-`
production track is OPEN** at
[`design_0_0_8_1_clausething_production_track.md`](design_0_0_8_1_clausething_production_track.md)
(parser-first determination; crate `simthing-clausething`; consumer: the Stellaris/Clausewitz-engine
grand-strategy audience). Production `SimSession` wiring remains Tier-2 and separately
product-authorized.

---

## 6. Pointers
- Permanent paradigm reference: [`simthing_core_design.md`](simthing_core_design.md)
- Binding structural rules: [`invariants.md`](invariants.md)
- Gating mechanics + anti-loop + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Live production status ledger: [`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
- Recursive rehearsal closure evidence: [`tests/runtime_0080_rr_4_results.md`](tests/runtime_0080_rr_4_results.md)
- PALMA semiring pathfinding integration guide (PATH-0): [`design_0_0_8_1_palma_pathfinding_integration_guide.md`](design_0_0_8_1_palma_pathfinding_integration_guide.md)
- PALMA / min-plus traversal: **production GPU utility seated** (PATH-7) with **GPU-native W input / resident D output** (PATH-8), **no public `tick()` scaffold** (PATH-8R), **no public PALMA legacy aliases** (PATH-8R-CLEAN), and **downstream GPU D probe smoke** (PATH-9). Runtime API is generic `min_plus_traversal_field` + `MinPlusTraversalDProbeOp` — explicit `dispatch_gpu_resident` + `TraversalFieldGpuInput`; compact probe readback only for downstream consumer assertions. CPU shadow gather/full-D readback only via named diagnostic/compatibility dispatch. PATH-0–5 proof/benchmark rungs; PATH-6 opt-in band scheduling (**default `SimSession` tick not wired**). **No pathfinding engine or movement policy.** Fable handoff: [`tests/palma_path_9_downstream_gpu_consumer_results.md`](tests/palma_path_9_downstream_gpu_consumer_results.md). Guide: [`design_0_0_8_1_palma_pathfinding_integration_guide.md`](design_0_0_8_1_palma_pathfinding_integration_guide.md)
- BH / saturating-flux operator: **BH-0 IMPLEMENTED / PASS** — generic GPU `SaturatingFlux` stencil with transient register-local C, symmetric `0.5*(C_i+C_j)` flux, zero-flux boundaries, opt-in/default-off admission (`RegionFieldOperatorSpec::SaturatingFlux { u_sat, chi, choke_output_col }`, `chi ≤ 0.25`). **BH-1 IMPLEMENTED / PASS** — optional GPU-resident choke readout column `1 − C/χ` in the same dispatch. **BH-1R IMPLEMENTED / PASS** — compact GPU `SaturatingFluxChokeThresholdOp` (4-float readback). **BH-1R-SCALE IMPLEMENTED / PASS** — staged parallel reduction (256-thread pass 1 + partial fold pass 2; no single-lane full-grid scan). **BH-2A IMPLEMENTED / PASS** — named consumer `CT-4b_Local_Automata_W_Feedstock` opens BH-2. **BH-2B IMPLEMENTED / PASS** — generic GPU `WImpedanceComposeOp` (linear `base_w + weight_a*choke_a + weight_b*choke_b` per profile; admission via `WImpedanceComposeSpec`; bridge `compiled_w_impedance_compose_to_gpu_config`). **BH-2S IMPLEMENTED / PASS** — generic GPU `StressComposeOp` (overlap/mismatch/weighted/velocity stress algebra; admission via `StressComposeSpec`; bridge `compiled_stress_compose_to_gpu_config`; max 4 input field columns, max 8 profiles). **BH-2S-API-DOC DOCUMENTED / PASS** — consumer service-surface handoff (§11). **BH-2C IMPLEMENTED / PASS** — composed W feeds PALMA `GpuInterleavedW` → resident D + compact probe; live API `composed_w_min_plus_stencil_config`. **BH-2D IMPLEMENTED / PASS** — CT-4b 200×200 fixture proof over generic source-family fields; full resident feedstock chain; test fixture quarantined in `ct4b_field_fixture.rs`. **BH-2D-OBS-100R OBSERVATION / PASS** — 100-tick CT-4b dynamic scenario observation (test-only pulsed/mobile emitters + candidate sampler displacement; compact probe + diagnostic aggregates). CPU oracle test-only; no native sqrt in BH hot paths. No border service, pathfinding engine, movement policy, route/predecessor objects. Handoff: [`tests/bh0_saturating_flux_results.md`](tests/bh0_saturating_flux_results.md), [`tests/bh1_choke_readout_results.md`](tests/bh1_choke_readout_results.md), [`tests/bh1r_choke_consumption_results.md`](tests/bh1r_choke_consumption_results.md), [`tests/bh1r_scale_parallel_reduction_results.md`](tests/bh1r_scale_parallel_reduction_results.md), [`tests/bh2_w_composition_results.md`](tests/bh2_w_composition_results.md), [`tests/bh2s_overlap_stress_results.md`](tests/bh2s_overlap_stress_results.md), [`tests/bh2c_palma_feedstock_results.md`](tests/bh2c_palma_feedstock_results.md), [`tests/bh2d_ct4b_fixture_results.md`](tests/bh2d_ct4b_fixture_results.md), [`tests/bh2d_ct4b_100tick_scenario_observations.md`](tests/bh2d_ct4b_100tick_scenario_observations.md). Track: [`design_0_0_8_1_border_hack_track.md`](design_0_0_8_1_border_hack_track.md) (§11 API surfaces, §12 BH-2C, §13 BH-2D, §14 BH-2D-OBS-100R)
- Archived design lineage (historical only): `archive/superseded_design/`
- **R1-TEST-PURGE (2026-06-11):** Legacy R1* proof-ledger/report/checksum tests were deleted or quarantined from the default workspace gate. Default workspace now retains only fast production-relevant R1* sentinels. Historical proof batteries must not be reintroduced as default tests. Full purge: [`tests/r1_default_workspace_purge_results.md`](tests/r1_default_workspace_purge_results.md)
- **FABLE-REVIEW-FREEZE (2026-06-11):** BH/BH-2 track closed for review; canonical handoff [`tests/fable_review_bh2_track_packet.md`](tests/fable_review_bh2_track_packet.md). No runtime churn in freeze pass.
