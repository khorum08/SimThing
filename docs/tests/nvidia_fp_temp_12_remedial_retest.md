# NVIDIA FP Temp Battery 12 - Remedial Retest

**Recipient model:** ChatGPT / production harness
**Role:** production implementation agent
**Date:** 2026-06-04
**Status:** PARTIAL / TARGETED REMEDIAL PASS; WORKSPACE STILL OPEN

Battery 12 remediated the four parked non-NVIDIA-FP blockers from Batteries 07, 08, and 11 without changing tolerances, shader math, WGSL, or permanent production-track docs.

## Adapter Gate

Command:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store_gpu gpu_adapter_is_discrete_rtx_target -- --nocapture *>&1 | Tee-Object docs/tests/nvidia_fp_temp_12_remedial_retest.md
```

Evidence:

```text
selected_adapter_name: NVIDIA GeForce RTX 4080 Laptop GPU
adapter_target_matched: true
selected_adapter_is_discrete_rtx: true
gpu_tier_ran: true
test result: ok. 1 passed; 0 failed; 0 ignored; 9 filtered out
```

Note: PowerShell returned a nonzero command status because cargo warning output was emitted on stderr while the test itself passed. Subsequent retests used `cmd /c "... 2>&1"` to avoid PowerShell error-record handling.

## Remedial Scope

1. `jit_grad0_mag2_not_overclaimed_if_approximate`
   - Before: stale doc guard referenced archived `docs/accumulator_op_v2_production_plan.md`.
   - Change: retargeted to active 0.0.8 docs and added code-level assertions that the observer WGSL does not call `sqrt(` and writes squared magnitude as `mag2`.
   - Result: final retest passed; one intermediate remedial retest failed on an overly literal active-doc string (`raw f32` vs `Raw f32`) and was corrected before final evidence.

2. `jit_exec1_distinct_graphs_remain_separate_entries`
   - Before: test used a global atomic execution tripwire that could race under parallel test execution.
   - Change: split admission-only validation from GPU execution, so distinct graphs reject before constructing a GPU context.
   - Result: retest passed.

3. `phase_m_boundary_cadence_doctrine`
   - Before: compile depended on missing `docs/workshop/workshop_current_state.md`.
   - Change: retargeted the doc guard to active 0.0.8 and workshop guidance files that exist.
   - Result: retest passed.

4. `phase_m_jit_desc0_kernel_descriptor`
   - Before: local test descriptor literals included fields no longer present in the fixture-local `KernelDescriptor` shape.
   - Change: removed the stale local fields without changing production descriptor types.
   - Result: retest passed.

## Targeted Retests

Commands:

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cmd /c "cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture 2>&1" | Tee-Object -Append docs/tests/nvidia_fp_temp_12_remedial_retest.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cmd /c "cargo test -p simthing-driver --test phase_m_jit_exec1_cohort_execution_fixture -- --nocapture 2>&1" | Tee-Object -Append docs/tests/nvidia_fp_temp_12_remedial_retest.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cmd /c "cargo test -p simthing-driver --test phase_m_boundary_cadence_doctrine -- --nocapture 2>&1" | Tee-Object -Append docs/tests/nvidia_fp_temp_12_remedial_retest.md
```

```powershell
$env:SIMTHING_RUN_GPU_TESTS=1; $env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1; cmd /c "cargo test -p simthing-driver --test phase_m_jit_desc0_kernel_descriptor -- --nocapture 2>&1" | Tee-Object -Append docs/tests/nvidia_fp_temp_12_remedial_retest.md
```

Final targeted results:

| Test target | Result | Timing note |
|---|---:|---|
| `phase_m_jit_grad0_spatial_observer` | PASS - 8 passed / 0 failed / 0 ignored | finished in 2.52s |
| `phase_m_jit_exec1_cohort_execution_fixture` | PASS - 5 passed / 0 failed / 0 ignored | finished in 0.99s |
| `phase_m_boundary_cadence_doctrine` | PASS - 7 passed / 0 failed / 0 ignored | finished in 1.02s |
| `phase_m_jit_desc0_kernel_descriptor` | PASS - 5 passed / 0 failed / 0 ignored | finished in 0.00s |

Observed targeted evidence:

```text
mag2_r1_classification=ApproximateJitOnly
descriptor_evidence: grad0_mag2=ApproximateDiagnostic grad1_score=ExactAuthoritative sqrt=ApproximateDiagnostic all=TestOnly
```

## Workspace Retest

Command:

```powershell
$env:SIMTHING_RUN_GPU_TESTS='1'
$env:SIMTHING_GPU_ADAPTER_CONTAINS='RTX'
$env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'
cmd /c "cargo test --workspace -- --nocapture >> docs\tests\nvidia_fp_temp_12_remedial_retest.md 2>&1"
```

Workspace result:

```text
FAILED: simthing-spec --test jit_kernel_cohort_preview
test jit_cohort0_distinct_graphs_split ... FAILED
left: ["variant"]
right: ["base"]
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
error: test failed, to rerun pass `-p simthing-spec --test jit_kernel_cohort_preview`
```

Classification:

- The remaining failure is outside the four Battery 12 remedial files and outside the NVIDIA FP tolerance/shader-math surface.
- The local diff does not touch `simthing-spec`; the failure is a cohort preview ordering assertion, not RTX f32 drift.
- Full NVIDIA sweep closure remains parked pending follow-up triage or a separate accepted remedial for `jit_cohort0_distinct_graphs_split`.

## Remaining Failures

Still open:

```text
cargo test --workspace -- --nocapture
  simthing-spec --test jit_kernel_cohort_preview
  jit_cohort0_distinct_graphs_split
  left: ["variant"]
  right: ["base"]
```

Resolved by Battery 12:

- Battery 07 stale `jit_grad0` active-policy guard.
- Battery 07 `jit_exec1` admission-before-GPU guard.
- Battery 08 missing workshop doc include.
- Battery 11 fixture-local descriptor field skew.

## Section 0.5 Self-Check

Battery 12 is evidence hygiene and test-harness repair only. It does not add gameplay resource-flow behavior, does not alter recursive allocation, does not introduce CPU planner decisions, does not add `simthing-sim` semantics, does not change WGSL, does not change f32 tolerances, and does not default-wire any production session.

## Handoff Back

Recipient model: ChatGPT / production harness
Role: production implementation agent

NVIDIA FP remedial Battery 12 complete.

Status:
- Targeted remedial tests: PASS.
- RTX adapter gate: PASS, selected NVIDIA GeForce RTX 4080 Laptop GPU.
- Full workspace retest: FAIL, still open on `simthing-spec --test jit_kernel_cohort_preview::jit_cohort0_distinct_graphs_split`.

NVIDIA sweep remains parked pending the remaining workspace remedial. Battery 12 resolves the four previously parked Batteries 07/08/11 blockers and does not justify durable production-doc fold-in yet.
