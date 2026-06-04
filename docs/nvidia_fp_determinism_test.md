# Temporary production PR track — NVIDIA FP determinism sweep

**Recipient model:** Cursor  
**Role:** production implementation agent  
**Track owner:** 0.0.8.0 production harness  
**Date opened:** 2026-06-03  
**Temporary cleanup rule:** all result files created for this track must be named `docs/tests/nvidia_fp_temp*.md` so they can be deleted together when this sweep is closed.

> **CLOSED / COMPLETE (2026-06-04, Opus).** The NVIDIA RTX 4080 validation ladder is **complete**. Battery 12 resolved the parked Battery 07/08/11 remedials; **Battery 13 resolved the last blocker** — `simthing-spec jit_kernel_cohort_preview::jit_cohort0_distinct_graphs_split` (a stale positional assertion on canonically `stable_key`-ordered cohorts; cohort-preview impl unchanged, test corrected to be position-insensitive). **Full `cargo test --workspace` is green on the discrete RTX 4080** (60 test binaries, 0 failed). Adapter-scope caveat **lifted**. Closeout: `docs/tests/nvidia_fp_temp_13_workspace_closeout.md`.

**Parked summary:** `docs/tests/nvidia_fp_temp_99_summary.md` — historical parked summary. Current remedial evidence: `docs/tests/nvidia_fp_temp_12_remedial_retest.md`.

## 0. Purpose

This temporary track validates GPU-dependent tests that previously ran on the Intel iGPU before `GpuContext::new_blocking()` was fixed to select the discrete adapter when present.

Source inventory:

- `docs/tests/gpu_intel_run_inventory_2026_06_03.md`
- `crates/simthing-gpu/src/context.rs`
- `docs/tests/scenario_0080_2_atlas_batch_0_store_gpu_parity_2026_06_03.txt`

Known current state:

- `GpuContext` now selects the discrete GPU when present.
- `ATLAS-BATCH-0-STORE-GPU` has RTX 4080 evidence and is bit-exact for integer owner/channel masked reductions.
- Priority f32 / `GpuVerified` batteries 01–06 and 09–10 executed on RTX 4080 with PASS (see status table).
- NVIDIA RTX 4080 priority validation is substantially complete. Battery 12 resolved the four previously parked non-NVIDIA-FP triage items.
- The sweep remains **parked** rather than closed because the Battery 12 full workspace retest now stops in `simthing-spec --test jit_kernel_cohort_preview::jit_cohort0_distinct_graphs_split`, an out-of-scope cohort ordering assertion and not NVIDIA FP drift.

## Parked Opus triage index

Full detail: `docs/tests/nvidia_fp_temp_99_summary.md` §Parked Opus triage index.

### Resolved by Battery 12

```text
Evidence:
  docs/tests/nvidia_fp_temp_12_remedial_retest.md

Resolved:
  Battery 07 jit_grad0 stale active-policy/doc guard.
  Battery 07 jit_exec1 admission-before-GPU guard.
  Battery 08 phase_m_boundary_cadence_doctrine missing doc include.
  Battery 11 phase_m_jit_desc0_kernel_descriptor fixture-local descriptor field skew.

Interpretation:
  These four items were non-NVIDIA-FP harness/doc/compile blockers.
  Targeted RTX-gated remedial retests passed.
```

### Still open after Battery 12

```text
Item:
  cargo test --workspace -- --nocapture
  simthing-spec --test jit_kernel_cohort_preview
  jit_cohort0_distinct_graphs_split

Evidence:
  docs/tests/nvidia_fp_temp_12_remedial_retest.md

Current interpretation:
  Workspace retest advances past the Battery 07/08/11 blockers and then fails on a simthing-spec cohort preview ordering assertion:
    left: ["variant"]
    right: ["base"]
  This is outside the four Battery 12 remedial files and outside NVIDIA FP tolerance/shader-math behavior.

Decision needed:
  Follow-up remedial or acceptance as pre-existing/out-of-scope before full NVIDIA sweep closure.
```

### Battery 07 — stale doc-hygiene guard

