# EML EXP/LN Unification Expansion — intrinsic contention, recursive CostBands, and tiled EML execution

> **Status: WORKSHOP / REVIEW CANDIDATE — NOT DESIGN-ADMITTED, NOT A WORKPLAN AMENDMENT.**
>
> Owner-directed horizon exploration, 2026-08-04. This document develops a candidate expansion of the EML + CostBand + StemThing unification for review. It deliberately separates **semantic canonization candidates** from **performance research hypotheses** so that a successful optimization cannot become semantic authority by accident. Nothing here changes the 0.0.8.7 pointer, rung ordering, core constitution, or the settled semantics of existing rungs.
>
> Governing anchors: [`full_eml_unification.md`](full_eml_unification.md), [`stead_stemthing_unification.md`](stead_stemthing_unification.md), [`stead_spatial_contract.md`](stead_spatial_contract.md), [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md), and [`design_0_0_8_7_rf_arena_modernization.md`](design_0_0_8_7_rf_arena_modernization.md).

---

## 0. Executive thesis

The EXP/LN completion does more than add two expensive mathematical operations to EML. Together with CostBand, the generic field sweep, the SSA JIT, intrinsic ownership, the closed-loop resolution site, and the StemThing direction, it exposes two adjacent opportunities:

1. **Semantic opportunity — make contention intrinsic to the base recursive SimThing.** A constrained parent already receives claims, has finite capacity, allocates, and disburses. Failure of committed claims to clear can therefore create an intrinsic **contention CostBand state-transition hinge** without authoring an adversarial pair or domain-specific contest. The same facility can recurse at every parent/child level. Spatial SimThings additionally expose the consequences laterally through STEAD and Gu-Yang. Complex social/economic/adversarial forms should be compositions of this one base stub, not new mechanisms.

2. **Execution opportunity — lower EML into self-contained tensor-like tiles over aligned SimThing rows.** The semantic object remains the one StemThing/SimThing. Tiling is only a backend: dependency-level EML operations are packed into fixed resource-bounded tiles, evaluated over large aligned row batches, with inputs loaded once, intermediates retained on-chip where possible, and steady-state table allocation/rebuild eliminated. The hypothesis is that this can approach or exceed bespoke/native GPU throughput at the million-row scale, especially when several EML/STEAD laws share the same resident inputs and topology gather.

The proposed unification is therefore:

```text
                         STEMTHING
                  lone semantic root object
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
      RF                  EML                STEAD
       │                   │                   │
       └──── intrinsic contention CostBand ────┘
                           │
                    ordinary lanes
                           │
                 ─ semantic boundary ─
                           │
                   EML tile lowering
                           │
               stable GPU execution fabric
```

**Hard guardrail:** if contention, adversariality, or tiled EML requires a second causal substrate beside SimThing, the proposal fails. Contention must arrive as a lane/stub on the base object; tiling must arrive as a disposable physical lowering of the same admitted program. The StemThing anchor's test applies unchanged: **a lane, not a new leg**.

---

## 1. Deliverables proposed for review

This exploration converged on the following candidate deliverables.

### 1.1 Canonization candidate — CostBand as the state-transition hinge

Expand the CostBand interpretation from “resource sink attached to an observation” into the general hinge between numerical perception and committed resource action:

```text
observation / derived state
        ↓
       EML
        ↓
     CostBand
        ↓
resource state transition
        ↓
RF / allocation / disbursement
        ↓
new state and fields
```

The settled CostBand algebra remains unchanged. The expansion is conceptual and compositional: EML computes the price/condition; CostBand converts available continuous value into an exact committed draw; the draw changes ordinary state. No rival action mechanism is minted.

### 1.2 Canonization candidate — intrinsic contention CostBand stub on the base StemThing

Every resource-bearing parent already has enough information to derive contention from ordinary quantities. At minimum:

```text
claim_i
allocation_i
capacity
```

with a residual such as:

```text
unresolved_i = max(claim_i - allocation_i, 0)
```

The base facility is **inert when claims clear**. An unresolved claim is not automatically “hostility”; it is contention pressure. EML evaluates whether/how much the claimant is willing and able to spend to sustain that unresolved claim. The resulting CostBand is the paid commitment.

A useful semantic distinction:

- **contention** = committed claims cannot all clear against constrained state;
- **adversariality** = the degree to which claimants pay to persist, resist accommodation, or alter the clearing surface;
- **relation/policy** = an input/prior that may influence clearing or willingness to persist, never a hard engine branch.

