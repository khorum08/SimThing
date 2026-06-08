# Phase B-0 — Narrow Driver-Only D-2a Hard-Currency Ordering Smoke

**Date:** 2026-05-30  
**Base HEAD (pre-B-0 branch):** `ce99eafa3d61909b0fb33576156a47632fc44978` (C-2-ACCEPT-0)  
**Branch:** `b0-d2a-hard-currency-ordering`  
**PR:** #357 (merged to `master`)  
**Class:** GpuVerified (transfer OrderBand path) + exact CPU oracle parity  
**Verdict:** **PASS — implemented; pending Opus/design-authority review** (not accepted)

---

## C-2 / v7.8 state summary

- **L0 Frontier consumer:** accepted  
- **L1 simthing-spec buildout:** accepted  
- **L2 CLAUSE-SPEC-0:** accepted  
- **Line C/M:** C-0/C-1/C-2 accepted; map batching **CLOSED** at designer surface  
- **Line A/E:** NamedScenarioAccepted; **A-0 queued**, not opened  
- **Line B/T:** NamedScenarioAccepted; **B-0 selected and implemented** (this report)  
- **L3 ClauseThing/ClauseScript:** parked  
- **FrontierV2-5 / ACT/EVENT/OBS/PIPE:** unauthorized  

---

## D-2a readiness summary

[`reviews/d2a_boundary_transaction_scheduling_readiness.md`](../reviews/d2a_boundary_transaction_scheduling_readiness.md) and [`reviews/d1_discrete_transaction_contention_memo.md`](../reviews/d1_discrete_transaction_contention_memo.md) already landed policy A (same-band consumed-input rejection) and identified the gap: authored `order_band` was preserved through compile/materialization reports but flattened to band 0 at GPU boundary planning. B-0 closes that narrow gap without a global scheduler.

---

## Pre-edit audit

| Question | Answer |
|---|---|
| 1. Where is `order_band` authored? | `ResourceTransferSpec::order_band` in `simthing-spec` |
| 2. Where preserved today? | `CompiledResourceTransfer.order_band`; `transfer_order_band_by_id` in materialization report |
| 3. Where dropped? | `DiscreteTransferRegistration` lacked field; `plan_transfer_ops` hardcoded `OrderBand(0)`; `encode_transfer_into` dispatched only band 0; CPU oracle `max_transfer_band` ignored bands |
| 4. Minimal driver change | Carry `order_band` on registrations; use in planner; multi-band GPU encode; boundary schedule report |
| 5. Same-band rejection unchanged? | Spec compile contention tracker + GPU planner per-band `(band, slot, col)` keys |
| 6. Cross-band sequential debits? | Ascending OrderBand passes; band 0 debit completes before band 1 |
| 7. CPU oracle | Sort/execute by band via `execute_ops_cpu`; exact integer bit patterns |
| 8. Queued/parked | A-0, Line C runtime, L3, FrontierV2-5, ACT/EVENT/OBS/PIPE |
| 9. Not global scheduler? | No cross-tick cache; only existing AccumulatorOp OrderBand dispatch at boundary upload |
| 10. Not Resource Flow? | Hard-currency stays on discrete transfer AccumulatorOp path; RF flag off |

---

## Files changed

| File | Change |
|---|---|
| `crates/simthing-core/src/accumulator_op_builder.rs` | `DiscreteTransferRegistration.order_band`; `discrete_transfer_registration_to_op` → `GateSpec::OrderBand(n)` |
| `crates/simthing-gpu/src/transfer_accumulator.rs` | `TransferRegistration.order_band`; per-band contention; `n_bands` from max band |
| `crates/simthing-gpu/src/accumulator_op/session.rs` | `encode_transfer_into` dispatches all `n_bands` (existing execute pipeline, no new WGSL) |
| `crates/simthing-gpu/src/passes.rs` | Pass `accumulator_transfer_bands` to transfer encode |
| `crates/simthing-driver/src/resource_economy_compile.rs` | Materialize `order_band` onto registrations |
| `crates/simthing-driver/src/resource_economy_oracle.rs` | `max_transfer_band` reads registration bands |
| `crates/simthing-driver/src/resource_economy_boundary_schedule.rs` | **New** deterministic boundary schedule report |
| `crates/simthing-driver/tests/phase_t_b0_d2a_hard_currency_ordering.rs` | **New** B-0 proof tests (11) |
| `crates/simthing-driver/tests/resource_economy_compile.rs` | Assert wired band on materialized op |
| `crates/simthing-sim/tests/e2a_resource_transfer_discrete_builder.rs` | Struct literal `order_band: 0` |
| Docs | production track, design_v7_8 note, mapping guidance, worklog |

---

## Implementation summary