```text
Item:
  jit_grad0_mag2_not_overclaimed_if_approximate

Evidence:
  docs/tests/nvidia_fp_temp_07_robust_exact_jit.md
  crates/simthing-driver/tests/phase_m_jit_grad0_spatial_observer.rs
  docs/accumulator_op_v2_production_plan.md
  docs/workshop/mapping_current_guidance.md

Current interpretation:
  Likely stale policy/doc-string guard.
  The test reads docs/accumulator_op_v2_production_plan.md, but that file is closed/archived and says not to use it as the active production track.
  Shader path avoids sqrt and uses mag2.
  This is not native sqrt and not NVIDIA FP tolerance drift.

Opus decision needed:
  Retarget guard to active 0.0.8 guidance, replace it with code-level authority classification, or remove stale doc-string guard.
```

### Battery 07 — admission-ordering guard

```text
Item:
  jit_exec1_distinct_graphs_remain_separate_entries

Evidence:
  docs/tests/nvidia_fp_temp_07_robust_exact_jit.md
  crates/simthing-driver/tests/phase_m_jit_exec1_cohort_execution_fixture.rs

Current interpretation:
  Likely admission-ordering / harness guard issue.
  Mixed/distinct cohort should reject before GPU execution helper is invoked.
  This is not native sqrt and not NVIDIA FP tolerance drift.

Opus decision needed:
  Decide whether the guard expectation is correct and where rejection should happen.
```

### Battery 08 — missing workshop doc include

```text
Item:
  phase_m_boundary_cadence_doctrine

Evidence:
  docs/tests/nvidia_fp_temp_08_sead_boundary_scheduler.md
  crates/simthing-driver/tests/phase_m_boundary_cadence_doctrine.rs
  docs/workshop/mapping_current_guidance.md
  missing docs/workshop/workshop_current_state.md

Current interpretation:
  Stale/missing doc-hygiene dependency.
  Test cannot compile because it includes missing docs/workshop/workshop_current_state.md.
  Executed SEAD/scheduler tests passed on RTX.
  This is not NVIDIA FP drift and not SEAD runtime failure.

Opus decision needed:
  Restore missing doc, retarget guard to active 0.0.8 docs, or remove stale doc-string guard.
```

### Battery 11 — workspace compile skew

```text
Item:
  phase_m_jit_desc0_kernel_descriptor workspace compile stop

Evidence:
  docs/tests/nvidia_fp_temp_11_feeder_workspace_sweep.md
  crates/simthing-driver/tests/phase_m_jit_desc0_kernel_descriptor.rs
  KernelDescriptor definition / descriptor call sites

Current interpretation:
  Workspace smoke stops at compile before a full workspace test tally.
  Error includes KernelDescriptor field skew, including exact_sqrt_artifact.
  simthing-feeder itself passed 5/0/0 on RTX.
  This is not NVIDIA FP drift, but it prevents full workspace closure.

Opus decision needed:
  Decide whether descriptor test is stale relative to current KernelDescriptor shape, or whether descriptor struct/call sites need a design-authority remedial.
```

### Current interpretation

Battery 12 resolved the four previously parked Battery 07, Battery 08, and Battery 11 items. Keep the historical detail above visible for provenance. Do **not** mark the full NVIDIA sweep complete until the remaining `simthing-spec` workspace failure is accepted as non-blocking or remediated in a separate handoff.

- Resolved: Battery 07 `jit_grad0`: not native `sqrt`; stale active-policy check; not NVIDIA FP drift.
- Resolved: Battery 07 `jit_exec1`: admission/harness ordering; not NVIDIA FP drift.
- Resolved: Battery 08 `phase_m_boundary_cadence_doctrine`: missing doc include; not NVIDIA FP drift.
- Resolved: Battery 11 `phase_m_jit_desc0_kernel_descriptor`: workspace compile skew; feeder PASS on RTX; not NVIDIA FP drift.
- Still open: `simthing-spec --test jit_kernel_cohort_preview::jit_cohort0_distinct_graphs_split`; workspace failure, not NVIDIA FP drift.

