# SimThing — Design 0.0.8.0 Constitution (Consumer-Pulled Phase)

> **Status: ACTIVE constitution (opened 2026-06-02, design authority Opus).** 0.0.8.0 is the operating
> constitution for the phase after the 0.0.7.9 mobility/transfer substrate landed and parked. It
> **carries forward and consolidates** the operating doctrine of `design_v7_8.md` and the cadence
> lessons of the 0.0.7.9 track; those files become **historical / superseded as active constitution**
> (their content stays as record).
>
> **Binding homes are unchanged.** The *structural* invariants live in [`invariants.md`](invariants.md)
> (binding; any change is Tier-2). The *gating* mechanics live in
> [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md) (Tier-1/
> Tier-2 + §6 proven-capability stop rule). v7.7 ([`design_v7_7.md`](design_v7_7.md)) stays the closed
> constitutional baseline. This doc **surfaces** the doctrine for immediate visibility and **rules** on
> the parked inventory and the next track — it does not re-derive the binding rules.

---

## 1. What 0.0.8.0 is — and the lesson it encodes

By the end of 0.0.7.9 the project had built a **large, correct, proven, and entirely parked**
substrate: the full mobility/transfer ladder (allocation, re-enrollment, identity routing,
clearinghouse economy, owner overlays, GPU kernel execution) — all green, opt-in, default-off, and
**consumed by nothing in production**. Building it generated repeated **hygiene loops**: opening-review
treadmills and recombination-soak spirals, because the work had no consumer pulling it and kept
filling the permitted default-off space with diminishing-value variants.

**0.0.8.0's governing stance: stop building substrate ahead of consumers. Build the consumer that
names what to wire.** Every parked substrate is gated on a *named product scenario*. The next
production work is therefore the **consumer edge** — authoring the first named product scenario
(through the accepted designer-admission layer) that *pulls* one parked substrate into a real
production path. Substrate is no longer built speculatively; it is wired when a scenario names it.

This is not a new architecture. It is a **redirection of effort** from substrate-ahead-of-need to
consumer-pulled integration, under the same two-track fastlane and the same non-negotiables.

---

## 2. Operating doctrine (consolidated — read before any 0.0.8.0 work)

### 2.1 Guardrails live at the designer-facing barrier
Two-layered (`invariants.md` "Guardrail placement is two-layered"): the **RON/designer/spec layer owns
expressive policy and rejects unsafe *authoring* at import**; the **runtime enforces hard safety
unconditionally** as the last line. As work moves toward the designer surface, guardrails **relocate to
spec admission** — they do not disappear and they do not stay as hard-coded runtime/fixture
special-cases. This is the optimal home: reject early with good diagnostics; the runtime catches
anything that slips.

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
- **One principle per class — no per-slice accretion** (`invariants.md` governing doctrine). A
  constraint is stated once; a change that adds an Nth restatement of an existing principle is rejected
  as redundant. Active docs carry a **compact status table**; per-slice narrative is a one-line worklog
  entry, never an accreting litany.

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
| **0.0.7.9 mobility/transfer substrate** (ALLOC, REENROLL, IDROUTE, ECON, OWNER + RUNTIME-0/1A/1B + semantic-free GPU kernel substrate) | COMPLETE + PARKED (MOBILITY-GPU-SUBSTRATE-DIRECTION-0 → PARK). [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md) | a named scenario needs mobility/ownership/economy in the default `SimSession` path |
| **Line A — nested Resource Flow (A-0)** | ACCEPTED, static nested first slice; production posture is `FlatStarResourceFlow`. | a named economy needs depth>2 nested fanout |
| **Line B — discrete hard-currency ordering (B-0)** | ACCEPTED, narrow smoke; no B-1. | a named multi-transaction hard-currency workload |
| **Line C — atlas / multi-theater mapping (C-0/C-1/C-2)** | ACCEPTED, **map batching CLOSED at the designer surface.** | a named multi-theater scenario opens the atlas production runtime gate (see §4) |
| **simthing-spec → CLAUSE-SPEC (L0/L1/L2)** | ACCEPTED designer-admission substrate. | it is the *engine* of the next track (§5) |
| **ClauseThing / ClauseScript (L3)** | PARKED pending product authorization. | product authorizes the front-end (§5) |
| **Deferred-by-design** | E-11B-5, atlas production runtime, B-1, Hybrid-Strata/faction-index ECON scaling, FrontierV2-5, ACT/EVENT/OBS/PIPE | each behind its own named scenario; none is an open question (see §4) |

---

## 4. Closure of the dangling E-11B and M-4 questions

