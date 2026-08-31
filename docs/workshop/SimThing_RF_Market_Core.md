# SimThing RF Market Core
## Receive → resolve → settle → disburse, recursively

> **Status: WORKSHOP DRAFT / NON-NORMATIVE / DA+OWNER REVIEW CANDIDATE.**
>
> This document records the Owner/engineering design-session convergence reached while Phase 14
> `RESIDENT-CLEARING-*` is held for the RF-market-core ruling. It is deliberately housed under
> `docs/workshop/` until Owner and DA review approve its laws and bind them into the active 14.x
> rows. It does not itself amend the ladder, the constitution, or the frozen clearing oracle.
>
> **Current governance boundary:**
>
> - Phase-14 proposal and Owner Germ Mandate: Board
>   [`5471915320`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5471915320)
>   and [`5472392553`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5472392553).
> - 14.2 germ-form remand, orthogonal to this market-law review: Board
>   [`5480224895`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5480224895).
> - Native-field-clearing design hold: Board
>   [`5480719752`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5480719752).
> - **Germ Self-Consumption Law:** Board
>   [`5483829845`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5483829845). Its
>   normative home remains that Board mint until canonization; this workshop section is only its
>   engineering integration candidate.
> - Canonical law remains
>   [`../simthing_core_design.md`](../simthing_core_design.md),
>   [`../stead_stemthing_unification.md`](../stead_stemthing_unification.md), and the live
>   0.0.8.7 workplan. Where this workshop draft conflicts with those authorities, **STOP and
>   escalate**.
>
> **Drafting method:** dissolve before inventing. First locate the required value as an already-born
> RF/Field-Triad surface. Then locate the required function as an already-admitted RF/EML operation.
> Only after both fail may Phase 14 name a new runtime surface.
>
> **Research caveat:** §9 contains mathematical interpretations, candidate recurrences, and
> performance hypotheses. They are retained to aid later proof and optimization, not to smuggle new
> normative physics into the germ. Every §9 candidate remains subordinate to landed semantics,
> source archaeology, measured performance, and DA/Owner review.
>
> **Latest amendment — DA review closure of recursive-germ residuals.** The full conformance pass is
> retained, with three sharpenings: role names over a self-consuming product must be aliases or truly
> conversion-free views rather than paired newtypes bridged by `From`/`Into`; the possible `T_s`/`T_d`
> collapse is now an explicit signed-RF archaeology candidate rather than an aesthetic suggestion; and
> detached seams are explicitly falsified if they translate the economic payload. A settled-code
> recursive-port census is also named as an edict obligation so any newly exposed historical mismatch
> becomes dated constitutional debt rather than an unscheduled surprise.

---

# 0. Executive thesis

The constrained-resource market is not a clearing service beside SimThing.

It is the complete recursive breath of the StemThing RF organ:

```text
children expose lawful need, deficit, and pressure
                    ↓
            recursive RF reduce-up
                    ↓
      STEAD / PALMA / Gu-Yang resolution
                    ↓
 branch pressure shapes continuous allocation
                    ↓
       continuous allocation / disbursement
                    ↓
      exact type-preserving settlement
                    ↓
      exact constrained product / U
                    ↓
 child consumes the same exact product as supply
                    ↓
       the same market germ recurses
```

The upward stroke receives market pressure. The Field Triad resolves value, impedance, and
realizability. The downward stroke disburses the continuously resolved flow. Exact settlement is the
terminal continuous-to-discrete stage of that same program.

The historical defect is narrower than “receiving was missing” or “clearing was missing”:

> **Continuous receiving and continuous disbursement already existed as two strokes of the RF
> cycle. Exact constrained settlement was implemented beside that cycle rather than sealed as its
> intrinsic terminal stage, and first-order unresolved pressure was not yet proven to re-enter the
> same demand germ without authored bridging.**

The desired closure is not a smarter clearinghouse. It is recognition and sealing of the one RF
market filter already distributed through the germ.

## 0.1 The mirror insight that exposed the missing edge

The design-session question was simple:

> If a parent already knows how to disburse a finite constrained resource among `N` children, is the
> receiving side not the mirror of that same operation?

The answer is yes. A parent cannot lawfully disburse under the RF model without receiving, directly or
recursively, the demand and shortfall condition of each child branch.

That mirror exposed the remaining feedback hinge:

```text
unresolved descendant pressure
        ↓
branch-attributed parent visibility
        ↓
next-generation continuous allocation guidance
```

A design in which the parent can see branch scarcity but allocates as though every branch were equally
unstressed is not a complete RF market germ. It is an instrumented but economically inert disbursement
pass.

> **Phase 14 may not graduate with descendant pressure visible but economically inert.**

## 0.2 The self-consumption insight that exposed the port seam

A recursive germ facility necessarily consumes its own output:

```text
level n germ emits product T
        ↓
level n+1 germ consumes product T
        ↓
level n+1 germ emits product T
```

If a conversion exists only because the producer calls the value “grant” while the consumer calls it
“supply,” the conversion is not architecture; it is evidence that the common conserved product was
modeled at the wrong level.

This applies twice in the RF market:

```text
DOWNWARD SUPPLY CLOSURE
    exact constrained product T_s emitted by parent
        =
    exact constrained product T_s consumed by child

UPWARD / TEMPORAL DEMAND CLOSURE
    lawful unmet-demand product T_d emitted upward
        =
    lawful unmet-demand product T_d consumed by parent reduction

    unresolved T_d at generation N
        =
    first-order demand/pressure T_d entering generation N+1
```

The law does **not** yet prove `T_s == T_d`. They may be two closed product algebras, or archaeology may
show that both are projections of one signed RF quantity. The concrete candidate to prove or falsify is
that the existing signed Gu-Yang/RF flow substrate already carries supply and demand as opposite-signed
roles of one conserved product. That hypothesis is explicitly subordinate to archaeology and DA/Owner
review; falsifier 31 forbids collapsing the types merely because the symmetry looks elegant.

## 0.3 Candidate identity sentence

> **The GPU-resident RF market endpoint is the terminal exact-settlement stage of the Field Triad's
> ordinary reduce/disburse cycle. Every StemThing intrinsically receives branch-attributed
> descendant market pressure, lets that pressure inform later continuous allocation, disburses
> Triad-resolved constrained flow, and type-preservingly settles that flow into the exact constrained
> product its children themselves consume as supply. Every child carries the same germ. The endpoint
> does not independently value, route, model, age, or translate claims.**

## 0.4 The intended deletion

The following must not survive as peer runtime concepts:

```text
ClearinghouseThing
ClearingManager
ClearingService
market-side route or congestion evaluator
clearing-owned urgency model
clearing-owned weight/share cache
host preweight table
private field solver
second receive/disbursement engine
second market history
host descendant-pressure scan
germ-output → germ-input adapter/converter/projector
GrantDTO → ChildSupplyDTO economic translation
U → authored persistence → first-order Need bridge
role-newtype pair + From/Into bridge for one recursive product
```

“Clearinghouse” remains useful in design conversation only as a locator for the exact-settlement
endpoint. The production architecture should be expressible without that peer noun.

---

# 1. The RF market mirror

## 1.1 Upward stroke — receive branch-attributed pressure

A parent receives the market condition of its descendants through ordinary RF reduction:

```text
leaf or interior lawful Need / deficit / IntrinsicFlow / Balance / U
                              ↓
                         reduce-up
                              ↓
one aggregate per direct child branch at the governing parent
                              ↓
parent-local aggregate by owner / resource / scope
                              ↓
residual pressure continues upward when not settled locally
```

The pressure must remain **branch-attributed** at each allocation boundary. A single parent-wide total
such as “subtree unmet demand = 500” is insufficient to determine which direct child branch should
receive more of a scarce resource.

For parent `p` and direct child `c`, the semantic equivalent of the following must be available without
scanning all descendants at allocation time:

```math
P_{p\rightarrow c,N}
=
\text{unresolved lawful pressure represented by child }c\text{ and its subtree at }N
```

This does not authorize a new `BranchPressure` column. It requires the existing RF upsweep to preserve
enough child-segment attribution for the parent's continuous allocation pass.

