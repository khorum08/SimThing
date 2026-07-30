# TP-PURGE-0 — Stage C results (DETACH / DE-NAME / DELETE)

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **PROBATION / proof-present / DA-review-pending** (pointer stays `TP-PURGE-0` until DA stamp)
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `0c1168be074cd145233f4ed2b55a9daaa8b5e613`
- Board dispatch: `5133929991`

## Stage C contract executed

### DETACH
- Removed `simthing-clausething` and `simthing-mapeditor` from
  `crates/simthing-driver/Cargo.toml` **dev-dependencies**.
- Lowered `DEV_COUPLING_CEILING` **2 → 0** in `scripts/ci/detachability_check.sh`.
- Remaining clausething-coupled driver integration suites that blocked ceiling→0
  were paired-reaped with inventory (including open-track 5.3-era TP/corpus
  referees `anchor_table_surface_0`, `canonical_anchor_materialization_0`,
  `order_weight_class_0`, `mapgen_pr8_scheduled_concurrency`). Stage B inline +
  s6 + conservation units are the intended substrate replacements.

### DE-NAME (value-preserving)
- Retired `owner == "pirate"` posture select; fleets author `border_posture` /
  `interior_posture` with defaults.
- TP value preserved by adding those keys on pirate fleets in
  `scenarios/terran_pirate_galaxy.clause` (and clausething fixture mirror).
- Combat arena TP-named weapon fields retired with module deletion (below), not
  renamed in place.

### DELETE
- Deleted `crates/simthing-clausething/src/hydrate_combat_arena.rs` and stripped
  combat plumbing / corpus combat block (main scenario + fixture).
- Reaped workshop `tp_rf_reduce_up_golden` (+ test) and closeout/handoff objects
  `TP-CLAUSE-ECONOMY-AUTHOR-0`, `TP-EMERGENT-TENSION-PROOF-0` leased under closed
  0.0.8.6.
- Updated `tp_full_transpile_0` expectations (no combat payload; fleet count 20).

### Stamp
- Design ladder 5.9 status → **PROBATION**
- Active pointer remains `TP-PURGE-0` (moves only at DA stamp)

## EXIT-PROOF evidence

| check | result |
|---|---|
| `detachability_check.sh` | **PASS** `production_coupling=0 proof_coupling=0 ceiling=0` |
| `detachability_check.sh --selftest` | **PASS** (4 fixtures) |
| `test_inventory_drift_check.sh` | **PASS** 1000/1000 |
| `test_lifecycle_expiry_check.sh --scheduled` | **PASS** expired=0 |
| `cargo build --workspace` | **PASS** |
| adapter-pinned `cargo test -p simthing-driver` | **PASS** 41 passed / 0 failed / 1 ignored / 11 harnesses (`WGPU_BACKEND=vulkan`, `SIMTHING_GPU_ADAPTER_CONTAINS=4080`, `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1`) |
| `agent_scan.sh` (includes doctrine delta scan) | **PASS** delta_inspect=0 / `DOCTRINE-SCAN-VERDICT: PASS` |
| whole-tree `doctrine_scan.sh` | started locally; coding gate satisfied by `agent_scan` (whole-tree left for CI/DA) |
| falsifiable corpus-delete demo | **PASS** — moved `scenarios/terran_pirate_galaxy.clause` aside; s6 + `invariant_set_inline_0` + conservation unit green; restored |

## Fences observed

- No corpus edit to make unrelated engine law pass (posture keys are de-name only)
- No expiry extension; 2026-08-11 clock remains sovereign
- No rename-instead-of-delete for REPLACE wave
- No 5.4–5.8 work; no pointer advance
