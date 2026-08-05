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
>
> **Amended 2026-08-03 (decorrelated DA review — Codex Sol Max: ADMIT WITH TARGETED REMAND; all
> five repairs adopted).** The ruling formally ends the transcendental prohibition — *"`EXP` and
> `LN` now have standing permission to seek admission; the determinism objection was the right
> objection then; it is now a gate to pass, not a reason never to attempt the capability."*
> Repairs: (1) primitive input domains become a **sealed admission type** (they were claimed of a
> two-variant enum that cannot express them); (2) the cost claim is reconciled with the real gate
> (`EmlResourceClass` prices node count + stack only; the strict-improvement cost law is used
> as-is against the **gadget-encoding baseline**, and is not weakened); (3) **full-domain `EXP`
> is the canonical first landing** and the bounded variant is dropped — with the Softmax/Logistic
> forms corrected to their stabilized constructions; (4) the parallelism claim is rewritten —
> log-domain accumulation is a **new authored numerical law**, never a semantics-preserving
> Product optimization; (5) the determinism contract specifies the **observable invariant**
> (exhaustive artifact parity on a certified toolchain), never a compiler-control device.
>
> **Final ruling 2026-08-03 (Sol Max): DESIGN-ADMITTED.** *"`EXP`: proceed to
> primitive-admission implementation planning. `LN`: proceed immediately behind."* Three
> surgical corrections folded in place: range-certified input vs. explicitly-guarded semantics
> distinguished (§4 — a clamp is *chosen semantics*, never a validity proof); bitcast fencing
> demoted from law to optional implementation aid (§6 — *"if a backend passes exhaustive parity
> with naked straight-line arithmetic, ship the faster naked arithmetic"*); the stale
> transcendental-weight sentence deleted (§6). The retained rigor is the cheap kind: the 2³²
> exhaustive qualification costs nothing in the hot path and exhausts the input universe.

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

**Domain contract — a generic admission addition this plan owes first (remand repair 1).** The
door's `ExactPrimitiveDomainPolicy` today has exactly two variants
(`FiniteOnlyRejectNanAndInfinity`, `PreserveIeeeNanInfinityAndSignedZero`) — it **cannot express
an interval**, and the registry carries no static interval proof per call site. Before the first
primitive lands, the door gains a sealed domain type, conceptually:

```
PrimitiveDomain { min_bits: u32, max_bits: u32, special_value_policy }
```

with the call-site obligation discharged by **exactly one of two cheap admission shapes — and
the distinction between them is semantic, not cosmetic** (final Sol correction 1):

1. **Range-certified input** — an existing sealed property/sub-field range guarantee (clamp
   behavior, bounded-output gadget, literal in range) already proves the input satisfies the
   primitive domain. **Zero runtime cost**; admission verifies the certificate.
2. **Explicitly guarded semantics** — the author deliberately wraps the argument
   (`CLAMP_BOUNDED`, `MAX`, `SELECT`, …), and **that guard is part of the formula's authored
   semantics, not a proof about the source value**. `EXP(CLAMP(x, lo, hi))` *means* saturated
   exponential — a legitimate authored law. But a clamp does not certify that `x` was in domain;
   for `LN`, silently converting `x ≤ 0` into `LN(min_normal)` can mask an authoring error, so
   the gate records shape 2 as *chosen semantics*, never as validity of the unguarded input.

Either shape is trivially checkable at admission; neither is an interval analyzer (EML inputs
are runtime columns whose ranges are unknowable statically — a full interval prover would be
heavy, conservative, and unnecessary). This is generic door machinery, not an `EXP` exception.

**Domain per primitive:**

- `EXP` — **full domain `[−87.33, +88.72]` is the canonical first landing** (remand repair 3,
  option A adopted). Rationale: the primitive's internals are raw WGSL — unconstrained by the
  EML vocabulary — where full range reduction is standard exponent-field bit assembly; the
  exhaustive sweep is 2³² either way; and opcodes are append-only forever, so permanently
  carrying a nearly-duplicate `EXP_NEG` is the worse trade. Output spans positive finite f32;
  an unguarded, uncertified call site is a spanned admission error (shape 1 or 2 above, always).
- `LN` — **positive finite normals `[2⁻¹²⁶, f32::MAX]`** (notation corrected: infinity is
  rejected, so the domain is closed at `f32::MAX`), output finite. The paper's extended-real
  conventions (`ln(0) = −∞`) are **rejected, not emulated** — the engine has no ∞-propagation
  discipline and must not grow one.

**Cost key — reconciled with the real gate (remand repair 2).** `EmlResourceClass` prices node
count and peak stack **only**; no transcendental weight exists in the classification, and this
plan **mints none**. The existing exact-primitive cost law — no regression plus at least one
strict improvement in compiled resource effects — is used **unweakened**, with the natural
baseline it implies: a pinned polynomial `exp` approximation is *expressible today* as an
ordinary `ADD`/`MUL`/`FLOOR` tree (~20 nodes, `ExactDeterministic`-legal, authored data). **The
primitive must beat its own gadget encoding.** A straight-line JIT block against a ~20-node
interpreted tree either wins that comparison or does not deserve admission — and if the cost law
itself is ever thought wrong, that is a separate DA argument, not a rider on this landing.

**Sequencing through the door** (one proven primitive per landing, per 5.7's own rule):

1. **`EXP` (full domain)** — consumers: CostBand smooth steering curve (6.1b, landed), exact
   exponential STEAD falloff (§7).
2. **`LN`** — consumers: log-domain accumulation (§7), and jointly with `EXP` the **power-law
   gadget** `POW(x, a) = EXP(a · LN(x))` — the entire power-law family with **no third opcode**
   (and requiring full-domain `EXP`, since `a·LN(x)` is signed — the bounded variant could not
   have delivered this, which is repair 3's point).

### 4.1 Arithmetic semantics and the uniqueness rule (5.14 / DA `5192270934`)

Author-facing EvalEML arithmetic meanings — arm-independent:

| Opcode / composition | Meaning |
|---|---|
| `ADD`, `SUB`, `MUL`, `DIV` | IEEE-754 single-rounding; **no reassociation** |
| `MIN`, `MAX`, `CLAMP_BOUNDED`, `CLAMP_FLOORED` | Exact selections (no rounding) |
| `EXP`, `LN` | Pinned algorithm-as-spec digests (unchanged by 5.14) |
| **Uniqueness contraction** (DA `5192270934`) | A `MUL` result is fused into its consuming **`ADD`** **iff that fusion is UNIQUE** |

- Exactly one `MUL` feeds an `ADD` → **FUSED** (one rounding; intrinsic `fma` / `mul_add`).
- Two or more `MUL` results feed the same `ADD` → **UNFUSED** (`U`): each `MUL` rounds to
  f32 before the `ADD` consumes it. **No tie-break exists or will exist.**
- `SUB` is **not** under this contraction rule — only its standalone IEEE-754 single-rounding
  meaning. No fused-multiply-subtract law is minted here.
- The historical **SEAM LAW** (map ends `[..,MUL,RETURN]` + canonical Sum fold) is an **instance**
  of uniqueness at a map/fold boundary — not a peer law.
- Each execution arm must **prove** it lowers to these meanings. Observing that an interpreter
  happens to produce `U` is not the language meaning (Exact-Value Provenance Law).

Per-consumer exactness digest policing (5.13) is **deleted**. Cross-arm bit identity of consumers
is a language-level witness following from these meanings, not an admission census.

## 5. Gadget-library integration

New library entries, each an authored tree over the widened vocabulary — **no new kernels, no new
opcodes beyond the two primitives**, each carrying the standard bounded-feedback contract:

All forms below are the **stabilized constructions** (remand repair 3) — arguments to `EXP` are
kept in the numerically safe region *by construction*, which is both the clamp-guard-friendly
form and the numerically correct one:

| Gadget | Tree | Contract note |
|---|---|---|
| `Logistic(x; k, x₀)` | sign-stable form over `EXP(−ABS(k·(x−x₀)))` with a `SELECT` on the sign | exponential argument ≤ 0 always; output bounded (0,1) — P3-friendly smooth gate; upgrades 6.1b banded steering from `SELECT` staircases to curves |
| `ContinuousDecay(x; λ, dt)` | `x · EXP(−λ·dt)` | exact decay under variable dt; argument ≤ 0 and output bounded for λ,dt ≥ 0 |
| `PowerLaw(x; a)` | `EXP(a · LN(x))` | requires **full-domain `EXP`** (`a·LN(x)` is signed); `LN` clamp guard at the call site (x in positive normals) |
| `LogAccumulate` | `LN` map before the existing Sum reduction | a **new authored numerical law**, not a Product optimization — see §7 |
| `SoftmaxWeight(zᵢ; β)` | `EXP(β·(zᵢ − max z))` — max via the existing `MAX` reduction band, then map + Sum reduce + normalize | **the stabilized softmax**: every exponential argument ≤ 0 by construction; the naive `EXP(βzᵢ)` form is rejected as numerically unsound regardless of domain policy. β is an ordinary personality column |
| `Entropy(p)` | `−Σ p·LN(p)` via map + Sum | corpus observable; diagnostic lane; `LN` guard at p > 0 (the p = 0 term is authored away via `SELECT`, matching the measure-theoretic convention) |
| **`eml(x, y)`** | `SUB(EXP(x), LN(y))` | **three nodes**, requiring **full-domain `EXP`** (`eml(1, y)` needs `EXP(1)`). With option A landed, the operator the interpreter is named for becomes expressible and Anchor B's universality claim becomes literal — a claim the bounded variant could not have made |

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
- **Determinism — the observable invariant is the law (final Sol correction 2).** WGSL's
  `+`/`−`/`×` are individually correctly rounded, but the spec **explicitly permits
  reassociation and fusion** when the transform is at least as accurate — so "we wrote the
  polynomial as a fixed sequence" is *not by itself* a portable bit-semantics specification. The
  binding rule, stated as the invariant rather than a compiler-control device:

  > **The pinned algorithm is the reference semantics. The generated artifact must exhaustively
  > match its CPU twin over the admitted f32 domain on each supported certified toolchain** —
  > the (compiler, backend, driver) combination is part of the certified substrate, with
  > automatic invalidation and requalification whenever that trust chain changes.

  Source-level anti-reassociation devices — `f32 → u32 → f32` bitcast fences and their kin —
  are **implementation aids only, not law**: the WGSL spec does not guarantee that a reversed
  bitcast round-trip constitutes a reassociation barrier, so the architecture must not depend on
  that inference. Retain a fence **only where measurement shows it is necessary and free**; if a
  backend passes exhaustive parity with naked straight-line arithmetic, **ship the faster naked
  arithmetic**. (Memory round-trip fences remain rejected outright — performance-hostile.) The
  2³² digest is the sole tripwire either way: any transform that changes bits reds the sweep.
  `div` is excluded from primitive internals (2.5-ULP latitude); reciprocal forms use pinned
  Newton–Raphson steps from `mul`/`sub`.
- **Cost key:** compiled primitive cost is judged against the gadget/interpreter baseline
  through the **existing exact-primitive cost key** (§4) — no per-primitive resource-class
  entries, no transcendental weight, nothing minted (final Sol correction 3).

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

**Log-domain accumulation — a new authored numerical law, stated lawfully (remand repair 4).**
The mathematical identity `∏xᵢ = exp(Σ ln xᵢ)` does **not** transfer bit semantics:
`EXP(Σ LN(x))` is *not* bit-equivalent to the sequential f32 product, f32 addition is
non-associative, and taking logarithms **does not legalize tree-splitting a fold** — the
field-sweep law (canonical order sealed; no tree reduction without an associativity proof)
applies to the log-sum exactly as to any Sum. What the change of variables *lawfully* buys:

- **`LogAccumulate` is a different authored law a designer may choose** — with its own numerics,
  documented — never a substitution for exact Product on a conservation path.
- Once chosen, the log-sum **rides the Sum lane's existing discipline unchanged**: canonical
  per-fold order within each parent, **parallelism *across* the hierarchy** (the C-5/C-6
  OrderBand shape — many parents reducing concurrently), `SlotRange` coalescing, and
  (post-StemThing) contiguous range folds. That is where the real, already-optimized parallelism
  lives, and multiplicative dynamics currently cannot ride it at all.
- Any *intra-fold* tree split of one long chain would require its own explicitly admitted
  reduction/error contract — a separate proof about the reduction algorithm, not a rider on
  exact `LN`/`EXP` primitives. Nothing in this plan requests it.

Softmax is the same lawful shape: stabilized `EXP` map + `MAX` and Sum reduction bands +
normalize — banded sweeps over existing machinery, no new mechanism and no reordering anywhere.

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
  (log pricing) and capacity-depletion curves — authored as gadgets when the phase lands.
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

Two admission plans through the landed 5.7 door, in order — preceded by the one generic door
addition (the sealed `PrimitiveDomain` type + clamp-guard admission, §4): **`EXP` (full domain)
with the CostBand steering + STEAD falloff consumers**, then **`LN` with the log-domain
accumulation + `POW` consumers**. Each landing carries: the pinned sequence and its CPU-oracle twin, the exhaustive
2³² reference artifact, per-backend replay artifacts, the resource-class cost entry, the
`CLOSED_OPCODES` DA-scoped vocabulary change, the gadget-library entries it enables, and the
scan/allowlist co-evolution the doctrine-CI contract requires. No rung is opened by this
document; the admission plans are DA-reserve work that can proceed independently of Phase 6
completion and the StemThing HARD HOLD.

The deferral held the door shut until the substrate could prove what walks through it. The
substrate now can. Completing EML is one landing per primitive — and at the end of it, the
interpreter named for `eml(x, y)` computes it.

## 10. Integration plan — ladder placement (Owner-directed 2026-08-03)

### 10.1 Reopen review — no landed rung reopens, and the reason is the track's own achievement

Every landed rung that *would* have needed reopening in the pre-5.x architecture was reviewed.
None does, because the 5.x remodel converted exactly the surfaces `EXP`/`LN` touch into
**authored data**: new opcodes flow through them without a diff.

| Rung reviewed | Interaction | Reopen? |
|---|---|---|
| 5.5/5.6 field-sweep IR + adjacency | map/fold programs gain vocabulary; exponential falloff laws become authorable | **No** — programs are data; the IR is untouched |
| 5.7 JIT + resource classes | two new opcode lowering arms (straight-line blocks) | **No** — extension within 5.7's designed surfaces, landed as part of the primitive rungs |
| Gu-Yang piecewise `σ(u)` | a smooth logistic saturation becomes authorable | **No** — a **new authored field law beside the old one**, with its own `FieldLawProof`; the landed law and its bit-exact referees are untouched. Replacing the landed law would be a physics change requiring its own ruling — explicitly not proposed |
| 6.1b CostBand steering | `SELECT` staircases upgrade to curves | **No** — authored-data upgrade on the landed surface; the consumer, not a reopen |
| 5.8 comparative projections | none | **No** |

The one rung whose *pending work changes* is **9.1 `GATED-RATES-EML-REWIRE-0`** — a positive
dependency, not a reopen: 9.1 rewrites gate evaluation into the authored EML library stack and
already binds "one library, one cap" (6.1b). If `EXP`/`LN` land **before** 9.1, the gate rewire
authors smooth laws once; if after, staircase forms get authored and then re-authored. **The
primitives must precede 9.1.**

### 10.2 Placement — Phase 5 completion rungs, not a StemThing component

Minted as **`5.10` / `5.11` / `5.12`** (Phase 5's tail is `5.9d`; the EML/field-sweep phase
gains its completion rungs, which is where they belong taxonomically — this is vocabulary work,
not event work, movement work, or memory work):

| Rung | ID | Deliverable | Lane |
|---|---|---|---|
| 5.10 | `EML-PRIMITIVE-DOMAIN-0` | The generic door machinery (§4): sealed `PrimitiveDomain` type; the two admission shapes (range-certified / guarded-semantics) with the semantic distinction enforced; spanned admission errors for unguarded call sites. No primitive admitted by this rung | DA-reserve · Frontier |
| 5.11 | `EML-EXP-PRIMITIVE-0` | Full-domain `EXP` through the door: pinned algorithm + CPU twin, exhaustive 2³² digest per certified toolchain, cost-gate win vs. the gadget baseline, `CLOSED_OPCODES` widening, JIT lowering arm, interpreter arm, scan/allowlist co-evolution. Consumers: CostBand steering curve, STEAD falloff law | Frontier |
| 5.12 | `EML-LN-PRIMITIVE-0` | `LN` by the 5.11 template: same artifacts, same gates. Consumers: `LogAccumulate`, `PowerLaw` (with `EXP`), entropy diagnostics | Std — Grok (template established by 5.11) |

**Not part of StemThing, deliberately.** The doc already rules this work independent of the
HARD HOLD; folding it into StemThing-A would chain two independent capabilities behind the §3.1
Tier-2 ruling for no benefit. The dependency runs the other way and later: StemThing-B's
derivation-pricing curves are a *horizon consumer* of these primitives.

### 10.3 Dispatch window and certification posture

- **Dispatch after 6.3's graduation stamp** — not for design reasons but for merge hygiene:
  6.2b's resolution-site work executes EML in-shader, and landing opcode-vocabulary diffs under
  it risks collisions on the interpreter surfaces. Post-Phase-6, the EML surfaces are quiet.
- **The window is the Phase 6 → 7 gap — SERIAL with StemThing-A implementation (Owner sequencing
  ruling, 2026-08-04, stamped in board comment 5182422593; supersedes the earlier parallel
  default):** the StemThing census/Tier-2 DA work completed inside the gap, but the dispatch
  queue is **5.11 → 5.12 → Owner pointer flip to 6.4**. The EML primitives graduate before
  StemThing-A implementation begins; the hard ordering constraint **before 9.1** stands.
- **Certification is a phase-boundary local act, not CI.** CI runs no cargo tests by standing
  Owner ruling; the exhaustive digest and per-toolchain replay artifacts are produced locally at
  landing, pinned in-repo (the Candidate-F artifact pattern), and the doctrine scans verify
  artifact presence and freshness — never re-execution.

### 10.4 Summary ordering

```
[ Phase 6 completes: 6.2 → 6.2b → 6.3 ]
        │ (6.3 stamp = shared trigger)
        ├── StemThing-A: census → §3.1 Tier-2 ruling → A-rows minted   (DA lane)
        └── 5.10 → 5.11 (EXP) → 5.12 (LN) → [Owner flip] → 6.4        (serial; Owner ruling 2026-08-04)
        │
[ Phase 7 movement (under §7.1 clause) ] → [ 8.1/8.2 ] → [ StemThing-B ]
        │
[ 9.1 gated-rates rewire — CONSUMES the completed vocabulary, authors smooth laws once ]
        → 9.2 → 10.1 (scan reconcile absorbs any residue) → 11.x → 12.x
```

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