## 1.2 Center — continuous field resolution

The RF/Field-Triad center supplies the continuous market facts:

| authority | market meaning |
|---|---|
| **STEAD** | lawful demand pressure, stakes, unresolved pressure, and resident value state |
| **PALMA** | reachability and accumulated impedance to the governing market home |
| **Gu-Yang** | signed conservative throughput, available/realized flow, saturation, stall, blockage |
| **EML** | admitted numerical coupling among already-resident operands; never a second market service |

The generic field engine is already one map/fold/post execution IR; algebra remains admitted EML data,
not a runtime field-kind branch. See
[`../../crates/simthing-kernel/src/field_sweep.rs`](../../crates/simthing-kernel/src/field_sweep.rs).

## 1.3 Native pressure-to-allocation feedback

The mirror closes only when unresolved branch pressure influences the later continuous allocation to
that branch.

Conceptually:

```math
W_{p\rightarrow c,N+1}
=
G\!\left(
P_{p\rightarrow c,N+1},
Policy_{p\rightarrow c,N+1},
Triad_{p\rightarrow c,N+1}
\right)
```

where `W` is the already-existing `AllocatorWeight` or a proved equivalent—not a new clearing-owned
weight.

The resulting continuous child allocation may retain the existing guarded share form:

```math
X_{p\rightarrow c}
=
S_p
\frac{W_{p\rightarrow c}}
{\sum_j W_{p\rightarrow j}}
```

subject to lawful request, PALMA reach/impedance, and Gu-Yang serviceability or commitment semantics.

The required native behavior is:

```text
larger unresolved lawful branch pressure
        ↓
stronger later continuous allocation pressure for that branch
        ↓
all else equal, larger provisional allocation
        ↓
exact settlement follows the provisional allocation
```

Authored policy may deform, cap, decay, reverse, or deliberately occlude that pressure. Authored policy
must not be the only source of first-order urgency.

## 1.4 Downward stroke — disburse continuous flow

The existing accumulator anatomy already contains:

```text
IntrinsicFlow
AllocatedFlow { arena }
Balance(...)
AllocatorWeight { arena }
```

See
[`../../crates/simthing-core/src/accumulator_spec.rs`](../../crates/simthing-core/src/accumulator_spec.rs).

The standing archaeology identifies existing weight-sum propagation, a guarded `child_share_formula`,
and an `AllocatedFlow` downsweep as the presumptive continuous market-share path.

The intended complete path is:

```text
branch-attributed Need / Balance / U
                ↓ upsweep
existing pressure-to-weight/share binding
                ↓
parent weight sum / available flow
                ↓ guarded child_share_formula or proved equivalent
child AllocatedFlow
                ↓ downsweep
```

Under Germ Self-Consumption, `AllocatedFlow` now has an explicit proof obligation: **the continuous
flow received by an interior child must be the same continuous RF vocabulary that child consumes when
acting as allocator for its own descendants.** Any copy/project/retype step into a separate
“available-supply” representation is presumptively a germ-form defect.

## 1.5 Edge — type-preserving exact settlement

Continuous flow is not yet exact possession. The terminal stage converts the already-resolved
continuous allocation into exact units, but **the boundary is numerical, not ontological**.

```text
continuous constrained product
        ↓ exact quantization
exact constrained product T_s
        ↓
child consumes T_s as its own supply
```

The current CPU constrained-clear oracle supplies settled laws for exact grouping, integer
apportionment, remainder order, tie rotation, typed failure, and grant construction. See
[`../../crates/simthing-spec/src/spec/constrained_clearing.rs`](../../crates/simthing-spec/src/spec/constrained_clearing.rs).

Its host collections and separate `Supply`/`Grant` conveniences are an oracle implementation detail,
not automatically the permanent resident product ontology.

The exact product must preserve every semantic fact that survives the parent→child role transition,
presumptively including exact resource identity, quantity, commitment state, provenance/lineage,
lifecycle, and whatever scope identity remains semantically persistent. “Grant” and “supply” may remain
role names over the same type, but those role names must be aliases or truly conversion-free views.
A pair of wrapper/newtypes connected by `From`, `Into`, a constructor, or a copy/projection function is
not self-consumption; it is the forbidden adapter wearing type-safety ceremony.

## 1.6 Re-enter — exact product becomes child supply

The defining recursive identity is:

```text
parent perspective: grant T_s
              =
child perspective: supply T_s
```

An interior node is simultaneously grantee of its parent and granter to its children. The root is the
degenerate case with no upstream recursive granter; a leaf is the degenerate case with no recursive
child disbursement.

No semantic transition is admitted between parent grant and child supply. Serialization, realm
qualification, generation stamps, and seam-fact envelopes are lawful transport/authority wrappers;
they may not change the economic payload type.

---

# 2. One germ at every depth

For a StemThing node `v`, the market germ can be described abstractly as:

```math
\mathcal{M}_v(S_v, \{P_c\}, \Phi_v)
\rightarrow
(L_v, \{G_c\}, \{U_c\})
```

where:

- `S_v` is exact constrained supply available to node `v`;
- `P_c` is lawful branch pressure surfaced by child `c` and its subtree;
- `Φ_v` is the already-resolved RF/Field-Triad state at the governing scope;
- `L_v` is lawful local retention/consumption;
- `G_c` is the exact child allocation expressed in the same exact constrained-product type as `S_v`;
- `U_c` is unresolved lawful demand.

This is a descriptive model, not authorization for new fields or types.

```text
ROOT:
    authored/intrinsic source binds canonical supply product T_s
        → continuous allocation
        → exact child T_s

INTERIOR (canonical form):
    exact parent T_s
        → same RF market operator
        → exact child T_s

LEAF:
    exact parent T_s
        → ordinary holding/actuation sink
        → no recursive child emission
```

**Port identity at every depth:** root, interior, and leaf expose the same recursive germ ports. The
root is special only because its intake is bound from an admitted source. The leaf is special only
because its output terminates in a non-germ sink. The interior node is the canonical form: `T_s in →
T_s out` with no economic adapter.

The same principle applies to recursive demand:

```text
leaf/interior demand product T_d
        ↑
parent reduction consumes T_d and emits reduced T_d upward
```

and temporally:

```text
unresolved T_d at N
        ↓ Current→Next germ recurrence
T_d pressure/Need at N+1
```

Whether `T_s` and `T_d` are ultimately one signed RF product is an open archaeology/design question.
The concrete candidate is the already-signed RF/Gu-Yang quantity substrate: supply and demand may prove
to be opposite-signed projections/roles of one conserved flow product. This candidate must be proven
from landed semantics and consumers, not inferred from elegance alone.

---

# 3. RECEIVE, not query, recompute, or translate

The exact-settlement endpoint **receives** a completed continuous allocation from the preceding RF
program stage.

The settlement stage may not perform or own:

```text
STEAD reduction
PALMA relaxation or route walk
Gu-Yang solve or private flux estimate
host route-distance lookup
host congestion lookup
host descendant-pressure scan
private urgency calculation
private policy evaluator
new persistent NativeClearingPotential plane
new persistent FieldShare plane
new persistent BranchPressure plane
copied per-tick weight buffer
clearing-owned cache
second FieldSweep registration
host-built per-claim preweight table
grant→supply economic translation
```

Its lawful inputs are bindings to already-authoritative Current-plane roles such as:

```text
lawful requested quantity / legal cap
continuous AllocatedFlow or proved equivalent
exact available supply product T_s
Gu-Yang serviceability / commitment bound where applicable
claimant logical identity
generation authority
commitment semantics
existing hard-precedence authority
```

The upstream RF allocation plan—not settlement—may bind the existing branch-attributed pressure and
policy inputs needed to produce `AllocatedFlow`.

> **No new persistent per-claim or per-branch state is admitted merely to make settlement
> field-aware or urgency-aware. No adapter is admitted merely to reconcile two role names for the
> same recursive germ product.**

`NativeClearingPotential`, `FieldShare`, `ClaimFieldView`, and `BranchPressure` remain explanatory
terms until the surface-reuse archaeology proves that no existing surface carries the required
meaning.

