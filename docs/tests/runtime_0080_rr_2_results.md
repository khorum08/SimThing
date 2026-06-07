# RUNTIME-0080-RR-2 Results

Status: IMPLEMENTED / PASS — planet-surface labor economy on GPU
Verdict: PASS
Primitive: `SURFACE-LABOR-ECONOMY-GPU-0`
Rung: `RUNTIME-0080-RR-2`
Scope: RR-0 recursive world + RR-1 nested residency consumed; Terran and Pirate planet surfaces materialized; pop→factory labor emit/transfer/recipe on GPU via generic AccumulatorOp; bit-exact parity vs RR-0 surface tick oracle; no recursive reduce-up
Stable report checksum: `bbf8651c0e613c6f`
Deterministic replay checksum: FNV over parity rows (per-surface GPU labor/production bits)

## Scope Ledger

| Spec element | Required by spec | Implemented in RR-2 | Status | Evidence | Deviation |
|---|---|---|---|---|---|
| RR-0 recursive world consumed | yes | yes | implemented | `build_recursive_world`; `rr_0_world_consumed=true` | |
| RR-1 nested residency consumed | yes | yes | implemented | `DescendToSurface` Terran+Pirate; `surface_materialized_rows==100` | |
| Terran planet surface materialized through RR-1 | yes | yes | implemented | `terran_proof.materialized_through_rr_1` | |
| Pirate planet surface materialized through RR-1 | yes | yes | implemented | `pirate_proof.materialized_through_rr_1` | |
| Pop cohort child is GPU labor emitter | yes | yes | implemented | `labor_emit` AccumulatorOp `AddToTarget` band 0 | |
| Factory child is GPU labor consumer | yes | yes | implemented | discrete transfer pop→factory band 1 | |
| Labor emission computed on GPU | yes | yes | implemented | `gpu_labor_bits` match CPU oracle | |
| Factory labor consumption computed on GPU | yes | yes | implemented | factory labor remaining 0 after recipe | |
| Production generation computed on GPU | yes | yes | implemented | conjunctive recipe band 2 | |
| GPU output compared to RR-0 CPU oracle | yes | yes | implemented | `parity_rows` bit-exact | |
| Bit-exact labor parity | yes | yes | implemented | factory labor bits == 0 post-tick | |
| Bit-exact production parity | yes | yes | implemented | factory production bits == 1 | |
| Disabled labor-emitter check fails parity | yes | yes | implemented | `labor_emit_enabled=false` | |
| Re-enabled labor-emitter restores parity | yes | yes | implemented | `labor_emit_enabled=true` | |
| Inactive surface emits no labor | yes | yes | implemented | pirate inactive when not in `active_system_ids` | |
| Inactive surface produces no factory output | yes | yes | implemented | inactive pirate production==0 | |
| No cross-surface labor leakage | yes | yes | implemented | Terran pop→Pirate factory pairing fails parity | |
| Surface economy remains at planet surface tier, not flattened | yes | yes | implemented | 2 surfaces × 100 slots; per-cell pop/factory bindings | |
| Recursive GPU reduce-up/disburse-down deferred to RR-3 | no | n/a | deferred | surface-only staging | |
| Integrated recursive GPU rehearsal deferred to RR-4 | no | n/a | deferred | 100-tick recursive GPU horizon | |
| Standalone M-4A parallel theater track not claimed | no | n/a | deferred | nested RR track only | |

No Deviation Record required — rows 1–18 are `implemented`.

## RR-0 / RR-1 consumption summary

RR-2 consumes `build_recursive_world(0x0080_2000)` and proves Terran and Pirate surfaces are resident through RR-1 (`run_runtime_0080_rr_1` with `DescendToSurface`). Pop and factory slots are bound at real RR-0 surface cell indices within a 2×100-slot GPU layout (Terran base 0, Pirate base 100). The economy is not flattened to system or galaxy scalars (`not_flattened_scalar=true`).

## GPU surface economy model

```text
Band 0 — labor emit:  Constant(1)×POP_LABOR → pop slot col 0 (AddToTarget)
Band 1 — transfer:      pop col 0 → factory col 0 (discrete transfer, 10 labor)
Band 2 — factory recipe: conjunctive recipe factory labor → production col 1 (cost 10 → 1 production)
```

CPU oracle per surface: `factory_recipe_production(POP_LABOR_PER_TICK)` → 10 labor consumed, 1 production, 0 labor remaining at factory.

## Parity proof (tick 0, both active surfaces)

| System | Owner | Labor emitted | Labor consumed | Production | Parity |
|---|---|---:|---:|---:|---|
| Terran | Terran | 10 | 10 | 1 | yes |
| Pirate | Pirate | 10 | 10 | 1 | yes |

Negative controls: disabled emitter, disabled recipe, inactive pirate surface, cross-surface leak — all behave as specified.

## GPU input-list fix (simthing-gpu)

Conjunctive recipe ops require populated GPU input-list storage. RR-2 uses `AccumulatorOpSession::upload_ops_resolving_input_lists` (new) so `MinAcrossInputs`/`ConjunctiveCrossing` recipes execute correctly; dummy input-list buffer now includes `COPY_DST`.

## Explicit non-claims

- **No RR-3 recursive GPU reduce/disburse** — surface-only staging.
- **No RR-4 integrated recursive GPU 100-tick rehearsal**.
- **No standalone M-4A parallel-theater track**.
- **No default session wiring** — opt-in/default-off.
- **No invariant edit** — `docs/invariants.md` untouched.
- **No RUNTIME-0080-0 reopen** — flat R2 loop unchanged.

## Foreground command results

```text
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
