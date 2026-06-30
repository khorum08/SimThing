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
