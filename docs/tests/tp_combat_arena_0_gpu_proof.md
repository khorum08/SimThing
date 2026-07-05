# TP-COMBAT-ARENA-0 — Owner-local GPU parity proof

## DOCTRINE-TESTS-VERDICT: PASS

## Environment

- Machine: MSI (hostname `MSI`)
- OS: Microsoft Windows NT 10.0.26200.0
- GPU / adapter observed: NVIDIA GeForce RTX 4080 Laptop GPU (discrete; `GpuContext::new_blocking` prefers `DeviceType::DiscreteGpu` per `simthing-kernel/src/context.rs`); also Intel UHD Graphics + Parsec Virtual Display Adapter present on host
- Repo: `khorum08/SimThing`
- Tested SHA: `72dc435540e274a168d1226172330594cb823e15`
- Branch: `master` (includes merged #1145 combat rung and subsequent docs commits through #1147)
- Timestamp: 2026-07-05T04:00:00Z (owner-local run; UTC approximate)

## Command

```bash
cargo test -p simthing-clausething --test tp_combat_arena_0 -- --nocapture
```

## Results

- gpu_two_fleet_contact_matches_transfer_oracle: PASS
- zero_hp_threshold_requests_boundary_removal: PASS
- owner_weapon_damage_mult_changes_damage_via_overlay_only: PASS
- Overall: PASS

## GPU proof

- GPU adapter initialized: yes — `require_gpu()` calls `GpuContext::new_blocking().expect("TP-COMBAT-ARENA-0 requires a real GPU adapter")`; failure panics (no skip path). Test binary exited 0, so adapter + device init succeeded twice (session path + isolated `WorldGpuState`).
- GPU path actually executed: yes — `gpu_two_fleet_contact_matches_transfer_oracle` runs `pipelines.run_tick_pipeline_with_accumulators(...)` on isolated `WorldGpuState`, then `isolated.read_values()` and bit-exact `assert_eq!(cpu.to_bits(), gpu.to_bits())` on hull damage columns.
- Any skip/ignore behavior: no — `0 ignored; 0 filtered out`; no `#[ignore]` on any test in `tp_combat_arena_0.rs`.
- Evidence line(s) from output:

```text
running 3 tests
test owner_weapon_damage_mult_changes_damage_via_overlay_only ... ok
test zero_hp_threshold_requests_boundary_removal ... ok
test gpu_two_fleet_contact_matches_transfer_oracle ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
```

## Raw tail

```text
warning: `simthing-clausething` (test "tp_combat_arena_0") generated 3 warnings (run `cargo fix --test "tp_combat_arena_0" -p simthing-clausething` to apply 3 suggestions)
    Finished `test` profile [optimized + debuginfo] target(s) in 3.31s
     Running tests\tp_combat_arena_0.rs (target\debug\deps\tp_combat_arena_0-88203e6acc9d09dc.exe)

running 3 tests
test owner_weapon_damage_mult_changes_damage_via_overlay_only ... ok
test zero_hp_threshold_requests_boundary_removal ... ok
test gpu_two_fleet_contact_matches_transfer_oracle ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
```

## Conclusion

Citable owner-local GPU proof: **yes** — all three tests passed; GPU adapter initialized without panic; `gpu_two_fleet_contact_matches_transfer_oracle` executed the accumulator transfer GPU pipeline and read back values for bit-exact CPU oracle comparison; no ignored or filtered tests.