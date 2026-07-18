# RF-NEED-WEIGHT-PROFILE-TRANSPORT-0 (RF-5) Results

## Status

**PROBATION / proof-present / DA-review-pending** — 2026-07-18 (coder=Grok-CLI).

Owner alone closes screenshot-C [OVL]. Coding agent does not graduate, mark ready, or merge.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-WEIGHT-PROFILE-TRANSPORT-0` (RF-5) |
| PR | #1414 |
| Base / master | `99fa9d06898a1b03b490b7f876a17dfd4568e500` |
| RF-4 graduation | **DA-GRADUATED / merged #1413 @ `99fa9d06`** |
| HD-RECEIPT | `9132665903e6` |
| ORIENT-RECEIPT | `2c9fde39d1d6` (role=coding) |
| Expected route | `DA-RESERVE(gate-wiring)` |
| tested_code_sha | `5ca3b3e093fbd84f7683741ed19f8477e1b7b427` |
| coverage_basis | PASS (workshop RF-5 3/3 + mapeditor RF-4 elevate 8/8 + agent_scan) |

## What landed

1. **Governance (first commit):** stamped RF-4/12.9 DA-GRADUATED, added RF-5 to the ladder, made RF-5 active, regenerated orientation.
2. **Generic spec binding** `NeedWeightProfileBindingSpec` on `ResourceFlowSpec.need_weight_profiles`: hydrated WeightedAccumulator EML stack → admitted Arena participant need cell.
3. **Install + EvalEML band** (`simthing-driver::need_weight_profile`): resolve at session open (fail-closed on empty weight seeds / misbound target), seed weights, register ExactDeterministic EvalEML WeightedAccumulator writing the live need column; optional mirror into AllocatorWeight; threshold metadata retained for readout.
4. **Studio attach:** field-bearing recursive RF profile promotes pack `weight_profiles` into admitted bindings on the selected Owner; Studio_ops telemetry surfaces profile id/kind, seeds, live need, threshold result.
5. **Focused proofs** (workshop, 3/3 PASS): paired weight seeds change live need and below/cross threshold; empty seeds fail closed; misbound install fails closed.

## Load-bearing proofs

| Claim | Result |
|---|---|
| Paired authorings differing only in weight_seeds → different live need | PASS (`0.4` vs `2.0` for input 2.0 × seeds 0.2 / 1.0) |
| Below-threshold / crossing controls | PASS (thr=1.5) |
| Empty weight_seeds fail closed (no default-weight fallback) | PASS |
| Misbound install target fail closed | PASS |
| RF-4 mapeditor elevate battery | PASS 8/8 |
| `agent_scan` | PASS (post-implementation re-run binds final SHA) |

## Fences held

- No new ClauseScript grammar, kernel/WGSL primitive, planner, or scenario API.
- No Studio-side RF arithmetic / feeder / direct accumulator mutation outside install seed + EvalEML/Accumulator ops.
- No sealed-crate TP special case; proofs workshop-homed.
- RF-1 and RF-4 remain unchanged authorities; this rung is transport/threshold only (not 12.10 macro emergence).

## Owner [OVL]

**Open.** Screenshot C from frozen Windows debug `simthing-studio` must show scenario, tick, authored profile identity/value, live need value, and threshold result. Owner alone closes OVL.

## Known gaps / residual

- GPU Threshold+EmitEvent ingress for need cells is metadata + live need comparison in telemetry; sealed EmitEvent wiring for need can deepen under DA if required beyond EvalEML need write + AllocatorWeight mirror.
- Canonical TP multi-profile Studio attach uses explicit baseline seeds (1.0) + literal inputs (2.0); paired-authoring falsifiers override seeds only.

## Graduation routing (for orchestrator/DA)

- CI: green on focused crates + agent_scan
- Risk class: gate-wiring / DA-reserve (expected)
- Falsification: workshop RF-5 battery + RF-4 elevate nonregression
- Recommended posture: DEEP-TREE DA after Owner OVL screenshot C
