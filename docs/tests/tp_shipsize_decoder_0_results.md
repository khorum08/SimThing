# TP-SHIPSIZE-DECODER-0 Results

## Status

**DONE — DA-APPROVED (2026-07-04, executive DA deep review + fix + lab waiver).** EML ≤32 bound + CPU-oracle parity + ambiguity rejection verified; lowers to existing substrate (no new AccumulatorRole/opcode/GPU); semantic-free below spec; scope clean. **DA fix:** the 2 new tests' `birth_track` was corrected from the rung id to the `0.0.8.5-terran-pirate` scenario envelope (§0A) and the rung-as-track registration removed. **Lab round-trip waived** (horizon generalization, not scenario acceptance; §0A). Merge-cleared.

## Identity

| Field | Value |
|---|---|
| PR | [#1136](https://github.com/khorum08/SimThing/pull/1136) |
| Branch | `tp-shipsize-decoder-0` |
| Base | `origin/master` @ `71065ae4528dce879ac631465e235ac4252bb26e` (post-#1135 merge) |
| Proof | tested_code_sha binding per coverage_basis |

## Scope

Decoder/lowering rung only. No fleets, ships, combat arenas, diplomacy, or AI commitments authored.

## 0R remedial fixes (TP-SHIPSIZE-DECODER-0R)

| Fix | Status |
|---|---|
| Lifecycle boundary check recorded in proof battery | done |
| Ambiguity negative case in public table proof | done (`duplicate_registered_class_is_ambiguous`) |
| Results-doc retention basis corrected | done |
| Lab proof graduation waiver/run requirement explicit | done |

Hydrated class-map ambiguity is structurally eliminated by unique class registration (`BTreeMap` keys). The public proof covers longest-match collision resolution plus the decoder fail-closed branch for duplicate/ambiguous registered class vectors passed directly to `decode_ship_modifier_key`.

## CI/orchestrator surface compliance

- Read `docs/ci_screening_surface.md` before implementation.
- New tests ledgered in `scripts/ci/test_inventory.tsv` with `birth_track=TP-SHIPSIZE-DECODER-0`, `dsu_survivals=0`.
- `TEST-INVENTORY-DRIFT` / `TEST-BUDGET`: exercised via `doctrine_scan.sh` stock gates.
- Lifecycle `--schema` / `--prove` / boundary check run locally.
- No Doctrine Scan INSPECT disposition required on agent host (whole-tree scan expected PASS).
- `/seal-proof` not used (targeted local cargo proof sufficient).
- No workflow/scanner/allowlist edits.

## Implemented decoder forms

| Form | Lowering |
|---|---|
| `shipsize_{class}_{attr}_{add\|mult}` | Longest-match class segmentation; `_add` → `TransformOp::Add` leaf-only on `Custom(...)` install target; `_mult` → `TransformOp::Multiply` subtree on `Ship` |
| `ship_{attr}_{add\|mult}` | Overlay on registered ship combat property |
| `ships_{attr}_{add\|mult}` | Overlay on ship upkeep surface |
| `country_naval_cap_add` | Overlay on faction naval-cap property |
| `triggered_modifier { potential … key }` | Column-backed `potential` alias → `Suspended` overlay + threshold `EventSpec` |
| `complex_trigger_modifier { trigger { property } key }` | Column-backed trigger only; gated-rate `rate_formula` + trigger gate |
| `value:` script values | `RateFormulaSpec` → bounded `EvalEML` tree ≤32 nodes |

## Lowering contract

- No new `AccumulatorRole`, GPU shader, CPU planner, or runtime traversal.
- Skipped/unverified lab proof is not PASS.
- Economic CT-2c path still rejects `shipsize_*` keys (ship decoder is separate admission surface).

## Public synthetic proof table

`crates/simthing-clausething/tests/tp_shipsize_decoder_0.rs` — single table-driven test covering decode (including longest-match and duplicate-class ambiguity rejection), hydration, overlays, triggered/complex-trigger paths, value-formula oracle bit-exact, and non-column-backed hard-error.

## Lab corpus proof status

| Item | Status |
|---|---|
| `CLAUSER_LAB_DIR` available on agent host | no |
| `modifiers.log` committed | no |
| Lab subset proof (`#[ignore]`) | not run |

DA/Owner must either:

1. run the ignored lab proof locally with `CLAUSER_LAB_DIR`, or
2. explicitly waive lab corpus round-trip for this rung and accept the synthetic proof as sufficient for graduation.

## Test lifecycle / inventory updates

| Test | birth_track | retention basis | downstream consumer note | dsu_survivals |
|---|---|---|---|---|
| `tp_shipsize_decoder_0_table` | `TP-SHIPSIZE-DECODER-0` | `permanent-residue:oracle-parity` | `TP-FLEETS-SHIPS-0` | 0 |
| `tp_shipsize_decoder_0_lab_modifiers_log_subset` | `TP-SHIPSIZE-DECODER-0` | `permanent-residue:oracle-parity` | `TP-FLEETS-SHIPS-0` / lab-only | 0 |

Inventory columns: `promotion_target` = `permanent-residue:oracle-parity`; downstream consumer noted in `note` column.

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-clausething --test tp_shipsize_decoder_0 -- --nocapture` | PASS (1 passed, 1 ignored lab; includes `duplicate_registered_class_is_ambiguous`) |
| `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 -- --nocapture` | PASS (12 passed) |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0; 2 pre-existing extra fixture rows) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_boundary_check.sh` | PASS (`TEST-LIFECYCLE-BOUNDARY-CHECK-VERDICT: PASS`) |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS failures=0 inspect=0 |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `git diff --check origin/master...HEAD` | PASS |

## Scope ledger

| Item | Touched? |
|---|---|
| product crates (clausething only) | yes — decoder module + tests (0R: tests + docs only) |
| simthing-spec | no |
| simthing-gpu/driver/sim/mapeditor/tools | no |
| workflows | no |
| doctrine_exec_profiles.tsv | no |
| doctrine_tests.sh | no |
| scans/allowlists | no |
| committed lab corpus | no |
| cargo workspace run | no |

## Graduation routing

- TP-SHIPSIZE-DECODER-0 / 0R complete
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next rung after clearance: TP-FLEETS-SHIPS-0