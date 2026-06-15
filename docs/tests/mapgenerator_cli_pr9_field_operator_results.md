# MapGeneratorCLI PR9 — Nebula / Field-Operator Declarative Producer Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (MERGED #689 — mechanical rung per ladder §4/§5, no DA sign-off required; DA confirmatory audit clean — see sign-off).

## Verdict

**PASS — MERGED (#689); mechanical rung (ladder §4 row 9 = "Cursor", no DA review required); DA confirmatory audit clean** — bounded producer-side nebula placement and closed-surface `nebula = { name radius }`
feedstock emission; initializer bucket refs emit sibling definitions once; inert metadata passthrough **deferred**
(dry-run report only). Generated output parses and lowers through existing closed `mapgen_lattice` +
Movement-Front `RegionFieldSpec` surfaces without front-end widening. **Zero** `crates/simthing-clausething/src/`
changes. No route/path/predecessor/movement/border/frontline semantics, runtime/GPU execution, semantic WGSL,
simthing-sim, new `SimThingKind`, Euclidean authority in output, or FIELD-MOVIE-DATASET-0 work.

**PR9 proof scope (unchanged):** declarative nebula feedstock parses and lowers through closed lattice +
Movement-Front RegionFieldSpec surfaces only. PR9 did **not** execute GPU or runtime field stepping, did **not**
emit scenario-container `field_operator` blocks, and did **not** widen the lowerer.

**PR10 proceeded** under MapGeneratorCLI PR10 (generated scenario admit/install + GPU compact evidence).

## Track scope

0.0.8.6 MapGeneratorCLI PR9 adds declarative nebula field feedstock only. The closed neutral-AST surface is
`nebula = { name = "..." radius = N }` inside `static_galaxy_scenario` (not scenario-container `field_operator`
blocks). Movement-Front lowering (`generate_mapgen_movement_front_authoring`) produces the admitted
`RegionFieldSpec` / SaturatingFlux operator from existing PR3–PR6 surfaces — no new grammar.

**PR10 next:** generated scenario admit/install + GPU compact evidence on a real adapter (only if this
declarative field path remains fully closed-surface).

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr1_params_results.md` through `pr8` | CURRENT_EVIDENCE | Unchanged — preserved |
| `docs/tests/mapgenerator_cli_pr9_field_operator_results.md` | CURRENT_EVIDENCE | Promoted after DA approval (#689) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Nebula placement | `crates/simthing-mapgenerator/src/nebula.rs` |
| Closed-surface emission | `crates/simthing-mapgenerator/src/field_operator.rs` |
| Metadata deferral report | `crates/simthing-mapgenerator/src/metadata.rs` |
| Emitter wiring | `crates/simthing-mapgenerator/src/emitter.rs`, `src/lib.rs` |
| Crate tests | `tests/nebula.rs`, `tests/field_operator.rs`, `tests/metadata.rs` |
| Integration lowering proof | `crates/simthing-clausething/tests/mapgenerator_cli_pr9_field_operator_lower.rs` |
| Prior integration tests (num_nebulas=0 guard) | `mapgenerator_cli_pr{5,6}*_lower.rs`, `partition_bridge_lower.rs`, `special_routes_lower.rs` |
| Docs | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md`, `docs/design_0_0_8_1_clausething_production_track.md`, `docs/clausething/MapGeneratorCLI.md` |

## Nebula model summary

- Params: `num_nebulas`, `nebula_size`, `nebula_min_dist` (validated in PR1).
- Deterministic Chebyshev-lattice center placement from placed-system coords + lattice center.
- Min-distance between nebula centers enforced with integer Chebyshev distance (no Euclidean authority).
- Fail-closed `NebulaError::ImpossibleRequest` when count/min-dist cannot be satisfied.
- Affected systems tracked producer-side only (integer radius ball over lattice cells).
- Emitted as `nebula = { name = "generated_nebula_N" radius = R }` scalar radius (integer cells).

## Field-operator emission summary

Closed MapGen neutral-AST surface only — keys `name` and `radius`. No `field_operator` keyword, no route/path/
predecessor/movement/border/frontline terms, no semantic WGSL. Lowering proof uses existing Movement-Front path to
`RegionFieldSpec::SaturatingFlux` after PR3 lattice + PR4 RF + PR5 links enrollment.

## Initializer bucket summary

- Strategies continue assigning `PlacedSystemSeed.bucket` from `initializer_bucket_{core,arm,fringe}` params.
- Cluster assignment may override bucket to `initializer_bucket_cluster` for non-anchor clusters.
- Emitter resolves bareword initializer refs and emits sibling initializer definitions once per unique ref.
- Shared refs remain collision-free via existing #680 lowerer amendment.

## Inert metadata passthrough summary

**Deferred.** Closed `static_galaxy_scenario` reader admits `name`, `random_hyperlanes`, `system`, `add_hyperlane`,
and `nebula` only. `metadata_passthrough_report()` captures `InertMetadataParams` for producer dry-run reports;
values are not emitted into scenario text.

## Parse / lattice / field-operator lowering proof

`mapgenerator_cli_pr9_field_operator_lower.rs`:

1. MapGeneratorCLI static placement + hyperlanes + nebula emission
2. `parse_mapgen_neutral_document`
3. `generate_mapgen_lattice_hierarchy`
4. `generate_mapgen_resource_flow_enrollment` → `generate_mapgen_links`
5. `generate_mapgen_movement_front_authoring` → `RegionFieldSpec::SaturatingFlux` admitted

## Closed-source gate result

**PASS** — `git diff --name-only master...HEAD` excludes all forbidden closed `src/` paths.

## Forbidden semantics scan

No route/path/predecessor/movement/border/frontline/`field_operator`/GPU/WGSL/runtime surfaces in emitted PR9
scenario text (nebula blocks only).

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr9_field_operator_lower
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results

| Suite | Result |
|---|---|
| `cargo test -p simthing-mapgenerator` | PASS |
| `mapgenerator_cli_pr9_field_operator_lower` | 6 passed |
| `mapgen_neutral_ast_parse` | PASS |
| `mapgen_lattice_hierarchy` | PASS |
| `mapgen_constitution_guards` | PASS |

## DA sign-off status

**MERGED — MECHANICAL RUNG, NO DA SIGN-OFF REQUIRED (#689).** *Correction (2026-06-15):* PR9 is designated a
**mechanical rung** in the ladder — §4 PR table row 9 is owner **"Cursor"** (not "Cursor (DA review)"), and §5
states verbatim *"PR9 is mechanical under §3."* So merging PR9 without a DA sign-off was **correct per the ladder**,
not a governance lapse. An earlier revision of this report wrongly framed it as an "owner-merge-before-review breach"
and stamped it "DA-RATIFIED RETROACTIVELY"; that framing is **withdrawn**. The only accurate nit: a mechanical rung
should not carry a "DA-APPROVED" label at all (mechanical rungs receive no DA sign-off) — hence this "merged,
mechanical" status instead.

**DA confirmatory audit (optional, not a required gate): clean.** While reviewing the DA-sensitive PR10 (which
depends on PR9), the DA spot-checked PR9 and found it sound: `field_operator.rs` emits **only** the closed-surface
`nebula = { name radius }` block that `mapgen_lattice` already accepts — it does **not** invent a
`field_operator`/`gpu_operator` grammar block (it ships `forbidden_field_surface_term()` to scan emitted text);
`nebula.rs`/`metadata.rs` are producer-side placement + inert reporting; inert metadata is report-only. **Zero
closed `src/`; no `simthing-*` dep.** Battery (run as part of PR10 review): `cargo fmt --check` clean;
`cargo test -p simthing-mapgenerator` zero failures; `mapgenerator_cli_pr9_field_operator_lower` 6 passed;
`mapgen_constitution_guards` 21, `mapgen_resource_flow` 16, `mapgen_links` 19 green. **Governance rule (accurate
form):** the ladder's per-rung *DA-sensitive?* column governs — DA-review-marked rungs await DA review before merge;
**mechanical rungs (like PR9) are correctly Cursor-merged.**
