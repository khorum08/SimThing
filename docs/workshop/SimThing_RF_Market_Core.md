# SimThing RF Market Core
## Receive → resolve → settle → disburse, recursively

> **Status: WORKSHOP DRAFT / NON-NORMATIVE / REVIEW CANDIDATE.**
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
> - Canonical law remains
>   [`../simthing_core_design.md`](../simthing_core_design.md),
>   [`../stead_stemthing_unification.md`](../stead_stemthing_unification.md), and the live
>   0.0.8.7 workplan. Where this workshop draft conflicts with those authorities, STOP and escalate.
>
> **Drafting method:** dissolve before inventing. First locate the required value as an already-born
> RF/Field-Triad surface. Then locate the required function as an already-admitted RF/EML operation.
> Only after both fail may Phase 14 name a new runtime surface.
>
> **Latest amendment — branch-pressure mirror insight.** The same parent that disburses a finite
> constrained resource among `N` children necessarily receives the market pressure of those child
> branches through the upward RF stroke. That pressure cannot be merely visible and economically
> inert: each child branch's unresolved lawful pressure must natively inform the parent's later
> continuous allocation toward that branch. Exact settlement consumes that allocation; it does not
> privately calculate urgency. This amendment closes the last load-bearing feedback edge of the
> recursive RF market cycle.

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
          exact discrete settlement
                    ↓
      grant / U / in_flight / occupied
                    ↓
 settled grant becomes the child's supply
                    ↓
       the same market germ recurses
```

The upward stroke receives market pressure. The Field Triad resolves value, impedance, and
realizability. The downward stroke disburses the continuously resolved flow. Exact settlement is the
terminal continuous-to-discrete stage of that same program.

The historical defect is therefore narrower than “receiving was missing” or “clearing was missing”:

> **Continuous receiving and continuous disbursement already existed as two strokes of the RF
> cycle. Exact constrained settlement was implemented beside that cycle rather than sealed as its
> intrinsic terminal stage, and the native feedback from unresolved branch pressure into later
> allocation had not yet been proven as one closed path.**

The desired closure is not a smarter clearinghouse. It is recognition and sealing of the one RF
market filter already distributed through the germ.

## 0.1 The insight that exposed the missing edge

The design-session question was simple:

> If a parent already knows how to disburse a finite constrained resource among `N` children, is the
> receiving side not the mirror of that same operation?

The answer is yes. A parent cannot lawfully disburse under the RF model without receiving, directly or
recursively, the demand and shortfall condition of each child branch. The parent already has or should
have:

```text
one direct-child branch segment
    carrying that child's own and descendant need

one continuous allocation result
    returned toward that branch

one exact settlement
    turning the result into possession
```

That mirror revealed the remaining hinge:

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

The implementation remains archaeology-first: existing `Need`, `Balance`, `U`, `AllocatorWeight`,
weight-sum, and `AllocatedFlow` surfaces must be reused or rebound where they already carry this law.
But the required semantic outcome is no longer optional:

> **Phase 14 may not graduate with descendant pressure visible but economically inert.**

## 0.2 Candidate identity sentence

> **The GPU-resident RF market endpoint is the terminal exact-settlement stage of the Field Triad’s
> ordinary reduce/disburse cycle. Every StemThing intrinsically receives branch-attributed
> descendant market pressure, lets that pressure inform the later continuous allocation to each
> child branch, disburses Triad-resolved constrained flow, and settles that flow into exact grants
> for its children. Every child carries the same germ, so a settled grant becomes that child’s
> lawful constrained supply for its descendants. The endpoint does not independently value, route,
> model, or age claims.**

## 0.3 The intended deletion

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

This is “receiving” in the market sense. It is not receipt of possession; it is receipt of legible
downstream demand and constraint pressure.

The pressure must remain **branch-attributed** at each allocation boundary. A single parent-wide total
such as “subtree unmet demand = 500” is insufficient to determine which direct child branch should
receive more of a scarce resource.

For parent `p` and direct child `c`, the semantic equivalent of the following must be available without
scanning all descendants at allocation time:

\[
P_{p\rightarrow c,N}
=
\text{unresolved lawful pressure represented by child }c\text{ and its subtree at }N
\]

This does not authorize a new `BranchPressure` column. It requires the existing RF upsweep to preserve
enough child-segment attribution for the parent's continuous allocation pass.

The parent is not notified by a side channel. The market state is already resident state required by
the RF program itself.

## 1.2 Center — continuous field resolution

The RF/Field-Triad center supplies the continuous market facts:

| authority | market meaning |
|---|---|
| **STEAD** | lawful demand pressure, stakes, unresolved pressure, and the resident value field |
| **PALMA** | reachability and accumulated impedance to the applicable clearing/disbursement home |
| **Gu-Yang** | signed conservative throughput, available/realized flow, saturation, stall, and blockage |
| **EML** | admitted numerical coupling among already-resident operands; never a second market service |

The generic field engine is already one map/fold/post execution IR; algebra remains admitted EML data,
not a runtime field-kind branch. See
[`../../crates/simthing-kernel/src/field_sweep.rs`](../../crates/simthing-kernel/src/field_sweep.rs).

## 1.3 Native pressure-to-allocation feedback

The mirror closes only when unresolved branch pressure influences the later continuous allocation to
that branch.

Conceptually:

\[
W_{p\rightarrow c,N+1}
=
G\!\left(
P_{p\rightarrow c,N+1},
Policy_{p\rightarrow c,N+1},
Triad_{p\rightarrow c,N+1}
\right)
\]

where `W` is the already-existing `AllocatorWeight` or a proved equivalent—not a new clearing-owned
weight.

The resulting continuous child allocation may retain the existing guarded share form:

\[
X_{p\rightarrow c}
=
S_p
\frac{W_{p\rightarrow c}}
{\sum_j W_{p\rightarrow j}}
\]

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

The feedback is generation-paced. Shortfall at `N` informs a later Current plane and allocation cycle;
it does not trigger a same-generation reweight/re-clear loop.

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

The standing archaeology further identifies existing weight-sum propagation, a guarded
`child_share_formula`, and an `AllocatedFlow` downsweep as the presumptive continuous market-share
path. These surfaces must be verified at their live consumers before any replacement or new name is
admitted.

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

## 1.5 Edge — settle exact possession

Continuous flow is not yet exact possession. The terminal stage must convert the already-resolved
continuous allocation into:

```text
exact integer grant
unresolved quantity U
free / in_flight / occupied deltas
unforgeable relationship provenance
one recorded schedule/replay fact set
```

The current CPU constrained-clear oracle supplies settled laws for exact grouping, integer
apportionment, remainder order, tie rotation, typed failure, and grant construction. See
[`../../crates/simthing-spec/src/spec/constrained_clearing.rs`](../../crates/simthing-spec/src/spec/constrained_clearing.rs).

Its host collections and score-loop shape are an oracle implementation, not automatically the
permanent resident germ shape.

## 1.6 Re-enter — grant becomes child supply

The defining recursive identity is:

```text
parent's exact grant to child
              =
