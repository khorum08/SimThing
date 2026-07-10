# SimThing — Design 0.0.8.3 Constitution (unified; ClauseThing vertical closed)

> **Status: ACTIVE constitution (promoted 2026-06-15, executive design authority).** Supersedes
> [`design_0_0_8_1.md`](design_0_0_8_1.md) (now SUPERSEDED-but-cited). It sits *beneath*
> [`simthing_core_design.md`](simthing_core_design.md) — the permanent paradigm — and *above* the
> production tracks. This version **carries §0 (the transient constitution) forward verbatim** per §0's
> own propagation mandate, **ratifies the closed ClauseThing / MapThing / MapGeneratorCLI vertical**
> (§A), and **incorporates the predecessor's still-binding mechanics by explicit reference** (§B) — no
> binding doctrine is silently dropped (§0.6).
>
> **What changed from 0.0.8.1:** the version-specific lineage/track/pointers are refreshed; the
> ClauseThing vertical is recorded as closed with its decision record and clearinghouse named; §0 is
> unchanged (amended only by addition is permitted, and none was needed). Enforcement still lives in
> [`invariants.md`](invariants.md).

---

## Typeface / simthing-tools reference

The closed typeface track is now consolidated in:

- `docs/simthing_tools_typeface_adr.md` — root ADR (ACCEPTED / CLOSED / DA-APPROVED)
- `docs/tests/current_evidence_index.md`

Archived production-track mechanics and rationale live under:

- `docs/archive/typeface_track_2026_06/design_typeface_ladder.md`
- `docs/archive/typeface_track_2026_06/design_simthing_typeface_track_proposal.md`

The resulting runtime lives in `crates/simthing-tools` as a presentation/support crate. It is not
simulation authority and must remain subordinate to `simthing_core_design.md`, §0 GPU-residency doctrine,
and semantic-free shader rules.

---

## 0. Transient constitution — carry-forward doctrine (MUST propagate to every future version)

> **This section is transient by design and is the cross-version spine.** It holds doctrine that
> outlives any single constitution version. **Every future constitution version MUST copy §0 forward
> verbatim**, amending only by *addition* — never silent removal or weakening. The version-specific
> track, parked inventory, and operating mechanics below §0 are not carried forward automatically; §0
> is. If a future version omits §0, that version is defective. (Carried verbatim from 0.0.8.0 → 0.0.8.1 → 0.0.8.3.)

### 0.0 Purpose — the unitary vision (why §0 is transient)
Maximal SimThing conformance is in the transient constitution for **one** reason: it is the mechanism
by which **conflict, opportunity, ambition, and extraction collapse into a single generic,
GPU-resident SimThing.** Each is the *same* mechanism wearing a different label —
- **conflict** → combat (`HP/Damage` arena), disruption (decaying accumulator);
- **opportunity** → desirability fields and gradients (where to go);
- **ambition** → owner-entity drives, expansion, fight-or-flight (threshold-gated value decisions);
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

### Terminology correction — owner, not faction

“Faction” is game-language. The constitutional ontology term is **Owner** / **owner-entity**. Existing
grand-strategy examples may describe an owner as an **owner-faction**, but the engine model is not
faction-specific. Owner entities are sibling GameSession children; policies, bonuses, penalties,
capability subtrees, and stockpiles may live on Owner SimThings as properties, overlays, and children.
Assets refer to owners through owner references, properties, and columns. Ownership changes do not
spatially reparent assets.


### Terminology correction � Scenario wrapper and GameSession root

A canonical saved scenario is rooted at `Scenario`, which is the save/load authority wrapper. The
`Scenario` root has exactly one direct `GameSession` child. `GameSession` is the runtime session root
beneath that wrapper; its direct children include Owner SimThings and the root spatial GalaxyMap /
WorldStateMap Location. Future tracks must not conflate the save/load wrapper with the runtime root.

RF membership is channel identity, not ownership containment. A spatial participant joins owner/resource/
scope lanes through RF metadata on properties/overlays; the Owner SimThing remains a GameSession sibling.
Local owner/resource/scope channels settle inside each parent Location before net surplus/deficit bubbles
upward. This is permanent doctrine: `simthing_core_design.md` �5 is the canonical home, not only RF ADRs.

