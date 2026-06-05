# NVIDIA FP temporary sweep summary — parked for Opus triage

**Temporary file:** `docs/tests/nvidia_fp_temp_99_summary.md`  
**Track:** `docs/nvidia_fp_determinism_test.md`  
**Date:** 2026-06-03  
**Status:** `CLOSED / COMPLETE (2026-06-04, Opus)`

> **Superseded (2026-06-04, Opus) — the content below is historical.** **Current state: the NVIDIA ladder is COMPLETE.** Battery 07 (stale doc guard; admission ordering), Battery 08 (missing doc include), Battery 11 (descriptor compile skew) → **Resolved by Battery 12.** The last Still-Open item, `simthing-spec jit_kernel_cohort_preview::jit_cohort0_distinct_graphs_split` → **Resolved by Battery 13** (test corrected to assert split membership order-insensitively; the cohort `BTreeMap` `stable_key` ordering is the intended determinism — impl unchanged). **Full `cargo test --workspace` is green on the discrete NVIDIA RTX 4080 — 60 binaries, 0 failed.** Adapter-scope caveat lifted. Closeout evidence: `nvidia_fp_temp_13_workspace_closeout.md`.

NVIDIA RTX 4080 priority validation is substantially complete, but the sweep is parked rather than closed because four non-NVIDIA-FP triage items remain open.

Do **not** interpret this file as:

- full workspace PASS
- full NVIDIA sweep closed
- all GPU-touching tests green
- production docs finalized

---

## Evidence table

| Battery | Evidence file | Status | RTX adapter proven | Result | Interpretation |
|---|---|---:|---:|---|---|
| 01 | `docs/tests/nvidia_fp_temp_01_adapter_gate.md` | PASS | yes | STORE-GPU smoke/adapter gate passed | RTX path verified |
| 02 | `docs/tests/nvidia_fp_temp_02_pack_gpu.md` | PASS | yes | PACK-GPU EC-A2b GpuVerified passed | f32 tolerance path validated on RTX |
| 03 | `docs/tests/nvidia_fp_temp_03_structured_field.md` | PASS | yes | structured-field batch passed | direct adapter proof after remedial |
| 04 | `docs/tests/nvidia_fp_temp_04_atlas_protocol_and_m5_gradients.md` | PASS | yes | atlas/M5 batch passed | direct adapter proof after remedial |
| 05 | `docs/tests/nvidia_fp_temp_05_first_slice.md` | PASS | yes | first-slice family passed | substitution recorded |
| 06 | `docs/tests/nvidia_fp_temp_06_sim_f32_c_series.md` | PASS | yes | sim f32 C-series passed | exact integration binaries used |
| 07 | `docs/tests/nvidia_fp_temp_07_robust_exact_jit.md` | FAIL / TRIAGE | yes | 150 pass / 2 fail / 3 ignored | failures are doc-hygiene/admission-ordering, not NVIDIA FP drift |
| 08 | `docs/tests/nvidia_fp_temp_08_field_policy_boundary_scheduler.md` | PARTIAL / TRIAGE | yes | 105 pass / 0 fail / 0 ignored; 1 blocked | missing doc include blocks boundary cadence test |
| 09 | `docs/tests/nvidia_fp_temp_09_runtime_eml_economy_nested.md` | PASS | yes | 106 pass / 0 fail / 0 ignored | runtime/EML/economy/nested/session passed |
| 10 | `docs/tests/nvidia_fp_temp_10_sim_broad_integration.md` | PASS | yes | 106 pass / 0 fail / 1 ignored | broad simthing-sim sweep passed; ignored perf test not correctness failure |
| 11 | `docs/tests/nvidia_fp_temp_11_feeder_workspace_sweep.md` | PARTIAL / KNOWN TRIAGE | yes | feeder 5/0/0; workspace compile stopped | feeder PASS; workspace incomplete due JIT descriptor compile skew |

**Adapter (all executed batteries):** NVIDIA GeForce RTX 4080 Laptop GPU, `adapter_target_matched: true`, Intel not selected.

---

## Parked Opus triage index

### 4.1 Battery 07 — stale doc-hygiene guard

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

### 4.2 Battery 07 — admission-ordering guard

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

### 4.3 Battery 08 — missing workshop doc include

```text
Item:
  phase_m_boundary_cadence_doctrine

Evidence:
  docs/tests/nvidia_fp_temp_08_field_policy_boundary_scheduler.md
  crates/simthing-driver/tests/phase_m_boundary_cadence_doctrine.rs
  docs/workshop/mapping_current_guidance.md
  missing docs/workshop/workshop_current_state.md

Current interpretation:
  Stale/missing doc-hygiene dependency.
  Test cannot compile because it includes missing docs/workshop/workshop_current_state.md.
  Executed FIELD_POLICY/scheduler tests passed on RTX.
  This is not NVIDIA FP drift and not FIELD_POLICY runtime failure.

Opus decision needed:
  Restore missing doc, retarget guard to active 0.0.8 docs, or remove stale doc-string guard.
```

### 4.4 Battery 11 — workspace compile skew

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

---

## Conclusion

The RTX 4080 validation ladder has enough evidence to say the priority f32/tolerance-sensitive batteries passed on NVIDIA for all non-triage targets that executed. The sweep should remain parked, not closed, because Battery 07, Battery 08, and the Battery 11 workspace smoke expose non-FP triage issues requiring Opus review.

No source, shader, math, tolerance, gameplay, FIELD_POLICY semantic, recursive allocation, or production-session changes were made by this evidence sweep.

---

## Temporary cleanup warning

All `docs/tests/nvidia_fp_temp*.md` files are temporary. Do not delete them until Opus reviews the parked triage index and durable conclusions are folded into permanent docs.

Permanent docs not yet updated:

- `docs/design_0_0_8_0_consumer_pulled_production_track.md`
- `docs/worklog.md`

---

## §0.5 check

Documentation-only parked summary; no gameplay resource-flow behavior, no recursive allocation change, no CPU planner logic, no shader/math/tolerance/source changes, no simthing-sim semantic expansion, no default session wiring.
