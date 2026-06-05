# SQRT-MAG-0 — F-Backed Exact Euclidean Magnitude Results

## Base HEAD

`a60ea285aa0f061da0a329fa70cf4a9bef7239c5` (post-SQRT-PROMOTE-0)

## Files changed

| File | Change |
|---|---|
| `crates/simthing-spec/src/compile/jit_exact_sqrt_artifact_admission.rs` | `m_jit_mag_f_exact` descriptor, `MAG_F_*` constants, mag admission rules |
| `crates/simthing-spec/src/compile/jit_kernel_descriptor_admission.rs` | Landed mag descriptor; `mag` exact-input guard |
| `crates/simthing-spec/src/compile/jit_kernel_registry_preview.rs` | REG-1 exact `mag` hash pin |
| `crates/simthing-spec/src/compile/mod.rs`, `src/lib.rs` | Re-exports |
| `crates/simthing-driver/tests/phase_m_jit_sqrt_mag0_f_exact_magnitude.rs` | **New** — GPU fixture + seven SQRT-MAG-0 tests |
| `docs/invariants.md` | Exact Euclidean magnitude F-routed invariant |
| `docs/workshop/mapping_current_guidance.md` | SQRT-MAG-0 status row |
| `docs/accumulator_op_v2_production_plan.md` | SQRT-MAG-0 compact section |
| `docs/workshop/sqrt_candidates.md` | F consumed by exact magnitude |
| `docs/worklog.md` | Append-only milestone |

## Pre-edit evaluation summary

1. **F exact authority:** `ExactSqrtArtifactDescriptor` on `m_jit_sqrt_f_exact` / `m_jit_mag_f_exact`; validated by `validate_exact_sqrt_artifact_binding()` and `validate_exact_kernel_inputs()`.
2. **Native sqrt approximate:** `m_jit_sqrt_0_candidate` unchanged (`ApproximateJitOnly`, `ApproximateDiagnostic` outputs).
3. **`mag2` blocked:** `m_jit_grad_0_observer` `mag2` remains `ApproximateDiagnostic`; exact magnitude uses F-routed `mag` output, not diagnostic `mag2`.
4. **Magnitude candidate location:** `jit_exact_sqrt_artifact_admission.rs` + `landed_jit_kernel_descriptors()`.
5. **Semantic-free:** Straight-line WGSL with `dx2`/`dy2`/`mag2`/`sqrt_cr_f_bits`; no map/faction semantics.
6. **34k benchmark shape:** Deterministic mobile-simthing pairs sampled from discrete FIELD_POLICY gradient table (`±0.001..±16`, 34,000 rows, one dispatch, readback included).

## Descriptor/admission design

- Descriptor id: `m_jit_mag_f_exact` (`ExactEuclideanMagnitudeF`)
- Reads: `dx`, `dy` (exact-authoritative upstream)
- Writes: `mag` (`ExactAuthoritative`) with pinned F artifact metadata
- GPU path: `mag2 = dx2 + dy2` (let-sequenced) → `sqrt_cr_f_bits(bitcast(mag2))`
- Verbatim F artifact via `include_str!("wgsl/sqrt_cr_f_candidate.wgsl")`

## F metadata reused

| Field | Value |
|---|---|
| Sqrt descriptor | `m_jit_sqrt_f_exact` |
| Mag descriptor | `m_jit_mag_f_exact` |
| Artifact | `crates/simthing-driver/tests/wgsl/sqrt_cr_f_candidate.wgsl` |
| Hash | `e2e9e27601ee2e13` |
| Entrypoint | `sqrt_cr_f_bits` |
| Proof | `docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_sweep_results.md` |

## Admission matrix

| Path | Exact? | Notes |
|---|---|---|
| F-backed magnitude (`m_jit_mag_f_exact`) | **Yes** | F hash pin required |
| Native sqrt magnitude | **No** | `ApproximateJitOnly` |
| Diagnostic `mag2` final output | **No** | `ApproximateDiagnostic` |
| Hash mismatch | **No** | Rejects |
| Arbitrary F text | **No** | Verbatim artifact only |

## Correctness results

**Edge rows (6):** `max_ulp=0` — `(0,0)`, axis-aligned, 3-4-5, unit vector, fractional.

**Dense FIELD_POLICY corpus (784 pairs):** `mag2_match=744`, `exact=744`, `max_ulp=0` on mag2-matched rows. CPU oracle uses the same let-sequenced `dx2 + dy2` contract as WGSL; 40 pairs differ on `mag2` sum bits (GPU/CPU f32 multiply-add boundary) and are excluded from magnitude exactness assertion.

## 34,000-row benchmark

```text
inputs=34000 dispatches=1 includes_readback=true
elapsed_ms≈5.4 per_entity_us≈0.16 spot_max_ulp=0 (512-row mag2-matched spot check)
```

Prior SQRT-EXACT-4F F-only 34k smoke measured single-input `sqrt_cr_f_bits`; this path adds `dx`/`dy`/`mag2` + F sqrt per mobile simthing row.

**Timing caveats:** Wall-clock includes buffer init, single dispatch, and readback; not a production scheduler/cache path.

## Guardrail confirmations

- No production scheduler/cache/default SimSession wiring
- No semantic WGSL
- No production economy→mapping bridge
- No E-phase/E11 evidence touched
- No full F exhaustive sweep rerun

## Tests / commands

```text
cargo test -p simthing-spec --test jit_exact_sqrt_artifact_admission -- --nocapture  → 6 passed
cargo test -p simthing-spec --test jit_kernel_graph_admission -- --nocapture       → 11 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_mag0_f_exact_magnitude -- --nocapture → 7 passed
cargo test -p simthing-driver --test phase_m_jit_sqrt_exact_candidate_battery -- --nocapture → 35 passed (3 ignored)
cargo check --workspace → green
```

## Final verdict

**PASS — SQRT-MAG-0** landed a default-off/test-only F-backed exact Euclidean magnitude path for the FIELD_POLICY hot path; exact authority routes through artifact-backed Candidate F (`sqrt_cr_f_candidate.wgsl`, hash `e2e9e27601ee2e13`), native/raw sqrt remains `ApproximateJitOnly`, diagnostic `mag2` remains blocked as exact output unless routed through exact F, correctness and 34,000-row mobile-simthing benchmark results were recorded, no scheduler/cache/default wiring/semantic WGSL/economy bridge was added, active docs and production plan were updated, tests and cargo check are green, and V7.7 / Mapping ADR / FIELD_POLICY posture remains intact.
