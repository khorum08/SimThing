# SQRT-EXACT-3E — Correctly-Rounded Integer Mantissa Core for Candidate E

**Lane:** Tier-2 test-only shader/software deterministic `sqrt` remediation.

**Base HEAD:** `c9e889272d94305cc3db4c59e0ff64bfb8dd8792`

**Files changed:**
- `crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl`
- `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_exact3e_integer_core_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/sqrt_candidates.md`
- `docs/worklog.md`

---

## Candidate E artifact identity

- Artifact path: `crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl`
- Include path: `SQRT_CR_E_WGSL = include_str!("wgsl/sqrt_cr_e_candidate.wgsl")`
- 2E hash (FNV-1a64): `14e7c59e8ae698a1`
- 3E hash (FNV-1a64): `1efe4505c2a21fed`
- Authoritative entrypoint: `fn sqrt_cr_e_bits(x_bits: u32) -> u32`

---

## Algorithm summary (3E integer core)

Candidate E finite positive path is now integer-authoritative:

1. Decode `x_bits` to sign/exp/mantissa.
2. Classify special values in integer domain (`+0`, `-0`, `+inf`, NaN, negative finite/`-inf`).
3. Normalize positive finite inputs to integer significand `sx` (`24-bit with hidden bit`) and unbiased exponent `exu` (subnormals shifted into normalized form).
4. Compute output exponent guess `eyu = exu >> 1`.
5. Compute floor significand with monotonic integer binary search:
   - compare `sy^2` vs normalized input using integer arithmetic only.
6. Apply nearest-even rounding with exact integer remainder distances:
   - `diff_lo = x_scaled - floor^2`
   - `diff_up = (floor+1)^2 - x_scaled`
   - choose nearest; ties go to even.
7. Recompose IEEE-754 output bits directly from integer exponent/significand.

### Integer width / limb strategy

- Uses explicit `u32` limb-pair arithmetic for 48-bit values (`vec2<u32>(hi, lo)`).
- `sq24_u48` computes exact 24-bit square into 48 bits.
- `shl_u48`, `cmp_u48`, `sub_u48` provide deterministic widened operations without `u64`.

### Rounding rule

- Integer nearest-even at significand decision point.
- No authoritative floating-point arithmetic in finite positive path.

### NaN policy

- Canonical quiet NaN (`0x7fc00000`) for negative finite and `-inf`.
- NaN input returns quiet NaN class (payload/sign parity not overclaimed).

---

## Required test results

### Compile / semantic / finite-path constraints

- `sqrt_exact3e_candidate_e_wgsl_compiles_semantic_free` -> **PASS**
- `sqrt_exact3e_candidate_e_no_authoritative_fp_path` -> **PASS**
- `sqrt_exact3e_candidate_e_uses_u32_bit_io` -> **PASS**
- `sqrt_exact3e_candidate_e_artifact_hash_recorded` -> **PASS**

Artifact checks:
- no `sqrt(` in E artifact
- no `fma(`
- no `array<f32>`
- no `f64`, `SHADER_F64`, `F64RoundDown`, `sqrt_cr_c`

### Edge rows

- Test: `sqrt_exact3e_candidate_e_edge_rows`
- Result:
  - total: `21`
  - exact (non-NaN): `18`
  - normal exact: `16`
  - normal max ULP: `0`
  - subnormal exact: `2`
  - NaN class parity rows: `3`

### Subnormal sweep

- Test: `sqrt_exact3e_candidate_e_subnormal_sweep`
- Corpus size: `2572`
- Result:
  - exact bits: `2572`
  - max ULP: `0`
  - flush count: `0`

### Dense normal sweep

- Test: `sqrt_exact3e_candidate_e_dense_normal_sweep`
- Corpus size: `1043`
- Result:
  - exact bits: `1043`
  - max ULP: `0`
  - classification: `ExactCandidatePendingExhaustiveSweep`

### E3 vs E2 vs D

- Test: `sqrt_exact3e_candidate_e_compared_to_d_and_e2`
- Result:
  - D dense mismatches: `12`
  - E2 dense mismatches: `788`
  - E3 dense mismatches: `0`
  - D subnormal flush: `2572`
  - E2 subnormal flush: `0`
  - E3 subnormal flush: `0`

---

## Worst-case rows

- Dense normal worst-case ULP: `0` (none)
- Subnormal worst-case ULP: `0` (none)
- Remaining non-exact rows are NaN payload/sign-policy rows only (class parity preserved).

---

## Classification

**Candidate E (3E):** `ExactCandidatePendingExhaustiveSweep`

Rationale:
- Edge rows: non-NaN exact with normal `max_ulp=0`.
- Dense normal sweep: `max_ulp=0`.
- Subnormal sweep: `max_ulp=0`, `flush_count=0`.
- Promotion still blocked until full exhaustive ignored sweep is run and passes.

### Optional exhaustive sweep

- Added: `#[ignore] sqrt_exact3e_candidate_e_full_exhaustive_sweep`
- Not run by default.
- Command:
  - `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact3e_candidate_e_full_exhaustive_sweep -- --ignored --nocapture`

---

## Guardrail confirmations

- Candidate C/f64 not implemented.
- No production sqrt admission added.
- Native sqrt remains `ApproximateJitOnly`.
- `mag2` remains blocked as exact input.
- M-JIT remains closed at PROD-0.

---

## Commands run

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` -> **PASS** (`24 passed`, `2 ignored`)
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` -> **PASS** (`8 passed`)
- `cargo check --workspace` -> **PASS**

---

## Required scans (recorded)

1. `rg "sqrt_cr_e_candidate|SQRT_CR_E_WGSL|sqrt_cr_e_bits|CorrectlyRoundedInteger" crates docs` -> E artifact/include/test presence confirmed.
2. `rg "array<u32>|input_bits|output_bits|to_bits|from_bits" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact3e_integer_core_results.md` -> bit-IO path confirmed.
3. `rg "sqrt\\(|fma\\(|array<f32>" crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl` -> no matches.
4. `rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs` -> no C/f64 implementation in this slice; guardrail/rejection text and unrelated legacy `as_secs_f64` references only.
5. `rg "ApproximateJitOnly|ExactDeterministic|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" ...` -> guardrails intact.
6. `rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" ...` -> guardrail language only; no authorization added.

---

## Transient cleanup

Equivalent `docs/tests` log/tmp/scratch listing run. Existing historical `*.log` evidence retained; no new scratch/tmp artifacts removed.

---

## Final verdict

**PASS — SQRT-EXACT-3E landed** as a test-only Candidate E integer-core remediation. Candidate E remains a standalone verbatim WGSL artifact with `u32` bit IO, finite positive path avoids authoritative `sqrt`/`fma`/`f32` output, edge/subnormal/dense sweeps and E3-vs-E2-vs-D comparison were recorded, Candidate C/f64 was not implemented, no production exact sqrt admission or `mag2` authority flip was added, native sqrt remains `ApproximateJitOnly`, M-JIT remains closed at PROD-0, and V7.7 / Mapping ADR / SEAD guardrails remain intact.
