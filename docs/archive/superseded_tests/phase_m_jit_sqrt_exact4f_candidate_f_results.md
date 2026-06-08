# SQRT-EXACT-4F — Verbatim WGSL Candidate F Hot-Path Probe Results

**Lane:** Tier-2 test-only shader/software deterministic sqrt hot-path probe.

**Base HEAD:** `13fae2ca3dbc4277f28ca77506a6c9a702f0dd34`

## Candidate F artifact identity

- Artifact path: `crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl`
- Include path: `SQRT_CR_F_WGSL = include_str!("wgsl/sqrt_cr_f_candidate.wgsl")`
- Artifact hash (FNV-1a64): `e2e9e27601ee2e13`
- Artifact bytes: `5964`
- Authoritative entrypoint: `fn sqrt_cr_f_bits(x_bits: u32) -> u32`
- Optional wrapper present: `fn sqrt_cr_f(x: f32) -> f32`

## Verbatim confirmation and compile blocker handling

- Candidate F WGSL was copied verbatim from `docs/workshop/sqrt_candidates.md` into `sqrt_cr_f_candidate.wgsl`.
- No algorithmic deviation was required in the artifact.
- One compile blocker occurred in a **test probe wrapper only** (`isFinite` unresolved in WGSL wrapper code); it was fixed by integer finite-positive checks in the wrapper and did not modify F's algorithm/artifact.
- No Rust dynamic-emitter path was introduced for F (`emit_sqrt_cr_f*` absent).

## Algorithm summary

Candidate F uses:

1. Integer-domain decode/classification from raw `u32` bits.
2. `[1,4)` normalized mantissa path with exponent folding.
3. Hardware `sqrt` seed on normalized finite positive mantissa only.
4. Bitmask split + Dekker residual without `fma`.
5. Directional Markstein snap.
6. Integer-bit exponent reconstruction and `u32` output bits.

## Required test outcomes

### Verbatim/artifact/bit-IO tests

- `sqrt_exact4f_candidate_f_wgsl_compiles_semantic_free` -> **PASS**
- `sqrt_exact4f_candidate_f_uses_verbatim_wgsl_artifact` -> **PASS**
- `sqrt_exact4f_candidate_f_uses_u32_bit_io` -> **PASS**

Assertions covered:
- `include_str!` artifact inclusion
- contiguous artifact embedding in wrappers
- `array<u32>` authority path
- no dynamic Rust F emitter
- no forbidden terms (`f64`, `SHADER_F64`, `F64RoundDown`, `sqrt_cr_c`)

### Edge rows

- Test: `sqrt_exact4f_candidate_f_edge_rows`
- Result:
  - total rows: `24`
  - exact bits (non-NaN): `21`
  - normal exact: `19`
  - normal max ULP: `0`
  - subnormal exact: `2`
  - NaN class parity rows: `3`
- NaN policy: class parity preserved; payload/sign policy not overclaimed.

### Subnormal sweep

- Test: `sqrt_exact4f_candidate_f_subnormal_sweep`
- Corpus size: `2572`
- Result:
  - exact bits: `2572`
  - max ULP: `0`
  - flush count: `0`
  - worst rows: none

### Dense normal sweep

- Test: `sqrt_exact4f_candidate_f_dense_normal_sweep`
- Corpus size: `1043`
- Result:
  - exact bits: `1043`
  - max ULP: `0`
  - flush count: `0`
  - correction count: `129` (`up=128`, `down=1`)
  - classification on sampled sweeps: `ExactCandidatePendingExhaustiveSweep`

### F vs E3 correctness comparison

- Test: `sqrt_exact4f_candidate_f_compared_to_e3`
- Result:
  - E3 dense max ULP: `0`
  - F dense max ULP: `0`
  - E3 subnormal max ULP: `0`
  - F subnormal max ULP: `0`
  - E3 dense mismatches: `0`
  - F dense mismatches: `0`
  - rows where F fails and E3 passes: `0`

### Contraction-risk probe

- Test: `sqrt_exact4f_candidate_f_contraction_probe`
- Result:
  - tested: `1043`
  - native sqrt mismatches: `129`
  - F mismatches: `0`
  - correction count: `129`
  - up corrections: `128`
  - down corrections: `1`
  - F changes vs native: `129`
  - residual reassociation rows flagged: `0`