child's lawful constrained supply for its own descendants
```

An interior node is simultaneously:

```text
grantee of its parent
and
granter to its children
```

The WorldState/Session root is the degenerate case with no upstream granter. A leaf is the degenerate
case with no descendants to disburse to. Neither requires a different market implementation.

---

# 2. One germ at every depth

For a StemThing node `v`, the market germ can be described abstractly as:

```text
inputs from descendants:
    branch-attributed lawful need / deficit / pressure

inputs from parent or local production:
    constrained supply

inputs from the field organ:
    continuous value, impedance, realizability, and allocation state

continuous act:
    map branch pressure and policy into the existing allocation/share surface

terminal act:
    exact settlement of the already-resolved continuous distribution

outputs to descendants:
    exact grants / U / commitment state

outputs upward:
    residual need, surplus, impairment, and unresolved pressure
```

A conceptual mapping is:

\[
\mathcal{M}_v(S_v, \{P_c\}, \Phi_v)
\rightarrow
(L_v, \{G_c\}, \{U_c\})
\]

where:

- `S_v` is exact constrained supply available to node `v`;
- `P_c` is lawful branch pressure surfaced by child `c` and its subtree;
- `Φ_v` is the already-resolved RF/Field-Triad state at the governing scope;
- `L_v` is lawful local retention/consumption;
- `G_c` is the exact child grant;
- `U_c` is the unresolved portion.

This is a descriptive model, not authorization for new fields or types.

## 2.1 Root form

```text
intrinsic / accumulated world-level supply
        +
branch-attributed demand from major subtrees
        ↓
root RF/Triad market resolution
        ↓
continuous allocation to major branches
        ↓
exact grants to major subtrees
```

No WorldMarketManager exists.

## 2.2 Interior form

```text
exact parent grant + local production
        ↓
interior-node constrained supply
        +
branch pressure from its children
        ↓
local RF/Triad resolution
        ↓
continuous allocation and exact grants to grandchildren
```

This is the ordinary form and the reason the market capability belongs in the germ.

## 2.3 Leaf form

```text
exact received grant
        ↓
local consumption / holding / action
        ↓
remaining need or shortfall contributes pressure upward
        ↓
no child disbursement because child count = 0
```

A leaf does not need a separate receive facility. The same germ simply has an empty child segment.

---

# 3. RECEIVE, not query and not recompute

The exact-settlement endpoint **receives** a completed continuous allocation from the preceding RF
program stage.

“Receive” is semantic. A GPU pass necessarily loads resident inputs, but it does not query a service,
invoke a route solver, author another market model, or reconstruct descendant urgency.

## 3.1 Prohibited clearing-side work

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
```

## 3.2 Lawful physical inputs

The settlement plan may bind to already-authoritative Current-plane roles, presumptively including:

```text
lawful requested quantity / legal cap
continuous AllocatedFlow or proved equivalent
exact available supply
Gu-Yang serviceability / commitment bound where applicable
claimant logical identity
generation authority
commitment semantics
existing hard-precedence authority
```

The upstream RF allocation plan—not settlement—may bind the existing branch-attributed pressure and
policy inputs needed to produce `AllocatedFlow`.

These are bindings to resident authorities, not copied semantic state.

If physical fusion keeps an upstream post-stage result in registers or kernel-private transients, that
is a lowering of the same admitted RF program. It does not create settlement-owned meaning.

## 3.3 Zero-new-state presumption

The target is stronger than “near-zero memory”:

> **No new persistent per-claim or per-branch state is admitted merely to make settlement
> field-aware or urgency-aware.**

`NativeClearingPotential`, `FieldShare`, `ClaimFieldView`, and `BranchPressure` remain explanatory
terms until the surface-reuse archaeology proves that no existing surface carries the required
meaning.

---

# 4. Existing authorities to consume, not duplicate

## 4.1 RF role anatomy

The canonical compile-time roles already identify the core continuous-market vocabulary:

| existing role | candidate market use |
|---|---|
| `IntrinsicFlow` | resident signed need/production contribution |
| `AllocatedFlow { arena }` | continuous parent-to-child allocation |
| `Balance(BalanceSpec)` | integrated surplus/need ledger |
| `AllocatorWeight { arena }` | continuous child-split input |

These roles compile away before GPU execution; `simthing-sim` must not branch on them as domain
kinds.