---

# 4. Existing authorities to consume, not duplicate

## 4.1 RF role anatomy

| existing role | candidate market use |
|---|---|
| `IntrinsicFlow` | resident signed need/production contribution |
| `AllocatedFlow { arena }` | continuous parent-to-child allocation |
| `Balance(BalanceSpec)` | integrated surplus/need ledger |
| `AllocatorWeight { arena }` | continuous child-split input |

These roles compile away before GPU execution; `simthing-sim` must not branch on them as domain kinds.

## 4.2 OrderBand and arena stage order

Existing RF and resource-economy machinery already uses `OrderBand`/band-layout concepts to stage
operations. Archaeology must distinguish:

```text
execution stage ordering
    versus
hard economic precedence among lawful claims
```

No second `OrderBand` vocabulary may be minted. Existing stage order must not be silently redefined as
market preference merely because the names resemble one another.

## 4.3 Continuous allocation surfaces

The presumptive existing continuous market path is:

```text
children's IntrinsicFlow / Balance / U + AllocatorWeight
                ↓ upsweep
parent branch pressure + intrinsic-flow sum + weight sum
                ↓ broadcast / continuous resolution
parent available / allocated flow
                ↓ guarded child_share_formula
                ↓ downsweep
child AllocatedFlow
```

The required archaeology question is whether the live production meaning, units, sign, bounds, scope,
ordering, branch attribution, and **self-consumption path** are sufficient for constrained exact
settlement.

## 4.4 Native branch-pressure feedback surface

The presumptive binding is:

```text
Need / Balance / U from child subtree
        ↓ ordinary branch-attributed upsweep
existing bounded pressure/policy interpretation
        ↓
AllocatorWeight or proved existing continuous-share operand
        ↓
weight-sum / child-share / AllocatedFlow
```

The implementation must determine whether this already exists as production authority, is wired but
not default, is proof/preview residue, or is genuinely missing.

## 4.5 Grant lifecycle and holding accounts

The current grant-lifecycle substrate already provides exact relationship facts and the conserved
capacity grammar:

```text
free
in_flight
occupied
capacity
```

See
[`../../crates/simthing-core/src/grant_lifecycle.rs`](../../crates/simthing-core/src/grant_lifecycle.rs).

Under self-consumption these are states/roles of the same conserved exact product relationship, not
conversions among unrelated economic objects.

## 4.6 Placement is downstream physics, not product completion

For residency-like markets, a capacity grant already is the child's exact constrained supply. Kernel
placement answers a distinct question:

```text
market T_s:
    WHO / WHETHER / HOW MUCH exact capacity belongs to the child

placement boundary:
    WHERE that capacity is physically realized
```

`VerifiedGrowthResidencyCommit` or analogous structural authorization is therefore a consequence of
using `T_s`, not an adapter that turns a grant into usable supply.

## 4.7 Generation and async seams

Per-tree generation authority, no-wait stamped integration, and one recorded schedule already permit
independently advancing subtrees. The RF market core must remain per executing tree and must not assume
one global clock, host, process, device, schedule, registry, or raw-ID namespace.

The seam may wrap the canonical product with:

```text
RealmQualified<T_s>
GenerationStamped<T_s>
SeamFact<T_s>
serialized T_s bytes
```

but may not translate `T_s` into a different economic payload for the destination.

---

# 5. Continuous market versus exact settlement

## 5.1 Field Triad authority

For claim `i`, explanatory terms are:

- `r_i`: lawful requested quantity;
- `a_i`: current serviceable quantity under route/channel constraints;
- `p_i`: branch-attributed unresolved lawful pressure;
- `x_i`: continuous provisional allocation emitted by the RF/Triad disbursement program;
- `g_i`: exact integer quantity carried by the canonical exact constrained product `T_s`.

The intended constraints are:

```math
0 \le x_i \le a_i \le r_i
```

and:

```math
\sum_i x_i \le S
```

for exact supply `S` at the governing parent.

Holding policy and serviceability equal, native pressure should be monotone in continuous share:

```math
p_i' > p_i
\quad\Longrightarrow\quad
x_i' \ge x_i
```

subject to admitted caps, competing pressure, bounded recurrence, and Gu-Yang realizability.

## 5.2 Exact endpoint authority — a type-preserving quantizer

The settlement stage owns only:

1. exact representability/admission of the settlement envelope;
2. continuous-to-integer quantization;
3. exact supply conservation;
4. indivisible residue under canonical deterministic law;
5. exact constrained-product quantity and commitment-state transition;
6. exact relationship provenance;
7. one replay/schedule history committed atomically with state.

The conceptual boundary is:

```text
continuous constrained allocation x_i
        ↓ exact quantization
canonical exact constrained product T_s(g_i)
```

The endpoint may not re-run the continuous market, calculate branch urgency, or mint a second economic
ontology merely because the result is discrete.

## 5.3 The CPU oracle is exempt from self-consumption, not from canonical semantics

The CPU oracle is a one-way non-germ consumer and is therefore outside the self-consumption scope. It
may use fixture/report wrappers. But it must referee the same canonical product semantics:

```text
resident germ:
    T_s → T_s

CPU oracle:
    reads canonical T_s fixture
    predicts canonical settlement T_s
    compares result
```

Separate host-oriented `Supply`/`Grant` structs may not dictate the production ABI if they encode an
ontology the recursive resident germ no longer has.

---

# 6. Hard precedence is not continuous share

The current oracle clears higher exact score bands completely before lower bands and shares only among
claims with identical score bits. Feeding an arbitrary continuous field scalar directly into that
score law can turn float dust into strict precedence:

```text
0.8731 > 0.8729 > 0.8714
```

The market core therefore keeps two meanings separate:

```text
hard precedence
    explicit legal/emergency/policy order

continuous allocation
    RF/Triad-resolved soft share informed by branch pressure
    within a precedence class
```

The hard-precedence authority must be found in existing `OrderBand`, demand priority, or order-weight
surfaces. The continuous allocation must be found in existing `AllocatedFlow` or a proved existing RF
post/fold result.

The generalized law must contain the frozen neutral case:

```text
one neutral precedence class
continuous share basis = lawful requested quantity
exact settlement = current proportional-by-request law
largest remainder + generation-rotated exact ties
```

The exact neutral pressure→weight transform remains an Owner ruling (§16).

---

# 7. Three shortfall stages must remain distinct

## 7.1 Impaired lawful demand — before settlement

```math
U_i^{impairment} = r_i - a_i
```

Lawful demand that cannot currently traverse the admitted route/channel.

## 7.2 Contention shortfall — at settlement

```math
U_i^{contention} = a_i - g_i
```

Physically serviceable demand that did not receive exact supply.

## 7.3 Delivery shortfall — after entitlement

For exact grant quantity `g_i` and realized delivery `y_i`:

```math
B_i^{delivery} = g_i - y_i
```

The true blocked legal grant where entitlement may exist before realization.

## 7.4 Commitment semantics

```text
immediately executable flow
    exact T_s quantity is capped by current Gu-Yang serviceability

entitlement then deliver
    exact T_s may enter in_flight
    Gu-Yang governs later realization
```

---

# 8. Urgency, persistence, branch attribution, and the second self-consumption loop

The governing scope can already receive or derive demand, surplus/deficit, PALMA impedance, Gu-Yang
flow/saturation/stall, `U`, refusal disposition, and grant commitment state.

Visibility alone is not enough. Branch-attributed pressure must participate in the later continuous
allocation to that branch.

## 8.1 Demand self-consumption

The recursive demand side must close under one authoritative quantity/product vocabulary:

```text
child emits lawful demand T_d upward
        ↓
parent RF reduction consumes T_d
        ↓
parent emits reduced T_d upward
```

At the generation boundary, first-order unresolved quantity must also self-consume:

```text
unresolved T_d at N
        ↓ ordinary bounded Current→Next recurrence
T_d / pressure at N+1
```

The current explicit unresolved-demand consequence path routes through authored EML persistence
valuation, CostBand funding, and a later OverlayThing consequence. That path remains lawful for
**authored secondary consequences**. It may not be the required adapter that recreates first-order
market demand after the germ emitted unresolved demand.

