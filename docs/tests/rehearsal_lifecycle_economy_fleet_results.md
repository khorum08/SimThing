# 2.2 economy-to-fleet: STOP at native refinery input lowering

**STOP / PROBATION / proof-present / review-pending. OPEN / UNMERGED.**
The frozen two-input refinery cannot be carried faithfully through the current
native `production_building` ingestion door. Accepted source silently keeps
only the last `input`, so changing field order changes the economic contract.
No 2.2 PASS, fleet completion, or graduation is claimed. Model 2 stays CLOSED.

Authority: [handoff 5725574222](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5725574222),
[landed DA release 5725499644](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5725499644).
Exact base: `34cf26df39cfd39b0f2a183f489604271f9253c0`.
Implementation/test head: `9cc68e55a4ee168beca1df2a4629a14c9b6bfe39`.
The final evidence-bearing head, hosted artifacts and fresh clearance are bound
in the PR body and Board return. One coding lane; no subagents or 2.3 work.

## Reproducer through the existing authored asset

The owning test is
`crates/simthing-workshop/tests/rehearsal_lifecycle_economy_fleet.rs`.
It reads the existing `scenarios/stellaristhing_base.clause`; both original
dependencies are copied byte-for-byte into temporary source bundles. The
canonical asset/dependency files remain untouched. There is no substitute
scenario, programmatic economy, edited hydrated profile, or runtime patch.

For both existing faction refinery declarations, the only source change in
the failing probe is to add the required energy input beside the mineral input:

```clause
input = { resource = minerals amount = @refinery_input }
input = { resource = energy amount = 1 }
```

The test repeats with those two fields reversed. Both sources go through the
production `ingest_clause_scenario_path` parse/expand/hydrate/structural-rebind
pipeline. It collects both faction outcomes before asserting the conjunction.

| Accepted field order | Terran hydrated input | Pirate hydrated input | Missing cost |
|---|---|---|---|
| minerals, energy | A1_energy_quantity, cost 1 | E1_energy_quantity, cost 1 | minerals 2 |
| energy, minerals | A1_minerals_quantity, cost 2 | E1_minerals_quantity, cost 2 | energy 1 |

All four accepted recipes have **one input**, not two. The second test is
deliberately still RED on the frozen obligation; the defect is not converted
into an expected-success regression test. This is an ingestion failure before
economic execution, not a claim that the existing generic recipe engine
cannot perform conjunction.

## Exact source diagnosis and alternative-door audit

- `simthing-clausething/src/hydrate_field_economy.rs:63` represents a production
  building with singular `input_resource` / `input_amount`.
- Its parser starts `let mut input = None` at line 436; line 444 assigns `Some`
  on every `input`, silently overwriting the earlier one. It does not reject
  the duplicate or preserve a list.
- The lowering at line 1396 constructs a singleton `Vec<RecipeInputSpec>`.
  Downstream `simthing-spec/src/spec/resource_economy.rs:60` already has the
  generic `inputs: Vec<RecipeInputSpec>` door proven by 2.1.
- Input parsing at line 833 accepts only resource/amount; line 1748 derives
  a location-local quantity property in the field-economy namespace. The
  diagnostic energy input therefore means `meridian_material::A1_energy_quantity`
  or `E1_energy_quantity`, **not** the existing native `meridian::energy`
  balance. No alias or second energy ledger is introduced to pretend otherwise.
- The other multi-input field-economy shape, `flow_coupling`, is a three-input
  suppression recipe: production/quantity source + disruption-presence pressure
  + owner-stockpile weight, each with strictly positive cost. Its validators
  at lines 1198–1264 require those concrete loci. It cannot be substituted for
  this two-input refinery without an extra consumed input / renamed authority.
- The scenario parser only attaches the existing field-economy lowering to
  `GameModeSpec.resource_economy`; its root match refuses unsupported fields.
  The programmatic-spec door remains canonical, but constructing a replacement
  recipe/model in this test would leave the mandated native source-of-truth path.

**Required return:** Orchestration must admit the generic native recipe-input
mapping needed to preserve multiple authored costs and bind their real
property/role/host loci, or identify another already-admitted native form that
actually preserves this frozen law. No parser, lowering, spec, engine, sealed
component, gate or class repair is attempted in this O* leaf. This report does
not prescribe a new allocator, construction ledger, or Model 2 capability.

## Ordinary execution control and generation traces

The first test opens the unchanged asset using the same public profile builder
and `SimSession::open_from_spec` path used by Studio, then executes G1–G3.
It repeats with only authored generator rate and both starting energy balances
set to zero. Observations use `AnchorTableSnapshot` and
`observe_hosted_property_cell`; no readback supplies economic authority.

