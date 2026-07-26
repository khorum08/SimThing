# 0.0.8.7 — RF Arena Modernization (the SimThing unification)

> **Status: STUB / AUTHORED — NOT OPENED.** Owner-directed 2026-07-19. 0.0.8.6 is CLOSED
> (2026-07-25); this track opens when the Owner commits the full plan
> (via `gen_orientation.sh --open`; pointer-lifecycle gate applies). Absorbs the former
> 0.0.8.4.8.4.2 RF-harness track (Owner ruling 2026-07-26; see §3 [HARNESS] phases and §5.2).
> Do not flip the pointer; do not dispatch from this doc. Phase ladders below are
> commitment-time placeholders — rung decomposition happens when the Owner commits.
>
> **CORE track.** At completion this is **built INTO THE CORE**: the canonized object model
> lands as constitutional sections of `simthing_core_design.md`, and the **SimThing object is
> complete and distinct** — one concrete, working, GPU-resident simulation kernel.

## 1. North star

Unify the SimThing principle: **a SimThing with resource properties, parented into a tree,
simulates — intrinsically, recursively, on the GPU — with zero scenario-side wiring.** The
recursive RF arena regulating resource economics and Wei's STEAD automata expressed as spatial
heatmaps ARE the Sim in SimThings; this track makes that a property of the *object*, not of
per-session configuration. Future scenarios — and other games — script onto SimThing trees and
**work**.

**The prize is emergence (Owner, 2026-07-20).** The unification is real and tonal — STEAD is
control economics: the same allocation kernel run over a non-conserved signal commodity, where
stocks are the plant, the field is the controller, the arena-pressure projection is the sensor,
banded commitments are the actuator, and authored weights are the prices. But the unification is
the *rail*, not the prize. **The value this track exists to capture is Wei's emergent behavior**:
fronts, borders, escalations, and strategy arising UNSCRIPTED from one shared local rule plus
authored prices (attractor dynamics — 12.10's `38/38/1` divergence from two weight scalars was an
attractor-relocation proof, the first captured instance). Every phase privileges that emergence
while aligning strictly to the SimThing recursive emission-band RF topology: mechanisms serve the
dynamics and must never script the outcomes they exist to let emerge.

Evidence base: the 0.0.8.6 RF sub-track (RF-1…RF-5A). Every expensive failure was the same
gap — the RF kernel living as session wiring instead of object semantics (engine disengaged
for three eras behind opt-in flags; RF-5A's manual plumbing; the cosplay era only possible
because there was a seam to fake). The substrate ADR's original intent ("one execution model;
allocation is intrinsic") is completed at the object level, now falsifiable because the
executed recursive default (RF-2), the conservation oracle (RF-1), and repaired kernel
contracts (RF-2A) exist.

## 2. Pillars (Owner requirements, binding at commitment)

