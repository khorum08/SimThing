# Phase M-JIT-COHORT-0 — Kernel Graph Cohort Grouping Preview — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `4d6d856a8baacc5c66a904d7e284c93d1b66774b` (M-JIT-KEY-0 merge on `master`)  
**Final commit SHA:** _(set at commit time; authoritative post-merge SHA is the GitHub squash-merge commit)_  
**Lane classification:** Tier-2 GPU/JIT cohort grouping preview (V7.7 §5)  
**Decision:** **IMPLEMENTED — spec-layer kernel graph cohort grouping preview**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected KEY-0 identity, DESC-2 graph admission, DESC-1 descriptor admission, prior reports, mapping guidance, accumulator plan, and invariants.

| Question | Answer |
|----------|--------|
| Can many graph requests be grouped by identity without scheduling? | Yes — `preview_kernel_graph_cohorts` calls `preview_kernel_graph_identity` per request then groups by `stable_key`. |
| Can identical graphs with different authoring order land in the same cohort? | Yes — KEY-0 canonicalization normalizes node/edge order before grouping. |
| Can distinct graphs land in distinct cohorts? | Yes — different canonical text → different stable keys. |
| Can invalid graphs be rejected before grouping? | Yes — identity preview runs admission first; batch aborts on first failure. |
| Can grouping preserve canonical text? | Yes — each cohort stores `canonical_text`; collision guard rejects same key with different text. |
| Can this stay semantic-free and spec-only? | Yes — no GPU/runtime imports; descriptor content unchanged. |
| Which production gates remain closed? | Production registry, scheduler, runtime kernel cache, observer scheduling/caching, JIT cohort dispatch, default mapping, economy→mapping bridge. |

---

## Cohort Preview Model

Module: `crates/simthing-spec/src/compile/jit_kernel_cohort_preview.rs`

```rust
KernelGraphRequestSpec { request_id, graph }
KernelGraphCohortPreview { stable_key, canonical_text, request_ids }
KernelGraphCohortPreviewSet { cohorts }

preview_kernel_graph_cohorts(&[KernelGraphRequestSpec]) -> Result<KernelGraphCohortPreviewSet, SpecError>
```

Test-only collision helper: `test_group_cohort_previews_from_resolved` (`#[cfg(test)]`).

---

## Grouping Rules

1. Reject empty request set.
2. Reject duplicate request IDs.
3. Per request: `preview_kernel_graph_identity` (admission + canonicalization).
4. Group by `stable_key`.
5. Reject if same `stable_key` has conflicting `canonical_text` in batch.
6. Sort cohorts by stable key (BTreeMap order).
7. Sort request IDs within each cohort.

No scheduling, WGSL emission, caching, or dispatch.

---

## Stable-Ordering Result

Same three-request batch in permuted input order produces identical `KernelGraphCohortPreviewSet`.

---

## Identical-Graph Grouping Result

Two valid GRAD-0→scorer graphs (different node/edge order): **one cohort**, request IDs `["req_a", "req_b"]` sorted.

---

## Distinct-Graph Split Result

Base graph vs variant with additional scorer read `bias`: **two cohorts**, different stable keys and canonical text.

---

## Invalid-Graph Rejection Result

| Invalid request | Result |
|-----------------|--------|
| GRAD-0 `mag2` → exact input | **Rejected** before grouping |
| Cycle A↔B | **Rejected** before grouping |

---

## Collision Guard Result

`test_group_cohort_previews_from_resolved` injects same synthetic `stable_key` with different canonical text → **rejected** with "conflicting canonical text". Production identity path uses FNV-1a over full canonical text; collision guard is preview-level defense-in-depth.

---

## Proof: No Scheduler / Cache / GPU Dispatch

Cohort module contains no GPU crate imports, dispatch APIs, session wiring, or cache structures. Grouping is pure in-memory preview over identity results.

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
rg "KernelGraphRequestSpec|KernelGraphCohortPreview|KernelGraphCohortPreviewSet|preview_kernel_graph_cohorts|request_id|cohort" crates/simthing-spec docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md
```

**Result:** cohort preview in `jit_kernel_cohort_preview.rs`, tests, exports; no runtime cache/scheduler.

```
rg "KernelGraphIdentity|preview_kernel_graph_identity|stable_key|canonical_text|cache key|identity" crates/simthing-spec docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md
```

**Result:** cohort groups by identity; canonical text preserved per cohort.

```
rg "dispatch_workgroups|create_shader_module|GpuContext|EmlGpuProgramTable|AccumulatorOpSession|tick_with_eml|kernel cache|scheduler|registry" crates/simthing-spec/src/compile/jit_kernel* crates/simthing-spec/tests/jit_kernel* docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md
```

**Result:** no GPU dispatch/runtime session/production registry/scheduler/cache in cohort preview module.

```
rg "ApproximateDiagnostic|ExactAuthoritative|NativeMathClass|mag2|sqrt_out|score|ProductionCandidate|production_wiring" crates/simthing-spec crates/simthing-driver/tests docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md
```

**Result:** exact/approx classifications preserved; unsafe graphs reject.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-spec docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md
```

**Result:** forbidden terms only in guardrail context; canonical content semantic-free.

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
- `docs/tests/phase_m_jit_cohort0_kernel_graph_cohort_preview_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-COHORT-0 is a spec-layer kernel graph cohort grouping preview without production caching or scheduling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
