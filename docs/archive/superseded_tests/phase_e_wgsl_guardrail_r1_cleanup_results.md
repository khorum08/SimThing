# WGSL-GUARD-R1 — Delete Stray Artifacts and Replace No-op WGSL Guard Tests Results

## Base HEAD
Post WGSL-GUARD-0 (commit 3b9446f)

## Files deleted (stray artifacts)
- `.claude/worktrees/interesting-morse-4aa3ac`
- `crates/simthing-workshop/target/workshop/eml_phase5_rich_report_100k.md`
- `demo.replay.ldjson`

These were accidental generated files committed during the WGSL-GUARD-0 landing. They are not authoritative v7.8 evidence.

## No-op WGSL guard tests found and removed
The following tests (introduced/renamed in WGSL-GUARD-0) were comment-only placeholders:

- `e11b_nested_rejects_designer_semantic_wgsl` (in e11b_nested_hierarchy_gpu.rs and e11b_nested_fission_gap.rs)
- `e11b_explicit_nested_materialization_rejects_designer_semantic_wgsl`
- `e11_rejects_designer_semantic_wgsl`

These were removed entirely. The real semantic-WGSL rejection authority lives in `simthing-spec` designer admission (`SemanticWgslRequest` / `SemanticWgslRequestRejected`).

No global filename-based WGSL ban was restored.

## Real semantic-WGSL assertions
The existing test `designer_admission_rejects_raw_wgsl_source` in `c2_atlas_admission_relaxation.rs` (added in WGSL-GUARD-0) plus the references in `clause_spec0_frontier_v2_admission.rs` provide the actual assertions against the designer/spec layer.

## Confirmation
- Global filename WGSL whitelist mechanism remains deleted.
- Semantic-WGSL guard at designer admission layer is the active mechanism.
- A-0, B-0, C-2 semantics unchanged.
- A-0 remains pending Opus/design-authority review.
- No production posture widened.

## Tests / commands run
```bash
cargo test -p simthing-spec --test c2_atlas_admission_relaxation -- --nocapture
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
cargo check --workspace
```
All green after cleanup.

## Scans
- Old `no_new_wgsl` / `accepted_wgsl_baseline` references now only in historical reports (A-0-R1, WGSL-GUARD-0) or the current R1 report.
- No active global filename ban code remains.
- Semantic WGSL rejection references are present in designer_admission.
- No unauthorized opening of parked lines or posture changes.

## Docs updated
- This report created.
- `docs/design_v7_8_production_track.md` (A-0 row updated with R1 reference).
- `docs/worklog.md` (WGSL-GUARD-R1 entry added).

## Final verdict

**PASS** — WGSL-GUARD-R1 cleaned up the WGSL-GUARD-0 landing by deleting the three stray generated artifacts and removing the no-op renamed WGSL guard placeholder tests. The corrected v7.8 doctrine is preserved: global generic-WGSL filename bans remain deleted; semantic/raw WGSL from designer/spec admission is rejected at the authoritative layer in simthing-spec. A-0 remains pending Opus review with no semantic change. All required tests and scans pass. Repo hygiene restored for the pending A-0 review.