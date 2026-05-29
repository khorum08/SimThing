# Phase M-JIT-KEY-0 — Deterministic Kernel Graph Identity / Cache-Key Preview — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `d4e1c1aaf196141896d1d788f1ea5bf89ca1be29` (M-JIT-DESC-2 merge on `master`)  
**Final commit SHA:** `d29dec7` (branch `phase-m-jit-key-0-graph-identity`; authoritative post-merge SHA is the GitHub squash-merge commit)  
**Lane classification:** Tier-2 GPU/JIT registry-readiness implementation (V7.7 §5)  
**Decision:** **IMPLEMENTED — spec-layer deterministic kernel graph identity/cache-key preview**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected DESC-2 graph admission, DESC-1 descriptor admission, prior reports, mapping guidance, accumulator plan, and invariants.

| Question | Answer |
|----------|--------|
| Can graph identity be derived from descriptor IDs plus output/input names? | Yes — canonical text encodes sorted nodes (reads/writes/authority/flags) and sorted edges. |
| Can identity be computed without scheduling? | Yes — `preview_kernel_graph_identity` validates admission then canonicalizes; no order bands. |
| Can this remain spec-only/default-off? | Yes — no GPU/runtime imports; admission gate preserved. |
| Which production gates remain closed? | Production descriptor registry, scheduler, runtime kernel cache, observer scheduling/caching, JIT cohort dispatch, default mapping, economy→mapping bridge. |

---

## Identity Model

Module: `crates/simthing-spec/src/compile/jit_kernel_graph_identity.rs`

```rust
KernelGraphIdentity { canonical_text: String, stable_key: String }
preview_kernel_graph_identity(&KernelGraphSpec) -> Result<KernelGraphIdentity, SpecError>
```

`preview_kernel_graph_identity` calls `validate_kernel_graph_admission` first. `stable_key` is `jit-graph-v1:` + FNV-1a hex digest of `canonical_text`.

---

## Canonicalization Rules

Stable under input ordering:

- nodes sorted by ID;
- reads sorted lexicographically;
- writes sorted by output name;
- edges sorted by `(from_kernel, from_output, to_kernel, to_input, required_authority)`.

Canonical text includes node lane, reads, writes with authority, native math class, semantic_free/default_off/production_wiring flags, and edge authority.

---

## Stable-Ordering Result

Same valid GRAD-0→scorer graph with permuted node/edge order produces identical `canonical_text` and `stable_key`.

---

## Authority / Native-Math Identity-Change Results

| Change | Identity | Admission |
|--------|----------|-----------|
| GRAD-0 `descent_x` → ApproximateDiagnostic | Rejects (no identity) | Rejected |
| Scorer `score` → ApproximateDiagnostic | Changes identity | Admitted |
| SQRT native math None (still approx output edge) | Rejects | Rejected |
| GRAD-0 ApproximateJitOnly with exact outputs | Rejects | Rejected |
| Scorer adds read `bias` | Changes identity | Admitted |

---

## Invalid-Graph Rejection Result

| Invalid graph | Result |
|---------------|--------|
| GRAD-0 `mag2` → exact input | **Rejected** (no identity) |
| Cycle A↔B | **Rejected** |
| Missing producer output | **Rejected** |

---

## Proof: No Scheduler / Cache / GPU Dispatch

Identity module contains no `dispatch_workgroups`, `create_shader_module`, `GpuContext`, session wiring, runtime cache structures, or GPU crate imports. Preview computes text/key only; nothing is stored or dispatched.

---

## Tests Run and Results

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
rg "KernelGraphIdentity|preview_kernel_graph_identity|canonical_text|stable_key|cache key|identity" crates/simthing-spec docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md
```

**Result:** identity preview in `jit_kernel_graph_identity.rs`, tests, exports; report documents cache-key preview without runtime cache.

```
rg "KernelGraphSpec|validate_kernel_graph_admission|ApproximateDiagnostic|ExactAuthoritative|NativeMathClass|mag2|sqrt_out|score" crates/simthing-spec crates/simthing-driver/tests docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md
```

**Result:** authority classes preserved; mag2/sqrt_out approximate; score exact-authoritative.

```
rg "dispatch_workgroups|create_shader_module|GpuContext|EmlGpuProgramTable|AccumulatorOpSession|tick_with_eml|kernel cache|scheduler|registry" crates/simthing-spec/src/compile/jit_kernel* crates/simthing-spec/tests/jit_kernel* docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md
```

**Result:** no GPU dispatch/runtime session/scheduler/cache in spec-layer identity module (guardrail comment context only elsewhere).

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-spec docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md
```

**Result:** forbidden terms only in guardrail lists/tests/report; canonical text semantic-free.

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

- `crates/simthing-spec/src/compile/jit_kernel_graph_identity.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_graph_identity.rs`
- `docs/tests/phase_m_jit_key0_kernel_graph_identity_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-KEY-0 is a spec-layer deterministic kernel graph identity/cache-key preview preserving exact vs approximate output authority without scheduling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
