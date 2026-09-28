# 2.2 native birth resume: post-birth RF enrollment STOP

**STOP / PROBATION / proof-present / review-pending. Same #2075 OPEN / DRAFT / UNMERGED.**

Coordinated dispatch: Board 5738509297 and 5738507929, runtime release #2079 / DA 5738076985.
One-time exact rebase: `da1d7c1535f8953cde4ac70703a4a5f8313a0734`. Runtime prerequisite: `60830eef4907b2bc9a8bf94325f9b50f8891d528`.
Code head tested by this packet: `2e98c82e07189dea350ec4fd2d430d288f8d4788`; tree `f09ae7babe65a3842b4304ae5a5360c839c08474`.
Seventeenth E8 pin `0x17a0_d148_7d18_f02e` unchanged; all 32 sealed component blobs unchanged.

The ordered conjunction, capped-stock/recovery and native funded-birth gates passed. Both factions
fund at G5 and birth at G6. The next mandatory floor fails: native AddChild products carry an
observable `meridian::energy` flow of -1, but they are absent from the energy RF arena. At G8,
refinery and shipyard still each receive +1 energy, total +2; with one born corvette the required
net is +1. The zero-upkeep control also settles +2. This is a concrete ongoing-economy gap,
not a missing birth or missing observable property. No generic repair was attempted.

## Ordered execution

1. Original conjunction FIRST immediately after rebase: 1 passed / 0 failed / 0 ignored / 0 measured / 3 filtered; 3.54s.
2. Capped stock and generator-withheld/restored cases: 1 / 0 / 0 / 0 / 3; 2.74s.
3. Native funded birth after source authoring corrections: 1 / 0 / 0 / 0 / 3; 1.98s.
4. New post-birth RF diagnostic, two explicit source cases: 0 / 1 / 0 / 0 / 4; 2.87s, exit 101.
5. Committed-code full target: **4 passed / 1 failed / 0 ignored / 0 measured / 0 filtered**, 11.81s, exit 101.

The final documentation head and its verification/hosted artifact bindings are recorded in the PR
body and Board return; these committed files bind the preceding code head without a circular SHA claim.

```text
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_born_energy_upkeep_participates_in_resource_flow -- --exact --nocapture --test-threads=1
cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet
bash scripts/ci/agent_scan.sh --base da1d7c1535f8953cde4ac70703a4a5f8313a0734
```

Code-head check PASS. AGENT-SCAN PASS (48s), hard failures 0, INSPECT 0, TEST-BUDGET PASS.
The scan's embedded selftest is explicitly SKIPPED; it is not claimed executed.

## What the native gate proves

The existing `stellaristhing_base.clause` is copied with unchanged base/dependency JSON, then authored
with the already proven capped economy, ordinary shipyard energy WIP and generic `structural_product`.
Two bounded products per faction use the real corvette scalar funding locus, spatial parent A1/E1,
explicit owner, a detached Fleet root, two Cohort children, and an observable Amount/overlay probe.
The native pack/profile/ordinary session path installs the existing ActionBand crossing door once.

No counted fleet is at N0. All 12 new subtree IDs are absent from the N0 set. Each root has the
correct faction owner, two fresh owned children, one overlay, ChildOf(A1/E1) membership and a committed
three-row placement. N0 is 38 live rows with capacity 76; G40 is 50 live rows with capacity 76.
Terran births at G6/G10 after G5/G9 funding; Pirate births at G6/G11 after G5/G10 funding.
Both recipes debit exactly 6 alloys and 4 energy-work per funded unit, maximum one per generation.

The birth gate isolates bounded structural ingress: seven scalar outputs accrue per faction by G40,
but the declaration authorizes two births per faction. The remaining five outputs per faction are
NOT claimed as completed fleets, refundable WIP, or discharged reservation disposition. The recurring
doubling Amount overlay is a non-economic observation probe, not a finished corvette hull design or
an upkeep substitute. Its G40 value equals 2^(40 - birth_generation), proving post-birth execution.

Refinery scarcity/recovery remains the earlier GREEN exact eight-generation case. It is not a claim
that the separate insufficient-alloy / insufficient-energy-work fleet recovery floor has passed.

## Exact upkeep falsifier and source diagnosis

Two source variants differ only in the Fleet template's energy flow: 0 and -1. Both carry the already
registered `meridian::energy` property, weight 0, balance 0. Each is born under an existing participating
spatial parent, so the normal property-bearing-parent fallback is applicable. Both factions birth at G6;
the G8 check allows two subsequent ordinary generations. Anchored observation confirms the authored
flow on the live born row; the arena registry has no membership for it. No boundary refill, test-side
RF registration, synthetic anchor table, raw-matrix observation bypass, or second energy ledger is used.

Source path (at the runtime release):

- `simthing-driver/src/resource_flow_derivation.rs:237,287`: initial RF participants are derived from
  the populated N0 tree. Lines 319-326 use a participating physical parent when no resource edge is explicit.
- `simthing-driver/src/structural_product.rs:359`: the detached instance has authored properties,
  owner and children, and lowers through StructuralAuthorization/AddChild; it has no N0 RF participant.
