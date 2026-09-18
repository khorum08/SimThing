# 2.2 resume: conjunction GREEN; refinery-rate STOP

**STOP / PROBATION / proof-present / review-pending. #2075 OPEN / UNMERGED.**
The original input-loss gap is discharged. The next concrete gap is the frozen
refinery rate: a single admitted refinery consumes every affordable batch, not
at most one batch per generation. No production repair or 2.2 PASS is claimed.

Authority: Board [5735376354](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5735376354)
and release [5735380927](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5735380927),
following DA [5735025720](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5735025720).
Same branch/PR rebased once onto exact `219ca81361dc73a15b06bf93d47697b4e42d198e`.
Historical STOP head `366d7b60314f6d53ffe7afa02e621404b80f4286` is retained at
`codex/0088-economy-fleet-stop-366d7b60`; the two original results files remain
byte-unchanged historical evidence, not current verdicts. Code proof head:
`c88d1b1ba9a9afd17fff355e337e00ee1491cfb8`. Final head/tree, final local repetition,
hosted artifacts and fresh clearance are bound in the PR body and Board return.

## Conjunction-first gate

Before continuing the economy probe, the original referee was updated to the
released canonical energy locus and executed alone:

```text
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet rehearsal_economy_fleet_refinery_retains_every_authored_cost -- --exact --nocapture --test-threads=1
1 passed / 0 failed / 0 ignored / 0 measured / 1 filtered; 8.01s; PASS
```

Both factions retain exactly two inputs: local minerals cost 2 plus their
existing owner's `meridian::energy` balance cost 1. Both recipes admit together
in band 0 and execute in one ordinary `SimSession`. Both authored field orders
have equal economics (only provenance spans normalized) and identical stock
trajectories: owner energy 10/8 becomes 9/7, alloys 4/3 becomes 5/4. This is the
released witness, not the final choice of spendable energy locus.

## Same-asset composition and source authority

`generator_stock_source` extends the existing `stellaristhing_base.clause` in
temporary bundles with its two dependencies copied unchanged. No replacement
programmatic scenario, modified hydrated profile, UI feedback or runtime stock
patch is used. The canonical checked-in asset remains untouched while this
precondition probe is RED.

- Energy: the existing refinery cohort's `meridian::energy / balance` is the
  faction's spendable stock, explicitly owner-bound. The 10/8 endowments move
  from the static owner balances to these cohorts; owner balances are zero.
- The two existing generator cohorts per faction still produce 2 energy each;
  the two existing facility flows still cost 1 each. Ordinary RF delivers the
  net 2 to the refinery cohort by authored sibling weights (others zero, stock
  host one). No owner income is added. Other cohort/site/owner energy balances
  are asserted zero every generation; there is one spendable energy stock.
- Minerals: the existing mine cohort uses an ordinary native RF property,
  `meridian::minerals`, with flow 3 and an explicitly owned Balance initialized
  to 20/14. The original quantity emission is removed, not duplicated. This
  separates the mine's per-generation production from its N0 stock.
- Alloys remain the existing A1/E1 material quantity, initialized to 4/3.
  Each refinery has only the two frozen costs, unit output coefficient, and
  `throttle_hint_max_per_tick = 1`. There are no extra emissions, stock-staging
  transfers, production tokens or fabricated work inputs in this composition.
- Spatial topology remains the shipped tree. Resource-parent edges and owner
  references are authored separately; the stock cohorts remain under A1/E1,
  not spatially reparented beneath Owner seats.

An initial attempt to place 20/14 on the original Constant-emission mineral
cells was overwritten to 3/3 by `install.rs:621–624` (Constant seed at 1007–1010).
That is not this return's blocker: the existing RF Balance composition above
preserves the exact endowments and mine rate without production changes.

## Executed accounting and failure

The test asserts these identities at every generation for both factions:

`energy_after = energy_before + settled_energy - alloys_produced`

`minerals_after = minerals_before + 3 - 2 * alloys_produced`

Energy never goes negative in these cases, and all non-stock energy balances
stay zero. Recipe consumption and generator replenishment reconcile, but the
number of refinery batches violates the frozen rate.

Tuple columns are `(minerals, energy, alloys)`:

| G | Terran | Pirate | New alloys TD / PC |
|---|---|---|---|
| 0 | (20,10,4) | (14,8,3) | — |
| 1 | (3,2,14) | (3,3,10) | **10 / 7** |
| 2 | (4,3,15) | (4,4,11) | 1 / 1 |
| 3 | (3,3,17) | (3,4,13) | **2 / 2** |
| 4 | (4,4,18) | (4,5,14) | 1 / 1 |
| 5 | (3,4,20) | (3,5,16) | **2 / 2** |
| 6 | (4,5,21) | (4,6,17) | 1 / 1 |

Scarcity variation: both energy endowments withheld (0); each generator starts
at flow 1, so total production 2 covers the unchanged two facility costs and
leaves zero spendable surplus. After G3 the ordinary boundary overlay door sets
the four actual generators back to flow 2, with source generation 3. No owner
income, stock refill, or observation-conditioned decision is introduced.

| G | Terran | Pirate | Settled energy TD / PC | New alloys TD / PC |
|---|---|---|---|---|
| 0 | (20,0,4) | (14,0,3) | — | — |
| 1 | (23,0,4) | (17,0,3) | 0 / 0 | 0 / 0 |
| 2 | (26,0,4) | (20,0,3) | 0 / 0 | 0 / 0 |
| 3 | (29,0,4) | (23,0,3) | 0 / 0 | 0 / 0 |
| 4 | (32,0,4) | (26,0,3) | 0 / 0 | 0 / 0 |
| 5 | (35,2,4) | (29,2,3) | 2 / 2 | 0 / 0 |
| 6 | (34,2,6) | (28,2,5) | 2 / 2 | **2 / 2** |

