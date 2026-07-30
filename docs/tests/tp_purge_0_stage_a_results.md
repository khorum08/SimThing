# TP-PURGE-0 — Stage A results (REAP)

- Track: 0.0.8.7 RF arena modernization (rung 5.9 / `TP-PURGE-0`)
- Status: **Stage A complete** (separately relayable) — Stages B/C follow
- HD-RECEIPT: `3555f6da869e`
- ORIENT-RECEIPT (coding, fresh @ exact head): `56fe3e6032b0`
- orientation_rule_stamp: `1497628db25456ff`
- Exact master / operational base: `0c1168be074cd145233f4ed2b55a9daaa8b5e613`
- Board dispatch: `5133929991`

## Stage A contract executed

Paired reap of approved split rows where `disposition=PAIR-REAP` **or** `invariant=NONE`
from `docs/tests/lifecycle_invariant_split_proposal_2026_08_11.tsv`.

| metric | value |
|---|---|
| Stage A unique targets | **441** |
| Inventory rows removed | **441** |
| Production coupling after Stage A | **0** (unchanged) |

### Production safety (STOP condition honored)

An initial remover matched a non-test `pub fn default_unattached` on
`StudioLiveSessionBridgeReadout` (same name as a unit smoke test). That broke
production compile. All Stage-A `/src/` files were restored from `0c1168be` and
re-reaped with a **`#[test]`-attribute-required** remover. Production methods
preserved.

Tooling: `scripts/ci/tp_purge_stage_a_reap.py` (supports `--source-only`).
Report: `docs/tests/tp_purge_0_stage_a_reap_report.tsv`.

## Stage A gates

| check | result |
|---|---|
| `cargo build --workspace` | **PASS** |
| `test_inventory_drift_check.sh` | **PASS** (1242/1242) |
| `test_lifecycle_expiry_check.sh --scheduled` | **PASS** expired=0 |
| `detachability_check.sh` | **PASS** production=0 proof=2 ceiling=2 |

## Fences observed

- No corpus edit to make proofs pass
- No expiry extension
- No rename-in-place of deleted identities
- No 5.4–5.8 work
- 268 `TP-PURGE-SUCCESSOR` non-NONE residue remains for the sovereign 2026-08-11 clock / later stages as applicable

## Next

- **Stage B:** collapse 222 `REPLACE-INLINE-INVARIANT` rows to low-single-digit inline proofs per (invariant × mechanism), then paired-reap the old forms
- **Stage C:** detach (ceiling 2→0), de-name, delete `hydrate_combat_arena.rs` + workshop/handoff residue; stamp 5.9 PROBATION