- `simthing-driver/src/session.rs:2285-2289` and `:2473`: ordinary boundary dispatch reacts to fission
  enrollment. `:2629-2653` returns early when `fission_pairs` is empty.
- `simthing-driver/src/resource_flow_fission_enrollment.rs:40-133`: runtime admission consumes only
  fission parent/child pairs. The code has no analogous native AddChild product enrollment consumer.
- `simthing-driver/src/arena_registry.rs:250`: `refresh_subtree` only visits existing participants;
  it cannot discover a newly born participant. Calling `admit_participant_runtime` directly from this
  test would add authority outside the ordinary source-to-session path and is not a solution here.

The needed decision belongs to Orchestration/DA: identify an existing ordinary admission path missed
by this consumer, or admit the generic post-birth RF enrollment seam with its laws and first consumer.
No fleet-specific executor or Model 2 opening is proposed. Even fixing energy enrollment alone would
not prove the full 1 energy + 0.2 alloys upkeep, funding disposition, or continuation obligations.

## Corrections made while developing the probe

The initial future-only hull property had no N0 value-placing candidate. A zero-valued existing site
admits the shared column without an N0 fleet or economic stock. A custom-only `durability` role had
no primary Amount/Velocity anchor in `anchor_remap_encode.rs:144-156`; the probe uses the ordinary
Amount role. The recurring multiplier was initially misasserted as a one-time multiplier; its exact
per-generation expectation was corrected. These were source/test corrections, not substrate repairs,
and are not the final STOP reason. The final exact source variants are printed in the raw log.

## Scope and unfinished obligations

All six older evidence files remain byte-identical. Old STOP head `ec04a6352ed64ad10c0e71a794cbf22f9a5ec922`
is preserved remotely as `codex/0088-economy-fleet-stop-ec04a635` in addition to the two earlier backups.
The canonical scenario is unchanged (SHA256 `5821be96f0bfb439ce8717dba7dd280e994ae6a1113bff42d3137766031df328`).
Same-asset source variants are the current executable probes; canonical full-specimen integration is
not claimed complete. No parser/spec/session/engine/kernel/sealed file, gate, class, or doctrine edit.

STOP leaves these full-rung obligations unproved: canonical bounded storage/specimen integration;
fleet-specific insufficient-alloy and energy-work recovery; full funding/reservation disposition on
refusal/cancellation; 1 energy + 0.2 alloys upkeep and resulting stock reconciliation; complete
source/declaration/physical-order equivalence; identity/replay/fail-stop negatives; post-birth save/restore.
No 2.3 work or Model 2. Passing the three resumed gates does not establish PASS 2.2.

Bonsai's exclusive `native_structural_products_session_0.rs` is unchanged. Sequential capacity
exhaustion was not duplicated. At this return preparation, no Bonsai patch/evidence has arrived and
its published branch remains at `60830eef...`; review/integration/rerun and trial evaluation remain
pending. Board 5738619274 makes this a one-rung two-lane exception, expiring after both obligations
are discharged. No third lane was opened. This Astra STOP does not evaluate an unreturned patch.

## Runtime trace excerpts

```text
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=214 id=262
UPKEEP_BIRTH case=zero-upkeep-control generation=6 parent=220 id=268
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=zero-upkeep-control generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=262 parent=214 slot=Some(SlotIndex(38)) authored_observed_flow=0 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=zero-upkeep-control born=268 parent=220 slot=Some(SlotIndex(41)) authored_observed_flow=0 energy_arena=1 members=[]
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=1 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=2 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=3 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=4 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=5 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=38
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=281 id=329
UPKEEP_BIRTH case=one-energy-upkeep generation=6 parent=287 id=335
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=6 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=7 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=terran refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_SETTLEMENT case=one-energy-upkeep generation=8 owner=pirate refinery_rate=1 yard_rate=1 surplus=2 live=44
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=329 parent=281 slot=Some(SlotIndex(38)) authored_observed_flow=-1 energy_arena=1 members=[]
UPKEEP_MEMBERSHIP case=one-energy-upkeep born=335 parent=287 slot=Some(SlotIndex(41)) authored_observed_flow=-1 energy_arena=1 members=[]
NATIVE_BIRTH generation=6 parent=552 id=600 faction=0
NATIVE_BIRTH generation=6 parent=558 id=606 faction=1
NATIVE_BIRTH generation=10 parent=552 id=603 faction=0
NATIVE_BIRTH generation=11 parent=558 id=609 faction=1
BIRTH_SHAPE owner=terran id=600 children=[SimThingId(601), SimThingId(602)] hull=17179870000 extent=3 parent=552 generation=6
BIRTH_SHAPE owner=terran id=603 children=[SimThingId(604), SimThingId(605)] hull=1073741800 extent=3 parent=552 generation=10
BIRTH_SHAPE owner=pirate id=606 children=[SimThingId(607), SimThingId(608)] hull=17179870000 extent=3 parent=558 generation=6
BIRTH_SHAPE owner=pirate id=609 children=[SimThingId(610), SimThingId(611)] hull=536870900 extent=3 parent=558 generation=11
NATIVE_BIRTH_PASS first_funding=[Some(5), Some(5)] funded_total=[7.0, 7.0] n0_ids={545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 572, 573, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597} fresh_ids=[600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611] capacity=76
```

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb
