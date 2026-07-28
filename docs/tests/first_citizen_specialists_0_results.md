# FIRST-CITIZEN-SPECIALISTS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 3.2)
- Status: **PROBATION / proof-present / DA-review-pending** (orchestrator remand `5100264200` discharged locally; rides PR #1470 draft)
- HD-RECEIPT: `11ddf57fcac0`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- base_sha: `4aeada878f08fc2228d5cc9341baffdd01d90e61` (PR base; handoff listed `2c596973…` at dispatch)
- expected_route: `DA-RESERVE(unclassified-scope)` (handoff); live clearance may report `DA-RESERVE(gate-wiring)` until remedial head + body refresh

## What landed (remand-corrected)

- Authored `specialization = <profile>` on location/entity; hydration stamps structural col/row for `system_target`-enrolled Locations.
- Referees use hydrate → `preview_install` / `InstallError::Specialization` only (no test-side structural mint; no helper-only terminals).
- Citizen counts: `SpecializationReport::citizen_counts()` kept in `specialization.rs` (no `lib.rs` widen) → `gen_specialization_citizen_counts.sh` executable source with `--check`; corrupted TSV fails freshness; board/orientation consume generator TSV.
- HEURISTIC mint guard: authored `*.clause` / `*.simthing-scenario.json` only; hydration-derived dumps excluded; clause+JSON fire; derived quiet; reach-log wired.
- Ladder 3.2 PROBATION; Active open → `ROW-SLOT-OBJECT-SEMANTICS-0`; `docs/sanctioned_surface.md` regenerated.

## Proof

| Check | Result |
|---|---|
| NEW `first_citizen_specialists_0` | 5 passed / 0 failed / 1 ignored (generator CLI) |
| UNMODIFIED `specialization_protocol_0` | 8/8 |
| Full `simthing-driver` live GPU | **119 passed / 0 failed / 13 ignored / 63 harnesses** |
| Command | `$env:WGPU_BACKEND='vulkan'; $env:SIMTHING_GPU_ADAPTER_CONTAINS='4080'; $env:SIMTHING_GPU_REQUIRE_ADAPTER_MATCH='1'; cargo test -p simthing-driver` |
| Adapter pin | `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1` + `ADAPTER_CONTAINS=4080` (Vulkan); mismatch-fatal |
| `agent_scan` | INSPECT (TEST-BUDGET + WORKSHOP-HOMING heuristic; 0 hard FAIL) |
| `gen_digest --check` / sanctioned surface | regenerated + committed on remedial head |
| `doc_budget_check --check` | PASS (pre-remand; re-verify on head) |

## Fences held

- Zero new `SpecializationRequirement` variants
- Zero runtime/tick profile consultation
- Zero edits to existing tests
- Zero kernel/GPU/WGSL edits
- PR #1470 remains draft; coding does not merge
