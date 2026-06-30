# KERNEL-CRATE-EXTRACT-0 Results

## Status

**PROBATION** — sealed authoritative runtime surface extracted into `simthing-kernel`; write/emission/participation seals are dependency-enforced. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-crate-extract-0`
- PR: https://github.com/khorum08/SimThing/pull/996 (+ https://github.com/khorum08/SimThing/pull/998 0R)
- Merge: `b677b43cb8` (0), `0438557e8a` (0R)

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
| `EmissionRecord` | `compile_fail` | None | `readback::*` + `ReadbackAuthority` from gpu readback path | OK (post-0R) |
| `ThresholdEmission` | `compile_fail` | None | `readback::*` + authority | OK (post-0R) |
| `ThresholdEvent` | `compile_fail` (struct + named) | None | Pass-7 readback / oracle + authority for bridges | OK (post-0R) |
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
| Commit | `b44d9659126bb8f4a687cad5053253e167e6e6b1` | `b677b43cb8` |
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

## 0R remediation — public POD readback bridge launder vector

**DA hold source:** `KERNEL-CRATE-EXTRACT-0` left public readback bridges (`threshold_events_from_gpu(gpu: &[ThresholdEventGpu])`, etc.) accepting forgeable public POD with all-public fields — external crates could fabricate POD bytes and mint sealed authoritative types without kernel emission.

**Exact fix (Option A):** Added ZST `ReadbackAuthority(())` with private field, no public constructor, `compile_fail` on struct literal, and `#[doc(hidden)] for_kernel_readback()` for `simthing-gpu` session/buffer readback paths only. Every bridge returning a sealed type now requires `ReadbackAuthority`:

| Bridge | Gated |
|---|---|
| `threshold_events_from_gpu` | yes |
| `threshold_emissions_from_gpu` | yes |
| `emission_records_from_gpu` | yes |
| `threshold_event_from_pass7_readback` | yes |
| `emission_record_from_kernel_emit_event` | yes |
| `emission_record_from_cpu_oracle` | yes |
| `threshold_emission_from_cpu_oracle` | yes |

**Public bridge audit (post-0R):**

| Function | Returns sealed type? | Forgeable input without authority? | Verdict |
|---|---|---|---|
| `threshold_events_from_gpu` | yes | POD forge blocked — requires `ReadbackAuthority` | OK |
| `threshold_emissions_from_gpu` | yes | same | OK |
| `emission_records_from_gpu` | yes | same | OK |
| `threshold_event_from_pass7_readback` | yes | raw tuple blocked — requires authority | OK |
| `emission_record_from_kernel_emit_event` | yes | same | OK |
| `emission_record_from_cpu_oracle` | yes | same | OK |
| `threshold_emission_from_cpu_oracle` | yes | same | OK |
| `cpu_oracle_threshold_events` | yes | derives from buffer crossings + registrations | OK — not POD launder |
| `validate_and_mint_placed_participants_by_location_id` | `PlacedParticipant` | validates structural table | OK |
| `ResolvedWriteAuthority::for_boundary_install` | write token | hidden gpu boundary path | OK (write seal) |

**New compile-fail proofs (3, not counting prior 7 direct-forge):**

| Proof | Catches |
|---|---|
| `readback.rs` `external_pod_bridge_launder` | Forged `ThresholdEventGpu` + bridge without authority |
| `readback.rs` `external_emission_pod_bridge_launder` | Forged `EmissionRecordGpu` + bridge without authority |
| `readback_authority.rs` `external_readback_authority_forge` | Direct `ReadbackAuthority(())` construction |

**0R proofs rerun:** `cargo test -p simthing-kernel --doc` (10 compile_fail), `cargo test -p simthing-gpu threshold --lib` (18), `cargo test -p simthing-sim --test s6_threshold_sunset` (4), `c1_threshold_perf --release` (0.2060 ms readback).

**0R scope:** `crates/simthing-kernel/src/sealed/readback_authority.rs`, `readback.rs`, `sealed/mod.rs`, `lib.rs`; `simthing-gpu` session/world_state/cpu_oracle/emission_accumulator call sites.

**0R performance:** Prior extraction readback `new_ms=0.1888`; post-0R `new_ms=0.2060` (baseline pre-extraction `0.2145`). ZST authority parameter is zero-cost; no regression beyond noise.

**0R inlining:** `ReadbackAuthority` is a ZST passed by value (same pattern as `ResolvedWriteAuthority`); extraction LTO/inlining proof still applies — authority param erases at compile time, no runtime branch.

## 0R2 remediation — in-crate readback minting

**DA hold source:** 0R `ReadbackAuthority::for_kernel_readback()` is `pub` (`#[doc(hidden)]` only hides docs). Any crate depending on `simthing-kernel` can mint authority and launder forged GPU POD through public bridges — token gating cannot work cross-crate (Rust has no friend-crate visibility).

**Why token gating fails cross-crate:** private tuple field blocks struct literal, but a public minter reopens the seal; appearance is not enforcement.

**Exact readback/source-buffer move:**

| Kernel-owned type | Buffers | Public read API |
|---|---|---|
| `EmissionRecordReadback` | emission records + count | `read_records`, `read_records_capped` → `Vec<EmissionRecord>` |
| `ThresholdEmissionReadback` | threshold emissions + count | `read_threshold_emissions`, `read_threshold_events` → sealed vecs |
| `ThresholdEventCandidatesReadback` | Pass 7 event_candidates + count | `read_events` → `Vec<ThresholdEvent>` |