## Temporary sweep status table

| Battery | Evidence file | Status | RTX adapter proven | Result | Notes |
|---|---|---:|---:|---|---|
| 01 | `docs/tests/nvidia_fp_temp_01_adapter_gate.md` | PASS | yes | STORE-GPU smoke/adapter gate passed | Integer exact smoke path |
| 02 | `docs/tests/nvidia_fp_temp_02_pack_gpu.md` | PASS | yes | PACK-GPU EC-A2b GpuVerified passed | f32 tolerance path |
| 03 | `docs/tests/nvidia_fp_temp_03_structured_field.md` | PASS | yes | structured-field batch passed | Direct adapter proof after remedial |
| 04 | `docs/tests/nvidia_fp_temp_04_atlas_protocol_and_m5_gradients.md` | PASS | yes | atlas/M5 batch passed | Direct adapter proof after remedial |
| 05 | `docs/tests/nvidia_fp_temp_05_first_slice.md` | PASS | yes | first-slice family passed | substitution recorded |
| 06 | `docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md` | PASS | yes | sim f32 C-series passed | exact integration binaries used |
| 07 | `docs/tests/nvidia_fp_temp_07_robust_exact_jit.md` | FAIL / TRIAGE | yes | 150 pass / 2 fail / 3 ignored | likely stale doc guard + admission-ordering issue; not NVIDIA FP drift |
| 08 | `docs/tests/nvidia_fp_temp_08_sead_boundary_scheduler.md` | PARTIAL / TRIAGE | yes | 105 pass / 0 fail / 0 ignored; 1 blocked | missing doc include blocks boundary cadence test |
| 09 | `docs/tests/nvidia_fp_temp_09_runtime_eml_economy_nested.md` | PASS | yes | 106 pass / 0 fail / 0 ignored | runtime/EML/economy/nested/session passed |
| 10 | `docs/tests/nvidia_fp_temp_10_sim_broad_integration.md` | PASS | yes | 106 pass / 0 fail / 1 ignored | broad simthing-sim sweep passed; ignored perf test not correctness failure |
| 11 | `docs/tests/nvidia_fp_temp_11_feeder_workspace_sweep.md` | PARTIAL / KNOWN TRIAGE | yes | feeder 5/0/0; workspace compile stopped | feeder PASS; workspace incomplete (JIT descriptor compile + known triage family) |
| 12 | `docs/tests/nvidia_fp_temp_12_remedial_retest.md` | PARTIAL / TARGETED PASS; WORKSPACE OPEN | yes | adapter 1/0/0; targeted remedials 25/0/0; workspace fails 1 simthing-spec test | Battery 07/08/11 blockers resolved; NVIDIA sweep remains parked pending `jit_cohort0_distinct_graphs_split` |

## 1. Hard verification gate

Every result file for this temporary track must show:

```text
adapter/device: NVIDIA GeForce RTX 4080 Laptop GPU
```

or equivalent selected-adapter evidence proving the selected adapter contains one of:

```text
NVIDIA
RTX
4080
```

A test run that names `Intel(R) RaptorLake-S` is **not accepted** for this track.

Recommended env:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1
$env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"
$env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1
```

If a battery cannot force/confirm the NVIDIA adapter, mark the battery `BLOCKED`, save the log, and stop. Do not claim PASS.

## 2. Result-file convention

Save every battery as Markdown under `docs/tests/`:

```text
docs/tests/nvidia_fp_temp_01_adapter_gate.md
docs/tests/nvidia_fp_temp_02_pack_gpu.md
docs/tests/nvidia_fp_temp_03_structured_field.md
docs/tests/nvidia_fp_temp_04_first_slice.md
docs/tests/nvidia_fp_temp_05_m5_gradients.md
docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
docs/tests/nvidia_fp_temp_07_robust_exact.md
docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md
```

Use the template in `docs/tests/nvidia_fp_temp_00_result_template.md` for every result file.

Each file must include:

- exact command(s);
- adapter inventory / selected adapter when printed;
- test count and pass/fail/ignored count;
- tolerance or parity standard;
- final status: `PASS`, `FAIL`, or `BLOCKED`;
- short note on whether the result replaces prior Intel-only evidence.

## 3. Battery 01 — adapter gate and smoke proof

**Result file:** `docs/tests/nvidia_fp_temp_01_adapter_gate.md`

Purpose: prove the local test environment is routing `GpuContext::new_blocking()` to the RTX before burning time on the longer batteries.

Suggested commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_01_adapter_gate.md
```

