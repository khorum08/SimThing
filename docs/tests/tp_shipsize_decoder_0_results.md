# TP-SHIPSIZE-DECODER-0 Results

## Status

**PROBATION / DA-OWNER REVIEW** — shipsize / `ship_*` modifier decoder family. Not self-mergeable; DA/Owner clearance required.

## Identity

| Field | Value |
|---|---|
| PR | (open at push time) |
| Branch | `tp-shipsize-decoder-0` |
| Base | `origin/master` @ `71065ae4528dce879ac631465e235ac4252bb26e` (post-#1135 merge) |
| Head | Proof run: current branch tip at proof time; final PR head verified by orchestrator. |

## Scope

Decoder/lowering rung only. No fleets, ships, combat arenas, diplomacy, or AI commitments authored.

## CI/orchestrator surface compliance

- Read `docs/ci_screening_surface.md` before implementation.
- New tests ledgered in `scripts/ci/test_inventory.tsv` with `birth_track=TP-SHIPSIZE-DECODER-0`, `dsu_survivals=0`.
- `TEST-INVENTORY-DRIFT` / `TEST-BUDGET`: exercised via `doctrine_scan.sh` stock gates.
- Lifecycle `--schema` / `--prove` run locally.
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

`crates/simthing-clausething/tests/tp_shipsize_decoder_0.rs` — single table-driven test covering decode, hydration, overlays, triggered/complex-trigger paths, value-formula oracle bit-exact, and non-column-backed hard-error.

## Lab corpus proof status

| Item | Status |
|---|---|
| `CLAUSER_LAB_DIR` available on agent host | no |
| `modifiers.log` committed | no |
| Lab subset proof (`#[ignore]`) | not run |
| DA/Owner local proof required | yes (if DA requires lab round-trip for graduation) |

## Test lifecycle / inventory updates

| Test | birth_track | promotion_target | dsu_survivals |
|---|---|---|---|
| `tp_shipsize_decoder_0_table` | `TP-SHIPSIZE-DECODER-0` | `promotion_target:TP-FLEETS-SHIPS-0` | 0 |
| `tp_shipsize_decoder_0_lab_modifiers_log_subset` | `TP-SHIPSIZE-DECODER-0` | `promotion_target:TP-FLEETS-SHIPS-0` | 0 |

## Proof commands

| Command | Result |
|---|---|
| `cargo check -p simthing-clausething` | PASS |
| `cargo test -p simthing-clausething --test tp_shipsize_decoder_0 -- --nocapture` | PASS (1 passed, 1 ignored lab) |
| `cargo test -p simthing-clausething --test tp_planet_surface_payload_0 -- --nocapture` | PASS (12 passed) |
| `bash scripts/ci/test_inventory_check.sh` | INSPECT (exit 0; 2 pre-existing extra fixture rows) |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --prove` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | PASS failures=0 inspect=0 |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `git diff --check origin/master...HEAD` | PASS |

## Scope ledger

| Item | Touched? |
|---|---|
| product crates (clausething only) | yes — decoder module + tests |
| simthing-spec | no |
| simthing-gpu/driver/sim/mapeditor/tools | no |
| workflows | no |
| doctrine_exec_profiles.tsv | no |
| doctrine_tests.sh | no |
| scans/allowlists | no |
| committed lab corpus | no |
| cargo workspace run | no |

## Graduation routing

- TP-SHIPSIZE-DECODER-0 complete
- PROBATION / DA-OWNER REVIEW
- DA/Owner clearance required
- not self-mergeable
- next rung after clearance: TP-FLEETS-SHIPS-0