This lets adversariality emerge from repeated unresolved contention without requiring an authored `A conflicts_with B` relation.

### 1.3 Canonization candidate — intrinsic recursive disbursement facility

The contention stub should be the base disbursement facility for constrained resources at every resource-bearing parent, not a special geopolitical/spatial facility. Spatial SimThings receive the same vertical mechanism and additionally participate in lateral field propagation.

The recursive shape is the existing SimThing tree:

```text
leaf claims
    ↓ reduce
parent constrained clearing
    ↓ unresolved parent deficit
higher parent constrained clearing
    ↓
...

then disbursement returns downward through the same hierarchy.
```

No `CostBandTree` object should exist. The SimThing tree is already the recursion authority.

### 1.4 Research candidate — general EML-as-tensor-tile backend

Build and measure a semantics-free physical executor that packs dependency-level EML work into fixed, self-contained tiles over aligned SimThing row batches. It must prove bit-equivalence to the reference/SSA-JIT path before any performance claim matters.

The target question is not whether a tile can make an ADD faster than native ADD. The target is whether a tile can make the **whole evaluation architecture** cheaper by combining:

- homogeneous massive batching;
- aligned/coalesced column ingress;
- no opcode dispatch/stack walk in the hot path;
- dependency-level parallelism within an expression;
- reuse of common inputs and subexpressions;
- workgroup/register scratch rather than intermediate global tables;
- shared topology gathers across several field channels;
- stable preallocated scratch and zero steady-state table malloc/rebuild;
- cross-law fusion unavailable to isolated bespoke kernels.

### 1.5 Validation candidate — million-row reduce/disburse benchmark

The tiled backend must be tested on the workload SimThing actually wants: large GPU-resident populations undergoing recursive reduce/allocate/disburse + EML/CostBand evaluation, not only isolated arithmetic microbenchmarks.

At minimum compare:

1. bespoke/native WGSL reference;
2. current EML interpreter;
3. current SSA JIT;
4. tiled EML;
5. tiled/fused EML with shared field/topology work where lawful.

The scale curve should extend from small populations through at least the million-row regime.

---

## 2. Constitutional boundary: one StemThing or reject the idea

The recursive hierarchical-spatial causal fabric is acceptable only if it remains a description of what the base SimThing already does through ordinary lanes.

The semantic anatomy stays:

```text
participate
act
originate
receive
```

Contention is a property of resource participation and action. It is not a fifth leg. EML tiling is an execution lowering. It is not a sixth leg.

### 2.1 Required invariant

A lawful implementation must satisfy:

> **Every authoritative result produced by a tiled execution must be reproducible by evaluating the same admitted StemThing lanes through the non-tiled reference execution. The tile layer may batch, fuse, cache, place, route, and schedule; it may not create state, identity, lifecycle, resource, contention, field, ordering, or history semantics.**

A tile executor can therefore be deleted and replaced with the slow reference path without changing simulation meaning.

### 2.2 Forbidden expansion shapes

Reject any proposal that introduces:

- `ContentionThing`, `ConflictThing`, or a separate adversarial object hierarchy;
- an authored domain contest registry as the primary source of contention;
- a second causal graph with lifecycle independent of the SimThing tree;
- tile-owned authoritative state not represented by admitted SimThing/field lanes;
- a tile-specific history/replay mechanism;
- a tile-specific ordering law;
- a new domain-specific field kernel;
- semantic dependence on physical row order, subgroup width, tile shape, or adapter.

---

## 3. Three data categories must remain distinct

The richer causal fabric will fail if every useful scalar is casually called a “resource.” Review should explicitly preserve three categories.

### 3.1 Conserved / constrained resource lane

Examples: stock, capacity, fuel, budget, slots, bounded supply. Claims on these lanes can create true contention. Their conservation/creation/destruction laws are explicit.

### 3.2 Derived compound or aggregate signal

Examples: scarcity, risk tolerance, allocation concentration, access pressure, perceived inequality, reliability, prestige pressure. These are authored EML projections over state. They are not automatically conserved merely because they influence resource flow.

### 3.3 Committed claim/action

A derived signal influences an EML law; a CostBand turns that evaluation into an actual draw/claim on a constrained lane. Contention arises from incompatible **committed claims**, not from two scalar signals being labelled adversarial.

This distinction lets content author rich quantities without laundering them into conservation semantics.

---

## 4. Intrinsic contention CostBand — proposed base law

### 4.1 The minimal stub

At any constrained disbursement site, the base SimThing has:

```text
available capacity K
claim set {C_i}
cleared allocations {A_i}
```

