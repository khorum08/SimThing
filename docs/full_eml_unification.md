# Full EML Unification — completing the operator the interpreter is named for

> **Status: ADMISSION CASE + INTEGRATION PLAN (Owner-directed 2026-08-03; DA-authored, Fable).**
> Non-binding on process. The binding surfaces are the `ExactPrimitiveAdmission` door
> (`simthing-kernel/src/eml_opcode_gate.rs`, landed at rung 5.7 with **zero admitted
> primitives**), the EML growth law (core design §4.1), the C-8 class discipline
> (`adr_accumulator_op_v2.md`), and [`eml_gadget_library.md`](eml_gadget_library.md). This
> document is the case for admitting `EXP` and `LN` through that door, the formalization options,
> the gadget-library and JIT integration plan, and the STEAD/RF Triad consequences. It opens no
> rung and admits nothing; each primitive lands through the door under its own rules — one proven
> primitive per landing, with a named consumer.
>
> **This work is independent of the StemThing HARD HOLD and of Phase 6 completion.** It is
> 5.7-door consumer work touching neither the slot-identity ruling nor the movement rungs.

---

## 1. Archeology — what was actually ruled, and why

The historical record, with receipts:

- **C-8 (2026-05, `adr_accumulator_op_v2.md`)** admitted exactly one EML class to production:
  **`ExactDeterministic`** — *"no transcendentals, ≤16 nodes, deterministic IEEE-754 ops only,
  bit-exact CPU↔GPU."* Three further classes were structurally prepared and left disabled:
  `SoftDeterministic` (documented `max_abs_error`), `FastApproximate` (*"vendor-native math; not
  replay-safe under current model"*), `CpuOracleOnly`.
- **The gadget library** encoded the gate: *"No new EML opcode, including transcendental, without
  a separate explicit substrate gate"* and *"No transcendental inside an `ExactDeterministic`
  gadget."*
- **The actual reason** was never that EML should lack `exp`. It was that the only available
  `exp` was the driver's: WGSL transcendental built-ins carry implementation-defined precision,
  so admitting them would have poisoned bit-exact replay, cross-vendor reproducibility, and —
  once the dual mission was named — the corpus. The deferral was a **determinism fence**, and it
  was correct.

**The standing irony, verified at the opcode level:** the interpreter's roster is
`LITERAL_F, SLOT_VALUE, PARAM, ADD, SUB, MUL, NEG, DIV, MIN, MAX, CLAMP_BOUNDED, CLAMP_FLOORED,
ABS, FLOOR, CMP_{LT,LE,GT,GE,EQ}, SELECT, RETURN_TOP` — **`EvalEML` contains no `EXP`, no `LN`,
and therefore cannot compute `eml(x,y) = exp(x) − ln(y)`.** SimThing adopted the paper's
execution model (the RPN opcode stack) and its license (behavior as data over one interpreter)
while leaving behind its operator. Anchor B's universality claim (core design §1.1) is true of
the paper and honorific of the implementation.

## 2. The prophecy — the deferral named its own unlock

C-8 did not close the door; it specified the door:

> *"The ADR does **not** permanently restrict the GPU substrate to zero-transcendental formulas;
> it permanently restricts **conservation paths** to `ExactDeterministic`."*

Read together with the gadget library's gate demand, the deferral was a conditional with four
implicit unlock conditions: (1) an explicit substrate gate for opcode admission, (2) a
determinism-proof discipline for functions the compiler cannot check, (3) a cost-accounting
mechanism, and (4) a consumer that needs the capability rather than admires it.

## 3. 0.0.8.7 as the emergent consumer — every condition discharged, none planned as EML work

| Unlock condition (2026-05) | Discharged by (2026-07/08) |
|---|---|
| an explicit substrate gate | **`ExactPrimitiveAdmission`** (rung 5.7): sovereign determinism key — specified bit semantics, `ExactPrimitiveDomainPolicy`, exhaustive reference artifact, **supported-backend replay artifact** — plus conjunctive cost key and consumer key; sealed token; vocabulary expansion remains a separate DA-scoped `CLOSED_OPCODES` change |
| exhaustive-proof precedent | Candidate F: f32 unary = 2³² inputs, provable by enumeration in GPU-hours. (Caution stands: Candidate F is a Q16.16 magnitude mechanism, **not** a general `exp`/`ln` method — the precedent transferred is the *proof pattern*, not the algorithm) |
| cost accounting | **`EmlResourceClass`** (5.7): closed set, deterministic smallest-fit; transcendental weight is exactly what the cost key prices |
| a compilation home | the **SSA JIT** (5.7), which beat the bespoke shader on PALMA (0.80× worst-case) — a straight-line polynomial block is its ideal input |
| a named consumer | **CostBand EML-computed steering (6.1b, graduated)** — the 5.7 row itself pre-authorized the bounded-domain narrowing as "the cheapest first customer for the door" |

This is the cheap-and-powerful justification in one sentence: **the track built the door, the
price tags, the compiler, and the first customer without anyone planning an EML-completion
program — the remaining work is one admission plan per primitive.**

## 4. Formalizing `EXP` and `LN` — the two admission shapes

The door's determinism key demands *specified bit semantics*. Two lawful ways to specify them:

**(i) Algorithm-as-spec (recommended first landing).** The primitive's semantics **are** its
pinned evaluation sequence: a fixed-order polynomial/bit-manipulation routine built **only from
correctly-rounded WGSL operations (`add`/`sub`/`mul` and integer/bit ops; no `div`, no vendor
transcendentals, no reassociation)**, with the CPU oracle implementing the identical sequence.
Bit identity CPU↔GPU is proven by **exhaustive 2³² enumeration**; adapter conformance by the
per-backend replay artifact. This is Candidate F's own pattern (its Q16.16 pipeline is
algorithm-as-spec, not correctly-rounded sqrt). Cheap, lawful, and sufficient — SimThing's
requirement is **reproducibility, not mathematical correct rounding**.

