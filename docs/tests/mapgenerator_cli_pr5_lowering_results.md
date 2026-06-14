# MapGeneratorCLI PR5 — Generated static_galaxy_scenario Lowering Proof Results

> **Artifact lifecycle: PROBATION** (pending DA review — do not treat as CURRENT_EVIDENCE until DA approves).

## Verdict

**PASS pending DA review** — PR4 `static_galaxy_scenario` output parses through the existing neutral-AST
parser and lowers through the **amended** closed `generate_mapgen_lattice_hierarchy` path without front-end
widening. **This cleaned PR5 contains zero closed `crates/simthing-clausething/src/` changes.**
**No topology, links, field operators, RF, Movement-Front, PALMA, driver/GPU, or FIELD-MOVIE-DATASET-0 work.**

## Track scope

0.0.8.6 MapGeneratorCLI PR5 proves the producer→closed-front-end seam above DA-approved PR4 emission.
**0.0.8.2.5 MapGen remains closed and is not reopened.** PR5 previously exposed a latent lowerer child-id
collision bug; the fix was split into the separate 0.0.8.2.5 amendment
([`mapgen_lowerer_child_id_amendment_results.md`](mapgen_lowerer_child_id_amendment_results.md), PR #680).
PR5 rebases on that amendment and proves lowering only — it does not carry the lowerer source edit.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr4_emitter_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_lowerer_child_id_amendment_results.md` | PROBATION | Prerequisite amendment (Part A, #680) |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| `mapgenerator_cli_pr5_lowering_results.md` | PROBATION | Updated for decoupled remediation |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified in PR5 |

No MapGen baseline artifacts deleted or archived.

## Files changed (cleaned PR5 only)

| Area | Path |
|---|---|
| Lowering proof test | `crates/simthing-clausething/tests/mapgenerator_cli_pr5_generated_static_lowers.rs` |
| Dev dependency | `crates/simthing-clausething/Cargo.toml` (`simthing-mapgenerator` under `[dev-dependencies]` only) |
| Emitter test update | `crates/simthing-mapgenerator/tests/emitter.rs` — per-system initializer assertion strengthened |
| Ladder | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md` (gate + PR5 status; amended in Part A base) |
| Production track | `docs/design_0_0_8_1_clausething_production_track.md` |

**Not in PR5:** `crates/simthing-clausething/src/mapgen_lattice.rs` or any other closed lowerer source edit
(those belong to Part A amendment PR #680).

No changes to `hydrate_scenario`, `simthing-sim`, `simthing-gpu`, `simthing-driver`, or MapGen baseline fixtures.

## Decoupled remediation (2026-06-14)

Original PR5 (#679) **DEFERRED** by DA for conflating producer-track proof with a closed lowerer fix.

| Issue | Resolution |
|---|---|
| Emitter fidelity regression (dedup dropped per-system `initializer =`) | **Remediated on master/PR4 path** — every system block emits its initializer bareword; sibling definition blocks remain deduped |
| Dependency hygiene | `simthing-mapgenerator` listed only under `[dev-dependencies]`; test `mapgenerator_is_dev_dependency_only_in_clausething` guards this |
| Latent lowerer child-id collision on shared initializers | **Split to Part A** — 0.0.8.2.5 amendment scopes child ids to `{system.id}_{initializer.id}_planet/_deposit` (PR #680) |
| Producer PR carrying closed `src/` changes | **Removed from PR5** — constitutional gate: closed defect ⇒ stop, split, amend, resume producer PR |

## Closed-src gate (PR5 content vs Part A base)

```text
git diff --name-only mapgen-lowerer-child-id-amendment...HEAD
```

Result (2026-06-14): **no** paths under `crates/simthing-clausething/src/`, `simthing-sim`, `simthing-gpu`,
`simthing-driver`, or `simthing-spec`.

## Generated sample summary

- **Static path:** 4 explicit lattice cells via `static` strategy → `generated_static` root block.
- **Elliptical path:** 5 systems via elliptical strategy → `generated_elliptical` root block.
- Text generated in-memory inside tests; no committed scenario fixture files.

## Parser path summary

`place_and_emit_scenario` → `parse_mapgen_neutral_document` (existing PR2 neutral-AST parser).

## Lattice lowering path summary

`generate_mapgen_lattice_hierarchy` with `fixture_lattice_edge = 3` on the **amended** closed lowerer (Part A
base). No `mapgen_links` required for PR5.

Assertions:
- gridcell `Location` count matches placement count
- system ids `"0".."N-1"` preserved
- inert render `x`/`y`/`z=0` metadata matches source lattice integers
- every system block emits `initializer =` bareword; one sibling definition block
- shared initializer resolves on every system; lowered child node IDs are unique (amended lowerer)
- empty `grid_metadata.links`
- no RF/PALMA/commitment surfaces on lowered pack

## Front-end no-widening summary

**Zero** changes to `crates/simthing-clausething/src/` in this PR. No changes to `hydrate_scenario` accepted
fields or grammar. Proof passes against the amended closed lowerer without producer-specific forgiveness.

## Initializer resolution summary

Every generated system carries `initializer = example_rim_initializer` when sharing the default/core bucket.
One sibling `example_rim_initializer = { planet = { count = 1 } }` block is emitted. Lowering produces a
`Cohort` planet payload child on **each** system with system-scoped child ids from the Part A amendment.

## No topology / no GPU boundary summary

No `add_hyperlane`, links, field operators, RF, Movement-Front, PALMA, driver install, or GPU session
calls. `mapgen_links` not exercised in PR5 (links remain PR6).

## Dependency boundary

- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only.
- `simthing-mapgenerator` does **not** depend on `simthing-clausething` or other forbidden crates.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr5_generated_static_lowers
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only mapgen-lowerer-child-id-amendment...HEAD
```

## Test results (2026-06-14 local validation — cleaned PR5)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | pass |
| `cargo test -p simthing-clausething --test mapgenerator_cli_pr5_generated_static_lowers` | 18 passed |
| `cargo test -p simthing-clausething --test mapgen_neutral_ast_parse` | pass |
| `cargo test -p simthing-clausething --test mapgen_lattice_hierarchy` | pass |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | pass |
| `git diff --check` | pass |
| closed-src gate vs Part A base | pass — no forbidden paths |

## DA sign-off status

**Pending DA review.** Only the Design Authority writes DA sign-off. This report does not pre-file approval.
Requires Part A amendment (#680) DA approval and merge before PR5 merge.

## Whether PR6 may proceed

**No — await DA approval of Part A (#680) and cleaned PR5 (#679).** After both merge, PR6 = bounded hyperlane
topology / `add_hyperlane` emission — still no route/path/predecessor semantics and no GPU.

## Carried-forward DA notes (not addressed in PR5)

1. PR2: `OccupancyGrid::insert_relocated` O(cells) rebuild per insertion — revisit before scale-envelope rung.
2. PR2: `SquareLattice::cell_count` u32 edge² overflow — revisit before scale-envelope rung.
3. PR3: `strategy_by_name` / `executable_strategy_names` should be single-sourced before PR8.
4. PR3: procedural validation rejects `arbitrary_static` but not `static`; consider unifying the mode gate.
