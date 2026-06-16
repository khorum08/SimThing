# MAPGENCLI-EDITOR-PREP-0 — fail-closed shape params + JSON generation report

**Classification: PROBATION until DA approval**

## Artifact lifecycle audit

| Artifact | Classification | Notes |
|---|---|---|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER | Updated with editor-prep guardrail + PROBATION row |
| Existing PNG visual artifacts | CURRENT_EVIDENCE | Unchanged (DA-approved in ledger) |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.png` | PROBATION | Sample render for editor-prep command verification |
| `docs/tests/mapgenerator_cli_spiral2_dense_3000_editor_prep.report.json` | PROBATION | Sample `mapgenerator.report.v1` JSON |
| `docs/tests/mapgenerator_cli_editor_prep_0_results.md` | PROBATION | This report |

## Shape-param validation behavior

Fail-closed at CLI parse time and at `MapGeneratorParams::validate`:

| Input | Result |
|---|---|
| `--shape-param arm_width=14` (spiral_2) | PASS |
| `--shape-param arm_tightness=0.6` | PASS |
| `--shape-param jitter=2` | PASS |
| `--shape-param arm_width` (no `=`) | FAIL — `Invalid shape param 'arm_width': expected KEY=VALUE with numeric VALUE` |
| `--shape-param arm_width=` | FAIL — missing numeric value |
| `--shape-param arm_width=abc` | FAIL — non-numeric value |
| `--shape-param unknown_param=1` | FAIL — `Unknown shape param 'unknown_param' for shape …` |
| `--shape-param arm_width=14=bad` | FAIL — invalid format |
| `arm_width=14` on `elliptical` | FAIL — `Shape param 'arm_width' is not valid for shape elliptical` |

Implementation: `shape_param_spec.rs` (`ShapeParamSpec`, `parse_shape_param_assignment`, `apply_cli_shape_params`, `validate_shape_params`).

## JSON report schema summary

- **schema_version:** `mapgenerator.report.v1`
- **generator:** crate name, optional version, track, seed
- **request:** shape, star/lattice counts, hyperlane target, connectivity flags, shape_params map
- **output:** system/hyperlane counts, coupling counts, connectivity stats, degree stats, bounding box, occupancy/duplicate cell counts
- **artifacts:** scenario/png/report paths (nullable)
- **constitution:** structural-coordinate authority flags (render non-authoritative; no sqrt/pathfinding/runtime)

Library API: `build_generation_report`, `write_generation_report_json`, `normalized_report_json` (tests).

## Regeneration command

```text
cargo run -p simthing-mapgenerator --bin mapgen -- \
  --shape spiral_2 \
  --stars 3000 \
  --lattice-edge 300 \
  --seed 42 \
  --num-hyperlanes 6000 \
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

Connectivity is ON by default (`ensure_connected`); `--connect-galaxy` is redundant. Use `--allow-disconnected` to opt out.

## Files changed

- `crates/simthing-mapgenerator/src/shape_param_spec.rs` — fail-closed param parsing + bounds
- `crates/simthing-mapgenerator/src/report.rs` — JSON generation report
- `crates/simthing-mapgenerator/src/main.rs` — `--report-json`, strict `--shape-param`
- `crates/simthing-mapgenerator/src/params.rs` — validation error messages + delegate to spec validator
- `crates/simthing-mapgenerator/src/lib.rs` — exports
- `crates/simthing-mapgenerator/tests/editor_prep.rs` — 16 focused tests
- `docs/clausething/MapGeneratorCLI.md` — editor-facing producer contract section
- `docs/tests/current_evidence_index.md` — guardrail + PROBATION row

## Commands run

```text
cargo fmt --all
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test stead_spatial_contract_guards
cargo run -p simthing-mapgenerator --bin mapgen -- (regeneration command above)
git diff --check
```

## Tests run

All `simthing-mapgenerator` tests (including `editor_prep.rs` ×16). STEAD spatial contract guards (11 green).

## DA status

**PROBATION** — pending owner/design-authority approval. Do not pre-file DA approval.

## Known caveats

- `ShapeParamSpec` numeric bounds are minimal (not exhaustive per-shape tuning docs); richer `--help` listing deferred.
- `topology_hyperlane_count` is derived as `base_hyperlane_count - bridge_count` when connectivity ran.
- Sample JSON/PNG paths use `_editor_prep` suffix to avoid conflicting with DA-approved dense spiral artifacts.
