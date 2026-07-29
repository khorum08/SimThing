# CANONICAL-ANCHOR-MATERIALIZATION-0 — implementation results (PROBATION)

- Track: 0.0.8.7 RF arena modernization (rung 5.3b)
- Status: **PROBATION** — exit proofs green locally; DA alone graduates / advances pointer
- HD-RECEIPT: `63f01c28e4df`
- ORIENT-RECEIPT: `8555a6c8ed5c`
- orientation_rule_stamp: `428229b9129d598e`
- Dispatch: orch `5124117080` · DA HOLD lift `5124095512` · PR #1499 @ `09b2e98c`
- Prior census STOP: #1495 / HD `c1599e8b6a89` (superseded)

## Binding law applied

1. **Seven DA-authorized Unobserved{reason} edits** in `scenarios/terran_pirate_galaxy.clause`
   (`tp::{hull,weapon_damage,upkeep}` + four `tp::combat_*`). Reasons name the
   uninstantiated host class and Phase 8 (8.1–8.2) as successor.
2. **RESIDENCY = value-placing relations ONLY** (economy emission/transfer/recipe
   `host_entity`, need_binding locus, threshold host, hosted_observation location,
   RF parent edge). Governance overlays / policy-weight authority never elect.
3. Install-time observation-host materialization door in `compile_and_install`
   (after economy/need/events; before admission report). Existing loci win; only
   Anchored properties with zero live loci enter derivation. Fail-closed on
   missing/ambiguous/out-of-tree when value-placing vocabulary is present.
4. Disposition-only / micro installs (no value-placing vocabulary) leave the door inert.

## Exit proof

| check | result |
|---|---|
| Anchored census (value-placing) | `exact=18 / zero=0 / conflict=0` |
| Inventory (`property_admission_inventory.tsv`) | `18 Anchored / 7 Unobserved / total 25` |
| Live loci (TP economy, governance overlays excluded from place) | 18 loci / 18 properties |
| Dark cells | 7 Unobserved (no anchored GPU loci) |
| Economy value preservation | seeded amounts retained |
| Topology | unmodified (no hosting DSL; observation hosts only) |

### Re-derived goldens / expectation updates

| artifact | change | why |
|---|---|---|
| `scripts/ci/property_admission_inventory.tsv` | `25/0 → 18/7` + 7 dark rows | DA-authorized Unobserved conversions |
| `docs/orchestrator_orientation.md` | admission line + inventory stamp | regenerated from inventory |
| `anchor_disposition_admission_0` | expects 18/7; Board dark len=7 | inventory law |
| `anchor_table_surface_0` tripwire | 0→18 live tp_economy loci | 5.3b exit replaces 5.3 zero baseline |
| `canonical_anchor_materialization_0` | STOP lock → PASS lock | residency law + materialization |

### Explicit non-changes

- No 5.3 table schema / WGSL / writer / remap / typed-readback edits
- No Field Sweep 5.4–5.8 / Phase 6
- Active open rung remains `CANONICAL-ANCHOR-MATERIALIZATION-0` (DA advances at graduation)
- RF-1 sums unchanged (observation hosts; layout defaults only)

## Reproduce

```bash
cargo test -p simthing-driver --test canonical_anchor_materialization_0 -- --nocapture
cargo test -p simthing-clausething --test anchor_disposition_admission_0 -- --nocapture
bash scripts/ci/gen_property_admission_inventory.sh --check
```
