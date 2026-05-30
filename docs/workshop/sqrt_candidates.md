# Deterministic Bit-Exact GPU `sqrt` — Candidate Design Note

> **Status:** Active design note (2026-05-29).
> **Design authority:** Opus 4.8, delegated by project owner this session.
> **Track:** "Shader/software deterministic `sqrt` path" — **follow-on track #1** spun
> off from the Phase M-JIT closure handoff. This is **not** a JIT-closure blocker; the
> M-JIT track closes at the default-off production registry shell boundary with native
> `sqrt` still **not exact-authoritative**. This note opens the separate track that makes
> a bit-exact `sqrt` available as an `ExactDeterministic` kernel.
> **Gate:** opening the exact-`sqrt` authority contract is **Tier-2** (new exact-output
> kernel + admission rule). Promoting a candidate that passes the exhaustive proof is
> **Tier-1** (within the accepted contract, generic, opt-in, oracle-parity-backed,
> reversible).
>
> **Companions:** `design_v7_7.md` §5 (gating), [`../invariants.md`](../invariants.md) (I8 bit-exact parity;
> Mapping/JIT exact-authority rows), [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md),
> [`../tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md`](../tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md) (native sqrt blocker evidence),
> [`../tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md`](../tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md) (`mag2` classification),
> [`../tests/phase_m_jit_prod0_registry_shell_test_results.md`](../tests/phase_m_jit_prod0_registry_shell_test_results.md) (M-JIT closure authority),
> [`mapping_current_guidance.md`](mapping_current_guidance.md) (active status table).

---

## 1. Scope and relationship to M-JIT closure

The M-JIT track is accepted as closed at the PROD-0 default-off registry-shell
boundary. Among its **binding guardrails**: *native `sqrt` is not exact-authoritative*
and *approximate `mag2` cannot feed exact score inputs.* Those guardrails stay live and
are **not** diluted by this track. This note does the opposite of diluting them: it
defines the only path by which `sqrt` could ever **earn** exact authority — an
exhaustive bit-exactness proof — and keeps the guardrail enforced at admission until
that proof exists.

Nothing here authorizes a scheduler, a runtime cache, default `SimSession` wiring, a
production economy→mapping bridge, semantic WGSL, or any `simthing-sim` awareness.

## 2. Problem: correctly-rounded `sqrt` vs native WGSL `sqrt`

The SQRT-0 battery established the precise failure: native WGSL `sqrt(x)` on the naga →
DX12 backend is **max ULP = 1** against the CPU oracle (`f32::sqrt`), so the candidate is
pinned `ApproximateJitOnly`.

This is IEEE-754 semantics, not a bug:

- **CPU `f32::sqrt` is correctly rounded.** IEEE-754 *mandates* `sqrt` be correctly
  rounded; x86 `SQRTSS` / ARM `FSQRT` deliver the correctly-rounded result.
- **WGSL `sqrt()` is not guaranteed correctly rounded.** Drivers implement it as
  `x·rsqrt(x)` or a seed-polynomial + Newton steps landing within ~1 ULP. Vendor
  precision differs (NVIDIA/AMD/Intel/Apple).

Therefore **"bit-exact with the CPU" ≡ "correctly-rounded f32 `sqrt`."** Produce the
correctly-rounded result in WGSL and it matches the CPU bit-for-bit by definition.

Two structural facts make this tractable and let us *prove* rather than *sample*:

1. **The f32 domain is small enough to test exhaustively.** There are ~2³¹ non-negative
   finite f32 values. We can dispatch every one and assert `max_ulp == 0` over the whole
   input space — not a corpus, the entire domain.
2. **`sqrt` has no exact ties.** A known theorem of binary floating point: the square
   root of a float is *never* exactly halfway between two adjacent floats. So
   round-to-nearest-**even** never actually fires for `sqrt`; the rounding decision is
   always strict and unambiguous. This removes the trickiest correctness corner.

**Empirical status (SQRT-EXACT-0, #305).** Candidates A (`fma` residual) and B (Veltkamp
Newton) were built and probed on DX12/naga and **both stuck at ≥1 ULP** — A's correction
never fired, B failed normal-range boundaries, and both flushed subnormals to 0. The lesson
is sharp: the adversary is **backend FP contraction/reassociation + flush-to-zero**, not the
math. That is exactly why the lead candidate (**D**, §5) does its splitting with an integer
**bitmask** the FP optimizer cannot rewrite. See §5 and §10.