A generic residual can be derived:

```text
U_i = max(C_i - A_i, 0)
```

or, where the feasible region is more complex, the equivalent unresolved portion from the admitted clearing result.

`U_i > 0` means a claim did not clear. That is contention pressure. It does **not** mean the claimant is hostile or must pay attrition.

The claimant's admitted EML law evaluates a persistence/commitment price from ordinary state:

```text
P_i = EML(U_i, local_state, relation/prior, reserve, history, ...)
```

The CostBand then determines the actual affordable committed draw. A zero/near-zero price path describes accommodation/waiting; a high-price path describes costly persistence.

### 4.2 Why CostBand is the right hinge

CostBand already states the universal action boundary: observation is free; action is observation with a resource sink. That matches contention precisely.

- A failed claim alone is an observation.
- Persisting, bargaining, rerouting, acquiring, suppressing, attacking, reserving, or otherwise acting because the claim failed consumes something.
- The consumed quantity is a CostBand draw.

No new “conflict action” primitive is needed.

### 4.3 Clearing and CostBand must not be conflated

The base facility still needs a generic constrained-clearing operation. CostBand answers **what committed action can a claimant afford?** Clearing answers **how does bounded supply divide among simultaneous claims?**

The useful research question is whether the apparent resolution-rule taxonomy can mostly collapse into EML parameter regimes rather than engine branches.

Candidate regimes to explore:

- proportional/cooperative sharing;
- policy/priority ordered distribution;
- deterministic first-arrival/ordered service;
- price-clearing;
- winner-take-most / steep soft weighting;
- refusal-to-yield / attritive persistence.

EXP enables stabilized soft weighting (`EXP(beta * (z_i - max z))`), so a continuous surface may span cooperative to winner-take-most behavior without an enum. LN permits log-domain prices/ratios where lawful. **This is a hypothesis, not yet a canonized clearing formula.**

### 4.4 Attrition should be tested as second-order CostBand behavior

The current 8.2 design lists attrition beside proportional/priority/price-clearing as an authored resolution rule. This exploration suggests a potentially cleaner factorization:

> ordinary constrained clearing resolves what can be allocated now; **persistent unresolved contention activates costly persistence CostBands**, and those expenditures alter future claims/capacity/weights.

Under that model, attrition is not a separate disbursement algorithm. It is a dynamic consequence of maintaining incompatible commitments over time.

This should be explicitly reviewed before altering 8.2. It may be superior, but it is not yet admitted.

---

## 5. Fractal recursion: from one stub to the hierarchical causal fabric

The base stub recurses without a new recursive data structure.

A leaf may generate a deficit because its claim did not clear. The deficit reduces to its parent. If the parent cannot clear all child claims, the same contention law applies at the parent. That parent may surface a larger deficit upward. Disbursement then returns down through the same tree.

```text
                       root
                        │
                 constrained clearing
                        │
             ┌──────────┴──────────┐
             │                     │
          parent                 parent
       clearing/CostBand      clearing/CostBand
          /     \                /     \
       child   child          child   child
```

The apparent complexity is therefore fractal composition of the same base mechanism.

### 5.1 Inert-by-default requirement

If all claims clear, the contention facility must add no retained contention object, no dynamically allocated table, and ideally no meaningful extra work beyond a branchless zero residual in the ordinary lane evaluation.

### 5.2 Relation is data, not a branch

“Allied”, “hostile”, contractual priority, queue priority, ownership, or other relationship concepts may influence EML prices/weights, but the core facility must not contain:

```text
if hostile { attrition } else if allied { ordered_distribution }
```

The same mechanism should produce different regimes because the data differs.

### 5.3 First-come/ordered behavior must be deterministic data

Any ordered service regime must derive from admitted ordering data such as authored order or recorded arrival/generation identity, never incidental vector or physical-row order.

---

## 6. Lateral recursion: STEAD and Gu-Yang

RF provides vertical hierarchical recursion. STEAD provides lateral influence. Gu-Yang provides a generic conservative saturation/stall observable over committed spatial flows.

The proposed loop is:

```text
resource state
    ↓
compound EML signals
    ↓
STEAD perception/influence
    ↓
EML response
    ↓
CostBand commitment
    ↓
resource claims / flows
    ↓
contention
    ↓
spatial committed counterflow
    ↓
Gu-Yang stall/saturation
    ↓
ordinary field/state columns
    ↓
next generation
```

### 6.1 Gu-Yang must not become “adversariality memory”

