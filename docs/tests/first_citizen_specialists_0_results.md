# FIRST-CITIZEN-SPECIALISTS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 3.2)
- Status: **PROBATION / proof-present / DA-review-pending** (remand `5100264200` + narrow remand `5103785581` discharged locally; rides PR #1470 draft)
- HD-RECEIPT: `11ddf57fcac0`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- base_sha: `4aeada878f08fc2228d5cc9341baffdd01d90e61`
- expected_route: `DA-RESERVE(unclassified-scope)` (handoff); orchestrator will re-issue `/clearance` on final head

## Remand 2 (`5103785581`) disposition

1. **TEST-INVENTORY-DRIFT** — ledgered ignored `generator_cli` (`test_inventory_drift_check` PASS: unledgered=0).
2. **INSPECT triage** (exact file/symbol; no scanner-exclusion widen):
   - `TEST-BUDGET` → `inspect_justifications.tsv` + `triage_log.tsv` (`first-citizen-specialists-0`): six distinct obligations, not duplicated parameter cases.
   - `WORKSHOP-HOMING-DETECTION` → same ledgers: test-only canonical `scenarios/terran_pirate_galaxy.clause` read for installed citizen-count oracle; no production vocabulary homing.

## What landed

- Authored `specialization = <profile>` on location/entity; hydration stamps structural col/row for `system_target`-enrolled Locations.
- Referees use hydrate → `preview_install` / `InstallError::Specialization` only.
- Citizen counts: `SpecializationReport::citizen_counts()` in `specialization.rs` → `gen_specialization_citizen_counts.sh` + `--check`.
- HEURISTIC mint guard: authored `*.clause` / `*.simthing-scenario.json`; hydration-derived excluded.
- Ladder 3.2 PROBATION; Active open → `ROW-SLOT-OBJECT-SEMANTICS-0`; `docs/sanctioned_surface.md` regenerated.

## Proof (local, pre-final-head stamp)

| Check | Result |
|---|---|
| NEW `first_citizen_specialists_0` | 5 passed / 0 failed / 1 ignored |
| UNMODIFIED `specialization_protocol_0` | 8/8 |
| Full `simthing-driver` live GPU (prior remedial head `8968a813`) | **119/0/13 / 63 harnesses** adapter-pinned |
| Command | `$env:WGPU_BACKEND='vulkan'; $env:SIMTHING_GPU_ADAPTER_CONTAINS='4080'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'; cargo test -p simthing-driver` |
| `test_inventory_drift_check` | PASS |
| `gen_specialization_citizen_counts.sh --check` | PASS |
| `gen_orientation.sh --check` | PASS |
| `doc_budget_check --check` | PASS |
| `agent_scan` | INSPECT×2 (justified + triage_log green); 0 hard FAIL |

Final SHA / hosted Doctrine Scan+Exec IDs are filled on the push that lands this evidence.

## Fences held

- Zero new `SpecializationRequirement` variants
- Zero runtime/tick profile consultation
- Zero edits to existing tests
- Zero kernel/GPU/WGSL edits
- PR #1470 remains draft; coding does not merge; no next-rung dispatch
