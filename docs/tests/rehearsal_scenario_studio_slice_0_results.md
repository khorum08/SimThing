# 0088 Studio slice, Leaf A: authored Meridian Arm

PROBATION / proof-present / OPEN / UNMERGED. Return to Orchestration only.
This packet covers the authored-scenario leaf of `0088-STUDIO-SLICE-0`.
It is not the full rung's GUI, continuation, or timing exit proof.

Dispatch/resume: Board 5646339090; DA authority 5646203717.
Canonical HD: `handoffs/0088-STUDIO-SLICE-0.hd.md`.
HD-RECEIPT: a0bbed366a57. ORIENT-RECEIPT: 28f56884d309.
All 48 projected anchors ACKed at Board 5646388683. ClauseScript archaeology
5646311926 was carried into this implementation.
Base: `c99e2ec3253ef9e5651276d33281f37dddc88695`.
The exact tested head, hosted scan artifact and fresh clearance travel in the
PR/Board return; no commit claims its own hash.

## Authored bundle and relations

The bundle is `scenarios/stellaristhing_base.clause`, its sibling
`stellaristhing_base.base.json`, and `stellaristhing_base.dependencies.json`.
The JSON supplies only the imported structural map through the existing
`static_galaxy_scenario` door. Its provenance says authored structural data,
with seed zero, rather than claiming a MapGenerator run. The structural
format's required placeholder Owner and payload children are not imported as
economic actors. Factions, properties, policy, RF edges and recipes come from
the native source.

Seven occupied cells in a 2-by-6 frame form the connected Meridian Arm:
A(0,0), B(0,1), C(0,2), D(0,3), E(0,4), F(1,4), G(0,5), with links
A-B, B-C, C-D, D-E, E-F and E-G. Native rebind reports seven placements,
six links, no link residue, and STEAD validation PASS. These map links are
structural adjacency; they do not automatically transfer resources.

| Relation | Terran Directorate | Pirate Compact |
| --- | --- | --- |
| Spatial declaration | A1 uses `system_target = row0_col0`, system ID 1 | E1 uses `system_target = row0_col4`, system ID 5 |
| Physical tree | A1 has four Cohort children | E1 has four Cohort children |
| Energy RF ancestry | four children -> A1 -> terran -> session root | four children -> E1 -> pirate -> session root |
| Ownership | A1 and all four children resolve to terran | E1 and all four children resolve to pirate |
| Owner policy | allocator-weight multiplier 1 | allocator-weight multiplier 1.5 |
| Local material coupling | A1 minerals -> A1 alloys recipe | E1 minerals -> E1 alloys recipe |

The admitted runtime tree has the two Owners, galaxy map, A1 and E1 as
GameSession children. A1/E1 are joined to their system coordinates through
STEAD; this packet does not claim they are physical children of the imported
star nodes. Both Owner seats have zero physical children. Site children retain
their site's ownership. RF parentage is separately authored and derived at
admission: one arena, 13 participants, the two owner branches above, no extra
resource coupling between factions. The common RF root has zero injection.

## Executable economy and bounded observation

Each site has two energy generators at intrinsic flow +2 and two facility
upkeep flows at -1, with weight 1 on each child. Each native child allocation
is 0 at opening and 0.5 at generations 1 through 6. The site's +4 supply and
-2 upkeep leave +2 distributed across its four children. This is the current
signed RF path, not a multi-input energy/mineral production recipe. Owner
energy Balance opens at 10/8 and stays there in this observation; no withdrawal
or stored-energy surplus is claimed. Owner policy attachments are present;
this zero-root-injection workload does not demonstrate competition between
factions or a nonzero owner policy effect.

Each mineral source uses the admitted `field_resource_quantity` form at 3.
Its constant emission establishes opening quantity 3; its infrastructure
overlay supplies the live path. The local recipe consumes 2 minerals per
output unit and produces coefficient 1 of alloys. Alloys open at 4/3.
The admitted recipe path emits affordable exact units: its authored
`throttle_hint_max_per_tick = 1` is not a hard one-batch cap. This is explicit
in `simthing-kernel/src/transfer_accumulator.rs`'s
`conjunctive_recipe_registration_to_transfer` contract and in these results.

Observed through `observe_hosted_property_cell` on the real session:

| Completed generations | A1 minerals | A1 alloys | E1 minerals | E1 alloys |
| --- | --- | --- | --- | --- |
| 0 | 3 | 4 | 3 | 3 |
| 1 | 4 | 5 | 4 | 4 |
| 2 | 3 | 7 | 3 | 6 |
| 3 | 4 | 8 | 4 | 7 |
| 4 | 3 | 10 | 3 | 9 |
| 5 | 4 | 11 | 4 | 10 |
| 6 | 3 | 13 | 3 | 12 |