## 4.2 FieldSweep

`FieldSweepRegistration` already hosts admitted map/fold/post EML programs over GridOffsets or
LinkGraph adjacency. The settlement endpoint must not acquire a second field executor.

## 4.3 OrderBand and arena stage order

Existing RF and resource-economy machinery already uses `OrderBand`/band-layout concepts to stage
operations. Archaeology must distinguish:

```text
execution stage ordering
    versus
hard economic precedence among lawful claims
```

No second `OrderBand` vocabulary may be minted. Existing stage order must not be silently redefined as
market preference merely because the names resemble one another.

## 4.4 Continuous allocation surfaces

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

The required archaeology question is not whether this shape is elegant. It is whether the live
production meaning, units, sign, bounds, scope, ordering, and branch attribution are sufficient for
constrained exact settlement.

## 4.5 Native branch-pressure feedback surface

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

A new urgency or branch-pressure lane is forbidden until that classification is complete.

## 4.6 StemThing-B market vocabulary

The current market spec already separates:

```text
unit_cost
    → CostBand price

default_clearing_weight
    → inherited clearing-weight lane

Draw
    → lawful claim authorization, never a grant
```

See
[`../../crates/simthing-spec/src/spec/flow_market.rs`](../../crates/simthing-spec/src/spec/flow_market.rs).

The new RF-market-core law may rehome where first-order continuous valuation occurs, but it must not
collapse unit price, hard precedence, branch pressure, continuous share, and exact grant into one
scalar.

## 4.7 Exact clearing oracle

The current CPU oracle preserves load-bearing exact laws:

```text
scope segregation
finite/non-negative EML score validation
canonical signed zero
f32 total order and exact score-bit bands
checked wide-integer requested totals/products
integer base allocation
exact fractional remainder ordering
logical-identity secondary order
granter+generation exact-tie rotation
canonical grant ordering
typed failure / no partial output
grant and U construction
```

Those laws remain proof cargo. Its score-band organization is not automatically the permanent
continuous-market architecture.

## 4.8 Grant lifecycle and holding accounts

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

This is the presumptive existing authority for distinguishing immediate executable flow from an
entitlement that remains in flight before realization.

## 4.9 Generation and async seams

Per-tree generation authority, no-wait stamped integration, and one recorded schedule already permit
independently advancing subtrees. See
[`../../crates/simthing-core/src/generation_stamp.rs`](../../crates/simthing-core/src/generation_stamp.rs).

The RF market core must remain per executing tree and must not assume one global clock, host, process,
device, schedule, registry, or raw-ID namespace.

---

# 5. Continuous market versus exact settlement

## 5.1 The Field Triad owns continuous economic resolution

The intended continuous result is claimant-local allocated flow after lawful need, branch pressure,
policy, route impedance, competition, and channel capacity have already interacted.

For claim `i`, explanatory terms are:

- `r_i`: lawful requested quantity;
- `a_i`: current serviceable quantity under route/channel constraints;
- `p_i`: branch-attributed unresolved lawful pressure;
- `x_i`: continuous provisional allocation emitted by the RF/Triad disbursement program;
- `g_i`: exact integer grant settled at the endpoint.

The desired relation is:

\[
0 \le x_i \le a_i \le r_i
\]

with:

\[
\sum_i x_i \le S
\]

for exact supply `S` at the clearing/disbursement home.

The continuous allocation should be monotone in native unresolved pressure when policy and
serviceability are held equal:

\[
p_i' > p_i
\quad\Longrightarrow\quad
x_i' \ge x_i
\]

subject to admitted caps, bounded feedback, competing pressure, and Gu-Yang realizability.

These symbols describe semantics. They do not authorize new columns.

## 5.2 The exact endpoint owns only the discrete residue

The settlement stage may:

1. validate that the admitted exact settlement envelope is representable;
2. convert the continuous allocation into exact units;
3. preserve exact supply conservation;
4. settle indivisible residue under canonical deterministic law;
5. publish `grant`, `U`, and commitment-state deltas;
6. mint exact relationship provenance;
7. append the one replay/schedule history atomically with state publication.

It may not re-run the continuous market or calculate branch urgency.

## 5.3 The exact conversion remains an open proof item

Archaeology must determine the units and authority of `AllocatedFlow` before selecting the conversion.
The exact endpoint requires:

```text
an exact integer total-settlement envelope
+
a continuous share/allocation vector
+
exact legal/request/serviceability caps
```

The continuous vector does not privately manufacture the exact total.

Phase 14.4 must prove the final conversion, including wide-integer overflow/refusal boundaries and
work-conserving residue where serviceability permits.

---

# 6. Hard precedence is not continuous share

## 6.1 The float-dust score-band trap

The current oracle clears higher exact score bands completely before lower bands and shares only among
claims with identical score bits.

Feeding a continuous field value directly into that score law can create:

```text
0.8731 > 0.8729 > 0.8714
```

and therefore strict winner-take-all precedence from insignificant floating-point differences.

That is not graceful field allocation.

## 6.2 Required semantic separation

The market core needs two distinct meanings:

```text
hard precedence class
    explicit legal/emergency/policy order

continuous allocation
    RF/Triad-resolved soft share informed by branch pressure
    within a precedence class
```

No new runtime names are authorized by this statement.

The hard-precedence authority must be found in existing `OrderBand`, demand priority, or order-weight
surfaces. The continuous allocation must be found in existing `AllocatedFlow` or a proved existing
RF post/fold result.

## 6.3 Neutral-case seal

The generalized law must contain the current ratified law as its neutral case:

