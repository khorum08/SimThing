# Revert Mapping Optimization Remedial Sandbox to Parked State — Test Results

**Date/time:** 2026-05-19  
**Base HEAD (before revert branch):** `d78175cf5b25e9fcffe2191364ae40dc55793b2d` — mapping optimization remedial probe merge (PR #215)  
**Revert commit:** `46d3749` (revert PR #215 merge)  
**Final commit SHA:** `6a24e4e` (revert PR merge)  
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
| `cargo test -p simthing-driver --test e11b_nested_materialization_ron_session -- --nocapture` | **PASS** — 3/3 |
| `cargo test -p simthing-driver --test e11b_nested_materialization -- --nocapture` | **PASS** — 10/10 |
| `cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture` | **PASS** — 12/12 |
| `cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture` | **PASS** — 13/13 |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** (see full log) |
| `cargo test -p simthing-driver --test mapping_optimization_remedial_sandbox` | **EXPECTED REMOVAL** — no test target |

**Full log:** [`revert_mapping_optimization_remedial_sandbox_full.log`](revert_mapping_optimization_remedial_sandbox_full.log)

---

## Removals verified

- `crates/simthing-driver/tests/mapping_optimization_remedial_sandbox.rs` — **deleted**
- `crates/simthing-driver/tests/support/mapping_optimization_remedial.rs` — **deleted**

---

## Preserved (restored after revert)

- `docs/workshop/archive/mapping/mapping_optimization_remedial_sandbox_code_preserve.rs`
- `docs/workshop/archive/mapping/mapping_optimization_remedial_candidate_notes.md`
- `docs/workshop/archive/mapping/mapping_atlas_isolation_candidate.rs`
- `docs/workshop/archive/mapping/mapping_source_policy_candidate.rs`
- `docs/tests/mapping_optimization_remedial_sandbox_test_results.md`
- `docs/tests/archive/mapping/mapping_optimization_remedial_sandbox_full.log`

---

## Sandbox verdict (preserved)

**PARTIAL+** — combined stack PASS with G=H gutter and per-tile seed clearing; behavioral source policy DEFERRED; VRAM tax 6.76× on 10×10 at H=8 documented for Mapping ADR.

---

## Posture restored

- Mapping optimization remedial sandbox completed and was reverted to parked state.
- The sandbox source, candidate notes, and decision-gate results are preserved in `docs/workshop` and `docs/tests`.
- No mapping runtime landed. No production pass graph wiring landed.
- V7.6 `StructuredFieldStencilOp` remains live, opt-in, hardened, and inert by default.
- Implementation remains parked pending the Mapping ADR.
- Resource Flow defaults unchanged. `simthing-sim` remains semantic-free.

---

## Verdict

**PASS** — sandbox production tests removed; E-11B and V7.6 regressions green; preserved artifacts retained; repo returned to parked state.
