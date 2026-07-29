# EML + N4 Expansion Digest

> **Status: PROVENANCE + PRESERVATION DIGEST** (Owner-directed, 2026-07-29). **Non-binding on process.**
> The binding surfaces are the §3b ladder rows (`5.4`–`5.8`, `10.1`), P5, and
> [`stead_spatial_contract.md`](stead_spatial_contract.md) §10. This document exists for one reason:
> **the rungs will be implemented across many DA and orchestration sessions, and the *why* must survive
> the churn.** When an implementation conversation proposes narrowing, deferring, or reversing a piece of
> the remodel, check it against §5 here first.
>
> **Owner directive at authoring time:** the seven-shader retirement is under challenge in review. §6
> records why it is the deliverable and not cleanup, and states precisely what may and may not be
> challenged without reopening the gap this remodel exists to close.

---

## 1. Origin — two threads that met

**The Owner's thread (predates the track).** Network topology as a SimThing ontology: every client,
trunk, failover, and load balancer a SimThing node; resources, needs, and deficits as matrix columns
resolved by GPU parallelism. Held as a horizon, never as a rung — now recorded as the network-governance
plug in [`simthing_lewm_corpora_case.md`](simthing_lewm_corpora_case.md) §4.1, the semantic-port domain in
which **observation space is state space**. The Owner's stated motive for eventually lifting N4 was
**corpus yield**: richer adjacency widens the §1 data-complexity axis that makes the LeWM corpus a
metrology instrument rather than another synthetic dataset.

**The substrate's thread.** Phase 11 independently named network saturation / load-balancing as a Vendor
Door target, and rung `11.2` asked for an N-node exemplar exercising the full Field Triad.

The collision exposed a defect: **the exemplar was not buildable as written.**

## 2. The adjacency gap, as found

- The Field Triad executed **only over grid N4**. `min_plus_stencil.wgsl` and
  `structured_field_stencil.wgsl` hardcode four directional reads; the CPU oracles hardcode the same four.
- The arbitrary-topology path (`spec.links` → `structural_link_accumulator_compile.rs`) carried **Sum
  only** — the generic kernel wired `SOURCE_INPUT_LIST` for `COMBINE_SUM` and nothing else.
- **Networks have no lattice.** So the entire STEAD / PALMA / Gu-Yang layer could not run over a graph
  *at all*.
- Three specifics: `SimThingScenarioLink` is `{from_system_id, to_system_id}` — no direction, weight,
  capacity, or latency; the INPUT_LIST gather ignores `unit_cost`; and the then-planned
  `SEMIRING-FIELD-TAGS-0` parameterized the *algebra* while leaving adjacency hardcoded, so it would not
  have closed the gap.

§10's separation of the two adjacencies was correct anti-drift law when the mechanisms were structurally
different. **The remodel does not conflate them — it parameterizes them.** That distinction is the whole
constitutional argument, and it is why the §10 amendment rides in 5.5 rather than being waved through.

## 3. How it became the field-sweep IR

The chain of moves, including the two that were wrong:

1. **First proposal, not the plan of record:** add narrow kernel arms (`MIN × INPUT_LIST`,
   `PRODUCT × INPUT_LIST`) plus an algebraic decomposition of the flux update. Closes the gap; buys
   nothing else. **Retained as the pre-named fallback in 5.4.**
2. **The Owner's reframe:** N4 was always arbitrary — a performance choice, not a principle — and the EML
   transcendental ban was an early guard against EML displacing faster native ops. Both provisional.
   *Grow EvalEML rather than working around it.*
3. **The unification:** a field sweep is fully described by
   **`adjacency × per-edge map × (fold + identity) × optional post-step`**, all authored data:

   | Operator | map | fold | post-step |
   |---|---|---|---|
   | STEAD accumulate | `u_j × falloff` | `(+, 0)` | — |
   | PALMA min-plus | `d_j` | `(min, +∞)` | `+ W_i` |
   | Gu-Yang flux | `(c_i+c_j)/2 · (u_j − u_i)` | `(+, 0)` | `+ u_i` |
   | Boolean reach | `r_j` | `(OR, false)` | — |
   | future domains | authored | authored | authored |

4. **Consequence:** the semiring enum never needs to exist. Making the algebra a runtime tag is
   drift-detector 7 — a policy enum where authored data is the answer — applied to the rung's own design.