Therefore implementation archaeology must classify whether ordinary `Need`/`Balance`/STEAD already:

```text
A. preserves unresolved lawful demand directly
B. accumulates it directly into greater pressure
C. currently requires the authored consequence path
```

If C is the current implementation, Phase 14 must rehome first-order persistence into the recursive
RF-demand germ or STOP for Owner/DA ruling.

## 8.2 Candidate bounded recurrence

If first-order persistence is missing, the candidate semantic recurrence is:

```math
P_{c,N+1}
=
\mathcal{B}\!\left(
P_{c,N},
U_{c,N}^{impairment}
+
U_{c,N}^{contention}
+
B_{c,N}^{delivery}
\right)
```

where `B` is the existing bounded Current→Next recurrence or a rehomed equivalent.

This is a semantic candidate, not permission to mint a new `BranchPressure` plane or `UrgencyManager`.

## 8.3 No double counting

Pressure is reduced once per tree edge. A parent consumes one aggregate per direct child branch; it
may not count both the branch aggregate and the same descendants again.

## 8.4 Market transparency

> **Market transparency is the default. Demand, urgency, impairment cause, saturation, impedance,
> provisional allocation, exact grant, and unmet-demand volume are visible to the governing scope as
> born field, RF, lifecycle, and schedule state. Occlusion is only an explicit authored act—an
> admitted, recorded, auditable policy or perception overlay—never substrate silence.**

---

# 9. Mathematical interpretation and performance research notes

This section records useful mathematical interpretations and optimization candidates. **None of these
formulae supersedes the archaeology-first reuse rule or the Germ Self-Consumption Law.** They are
retained to help prove, falsify, or optimize the existing RF market surfaces later.

> **DA/research review caveat:** every candidate below must be checked against the finally adjudicated
> product types, units, sign conventions, commitment classes, generation pacing, and actual GPU
> lowering. A mathematically attractive formulation is not acceptance evidence by itself. The DA may
> accept the interpretation, narrow it to a proof oracle, defer it to the performance ledger, or
> reject it without disturbing the core market laws.

## 9.1 Weighted proportional fairness explains the existing child-share shape

The normalized child-share form:

```math
x_i
=
S\frac{w_i}{\sum_j w_j}
```

is the exact optimum of the weighted proportional-fair problem:

```math
\max_{x_i>0}
\sum_i w_i\log x_i
```

subject to:

```math
\sum_i x_i \le S.
```

The Lagrangian first-order condition is:

```math
\frac{w_i}{x_i}=\lambda,
```

which yields the normalized share.

**Engineering consequence:** if archaeology confirms `child_share_formula`/`AllocatedFlow` as the live
continuous authority, the existing RF disbursement already has a standard welfare interpretation.
Branch pressure can determine or deform `w_i` without inventing a new market solver.

The broader alpha-fair family remains a research/policy candidate:

```math
U_\alpha(x)=
\begin{cases}
\log x,&\alpha=1\\
\frac{x^{1-\alpha}}{1-\alpha},&\alpha\ne1.
\end{cases}
```

For one uncapped parent constraint:

```math
x_i \propto w_i^{1/\alpha}.
```

This is a policy-family interpretation, not a mandate to add an `alpha` runtime field.

## 9.2 Backpressure is the closest established analogy for native urgency

Unresolved branch pressure has the same functional role as backlog in queue-backpressure / MaxWeight
systems:

```text
unresolved RF pressure
    ↔ queue backlog

PALMA reach / Gu-Yang serviceability
    ↔ feasible connectivity/capacity

continuous RF disbursement
    ↔ service allocation

AllocatedFlow
    ↔ service actually scheduled
```

A pure MaxWeight objective has the shape:

```math
\max_x \sum_i P_i x_i
```

subject to the feasible capacity region. That is useful as a stability/urgency lens, but for a single
parent it can become highly concentrated. Backpressure therefore remains an **interpretation of the
native urgency signal**, not a mandate to replace the existing smooth sharing law.

## 9.3 A combined continuous-market oracle candidate

A mathematically clean research oracle is:

```math
\max_{\{x_i\}}
\left[
\sum_i w_i(P_i)\log x_i
-
\eta\sum_i Z_i x_i
\right]
```

subject to:

```math
0\le x_i\le a_i,
\qquad
\sum_i x_i\le S.
```

For an uncapped active branch, the KKT condition yields:

```math
x_i
=
\frac{w_i(P_i)}{\lambda+\eta Z_i},
```

and with Gu-Yang caps:

```math
x_i
=
\min\left(
a_i,
\frac{w_i(P_i)}{\lambda+\eta Z_i}
\right).
```

**Cap caveat:** once caps are active, `lambda` is not a claimant-local closed formula; it must be
solved against the active set. Sort-based water filling is one conventional `O(n log n)` oracle
realization, **not** a mandated resident algorithm. Active-set selection, monotone root search, or
specialized segmented methods may also be viable later.

**Disposition:** research oracle / interpretation candidate only. If the current RF/Triad path already
produces equivalent shares, this mathematics validates it. If a missing continuous operation is
proven, it belongs in the existing RF/FieldSweep post/fold/disbursement stage, never in exact
settlement.

## 9.4 Electrical analogy — useful but bounded

On a unique parent→descendant tree, accumulated additive route impedance is exact by simple series
composition:

```math
Z_c = Z_p + z_{p\rightarrow c}.
```

For ancestor `p` and descendant `v`, if cumulative root-relative impedance is valid:

```math
Z_{p\rightarrow v}=Z_v-Z_p.
```

This means strict tree-vertical PALMA may not require a general graph solve or iterative relaxation.
A conductance-weighted rule such as:

```math
W_i \propto \frac{P_i}{Z_i}
```

is a candidate EML policy only when units and semantics support the analogy. It is not universal
substrate law.

The literal Ohm relation, where applicable, is:

```math
I = \frac{\Delta V}{R}=G\Delta V,
\qquad G=1/R.
```

No vertical Laplacian solver, matrix inversion, or QAOA mechanism is chartered.

## 9.5 PALMA vertical specialization

For a true rooted tree with one route to the governing ancestor:

```text
series edge costs:
    add along the path

alternative-route minimum:
    degenerate because no alternative exists
```

Therefore a `TreeVertical` lowering may use direct prefix/path composition, while a general
`LinkGraph` with alternate routes continues to use ordinary PALMA min-plus evaluation.

Do not assume every economic relation is tree-shaped merely because structural SimThing containment is
a tree.

## 9.6 Gu-Yang vertical serviceability recurrence

The simple edge clip:

```math
P_c^*=\min(P_c,C_{p\rightarrow c})S_{p\rightarrow c}
```

is valid only when `P_c` already represents recursively feasible demand and the commitment class
requires immediate deliverability.

A stronger rooted-tree research recurrence for immediately executable flow is:

```math
F_v
=
\min\left(
C_{p\rightarrow v}^{down},
d_v^{local}
+
\sum_{u\in children(v)}F_u
\right),
```

while raw lawful pressure remains:

```math
P_v
=
d_v^{local}
+
\sum_{u\in children(v)}P_u.
```

Then:

```math
U_v^{impairment}=P_v-F_v.
```

This yields two distinct facts:

```text
raw pressure P_v
    how much lawful demand exists

serviceable pressure F_v
    how much can presently be served through the delivery cut
```

**Directionality caveat:** the pressure upsweep is informational/market state and must not be silenced
merely because the downward delivery path is blocked. Archaeology must classify whether Gu-Yang
conductance/capacity is symmetric or directionally authored. If a reverse-direction physical channel
is actually modeled, it may have its own `C^{up}`; no symmetry is inferred from tree topology.

**Disposition:** possible tree lowering of existing Gu-Yang authority, not a new clipping subsystem.
If Gu-Yang already emits the needed branch-local serviceable flow, reuse it directly.

## 9.7 Pressure sums; peak observations do not replace them

Replacing conserved branch pressure with a tropical max:

```math
P_{parent}=\max_c(P_c+w_c)
```

