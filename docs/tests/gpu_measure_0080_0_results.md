# GPU-MEASURE-0080-0 Results

Status: IMPLEMENTED / PASS - rehearsal GPU measurement pass  
Date: 2026-06-04  
Adapter: NVIDIA GeForce RTX 4080 Laptop GPU  
Stable report checksum: `6b5340e84da289cb`

This pass measures the already-accepted `SCENARIO-0080-2` dress-rehearsal row/mask/threshold/emission-band shapes on the discrete GPU through the existing generic GPU substrates. It does not add semantic WGSL, does not add an `AccumulatorOp`, does not change invariants, does not change pinned numbers, does not wire default `SimSession`, and does not reopen `SCENARIO-0080-2`.

| Shape | Source | GPU verdict | CPU checksum | GPU checksum | Bound / delta |
| --- | --- | --- | ---: | ---: | --- |
| R1 disruption input + bounded recurrence | R1 | GPU-measured (integer bit-exact) | `bd7431ae46ee2ab7` | `bd7431ae46ee2ab7` | exact |
| R2 owner reduce-up + disburse-down | R2 | GPU-measured (integer bit-exact) | `6b139dca37e48068` | `6b139dca37e48068` | exact |
| R4 GradientXY + Candidate-F magnitude | R4 | GPU-measured (verified-approximate, within accepted f32 bound) | `3f10e26b117ae732` | `01f7ad80321e6355` | max abs delta `3.0994415e-6` <= `1.0e-4`; Candidate-F bits match |
| R6 combat damage reduce + attrition emission | R6 | GPU-measured (integer bit-exact) | `9778954b887a6f2e` | `9778954b887a6f2e` | exact |
| R6B construction threshold + fusion sum | R6B | GPU-measured (integer bit-exact) | `f6f327e95478d540` | `f6f327e95478d540` | exact |
| R6C integrated 100-tick whole-run execution | R6C | GPU-conformant; GPU execution not yet measured | `1bba891c779190a4` | `0000000000000000` | whole-run execution remains intentionally unmeasured |

R6C keeps the exact accepted wording `GPU-conformant; GPU execution not yet measured`. The measured claim applies per constituent shape above, not to the integrated 100-tick R6C whole-run scheduler.

Verification:

- `cargo test -p simthing-driver --test gpu_measure_0080_0 -- --nocapture` - PASS, 11 passed.
- `cargo test -p simthing-driver --test gpu_measure_0080_0` - PASS, 11 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu` - PASS, 10 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store` - PASS, 11 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run` - PASS, 22 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r6b_ship_cohort_reinforcement` - PASS, 24 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r6_combat_hp_damage` - PASS, 25 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r4_field_policy_consumption` - PASS, 16 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r2_recursive_allocation` - PASS, 13 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_r1_disruption_heatmap` - PASS, 34 passed.
- `cargo test -p simthing-spec --test mobility_reenroll0_substrate` - PASS, 16 passed.
- `cargo test -p simthing-spec --test mobility_runtime0_composition` - PASS, 23 passed.
- `cargo check --workspace` - PASS, pre-existing warnings only.

Nearest GPU-exec / KERNEL / Candidate-F / GPU substrate checks:

- `cargo test -p simthing-driver --test gpu_exec0_readiness_fixture` - PASS, 13 passed.
- `cargo test -p simthing-driver --test mobility_gpu_kernel0_fixture` - PASS, 16 passed.
- `cargo test -p simthing-driver --test mobility_gpu_kernel6_chain_fixture` - PASS, 22 passed.
- `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu` - PASS, 8 passed.
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude` - PASS, 12 passed.
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_mag2_0_fixed_exact` - PASS, 7 passed.
- `cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery` - PASS, 35 passed, 3 ignored exhaustive sweeps.
- `cargo test -p simthing-gpu --test structured_field_stencil` - PASS, 30 passed.
- `cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge` - PASS, 3 passed.