Gu-Yang should remain the instantaneous conservative-flux mechanism required by the spatial contract. It surfaces current stall/saturation. Persistence across generations is ordinary state/field evolution, potentially using EML decay/accumulation laws. EXP makes a finite-memory form cheap:

```text
memory_next = memory_now * EXP(-lambda * dt) + current_stall
```

The semantic distinction matters: Gu-Yang stays physics/mechanism; adversarial persistence is derived from its observed consequences plus CostBand commitments.

### 6.2 The 5.8 authored-competitor assumption becomes a review target

The landed comparative machinery usefully derives dominance, margin, contest, border, and choke after competing emitter classes are identified. The stronger future shape would preserve those projections while deriving the competitor set from **actual constrained/countervailing commitments** where possible.

Authored competitor classes can remain a valid shortcut/prior, but should not become the only route to contest if resource contention already proves that two flows oppose on the same constrained surface.

---

## 7. Compound-resource and aggregate emergence

The intended payoff is that content authors causal data, not pre-known conflicts.

A domain may author:

```text
food_availability = F(stock, production, imports, demand, accessibility)
elite_position_pressure = G(elite_population, finite_positions, rents, mobility)
allocation_asymmetry = H(claims, allocations, fulfillment)
```

A geopolitical package may later interpret some combination as perceived inequality. An industrial package may interpret the same mathematical shape as supplier concentration or allocation unfairness. The core does not know those nouns.

The engine should discover coupling when those derived values alter ordinary claims and the claims converge on shared constraints.

```text
authored transformations
        ↓
derived compound state
        ↓
STEAD perception
        ↓
CostBand commitment
        ↓
shared constrained lanes
        ↓
emergent contention
```

This is the candidate mechanism for surfacing subtle latent resource tensions that the author did not explicitly label as conflicts.

---

## 8. EML tensor tiles — terminology and hypothesis

“Tensor tile” here describes **rectangular execution/data layout**, not a requirement to use hardware matrix-multiply units. Arbitrary EML contains EXP/LN/CMP/SELECT/MIN/MAX and other operations that do not map directly to MMA hardware.

A software tile may look like:

```text
EmlTileClass {
    instance_width,   // SimThing rows evaluated together
    operation_width,  // independent EML sites at one level
    stage_depth,      // dependency stages retained in-tile
    ingress_slots,
    scratch_slots,
    egress_slots,
}
```

Example physical view:

```text
                           SIMTHING BATCH
                    0    1    2   ...   63

operation site 0    op   op   op         op
operation site 1    op   op   op         op
operation site 2    op   op   op         op
...
```

Inputs are aligned property/field columns. Intermediate results stay in registers/workgroup memory where possible. Outputs materialize only when semantically required or when a block boundary forces a spill.

### 8.1 Three parallelism axes

A tiled backend can potentially exploit:

1. **Across SimThings:** the same program over thousands/millions of rows.
2. **Across independent EML DAG nodes:** dependency-level operations can execute concurrently.
3. **Inside expensive atoms:** EXP/LN polynomial/range-reduction chains can expose independent arithmetic when their pinned semantics allow it.

The first axis is expected to dominate. Cooperative evaluation of one expression across many lanes should be an optional measured resource class, not the default assumption.

### 8.2 Tile is a physical container, not a semantic opcode

Do not inflate EML with domain/tile opcodes. A fused `SUB(EXP(a), LN(b))` lowering may become one physical microblock while remaining three semantic EML nodes. The exact EXP/LN semantics stay those admitted by their primitive doors.

### 8.3 DAG metrics replace raw node count as the useful performance predictor

For tiled execution, collect at least:

- semantic node count;
- normalized DAG node count;
- dependency depth;
- width by level;
- peak live frontier;
- ingress count;
- egress count;
- scratch words;
- tile count;
- block-cut count;
- tile utilization;
- predicted/observed register pressure.

A 500-node broad/shallow expression may be a better GPU candidate than a 70-node serial chain. This is especially relevant to deep pure-EML expansions and future trig experiments.

---

## 9. Stable residency: no steady-state table/malloc churn

The tiled backend should assume that the physical SimThing matrix, admitted adjacency, and scratch arenas are long-lived. Changing values or CostBand activity should not cause table reconstruction.

Proposed steady-state resources:

```text
persistent SimThing matrix
persistent adjacency / row-membership metadata
persistent tile-plan descriptors
preallocated scratch slab A
preallocated scratch slab B
optional active-row/index lists
```

Generation-to-generation changes should predominantly be values, masks/counts, weights, and CostBand depths.

### 9.1 Tile plan compilation cadence