- **P0 — The Root Contract (Owner constitutional restoration, 2026-07-26).** SimThing is the
  atomic root from which every specialist derives (SessionThing, GridcellThing, PopCohort,
  FleetObject, PlanetaryBuilding, …) — an **interface in the hard-contour sense**, not an
  abstract class: the contract is SHAPE (a dense-matrix row; columns reached only through the
  role pathway; emission bands/thresholds on cells; a parent slot + child-row allocation) plus
  PROTOCOL (the recursive cycle below). The implementation exists **exactly once — the unified
  kernel** — and descendants cannot supply their own: an interface whose every method the
  substrate itself implements. By derivation alone — no wiring, enrollment, or registry — every
  SimThing possesses the root functions: (a) RF arena resolution when it carries resource
  properties + a parent edge; (b) tree-wide effects queryable as GPU sweep reductions;
  (c) EML emission-band threshold branching pulled from the core; (d) dense-matrix residency +
  slotting; (e) **STEAD anchor semantics (Owner fulcrum ruling, 2026-07-26 — the
  un-sidesteppable Consumer Law).** Every resource-bearing property value store is **born a
  STEAD anchor** (default-Anchored: falloff profile + band ladder auto-derived from the
  property class, authoring-overridable); opting out is an authored, diff-visible
  `Unobserved { reason }`, and the dark-cell set is itself a standing board/Studio surface —
  observation is free, deferral is loud ("waiting for consumer" can never again close a turn
  silently; born from the 12.3 walled-disruption failure). Impact is enforced at the two choke
  points derivation cannot avoid: the unified write door **derives** band-crossing deltas in
  the same fused pass (nothing for derived code to remember), and structural ops —
  fission/fusion, table reallocation — **cannot encode** without an anchor-remap section
  (anchor identity minted at admission, stable across slot moves; the field survives topology
  change by construction). All observation reads ONE derived GPU-resident anchor table (band,
  last-crossing tick, urgency, falloff params) maintained solely by admission + the write door
  — data-shaped, never a listener framework. Band ladders are **value-domain-generic**:
  boolean, integer, and float/complex ladder variability is anticipated and exposed as coding
  paths (per-domain EML evaluation paths), never intrinsic scalar assumptions — anything the
  SimThing GPU matrix stack holds gets free auditability, STEAD actionability, and parallel
  evaluation. The STEAD model rides the recursive architecture for free: anchored emission
  reduces upward in the same passes as amounts, ladders are the same threshold registrations,
  urgency the same EML stacks — behavior and observation are one synchronous update (Wei).
  **Specialization is additive-only**: extend, never opt out of, reimplement, or
  shadow a root function (`DefaultDisabled` is the single sanctioned opt-out, authored, never
  kind-implied). **Lightweight guarantee:** a SimThing at rest is a row; core functions are
  shared dense sweeps — cost scales with `slots × columns`, never object-count × dispatch.
  **The recursive cycle (the core protocol):** each SimThing's resource property values reduce
  UPWARD to its parent as a velocity (the GovernedPair rate cell); the parent applies overlay
  modifications; final values disburse back DOWN — and it does the same to all its children.
  One shared rule at every level: the constitutional cycle and Wei's P2 emergence prerequisite
  are the same clause. The cycle resolves **within a single tick as ordered passes**
  (OrderBand), never pipelined across ticks. It already executes as
  `RecursiveArenaResourceFlow` — this track binds it to the *object*.
  Falsifier: when P1 lands, the `ArenaParticipant` wrapper kind is deprecated
  (StarSystem/Station disposition) — the wrapper kind existing was the symptom of
  participation-by-wiring; its deprecation is the proof of participation-by-derivation.
- **P1 — Intrinsic RF kernel at the object.** Resource properties + a parent edge ⇒ arena
  participation **derived at admission** (typed, inspectable, spanned hard-errors;
  `DefaultDisabled` opt-out retained). Behavior when parented and as a parent is intrinsic,
  natural, recursively automatic. Collapse the config seams (ResourceFlowSpec / ArenaRegistry /
  execution profiles / registrations) into the derivation.
- **P2 — Overlays as THE data/settings/directives layer.** Canonize overlays with a full,
  **living API surface**: lightweight and versatile enough to span capability-tree overlays
  all the way to **user-action mechanisms**. One overlay law from authored data to operator input.
- **P3 — Specialization protocol.** Specialized SimThings (spatial, owner, session, …) get a
  protocol **richer than a kind enumeration** — but it **must not break existing kind code**.
  Promotion, not rewrite; every rung keeps the existing corpus green (compatibility falsifier:
  full existing test suite passes unmodified at every rung).
- **P4 — GPU-residency intrinsic to the object.** SimThings ARE dense-matrix rows: the
  row/column setup is as intrinsic as RF flow, property-value emission bands, and thresholds.
  **A SimThing knows how to slot into a parent, and how child rows are allocated relative to
  it.** Slot/column identity flows from the object model through the role pathway — never
  minted beside it (consumes the Phase 0 column admission-gate; the Phase 8 sweep then
  migrates legacy sites onto the object-semantic doors this phase defines).
- **P5 — Intrinsic STEAD influence.** Resource accumulation on spatial SimThings emits
  influence falloff as *object behavior* (not authored emitters); falloff superposition forms
  the heatmaps; their interactions feed **Gu-Yang pathfinding and PALMA borders** through the
  Structural Execution Convergence Contract (stead §10) — existing ops, never bespoke kernels.
- **P6 — Portability proof.** A deliberately non-TP domain (ADR's own arena examples: food /
  research / colony) scripted **purely onto SimThing trees, zero engine code** — and it works.
  The 12.6 second-synthetic-scenario discipline scaled to a whole game.

## 3. Phase ladder (stub; decomposed at commitment)