GPU dispatch binds via `#[doc(hidden)] records_binding()` / `count_binding()` only; mint uses `pub(crate)` sealed minters inside kernel.

**Removed public minters/bridges:**

- `ReadbackAuthority` + `for_kernel_readback()` (deleted)
- `threshold_events_from_gpu`, `threshold_emissions_from_gpu`, `emission_records_from_gpu`
- `threshold_event_from_pass7_readback`, `emission_record_from_kernel_emit_event`
- `emission_record_from_cpu_oracle`, `threshold_emission_from_cpu_oracle`

**CPU oracle mint moved in-crate:** `cpu_oracle.rs` (`execute_ops_cpu_with_emissions`, `execute_threshold_ops_cpu`), `emission_oracle.rs` (`cpu_oracle_emission_records`); gpu re-exports or thin-wraps.

**Public API audit (post-0R2):**

| Function | Returns sealed? | Forgeable input? | Verdict |
|---|---|---|---|
| `EmissionRecordReadback::read_records` | yes | no — reads kernel-owned GPU buffer | OK |
| `ThresholdEmissionReadback::read_threshold_emissions/events` | yes | no | OK |
| `ThresholdEventCandidatesReadback::read_events` | yes | no | OK |
| `cpu_oracle_threshold_events` | yes | derives from buffer crossings + registrations | OK |
| `execute_ops_cpu_with_emissions` | yes | derives from op execution | OK |
| `execute_threshold_ops_cpu` | yes | derives from threshold execution | OK |
| `cpu_oracle_emission_records` | yes | derives from flat + registration formulas | OK |
| `validate_and_mint_placed_participants_by_location_id` | `PlacedParticipant` | validates structural table | OK |
| `ResolvedWriteAuthority::for_boundary_install` | write token | hidden gpu boundary path | OK |

No public function accepts forgeable POD and returns sealed authority.

**Compile-fail proofs (11 total doc tests):**

| Proof | Catches |
|---|---|
| `readback.rs` `external_mint_then_launder_threshold_event` | Mint authority + forged POD + bridge (real attack) |
| `readback.rs` `external_mint_then_launder_emission_record` | Mint authority + forged emission POD + bridge |
| `readback.rs` `external_pod_bridge_launder` | Bridge without authority (legacy) |
| `readback.rs` `external_emission_pod_bridge_launder` | Emission bridge without authority |
| Prior 7 direct-forge compile_fails on sealed types | Struct literal / named constructor forge |

**0R2 proofs rerun:** `cargo test -p simthing-kernel --doc` (11 compile_fail), `cargo test -p simthing-gpu threshold --lib` (18/18), `cargo test -p simthing-sim --test s6_threshold_sunset` (4/4), `mapgen_rf_stead_binding` (7/7), `c1_threshold_perf --release` readback `new_ms=0.2017`.

**Dependency graph:** `simthing-kernel` → `simthing-core`, `bytemuck`, `thiserror`, `wgpu`; no `simthing-kernel → simthing-gpu` cycle; `#![forbid(unsafe_code)]` retained.

**Value parity:** threshold scan CPU oracle, session readback, s6 sunset, mapgen RF binding — all green; no semantic/value changes.

**Performance parity:** pre-extraction `0.2145` ms; post-extraction `0.1888`; post-0R `0.2060`; post-0R2 `0.2017` — within noise; no new hot-path runtime check or dynamic dispatch (readback path unchanged except crate boundary for mint).

**Inlining proof:** accumulator hot path still dispatches from `simthing-gpu` session with kernel-owned buffer bindings only; readback mint is cold-path post-dispatch; no cross-crate indirection on encode/dispatch.

**0R2 scope ledger:**

| File | Change |
|---|---|
| `crates/simthing-kernel/src/gpu_readback.rs` | added — kernel-owned readback buffers + in-crate mint |
| `crates/simthing-kernel/src/cpu_oracle.rs` | added — CPU oracle mint in-crate |
| `crates/simthing-kernel/src/emission_oracle.rs` | added — emission CPU oracle |
| `crates/simthing-kernel/src/readback.rs` | compile_fail docs only; bridges removed |
| `crates/simthing-kernel/src/sealed/readback_authority.rs` | deleted |
| `crates/simthing-kernel/src/lib.rs`, `sealed/mod.rs` | exports updated |
| `crates/simthing-gpu/src/accumulator_op/session.rs` | embed kernel readback types |
| `crates/simthing-gpu/src/world_state.rs` | `ThresholdEventCandidatesReadback` |
| `crates/simthing-gpu/src/accumulator_op/cpu_oracle.rs` | re-export kernel oracle |
| `crates/simthing-gpu/src/emission_accumulator.rs` | delegate to kernel emission oracle |
| `docs/tests/kernel_crate_extract_0_results.md` | this section |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | rung 6 HELD → PROBATION |
| `docs/tests/current_evidence_index.md` | 0R2 row |

**Not touched:** `deny.toml`, 0.0.8.5 ClauseScript, closeout landings, full `AccumulatorOpSession` orchestration move.

**Known gaps / next:** DA re-review PROBATION → DONE; optional full session orchestration extraction; optional `cpu_oracle_threshold_events` test-only scope tighten.

