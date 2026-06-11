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
math. SQRT-EXACT-1D then proved the bitmask split beats contraction (D fired 117 corrections
where A fired 0, all normal edges exact) but still lost to FTZ on tiny terms (12 near-min-
normal misses) and subnormal store; SQRT-EXACT-2E proved a `u32` bit-IO contract kills the
flush but its weak integer root regressed accuracy. **SQRT-EXACT-3E then made E's pure-integer
core `max_ulp == 0` on every sweep** — correct and FTZ/contraction-immune by construction, but
its rounding is a **data-dependent integer loop** (~24 iterations) which **cannot meet the
34,000-AI-entity hot-path budget** (op count + warp divergence across 34k lanes). So E3 is the
**slow correctness reference / fallback**, not the production kernel. **The hot-path target is
Candidate F (§5.0): loop-free, fixed-op-count, hardware-`sqrt`-seeded** — D's residual on a
[1,4)-normalized mantissa with `u32` IO, which removes D's FTZ misses and subnormal flush. F is
the shader to build and test standalone. See §5.0 and §11.

**Update (SQRT-EXACT-4E / 4F).** E3 is now exhaustively proven (`ExactDeterministicCandidate`),
and F — built verbatim from §5.0 — passed every sampled/contraction probe, matched E3 on every
tested row, and beat E3 on throughput at 34k. F's own exhaustive run is the single remaining
gate before promotion. So the "exact sqrt vs abandon determinism for scale" dilemma resolves in
favor of *exact **and** fast*. Full numbers and the live next step are in §11.

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

## 5. Candidates (F is the hot-path target — loop-free, hardware-sqrt; E3 is the slow correctness reference)

**Performance requirement is decisive: the target is 34,000 AI entities, evaluated on the hot
path. An integer sqrt cannot meet it.** E3 is correct, but its correctness comes from a
**data-dependent integer loop** — ~24 limb-pair multiply+compare iterations per element. At 34k
lanes that is both a large op-count and a *warp-divergence* problem (lanes finish their search
at different iteration counts, serializing the workgroup). The same objection sinks the F′
hybrid: its ±1 correction is still a divergent integer loop over `compare_sq_to_x`. **The
production `sqrt` must be loop-free, fixed-op-count, and use the hardware sqrt ALU.** That is
**Candidate F**, and it is the build/test target. E3 stays admitted only as the slow, portable
correctness reference / fallback — never the hot-path kernel.

F must be **tested as its own standalone WGSL shader** — its own artifact and battery variant,
*not* folded into E3's integer machinery. Freeze it as
`crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl`, `include_str!` it, and run the
`u32` bit-IO edge/dense/subnormal **and** exhaustive sweeps against `f32::sqrt`, exactly as E
was tested. Authoritative entry `sqrt_cr_f_bits(x_bits: u32) -> u32`; `sqrt_cr_f` is the f32
opcode-boundary wrapper.

### 5.0 Candidate F — standalone WGSL shader (hot-path target)