The ordinary overlay/settlement delay is retained in the trace: generator flow
is still 1 at G4, becomes 2 at G5, and refining resumes at G6 from the settled
stock. This is generator-funded refinery recovery, not fleet recovery or an
instantaneous-refill claim. The final assertion collects all eight throughput
violations and stays RED. No failure is ignored or inverted into a PASS.

## Exact source diagnosis and alternative-door audit

- `simthing-clausething/src/hydrate_field_economy.rs:462–521` accepts only the
  throttle **hint** for this building; lowering at 1673–1693 carries that hint
  into `ResourceRecipeSpec`.
- `simthing-spec/src/spec/resource_economy.rs:74–76` explicitly defines it as
  metadata, not an enforced CPU/GPU cap.
- `simthing-core/src/accumulator_op_builder.rs:236–279` defines exact recipe
  count as `floor(min(input_i / cost_i))`, with no per-tick cap.
- `simthing-kernel/src/transfer_accumulator.rs:211–237` deliberately does not
  forward the hint and constructs `max_transfer: None`.
- `simthing-driver/src/resource_economy_sync.rs:68–90` installs recipe output
  coefficients and order bands, but no cap. Both-faction complete conjunctions
  therefore legitimately spend all affordable input stock in one generation.
- The older category-economy recipe grammar also lowers the same hint-only
  `ResourceRecipeSpec` (`hydrate_category_economy.rs:1455–1503`). Changing
  authoring dialect does not supply a different throughput authority.
- The lower-level transfer registration does have `max_transfer` for discrete
  transfers, but the two-input recipe authoring path does not expose it. Native
  `stockpile_silo` produces constant sources and transfers; substituting that
  would invent income/storage behavior, not preserve this refinery contract.
  Reducing endowments, changing the 2:1 coefficients, rate-limiting via readback,
  or consuming a third synthetic token is not a faithful two-input solution.

**Required disposition:** Orchestration must identify an already-admitted
faithful recipe-rate composition or admit the generic authored cap mapping/law
needed by this consumer. This packet does not silently promote a hint into new
semantics or prescribe a new solver. No parser/spec/engine/sealed repair made.

## Content binding and test status

Source identities use the production source-identity function:

| Variant | Source identity | GameMode projection identity |
|---|---|---|
| canonical conjunction, minerals first | fnv1a64:42ee87791f36157c:5978 | fnv1a64:8d561ec899c02e11:9132 |
| canonical conjunction, energy first | fnv1a64:33eaa5da797b5a26:5978 | fnv1a64:8d561ec899c02e11:9132 |
| RF stocks, endowed | fnv1a64:9f9d1af572ab41c8:7420 | fnv1a64:7327633ef9f77495:7632 |
| RF stocks, surplus withheld | fnv1a64:96987e754f036b70:7419 | fnv1a64:7327633ef9f77495:7632 |

GameMode digests exclude intrinsic values. The raw packet additionally records
`PROFILE_FULL` digests over canonical-object-key JSON of the GameMode, complete
intrinsic session tree and install targets. These include each admission's node
identities and bind that run; they are not identity-normalized cross-run pins.
Dependencies remain `fnv1a64:c49f9ca3c8c75e77:20370` and
`fnv1a64:2f064bfb3e043aa0:72`. The unchanged canonical source SHA-256 is
`5821be96f0bfb439ce8717dba7dd280e994ae6a1113bff42d3137766031df328`.

Code-head full test command (ordinary resident execution):

```text
cargo test --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet -- --nocapture --test-threads=1
2 passed / 1 failed / 0 ignored / 0 measured / 0 filtered; 14.47s; exit 101
```

Local check PASS, 14.06s; agent scan PASS, 33s, hard failures 0 / INSPECT 0 /
TEST-BUDGET PASS before the final evidence commit. Final-head checks and hosted
Scan/Exec artifact/step conclusions are reported in the PR and Board.
[Raw execution](rehearsal_lifecycle_economy_fleet_resume_raw_results.md) retains
the first-gate transcript and the complete focused test output after build warnings.

## Fences and outstanding floor

All 32 sealed-component blobs unchanged from the released base, fifteenth E8
pin `0xa3e6_82eb_e0de_36f6` unchanged in both literals, all standing E8 tests
unchanged. No production files, gates, classes, anchors, workplan, canonical
asset/dependencies or persistence authority changed. One additional ordinary
AUDIT/delete-at-closeout row; three owning tests total. No parallel lane,
merge, Model 2, 2.3 or DEAD-EXPORT work.

This is a bounded economy precondition probe, not the complete frozen spatial
specimen or bounded-storage proof. Fleet funding/birth/identity/placement,
finite-capacity refusal/reservation, insufficient-alloy/energy-work fleet
recovery, born-fleet upkeep, ordering/identity/fail-stop negatives, and post-birth
continuation remain unexecuted. No new fleet ID, placement, upkeep or capacity
result is claimed. The selected energy locus is proven for the refinery slice;
its future sharing with fleet funding/upkeep is not yet proven.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb

Session receipt carried; all three rule-source files are unchanged from the
oriented session. Direct Board dispatch, no HD receipt invented.