| Case | G | TD energy balance | PC energy balance | A1 minerals | A1 alloys | E1 minerals | E1 alloys |
|---|---:|---:|---:|---:|---:|---:|---:|
| canonical | 0 | 10 | 8 | 3 | 4 | 3 | 3 |
| canonical | 1 | 10 | 8 | 4 | 5 | 4 | 4 |
| canonical | 2 | 10 | 8 | 3 | 7 | 3 | 6 |
| canonical | 3 | 10 | 8 | 4 | 8 | 4 | 7 |
| energy withheld | 0 | 0 | 0 | 3 | 4 | 3 | 3 |
| energy withheld | 1 | 0 | 0 | 4 | 5 | 4 | 4 |
| energy withheld | 2 | 0 | 0 | 3 | 7 | 3 | 6 |
| energy withheld | 3 | 0 | 0 | 4 | 8 | 4 | 7 |

Each session reports 36 live rows / allocator capacity 36 throughout. This is
the allocator's current row census, **not a proof of a paid fleet slot or free
birth capacity**. Canonical initial hosts are TD207/slot2, PC208/slot3,
A1=213/slot26, E1=218/slot31 in the recorded run; all initial tree IDs and
execution realms are in the raw packet. No product identity is minted or
counted as a new fleet by this probe.

This is an honest baseline of the existing first Studio slice, not an assertion
that it already meets 2.2. Its N0 mineral loci are 3/3 rather than the required
20/14, its material recipe lacks an energy cost, and its throttle is explicitly
a hint, not a hard one-batch-per-generation cap. Alloy production with no energy
is supporting diagnosis of the current slice; the actual generic STOP is the
accepted two-input source losing a cost. No artificial scarcity recovery,
placement, birth or upkeep proof is inferred from these observations.

## Content binding

All entries below are emitted by the existing native source identity function.

| Source variant | Source identity | Executable GameMode projection identity |
|---|---|---|
| canonical | fnv1a64:ee4e4df9e8c9fbd9:5798 | fnv1a64:beaa4408e5b3f4aa:8892 |
| energy withheld | fnv1a64:874543addf5480a4:5797 | fnv1a64:beaa4408e5b3f4aa:8892 |
| minerals then energy | fnv1a64:d7198602c73a6ee7:5892 | fnv1a64:d44ccbe88ade842a:10760 |
| energy then minerals | fnv1a64:ae8a448726a37b6b:5892 | fnv1a64:beaa4408e5b3f4aa:8892 |

The GameMode projection digest does **not** hash the complete runtime profile;
intrinsic values live in its separate session tree. Its equality between the
first two cases is not a workload-equivalence claim. Exact source identities,
unchanged dependency identities and per-run host/slot/realm traces bind each
executed probe. A complete 2.2 born-fleet profile pin is not claimed.

Canonical source SHA-256: `5821be96f0bfb439ce8717dba7dd280e994ae6a1113bff42d3137766031df328`;
Git blob `ac16cd05daf8e793c9f33dd2003ca2b45a1ac64a`.
Declared dependencies: base JSON `fnv1a64:c49f9ca3c8c75e77:20370`,
manifest `fnv1a64:2f064bfb3e043aa0:72`.

## Proof status, fences and unexecuted obligations

```text
cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet
bash scripts/ci/agent_scan.sh --base 34cf26df39cfd39b0f2a183f489604271f9253c0
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1
```

Check PASS (30.80s). Agent scan PASS (30s), hard failures 0, INSPECT 0,
TEST-BUDGET PASS. Focused execution **RED: 1 passed /1 failed /0 ignored /
0 measured /0 filtered**, exit 101, 5.09s. Full execution is retained in
[raw results](rehearsal_lifecycle_economy_fleet_raw_results.md).
Hosted Scan/Exec and final-head local repetition are reported separately in
the PR/Board; hosted smoke cannot override the RED contract test.

Only an owning test, two ordinary AUDIT/delete-at-closeout inventory rows and
these evidence files are added. All 32 sealed component blobs, both fourteenth
E8 literals, all eight standing qualification tests, canonical asset, existing
tests, gate/class/anchor/workplan and production sources stay unchanged. No E8
roll is triggered; the standing battery is retained, not replaced by ingress.
The six prior DEAD-EXPORT advisories remain unchanged/outside this probe.

The complete exit floor remains **NOT PROVEN**: bounded storage/endowments,
full three-resource conjunction, both-faction funded birth by G40, fresh fleet
identity/bindings, paid capacity/exhaustion, insufficient alloy/energy-work
recovery, post-birth upkeep, post-birth save/restore, ordering invariance,
stale/foreign binding and touched-generation retry negatives. Work stops at
the concrete earlier input-ingress gap rather than fabricating later proofs.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb

Receipt carried: rule-source blobs unchanged from prior oriented session.
Direct Board dispatch; no HD receipt invented.
