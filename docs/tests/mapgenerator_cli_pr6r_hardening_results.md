# MapGeneratorCLI PR6R — PR6 Record Correction + Hyperlane Option Fail-Closed Hardening

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-14 after merge #685; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED & MERGED (#685, 2026-06-14).** Remedial PR before PR6b/PR7: corrects PR6 production-doc evidence status and adds
fail-closed validation for public `HyperlaneOptions`. **Zero** `crates/simthing-clausething/src/` changes. No
route/path/predecessor/movement/border/frontline semantics, field operators, RF, Movement-Front, PALMA, driver/GPU,
or FIELD-MOVIE-DATASET-0 work.

## Track scope

0.0.8.6 MapGeneratorCLI PR6R is a narrow hardening pass after DA-approved PR6 (#684). **0.0.8.2.5 MapGen remains
closed and is not reopened.** PR6b (special routes) and PR7 (partition/bridge) are **not** started in this PR.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr6_hyperlane_results.md` | CURRENT_EVIDENCE | Unchanged — PR6 proof preserved |
| `docs/tests/mapgenerator_cli_pr6r_hardening_results.md` | CURRENT_EVIDENCE (DA-approved #685) | Promoted from PROBATION |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Unchanged — preserved baseline |
| `docs/tests/mapgenerator_cli_pr1_params_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr2_lattice_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr3_strategy_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr4_emitter_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgenerator_cli_pr5_lowering_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/tests/mapgen_lowerer_child_id_amendment_results.md` | CURRENT_EVIDENCE | Unchanged |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |
| 0.0.8.2.5 LIVE_GUARDRAIL tests | LIVE_GUARDRAIL | Unchanged — not modified |

No MapGen baseline artifacts deleted or archived.

## Files changed

| Area | Path |
|---|---|
| Hyperlane option validation | `crates/simthing-mapgenerator/src/topology.rs` |
| Invalid-option tests | `crates/simthing-mapgenerator/tests/topology.rs` |
| Production track PR6 status correction + PR6R addendum | `docs/design_0_0_8_1_clausething_production_track.md` |
| PR6R results (this report) | `docs/tests/mapgenerator_cli_pr6r_hardening_results.md` |

Ladder (`docs/design_0_0_8_6_mapgenerator_cli_ladder.md`) already records PR6 as DA-approved & merged (#684),
PR6b for special routes, and PR7 for partition/bridge — unchanged in this PR.

## Production-doc status correction

`docs/design_0_0_8_1_clausething_production_track.md` PR6 addendum updated from **PASS pending DA review /
PROBATION** to **DA-APPROVED & MERGED (#684)** with `mapgenerator_cli_pr6_hyperlane_results.md` as
**CURRENT_EVIDENCE**. PR6 scope clarified as bounded `add_hyperlane` only; special routes remain **PR6b**.

## Hyperlane option validation summary

Added `validate_hyperlane_options(options: &HyperlaneOptions) -> Result<(), HyperlaneError>` and new error variants:

- `HyperlaneError::InvalidEdgeCounts` — `min_edge_count > max_edge_count`
- `HyperlaneError::InvalidFanoutCap` — `max_per_node_fanout == 0`
- `HyperlaneError::UnsatisfiedMinEdgeCount` — after bounded selection, `selected_count < min_edge_count`

`generate_hyperlane_topology` calls `validate_hyperlane_options` at entry and returns `UnsatisfiedMinEdgeCount` when
candidate/fanout/prevent constraints make the minimum impossible. Existing PR6 happy-path behavior unchanged for valid
options from `HyperlaneOptions::from_params`.

## Panic / invalid-option test summary

| Test | Asserts |
|---|---|
| `hyperlane_generation_rejects_min_greater_than_max` | `InvalidEdgeCounts` |
| `hyperlane_generation_rejects_zero_fanout_cap` | `InvalidFanoutCap` |
| `hyperlane_generation_errors_when_min_edge_count_cannot_be_satisfied` | `UnsatisfiedMinEdgeCount` |
| `hyperlane_generation_does_not_panic_on_invalid_public_options` | multiple invalid constructions return `Err`, no panic |
| `existing_hyperlane_generation_happy_path_still_passes` | valid options still produce bounded topology |

## Closed-source gate result

**PASS (expected)** — diff limited to allowed open producer + docs paths; no `crates/simthing-clausething/src/` changes;
no `simthing-mapgenerator` dependency on `simthing-clausething`.

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results (2026-06-14 local validation)

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test -p simthing-mapgenerator` | 105 passed |
| `cargo test -p simthing-clausething --test mapgen_constitution_guards` | 21 passed |
| `git diff --check` | pass |
| closed-src gate | pass — no forbidden paths |

## DA sign-off status

**DA-APPROVED & MERGED — 2026-06-14 (#685).** Project owner approval recorded after merge.

## Whether PR6b or PR7 may proceed

**PR6b may proceed** — PR6R record correction and hyperlane hardening are complete. **PR7** (partition/bridge structural producer + clustering) remains after PR6b per ladder ordering.
