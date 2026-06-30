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

- Branch: `kernel-dispatch-incrate-0r` (deleted on merge)
- Parent: [#1005](https://github.com/khorum08/SimThing/pull/1005) `43f78ab48b`
- PR: [#1007](https://github.com/khorum08/SimThing/pull/1007)
- Post-0R merge: `a7544e124f`

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

---

## 0R2 remediation — seal the authoritative buffer class (default-sealed)

### Status

**PROBATION** — all `WorldGpuState` authoritative buffer fields and in-crate bind/dispatch `&Buffer` accessors sealed to `pub(crate)`; B3 class grep clean; B6 Option B confirmed (not asserted). Not DA-approved.

### PR / branch / merge

- Branch: `kernel-dispatch-incrate-0r2`
- Parent: [#1007](https://github.com/khorum08/SimThing/pull/1007) `a7544e124f`
- PR: [#1009](https://github.com/khorum08/SimThing/pull/1009)
- Post-0R2 merge: `094178ed7b`

### Mission source

Handoff KERNEL-DISPATCH-INCRATE-0R2 — stop fixing instances; seal the class. §1.1 ruling + §5.2 catalogue in `docs/design_0_0_8_4_5_simthing_kernel.md`. Residue risk: B3, B6.

### What changed

- Sealed all nine authoritative `WorldGpuState` buffer fields to `pub(crate)`:
  `governed_pairs`, `overlay_deltas`, `slot_delta_ranges`, `intent_deltas`,
  `threshold_registry`, `child_starts`, `child_indices`, `column_rules`, `depth_slots`.
- Sealed in-crate world-summary bind paths: `WorldSummaryRuntime::{encode_into,dispatch}`,
  `WorldAccumulatorRuntime::dispatch_world_summary`, `AccumulatorOpSession::dispatch_world_summaries`
  → `pub(crate)`.
- Added compile-fail proof `external_threshold_registry_queue_write` — external
  `state.ctx.queue.write_buffer(&state.threshold_registry, …)` does not compile.
- No behavior change; upload/dispatch/readback entry points unchanged for consumers.

### Sealed authoritative buffer class

| Surface | After 0R2 |
|---|---|
| `WorldGpuState::resolved` | `pub(crate)` (0) |
| `WorldGpuState::{governed_pairs,overlay_deltas,slot_delta_ranges,intent_deltas,threshold_registry}` | `pub(crate)` |
| `WorldGpuState::{child_starts,child_indices,column_rules,depth_slots}` | `pub(crate)` |
| `ResolvedGpuBuffers::{values,previous_values,output_vectors,previous_output_vectors}` | `pub(crate)` (0R) |
| `AccumulatorOpSession::values_buffer()` | `pub(crate)` (0R) |
| `EmlGpuProgramTable` / `AccumulatorInputListTable` buffers | private + `pub(crate)` accessors (0R) |
| World-summary resolved bind params | `pub(crate)` |

External crates route through `upload_*()`, `dispatch_*()`, and `read_*()` — no direct field or buffer-handle access.

### B3 default-sealed grep (authoritative handles)

Command: `rg -nE "pub (fn \w+\(&self\) ?-> ?&.*Buffer|\w+: Buffer)" crates/simthing-kernel/src`

```
(no matches — zero public `Buffer` fields or `&Buffer` self-accessors)
```

All authoritative handles are `pub(crate)` only (representative hits):

```
world_state.rs:192:    pub(crate) governed_pairs: Buffer,
world_state.rs:196:    pub(crate) overlay_deltas: Buffer,
world_state.rs:199:    pub(crate) slot_delta_ranges: Buffer,
world_state.rs:203:    pub(crate) intent_deltas: Buffer,
world_state.rs:207:    pub(crate) threshold_registry: Buffer,
world_state.rs:218:    pub(crate) child_starts: Buffer,
world_state.rs:220:    pub(crate) child_indices: Buffer,
world_state.rs:222:    pub(crate) column_rules: Buffer,
world_state.rs:226:    pub(crate) depth_slots: Buffer,
resolved.rs:28:    pub(crate) fn values(&self) -> &Buffer {
session.rs:742:    pub(crate) fn values_buffer(&self) -> &Buffer {
```

### B1–B8 residue scan

| Class | Hit | Classification |
|---|---|---|
| B1 `unsafe` forge | `#![forbid(unsafe_code)]` on kernel | **Clean** |
| B2 sealed-type derives | only Clone/Copy/Debug/PartialEq on sealed types | **Clean** |
| B3 authoritative `&Buffer` / `Buffer` fields | class grep returns zero `pub` hits | **Sealed** |
| B4 POD bridge | removed (CRATE-EXTRACT-0R2) | **Clean** |
| B5 external shader/pipeline params | none on authoritative path | **Clean** |
| B6 `ctx`/`Queue`/`Device` pairing | public `WorldGpuState::ctx`; no authoritative buffer handle escapes public API | **Option B — confirmed harmless** |
| B7 bytemuck cast to sealed types | sealed types non-Pod | **Clean** |
| B8 deps/unsafe | dep budget unchanged | **Clean** |

### Per-item residue whitelist (declared tripwires — not holes)

| Item | Evidence |
|---|---|
| `write_max_candidate_f_magnitude_bits(..., target_values: &Buffer)` | (a) Not state/delta/registry/program/topology — diagnostic scratch-bit write utility for candidate-F probe. (b) Never read by accumulate→reduce→threshold sweep as authority. (c) External callers supply their own target buffer; kernel does not export session `values_buffer()` — ctx-unpairable with sealed authoritative handles. Production path uses `AccumulatorOpSession::write_max_candidate_f_magnitude_bits` (no buffer param). |
| `IndexedScatterOp::dispatch(src, dst, …)` + `WorldGpuState::dispatch_indexed_scatter_from_resolved_values(dest, …)` | (a) Generic bounded data-movement primitive; not a kernel-owned authoritative buffer field. (b) Resolved `src` is bound internally via `pub(crate) resolved.values()`; external code only names caller-owned `dest` (projection buffer). (c) Cannot pair public `ctx` with any sealed authoritative handle for queue write. |
| `cpu_oracle_threshold_events` | Sanctioned CPU twin (crossing-derived oracle); doctrine-blessed tripwire per §5.2 standing residue. |
| In-crate WGSL shader text | Permanent residue — CPU-oracle parity is admission; Rust cannot type-check shader. |

No categorical “tick-upload = non-authoritative” wave-through. Former tick-upload fields are sealed because they participate in authoritative computation (program/delta/registry/topology).

### Context / queue-pairing ruling (B6)

**Option B confirmed.** After 0R2, grep of external crates finds zero `state.{governed_pairs,threshold_registry,…}` field access. Public `ctx`/`device`/`queue` cannot be paired with any authoritative kernel buffer handle — compile-fail proves `threshold_registry` field write; resolved/EML/input-list proofs from 0R remain green.

### Compile-fail proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `external_threshold_registry_queue_write` | `WorldGpuState::threshold_registry` field + `ctx.queue.write_buffer` |
| (prior 25 proofs) | POD bridge, readback authority, resolved buffers, session/EML/input-list queue writes |

**Total:** 26 doc `compile_fail` tests.

### Value parity

| Harness | Result |
|---|---|
| `cargo test -p simthing-kernel --doc` | 26/26 |
| `cargo test -p simthing-kernel threshold --lib` | 19/19 |
| `cargo test -p simthing-sim --test s6_threshold_sunset` | 4/4 |
| `cargo test -p simthing-sim --test c8a_eml_infrastructure` | 10/10 |

### Performance parity

Command: `cargo test -p simthing-sim --test c1_threshold_perf -- --nocapture`

| Run | `new_ms` |
|---|---|
| 1 (0R2 post-seal) | 0.1869 |

Post-0R band ~0.20–0.22 ms; visibility-only change — no regression gate failure.

### Inlining proof

Visibility-only refactor: no new dynamic dispatch, trait objects, boxed callbacks, runtime checks, or per-tick allocation.

### Scope Ledger (0R2 additions)

| File / area | Change |
|---|---|
| `world_state.rs` | seal nine authoritative `Buffer` fields → `pub(crate)` |
| `accumulator_op/world_summary.rs` | seal `encode_into` / `dispatch` |
| `accumulator_op/runtime.rs` | seal `dispatch_world_summary` |
| `accumulator_op/session.rs` | seal `dispatch_world_summaries` |
| `readback.rs` | +1 compile_fail (`threshold_registry`) |

### Forward residue (unchanged — KERNEL-CLOSEOUT-0 lands permanently)

- `cpu_oracle_threshold_events` — sanctioned CPU twin tripwire
- WGSL shader text — permanent in-crate residue
- Next rung: **KERNEL-CLOSEOUT-0** (constitution §0 + core design doc + handoff spine landings)
