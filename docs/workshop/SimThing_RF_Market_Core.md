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
> **Latest amendment — mathematical and performance research adjudication.** The vertical-pressure
> research turn identified real implementation opportunities—unique-path PALMA composition,
> branch-capacity recursion, prefix/segmented scans, backpressure interpretation, and proportional-
> fairness structure—but also proposed several duplicate or mathematically over-strong frameworks.
> This revision records the useful mathematics as proof/performance candidates while preserving the
> germ rule: PALMA, Gu-Yang, STEAD, RF roles, span layouts, and ChangedLocus remain the presumptive
> authorities. No electrical solver, tropical subsystem, tree-contraction framework, delta engine,
> or clearing-owned field is authorized by this document.

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

## 0.1 The mirror insight that exposed the missing edge

The design-session question was simple:

> If a parent already knows how to disburse a finite constrained resource among `N` children, is the
> receiving side not the mirror of that same operation?

The answer is yes. A parent cannot lawfully disburse under the RF model without receiving, directly or
recursively, the demand and shortfall condition of each child branch.

That mirror exposed the remaining hinge:

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

The pressure must remain **branch-attributed** at each allocation boundary. A single parent-wide total
such as “subtree unmet demand = 500” is insufficient to determine which direct child branch should
receive more of a scarce resource.

For parent `p` and direct child `c`, the semantic equivalent of the following must be available without
scanning all descendants at allocation time:

$$
P_{p\rightarrow c,N}
=
\text{unresolved lawful pressure represented by child }c\text{ and its subtree at }N
$$

This does not authorize a new `BranchPressure` column. It requires the existing RF upsweep to preserve
enough child-segment attribution for the parent's continuous allocation pass.

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

$$
W_{p\rightarrow c,N+1}
=
G\!\left(
P_{p\rightarrow c,N+1},
Policy_{p\rightarrow c,N+1},
Triad_{p\rightarrow c,N+1}
\right)
$$

where `W` is the already-existing `AllocatorWeight` or a proved equivalent—not a new clearing-owned
weight.

The resulting continuous child allocation may retain the existing guarded share form:

$$
X_{p\rightarrow c}
=
S_p
\frac{W_{p\rightarrow c}}
{\sum_j W_{p\rightarrow j}}
$$

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

An interior node is simultaneously grantee of its parent and granter to its children. The root is the
degenerate case with no upstream granter; a leaf is the degenerate case with no child disbursement.

---

# 2. One germ at every depth

For a StemThing node `v`, the market germ can be described abstractly as:

$$
\mathcal{M}_v(S_v, \{P_c\}, \Phi_v)
\rightarrow
(L_v, \{G_c\}, \{U_c\})
$$

where:

- `S_v` is exact constrained supply available to node `v`;
- `P_c` is lawful branch pressure surfaced by child `c` and its subtree;
- `Φ_v` is the already-resolved RF/Field-Triad state at the governing scope;
- `L_v` is lawful local retention/consumption;
- `G_c` is the exact child grant;
- `U_c` is the unresolved portion.

This is a descriptive model, not authorization for new fields or types.

```text
ROOT:
    intrinsic/world supply + branch pressure
        → continuous allocation
        → exact child grants

INTERIOR:
    exact parent grant + local production + child pressure
        → continuous allocation
        → exact grandchild grants

LEAF:
    exact received grant
        → local consumption/holding/action
        → remaining shortfall contributes pressure upward
```

---

