# KERNEL-CRATE-EXTRACT-0 Results

## Status

**PROBATION** — sealed authoritative runtime surface extracted into `simthing-kernel`; write/emission/participation seals are dependency-enforced. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-crate-extract-0`
- PR: (pending)
- Merge: (pending)

## What changed

- Added workspace crate `crates/simthing-kernel` with `#![forbid(unsafe_code)]` and live authority code moved from `simthing-gpu`.
- `simthing-gpu` and `simthing-sim` depend on `simthing-kernel`; sealed types are re-exported from `simthing-gpu` for existing consumer paths while authority lives in the kernel crate.
- `ResolvedGpuBuffers` buffer fields are private in the kernel; `simthing-gpu` runtime uses `#[doc(hidden)]` accessors only (no public field reach).
- GPU session/oracle readback mints sealed records via `simthing_kernel::readback::*` bridge functions (no cross-crate `pub(crate)` minters).
- `simthing-sim` dependency budget allowlist updated to include `simthing-kernel`.

## Extracted authority surfaces

| Surface | Owner after extraction | Notes |
|---|---|---|
| `ResolvedWriteAuthority` | `simthing-kernel::sealed::write_authority` | ZST; `for_boundary_install()` hidden for gpu boundary installs |
| `ResolvedGpuBuffers` | `simthing-kernel::resolved` | Private `wgpu::Buffer` fields; hidden accessors/setters for gpu runtime |
| `EmissionRecord` / `ThresholdEmission` | `simthing-kernel::sealed::emission` | Private fields; mint via readback bridges |
| `ThresholdEvent` / `ThresholdEventGpu` | `simthing-kernel::sealed::threshold_event` | POD transport + sealed decision record |
| `cpu_oracle_threshold_events` | `simthing-kernel::sealed::threshold_event` | Public oracle twin (crossing-derived; justified soft point) |
| `ThresholdRegistration` + `DIR_*` / `THRESH_BUF_*` | `simthing-kernel::registration` | Producer registration POD |
| Readback minters | `simthing-kernel::readback` | Cross-crate mint bridge for gpu session/oracle |
| `PlacedParticipant` + validators | `simthing-kernel::participation` | Re-export from `simthing-core` participation seal |

**Deferred (still in `simthing-gpu`):** `AccumulatorOpSession`, `WorldGpuState` orchestration, WGSL pipelines, non-authoritative GPU utilities. These consume kernel types but do not own sealed mint authority.

## Dependency graph

```
simthing-core
    ↓
simthing-kernel  (bytemuck, thiserror, wgpu)
    ↓
simthing-gpu / simthing-sim → downstream consumers
```

Direct deps (justified):

| Dependency | Why |
|---|---|
| `simthing-core` | `PlacedParticipant` validators, structural types used by participation re-export |
| `bytemuck` | POD mirrors (`EmissionRecordGpu`, `ThresholdEventGpu`, …) |
| `thiserror` | Reserved for kernel error surfaces (participation re-export path) |
| `wgpu` | `ResolvedGpuBuffers` owns `Buffer` handles |

Proof: `cargo tree -p simthing-kernel`; `cargo tree -i simthing-kernel` shows gpu+sim as direct consumers, no cycle.

## Public API / sanctioned channels

**Observe:** import sealed types and oracle from `simthing_kernel` (or `simthing_gpu` re-exports for legacy paths).

**Mutate resolved state:** produce AccumulatorOp / packed upload / BoundaryProtocol install via `WorldGpuState` dispatch + `ResolvedWriteAuthority` boundary installs (gpu runtime; buffers owned by kernel type).

**Emit decisions:** register thresholds → GPU scan / EmitEvent → readback via kernel bridges → sealed `ThresholdEvent` / `EmissionRecord`.

**Participate spatially:** `validate_and_mint_placed_participants_by_location_id` → `PlacedParticipant` → RF arena enrollment.

## Constructor / minter audit

