# LEGACY-SURFACE-CONVERGENCE-CENSUS-0 results

**Status:** PROBATION / proof-present / orchestration-review-pending  
**Dispatch base:** `4a9ea09a130ef9108fbdff5605e9ad2c79acc417`  
**Coding orientation:** `ORIENT-RECEIPT: b13ebfff99f4` (`orientation_rule_stamp: 61b0dc4071f82089`)  
**Handoff:** `HD-RECEIPT: 155b0cd68976`; Board dispatch `5466416505`; DA authorization `5466350533`

This rung is a census only. It changes no production Rust semantics, performs none of the recorded future actions, creates no sibling checker, and does not start Phase 13.6 or closeout. The existing `constitutional_surfaces.tsv` declaration authority and `constitutional_surface_check.sh` gate now carry the two added census dimensions.

## Mechanical census result

The checked registry contains 16 Phase-13.5 rows: 12 Dimension-A legacy/transitional mediation rows and exactly four Dimension-B authoring-ingress rows. Every row has an explicit category, exact checked identity set, truth source, adapter/mediator, one closed future action, an owner or concrete blocker, and an ingress classification.

| Census result | Count |
|---|---:|
| Dimension A rows | 12 |
| Dimension B rows | 4 |
| `remove` | 2 |
| `internalize` | 5 |
| `preserve-as-compat` | 8 |
| `blocked` | 1 |
| Bounded removal/convergence worklist (`remove|internalize|blocked`) | 8 |

The worklist is directly filterable from the TSV's `future_action` plus `owner_or_blocked_reason` columns. No narrative inference is needed.

### Designer-admission seed enumeration

The first-step token census found and now pins all 16 distinct live file/token identities for the ClauseThing/ClauseScript parking vocabulary:

- `clause_spec.rs`: `clause_script_parser`, `clausething_runtime` (2)
- `diagnostic.rs`: `ClauseScriptParserParked`, `ClauseScriptParserRequestParked`, `ClauseThingRuntimeParked`, `ClauseThingRuntimeRequestParked` (4)
- `mobility_scenario0.rs`: `ClauseThingRuntimeRequestParked`, `reopen_clausething_l3` (2)
- `preflight.rs`: `ClauseScriptParser`, `ClauseScriptParserParked`, `ClauseThingRuntime`, `ClauseThingRuntimeParked` (4)
- `preview.rs`: `ClauseScriptParser`, `ClauseThingRuntime`, `clause_script_parser`, `clausething_runtime` (4)

The row disposition is `remove`, owned by the post-closeout legacy-surface refactor track. This rung only records that disposition.

### Dimension-A surface inventory

| Category | Checked surface | Members | Future action |
|---|---|---:|---|
| kernel vocabulary | designer-admission parking vocabulary | 16 | `remove` |
| ClauseThing lowerer | public `hydrate*.rs` lowerer surfaces | 21 | `preserve-as-compat` |
| ClauseThing lowerer | scenario projection/rebind adapters | 3 | `internalize` |
| Studio branch | `GenerationPreset` | 7 | `internalize` |
| Studio branch | MapGen-to-Spec hydration adapters | 6 | `internalize` |
| Studio branch | `StudioSessionSource` | 2 | `preserve-as-compat` |
| Studio branch | literal runtime vertical seed | 1 | `remove` |
| MapGen branch | generation modes | 2 | `preserve-as-compat` |
| MapGen branch | output forms | 3 | `preserve-as-compat` |
| MapGen branch | registered shape strategies | 11 | `preserve-as-compat` |
| MapGen branch | ClauseThing-local `mapgen*.rs` lowerers | 26 | `internalize` |
| dependency arrow | seven engine Cargo manifests | 1 zero marker | `preserve-as-compat` |

### Dimension-B ingress inventory

| Ingress family | Concrete live route | Classification | Future action |
|---|---|---|---|
| clause hydration | `ingest_clause_scenario_path|bytes` → `parse_raw_document` → `hydrate_scenario_with_source_base` → structural rebind / Studio session | `canonical` | `preserve-as-compat` |
| canonical JSON load | `load_scenario_spec_from_json_str` plus Studio path adapters | `interchange-with-stated-contract` | `internalize` |
| literal install | `admit_and_apply_pack|domain_pack` → compile/apply CPU snapshot only | `dated-deferred` | `blocked` — no live session consumer |
| programmatic spec | `SimSession::open_from_spec|open_from_spec_with_admitted_field_sweeps` → `install_atomic` | `canonical` | `preserve-as-compat` |

## Dependency-arrow proof

The checked engine scope is exactly `simthing-core`, `simthing-spec`, `simthing-kernel`, `simthing-sim`, `simthing-gpu`, `simthing-feeder`, and `simthing-driver`. Their dependency sections contain zero `simthing-clausething` edges. The census emits `ZERO-ENGINE-TO-CLAUSETHING`; a planted engine dependency fails as `ENGINE-CLAUSETHING-DEPENDENCY-ARROW`. The existing detachability gate independently reports `production_coupling=0 proof_coupling=0 ceiling=0`.

## Gate/selftest delta

All 12 pre-existing constitutional negative plants and the existing valid-binding and semantic-rename controls remain green. The extension adds:

- lawful explicit disposition (positive);
- illegal future action (negative, `LEGACY-CENSUS-DISPOSITION`);
- missing literal-ingress family (negative, `LEGACY-CENSUS-COVERAGE`);
- unrelated engine dependency (positive zero-arrow control);
- engine → ClauseThing dependency (negative, named zero-arrow reason).

Observed result: `CONSTITUTIONAL-SURFACE-SELFTEST: PASS planted=12 valid_binding=1 census_plants=3 zero_arrow_plants=2`.

## Verification

| Check | Result |
|---|---|
| `constitutional_surface_check.sh --check` | PASS — A=12, B=4, worklist=8, seed identities=16, zero-arrow marker=1 |
| `constitutional_surface_check.sh --selftest` | PASS — all prior plants plus 3 census / 2 zero-arrow plants |
| `detachability_check.sh` + selftest | PASS — live `0/0/0`; 4 fixtures |
| `gen_orientation.sh --check` | PASS |
| `gen_digest.sh --check` | PASS after regenerating the existing sanctioned-surface digest |
| `doc_budget_check.sh --check` | PASS |
| `anchor_check.sh --check` | PASS (`ANCHOR-COVERAGE` remains advisory INSPECT) |
| `agents_stub_check.sh --check` | PASS |
| `cargo test --workspace --no-fail-fast` | PASS — 133 suites, 590 passed, 0 failed, 15 ignored, 0 measured, 0 filtered |

The hosted Doctrine Scan is recorded in the PR/Board return against the final PR head.
