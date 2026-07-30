# CANONICAL-ANCHOR-MATERIALIZATION-0 — implementation results (PROBATION)

- Track: 0.0.8.7 RF arena modernization (rung 5.3b)
- Status: **PROBATION** — exit proofs green locally; DA alone graduates / advances pointer
- HD-RECEIPT: `dfacf5e8bb04` (supersedes `63f01c28e4df`)
- ORIENT-RECEIPT: `8555a6c8ed5c`
- orientation_rule_stamp: `428229b9129d598e`
- Remand: orch `5124550917` · DA totality ruling `5124532506` · amendment #1501 @ `a8040de7`
- Prior dispatch: orch `5124117080` · DA HOLD lift `5124095512`

## Binding law applied (TOTALITY)

1. **Seven DA-authorized Unobserved{reason} edits** in `scenarios/terran_pirate_galaxy.clause`
   (`tp::{hull,weapon_damage,upkeep}` + four `tp::combat_*`). Reasons name the
   uninstantiated host class and Phase 8 (8.1–8.2) as successor. Only authoring.
2. **RESIDENCY = value-placing relations ONLY** (economy emission/transfer/recipe
   `host_entity`, need_binding locus, threshold host, **hosted_observation location**,
   RF parent edge). Governance overlays / policy-weight authority never elect.
3. Install-time observation-host materialization door in `compile_and_install`
   (after economy/need/events; before admission report). Existing loci win —
   including overlay effect-host stores (lawful 2.1 admission residency). Only
   Anchored properties with zero live PropertyValue stores enter the derivation
   set. Fail-closed on missing/ambiguous/out-of-tree when value-placing
   vocabulary is present.
4. **Ordinary unmutated install** (domain packs + overlays ENABLED). No
   proof-side `retain` / `overlays.clear` / equivalent corpus mutation.
5. Prove **totality, not 1:1 cardinality**: inventory `18/7`; zero Anchored with
   zero live PropertyValue stores; one GPU identity coverage per
   observation-table locus; no repeated `(thing, property)`; multi-host
   residency (incl. overlay effect-hosts) accepted.

## Exit proof

| check | result |
|---|---|
| Hydrate value-placing census (Anchored) | `exact=18 / zero=0 / conflict=0` |
| Derivation set after ordinary install | empty (`\|set\|=0`; `zero=0 / conflict=0`) |
| Inventory (`property_admission_inventory.tsv`) | `18 Anchored / 7 Unobserved / total 25` |
| Tree totality (all Anchored have ≥1 PropertyValue) | PASS (incl. `studio_live_rf::owner_flow` Named RF stores) |
| Observation-table coverage (`tp_economy`) | 18 properties; multi-host lawful |
| Dark cells | 7 Unobserved (absent from observation locus map) |
| Economy value preservation | seeded amounts retained |
| Topology / RF-1 / 5.3 table semantics | unmodified |

### Re-derived goldens / expectation updates

| artifact | change | why |
|---|---|---|
| `scripts/ci/property_admission_inventory.tsv` | `25/0 → 18/7` + 7 dark rows | DA-authorized Unobserved conversions |
| `docs/orchestrator_orientation.md` | admission line + compact dark list | inventory + DOC-BUDGET prose-growth repair |
| `scripts/ci/gen_orientation.sh` | dark props → one-line list | keep orientation ≤ DOC-BUDGET cap |
| `anchor_disposition_admission_0` | expects 18/7; Board dark len=7 | inventory law |
| `anchor_table_surface_0` tripwire | ordinary install totality (no overlay strip) | remand TOTALITY law |
| `canonical_anchor_materialization_0` | cardinality → totality + derivation-set | HD `dfacf5e8bb04` |
| `hydrate_scenario` fleet_ship inject | only when `fleet_ship_payloads` non-empty | avoid false Anchored `tp::{hull,weapon,upkeep}` debt on field-economy-only opens |
| `install.rs` materialization scope | authored game-mode / candidate keys only | skip registry-only `_studio_live_bridge::seed` placeholders |

### Explicit non-changes

- No 5.3 table schema / WGSL / writer / remap / typed-readback edits
- No Field Sweep 5.4–5.8 / Phase 6
- Active open rung remains `CANONICAL-ANCHOR-MATERIALIZATION-0` (DA advances at graduation)
- RF-1 sums unchanged (observation hosts; layout defaults only)

## Reproduce

```bash
cargo test -p simthing-driver --test canonical_anchor_materialization_0 -- --nocapture
cargo test -p simthing-driver --test anchor_table_surface_0 -- --nocapture
cargo test -p simthing-clausething --test anchor_disposition_admission_0 -- --nocapture
bash scripts/ci/gen_property_admission_inventory.sh --check
bash scripts/ci/gen_orientation.sh --check
```