Then run the already-proven exact STORE-GPU battery as a smoke test:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_01_adapter_gate.md
```

Acceptance:

- selected adapter is NVIDIA / RTX / 4080;
- STORE-GPU remains `10 passed; 0 failed; 0 ignored` or better;
- parity remains exact for the integer STORE-GPU path.

## 4. Battery 02 — ATLAS PACK-GPU f32 `GpuVerified`

**Result file:** `docs/tests/nvidia_fp_temp_02_pack_gpu.md`

Purpose: re-run EC-A2b `GpuVerified` on RTX. The previous accepted PACK-GPU result was Intel-only and tolerance-based.

Command:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_pack_gpu -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_02_pack_gpu.md
```

Acceptance:

- selected adapter is NVIDIA / RTX / 4080;
- all PACK-GPU tile classes pass the accepted `GpuVerified` tolerance;
- do **not** upgrade claim to bit-exact unless test code explicitly proves bit-exact and Opus accepts that stronger standard.

## 5. Battery 03 — structured field stencil / structured region f32 suite

**Result file:** `docs/tests/nvidia_fp_temp_03_structured_field.md`

Purpose: re-prove f32 structured-field stencil and parent EML/region execution on NVIDIA.

Suggested commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-gpu structured_field_stencil -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_03_structured_field.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver structured_field -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_03_structured_field.md
```

Targets expected in this battery:

- `simthing-gpu` — `structured_field_stencil`
- `simthing-driver` — `structured_field_region_execution`
- `simthing-driver` — `structured_field_stencil_parent_eml`

Acceptance:

- selected adapter is NVIDIA / RTX / 4080 where GPU logs are printed;
- all f32 tolerance checks pass under their existing standards;
- timing/perf output, if present, is diagnostic only unless timestamp-query-backed.

## 6. Battery 04 — Phase M first-slice f32/product suite

**Result file:** `docs/tests/nvidia_fp_temp_04_first_slice.md`

Purpose: re-run the first-slice runtime/product fixture/product commitment/map residency/queue write/scenario spec/summary validity group on NVIDIA.

Suggested command:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_first_slice -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_04_first_slice.md
```

Expected target family:

- `phase_m_first_slice_runtime`
- `phase_m_first_slice_product_fixture`
- `phase_m_first_slice_product_commitment`
- `phase_m_first_slice_map_residency`
- `phase_m_first_slice_queue_write_hardening`
- `phase_m_first_slice_scenario_spec`
- `phase_m_first_slice_summary_validity`

Acceptance:

- all selected first-slice tests pass on RTX;
- any skipped GPU test is explicitly justified;
- no production claim is changed by timing alone.

## 7. Battery 05 — M5 gradients / f32 gradient sensitivity

**Result file:** `docs/tests/nvidia_fp_temp_05_m5_gradients.md`

Purpose: re-run M5 gradient families on NVIDIA because f32 gradients are adapter-sensitive.

Suggested commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_m5b -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_05_m5_gradients.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_m5c -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_05_m5_gradients.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_m5e -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_05_m5_gradients.md
```

Also include the atlas protocol oracle because it was listed as priority:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test phase_m_c0_m4_atlas_protocol_oracle -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_05_m5_gradients.md
```

Acceptance:

- all gradient and atlas protocol oracle tests pass;
- tolerance standard remains the existing test-defined standard;
- do not assert bit-exact unless explicitly proven.

## 8. Battery 06 — simthing-sim f32 C-series parity

**Result file:** `docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md`

Purpose: re-run f32 C-series parity tests on NVIDIA.

