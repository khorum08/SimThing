# MapGeneratorCLI PR10 — Generated Scenario Admit/Install + GPU Compact Evidence Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-15 after independent branch-source audit + real-adapter GPU battery rerun; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-15, executive design authority)** — a tiny deterministic MapGeneratorCLI-generated `static_galaxy_scenario` parses through
`parse_mapgen_neutral_document`, lowers through closed MapGen lattice / RF / links / Movement-Front surfaces,
admits/installs via `install_atomic` + compile previews, and produces GPU-resident compact evidence on a real
adapter using the existing generic W/PALMA + compact D-probe harness (mirrors 0.0.8.2.5 MapGen PR10). **Zero**
closed `src/` edits (`simthing-clausething`, `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`).
No new grammar, no lowerer widening, no new `SimThingKind`, no semantic WGSL, no new GPU kernels, no
route/path/predecessor/movement planner, no full-field readback, no Euclidean authority, and no
FIELD-MOVIE-DATASET-0 work.

## PR9 evidence-status correction (folded)

PR9 report promoted to **CURRENT_EVIDENCE** / **DA-APPROVED & MERGED (#689)** in this PR. PR9 scope remains:
declarative nebula feedstock parse/lowering only — no GPU/runtime execution, no scenario-container
`field_operator` blocks, no lowerer widening.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr1_params_results.md` through `pr8` | CURRENT_EVIDENCE | Unchanged — preserved |
| `docs/tests/mapgenerator_cli_pr9_field_operator_results.md` | CURRENT_EVIDENCE | Promoted (#689) in Part A |
| `docs/tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md` | PROBATION | New (this report) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgen_lowerer_child_id_amendment_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| PR10 integration harness | `crates/simthing-clausething/tests/mapgenerator_cli_pr10_gpu_compact_evidence.rs` |
| Test dev-deps (GPU buffer setup) | `crates/simthing-clausething/Cargo.toml` (`wgpu`, `bytemuck` dev-only) |
| PR9 evidence promotion | `docs/tests/mapgenerator_cli_pr9_field_operator_results.md` |
| PR10 report | `docs/tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md` |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |
| MapGeneratorCLI reference | `docs/clausething/MapGeneratorCLI.md` |

## Generated scenario feature summary

Primary admit/install + GPU fixture (`generated_fixture`):

- **Shape:** `static` / `ArbitraryStatic` with explicit 5-cell pentad-style layout (seed `10100`)
- **Systems:** 5 (fits existing RF slot cap without closed lowerer edits)
- **Hyperlanes:** bounded PR6 producer edges → `add_hyperlane` declarations
- **Partition/cluster:** PR7 bridge couplings (1 home + 1 open partition, 2 clusters)
- **Nebula:** 1 closed `nebula = { name radius }` block (PR9 feedstock)
- **Initializer buckets:** core/arm/fringe/cluster bareword refs with sibling definitions once
- **Special routes:** exercised in extended 9-system layout test (wormhole pair → `add_hyperlane`; parse/links
  lowering only — larger layouts exceed RF slot cap for full install without closed edits)
- **Metadata:** not emitted (deferred dry-run only, per PR9)

Deposit slot injection (`inject_deposit_for_rf`) matches pentad RF enrollment pattern so RF arenas admit within
caps (`deposit_max_participants` / `suppression_max_participants` = 24 in harness).

## Parse / lattice / RF / link / movement-front lowering proof

Harness tests:

| Test | Proof |
|---|---|
| `generated_pr10_scenario_parses` | `parse_mapgen_neutral_document` Ok |
| `generated_pr10_scenario_lowers_lattice` | `generate_mapgen_lattice_hierarchy` — 5 systems |
| `generated_pr10_scenario_lowers_resource_flow_enrollment` | RF arenas enrolled |
| `generated_pr10_scenario_lowers_links_and_lane_couplings` | Links + lane couplings |
| `generated_pr10_scenario_lowers_movement_front_region_field` | `RegionFieldSpec::SaturatingFlux`, horizon bounded |
| `generated_pr10_extended_layout_lowers_special_route_as_add_hyperlane` | 9-system wormhole pair → ≥2 hyperlanes |

## Admission / install proof

`generated_pr10_scenario_admits_installs`:

- Full authoring pack: region field, RF, PALMA, commitment, W compose
- `route_surface_count` = 0, `predecessor_surface_count` = 0
- `compile_region_field_preview` + `compile_w_impedance_compose_preview` Ok
- `install_atomic` Ok on generated pack
- `SimSession::open_from_spec` Ok when GPU adapter present; mapping default-off preserved

No admission bypass; no closed lowerer edits.

## GPU compact evidence proof

`generated_pr10_gpu_compact_evidence_real_adapter`:

- Real adapter required (`GpuContext::new_blocking().expect("PR10 PASS requires GPU adapter")`)
- `FirstSliceMappingSession` scheduled tick: `field_values`, `reduction_parent_value`, `eml_output` all `is_none()`
- Compact diagnostic readback (`diagnostic_readback_reduction_eml`) — finite threat/urgency; urgency crosses threshold
- Scheduled W/PALMA chain + compact D probe (`MinPlusTraversalDProbeOp`) — bounded probe cells (≤ 4 cap)
- `traversal_report.values.is_none()` — no full-field readback

`generated_pr10_uses_compact_readback_only` — static harness assertions on compact readback guards.

## Adapter identity / availability

- **Adapter present on validation machine:** yes (wgpu real adapter; GPU tests use `expect`, not skip)
- **GPU actually ran for PASS tests:** yes — `generated_pr10_gpu_compact_evidence_real_adapter` passed
- **GPU skip treated as PASS:** no — `pr10_pass_requires_gpu_adapter` asserts harness wording

## Compact readback scope

Compact only: mapping tick omits full field values; traversal D probe returns bounded cell rows; no CPU planner
over field contents; no semantic WGSL; no new kernels.

## Closed-source / source-change gate result

**PASS** — `git diff --name-only master...HEAD` excludes all forbidden closed `src/` paths
(`simthing-clausething/src`, `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`).

Allowed changes: integration test, clausething `Cargo.toml` dev-deps, docs only.

## Forbidden semantics scan

`generated_pr10_has_no_forbidden_semantic_terms`:

- No `field_operator`, route/path/predecessor/movement/border/frontline terms in emitted text
- `forbidden_field_surface_term` clean
- Serialized `game_mode` JSON excludes forbidden planner/graph tokens

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator`, `simthing-driver`, `simthing-gpu`, `simthing-spec`
  for integration/GPU harness only
- `simthing-mapgenerator` does **not** depend on forbidden runtime crates (`producer_still_has_no_forbidden_runtime_deps`)

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr10_gpu_compact_evidence
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results

| Suite | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS (see validation run) |
| `cargo test -p simthing-mapgenerator` | PASS (see validation run) |
| `mapgenerator_cli_pr10_gpu_compact_evidence` | 12 passed |
| `mapgen_neutral_ast_parse` | PASS (see validation run) |
| `mapgen_lattice_hierarchy` | PASS (see validation run) |
| `mapgen_links` | PASS (see validation run) |
| `mapgen_resource_flow` | PASS (see validation run) |
| `mapgen_constitution_guards` | PASS (see validation run) |
| `git diff --check` | PASS (see validation run) |

## DA sign-off status

**DA-APPROVED — 2026-06-15, executive design authority.** Independent branch-source audit: the harness mirrors the
closed `mapgen_pr10_end_to_end_compact_evidence` pattern but feeds **MapGeneratorCLI-generated** scenarios through
`parse_mapgen_neutral_document` → lattice/RF/links/Movement-Front lowering → `install_atomic` +
`SimSession::open_from_spec` → **GPU compact evidence on a real adapter**. Compact-readback-only is enforced by a
self-test asserting the harness contains `field_values.is_none()` / `reduction_parent_value.is_none()` /
`eml_output.is_none()` / `traversal_report.values.is_none()` (no full-field readback); `pr10_pass_requires_gpu_adapter`
gates on a real adapter (default-off). **Zero `crates/simthing-clausething/src/` changes** (test/docs/Cargo dev-dep
only); no new kernel/WGSL; producer carries no `simthing-*` dep. Battery rerun on the branch: `cargo fmt --check`
clean; `cargo test -p simthing-mapgenerator` zero failures; **`mapgenerator_cli_pr10_gpu_compact_evidence` 12 passed
including `generated_pr10_gpu_compact_evidence_real_adapter` (1.13s — real GPU executed, not skipped)**; PR9 integration
6; `mapgen_constitution_guards` 21; `mapgen_resource_flow` 16; `mapgen_links` 19.

**Honest scope accepted (correct §0 discipline):** the primary admit/install + GPU fixture is a 5-system pentad-style
static layout (seed 10100) because larger generated packs hit `ResourceFlowSlotOverflow` under the existing RF slot
cap — and widening that cap would require a **closed lowerer edit, which §0 forbids in a producer PR.** Cursor
correctly STOPPED at the gate and used a small fixture rather than touching closed `src/` (special routes covered
separately in the 9-system extended layout via parse + hyperlane extraction). This mirrors the closed 0.0.8.2.5 PR10,
which also proved compact evidence on a tiny pentad; the 1000-star scale is explicitly **PR11's** job. *(If PR11 needs
the RF slot cap raised to install a large generated pack, that is a DA-authorized 0.0.8.2.5 amendment — never a
producer-PR edit.)*

*Correction re PR9 (#689):* PR9 is a **mechanical rung** (ladder §4 row 9 = owner "Cursor"; §5 "PR9 is mechanical
under §3"), so merging it without a DA sign-off was **correct per the ladder** — an earlier note here wrongly
called it an "owner-merge breach." The DA spot-checked PR9 as a courtesy (clean) because PR10 depends on it; see
`mapgenerator_cli_pr9_field_operator_results.md`. **Governance rule (accurate):** the ladder's per-rung
*DA-sensitive?* column governs — DA-review-marked rungs (like this PR10) await DA review; mechanical rungs are
Cursor-merged.

## Whether PR11 1000-star scale envelope may proceed

**Yes — DA approved PR10 (2026-06-15).** PR11 = scale-envelope proof / 1000-star generated map stress:
bound O(cells), u32 edge² overflow, O(N²) topology/bridge enumeration, and artifact lifecycle closeout.
