# RUNTIME-0080-RR-4 Results

Status: IMPLEMENTED / PASS — integrated recursive 100-tick GPU rehearsal
Verdict: PASS
Primitive: `INTEGRATED-RECURSIVE-GPU-REHEARSAL-0`
Rung: `RUNTIME-0080-RR-4`
Scope: RR-0 recursive CPU oracle + RR-1 nested residency + RR-2 GPU surface economy + RR-3 recursive GPU transfers consumed; 100-tick persistent-GPU integrated loop with per-tick and final-state bit-exact parity vs RR-0 oracle; recursive rehearsal horizon reached
Stable report checksum: `8a3843dfb76c260f`
Deterministic replay checksum: FNV over 200 tick-parity rows + final stockpiles

## Scope Ledger

| Spec element | Required by spec | Implemented in RR-4 | Status | Evidence | Deviation |
|---|---|---|---|---|---|
| RR-0 recursive world/oracle consumed | yes | yes | implemented | `run_runtime_0080_rr_0` canonical 100-tick PASS | |
| RR-1 nested residency consumed | yes | yes | implemented | `DescendToSurface` Terran+Pirate | |
| RR-2 GPU surface economy consumed | yes | yes | implemented | bands 0–2 each tick | |
| RR-3 recursive GPU transfers consumed | yes | yes | implemented | bands 3+ each tick | |
| 100 recursive ticks executed | yes | yes | implemented | persistent GPU session loop | |
| Tick state feeds next tick | yes | yes | implemented | stockpile carry-forward | |
| Terran recursive path integrated for 100 ticks | yes | yes | implemented | 100 terran tick-parity rows | |
| Pirate recursive path integrated for 100 ticks | yes | yes | implemented | 100 pirate tick-parity rows | |
| Pop labor emission computed on GPU each tick | yes | yes | implemented | band 0 | |
| Factory labor consumption computed on GPU each tick | yes | yes | implemented | bands 1–2 | |
| Production generation computed on GPU each tick | yes | yes | implemented | conjunctive recipe | |
| Surface→planet reduce computed on GPU each tick | yes | yes | implemented | band 3 | |
| Planet→system reduce computed on GPU each tick | yes | yes | implemented | band 4 | |
| System→galaxy reduce computed on GPU each tick | yes | yes | implemented | band 5 | |
| Galaxy→faction stockpile reduce computed on GPU each tick | yes | yes | implemented | band 6 | |
| Faction→galaxy disburse computed on GPU each tick | yes | yes | implemented | disburse band base | |
| Galaxy→system disburse computed on GPU each tick | yes | yes | implemented | disburse band +1 | |
| System→surface/starport disburse computed on GPU each tick | yes | yes | implemented | disburse band +2 | |
| Per-tick labor parity vs RR-0 oracle | yes | yes | implemented | `per_tick_labor_parity_ok` | |
| Per-tick production parity vs RR-0 oracle | yes | yes | implemented | `per_tick_production_parity_ok` | |
| Per-tick reduce-up parity vs RR-0 oracle | yes | yes | implemented | `merge_reduce_rows` all ticks | |
| Per-tick disburse-down parity vs RR-0 oracle | yes | yes | implemented | starport delta + stockpile | |
| Final faction stockpile parity vs RR-0 oracle | yes | yes | implemented | Terran=400 Pirate=100 | |
| Final starport/target receipt parity vs RR-0 oracle | yes | yes | implemented | cumulative disburse sum | |
| No cross-owner leakage over 100 ticks | yes | yes | implemented | wrong_owner_routing fails | |
| No cross-tier shortcut over 100 ticks | yes | yes | implemented | direct_surface_to_stockpile fails | |
| Inactive systems/surfaces remain no-op over 100 ticks | yes | yes | implemented | inactive controls | |
| Not flattened to direct surface→faction scalar | yes | yes | implemented | per-system tier slots | |
| Scope Ledger present and required rows implemented | yes | yes | implemented | this report | |
| No Deviation Record required | yes | yes | implemented | rows 1–30 implemented | |
| Standalone M-4A parallel theater track not claimed | yes | yes | non-claim | nested RR track only | |
| Default session wiring not claimed | yes | yes | non-claim | opt-in/default-off | |
| Invariant edit not performed | yes | yes | non-claim | `docs/invariants.md` untouched | |

No Deviation Record required — rows 1–30 are `implemented`.

## Human-facing narrative — 100-tick integrated sweep

This rung closes the recursive rehearsal horizon specified by `RUNTIME-0080-RR-OPEN-0`. The consumer is not a report aggregation of RR-0 through RR-3: it is a single persistent `AccumulatorOpSession` that executes the full recursive tick pipeline 100 times, carrying faction stockpile and starport receipt state forward between ticks exactly as the RR-0 CPU oracle does.

