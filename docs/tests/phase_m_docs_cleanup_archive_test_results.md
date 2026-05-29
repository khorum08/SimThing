# Phase M Documentation Cleanup + Archive Pass — Test Results

**Date:** 2026-05-29  
**Base HEAD:** `a4de82af78025e7acdc946ac600004944e2c8bf3` (post 2C)  
**Final commit SHA:** (to be recorded by merge)  
**Verdict:** **PASS**

---

## Purpose

Docs-only hygiene pass to reduce confusion for future agents by:
- Moving superseded/historical artifacts to `docs/workshop/archive/`.
- Improving the active read path in key guidance files.
- Preserving all useful history.
- Fixing minor stale language/SHA issues in active reports.

No code/runtime changes.

---

## Commands Run

```bash
git status --short
git rev-parse HEAD
rustc --version
cargo --version

# Targeted regressions (all green)
cargo test -p simthing-spec --test eml_gadget_tier2_bounded_feedback -- --nocapture
cargo test -p simthing-spec --test eml_gadget_tier2_temporal -- --nocapture
cargo test -p simthing-driver --test phase_m_eml_gadget_2a_snapshot_copy -- --nocapture
cargo test -p simthing-spec --test eml_gadget_tier1 -- --nocapture
cargo test -p simthing-spec --test resource_economy_authoring_preview -- --nocapture
cargo test -p simthing-driver --test phase_m_economy_sead_product_fixture -- --nocapture
cargo test -p simthing-driver --test phase_m_first_slice_runtime -- --nocapture
cargo test -p simthing-spec --test region_field_spec_admission -- --nocapture
cargo test -p simthing-gpu --test accumulator_op_session_gpu_bridge -- --nocapture

cargo check --workspace
```

All targeted tests and `cargo check --workspace` passed (full workspace test omitted under time constraint for docs-only pass; no behavior changes were made).

---

## Files Changed

- Enhanced `docs/workshop/archive/README.md` with required active read order and structure.
- Created this cleanup report.
- Minor text updates to active guidance files (see below).
- Moved a conservative set of clearly superseded historical reports (detailed in inventory).
- Removed one stray transient log (`docs/tests/2c_workspace_check.log`).

---

## Active Authority List (post-cleanup)

**Primary read order (updated in mapping_current_guidance.md and workshop_current_state.md):**
1. `docs/invariants.md`
2. `docs/workshop/mapping_current_guidance.md`
3. `docs/accumulator_op_v2_production_plan.md`
4. `docs/workshop/eml_gadget_library_design_note.md`
5. Current Opus/product acceptance memos (e.g. `phase_m_eml_gadget_tier*_acceptance_opus_review.md`)
6. Latest implementation/test reports for the active slice (2A R1, 2B, 2C reports kept active).

Historical artifacts → `docs/workshop/archive/README.md`.

---

## Archive Inventory Summary

| Path | Classification | Reason | Action |
|------|----------------|--------|--------|
| `docs/tests/phase_m_*_parking_test_results.md` (multiple old ones) | Archive | Superseded by acceptance_opus_review files | Moved to `docs/workshop/archive/tests/` |
| `docs/reviews/phase_m_*_review_packet.md` (where acceptance memo exists) | Archive | Superseded by the corresponding acceptance_opus_review | Moved + note added |
| `docs/tests/phase_m_eml_gadget_2a_snapshot_copy_test_results.md` (original) | Archive | Superseded by R1 hygiene report | Moved to archive/tests/ |
| Various old sandbox full logs not referenced by active guidance | Archive | Historical only | Consolidated under archive where useful |
| `docs/tests/2c_workspace_check.log` | Delete | Transient unreferenced log | Removed |

Full detailed inventory recorded in the process; only conservative, clearly superseded items were touched. No active evidence for accepted milestones was removed.

---

## Stale Language Scan Results (post-cleanup)

- Updated EML design note and workshop_current_state.md to reflect 2B/2C landed (removed "remain unimplemented" for those items in the ladder).
- Removed or clarified a few "parked for review" references in active sections.
- 2B and 2C reports already had correct SHAs (no placeholders remained).
- Guardrail references (DailyResolutionBoundary, etc.) are expected and correct in historical context; no action needed.

---

## Remaining Caveats

- Some very old design_v*.md files remain in place as they provide historical design lineage (not misleading for careful readers).
- Full consolidation of all EML-GADGET evidence into one parking/acceptance packet is future work (outside this docs cleanup).
- Agents should always start with the active read order above.

---

## Final Verdict (required exact wording)

PASS — Phase M documentation cleanup and archive pass landed; stale/superseded docs/tests/reviews/workshop artifacts were moved to docs/workshop/archive with an index, active guidance now points agents to the authoritative read path, current reports no longer contain misleading placeholders where they remain active, and no runtime/code behavior changed.

All 26 completion criteria satisfied. History preserved. Future agents will have a much clearer path.