would lose total lawful demand. Double counting is prevented by once-per-edge branch attribution.

A second peak/hotspot observation may still be useful alongside the conserved sum:

```math
\left(
\sum_c P_c,
\max_c q_c
\right).
```

With existing `EXP`/`LN`, a smooth-max research option is:

```math
\operatorname{LSE}_\tau(q)
=
\tau\log\sum_i e^{q_i/\tau}.
```

This remains optional observation/policy, not the conserved pressure lane.

## 9.8 Tree scans before tree contraction

For contiguous subtree range `[start_v,end_v)` and prefix sum `Prefix` over node-local pressure:

```math
P_v=Prefix[end_v]-Prefix[start_v].
```

This suggests span/segmented-scan reuse before Miller-Reif rake/compress.

**Caveat:** nonlinear Gu-Yang clipping does not generally commute with a raw subtree sum:

```math
\min\left(\sum_i P_i,C\right)
\ne
\sum_i \min(P_i,C_i).
```

Depth-bucketed bottom-up serviceability may therefore remain necessary. Tree contraction stays a dated
performance candidate only if measured depth/pathology justifies it.

## 9.9 Delta masking dissolves into source-blind invalidation

Do not mint `DeltaFlowEngine` or `dirty_impedance_mask` as semantic machinery.

Exact-change reuse should consume existing `ChangedLocus` / `DerivedDependencyIndex`:

```text
no relevant source locus changed
    → reuse valid vertical result

relevant source locus changed
    → recompute affected span/region
```

An epsilon rule such as:

```math
|\Delta P|<\varepsilon \Rightarrow \text{skip}
```

is not semantics-preserving unless explicitly admitted as approximation policy.

## 9.10 Prune compute, never information

A fully choked branch may permit settlement/disbursement work to be skipped, but the system must still
preserve raw `U`/pressure, impairment visibility, required lifecycle/refusal state, and lawful
entitlement-first `in_flight` semantics.

A per-branch active mask is a physical optimization candidate, not semantic authority. A shader
`return` does not itself prove dispatch-scale savings.

> **Prune computation, never market information.**

## 9.11 Research references retained for future proof work

The following literature remains background, not current law:

1. **Kelly, Maulloo & Tan** — proportional fairness and shadow prices.
2. **Mo & Walrand** — generalized alpha-fair allocation.
3. **Tassiulas & Ephremides** — constrained queue/backpressure scheduling.
4. **Blelloch** — prefix sums and segmented scan formulations.
5. **Miller & Reif** — parallel tree contraction, deferred pending measured need.
6. **Spielman–Teng** — cyclic graph/Laplacian background only; not needed for strict vertical trees.
7. **QAOA/Gauss-law flow literature** — conceptual only; no alternate solver is chartered.

References must be revalidated against the final landed data model before acceptance rationale uses
them.

---

# 10. Recursive settlement and asynchronous subtrees

An attached tree may execute one feed-forward RF program within generation `N`:

```text
Current N sealed
    ↓
reduce branch-attributed demand T_d upward
    ↓
resolve continuous fields
    ↓
map pressure/policy into existing allocation surfaces
    ↓
disburse continuous allocation downward
    ↓
quantize into exact T_s at each applicable node
    ↓
child consumes exact T_s directly
    ↓
commit T_s / U / lifecycle facts at the generation barrier
```

This is recursive execution, not same-generation iteration.

Prohibited:

```text
settle
    ↓
change field
    ↓
reweight
    ↓
re-clear in the same generation
```

A detached/executing subtree receives its exact product through the stamped seam and subdivides that
local supply without synchronous ancestor RPC. **The seam carries the same constrained-product payload
that the source germ emitted and the destination germ consumes.** Detachment may add transport and
authority envelopes; it may not add economic translation.

The realm/seam/non-foreclosure laws remain those of Phase 14 and are not redefined here.

---

# 11. Surface-reuse and recursive-port census — mandatory before edict

Allowed dispositions are exactly:

```text
REUSE AS-IS
REUSE WITH BINDING
REHOME EXISTING AUTHORITY
MISSING — STOP FOR OWNER/DA RULING
```

## 11.1 Surface-reuse matrix

| Needed meaning | Existing candidate authority | Candidate disposition | Proof required before final law |
|---|---|---|---|
| Upward lawful demand quantity | `IntrinsicFlow`, `Balance`, owner-channel deficit/Need | **REUSE WITH BINDING** | One authoritative recursive quantity/product lane; Draw may authorize it but may not duplicate its quantity authority |
| Branch-attributed subtree pressure | direct-child RF segment over `Need`/`Balance`/`U` | **REUSE WITH BINDING** or **MISSING — STOP** | One aggregate per direct child branch, zero descendant double count, zero host scan |
| Branch pressure → continuous allocation | `Need`/`Balance`/`U` → `AllocatorWeight` or existing share operand | **REUSE WITH BINDING** or **MISSING — STOP** | Greater unresolved branch pressure informs later share when policy/serviceability are equal |
| Continuous child share | `child_share_formula` + `AllocatedFlow` | **REUSE WITH BINDING** | Prove units, sign, bounds, scope, pressure sensitivity, post-Triad ordering, and direct self-consumption at the next allocator level |
| PALMA vertical path cost | existing `D`/impedance + tree adjacency | **REUSE AS-IS** or **REUSE WITH BINDING** | Tree-vertical specialization only where unique-path semantics hold |
| Gu-Yang vertical serviceability | available/realized flux, net/gross/stall/saturation | **REUSE WITH BINDING** | Prove branch-local serviceability and classify directional vs symmetric capacity |
| Additive subtree pressure | logical subtree spans + RF reduction | **REUSE WITH BINDING** | Prove segmented/prefix scan shape before tree-contraction work |
| Dirty vertical state | `ChangedLocus` / `DerivedDependencyIndex` | **REUSE AS-IS** | No duplicate dirty registry; all contributing loci covered |
| Hard market precedence | existing demand priority, order-weight, or lawful OrderBand use | **MISSING — STOP FOR OWNER RULING** | Do not conflate stage order with economic precedence |
| Urgency persistence | `Balance`/Need/STEAD recurrence; authored persistence path | **REUSE / REHOME**, else **MISSING — STOP** | First-order unresolved T_d must self-consume into later T_d without authored bridge |
| Exact discrete settlement | constrained-clear integer/remainder/tie/provenance law | **REHOME EXISTING AUTHORITY** | Implement as type-preserving quantizer over canonical T_s |
| Exact settlement band position | existing RF band layout | **REUSE WITH BINDING** | Settlement is one terminal RF stage, not a peer path |
| Holding accounts | `free/in_flight/occupied/capacity` | **REUSE AS-IS** | Treat as states of canonical T_s relationship |
| Recursive grant subdivision | child-as-granter StemThing-B witness | **REUSE AS-IS** | Parent T_s → child T_s → grandchild T_s with no adapter |
| Settlement-product / supply-intake identity | settlement output vs constrained-supply intake | **REUSE AS-IS** if already one type; otherwise **REHOME** to one type | Any semantic adapter/conversion/projector, including paired newtypes bridged by `From`/`Into`, is **MISSING — STOP** |
| Detached-seam exact-product identity | source-side exact `T_s` payload vs destination-side exact `T_s` intake | **REUSE AS-IS** if identical payload; otherwise **REHOME** to one payload type | Serialization/realm/stamp envelope is lawful; any economic payload translation is **MISSING — STOP** |
| Replay/history | one `IntegrationSchedule` + Phase-14 resident head | **REHOME EXISTING AUTHORITY** | Replay wrappers may differ; simulation payload semantics remain canonical |

## 11.2 Recursive-port census

14.2 must produce a mechanical census for every recursive RF-market germ port:

