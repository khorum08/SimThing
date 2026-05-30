# SQRT-PROMOTE-0 — Artifact-Backed Candidate F Descriptor/Admission Results

## Base HEAD

`db7760ff8c9852d4e1b94f41eb8ee830679722e7` (pre-SQRT-PROMOTE-0 implementation)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | **New** — F artifact descriptor metadata, hash guard, landed F exact kernel |
| `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` | `exact_sqrt_artifact` field; F descriptor in `landed_jit_kernel_descriptors()` |
| `crates/simthing-spec/src/compile/jit_kernel_graph_identity.rs` | Artifact binding lines in canonical node text |
| `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs` | REG-1 allows F exact `sqrt_out`; requires F hash pin |
| `crates/simthing-spec/src/compile/mod.rs` | Module + re-exports |
| `crates/simthing-spec/src/lib.rs` | Public re-exports |
| `crates/simthing-spec/tests/jit_exact_sqrt_artifact_admission.rs` | **New** — six SQRT-PROMOTE-0 admission tests |
| `crates/simthing-spec/tests/*.rs` (6 files) | `exact_sqrt_artifact: None` on manual struct literals |
| `crates/simthing-driver/tests/phase_m_jit_*` (4 files) | Same struct-literal field |
| `crates/simthing-driver/tests/phase_m_jit_sqrt_exact_candidate_battery.rs` | Promotion assertions updated for F descriptor |
| `docs/workshop/mapping_current_guidance.md` | SQRT-PROMOTE-0 status row |
| `docs/accumulator_op_v2_production_plan.md` | SQRT-PROMOTE-0 compact section |
| `docs/workshop/sqrt_candidates.md` | F mechanically promoted status |
| `docs/worklog.md` | Append-only milestone line |

## Pre-edit evaluation summary

1. **JIT descriptor classes:** `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` — `KernelDescriptorSpec`, `OutputAuthority`, `NativeMathClass`, `landed_jit_kernel_descriptors()`.
2. **Native sqrt / mag2 rejection:** `validate_exact_kernel_inputs()` + graph admission edge authority matching; native `m_jit_sqrt_0_candidate` is `ApproximateJitOnly` with `ApproximateDiagnostic` outputs; `mag2` on `m_jit_grad_0_observer` is `ApproximateDiagnostic`.
3. **F descriptor location:** new sibling module `jit_exact_sqrt_artifact_admission.rs`; landed id `m_jit_sqrt_f_exact` in `landed_jit_kernel_descriptors()`.
4. **Metadata representation:** `ExactSqrtArtifactDescriptor` struct with pinned constants; canonical graph identity includes artifact path/hash/entrypoint/proof/domain.
5. **Hash mismatch / recomposed text rejection:** `validate_exact_sqrt_artifact_binding()` + `validate_exact_sqrt_artifact_admission()`; exact `sqrt_out` without binding rejects; wrong path/hash/entrypoint/proof rejects.
6. **Native sqrt stays approximate:** `m_jit_sqrt_0_candidate` unchanged (`ApproximateJitOnly`); `ApproximateJitOnly` + exact output guard unchanged.
7. **Docs needing compact updates:** `mapping_current_guidance.md`, `accumulator_op_v2_production_plan.md`, `sqrt_candidates.md`, `worklog.md`. `invariants.md` already states artifact-backed F authority — no weakening edit required.

## Descriptor/admission design

- Spec-layer exactness uses `OutputAuthority::ExactAuthoritative` plus optional `ExactSqrtArtifactDescriptor`.
- Only the landed F descriptor (`m_jit_sqrt_f_exact`) carries a valid artifact binding.
- Admission does **not** accept arbitrary WGSL source text; identity is metadata-only (path/hash/entrypoint/proof).
- Graph canonical text and REG-1 production-candidate gate include artifact hash pin for exact `sqrt_out` edges.
- E3 remains documented exact cross-adapter fallback/reference only (not added to landed descriptors in this slice).

## Exact F metadata

