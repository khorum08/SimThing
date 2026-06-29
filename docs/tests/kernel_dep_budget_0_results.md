# KERNEL-DEP-BUDGET-0 Results

## Status

**PROBATION** — `simthing-sim` direct dependency set audited, allowlisted, and gated at build time; unused `tempfile` dev-dep removed. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-dep-budget-0`
- PR: #983
- Merge: `4641715665` (master)

## What changed

- Added `crates/simthing-sim/src/dependency_budget.rs` — lib tests parse `Cargo.toml` and fail on any direct runtime/dev dependency outside the DA-signed allowlist.
- Added root `deny.toml` — workspace dependency-policy stub; `simthing-sim` allowlist enforcement is the lib test (no `cargo-deny` in repo toolchain today).
- Removed unused `[dev-dependencies] tempfile` from `crates/simthing-sim/Cargo.toml` (zero references in crate sources/tests).

## Dependency audit

| Dependency | Section | Used | Action |
|---|---|---|---|
| `simthing-core` | runtime | Yes — fabric/index/types, `SimThing`, accumulator surfaces | Retained |
| `simthing-gpu` | runtime | Yes — GPU tick backends, buffer upload, mapping/accumulator ops | Retained |
| `simthing-feeder` | runtime | Yes — `BoundaryProtocol`, `DispatchCoordinator`, boundary requests | Retained |
| `bytemuck` | runtime | Yes — `mapping_plan_tick.rs` buffer cast_slice | Retained |
| `thiserror` | runtime | Yes — `SimTickError`, `ReplayError` | Retained |
| `serde` | runtime | Yes — replay/threshold/registry serialization derives | Retained |
| `serde_json` | runtime | Yes — replay JSONL, threshold roundtrip tests | Retained |
| `tempfile` | dev | No — no `tempfile::` usage anywhere in crate | **Removed** |

No other direct dependencies present before or after.

## Allowlist + rationale

| Dependency | Rationale |
|---|---|
| `simthing-core` | Semantic-free fabric/index/SimThing types — kernel precursor surface |
| `simthing-gpu` | GPU resident tick execution — behavior is EML/WGSL data, not sim-side semantics |
| `simthing-feeder` | Boundary-protocol / dispatch-coordinator seam — day-boundary orchestration inputs |
| `bytemuck` | Zero-cost POD buffer casts for GPU upload paths |
| `thiserror` | Typed tick/replay error surfaces without pulling `anyhow` |
| `serde` | Deterministic replay/threshold persistence derives |
| `serde_json` | JSONL replay I/O and threshold semantic roundtrip |

Dev allowlist: **empty** (no dev-deps after audit).

Gate location: `crates/simthing-sim/src/dependency_budget.rs` (`RUNTIME_ALLOWLIST`, `DEV_ALLOWLIST`).

## Adversarial gate proof

| Check | Result |
|---|---|
| `dependency_gate_rejects_new_simthing_sim_dependency` | **PASS** — injects `anyhow = { workspace = true }` into parsed manifest; gate returns `unallowlisted simthing-sim runtime dependency \`anyhow\`` |
| `simthing_sim_direct_dependencies_match_allowlist` | **PASS** — live `Cargo.toml` matches allowlist exactly |

Local negative (manual): adding any line under `[dependencies]` without updating `RUNTIME_ALLOWLIST` fails `cargo test -p simthing-sim dependency_gate_rejects --lib`.

## Load-bearing proofs (+ what each catches)

| Proof | Catches |
|---|---|
| `cargo fmt -p simthing-sim -- --check` | Format drift in touched sim crate |
| `cargo check -p simthing-sim` | Build break after dev-dep removal |
| `cargo test -p simthing-sim dependency_gate_rejects --lib` | Sidecar dep added without allowlist update |
| `cargo test -p simthing-sim simthing_sim_direct_dependencies_match_allowlist --lib` | Allowlist/manifest mismatch |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | AS-4 public semantic-free surface regression |
| `cargo test -p simthing-sim sim_gpu_resident_state_ticks_vertical_seed_20_10 --test accumulator_plan_tick_convergence` | Resident tick path unchanged (**SKIP — pre-broken on master**, AS-INDEX-NEWTYPES fixture drift) |

## Value parity

No resolved-value or runtime behavior change. Gate is compile-time/test-only; one unused dev dependency removed.

## Performance parity

Zero runtime impact by construction — no hot-path code touched. Resident-tick smoke `sim_gpu_resident_state_ticks_vertical_seed_20_10` is **pre-broken on master** (`accumulator_plan_tick_convergence` fixture uses raw `u32` where `SlotIndex`/`ColumnIndex` are required — unrelated to this rung); no runtime delta expected for build-time-only gating.

## Scope Ledger

| File | Why touched |
|---|---|
| `crates/simthing-sim/Cargo.toml` | Remove unused `tempfile` dev-dep |
| `crates/simthing-sim/src/dependency_budget.rs` | Allowlist gate tests (new) |
| `crates/simthing-sim/src/lib.rs` | `mod dependency_budget` test wiring |
| `deny.toml` | Workspace dep-policy stub + gate documentation |
| `docs/tests/kernel_dep_budget_0_results.md` | Evidence ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 2 OPEN → PROBATION |

**Not touched:** `simthing-sim` runtime logic, `design_0_0_8_5_clausescript_terran_pirate_galaxy.md`, write/emission/participation seals, crate extraction, workspace dependency version bumps.

## Known gaps / next

- DA re-review: PROBATION → DONE.
- Install `cargo-deny` in CI/toolchain if workspace-wide `deny check` is desired beyond the lib test.
- **`KERNEL-WRITE-SEAL-0`** — next kernel seal rung (0.0.8.5 STEAD precondition).
