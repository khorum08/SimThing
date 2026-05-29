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
> **Companions:** `design_v7_7.md` §5 (gating), `invariants.md` (I8 bit-exact parity;
> Mapping/JIT exact-authority rows), `adr/mapping_sparse_regioncell.md`,
> `tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md` (the blocker evidence),
> `tests/phase_m_jit_desc0_kernel_descriptor_test_results.md` (exact-vs-approximate
> output authority machinery this track feeds).

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

## 3. Guardrail doctrine — exactness authority lives at the designer/spec-admission layer

**This is the load-bearing principle of the track, and it is where the safety lives.**
Bad numerics must be rejected *before* they reach the sim, not clamped after. Concretely:

- **Output-authority is a spec-admission property, not a runtime hope.** A kernel's
  output is `ExactDeterministic` only if it carries an admission-verified exactness
  proof. `sqrt` carries **no** such authority today and is admitted only as
  `ApproximateJitOnly`.
- **`validate_exact_inputs` is the firewall.** The DESC-0/DESC-1 descriptor machinery
  already rejects an approximate output wired into an exact input. A graph that feeds
  native `sqrt` (or `mag2`) into an exact score is **rejected at session build**, with a
  diagnostic, before any GPU dispatch. The sim never sees the bad edge.
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

- **Trust a truly-fused `fma`:** `r = fma(-y, y, x)` is exact in one instruction *iff*
  the backend's `fma` is single-rounding (Candidate A).
- **Trust nothing — compute `y²` exactly with a Veltkamp/Dekker TwoProduct split**
  (Candidate B). Portable across every adapter and the CPU with no fusion assumption.

## 5. Candidate A — hardware seed + one FMA residual correction *(cheapest; test first)*

Build on the within-1-ULP native `sqrt`, then snap to correctly-rounded:

```wgsl
fn sqrt_cr_a(x: f32) -> f32 {
    if (!(x > 0.0)) { return sqrt(x); }                 // 0, -0, NaN, x<0, +inf → native passthrough
    let y = sqrt(x);                                     // within 1 ULP (hardware/driver)
    let r = fma(-y, y, x);                               // EXACT residual iff fma is fused
    let u = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y); // ulp(y) via next-up neighbor
    let b = fma(y, u, 0.25 * u * u);                     // y·u + u²/4
    if (r >  b) { return bitcast<f32>(bitcast<u32>(y) + 1u); }
    if (r < -b) { return bitcast<f32>(bitcast<u32>(y) - 1u); }
    return y;
}
```

- **Cost:** 1 hardware `sqrt` + ~6 FMA/ALU ops. Negligible.
- **Why it likely wins first:** the hardware result is already correct on most inputs;
  this fixes the ±1-ULP stragglers and proves the rest.
- **Risk to validate:** whether naga lowers WGSL `fma` to a true single-rounding fused op
  on DX12 (SM5+ `fma` intrinsic should be fused). The exhaustive sweep detects a
  non-fused `fma` instantly — a two-rounding `fma` loses bits in the residual and the
  corrections misfire, so the sweep would report nonzero ULP. **If A passes the sweep, A
  is done and `fma` fusion is confirmed on this backend as a side effect.**

## 6. Candidate B — self-contained Newton + TwoProduct residual *(most portable)*