# 3. RECEIVE, not query and not recompute

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
```

Its lawful inputs are bindings to already-authoritative Current-plane roles such as:

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

> **No new persistent per-claim or per-branch state is admitted merely to make settlement
> field-aware or urgency-aware.**

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
ordering, and branch attribution are sufficient for constrained exact settlement.

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

## 4.6 Generation and async seams

Per-tree generation authority, no-wait stamped integration, and one recorded schedule already permit
independently advancing subtrees. The RF market core must remain per executing tree and must not assume
one global clock, host, process, device, schedule, registry, or raw-ID namespace.

---

# 5. Continuous market versus exact settlement

## 5.1 Field Triad authority

For claim `i`, explanatory terms are:

- `r_i`: lawful requested quantity;
- `a_i`: current serviceable quantity under route/channel constraints;
- `p_i`: branch-attributed unresolved lawful pressure;
- `x_i`: continuous provisional allocation emitted by the RF/Triad disbursement program;
- `g_i`: exact integer grant settled at the endpoint.

The intended constraints are:

$$
0 \le x_i \le a_i \le r_i
$$

and:

$$
\sum_i x_i \le S
$$

for exact supply `S` at the governing parent.

Holding policy and serviceability equal, native pressure should be monotone in continuous share:

$$
p_i' > p_i
\quad\Longrightarrow\quad
x_i' \ge x_i
$$

subject to admitted caps, competing pressure, bounded recurrence, and Gu-Yang realizability.

## 5.2 Exact endpoint authority

The settlement stage owns only:

1. exact representability/admission of the settlement envelope;
2. continuous-to-integer conversion;
3. exact supply conservation;
4. indivisible residue under canonical deterministic law;
5. `grant`, `U`, and commitment-state deltas;
6. exact relationship provenance;
7. one replay/schedule history committed atomically with state.

It may not re-run the continuous market or calculate branch urgency.

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

---

# 7. Three shortfall stages must remain distinct

## 7.1 Impaired lawful demand — before settlement

$$
U_i^{impairment} = r_i - a_i
$$

Lawful demand that cannot currently traverse the admitted route/channel.

## 7.2 Contention shortfall — at settlement

$$
U_i^{contention} = a_i - g_i
$$

Physically serviceable demand that did not receive exact supply.

## 7.3 Delivery shortfall — after entitlement

For exact grant `g_i` and realized delivery `y_i`:

$$
B_i^{delivery} = g_i - y_i
$$

The true blocked legal grant where entitlement may exist before realization.

## 7.4 Commitment semantics

```text
immediately executable flow
    exact grant is capped by current Gu-Yang serviceability

entitlement then deliver
    exact grant may enter in_flight
    Gu-Yang governs later realization
```

---

# 8. Urgency, persistence, branch attribution, and market transparency

The governing scope can already receive or derive demand, surplus/deficit, PALMA impedance, Gu-Yang
flow/saturation/stall, `U`, refusal disposition, and grant commitment state.

Visibility alone is not enough. Branch-attributed pressure must participate in the later continuous
allocation to that branch.

The current explicit unresolved-demand consequence path still routes through authored EML persistence
valuation, CostBand funding, and a later OverlayThing consequence. Therefore implementation archaeology
must classify whether ordinary `Need`/`Balance`/STEAD already:

```text
A. preserves unresolved lawful demand
B. accumulates it into greater pressure
C. requires the authored persistence path
```

But the Phase-14 outcome is mandatory:

> **Every StemThing must expose its own and its descendants’ unresolved lawful pressure through the
> branch-attributed RF upsweep, and at the governing parent that pressure must natively inform the
> existing continuous allocation weight/share for that child branch in a later generation.**

If first-order persistence is missing, the candidate bounded recurrence is:

$$
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
$$

where `B` is the existing bounded Current→Next recurrence or a rehomed equivalent.

Pressure is reduced once per tree edge. A parent consumes one aggregate per direct child branch; it
may not count both the branch aggregate and the same descendants again.

> **Market transparency is the default. Demand, urgency, impairment cause, saturation, impedance,
> provisional allocation, exact grant, and unmet-demand volume are visible to the governing scope as
> born field, RF, lifecycle, and schedule state. Occlusion is only an explicit authored act—an
> admitted, recorded, auditable policy or perception overlay—never substrate silence.**

---

# 9. Mathematical interpretation and performance research notes

This section records useful mathematical interpretations and optimization candidates. **None of these
formulae supersedes the archaeology-first reuse rule.** They are intended to help prove, falsify, or
optimize the existing RF market surfaces later.

## 9.1 Weighted proportional fairness explains the existing child-share shape

The normalized child-share form:

$$
x_i
=
S\frac{w_i}{\sum_j w_j}
$$

is not merely convenient arithmetic. It is the exact optimum of the weighted proportional-fair
problem:

$$
\max_{x_i>0}
\sum_i w_i\log x_i
$$

subject to:

$$
\sum_i x_i \le S.
$$

The Lagrangian first-order condition is:

$$
\frac{w_i}{x_i}=\lambda,
$$

which yields:

$$
x_i = \frac{w_i}{\lambda}
$$

and therefore the normalized share above.

**Engineering consequence:** if archaeology confirms `child_share_formula`/`AllocatedFlow` as the live
continuous authority, the existing RF disbursement already has a strong standard interpretation:
weighted proportional fairness over a parent's direct-child supply constraint. Branch pressure can
natively determine or deform `w_i` without inventing a new market solver.

The broader alpha-fair family remains a research/policy candidate:

$$
U_\alpha(x)=
\begin{cases}
\log x,&\alpha=1\\
\frac{x^{1-\alpha}}{1-\alpha},&\alpha\ne1.
\end{cases}
$$

For one uncapped parent constraint the optimum has the proportional shape:

$$
x_i \propto w_i^{1/\alpha}.
$$

Interpretation:

```text
alpha = 1
    proportional fairness