5. **First error, corrected by orchestration.** The map language *cannot* be built by repurposing the three
   zeroed `EmlEvalCtx` params. `EML_OP_SLOT_VALUE` reads only `ctx.eval_slot`, so a neighbor's arbitrary
   columns are unaddressable and "anything future: authored" would have been **false** — discovered by the
   first author needing two neighbor columns, after the IR shipped. Cure: a *designed* edge context
   `{target_slot, neighbor_slot, accumulator, edge_scalar, dt}` with `TARGET_VALUE(col)` /
   `NEIGHBOR_VALUE(col)` opcodes under the EML growth law.
6. **Second error, corrected by orchestration.** Degree buckets were presented as one object serving both
   warp occupancy *and* CFL stability, and that coincidence was offered as evidence the decomposition was
   right. It lets a **scheduling** decision author **physics**: rebucket for occupancy and the dynamics
   change. Cure: χ from a per-node weighted-degree/conductance certificate at admission; buckets may reuse
   the metadata and may never determine the bound.

Both corrections are load-bearing. The elegance did not survive them — it **began existing** at them.
Move 5 in particular means the original claim rested on a coincidence (three spare parameter slots) rather
than a design.

## 4. How DA and orchestration made it a ladder

Orchestration ruled **ADMIT WITH REMAND**; the Owner approved the synthesis. Four remands adopted in full:
parameter model, resource model, χ/bucketing separation, transcendental demotion.

| Rung | ID | Role |
|---|---|---|
| 5.4 | `FIELD-SWEEP-IR-PROBE-0` | Workshop-leaf **disposable** probe; publishes the measurement surface; re-derives current EML cap facts as ground truth; tiered gate (parity absolute, median ≤1.25×, supported-adapter worst ≤1.5×); failure routes to specialization/JIT, **never abandonment** |
| 5.5 | `FIELD-SWEEP-N4-PARITY-0` | Engine landing: edge context + `TARGET_VALUE`/`NEIGHBOR_VALUE`; `FieldSweepRegistration`; N4 bit-exact parity alongside the bespoke stencils; `SEMIRING-FIELD-TAGS-0` withdrawn in-diff; P5 + §10 + guards co-evolve in the same PR |
| 5.6 | `FIELD-ADJACENCY-GENERATORS-0` | Adjacency as a registration axis: weighted `GridOffsets [(dx,dy,w)]` with N4/N8/radius-r presets + `LinkGraph`; per-node χ certificate; degree-homogeneous scheduling that never authors physics |
| 5.7 | `EML-RESOURCE-CLASS-ADMISSION-0` | Interpreter resource classes; universal node cap → measured budget *only once measurable*; `ExactPrimitiveAdmission` door with **no primitive promised** |
| 5.8 | `GUYANG-COMPARATIVE-PROJECTIONS-0` | Renumbered, scope unchanged; consumes generic sweep outputs |
| 10.1 | `DOCTRINE-CI-RECONCILE-0` | Seven-shader retirement batch (§6), each classed in/out-of-family, last-consumer-verified |

Three refinements **outrank** the original proposal on the admission ladder:

- `FieldLawProof` + `CanonicalOrderProof` as **sealed types** — fold-order determinism moved from prose
  mitigation (rung 4) to type boundary (rung 1).
- Conservative folds **require an undirected-symmetry certificate**. The original design got conservation
  from the link compiler's dedup behaviour — an accident, not a contract.
- `resource_class` is a **closed set**: admission accepts only the legacy fixed-32-stack class until 5.7
  defines others, so the forward reference cannot rot.

## 5. Preservation invariants

**This is what the value falls out of. Losing any one returns the remodel to a bespoke kernel with extra
steps.**

1. **The algebra is data, never an enum.** A semiring tag, field-kind enum, or `match` on operator identity
   reappearing in the sweep path means the remodel has failed, regardless of what works.
2. **Adjacency is an axis, not a fork.** `GridN4` / `GridN8` / `radius-r` / `LinkGraph` are values of one
   parameter over **one** code path. Two code paths reopen the gap.
3. **Scheduling never authors physics.** χ comes from the per-node conductance certificate at admission.
   Degree buckets are an occupancy optimisation that may read that metadata and may never set the bound.
4. **Canonical neighbour order is a sealed proof.** Float folds are non-associative. Order is pinned for
   grid offset tables *and* graph neighbour lists; no tree reduction without an associativity proof.
5. **Conservation is a contract.** Conservative folds admit only with the undirected-symmetry certificate.
6. **Parity-alongside; referees unedited.** The bespoke stencils are the migration oracles. They are not
   deleted during landing, and the referee batteries are not edited to accommodate the new path.