```wgsl
// Candidate F — correctly-rounded f32 sqrt. Loop-free, fixed op count, hardware-sqrt seed.
//
// Built for the 34k-entity hot path: NO data-dependent loop (uniform cost across all lanes,
// no warp divergence), one hardware `sqrt` + straight-line ops. Correctness strategy: decode
// bits in the integer domain; fold the exponent so the work runs on a significand m in [1,4)
// (sqrt(x) = sqrt(m) * 2^k, k integer); seed with the hardware sqrt; get the EXACT residual
// via a bitmask-split Dekker TwoProduct (no fma); snap to correctly-rounded with a directional
// Markstein test; reconstruct the result exponent with integer ops and return the BIT PATTERN.
// Every live f32 value stays in [1,4) or [1,2) — always normal, so FTZ never touches a
// residual term — and the result is emitted as u32 so no subnormal f32 store/load can flush.
//
// Authoritative entry:  sqrt_cr_f_bits(x_bits: u32) -> u32
// CPU oracle for parity: f32::sqrt (already correctly rounded).
//
// Contraction safety: the split is a bitwise AND the FP optimizer cannot rewrite, and the
// residual has NO `a*b + c` fma candidates — every product is materialized into a `let` and
// `p` is read twice, so naga/DXC cannot fuse it. The snap thresholds may be contracted
// harmlessly (a 1-ULP error there cannot flip a decision; sqrt has no exact ties). The one
// open question the standalone exhaustive sweep must settle is whether DXC reassociates the
// error-term SUM despite the let-sequencing; if it does, the fix is a contraction barrier on
// those ~4 lines — NOT a switch to an integer loop (which would forfeit the 34k budget).

const F_QNAN: u32 = 0x7FC00000u;
const F_PINF: u32 = 0x7F800000u;

// Correctly-rounded sqrt of m in [1,4); result in [1,2]. All intermediates stay normal.
fn sqrt_cr_f_core(m: f32) -> f32 {
    let y0 = sqrt(m);                                       // hardware seed, ~1 ULP, in [1,2)

    // Bitmask split: clear the low 12 mantissa bits. y_hi has 12 significant bits, so
    // y_hi*y_hi is <=24 bits and EXACT in f32. (Integer AND — optimizer cannot touch it.)
    let y_hi = bitcast<f32>(bitcast<u32>(y0) & 0xFFFFF000u);
    let y_lo = y0 - y_hi;

    // Exact residual r = m - y0*y0 (Dekker TwoProduct). No fma candidates anywhere.
    let p           = y0 * y0;          // read twice below -> compiler must materialize it
    let yhi_yhi     = y_hi * y_hi;      // exact
    let yhi_ylo     = y_hi * y_lo;      // exact
    let two_yhi_ylo = yhi_ylo + yhi_ylo;
    let ylo_ylo     = y_lo * y_lo;      // exact
    let e0 = yhi_yhi - p;               // exact (Sterbenz)
    let e1 = e0 + two_yhi_ylo;
    let e  = e1 + ylo_ylo;              // e == y0*y0 - p exactly
    let sp = m - p;                     // exact (Sterbenz)
    let r  = sp - e;                    // EXACT residual m - y0*y0

    // Directional Markstein snap (sqrt has no exact ties -> strict comparison is safe).
    let y_up = bitcast<f32>(bitcast<u32>(y0) + 1u);
    let y_dn = bitcast<f32>(bitcast<u32>(y0) - 1u);
    let u_up = y_up - y0;               // gap above
    let u_dn = y0 - y_dn;               // gap below (half of u_up at a power of two)
    let t_up = (y0 * u_up) + (0.25 * u_up * u_up);
    let t_dn = (y0 * u_dn) - (0.25 * u_dn * u_dn);
    if (r >  t_up) { return y_up; }
    if (r < -t_dn) { return y_dn; }
    return y0;
}

fn sqrt_cr_f_bits(x_bits: u32) -> u32 {
    let sign = x_bits >> 31u;
    let exp  = (x_bits >> 23u) & 0xFFu;
    let mant = x_bits & 0x7FFFFFu;

    // Special values, bit-exact with f32::sqrt.
    if (exp == 0xFFu) {
        if (mant != 0u) { return F_QNAN; }          // NaN -> NaN
        if (sign == 0u) { return F_PINF; }          // +inf -> +inf
        return F_QNAN;                              // -inf -> NaN
    }
    if (x_bits == 0x00000000u) { return 0x00000000u; }   // +0 -> +0
    if (x_bits == 0x80000000u) { return 0x80000000u; }   // -0 -> -0
    if (sign == 1u) { return F_QNAN; }                   // negative finite -> NaN

    // Recover significand M2 in [1,2) (as bits) and unbiased exponent E2. (countLeadingZeros
    // is a fixed-latency intrinsic, not a loop — no divergence.)
    var m2_bits: u32;
    var e2: i32;
    if (exp == 0u) {
        // Subnormal: normalize the leading 1 to the hidden-bit position via integer shift.
        let lz   = countLeadingZeros(mant);          // mant in [1, 2^23-1] -> lz in [9, 31]
        let sh   = lz - 8u;                           // move leading 1 to bit 23
        let frac = (mant << sh) & 0x7FFFFFu;
        m2_bits = 0x3F800000u | frac;
        e2 = -118 - i32(lz);                          // E2 = (31 - lz) - 149
    } else {
        m2_bits = 0x3F800000u | mant;
        e2 = i32(exp) - 127;
    }

    // Fold to an EVEN exponent: x = m * 2^(2k), m in [1,4). sqrt(x) = sqrt(m) * 2^k.
    let k      = e2 >> 1u;                            // floor(E2/2), arithmetic shift
    let parity = bitcast<u32>(e2) & 1u;              // 0 -> m in [1,2); 1 -> m in [2,4)
    let m      = bitcast<f32>(m2_bits) * f32(1u << parity);   // *1 or *2, exact

    let root = sqrt_cr_f_core(m);                     // correctly rounded, in [1,2]

    // Reconstruct: final exponent = root's exponent field + k. Result is always normal
    // (sqrt domain [2^-74.5, 2^64)), so this never overflows/underflows.
    let root_bits = bitcast<u32>(root);
    let final_exp = i32((root_bits >> 23u) & 0xFFu) + k;
    return (u32(final_exp) << 23u) | (root_bits & 0x7FFFFFu);
}

// Opcode-boundary wrapper. PRODUCTION RULE: feed the input column's RAW u32 bits to
// sqrt_cr_f_bits — do NOT load the column as an f32 register first, because DAZ can flush a
// subnormal input on load. Prefer wiring sqrt_cr_f_bits directly to the raw column bits.
fn sqrt_cr_f(x: f32) -> f32 {
    return bitcast<f32>(sqrt_cr_f_bits(bitcast<u32>(x)));
}
```

