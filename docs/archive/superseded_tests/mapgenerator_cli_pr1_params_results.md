# MapGeneratorCLI PR1 — Lever Params + Shape Registry Shell Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after independent audit; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-14, executive design authority, after independent audit; Cursor PR1)** — CLI crate skeleton, full §3A lever parameter surface, data-driven
shape registry descriptor shell, arbitrary/static mode parameter shell, and validation-only dry-run. **No generation,
placement, topology, scenario emission, lowering, runtime, GPU, simthing-sim, or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR1 establishes the standalone producer crate above the closed 0.0.8.2.5 MapGen ingest/lowering
contract. **0.0.8.2.5 MapGen remains closed and is not reopened.**

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` | CURRENT_EVIDENCE (track plan) | Updated PR1 status |
| `docs/clausething/MapGeneratorCLI.md` | PROPOSAL / reference | Unchanged |
| `mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| New crate | `crates/simthing-mapgenerator/` (`Cargo.toml`, `lib.rs`, `main.rs`, `params.rs`, `shape_registry.rs`) |
| Tests | `crates/simthing-mapgenerator/tests/params.rs`, `shape_registry.rs` |
| Workspace | root `Cargo.toml` (member) |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

No changes to `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`, or MapGen lowering sources.

## Dependency boundary

`simthing-mapgenerator` depends only on `clap`, `serde`, `serde_json`, and `thiserror`.

**Forbidden dependencies absent:** `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec` (verified by
`crate_has_no_forbidden_sim_runtime_deps` test).

## Full lever surface summary

`MapGeneratorParams` groups: `ScaleCoreParams`, `ShapeParams`, `ClusteringParams`, `PartitioningParams`,
`HyperlaneGeometryParams`, `SpecialRouteParams`, `NebulaFieldParams`, `InitializerBucketParams`, `InertMetadataParams`,
`ArbitraryStaticParams`, `OutputParams`, plus `seed` / `variation_seed`.

Validation covers: positive `num_stars`; finite scale/core/lattice; registered shape; registry-declared shape params only;
cluster/partition/hyperlane/special-route/nebula bounds; initializer ref syntax; inert metadata passthrough; procedural vs
arbitrary mode rules.

## Shape registry descriptor summary

`ShapeRegistry` is data-driven (`BTreeMap` of `ShapeStrategyDescriptor`), not a fixed enum of strategies.

PR1 descriptors (algorithms deferred): `arbitrary_static`, `elliptical`, `ring`, `spiral_2`, `spiral_4`.

Unknown shape ⇒ `ValidationError::UnknownShape` listing registered names.

## Two-mode support summary

- **Procedural** — default mode; shape resolved via registry (not `arbitrary_static` in procedural mode).
- **ArbitraryStatic** — carries `explicit_point_cloud_path`, `explicit_graph_path`, `coordinate_transform`,
  `hyperlane_source_mode`; requires at least one path field; no file parsing in PR1.

## Inert metadata passthrough summary

`InertMetadataParams` fields (`num_empires`, fallen/marauder/advanced counts, odds, crisis strengths) are parsed, validated
as bounded numeric carriers, and included in dry-run summary — **never generated or simulated**.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
```

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 21 passed (18 params + 3 shape_registry) |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |

## DA sign-off status

**DA-APPROVED — 2026-06-14, executive design authority.** Independent audit performed (not relying on the
Cursor report): crate manifest verified to depend only on `clap`/`serde`/`serde_json`/`thiserror` (no
`simthing-*`); `lib.rs`/`main.rs`/`params.rs`/`shape_registry.rs` read in full; the shape set confirmed a
data-driven `BTreeMap` registry (C10), not a baked enum; `arbitrary_static` confirmed a registered entry
(C11 seam); inert metadata confirmed validated-and-carried-only (never generated/simulated); `main` confirmed
validation + dry-run only ("generation is not implemented"). Battery rerun locally: `cargo fmt --all --check`
clean, `cargo test -p simthing-mapgenerator` 21 passed (18 params + 3 registry), `mapgen_constitution_guards`
21 passed (closed-contract guards intact), `git diff --check` clean. One remediation applied by the DA before
sign-off: removed a dead no-op `if` block in `validate_mode` (params.rs).

## PR2 may proceed?

**Yes — DA approved PR1 (2026-06-14).** Next rung: deterministic RNG + square lattice occupancy core (`design_0_0_8_6_mapgenerator_cli_ladder.md` PR2).