larger alpha
    increasingly fairness-oriented sharing

smaller alpha
    increasingly concentration/efficiency-oriented sharing
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

$$
\max_x \sum_i P_i x_i
$$

subject to the feasible capacity region. That is useful as a stability/urgency lens, but for a single
parent it can become highly concentrated. The RF market should therefore treat **backpressure as the
native urgency signal**, while retaining a smooth continuous sharing law such as the existing
proportional/alpha-fair allocation.

## 9.3 A combined continuous-market oracle candidate

A mathematically clean research oracle is:

$$
\max_{\{x_i\}}
\left[
\sum_i w_i(P_i)\log x_i
-
\eta\sum_i Z_i x_i
\right]
$$

subject to:

$$
0\le x_i\le a_i,
\qquad
\sum_i x_i\le S.
$$

Here:

- `P_i` is branch-attributed unresolved pressure from STEAD/RF;
- `w_i(P_i)` is a bounded pressure-to-utility transform;
- `Z_i` is PALMA accumulated route impedance;
- `a_i` is Gu-Yang serviceability/capacity;
- `S` is parent supply;
- `x_i` is continuous `AllocatedFlow`.

For an uncapped active branch, the KKT condition yields:

$$
x_i
=
\frac{w_i(P_i)}{\lambda+\eta Z_i},
$$

and with Gu-Yang caps:

$$
x_i
=
\min\left(
a_i,
\frac{w_i(P_i)}{\lambda+\eta Z_i}
\right),
$$

where `lambda` is the common shadow price chosen so total assigned flow fits `S` when enough serviceable
need exists.

**Disposition:** this is a **research oracle / interpretation candidate**, not a new runtime solver.
If the current RF/Triad path already produces equivalent shares, this mathematics validates it. If a
missing continuous operation is proven, it belongs in the existing RF/FieldSweep post/fold/disbursement
stage, never in exact settlement.

## 9.4 Electrical analogy — useful but bounded

On a unique parent→descendant tree, accumulated additive route impedance is exact by simple series
composition:

$$
Z_c = Z_p + z_{p\rightarrow c}.
$$

If one cumulative root-relative impedance `Z_v` is resident, then for ancestor `p` and descendant `v`:

$$
Z_{p\rightarrow v}=Z_v-Z_p.
$$

This is the useful part of the electrical analogy: **vertical PALMA on a strict unique-path tree does
not need a general graph solve or iterative relaxation.**

A conductance-weighted rule such as:

$$
W_i \propto \frac{P_i}{Z_i}
$$

may be a useful EML policy when `P` and `Z` have compatible potential/resistance semantics. It is **not
universal electrical law**, and nonlinear forms such as:

$$
W_i \propto \frac{P_i^\gamma}{Z_i^\beta}
$$

are authored policy families, not substrate physics.

The literal Ohm relation, where applicable, is:

$$
I = \frac{\Delta V}{R}=G\Delta V,
\qquad G=1/R.
$$

No vertical Laplacian solver, matrix inversion, or QAOA mechanism is implied.

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