Authored transfer `order_band` now flows:

```text
ResourceTransferSpec.order_band
→ CompiledResourceTransfer.order_band
→ DiscreteTransferRegistration.order_band
→ GateSpec::OrderBand(n) in rebuild/plan paths
→ encode_transfer_into bands 0..n_bands-1 on existing AccumulatorOp execute pipeline
```

---

## Ordering model

Stable key: **`(order_band, kind_rank, authoring_id)`**

| kind_rank | Kind |
|---:|---|
| 0 | transfer |
| 1 | recipe |
| 2 | emission/threshold (not in B-0 fixture) |

B-0 fixture focuses on discrete transfers only; recipes remain band 0.

---

## Boundary schedule report shape

`ResourceEconomyBoundaryScheduleReport::build(registry)` → sorted `Vec<BoundaryScheduleEntry>` with:

- `key: BoundaryScheduleKey { order_band, kind_rank, authoring_id }`
- transfer cell metadata (`source_slot`, `source_col`, `target_slot`, `target_col`, `amount`)

---

## CPU oracle summary

`run_transfer_recipe_cpu_oracle` executes bands `0..=max(order_band)` using `execute_ops_cpu` with `GateSpec::OrderBand(n)` ops. Same-band contention unchanged at compile time; cross-band same-source allowed when bands differ.

---

## B-0 fixture

```text
treasury_A initial = 10
transfer_0: band 0, treasury_A → sink_0, amount 3
transfer_1: band 1, treasury_A → sink_1, amount 4
Expected: treasury=3, sink_0=3, sink_1=4 (exact)
```

---

## Exact parity table

| Cell | CPU (bits) | GPU (bits) | Match |
|---|---|---|---|
| treasury_A | `0x40400000` (3.0) | `0x40400000` | yes |
| sink_0 | `0x40400000` (3.0) | `0x40400000` | yes |
| sink_1 | `0x40800000` (4.0) | `0x40800000` | yes |

Burn-in `max_abs_conservation_error = 0.0`; `replay_bit_exact = true` across 3 ticks.

---

## Replay fingerprint

Not recorded (SHA/fingerprint hygiene out of scope for B-0 per handoff).

---

## Safety behavior matrix

| Case | Result |
|---|---|
| same source, same order_band | **reject** (spec compile) |
| same source, different order_band | **allowed** (B-0 fixture passes) |
| disjoint sources, any bands | allowed |
| Resource Flow for hard currency | **not used** |
| flag-off + populated transfer spec | **reject** (unchanged) |

---

## Guardrail scans

| Scan | Expected | Result |
|---|---|---|
| B-0 / order_band references in crates+docs | present in B-0 paths | pass |
| Resource Flow substitution for hard-currency | separate / not used | pass (`use_accumulator_resource_flow=false` in fixture) |
| new WGSL / AccumulatorRole / CPU fallback / global scheduler | guardrail-only | pass (no new WGSL; existing execute pipeline only) |
| A-0 / Line C runtime / M-6A / M-5 | deferred only | pass |
| ClauseThing / L3 / FrontierV2-5 / ACT-EVENT-OBS-PIPE | parked/rejected | pass |
| simthing-sim semantic awareness | none in lib.rs | pass |

---

## Test results

```text
cargo test -p simthing-driver --test phase_t_b0_d2a_hard_currency_ordering -- --nocapture
→ 11 passed; 0 failed

cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission -- --nocapture
→ 25 passed; 0 failed

cargo test -p simthing-spec --test v7_8_met_consumer_scenarios -- --nocapture
→ 10 passed; 0 failed

cargo check --workspace
→ Finished (green)

Regressions:
cargo test -p simthing-driver --test resource_economy_compile → 8/8
cargo test -p simthing-driver --test resource_economy_burn_in → 5/5
cargo test -p simthing-driver --test resource_economy_opt_in → 10/10
cargo test -p simthing-spec --test resource_economy_compile_rejections → 12/12
```

---

## Transient cleanup

No scratch/tmp/log artifacts under `docs/tests/` requiring deletion.

---

## Final verdict

**PASS — B-0 landed a narrow driver-only D-2a hard-currency ordering smoke:** authored `order_band` reaches existing AccumulatorOp OrderBand execution, deterministic boundary scheduling is reported, cross-band same-source sequential debits execute with exact CPU oracle parity, same-band double-debit rejection remains intact, replay is deterministic, Resource Flow is not used for hard-currency, and no new WGSL, AccumulatorRole, CPU fallback, global scheduler, simthing-sim semantics, A-0, Line C runtime, L3, FrontierV2-5, or ACT/EVENT/OBS/PIPE expansion was added. **B-0 is implemented and pending Opus/design-authority review.**
