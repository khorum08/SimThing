# Phase M-JIT-REG-0 — Test-Only Kernel Registry Manifest Preview — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `ac0a12169387ae33144a6e7892885f001740175d` (M-JIT-COHORT-0 R1 merge on `master`)  
**Final commit SHA:** _(set at commit; authoritative post-merge SHA is the GitHub squash-merge commit)_  
**Lane classification:** Tier-2 GPU/JIT registry manifest preview (V7.7 §5)  
**Decision:** **IMPLEMENTED — test-only kernel registry manifest preview**  
**Verdict:** **PASS**

---

## Pre-Edit Evaluation Summary

Inspected COHORT-0/R1, KEY-0, DESC-2, descriptor admission, prior reports, mapping guidance, accumulator plan, and invariants.

| Question | Answer |
|----------|--------|
| Can registry manifest be built from cohort previews without scheduling? | Yes — `preview_kernel_registry_manifest` calls `preview_kernel_graph_cohorts` then maps cohorts to entries. |
| Can entries remain TestOnly/default-off? | Yes — every entry is `TestOnlyPreview`, `default_off=true`, `production_wiring=false`. |
| Can identity remain stable under ordering? | Yes — cohort BTreeMap ordering + sorted request IDs; manifest stable under request permutation. |
| Can duplicate registry entries reject? | Yes — `validate_kernel_registry_manifest_preview` rejects duplicate stable keys. |
| Can approximate misuse reject upstream? | Yes — graph/cohort/identity admission runs before manifest conversion. |
| Can this stay semantic-free and spec-only? | Yes — no GPU/runtime imports. |
| Which production gates remain closed? | Production registry, scheduler, runtime kernel cache, observer scheduling/caching, JIT cohort dispatch, default mapping, economy→mapping bridge. |

---

## Registry Manifest Model

Module: `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs`

```rust
KernelRegistryLane { TestOnlyPreview }
KernelRegistryEntryPreview { stable_key, canonical_text, request_ids, lane, default_off, production_wiring }
KernelRegistryManifestPreview { entries }

preview_kernel_registry_manifest(&[KernelGraphRequestSpec]) -> Result<KernelRegistryManifestPreview, SpecError>
validate_kernel_registry_manifest_preview(&KernelRegistryManifestPreview) -> Result<(), SpecError>
```

---

## Manifest Validation Rules

Rejects:

- empty manifest entries;
- duplicate stable keys;
- empty canonical text;
- empty request IDs;
- non-`TestOnlyPreview` lane;
- `default_off == false`;
- `production_wiring == true`.

---

## Stable-Ordering Result

Three-request batch in permuted input order produces identical manifest.

---

## Cohort-to-Registry Result

| Input | Registry |
|-------|----------|
| `req_a` + `req_b` (identical graphs, different order) | One entry, 2 request IDs |
| `req_variant` (distinct admitted graph) | One entry, 1 request ID |
| Total | 2 entries, sorted by stable key |

---

## Invalid-Graph Rejection Result

| Invalid request | Result |
|-----------------|--------|
| GRAD-0 `mag2` → exact input | **Rejected** before manifest |
| Cycle A↔B | **Rejected** before manifest |
| Duplicate request IDs | **Rejected** |

---

## Production-Shaped Manifest Rejection Result

Manual invalid manifests reject for: empty entries, duplicate keys, empty canonical text, empty request IDs, `default_off=false`, `production_wiring=true`.

---

## Proof: No Scheduler / Cache / GPU Dispatch

Registry preview module contains no GPU crate imports, dispatch APIs, session wiring, or cache structures. Manifest is pure spec-layer conversion from cohort previews.

---

## Tests Run and Results

```
cargo test -p simthing-spec --test jit_kernel_registry_preview -- --nocapture
```

**Result:** 7 passed.

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
rg "KernelRegistryManifestPreview|KernelRegistryEntryPreview|KernelRegistryLane|preview_kernel_registry_manifest|validate_kernel_registry_manifest_preview|TestOnlyPreview" crates/simthing-spec docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md
```

**Result:** registry manifest preview in `jit_kernel_registry_preview.rs`, tests, exports; no runtime cache/scheduler.

```
rg "KernelGraphCohortPreview|preview_kernel_graph_cohorts|KernelGraphIdentity|stable_key|canonical_text|request_ids" crates/simthing-spec docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md
```

**Result:** manifest derives from cohort previews; identity fields preserved.

```
rg "dispatch_workgroups|create_shader_module|GpuContext|EmlGpuProgramTable|AccumulatorOpSession|tick_with_eml|kernel cache|scheduler|runtime registry|production registry" crates/simthing-spec/src/compile/jit_kernel* crates/simthing-spec/tests/jit_kernel* docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md
```

**Result:** registry appears only as preview/manifest terminology; no GPU dispatch/runtime session/production registry/cache.

```
rg "ApproximateDiagnostic|ExactAuthoritative|NativeMathClass|mag2|sqrt_out|score|ProductionCandidate|production_wiring|default_off" crates/simthing-spec crates/simthing-driver/tests docs/workshop/mapping_current_guidance.md docs/accumulator_op_v2_production_plan.md docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md
```

**Result:** exact/approx classifications preserved; unsafe graphs reject upstream.

```
rg "faction|ownership|owner|AI|threat|scarcity|opportunity|labor|price|logistics|routing|need|demand|supply|personality|drone|SEAD|simthing-sim|ResourceEconomySpec|SimSession" crates/simthing-spec docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md
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

- `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs`
- `crates/simthing-spec/src/compile/mod.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/jit_kernel_registry_preview.rs`
- `docs/tests/phase_m_jit_reg0_kernel_registry_manifest_test_results.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/workshop_current_state.md`
- `docs/accumulator_op_v2_production_plan.md`
- `docs/worklog.md`

---

## Posture Attestation

No semantic WGSL, no GPU dispatch/runtime scheduler/cache, no production JIT wiring, no production observer scheduling/caching, no production registry, no default mapping wiring, no simthing-sim Gadget/Personality/Memory semantics, no new production EML opcode, no production sqrt admission, no approximate output admitted as exact-authoritative input, no ProductionCandidate descriptor admitted, no chained scheduling, no automatic snapshot/copy scheduling, no CPU planner/urgency/commitment emission, no production economy→mapping bridge; M-JIT-REG-0 is a spec-layer TestOnly kernel registry manifest preview without production caching or scheduling; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
