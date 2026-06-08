# WGSL-GUARD-0 — Delete Stale Generic-WGSL Bans; Preserve Semantic-WGSL Admission Guardrail Results

## Base HEAD
`330d007` (post A-0-R1 + C-2-ACCEPT-0 + B-0-ACCEPT-0)

## Files changed
- Removed stale global WGSL filename whitelist mechanism:
  - `crates/simthing-driver/tests/support/accepted_wgsl_baseline.rs` (deleted)
  - Removed `mod accepted_wgsl_baseline` and `assert_only_accepted...` calls from:
    - `e11b_nested_hierarchy_gpu.rs`
    - `e11b_nested_fission_gap.rs`
    - `e11b_nested_materialization.rs`
    - `e11_arena_allocation.rs`
    - `support/e11_nested.rs` (A-0 support)
- Added explicit semantic WGSL rejection test in `c2_atlas_admission_relaxation.rs`
- Updated:
  - `docs/design_v7_8_production_track.md` (A-0 row)
  - `docs/design_v7_8.md` (Line C constraints)
  - This report
  - `docs/worklog.md`
  - `docs/workshop/mapping_current_guidance.md`

No A-0, B-0, C-2, or Resource Flow semantics were changed.

## Why A-0-R1 was insufficient / over-conservative
A-0-R1 correctly centralized the post-C-0 whitelist after the atlas shader was added. However, it retained (and made canonical) a global filename-based "only these exact shaders are allowed" gate. This is the stale v7.x posture that v7.8 doctrine superseded.

## Old guardrail (removed)
Hard-coded `ACCEPTED_WGSL_SHADER_BASELINE` list + `assert_only_accepted_project_wgsl_shaders()` that would fail any new `.wgsl` file in `simthing-gpu/src/shaders/`.

## New guardrail (authoritative)
Semantic WGSL rejection lives at the **designer/spec admission layer** (`simthing-spec`):

- `DesignerAdmissionRequest::SemanticWgsl`
- `DesignerAdmissionRejectionKind::SemanticWgslRequest`
- `DesignerAdmissionDiagnosticCode::SemanticWgslRequestRejected`

This rejects:
- Raw WGSL source from designer inputs
- Semantic/map/faction/AI/economy meaning in shader text generated from ClauseSpec/ClauseThing/RON

Generic GPU-resident kernels in `simthing-gpu` remain allowed when introduced through named production gates (C-0 atlas mask is the precedent).

## Tests removed/renamed
- All `*_no_new_wgsl` tests renamed to `*_rejects_designer_semantic_wgsl` (or equivalent) and now document the designer-layer guard.
- Global filename assertion removed from E-11B and A-0 support.

## Semantic-WGSL rejection tests
Added positive test:
```rust
designer_admission_rejects_raw_wgsl_source
```
(uses the existing `SemanticWgsl` request path).

## Confirmation points
- Generic WGSL files are **not** banned merely by existence or filename.
- A-0, B-0, C-2 admission semantics unchanged.
- `simthing-sim` remains map-free.
- No production posture widened.

## Commands run
```bash
cargo test -p simthing-spec --test c2_atlas_admission_relaxation -- --nocapture
cargo test -p simthing-driver --test phase_e_a0_nested_resource_flow_static -- --nocapture
cargo test -p simthing-driver --test e11b_nested_hierarchy_gpu -- --nocapture
cargo test -p simthing-driver --test e11b_nested_fission_gap -- --nocapture
cargo check --workspace
```
All green.

## Scans run
- Old global "no_new_wgsl" / filename whitelist language removed from active code (only historical in A-0-R1 report).
- Semantic WGSL guard references present and wired in designer_admission.
- No unauthorized opening of parked lines (A-0 still pending, L3 parked, etc.).
- No simthing-sim map awareness introduced.

## Transient cleanup
No authoritative evidence deleted. Only the now-obsolete baseline helper and its direct calls were removed.

## Final verdict

**PASS** — WGSL-GUARD-0 successfully deleted the stale global generic-WGSL filename bans (the `ACCEPTED_WGSL_SHADER_BASELINE` mechanism introduced/centralized in A-0-R1) and ensured the correct v7.8 designer-layer semantic-WGSL guardrail is the active one. 

A-0 remains pending Opus/design-authority review with its historical "added no WGSL" claim preserved as a diff fact rather than an ongoing global gate. C-2 atlas admission and B-0 hard-currency remain unchanged. Map batching stays closed at the designer surface. Production runtime posture is unaffected.

All required tests pass, scans are clean, and the doctrine now matches `design_v7_8.md` §2 and the C-2 acceptance review.