| recursive port | emitted type/authority | next recursive consumer | allowed result |
|---|---|---|---|
| continuous supply | parent allocation output | child allocator intake | **IDENTICAL vocabulary / direct consumption** |
| exact constrained supply | parent exact settlement output `T_s` | child exact-supply intake `T_s` | **IDENTICAL type** |
| upward demand | child demand output `T_d` | parent reduce-up input `T_d` | **IDENTICAL recursive quantity authority** |
| unresolved demand across time | unresolved `T_d` at N | first-order pressure/Need `T_d` at N+1 | **IDENTICAL recursive quantity authority** |
| detached exact supply | source `T_s` payload before seam envelope | destination `T_s` payload after seam envelope | **IDENTICAL economic payload type** |

For each row, enumerate any function, serialization step, projection, role wrapper, newtype, conversion
trait, or ABI transition between producer and consumer. Then classify:

```text
type alias / zero-conversion role name
    lawful

borrowed/view API over the same underlying T with no construction/copy/projection
    lawful

serialization / realm / stamp envelope preserving payload
    lawful

paired newtypes with From/Into or equivalent conversion
    STOP

semantic conversion between recursive producer and consumer
    STOP
```

A runtime validator that checks two different recursive types “match” does not satisfy the law. The
required seal is type/product identity: **define, don't validate**.

## 11.3 Settled-code recursive-port census and dated debt

The edict must name a one-time census of already-settled RF-market-adjacent code whose dated graduation
predates Germ Self-Consumption. The census is **enumeration before rewrite**, not permission to refactor
on suspicion. At minimum it must inspect:

```text
14.2 resident clearing plan ABI while still probationary/unmerged
resident/host grant disbursement lanes
grant lifecycle and holding-account rows
canonized Draw grammar and its runtime quantity authority
authored unresolved-demand persistence path
seam payloads that carry constrained grants/supply
```

Each finding is classified:

```text
CONFORMING
    already self-consuming / role-only

IN-FLIGHT FIX
    current Phase-14 probation surface; repair before graduation

DATED CONSTITUTIONAL DEBT
    previously graduated surface now inconsistent with the newer law;
    record exact provenance, named future consumer/rung, and retirement condition

NOT IN SCOPE
    one-way oracle/replay/observation/structural boundary
```

A newer law does not retroactively invalidate a dated graduation certificate, but it does prevent a
known mismatch from remaining untracked. Existing constitutional census/debt machinery should carry
these rows; no second debt registry is minted.

## 11.4 Matrix STOP rule

No Phase-14 design may add a new persistent field plane, clearing-owned weight, urgency property,
branch-pressure property, share column, EML formula family, score vocabulary, impairment observer,
market registry, receive facility, disbursement facility, tropical framework, delta engine, tree-
contraction data structure, clearing service, or germ-output→germ-input economic adapter until the
corresponding row proves the function genuinely missing and Owner/DA admits it. For recursive germ
self-consumption, an adapter is not an admissible solution.

---

# 12. Demand-vocabulary unification — one recursive quantity authority

The current tree contains at least two apparent demand vocabularies:

```text
RF Need / IntrinsicFlow / Balance / deficit
and
StemThing-B Draw-authorized runtime claim
```

Germ Self-Consumption makes their relationship non-cosmetic, but it does **not** automatically prove
that `Draw` must be the same Rust type as `Need`.

Two lawful shapes remain:

1. **Draw lowers into the one authoritative recursive demand product `T_d`.** Draw is admission-time
   syntax; the runtime quantity authority is `T_d`.
2. **Draw remains authorization/capability metadata over `T_d`.** It may seal offering, lifecycle,
   scope, and quantity envelope, while actual demanded quantity remains solely in `T_d`.

Unlawful:

```text
Draw.quantity = 7  (authoritative)
        ↓ conversion
Need.quantity = 7  (also authoritative)
```

or any arrangement in which Draw claims form a second independent demand universe beside RF pressure.

The matrix must therefore adjudicate Draw versus `Need`/`IntrinsicFlow` into **one recursive quantity
and product authority**, not blindly collapse unlike concepts merely because both mention quantity.

---

# 13. Phase-14 integration proposal

## 13.1 14.2 `RESIDENT-CLEARING-PLAN-0`

Keep all standing R1–R5 remand obligations unchanged.

Add the surface-reuse matrix **and recursive-port census** and require:

- exact RF band slot for settlement;
- proof whether `AllocatedFlow` is the continuous share and directly self-consumes at the next level;
- proof whether Draw quantity lowers into or merely authorizes one ordinary demand product;
- complete leaf → branch → parent allocation trace;
- proof whether branch pressure informs `AllocatorWeight` or another existing share operand;
- proof whether first-order unresolved demand persists directly or currently relies on an authored bridge;
- classification of tree-vertical PALMA and Gu-Yang bindings;
- settlement-product / supply-intake type identity under Germ Self-Consumption;
- compile-time proof that role labels are aliases/conversion-free views, not paired newtypes bridged by `From`/`Into`;
- detached-seam `T_s` payload identity across lawful realm/stamp/serialization wrappers;
- plan ABI census proving no separate `GrantRow → ChildSupplyRow` semantic transition is being frozen;
- settled-code recursive-port census with in-flight fixes or dated constitutional-debt rows for any discovered mismatch;
- explicit statement that no new math framework is added merely because a research analogy exists;
- `MISSING` rows route to Owner/DA STOP before graduation.

The current no-production-consumer posture remains lawful **probation scaffolding only**. Its expiry is
named now: at resident cutover, the first production consumer of resident clearing is resident clearing
one level down the same StemThing tree.

## 13.2 14.3 `RESIDENT-CLEARING-SCORE-AND-BANDS-0`

Reframe around archaeology. If continuous allocation is already `AllocatedFlow`, bind and prove it
rather than building a clearing-owned score layer.

The resident continuous-market witness must include:

```text
level n AllocatedFlow output
        ↓ direct recursive use
same RF allocator operation at level n+1
```

with no materialized intermediary economic representation.

The ordering audit must distinguish:

```text
RF execution-stage order
hard economic precedence
continuous pressure-informed share
exact deterministic residue order
```

Do not feed arbitrary continuous field values into exact score-bit precedence bands.

## 13.3 14.4 `RESIDENT-CLEARING-APPORTIONMENT-0`

Implement only the exact residue over the proved continuous allocation, and emit the canonical exact
constrained product directly:

```text
exact supply product T_s
continuous pressure-informed allocation/share
lawful request/serviceability caps
        ↓
exact quantization
        ↓
canonical child T_s
        +
exact unresolved demand T_d / U
```

No:

```text
GrantRow
    ↓ realization/conversion adapter
ChildSupplyRow
```

unless both names are type aliases or zero-conversion views over one canonical resident product ABI.
Distinct newtypes bridged by `From`/`Into`, constructors, copies, projections, or conversion kernels do
**not** satisfy this exception.

Preserve wide-integer overflow/refusal semantics and physical-order invariance.

## 13.4 14.5 `RESIDENT-CLEARING-PARITY-0`

The canonical germ witness is now at least a 3-edge self-consumption chain:

```text
root
  ↓ T_s(8)
child
  ↓ T_s(6)
grandchild
  ↓ T_s(4)
great-grandchild / ordinary sink
```

At every recursive edge, the emitted exact type is the receiving exact type. The witness must also
exercise upward demand recursion and temporal unresolved-demand self-consumption.

Add two planted adapter mutants:

```text
TYPE-CONVERSION MUTANT
T_s
 ↓ fake conversion / From / Into
T_s'

SEAM-TRANSLATION MUTANT
Stamped<RealmQualified<T_s>>
 ↓ transport integration changes economic payload type
Stamped<RealmQualified<T_s'>>
```

Both mutants must fail at compile/admission/type construction level, not through a runtime equality
scan. The first should specifically use the project's compile-fail pattern—e.g. make the forbidden
conversion bound or constructor unsatisfiable—so a ceremonially type-safe newtype bridge cannot pass.
This is the canonical **define-don't-validate** falsifier for Germ Self-Consumption.

Retain the asymmetric-pressure witness:

```text
root
 ├─ child A
 │   └─ highly constrained grandchildren
 └─ child B
     └─ lightly constrained grandchildren
```

and prove:

