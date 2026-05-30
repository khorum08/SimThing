# SQRT-EXACT-4E — Exhaustive Sweep for Candidate E3

**Lane:** Tier-2 shader/software deterministic `sqrt` proof gate (test-only).

**Base HEAD:** `41a349c74008329cd827b2b09f1301dfd9c17fc6`

## Candidate identity

- Artifact path: `crates/simthing-driver/tests/wgsl/sqrt_cr_e_candidate.wgsl`
- Harness include: `SQRT_CR_E_WGSL = include_str!("wgsl/sqrt_cr_e_candidate.wgsl")`
- Candidate entrypoint: `fn sqrt_cr_e_bits(x_bits: u32) -> u32`
- Artifact hash (FNV-1a64): `1efe4505c2a21fed`

## Exact command run

```bash
cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact3e_candidate_e_full_exhaustive_sweep -- --ignored --nocapture
```

## Full domain covered

- Domain target: `0x0000_0000..=0x7F7F_FFFF` (all finite non-negative `f32` bit patterns)
- Total values tested: `2,139,095,040`
- Range covered: `start=0x00000000`, `end=0x7f7fffff`
- Coverage proof:
  - Test enforces `tested == (end - start + 1)` and fails on mismatch.
  - Exhaustive summary line:
    - `split=full_or_explicit_range start=0x00000000 end=0x7f7fffff tested=2139095040 exact_bits=2139095040 max_ulp=0 flush_count=0 worst=none`
  - Batch summary log retained at `docs/tests/phase_m_jit_sqrt_exact4e_exhaustive_batches.log`.

## Runtime / batching strategy

- Monolithic ignored sweep initially exceeded short timeout window; harness was upgraded with deterministic batching/resume knobs:
  - `SIMTHING_SQRT_E4_BATCH`
  - `SIMTHING_SQRT_E4_PROGRESS_EVERY`
  - `SIMTHING_SQRT_E4_RANGE_START`, `SIMTHING_SQRT_E4_RANGE_END`
  - `SIMTHING_SQRT_E4_TOTAL_SPLITS`, `SIMTHING_SQRT_E4_SPLIT_INDEX`
- Full proof run used batch size `1,048,576` values and completed in ~53s on this host/GPU.
- Optional split smoke run was also executed (`split=0/128`) and logged; full-range run remains the authoritative proof claim.

## Proof result

- `max_ulp`: `0`
- exact count: `2,139,095,040 / 2,139,095,040`
- subnormal outputs exact: yes (included in full finite non-negative domain)
- `flush_count`: `0`
- worst-case rows: none (`worst=none`)
- proof passed: **yes**

## Classification

**`ExactDeterministicCandidate`**

E3 has now passed exhaustive finite non-negative proof. A production descriptor/admission authority flip remains a separate authorized pass and was not performed here.

## Guardrail confirmations

- Candidate F was not built or implemented in this pass.
- Candidate C/f64 was not implemented.
- No production exact `sqrt` admission flip was added in this pass.
- Native `sqrt` remains `ApproximateJitOnly` pending separate admission promotion.
- `mag2` remains blocked from exact-authoritative inputs pending separate admission promotion.
- M-JIT remains closed at PROD-0.
- V7.7 / Mapping ADR / SEAD posture remains intact.

## Tests and scans run

### Commands

- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery sqrt_exact3e_candidate_e_full_exhaustive_sweep -- --ignored --nocapture` (**PASS**)
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture` (**PASS**)
- `cargo check --workspace` (**PASS**)

### Required scans

1. `rg "sqrt_exact3e_candidate_e_full_exhaustive_sweep|SQRT_CR_E_WGSL|sqrt_cr_e_candidate|ExactCandidatePendingExhaustiveSweep|ExactDeterministicCandidate" crates docs`
   - Exhaustive test path and E artifact/include references present; 4E report/doc rows include `ExactDeterministicCandidate`.
2. `rg "sqrt_cr_f_candidate|SQRT_CR_F_WGSL|sqrt_cr_f_bits|CorrectlyRounded.*F" crates/simthing-driver/tests crates/simthing-driver/tests/wgsl docs/tests/phase_m_jit_sqrt_exact4e_exhaustive_sweep_results.md`
   - No Candidate F implementation artifact in test code; only policy/report text.
3. `rg "F64RoundDown|SHADER_F64|f64\\(|sqrt_cr_c|Candidate C" crates docs`
   - No Candidate C/f64 implementation path in this lane; matches are rejection/guardrail text and unrelated timing helpers.
4. `rg "ExactDeterministic|ApproximateJitOnly|native sqrt|mag2|validate_exact_inputs|landed_jit_kernel_descriptors" docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/invariants.md docs/tests/phase_m_jit_sqrt_exact4e_exhaustive_sweep_results.md crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs`
   - Guardrails still explicit; no production admission flip in this pass.
5. `rg "default SimSession|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|semantic WGSL|scheduler|kernel cache" crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs docs/tests/phase_m_jit_sqrt_exact4e_exhaustive_sweep_results.md docs/workshop/sqrt_candidates.md`
   - Guardrail language only; no authorization additions.

## Transient cleanup

- Attempted exact `find` command; host lacks WSL distribution.
- Equivalent PowerShell listing run for `docs/tests` (`*.log`, `*tmp*`, `*scratch*`).
- No obvious new scratch/tmp artifacts required deletion.
- Exhaustive batch log retained as proof evidence.

## Final verdict

**PASS — SQRT-EXACT-4E completed the exhaustive Candidate E3 finite non-negative `f32` sweep.** E3 achieved `max_ulp == 0` over `0x0000_0000..=0x7F7F_FFFF` with exact bit parity and `flush_count == 0`, and is now `ExactDeterministicCandidate` pending a separate descriptor/admission promotion slice. Candidate F was not implemented, Candidate C/f64 was not implemented, no production exact `sqrt` admission or `mag2` authority flip was added, M-JIT remains closed at PROD-0, and V7.7 / Mapping ADR / SEAD guardrails remain intact.
