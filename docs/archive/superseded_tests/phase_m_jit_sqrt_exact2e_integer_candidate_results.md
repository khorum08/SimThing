# SQRT-EXACT-2E — Integer-Only Candidate E Sqrt Probe Results

**Lane:** Tier-2 test-only shader/software deterministic `sqrt` contingency probe (no production admission, no JIT reopening).

**Base HEAD:** `d9c63e9c0f2bffb8d88a5593f3ccafeb077f54a7`

**Test file:** `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`

---

## Candidate E artifact and harness

- **Artifact path:** `crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl`
- **Artifact include:** `const SQRT_CR_E_WGSL: &str = include_str!("wgsl/sqrt_cr_e_candidate.wgsl");`
- **Artifact hash (FNV-1a 64):** `14e7c59e8ae698a1`
- **Artifact bytes:** `2322`

Candidate E is implemented as verbatim WGSL with integer-domain decode/normalize/isqrt logic and direct bit construction:

- Authoritative entrypoint: `fn sqrt_cr_e_bits(x_bits: u32) -> u32`
- Batch harness storage: `array<u32>`
- Output comparison path: `u32` bit parity (`to_bits` / `from_bits`), not `array<f32>` authority.

This avoids D’s readback hazard by returning output bit patterns directly from shader storage.

---

## Algorithm summary

Candidate E uses integer-domain steps:

1. Decode `x_bits` (`sign`, `exp`, `mant`) and classify special values in integer domain.
2. Preserve `+0`/`-0`; classify negative finite / `-inf` as canonical quiet NaN class.
3. Normalize positive finite values (including subnormals) to integer mantissa + unbiased exponent.
4. Compute mantissa-root approximation with `isqrt_u32`.
5. Recompose IEEE-754 bits directly as `u32` (normal/subnormal handling included).

No `f64`, `SHADER_F64`, `F64RoundDown`, `sqrt_cr_c`, or `fma` in the Candidate E implementation.

---

## Required test outcomes

### 1) E artifact compiles semantic-free

- `sqrt_exact2e_candidate_e_wgsl_compiles_semantic_free` -> **PASS**

### 2) E uses u32 bit IO

- `sqrt_exact2e_candidate_e_uses_u32_bit_io` -> **PASS**

### 3) E edge rows

- `sqrt_exact2e_candidate_e_edge_rows` -> **PASS**
- Metrics:
  - total rows: `21`
  - exact bits (non-NaN): `9`
  - normal max ULP: `13`
  - subnormal exact: `0/2`
  - NaN class parity rows: `3`

NaN policy: canonical quiet NaN is used for negative inputs; report claims NaN **class parity**, not payload/sign parity.

### 4) E subnormal sweep

- `sqrt_exact2e_candidate_e_subnormal_sweep` -> **PASS**
- Metrics:
  - tested: `2572`
  - exact bits: `35`
  - max ULP: `128`
  - flush count (`out_bits == 0` when CPU nonzero): `0`
- Interpretation: E removes D’s subnormal flush-to-zero symptom on this backend but is not bit-exact.

### 5) E dense normal sweep

- `sqrt_exact2e_candidate_e_dense_normal_sweep` -> **PASS**
- Metrics:
  - tested: `1043`
  - exact bits: `255`
  - max ULP: `119`
  - classification: `RejectedDeferred`

### 6) E vs D comparison

- `sqrt_exact2e_candidate_e_compared_to_d` -> **PASS**
- Metrics:
  - D dense mismatches: `12`
  - E dense mismatches: `788`
  - D subnormal flush count: `2572`
  - E subnormal flush count: `0`

Result: E improves subnormal flush behavior but significantly regresses dense-normal accuracy.

### 7) No promotion

- `sqrt_exact2e_no_exact_authority_promotion` -> **PASS**
- Native sqrt remains `ApproximateJitOnly`; no exact sqrt descriptor admitted; `mag2` stays blocked.

### 8) Optional exhaustive sweep

- Not added (criteria not met).

---

## Classification

**Candidate E:** `RejectedDeferred`

Reason:
- Dense normal accuracy is far from promotion criteria (`max_ulp=119`, high mismatch count).
- Subnormal flush blocker is improved (`flush=0`), but overall exactness is insufficient.

---

## Guardrail confirmations

- Candidate C/f64 not implemented.
- No production sqrt admission added.
- Native sqrt remains `ApproximateJitOnly`.
- `mag2` remains blocked as exact input.
- M-JIT remains closed at PROD-0.

---

## Required scans (recorded)

1. `rg "sqrt_cr_e_candidate|SQRT_CR_E_WGSL|sqrt_cr_e_bits|CorrectlyRoundedInteger" crates docs`
   - Candidate E artifact and include path present.
2. `rg "array<u32>|input_bits|output_bits|to_bits|from_bits" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact2e_integer_candidate_results.md`
   - u32 bit-IO path present.
3. `rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs`
   - No Candidate C implementation; only guardrail/rejection context and unrelated legacy timing code (`as_secs_f64`) outside this lane.
4. `rg "ApproximateJitOnly|ExactDeterministic|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" ...`
   - guardrail docs/tests remain intact; no authority flip.
5. `rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" ...`
   - Guardrail text only; no new authorization.

---

## Commands run

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` -> **PASS** (`23 passed`, `1 ignored`)
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` -> **PASS** (`8 passed`)
- `cargo check --workspace` -> **PASS**

Ignored exhaustive sweep was not run.

---

## Transient cleanup

`find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print` equivalent scan run via shell listing. Existing historical `*.log` artifacts retained; no obvious new scratch/tmp artifacts deleted.

---

## Final verdict

**PASS — SQRT-EXACT-2E landed as a test-only Candidate E integer/bit-pattern probe.** Candidate E is a standalone verbatim WGSL artifact using u32 bit IO; edge/subnormal/dense-normal plus E-vs-D comparison were recorded; Candidate C/f64 was not implemented; no production exact sqrt admission or `mag2` authority flip was added; native sqrt remains `ApproximateJitOnly`; M-JIT remains closed at PROD-0; V7.7 / Mapping ADR / FIELD_POLICY guardrails remain intact.
