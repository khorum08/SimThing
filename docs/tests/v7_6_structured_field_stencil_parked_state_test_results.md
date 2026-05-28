# V7.6 StructuredFieldStencilOp Parked State — Test Results

| Field | Value |
|---|---|
| Date/time | 2026-05-19 (local verification run) |
| Base HEAD | `81647c527c90bac2296f99e6350bd3bba9cfa015` |
| Final commit SHA | `495348d` |
| rustc | 1.95.0 (59807616e 2026-04-14) |
| cargo | 1.95.0 (f2d3ce0bd 2026-03-21) |
| Platform | Windows 10 (win32 10.0.26200) |
| GPU | Available (stencil tests executed) |

---

## Scope

Docs-only parking pass. No production code changes.

---

## Commands run

| Command | Result |
|---|---|
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** (11/11) |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** (2/2) |
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** (3/3) |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** (10/10) |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** (12/12) |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** (13/13) |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** |

---

## Parking summary

| Check | PASS / FAIL |
|---|---|
| V7.6 live and hardened in active docs | PASS |
| Implementation parked pending Mapping ADR | PASS |
| StructuredFieldStencilOp remains live toolkit code | PASS |
| No mapping runtime | PASS |
| No production pass graph wiring | PASS |
| Resource Flow defaults unchanged | PASS |
| simthing-sim semantic-free | PASS |
| Prior test artifacts preserved | PASS |

## Final verdict

**PASS** — V7.6 StructuredFieldStencilOp remains live, opt-in, hardened, and inert by default. Repo parked pending Mapping ADR.

## Notes

- Evidence retained: `v7_6_structured_field_stencil_promotion_test_results.md`, `v7_6_structured_field_stencil_promotion_full.log`, `v7_6_structured_field_stencil_guardrail_hardening_test_results.md`.
- Next work item: **Mapping ADR** (RegionCell fields, source policy, active-mask halo/frontier semantics, cadence tiers, column-aware parent bindings).
