# Docs Cleanup Pre-Phase M — Test Results

**Date/time:** 2026-05-19  
**Base HEAD:** `48203193748c5cc88a18b174f5f8fcb0b3bb6900` (Mapping ADR merge, PR #217)  
**Branch:** `docs-cleanup-pre-phase-m`  
**Final commit SHA:** `715a334` (docs-cleanup-pre-phase-m)  
**Scope:** Docs-only — no production Rust/WGSL changes.

---

## Goal

Archive superseded mapping/SEAD sandbox preserves, candidate notes, revert reports, and full logs; consolidate active mapping guidance around the approved Mapping ADR, V7.7, and invariants; leave Phase M-1 as the unambiguous next coding task.

---

## Archive directories created

| Path | Purpose |
|------|---------|
| `docs/workshop/archive/mapping/` | Superseded mapping sandbox preserves and candidate notes |
| `docs/workshop/archive/sead/` | Superseded SEAD sandbox preserves, prototype WGSL, notes |
| `docs/workshop/archive/resource_flow/` | Reserved (empty) |
| `docs/tests/archive/mapping/` | Mapping sandbox full logs |
| `docs/tests/archive/sead/` | SEAD sandbox full logs |
| `docs/tests/archive/reverts/` | Revert verification reports and full logs |
| `docs/tests/archive/resource_flow/` | Reserved (empty) |
| `docs/tests/archive/v7_6/` | V7.6 promotion evidence (superseded by guardrail/parked reports) |

Index files: `docs/workshop/archive/README.md`, `docs/tests/archive/README.md`, plus category READMEs under `mapping/`, `sead/`, and `reverts/`.

---

## Files moved (`git mv`)

### Workshop → `docs/workshop/archive/mapping/` (10)

- `mapping_active_frontier_halo_candidate.rs`
- `mapping_atlas_batching_candidate.rs`
- `mapping_atlas_isolation_candidate.rs`
- `mapping_cadence_tiers_candidate.rs`
- `mapping_dirty_macro_region_candidate.rs`
- `mapping_optimization_remedial_candidate_notes.md`
- `mapping_optimization_remedial_sandbox_code_preserve.rs`
- `mapping_optimization_toolkit_candidate_notes.md`
- `mapping_optimization_toolkit_sandbox_code_preserve.rs`
- `mapping_source_policy_candidate.rs`

### Workshop → `docs/workshop/archive/sead/` (17)

- `sead_operator_toolkit_sandbox_code_preserve.rs`
- `sead_sandbox_code_preserve.rs`
- `sead_strategic_horizon_sandbox_code_preserve.rs`
- `sead_tensor_stencil_clamped_prototype.wgsl`
- `sead_tensor_stencil_decayed_normalized_refinement.wgsl`
- `sead_tensor_stencil_decayed_prototype.wgsl`
- `sead_tensor_stencil_directed_refinement.wgsl`
- `sead_tensor_stencil_normalized_prototype.wgsl`
- `sead_tensor_stencil_normalized_refinement.wgsl`
- `sead_tensor_stencil_pingpong_refinement.wgsl`
- `sead_tensor_stencil_prototype.wgsl`
- `sead_tensor_stencil_prototype_notes.md`
- `sead_tensor_stencil_raw_additive_prototype.wgsl`
- `sead_tensor_stencil_refinement_notes.md`
- `sead_tensor_stencil_refinement_prototype.wgsl`
- `sead_tensor_stencil_refinement_sandbox_code_preserve.rs`
- `sead_tensor_stencil_wgsl_sandbox_code_preserve.rs`

### Tests → `docs/tests/archive/reverts/` (14)

- All `revert_*_test_results.md` (7)
- All `revert_*_full.log` (7)

### Tests → other archive subdirs

- `docs/tests/archive/mapping/` — `mapping_optimization_toolkit_sandbox_full.log`, `mapping_optimization_remedial_sandbox_full.log`
- `docs/tests/archive/sead/` — 5 SEAD sandbox full logs
- `docs/tests/archive/v7_6/` — `v7_6_structured_field_stencil_promotion_test_results.md`, `v7_6_structured_field_stencil_promotion_full.log`

### Kept active in `docs/tests/` root

- `v7_6_structured_field_stencil_guardrail_hardening_test_results.md`
- `v7_6_structured_field_stencil_parked_state_test_results.md`
- `mapping_optimization_toolkit_sandbox_test_results.md`
- `mapping_optimization_remedial_sandbox_test_results.md`
- `sead_field_intelligence_sandbox_test_results.md`
- `sead_operator_toolkit_sandbox_test_results.md`
- `sead_strategic_horizon_sandbox_test_results.md`
- `sead_tensor_stencil_wgsl_sandbox_test_results.md`
- `sead_tensor_stencil_refinement_sandbox_test_results.md`

---

## New / updated active docs

| File | Change |
|------|--------|
| `docs/workshop/mapping_current_guidance.md` | **Created** — pointer to ADR, V7.7, invariants, Phase M-1 |
| `docs/workshop/workshop_current_state.md` | V7.7 posture, docs-cleanup wording, archive paths |
| `docs/todo.md` | Docs cleanup done; Phase M-1 next |
| `docs/worklog.md` | ADR + cleanup entry; archive path fixes |
| `docs/accumulator_op_v2_production_plan.md` | Phase M gate note; SEAD preserve paths → archive |
| `docs/adr/mapping_sparse_regioncell.md` | Remedial candidate notes → archive path |
| `docs/design_v7_7.md` | Remedial candidate notes → archive path |
| Active test summaries (9) | Full-log and preserve links → archive paths |
| Archive README indexes | Created/updated |

Active read order preserved: invariants → Mapping ADR → design_v7_6 → design_v7_7 → cited test evidence.

---

## Commands run

| Command | Result |
|---------|--------|
| `git status --short` | **PASS** — docs-only staged changes; unrelated workshop `.txt` left unstaged |
| `git rev-parse HEAD` | **PASS** — base `4820319` |
| `rg -n "TODO\|FIXME\|BROKEN LINK\|missing\|not found" docs/adr docs/design_v7_7.md docs/invariants.md docs/accumulator_op_v2_production_plan.md docs/workshop/workshop_current_state.md` | **PASS** — no broken-link markers in active authoritative docs |
| `rg -n "mapping_optimization\|sead_\|revert_\|code_preserve\|candidate_notes" docs` | **PASS** — active-doc references point to archive or ADR; historical revert docs updated |
| `cargo check --workspace` | **PASS** |
| `cargo test --workspace` (`CARGO_BUILD_JOBS=1`) | **PASS** |

---

## Pass/fail table

| Criterion | Status |
|-----------|--------|
| Archive directories exist | **PASS** |
| Superseded mapping/SEAD workshop candidates archived | **PASS** |
| Stale revert/full-log artifacts archived | **PASS** |
| Active mapping guidance points to Mapping ADR | **PASS** |
| V7.7 and invariants remain authoritative | **PASS** |
| Production plan names Phase M-1 as next coding task | **PASS** |
| No production code changes | **PASS** |
| No mapping runtime implied | **PASS** |
| Reference search — no broken stale paths in active docs | **PASS** |
| This report created | **PASS** |
| `cargo check` / `cargo test` workspace | **PASS** |

---

## Final verdict

**PASS** — Docs cleanup completed; stale sandbox/revert/candidate artifacts archived; authoritative mapping guidance consolidated around Mapping ADR, V7.7, and invariants. Repo remains parked pending Phase M-1.