Prefer compiling/rebinding tile plans at admission or explicit epoch/remap boundaries. A program/parameter value changing should not imply physical table rebuild if the lane shape remains admitted.

This aligns with the StemThing logical/physical slot split: semantic identity is stable; physical placement may rebind at explicit recorded boundaries. Tiling must consume that substrate, never become another identity authority.

### 9.2 Sparse participation

If only a subset of a million rows participates in a law, use admitted membership/index data over stable rows rather than compacting/repacking the main state table each generation.

---

## 10. Multi-channel field bundling — likely highest-value optimization

The 5.7 Gu-Yang performance debt is a critical clue: generated arithmetic was not the fundamental problem; repeated global neighbor gathering was. The bespoke stencil tiled neighbors into workgroup memory and reused them.

The stronger tile-fabric opportunity is therefore:

> **Once a neighborhood has been paid into on-chip storage, evaluate as many lawful EML/STEAD channels over that neighborhood as possible before eviction.**

Instead of independent field sweeps:

```text
load neighborhood -> scarcity
load neighborhood -> elite pressure
load neighborhood -> security
```

use a vector-valued/channel slab:

```text
one neighborhood gather
        ↓
scarcity EML      elite-pressure EML      security EML
        ↓                 ↓                    ↓
STEAD channel      STEAD channel           STEAD channel
        └─────────────────┬────────────────────┘
                          ↓
               comparative / Gu-Yang EML
```

Each channel retains its own authored map/fold/post law and canonical fold order. Sharing WHERE values are loaded from must never silently reorder HOW a channel folds.

### 10.1 Materialization policy

Intermediate derived fields should not automatically become permanent matrix columns. Materialize only when:

- the value is authoritative persistent state;
- the Consumer/anchor law requires it observable;
- another execution block needs it across a global synchronization boundary;
- debugging/proof mode explicitly requests a witness.

Otherwise keep the value in bounded tile/block scratch.

---

## 11. Performance claim to falsify

The hypothesis is deliberately stronger than “EML can be made less slow”:

> **At large homogeneous populations, a tiled EML backend can approach native/bespoke GPU throughput, and fused multi-law tiled execution can sometimes outperform isolated bespoke kernels by exploiting global batching, shared ingress, shared topology loads, CSE, and reduced intermediate materialization.**

This is plausible because both generated WGSL and hand-written WGSL ultimately lower through the same GPU toolchain. The residual bespoke advantage is primarily layout/locality/manual specialization — exactly what the tile compiler is intended to recover generically.

The current 5.11 EXP landing strengthens the case: the exact primitive already showed a strict resource-cost win over its ordinary EML gadget baseline, and the 5.7 PALMA JIT established that generated EML WGSL can be competitive with bespoke execution. Conversely, Gu-Yang demonstrates that a generic compiler which ignores locality can lose badly. The proposed probe must therefore measure **memory traffic and locality**, not arithmetic timing alone.

---

## 12. Proposed benchmark matrix

A future validation workplan should benchmark exact semantic twins across several scales. Suggested populations:

```text
1K
16K
64K
256K
1M
```

Add a larger point only if the adapter's memory budget makes it meaningful.

### 12.1 Execution variants

For each applicable workload:

- CPU/reference oracle where feasible;
- bespoke/native WGSL;
- canonical EML interpreter;
- canonical SSA JIT;
- fixed tile backend;
- fused multi-law tile backend.

### 12.2 Required measurements

At minimum:

- total generation/pass time;
- rows/s and edges/s;
- global bytes read/written if measurable;
- dispatch count;
- pipeline count/cache hits;
- table/buffer allocations and rebuilds per generation;
- workgroup-memory usage;
- register pressure / spills where exposed;
- occupancy/stall data where admitted tooling exposes it;
- tile utilization;
- scratch high-water mark;
- exact parity result/digest.

### 12.3 Suggested workloads

1. **Tiny arithmetic law** — establishes tile overhead floor.
2. **EXP/LN law** — Logistic/PowerLaw/`eml(x,y)` sized programs.
3. **Deep broad EML DAG** — deliberately balanced expression exposing row parallelism.
4. **Deep serial EML DAG** — negative control showing dependency-depth limit.
5. **Parent contention clearing** — many children, constrained capacity, CostBand residuals.
6. **Recursive reduce/disburse tree** — multiple depths and owner/resource buckets.
7. **Multi-channel STEAD** — several channels sharing one topology gather.
8. **STEAD + Gu-Yang compound flow** — counterflow/stall after shared gathered inputs.
9. **Compound-resource emergence** — scenario-neutral synthetic signals feeding claims; no authored adversarial pair.
10. **Native-vs-fused multi-law case** — strongest test of whether the generic representation can beat isolated native kernels.