```text
TreeVertical:
    prefix / path composition

GeneralLinkGraph:
    ordinary PALMA field evaluation
```

Do not assume every economic relation is tree-shaped merely because structural SimThing containment is
a tree.

## 9.6 Gu-Yang vertical serviceability recurrence

The simplistic edge clip:

$$
P_c^*=\min(P_c,C_{p\rightarrow c})S_{p\rightarrow c}
$$

is only valid when `P_c` already represents the recursively feasible demand of the entire child
subtree and the commitment class requires immediate deliverability.

For a rooted tree, a more exact bottom-up recurrence for immediately executable flow is:

$$
F_v
=
\min\left(
C_{p\rightarrow v},
d_v^{local}
+
\sum_{u\in children(v)}F_u
\right),
$$

where:

- `d_v^{local}` is local lawful demand;
- each `F_u` is already clipped by deeper subtree chokes;
- `C_{p→v}` is the parent-edge serviceability/capacity envelope.

Retain raw pressure separately:

$$
P_v
=
d_v^{local}
+
\sum_{u\in children(v)}P_u.
$$

Then pre-settlement impairment is:

$$
U_v^{impairment}=P_v-F_v.
$$

This yields two distinct upward facts:

```text
raw pressure P_v
    how much lawful demand exists

serviceable pressure F_v
    how much can currently cross the vertical cut
```

**Disposition:** this is best understood as a possible **tree lowering of existing Gu-Yang capacity
authority**, not a new clipping subsystem. If Gu-Yang already emits the necessary branch-local
serviceable flow, reuse it directly.

## 9.7 Pressure sums; bottleneck/peak observations do not replace them

Replacing conserved branch pressure with a tropical max:

$$
P_{parent}=\max_c(P_c+w_c)
$$

would lose total lawful demand. Double counting is prevented by once-per-edge branch attribution, not
by changing sum into max.

A second peak/hotspot observation can still be useful alongside the conserved sum. A parent may
conceptually carry:

$$
\left(
\sum_c P_c,
\max_c q_c
\right)
$$

where total pressure drives resource allocation while peak normalized shock can drive an emergency
ActionBand or hard-precedence policy.

With existing `EXP`/`LN`, a smooth-max research option is log-sum-exp:

$$
\operatorname{LSE}_\tau(q)
=
\tau\log\sum_i e^{q_i/\tau}.
$$

This is an optional observation/policy projection, not the conserved pressure lane and not a new
tropical framework.

## 9.8 Tree scans before tree contraction

The standing logical-subtree directory and DFS-like contiguous spans suggest a cheaper first
implementation than Miller-Reif rake/compress for additive pressure.

If `Prefix[k]` is a prefix sum over node-local pressure in a flattened subtree layout, then for a
contiguous subtree range `[start_v,end_v)`:

$$
P_v
=
Prefix[end_v]-Prefix[start_v].
$$

This can produce every additive subtree total from one segmented/prefix-scan family of primitives
without a contraction forest.

**Caveat:** nonlinear Gu-Yang serviceability clipping does not generally commute with a raw subtree
sum:

$$
\min\left(\sum_i P_i,C\right)
\ne
\sum_i \min(P_i,C_i).
$$

Therefore the clipped serviceability recurrence may still require depth-bucketed bottom-up dependency.
Miller-Reif tree contraction remains a dated performance candidate only if measured depth/pathology
justifies its irregular bookkeeping.

## 9.9 Delta masking dissolves into source-blind invalidation

Do not mint `DeltaFlowEngine` or `dirty_impedance_mask` as new semantic machinery.

Exact-change reuse should consume the existing `ChangedLocus` / `DerivedDependencyIndex` substrate:

```text
no relevant source locus changed
    → reuse valid vertical result

relevant source locus changed
    → recompute only the affected span/region
```

An epsilon rule such as:

$$
|\Delta P|<\varepsilon \Rightarrow \text{skip}
$$

is **not** an invisible optimization because it changes outcomes and makes them trajectory-dependent.
Any nonzero epsilon is authored/qualified approximation policy and requires its own ruling.

