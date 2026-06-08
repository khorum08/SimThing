# C-2 — Atlas Admission Relaxation for Bounded Algebraic-G=0 Specs Results

## Base HEAD
Post C-ACCEPT-0 (Opus acceptance of C-0/C-1 and opening of C-2).

## Files changed
- `crates/simthing-spec/src/designer_admission/atlas.rs` (new)
- `crates/simthing-spec/src/designer_admission/mod.rs`
- `crates/simthing-spec/src/designer_admission/diagnostic.rs`
- `crates/simthing-spec/src/lib.rs`
- `crates/simthing-spec/tests/c2_atlas_admission_relaxation.rs` (new)
- `docs/tests/phase_m_c2_atlas_admission_relaxation_results.md` (this report)
- `docs/design_v7_8_production_track.md`
- `docs/design_v7_8.md`
- `docs/workshop/mapping_current_guidance.md`
- `docs/worklog.md`

**No production mapping runtime, no default SimSession wiring, no default-on atlas, no sparse-residency scheduler, no active mask/source identity, no physical gutter as C-2 admitted path, no A-0/B-0, no L3, no FrontierV2-5, no semantic WGSL, no simthing-sim map awareness.**

## C-ACCEPT-0 summary
Opus 4.8 + product accepted C-0 (real packed-atlas G=0 path with full-tile oracle) and C-1 (2000-star budget model). Opened C-2 as the narrow designer/spec admission relaxation for bounded algebraic-G=0 atlas specs only.

## C-2 scope (from acceptance review)
Designer/spec layer only. Admit bounded algebraic-G=0, homogeneous-square, protocol-oracle-backed specs that fit the active `V78AtlasVramBudget` with mandatory multiplier reporting. `request_atlas_batching` relaxed only through this scope. Production runtime/scheduler is a separate later gate.

## Accepted atlas spec shape (C-2)
See `AtlasAdmissionSpec`:
- `request_atlas_batching = true`
- `isolation = AlgebraicTileLocalMaskG0`
- `homogeneous_square_tiles = true`
- `protocol_oracle_backed = true`
- `active_vram_budget` fits (using C-1 style 128-byte effective accounting)
- `multiplier_reporting_required = true`
- All production/default/active-mask/source/physical-gutter flags explicitly false

## Rejection matrix
- Non-square / heterogeneous → rejected
- Wrong isolation (including physical gutter as accepted path) → rejected
- Missing protocol oracle → rejected
- Over active budget → rejected
- Missing multiplier reporting → rejected
- Any production/default/active-mask/source-identity flags → rejected

## Typical huge commodity profile (modeled)
- ~128×128 map, 1000 stars, 5×5 surfaces
- Algebraic G=0 ≈ 0.163 GiB (fits 1.5 GiB default easily)

## Horizon dedicated-server stress profile (from C-1)
- 200×150 + 2000×10×10 + ... = 7,230,000 cells
- Algebraic G=0 ≈ 0.862 GiB (fits default)
- Gutter ≈ 5.826 GiB (requires raised budget)

## Active budget checks
All admissions check against the *active* `V78AtlasVramBudget.max_bytes`. The budget is configurable with no architectural hard cap.

## Diagnostic codes (C-2 additions)
- `AtlasSpecNotHomogeneousSquareRejected`
- `AtlasSpecUnsupportedIsolationRejected`
- `AtlasSpecMissingProtocolOracleRejected`
- `AtlasSpecOverActiveBudgetRejected`
- `AtlasSpecMissingMultiplierReportingRejected`
- `AtlasSpecPhysicalGutterRequiresRaisedGateRejected`
- `AtlasProductionRuntimeRejected`

## Tests run
All required tests pass (see `c2_atlas_admission_relaxation.rs`).

## Scans run
All required rg scans performed. No unauthorized production code, no simthing-sim map awareness, no opening of parked lines.

## Transient cleanup
No authoritative evidence deleted.

## Final verdict
**PASS** — C-2 implemented bounded atlas admission relaxation at the designer/spec layer for algebraic-G=0 homogeneous-square, protocol-oracle-backed specs that fit the active budget with multiplier reporting. Both commodity huge and horizon stress profiles modeled. All other capabilities remain rejected or deferred as required. Ready for Opus review.

C-2 is the designer-facing closure of map batching (C-0 proof + C-1 model + C-2 admission). Production runtime remains a separate later gate.