**Why F is the hot-path kernel (and E3 is not):**
- **Loop-free, uniform cost.** ~1 hardware `sqrt` + ~30 straight-line ops, zero data-dependent
  iteration — identical work on every one of the 34k lanes, no warp divergence. E3 (~24-iter
  search) and F′ (±1 correction loop) both diverge across lanes and inflate op count; neither
  is viable at this scale.
- **FTZ defeated by construction.** All residual work runs on a [1,4)-normalized significand
  (`y_lo² ≈ 2⁻²⁴`, always normal), and the result is emitted as `u32` bits — so neither the
  near-min-normal misses nor the subnormal flush that hit D can recur.
- **Uses the hardware sqrt ALU** rather than re-deriving the root in the integer pipe.

**Test + promote (standalone):** add an F variant (`CorrectlyRoundedHwBitmaskNormalized`) backed
by the standalone `sqrt_cr_f_candidate.wgsl`, with `u32` bit-IO edge/dense/subnormal sweeps and
the `#[ignore]` exhaustive `0x0000_0000..=0x7F7F_FFFF` sweep, all against `f32::sqrt`. F is
promotable when its **own** exhaustive sweep is `max_ulp == 0`; that flips the `sqrt`/`mag`
descriptor to `ExactDeterministic` pointing at the F artifact, with E3 retained only as the slow
portable fallback. The single risk to watch in the sweep is DXC reassociating the error-term
sum; if it appears, fix it with a contraction barrier on those lines — do not fall back to a
loop.

### 5.1 Prior probes (context — superseded by F)

D and B below are the probes F synthesizes. **D** got the residual math right (normal edges
bit-exact, corrections fire) but let tiny terms and subnormal outputs reach FTZ; **B** is the
fully-portable-seed fallback but its Veltkamp split was reassociated by DXC. Neither is the
build target now — F is.

**Shared correctly-rounded snap (directional ULP).** Both prior candidates end with the same
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

### Candidate D — hardware seed + bitmask-split exact residual *(prior probe; matured into F)*

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

