# Phase M-JIT-PROD-0 — Production Registry Shell — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `eff5b05` (M-JIT-EXEC-1 merge on `master`)  
**Lane classification:** Tier-2 GPU/JIT default-off production registry shell (V7.7 §5)  
**Decision:** **IMPLEMENTED — ProductionKernelRegistryShell + doc surface cleanup**  
**Verdict:** **PASS**

---

## Production Shell Design

**Spec layer** (`simthing-spec`):

- `ProductionKernelRegistryShell` / `ProductionKernelRegistryShellConfig`
- `RegisteredProductionCandidate`
- `register_production_candidate` — accepts `ProductionCandidatePreview` only
- `require_registered_for_execution` — explicit execution gate
- `validate_production_candidate_preview_entry` — shared REG-1 validation rules

**Driver layer** (test fixture only):

- `phase_m_jit_prod0_registry_shell.rs` — REG→shell→GPU execution path

No runtime cache, scheduler, SimSession wiring, or driver lib integration.

---

## Registration Rules

1. Only `ProductionCandidatePreview` entries register.
2. `default_off == true`, `production_wiring == false`.
3. Stable key `jit-graph-v1:` prefix; semantic-free canonical text.
4. Rejects `mag2`, sqrt, `ApproximateJitOnly`, approximate markers.
5. **Duplicate policy:** idempotent re-register when stable key + canonical text + request IDs match byte-for-byte; reject same key with different canonical text.

---

## Execution Rules

1. `require_registered_for_execution(stable_key)` must pass before GPU helper runs.
2. Exact GRAD-0→scorer cohort path only (same fused WGSL as EXEC-1).
3. Test-invoked explicit opt-in only.

---

## Default-Off Result

`MappingExecutionProfile::default() == Disabled`. Registered candidates remain `default_off=true`, `production_wiring=false`.

---

## Exact Candidate Execution Result

Registered exact cohort: **20,000** observers, **1** dispatch, per-segment sampled CPU/GPU oracle parity bit-exact.

---

## Approximate / sqrt / mag2 Rejection Result

| Case | Result |
|------|--------|
| mag2 at REG-1 promotion | **Rejected** |
| sqrt descriptor | **Rejected** |
| TestOnly at shell register | **Rejected** |

---

## Duplicate Identity Policy

Idempotent identical re-register returns same entry; same key + different canonical text **rejects**.

---

## Documentation Cleanup Summary

**Deleted (not archived):**

- `docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md`
- `docs/tests/phase_m_jit_desc0_kernel_descriptor_test_results.md`
- `docs/tests/phase_m_jit_desc1_kernel_descriptor_admission_test_results.md`
- `docs/tests/phase_m_jit_grad0_spatial_observer_test_results.md`
- `docs/tests/phase_m_jit_sqrt_candidate_battery_test_results.md`
- `docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md`
- `docs/tests/phase_m_jit_evaleml_wgsl_prototype_test_results.md`
- `docs/workshop/e11_implementation_handoff.md`
- `docs/workshop/e11_readiness_review.md`
- `docs/workshop/pivot_forward_implementation_policy.md`

**Retained for Opus review:**

- `phase_m_jit_prod0_registry_shell_test_results.md` (this report)
- EXEC-1, EXEC-0, REG-1, COHORT-0 R1, KEY-0, DESC-2, GRAD-1, GRAD-0 R1, SQRT R1

**References updated:** `mapping_current_guidance.md`, `workshop_current_state.md`, `accumulator_op_v2_production_plan.md`, `worklog.md`

**No archive created.**

---

## Tests Run and Results

```
cargo test -p simthing-driver --test phase_m_jit_prod0_registry_shell -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-driver --test phase_m_jit_exec1_cohort_execution_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_jit_exec0_production_candidate_fixture -- --nocapture
cargo test -p simthing-spec --test jit_kernel_registry_admission -- --nocapture
cargo test -p simthing-spec --test jit_kernel_registry_preview -- --nocapture
cargo test -p simthing-spec --test jit_kernel_cohort_preview -- --nocapture
cargo test -p simthing-spec --test jit_kernel_graph_identity -- --nocapture
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture
cargo test -p simthing-driver --test phase_m_jit_grad1_observer_formula_fusion -- --nocapture
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
cargo check --workspace
```

**Result:** All green.

---

## Transient Cleanup Result

Historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-spec/src/compile/jit_kernel_production_registry_shell.rs`
- `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-driver/tests/phase_m_jit_prod0_registry_shell.rs`
- `docs/tests/phase_m_jit_prod0_registry_shell_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no production registry runtime cache, no production scheduler, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no ClauseThing runtime/front-end implementation, no new production EML opcode, no production sqrt admission, no approximate mag2/sqrt candidate executed, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-PROD-0 is a default-off production-shaped registry shell with explicit opt-in registered exact cohort execution only; V7.7 / Mapping ADR / SEAD GPU-resident posture intact.