Suggested commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c1_threshold_scan_parity -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c2_intent_accumulator_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c3_overlay_add_accumulator_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c4_overlay_orderband_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c5_weighted_mean_reduction_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c7_velocity_accumulator_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c8b_intensity_eml_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md
```

Acceptance:

- all seven f32 parity tests pass on RTX;
- if an individual test is CPU-only despite being listed in the Intel inventory, record that it is CPU-only / no adapter evidence and keep the battery status explicit.

## 9. Battery 07 — robust exact / low-risk confirmation

**Result file:** `docs/tests/nvidia_fp_temp_07_robust_exact.md`

Purpose: confirm adapter-independent exact/integer batteries on NVIDIA after the global discrete-adapter fix. These are lower risk but should be recorded.

Suggested commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_jit_sqrt -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_jit_grad -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_jit_exec -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_jit_prod0 -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver phase_m_jit_evaleml_wgsl_prototype -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c6_exact_reduction_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c8c_transfer_accumulator_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim c8d_emission_accumulator_parity -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_07_robust_exact.md
```

Acceptance:

- all exact/integer tests pass;
- if a test has no GPU adapter evidence, mark it as exact CPU parity confirmation rather than NVIDIA GPU evidence.

## 10. Battery 08 — remaining GPU-touching broad sweep

**Result file:** `docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md`

Purpose: run the broader GPU-touching inventory after priority f32 batteries pass.

Suggested first-pass command:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test --workspace -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md
```

If the full workspace is too large/noisy, split by crate:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-sim -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS="RTX"; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-feeder -- --nocapture *>&1 | Tee-Object -Append docs/tests/nvidia_fp_temp_08_workspace_gpu_touching.md
```

Acceptance:

- correctness asserts pass;
- perf-number differences are diagnostic unless timestamp-query-backed;
- failures must be triaged into: adapter-specific correctness drift, flaky/perf-only, unrelated existing failure, or blocked.

## 11. Temporary completion checklist

When all batteries are complete, create:

```text
docs/tests/nvidia_fp_temp_99_summary.md
```

The summary must include:

- list of all temp files generated;
- final status per battery;
- adapter evidence per battery;
- failed/blocked commands, if any;
- whether `ATLAS-BATCH-0-CLOSE` can proceed;
- explicit statement that all `nvidia_fp_temp*.md` files are temporary and can be deleted once results are folded into permanent docs.

Do **not** delete the temp files until the principal approves the cleanup.

## 12. Production doc policy

During the walk-through, keep permanent production docs unchanged unless a battery proves a stable closeout state that should be recorded.

Once the sweep closes, fold only the durable conclusions into:

```text
docs/design_0_0_8_0_consumer_pulled_production_track.md
docs/worklog.md
```

Do not preserve every raw temp log permanently unless Opus/Codex decides a file is durable evidence.

## 13. Stop conditions

Stop and escalate if:

- selected adapter is Intel for any required GPU battery;
- adapter inventory does not include NVIDIA / RTX / 4080;
- a f32 tolerance check fails only on NVIDIA;
- a test requires a new tolerance without design-authority approval;
- a test needs new WGSL or GPU/core changes outside the battery;
- failures cannot be classified cleanly.

## 14. Handoff-back format for each battery

```text
Recipient model: Opus
Role: design authority

NVIDIA FP determinism battery <NN> complete.

Temp result file:
- docs/tests/nvidia_fp_temp_<NN>_<name>.md

Commands:
- ...

Adapter:
- selected: ...
- target matched: true/false

Results:
- passed: ...
- failed: ...
- ignored/skipped: ...

Tolerance / parity:
- ...

Status:
- PASS / FAIL / BLOCKED

Notes:
- ...
```

## 15. §0.5 self-check

This temporary track is evidence hygiene only. It does not add gameplay resource-flow behavior, does not alter recursive allocation, does not introduce CPU planner decisions, does not add `simthing-sim` semantics, and does not default-wire any production session. It validates that GPU-dependent f32/tolerance-sensitive evidence is now collected on the discrete RTX 4080 rather than the formerly selected Intel iGPU.
