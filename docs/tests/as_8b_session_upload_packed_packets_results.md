# AS-8B Session Upload Packed Packets Results

## Status

PROBATION — public `AccumulatorOpSession` upload methods sealed behind private-field packed packets; encode/input-list/threshold sidecar behavior preserved. DA re-review required before AS-8B → DONE — DA-APPROVED.

## PR / branch / merge

- Branch: `codex/as-8b-session-upload-packed-packets`
- PR: https://github.com/khorum08/SimThing/pull/975
- Merge: `734f84d9d7` (master)

## What changed

- Added `packed_session_upload.rs` with `PackedAccumulatorUpload`, `PackedThresholdUpload`, `PackedIntentUpload` (private fields, pack constructors).
- Replaced public session upload methods with `upload_packed_ops`, `upload_packed_threshold_ops`, `append_packed_threshold_ops`, `upload_packed_intent_ops`, `write_packed_op_buffer`.
- Removed public `upload_ops`, `upload_ops_with_eml`, `upload_ops_resolving_input_lists`, `upload_threshold_ops`, `append_threshold_ops`, `upload_intent_ops`, `upload_gpu_ops`, `write_op_buffer`.
- `WorldAccumulatorRuntime` upload delegates pack via constructors before calling session packed upload.
- Call sites (driver, sim, passes, tests) pack before session upload.

## Session-upload boundary audit

| Surface | Public upload args | Classification |
|---|---|---|
| `AccumulatorOpSession::upload_packed_ops` | `&PackedAccumulatorUpload` | **Sealed (this rung)** |
| `AccumulatorOpSession::upload_packed_threshold_ops` | `&PackedThresholdUpload` | **Sealed** |
| `AccumulatorOpSession::append_packed_threshold_ops` | `&PackedThresholdUpload` | **Sealed** |
| `AccumulatorOpSession::upload_packed_intent_ops` | `&PackedIntentUpload` | **Sealed** |
| `AccumulatorOpSession::write_packed_op_buffer` | `&PackedAccumulatorUpload` (preserves threshold sidecar) | **Sealed** |
| `Packed*Upload::from_*` constructors | semantic ops / registrations / deltas / GPU rows | **Pack boundary** — allowed |
| `WorldAccumulatorRuntime::upload_*_ops` | semantic slices at runtime envelope | **Pack-then-upload** — runtime packs before session |
| `WorldGpuState::upload_accumulator_intents/thresholds` | semantic slices | **Pack-then-upload** via runtime |
| `upload_structural_rows_to_gpu` (AS-8) | `&PackedUpload` | **Sealed (AS-8)** |
| WGSL shader text | — | **Final GPU residue** — Rust type space cannot seal |

## Packed packet API

```rust
PackedAccumulatorUpload::from_ops / from_ops_with_eml / from_ops_resolving_input_lists / from_gpu_ops
PackedThresholdUpload::from_registrations
PackedIntentUpload::from_deltas

session.upload_packed_ops(ctx, &PackedAccumulatorUpload)
session.upload_packed_threshold_ops(ctx, &PackedThresholdUpload)
session.append_packed_threshold_ops(ctx, &PackedThresholdUpload)
session.upload_packed_intent_ops(ctx, &PackedIntentUpload)
```

## Load-bearing compile_fail proofs

| Proof | Location | Catches |
|---|---|---|
| `packed_accumulator_upload_fields_private_compile_fail` | `packed_session_upload.rs` doc-test | Field-literal packet construction |
| `session_upload_rejects_accumulator_ops_compile_fail` | doc-test | `upload_ops(&[AccumulatorOp])` |
| `session_upload_rejects_threshold_registrations_compile_fail` | doc-test | `upload_threshold_ops(&[ThresholdRegistration])` |
| `session_upload_rejects_intent_deltas_compile_fail` | doc-test | `upload_intent_ops(&[IntentDelta])` |
| `session_upload_rejects_free_gpu_ops_compile_fail` | doc-test | `upload_gpu_ops(&[AccumulatorOpGpu])` |
| `session_upload_rejects_eml_registry_argument_compile_fail` | doc-test | EML registry at upload seam |

## Encoding / sidecar / byte parity proofs

| Test | Result |
|---|---|
| `packed_accumulator_upload_encodes_same_ops` | PASS |
| `packed_accumulator_upload_resolves_input_lists_same_as_legacy` | PASS |
| `packed_threshold_upload_preserves_event_kinds_and_source_buffer` | PASS |
| `packed_intent_upload_encoding_preserved` | PASS |
| `packed_gpu_op_upload_preserves_raw_op_bytes` | PASS |
| `session_upload_packed_ops_preserves_n_ops` | PASS |
| `packed_threshold_append_preserves_existing_ops` | PASS |
| Existing session threshold/intent/overlay tests (migrated) | PASS |
| AS-8 structural upload tests | PASS (unchanged) |

## Scope Ledger

| File | Why touched |
|---|---|
| `crates/simthing-gpu/src/accumulator_op/packed_session_upload.rs` | Packed packets + compile_fail + pack tests |
| `crates/simthing-gpu/src/accumulator_op/session.rs` | Sealed upload API + session parity tests |
| `crates/simthing-gpu/src/accumulator_op/runtime.rs` | Pack-then-upload delegates |
| `crates/simthing-gpu/src/accumulator_op/mod.rs`, `lib.rs` | Exports |
| Driver/sim/passes call sites | Pack before upload |
| `docs/tests/as_8b_session_upload_packed_packets_results.md` | Evidence |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-8B ladder row → PROBATION |
| `docs/tests/current_evidence_index.md` | Index row |

**Not touched:** AS-F, 0.0.8.5, WGSL layouts, AccumulatorOp semantics redesign.

## Known residue / next

- GPU residue is now **shader-text-only** (pending DA review of this rung).
- `WorldAccumulatorRuntime::upload_*` still accepts semantic slices at the runtime envelope — packs internally; session seam is sealed.
- **AS-F closeout** — opens after DA approves AS-8B PROBATION → DONE.

## Validation (targeted)

- `cargo fmt -p simthing-gpu -p simthing-core -p simthing-driver -p simthing-sim -- --check` — PASS
- `cargo check -p simthing-gpu`, `-p simthing-core`, `-p simthing-driver`, `-p simthing-sim` — PASS
- `cargo test -p simthing-gpu --doc` — PASS (10 doc tests incl. 6 session compile_fails + 3 structural)
- `cargo test -p simthing-gpu packed_upload --lib` — PASS (7)
- `cargo test -p simthing-gpu packed_session_upload --lib` — PASS (5)
- `cargo test -p simthing-gpu accumulator_op --lib` — PASS (83)
- `cargo test -p simthing-gpu structural_upload --lib` — PASS (14)
- `cargo test -p simthing-core accumulator_builder_emits_same_op_after_index_newtypes --lib` — PASS
- `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` — PASS
- Scope grep: no 0.0.8.5 diff — PASS
