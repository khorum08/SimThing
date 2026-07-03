# TEST-PARE-LINGERING-OWNER-DEEP-DEAD-TESTS-0 Results

## Status

PROBATION / ORCHESTRATOR REVIEW. Merge is not authorized for Codex.

## Mission

Delete lingering owner-deep dead/fossil tests that survived earlier deletion waves because Linux-side desktop/audio/windowing/GPU probing was treated as preservation pressure.

This rung does not execute or compile Linux-side desktop/audio/windowing/GPU binaries to justify deletion. Under ci_screening_surface.md §10, deletion is proven by coverage map, survivor compile floor, and owner/local deletion findings. Platform-unavailable or non-compiling deleted binaries are stronger delete signals, not preservation reasons.

## Scope

- Edited only non-protected `simthing-driver` integration tests plus inventory, boundary, and evidence docs.
- Did not touch Admission Substrate (`simthing-spec/tests/**`, `simthing-clausething/tests/**`), SimThing-Kernel, CI profiles, scanner logic, allowlists, workflows, or `.github/**`.
- Deleted individual fossil test functions, not whole files, because each edited driver file still contains retained behavior-regression representatives.

## Owner §10 Deletion Doctrine

Deletion proof used:

1. Coverage map: surviving representatives remain in the same driver files and in corresponding `simthing-spec` coverage.
2. Compile floor: only GHA-safe survivor crates are checked locally; driver/GPU/Studio crates are not compiled as deletion proof.
3. Owner/local deletion model: platform-unavailable or non-compiling owner-deep binaries are delete signals, not reasons to probe Linux desktop/GPU dependencies.

## Candidate Discovery

Read and cross-checked:

- `docs/ci_screening_surface.md`
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/tests_compile_floor_non_bevy_0r2_results.md`
- `docs/tests/tests_compile_floor_non_bevy_0_results.md`
- `scripts/ci/test_inventory.tsv`
- `scripts/ci/test_pare_boundary_rows.tsv`
- `docs/tests/test_repair_or_replace_true_oracle_binaries_0_review.tsv`
- `docs/tests/test_repair_or_replace_true_oracle_binaries_0_results.md`
- `docs/tests/test_pare_clausething_studio_dead_binaries_0_review.tsv`
- `docs/tests/test_pare_clausething_studio_dead_binaries_0_results.md`
- `docs/tests/current_evidence_index.md`

Targeted join: current inventory rows under `simthing-driver`, `simthing-gpu`, `simthing-mapeditor`, and `simthing-tools` whose boundary row still recommended DELETE and whose source file still existed. That yielded five current DELETE rows, all `simthing-driver` legacy-default fossils.

## DELETE Rows

| Path | Deleted test |
|---|---|
| `crates/simthing-driver/tests/local_allocation_recursive_source.rs` | `local_allocation_recursive_source_compile_legacy_default_preserved` |
| `crates/simthing-driver/tests/local_effect_recursive_source.rs` | `local_effect_recursive_source_compile_legacy_default_preserved` |
| `crates/simthing-driver/tests/owner_silo_recursive_source.rs` | `owner_silo_recursive_source_compile_legacy_default_preserved` |
| `crates/simthing-driver/tests/runtime_rf_tick_source.rs` | `runtime_rf_tick_source_compile_composes_legacy_and_recursive_plans` |
| `crates/simthing-driver/tests/runtime_rf_tick_source_selection.rs` | `runtime_rf_tick_source_selection_compile_legacy_default_preserved` |

## KEEP Rows

- `crates/simthing-mapeditor/tests/canonical_scenario_load_save_display.rs` / `studio_legacy_terran_pirate_still_loads_as_legacy_fixture`: kept because boundary ledger marks B-T7-ACTIVE-TP-LIVE-RUNG / NEVER_PARE while the TP product track is open.
- `crates/simthing-mapeditor/tests/terran_pirate_skeleton.rs` / `write_terran_pirate_skeleton_legacy_fixture`: kept for the same active TP live-rung reason.
- Golden-byte, oracle-parity, seal-proof, doc-named, and active TP rows in the searched owner-deep surfaces were not touched.

## ESCALATE / Follow-On Rows

- `crates/simthing-mapeditor/tests/typeface_lr8.rs`: Studio/typeface owner-deep review remains `TEST-PARE-STUDIO-TYPEFACE-OWNER-DEEP-0`.
- `crates/simthing-tools/tests/typeface_lr*.rs`: typeface family is interleaved with golden-byte/oracle/local-owner-deep signals; focused owner-deep review required.
- Admission Substrate deferred dead-binary candidates remain protected and route to `TEST-PARE-ADMISSION-SUBSTRATE-DEAD-BINARIES-0`.

## Coverage Map

| Deleted surface | Coverage owner |
|---|---|
| local allocation recursive source legacy-default row | Surviving driver recursive-source tests in the same file plus `simthing-spec/tests/local_allocation_recursive_source.rs` |
| local effect recursive source legacy-default row | Surviving driver recursive-source tests in the same file plus `simthing-spec/tests/local_effect_recursive_source.rs` |
| owner silo recursive source legacy-default row | Surviving driver recursive-source tests in the same file plus `simthing-spec/tests/owner_silo_recursive_source.rs` |
| runtime RF comparison legacy/recursive composition row | Surviving driver runtime RF comparison tests plus `simthing-spec/tests/runtime_rf_tick_source.rs` |
| runtime RF selectable legacy-default row | Surviving driver selectable-source tests plus `simthing-spec/tests/runtime_rf_tick_source_selection.rs` |

## Proof

- Doctrine Scan: PASS (`DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0 selftest=SKIPPED`)
- Digest: PASS (`gen_digest --check: PASS`)
- Inventory check: PASS (`TEST-INVENTORY-CHECK-VERDICT: PASS`; rows/discovered 4,240)
- Boundary check: PASS (`TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS`; historical PARED rows 1,505)
- Drift check: PASS (`TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS`; unledgered 0, stale 0)
- Survivor compile floor: PASS
  - `cargo check -p simthing-core --tests`
  - `cargo check -p simthing-kernel --tests`
  - `cargo check -p simthing-sim --tests`
  - `cargo check -p simthing-workshop --tests`
  - `cargo check -p simthing-mapgenerator --tests`
- `git diff --check origin/master...HEAD`: PASS

## Forbidden Proof Avoided

- No `cargo check -p simthing-driver --tests`
- No `cargo check -p simthing-gpu --tests`
- No `cargo check -p simthing-mapeditor --tests`
- No `cargo check -p simthing-tools --tests`
- No `cargo test -p` for driver/GPU/mapeditor/tools
- No ALSA/libasound, X/Xvfb, Wayland, Mesa/Vulkan, libudev, Bevy, winit, wgpu, mapeditor, typeface, desktop/GPU setup, or `apt-get`

## Inventory Delta

- Before: 4,245 rows
- After: 4,240 rows
- Delta: -5 inventory rows
- Boundary rows retained as historical DELETE rows and annotated with this rung.

## Graduation Routing

```text
Graduation routing:
  Status: PROBATION / ORCHESTRATOR REVIEW
  Risk class: deletion wave / owner-deep residue pare
  Protected corpus touched: no
  CI profile/gate/scanner/allowlist/workflow touched: no
  DA question: none from this implementation; merge remains not authorized by Codex per handoff
```