Both sites sustain mineral input and add nine alloys over six generations.
This is the bounded productive-surplus demonstration for Leaf A. It does
not prove fleet construction, finite capacity, scarcity recovery, or energy
funding of the material recipe; those require their later authored contracts.

Two disposable source copies provided attribution through the same loader
and session APIs, without modifying the shipped source or existing fixtures:

- `@mine_rate = 0`: at generation 3 minerals remain zero and alloys remain
  at their opening 4/3, instead of growing to 8/7.
- `@facility_upkeep = 0`: generation-3 allocations of the mine and refinery
  children become 1 each, versus 0.5 with authored upkeep.

These observations exercise existing authorities; no economic reference
implementation or new test referee was added.

## Portability and identity

The source uses one sibling-relative `source_json` path and no resolver
entries. Existing `.gitattributes` preserves LF on checkout for all three
files. Native loading checks the sibling dependency manifest.

| Input | Native content identity |
| --- | --- |
| stellaristhing_base.clause | `fnv1a64:ee4e4df9e8c9fbd9:5798` |
| stellaristhing_base.base.json | `fnv1a64:c49f9ca3c8c75e77:20370` |
| stellaristhing_base.dependencies.json | `fnv1a64:2f064bfb3e043aa0:72` |

Copying all three files to a different directory and opening with default
ingest options preserved all three identities, the empty resolver map,
relations, and every recorded generation-0-through-6 observation. Each open
ran in a fresh process. Neither absolute source paths nor resolver tokens
are stored in the authored bundle.

**Existing profile-digest limitation, returned for Orchestration visibility:**
the native profile identities from those two opens were respectively
`fnv1a64:735cf4b0078d87b9:21477` and `fnv1a64:def2ee608b11e06f:21477`.
The serialized game modes and trees differed only in the ordering of tree
property pairs. `SimThing.properties` is a HashMap serialized as a pair list;
`authored_profile_content_identity` sorts install targets but serializes
the tree without canonicalizing those pairs. Source/dependency identities
and economic observations are stable; cross-process profile-byte identity
is not demonstrated. No substitute digest, serializer change or persistence
workaround was introduced. Later profile-pinned measurement/continuation
work must account for this finding before making stronger provenance claims.

## Existing production doors and local verification

A disposable local inspection adapter called these existing public APIs;
the adapter only loaded, stepped, printed structures and requested hosted
observations. No adapter or generated cache is installed into the repository.

1. `load_clause_studio_session_from_path(source, &ClauseScenarioIngestOptions::default(), cache, None)`:
   native parse -> default expansion -> source-relative hydrate ->
   StructuralRebindReady -> StudioSession with authored live profile.
2. Print the ingest report/cache identities, profile GameMode, install targets,
   location-system join, and runtime tree. Print the existing profile's Debug
   provenance identity, without recomputing it.
3. `StudioLiveSessionBridge::default().open_from_loaded_studio_session(&session)`:
   field-bearing `SimSession::open_from_spec` admission, then inspect
   `spec_state.resource_flow_derivation`, `proto.root.owner_of` and `child_count`.
4. At opening and after each `consume_scheduled_ticks(1)`, obtain
   `AnchorTableSnapshot::from_session(sim)` and call
   `observe_hosted_property_cell(registry, allocator, snapshot, target_id,
   PropertyKey, SubFieldRole)`. Material roles are Amount; energy roles are
   Amount and Named("balance"). The table uses A1/E1's install-target IDs.
5. Repeat with the relocated three-file bundle and the two parameter variants
   above, each in its own directory/process. Values are observations, not
   values recomputed by the adapter.

Windows local build and existing focused batteries passed:

```text
cargo build -p simthing-mapeditor --lib --message-format=json
cargo test -p simthing-clausething --test field_economy_grammar_0 --test rehearsal_ingress_records --test ct_0b_raw_model --test ct_0c_expansion --test ct_0d_scope --test ct_scenario_container
cargo test -p simthing-mapeditor --test rehearsal_ingress_native_rf --test rehearsal_ingress_source_cache --test rehearsal_ingress_literal_successor --test rehearsal_ingress_fidelity
```

ClauseThing: 14 passed, zero failed. Mapeditor: 9 passed, zero failed.
These retain raw/expansion/scope contracts, field-economy negative authoring,
literal successor behavior, source/dependency refusals, relocation, native
RF and source-cache parity. No test source, fixture, inventory row, lease,
or existing report was changed or renewed. The three authored files are
external assets; this packet is the sole new bounded evidence record.

Scope: `rehearsal-authored-scenario` only; novelty_claim: NO.
GUI picker operation, failure atomicity for this new slice, resident
continuation and Studio timing remain the later dispatched leaves. Hosted
Doctrine Scan and final-head Clearance results are reported on the PR and
Board, with every INSPECT returned to Orchestration for disposition.
