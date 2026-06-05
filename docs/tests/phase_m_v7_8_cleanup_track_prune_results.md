# V7.8-CLEAN-0 — Active-Docs Slimming, Archive Move, and Stale Evidence Prune Results

## Base HEAD

`385f2f6b97d6c58ea81543f505acf2439706103a` (post L1-1 merge, pre V7.8-CLEAN-0)

## Classification summary

| Category | Count | Action |
|---|---|---|
| KEEP_ACTIVE_AUTHORITY | 10 core + 2 governing ADRs | retained |
| KEEP_ACTIVE_REFERENCED | L0/L1 + E-phase/FIELD_POLICY/M-JIT evidence | retained |
| ARCHIVE_CLOSED_REFERENCE | 1 production plan | moved + stub |
| ARCHIVE_SUPERSEDED_DESIGN | 4 design docs | moved |
| ARCHIVE_SUPERSEDED_WORKSHOP | 4 workshop docs + narrative split | moved |
| ARCHIVE_SUPERSEDED_TEST_EVIDENCE | 7 test reports | moved |
| DELETE_STALE_SUPERSEDED | — | none (outcomes summarized in active track) |
| DELETE_SCRATCH_TMP | 13 `.log` files | deleted |
| REVIEW_NEEDED | 3 items | preserved active (see below) |

## 1. Active authority (KEEP_ACTIVE_AUTHORITY)

| File | Role |
|---|---|
| `docs/design_v7_8.md` | v7.8 constitution |
| `docs/design_v7_8_production_track.md` | **PR ladder home** |
| `docs/design_v7_7.md` | CLOSED baseline |
| `docs/invariants.md` | binding constraints (unchanged) |
| `docs/workshop/field_policy_track.md` | FIELD_POLICY/Frontier charter |
| `docs/workshop/mapping_current_guidance.md` | compact status table |
| `docs/worklog.md` | append-only history |
| `docs/accumulator_op_v2_production_plan.md` | CLOSED stub pointer |
| `docs/adr/mapping_sparse_regioncell.md` | governing Mapping ADR |
| `docs/adr/resource_flow_substrate.md` | governing Resource Flow ADR |

Also active: `docs/design_v7_6.md`, `docs/design_v7.md` (referenced by v7.7/v7.8 chain); remaining ADRs under `docs/adr/` (not superseded).

## 2. Active referenced evidence (KEEP_ACTIVE_REFERENCED)

### L0 Frontier (accepted)

- `docs/tests/phase_m_frontier_v1_5_live_field_agent_route_results.md`
- `docs/tests/phase_m_frontier_v2_0_closed_loop_consumer_results.md`
- `docs/tests/phase_m_frontier_v2_1_candidate_evolution_results.md`
- `docs/tests/phase_m_frontier_v2_2_movement_feedback_application_results.md`
- `docs/tests/phase_m_frontier_v2_3_structural_feedback_application_results.md`
- `docs/tests/phase_m_frontier_v2_4_combined_feedback_loop_results.md`
- Supporting V1 chain: v1-0..v1-4, v1 acceptance review

### L1 simthing-spec (landed)

- `docs/tests/phase_m_l1_0_designer_admission_substrate_results.md`
- `docs/tests/phase_m_l1_1_designer_preflight_manifest_results.md`

### E-phase / E11 / Resource Flow / FIELD_POLICY (preserved)

- FIELD_POLICY V1 consolidation + OBS/EVENT/PIPE/ACT reports
- E11 workshop: `e11_implementation_handoff.md`, `e11_readiness_review.md`, `e11_hierarchical_allocation_design.md`
- M-JIT retained evidence (PROD-0, EXEC-1, sqrt/grad R1 reports per mapping guidance)
- M-4A/M-6A readiness gates, gradient M-5A..E reports, first-slice/product fixture chain

## 3. Archived (closed/superseded reference)

### Closed production

| From | To |
|---|---|
| `docs/accumulator_op_v2_production_plan.md` (full) | `docs/archive/closed_production/accumulator_op_v2_production_plan.md` |

### Superseded design

| From | To |
|---|---|
| `docs/design_v4.md` | `docs/archive/superseded_design/design_v4.md` |
| `docs/design_v5.md` | `docs/archive/superseded_design/design_v5.md` |
| `docs/design_v6.md` | `docs/archive/superseded_design/design_v6.md` |
| `docs/design_v6.5.md` | `docs/archive/superseded_design/design_v6.5.md` |

