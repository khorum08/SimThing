# RUNTIME-0080-RR-3 Results

Status: IMPLEMENTED / PASS — recursive GPU reduce-up/disburse-down
Verdict: PASS
Primitive: `RECURSIVE-GPU-REDUCE-DISBURSE-0`
Rung: `RUNTIME-0080-RR-3`
Scope: RR-0 recursive world/oracle + RR-1 nested residency + RR-2 GPU surface production consumed; Terran and Pirate recursive paths proven; GPU reduce-up surface→planet→system→galaxy→faction stockpile and staged disburse-down stockpile→galaxy→system→starport with bit-exact tick-0 parity vs RR-0 oracle; negative controls for disabled tiers, wrong-owner routing, cross-tier shortcut, inactive surfaces/systems
Stable report checksum: `f6adf4116656e4a8`
Deterministic replay checksum: FNV over reduce-up + disburse-down parity rows

## Scope Ledger

| Spec element | Required by spec | Implemented in RR-3 | Status | Evidence | Deviation |
|---|---|---|---|---|---|
| RR-0 recursive world/oracle consumed | yes | yes | implemented | `run_runtime_0080_rr_0` canonical 100-tick PASS | |
| RR-1 nested residency consumed | yes | yes | implemented | `DescendToSurface` Terran+Pirate | |
| RR-2 GPU surface production consumed | yes | yes | implemented | `run_runtime_0080_rr_2` PASS + surface bands 0–2 | |
| Terran recursive path proven | yes | yes | implemented | terran reduce+disburse parity | |
| Pirate recursive path proven | yes | yes | implemented | pirate reduce+disburse parity | |
| Surface→planet reduce computed on GPU | yes | yes | implemented | band 3 discrete transfer | |
| Planet→system reduce computed on GPU | yes | yes | implemented | band 4 discrete transfer | |
| System→galaxy reduce computed on GPU | yes | yes | implemented | band 5 discrete transfer | |
| Galaxy→faction stockpile reduce computed on GPU | yes | yes | implemented | band 6 owner-masked transfer | |
| Faction→galaxy disburse computed on GPU | yes | yes | implemented | band 7 stockpile→galaxy | |
| Galaxy→system disburse computed on GPU | yes | yes | implemented | band 8 galaxy→system | |
| System→planet/surface/starport disburse computed on GPU | yes | yes | implemented | band 9 system→starport | |
| Reduce-up parity vs RR-0 oracle | yes | yes | implemented | `reduce_up_rows` | |
| Disburse-down parity vs RR-0 oracle | yes | yes | implemented | `disburse_down_rows` | |
| Bit-exact production/resource parity | yes | yes | implemented | stockpile+starport bits | |
| Disabled surface→planet reduce fails parity | yes | yes | implemented | `surface_to_planet_enabled=false` | |
| Re-enabled surface→planet reduce restores parity | yes | yes | implemented | `surface_to_planet_enabled=true` | |
| Disabled galaxy→faction reduce fails parity | yes | yes | implemented | `galaxy_to_stockpile_enabled=false` | |
| Re-enabled galaxy→faction reduce restores parity | yes | yes | implemented | `galaxy_to_stockpile_enabled=true` | |
| Disabled disburse-down fails parity | yes | yes | implemented | `disburse_down_enabled=false` | |
| Re-enabled disburse-down restores parity | yes | yes | implemented | `disburse_down_enabled=true` | |
| No cross-owner leakage | yes | yes | implemented | wrong_owner_routing fails parity | |
| No cross-tier shortcut | yes | yes | implemented | direct_surface_to_stockpile fails parity | |
| Inactive systems/surfaces do not reduce or disburse | yes | yes | implemented | inactive pirate surface + inactive starport system | |
| Not flattened to direct surface→faction scalar | yes | yes | implemented | per-system tier slots + stockpile slots | |
| Integrated recursive 100-tick GPU rehearsal deferred to RR-4 | no | n/a | deferred | tick 0 representative proof | |
| Standalone M-4A parallel theater track not claimed | no | n/a | deferred | nested RR track only | |

No Deviation Record required — rows 1–25 are `implemented`.

## RR-0 / RR-1 / RR-2 consumption summary

RR-3 consumes `build_recursive_world(0x0080_2000)`, proves RR-1 surface residency for Terran and Pirate active systems, and stages RR-2 surface economy bands 0–2 before recursive transfers. RR-0 oracle tick 0 supplies reduce-up and disburse-down expected bits. The economy is not flattened to a direct surface→faction scalar (`not_flattened_scalar=true`).

## GPU recursive transfer model

```text
Bands 0–2 — RR-2 surface economy (labor emit, pop→factory transfer, conjunctive recipe)
Band 3  — surface→planet reduce (discrete transfer)
Band 4  — planet→system reduce
Band 5  — system→galaxy reduce
Band 6  — galaxy→faction stockpile reduce (owner-masked)
Bands 7+ — staged disburse-down (stockpile→galaxy→system→starport, stride 3 per starport)
Band 40 — cross-tier shortcut negative control (disabled in PASS path)
```

## Parity proof (tick 0, Terran + Pirate active paths)

Reduce-up and disburse-down rows match RR-0 oracle tick 0 for both owners. Stockpile and starport resource bits are bit-exact. Negative controls: disabled reduce tiers, disabled disburse, wrong-owner routing, direct surface→stockpile shortcut, inactive pirate surface, inactive starport system — all behave as specified.

## Explicit non-claims

- **No RR-4 integrated recursive 100-tick GPU rehearsal** — tick-0 representative proof only.
- **No standalone M-4A parallel-theater track**.
- **No default session wiring** — opt-in/default-off.
- **No invariant edit** — `docs/invariants.md` untouched.
- **No RUNTIME-0080-0 reopen** — flat R2 loop unchanged.

## Foreground command results

```text
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
