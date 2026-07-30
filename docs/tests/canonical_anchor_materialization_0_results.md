# CANONICAL-ANCHOR-MATERIALIZATION-0 — implementation results (PROBATION)

- Track: 0.0.8.7 RF arena modernization (rung 5.3b)
- Status: **COMPLETE — DA-GRADUATED / merged #1500 @ 1294cc87** (DA deep-tree: totality proven on the ordinary unmutated install; full corpus reproduced on RTX 4080/Vulkan; one pre-existing sim failure carved out to rung 5.3c)
- HD-RECEIPT: `2fdb701f35e1` (supersedes `dfacf5e8bb04`, `a9b5482b9e2e`, …)
- ORIENT-RECEIPT: `8555a6c8ed5c`
- orientation_rule_stamp: `428229b9129d598e`
- Remand 2: orch `5124965247` · DA HOLD lift `5124943291` · corrected master #1503 @ `ed7e6264`
- Remand 3: orch `5125114823` — protected test identities restored as wrappers (no production delta)

## Binding law (admission governs existence)

1. A resource-bearing dimension is registered IFF an authored structure admits it.
2. Every **admitted Anchored** property must have ≥1 lawful live host after admitted
   structures + typed admission/disposition repairs. No registry carve-outs.
3. Genuinely hostless admitted properties declare authored `Unobserved{reason}`
   (5.1 door). That set is **derived from the corpus**, never a fixed count target.
4. RESIDENCY = value-placing relations ONLY. Hosted observation reaches production
   only as the lowered presence-emission’s typed `host_entity` (no id/name substring
   provenance classing).
5. Ordinary unmutated install (domain packs + overlays ENABLED) proves **totality**.

## Derived inventory (published, not targeted)

Regenerated from the live properties-only canonical TP install
(`scripts/ci/property_admission_inventory.tsv`):

| metric | derived value |
|---|---|
| Anchored | 18 |
| Unobserved | 7 |
| total resource properties | 25 |

Ordinary unmutated field-bearing install (domain packs + overlays enabled) additionally
publishes live tree counts in the referee (`Anchored` live identities / observation locus
rows / multi-host props). Those counts are observations, not fixed targets.

Canonical TP authored `Unobserved{reason}` rows (enumerated evidence, not a global cap):
`tp::{hull,weapon_damage,upkeep}` + four `tp::combat_*` — Phase 8 successor reasons.
`_studio_live_bridge::seed` is synthesis-site `Unobserved` (GPU column-shape placeholder).

## Exit proof

| check | result |
|---|---|
| Hydrate value-placing census (Anchored) | `exact>0 / zero=0 / conflict=0` (published in referee) |
| Derivation set after ordinary install | empty (`zero=0 / conflict=0`) |
| Tree totality | every Anchored has ≥1 live PropertyValue |
| Observation-table coverage | every Anchored `tp_economy` identity; multi-host lawful |
| Dark cells | absent from observation locus map |
| Economy value preservation | seeded amounts retained |
| Topology / RF-1 / 5.3 table semantics | unmodified |

## Typed repairs (ruling 5124757579)

| repair | site | why |
|---|---|---|
| Payload-conditional fleet_ship inject | `hydrate_scenario.rs` | only when `fleet_ship_payloads` non-empty |
| Synthesis-site Unobserved for bridge seed | `studio_live_session_bridge.rs` | GPU column-shape placeholder never Anchored hostless |
| No derivation carve-out | `install.rs` | all active admitted Anchored enter derivation |
| No `presence_emission` substring relabel | `install.rs` | typed emission `host_entity` only |

### Re-derived goldens / expectation updates

| artifact | change | why |
|---|---|---|
| `property_admission_inventory.tsv` | regenerated; counts published | admission law / corpus |
| `docs/orchestrator_orientation.md` | regenerated | inventory + design stamp |
| `test_inventory.tsv` | restore 3 protected identities; ledger 3 additional materialization tests | TRACK-CLOSEOUT deletion-guard + TEST-INVENTORY-DRIFT |
| 5.3b referees | drop fixed 18/7 asserts; protected `#[test]` names wrap corrected helpers | no fixed-count targets; identities preserved |

## Reproduce

```bash
cargo test -p simthing-driver --test canonical_anchor_materialization_0 -- --nocapture
cargo test -p simthing-driver --test anchor_table_surface_0 -- --nocapture
cargo test -p simthing-clausething --test anchor_disposition_admission_0 -- --nocapture
bash scripts/ci/gen_property_admission_inventory.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/doc_budget_check.sh --check
```