**(ii) Correctly-rounded (horizon upgrade, gold standard).** Semantics = the correctly-rounded
f32 result of the real function (CORE-MATH/RLIBM class). Portable and
algorithm-independent, but materially harder: published implementations lean on f64
intermediates WGSL does not guarantee, so an f32-only route needs compensated (error-free
transformation) arithmetic — real numerical engineering, still provable by the same exhaustive
enumeration.

**Immutability rule (binding on either shape):** admitted semantics are append-only. If (ii) is
ever pursued after (i) lands, it lands as a **new named primitive** (`EXP_CR`), never a mutation
of `EXP` — a semantics change is a replay epoch and a corpus fork, and the registry discipline
already forbids it.

**Domain policy per primitive** (the door's `ExactPrimitiveDomainPolicy` already expresses
these):

- `EXP` — first landing **bounded-domain**: input `[−87.33, 0.0]`, output `(0, 1]` —
  covers every decay, sigmoid, softmax-numerator, and falloff consumer; makes the exhaustive
  proof and the polynomial's range reduction trivial (no overflow arm). Full-domain
  `[−87.33, +88.72]` as a follow-on landing if a consumer names it. `FiniteOnlyRejectNanAndInfinity`.
- `LN` — positive normals `[2⁻¹²⁶, +∞)` (spanned admission error below), output finite.
  `FiniteOnlyRejectNanAndInfinity`; the paper's extended-real conventions (`ln(0) = −∞`) are
  **rejected, not emulated** — the engine has no ∞-propagation discipline and must not grow one.

