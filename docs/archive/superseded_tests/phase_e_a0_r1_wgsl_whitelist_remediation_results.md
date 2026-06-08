# A-0-R1 — Fix Stale E-11B WGSL Whitelist and Re-verify A-0

**Date:** 2026-05-30  
**Base HEAD:** `23cc864a72a7865670ac32236b1cb920b2184fae` (post A-0 / PR #358)  
**Class:** Verification remedial (test-only / whitelist-only)  
**Verdict:** **PASS — A-0-R1 fixed stale E-11B no-new-WGSL whitelist; A-0 remains pending Opus/design-authority review**

---

## Pre-edit evaluation

| # | Question | Answer |
|---|---|---|
| 1 | Which exact tests failed in the A-0 report? | `e11b_nested_no_new_wgsl` in `e11b_nested_hierarchy_gpu` (11 passed; 1 failed) and `e11b_nested_fission_gap` (12 passed; 1 failed) |
| 2 | Which exact assertion failed? | `assert!(allowed.contains(&name.as_str()), "unexpected WGSL file {name}; …")` — file `structured_field_stencil_atlas_mask.wgsl` not in the hard-coded allow list |
| 3 | Why did accepted C-0 atlas WGSL trip the E-11B whitelist? | C-0 (ACCEPTED) landed `structured_field_stencil_atlas_mask.wgsl` under `simthing-gpu/src/shaders/`; E-11B guards retained a pre-C-0 V7.6 allow list that omitted it |
| 4 | What files are the accepted pre-A-0 WGSL baseline? | `accumulator_op.wgsl`, `snapshot.wgsl`, `structured_field_stencil.wgsl`, `structured_field_stencil_atlas_mask.wgsl` (C-0), `values_fill.wgsl`; `world_summary.wgsl` retained for baseline continuity (not currently on disk) |
| 5 | How does the remedial fix continue to detect A-0-added WGSL? | Shared helper `assert_only_accepted_project_wgsl_shaders()` enumerates `simthing-gpu/src/shaders/` and fails on any filename not in `ACCEPTED_WGSL_SHADER_BASELINE` |
| 6 | Did A-0 add any WGSL? | **No** — A-0 diff added no files under `simthing-gpu/src/shaders/` |
| 7 | Did A-0 alter Resource Flow runtime posture? | **No** — `use_accumulator_resource_flow` default remains `false`; no production wiring change |
| 8 | What commands now pass? | All required A-0-R1 commands below — full green against final tree |

---

## Failed-command summary (from A-0 report, historical)

```text
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
→ 11 passed; 1 failed (e11b_nested_no_new_wgsl)

cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
→ 12 passed; 1 failed (e11b_nested_no_new_wgsl)

Failure message (both):
  unexpected WGSL file structured_field_stencil_atlas_mask.wgsl
```

---

## Root cause

E-11B nested hierarchy/fission-gap tests duplicated a stale inline allow list from the flat-star era. C-0 acceptance added `structured_field_stencil_atlas_mask.wgsl` as accepted project state ([`phase_m_c_acceptance_review_results.md`](phase_m_c_acceptance_review_results.md), [`phase_m_c0_m4_atlas_protocol_oracle_results.md`](phase_m_c0_m4_atlas_protocol_oracle_results.md)) but the E-11B guards were never updated. A-0's own `assert_no_new_wgsl()` in `e11_nested.rs` scanned `simthing-gpu/src/` (not `shaders/`) with wrong legacy names (`atlas_mask.wgsl`), so it did not catch the mismatch.

---

## Remedial change

| File | Change |
|---|---|
| `crates/simthing-driver/tests/support/accepted_wgsl_baseline.rs` | **New** — canonical `ACCEPTED_WGSL_SHADER_BASELINE` + `assert_only_accepted_project_wgsl_shaders()` |
| `crates/simthing-driver/tests/e11b_nested_hierarchy_gpu.rs` | Delegate `e11b_nested_no_new_wgsl` to shared baseline |
| `crates/simthing-driver/tests/e11b_nested_fission_gap.rs` | Same |
| `crates/simthing-driver/tests/e11b_nested_materialization.rs` | Same (consistency) |
| `crates/simthing-driver/tests/e11_arena_allocation.rs` | Same (consistency) |
| `crates/simthing-driver/tests/support/e11_nested.rs` | A-0 guard delegates to shared baseline (scans `shaders/` correctly) |
| Docs | This report; A-0 report annotation; production track; mapping; worklog |

No production shader code, Resource Flow execution code, or A-0 semantics changed.

---

## Proof that C-0 atlas WGSL is accepted baseline

- C-ACCEPT-0 ([`phase_m_c_acceptance_review_results.md`](phase_m_c_acceptance_review_results.md)): **ACCEPT C-0 and C-1**
- C-0 landed `crates/simthing-gpu/src/shaders/structured_field_stencil_atlas_mask.wgsl` + `atlas_mask.rs`
- On-disk shaders dir (5 files) matches baseline minus optional `world_summary.wgsl`

---

## Proof that A-0 added no WGSL

A-0 implementation ([`phase_e_a0_nested_resource_flow_static_results.md`](phase_e_a0_nested_resource_flow_static_results.md)) touched `arena_hierarchy.rs`, test fixtures, and docs only. No new `.wgsl` under `crates/simthing-gpu/src/shaders/` in PR #358.

---

## Test results (A-0-R1 final tree)

```text
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture
→ 19 passed; 0 failed

cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
→ 12 passed; 0 failed

cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
→ 13 passed; 0 failed

cargo test -p simthing-driver --test resource_flow_opt_in -- --nocapture
→ 13 passed; 0 failed

cargo test -p simthing-spec --test v7_8_met_consumer_scenarios -- --nocapture
→ 10 passed; 0 failed

cargo check --workspace
→ Finished (green)
```

---

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `atlas_mask\|structured_field_stencil_atlas_mask\|no_new_wgsl\|WGSL whitelist\|A-0-R1\|phase_e_a0_r1` in crates/docs | Baseline + guard references | PASS — shared baseline module + updated guards |
| `A-0\|E-11B\|…` in crates/docs | A-0/E-11B evidence only | PASS |
| Guardrail terms in this report + v7.8/invariants | Guardrail-only | PASS — no unauthorized widening documented |
| `B-1\|D-2\|…\|M-6A\|M-5` in remediation + production track | Line B closed; Line C runtime deferred | PASS |
| `ClauseThing\|L3\|FrontierV2-5\|ACT-5\|…` in docs | Parked/rejected only | PASS |
| `ArenaSpec\|ResourceFlowSpec\|…` in simthing-sim | No semantic awareness | PASS — empty |
| `find docs/tests … *.log / *tmp* / *scratch*` | No scratch artifacts | PASS — none found |

---

## Transient cleanup

No scratch/tmp/log artifacts under `docs/tests/` requiring deletion.

---

## Final verdict

**PASS — A-0-R1 fixed the stale E-11B WGSL whitelist so accepted C-0 atlas WGSL is part of the baseline while A-0-added Resource Flow WGSL remains prohibited.** A-0, E-11B nested hierarchy GPU, E-11B nested fission gap, Resource Flow opt-in, MET scenario regression, and `cargo check --workspace` all pass against the final tree. No Resource Flow semantics, WGSL, runtime posture, dynamic enrollment, Policy B, selector rerun, slot compaction, hard-currency routing, Line C runtime, B-1, L3, FrontierV2-5, ACT/EVENT/OBS/PIPE, or invariants were changed. **A-0 remains implemented and pending Opus/design-authority review.**