Static topology does not imply static impedance: policy, tax, blockade, access, congestion, owner
overlays, or conductance can dirty the vertical path. The dependency key must therefore cover every
contributing locus, not merely structural changes.

## 9.10 Prune compute, never information

A fully choked branch may permit substantial settlement/disbursement work to be skipped, but the system
must still preserve:

```text
raw U / pressure
impairment visibility
relevant refusal/lifecycle fact
entitlement-first in_flight semantics where admitted
```

A per-branch active mask is therefore a **physical dispatch optimization candidate**, not a semantic
state authority. A shader `return` does not automatically imply saved work at dispatch scale; real
benefit may require segment-level dispatch elision, compaction, or indirect work generation and must be
measured.

The law is:

> **Prune computation, never market information.**

## 9.11 Research references retained for future proof work

The following literature is relevant as mathematical/performance background, not current normative law:

1. **Kelly, Maulloo & Tan — Rate control for communication networks: shadow prices,
   proportional fairness and stability.** Useful for the weighted proportional-fair interpretation
   of the existing normalized child-share form.
2. **Mo & Walrand — Fair end-to-end window-based congestion control.** Useful for the generalized
   alpha-fair family and policy deformation of smooth sharing.
3. **Tassiulas & Ephremides — constrained queue/backpressure scheduling.** Useful for interpreting
   unresolved branch pressure as backlog and Gu-Yang/PALMA as the feasible service region.
4. **Blelloch — Prefix Sums and Their Applications.** Useful for flattened subtree sums, segmented
   reductions, and GPU-friendly scan formulations.
5. **Miller & Reif — Parallel Tree Contraction and its Applications.** Retained as a performance
   candidate if measured tree depth/pathology defeats simpler span/scan or depth-bucketed lowerings.
6. **Spielman–Teng graph-Laplacian work.** Relevant only to general cyclic graph/electrical problems;
   not required for strict vertical tree PALMA.
7. **QAOA/Gauss-law flow literature.** Conceptually interesting but currently decorative for the RF
   market implementation; no quantum or alternate flow solver is chartered.

These references should be revalidated against the final landed data model before being used as
acceptance rationale.

---

# 10. Recursive settlement and asynchronous subtrees

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

A detached/executing subtree receives its exact grant through the stamped seam and subdivides that
local supply without synchronous ancestor RPC. The realm/seam/non-foreclosure laws remain those of
Phase 14 and are not redefined here.

---

# 11. Surface-reuse matrix — mandatory before edict

Allowed dispositions are exactly:

```text
REUSE AS-IS
REUSE WITH BINDING
REHOME EXISTING AUTHORITY
MISSING — STOP FOR OWNER RULING
```