```text
no special hard precedence
        ↓
one neutral precedence class

no full-Triad continuous allocation binding
        ↓
continuous share basis = lawful requested quantity
        ↓
exact settlement = current proportional-by-request law
        ↓
largest remainder + generation-rotated exact ties
```

The full current oracle corpus remains the neutral-case seal.

The CPU oracle must eventually receive the same generalized settlement-over-a-share-vector interface so
it remains the referee for field-resolved cases, not merely for the neutral case.

---

# 7. Three shortfall stages must remain distinct

## 7.1 Impaired lawful demand — before settlement

Explanatory quantity:

\[
U_i^{impairment} = r_i - a_i
\]

This is lawful demand that cannot currently traverse the admitted route/channel.

Examples include:

```text
PALMA unreachable route
blockade
Gu-Yang saturation or zero signed availability
policy overlay reducing conductance
legal hold represented in the ordinary field/gate state
```

This is not yet a blocked grant.

## 7.2 Contention shortfall — at settlement

Explanatory quantity:

\[
U_i^{contention} = a_i - g_i
\]

This is physically serviceable demand that did not receive exact supply under the constrained market.

## 7.3 Delivery shortfall — after entitlement

For exact grant `g_i` and realized delivery `y_i`:

\[
B_i^{delivery} = g_i - y_i
\]

This is the true blocked legal grant when the resource model permits entitlement before realization.

## 7.4 Commitment semantics

The germ must support both semantic classes without a domain branch:

```text
immediately executable flow
    exact grant is capped by current Gu-Yang serviceability

entitlement then deliver
    exact grant may enter in_flight
    Gu-Yang governs later realization
```

These should reuse existing admitted resource/commitment and grant-lifecycle semantics. The terms above
are explanatory until archaeology identifies the exact existing authority.

---

# 8. Urgency, persistence, branch attribution, and market transparency

## 8.1 What is already visible

The governing scope can already receive or derive from born state:

```text
demand and priority
surplus and deficit
PALMA impedance/reach
Gu-Yang availability, flow, saturation, and stall
U and typed refusal disposition
grant commitment and in-flight state
```

A parent does not require a notification service to know that a descendant market is constrained.

The new design-session insight is that visibility alone is not enough. The branch-attributed pressure
must participate in the later continuous allocation to that branch.

## 8.2 Mandatory semantic outcome, archaeology-first implementation

The existing explicit unresolved-demand consequence path uses:

```text
UnresolvedDemandObservation
    ↓
authored EML persistence valuation
    ↓
CostBand funding
    ↓
later OverlayThing consequence
```

Therefore the implementation must still classify the Current→Next behavior of ordinary
`Need`/`Balance`/STEAD lanes:

```text
A. unresolved lawful demand remains resident unchanged
B. unresolved lawful demand accumulates into increased pressure
C. only the authored persistence consequence retains/escalates it
```

But the Phase-14 outcome is mandatory:

> **Every StemThing must expose its own and its descendants’ unresolved lawful pressure through the
> branch-attributed RF upsweep, and at the governing parent that pressure must natively inform the
> existing continuous allocation weight/share for that child branch in a later generation.**

The implementation disposition may be:

```text
REUSE AS-IS
REUSE WITH BINDING
REHOME EXISTING AUTHORITY
```

If none is possible without new law, the matrix result is:

```text
MISSING — STOP FOR OWNER RULING
```

It is not lawful to graduate with pressure visible but economically inert.

## 8.3 Candidate bounded recurrence

If archaeology proves first-order persistence or accumulation missing, the Owner may amend the
baseline with the semantic equivalent of:

\[
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
\]

where `B` is the existing bounded Current→Next recurrence or a rehomed equivalent.

Authored EML may deform decay, saturation, escalation, policy response, or deliberate occlusion. It
does not create first-order persistence or the first-order connection between pressure and allocation.

No unbounded positive recurrence and no per-claim age-counter service is admitted.

## 8.4 No double counting

Pressure is reduced once per tree edge.

A parent consumes one aggregate per direct child branch. It may not count:

```text
the child branch aggregate
+
the same child's descendant rows again
```

The hierarchy therefore behaves as a recursive reduction, not a population-wide weighted scan.

## 8.5 Market transparency law

> **Market transparency is the default. Demand, urgency, impairment cause, saturation, impedance,
> provisional allocation, exact grant, and unmet-demand volume are visible to the governing scope as
> born field, RF, lifecycle, and schedule state. Occlusion is only an explicit authored act—an
> admitted, recorded, auditable policy or perception overlay—never substrate silence.**

This permits fog of war, private ledgers, or sealed bids as authored market design without making
information asymmetry a hidden kernel behavior.

---

# 9. Recursive settlement and asynchronous subtrees

## 9.1 Attached-tree recursion

An attached tree may execute one feed-forward RF program within generation `N`:

```text
Current N sealed
    ↓
reduce branch-attributed need and pressure upward
    ↓
resolve continuous fields
    ↓
map pressure and policy into existing continuous allocation surfaces
    ↓
disburse continuous allocation downward
    ↓
derive exact child settlements at each applicable node
    ↓
commit all grant/U/lifecycle facts at the generation barrier
```

This is recursive execution, not same-generation iteration.

## 9.2 Prohibited loop

```text
settle
    ↓
change field
    ↓
reweight
    ↓
re-clear in the same generation
```

No same-generation receive/originate convergence, retry solver, or market fixed-point loop is admitted.

## 9.3 Detached/executing subtree

A provisioned subtree may become a local executing root under its own:

```text
TreeRealmId
execution incarnation
per-tree GenerationStamp
per-tree resident plan
per-tree IntegrationSchedule
seam attachment to its ancestor/granter
```

