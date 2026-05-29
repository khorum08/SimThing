# Phase M-JIT-DESC-2 — Kernel Graph Composition Admission Preview — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `03237ad40faceeb97f47044586cfc2defc0e758d` (M-JIT-DESC-1 merge on `master`)  
**Final commit SHA:** `fb17913` (branch `phase-m-jit-desc-2-graph-admission`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT descriptor graph admission preview (V7.7 §5)  
**Decision:** **IMPLEMENTED — spec-layer kernel graph composition admission preview**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected DESC-1 module/tests, DESC-0/DESC-1 reports, GRAD/SQRT reports, mapping guidance, accumulator plan, and invariants.

| Question | Answer |
|----------|--------|
| Can graph composition be represented purely as descriptor IDs plus output/input names? | Yes — `KernelGraphEdgeSpec { from_kernel, from_output, to_kernel, to_input, required_authority }`. |
| Can graph composition validate exactness without scheduling? | Yes — edge authority checked against producer output authority; no order bands emitted. |
| Can cycles be rejected without runtime order bands? | Yes — DFS cycle detection on node IDs only. |
| Can this remain spec-only and test-only/default-off? | Yes — every node passes DESC-1 admission; no GPU/runtime imports. |
| Which production gates remain closed? | Production descriptor registry/scheduler/cache, observer scheduling/caching, JIT cohort dispatch, default mapping, economy→mapping bridge. |

---

## Graph Model

Module: `crates/simthing-spec/src/compile/jit_kernel_graph_admission.rs`

```rust
KernelGraphSpec { nodes: Vec<KernelDescriptorSpec>, edges: Vec<KernelGraphEdgeSpec> }
KernelGraphEdgeSpec { from_kernel, from_output, to_kernel, to_input, required_authority }

validate_kernel_graph_admission(&KernelGraphSpec) -> Result<(), SpecError>
```

Errors reuse `SpecError::JitKernelDescriptorAdmission`.

---

## Admission Rules

`validate_kernel_graph_admission`:

1. Validates every node via `validate_kernel_descriptor_admission`.
2. Rejects duplicate node IDs.
3. Rejects missing producer/consumer nodes.
4. Rejects missing producer outputs.
5. Rejects undeclared consumer inputs.
6. Rejects exact-required edges from approximate/rejected/deferred outputs.
7. Rejects self-edges.
8. Rejects cycles (DFS).
9. Does not infer scheduling or emit kernels.

---

## Valid Graph Result

GRAD-0 `descent_x`/`descent_y` → GRAD-1-style scorer exact inputs: **Admitted**.

---

## Rejection Cases

| Test | Trigger | Result |
|------|---------|--------|
| `jit_desc2_rejects_mag2_as_exact_input` | GRAD-0 `mag2` → exact input | **Rejected** |
| `jit_desc2_rejects_sqrt_output_as_exact_input` | SQRT `sqrt_out` → exact input | **Rejected** |
| `jit_desc2_rejects_missing_output` | GRAD-1 `mag2` edge (no such output) | **Rejected** |
| `jit_desc2_rejects_duplicate_node_ids` | duplicate node id | **Rejected** |
| `jit_desc2_rejects_missing_producer_or_consumer` | missing node | **Rejected** |
| `jit_desc2_rejects_cycles` | A→B, B→A | **Rejected** |
| `jit_desc2_rejects_self_edges` | same kernel producer/consumer | **Rejected** |
| `jit_desc2_rejects_production_candidate_node` | ProductionCandidate node | **Rejected** |
| `jit_desc2_rejects_production_wired_node` | production_wiring node | **Rejected** |

---

## Exact/Approx Authority Results

| Edge | Result |
|------|--------|
| GRAD-0 `descent_x`/`descent_y` → scorer (ExactAuthoritative) | **Admitted** |
| GRAD-0 `mag2` → exact input | **Rejected** |
| SQRT `sqrt_out` → exact input | **Rejected** |
| GRAD-1 `score` output authority | **ExactAuthoritative** (unchanged) |

---

## Cycle / Self-Edge Result

Cycles and self-edges reject before any scheduling inference.

---

## Proof: No Scheduling / Cache / GPU Dispatch

Spec-layer graph module contains no `dispatch_workgroups`, `create_shader_module`, `GpuContext`, session wiring, scheduler, or kernel cache. Validation is pure admission logic over descriptor metadata.

---

## Tests Run and Results

```
cargo test -p simthing-spec --test jit_kernel_descriptor_admission -- --nocapture
```

**Result:** 8 passed.

```
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture
```

**Result:** 11 passed.

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
rg "KernelGraphSpec|KernelGraphEdgeSpec|validate_kernel_graph_admission|from_kernel|to_kernel|required_authority" crates/simthing-spec docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md
```

**Result:** graph admission preview in `jit_kernel_graph_admission.rs`, tests, exports; no runtime scheduler/cache.

```
rg "ApproximateDiagnostic|ExactAuthoritative|RejectedDeferred|mag2|sqrt_out|score|ProductionCandidate|production_wiring|cycle|self-edge" crates/simthing-spec crates/simthing-driver/tests docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md
```

**Result:** graph tests/report preserve exact/approx authority; unsafe edges reject.

```
rg "dispatch_workgroups|create_shader_module|GpuContext|EmlGpuProgramTable|AccumulatorOpSession|tick_with_eml|kernel cache|scheduler|registry" crates/simthing-spec/src/compile/jit_kernel* crates/simthing-spec/tests/jit_kernel* docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md
```

**Result:** no GPU dispatch, runtime session, scheduler, or cache in spec-layer graph admission (comment-only "registry/scheduler" guardrail context).

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-spec docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md
```

**Result:** forbidden terms only in guardrail/report context; graph node IDs remain semantic-free.

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

- `crates/simthing-spec/src/compile/jit_kernel_graph_admission.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_graph_admission.rs`
- `docs/tests/phase_m_jit_desc2_kernel_graph_admission_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-DESC-2 is a spec-layer kernel graph admission preview preserving exact vs approximate output authority without scheduling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