**What ran.** For each tick 0..99 on the canonical Terran/Pirate fixture (13 active systems, 4 starports): GPU labor emit → factory transfer → conjunctive recipe → surface→planet→system→galaxy→faction reduce-up → staged stockpile→galaxy→system→starport disburse-down. Tier slots reset each tick through discrete transfers; stockpile and starport slots accumulate. Each tick was read back and compared bit-exact to the corresponding `Runtime0080Rr0OracleTick` from the RR-0 100-tick recursive CPU oracle.

**Timing (foreground GPU loop, Windows, discrete GPU).**

| Metric | Value |
|---|---|
| Total integrated GPU loop wall time | **622.5 ms** |
| Mean per-tick wall time | **5.42 ms** |
| Min per-tick wall time | **3.11 ms** |
| Max per-tick wall time | **9.67 ms** |
| GPU order bands dispatched per tick | **19** |
| Readback cadence | per-tick full readback after integrated dispatch |

**Memory footprint.**

| Metric | Value |
|---|---|
| GPU session persistent buffers (VRAM) | **87,868 bytes** (~85.8 KiB) |
| GPU values buffer | **568 bytes** |
| CPU readback staging peak | **568 bytes** |
| Mean process working set (RAM) | **~523.5 MiB** |
| Ending process working set (RAM) | **~524.6 MiB** |

VRAM here is the persistent `AccumulatorOpSession` allocation (`persistent_buffer_bytes`); ephemeral per-readback staging is not included in the persistent total.

**Final state vs RR-0 oracle (tick 99).**

| Owner | Faction stockpile | Cumulative starport receipt | Parity |
|---|---:|---:|---|
| Terran | 400 | matches oracle cumulative disburse | yes |
| Pirate | 100 | matches oracle cumulative disburse | yes |

All 200 tick-parity rows (Terran + Pirate per tick) report `parity=true`. Stockpile monotonicity across ticks confirms tick state feeds the next tick.

## RR-0 / RR-1 / RR-2 / RR-3 consumption summary

- **RR-0:** `build_recursive_world(0x0080_2000)` + 100-tick recursive CPU oracle; per-tick expected labor, production, reduce-up, disburse-down, and stockpile-after values.
- **RR-1:** Terran and Pirate surfaces materialized through nested residency (`DescendToSurface`); inactive-system/surface negative controls preserved.
- **RR-2:** Surface economy bands 0–2 run on GPU every tick; factory labor consumed to zero post-recipe each tick.
- **RR-3:** Reduce-up bands 3–6 and staged disburse bands 7+ run on GPU every tick; not a tick-0-only proof replayed 100 times.

## Integrated recursive 100-tick model

```text
Persistent AccumulatorOpSession (one upload, 100 tick dispatches):
  for tick in 0..100:
    bands 0–2  — RR-2 surface economy
    bands 3–6  — RR-3 reduce-up
    bands 7..  — RR-3 staged disburse-down (stride 3 per starport)
    readback + parity vs oracle_ticks[tick]
    stockpile + starport state carries forward on GPU
```

## Per-tick parity (representative samples)

| tick | owner | labor | production | stockpile after | disburse Δ | parity |
|---:|---|---:|---:|---:|---:|---|
| 0 | Terran | 130 | 13 | 2 | 8 | yes |
| 0 | Pirate | 130 | 13 | 2 | 8 | yes |
| 50 | Terran | 130 | 13 | 202 | 8 | yes |
| 50 | Pirate | 130 | 13 | 52 | 8 | yes |
| 99 | Terran | 130 | 13 | 400 | 8 | yes |
| 99 | Pirate | 130 | 13 | 100 | 8 | yes |

(Full 200-row table available in `Runtime0080Rr4Report.tick_parity_rows`; all rows parity=true.)

## Explicit non-claims

- **No standalone M-4A parallel-theater track.**
- **No default session wiring** — opt-in/default-off.
- **No invariant edit** — `docs/invariants.md` untouched.
- **No RUNTIME-0080-0 reopen** — flat R2 loop unchanged.
- **No generalized multi-faction economy** beyond Terran/Pirate fixture.
- **Recursive rehearsal horizon reached** — RR-4 is the specified end of the RR ladder.

## Foreground command results

```text
cargo test -p simthing-driver --test runtime_0080_rr_4          → 38 passed
cargo test -p simthing-driver --test runtime_0080_rr_3          → 34 passed
cargo test -p simthing-driver --test runtime_0080_rr_2          → 29 passed
cargo test -p simthing-driver --test runtime_0080_rr_1          → 30 passed
cargo test -p simthing-driver --test runtime_0080_rr_0          → 30 passed
cargo test -p simthing-driver --test runtime_0080_0_r2          → 19 passed
cargo test -p simthing-driver --test atlas_0080_0               → 17 passed
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run → 22 passed
cargo test -p simthing-gpu                                      → 203 passed (1 ignored)
cargo build --workspace                                         → ok
cargo fmt --all -- --check                                      → ok
cargo check --workspace                                         → ok
```

## Scratch/log cleanup

No scratch logs, `target/`, worktree artifacts, or replay LDJSON committed. All visibility in this report only.