Its exact received grant becomes local supply. It subdivides that supply without synchronous ancestor
RPC. Missing future upstream capacity affects only the dependent operation as staleness, unavailable
capacity, `U`, or typed refusal; unrelated local generations may continue.

The realm/seam/non-foreclosure laws remain those of Phase 14 and are not redefined here.

---

# 10. Surface-reuse matrix — mandatory before edict

The matrix is a constitutional archaeology deliverable, not a new registry. Its posture must reuse the
existing census vocabulary: production authority, proof/oracle, preview, deferred, or residue.

Allowed dispositions are exactly:

```text
REUSE AS-IS
REUSE WITH BINDING
REHOME EXISTING AUTHORITY
MISSING — STOP FOR OWNER RULING
```

| Needed meaning | Existing candidate authority | Current evidence/posture | Candidate disposition | Proof required before final law |
|---|---|---|---|---|
| Upward lawful need | `IntrinsicFlow`, `Balance`, owner-channel deficit/Need | Existing RF roles and reduce-up vocabulary | **REUSE WITH BINDING** | Identify one authoritative requested-quantity lane; prove Draw claims lower into it or remain a justified distinct envelope |
| Parent demand visibility | ordinary RF reduce-up by owner/resource/scope | Production-shaped and constitutionally required | **REUSE AS-IS** | Prove every constrained child claim reaches the governing clearing scope without host reconstruction |
| Branch-attributed subtree pressure | direct-child RF segment over `Need`/`Balance`/`U` | Required by mirror law; live wiring unproven | **REUSE WITH BINDING** or **MISSING — STOP** | Prove one aggregate per direct child branch, zero descendant double count, zero host scan |
| Branch pressure → continuous allocation feedback | `Need`/`Balance`/`U` upsweep → `AllocatorWeight` or existing share operand | Load-bearing hinge; current authority unresolved | **REUSE WITH BINDING** or **MISSING — STOP** | Prove greater unresolved branch pressure monotonically informs later continuous share when policy/serviceability are equal, with no new persistent state |
| Surplus/deficit visibility | `Balance`, RF own aggregates, `U` | Existing resident/report surfaces | **REUSE WITH BINDING** | Prove local and upward wiring, not mere representability |
| RF stage ordering | existing `OrderBand` / `ArenaBandLayout` | Existing execution-stage authority | **REUSE WITH BINDING** | Locate exact settlement after continuous disbursement and before residual/integration without minting a scheduler |
| Hard market precedence | existing demand priority, order-weight, or lawful use of existing OrderBand | Meaning not yet adjudicated | **MISSING — STOP FOR OWNER RULING** | Determine which existing authority owns economic precedence; do not conflate with stage order |
| Continuous weight input | `AllocatorWeight` | Existing RF role | **REUSE WITH BINDING** | Prove branch pressure plus Triad/overlay state feeds it without clearing-private projection |
| Weight aggregation | existing weight-sum / propagated-weight-sum surfaces | Archaeology reports existing upsweep | **REUSE AS-IS** | Verify live production consumer and hierarchy depth |
| Continuous child share | `child_share_formula` + `AllocatedFlow` | Strong presumptive existing shape | **REUSE WITH BINDING** | Prove units, sign, request/supply bounds, clearing-home scope, pressure sensitivity, and post-Triad ordering |
| Continuous disbursement recursion | `AllocatedFlow` downsweep | Existing recursive RF design | **REUSE AS-IS** | Prove multi-level attached-tree live consumer, not only a proof fixture |
| PALMA route cost | existing `D`/impedance column | Born field output | **REUSE AS-IS** | Prove the field is targeted to the applicable clearing/disbursement home |
| Gu-Yang realizability | available/realized flux, net/gross/stall/saturation | Born field/comparative outputs | **REUSE WITH BINDING** | Identify claimant-local serviceability/provisional-flow authority and commitment-mode interpretation |
| STEAD stakes/pressure | existing resident pressure/Need fields | Born field output | **REUSE AS-IS** | Prove which sealed Current lane informs branch pressure and continuous allocation |
| Impairment reading | request/reach/flux/stall/U/refusal conjunction | All primitive facts appear to exist | **REUSE WITH BINDING** | Keep as derived reading unless a missing causal distinction is proven |
| Urgency persistence | `Balance`/Need/STEAD recurrence; authored persistence path | Authority unresolved | **REUSE / REHOME**, else **MISSING — STOP** | Classify persistence vs accumulation vs authored-only consequence; pressure may not remain economically inert |
| Exact discrete settlement | current constrained-clear integer/remainder/tie/provenance law | CPU oracle / production host path | **REHOME EXISTING AUTHORITY** | Extract exact settlement over proved continuous shares without preserving host collection shape |
| Exact settlement band position | existing RF band layout | Not yet bound | **REUSE WITH BINDING** | Prove settlement is one terminal RF stage, not a peer execution path |
| Grant lifecycle | `GrantLifecycleFact`, relationship state | Existing core authority | **REUSE AS-IS** | Preserve atomic state/history commit under resident execution |
| Holding accounts | `free/in_flight/occupied/capacity` | Existing conserved grammar | **REUSE AS-IS** | Map immediate-flow vs entitlement-first semantics without new domain enums where existing class data suffices |
| Recursive grant subdivision | child-as-granter StemThing-B witness and grant realization | Graduated law/proof | **REUSE AS-IS** | Bind exact parent grant as exact child supply; no host conversion or second recursion API |
| Grant-to-supply re-entry | grant lanes + ordinary child market supply | Expected but must be mechanically traced | **REUSE WITH BINDING** | Demonstrate parent grant → child resident supply → grandchild allocation end to end |
| Replay/history | one `IntegrationSchedule` and resident schedule-head law | Existing authority; Phase 14 resident cutover pending | **REHOME EXISTING AUTHORITY** | N+1 proceeds before host drain; resident consequences and schedule rows commit atomically |
| Market occlusion | OverlayThing/perception policy | Existing actuation/policy surface | **REUSE AS-IS** | Prove default transparency and recorded authored occlusion |