## 3. Guardrail doctrine — exactness authority lives at the designer/spec-admission layer

**This is the load-bearing principle of the track, and it is where the safety lives.**
Bad numerics must be rejected *before* they reach the sim, not clamped after. Concretely:

- **Output-authority is a spec-admission property, not a runtime hope.** A kernel's
  output is `ExactDeterministic` only if it carries an admission-verified exactness
  proof. `sqrt` carries **no** such authority today and is admitted only as
  `ApproximateJitOnly`.
- **`validate_exact_inputs` is the firewall.** Production registry and kernel-graph admission
  (`validate_exact_kernel_inputs` / `validate_exact_inputs` in `simthing-spec`) already reject
  an approximate output wired into an exact input. A graph that feeds native `sqrt` (or `mag2`)
  into an exact score is **rejected at session build**, with a diagnostic, before any GPU dispatch.
  The sim never sees the bad edge.
- **Promotion changes one admission bit, nothing downstream.** When a candidate here is
  exhaustively proven, the *only* change is that this specific kernel's descriptor flips
  to `ExactDeterministic` output authority. No runtime path is loosened; admission
  simply stops rejecting graphs that use the proven kernel as an exact source.
- **The runtime remains the unconditional last line.** Even with admission correct, the
  kernel still validates finiteness/column ranges at runtime. Authoring is never trusted
  to have been safe; admission is the *first* rejection, runtime is the *last*.

So the deliverable of this track is not merely "a faster sqrt" — it is **a kernel whose
exact-output authority the admission layer can prove and therefore safely permit**, with
the guardrail against unproven `sqrt` staying fully intact for everything else.

## 4. The shared math

Given an approximation `y ≈ √x` accurate to < 1 ULP, the **exact residual** `r = x − y²`
decides correct rounding. With `u = ulp(y)`, the Markstein round-to-nearest criterion is:

```
round up to (y + u)   iff   r  >  y·u + u²/4
round down to (y − u) iff   r  < −(y·u + u²/4)
otherwise y is already correctly rounded
```

`y·u` is the half-ULP boundary mapped into the squared domain; `u²/4` is the exact-tie
term. Because **`sqrt` has no exact ties** (§2), `r` never equals the boundary, so the
comparison is always strict and the `u²/4` term only guards against representational
edge effects — round-to-even never has to be resolved.

The entire correctness question reduces to **computing `r = x − y²` exactly.** Two ways,
and that choice is what separates the candidates:

- **Exact residual via a Dekker TwoProduct split — trust nothing.** Split `y` into two
  ≤12-bit halves so `y²` is computed exactly, then `r = (s−p)−e`. The split is either a
  **bitmask** of the low 12 mantissa bits (**Candidate D**, primary — one AND, no `fma`, no
  divide) or a **Veltkamp `4097·y` multiply** (**Candidate B**, the portable fallback). Both
  are exact and assume nothing about `fma`.
- **`fma` shortcut (micro-opt only):** where the backend's `fma` is confirmed single-
  rounding, the residual collapses to `r = fma(-y, y, x)`. This was the original Candidate A;
  it is now just an optional fast form of D's residual, not a separate candidate.

## 5. The two candidates (D primary, B fallback)

**Shared correctly-rounded snap (directional ULP).** Both candidates end with the same
Markstein correction. The threshold must use the **directional** ULP — the gap above `y`
for an up-snap, the gap below for a down-snap — because at an exact power of two those gaps
differ by 2× and the tie term flips sign. `sqrt` has no exact ties, so the `y·u` term
dominates and the `u²/4` term is only a guard, but the exhaustive sweep demands the
directional form be correct at power-of-two boundaries:

```wgsl
fn snap_cr(y: f32, r: f32) -> f32 {   // r = exact residual (s - y*y)
    let u_up = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);   // gap to next-up
    let u_dn = abs(y - bitcast<f32>(bitcast<u32>(y) - 1u));   // gap to next-down (½ at pow2)
    if      (r >   (y * u_up + 0.25 * u_up * u_up)) { return bitcast<f32>(bitcast<u32>(y) + 1u); }
    else if (r < -(y * u_dn - 0.25 * u_dn * u_dn)) { return bitcast<f32>(bitcast<u32>(y) - 1u); }
    return y;
}
```

### Candidate D — hardware seed + bitmask-split exact residual *(primary; recommended)*