### 0.1 Maximal SimThing conformance (the founding premise)
**Everything is a SimThing.** There are no privileged engine-side special cases for game concepts:
gamesession, owners, worldstate, starmap, star systems, planets, grid cells, fleets, and cohorts are
all SimThings in one recursive `{properties, overlays, children}` tree. New behavior is modeled by
adding SimThings, properties, overlays, and `AccumulatorOp` registrations — **never** by a bespoke
subsystem sitting outside the tree. When a design seems to need special-case logic, the correct move is
almost always to express it as *more SimThing*. (This is the antidote to the math-in-a-vacuum drift that
`invariants.md` "Scenario Proof" now also guards: behavior is proved through the tree, not beside it.)

### 0.2 Allocation is always recursive (overrides the flat-star carve-out)
Resource flow is **one** mechanism: reduction **up** the tree (each parent reduces its children's flow
into a surplus or deficit and passes it to its parent) and disbursement **down** the tree (the
gamesession root sends flow down; owner entities hold the stockpiled values, resolve deficits from the
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
   specified feature — is the failure mode this law exists to kill. **This law governs documents and
   process, not the running compliance layer.** The **CI doctrine-scan is the primary automated compliance
   mechanism** (a binding-6 running guard) — real enforcement, not "harness ceremony," despite emitting a
   report. The positive half — "the specified consumer must actually run" — is the anti-fabrication floor:
   *asserting or fabricating a result you did not run* is the worst ceremony of all. Do not cite this law
   (or D8) against the CI screen or to skip running it (`invariants.md` → "The CI doctrine-scan is the
   primary automated compliance mechanism").
6. **No inert scaffolding (appearance-of-completeness is void).** *(Added 2026-06-29 by design authority on
   owner direction, after an unwired `deny.toml` stub provided the *appearance* of dependency compliance
   while enforcing nothing.)* An artifact that **looks like** a capability, gate, check, or completed
   structure but **does nothing** — an unwired config, an empty/placeholder module, an uncalled stub, a
   dead allowlist, a scaffold for a feature that never landed — is **not neutral; it is a liability.**
   Each looks benign alone; **in aggregate they fake completeness and compliance**, and any one can be
   cited — by a low-context agent or a reviewer — as evidence of a capability that does not exist (the
   **handwave vector**). This is the inverse face of binding 1: that forbids flattening a real spec into a
   proxy; this forbids accumulating empty shells that aggregate into a false "done." Bindings: **(a) do not
   produce inert scaffolding** — the real thing is created when it is actually wired; until then it does
   not exist; **(b) an artifact that looks like a gate but isn't one is removed, not annotated** (so the
   real enforcement is unambiguous — an honest comment inside a *working* file is fine; a whole artifact
   whose only content is "I am a stub" is not); **(c) delete it when encountered** — any track may remove
   verified-inert scaffolding it crosses (net-negative, no approval needed). Enforcement is a **fact, not
   an appearance**: a compliance/admission artifact must occupy a real admission rung (type / hard-error /
   running guard); one that implies a rung it does not occupy is worse than no artifact
   ([`simthing_core_design.md`](simthing_core_design.md) §1.2). SimThing already carries accumulated inert
   scaffolding; at minimum no track adds more, and tracks should delete it as they cross it.

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

**Candidate F is permanently enshrined (STEAD-CONTRACT-0, owner-directed).** Bit-exact Euclidean
distance and other sqrt-requiring decision-gate operations are **first-class and routed *through*
Candidate F — never demoted, avoided, or designed around.** The earlier instinct to "avoid Euclidean /
avoid sqrt to stay exact" is **withdrawn**: the artifact *is* the exact path, so exactness no longer
costs the operation. This applies to STEAD/Movement-Front magnitudes whenever an exact gate is needed;
it does **not** license treating gridcell positions as inert — the integer stencil still walks neighbors
by index arithmetic (core §0, §7), and Candidate F governs the *magnitude* of a gate, not whether space
is real. (See [`stead_spatial_contract.md`](stead_spatial_contract.md) invariant 8.)

Historical R4 ledger detail remains in
[`design_0_0_8_0_consumer_pulled_production_track.md`](design_0_0_8_0_consumer_pulled_production_track.md);
§0.7 is the carry-forward constitutional rule.

**Token API (OC-K-EXACT-GATE-0).** Magnitude-sensitive threshold / commitment registration accepts
only an `ExactMagnitudeProof` minted from Candidate F. `ApproximateDiagnostic` (native sqrt /
telemetry) cannot construct, convert into, or feed that proof. Sanctioned mint path:

```text
GradientPairGpu { dx, dy }
  → mint_exact_magnitude_proof_candidate_f(_cpu)  // Candidate F CR-F + Q16 mag2
  → ExactMagnitudeProof { bits }                  // private field; not forgeable from f32
  → ThresholdRegistration::register_exact_magnitude_sensitive(...)
  → CommitmentRegistration::register_exact(...)
```

**Worked bit-exact sqrt example (3-4-5).** Fixed-point Q16 mag² of `(dx=3, dy=4)` is exactly
`25.0` (`f32::to_bits()` identity). Candidate F CR-F (`sqrt_cr_f_bits`) on those mag² bits yields
the exact Euclidean magnitude bit pattern used as the proof token; the same bits must appear in the
threshold registration field. Native `f32::sqrt` may match for this triangle but is still
`ApproximateDiagnostic` only — it cannot mint `ExactMagnitudeProof`.

### 0.8 STEAD/Mapping spatial substrate carry-forward (STEAD-CONTRACT-0)

> **STEAD/Mapping is a load-bearing pillar of SimThing, not an optional feature. This clause and its
> mandatory pointer to [`stead_spatial_contract.md`](stead_spatial_contract.md) MUST propagate to every
> future constitution version verbatim, amended only by addition.** It exists because the spatial
> substrate drifted out of context **three** times (positions-inert, dense-global, edge-cap); enshrining
> it in §0 is what makes a fourth drift a constitutional violation, not a forgivable oversight.

The map **is** a grid of gridcell `SimThing`s run as a cellular automaton (the Movement-Front automaton,
[`simthing_core_design.md`](simthing_core_design.md) §0 + §7). A `SimThingKind::Location` **is** a
structural gridcell; spatial identity is intrinsic, never inert metadata. The binding, normative form —
8 defined terms, 9 sections — is [`stead_spatial_contract.md`](stead_spatial_contract.md), **mandatory
reading** (`agents.md`) for any task touching MapGen, Location grids, Movement-Front, STEAD, heatmaps,
falloff, PALMA, Gu-Yang/SaturatingFlux, or RF/Accumulator arenas over gridcell Locations.

The non-negotiable invariants (full text in the contract; summarized in core §0):
1. A `Location` **is** a structural gridcell; the parent grid owns the spatial arena (`grid_metadata`).
2. Emitted integer `(col,row)` are **structural** coordinates the lowerer honors — never render,
   emission-order, or row-major fill.
3. Unoccupied cells are **ambient field**; lattices are sparse and may be **vast** (`200×200` is small).
4. Heatmaps, falloff, fronts, Gu-Yang, PALMA, and RF pressure are **expressions over the structural
   substrate**, not independent services.
5. **Layout admission** (budget-based, no fixed edge cap) is separate from **execution-profile admission**
   (the ≤10/32-per-edge bounded theater); a vast layout may pass while a dense profile **defers to the
   atlas** — that is not "the map is too large." Dense theater caps cannot shrink or invalidate the layout.
6. RF/Accumulator stays generic, **but** an arena over gridcell `Location`s is **spatially bound**: each
   participant requires a structural `grid_metadata` placement (`validate_spatial_binding`), and the arena
   records its `StructuralGridFrame`. Spatially-neutral arenas need no grid. **Whenever generic
   RF/Accumulator code admits a new arena, it must confront whether that arena is spatially bound.**
7. Candidate F (§0.7) governs exact-magnitude **decision gates** but **never** licenses treating positions
   as inert; exact sqrt/Euclidean ops route *through* it, never avoided.

Enforced by `crates/simthing-clausething/tests/stead_spatial_contract_guards.rs` (section-aware
withdrawn-phrase scan + structural/MF/PALMA/ledger guards, including a guard that **this §0.8 clause and
its `stead_spatial_contract.md` pointer remain present in §0**) and `mapgen_rf_stead_binding.rs`. A future
constitution that drops this clause or the pointer is **defective** (§0 preamble).

### 0.9 Doctrine-as-Type, the kernel authority, and residue-as-tripwire (carry-forward)

> **Added 2026-06-29 by design authority on owner mandate, after the 0.0.8.4 / 0.0.8.4.5 admission-substrate
> + SimThing-Kernel tracks. Like the rest of §0, this carries forward verbatim to every future version,
> amended only by addition.** The full paradigm is [`simthing_core_design.md`](simthing_core_design.md) §1.2
> + §1.2.1 (load-bearing and governing); a longer `simthing-kernel` ADR may elaborate but never supersedes it.

1. **Doctrine-as-Type.** Encode an invariant at the highest admission rung that can express it — **type
   boundary (illegal state uncompilable) > admission hard-error > guard scan > prose.** A guard scan or prose
   rule that exists only because a type didn't is a **promotion target**, not a fixture. This is permanent and
   applies to *past* routes (refactor them up) as much as new ones.
2. **The kernel is the authority.** `simthing-kernel` is the sole owner of authoritative state and sole minter
   of authoritative effects; consumers get a read-only view + sanctioned in-crate doors. It is the
   runtime-authority layer and composes with (never replaces) content-admission.
3. **The cross-crate seal law.** *Mint authoritative types in the crate that privately owns their source of
   truth; never re-seal across a crate boundary with a token* (Rust has no friend-crate visibility; cross-crate
   `pub`/`#[doc(hidden)]` is capability-for-everyone). When sealing conflicts with crate convenience,
   Doctrine-as-Type wins and the code moves *into* the owning crate. Binding on every future authority
   extraction (spec, scenario, …).
4. **Residue-as-tripwire.** The irreducible residue (CPU-oracle twin, WGSL text, inert utilities) is a
   **named, greppable tripwire catalogue**, admissible only per-item and justified — never a categorical
   wave-through — so routing through it is a *declared, deliberate circumvention* the orchestrator flags
   (`seal-residue-risk`). An artifact that looks like a gate but enforces nothing is **deleted, not
   annotated** (§0.6.6).
5. **CI doctrine-scan is the automated rung-3 layer, DA-equivalent when clean.** *(Added 2026-07-01 by
   design authority on owner mandate, `0.0.8.4.6` CI Scaffolding track; full contract:
   [`design_0_0_8_4_6_ci_scaffolding.md`](design_0_0_8_4_6_ci_scaffolding.md).)* The GitHub-side grep layer
   (`scripts/ci/doctrine_scan.sh` + `scripts/ci/scans.tsv` + `scripts/ci/allow/*.txt`) is the executable form
   of the admission ladder's guard-scan rung, standing in for a type/admission boundary that does not exist
   yet. **The DA-equivalence contract:** a clean **RELIABLE** scan (allowlist scans especially) is
   **DA-equivalent** for that scan — trusted without re-verification; `FAIL` is a **HOLD**; `INSPECT` (a
   **HEURISTIC** hit, or a RELIABLE hit in a known false-positive zone) is **look/triage**, never a silent
   pass and never an automatic block. **The §1A triage contract:** an INSPECT routes first through a free
   triage tier before the scarce DA — the PR author pays first with a one-line structured justification per
   flag; a bounded loop + greppable spam-bounds (excess INSPECT volume, symbol-walking across HEURISTIC
   scan-ids, INSPECT rising while a RELIABLE FAIL stays open) force hill-climbing to escalate-as-FAIL instead
   of laundering through the soft tier; a DA spot-audit remains the backstop over triage clearances. **The
   allowlist contract:** an `allow/*.txt` entry is a **typed admission record**, not a babysat list — a new
   entry is a **deliberate, reviewed widening of a sanctioned door** (correct door-class grammar + rationale
   + promotion-blocker), never a scanner edit to dodge a valid finding. **The scan-retirement obligation:** a
   scan is itself residue (§0.9.4) — when the invariant it guards is promoted to a type boundary or admission
   hard-error, the rung that promotes it **retires (narrows or deletes) the now-redundant scan in the same
   PR**; promotion also retires the now-redundant test in the same PR. Tests are ladder residue, like scans:
   a KEEP-class test is admitted only when it names the regression nothing higher on the ladder owns. A KEEP
   row without a permanent-residue class or promotion target is illegal. In `simthing-kernel` and
   `simthing-sim`, KEEP-class tests are legal only for never-pare/permanent-residue classes; admission-rejection
   enumeration and hygiene tests are drift failures, not INSPECTs. A RELIABLE scan with no promotion-blocker is
   a flagged anomaly. **The Necessity Test *(added 2026-07-03 by design authority on owner mandate; retires the
   "one representative per boundary" premise as a fossil compromise of the Rustification mission)*.** A test is
   admissible **only if it catches a regression that neither (1) the compiler / a type boundary, (2) a
   production admission hard-error on a live path, nor (3) an existing integration/canonical path already
   catches.** Deletion rule: **if deleting a test cannot break production and it is not a downstream dependency
   or required for canonical function, it is DELETED — never kept as a "representative."** The retired premise
   assumed a boundary needs a test for coverage; the kernel admission substrate made that false (the type or the
   production admission hard-error *is* the coverage). The floor is **zero** for any invariant a type or a live
   admission hard-error enforces; the only legitimate keeps are tests that pass the Necessity Test — genuine
   parser/format behavior a type cannot absorb, CPU-oracle/GPU parity, determinism/golden byte-exactness,
   doc-named invariant proofs, escaped-bug regressions, and the CI scanner's own known-bad fixtures. **The
   Rustified test lifecycle *(added 2026-07-04 at Track D closeout, owner mandate)*:** a test is a **scoped
   borrow, not a permanent asset** — born owned by its birth PR ladder/track and **assumed DELETED at that
   track's closure** (deletion is free; keeping needs a recorded reason). A test outlives its birth ladder only
   by (a) carrying a canonical notion — which must then be **promoted into a `simthing-kernel` type/seal or an
   EML opcode-stack construct** (deleting the test), (b) being a `TIER7` terminal proof class with a `catches:`
   note, or (c) being a non-runnable `dependency-floor` helper. This is standing law for every future track so
   the corpus can never re-propagate (Track D pared 6,301 → 731); the full-workspace `cargo test` is a one-time
   PR-ladder **closure certificate**, not routine proof. Full regime: CI-scaffolding design §4.1. **The
   merge-hold rule:** no rung that
   changes PROBATION / authority / gate-state semantics merges before DA/Owner clearance; a truthful
   corrective self-report of a breach may be accepted on its merits, but it is never precedent for skipping
   clearance again. **The verification rule:** the DA (and any orchestrator relaying a proof) verifies the
   *tree*, never a relayed report — a clean CI check is not a substitute for the DA independently confirming
   the branch matches what was claimed.
6. **The breakthrough valve — gated *and* invited.** *(Added 2026-07-01 by design authority on owner mandate.)*
   The boundary is the boundary — but a genuine structural win a seal would block must be **surfacible, not
   suppressed.** An architectural experiment against a sealed boundary travels as a **pair** (neither half is
   safe alone): the **gate** — a *working, conformant baseline delivered first* (no baseline → not entertained;
   this defeats hill-climbing into persuasive prose to dodge a hard problem) + bit-exact CPU-oracle parity + a
   *measured* local hot-path dividend + adversarial **D2**-substrate-ladder exhaustion by a **decorrelated**
   reviewer — makes the channel **costlier than conformance**, never a shortcut; and the **incentive** — the
   experiment rides **risk-free alongside** the already-green conformant rung, **never gates its PASS**, and is
   **explicitly invited** — keeps a closure-seeking agent from *burying* a real insight. Gate-without-incentive
   is **sterile**; incentive-without-gate is a **loophole**. Owner-gated throughout via the Admission-Substrate
   Amendment Valve (kernel-track §3A): **surface, never self-grant.**
7. **The orchestration contract and the CI screening surface.** *(Added 2026-07-03 by design authority on
   owner mandate.)* **[`ci_screening_surface.md`](ci_screening_surface.md) is mandatory onboarding reading for
   any orchestration session** — a new orchestration chat that has not read it (plus core §1.2/§1.2.1, this
   §0.9, the kernel track §5.2 bypass catalogue, and [`handoff_template.md`](handoff_template.md)) is not yet
   qualified to route work; its §5A carries the operational orchestrator contract. **Orchestrator merge
   authority (owner-directed 2026-07-03):** with branch protection active (doctrine-scan required; no direct
   pushes; owner bypass reserved), the orchestrator MAY authorize merge without DA escalation when ALL hold:
   (1) declared risk class is `none` / `semantic` / `data-deliverable` AND the rung follows an
   already-DA-approved pattern — a precedented wave class executing under a standing ruling; (2) it is NOT
   gate-wiring, seal-residue, allowlist-edit, protected-corpus touch (kernel / Admission Substrate / CI
   architecture), first-of-class, or anything a standing ruling reserves to DA/Owner; (3) all RELIABLE gates
   are green on the PR head, with SHA-bound targeted proof where a profile exists; (4) the Graduation-routing
   block PLUS a one-paragraph merge rationale is filed in the PR thread BEFORE merging; (5) confidence is
   genuine — any doubt, novelty, or precedent-setting element escalates. The DA spot-audits a sample of
   orchestrator-authorized merges against the tree; a wrong self-authorized merge is a FAIL-class process
   finding and suspends this authority pending owner review. The absolute merge-hold rule stays in force for
   the reserved classes. **Owner supremacy:** the owner may merge or bypass anything at any time,
   platform-visible and recorded — a right of the owner, never a loophole for agents to cite.
8. **Orientation curation + graduated kernel gates (0.0.8.4.8.3 CLOSED).** *(Added 2026-07-10 by design
   authority on owner mandate.)* **Doctrine reaches agents as thin anchors + path triggers + query — not
   digest essays.** Live surfaces: `scripts/ci/doctrine_anchors.tsv`, `anchor_triggers.tsv`,
   `anchor_query.sh` (THE doctrine lookup; appends `anchor_reach_log.tsv`), `anchor_check.sh --resync` after
   anchored-doc edits, and the Cold-Start Spine emitted by `gen_orientation.sh` / `orient.sh` (pointers only;
   resolve verbatim via query). Clearance sticky emits `REQUIRED-ANCHORS:` on `DA-RESERVE`; handoffs ACK with
   `ANCHOR-ACK`. **Graduated kernel doors (Lane B / OC-K\*):** (K2) role pathway → `ColumnIndex` via
   `col_for_role` — residual tripwire `COLUMN-INDEX-MINT` (OC-K2.1); (K3) exact registrations require
   `ExactMagnitudeProof` minted from Candidate F — `ApproximateDiagnostic` cannot mint; (K1) sealed decision
   ingress `ThresholdCrossingToken → EmissionToken → BoundaryEmissionToken → StructuralCommitment` — CPU /
   approximate diagnostics cannot mint commitments; (K4) closed EvalEML vocab via `OpcodeRegistrationGate` /
   `AdmittedEvalEmlOpcode|Combine` — semantic ops rejected; authoring surface
   [`eml_gadget_library.md`](eml_gadget_library.md). Door catalogue: generated
   [`sanctioned_surface.md`](sanctioned_surface.md) from `allow/*.txt`.

---

## A. Closeout addendum — the ClauseThing vertical (ratified 2026-06-15)

> *Small by mandate (constitution §0.5 Rule 3 — link out, never inline). The durable decisions live in
> the ADR; the concepts/practices/APIs live in the clearinghouse; the blow-by-blow lives in the archived
> ladders.*

The **ClauseScript → MapThing → MapGeneratorCLI** vertical — designer-facing Clausewitz ingestion, the
Stellaris-starmap ingest/lowering layer, and the standalone galaxy producer — is **CLOSED and ratified**.
It is one proof, three times over, of the §0 premise: *foreign grand-strategy content is ingested as
declarative SimThing structure and lowered onto the generic `accumulate → reduce → mask → threshold` tree
— no new engine, no new `SimThingKind`, no Euclidean/pathfinding authority in the lowered output.*

- **Decision record:** [`adr/ClauseThingADR.md`](adr/ClauseThingADR.md) (D1–D10 — the durable adjudications).
- **Clearinghouse (concepts / practices / APIs):** [`clausething/ClauseThingDoc.md`](clausething/ClauseThingDoc.md).
- **Governing ADRs (unchanged):** [`adr/mapping_sparse_regioncell.md`](adr/mapping_sparse_regioncell.md), [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md). Core mapping doctrine: [`simthing_core_design.md`](simthing_core_design.md) §7.
- **Archived production tracks** (`archive/closed_production/`): ClauseThing (`design_0_0_8_1_clausething_production_track.md`, `design_0_0_8_2_clausething_closeout_ladder.md`), MapThing/MapGen (`design_0_0_8_2_5_mapgen_ladder.md`), MapGeneratorCLI (`design_0_0_8_6_mapgenerator_cli_ladder.md`), substrate sub-tracks (`design_0_0_8_1_border_hack_track.md`, `design_0_0_8_1_palma_pathfinding_integration_guide.md`).

**Carry-forward gates this vertical added to the project's working rules** (enforced per-track, not §0 amendments):
1. **Closed lowering layers are closed.** A producer/front-end PR makes **zero** `crates/simthing-clausething/src/` edits; a needed lowerer change splits to a **DA-authorized 0.0.8.2.5 amendment** (precedent `#680`).
2. **Producers emit only already-accepted grammar** (`static_galaxy_scenario` neutral-AST), never widening `hydrate_scenario`/the lowerer.
3. **Per-rung DA-sensitivity governs merges; only the DA writes a DA sign-off, never pre-filed.**
4. **STEAD spatial layout is privileged (STEAD-PRIVILEGE-0, 2026-06-15 amendment).** A `Location`'s gridcell
   coordinate is **structural-spatial** (core §7): the closed lowerer **honors the emitted integer position** as
   the authoritative `(col,row)`, so the generated galactic pattern **is** the lattice the Movement-Front
   automaton runs on. §0.7 (Candidate F) governs **decision-gate magnitudes only** — it never licensed treating
   positions as inert or placing cells by emission order. A prior "positions inert / index-order placement"
   ruling was a drift, **withdrawn and corrected** (ADR D6).
5. **Structural scale is budget-based — no fixed edge cap (STEAD-SCALE-0/-1, 2026-06-15).** Structural gridcell
   layout **has no fixed edge cap**; it scales by **explicit admission budgets and memory constraints**
   (`MapgenStructuralGridBudget`, checked-`u128` capacity), not a magic constant — `200×200` is a *small*
   reference (not a canonical upper bound) and `65,535` was a temporary arithmetic ceiling, **not doctrine**
   (removed). **Layout and execution scale are decoupled:** execution profiles (Movement-Front/PALMA/RF dense
   fields) may impose bounded-theater limits, and **a vast layout may be admitted even when a particular dense
   execution profile defers to atlas scheduling** — represented as *layout admitted, execution requires
   atlas/tile scheduling*, never "the map is too large."

**One outstanding item (future, separate, DA-authorized — not a producer PR):** an **RF capacity amendment**
(raise/scale RF participant/slot caps or add scalable deposit-initializer feedstock) is required before
galaxy-scale generated packs can admit/install. Until then galaxy-scale runtime is gated and that gate is
honest (ADR §5). **Next track:** FIELD-MOVIE-DATASET-0 (editor/corpus/export) — not opened here.

## B. Carried mechanics (incorporated by explicit reference — not silently dropped)

Per §0's mandate, only §0 carries forward automatically; the predecessor's version-specific **operating
mechanics remain binding and are incorporated here by reference** until folded:

- **Operating doctrine** — [`design_0_0_8_1.md`](design_0_0_8_1.md) §2 (consolidated; read before any 0.0.8.x work).
- **Parked capability inventory** (proven, default-off, awaiting a consumer) — `design_0_0_8_1.md` §3.
- **Closed questions** (no dangling open design questions) — `design_0_0_8_1.md` §4.
- **Design lineage** (v4 → 0.0.8.1, archived `archive/superseded_design/`) — `design_0_0_8_1.md` §1.

`design_0_0_8_1.md` is **SUPERSEDED by this 0.0.8.3** as the active constitution but retained in place as
the cited source for the above (moving it would break the cross-link web; it carries a superseded banner).

## C. Pointers
- Permanent paradigm: [`simthing_core_design.md`](simthing_core_design.md). Structural invariants / gates: [`invariants.md`](invariants.md). State authority: [`state-authority.md`](state-authority.md). Agent router: [`agents.md`](agents.md).
- The ClauseThing vertical: [`adr/ClauseThingADR.md`](adr/ClauseThingADR.md) + [`clausething/ClauseThingDoc.md`](clausething/ClauseThingDoc.md).
