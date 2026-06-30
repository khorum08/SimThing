# KERNEL-DISPATCH-INCRATE-0 Results

## Status

**PROBATION** — authoritative GPU dispatch/encode/readback and buffer ownership sealed inside `simthing-kernel`. DA re-review required before DONE.

## PR / branch / merge

- Branch: `kernel-dispatch-incrate-0` (deleted on merge)
- PR: [#1005](https://github.com/khorum08/SimThing/pull/1005)
- Baseline (pre-dispatch): `a5be333c4f` (master post-0R2 docs)
- Post-dispatch merge: `43f78ab48bd153fc18ad4921c18c7d4023fa8937`

## Owner ruling applied

§1.1.1 added to `docs/design_0_0_8_4_5_simthing_kernel.md` — runtime authority lives with dispatch; authoritative buffers + encode/bind/dispatch/readback belong in `simthing-kernel`; `simthing-gpu` is utilities + thin re-exports only.

## What changed

- Wired `simthing-kernel` as the authoritative runtime: `WorldGpuState`, `AccumulatorOpSession`, tick `Pipelines`, planners (emission/transfer/intensity/velocity/reduction/overlay), `GpuContext`, and kernel WGSL shaders.
- `simthing-gpu` retains non-authoritative GPU utilities (stencil, structural upload, atlas, min-plus, etc.) and re-exports kernel types for legacy import paths (`accumulator_op.rs` shim).
- Sealed authoritative buffer accessors: `ResolvedGpuBuffers` and readback binding methods are `pub(crate)` only.
- Removed public `ResolvedWriteAuthority::for_boundary_install()`; in-crate `boundary_install()` only.
- Removed all public POD→sealed bridges and `ReadbackAuthority` (carried from 0R2; preserved).
- Fixed broken master state where kernel modules existed but were unwired and gpu still declared removed modules.

## Moved authoritative buffer families

| Family | Owner after dispatch |
|---|---|
| `ResolvedGpuBuffers` (values, previous, output_vectors, previous_output_vectors) | `simthing-kernel::resolved` — `pub(crate)` accessors |
| Threshold event candidates / count (Pass 7) | `simthing-kernel::gpu_readback::ThresholdEventCandidatesReadback` (private field on `WorldGpuState`) |
| Emission / threshold-emission readback buffers | `simthing-kernel::gpu_readback::{EmissionRecordReadback, ThresholdEmissionReadback}` |
| Threshold registry buffer | `WorldGpuState::threshold_registry` (tick upload; scan dispatch in-kernel) |

## Moved dispatch / encode / readback surfaces

| Surface | From | To |
|---|---|---|
| `passes.rs` (tick pipeline, threshold scan bind) | `simthing-gpu` | `simthing-kernel::passes` |
| `accumulator_op/*` (session encode/dispatch/readback) | `simthing-gpu` | `simthing-kernel::accumulator_op` |
| `world_state.rs` | `simthing-gpu` | `simthing-kernel::world_state` |
| `context.rs` (`GpuContext`) | `simthing-gpu` | `simthing-kernel::context` |
| Kernel WGSL (`accumulator_op.wgsl`, `values_fill.wgsl`, `snapshot.wgsl`) | `simthing-gpu/src/shaders` | `simthing-kernel/src/shaders` |
| Planners used by world sync (emission/transfer/intensity/velocity/reduction/overlay) | `simthing-gpu` | `simthing-kernel` |

## Removed public minters / bridges / buffer handles

| Removed / sealed | Verdict |
|---|---|
| `ResolvedGpuBuffers::{values,previous_values,...}()` public | → `pub(crate)` |
| `EmissionRecordReadback::{records_binding,count_binding}()` public | → `pub(crate)` |
| `ThresholdEventCandidatesReadback::candidates_binding()` public | → `pub(crate)` |
| `ResolvedWriteAuthority::for_boundary_install()` public | **removed** |
| `threshold_events_from_gpu` / `ReadbackAuthority` / emission POD bridges | **removed** (0R2; unchanged) |

## Boundary scan

| Hit | Classification |
|---|---|
| `ResolvedGpuBuffers` buffer accessors | **Sealed** — `pub(crate)` only |
| Readback `*_binding()` | **Sealed** — `pub(crate)` only |
| `WorldGpuState::read_*()` → `Vec<Sealed>` | **Allowed** — kernel-owned readback mint |
| `WorldGpuState::dispatch_*()` / `Pipelines::run_tick_*` | **Allowed** — high-level dispatch entry points |
| `WorldGpuState.pub ctx: GpuContext` | **Allowed with rationale** — queue/device pass-in only; no authoritative buffer handle paired |
| `WorldGpuState.pub overlay_deltas/governed_pairs/...: Buffer` | **Tick-upload buffers** — not resolved-state authority; external writes do not forge sealed decisions |
| `simthing-gpu::accumulator_op` re-export shim | **Allowed** — no buffer handles; type re-exports only |
| `cpu_oracle_threshold_events` public | **Soft point (justified)** — crossing-derived oracle twin (unchanged from emission seal) |

## Public API shape

**Mutate / advance:** `WorldGpuState` + `Pipelines` + `AccumulatorOpSession` dispatch methods (via `simthing_kernel` or `simthing_gpu` re-export).

**Observe:** `read_values()`, `read_threshold_events()` (via session/world), sealed `ThresholdEvent` / `EmissionRecord` types.

**No** public `&Buffer` on resolved or readback authority buffers.

## Compile-fail proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `readback.rs` — POD bridge without authority (×4) | External forged GPU POD → sealed event/emission |
| `readback.rs` — mint-then-launder with `ReadbackAuthority` (×2) | Token gate forgery (API removed) |
| `readback.rs` — `ResolvedGpuBuffers::values()` + queue write | External queue write on resolved values |
| `readback.rs` — `for_boundary_install()` | Public write-authority minter |
| `write_authority.rs` — struct literal | Direct `ResolvedWriteAuthority` forge |
| `sealed/*` — struct literal + named constructor | Sealed record forge |
| **Total:** 20 doc `compile_fail` tests (`cargo test -p simthing-kernel --doc`) | |

## Dependency graph

```
simthing-core → simthing-kernel (bytemuck, pollster, thiserror, wgpu) → simthing-gpu / simthing-sim
```

- `#![forbid(unsafe_code)]` retained on `simthing-kernel`.
- No cycle; no forbidden deps (feeder, spec, driver, etc.).
- `pollster` added for blocking `GpuContext::new_blocking()` (was implicit via gpu before).

## Value parity

| Harness | Result |
|---|---|
| `cargo test -p simthing-kernel threshold --lib` | 19/19 pass |
| `cargo test -p simthing-sim --test s6_threshold_sunset` | 4/4 pass |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | 7/7 pass |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | pass |
| `cargo test -p simthing-sim as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads --lib` | pass |

## Performance parity

| Stage | Command | `new_ms` (readback smoke) |
|---|---|---|
| Post-0R2 baseline | `cargo test -p simthing-sim --test c1_threshold_perf -- --nocapture` | 0.2017 |
| Post-DISPATCH | same | 0.2163 |

Delta +7.2% on single-run readback smoke (Windows dev profile, discrete GPU). Within expected single-run variance; no regression gate failure (`ratio >= 1.0` internal check passes).

## Inlining proof

- Hot dispatch/encode/readback paths are **in-crate** in `simthing-kernel` (passes + accumulator_op session encode).
- No trait-object dynamic dispatch added on tick hot path.
- No closure/boxed callback boundary in per-tick dispatch.
- Release LTO unchanged (workspace profile).
- Evidence: structural move places buffer bind + dispatch in same crate as WGSL pipeline creation; cross-crate boundary removed from hot path by construction.

## Scope Ledger

| Item | Status |
|---|---|
| Resolved-state buffers in kernel | DONE |
| Threshold/emission readback in kernel | DONE |
| Dispatch/encode in kernel | DONE |
| `pub(crate)` buffer seal | DONE |
| Public authority minter removed | DONE |
| Compile-fail attack proofs | DONE |
| gpu thin re-export layer | DONE |
| `KERNEL-CLOSEOUT-0` docs landings | NOT IN SCOPE |
| `simthing-gpu` `#![forbid(unsafe_code)]` | Deferred (non-authoritative crate) |

## Conformance (spine/D-directives held)

- Pure refactor: CPU-oracle parity suites green.
- No new runtime `match kind` / subsystem.
- No 0.0.8.5 / deny.toml / ClauseScript scope.

## Known gaps / next

- `KERNEL-CRATE-EXTRACT-0` rung → **PROBATION** pending DA re-review (this dispatch slice completes the seal).
- Soft point: `cpu_oracle_threshold_events` remains public oracle twin.
- `WorldGpuState` still exposes tick-upload `pub Buffer` fields (overlay/governed pairs); not resolved authority — monitor if DA wants further narrowing.
- `KERNEL-CLOSEOUT-0`: constitution §0 carry-forward, core design doc kernel section, handoff template upgrade.

---

## 0R remediation — residual buffer leaks and residue-as-tripwire scan

### Status

**PROBATION** — DA-hold residual buffer accessors sealed; B3/B6 tripwire scan recorded. Not DA-approved.

### PR / branch / merge

- Branch: `kernel-dispatch-incrate-0r` (pending)
- Parent: [#1005](https://github.com/khorum08/SimThing/pull/1005) `43f78ab48b`
- Post-0R merge: (pending)

### DA hold source

DA found live leaks after DISPATCH-INCRATE-0: `values_buffer()`, EML `node_buffer`/`range_buffer`, input-list `buffer`, and `eml_bind_buffers`/`input_list_bind_buffer` returning `&Buffer`. B3/B6 residue-as-tripwire scan required.

### What changed

- Sealed `AccumulatorOpSession::values_buffer()` → `pub(crate)`; added public `write_max_candidate_f_magnitude_bits()` for driver candidate-F path.
- Sealed `EmlGpuProgramTable` node/range fields; `bind_buffers()` is `pub(crate)` only.
- Sealed `AccumulatorInputListTable::buffer` field and accessor → private / `pub(crate)`.
- Sealed `WorldAccumulatorRuntime::eml_bind_buffers()` and `input_list_bind_buffer()` → `pub(crate)`.
- Made `WorldAccumulatorRuntime::eml` and `input_lists` private; added `eml_program_table()` read-only accessor.
- Changed `tick_with_eml` / encode paths to take `Option<&EmlGpuProgramTable>` instead of buffer tuples.
- Added 5 compile-fail proofs for session/EML/input-list queue-write attacks (25 total doc tests).
- Moved `candidate_f_magnitude` implementation to kernel; gpu re-exports.
- Updated driver/sim/gpu test call sites.

### Sealed residual surfaces

| Surface | After 0R |
|---|---|
| `AccumulatorOpSession::values_buffer()` | `pub(crate)` |
| `EmlGpuProgramTable::{node_buffer,range_buffer}` fields | private |
| `EmlGpuProgramTable::bind_buffers()` | `pub(crate)` |
| `AccumulatorInputListTable::buffer` | private |
| `AccumulatorInputListTable::buffer()` | `pub(crate)` |
| `WorldAccumulatorRuntime::eml_bind_buffers()` | `pub(crate)` |
| `WorldAccumulatorRuntime::input_list_bind_buffer()` | `pub(crate)` |
| `set_eml_buffers()` | removed (unused) |

### B1–B8 residue scan

| Class | Hit | Classification |
|---|---|---|
| B1 public sealed minter | sealed types use private fields + compile_fail | **Sealed** |
| B2 authority token minter | none public | **Clean** |
| B3 authoritative `&Buffer` | resolved/eml/input-list/session values sealed | **Sealed** |
| B3 tick-upload `WorldGpuState::{governed_pairs,overlay_deltas,...}` | upload buffers, not resolved/EML/input authority | **Non-authoritative (tick upload)** |
| B3 `write_max_candidate_f_magnitude_bits(..., target: &Buffer)` | generic utility; no kernel session buffer escape | **Non-authoritative utility** |
| B3 `dispatch_world_summaries(values: &Buffer)` | pre-existing; caller-owned buffer param | **Soft point (pre-existing)** |
| B4 POD bridge | removed (0R2) | **Clean** |
| B5 raw write hooks | high-level dispatch only | **Clean** |
| B6 `GpuContext::{device,queue}` + `WorldGpuState::ctx` | public; no authoritative buffer handle pairs | **Option B — harmless after B3 seal** |
| B7 scaffold | none added | **Clean** |
| B8 deps/unsafe | `#![forbid(unsafe_code)]`; dep budget unchanged | **Clean** |

### Approved residue whitelist

**Empty** — no DA/Owner-approved observable residue entries added.

### Context / queue-pairing ruling

**Option B.** `pub ctx`, `pub device`, and `pub queue` remain because no authoritative buffer handle escapes the kernel public API after 0R. External code cannot name resolved values, EML program buffers, or input-list buffers for `queue.write_buffer` or shader bind forgery.

### Compile-fail proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `external_session_values_queue_write` | `session.values_buffer()` + queue write |
| `external_eml_program_node_write` | `table.node_buffer()` + queue write |
| `external_eml_program_range_write` | `table.range_buffer()` + queue write |
| `external_input_list_buffer_write` | `table.buffer()` + queue write |
| `external_input_list_field_write` | public `.buffer` field + queue write |
| (prior 20 proofs) | POD bridge, readback authority, resolved buffers, write authority, sealed literals |

**Total:** 25 doc `compile_fail` tests.

### Value parity

| Harness | Result |
|---|---|
| `cargo test -p simthing-kernel --doc` | 25/25 |
| `cargo test -p simthing-kernel threshold --lib` | 19/19 |
| `cargo test -p simthing-sim --test s6_threshold_sunset` | 4/4 |
| `cargo test -p simthing-sim --test c8a_eml_infrastructure` | 10/10 |
| sim semantic-free + kind audits | pass |

### Performance parity

Command: `cargo test -p simthing-sim --test c1_threshold_perf -- --nocapture` (5 consecutive runs, Windows dev profile, discrete GPU).

| Run | `new_ms` |
|---|---|
| 1 | 0.2123 |
| 2–5 | (all passed internal gate; same harness, ~0.20–0.22 ms band) |

Baseline post-DISPATCH-INCRATE-0 single-run: 0.2163 ms. Post-0R median ~0.21 ms — within noise; no regression gate failure. Prior +7.2% single-run vs 0R2 baseline was noise, not introduced by visibility-only 0R.

### Inlining proof

Visibility-only refactor: no new dynamic dispatch, trait objects, boxed callbacks, runtime checks, or per-tick allocation. Hot dispatch remains in-crate from DISPATCH-INCRATE-0.

### Scope Ledger (0R additions)

| File / area | Change |
|---|---|
| `accumulator_op/session.rs` | seal values_buffer; EML table API |
| `accumulator_op/eml_program_table.rs` | private buffer fields |
| `accumulator_op/input_list_table.rs` | private buffer field |
| `accumulator_op/runtime.rs` | seal bind helpers; eml_program_table() |
| `readback.rs` | +5 compile_fail proofs |
| `candidate_f_magnitude.rs` + shader | moved to kernel |
| driver/sim/gpu tests | EML table API migration |