Contributed and adopted as the lead. It keeps the within-1-ULP hardware `sqrt` seed (no
Newton), and computes the **exact** residual with a single bitwise mask — no `fma`-fusion
assumption, no Veltkamp multiply, no division.

```wgsl
fn sqrt_cr_d(x: f32) -> f32 {
    if (!(x > 0.0)) { return sqrt(x); }                 // 0, -0, NaN, x<0, +inf → native passthrough

    // Subnormal guard: scale by 2^24 (exact), unscale by 2^-12. sqrt(subnormal) is always
    // normal, so the unscale never underflows. (Single multiply; see the fix note below.)
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) { s = x * 1.6777216e7; scale = 1.0 / 4096.0; }  // 2^24 → 2^-12

    var y = sqrt(s);                                     // within 1 ULP (hardware/driver)

    // Bitmask split: clear the low 12 mantissa bits. y_hi keeps 11 explicit + 1 hidden
    // = exactly 12 significant bits, so y_hi*y_hi is ≤24 bits → EXACT in f32.
    let y_hi = bitcast<f32>(bitcast<u32>(y) & 0xFFFFF000u);
    let y_lo = y - y_hi;                                 // exact (leading-bit cancellation)

    // Dekker TwoProduct → exact residual r = s - y*y, with no fma and no division.
    let p = y * y;                                       // rounded product
    let e = ((y_hi * y_hi - p) + 2.0 * y_hi * y_lo) + y_lo * y_lo;  // y*y == p + e exactly
    let r = (s - p) - e;                                 // EXACT residual

    return snap_cr(y, r) * scale;
}
```

- **Cost:** 1 hardware `sqrt` + ~10 ALU (one AND, a handful of mul/add). No `fma`, no divide.
- **Why it's the lead — now empirically motivated.** SQRT-EXACT-0 (#305) tested A and B on
  DX12/naga and **both stuck at ≥1 ULP**: A's residual correction *never fired*
  (`correction_count == 0`), and B failed normal-range rounding boundaries. The cause is the
  backend's **FP contraction/reassociation** (and effectively non-fused `fma`) silently
  rewriting the residual arithmetic that A and B depend on. D's split is a **bitwise AND on
  the bit pattern** — the FP optimizer *cannot* reassociate or contract it — so `y_hi` is
  genuinely 12 bits regardless of compiler behavior. D attacks the exact failure mode that
  killed A and B.
- **Exactness proof:** `0xFFFFF000` keeps 12 significant bits in `y_hi`; its significand is
  a multiple of 2⁻¹¹ in [1,2), so `y_hi²` is a multiple of 2⁻²² in [1,4) — ≤24 bits, exact.
  `y_lo`, `y_lo²`, `2·y_hi·y_lo` are each ≤24-bit and exact. Dekker then gives `e = y²−p`
  exactly; `s−p` is exact by Sterbenz; so `r = (s−p)−e` is the exact residual. No overflow:
  `y = √s ≤ ~1.8e19 ≪ f32::MAX`.
- **Two hardening requirements surfaced by #305 — mandatory for D:**
  1. **Contraction barrier on the residual.** The integer split is safe, but the Dekker
     error term `e = ((y_hi*y_hi − p) + …)` is still FP and *will* be contracted/reassociated
     by DXC unless prevented (this is what defeated B's Veltkamp split). WGSL `let` bindings
     are necessary but proved insufficient under naga→DXC; D must verify, per generated HLSL,
     that no `mad`/reassociation collapses the error term — and if naga offers no contraction
     control, fall back to **integer-domain residual** (compute `y_hi²`, `2·y_hi·y_lo`,
     `y_lo²` and the subtraction as exact integer mantissa ops, which the FP optimizer cannot
     touch at all). This is the single most important thing to get right.
  2. **Integer subnormal normalization (not FP scaling).** DX12 runs **flush-to-zero**, so
     `s = x * 2^24` flushes a subnormal *input* `x` to 0 before the multiply (this is why A
     and B both returned `0.0` on the subnormal edge rows). The subnormal path must normalize
     `x` by **integer bit manipulation** of its mantissa/exponent, not an FP multiply.
- **Seed assumption (shared with A):** D checks only the ±1 neighbor, so it relies on the
  native `sqrt` seed being within 1 ULP (true on the SQRT-0 backend). If a future adapter's
  `sqrt` is worse, D needs no new algorithm — it already has the exact `r`, so one Newton
  step `y += 0.5 * r / y` before the snap widens the seed tolerance for free.
- **Candidate A (absorbed, now empirically dead on this backend):** A was "hardware seed +
  `r = fma(-y,y,x)` + snap." #305 showed its correction never fires on DX12/naga (non-fused
  `fma`), so it is **not** a separate live candidate — D's bitmask residual is the fix. The
  `fma` shortcut may only be re-enabled if a backend is *proven* to fuse.

