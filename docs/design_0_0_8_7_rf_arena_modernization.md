# 0.0.8.7 — RF Arena Modernization (the SimThing unification)

> **Status: COMMITTED / OPEN (Owner commitment 2026-07-26).** Track opened via
> `gen_orientation.sh --open`; the active pointer is on this doc. Rung ladder = §3b
> (dispatch from §3b ONLY; §3 phase rows carry the design detail each rung references).
> Absorbs the former 0.0.8.4.8.4.2 RF-harness track ([HARNESS] phases; §5.2).
> Next-Rung pointer governs; ONE rung in flight at a time.
>
> **CORE track.** At completion this is **built INTO THE CORE**: the canonized object model
> lands as constitutional sections of `simthing_core_design.md`, and the **SimThing object is
> complete and distinct** — one concrete, working, GPU-resident simulation kernel.

## 1. North star

Unify the SimThing principle: **a SimThing with resource properties, parented into a tree,
simulates — intrinsically, recursively, on the GPU — with zero scenario-side wiring.** The
recursive RF arena regulating resource economics and the **Field Triad** — Wei's STEAD automata
as spatial heatmaps, PALMA potentials, Gu-Yang saturating flux (P5: one sweep mechanism, three
conservation classes) — ARE the Sim in SimThings; this track makes that a property of the *object*, not of
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
  properties + a parent edge; (b) tree-wide effects queryable as GPU sweep reductions (including the P5 Field Triad — influence, potentials, flux — whose outputs are anchored columns);
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
  last-crossing generation, urgency, falloff params) maintained solely by admission + the write door
  — data-shaped, never a listener framework. Band ladders are **value-domain-generic**:
  boolean, integer, and float/complex ladder variability is anticipated and exposed as coding
  paths (per-domain EML evaluation paths), never intrinsic scalar assumptions — anything the
  SimThing GPU matrix stack holds gets free auditability, STEAD actionability, and parallel
  evaluation. **Two fences keep the rider lightweight (Owner-affirmed 2026-07-26):
  (i) the matrix stays HOMOGENEOUS lanes — domains are admission-tag interpretations, never
  storage types; wider values (complex, 64-bit) are multi-lane properties via the existing
  layout width machinery; tagged-union / templated / heterogeneous cell storage is a
  constitutional violation (it forfeits the dense-sweep economics the lightweight guarantee
  rests on). (ii) band ladders bind ONLY to ordered scalars — non-ordered domains reach the
  ladder through sealed EML projection stacks (magnitude / phase / component: one comparator,
  N projections; "new domain" = new projection stack at library speed, zero object or kernel
  change). Complex-magnitude projections needing deterministic sqrt route through the
  sanctioned Candidate-F exact-magnitude door (bit-exact; cf. the workshop sqrt-candidate
  track) rather than minting a new float path. Agents reaching for cell-storage polymorphism
  are CAUGHT, not argued with: a heuristic tripwire (Phase 0 ride-along) flags the reach and
  logs it to the reach log — accumulated reaches are the evidence base for any future
  expansion of this ruling.** The STEAD model rides the recursive architecture for free: anchored emission
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
  are the same clause. The cycle resolves **within a single generation as ordered passes**
  (OrderBand), never pipelined across generations. It already executes as
  `RecursiveArenaResourceFlow` — this track binds it to the *object*.
  **The atomic SimThing tick (Owner ruling, 2026-07-26):** the constitutional time unit is
  the **generation** — exactly one run of the reduce → overlay → disburse → STEAD-update
  cycle. Calendar and day semantics, pacing, and scheduling are **front-end cadence
  bindings**, never constitutional: the Stellaris/Clausewitz day-tick is clausething's
  binding (`day := generation` + pacing/calendar), coexisting for now but not locked in.
  The kernel is already calendar-free (gpu: 0 day-refs; kernel: 1 comment); the core's
  residual calendar vocabulary (`evaluate.rs` `day` field, spawn-day) is renamed to
  generation terms as a Phase 1 ride-along. Two **execution postures** over ONE kernel:
  **paced** (front-end-scheduled generation barriers — the game model, default) and
  **continuous** (free-running batched generations; the CPU semantic side is a submission
  pump + observer). Posture is a scheduling policy, never a kernel fork.
  Falsifier: when P1 lands, the synthetic arena-participation wrapper kind is
  **eliminated**, not retained as a compatibility fossil — the wrapper existing was the
  symptom of participation-by-wiring; its absence is the proof of
  participation-by-derivation.
- **P1 — Intrinsic RF kernel at the object.** Resource properties + a parent edge ⇒ arena
  participation **derived at admission** (typed, inspectable, spanned hard-errors;
  `DefaultDisabled` opt-out retained). Behavior when parented and as a parent is intrinsic,
  natural, recursively automatic. Collapse the config seams (ResourceFlowSpec / ArenaRegistry /
  execution profiles / registrations) into the derivation.