| Needed meaning | Existing candidate authority | Candidate disposition | Proof required before final law |
|---|---|---|---|
| Upward lawful need | `IntrinsicFlow`, `Balance`, owner-channel deficit/Need | **REUSE WITH BINDING** | Identify one authoritative quantity lane; prove Draw claims lower into it or remain authorization only |
| Branch-attributed subtree pressure | direct-child RF segment over `Need`/`Balance`/`U` | **REUSE WITH BINDING** or **MISSING — STOP** | One aggregate per direct child branch, zero descendant double count, zero host scan |
| Branch pressure → continuous allocation | `Need`/`Balance`/`U` → `AllocatorWeight` or existing share operand | **REUSE WITH BINDING** or **MISSING — STOP** | Greater unresolved branch pressure monotonically informs later share when policy/serviceability are equal |
| Continuous child share | `child_share_formula` + `AllocatedFlow` | **REUSE WITH BINDING** | Prove units, sign, bounds, scope, pressure sensitivity, and post-Triad ordering |
| PALMA vertical path cost | existing `D`/impedance column + tree adjacency | **REUSE AS-IS** or **REUSE WITH BINDING** | Prove tree-vertical unique-path specialization where applicable; retain general PALMA for alternate-route LinkGraph |
| Gu-Yang vertical serviceability | available/realized flux, net/gross/stall/saturation | **REUSE WITH BINDING** | Prove branch-local serviceable-flow/capacity recurrence and commitment-class semantics |
| Additive subtree pressure | 7.8a/13.x logical subtree spans + RF reduction | **REUSE WITH BINDING** | Prove segmented/prefix scan shape before considering tree contraction |
| Dirty vertical state | `ChangedLocus` / `DerivedDependencyIndex` | **REUSE AS-IS** | No duplicate mask/dependency registry; all contributing loci covered |
| Hard market precedence | existing demand priority, order-weight, or lawful OrderBand use | **MISSING — STOP FOR OWNER RULING** | Determine which existing authority owns economic precedence; do not conflate with stage order |
| Urgency persistence | `Balance`/Need/STEAD recurrence; authored persistence path | **REUSE / REHOME**, else **MISSING — STOP** | Pressure may not remain economically inert |
| Exact discrete settlement | constrained-clear integer/remainder/tie/provenance law | **REHOME EXISTING AUTHORITY** | Exact settlement over proved continuous shares without preserving host collection shape |
| Exact settlement band position | existing RF band layout | **REUSE WITH BINDING** | Settlement is one terminal RF stage, not a peer execution path |
| Holding accounts | `free/in_flight/occupied/capacity` | **REUSE AS-IS** | Immediate-flow vs entitlement-first semantics |
| Recursive grant subdivision | child-as-granter StemThing-B witness | **REUSE AS-IS** | Parent grant → child supply → grandchild allocation end to end |
| Replay/history | one `IntegrationSchedule` + Phase-14 resident head | **REHOME EXISTING AUTHORITY** | Resident consequences/history commit atomically; host drain is not N+1 gate |

## 11.1 Matrix STOP rule

No Phase-14 design may add a new persistent field plane, clearing-owned weight, urgency property,
branch-pressure property, share column, EML formula family, score vocabulary, impairment observer,
market registry, receive facility, disbursement facility, tropical framework, delta engine, tree-
contraction data structure, or clearing service until the corresponding matrix row proves the
semantic/physical function missing and the Owner/DA admits it.

---

# 12. Demand-vocabulary unification

The current tree contains at least two apparent demand vocabularies:

```text
RF Need / IntrinsicFlow / Balance / deficit
and
StemThing-B Draw-authorized runtime claim
```

The market mirror implies one of two lawful outcomes:

1. Draw authorization lowers into the ordinary authoritative RF need lane; or
2. Draw remains a strict authorization envelope while quantity is still read from ordinary RF need.

A Draw grants nothing. An outcome in which Draw claims form a second independent demand universe beside
RF pressure requires an explicit Owner ruling.

---

# 13. Phase-14 integration proposal

## 13.1 14.2 `RESIDENT-CLEARING-PLAN-0`

Keep all standing R1–R5 remand obligations unchanged.

Add the surface-reuse matrix and require:

- exact RF band slot for settlement;
- proof whether `AllocatedFlow` is the continuous share;
- proof whether Draw quantity lowers into ordinary Need/Balance;
- complete leaf → branch → parent allocation trace;
- proof whether branch pressure informs `AllocatorWeight` or another existing share operand;
- proof whether urgency persistence is native, persistent-only, or authored-only;
- classification of tree-vertical PALMA and Gu-Yang bindings;
- explicit statement that no new math framework is added merely because a research analogy exists;
- `MISSING` rows route to Owner/DA STOP before graduation.

## 13.2 14.3 `RESIDENT-CLEARING-SCORE-AND-BANDS-0`

Reframe around archaeology. If continuous allocation is already `AllocatedFlow`, bind and prove it
rather than building a clearing-owned score layer.

The ordering audit must distinguish:

```text
RF execution-stage order
hard economic precedence
continuous pressure-informed share
exact deterministic residue order
```

Do not feed arbitrary continuous field values into exact score-bit precedence bands.

## 13.3 14.4 `RESIDENT-CLEARING-APPORTIONMENT-0`

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

## 13.4 14.5 `RESIDENT-CLEARING-PARITY-0`

Retain the full positive/negative parity battery and add recursive/field witnesses:

```text
root
 ├─ child A
 │   └─ highly constrained grandchildren
 └─ child B
     └─ lightly constrained grandchildren
```

With equal authored policy and comparable serviceability, prove:

