# WRITE-DOOR-BAND-DELTA-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.2)
- Status: **COMPLETE — DA-GRADUATED / merged #1488 @ d41a079b** (DA verified census+selftest true-exit 0; referee 10/0; driver 133/0/13 across 65 harnesses on RTX 4080/Vulkan)
- HD-RECEIPT: `bfd7fd9c217b`
- ORIENT-RECEIPT: `16b366e49528`
- orientation_rule_stamp: `76fd13d17f16f2f7`
- ANCHOR-ACK: `simthing-0087-pillars@42b6ba6442aa`
- ANCHOR-ACK: `simthing-0087-binding-laws@91270dd77e96`
- ANCHOR-ACK: `rf-arena-substrate@17b5f1e5c2ba`
- Board dispatch: comment `5111853299`
- Remand-1: `5112136052` · Remand-2: `5117474928` · Remand-3: `5117961061` · Remand-4: `5118362104` (DA `5117893595`)
- base_sha: `77ea7f12a933b5f0362afdaa4edf6970b4339ffc` (handoff)
- tested_code_sha / implementation_code_sha: `f0962be98dcf027a3596448ebacf4225e22f7aa6`
- final_head_sha / clearance_pr_head: PR-body-bound only (this file does not self-hash)
- coverage_basis: brace-balanced `#[cfg(test)] mod tests` oracle-door filter + `--selftest` negative (in-module excluded, post-brace visible); plan_struct census exit 0; write-door census + focused 5.2 referees green
- expected_route: `DA-RESERVE(gate-wiring)`
- CLEARANCE-VERDICT: orch owns `/clearance` on exact tip

## Remand-4 discharge (`5118362104`)

**Defect:** Remand-3 `in_cfg_test_mod_region` treated every line after `#[cfg(test)] mod tests` as excluded without locating the module closing brace — a production oracle-door after a closed test module would be silently filtered.

**Fix (preferred):** brace-balanced lexical span (`cfg_test_mod_spans` / depth walk). `--selftest` proves in-module hit excluded and post-closing-brace hit remains visible/retained by the filter.

**Proof:**
- `bash scripts/ci/plan_struct_typing_census.sh --selftest` → exit 0
- `bash scripts/ci/plan_struct_typing_census.sh` → exit 0
- `bash scripts/ci/write_door_band_delta_census.sh` PASS
- focused 5.2 referees: core 8/0, sim encode 4/0, driver 10/0

Accepted 5.2 production code intact. No Phase 10.1 scan-wiring addendum. No merge / no `ANCHOR-TABLE-SURFACE-0` / no `/clearance` from coder lane.

## Remands 1–3 — retained

Exact remaps; typed band deltas; independent completeness; multi-edge GPU transport; cfg(test) structural exclusion class (now brace-balanced).