**E-11B / E-11B-5 (nested Resource Flow dynamic enrollment) — CLOSED as a distinct question.**
A-0 proved static nested materialization; E-11B-5 (dynamic enrollment) was deferred because no named
scenario required it. The 0.0.7.9 **REENROLL bilateral arena re-enrollment substrate** is a *more
general, proven* mechanism for runtime participant set changes (deregister-from-origin +
register-into-destination, atomic-or-reject, no compaction, deterministic). Re-enrollment is therefore
a solved-and-parked substrate. **E-11B-5 is folded into that parked substrate — it is no longer a
separate dangling question.** `FlatStarResourceFlow` remains the accepted production posture; nested
dynamic enrollment, if ever named, is a scenario that pulls the parked re-enrollment + nested-RF
substrate, not new architecture.

**M-4 / atlas (multi-theater mapping) — designer surface CLOSED; production runtime is a parked gate,
not an open question.** C-0/C-1/C-2 closed map batching at the designer surface: `request_atlas_batching`
admits bounded algebraic-G=0, homogeneous-square, protocol-oracle-backed specs within the active
`V78AtlasVramBudget`. The **atlas production runtime / sparse-residency scheduler (M-4A territory)** is a
parked gate behind a named multi-theater scenario — the same posture as every parked substrate. There
is no open M-4 *question*; there is a parked gate awaiting a consumer.

**Net: no dangling open questions remain on the M/E/T or mobility tracks.** All are either accepted at a
first slice, closed at the designer surface, or parked behind a named scenario. The only thing missing
everywhere is the same thing — a **named product scenario** — which is exactly what §5 sets out to
produce.

---

## 5. Proposed next production track — the consumer edge (SCENARIO-FIRST)

**Proposal (design authority): the 0.0.8.0 production track is consumer-pulled scenario authoring, not
more substrate.** Its job is to produce the **first named product scenario** through the accepted
simthing-spec / CLAUSE-SPEC admission layer (advancing toward the ClauseThing/ClauseScript aspiration,
L3) and let that scenario **pull exactly one parked substrate** into a real production path.

This is the deliberate off-ramp from the substrate-loop: the next artifact is a *scenario*, and a
substrate's production-path gate opens **only because a named scenario consumes it** — never speculatively.

**First gate — `SCENARIO-0080-0` (Tier-2, scenario/admission only):**
- Deliverable: a named product-scenario / admission packet (the discipline that worked for
  MOBILITY-SCENARIO-0) — declares the scenario, the one parked substrate it consumes, the bounds, and
  the rejection vocabulary. **No runtime implementation; no production wiring.**
- It must name which parked substrate it pulls (most-ready candidate: the 0.0.7.9 mobility substrate,
  whose production-path gate is already mapped — "first non-test-support default `SimSession` path").
- Acceptance opens **only** that substrate's already-defined production-path gate — nothing else.
- Stop conditions (reject the scenario if it requires any): owner-entity as spatial parent;
  capture-as-reparenting; semantic/raw WGSL; default-on without a gate; hard-currency through Resource
  Flow; a CPU planner/urgency/commitment; or reopening a closed ladder (atlas runtime, E-11B-5, B-1,
  ClauseThing/L3 front-end, FrontierV2-5, ACT/EVENT/OBS/PIPE) without its own product authorization.

**L3 ClauseThing/ClauseScript** remains the long-horizon aspiration that this track advances toward; its
parser/front-end and any production `SimSession` wiring stay Tier-2 and product-authorized — not opened
by this proposal.

**Two-track fastlane applies:** designer-admission scenario/vocabulary work within the accepted L0/L1/L2
layer is Tier-1 fast-lane; a new scenario *gate*, a new architecture, or any production-path wiring is
Tier-2.

---

## 6. Pointers
- Binding structural rules: [`invariants.md`](invariants.md)
- Gating mechanics + anti-loop + proven-capability stop rule: [`workshop/phase_m_gating_and_doc_policy.md`](workshop/phase_m_gating_and_doc_policy.md)
- Closed baseline constitution: [`design_v7_7.md`](design_v7_7.md)
- Superseded-as-active expansion constitution (historical): [`design_v7_8.md`](design_v7_8.md)
- Parked 0.0.7.9 mobility/transfer track: [`design_v7_9_mobility_transfer_allocation_production_track.md`](design_v7_9_mobility_transfer_allocation_production_track.md)
- Workshop architecture record: [`workshop/mobility_and_transfer_allocation.md`](workshop/mobility_and_transfer_allocation.md)
- Active status table + read order: [`workshop/mapping_current_guidance.md`](workshop/mapping_current_guidance.md)