### Candidate B — self-contained Newton + TwoProduct residual *(portable fallback)*

No dependence on hardware-transcendental precision **and** no dependence on `fma` fusion —
the only candidate whose *seed* is also backend-independent. Integer bit-trick seed → Newton
refinement → exact residual via Veltkamp split. Kept as the fallback for any adapter whose
native `sqrt` exceeds 1 ULP (where D's hardware seed assumption fails).

```wgsl
fn two_prod_resid(y: f32, x: f32) -> f32 {
    // exact (x - y*y) via Dekker TwoProduct; no fma required.
    let c  = 4097.0 * y;            // splitter 2^12 + 1 for f32 (24-bit mantissa)
    let yh = c - (c - y);           // high 12 bits
    let yl = y - yh;                // low part
    let p  = y * y;                 // rounded product
    let e  = ((yh * yh - p) + 2.0 * yh * yl) + yl * yl;  // y*y == p + e exactly
    return (x - p) - e;             // exact residual
}

fn sqrt_cr_b(x: f32) -> f32 {
    if (!(x > 0.0)) { return sqrt(x); }
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) { s = x * 1.6777216e7; scale = 1.0 / 4096.0; }  // 2^24 → 2^-12 (fixed)
    var y = bitcast<f32>(0x1fbd1df5u + (bitcast<u32>(s) >> 1u));  // ~3-bit sqrt seed
    y = 0.5 * (y + s / y);          // Newton step 1
    y = 0.5 * (y + s / y);          // Newton step 2
    y = 0.5 * (y + s / y);          // Newton step 3 → < 1 ULP
    let r = two_prod_resid(y, s);
    return snap_cr(y, r) * scale;
}
```

- **Cost:** 3 Newton divides + ~12 ALU. Still trivial at our scale.
- **Why it matters:** the **strongest cross-adapter / replay guarantee** — every step is
  plain IEEE f32 add/mul/div (all correctly-rounded and portable) with no reliance on
  vendor `sqrt`/`rsqrt` or `fma` fusion, *including the seed*. Survives a future portability
  or replay-determinism audit even on adapters where native `sqrt` is poor.
- **Care points:** the seed constant `0x1fbd1df5` is a Newton seed only (refined away);
  subnormal scaling is the corrected single-`2^24` form; `4097*y` cannot overflow because
  `y = √x ≤ ~1.8e19 ≪ f32::MAX`.

> **Subnormal-scale fix (applies to D and B).** An earlier draft scaled the input by `2^48`
> but unscaled by `2^-12`, a factor-`2^12` error. The correct pairing is a single `2^24`
> input scale with a `2^-12` unscale (`sqrt(x·2^24) = sqrt(x)·2^12`). `2^24` already lifts
> every subnormal into the normal range, and `sqrt` of any subnormal is normal, so the
> `2^-12` unscale never underflows.

> **Rejected (do not test): f64 compute-then-round, `f32(sqrt(f64(x)))`.** It is provably
> correctly-rounded (Figueroa double-rounding theorem), but useless to us: WebGPU has no
> `f64` and naga's `SHADER_F64` support is partial/non-portable, so it can never be the
> production path; and as a cross-check it adds nothing the Rust `f32::sqrt` oracle does
> not already provide. Eliminated up front so no effort is spent implementing or testing it.

## 6. Special-value handling (must match CPU `f32::sqrt` bit-for-bit)

| Input | CPU `f32::sqrt` result | Candidate handling |
|---|---|---|
| `+0.0` | `+0.0` | native passthrough (`!(x>0)` branch) |
| `-0.0` | `-0.0` | native passthrough |
| `x < 0` (incl. `-inf`) | `NaN` | native passthrough |
| `NaN` | `NaN` (quiet) | native passthrough |
| `+inf` | `+inf` | native passthrough (`x>0` true, but `sqrt(inf)=inf`, residual path is inert) |
| smallest subnormal … `f32::MAX` | correctly-rounded | D & B: exact `2^24`/`2^-12` scale guard; correction runs in normal range |

The `!(x > 0.0)` guard funnels every non-normal-positive input to native `sqrt`, whose
behavior on `0/-0/NaN/neg` already matches the CPU. Correction logic only runs on finite
positive inputs.

## 7. Verification — exhaustive, not sampled

A new test battery `phase_m_jit_sqrt_exact_candidate_battery.rs` (test-only, mirroring the
existing SQRT-0 harness and its `FORBIDDEN_SEMANTIC_TERMS` scan):

1. **Two candidate variants** (`CorrectlyRoundedHwBitmask` = D, primary;
   `CorrectlyRoundedNewtonTwoProduct` = B, fallback) emitted as semantic-free WGSL. The
   `fma`-shortcut residual is a per-variant toggle on D, not a third variant.
2. **Oracle:** Rust `f32::sqrt` (correctly rounded). Compare GPU `to_bits()` vs oracle
   `to_bits()`.
3. **Exhaustive sweep:** dispatch every finite non-negative f32 bit pattern in batches
   (`0x0000_0000 … 0x7F7F_FFFF`), assert `max_ulp == 0` over the entire domain. This is
   the step that earns `ExactDeterministic` honestly.
4. **Explicit edge rows** asserted by name: `+0`, `-0`, smallest/largest subnormal, `1.0`,
   exact perfect squares, values straddling rounding boundaries, `f32::MAX`, `+inf`, `NaN`,
   negatives → all bit-identical to the oracle.
5. **`fma`-fusion probe (for A):** record whether the residual-based correction ever fires
   and whether any input remains off-by-one; a non-fused `fma` surfaces as nonzero ULP and
   fails A automatically.

## 8. Promotion gate / classification ladder

```
ApproximateJitOnly            ← native sqrt today (blocked from exact authority)
        │  candidate passes full exhaustive sweep, max_ulp == 0, all edge rows exact
        ▼
ExactDeterministicCandidate   ← proven on this backend/corpus; cross-adapter not yet shown
        │  (B) portability argument holds OR (D) confirmed on each target adapter
        ▼
ExactDeterministic            ← admission may grant exact-output authority to THIS kernel
```

- The first candidate that is exhaustively `max_ulp == 0` with all edge rows exact is
  reclassified and its DESC descriptor flips to `ExactDeterministic` output authority.
  That single bit is what lets admission permit `sqrt` (and true Euclidean `mag`) as an
  exact score input — replacing the `mag2`-only workaround where exactness is needed.
- **Expected outcome:** D passes and promotes fastest (Tier-1 once the exact-`sqrt`
  contract is accepted) — fast hardware seed, exact bitmask residual, no `fma` trust. B is
  retained as the portability/replay-hardened fallback and is the preferred default where
  cross-adapter determinism (including a poor native `sqrt` seed) must be guaranteed.
- Until a candidate is proven, the §3 guardrail is unchanged: native `sqrt` and `mag2`
  stay blocked from exact authority and admission keeps rejecting graphs that violate it.

## 9. Where the promoted kernel lives

A promoted candidate **must** be WGSL — the runtime evaluates on GPU, so shader text is the
only place it can execute. There is no Rust/CPU runtime path for it; the CPU side appears
only as the *oracle* at test time. Concretely:

- **GPU side — a fixed, audited WGSL intrinsic.** The correctly-rounded routine lives as a
  generic semantic-free WGSL function (`fn sqrt_cr(x: f32) -> f32 { … }`) that the EvalEML
  `sqrt`/`mag` opcode **lowers to** — included verbatim in the emitted/runtime shader wherever
  the opcode appears. The lowering chain is: `EvalEML sqrt opcode → audited sqrt_cr WGSL
  function → shader text`.
- **Current canonical text under test (SQRT-EXACT-1D-R1).** Candidate D is now frozen as a
  standalone test artifact at
  `crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl`, consumed by the battery via
  `include_str!` (`SQRT_CR_D_WGSL`). Batch/probe wrappers prepend this artifact verbatim and do
  not regenerate D from Rust helper text.
- **It is a frozen intrinsic, not freely re-composed text.** D's correctness depends on the
  exact operation ordering surviving naga → DXC; uncontrolled re-emission would let the
  compiler reassociate/contract the Dekker error term and break exactness — exactly the
  failure mode that killed A and B in #305. So the proven kernel is referenced as a
  **known-good, contraction-audited primitive** (verified at the generated-HLSL level), never
  reconstructed per formula by general EvalEML emission.
- **Future promoted location rule unchanged.** If/when D (or successor) reaches exhaustive
  `max_ulp == 0`, production lowering must reference the audited intrinsic verbatim from its
  accepted runtime location; promotion does not authorize expression recomposition.
- **CPU side stays trivial.** The oracle is plain `f32::sqrt` (already correctly rounded).
  The CPU does **not** reimplement Markstein/bitmask; the entire parity claim is
  "GPU `sqrt_cr` bits == `f32::sqrt` bits."
- **Until promotion, it stays out of baseline shaders.** Today it is test-only WGSL in the
  battery; `accumulator_op.wgsl` remains `sqrt`-free. On promotion, the *only* runtime change
  is that the `sqrt`/`mag` opcode's descriptor flips to `ExactDeterministic` and points at the
  audited intrinsic — admission then stops rejecting graphs that use it as an exact source.

## 10. Constitutional posture

Generic semantic-free numeric kernel; opt-in; CPU-oracle parity (exhaustively proven, not
sampled); no new GPU *primitive* beyond an EvalEML opcode/kernel mapping; no scheduler, no
cache, no default `SimSession` wiring, no economy→mapping bridge, no `simthing-sim`
awareness, no semantic WGSL. Exactness authority is granted only at the spec-admission
layer and only on exhaustive proof; the runtime remains the unconditional last line.

## 11. Open decisions (design authority — Opus)

**Status after SQRT-EXACT-0 (#305), SQRT-EXACT-1D, SQRT-EXACT-1D-R1, SQRT-EXACT-2E, and SQRT-EXACT-3E:** A and B were implemented and probed; **both stuck at
≥1 ULP on DX12/naga** (A's correction never fired; B failed normal-range boundaries), and
both flushed subnormals to 0. Root cause is **backend FP contraction/reassociation + flush-
to-zero**, not the underlying math. **Candidate D** was frozen as verbatim WGSL and still
remains `ApproximateJitOnly` (dense normal max ULP = 1; subnormal flush unresolved). **Candidate E**
now runs via integer-domain `u32` bit-IO. 2E removed D-style subnormal flush but was
`RejectedDeferred` (`max_ulp=119`). 3E replaced the weak approximation with a correctly-rounded
integer mantissa core and now reaches zero ULP on edge/dense/subnormal sweeps in the battery,
moving E to `ExactCandidatePendingExhaustiveSweep` pending full ignored exhaustive proof.

1. **D landed as lead candidate probe** with contraction barrier (split helpers) and integer
   subnormal *input* normalization. Classification: **ApproximateJitOnly** on this backend;
   subnormal output path **unresolved**.
2. **Contingency — Candidate E (integer-only).** Implemented and remediated through SQRT-EXACT-2E
   and SQRT-EXACT-3E with verbatim WGSL `sqrt_cr_e_bits(x_bits: u32) -> u32` and authoritative
   `array<u32>` harness. The 3E core uses `u32` limb-pair widened arithmetic with exact integer
   nearest-even rounding, removing 2E's dense-normal error and yielding `max_ulp=0` in current
   edge/dense/subnormal sweeps. Promotion is still gated on ignored exhaustive sweep.
3. **`fma` shortcut is dead on this backend.** #305 confirms naga/DX12 does not fuse `fma`
   for residual purposes — do not spend further effort on the A/`fma` form unless a *different*
   adapter is proven to fuse. Recorded as a binding backend note.
4. **Exhaustive-sweep budget.** The full 2³¹ sweep exists as an `#[ignore]` gate in the
   battery; promotion still requires running it to `max_ulp == 0`. Confirm it batches within
   GPU time when D is ready; the dense stratified sweep stays the default-run signal.
5. **`mag` re-enablement.** Once `sqrt` is `ExactDeterministic`, open the follow-on to let
   true Euclidean `mag` replace `mag2` where exactness (not just ordering) is required —
   separate slice, separate row.

---

*This track exists to let `sqrt` **earn** exact authority through proof, not assertion.
Until it does, the guardrail holds at admission and the sim never sees an unproven exact
`sqrt`. When it does, exactly one admission bit changes and the runtime is untouched.*