Phases tagged **[HARNESS]** are guard/CI/lifecycle work — **not SimThing core simulation work**
(Owner ruling 2026-07-26, absorbing the former 0.0.8.4.8.4.2 RF-harness track). They protect and
de-cruft the substrate the core phases are built on, but they are NOT part of the object model
and do NOT land in `simthing_core_design.md` at canonization. Split rule for any further RF drift
interventions the Owner adds at commitment: **guards early, sweeps late.**

| Phase | Theme | Note |
|---|---|---|
| 0 | **[HARNESS]** RFH-1 `RF-COLUMN-ADMISSION-GATE-0` + hygiene prerequisites | **Type boundary (= OC-K2.1a; kernel lane, Owner-gated K2 lineage) — lands NON-BREAKING before core work begins.** `ColumnIndex` constructor taxonomy in `simthing-core`: `new` goes non-public; legal doors = layout-derived paths (`PropertyLayout::offset_of` / `col_for_role` / arena-layout ranges) + two doc-fenced choke points (GPU round-trip constructor; fenced raw door for oracle/rehearsal code — existing ~170 call sites keep compiling through it, only NEW code is steered). Retarget `COLUMN-INDEX-MINT` to the choke-point tokens; DA-gate exclusion edits on the scan. Rides along: execution-status taxonomy `executed / oracle / rehearsal / compile-plan`, board-surfaced. Falsifier: post-rung, the scan proves every remaining `::new` is inside the fence. **Interim standing order (in force NOW, until this phase lands): `COLUMN-INDEX-MINT` exclusion additions are FROZEN — DA sign-off required; peer-citation is not a valid justification.** |
| 1 | Intrinsic RF kernel (P1) | Derived-at-admission participation; config-seam collapse; RF-1 oracle + determinism judge every rung. |
| 2 | Overlay API canonization (P2) | Data → settings → directives → capability trees → user actions; one living surface. |
| 3 | Specialization protocol (P3) | Richer-than-kind, kind-compatible; spatial/owner/session first citizens. |
| 4 | Intrinsic GPU residency (P4) | Row/col + parent-slotting + child-row allocation as object semantics. |
| 5 | Intrinsic STEAD influence (P5) | Falloff emission + heatmap interaction + Gu-Yang/PALMA consumption. **STEAD-as-emission-bands (Owner correction 2026-07-20):** STEAD influence on a parent SimThing is READ as **emission-band thresholds** — a banded ladder of threshold registrations on the accumulated cell (existing machinery composed: N band edges = N threshold registrations, each with its own event kind/band id; the RF-5A append-scan proved multi-scan sequencing). Bands bind **1:1 to the falloff influence bands in the heatmap**; intensity (band), velocity (banded thresholds on the GovernedPair rate cell), and magnitude (value) are all observable/measurable through existing property mechanisms. **Quantize the READING, never the FIELD**: falloff superposition + conservation math stay continuous; bands are the observation/decision/coupling surface only — band edges must not enter accumulation math (replay determinism + RF-1 envelope). **Implements the P0(e) STEAD fulcrums (Owner ruling 2026-07-26):** (1) anchor disposition at admission — default-Anchored, authored `Unobserved{reason}` opt-out, dark-cell board surface; (2) write-door impact — band-crossing deltas derived in the fused mutation pass; fission/fusion + table-reallocation ops refuse to encode without anchor remaps; (3) the derived anchor table as the SOLE observation surface for heatmap/Gu-Yang/PALMA/Studio/telemetry; (4) domain-generic ladders — bool/int/float-complex band evaluation as per-domain EML coding paths. |
| 6 | **Movement-Front execution** (Owner-added 2026-07-20) | Fleet movement AUTHORITY: spatial SimThings move along STEAD need/falloff gradients (core §7 automaton, Gu-Yang pathfinding, PALMA borders) under the ordinary tick — decisions stay threshold crossings, movement state is sim authority. **Decision ingress = sealed band-crossing events from Phase 5's emission bands (OC-K-DECISION-INGRESS-0 pattern) — never CPU branching on raw field/heatmap reads** (the naive-Phase-6 temptation is pre-empted by design). Fills the 12.4 `InTransit` readback seam ("test-private until authoritative movement readback exists"); the 12.5 icon descriptors start expressing real transit with zero icon-layer change (the renderer-seam design anticipates this). |
| 7 | **Combat as RF arena** (Owner-added 2026-07-20) | The axiom made mechanical: combat = RF arena resolution (HP/damage as resource flows). Promote the R6/R6B/R6C dress rehearsals from falsification-oracle to executed default via the proven RF promotion pattern (oracle-first → executed → legacy/rehearsal re-anchored); RF-1-style conservation judging; rehearsals retain oracle role against the executed path. |
| 8 | **[HARNESS]** RFH-2 `RF-COLUMN-MINT-MIGRATE-0` | **Migration sweep (= OC-K2.1b) — deliberately LATE (Owner ruling 2026-07-26): Phase 4 makes row/col identity object semantics, so the sweep runs ONCE onto the FINAL object-semantic doors.** Scope per the DA exclusion audit (2026-07-26, all 18 exclusions dispositioned): **A** law-itself (`column_index`/core `registry`/`accumulator_op*` — the role pathway + fused-kernel plumbing; exclusions dissolve with the Phase 0 taxonomy) · **B** GPU round-trips (~8 files: `arena_allocation_plan`, `resource_economy_compile`, `transfer/emission/intensity_accumulator`, silo/link compiles, `region_field_admission`) → **type the plan structs** (`ColumnIndex` end-to-end; raw u32 only at the single WGSL encode/decode boundary) · **C** genuine violations: `gated_rates` gate evaluation → **authored EML library stack** + role-pathway columns; `first_slice_mapping_runtime` (hardcoded `eml_resource_col=1`/`eml_output_col=4` magic numbers; a hand-rolled P0 recursive cycle) → **deleted, superseded by P0 derivation** · **D** oracles/rehearsals (`cpu_oracle` ×2, R6 family) → fenced raw door (judging independence requires raw mint) · **E** era-0080 (~86 mints) → **already EXCISED pre-track 2026-07-26** (Owner-directed: 18 src + 5 test files deleted, exclusions removed, inventory reconciled). Endgame unchanged: **exclusion list deleted; tripwire retired; full build + RF batteries green.** |
| 9 | **[HARNESS]** Doctrine-CI reconciliation (Owner-added 2026-07-26) | **The rustification CI must reflect the doctrine as it emerged, not as it was.** Standing rule from day one: **doctrine-CI co-evolution** — any rung that changes doctrine carries its scan/anchor/digest/selftest-fixture updates IN THE SAME PR (the graduation-stamp philosophy: reconcile at merge, never batch; the #1434 stale-digest red is the tripwire proving CI notices). This phase is then the residue sweep, not a migration: (a) retire/demote detectors whose watched violation the new type boundaries made impossible (beyond RFH-2's `COLUMN-INDEX-MINT`: audit every scans.tsv row + doctrine anchor against the landed object model); (b) mechanize the NEW gates the constitution created — EML library additions (opcode/stack) route DA-RESERVE like gate-wiring, telemetry-lifecycle binding check, Root-Contract admission-error family coverage; (c) re-point anchors/digests/orientation at the canonized §-numbers; (d) selftest fixtures prove the updated gates fire; (e) **allowlist re-baseline (Owner-added 2026-07-26):** the `scripts/ci/allow/*.txt` door-class records are reconciled to the emerged doctrine — their standing promotion-blockers ("retire when … closed by type-boundary admission") come DUE as each phase lands its boundary, and rows retire in the same PR (co-evolution rule); end-state falsifier: `kernel_surface.txt` (227 authority-exports at track-open) shrinks to the closed post-admission set. **Standing allowlist admission standard (in force from track-open, ahead of this phase):** new entries admit ONLY along the emerged SimThing paths — RF/simulation/resource-evaluation symbols must conform to the Root-Contract RF paths (recursive cycle + EML library doors, never bespoke evaluation exports); row/col symbols only via the Phase 0 constructor-taxonomy standards; event emission listeners / boundary events only via the sealed threshold/band-crossing machinery (threshold registrations, sealed post-RF appends, band-crossing event kinds — the decision-ingress pattern). Non-conformant additions route DA-RESERVE and are refused absent DA sign-off. Falsifier: zero scans watching impossible states; zero new doctrine surfaces unwatched; selftest battery green on the reconciled set. |
| 10 | Portability proof + CORE canonization (P6) | Non-TP domain works untouched; object model — **P0 Root Contract, recursive cycle, and EML-ISA laws included** — written into `simthing_core_design.md` (HARNESS phases excluded — they canonize nothing); SimThing complete and distinct. |

## 4. Binding laws (carry from day one)

**The Consumer Law (Owner mandate, 2026-07-20).** STEAD, PALMA, Gu-Yang, and Studio observation
are STANDING CONSUMERS of all data accumulated at SimThings — accumulated data is never walled
off as hygiene ("waiting for a consumer" is not a state that exists; STEAD is the consumer).
Hygiene disciplines govern PROOF (source loci cannot close an OVL or prove a flow) and MUTATION
(no side-channel writes) — never READ ACCESS. CPU-side consumer reads route through the one
canonical generic read seam (hosted SimThingId + typed PropertyKey/role, role-pathway loci,
coherent per-tick snapshots); observation never becomes CPU decision branching — decisions stay
on-device threshold/band crossings. **The Consumer Law does NOT shield tests from lifecycle:**
it governs production read access, never test/evidence retention — almost no test outlives its
workplan/track (default-delete at closeout stands) unless deleting it breaks production code;
"STEAD might consume this someday" is not a lifecycle exemption. Born from the 12.3 stop: the 12.2→12.8 horizon seam was
dropped and the disruption map sat walled at 0.0 while macro disruption ran ~8446.
**EML is the SimThing ISA; the kernel is its microarchitecture (Owner-ruled 2026-07-26).**
Core derivable functions are canonically EXPRESSED as **sealed EML stacks** in a core library —
the queryable, predictable spec every descendant pulls (including emission-band threshold
branching). The lowerer recognizes well-known core stacks and EXECUTES them as the existing
fused kernels (reduction passes, accumulator ops): the spec is inspectable, the invariant hot
loop pays zero interpreter cost. Authored/variable logic — bands, urgency, allocation weights,
specialization behavior — stays interpreted EvalEML. **Library growth law:** the library grows
freely in posture, but **every addition — opcode or stack — is DA-approved**; core stacks are
sealed (composable, never shadowed); the consuming-stack requirement is WAIVED for
horizon-targeted additions, which must carry a dated `HORIZON-ENTRY(iso-date)` marker (undated
horizon seams were dropped twice in 0.0.8.6 — dated markers are what the tripwires can assess).
**Call-tracking telemetry:** per-slot × per-stack invocation profiles are always-on **iff bound
to an existing checked/cleared lifecycle of matching scope** (bind candidate at implementation:
the session observation/snapshot lifecycle); if no such lifecycle exists, fall back to global
per-stack-id counters always-on + per-slot profiles as an opt-in observation channel. Telemetry
is **write-only with respect to the sim** — counters never feed any tick computation (bit-exact
replay; decisions stay on-device). The call profile is a standing Consumer-Law surface.
**Bespoke parallel systems are constitutional violations, not conveniences.** Named
transitional debts with scheduled promotions: the R6/R6B/R6C combat rehearsals (Phase 7) and
the movement readback seam (Phase 6). Anything new of that shape fails admission.
**Emergence-first (the prize law).** Every phase carries a 12.10-style emergence falsifier:
authored-parameter changes must produce qualitatively different, UNSCRIPTED macro outcomes; a
phase that leaves the mechanism sound but the dynamics inert — over-quantized bands, over-
constrained admission, scripted outcomes, dead attractors — FAILS its exit-proof regardless of
green checks. P1 bounded-horizon and P2 one-shared-rule are preserved as the conditions under
which Wei’s emergence generalizes (per the paper); band ladders quantize the READING never the
FIELD, so the continuous dynamics that carry the emergence are never discretized away.
Promotion, not rewrite — existing code keeps working at every rung. Intrinsic ≠ implicit:
all automatic behavior is derived at admission, typed, spanned-erroring — never runtime magic.
Oracle-judged: RF-1 conservation + bit-exact replay at every phase. Deferred capabilities from
the RF-5A rulings (mid-session authored refresh; authored complete arena composition) land
here, not as scenario patches.

## 5. Open conditions

1. ~~0.0.8.6 TP chain closed~~ **SATISFIED** — 0.0.8.6 CLOSED 2026-07-25 (receipt `21f77abb55c9`, PR #1431).
2. ~~RF-harness (0.0.8.4.8.4.2) disposition~~ **RULED 2026-07-26** — folded into this track: RFH-1 = Phase 0 (guard, early, non-breaking), RFH-2 = Phase 8 (sweep, late, after Phase 4's object model). The separate track is dissolved; `design_0_0_8_4_8_4_2_RF_harness.md` reduced to a superseded pointer.
3. Owner commits the full plan (rung decomposition of §3, including any further RF drift interventions — guards early, sweeps late); then `gen_orientation.sh --open`.