### Superseded workshop

| From | To |
|---|---|
| `docs/todo.md` | `docs/archive/superseded_workshop/todo.md` |
| `docs/workshop/workshop_current_state.md` | `docs/archive/superseded_workshop/workshop_current_state.md` |
| `docs/workshop/simthing_spec_progress_log.md` | `docs/archive/superseded_workshop/simthing_spec_progress_log.md` |
| mapping guidance verbose narrative (lines 74+) | `docs/archive/superseded_workshop/mapping_current_guidance_historical_narrative.md` |

### Superseded test evidence

| From | To | Reason |
|---|---|---|
| `phase_m_jit_doc_closeout_cleanup_results.md` | `docs/archive/superseded_tests/` | M-JIT closed at PROD-0; outcome in active track |
| `phase_m_sqrt_doc0_active_guidance_integration_results.md` | `docs/archive/superseded_tests/` | superseded by FIELD_POLICY-V1 consolidation |
| `phase_m_frontier_v1_post_acceptance_roadmap_results.md` | `docs/archive/superseded_tests/` | superseded by v7.8 production track |
| `revert_mapping_atlas_algebraic_mask_sandbox_to_parked_state_test_results.md` | `docs/archive/superseded_tests/` | parking outcome in active guidance |
| `restore_m4_parked_posture_test_results.md` | `docs/archive/superseded_tests/` | parking outcome in active guidance |
| `m4a_architectural_implications_doc_update_test_results.md` | `docs/archive/superseded_tests/` | ratification summarized in v7.8 track |
| `opus_m4a_ratification_docs_reconciliation_test_results.md` | `docs/archive/superseded_tests/` | ratification summarized in v7.8 track |

### New archive index

- `docs/archive/README.md`

## 4. Deleted (scratch/tmp/stale)

| File | Reason |
|---|---|
| `docs/tests/mapping_atlas_algebraic_mask_sandbox_full.log` | scratch log |
| `docs/tests/phase_m_boundary_cadence_doctrine_full.log` | scratch log |
| `docs/tests/phase_m_daily_economy_fixture_full.log` | scratch log |
| `docs/tests/phase_m_economy_field_policy_product_fixture_full.log` | scratch log |
| `docs/tests/phase_m_eml_gadget_2a_snapshot_copy_full.log` | scratch log |
| `docs/tests/phase_m_eml_gadget_2a_snapshot_copy_r1_hygiene_full.log` | scratch log |
| `docs/tests/phase_m_eml_gadget_2b_velocity_decay_ema_full.log` | scratch log |
| `docs/tests/phase_m_first_slice_map_residency_full.log` | scratch log |
| `docs/tests/phase_m_first_slice_summary_validity_full.log` | scratch log |
| `docs/tests/phase_m_jit_sqrt_exact4e_exhaustive_batches.log` | scratch log |
| `docs/tests/phase_m_jit_sqrt_exact5f_exhaustive_batches.log` | scratch log |
| `docs/tests/phase_m_queue_write_scale_hardening_full.log` | scratch log |
| `docs/tests/phase_m_resource_economy_authoring_ergonomics_full.log` | scratch log |

No SHA/fingerprint reconciliation performed.

## 5. Review needed (preserved active)

| File | Reason |
|---|---|
| `docs/design_v7.md` | older design chain; still referenced by v7.6/v7.7; not archived without explicit supersession check |
| `docs/design_v7_6.md` | constitutional surfacing in mapping guidance |
| `docs/adr_accumulator_op_v2.md` | historical ADR; may still be cited; left active |

## 6. Active pointer updates

| File | Update |
|---|---|
| `docs/design_v7_8.md` | accumulator companion → CLOSED stub + archive path |
| `docs/design_v7_8_production_track.md` | accumulator companion; Cleanup / evidence hygiene section + V7.8-CLEAN-0 row |
| `docs/workshop/mapping_current_guidance.md` | read order → production track first; V7.8-CLEAN-0 row; narrative archived |
| `docs/workshop/field_policy_track.md` | V7.8-CLEAN-0 cleanup note |
| `docs/workshop/README.md` | points to v7.8 production track; archived superseded workshop docs |
| `docs/worklog.md` | append-only V7.8-CLEAN-0 line |
| `docs/accumulator_op_v2_production_plan.md` | replaced with CLOSED stub |

## 7. Accumulator production-plan handling

**Stub active path + archive full content.**

