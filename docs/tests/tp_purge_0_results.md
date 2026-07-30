# TP-PURGE-0 results — Remand 2 (mechanism-honest Stage B STOP + Stage C truth)

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **STOP for orchestration/DA** — Stage B `CLASSIFICATION-CONFLICT` majority; Stage A/C re-proven on reconciled master
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT: `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- orientation_digest_sha: `90962d8739ea4ec2aad9a388bde9a4e9be55039c92020016d334f9d5edca3ed5`
- Exact master / operational base: `df08db5ebb2d4f8af874cfe151c1aa157100af36` (#1521 FRESHNESS-NO-CARGO)
- Board dispatch: `5133929991` · Remand 1: `5134500978` · Remand 2: `5135691949`
- expected_route: `DA-RESERVE(gate-wiring)`
- Rung: `TP-PURGE-0`

## Stage package

| stage | disposition | headline |
|---|---|---|
| A REAP | provisionally ACCEPTED; re-proven | 441/441 ok=441 fail=0; ledger-only=2; stage_b_restored=3; authorized deletions **22** |
| B REPLACE | **STOP** | 222 rows → **37** real inv×mech families; **4 SURVIVOR / 218 CLASSIFICATION-CONFLICT** |
| C DETACH/DE-NAME/DELETE | direction ACCEPTED; truth refreshed | ceiling **0/0/0**; combat corpus properties removed; admission **anchored=18 unobserved=3 total=21** |

## Remand 2 repairs

1. Reconciled onto master `df08db5ebb2d4f8af874cfe151c1aa157100af36` (preserve #1521 no-cargo ordinary orient).
2. Stage C: removed four combat `Unobserved` property defs from `scenarios/terran_pirate_galaxy.clause`; regenerated via `ORIENTATION_VERIFY_EXECUTABLE_SOURCES=1` → dark=`tp::hull`,`tp::upkeep`,`tp::weapon_damage` only.
3. Stage B map rebuilt mechanism-honest (`docs/tests/tp_purge_0_stage_b_replacement_map.tsv`); stale `cpu_gpu_parity_inline_pack_matches_registration_count` claim scrubbed.
4. Corpus-absent engine matrix: **177 passed / 0 failed / 1 ignored** (37 harnesses); corpus restored; workspace build PASS; adapter-pinned driver **41/0/1** (11 harnesses); `tp_full_transpile_0` PASS.

### Honest Stage B survivors (4 rows)

| invariant × mechanism | survivor |
|---|---|
| conservation × composite-conservation | `composite_bite_nonconservative_fails_conservative_passes` (×2 old rows) |
| conservation × allocator-conservation | `allocator_broken_disburse_exceeding_eps_bound_fails` |
| residency-typing × column-index-residency | `gpu_round_trip_door_preserves_column_bits` |

218 CONFLICT families (mobility-gpu, eml-intensity, replay-determinism, atlas-gpu-parity, studio-typed-observation, …) require DA direction for genuine inline falsifiers — not invented mappings.

## STOP

Pointer remains `TP-PURGE-0`. No merge / graduation / Active advance without DA.