> **Gate satisfied — design authority has released the exact-sqrt guardrail (Opus, 2026-05-29).**
> Candidate F passed the full-domain exhaustive proof (SQRT-EXACT-5F: 2,139,095,040 values,
> `max_ulp == 0`, flush 0, ~47 s, hash `e2e9e27601ee2e13`) → `ExactDeterministicCandidate`.
> On that proof, the binding invariant now **accepts the artifact-backed Candidate F as the
> exact hot-path `sqrt` authority** (see `invariants.md` "Exact sqrt authority is artifact-backed
> (Candidate F)"). The release is surgical: authority binds to the F **artifact hash** (any
> change requires renewed proof); native/raw `sqrt`, Candidate D, `mag2`, and Candidate C/f64
> stay non-exact; E3 stays the exact cross-adapter fallback; **no** semantic WGSL, default
> wiring, scheduler/cache, or economy bridge is authorized. The remaining work is the
> mechanical **SQRT-PROMOTE-0** slice: wire the descriptor/admission representation + hash guard
> + tests to this released authority (implementer task, not a further design decision).
> **Update (SQRT-PROMOTE-0 landed):** F is mechanically promoted via descriptor/admission
> (`m_jit_sqrt_f_exact`, hash `e2e9e27601ee2e13`); any F artifact change invalidates authority
> until renewed exhaustive proof; native sqrt remains non-exact.
> **Update (SQRT-MAG-0 landed):** F is consumed by exact Euclidean magnitude candidate
> (`m_jit_mag_f_from_exact_mag2` / `ExactEuclideanMagnitudeFFromExactMag2` when mag2 is exact;
> raw dx/dy probe `m_jit_mag_f_from_dxdy_probe`); F artifact hash remains binding;
> approximate performance mode is still not authorized.
> **Update (SQRT-MAG-0 R1):** F proves sqrt exactness over input bits. It does not by itself prove
> dx²+dy² construction exactness across CPU/GPU operation order.
> **Update (SQRT-MAG2-0):** F sqrt is exact over input bits. Full Euclidean magnitude exactness now
> depends on exact mag2 construction; SQRT-MAG2-0 provides the first fixed-point path if landed.
> **Update (SQRT-MAG2-PERF-0):** F remains exact hot-path sqrt. End-to-end exact magnitude cost is
> dominated by pre-sqrt mag2 construction unless optimized; see SQRT-MAG2-PERF-0.
> **Update (SQRT-REPIN-0, 2026-06-11):** the F artifact hash is re-pinned to
> **`59ab4b2892e3c690`** — the canonical LF repository bytes (5855 B). The original pin
> `e2e9e27601ee2e13` was computed over a CRLF working-tree checkout (5964 B; the archived proof
> log's byte count confirms), so the hash guard failed on every canonical/LF checkout and passed
> only on line-ending-mangled ones. The shader text is character-identical; the full-domain
> exhaustive sweep was **re-run over the canonical artifact at re-pin** (2,139,095,040 values,
> `max_ulp == 0`, flush 0 — addendum in the proof report) so exact authority attaches to the
> canonical identity. `.gitattributes` now pins LF checkout repo-wide, making the hash
> host-stable. Native sqrt remains non-exact; nothing else changes.

```
ApproximateJitOnly            ← native/raw sqrt (stays here — never exact-authoritative)
        │  candidate passes full exhaustive sweep, max_ulp == 0, all rows exact
        ▼
ExactDeterministicCandidate   ← E3 (4E) and F (5F) both proven over the full domain
        │  design-authority release on the proof + artifact-hash pinning
        ▼
ExactDeterministic            ← admission grants exact-output authority to the F ARTIFACT
                                (hash 59ab4b2892e3c690, LF canonical — SQRT-REPIN-0),
                                via SQRT-PROMOTE-0 descriptor wiring
                                **LANDED** — `m_jit_sqrt_f_exact` in landed descriptors
```

- **Outcome (landed):** F is the released exact hot-path authority (loop-free, hardware-seeded,
  beats E3 0.70× at 34k); E3 is the exact cross-adapter fallback. Flipping F's descriptor to
  `ExactDeterministic` is the single admission bit that lets graphs use exact `sqrt` (and true
  Euclidean `mag`) as a score input — replacing the `mag2`-only workaround where exactness is
  needed. That bit is wired by SQRT-PROMOTE-0; nothing in the runtime loosens.
- The §3 guardrail holds for everything else: native/raw `sqrt`, D, and `mag2` stay blocked
  from exact authority; admission keeps rejecting graphs that route them into exact inputs
  unless the value comes through the proven F artifact descriptor.

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

**Status after SQRT-EXACT-0 (#305), SQRT-EXACT-1D, SQRT-EXACT-1D-R1, SQRT-EXACT-2E, SQRT-EXACT-3E, SQRT-EXACT-4E, SQRT-EXACT-4F, and SQRT-EXACT-5F:** A and B were implemented and probed; **both stuck at
≥1 ULP on DX12/naga** (A's correction never fired; B failed normal-range boundaries), and
both flushed subnormals to 0. Root cause is **backend FP contraction/reassociation + flush-
to-zero**, not the underlying math. **Candidate D** was frozen as verbatim WGSL and still
remains `ApproximateJitOnly` (dense normal max ULP = 1; subnormal flush unresolved). **Candidate E**
now runs via integer-domain `u32` bit-IO. 2E removed D-style subnormal flush but was
`RejectedDeferred` (`max_ulp=119`). 3E replaced the weak approximation with a correctly-rounded
integer mantissa core and reached zero ULP on edge/dense/subnormal sweeps. 4E then ran the full
finite non-negative exhaustive sweep (`0x0000_0000..=0x7F7F_FFFF`) with `max_ulp=0`,
`exact_bits=2,139,095,040`, and `flush_count=0`, moving E to `ExactDeterministicCandidate`
pending a separate descriptor/admission promotion slice. **Candidate F** is now built as a
standalone verbatim WGSL artifact with `u32` bit-IO in SQRT-EXACT-4F; edge/dense/subnormal
corpora and F-vs-E3 checks reached `max_ulp=0`, contraction probe was clean on this backend, and
smoke throughput favored F over E3 at 34k rows. **SQRT-EXACT-5F then ran F's full finite
non-negative exhaustive sweep (`0x0000_0000..=0x7F7F_FFFF`) green with `max_ulp=0`,
`exact_bits=2,139,095,040`, and `flush_count=0`, moving F to `ExactDeterministicCandidate`
pending a separate descriptor/admission promotion slice.**

1. **E3 is the proven exact candidate.** Candidate E now has full-domain finite non-negative
   proof at `max_ulp == 0` with zero flush. Production authority flip remains a separate explicit
   admission/descriptors pass.
2. **Candidate F is now exhaustively proven as an exact hot-path candidate.** F is implemented
   as standalone verbatim WGSL (`sqrt_cr_f_candidate.wgsl`) with authoritative `u32` bit IO and
   now classifies as `ExactDeterministicCandidate` for the finite non-negative domain gate.
3. **`fma` shortcut is still dead on this backend.** #305 confirms naga/DX12 does not fuse `fma`
   for residual purposes. Any future fast-path work must preserve bit-exact proof obligations and
   avoid reintroducing unproven `fma` assumptions.
4. **Exhaustive gate is satisfied for both E3 and F.** Both ignored 2^31 finite non-negative
   gates have executed green with zero ULP in their respective slices.
5. **Promotion remains gated at admission.** Exact-candidate status does not itself change runtime
   authority; descriptor/admission flip is intentionally separate.
6. **`mag` re-enablement.** Once `sqrt` is `ExactDeterministic`, open the follow-on to let
   true Euclidean `mag` replace `mag2` where exactness (not just ordering) is required —
   separate slice, separate row.

---

*This track exists to let `sqrt` **earn** exact authority through proof, not assertion.
Until it does, the guardrail holds at admission and the sim never sees an unproven exact
`sqrt`. When it does, exactly one admission bit changes and the runtime is untouched.*
