# Phase M-JIT-COHORT-0 R1 — Collision-Test Helper Fence — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `05b9c175c1288c44f04b2f4ee60bb866f95ab643` (M-JIT-COHORT-0 merge on `master`)  
**Final commit SHA:** _(set at commit; authoritative post-merge SHA is the GitHub squash-merge commit)_  
**Lane classification:** Tier-2 GPU/JIT remedial guardrail pass (V7.7 §5)  
**Decision:** **IMPLEMENTED — collision-test helper fenced from public spec API**  
**Verdict:** **PASS**

---

## Found Issue

| Finding | Detail |
|---------|--------|
| Helper visibility | `test_group_cohort_previews_from_resolved` was `pub` with `#[doc(hidden)]`, not `#[cfg(test)]`. |
| Re-export | Helper was re-exported from `compile/mod.rs` and `lib.rs`. |
| Bypass risk | Helper allowed injected `(stable_key, canonical_text)` without `preview_kernel_graph_identity`. |
| Report mismatch | COHORT-0 report described helper as test-only/`#[cfg(test)]`; implementation did not match. |

Public preview path remains: `preview_kernel_graph_cohorts(&[KernelGraphRequestSpec])`.

---

## Fix Applied

1. Removed `test_group_cohort_previews_from_resolved` from `jit_kernel_cohort_preview.rs`.
2. Removed helper re-exports from `compile/mod.rs` and `lib.rs`.
3. Moved collision-test helper into `tests/jit_kernel_cohort_preview.rs` as a test-local function mirroring grouping collision guard logic.
4. Extended source scan test to assert cohort module does not contain helper name.

Cohort grouping semantics unchanged; `group_resolved_requests` remains private in production module.

---

## Collision Guard Test Result

`jit_cohort0_rejects_same_key_different_canonical_text` passes using test-local injected identities; rejects with "conflicting canonical text".

---

## Public API Confirmation

Exported from `simthing-spec`:

- `KernelGraphRequestSpec`
- `KernelGraphCohortPreview`
- `KernelGraphCohortPreviewSet`
- `preview_kernel_graph_cohorts`

No injected-identity helper in `src/`.

---

## Tests Run and Results

```
cargo test -p simthing-spec --test jit_kernel_cohort_preview -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-spec --test jit_kernel_graph_identity -- --nocapture
```

**Result:** 7 passed.

```
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture
```

**Result:** 11 passed.

```
cargo test -p simthing-spec --test jit_kernel_descriptor_admission -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-driver --test phase_m_jit_desc0_kernel_descriptor -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad1_observer_formula_fusion -- --nocapture
```

**Result:** 5 passed.

```
cargo test -p simthing-driver --test phase_m_jit_grad0_spatial_observer -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
```

**Result:** 8 passed.

```
cargo check --workspace
```

**Result:** PASS.

---

## Scans Run and Results

```
rg "test_group_cohort_previews_from_resolved" crates/simthing-spec/src crates/simthing-spec/tests docs/tests/phase_m_jit_cohort0_r1_collision_helper_fence_test_results.md
```

**Result:** no match in `src/`; helper only in test file and R1 report.

```
rg "pub fn test_group|doc\(hidden\).*test_group|pub use .*test_group_cohort" crates/simthing-spec/src
```

**Result:** no matches.

```
rg "KernelGraphRequestSpec|KernelGraphCohortPreview|preview_kernel_graph_cohorts|stable_key|canonical_text|cohort" crates/simthing-spec docs/tests/phase_m_jit_cohort0_r1_collision_helper_fence_test_results.md
```

**Result:** public preview API intact; collision guard still tested.

```
rg "dispatch_workgroups|create_shader_module|GpuContext|EmlGpuProgramTable|AccumulatorOpSession|tick_with_eml|kernel cache|scheduler|registry" crates/simthing-spec/src/compile/jit_kernel* crates/simthing-spec/tests/jit_kernel* docs/tests/phase_m_jit_cohort0_r1_collision_helper_fence_test_results.md
```

**Result:** no GPU dispatch/runtime session/production registry/scheduler/cache in cohort preview.

```
rg "production JIT|observer scheduling|observer caching|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim.*Gadget|simthing-sim.*Personality|simthing-sim.*Memory|chained OrderBand|automatic snapshot|atlas|M-4A|ActiveOnlyExperimentalNoHalo|source_mask|source identity" crates docs
```

**Result:** guardrail/deferred context only; no new production/default wiring.

---

## Transient-Log Cleanup Result

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

(PowerShell equivalent used.)

**Result:** historical `*_full.log` files only; no scratch/tmp artifacts removed.

---

## Files Changed

- `crates/simthing-spec/src/compile/jit_kernel_cohort_preview.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_cohort_preview.rs`
- `docs/tests/phase_m_jit_cohort0_r1_collision_helper_fence_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-COHORT-0 R1 fences the collision-test helper out of the public spec API while preserving cohort grouping preview behavior; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