1. A's descendants create greater unresolved lawful pressure.
2. That pressure reduces once into A's branch aggregate.
3. A receives a larger continuous share in a later generation under the adjudicated neutral law.
4. Exact settlement follows that share.
5. A's exact `T_s` becomes A's supply with no semantic adapter.
6. A subdivides the same `T_s` vocabulary toward its own children.
7. Unresolved `T_d` re-enters later demand directly.
8. No descendant row is double-counted.
9. No physical row, upload order, claim arrival, CPU table, or adapter determines the result.

Also retain:

```text
adjacent/open versus distant/open route
adjacent/saturated route
fully impaired lawful demand
high-stakes distant demand
clip-induced serviceability switching
authored hard-precedence override
immediate-flow grant
entitlement entering in_flight before realization
```

## 13.5 14.6 `RESIDENT-CLEARING-CUTOVER-0`

The production census must show no host-built weight/urgency table, CPU PALMA query, CPU Gu-Yang query,
CPU descendant-pressure scan, clearing-owned field cache, private flux/urgency solver, duplicate market
feedback path, duplicate settlement/disbursement path, germ output→input economic adapter, seam economic
payload translator, paired role-newtype conversion bridge, or synchronous host schedule append before
N+1.

The resident schedule segment remains the live head of the one schedule. CPU execution remains explicit
vendorized oracle posture, never automatic fallback.

---

# 14. Candidate binding laws

These are workshop integration candidates except where an external Board mint is explicitly named.
DA/Owner review may narrow wording, move a clause to an existing canonical law, or reject a candidate
without invalidating unrelated sections.

## 14.1 RF Market Mirror Law

> Child lawful need, deficit, and pressure reduce upward through the ordinary RF/Field-Triad cycle;
> continuous allocation disburses downward through the ordinary RF allocation planes. The two are
> the receiving and disbursing strokes of one intrinsic StemThing market filter.

## 14.2 Native Pressure-to-Allocation Law

> Every StemThing exposes its own and its descendants' unresolved lawful pressure through the
> branch-attributed RF upsweep. At the governing parent, that pressure natively informs the existing
> continuous allocation weight or share for that direct child branch in a later generation.
> Authored policy may deform, cap, decay, reverse, or deliberately occlude the pressure; it does not
> create the first-order feedback.

## 14.3 Branch Attribution and No-Double-Count Law

> Pressure is reduced once per tree edge. A parent consumes one aggregate per direct child branch and
> may not count both the branch aggregate and the same descendants again.

## 14.4 Terminal Exact Settlement Law

> Exact constrained settlement is the terminal continuous-to-discrete stage of the RF disbursement
> cycle and is a type-preserving quantizer of the canonical constrained product. It is not a peer
> clearing facility, manager, scheduler, registry, market engine, or product translator.

## 14.5 Receive-Not-Recompute-or-Translate Law

> The settlement stage receives already-resolved resident RF/Triad outputs. It may not privately
> reduce STEAD, relax PALMA, solve Gu-Yang, author urgency, query host state, scan descendants, or
> materialize/translate a clearing-owned pressure, field, weight, share, grant, or supply ontology.

## 14.6 Recursive Grant-to-Supply Law

> A settled exact constrained product emitted to a child is the child's exact constrained supply for
> its own descendants. “Grant” and “supply” are at most aliases or conversion-free role views over the
> same recursive product; paired newtypes requiring conversion do not satisfy the law.

## 14.7 Continuous/Discrete Authority Law

> The Field Triad owns continuous value, route impedance, realizable conservative flow, and the
> pressure-informed allocation surface. Exact settlement owns only exact quantity conservation,
> numerical quantization/residue, commitment-state transition, provenance, and replay.

## 14.8 Gu-Yang Authority Law

> Gu-Yang remains capacity and realization authority. No urgency, weight, policy, hard-precedence
> class, or settlement residue may manufacture flow the conservative field forbids.

## 14.9 Hard-Precedence/Soft-Share Law

> Hard precedence is explicit admitted policy. Continuous RF/Triad allocation governs soft sharing
> within a precedence class and is natively informed by branch pressure. Arbitrary floating-point
> field differences may not silently become strict exact-score winner-take-all precedence.

## 14.10 Vertical Specialization Law

> Unique-path tree relations may lower PALMA to direct path composition and Gu-Yang to an admitted
> capacity-bounded tree recurrence without changing their semantic authorities. General alternate-
> route LinkGraphs continue to use ordinary field evaluation. A topology specialization is a physical
> lowering, never a second market law.

## 14.11 Exact Invalidation Law

> Vertical market results reuse the existing source-blind invalidation substrate. Exact unchanged
> state may skip recomputation. Nonzero epsilon/hysteresis is semantic policy and is never introduced
> as an invisible optimization.

## 14.12 Market Transparency Law

> Demand, urgency, impairment cause, saturation, impedance, provisional allocation, settlement, and
> unmet quantity are visible to the governing scope as born state. Occlusion is an explicit recorded
> policy, never substrate silence.

## 14.13 Generation-Pacing Law

> Sealed Current state at generation `N` informs continuous allocation and exact settlement into
> later state. No same-generation field mutation, reweight, retry, or re-clear loop is admitted.

## 14.14 Native Persistence / Demand Self-Consumption Law

> First-order unresolved lawful demand emitted by the RF market at generation `N` must re-enter the
> same recursive demand/pressure authority in later state through the ordinary bounded recurrence.
> Authored EML may deform decay, escalation, or secondary consequences; it may not be the adapter that
> recreates the germ's baseline demand product.

## 14.15 Germ Self-Consumption Law — Board-minted