1. A's descendants create greater unresolved lawful pressure.
2. That pressure reduces once into A's branch aggregate.
3. A receives a larger continuous share in a later generation.
4. Exact settlement follows that share.
5. A's exact grant becomes supply for its own children.
6. When A's pressure subsides, its share subsides under the bounded recurrence.
7. No descendant row is double-counted.
8. No physical row, upload order, claim arrival, or CPU table determines the result.

Also include:

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

For the clip-induced switching case, prove bounded recurrence damps or otherwise lawfully bounds
oscillation at the serviceability boundary.

## 13.5 14.6 `RESIDENT-CLEARING-CUTOVER-0`

The production census must show no host-built weight/urgency table, CPU PALMA query, CPU Gu-Yang query,
CPU descendant-pressure scan, clearing-owned field cache, private flux/urgency solver, duplicate market
feedback path, duplicate settlement/disbursement path, or synchronous host schedule append before N+1.

The resident schedule segment remains the live head of the one schedule. CPU execution remains explicit
vendorized oracle posture, never automatic fallback.

---

# 14. Candidate binding laws

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
> cycle. It is not a peer clearing facility, manager, scheduler, registry, or market engine.

## 14.5 Receive-Not-Recompute Law

> The settlement stage receives already-resolved resident RF/Triad outputs. It may not privately
> reduce STEAD, relax PALMA, solve Gu-Yang, author urgency, query host state, scan descendants, or
> materialize a clearing-owned pressure/field/weight/share representation.

## 14.6 Recursive Grant-to-Supply Law

> A settled exact grant to a child becomes that child's exact constrained supply for its own
> descendants. The same germ applies at root, interior, and leaf nodes without a recursion service or
> domain-specific manager.

## 14.7 Continuous/Discrete Authority Law

> The Field Triad owns continuous value, route impedance, realizable conservative flow, and the
> pressure-informed allocation surface. Exact settlement owns only exact quantity conservation,
> discrete residue, commitment-state transition, provenance, and replay.

## 14.8 Gu-Yang Authority Law

> Gu-Yang remains capacity and realization authority. No urgency, weight, policy, hard-precedence
> class, or settlement residue may manufacture flow the conservative field forbids.

## 14.9 Hard-Precedence/Soft-Share Law

> Hard precedence is explicit admitted policy. Continuous RF/Triad allocation governs soft sharing
> within a precedence class and is natively informed by branch pressure. Arbitrary floating-point
> field differences may not silently become strict exact-score winner-take-all precedence.

## 14.10 Vertical Specialization Law

> Unique-path tree relations may lower PALMA to direct path composition and Gu-Yang to the existing
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

## 14.14 Native Persistence Law — conditional implementation, mandatory outcome

> If archaeology proves first-order unresolved-pressure persistence missing, unresolved lawful
> quantity re-enters next-generation STEAD pressure through the ordinary bounded recurrence. Authored
> EML deforms persistence; it does not create the baseline. Regardless of physical implementation,
> Phase 14 may not graduate with branch pressure visible but disconnected from later allocation.

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
8. cannot trace leaf shortfall → branch pressure → parent share → grant → child supply;
9. double-counts branch aggregate and descendants;
10. creates a second demand universe beside RF Need/Balance;
11. conflates `U` with CostBand `R`;
12. conflates impairment, contention shortfall, and delivery shortfall;
13. exceeds Gu-Yang serviceability for immediate-flow grants;
14. forbids lawful entitlement-first `in_flight` semantics;
15. hides scarcity state without authored policy;
16. builds same-generation reweight/re-clear convergence;
17. requires synchronous ancestor RPC for unrelated subtree generations;
18. creates a second history or permits state commit without schedule rows;
19. uses root-only or leaf-only market paths;
20. preserves CPU collection shape merely because it is the oracle;
21. replaces additive lawful pressure with max/tropical pressure;
22. adds Laplacian/electrical solver machinery for a strict unique-path vertical tree;
23. adds a separate delta/dirty subsystem beside `ChangedLocus`;
24. claims epsilon-based skipping as semantics-preserving without an explicit approximation ruling;
25. prunes raw U/pressure or impairment visibility merely because a branch is currently choked.