**Sequencing through the door** (one proven primitive per landing, per 5.7's own rule):

1. **`EXP` (bounded)** — consumers: CostBand smooth steering curve (6.1b, landed), exact
   exponential STEAD falloff (§7).
2. **`LN`** — consumers: log-domain accumulation (§7), and jointly with `EXP` the **power-law
   gadget** `POW(x, a) = EXP(a · LN(x))` — the entire power-law family with **no third opcode**.

## 5. Gadget-library integration

New library entries, each an authored tree over the widened vocabulary — **no new kernels, no new
opcodes beyond the two primitives**, each carrying the standard bounded-feedback contract:

| Gadget | Tree | Contract note |
|---|---|---|
| `Logistic(x; k, x₀)` | `1 / (1 + EXP(−k·(x−x₀)))` via `DIV`-free reciprocal form | output bounded (0,1) by construction — P3-friendly smooth gate; upgrades 6.1b banded steering from `SELECT` staircases to curves |
| `ContinuousDecay(x; λ, dt)` | `x · EXP(−λ·dt)` | exact decay under variable dt; bounded for λ,dt ≥ 0 |
| `PowerLaw(x; a)` | `EXP(a · LN(x))` | domain guard at admission (x > 0); the falloff/scaling family |
| `LogAccumulate` | `LN` map before the existing Sum reduction | converts product chains to sums — see §7 for why this is a parallelism unlock |
| `SoftmaxWeight(xᵢ; β)` | `EXP(β·xᵢ)` map + existing Sum reduce + normalize band | temperature-controlled choice; β is an ordinary personality column |
| `Entropy(p)` | `−Σ p·LN(p)` via map + Sum | corpus observable; diagnostic lane |
| **`eml(x, y)`** | `SUB(EXP(x), LN(y))` | **three nodes.** The operator the interpreter is named for becomes expressible; Anchor B's universality claim becomes literal. Documented as a library entry for exactly that reason |

**What completing must never mean:** the paper's minimalism is a trap. Pure-EML encoding is a
15–35× expression blowup (`x·y` is K=17 leaves; trig "too large to print"), with catastrophic
cancellation by construction and complex-domain requirements for full generality. **The 21-op
roster stays; `eml()` becomes derived, never foundational.** The paper's value was always the
license — behavior as data over one interpreter — not the operator's economy.

## 6. JIT / WGSL implementation exploration

- **Interpreter path:** two new opcode arms evaluating the pinned sequence inline. Stack cost is
  unchanged (unary ops); the sequence's ALU cost (~15–25 fused steps) rides in the memory shadow
  of the gathers for sweep-shaped consumers — the regime 5.7's PALMA result measured. The
  occupancy question is the resource class's to answer, not a reason to pre-shrink.
- **JIT path (the natural home):** the SSA JIT emits the pinned sequence as a **straight-line
  block** — no loop, no branch beyond the domain guard, register-resident intermediates. This is
  the disciplined version of "bespoke EML shader blocks": the blocks exist, but they are
  JIT-emitted from the one IR under `FIELD-SWEEP-SINGLE-PATH`, never hand-written shaders.
- **Determinism hazards, named:** backend compilers may contract (`fma`) or reassociate. The
  determinism key's source-level proof must pin the WGSL to forms naga/backends translate
  1:1 (documented per supported adapter), and the **exhaustive check is the tripwire that
  catches any violation** — a fused multiply changes bits and the 2³² sweep reds. `div` is
  excluded from primitive internals (2.5-ULP latitude); reciprocal forms use pinned
  Newton–Raphson steps from `mul`/`sub` where needed.
- **Cost key:** each primitive lands with a measured resource-class entry (transcendental
  weight); admission rejects a stack whose class cannot afford it — the 5.7 machinery, unchanged.

## 7. STEAD / RF Triad integration — immediate and horizon

**Immediate (consumers that exist today):**

- **CostBand smooth steering (6.1b).** Banded magnitude responses become curves; a `Logistic`
  over `N` replaces nested `SELECT` ladders. The primary event-steering surface gains continuous
  response with zero new mechanism.
- **Exact exponential falloff.** STEAD falloff weights are currently authored per hop band;
  `EXP(−λ·d)` makes falloff an authored *law* rather than a table — fewer authored numbers, exact
  gradients, and P3-friendlier fronts (smooth `σ(u)` saturation available to the Gu-Yang
  conductance instead of piecewise clamp corners, which is where oscillation artifacts
  concentrate).
- **PALMA multiplicative impedance.** Min-plus is additive; `LN` converts multiplicative costs
  (attenuation, reliability, compounding penalty) into additive `W` — reliability-weighted reach
  becomes *exactly* expressible in the existing sweep with a log-domain compose band.

**The parallelism unlock (the load-bearing performance claim):** long **product chains are
sequential or fixed-order today** (banded multiplies; the fold's canonical linear order). In log
domain they become **sums — and sums own the entire existing reduction infrastructure**:
OrderBand hierarchical reduction, `SlotRange` coalescing, and (post-StemThing) contiguous range
folds. `LN`-map → Sum-reduce → `EXP`-post converts an O(n)-deep dependent chain into the
tree-parallel reduction the substrate has optimized since C-5/C-6. Softmax is the same shape:
`EXP` map + Sum reduce + normalize — two banded sweeps, no new machinery. This is not a new
parallel mechanism; it is **admission of a change of variables that lets multiplicative dynamics
ride the parallel machinery that already exists.**

**Horizon (consumers this unlocks, gated as ever by consumer-pull):**

- **Boltzmann Layer-3 decisions.** `SoftmaxWeight` over reduced pressure columns with a
  personality β — temperature-controlled commitment sharpness ("decisiveness" as an ordinary
  column). The FIELD_POLICY model gains graded stochastic-looking choice while staying fully
  deterministic.
- **Corpus yield.** Entropy/log-partition observables as native ground-truth columns;
  power-law and log-normal synthetic regimes widening the §1 data-complexity axis — dynamics
  classes no lattice-arithmetic corpus can produce (multiplicative noise, preferential
  attachment, scale-free LinkGraph weights for 5.6 adjacency).
- **StemThing derivation pricing (phase-gated).** Tier price curves with diminishing returns
  (log pricing) and potency-depletion curves — authored as gadgets when the phase lands.
- **Domain exemplars (11.2-class).** The canonical volume-delay function in traffic assignment
  is a power law; exceedance/return-period curves in hazard are log; utility, backoff, and
  congestion pricing in the network-governance plug are exp/log-shaped; growth phases in
  epidemiology are exponential. **`POW` and `EXP` are the difference between "the exemplar
  approximates its domain's canonical law" and "states it."**

## 8. Non-goals and risks

**Non-goals:** no pure-EML vocabulary replacement (§5); no vendor-native `exp`/`log` on exact
paths, ever; no trig in this plan (same door, future case, only with a named consumer); no f64;
no ∞-propagation semantics.

**Risks:** compiler fusion/reassociation (mitigated by pinned source forms + the exhaustive
tripwire + per-backend replay artifacts); cost creep in hot sweeps (priced by the resource
class, measured not assumed); precision-expectation mismatch (algorithm-as-spec documents exact
bit behavior; a consumer requiring true correct rounding must name it, which routes to the
`EXP_CR`-class follow-on); replay-epoch discipline (admitted semantics immutable; any change is
a new primitive name).

## 9. Disposition

Two admission plans through the landed 5.7 door, in order: **`EXP` (bounded) with the CostBand
steering + STEAD falloff consumers**, then **`LN` with the log-domain accumulation + `POW`
consumers**. Each landing carries: the pinned sequence and its CPU-oracle twin, the exhaustive
2³² reference artifact, per-backend replay artifacts, the resource-class cost entry, the
`CLOSED_OPCODES` DA-scoped vocabulary change, the gadget-library entries it enables, and the
scan/allowlist co-evolution the doctrine-CI contract requires. No rung is opened by this
document; the admission plans are DA-reserve work that can proceed independently of Phase 6
completion and the StemThing HARD HOLD.

The deferral held the door shut until the substrate could prove what walks through it. The
substrate now can. Completing EML is one landing per primitive — and at the end of it, the
interpreter named for `eml(x, y)` computes it.

## References

- Odrzywołek, *All elementary functions from a single operator* (arXiv:2603.21852v2) — the
  operator, the grammar `S → 1 | eml(S,S)`, the RPN machine; also the expression-blowup and
  complex-domain facts that make pure-EML minimalism a non-goal (§5).
- `adr_accumulator_op_v2.md` — the C-8 class discipline and the prophecy clause (§2).
- [`eml_gadget_library.md`](eml_gadget_library.md) — the gate demand; the library this plan
  extends.
- `crates/simthing-kernel/src/eml_opcode_gate.rs` — `ExactPrimitiveAdmission`,
  `ExactPrimitiveDomainPolicy`, `EmlResourceClass` (rung 5.7).
- `design_0_0_8_7_rf_arena_modernization.md` — rows 5.7 (the door), 6.1b (the consumer);
  [`eml_n4_expansion_digest.md`](eml_n4_expansion_digest.md) (field-sweep provenance).
- [`simthing_core_design.md`](simthing_core_design.md) §1.1 Anchor B, §4.1 the extension ladder.