| Field | Value |
|---|---|
| Descriptor id | `m_jit_sqrt_f_exact` |
| Artifact path | `crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl` |
| Hash (FNV-1a64) | `e2e9e27601ee2e13` |
| Entrypoint | `sqrt_cr_f_bits` |
| IO contract | `u32_bit_io` |
| Proof report | `docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md` |
| Domain | `0x0000_0000..=0x7F7F_FFFF` |
| Authority class | `ExactDeterministic` (`ExactSqrtAuthorityClass::ExactDeterministic`) |

## Admission matrix

| Path | Exact accepted? | Notes |
|---|---|---|
| F artifact-backed sqrt (`m_jit_sqrt_f_exact`) | **Yes** | Hash-valid binding required |
| Native/raw sqrt (`m_jit_sqrt_0_candidate`) | **No** | `ApproximateJitOnly` / `ApproximateDiagnostic` |
| Candidate D | **No** | Not in landed descriptors |
| Candidate C/f64 | **No** | Not implemented |
| Diagnostic `mag2` | **No** | Blocked unless routed through exact F |
| Arbitrary/recomposed F text | **No** | Wrong path/hash/entrypoint/proof rejects |
| Hash mismatch | **No** | `validate_exact_sqrt_artifact_binding` rejects |
| E3 | Fallback/reference | Documented only; not hot-path default |

## Test results

```text
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture
  6 passed; 0 failed

cargo test -p simthing-spec --test jit_kernel_registry_admission -- --nocapture
  8 passed; 0 failed

cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture
  11 passed; 0 failed

cargo test -p simthing-spec --test jit_kernel_descriptor_admission -- --nocapture
  8 passed; 0 failed (landed count=6)

cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture
  35 passed; 0 failed; 3 ignored (exhaustive sweeps not rerun)

cargo test -p simthing-driver --test phase_m_jit_sqrt_candidate_battery -- --nocapture
  8 passed; 0 failed

cargo check --workspace
  Finished (green)
```

## Scans run

1. `sqrt_cr_f_candidate|e2e9e27601ee2e13|sqrt_cr_f_bits|ExactSqrtArtifact|SQRT_F_*` — F descriptor/admission metadata present in spec module, tests, battery, docs.
2. `ApproximateJitOnly` — native sqrt remains approximate; F uses `NativeMathClass::None`.
3. `mag2|validate_exact_kernel_inputs` — mag2 still blocked; F sqrt_out accepted via artifact guard.
4. `Candidate C|f64|sqrt_cr_c` — guardrail/rejection text only; no implementation.
5. Production wiring/scheduler/cache/semantic WGSL — guardrail-only references in changed files; no authorization added.
6. `docs/tests/*.log` — exhaustive proof logs retained (`phase_m_jit_sqrt_exact4e/5f_exhaustive_batches.log`); no scratch/tmp deleted.

## Guardrail confirmations

- **No production scheduler/cache/default wiring** — all descriptors remain `default_off=true`, `production_wiring=false`.
- **No semantic WGSL** — admission module contains no WGSL generation or semantic terms.
- **No production economy→mapping bridge** — no `simthing-sim` / `SimSession` wiring added.
- **No E-phase/E11 evidence touched.**
- **Transient cleanup:** no scratch/tmp artifacts removed (only proof logs present).

## Final verdict

**PASS — SQRT-PROMOTE-0** landed the mechanical descriptor/admission implementation for artifact-backed Candidate F exact sqrt authority. The descriptor records `sqrt_cr_f_candidate.wgsl`, hash `e2e9e27601ee2e13`, entrypoint `sqrt_cr_f_bits`, `u32` bit IO, and the SQRT-EXACT-5F proof report. Native/raw WGSL sqrt remains `ApproximateJitOnly`; diagnostic `mag2` remains blocked as exact input unless routed through exact F; arbitrary/recomposed F text and hash mismatch reject; no production scheduler/cache/default wiring/semantic WGSL/economy bridge was added; active docs and production plan were updated; tests and `cargo check --workspace` are green; V7.7 / Mapping ADR / SEAD posture remains intact.
