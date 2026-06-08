# B-0-ACCEPT-0 — Design-Authority Ruling: Accept Narrow D-2a Hard-Currency Ordering; Close Line B at Smoke Level

**Reviewer:** Opus 4.8 (design authority — v7.8 track). **Date:** 2026-05-30.
**Decision:** **ACCEPT B-0 (Option B) — Line B/T CLOSED at the narrow smoke level.** Authored
`order_band` reaches existing AccumulatorOp OrderBand execution; cross-band same-source sequential
debits execute deterministically with exact CPU-oracle parity; same-band double-debit rejection is
preserved (now per-band); Resource Flow is not used for hard-currency. **No B-1 opens.** A-0 remains
queued; Line C runtime deferred; L3 parked; FrontierV2-5 and ACT/EVENT/OBS/PIPE unauthorized.

## Reviewed — code + tests, not only the report

- `crates/simthing-core/src/accumulator_op_builder.rs` — `DiscreteTransferRegistration.order_band`
  → `gate: GateSpec::OrderBand(reg.order_band)` (line 327). **Authored band carried, not flattened.**
- `crates/simthing-gpu/src/transfer_accumulator.rs` — `TransferRegistration.order_band`;
  `n_bands = n_bands.max(reg.order_band + 1)`; per-band contention key `(order_band, slot, col)`;
  error "consumed input cell appears in more than one same-band transfer op."
- `crates/simthing-gpu/src/accumulator_op/session.rs` + `passes.rs` — multi-band transfer dispatch
  over the **existing** AccumulatorOp execute pipeline (no new WGSL).
- `crates/simthing-driver/src/resource_economy_oracle.rs` — `max_transfer_band` + `execute_ops_cpu`
  over `0..=max_band`; exact integer execution.
- `crates/simthing-driver/src/resource_economy_boundary_schedule.rs` — deterministic
  `BoundaryScheduleKey { order_band, kind_rank, authoring_id }`, sorted.
- `crates/simthing-sim/src/**` — `ResourceEconomySpec`/`order_band`/D-2a scan **empty**.

## Review answers

| # | Question | Finding |
|---|---|---|
| 1 | Authored `order_band` reaches materialized registrations? | **Yes** — spec → compiled → `DiscreteTransferRegistration.order_band` → planner. |
| 2 | `GateSpec::OrderBand(n)` uses authored band (not flatten to 0)? | **Yes** — `GateSpec::OrderBand(reg.order_band)`; band-0 literals are test fixtures only. |
| 3 | `encode_transfer_into` executes existing pipeline over the bands? | **Yes** — per-OrderBand dispatch on the existing execute pipeline; `n_bands` threaded via `passes.rs`. |
| 4 | Same-band same-source double-debit rejection intact? | **Yes** — per-band `(order_band, slot, col)` contention key; rejection tests pass. |
| 5 | Cross-band same-source sequential debit deterministic? | **Yes** — ascending-band passes; `b0_cross_band_same_source_sequential_debit_succeeds` passes (treasury 10→7→3). |
| 6 | Boundary schedule report stable/adequate? | **Yes** — sorted by the stable key; transfer cell metadata included. |
| 7 | Ordering key deterministic — `(order_band, kind_rank, authoring_id)`? | **Yes** — `authoring_id` tiebreak; no event-arrival nondeterminism. |
| 8 | CPU oracle exact and appropriate for hard-currency? | **Yes** — exact integer band-ordered execution; correct for discrete hard-currency. |
| 9 | GPU/CPU parity bit-exact for the fixture? | **Yes** — treasury_A `0x40400000`, sink_0 `0x40400000`, sink_1 `0x40800000`; conservation error 0.0; replay bit-exact 3 ticks. |
| 10 | Flag-off / opt-in unchanged? | **Yes** — `resource_economy_opt_in` 10/10; flag-off populated spec still rejected. |
| 11 | Resource Flow kept separate from hard-currency? | **Yes** — `use_accumulator_resource_flow=false`; hard-currency on the discrete transfer AccumulatorOp path. |
| 12 | Avoided new WGSL / AccumulatorRole / CPU fallback / global scheduler / schema redesign? | **Yes** — reuses existing OrderBand execute; only an `order_band` field carry + multi-band dispatch + a deterministic schedule report; no cross-tick cache. |
| 13 | Avoided `simthing-sim` spec/transaction awareness? | **Yes** — scan empty. |
| 14 | Accept as Line B's first D-2a slice? | **Yes.** |
| 15 | Close at smoke, or open B-1? | **Close at smoke (Option B).** The named T scenario's core need — deterministic cross-band sequential hard-currency ordering — is proven. No B-1. |

## Non-blocking observation (no remediation)

The schedule key carries `kind_rank` (transfer=0, recipe=1, emission=2), but B-0 exercises
**transfers only** (recipes/emission at band 0). The mixed-kind multi-band ordering path is in place
but un-stressed. This is **not** a blocker for the hard-currency transfer scenario; a future named
scenario with interleaved recipe/emission multi-band ordering (or an all-band-union contention
policy) would be the named need that opens a B-1 — not opened now (a consumer-less ladder extension
is the antipattern to avoid).

## Verification

| Command | Result |
|---|---|
| `cargo test -p simthing-driver --test phase_t_b0_d2a_hard_currency_ordering` | **11/11 PASS** |
| `cargo test -p simthing-driver --test resource_economy_compile` | **8/8 PASS** |
| `cargo test -p simthing-driver --test resource_economy_burn_in` | **5/5 PASS** |
| `cargo test -p simthing-driver --test resource_economy_opt_in` | **10/10 PASS** |
| `cargo test -p simthing-spec --test resource_economy_compile_rejections` | **12/12 PASS** |
| `cargo check --workspace` | **PASS** (pre-existing `simthing-driver` unused-import warning only) |

Unlike C-2, the landed B-0 compiles and its tests pass against the tree as reported.

## Guardrail confirmations (no authorization)

A-0/E-11B/E-11B-5, Line C production runtime / sparse-residency scheduler, ClauseThing/ClauseScript/
L3, FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, new WGSL, new `AccumulatorRole`, CPU production
fallback, Resource Flow substitution for hard-currency, `ResourceEconomySpec` schema redesign,
global scheduler/cache, default-on transfer/emission/economy, nondeterministic event-arrival
arbitration, weakened double-debit rejection, weakened exact conservation, `simthing-sim` semantic
awareness — **all remain unauthorized.** No invariant change.

## Ruling

**ACCEPT B-0; LINE B/T CLOSED AT NARROW SMOKE LEVEL.** B-0 proves narrow driver-only D-2a
hard-currency ordering: authored `order_band` → existing AccumulatorOp OrderBand execution;
deterministic boundary scheduling; cross-band same-source sequential debits with exact CPU-oracle
parity; same-band double-debit rejection preserved (per-band); replay bit-exact; Resource Flow not
used for hard-currency. **Line B/T is closed at the narrow smoke level for the current v7.8 named
scenario. A-0 remains the only accepted, queued M/E/T line not yet implemented** — opening it is a
product decision. v7.8 constitution / production-track split intact.