No dependence on hardware-transcendental precision **and** no dependence on `fma` fusion.
Integer bit-trick seed → Newton refinement → exact residual via Veltkamp split:

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
    // Subnormal guard: scale by an even power of two (exact), sqrt, unscale.
    var s = x;
    var scale = 1.0;
    if (x < 1.1754944e-38) { s = x * 1.6777216e7 * 1.6777216e7; scale = 1.0 / 4096.0; } // 2^48 / 2^-24
    var y = bitcast<f32>(0x1fbd1df5u + (bitcast<u32>(s) >> 1u));  // ~3-bit sqrt seed
    y = 0.5 * (y + s / y);          // Newton step 1
    y = 0.5 * (y + s / y);          // Newton step 2
    y = 0.5 * (y + s / y);          // Newton step 3 → < 1 ULP
    let r = two_prod_resid(y, s);
    let u = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
    let b = (y * u) + (0.25 * u * u);
    if (r >  b) { y = bitcast<f32>(bitcast<u32>(y) + 1u); }
    else if (r < -b) { y = bitcast<f32>(bitcast<u32>(y) - 1u); }
    return y * scale;
}
```

- **Cost:** 3 Newton divides + ~12 ALU. Still trivial at our scale.
- **Why it matters:** the **strongest cross-adapter / replay guarantee** — every step is
  plain IEEE f32 add/mul/div (all correctly-rounded and portable) with no reliance on
  vendor `sqrt`/`rsqrt` or `fma` fusion. This is the candidate that survives a future
  portability or replay-determinism audit.
- **Care points:** the seed constant `0x1fbd1df5` is a Newton seed only (refined away);
  subnormal inputs use exact power-of-two scaling; the `4097*y` split cannot overflow
  because `y = √x ≤ ~1.8e19 ≪ f32::MAX`.

## 7. Candidate C — f64 compute-then-round *(oracle + opportunistic fast path)*

Where the adapter exposes the native `SHADER_F64` feature:

```wgsl
// requires native f64 feature; unavailable on WebGPU and many adapters
fn sqrt_cr_c(x: f32) -> f32 { return f32(sqrt(f64(x))); }
```

- **Provably correctly-rounded** by Figueroa's double-rounding theorem: a correctly-rounded
  f64 `sqrt` rounded to f32 equals the correctly-rounded f32 `sqrt`, because f64 carries
  far more than 2 × 24 bits — no double-rounding hazard.
- **Not portable** (no WebGPU f64; naga f64 support is partial), so it cannot be *the*
  production path. It is the **cleanest exactness oracle** to cross-check A and B on
  capable hardware, and a legitimate feature-gated fast path on native adapters that
  expose f64.

## 8. Special-value handling (must match CPU `f32::sqrt` bit-for-bit)

| Input | CPU `f32::sqrt` result | Candidate handling |
|---|---|---|
| `+0.0` | `+0.0` | native passthrough (`!(x>0)` branch) |
| `-0.0` | `-0.0` | native passthrough |
| `x < 0` (incl. `-inf`) | `NaN` | native passthrough |
| `NaN` | `NaN` (quiet) | native passthrough |
| `+inf` | `+inf` | native passthrough (`x>0` true, but `sqrt(inf)=inf`, residual path is inert) |
| smallest subnormal … `f32::MAX` | correctly-rounded | A: native seed handles subnormals; B: exact power-of-two scale guard |

The `!(x > 0.0)` guard funnels every non-normal-positive input to native `sqrt`, whose
behavior on `0/-0/NaN/neg` already matches the CPU. Correction logic only runs on finite
positive inputs.

## 9. Verification — exhaustive, not sampled

A new test battery `phase_m_jit_sqrt_exact_candidate_battery.rs` (test-only, mirroring the
existing SQRT-0 harness and its `FORBIDDEN_SEMANTIC_TERMS` scan):

1. **Three candidate variants** (`CorrectlyRoundedHwFma`, `CorrectlyRoundedNewtonTwoProduct`,
   `F64RoundDown`) emitted as semantic-free WGSL.
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

## 10. Promotion gate / classification ladder

```
ApproximateJitOnly            ← native sqrt today (blocked from exact authority)
        │  candidate passes full exhaustive sweep, max_ulp == 0, all edge rows exact
        ▼
ExactDeterministicCandidate   ← proven on this backend/corpus; cross-adapter not yet shown
        │  (B) portability argument holds OR (A/C) confirmed on each target adapter
        ▼
ExactDeterministic            ← admission may grant exact-output authority to THIS kernel
```

- The first candidate that is exhaustively `max_ulp == 0` with all edge rows exact is
  reclassified and its DESC descriptor flips to `ExactDeterministic` output authority.
  That single bit is what lets admission permit `sqrt` (and true Euclidean `mag`) as an
  exact score input — replacing the `mag2`-only workaround where exactness is needed.
- **Expected outcome:** A passes and promotes fastest (Tier-1 once the exact-`sqrt`
  contract is accepted). B is retained as the portability/replay-hardened variant and is
  the preferred default if cross-adapter determinism is required. C is oracle + native
  fast path.
- Until a candidate is proven, the §3 guardrail is unchanged: native `sqrt` and `mag2`
  stay blocked from exact authority and admission keeps rejecting graphs that violate it.

## 11. Constitutional posture

Generic semantic-free numeric kernel; opt-in; CPU-oracle parity (exhaustively proven, not
sampled); no new GPU *primitive* beyond an EvalEML opcode/kernel mapping; no scheduler, no
cache, no default `SimSession` wiring, no economy→mapping bridge, no `simthing-sim`
awareness, no semantic WGSL. Exactness authority is granted only at the spec-admission
layer and only on exhaustive proof; the runtime remains the unconditional last line.

## 12. Open decisions (design authority — Opus)

1. **Default production candidate.** Lean **B** (portability/replay) as the default once
   both pass, with **A** as the fast path where `fma` fusion is confirmed and `C` where
   f64 is exposed. Decide after the sweep shows which pass on the target adapters.
2. **`fma` fusion on naga/DX12.** Resolve empirically via the A sweep; if non-fused,
   A is dropped in favor of B and the finding is recorded as a binding backend note.
3. **Exhaustive-sweep budget.** Confirm the full 2³¹ sweep batches within CI/GPU time;
   if not, tier it (full sweep gated behind `--ignored`, a dense stratified sweep in the
   default run) without weakening the promotion criterion.
4. **`mag` re-enablement.** Once `sqrt` is `ExactDeterministic`, open the follow-on to let
   true Euclidean `mag` replace `mag2` where exactness (not just ordering) is required —
   separate slice, separate row.

---

*This track exists to let `sqrt` **earn** exact authority through proof, not assertion.
Until it does, the guardrail holds at admission and the sim never sees an unproven exact
`sqrt`. When it does, exactly one admission bit changes and the runtime is untouched.*
