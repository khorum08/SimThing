# SQRT-EXACT-5F — Exhaustive Sweep for Candidate F Hot-Path Sqrt

**Lane:** Tier-2 shader/software deterministic sqrt proof gate (test-only).

**Base HEAD:** `160bb596f62af3f9b018f566c1a9e1ea2cbe2c42`

## Candidate identity

- Candidate F artifact path: `crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl`
- Harness include: `SQRT_CR_F_WGSL = include_str!("wgsl/sqrt_cr_f_candidate.wgsl")`
- Candidate entrypoint: `fn sqrt_cr_f_bits(x_bits: u32) -> u32`
- Artifact hash (FNV-1a64): `e2e9e27601ee2e13`

## Required pre-edit evaluation answers

1. **F verbatim include via `include_str!`:** yes (`SQRT_CR_F_WGSL` constant in battery).
2. **F `sqrt_cr_f_bits` + `array<u32>` bit-IO:** yes (artifact and wrappers use `u32` input/output bits).
3. **4F sampled sweeps + contraction probe:** yes (`max_ulp=0` sampled; contraction probe clean).
4. **4F 34k perf vs E3:** yes in 4F report (`E3 1.954 ms`, `F 1.376 ms`, ratio `0.7045`).
5. **F still unpromoted before this pass:** yes (`ExactCandidatePendingExhaustiveSweep`).
6. **Closed guardrails:** M-JIT closed at PROD-0; no scheduler/cache/default wiring; no production bridge; no semantic WGSL; no Candidate C/f64; no native sqrt/`mag2` exact-authority flip.

## Exact command run

```bash
cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact5f_candidate_f_full_exhaustive_sweep -- --ignored --nocapture
```

## Full domain covered

- Domain target: `0x0000_0000..=0x7F7F_FFFF` (all finite non-negative `f32` bit patterns)
- Total values tested: `2,139,095,040`
- Range covered: `start=0x00000000`, `end=0x7f7fffff`
- Coverage proof:
  - Test enforces `tested == (end - start + 1)` and fails on mismatch.
  - Exhaustive summary:
    - `split=full_or_explicit_range start=0x00000000 end=0x7f7fffff tested=2139095040 exact_bits=2139095040 max_ulp=0 flush_count=0 worst=none`
  - Batch summary log appended to `docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_batches.log`.

## Runtime / batching strategy

- F exhaustive test now supports deterministic batching/range splitting via:
  - `SIMTHING_SQRT_F_TOTAL_SPLITS`
  - `SIMTHING_SQRT_F_SPLIT_INDEX`
  - `SIMTHING_SQRT_F_RANGE_START`
  - `SIMTHING_SQRT_F_RANGE_END`
  - `SIMTHING_SQRT_F_BATCH`
  - `SIMTHING_SQRT_F_PROGRESS_EVERY`
- Full proof run used defaults:
  - batch: `1,048,576`
  - progress_every: `64`
  - full range (no explicit split)
- Runtime for full exhaustive run: ~`47.43s` on this host/GPU.

## Proof result

- `max_ulp`: `0`
- exact count: `2,139,095,040 / 2,139,095,040`
- `flush_count`: `0`
- worst-case rows: none (`worst=none`)
- proof passed: **yes**

## Classification

**Candidate F:** `ExactDeterministicCandidate`

Interpretation:
- F is now an exact hot-path candidate by full finite non-negative exhaustive proof.
- E3 remains exact deterministic fallback/reference.
- Any production descriptor/admission authority flip remains a separate explicitly authorized step.

## E3 comparison

- E3 already passed exhaustive proof in SQRT-EXACT-4E and remains exact deterministic fallback/reference.
- F now also passes exhaustive proof, aligning the hot-path target with exactness requirements.

## Guardrail confirmations

- Candidate C/f64 not implemented.
- No production sqrt admission flip added in this pass.
- Native sqrt remains `ApproximateJitOnly` until separate admission promotion.
- `mag2` remains blocked as exact input until separate admission promotion.
- M-JIT remains closed at PROD-0.
- V7.7 / Mapping ADR / SEAD posture remains intact.

## Tests and scans run

### Commands

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact5f_candidate_f_full_exhaustive_sweep -- --ignored --nocapture` -> **PASS**
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` -> **PASS**
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture` -> **PASS**
- `cargo check --workspace` -> **PASS**

### Required scans

1. `rg "sqrt_exact.*f.*full_exhaustive|SQRT_CR_F_WGSL|sqrt_cr_f_candidate|ExactCandidatePendingExhaustiveSweep|ExactDeterministicCandidate|ApproximateJitOnlyPerformanceCandidate" crates docs`
   - F exhaustive proof path and updated classification references present.
2. `rg "sqrt_cr_f_candidate|SQRT_CR_F_WGSL|sqrt_cr_f_bits|CorrectlyRoundedHwBitmaskNormalized" crates/simthing-driver/tests crates/simthing-driver/tests/wgsl docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md`
   - F remains verbatim artifact loaded via `include_str!`.
3. `rg "fn emit_sqrt_cr_f_fn|emit_sqrt_cr_f" crates`
   - No dynamic F emitter implementation; only negative-test guard string remains.
4. `rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs`
   - No Candidate C/f64 implementation in this lane; only guardrail/rejection text and unrelated timing helpers.
5. `rg "ExactDeterministic|ApproximateJitOnly|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/invariants.md docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
   - Guardrails intact; no production authority flip in this slice.
6. `rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md docs/workshop/sqrt_candidates.md`
   - Guardrail-only references; no authorization additions.

## Transient cleanup

- Requested POSIX `find` command is not available on this Windows shell.
- Equivalent docs-tests listing was run for `*.log`, `*tmp*`, `*scratch*`.
- Existing historical logs retained as evidence; no new scratch/tmp artifacts required deletion.
- No E-phase/E11 evidence deleted.

## Final verdict

**PASS — SQRT-EXACT-5F completed the exhaustive Candidate F finite non-negative `f32` sweep.** Candidate F achieved `max_ulp == 0` across the full domain with exact bit parity and `flush_count == 0`, is now `ExactDeterministicCandidate` pending separate descriptor/admission promotion, Candidate C/f64 was not implemented, no production exact sqrt admission or `mag2` authority flip was added in this pass, M-JIT remains closed at PROD-0, and V7.7 / Mapping ADR / SEAD guardrails remain intact.