7. **Determinism keys are sovereign.** For any exact primitive, the Candidate-F-class proof is
   non-substitutable; a cost/occupancy argument is a conjunctive second key, never an alternative.
8. **The IR outlives its interpreter.** If the generic interpreter loses on measured performance, the answer
   is specialisation or JIT — never abandonment. Interpreted tree = specification; compiled kernel = its form.
9. **No eighth bespoke stencil, ever.** A new hand-written field kernel is a regression regardless of its
   benchmark.
10. **Emergence is the exit proof.** The same authored map/fold on different adjacency must produce
    qualitatively distinct *unscripted* geometry. Mechanism sound + dynamics inert = rung FAIL on green checks.

## 6. Why the seven shaders retire — Owner-directed record

The family: `min_plus_stencil`, `structured_field_stencil`, `min_plus_traversal_d_probe`,
`saturating_flux_choke_threshold`, `structured_field_stencil_atlas_mask`, `w_impedance_compose`,
`stress_compose`. Retirement is batched at `10.1`.

**Retirement is the deliverable, not cleanup.** The seven exist for exactly one reason: there was no shared
field-sweep IR, so every new field need was answered with a hand-written kernel. If they survive the
remodel, the track has *added* a generic path and *kept* seven exceptions — **strictly more kernel surface
than before it started**, and every future domain gets to argue for an eighth. The value the Owner is
buying is the deletion:

- the `kernel_surface` allowlist shrinks, which is Phase 10's own stated target;
- one CPU-oracle parity proof replaces seven;
- the JIT receives a single canonical input form;
- and no future field need can justify a bespoke kernel, because a conformant path exists.

This follows the admission-substrate precedent, where the deliverable was the **deletion** of guard scripts
rather than their accumulation. A rung-3 device that exists only because a type didn't is a promotion
target; a bespoke kernel that exists only because an IR didn't is the same species.

**What may legitimately be challenged** — membership and sequencing:

- whether a given file is in-family. `stress_compose` and `structural_validation` were the two that could
  not be classed by inspection; `10.1` therefore requires **each** classed in/out explicitly;
- last-consumer verification before any removal;
- ordering relative to 5.8's comparative projections.

**What may not be challenged without reopening the gap** — retirement as the goal. *"They work and they're
fast"* is not an argument for keeping them: it is precisely the question the 5.4 probe gate exists to
settle, and that gate's failure branch routes to **specialisation or JIT with the IR retained**, never to
preserving bespoke kernels as the permanent path. A challenge that ends with seven hand-written field
kernels surviving indefinitely has not refuted the remodel — it has cancelled it.

## 7. Deliberately unfinished

- **The universal EML node cap survives into 5.7** and is replaced only once resource classes are
  measurable. It is a magic constant of the same species as the `65,535` edge ceiling that STEAD-SCALE-1
  removed. If 5.7 stalls, the fossil stays — **do not read its survival as doctrine.**
- **No transcendental primitive is promised.** 5.7 lands the admission door; `exp`/`ln` admit only when a
  candidate implementation passes it. Candidate F is a Q16.16 magnitude-sqrt mechanism, not a general
  `exp`/`ln` method, and exhaustive execution on one adapter is not cross-backend proof.
- **Directed / asymmetric graph flux is out of scope.** Conservation rests on undirected symmetry (§5.5).
- **Rung `11.2`'s network exemplar** remains the original consumer and becomes buildable at 5.6.

## References

- [`design_0_0_8_7_rf_arena_modernization.md`](design_0_0_8_7_rf_arena_modernization.md) §3b rows
  `5.4`–`5.8`, `10.1`; P5 (the Field Triad) and the TRIAD DOORS law — **these govern; this digest does not.**
- [`stead_spatial_contract.md`](stead_spatial_contract.md) §10 — the adjacency table amended by 5.5.
- [`simthing_lewm_corpora_case.md`](simthing_lewm_corpora_case.md) §4.1 — the network-governance plug and
  the corpus-yield motive.
- [`simthing_core_design.md`](simthing_core_design.md) §1.1 (Anchors A/B), §1.2 (admission ladder), §4.1
  (EML extension ladder), §7 (Movement-Front), §9 (drift detectors).
- Owner/DA rulings: [issue 1332 comment 5107867267](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5107867267)
  (amendment approved, held behind 4.2) and
  [comment 5109630636](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5109630636)
  (updated amendment). Orchestration's ADMIT-WITH-REMAND ruling supplied the four corrections in §3 and §4.