---

## 13. Candidate falsifiers / exit conditions

No production promotion should occur unless all semantic and resource invariants are falsifiable.

### 13.1 Semantic parity

Same initial admitted state + same recorded integration schedule must produce bit-equivalent authoritative outputs between reference and tiled execution. A planted tile-route/source-column defect must turn the referee red.

### 13.2 No new semantic authority

A grep/type/callgraph proof should show tile descriptors do not become authoritative identity, lifecycle, ordering, contention, or history structures.

### 13.3 Zero steady-state churn

After warm-up/admission, ordinary generations must show zero tile-table/buffer allocation and zero SimThing matrix rebuild attributable to tile execution. Explicit epoch remap remains lawful and separately recorded.

### 13.4 Memory-bound regression detection

The probe must have a case that fails if tiled execution regresses to naive per-edge global gathers, so the Gu-Yang 5.7 failure shape cannot quietly return.

### 13.5 Parallelism must be real

Record dependency width/utilization and include a balanced-vs-serial expression pair. A backend that merely wraps sequential EML in a rectangular buffer is not a successful tile fabric.

### 13.6 Fair native baseline

The bespoke/native comparator must implement the same semantics and consume equivalent resident inputs. Do not compare fused EML against an intentionally unfused or CPU-orchestrated strawman.

### 13.7 Performance threshold — review required

Do not canonize a numeric gate in this document. Candidate review should decide whether success means native parity within a small band at 1M rows plus at least one strict fused-case win, or a different measured threshold. The inherited 5.7 gates are useful precedent but should not be copied mechanically before the workload is characterized.

---

## 14. Additional questions that should be explored

These are material to a future test/validation plan.

### 14.1 Resolution-law factorization

Can proportional, ordered, price-clearing, winner-take-most, and persistence/attrition be expressed as one generic constrained-clearing surface plus EML parameters/CostBands, or are multiple admitted numerical laws genuinely required? Avoid both premature enum creation and fake “one formula” purity.

### 14.2 Emergent adversariality vs relation priors

How should prior relations/policies influence willingness-to-yield without making relation identity the definition of contention? Test neutral parties becoming adversarial through persistent failed clearing, and hostile priors peacefully clearing when supply is abundant.

### 14.3 Temporal memory and hysteresis

What generic finite-memory lanes are needed for persistent contention? EXP-backed decay/EMA is available, but the system needs explicit horizons and must avoid permanent conflict flags.

### 14.4 Compound-value provenance

Preserve dependency provenance from a derived signal/CostBand back to source lanes for Studio/debug/corpus use. This is observational metadata, not a second causal graph.

### 14.5 Corpus surfaces

Candidate scenario-neutral columns include claim pressure, capacity utilization, fulfillment dispersion, allocation concentration/entropy, residual claim, CostBand expenditure, Gu-Yang gross/net/stall, persistence age/decayed memory, and ownership crossings. These can form **resource-tension trajectories** without domain labels such as war/revolution/crisis.

### 14.6 Multi-channel STEAD semantics

Determine whether several channels can legally share one gather/workgroup tile while retaining distinct domain certificates, canonical order, conductance proofs, and output/materialization policies.

### 14.7 Tree-depth scheduling

Explore depth-bucketed reduce-up and reverse-depth disburse scheduling over the same stable rows. Physical batching must never make physical row order semantic.

### 14.8 Active membership without compaction

Measure index-list / mask approaches for sparse active tile populations. Repacking the state matrix to improve a temporary EML cohort should be presumed wrong unless an explicit epoch/placement decision authorizes it.

### 14.9 Tile shape auto-selection

Tile shape may be adapter-specific and measurable while semantics remain adapter-independent. Explore several fixed classes rather than one universal size; cache plans by complete semantic-program identity + physical resource class.

### 14.10 CSE and cross-law fusion boundaries

How far may common-subexpression elimination/fusion extend while preserving exact arithmetic order? Pure independent subexpressions may be shared; reassociation of non-associative folds is not implied.

### 14.11 Primitive-vs-tile economics

Use the tile probe to reassess when deep EML expansions truly justify a new exact primitive. EXP/LN are admitted because they have independent semantic and cost value. If a future trig/deep expansion becomes fast enough when tiled, the pressure to grow `CLOSED_OPCODES` may fall.

### 14.12 Odrzywolek/pure-EML circuit experiment