> A recursive StemCell germ facility is necessarily its own consumer. By the germ's fractal nature,
> its emission at one tree level is its intake at the next; therefore every germ facility is
> input/output symmetrical in its constrained-product vocabulary: the settlement/emission product
> type is identical to the supply/intake type — one type, at most two role names. Any adapter,
> conversion, or projection between a germ's output port and its own input port one level down is a
> STOP-grade defect. Scope: facilities every StemThing carries fractally; one-way non-germ doors
> (oracles, replay/persistence, observation egress) have external consumers by design and are out of
> scope. Enforcement is the type identity itself (define-don't-validate), never a guard scan.

Normative authority for 14.15 remains Board mint
[`5483829845`](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5483829845) until DA/Owner
canonization.

## 14.16 Recursive Product-Algebra Caveat

> Germ Self-Consumption closes each recursive product port under itself; it does not by itself prove
> that demand, continuous allocation, exact supply, placement geometry, replay, or observation must
> all share one universal type. Broader type collapse requires independent semantic proof and DA/Owner
> review. The specific signed-RF `T_s == T_d` possibility is therefore a candidate to prove or falsify,
> not an implied consequence of 14.15.

---

# 15. Falsifiers and remand conditions

The draft is falsified or remanded if implementation:

1. creates a peer clearinghouse/market/disbursement manager;
2. adds persistent per-claim/per-branch pressure/share/urgency state before proving existing surfaces insufficient;
3. runs private PALMA, Gu-Yang, or STEAD work in settlement;
4. requires host preweight, route, congestion, descendant-scan, or urgency upload;
5. leaves descendant pressure visible but unable to influence later continuous share;
6. uses continuous float differences as hard exact-score precedence absent explicit policy;
7. lets atomic/order/physical placement decide grants;
8. cannot trace leaf shortfall → branch pressure → parent share → exact T_s → child supply;
9. double-counts branch aggregate and descendants;
10. creates a second authoritative demand quantity universe beside recursive RF demand;
11. conflates `U` with CostBand `R`;
12. conflates impairment, contention shortfall, and delivery shortfall;
13. exceeds Gu-Yang serviceability for immediate-flow grants;
14. forbids lawful entitlement-first `in_flight` semantics;
15. hides scarcity state without authored policy;
16. builds same-generation reweight/re-clear convergence;
17. requires synchronous ancestor RPC for unrelated subtree generations;
18. creates a second history or permits state commit without schedule rows;
19. uses root-only or leaf-only market implementations instead of degenerate bindings of the interior germ;
20. preserves CPU collection/type shape merely because it is the oracle;
21. replaces additive lawful pressure with max/tropical pressure;
22. adds Laplacian/electrical solver machinery for a strict unique-path vertical tree;
23. adds a separate delta/dirty subsystem beside `ChangedLocus`;
24. claims epsilon-based skipping as semantics-preserving without explicit approximation ruling;
25. prunes raw U/pressure or impairment visibility merely because a branch is currently choked;
26. inserts any semantic adapter/conversion/projection between recursive exact output `T_s` and next-level exact intake `T_s`;
27. routes first-order unresolved demand through an authored persistence/overlay bridge before it can exist again as recursive demand;
28. uses separate `GrantRow` and `ChildSupplyRow` semantic ABIs requiring conversion rather than aliases or truly conversion-free views over one canonical product;
29. validates at runtime that two recursive product types “match” instead of making the mismatch unrepresentable;
30. silently assumes Draw and Need are one type without proving that Draw is not merely authorization metadata;
31. silently collapses demand and supply into one universal type without independent semantic proof;
32. uses paired role newtypes connected by `From`, `Into`, constructors, copies, or projection kernels to simulate self-consumption while preserving two recursive product types;
33. changes the economic payload type when exact constrained supply crosses a detached subtree seam, even if serialization/realm/stamp wrappers are otherwise lawful;
34. discovers a settled-code recursive-port mismatch and leaves it neither fixed in-flight nor recorded as dated constitutional debt with an owner and retirement condition.

---

# 16. Open evidence, Owner rulings, and performance questions

1. Is `AllocatedFlow` the complete continuous settlement input and directly self-consuming at the next allocator level?
2. Which existing lane owns branch pressure, and which binding maps it into allocation?
3. Which existing surface owns hard precedence?
4. Does unresolved lawful demand persist/accumulate directly, or does current first-order behavior still depend on the authored persistence path?
5. Where exactly does settlement land in the existing RF band layout?
6. What is the exact continuous-to-integer contract and canonical `T_s` schema?
7. Can the CPU oracle referee canonical `T_s` semantics without imposing its host-oriented type split on production?
8. For tree-vertical markets, can PALMA path impedance be supplied by prefix/path composition from existing tree metadata with no extra persistent plane?
9. Does Gu-Yang already expose branch-local serviceable flow, or must a tree-capacity recurrence be bound as a lowering of existing authority?
10. Do contiguous subtree spans permit one segmented/prefix scan for all additive branch pressures at target cardinality?
11. At measured tree depths, is a depth-bucketed Gu-Yang serviceability recurrence faster/simpler than tree contraction?
12. What fraction of vertical impedance/pressure work can `ChangedLocus` invalidation actually elide at realistic policy/churn rates?
13. Does active-segment dispatch pruning outperform dense resident execution after compaction/indirect-dispatch overhead?
14. Does the proportional-fair interpretation match the exact semantics of the existing `child_share_formula`, including guards, caps, and sign behavior?
15. Would an alpha-fair or impedance-penalized EML policy improve domain behavior without instability or replay drift?
16. **OWNER RULING — neutral pressure-to-weight transform:** what is the neutral `w_i(P_i)` when no policy is authored? Native feedback requires a default and therefore defines innate germ behavior. This is not archaeology.
17. Is vertical Gu-Yang capacity/conductance symmetric or directionally authored per edge/resource class?
18. What existing exact type(s) currently stand on each side of settlement output and child supply intake, and what conversions exist between them?
19. Does `AllocatedFlow` pass from child receipt to child allocator use without a copy/project/retype into separate “available supply” state?
20. Can upward demand and unresolved-next-generation demand be proven to share one recursive quantity/product authority `T_d`?
21. **DA/OWNER REVIEW — signed-RF unification candidate:** does the landed signed RF/Gu-Yang quantity substrate prove that `T_s` and `T_d` are one conserved product with opposite signs/roles, or are they intentionally distinct closed product algebras? Evidence must come from semantics and live consumers; elegance alone is insufficient.
22. Which wrappers around `T_s` are pure authority/transport envelopes and which currently alter economic payload semantics?
23. Does the 14.2 resident plan ABI risk freezing separate `ClaimRow`, `SupplyRow`, `GrantRow`, and `ChildSupplyRow` semantic ontologies before the recursive-port census resolves them?
24. Which previously graduated RF-market-adjacent surfaces fail Germ Self-Consumption today, and for each is the lawful disposition in-flight fix, dated constitutional debt, or not-in-scope one-way boundary?

---

# 17. Review checklist

A reviewer should be able to answer **yes** to all of the following before promotion:

- one receive/resolve/settle/disburse RF market cycle, not a clearing subsystem;
- branch pressure is preserved per direct child and influences later allocation;
- no descendant double counting;
- `AllocatedFlow` is reused or disproven before a replacement is named;
- continuous RF output self-consumes directly at the next allocator level;
- hard precedence stays distinct from continuous share;
- exact settlement is a type-preserving quantizer, not a product translator;
- exact parent output `T_s` is identical to child supply input `T_s`;
- grant/supply role names are aliases or truly conversion-free views, never paired newtypes bridged by conversion;
- first-order unresolved demand self-consumes into later recursive demand without authored bridging;
- Draw and Need have one quantity authority even if Draw remains authorization metadata;
- root and leaf are degenerate bindings of the interior germ, not separate market implementations;
- Gu-Yang and PALMA are consumed from born state rather than recomputed in settlement;
- tree-vertical specializations remain lowerings of PALMA/Gu-Yang, not new frameworks;
- pressure sum remains conserved; peak/max observations do not replace it;
- exact-change masking reuses source-blind invalidation;
- U, R, impairment, contention shortfall, and delivery shortfall remain distinct;
- market transparency is default and occlusion authored;
- there is no same-generation re-clear loop;
- async subtree, realm, schedule, and no-CPU-fallback laws remain intact;
- detached seams preserve the identical `T_s` economic payload under lawful wrappers;
- any newly exposed settled-code mismatch is scheduled as in-flight repair or dated constitutional debt;
- the neutral case reproduces the frozen clearing oracle;
- the implementation deletes more vocabulary than it adds;
- research math remains advisory until DA/Owner review and measured proof promote it.

---

# 18. Relationship to CausalBand and the ML corpus

CausalBand may consume the RF market core's born state:

```text
branch-attributed demand / pressure
PALMA impedance
Gu-Yang flow / saturation
continuous allocation
exact T_s / U / in_flight
market-policy deformation
```

The native pressure-to-allocation loop is especially valuable to the corpus because the stored atlas
can show where shortfall originated, how it propagated upward, how allocation shifted later, which
exact products followed, and what pressure remained.

This does not give the atlas or a learned model runtime settlement authority. The RF market core remains
simulation truth; atlas and replay are downstream observation/training artifacts and are explicitly
outside recursive self-consumption scope.

---

# 19. Proposed final synthesis

> **Every StemThing is an RF market germ. Descendant need, deficit, and unresolved pressure reduce
> upward with direct-child branch attribution. At each governing parent, that pressure natively
> informs later continuous allocation through the existing RF/Field-Triad surfaces. Continuous flow
> disburses downward; exact settlement type-preservingly quantizes that flow into the same exact
> constrained product the child consumes as its own supply; the child then repeats the identical germ
> with its descendants. First-order unresolved demand likewise re-enters the same recursive demand
> authority in later state. Receiving, market guidance, settlement, and disbursement are not peer
> facilities but stages of one self-consuming recursive market operator.**

The research interpretation remains a non-normative scientific gloss:

```text
backpressure
    native urgency signal

PALMA
    path cost / potential

Gu-Yang
    feasible capacity envelope

existing proportional-fair-shaped RF disbursement
    smooth continuous allocation

exact settlement
    type-preserving integer quantization + provenance + replay
```

The Germ Self-Consumption seal is:

```text
continuous output at level n
    → direct continuous intake at level n+1

exact T_s at level n
    ==
exact T_s intake at level n+1

upward demand T_d from child
    ==
parent reduce-up input T_d

unresolved T_d at N
    ==
first-order T_d pressure at N+1

seam-wrapped T_s payload at source
    ==
seam-unwrapped T_s payload at destination
```

That is the Phase-14 design target this workshop draft submits for DA/Owner review.