- **P2 — Overlays as THE data/settings/directives layer (Owner re-grounding, 2026-07-26).**
  Canonize overlays with a full, **living API surface** — one modification law from authored
  data to operator input, per core design §6 ("every modifier is an overlay; there is no other
  modification mechanism"). The canonized taxonomy, so agents cannot re-interpret it:
  **(1) Capability bestowals** — the Capability Tree contract per `capability_tree_v1.md`:
  a tree is ONE SimThing child of its owner (never a tree of SimThings); entries are
  progress/rate sub-field pairs (GPU-tracked, threshold-detected, **anchored per P0(e)** —
  unlock proximity is observable) plus one **suspended overlay** per entry; unlock = threshold
  crossing → prerequisite gate → `ActivateOverlay` at the generation boundary; **tiered AND
  mutual prerequisites are admission-validated data** (typed DAG — cycles/dangling refs are
  spanned admission errors; the runtime gate check stays boundary work), with mutual
  exclusivity (`max_active`) enforced by same-barrier sibling suspension — atomic at the
  generation. **The effect-target discipline is load-bearing** (ADR
  `capability_effect_target_scope.md`): overlay-prep applies by TREE POSITION, so every effect
  overlay must live on its resolved host (`EffectTarget` Owner / tree / root via
  `overlay_hosts`) — the documented v0 silent-misdelivery trap is CLOSED by canonization, not
  re-documented. Bestowed values with `EffectTarget::Owner` write the **owner SimThing's
  policy/weight columns** — a bestowal IS a standing policy delta at the operator's seat.
  Fission inheritance rides `capability_container_kinds` / `clone_capability_children`.
  **(2) Standing policies** — overlays writing weight columns read by the allocation sweep:
  "a policy IS its numeric pressure on the flow" — never a policy enum, never a branch.
  **(3) Lifecycled transient effects** — `Transient{DissolveConditions}` overlays (property
  thresholds, generation timers, override) over the tree RF evaluation; activation/dissolution
  is boundary-protocol work at generation barriers (under continuous posture, via the Phase 6
  async command queue with logged injection generations), never mid-generation mutation.
  **(4) Operator directives** — `OverlaySource::Player`, structurally IDENTICAL to AI policy:
  **orders are price injections, never command channels.** The canonical example is binding:
  a user-selected destination = a dominant weight overlay (e.g. +10000) on the fleet's need
  columns — the fleet still behaves entirely by STEAD banded commitments, but the price
  dominance makes the outcome undeniable, so it FEELS like a responsive order while remaining
  lawful emergence. Dominance magnitudes are an **authored order-weight class** (sanctioned
  band that dominates ambient prices; finite — never infinities, preserving RF-1 envelopes +
  replay). Directive latency = decision-ingress latency (band crossing at the next
  generation), giving orders their responsive feel by construction.
- **P3 — Specialization protocol.** Specialized SimThings (spatial, owner, session, …) get a
  protocol **richer than a kind enumeration** — but it **must not break existing kind code**.
  Promotion, not rewrite; every rung keeps the existing corpus green (compatibility falsifier:
  full existing test suite passes unmodified at every rung).
- **P4 — GPU-residency intrinsic to the object.** SimThings ARE dense-matrix rows: the
  row/column setup is as intrinsic as RF flow, property-value emission bands, and thresholds.
  **A SimThing knows how to slot into a parent, and how child rows are allocated relative to
  it.** Slot/column identity flows from the object model through the role pathway — never
  minted beside it (consumes the Phase 0 column admission-gate; the Phase 9 sweep then
  migrates legacy sites onto the object-semantic doors this phase defines).
- **P5 — Intrinsic field influence: the Field Triad (Owner PALMA + Gu-Yang elevations,
  2026-07-26).** Field propagation over the lattice is ONE sweep mechanism parameterized by
  semiring — three canonical instances spanning the conservation classes: **STEAD**
  (non-conserved signal, superposition/accumulate — the heatmaps), **PALMA** (selection,
  `(min,+)` relaxation — potential field `D` from impedance `W`; lineage: the PALMA
  tropical-algebra paper, algebra borrowed, ARM implementation explicitly not), and **Gu-Yang**
  (conserved **saturating flux** — per `stead_spatial_contract.md` §7 a conservative-flux
  stencil, never a border/frontline semantic service). Boolean reachability (`(OR,AND)`) rides
  the P0(e) domain rider; future domains = a fourth semiring tag at library speed.
  Resource accumulation on spatial SimThings emits influence falloff as *object behavior* (not
  authored emitters); `W` is a **sealed EML cost projection over observables** (STEAD bands,
  overlay-modified values, RF pressure) — "arbitrary value pathing" = new projection stack +
  semiring tag, zero kernel change. All field outputs are ORDINARY property columns
  (homogeneous lanes) and therefore **born STEAD anchors** per P0(e) — triad outputs are STEAD
  inputs, recursively; STEAD commitments bind to their bands like any emission band (this IS
  the guaranteed STEAD access — no separate access mechanism exists or is needed).
  **Border/chokepoint semantics are DERIVED OBSERVABLES, never services (the Gu-Yang border
  door — positive complement to the withdrawn doctrine):** Gu-Yang's saturation IS the border
  mechanism — opposing conservative fluxes meet and stall, and the stall locus is the front.
  The sanctioned derivations are sealed EML comparative projections over co-located field
  columns: **dominance** (argmax over competing emitter classes + margin top1−top2 — 2-3
  columns regardless of N owners; deterministic authored tie-break), **border** (sign-flip
  band of the margin), **contest/border-pressure** (flux-stall magnitude; both-strong at small
  margin), **chokepoint** (band conjunction: contested border ∧ PALMA low-`D` corridor). Each
  is an anchored column — front-formed/hardened/chokepoint-emerged arrive as sealed band
  crossings for free. Border/contour/frontline SERVICES (tracers, polyline engines, border
  objects) are constitutional violations — polyline extraction is presentation-side
  projection/cache only (§10); a HEURISTIC border-service tripwire rides Phase 0.
  **Default-derived guarantee (the first-order elevation):** spatial specialists ASSUME the
  triad — admitted stencil topology ⇒ PALMA potentials derived; ≥2 admitted competing emitter
  classes ⇒ dominance/margin/contest projections **born, not wired** (default-on like anchors;
  authored opt-out; dark-cell visibility). Movement descends PALMA potentials; fronts are
  Gu-Yang-derived observables — movers are field samplers, never planner clients; all triad
  fields are eventually-consistent (Wei P1 finite information speed), never instant-query
  services (CPU oracles retained for tooling/proof only).
  **The PALMA pathing door (whitelist law):** any quantitative pathing between nodes derives
  through the PALMA semiring field sweep over admitted stencil topology — the SOLE sanctioned
  door, operationalizing §7's "no CPU map planner"; `D` is a field, not a route (no
  predecessors, no path objects). Bespoke graph-search in production crates
  (A*/Dijkstra/priority-queue search) is a constitutional violation; a HEURISTIC tripwire
  (Phase 0 ride-along) flags such reaches to the reach log.
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
| 0 | **[HARNESS]** RFH-1 `RF-COLUMN-ADMISSION-GATE-0` + hygiene prerequisites | **Type boundary (= OC-K2.1a; kernel lane, Owner-gated K2 lineage) — lands NON-BREAKING before core work begins.** `ColumnIndex` constructor taxonomy in `simthing-core`: `new` remains a deprecated public compatibility alias delegating to the fenced raw door; legal doors = layout-derived paths (`PropertyLayout::offset_of` / `col_for_role` / arena-layout ranges) + two doc-fenced choke points (GPU round-trip constructor; fenced raw door for oracle/rehearsal code — existing ~84 call sites keep compiling untouched, only NEW code is steered). Retarget `COLUMN-INDEX-MINT` to the choke-point tokens; DA-gate exclusion edits on the scan. Rides along: execution-status taxonomy `executed / oracle / rehearsal / compile-plan`, board-surfaced; **cell-storage-polymorphism tripwire (P0(e) fence i)** — HEURISTIC scan flagging tagged-union/templated/heterogeneous matrix-cell reaches, logged to the reach log as the evidence base for future ruling expansion; **bespoke-pathfinder tripwire (P5 PALMA door)** — HEURISTIC scan flagging A*/Dijkstra/priority-queue graph-search reaches in production crates, logged to the reach log; **border-service tripwire (P5 Gu-Yang door)** — HEURISTIC scan flagging border/contour/frontline-service reaches (tracers, polyline engines, border objects) in production crates, logged to the reach log. Falsifier: post-rung, the scan proves every remaining `::new` is inside the fence. **Interim standing order (in force NOW, until this phase lands): `COLUMN-INDEX-MINT` exclusion additions are FROZEN — DA sign-off required; peer-citation is not a valid justification.** |
| 1 | Intrinsic RF kernel (P1) | Derived-at-admission participation; config-seam collapse (incl. `min_plus_traversal_field` enable/disable — same session-wiring family); RF-1 oracle + determinism judge every rung; core calendar-vocabulary rename ride-along (P0 generation ruling). |
| 2 | Overlay API canonization (P2: the four-family overlay law) | Canonizes the P2 taxonomy — capability bestowals / standing policies / lifecycled transients / operator directives — as ONE living surface. Concrete obligations: **close the effect-target v0 trap** (ADR `capability_effect_target_scope.md` — host placement + `overlay_hosts` become the admission-checked canonical path; silent misdelivery becomes a spanned error); **admission-validate prerequisite DAGs** (tiered + mutual; cycles/dangling = spanned errors; `max_active` same-barrier atomicity); **generation-align the unlock/activation boundary protocol** (day-boundary language retired per the P0 generation ruling; continuous posture routes through Phase 6's logged command queue); **land the authored order-weight class** (finite dominance band for operator directives — the "+10000 feels like an order" mechanism); capability progress sub-fields anchored per P0(e). Exit-proof: a capability unlock, a policy shift, a timed crisis, and a user destination-order all express as overlays through the one surface — zero bespoke channels — and the misdelivery fixture (effect authored against a host that lacks the property) hard-errors at admission instead of silently landing. |
| 3 | Specialization protocol (P3) | Richer-than-kind, kind-compatible; spatial/owner/session first citizens. |
| 4 | Intrinsic GPU residency (P4) | Row/col + parent-slotting + child-row allocation as object semantics. |
| 5 | Intrinsic field influence (P5: the Field Triad — STEAD + PALMA + Gu-Yang) | Falloff emission + heatmap interaction + Gu-Yang/PALMA consumption; 5.4-5.8 carry the Owner-approved FIELD-SWEEP map/fold remodel (adjacency axis, semiring rung withdrawn — full P5 pillar rewrite lands with 5.5). **STEAD-as-emission-bands (Owner correction 2026-07-20):** STEAD influence on a parent SimThing is READ as **emission-band thresholds** — a banded ladder of threshold registrations on the accumulated cell (existing machinery composed: N band edges = N threshold registrations, each with its own event kind/band id; the RF-5A append-scan proved multi-scan sequencing). Bands bind **1:1 to the falloff influence bands in the heatmap**; intensity (band), velocity (banded thresholds on the GovernedPair rate cell), and magnitude (value) are all observable/measurable through existing property mechanisms. **Quantize the READING, never the FIELD**: falloff superposition + conservation math stay continuous; bands are the observation/decision/coupling surface only — band edges must not enter accumulation math (replay determinism + RF-1 envelope). **Implements the P0(e) STEAD fulcrums (Owner ruling 2026-07-26):** (1) anchor disposition at admission — default-Anchored, authored `Unobserved{reason}` opt-out, dark-cell board surface; (2) write-door impact — band-crossing deltas derived in the fused mutation pass; fission/fusion + table-reallocation ops refuse to encode without anchor remaps; (3) the derived anchor table as the SOLE observation surface for heatmap/Gu-Yang/PALMA/Studio/telemetry; (4) domain-generic ladders — bool/int/float-complex band evaluation as per-domain EML coding paths. **Implements the P5 PALMA elevation (Owner ruling 2026-07-26):** semiring tag on field registrations (accumulate / min-plus / Boolean over ONE sweep mechanism); `W` = sealed EML cost projection over observables; `W`/`D` land as ordinary anchored property columns (D-field bands emit corridor events for free); promotes `min_plus_traversal_field` from its enable/disable session-wiring posture to derived-at-admission (the P1 kill-list pattern) — the existing GPU-resident dispatch + CPU oracle become the executed instance + judge. **Implements the P5 Gu-Yang elevation (Owner ruling 2026-07-26):** dominance/margin/contest/border-band/chokepoint as sealed EML comparative projections over co-located triad columns (all anchored -> free front events); default-derived for spatial specialists with >=2 admitted competing emitter classes (born, not wired; authored opt-out); deterministic authored tie-break at margin==0; `stead_spatial_contract_guards` + mapgen batteries stay green every rung (contract section 8). |
| 6 | **Generation spine & async transport** (Owner-added 2026-07-26) | Implements the P0 atomic-tick ruling and pre-empts the async hazards of continuous posture. (a) Every emission/threshold event carries an explicit **generation stamp** — today's transport timestamps implicitly via the per-tick CPU readback cadence (a hidden day-tick dependence; `EmissionRecord` carries `reg_idx`+`emit_count` and no time at all); (b) event egress = a generation-stamped **ring/stack with an admission-time backpressure policy** (coalesce-per-band / overwrite-oldest / throttle) — observer lag must NEVER stall or perturb the sim; (c) CPU semantic actions become an **async command queue** landing at the next generation barrier, injection generations LOGGED (bit-exact replay under asynchrony); (d) shadow trees and telemetry read **generation-consistent snapshots** (double-buffered, no torn reads); (e) Field-Triad band events (need, corridor, front, chokepoint) ride the SAME stamped egress — one transport for every observable family. On-device decision machinery (threshold/band crossings, EvalEML warp branching, sealed appends) needs NO redesign — it is already generation-local and CPU-free; this phase modernizes only the egress/ingress transport. Per-subtree multi-rate stepping = `HORIZON-ENTRY(2026-07-26)` — breaks Wei P2 synchrony + replay; observation recursion (per-level anchors disposing into one stamped stream: crossing density per subtree per generation window) is free and encouraged. Exit-proof: N-generation **continuous soak** with forced observer lag — zero sim perturbation, declared backpressure observed, replay bit-exact from the injection log. |
| 7 | **Movement-Front execution** (Owner-added 2026-07-20) | Fleet movement AUTHORITY: spatial SimThings move along STEAD need/falloff gradients (core §7 automaton; the FIRST full Triad consumer: driven by STEAD need bands, descends PALMA potentials, respects Gu-Yang fronts/chokepoints — all via sealed band crossings, never field reads; legacy swapped labels corrected 2026-07-26) under the ordinary tick — decisions stay threshold crossings, movement state is sim authority. **Decision ingress = sealed band-crossing events from Phase 5's emission bands (OC-K-DECISION-INGRESS-0 pattern) — never CPU branching on raw field/heatmap reads** (the naive-Phase-6 temptation is pre-empted by design). Fills the 12.4 `InTransit` readback seam ("test-private until authoritative movement readback exists"); the 12.5 icon descriptors start expressing real transit with zero icon-layer change (the renderer-seam design anticipates this). |
| 8 | **Combat as RF arena** (Owner-added 2026-07-20) | The axiom made mechanical: combat = RF arena resolution (HP/damage as resource flows). Promote the R6/R6B/R6C dress rehearsals from falsification-oracle to executed default via the proven RF promotion pattern (oracle-first → executed → legacy/rehearsal re-anchored); RF-1-style conservation judging; rehearsals retain oracle role against the executed path. Combat context (where fronts harden, which chokepoints contest) arrives as Triad band observables — consumed via bands, never via border/path services (Triad Doors law). |
| 9 | **[HARNESS]** RFH-2 `RF-COLUMN-MINT-MIGRATE-0` | **Migration sweep (= OC-K2.1b) — deliberately LATE (Owner ruling 2026-07-26): Phase 4 makes row/col identity object semantics, so the sweep runs ONCE onto the FINAL object-semantic doors.** Scope per the DA exclusion audit (2026-07-26, all 18 exclusions dispositioned): **A** law-itself (`column_index`/core `registry`/`accumulator_op*` — the role pathway + fused-kernel plumbing; exclusions dissolve with the Phase 0 taxonomy) · **B** GPU round-trips (~8 files: `arena_allocation_plan`, `resource_economy_compile`, `transfer/emission/intensity_accumulator`, silo/link compiles, `region_field_admission`) → **type the plan structs** (`ColumnIndex` end-to-end; raw u32 only at the single WGSL encode/decode boundary) · **C** genuine violations: `gated_rates` gate evaluation → **authored EML library stack** + role-pathway columns; `first_slice_mapping_runtime` (hardcoded `eml_resource_col=1`/`eml_output_col=4` magic numbers; a hand-rolled P0 recursive cycle) → **deleted, superseded by P0 derivation** · **D** oracles/rehearsals (`cpu_oracle` ×2, R6 family) → fenced raw door (judging independence requires raw mint) · **E** era-0080 (~86 mints) → **already EXCISED pre-track 2026-07-26** (Owner-directed: 18 src + 5 test files deleted, exclusions removed, inventory reconciled). Endgame unchanged: **exclusion list deleted; tripwire retired; full build + RF batteries green.** |
| 10 | **[HARNESS]** Doctrine-CI reconciliation (Owner-added 2026-07-26) | **The rustification CI must reflect the doctrine as it emerged, not as it was.** Standing rule from day one: **doctrine-CI co-evolution** — any rung that changes doctrine carries its scan/anchor/digest/selftest-fixture updates IN THE SAME PR (the graduation-stamp philosophy: reconcile at merge, never batch; the #1434 stale-digest red is the tripwire proving CI notices). This phase is then the residue sweep, not a migration: (a) retire/demote detectors whose watched violation the new type boundaries made impossible (beyond RFH-2's `COLUMN-INDEX-MINT`: audit every scans.tsv row + doctrine anchor against the landed object model); (b) mechanize the NEW gates the constitution created — EML library additions (opcode/stack) route DA-RESERVE like gate-wiring, telemetry-lifecycle binding check, Root-Contract admission-error family coverage; (c) re-point anchors/digests/orientation at the canonized §-numbers; (d) selftest fixtures prove the updated gates fire; (e) **allowlist re-baseline (Owner-added 2026-07-26):** the `scripts/ci/allow/*.txt` door-class records are reconciled to the emerged doctrine — their standing promotion-blockers ("retire when … closed by type-boundary admission") come DUE as each phase lands its boundary, and rows retire in the same PR (co-evolution rule); end-state falsifier: `kernel_surface.txt` (227 authority-exports at track-open) shrinks to the closed post-admission set. **Standing allowlist admission standard (in force from track-open, ahead of this phase):** new entries admit ONLY along the emerged SimThing paths — RF/simulation/resource-evaluation symbols must conform to the Root-Contract RF paths (recursive cycle + EML library doors, never bespoke evaluation exports); row/col symbols only via the Phase 0 constructor-taxonomy standards; event emission listeners / boundary events only via the sealed threshold/band-crossing machinery (threshold registrations, sealed post-RF appends, band-crossing event kinds — the decision-ingress pattern); pathing/border/edge/dominance symbols only via the P5 Triad Doors (PALMA potential sweep; Gu-Yang comparative projections). Non-conformant additions route DA-RESERVE and are refused absent DA sign-off. Falsifier: zero scans watching impossible states; zero new doctrine surfaces unwatched; selftest battery green on the reconciled set. |
| 11 | **Embedder's Interface — the Vendor Door** (Owner-added 2026-07-26) | **Short phase.** Formalize the onboarding/entry surface for vendoring the SimThing simulation kernel into ANY domain (network saturation / load-balancing, finance, population models, …) — an easily **human-readable interface inheriting the stem-cell simplicity: five verbs mirroring the object's anatomy, and nothing else.** (1) **Derive** — declare specialists as data (P3 specialization spec: kind/custom + properties; no engine code); (2) **Populate** — build the tree + RF property values; admission does the rest (arena participation derived per P1, anchors born per P0(e)); (3) **Overlay** — the P2 authored data/settings/directives layer; (4) **Bind** — boundary event trees as band→commitment bindings + threshold registrations (incl. Triad bands: need/corridor/front/chokepoint — declaring competing emitter classes in (1) makes dominance/border observables BORN per P5, and admitted stencil topology makes PALMA potentials born), and **CPU shadow tree objects** = the canonical consumer read seam formalized as an embedder-facing handle (coherent per-generation snapshots; strictly read-only observation — decisions stay on-device, never a second authority); (5) **Run** — one standard initialize / start / tick / serialize lifecycle (bit-exact replay included) with an **execution-posture choice: paced | continuous** (P0 atomic-tick ruling; same kernel, scheduling policy only). **Scale-invariant by construction:** the same five verbs for one SimThing or a tree of thousands — a tree is a node with children; any scale-dependent special case in the interface is a defect. **`simthing-clausething` is repositioned as ONE front-end** that lowers to this interface; other domains write specs directly or bring their own front-ends — the interface is the layer clause authoring already lowers into, formalized and documented, not a new subsystem. Deliverables: (a) the thin API surface (existing admission/hydrate/session mechanisms, formalized); (b) a DOC-BUDGET-capped **Embedder's Guide** (cold-start human-readable, owner_authoring_guide lineage); (c) two micro-exemplar specs from non-game domains (e.g. 3-node finance toy; N-node network-saturation toy — which exercises the full Triad natively: congestion impedance → PALMA potentials, saturating link flux → Gu-Yang fronts, load bands → STEAD) that are admission-checked in CI and double as Phase 12's portability seeds; (d) the vendoring crate boundary (which crates an embedder takes, feature-gated). Exit-proof: a cold reader stands up a running, serialized, STEAD-observed tree from the guide alone — zero engine edits, zero scenario-side wiring. |
| 12 | Portability proof + CORE canonization (P6) | Non-TP domain works untouched **— scripted through the Phase 11 Embedder's Interface, which this proof exercises end-to-end**; object model — **P0 Root Contract, recursive cycle, and EML-ISA laws included** — written into `simthing_core_design.md` (HARNESS phases excluded — they canonize nothing); SimThing complete and distinct. |

## 3b. Committed PR ladder (Owner commitment, 2026-07-26)

**Cadence and protocol (binding):** ONE rung in flight at a time; handoffs via
`handoff_dispatch.sh` (HD-RECEIPT, projection/ingress caps); the orchestrator owns the
verification tier (exact-head clearance, remands); the DA rules once per relay and stamps
graduation AT MERGE (pointer is machine truth); scenario-born candidate code homes in
`simthing-workshop` (§12); default-delete at closeout stands. Corpus-green law: hosted green ≠
corpus green — GPU/cargo batteries run at the head before graduation.

**Greenfield discretion charter (Owner, 2026-07-26):** much of this ladder reconfigures known
SimThing concepts, but where a rung builds **greenfield surfaces** — the anchor table, the EML
core library, the generation transport spine, semiring tags, comparative projections, the
embedder API — the DA is directed to **seize the opportunity for a stronger, more elegant,
more performant core**. Canonical-simplicity choices within the binding laws need no Owner
ping; anything that would alter a §2/§4 law escalates. The projected anchor library
(`core-0087` domain) pre-seeds this guidance to every coder and orchestrator.

**Coder-lane column (Owner directive 2026-07-26; assignments are DATA per the roles-are-slots
doctrine, revisable without ceremony):** `Frontier — Codex 5.6` (Fable may implement at DA
discretion); `Std — Grok CLI` (`grok-4.5` pinned); `Fable` = DA-implemented harness work.

| Rung | ID | Scope (details in the phase row) | Exit proof | Tier — Coder | Status |
|---|---|---|---|---|---|
| 0.1 | `RF-COLUMN-ADMISSION-GATE-0` | Phase 0: `ColumnIndex` constructor taxonomy, NON-BREAKING (fenced raw door); retarget `COLUMN-INDEX-MINT` to choke-point tokens; mechanize DA-gated exclusion edits. | **DA-GRADUATED / merged [#1447](https://github.com/khorum08/SimThing/pull/1447) @ `4db8ac14`** — ColumnIndex admission doors; scan proves remaining `::new` inside fence; corpus green. | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1447** |
| 0.2 | `EXECUTION-STATUS-TAXONOMY-0` | Phase 0: `executed / oracle / rehearsal / compile-plan` classification as data; board-surfaced. | **DA-GRADUATED / merged [#1448](https://github.com/khorum08/SimThing/pull/1448) @ `407850e2`** — export-module census; DA mixed-posture primaries (reduction=`compile-plan`, world_state=`executed`; relay `5085410594`). | Std — Grok | **DA-GRADUATED / merged #1448** |
| 0.3 | `CONSTITUTION-TRIPWIRES-0` | Phase 0: the three HEURISTIC tripwires (cell-storage polymorphism; bespoke pathfinder; border service) + reach-log wiring; DA 0.2 primary-class fold + Board/orient pointer repair. | **DA-GRADUATED / merged [#1450](https://github.com/khorum08/SimThing/pull/1450) @ `1b7e2432`** — fixtures fire; INSPECT-only; reach-log lands; `mixed_ruled=2`. | Std — Grok | **DA-GRADUATED / merged #1450** — ruling `5086131411`; `HD-RECEIPT: bf9ece38aea8`. |
| 1.1 | `ROOT-DERIVE-PARTICIPATION-0` | Phase 1: admission derives arena participation from resource properties + parent edge (typed, spanned; `DefaultDisabled` retained); collapse the default-path config seams. | **DA-GRADUATED / merged [#1453](https://github.com/khorum08/SimThing/pull/1453) @ `488ad2c9`** — zero explicit arena wiring; RF-1 + replay referees unedited, battery 108/0. | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1453** — corpus battery 108/0 at `8333ec56`; `HD-RECEIPT: d0110f30ed12` |
| 1.2 | `SESSION-WIRING-KILL-SWEEP-0` | Phase 1: kill remaining opt-in toggles incl. `min_plus_traversal_field` enable/disable → derived-at-admission; core calendar-vocabulary rename (`evaluate.rs day`, spawn-day → generation). | **DA-GRADUATED / merged [#1456](https://github.com/khorum08/SimThing/pull/1456) @ HEAD `dc53dfa8`** — toggles dead (constructor census 0 tree-wide); referee `mapgen_palma` unedited 19/0; serde aliases exactly `FieldSnapshot.day` + `SimThing.spawned_day` w/ round-trip tests; core day-refs = generation vocabulary | Std — Grok | **DA-GRADUATED / merged #1456** — corpus battery 108/0 reproduced at `dc53dfa8`; `HD-RECEIPT: b2a5237b0aa7`; ingress-cap sticky waived (wrapper-overhead defect → Phase 10) |
| 1.3 | `ARENA-PARTICIPANT-DEPRECATION-0` | Phase 1: eliminate the synthetic participation wrapper, its scaffold, and reserved-gap pools; admitted SimThings host flow properties on their own rows and carry resource-parent membership in the registry. | **DA-GRADUATED / merged [#1459](https://github.com/khorum08/SimThing/pull/1459) @ HEAD `cc29b588`** — ZERO ArenaParticipant refs tree-wide; ledger 53 sites ALL fixed-now (0 deferred); participants on own rows; sparse single-writer INPUT_LIST lowering (§10-conformant, no kernel edits) | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1459** — battery 112/0 reproduced at `cc29b588` incl. both live-GPU referees; economics fingerprint diff empty; 5-file surface amendment ADMITTED; `HD-RECEIPT: 28d2e8e276f2` |
| 2.1 | `OVERLAY-EFFECT-HOST-ADMISSION-0` | Phase 2: close the effect-target v0 trap — host placement + `overlay_hosts` admission-checked; misdelivery = spanned error. | **DA-GRADUATED / merged [#1461](https://github.com/khorum08/SimThing/pull/1461) @ HEAD `1d09fede`** — misdelivery = spanned admission error (overlay/host/property/scalar-token span); overlay_hosts==affects==physical placement proven; production GPU execution proof (Owner cell bit-exact 6.0); corpus census 7/7 admitted 0 mis-hosted | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1461** — battery 114/0 reproduced w/ exact adapter-pinned command; remedial referee 3/3; `HD-RECEIPT: dbace3440530` |
| 2.2 | `CAPABILITY-PREREQ-DAG-ADMISSION-0` | Phase 2: tiered + mutual prerequisites as admission-validated typed DAG; `max_active` same-barrier atomicity; generation-aligned boundary protocol. | **DA-GRADUATED / merged [#1463](https://github.com/khorum08/SimThing/pull/1463) @ HEAD `efcb1619`** — prereq DAGs admission-validated w/ spans; max_active same-barrier atomicity proven on the AUTHORITATIVE runtime tree; ship-size hydration fail-closed; existing corpus admits unchanged | Std — Grok | **DA-GRADUATED / merged #1463** — battery 114/0 + atomicity 2/2 reproduced at `efcb1619`; sim observation surface ADMITTED per Consumer Law; `HD-RECEIPT: 739c562571c0` |
| 2.3 | `ORDER-WEIGHT-CLASS-0` | Phase 2: authored finite order-weight class + `OverlaySource::Player` directive path (orders = price injections). | **DA-GRADUATED / merged [#1465](https://github.com/khorum08/SimThing/pull/1465) @ HEAD `c1e2bede`** — typed finite order-weight class; Player directives = ordinary overlays via the single feeder drain gate; canonical TP exemplar: dominance (2.5,27.5)→dissolve→(15,15), causal lifecycle, bit-exact reversibility twin, generation-stamped replay reinjection | Std — Grok | **DA-GRADUATED / merged #1465** — battery 119/0 + order referee 5/5 reproduced adapter-pinned at `c1e2bede`; dominance contract RULED install-envelope-at-admission (orders may be lawfully outbid — emergence, not defect); commitment-state oracle bound to 7.1 exit-proof; `HD-RECEIPT: 1b74667c3200` |
| 3.1 | `SPECIALIZATION-PROTOCOL-0` | Phase 3: the richer-than-kind protocol, kind-compatible (promotion not rewrite). | **DA-GRADUATED / merged [#1467](https://github.com/khorum08/SimThing/pull/1467) @ HEAD `da0dbe4f`** — profiles = typed admission data (KindIdentity; artifact-bound contracts: structural col+row stamps; field-economy policy/weight authority stamp; strict sole-direct-child root); ONE installed canonical report derives all three populations vs independent oracles; authored span provenance both error classes; pre-3.1 wire fixture; full suite UNMODIFIED | DA-reserve · Frontier — **Fable** (Owner routing 2026-07-27: Codex rate-limited) | **DA-GRADUATED / merged #1467** — 3 orch remands discharged; referee 8/8 + driver 119/0 + core 36/0 reproduced; surfaces hydrate_scenario.rs + spec/src ADMITTED; one-tree + §7-placement rulings CONFIRMED; `HD-RECEIPT: 691d281978da` |
| 3.2 | `FIRST-CITIZEN-SPECIALISTS-0` | Phase 3: spatial / owner / session as first citizens on the protocol. | **DA-GRADUATED / merged [#1470](https://github.com/khorum08/SimThing/pull/1470) @ HEAD `156f1570`** — authored `specialization=` on location+entity blocks w/ span capture; OWNER_POLICY_WEIGHT_AUTHORITY out-of-band mint guard (HEURISTIC, both authored formats, reach-log wired); citizen counts via generator (spatial=1500 owner-seat=2 session-root=1) | Std — Grok | **DA-GRADUATED / merged #1470** — new referee 5/0 + 3.1 referee 8/8 UNMODIFIED reproduced; battery 119/0 adapter-pinned (evidence-only delta verified); zero new requirement variants; `HD-RECEIPT: 11ddf57fcac0` |
| 4.1 | `ROW-SLOT-OBJECT-SEMANTICS-0` | Phase 4: parent-slotting + child-row allocation as object semantics through the role pathway. | **DA-GRADUATED / merged #1475 @ 272f82e2** — typed object-issued residency is the sole production structural row door; allocator-derived layouts match the legacy canonical + uneven fixtures exactly; install/AddChild/fission-clone/reparent/tombstone and sidecar rejection referees green; structural file splits landed with `mixed_ruled=0`; DA reproduced full corpus green (core 36/0, kernel 78/0, sim 35/0, driver 119/0/13 adapter-pinned RTX 4080/Vulkan). | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1475 @ 272f82e2** |
| 4.2 | `PLAN-STRUCT-TYPING-0` | Phase 4: exclusion-audit Family B — `ColumnIndex` end-to-end in plan structs; raw u32 only at the single WGSL encode/decode boundary. | **DA-GRADUATED / merged #1480 @ 6e7fa2b3** — Family B plan/compile intermediates typed end-to-end; production `from_gpu_round_trip` collapsed to `wgsl_encode`; typed-plan + wire-parity referees + seven-arm census green; DA reproduced corpus green (core 37/0, kernel 81/0, sim 35/0, driver 123/0/13 across 64 harnesses, RTX 4080/Vulkan; `mixed_ruled=0`) | Std — Grok | **DA-GRADUATED / merged #1480 @ 6e7fa2b3** |
| 5.1 | `ANCHOR-DISPOSITION-ADMISSION-0` | Phase 5: P0(e) fulcrum 1 — default-Anchored disposition; authored `Unobserved{reason}`; dark-cell board surface. | **DA-GRADUATED / merged #1485 @ 06103edf** — ordinary hydrate/compile/install assigns exactly one typed disposition to every resource property; canonical TP inventory generated from live registry state (`anchored=25`, `unobserved=0`, dark-cell set fixture-only per DA expectation); dark-property reason/span + blank-reason referees green; DA reproduced corpus green (core 37/0, spec 75/0, sim 35/0, referee 6/0, driver 123/0/13 on RTX 4080/Vulkan; censuses PASS, `mixed_ruled=0`). | DA-reserve · Frontier — Codex 5.6/Fable | **DA-GRADUATED / merged #1485 @ 06103edf** |
| 5.2 | `WRITE-DOOR-BAND-DELTA-0` | Phase 5: fulcrum 2 — in-pass band-crossing derivation; structural ops refuse to encode without anchor remaps. | **DA-GRADUATED / merged #1488 @ d41a079b** — typed BandCrossingDelta from the fused pass; refuse-to-encode law exact (fabricated remap_not_required spans vs independently derived keys; omitted/extra/duplicate/wrong-endpoint reject pre-encode); same-cell two-edge GPU referee replays bit-exact; cfg(test)-aware census + selftest landed (remand 4); DA verified census/selftest/write-door true-exit 0, referee 10/0, driver 133/0/13 across 65 harnesses on RTX 4080/Vulkan | Std — Grok | **DA-GRADUATED / merged #1488 @ d41a079b** |
| 5.3 | `ANCHOR-TABLE-SURFACE-0` | Phase 5: fulcrum 3 — derived GPU anchor table as the SOLE observation surface; consumers repointed (substrate scope; canonical live-host cardinality deferred to 5.3b). | Studio/telemetry read only the table; no second observation path (grep-proven); populated-fixture + production-sequence referees green; canonical install inventory exactly `25 Anchored / 0 Unobserved` with exactly `0` live loci (honest baseline for 5.3b); **DA-GRADUATED / merged #1491 @ d9544c52** — DA verified censuses true-exit 0, referee 13/0, studio 10/0, driver 146/0/13 across 66 harnesses on RTX 4080/Vulkan; two pre-existing legacy 0.0.8.3 doc-guard failures verified reproducing at base (not chargeable; routed to corpus debt) | Std — Grok | **DA-GRADUATED / merged #1491 @ d9544c52** |
| 5.3b | `CANONICAL-ANCHOR-MATERIALIZATION-0` | Phase 5: admission governs existence — host-materialization from value-placing relations only; hostless admitted properties declare `Unobserved{reason}`; no fixed Anchored/Unobserved count target; ordinary unmutated install proves TOTALITY | **DA-GRADUATED / merged #1500 @ 1294cc87** — admission governs existence; totality proven on the ORDINARY unmutated install (packs + overlays enabled): every admitted Anchored property has >=1 live host, derivation set empty, no repeated (thing, property), lawful multi-host preserved; derived inventory published (Anchored=18 / Unobserved=7) as result not target; DA reproduced core 47/0, sim 27/0 (one PRE-EXISTING failure carved out to 5.3c), 5.3b referee 4/0, unmodified 5.3 table 13/0, Studio 10/0, driver 150/0/13 on RTX 4080/Vulkan, three censuses true-exit 0 | Std — Grok | **DA-GRADUATED / merged #1500 @ 1294cc87** |
| 5.3c | `THRESHOLD-EVENT-REGRESSION-REPAIR-0` | Phase 5: repair the GPU threshold-event regression introduced by 5.3 (DA bisect: PASS at `d41a079b`, FAIL at `d9544c52`) — `s6_threshold_events_match_cpu_golden` reads back ZERO events where the golden expects one. Determine whether the emission path regressed or the direct-drive fixture's preconditions changed under 5.3, then repair the cause (never the golden). Evidence MUST include the full crate matrix: core, kernel, sim, driver, mapeditor. | **DA-GRADUATED / merged #1507 @ 462cc794** — root cause was the 5.3 convenience-dispatch dropping the `prepare_threshold_scan`/`finish_threshold_scan` lifecycle around the fused encode/submit; repair is that two-call restoration, golden and event payload unedited. DA reproduced at exact head: s6 green, write-door 10/0, anchor-table 13/0, materialization 4/0, sim suite green, detachability PASS (production=0), expiry PASS (expired=0), and the `--rungclose` gate itself verified intact through the reconstruction. | Std — Grok | **DA-GRADUATED / merged #1507 @ 462cc794** |
| 5.4 | `FIELD-SWEEP-IR-PROBE-0` | Phase 5: workshop-leaf disposable probe — minimal target/neighbor-ctx EML map/fold interpreter over a gather; publishes the measurement surface (adapter, adjacency kind, theater size, degree distribution, node count, actual stack depth, column reads/edge, time/sweep, edges/s, resource class) and re-derives current EML cap facts as ground truth; the N8 case runs on a THROWAWAY workshop gather (engine N8 stays 5.6 — accepted scope, lane held Std with eyes open); fallback pre-named (MIN × INPUT_LIST, PRODUCT × INPUT_LIST + banded flux). | Parity absolute; N4 generic median ≤1.25× bespoke, supported-adapter worst ≤1.5× at matched measured occupancy with stall counters confirming the memory-shadow claim; N8 cliff located; failure routes to specialization/JIT (IR retained as specification), never abandonment. | Std — Grok | TODO |
| 5.5 | `FIELD-SWEEP-N4-PARITY-0` | Phase 5: engine landing — EML edge context {target_slot, neighbor_slot, accumulator, edge_scalar, dt} + `TARGET_VALUE`/`NEIGHBOR_VALUE` opcodes under the EML growth law; `FieldSweepRegistration {adjacency, map_program, fold_program, identity_bits, post_program, FieldLawProof, CanonicalOrderProof, resource_class}`; `FieldLawProof` for conservative folds REQUIRES an undirected-symmetry certificate of the adjacency (conservation is a contract, not a compiler accident); `resource_class` defaults to the single legacy class (fixed 32-stack interpreter) and admission accepts ONLY that default until 5.7 defines classes; fixed linear fold order; grid offset-table order pinned as canonical authored data; `SEMIRING-FIELD-TAGS-0` WITHDRAWN in this diff (replacement registration with sealed proof metadata); P5 pillar + STEAD contract §10 amendment + `stead_spatial_contract_guards` co-evolve in the same PR.; land the permanent `FIELD-SWEEP-SINGLE-PATH` doctrine tripwire (grep-class, Track-A shape), two arms: (a) no algebra enum, field-kind tag, or operator-identity `match` in the sweep path; (b) new field-kernel `.wgsl` admitted only against the retiring seven-shader allowlist, which EMPTIES at 10.1 so any later bespoke field kernel FAILS the scan | PALMA + Gu-Yang reproduced bit-exact on N4 as authored instances ALONGSIDE the bespoke stencils; referees unedited; no semiring enum anywhere in the diff; corpus green.; `FIELD-SWEEP-SINGLE-PATH` tripwire green and self-tested (fires on a planted algebra enum and on a planted new field kernel) | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 5.6 | `FIELD-ADJACENCY-GENERATORS-0` | Phase 5: adjacency as a registration axis — weighted `GridOffsets [(dx,dy,w)]` with N4/N8/radius-r presets (N8 diagonal weights AUTHORED, not free) + LinkGraph over the existing INPUT_LIST; per-node weighted-degree/conductance χ certificate at admission (χ_i · Σ_j abs(c_ij) ≤ admitted bound); LinkGraph CanonicalOrderProof basis = the link compiler's existing sorted+deduped undirected neighbor lists; scheduling buckets are degree-homogeneous but NEVER author the physics and NEVER reorder a node's own neighbor list. | Emergence falsifier: the same authored map/fold on GridN4 vs GridN8 vs LinkGraph yields qualitatively distinct unscripted front geometry (diamond / octagonal / topology-following) — indistinguishable dynamics FAIL the rung on green checks; over-bound admissions span; corpus green.; ZERO production callers of the seven bespoke stencils remain (grep-proven) — they survive only as test-only migration oracles, which makes 10.1 a pure deletion and not a deferrable migration | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 5.7 | `EML-RESOURCE-CLASS-ADMISSION-0` | Phase 5: specialized interpreter resource classes (pipeline-constant stack sizes) and/or JIT-backed classes; universal node cap replaced by a measured resource-class budget ONLY once classes are measurable; `ExactPrimitiveAdmission` door — sovereign determinism key (specified bit semantics, domain/special-value policy, exhaustive reference artifact, supported-backend replay artifact) AND conjunctive cost key (resource class); at most one proven primitive iff a candidate with a concrete consumer exists (none promised; sqrt rides the Candidate-F track through this door when it lands).; the door's `domain_policy` admits BOUNDED-DOMAIN primitives — a smooth gate proven exhaustively over a fixed f32 range is tractable where general-domain exp/ln (range reduction, special values, cross-backend proof) is not, and is the cheapest first customer for the door | Resource classes measurably control occupancy; cap-to-budget re-expression reproduces the currently admitted set exactly; door admits/rejects per the two keys with spanned errors; corpus green. | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 5.8 | `GUYANG-COMPARATIVE-PROJECTIONS-0` | Phase 5: dominance/margin/contest/border-band/chokepoint projections; default-derived at ≥2 emitter classes; deterministic tie-break; consumes generic field-sweep outputs. | TP scenario surfaces fronts + a chokepoint event with zero scenario wiring; `stead_spatial_contract_guards` green. | Std — Grok | TODO |
| 5.9 | `TP-PURGE-0` | **Phase 5 hygiene — INTENT NOTED, sequencing Owner-scheduled and MAY preempt 5.4.** Terran-Pirate was authored as a ClauseScript hydration test and has become canonical infrastructure by accretion: production hydration branches on a faction string (`hydrate_scenario.rs` owner == pirate posture select) and carries TP-named production fields (`hydrate_combat_arena.rs` terran/pirate weapon damage); 20 referees plus 5.1's generated `property_admission_inventory.tsv` bind engine admission law to the TP corpus, so retiring TP would today break the engine's own proofs. **This is a Corpus Boundary Law repair, not hygiene.** Purge scope: (a) de-name the production breaches VALUE-PRESERVING with the prescribed shape — `terran_weapon_damage`/`pirate_weapon_damage` (public serde-bound fields on `HydratedCombatArenaPayload`, mirrored in `ParsedCombatArenaPayload`, and authored keys in the corpus) become owner-keyed authored data admitting N sides; the `owner == pirate` posture select becomes a per-owner authored posture with a default; hydrated output for TP stays identical; (b) re-base admission proofs onto a minimal synthetic corpus so TP is one witness among several and never the definition — no engine law stated in terms of one corpus's contents, and no corpus edited to make engine law pass; (c) reap the unleased workshop survivor `crates/simthing-workshop/src/tp_rf_reduce_up_golden.rs` (zero rows in `closeout_artifacts.tsv` — no clock will ever reap it) and the two TP handoff objects still leased under the closed 0.0.8.6 track. **Fix-then-ratchet:** the `WORKSHOP-HOMING-DETECTION` promotion from HEURISTIC to hard-fail is the LOCK-IN and lands at 10.1, never before (a) and (b) — a gate promoted ahead of the debt fails against the debt it exists to prevent. Root cause on record: the escaped code was authored directly into engine crates, so it never entered the workshop leaf and was therefore never subject to the default-delete sweep or to an elevation ceremony; the 0.0.8.5 closeout correctly reaped what WAS contained (six post_hydration modules, #1284). | **DA-GRADUATED / merged #1520 @ 153ba40c** — DA independently reproduced at exact head `9ccd800c`: **THE FALSIFIABLE TEST PASSES — engine 181/0/1 across all six crates with `terran_pirate_galaxy.clause` DELETED**, corpus restored clean. Stage-B arithmetic verified from the map itself: 222 rows = 145 INLINE + 73 REAP + 4 SURVIVOR, 0 CONFLICT; exactly 2 parametrized harnesses, exactly 10 approved cases, each with a demonstrated planted-defect red. JIT determinism case verified live-vs-live in the green posture (`compile_eml_gadget` both sides), mutant only on side B — the assertion is wired to the live compiler, not a replica. Detachability `production=0 proof=0 ceiling=0`; inventory drift PASS; expiry PASS `expired=0`; `hydrate_combat_arena.rs` and the unleased workshop golden confirmed gone. RESIDUE NAMED, NOT WAIVED: 26 `simthing-driver/src` files (14 dress-rehearsal/demo, mostly `*_0080_*`-era) still carry scenario vocabulary, and `simthing-spec/src` carries one `//!` doc-comment mention. Neither couples the engine to the corpus (proven by the corpus-absent run); both are assigned to 10.1 as the PRECONDITION of its `WORKSHOP-HOMING-DETECTION` hard-fail promotion — the ratchet cannot land until they clear. | Std — Grok | **DA-GRADUATED / merged #1520 @ 153ba40c** |
| 6.1 | `EVENT-GENERATION-STAMP-0` | Phase 6: generation stamps on emission/threshold events; stamped ring egress with admission-time backpressure. | Events carry stamps; forced observer lag honors declared backpressure without perturbing the sim. | Std — Grok | TODO |
| 6.2 | `ASYNC-COMMAND-QUEUE-0` | Phase 6: async CPU action queue landing at generation barriers; injection generations logged; generation-consistent snapshots. | Replay bit-exact from the injection log; shadow-tree reads torn-free (double-buffer proof). | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 6.3 | `CONTINUOUS-POSTURE-SOAK-0` | Phase 6: continuous posture (batched generations) + N-generation soak under forced lag. | Soak exit-proof per the phase row; paced posture unchanged (TP regression green). | Std — Grok | TODO |
| 7.1 | `MOVEMENT-DECISION-INGRESS-0` | Phase 7: movement authority — decisions as sealed Triad band crossings (need + potentials + fronts); fills the 12.4 `InTransit` seam. | Fleets move under the ordinary generation from band crossings only (no CPU branching on field reads); §7 automaton conformant. | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 7.2 | `MOVEMENT-AUTHORITY-READBACK-0` | Phase 7: authoritative movement readback; 12.5 icon descriptors express real transit. | Studio shows authoritative transit with ZERO icon-layer change; test-private seam retired. | Std — Grok | TODO |
| 8.1 | `COMBAT-CONSERVATION-JUDGE-0` | Phase 8: RF-1-style conservation judge over combat flows (oracle-first). | Judge red/green proven on seeded violations against R6 rehearsal runs. | Std — Grok | TODO |
| 8.2 | `COMBAT-ARENA-EXECUTED-0` | Phase 8: promote combat to executed RF arena (HP/damage as flows); rehearsals re-anchor as oracles; Triad-band combat context. | Executed path judged green; rehearsals retained as oracles; emergence falsifier (weight change → different battle outcome). | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 9.1 | `GATED-RATES-EML-REWIRE-0` | Phase 9: exclusion-audit Family C — `gated_rates` gate evaluation → authored EML library stack; role-pathway columns; delete `first_slice_mapping_runtime` (superseded by P0 derivation). | Gate behavior parity via oracle; hardcoded cols gone; corpus green. | Std — Grok | TODO |
| 9.2 | `RF-COLUMN-MINT-MIGRATE-0` | Phase 9: the sweep — remaining sites onto doors; DELETE the exclusion list; retire `COLUMN-INDEX-MINT`. | Exclusion list empty; tripwire retired per its promotion-blocker; full build + RF batteries green. | Std — Grok | TODO |
| 10.1 | `DOCTRINE-CI-RECONCILE-0` | Phase 10: retire impossible-state detectors; mechanize new gates (EML-addition DA-RESERVE, telemetry-lifecycle, Root-Contract admission errors); promote `WORKSHOP-HOMING-DETECTION` from HEURISTIC to hard-fail for engine `src` as the TP-PURGE-0 lock-in (never before its de-naming lands); allowlist re-baseline (`kernel_surface` shrink) incl. the seven-shader bespoke stencil-family retirement (`min_plus_stencil`, `structured_field_stencil`, `min_plus_traversal_d_probe`, `saturating_flux_choke_threshold`, `structured_field_stencil_atlas_mask`, `w_impedance_compose`, `stress_compose` — each classed in/out-of-family, last-consumer-verified, superseded by 5.5-5.6 field-sweep instances); anchors re-pointed. | Phase-row falsifier: zero impossible-state scans; zero unwatched new surfaces; selftests green. | DA-reserve — Fable | TODO |
| 11.1 | `EMBEDDER-INTERFACE-0` | Phase 11: the five-verb Vendor Door API (formalization of existing mechanisms) + vendoring crate boundary. | Five verbs stand up a tree end-to-end in a test; posture choice on Run; no new subsystem (diff-proven). | DA-reserve · Frontier — Codex 5.6/Fable | TODO |
| 11.2 | `EMBEDDER-GUIDE-EXEMPLARS-0` | Phase 11: DOC-BUDGET-capped Embedder's Guide + two non-game exemplars (finance toy; network-saturation full-Triad exercise), CI admission-checked. | Cold-reader exit-proof per phase row; exemplars green in CI. | Std — Grok | TODO |
| 12.1 | `PORTABILITY-PROOF-0` | Phase 12: non-TP domain through the Vendor Door end-to-end, zero engine edits. | Exemplar-seeded domain simulates + STEAD-observes + serializes untouched. | Std — Grok | TODO |
| 12.2 | `CORE-CANONIZATION-0` | Phase 12: object model (P0 Root Contract + cycle + EML-ISA + Triad Doors + overlay law) into `simthing_core_design.md`; HARNESS phases canonize nothing; track closeout follows. | Canonized sections land; net-prose discipline; SimThing complete and distinct; CLOSEOUT protocol run (Owner-gated). | DA-reserve · Frontier — Codex 5.6/Fable | TODO |

| Item | State |
|---|---|
| Active open rung | `FIELD-SWEEP-IR-PROBE-0` |

## 4. Binding laws (carry from day one)

**The Native Ingestion Law (Owner mandate, 2026-07-30).** **An authored Clausewitz script must
always be natively ingestible through ClauseThing.** `scenarios/terran_pirate_galaxy.clause` is a
ClauseScript source written in the Clausewitz scripting language; it parses (jomini), hydrates, and
transpiles to a canonical `ScenarioSpec` **without engine changes**, and that must remain true
however radically the substrate is rebuilt. This is the deliberate complement to the Detachability
Law, not a contradiction of it: **the engine must never DEPEND on the corpus; the authoring layer
must ALWAYS be able to READ it.** Accordingly the Invariant Set governs the SUBSTRATE, and the
authoring layer carries exactly one invariant of its own — **native-ingestion** — witnessed by the
~40 MINIMAL authored fixtures in `crates/simthing-clausething/tests/fixtures/*.clause` (31B–3.4KB,
grammar-level), **never by the game corpus**. Proving the language on an 11.5KB galaxy proves that
one scenario transpiles, not that the language works. **`.clause` files are EXTERNAL ASSETS** —
like an image or a mesh. An asset may sit in the tree unread and unversioned by any proof; it
influences the corpus **only when a proof reads it**, and at that moment it becomes corpus
coupling subject to every rule above. So: **delete the proofs that read the game corpus; keep the
asset.** The engine's independence is proven by the corpus-absent run; the language's health is
proven by minimal fixtures; and the artefact itself persists as authored Clausewitz source that
must remain ingestible however the substrate is rebuilt.

**The Invariant Set (Owner mandate, 2026-07-30) — the COMPLETE proof surface of the substrate.**
Engine correctness is these laws and nothing else. Each holds for **any** input, so each is
provable over inline-constructed input: **a corpus, fixture, or generator is never a
prerequisite for proof.** (1) **Conservation** — flow in equals flow out, modulo authored
injection. (2) **Determinism** — identical input twice yields byte-identical output; fold and
neighbour order are authored data, never an accident of layout. (3) **CPU/GPU parity** — the
reference implementation and the kernel agree. (4) **Boundedness** — no NaN/inf escapes; clamps
hold. (5) **Admission totality** — every admitted Anchored property has a host or admission fails
closed. (6) **Residency/typing** — illegal states are unrepresentable at the type boundary, which
*deletes* tests rather than adding them. **A per-rung referee is INADMISSIBLE unless it names a
NEW invariant.** A rung is done when the invariant set still holds and the new capability is
exercised *by that set*; a rung that needs a bespoke referee has revealed a capability the
invariant language cannot express, which is a design signal, not a testing gap. Corollary:
**emergence checks are DEMONSTRATIONS, not gates** — they are what you look at to see the
mechanism is alive, and gating on them invites making-the-referee-pass pressure. Corollary:
generated input buys *coverage* (a fuzzer) and *measurement* (performance curves); it never
buys validity. This law is the standing answer to per-rung proof accretion: 13 rungs produced
123 lifecycle rows because rungs, not invariants, were the unit of proof.

**The Detachability Law (Owner mandate, 2026-07-30).** **SimThing exists without ClauseThing.**
The engine (`-core`, `-spec`, `-kernel`, `-sim`, `-gpu`, `-feeder`, `-driver`) may never depend on
the authoring/app layer (`-clausething`, `-mapeditor`, `-workshop`); the arrow points one way only.
Scenario vocabulary, where it must exist at all, lives in `-clausething` and is data the engine
reads back, never a literal the engine names. Mechanized by `scripts/ci/detachability_check.sh`:
production coupling is a hard FAIL (verified 0 at ruling time — the gate locks in a property the
repo already had), and proof coupling via dev-dependencies is metered against a ceiling that
**may only decrease**, each reduction being a rung that moved an engine proof onto the Invariant
Set instead of a hydrated scenario.

**The Corpus Boundary Law (Owner mandate, 2026-07-30).** **No scenario's vocabulary may be
hardcoded in engine crates.** No faction, owner, entity, or scenario name may appear in
production `src` of `simthing-core`, `-spec`, `-kernel`, `-sim`, `-gpu`, `-clausething`, or
`-driver` — not as a control-flow literal, not as a struct field, not as a serde/wire key, not as
an authored-grammar key the engine names rather than reads. Scenario identity enters ONLY as data
the engine reads back. Two corollaries, because both failures already happened: (a) **a corpus may
WITNESS engine law and may never DEFINE it** — engine law is never stated in terms of one corpus's
contents, and a corpus is never edited to make engine law pass; (b) **engine law must be provable
against at least two corpora, one of which is minimal and synthetic** — plurality, not abstinence,
is what stops a single corpus becoming definitional, and it preserves the emergence falsifiers that
require a rich witness. Hardcoded scenario vocabulary is ALSO a generality defect, not merely
hygiene: a two-field `terran_/pirate_` payload cannot express a third faction, and
`if owner == "pirate"` cannot express a non-pirate raider. Enforcement is `TP-PURGE-0` (de-name,
owner-keyed data, N-side capable) with the `WORKSHOP-HOMING-DETECTION` HEURISTIC to hard-fail
promotion at 10.1 as the lock-in — fix, then ratchet. Sealed-core status at ruling time: `-core`,
`-spec`, `-sim`, `-gpu` production `src` already CLEAN; the sole `-kernel` match is a `#[test]`
negative proof that scenario-tagged opcodes are inadmissible, which is this law working.

**The Consumer Law (Owner mandate, 2026-07-20).** STEAD, PALMA, Gu-Yang, and Studio observation
are STANDING CONSUMERS of all data accumulated at SimThings — accumulated data is never walled
off as hygiene ("waiting for a consumer" is not a state that exists; STEAD is the consumer).
Hygiene disciplines govern PROOF (source loci cannot close an OVL or prove a flow) and MUTATION
(no side-channel writes) — never READ ACCESS. CPU-side consumer reads route through the one
canonical generic read seam (hosted SimThingId + typed PropertyKey/role, role-pathway loci,
coherent per-generation snapshots); observation never becomes CPU decision branching — decisions stay
on-device threshold/band crossings. **The Consumer Law does NOT shield tests from lifecycle:**
it governs production read access, never test/evidence retention — almost no test outlives its
workplan/track (default-delete at closeout stands) unless deleting it breaks production code;
"STEAD might consume this someday" is not a lifecycle exemption. Born from the 12.3 stop: the 12.2→12.8 horizon seam was
dropped and the disruption map sat walled at 0.0 while macro disruption ran ~8446.
**THE TRIAD DOORS (allowlist law, Owner-ruled 2026-07-26).** The P5 Field Triad doors are the
ONLY sanctioned derivations for their semantic families, repo-wide: **pathfinding/routing →
the PALMA door** (semiring potential sweep; `D` is a field, not a route); **borders/edges/
dominance/contest/fronts → the Gu-Yang door** (sealed comparative projections + band
crossings over flux fields); **observation/urgency/actionability → STEAD anchors + bands**.
Any reach for these semantics outside the doors — CPU graph search, border/contour/frontline
services, listener frameworks — is a constitutional violation regardless of where it lands
(engine, studio, scenario, workshop-elevation): tripwires flag the reach (Phase 0
ride-alongs), the reach log accumulates the evidence, and allowlist/clearance additions in
these families route DA-RESERVE. PALMA and Gu-Yang are deliberately NOT first-order root
values — the VALUES are ordinary anchored columns; the DOORS are the constitution.
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
is **write-only with respect to the sim** — counters never feed any generation computation (bit-exact
replay; decisions stay on-device). The call profile is a standing Consumer-Law surface.
**Bespoke parallel systems are constitutional violations, not conveniences.** Named
transitional debts with scheduled promotions: the R6/R6B/R6C combat rehearsals (Phase 8) and
the movement readback seam (Phase 7). Anything new of that shape fails admission.
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
2. ~~RF-harness (0.0.8.4.8.4.2) disposition~~ **RULED 2026-07-26** — folded into this track: RFH-1 = Phase 0 (guard, early, non-breaking), RFH-2 = Phase 9 (sweep, late, after Phase 4's object model). The separate track is dissolved; `design_0_0_8_4_8_4_2_RF_harness.md` reduced to a superseded pointer.
3. ~~Owner commits the full plan~~ **SATISFIED 2026-07-26** — §3b ladder committed (guards early, sweeps late honored: 0.x guards lead, 9.x sweeps trail); track opened via `gen_orientation.sh --open`.
