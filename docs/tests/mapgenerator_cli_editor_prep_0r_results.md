# MAPGENCLI-EDITOR-PREP-0R — report-quality gates + healthy editor-prep sample

**Classification: PROBATION until DA approval**

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Guardrail row + PROBATION amendment for 0R |
| `docs/tests/mapgenerator_cli_editor_prep_0_results.md` | PROBATION | 0 pass; sample stats superseded in-place by 0R |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.png` | PROBATION | **Regenerated in-place** (same path; healthy dense spiral) |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json` | PROBATION | **Regenerated in-place** with quality gates |
| `docs/tests/mapgenerator_cli_editor_prep_0r_results.md` | PROBATION | This report |

## Old bad sample stats (#723)

The original editor-prep sample exposed a silent topology clamp bug:

| Field | Bad value (#723) | Root cause |
|---|---|---|
| `requested_target_hyperlanes` | 6000 | `--num-hyperlanes 6000` |
| `actual_topology_hyperlanes` | 3 | `num_hyperlanes_max` default **3** clamped effective target |
| `connectivity_bridge_count` | 2996 | Connectivity repair spanning tree dominated the map |
| `topology_target_ratio` | ~0.0005 | Would FAIL quality gate |
| `connectivity_bridge_ratio` | ~0.999 | Would FAIL quality gate |
| `average_degree` | 2.0 | WARN on dense preview |

## New sample command

```text
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_2 \
  --stars 3000 \
  --lattice-edge 300 \
  --seed 42 \
  --num-hyperlanes 6000 \
  --max-hyperlane-distance 8 \
  --shape-param arm_width=14 \
  --shape-param arm_tightness=0.6 \
  --shape-param jitter=2 \
  --no-partitions \
  --cluster-count 4 \
  --cluster-radius 500 \
  --hyperlanes base \
  --hyperlane-color blue \
  --draw-core \
  --png-size 3000 \
  --render-png docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.png \
  --report-json docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json
```

Fixes applied:

1. CLI raises `num_hyperlanes_max` to match `--num-hyperlanes` when higher (prevents silent clamp).
2. Explicit `--max-hyperlane-distance 8` for dense 300×300 spiral local adjacency window.

## New sample report stats

From `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json`:

| Field | Value |
|---|---|
| `actual_topology_hyperlanes` | 5917 |
| `connectivity_bridge_count` | 2 |
| `actual_base_hyperlanes` | 5919 |
| `topology_target_ratio` | 0.9862 |
| `topology_target_deficit` | 83 |
| `connectivity_bridge_ratio` | 0.0003 |
| `average_degree` | 3.95 |
| `component_count` | 1 |
| `isolated_system_count` | 0 |
| `map_quality_status` | PASS |

## Quality gate definitions

Implemented in `crates/simthing-mapgenerator/src/report.rs`:

| Status | Condition |
|---|---|
| **FAIL** | `system_count != star_count` |
| **FAIL** | `duplicate_cell_count > 0` |
| **FAIL** | `ensure_connected` and `component_count != 1` |
| **FAIL** | `ensure_connected` and `isolated_system_count > 0` |
| **FAIL** | `topology_target_ratio < 0.50` |
| **FAIL** | `connectivity_bridge_ratio > 0.50` |
| **WARN** | `connectivity_bridge_ratio > 0.25` |
| **WARN** | dense preview (`stars` or target ≥ 1000) and `average_degree < 2.5` |
| **WARN** | `longest_bridge_chebyshev > 32` |
| **WARN** | requested target clamped by `num_hyperlanes_max/min` |

CLI: always writes report; prints WARN/FAIL to stderr; `--fail-on-quality-warn` exits non-zero.

## Shape-param cleanup

- Removed `coordinate_transform` from numeric `ShapeParamSpec` table (static shapes).
- `coordinate_transform` rejected via `NonNumericShapeParam` when passed as `--shape-param`.
- NaN/inf rejected at parse time (`shape_param_rejects_nan`, `shape_param_rejects_inf`).

## Files changed

- `crates/simthing-mapgenerator/src/report.rs` — quality fields + gate evaluation
- `crates/simthing-mapgenerator/src/lib.rs` — pre-connectivity topology count on result
- `crates/simthing-mapgenerator/src/main.rs` — max auto-raise, quality stderr, `--fail-on-quality-warn`
- `crates/simthing-mapgenerator/src/shape_param_spec.rs` — non-numeric param rejection
- `crates/simthing-mapgenerator/src/params.rs` — `NonNumericShapeParam` error
- `crates/simthing-mapgenerator/tests/editor_prep.rs` — 9 new tests (25 total)
- `docs/clausething/MapGeneratorCLI.md` — quality gates + fixed sample command
- `docs/tests/current_evidence_index.md` — guardrail + PROBATION row
- `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.{png,report.json}` — regenerated in-place

## Commands run

```text
cargo fmt --all
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo run -p simthing-mapgenerator --bin mapgen -- (regeneration command above)
git diff --check
```

## Tests run

All `simthing-mapgenerator` tests (including `editor_prep.rs` ×25). STEAD spatial contract guards (11 green).

## DA status

**PROBATION** — pending owner/design-authority approval. Do not pre-file DA approval.

## Known caveats

- `topology_target_satisfied` compares against **effective** clamped target, not raw request alone.
- Topology may fall slightly short of requested target (5917 vs 6000) due to fanout/candidate caps; ratio gate catches severe deficits.
- `longest_bridge_chebyshev` WARN threshold (32) is a documented producer default, not a STEAD contract bound.