- Active stub at `docs/accumulator_op_v2_production_plan.md` (19 lines) points to `design_v7_8_production_track.md`.
- Full closed plan at `docs/archive/closed_production/accumulator_op_v2_production_plan.md`.

## 8. v7.8 production track as ladder home

Confirmed. Active read order in `mapping_current_guidance.md` item 4 is `design_v7_8_production_track.md`. Constitution, production track, and accumulator stub all cross-link correctly.

## 9. Intentionally parked

| Item | Posture |
|---|---|
| L2 CLAUSE-SPEC-0 | parked downstream of L1 |
| L3 ClauseThing / ClauseScript | parked downstream of L2 |
| Lines A/B/C (E-11B, D-2/D-2a, M-4/M-4A) | parked behind named scenarios |
| FrontierV2-5 | rejected (hygiene loop) |
| ACT-5 / EVENT-3 / OBS-5 / PIPE-1 | not authorized (negative refs only) |

## 10. Next implementation gate

**L1 simthing-spec buildout** continues (L1-0 and L1-1 landed; next L1 step per product/design authority — not CLAUSE-SPEC-0, not ClauseThing).

## Files inspected

- `docs/` (maxdepth 2): 130+ files
- `docs/tests/` (maxdepth 1): 107 active report files (post-cleanup)
- `docs/workshop/`: active guidance + design notes
- `docs/adr/`: 9 ADR files
- `crates/simthing-spec/tests/`: L1-0, L1-1 tests (unchanged)
- `crates/simthing-driver/tests/`: Frontier/FIELD_POLICY acceptance tests (unchanged)

## Required scans

### `find docs -maxdepth 2 -type f | sort`

PowerShell equivalent run (WSL unavailable). See git status for changed paths; 130+ files under `docs/` at depth ≤2.

### `find docs/tests -maxdepth 1 -type f | sort`

107 `.md` report files remain active under `docs/tests/` (13 `.log` scratch files removed).

### Reference scan (active authority docs)

```
rg "accumulator_op_v2_production_plan|design_v7_8_production_track|phase_m_.*results|frontier_v|l1_0|E-11|E11|resource_flow|D-2|M-4|atlas" docs/design_v7_8.md docs/design_v7_8_production_track.md docs/workshop/field_policy_track.md docs/workshop/mapping_current_guidance.md docs/adr docs/invariants.md docs/worklog.md
```

Result: active authority docs reference L0/L1 reports, E11/Resource Flow ADR, Lines A/B/C; production track is primary ladder reference.

### Rejected ladder authorization scan

```
rg "FrontierV2-5|ACT-5|EVENT-3|OBS-5|PIPE-1" crates docs
```

Result: **no authorization in crates**; docs contain guardrail/negative references only (expected PASS).

### CLAUSE-SPEC / ClauseThing posture

```
rg "CLAUSE-SPEC-0|ClauseThing|ClauseScript" docs/design_v7_8_production_track.md docs/workshop/field_policy_track.md docs/workshop/mapping_current_guidance.md
```

Result: CLAUSE-SPEC is L2/downstream; ClauseThing/ClauseScript parked (expected PASS).

### Scratch scan

```
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```

Result: **0 files** after deletion (PASS).

## Commands run

| Command | Result |
|---|---|
| `git mv` (archive moves) | 15 files relocated |
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

No full test battery run (docs-only cleanup; no code/test file changes).

## Tests/checks run

- `cargo check --workspace` — green
- No `cargo test --workspace --no-run` (no test file moves/deletes)

## Transient cleanup result

13 scratch `.log` files deleted from `docs/tests/`. No separate scratch logs retained.

## v7.8 track status after cleanup

| Ladder | Status |
|---|---|
| L0 Frontier consumer | landed + ACCEPTED |
| **L1 simthing-spec buildout** | **active** (L1-0, L1-1 Done) |
| L2 CLAUSE-SPEC | parked |
| L3 ClauseThing | parked |
| Lines A/B/C | parked |
| V7.8-CLEAN-0 | **Done** (this report) |

## Final verdict

**PASS** — V7.8-CLEAN-0 slimmed active docs, archived closed/superseded design/workshop/production files, stubbed the closed AccumulatorOp v2 production plan, deleted only stale/scratch evidence (13 `.log` files), preserved authoritative L0/L1 and E-phase evidence, updated active pointers to the v7.8 production track, saved cleanup results in `docs/tests`, avoided SHA/fingerprint reconciliation, made no runtime behavior changes, and left the next implementation gate as **L1 simthing-spec buildout**.