As a research workload only, place-and-route known pure-EML constructions and record node count versus DAG depth/width/tile utilization. Expression blowup may create parallel width, but pure EML remains a poor default vocabulary unless measurements overturn the existing conclusion.

### 14.13 Subgroup/cooperative execution

Test only after the across-SimThing path. Cooperative lane execution of one wide expression may be useful for unusually broad DAGs, but barriers/subgroup-width portability/underfill can make it worse than assigning lanes to independent SimThings.

### 14.14 Hardware matrix/tensor units

Treat MMA/cooperative-matrix use as an optional backend for subgraphs that genuinely map to matrix operations. Never make arbitrary EML semantics depend on tensor-core availability.

### 14.15 Async subtree seams

The 6.1/6.2 generation-stamp and recorded-integration laws must remain sovereign if different subtrees/tile blocks run at different speeds. Tiling may change placement/scheduling, never the recorded schedule semantics.

### 14.16 Residency interaction

The StemThing residency lane already treats capacity/allocation as RF + CostBand. Explore whether tile scratch/residency can consume the same admitted residency vocabulary without making the execution optimizer a semantic resource claimant. The safe default is that execution scratch is kernel physics; promote only if a real consumer requires authored competition for it.

---

## 15. Recommended decomposition toward testable rungs

This document should not become one giant rung. The semantic and execution questions should fail independently.

### Candidate A — `CONTENTION-COSTBAND-STEM-STUB-0`

**Purpose:** prove the intrinsic contention state-transition hinge on the base SimThing with no tile dependency.

Candidate scope:

- scenario-neutral parent with N child claims and bounded capacity;
- derive unresolved claims from the ordinary clearing result;
- route unresolved pressure through ordinary EML -> CostBand;
- prove clear claims produce zero contention action;
- prove persistent failed claims can create paid commitment without a domain/adversarial pair;
- recurse the same mechanism over at least two tree depths;
- preserve per-resource conservation and existing seam holding-account law;
- zero retained contention object/registry.

**Key planted defect:** hard-code a competitor/adversary relation or bypass the CostBand sink; referee must red.

### Candidate B — `EML-TENSOR-TILE-PROBE-0`

**Purpose:** workshop-leaf performance/feasibility probe only; no semantic production dependency.

Candidate scope:

- normalize EML to a dependency DAG;
- levelize and pack several fixed tile classes;
- aligned column ingress + bounded on-chip scratch;
- exact parity with reference evaluation;
- benchmark arithmetic, EXP/LN, balanced deep DAG, serial negative control;
- scale through 1M rows;
- record node/depth/width/utilization/memory/dispatch metrics;
- prove zero steady-state table allocation.

**Exit:** either demonstrate a credible native-parity asymptote or publish a negative result and reap the probe.

### Candidate C — `EML-TILED-REDUCE-DISBURSE-PROBE-0`

Only if B succeeds.

**Purpose:** test the actual SimThing workload rather than isolated expressions.

Candidate scope:

- depth-bucketed reduce-up;
- constrained parent clearing;
- intrinsic contention CostBand evaluation;
- reverse disbursement;
- stable residency/index maps;
- compare native/interpreter/JIT/tiled paths at scale.

### Candidate D — `FIELD-BUNDLE-TILED-GATHER-PROBE-0`

Only if the first tile probe and current tiled-gather debt align.

**Purpose:** test vector-valued STEAD + Gu-Yang field evaluation with one shared neighborhood load and several lawful channels.

This is the candidate most likely to show a strict win over isolated bespoke kernels because it can reuse topology/memory work across multiple authored laws.

### Candidate E — hierarchical-spatial emergence falsifier

Only after the semantic stub and relevant tile/field surfaces stand independently.

Use a scenario-neutral resource ecosystem where no adversarial pair is authored. Vary only capacities/recipes/weights/relations and demonstrate:

- no scarcity -> no contention;
- scarcity -> unresolved claims;
- persistent costly claims -> increasing contention pressure;
- spatial committed counterflows -> Gu-Yang stall/choke;
- relief/policy change/exhaustion -> de-escalation;
- same mechanics at multiple tree depths.

A domain-flavored “food availability / elite overproduction / inequality” example can be a later vendor exemplar, not the proof basis.

---

## 16. Likely interaction with 8.1 / 8.2

This exploration is timely because 8.1/8.2 are where contention becomes first-class executed RF behavior.

The present workplan already has two strong laws worth preserving:

- contention is N claims against bounded supply, not a combat domain;
- clearing/resolution is authored data, never a kind/enum branch.