## 10.1 Matrix STOP rule

No Phase-14 design may add a new:

```text
persistent field plane
clearing-owned weight
urgency property
branch-pressure property
share column
EML formula family
score vocabulary
impairment observer
market registry
receive facility
disbursement facility
clearing service
```

until the corresponding matrix row proves the semantic function missing.

---

# 11. Demand-vocabulary unification

The current tree contains at least two apparent demand vocabularies:

```text
RF Need / IntrinsicFlow / Balance / deficit
and
StemThing-B Draw-authorized runtime claim
```

The market mirror implies one of two lawful outcomes:

1. Draw authorization lowers into the ordinary authoritative RF need lane; or
2. Draw remains a strict authorization envelope while the quantity itself is still read from the
   ordinary RF need lane.

An outcome in which Draw claims form a second independent demand universe beside RF pressure requires an
explicit Owner ruling.

A Draw still grants nothing. It seals which offering, lifecycle, and quantity envelope may lawfully
participate.

The branch-pressure law applies to the authoritative quantity lane, not to a duplicated claim record.

---

# 12. Phase-14 integration proposal

This section is a workshop recommendation. DA owns row wording and sequencing.

## 12.1 14.2 `RESIDENT-CLEARING-PLAN-0`

Keep all standing R1–R5 remand obligations unchanged.

Add the surface-reuse matrix and require:

- census posture and live-consumer evidence for every row;
- the exact RF band slot for settlement;
- proof whether `AllocatedFlow` is the existing continuous share;
- proof whether Draw demand lowers into ordinary Need/Balance;
- a complete leaf → interior branch → parent allocation trace;
- proof whether branch pressure already informs `AllocatorWeight` or another share operand;
- proof whether urgency persistence is native, persistent-only, or authored-only;
- a closed resident binding to reused authorities, not new duplicated planes;
- `MISSING` rows route to Owner/DA STOP before 14.2 graduation.

The 14.2 plan remains generation-independent. Current generation is execution-header authority, not
semantic plan identity.

## 12.2 14.3 `RESIDENT-CLEARING-SCORE-AND-BANDS-0`

Reframe the rung around the archaeology result.

If continuous allocation is already `AllocatedFlow`, 14.3 binds and proves it rather than building a
clearing-owned score layer.

The resident path must prove:

```text
branch-attributed pressure
        ↓ existing bounded RF/Triad interpretation
AllocatorWeight or existing share operand
        ↓
AllocatedFlow
```

with no CPU descendant scan, host urgency table, or clearing-owned pressure cache.

The mandatory ordering audit must distinguish:

```text
RF execution-stage order
hard economic precedence
continuous pressure-informed share
exact deterministic residue order
```

Do not feed arbitrary continuous field values into exact score-bit precedence bands.

Any existing authored clearing weight must be dispositioned as one of:

- an upstream deformation of RF/Triad pressure, conductance, or `AllocatorWeight`;
- an existing hard-precedence policy surface;
- retained neutral-case oracle input only;
- superseded residue scheduled for deletion after equivalence proof.

## 12.3 14.4 `RESIDENT-CLEARING-APPORTIONMENT-0`

Implement only the exact residue over the proved continuous allocation:

```text
exact supply envelope
continuous pressure-informed allocation/share
lawful request/serviceability caps
        ↓
exact base grants
exact fractional residue
canonical deterministic remainder assignment
exact U / holding-account transition
```

Preserve wide-integer overflow/refusal semantics and physical-order invariance.

## 12.4 14.5 `RESIDENT-CLEARING-PARITY-0`

Retain the full positive and negative parity battery.

Add the following recursive witness:

```text
root
 ├─ child A
 │   └─ highly constrained grandchildren
 └─ child B
     └─ lightly constrained grandchildren
```

With equal authored policy and comparable serviceability, prove:

1. A's descendants create greater unresolved lawful pressure.
2. That pressure reduces once into A's branch aggregate at the root.
3. A receives a larger continuous share in a later generation.
4. Exact settlement follows that share.
5. A's exact grant becomes supply for its own children.
6. When A's pressure subsides, its share subsides under the bounded recurrence.
7. No descendant row is double-counted.
8. No physical row, upload order, claim arrival, or CPU table determines the result.

Also retain witnesses for:

```text
adjacent/open route versus distant/open route
adjacent/saturated route
fully impaired lawful demand
high-stakes distant demand
authored hard-precedence override
immediately executable flow grant
entitlement entering in_flight before realization
local Balance/U visibility and reduce-up
```

Prove:

- zero additional field dispatch;
- zero new persistent clearing potential/share/pressure state;
- zero CPU preweight, route lookup, flux lookup, or descendant scan;
- Gu-Yang retains capacity/realization authority;
- impairment, contention shortfall, and delivery shortfall remain distinguishable;
- neutral case reproduces the frozen oracle;
- physical upload/row/workgroup/dispatch order remains irrelevant;
- a settled child grant becomes child supply under the same germ.

## 12.5 14.6 `RESIDENT-CLEARING-CUTOVER-0`

The production census must show that full-Triad market settlement contains no:

```text
host-built weight or urgency table
CPU PALMA query
CPU congestion/Gu-Yang query
CPU descendant-pressure scan
clearing-owned pressure/share/field cache
private flux or urgency solver
scenario-authored per-claim first-order intelligence
second pressure-to-allocation feedback path
second settlement/disbursement path
synchronous host schedule append before N+1
```

