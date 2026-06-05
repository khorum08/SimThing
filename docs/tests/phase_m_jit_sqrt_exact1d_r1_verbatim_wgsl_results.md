# SQRT-EXACT-1D-R1 — Verbatim WGSL Intrinsic Harness for Candidate D

**Lane:** Tier-2 test-only shader/software deterministic `sqrt` remediation (no production admission).

**Base HEAD:** `4bb3e900de325c77d627803613e7c85db3cf3945`

---

## Scope and pre-edit evaluation

Inspected:
- `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_exact1d_candidate_d_results.md`
- `docs/workshop/sqrt_candidates.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/invariants.md`

Pre-edit answers:
- Candidate D WGSL was emitted dynamically from Rust helper `emit_sqrt_cr_d_fn()`.
- There was no standalone canonical D WGSL artifact file in the repo.
- The harness can include a `.wgsl` file verbatim via `include_str!`.
- New proof points required: verbatim artifact usage test, compile/semantic test against artifact, artifact identity hash test, and reproducibility of edge/dense/subnormal/probe against artifact-based wrappers.
- Remaining SQRT-EXACT-1D failures were unchanged: dense normal `max_ulp=1`, subnormal FTZ output flush to `0.0`, D classification `ApproximateJitOnly`.

---

## Files changed

- `crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl` (**new** canonical Candidate D intrinsic artifact)
- `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
- `docs/tests/phase_m_jit_sqrt_exact1d_r1_verbatim_wgsl_results.md` (**this file**)
- `docs/workshop/mapping_current_guidance.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/workshop/sqrt_candidates.md`
- `docs/worklog.md`

---

## Verbatim artifact

- **Path:** `crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl`
- **Binding in harness:** `const SQRT_CR_D_WGSL: &str = include_str!("wgsl/sqrt_cr_d_candidate.wgsl");`
- **Artifact hash (FNV-1a 64):** `06371a83b9ba18a1`
- **Artifact bytes:** `2299`

Artifact contains `sqrt_cr_d`, `snap_directional`, `dekker_residual_hardened`, and integer subnormal normalization (`sqrt_cr_d_subnormal_integer`). Artifact contains no `f64`, `SHADER_F64`, `F64RoundDown`, `sqrt_cr_c`, or semantic terms.

---

## Verbatim inclusion proof

Test: `sqrt_exact1d_r1_candidate_d_uses_verbatim_wgsl_artifact`

- Confirms `SQRT_CR_D_WGSL` loads and exposes required D functions.
- Confirms D batch/probe wrappers each contain the exact artifact text as one contiguous substring.
- Confirms D path no longer depends on `emit_sqrt_cr_d_fn` (helper removed from file).

---

## Compile + semantic proof

Test: `sqrt_exact1d_r1_verbatim_d_wgsl_compiles_semantic_free`

- Artifact-level semantic and forbidden-term scan: **PASS**
- Wrapped batch shader compile on GPU backend: **PASS**
- Forbidden C/f64 terms absent in implementation path: **PASS**

---

## Reproduced Candidate D results (artifact-backed)

### Edge rows

- Test: `sqrt_exact1d_candidate_d_edge_rows`
- Result: `total=21`, `exact=16`, `normal_exact=16`, `normal_max_ulp=0`, `subnormal_exact=0`, `nan_ok=3`
- Subnormal edge rows still flush (`smallest_subnormal`, `largest_subnormal` -> `0.0` GPU)

### Dense normal sweep

- Test: `sqrt_exact1d_candidate_d_dense_normal_sweep`
- Result: `tested=1044`, `exact_bits=1032`, `max_ulp=1`, `class=ApproximateJitOnly`
- Corrections: `117` total (`up=116`, `down=1`)

### Subnormal sweep

- Test: `sqrt_exact1d_candidate_d_subnormal_sweep`
- Result: `tested=2572`, `exact_bits=0`, `max_ulp=536870911`, `integer_path=2572`, `flush=2572`

### Contraction-barrier probe

- Test: `sqrt_exact1d_candidate_d_contraction_barrier_probe`
- Result: `tested=1043`, `native_mismatch=129`, `d_mismatch=12`, `corrections=117`, `up=116`, `down=1`, `d_changes_vs_native=117`
- Confirms D is not just returning native `sqrt`.

### Comparison vs SQRT-EXACT-1D

- Outcome is **same** as SQRT-EXACT-1D (expected for verbatim-harness remediation): correction behavior preserved; dense normal `max_ulp=1`; subnormal flush unresolved.

---

## Classification and guardrails

- Candidate D remains **`ApproximateJitOnly`**.
- No exhaustive zero-ULP proof; no promotion.
- No Candidate C/f64 implementation.
- No production exact sqrt admission added.
- Native sqrt remains `ApproximateJitOnly`.
- `mag2` remains blocked as exact input.
- M-JIT remains closed at PROD-0.

---

## Required scans (recorded)

### Verbatim include scan

`rg "include_str!.*sqrt_cr_d_candidate|sqrt_cr_d_candidate.wgsl|SQRT_CR_D_WGSL" crates docs`

Result: `SQRT_CR_D_WGSL` and artifact path present in battery and docs.

### Dynamic-emitter removal scan

`rg "fn emit_sqrt_cr_d_fn|sqrt_cr_d\\(" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl`

Result: no `emit_sqrt_cr_d_fn`; `sqrt_cr_d` defined in artifact and used in wrappers/tests.

### C/f64 guardrail scan

`rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs`

Result: no Candidate C implementation path; matches are guardrail/rejection text and unrelated existing `as_secs_f64` timing code outside sqrt implementation.

### Exact-authority guardrail scan

`rg "ApproximateJitOnly|ExactDeterministic|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/invariants.md docs/tests/phase_m_jit_sqrt_exact1d_r1_verbatim_wgsl_results.md crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`

Result: native sqrt and `mag2` remain blocked from exact authority.

### Production-wiring guardrail scan

`rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_d_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact1d_r1_verbatim_wgsl_results.md docs/workshop/sqrt_candidates.md`

Result: no authorization in test/artifact/report; `sqrt_candidates.md` retains guardrail statements only.

---

## Commands run

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` -> **PASS** (15 passed, 1 ignored)
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` -> **PASS** (8 passed)
- `cargo check --workspace` -> **PASS**

Ignored exhaustive sweep was not run and no promotion was attempted.

---

## Transient cleanup

Transient scan executed (`docs/tests` log/tmp/scratch listing). Existing `*.log` artifacts are historical evidence and were retained; no obvious new scratch/tmp artifacts from this slice were deleted.

---

## Final verdict

**PASS — SQRT-EXACT-1D-R1 landed.** Candidate D is now tested from a standalone verbatim WGSL artifact via `include_str!`, artifact identity hash is recorded, edge/dense/subnormal/probe behavior is reproduced, Candidate C/f64 remains unimplemented, no production exact sqrt admission or `mag2` authority flip was added, native sqrt remains `ApproximateJitOnly`, M-JIT remains closed at PROD-0, and V7.7 / Mapping ADR / FIELD_POLICY guardrails remain intact.
