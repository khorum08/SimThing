# Revert Mapping Optimization Toolkit Sandbox to Parked State — Test Results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `09c626bac86ffcf476766fd8c40e11c2da050e55` — mapping optimization toolkit probe merge (PR #213)  
**Revert commit:** `4f2cf6b` (revert PR #213 merge)  
**Final commit SHA:** (set at merge)  
**rustc:** `rustc 1.95.0 (59807616e 2026-04-14)`  
**cargo:** `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`  
**Platform/OS:** Windows 10 (win32 10.0.26200), PowerShell  
**GPU availability:** Local GPU present — structured field stencil 11/11 PASS.

---

## Commands

| Command | Result |
|---------|--------|
| `cargo test -p simthing-gpu --test structured_field_stencil -- --nocapture` | **PASS** — 11/11 |
| `cargo test -p simthing-driver --test structured_field_stencil_parent_eml -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test eml_field_formula_admission -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-spec --test resource_flow_nested_participant_roundtrip -- --nocapture` | **PASS** — 2/2 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** |
| `cargo test -p simthing-driver --test mapping_optimization_toolkit_sandbox` | **EXPECTED REMOVAL** — no test target |

---

## Removals verified

- `crates/simthing-driver/tests/mapping_optimization_toolkit_sandbox.rs` — **deleted**
- `crates/simthing-driver/tests/support/mapping_optimization_toolkit.rs` — **deleted**

---

## Preserved (restored after revert)

- `docs/workshop/archive/mapping/mapping_optimization_toolkit_sandbox_code_preserve.rs`
- `docs/workshop/archive/mapping/mapping_optimization_toolkit_candidate_notes.md`
- `docs/workshop/archive/mapping/mapping_atlas_batching_candidate.rs`
- `docs/workshop/archive/mapping/mapping_cadence_tiers_candidate.rs`
- `docs/workshop/archive/mapping/mapping_dirty_macro_region_candidate.rs`
- `docs/workshop/archive/mapping/mapping_active_frontier_halo_candidate.rs`
- `docs/tests/mapping_optimization_toolkit_sandbox_test_results.md`
- `docs/tests/archive/mapping/mapping_optimization_toolkit_sandbox_full.log`

---

## Sandbox verdict (preserved)

**PARTIAL** — optimization toolkit promising for Mapping ADR; atlas batching and dirty skip strong; gutter/isolation and H-hop halo semantics need ADR policy before production adoption.

---

## Posture restored

- Mapping optimization toolkit sandbox reverted; decision-gate evidence preserved in docs.
- V7.6 `StructuredFieldStencilOp` remains live, opt-in, hardened, inert by default.
- No mapping runtime. No production pass graph wiring.
- Implementation parked pending Mapping ADR.
- Resource Flow defaults unchanged. `simthing-sim` remains semantic-free.

---

## Verdict

**PASS** — sandbox production tests removed; E-11B and V7.6 regressions green; preserved artifacts retained; repo returned to parked state.