The resident schedule segment remains the live head of the one schedule. Replay draining is asynchronous
within admitted capacity; capacity reservation precedes state/schedule commit.

CPU execution remains explicit vendorized oracle posture, never automatic fallback.

---

# 13. Candidate binding laws

These are review candidates, not standing law.

## 13.1 RF Market Mirror Law

> Child lawful need, deficit, and pressure reduce upward through the ordinary RF/Field-Triad cycle;
> continuous allocation disburses downward through the ordinary RF allocation planes. The two are
> the receiving and disbursing strokes of one intrinsic StemThing market filter.

## 13.2 Native Pressure-to-Allocation Law

> Every StemThing exposes its own and its descendants' unresolved lawful pressure through the
> branch-attributed RF upsweep. At the governing parent, that pressure natively informs the existing
> continuous allocation weight or share for that direct child branch in a later generation.
> Authored policy may deform, cap, decay, reverse, or deliberately occlude the pressure; it does not
> create the first-order feedback. Exact settlement consumes the resulting continuous allocation and
> does not privately calculate urgency.

## 13.3 Branch Attribution and No-Double-Count Law

> Pressure is reduced once per tree edge. A parent consumes one aggregate per direct child branch and
> may not count both the branch aggregate and the same descendants again. Allocation work is
> proportional to direct-child segments and admitted RF reductions, never a host population scan.

## 13.4 Terminal Exact Settlement Law

> Exact constrained settlement is the terminal continuous-to-discrete stage of the RF disbursement
> cycle. It is not a peer clearing facility, manager, scheduler, registry, or market engine.

## 13.5 Receive-Not-Recompute Law

> The settlement stage receives already-resolved resident RF/Triad outputs. It may not privately
> reduce STEAD, relax PALMA, solve Gu-Yang, author urgency, query host state, scan descendants, or
> materialize a clearing-owned pressure/field/weight/share representation.

## 13.6 Recursive Grant-to-Supply Law

> A settled exact grant to a child becomes that child's exact constrained supply for its own
> descendants. The same germ applies at root, interior, and leaf nodes without a recursion service or
> domain-specific manager.

## 13.7 Continuous/Discrete Authority Law

> The Field Triad owns continuous value, route impedance, realizable conservative flow, and the
> pressure-informed allocation surface. Exact settlement owns only exact quantity conservation,
> discrete residue, commitment-state transition, provenance, and replay.

## 13.8 Gu-Yang Authority Law

> Gu-Yang remains capacity and realization authority. No urgency, weight, policy, hard-precedence
> class, or settlement residue may manufacture flow the conservative field forbids.

## 13.9 Hard-Precedence/Soft-Share Law

> Hard precedence is explicit admitted policy. Continuous RF/Triad allocation governs soft sharing
> within a precedence class and is natively informed by branch pressure. Arbitrary floating-point
> field differences may not silently become strict exact-score winner-take-all precedence.

## 13.10 Market Transparency Law

> Demand, urgency, impairment cause, saturation, impedance, provisional allocation, settlement, and
> unmet quantity are visible to the governing scope as born state. Occlusion is an explicit recorded
> policy, never substrate silence.

## 13.11 Generation-Pacing Law

> Sealed Current state at generation `N` informs continuous allocation and exact settlement into
> later state. No same-generation field mutation, reweight, retry, or re-clear loop is admitted.

## 13.12 Native Persistence Law — conditional implementation, mandatory outcome

> If archaeology proves first-order unresolved-pressure persistence missing, unresolved lawful
> quantity re-enters next-generation STEAD pressure through the ordinary bounded recurrence. Authored
> EML deforms persistence; it does not create the baseline. Regardless of physical implementation,
> Phase 14 may not graduate with branch pressure visible but disconnected from later allocation.

---

# 14. Falsifiers and remand conditions

The draft is falsified or remanded if implementation does any of the following:

1. Creates a peer clearinghouse/market/disbursement manager beside the StemThing RF program.
2. Adds a persistent per-claim or per-branch potential/share/urgency/pressure plane before proving the
   existing surface insufficient.
3. Runs a private PALMA, Gu-Yang, or STEAD computation in settlement.
4. Requires a host preweight, route lookup, congestion lookup, descendant scan, or urgency upload
   before resident settlement.
5. Leaves descendant pressure visible to a parent but unable to influence the later continuous share.
6. Uses continuous float differences as strict exact score bands without explicit authored hard
   precedence.
7. Lets atomic append, upload order, physical row, workgroup schedule, or dispatch partition decide a
   grant.
8. Cannot trace leaf shortfall → child-branch pressure → parent share → exact child grant → grandchild
   supply through the same germ.
9. Counts both a branch aggregate and the branch's descendant rows.
10. Creates a second demand vocabulary with independent authority beside RF Need/Balance.
11. Conflates `U` with CostBand remainder `R`.
12. Conflates pre-settlement impairment, contention shortfall, and post-grant delivery shortfall.
13. Grants immediately executable flow beyond Gu-Yang serviceability.
14. Forbids lawful entitlement-first `in_flight` commitment merely because current delivery is blocked.
15. Hides demand, impairment, or unmet quantity from the governing scope without an authored policy.
16. Builds same-generation reweight/re-clear convergence.
17. Requires synchronous ancestor RPC for unrelated local subtree generations.
18. Creates a second schedule/history or permits state commit without its exact schedule rows.
19. Uses a root-only or leaf-only special market path instead of the same degenerate germ.
20. Preserves CPU collection shape merely because it is the oracle, rather than extracting the exact
    settlement law.

---

# 15. Open evidence questions

The design has converged; these implementation facts still require measured/source proof.