Interpretation: on this backend/corpus, F corrects all observed native mismatches and no residual reassociation symptom was detected.

### No promotion assertions

- `sqrt_exact4f_no_exact_authority_promotion` -> **PASS**
- `sqrt_exact4f_perf_is_not_authority` -> **PASS**

Confirmed:
- native sqrt remains `ApproximateJitOnly`
- no exact sqrt descriptor admission added
- `mag2` remains blocked as exact input
- baseline `accumulator_op.wgsl` remains sqrt-free

## Performance smoke benchmark

- Test: `sqrt_exact4f_perf_e3_vs_f_smoke`
- Measurement includes shader dispatch + readback in the current harness (`includes_readback=true`)
- Dispatch count per run: `1`

| Inputs | E3 ms | F ms | F/E3 ratio |
|---|---:|---:|---:|
| `1,000` | `11.144` | `1.894` | `0.1700` |
| `10,000` | `2.072` | `1.290` | `0.6228` |
| `34,000` | `1.954` | `1.376` | `0.7045` |
| `100,000` | `4.482` | `2.229` | `0.4974` |

Caveats:
- smoke-only timing (single-run per size, warm-state noise visible)
- includes readback and host timing jitter
- not an authority gate

Ignored (not run by default):
- `sqrt_exact4f_candidate_f_full_exhaustive_sweep`
- optional large perf size (`1_000_000`) documented by test output note

## Classification

**Candidate F:** `ExactCandidatePendingExhaustiveSweep`

Rationale:
- sampled edge/subnormal/dense corpora are all `max_ulp=0` with zero subnormal flush
- contraction probe is clean on this backend/corpus
- full finite non-negative exhaustive proof for F is still required before any exact-authority promotion

## Guardrail confirmations

- Candidate C/f64 not implemented.
- No production sqrt admission added.
- Native sqrt remains `ApproximateJitOnly`.
- `mag2` remains blocked as exact input.
- M-JIT remains closed at PROD-0.
- No scheduler/cache/default wiring/economy bridge/semantic WGSL authorization added.

## Commands run

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` -> **PASS** (`35 passed`, `3 ignored`)
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` -> **PASS** (`8 passed`)
- `cargo check --workspace` -> **PASS**

## Required scans (recorded)

1. `rg "sqrt_cr_f_candidate|SQRT_CR_F_WGSL|sqrt_cr_f_bits|CorrectlyRounded.*F|Candidate F" crates docs`
   - F artifact/include/tests present in battery + docs.
2. `rg "fn emit_sqrt_cr_f_fn|emit_sqrt_cr_f" crates`
   - No dynamic F emitter function definitions; only a negative-test substring check.
3. `rg "array<u32>|input_bits|output_bits|to_bits|from_bits" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact4f_candidate_f_results.md`
   - `u32` bit-IO path confirmed.
4. `rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs`
   - No Candidate C/f64 implementation in this slice; matches are guardrail/rejection text and unrelated timing helpers.
5. `rg "ExactDeterministic|ApproximateJitOnly|Approximate.*performance|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/invariants.md docs/tests/phase_m_jit_sqrt_exact4f_candidate_f_results.md crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
   - No authority flip; guardrails remain explicit.
6. `rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact4f_candidate_f_results.md docs/workshop/sqrt_candidates.md`
   - Guardrail-only references; no new authorization.

## Transient cleanup

- Listing equivalent of requested docs-tests transient scan found existing historical logs only (including prior exhaustive evidence log); no new `tmp`/`scratch` artifacts were created.
- No E-phase/E11 or prior sweep evidence was deleted.

## Final verdict

**PASS — SQRT-EXACT-4F landed** as a test-only verbatim WGSL Candidate F hot-path probe. F is included via `include_str!`, uses authoritative `u32` bit IO, shows `max_ulp=0` on edge/dense/subnormal corpora, outperforms E3 in the recorded 34,000-row smoke benchmark, and preserves all guardrails (no Candidate C/f64 implementation, no production exact sqrt admission, no `mag2` authority flip, no M-JIT reopening). Exact authority promotion remains blocked pending F's own exhaustive full-domain proof.