---

# 16. Open evidence and performance questions

1. Is `AllocatedFlow` the complete continuous settlement input?
2. Which existing lane owns branch pressure, and which binding maps it into allocation?
3. Which existing surface owns hard precedence?
4. Does unresolved lawful demand persist or accumulate natively?
5. Where exactly does settlement land in the existing RF band layout?
6. What is the exact continuous-to-integer contract?
7. Can the CPU oracle expose the same generalized settlement-over-share-vector interface?
8. For tree-vertical markets, can PALMA path impedance be supplied by prefix/path composition from existing tree metadata with no extra persistent plane?
9. Does Gu-Yang already expose branch-local serviceable flow, or must the tree-capacity recurrence be bound as a new lowering of existing authority?
10. Do 7.8a-style contiguous subtree spans permit one segmented/prefix scan for all additive branch pressures at target cardinality?
11. At measured tree depths, is a depth-bucketed Gu-Yang serviceability recurrence faster/simpler than tree contraction?
12. What fraction of vertical impedance/pressure work can `ChangedLocus` invalidation actually elide at realistic policy/churn rates?
13. Does active-segment dispatch pruning outperform dense resident execution after accounting for compaction/indirect-dispatch overhead?
14. Does the proportional-fair interpretation match the exact semantics of the existing `child_share_formula`, including guards, caps, and sign behavior?
15. Would an alpha-fair or impedance-penalized EML policy improve domain behavior without creating instability or undermining replay exactness?

---

# 17. Review checklist

A reviewer should be able to answer **yes** to all of the following before promotion:

- one receive/resolve/settle/disburse RF market cycle, not a clearing subsystem;
- branch pressure is preserved per direct child and influences later allocation;
- no descendant double counting;
- `AllocatedFlow` is reused or disproven before a replacement is named;
- hard precedence stays distinct from continuous share;
- exact settlement remains small and discrete;
- child grant becomes child supply without a second API;
- the same germ applies to root, interior, and leaf;
- Gu-Yang and PALMA are consumed from born state rather than recomputed in settlement;
- tree-vertical specializations remain lowerings of PALMA/Gu-Yang, not new frameworks;
- pressure sum remains conserved; peak/max observations do not replace it;
- exact-change masking reuses source-blind invalidation;
- U, R, impairment, contention shortfall, and delivery shortfall remain distinct;
- market transparency is default and occlusion authored;
- there is no same-generation re-clear loop;
- async subtree, realm, schedule, and no-CPU-fallback laws remain intact;
- neutral behavior reproduces the frozen clearing oracle;
- the implementation deletes more vocabulary than it adds.

---

# 18. Relationship to CausalBand and the ML corpus

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
can show where shortfall originated, how it propagated upward, how allocation shifted later, which
exact grants followed, and what pressure remained.

This does not give the atlas or a learned model runtime settlement authority. The RF market core remains
simulation truth; atlas and replay are downstream observation/training artifacts.

---

# 19. Proposed final synthesis

> **Every StemThing is an RF market germ. Descendant need, deficit, and unresolved pressure reduce
> upward with direct-child branch attribution. At each governing parent, that pressure natively
> informs the later continuous allocation to each branch through the existing RF/Field-Triad
> surfaces. Allocated flow disburses downward; exact settlement turns that resolved flow into
> discrete identity-bearing possession; the child then repeats the same cycle with its own
> descendants. Receiving, market guidance, clearing, and disbursement are not peer facilities but
> stages of one recursive market filter. The exact endpoint receives rather than recomputes: it owns
> no route, flux, urgency, pressure, weight, or field model—only exact conservation, integer residue,
> commitment state, provenance, and replay.**

The research interpretation now adds one useful scientific gloss without changing that architecture:

```text
backpressure
    supplies the native urgency signal

PALMA
    supplies path cost / potential

Gu-Yang
    supplies the feasible capacity envelope

existing proportional-fair-shaped RF disbursement
    supplies the smooth continuous allocation

exact settlement
    supplies integer possession and replay
```

That is the Phase-14 design target this workshop draft submits for review.