| Type | Public struct literal | Public named forge constructor | Production mint path | Verdict |
|---|---|---|---|---|
| `ResolvedWriteAuthority` | `compile_fail` | None | Hidden `for_boundary_install()` | OK |
| `ResolvedGpuBuffers` | N/A (fields private) | `new()` takes buffers only | Gpu runtime constructs; no external mutation | OK |
| `EmissionRecord` | `compile_fail` | None | `readback::*` from real GPU/oracle bytes | OK |
| `ThresholdEmission` | `compile_fail` | None | `readback::*` | OK |
| `ThresholdEvent` | `compile_fail` (struct + named) | None | Pass-7 readback / `cpu_oracle_threshold_events` | OK — oracle is crossing-derived |
| `PlacedParticipant` | `compile_fail` | None | Validators in kernel participation module | OK |

**Soft point (justified):** `cpu_oracle_threshold_events` remains public in the kernel for parity tests and CPU-oracle twins. It derives events from buffer crossings + registrations; callers cannot pick arbitrary `(slot, col, event_kind)` tuples.

## Load-bearing compile-time proofs

| Proof | Catches |
|---|---|
| `cargo check -p simthing-kernel -p simthing-gpu -p simthing-sim` | Cycle-free graph; cross-crate wiring |
| `cargo test -p simthing-kernel --doc` (7 `compile_fail`) | External forge of write authority + sealed records |
| `cargo test -p simthing-kernel dependency_budget` | Forbidden deps on kernel crate |
| `cargo test -p simthing-sim dependency_budget` | Sim allowlist includes kernel |
| Private `ResolvedGpuBuffers` fields + accessor migration in `passes.rs` | External direct buffer field mutation |
| `cargo test -p simthing-gpu threshold --lib` (18 tests) | Threshold scan/readback parity through kernel types |

## Value parity

| Harness | Result |
|---|---|
| `cargo test -p simthing-sim --test s6_threshold_sunset` | 4/4 pass |
| `cargo test -p simthing-gpu threshold --lib` | 18/18 pass |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | 7/7 pass |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | pass |
| `cargo test -p simthing-sim as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads --lib` | pass |

No resolved-value or threshold-event semantics changed; refactor moves types and enforces boundaries.

## Performance parity

| | Baseline (master) | After extraction |
|---|---|---|
| Commit | `b44d9659126bb8f4a687cad5053253e167e6e6b1` | (this branch) |
| Command | `cargo test -p simthing-sim --test c1_threshold_perf --release c1_accumulator_threshold_readback_smoke -- --nocapture` | same |
| Environment | Windows 10, local GPU | same |
| `new_ms` (10k regs, readback-only) | **0.2145** | **0.1888** |

Delta within noise; no regression. Smoke asserts finite timing (no ratio gate on this harness).

## Inlining proof

- Root `Cargo.toml` `[profile.release] lto = "thin"` (unchanged).
- `ResolvedGpuBuffers::{values,previous_values,…}` are single-field `#[doc(hidden)]` accessors — eligible for inlining across the gpu↔kernel boundary under LTO.
- Readback bridges (`threshold_events_from_gpu`, etc.) are thin `map` over POD slices; no heap allocation or dynamic dispatch on the hot path.
- `c1_threshold_perf` release smoke (above) exercises threshold readback conversion after extraction with no measured regression.

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-kernel/**` | New authority crate |
| `Cargo.toml`, `Cargo.lock` | Workspace member |
| `crates/simthing-gpu/**` | Consume kernel types; remove duplicate sealed definitions |
| `crates/simthing-sim/Cargo.toml`, `dependency_budget.rs` | Kernel dependency adoption |
| `docs/tests/kernel_crate_extract_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 6 OPEN → PROBATION |

**Not touched:** `AccumulatorOpSession` full move, `deny.toml`, 0.0.8.5 ClauseScript work, closeout documentation landings.

## Conformance (spine / D-directives held)

- One authoritative path preserved; seals promoted to dependency boundary.
- No placeholder crate, no inert modules, no new runtime checks on hot path.
- No `simthing-kernel → simthing-gpu` cycle.
- `#![forbid(unsafe_code)]` on kernel.

## Known gaps / next

- **`KERNEL-CLOSEOUT-0`** — four mandatory documentation landings (core design, constitution §0, handoff template, STEAD-completeness).
- Move `AccumulatorOpSession` / full tick orchestration into kernel in a follow-on slice if DA wants authority code colocated (optional; seals already dependency-enforced).
- Tighten `cpu_oracle_threshold_events` to test-only scope if DA prefers after parity harness audit.
- DA re-review: PROBATION → DONE.
