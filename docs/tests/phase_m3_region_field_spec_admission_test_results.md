# Phase M-3 — RegionFieldSpec RON + Mapping Admission Framework — Test Results

**Date/time:** 2026-05-28  
**Base HEAD:** `7727913bd9c93aff5a292da53a883d3240903b5a` (Phase M-2.1 merge, PR #222)  
**Branch:** `phase-m3-region-field-spec-admission`  
**Final commit SHA:** `09e6a2c457dd1c80391f1c7694c1bab9c1c8eb45` (phase-m3-region-field-spec-admission)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell

---

## Goal

Implement Phase M-3 designer/spec admission layer: `RegionFieldSpec` RON, validation, and compile preview into generic substrate configs without production mapping runtime or pass graph wiring.

---

## RON roundtrip summary

| Case | Result |
|------|--------|
| Standard suppression field (`grid_size=10`, `SourceCappedNormalized`, `EveryTick`) | **PASS** — parse → serialize → reparse stable |
| `GameModeSpec.region_fields` + `mapping_execution_profile` serde | **PASS** — defaults roundtrip via existing game mode tests |

---

## Admission rejection summary

| Rejected condition | Result |
|--------------------|--------|
| `grid_size=0` | **PASS** |
| `grid_size=20` under StandardSquare (cap 10) | **PASS** |
| `grid_size=33` under ExtendedSquare (cap 32) | **PASS** |
| `SourceCappedNormalized` without `source_cap` | **PASS** |
| `Normalized` with `source_cap` | **PASS** |
| `horizon=0` | **PASS** |
| `horizon=12` without `allow_extended_horizon` | **PASS** |
| `horizon=17` (absolute cap 16) | **PASS** |
| Extended horizon without source-cap stability contract | **PASS** |
| `EveryN(0)` cadence | **PASS** |
| `reduction.child_slot_count=0` | **PASS** |
| Unknown formula class | **PASS** |
| `request_atlas_batching=true` (M-4 provisional) | **PASS** |

---

## Compiled/preview output summary

| Output | Mapping |
|--------|---------|
| `CompiledRegionFieldStencilSpec.width/height` | `grid_size` (square N) |
| `mask_mode` | `All` (production v1) |
| `boundary_mode` | `Zero` |
| `operator` | `Normalized` / `SourceCappedNormalized` |
| `source_policy` | `CallerManagedOneShotSeedThenZero` |
| `CompiledFieldCadence` | `EveryTick` / `EveryN { n }` / `OnEvent` |
| `ColumnAwareReductionSpec` | SlotRange Sum wrapper when reduction present |
| GPU bridge (test-only) | Intermediate spec validates as `StructuredFieldStencilConfig` |

First-slice-shaped spec (10×10 tactical suppression + reduction + `field_urgency`) compiles preview only; no execution wiring.

---

## Default-off posture summary

| Check | Result |
|-------|--------|
| `MappingExecutionProfile::default() == Disabled` | **PASS** |
| `RegionFieldSpec` presence alone does not enable execution | **PASS** |
| `PipelineFlags::default().use_accumulator_resource_flow == false` | **PASS** |
| `ResourceFlowExecutionProfile::default() == DefaultDisabled` | **PASS** |
| `passes.rs` / `session.rs` / `simthing-sim` contain no RegionFieldSpec runtime wiring | **PASS** |

---

## Commands run

| Command | Result |
|---------|--------|
| `cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test phase_m2_field_scheduler -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** — 16/16 |
| `cargo test -p simthing-driver --test structured_field_region_execution -- --nocapture` | **PASS** — 5/5 |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` | **PASS** |

---

## Pass/fail table

| Criterion | Status |
|-----------|--------|
| RegionFieldSpec RON exists | **PASS** |
| Valid RegionFieldSpec roundtrips | **PASS** |
| Square `grid_size=N` admitted with caps | **PASS** |
| N=0 and over-cap rejected | **PASS** |
| No rectangular designer width/height in v1 | **PASS** |
| Operator/source/horizon/cadence validation | **PASS** |
| Field formula class admission | **PASS** |
| Reduction compile over SlotRange Sum | **PASS** |
| MappingExecutionProfile default Disabled | **PASS** |
| Spec presence alone does not enable execution | **PASS** |
| First-slice spec compiles preview only | **PASS** |
| No mapping runtime | **PASS** |
| No production pass graph wiring | **PASS** |
| simthing-sim map-free | **PASS** |
| Resource Flow / E-11B ladder preserved | **PASS** |
| Full workspace check/test | **PASS** |

---

## Important excerpts

- New spec types: `crates/simthing-spec/src/spec/region_field.rs`
- Admission/compile: `crates/simthing-spec/src/compile/region_field_admission.rs`
- Tests A–J: `crates/simthing-spec/tests/region_field_spec_admission.rs`
- Perception field structure deferred (TODO in guidance); core RegionFieldSpec not blocked.

---

## Final verdict

**PASS** — Phase M-3 RegionFieldSpec RON and mapping admission framework landed; bounded square grid specs compile/preview to generic substrate configs; invalid/admitted cases are enforced at designer/spec layer; no production mapping runtime or pass graph wiring landed.