The proposed expansion asks review to consider two refinements before those laws become harder to change:

1. **Intrinsic base stub:** make unresolved constrained clearing an intrinsic stem-cell contention surface rather than something an arena-specific caller must author.
2. **Attrition factorization:** test whether attrition should be modeled primarily as a persistence CostBand generated by unresolved contention rather than as a sibling clearing algorithm beside proportional/priority/price-clearing.

Do not amend 8.1/8.2 from this workshop document. A DA review should first decide whether these are true clarifications of the existing mechanism or a semantic change requiring explicit ladder amendment.

---

## 17. Proposed canonization text candidates for review

These are drafting candidates only.

### 17.1 CostBand state-transition hinge

> **CostBand is the universal state-transition hinge between observation and committed resource action.** EML/field evaluation may derive any lawful scalar pressure; no state-changing action follows merely because the value exists or crosses a band. A committed action exists only where an admitted sink draws ordinary constrained state through CostBand. The resulting state participates in the same RF/field cycle and may create later observations and CostBands; no rival action mechanism is admitted.

### 17.2 Intrinsic contention stub

> **Contention is intrinsic to constrained disbursement.** Whenever simultaneous committed claims cannot all clear against a SimThing's admitted constrained capacity, the unresolved portion is an ordinary derived contention surface on that same base SimThing. Unresolved demand is observation; costly persistence is an EML-priced CostBand. Adversariality is therefore an emergent degree of paid persistence under unresolved contention, never a required domain label or authored competitor pair.

### 17.3 Recursive law

> **The contention facility recurses only through the SimThing tree.** Child claims reduce upward, each constrained parent applies the same generic clearing/contention law, and allocations disburse downward. Spatial specialists gain no second contention mechanism: STEAD carries the resulting influence laterally and Gu-Yang exposes conservative counterflow stall. A separate contention tree/graph/object hierarchy is forbidden.

### 17.4 Tile law

> **EML tiling is execution-only.** A tile is a fixed resource-bounded physical packing of already-admitted EML/lane evaluation over aligned SimThing rows. Tile shape, adapter, workgroup, subgroup, scratch placement, fusion, and scheduling have no semantic authority. Reference and tiled execution of the same admitted generation must agree under the declared exactness law; otherwise the tile backend is invalid.

---

## 18. Decision questions for DA / architecture review

A review should rule at least these questions before rung minting:

1. Is CostBand-as-state-transition-hinge a clarification of the existing CostBand law or a Tier-level semantic amendment?
2. Should intrinsic contention attach to every constrained resource-bearing parent, with spatial behavior purely additive through STEAD/Gu-Yang?
3. Is `unresolved claim -> contention observation` sufficient as the base stub, or does generic clearing need a richer feasible-envelope representation?
4. Should attrition move from “resolution rule” toward “costly persistence under unresolved contention,” and if so what remains of the 8.2 authored rule taxonomy?
5. Which relationship/priority inputs are already admissible data, and which would accidentally mint a new relation authority?
6. What exact data distinguishes conserved resources, derived signals, and committed claims at admission?
7. Does the tile probe run before 8.1/8.2, in parallel as workshop evidence, or after the semantic contention stub?
8. What native-parity performance gate is strong enough to justify production promotion without preordaining a win?
9. Can multi-channel field gathering be introduced as a JIT/tile optimization while preserving `FIELD-SWEEP-SINGLE-PATH` and canonical fold order?
10. Which tile-plan changes are legal mid-session versus epoch-bound, given 6.4/6.5 logical identity and residency laws?

---

## 19. Recommended immediate next action

Treat this document as the input to a decorrelated design review, not as implementation authority.

The most useful review order is:

1. **Semantic ruling first:** challenge the intrinsic contention CostBand stub and attrition factorization against CostBand, RF conservation, 8.1/8.2, and the one-StemThing principle.
2. **Execution ruling second:** challenge whether EML tensor tiles can remain a semantics-free backend under exact determinism, canonical fold order, async schedules, and stable residency.
3. **Only then mint probes/rungs:** at minimum separate the semantic contention proof from the performance tile probe.

The success condition is not “we invented a new contention engine” or “we invented a tensor VM.” It is the opposite:

> **One recursive SimThing acquires no new anatomy. Its ordinary resource lanes naturally surface constrained contention; CostBand remains the only action hinge; STEAD/Gu-Yang remain the lateral mechanisms; and the GPU learns to execute the same admitted EML/recursive flow at massive scale through a disposable tiled lowering.**