## 15.1 Is `AllocatedFlow` the complete continuous settlement input?

Prove:

- units match the constrained resource;
- sign semantics match grant semantics;
- values are bounded by legal request and exact supply;
- the scope is the correct clearing/disbursement home;
- the value is produced after the relevant Triad state;
- the attached-tree downsweep reaches every required depth;
- branch-attributed unresolved pressure can influence it monotonically;
- it supports or can bind both immediate-flow and entitlement-first commitment modes.

If yes, do not mint `FieldShare`.

## 15.2 Which existing surface owns branch pressure?

Trace:

```text
leaf Need / Balance / U
    ↓
interior branch aggregate
    ↓
parent direct-child segment
```

Prove one reduction per tree edge, no descendant double count, no host scan, and stable logical
identity across row rebinds.

## 15.3 Which existing surface binds pressure to allocation?

Candidates include:

```text
Need / Balance / STEAD pressure
        ↓ existing EML / governed_by / accumulator relation
AllocatorWeight
        ↓
child_share_formula / AllocatedFlow
```

Classify the live path as production, proof, preview, deferred, or residue. An opt-in `field_urgency`
formula is not automatically the authority merely because it exists.

## 15.4 Which existing surface owns hard precedence?

Candidates:

```text
existing OrderBand execution staging
existing demand priority
existing order-weight policy
```

This is an Owner/DA semantic decision if archaeology does not produce one clear authority.

## 15.5 Does unresolved lawful demand persist or accumulate natively?

Trace the exact Current→Next path for `Need`, `Balance`, STEAD pressure, and `U`.

Do not infer authority from the existence of a dormant or preview-only urgency formula.

## 15.6 Where exactly does settlement land in the existing band layout?

The target is after continuous allocation is complete and before residual/integration publication.
No new scheduler or parallel band vocabulary is permitted.

## 15.7 What is the exact continuous-to-integer contract?

Determine:

- exact integer target total;
- continuous share units;
- serviceability/request caps;
- floor/base rule;
- fractional-residue representation;
- work-conserving remainder condition;
- overflow/refusal behavior;
- commitment-state output.

## 15.8 How does the CPU oracle generalize?

The CPU oracle must remain an exact referee for:

```text
neutral proportional case
pressure-informed continuous-share case
hard-precedence classes
immediate-flow and entitlement-first commitment
all typed failures
```

The oracle may be rehomed or refactored; it must not become the target resident architecture.

---

# 16. Review checklist

A reviewer should be able to answer **yes** to all of the following before promotion:

- Does the document describe one receive/resolve/settle/disburse RF market cycle rather than a
  clearing subsystem?
- Is the mirror insight—the disbursement parent already receives branch market pressure—stated and
  carried into a mandatory pressure-to-allocation law?
- Is every new-seeming value first mapped to an existing authority candidate?
- Is every new-seeming operation first mapped to an existing RF/EML operation candidate?
- Is `AllocatedFlow` treated as presumptive continuous share until disproven?
- Are `OrderBand` stage order and hard economic precedence kept distinct pending audit?
- Is branch attribution preserved one aggregate per direct child?
- Is descendant double counting forbidden?
- Can unresolved branch pressure influence later allocation without host scans or new persistent
  state?
- Does exact settlement remain small and discrete?
- Does a child grant become child supply without a second API?
- Does the same germ apply to root, interior, and leaf?
- Are Gu-Yang capacity and PALMA impedance consumed from born state rather than recomputed?
- Are `U`, `R`, impairment, contention shortfall, and delivery shortfall distinct?
- Is market transparency default and occlusion authored?
- Is urgency authority stated honestly rather than inferred from a dormant formula?
- Is there no same-generation re-clear loop?
- Are async subtree, realm, schedule, and no-CPU-fallback laws preserved?
- Does the neutral case reproduce the frozen clearing oracle?
- Does the design delete more vocabulary than it adds?

---

# 17. Relationship to CausalBand and the ML corpus

CausalBand may consume the RF market core's born state:

```text
branch-attributed demand / pressure
PALMA impedance
Gu-Yang flow / saturation
continuous allocation
exact grant / U / in_flight
market-policy deformation
```

The native pressure-to-allocation loop is especially valuable to the corpus because the stored atlas
can show:

```text
where shortfall originated
how it propagated upward
how allocation shifted later
which exact grants followed
what pressure remained
```

Those channels can become part of the temporal CausalBand Atlas and its replay-grounded ML corpus.

This does not give the atlas or a learned model runtime settlement authority. The RF market core remains
simulation truth; atlas and replay are downstream observation/training artifacts.

---

# 18. Proposed final synthesis

> **Every StemThing is an RF market germ. Descendant need, deficit, and unresolved pressure reduce
> upward with direct-child branch attribution. At each governing parent, that pressure natively
> informs the later continuous allocation to each branch through the existing RF/Field-Triad
> surfaces. Allocated flow disburses downward; exact settlement turns that resolved flow into
> discrete identity-bearing possession; the child then repeats the same cycle with its own
> descendants. Receiving, market guidance, clearing, and disbursement are not peer facilities but
> stages of one recursive market filter. The exact endpoint receives rather than recomputes: it owns
> no route, flux, urgency, pressure, weight, or field model—only exact conservation, integer residue,
> commitment state, provenance, and replay.**

The desired implementation should therefore make the “clearinghouse” disappear into the existing RF
program while preserving the exact settlement oracle as proof and closing the one remaining native
feedback edge:

```text
descendant pressure rises
        ↓
parent continuous allocation shifts
        ↓
exact grants follow
        ↓
children recursively disburse
        ↓
remaining pressure reports the next market state
```

That is the Phase-14 design target this workshop draft submits for review.
