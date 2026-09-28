# 2.2 pin-19 continuation: storage and recovery GREEN; alloy-output ingress STOP

**STOP / PROBATION / orchestration-review-pending. Same PR #2075 remains draft and unmerged.**
This packet supersedes the driver-qualification hold as current evidence, while preserving all historical packets.
Sole handoff: Board [5744331095](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744331095), revision 2026-09-28T13:31:25Z.

## Identity and orientation

- Pre-sync head: `119c8e75debb801d7a4ee7c7f0851f76622efff3`; preserved remote backup `codex/0088-economy-fleet-stop-119c8e75`.
- Consumer base: `681600b0cb670b38e63ece5380927c1b0bede3f9`.
- Exactly-one synchronization was already consumed: 16 clean replayed commits, one start at that base and finish at `ee805483b1a0b1bc3591e9f04bef573e8f9389e7`, tree `ed079eeaa24bfd4559b901b2bb650b481c6d80a4`. Retained reflog is in the raw manifest. No further rebase occurred.
- Governance checkout: `c5286c95637e56be42b100ca64187cf98d3828f7`, distinct from consumer base. Refreshed once there, actually ingested the coding orientation, then `--since=f86020d2b2ae` returned `ORIENT-SINCE-VERDICT: CURRENT` there.
- Tested code: **`beaa99b97f1f68149368951a3a04c0ff12295cb8`**, tree **`c55ec542c38d393962ce46a9ee8eb8367c37680b`**. Later evidence commits change only these pin19 result documents; the final published head/tree and hosted results belong in the PR/Board return.
- Both E8 literals remain **`0xef92_b0f2_866c_ef5d`**. All 32 sealed component working bytes and committed blobs equal the authorized release. No production, parser, runtime, driver, persistence, gate, pin, or dependency edits.

ORIENT-RECEIPT: f86020d2b2ae
role: coding
orientation_rule_stamp: 14f47e2289d3be96
orientation_digest_sha: abf87af75e48f20a87b7aa243f0ab971928962aa35aaf8b35f19216327e14e4f
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b

## Ordered accepted baseline after synchronization

All seven ordered semantic acts ran at untouched `ee805483...` before continuation edits; all passed.
The unchanged companion retains 12 sequential three-row births, N+0.5 causality, exact G17 `MarketUnresolved { granted: 0 }`, and complete prior placement preservation.

| Act | Exit | Exact result | Wall seconds |
| --- | --- | --- | --- |
| conjunction | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 1m 18s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.69s | 82.92 |
| stock | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.59s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.25s | 3.09 |
| birth | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.57s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.80s | 4.63 |
| rf | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.57s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.64s | 3.45 |
| capacity | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 17.67s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 2.86s | 21.83 |
| native | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.57s; test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.38s | 23.61 |
| parent | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.55s; test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.09s | 13.95 |
| inventory | 0 | TEST-INVENTORY-DRIFT-CHECK REPORT; rows: 1139; discovered: 1139; unledgered: 0; stale: 0; TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS | 4.24 |
| check | 0 | Finished `dev` profile [optimized + debuginfo] target(s) in 12.03s | 12.13 |
| scan | 0 | DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED; AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=53s | 53.5 |

## Bounded stock: completed consumer proof

The owning stock test retains both prior frozen-economy cases and adds four native source variants: omitted clamp, explicit Unbounded, Bounded(0,24), and that same bounded source through canonical cache/rebind. Each executes 50 generations.

The sole mineral Balance remains the actual refinery input; the test resolves its slot and column through the live allocator/registry and checks the compiled recipe consumes precisely that cell at cost 2. Its two fields are only `balance_rate` and governed `balance`. All mineral RF carrier fields and parent edges are absent, and the test checks no RF arena owns the property. Energy retains the existing conserved RF path and reconciles against the same capped 2-mineral/1-energy recipe.

The source authors reset-then-add producer overlays at the mines: rate 0, then local supply potential 3/gen. Fixed boundary requests suspend supply at the G11 boundary and reactivate at G25, affecting hot cycles G12..G25 and G26 onward. The schedule never depends on stock readback. No profile patch, GPU write, CPU balance feedback, or second ledger drives execution.

| Milestone | Bounded TD / PC mineral stock | Unbounded discriminator |
| --- | --- | --- |
| G11 saturation | 24 / 24 | 31 / 25 |
| G12 ordinary spending while supply is off | 22 / 22 | 29 / 23 |
| G23 and G25 floor | 0 / 0 | Unfunded recipes cannot overdraw bounded stock |
| G26 first restored production | 3 / 3 | Exactly one generation, no banked production |
| G27 spending resumes | 4 / 4 | One capped batch |
| G47 re-saturation | 24 / 24 | 25 / 25 |
| G50 stable ceiling | 24 / 24 | 28 / 28 |

The independent observation oracle checks every generation: `stock + 2 * cumulative_batches = initial_stock + cumulative_accepted_production`. At G50 both bounded cases execute 47 refinery batches per faction. TD accepted production is 98 with 10 curtailed; PC 104 with 4 curtailed. Thus `20+98-94=24` and `14+104-94=24`. Offered local potential totals 108 each. The 10/4 curtailed units are **unrealized local production outside RF**, not units disbursed by a conserved arena and discarded. No overflow/refund authority is invented. Canonical native/cache histories match exactly; omitted and explicit-unbounded histories match exactly. Live and cached specs and live registry retain the authored clamp. Floor safety is proved on the executed path; parser-malformation law remains the landed #2084 prerequisite, not newly retested here.

## Separate fleet-input recovery and funded refusal

Both recovery cases preserve total starting minerals/alloys/energy and frozen costs/rates. Source-authored zero allocation weights withhold a producer; an ordinary fixed AttachOverlay at the G5 boundary restores weight 1.

- **Alloy-only gap:** bootstrap energy is moved from refinery WIP to yard WIP within each faction's unchanged total 10/8. Refinery starts with zero energy allocation, so alloys remain 4/3, below cost 6, while yard energy is independently sufficient. No funding/birth occurs during G1..G5. Natural refining resumes; first funding is TD G9 / PC G10 and births G10/G11. Second funding G15/G16 yields births G16/G17.
- **Energy-work-only gap:** yard has zero energy allocation; refinery retains original bootstrap 10/8 and produces alloys. At G4/G5 alloys are sufficient while yard energy is exactly zero. Restoring the yard's allocation supplies four energy-work units; both factions first fund G10 and birth G11, then fund G14 and birth G15. Neither creates a birth from incomplete inputs.
- Every generation checks actual mineral/alloy/refinery-energy/yard-energy balances against an independent input-debit/output-credit oracle, scalar funding, and exactly one following-boundary birth per admitted product unit. The two-product recovery variants' later scalar receipts are explicitly not claimed as additional completed fleets.

The 80-generation refusal composition raises only authored product cardinality to 50 per faction. It consumes the admitted 2.1 disposition (`ordinary_funded_placement_and_restart_matrix`): consumed inputs stay consumed, failed placement scraps that funded attempt, receipt is not reservation, and the old instance never retries. No subtree is removed. Twelve three-row fleets fit; the first further attempt at G40 receives `MarketUnresolved { granted: 2 }` because two rows remain and a whole three-row subtree is required. This differs from the unchanged companion's **zero**-grant fixture, which remains GREEN. Each refused candidate has no live node, slot, or committed placement; every prior placement is unchanged; capacity/live-count reconcile exactly. At G80 each faction has **13 paid receipts = 6 births + 7 explicit refusals**, no pending unit, yard energy 28, alloys TD6/PC5. All 6-alloy/4-energy debits persist with no phantom refund or retry. This proves funded refusal disposition; it does not exercise removal/recycled-capacity restart, which remains fenced.

## Exact remaining STOP: native output locus

The two accepted born-energy controls still prove automatic RF enrollment and +2 surplus versus +1 after one born corvette. The new next-stage probe attempts one authored `meridian::alloys` Balance, initialized to the same 4/3 stocks, as BOTH refinery output and shipyard input:

```clause
output = { entity = A1 property = "meridian::alloys" role = balance coefficient = @refinery_output }
input = { entity = A1 property = "meridian::alloys" role = balance amount = 6 }
```

Native hydration refuses at **token 720**, before activation: **`unsupported output field entity`**. `hydrate_field_economy::parse_resource_output` accepts only `resource` and `coefficient`; lowering unconditionally constructs the generated location/material quantity with role Amount. In contrast, the existing generic `ResourceRecipeSpec` already has target property, role and host fields, and canonical input projection is landed. No test changes those protected surfaces.

**Return dependency:** Orchestration/DA must supply an admitted native composition that connects actual refinery output to the sole alloy stock needed by born upkeep, or authorize the missing canonical output-locus projection. The proposed source form is an ingress probe, not existing admitted syntax. This result establishes that specific authoring restriction; it does not prove all conceivable alloy-upkeep compositions impossible, nor does adding the output syntax alone prove the remaining runtime semantics. Full 0.2-alloy upkeep, with its conservation and stock accounting, still requires execution after disposition. No runtime/kernel change is requested or presumed by this consumer.

## Final test results and scope

| Act | Exit | Exact result | Wall seconds |
| --- | --- | --- | --- |
| stock | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 12.71s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 8.41s | 22.55 |
| birth | 0 | Finished `test` profile [optimized + debuginfo] target(s) in 0.53s; test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 9.66s | 10.47 |
| rf-alloy | 101 | Finished `test` profile [optimized + debuginfo] target(s) in 0.54s; test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.48s | 3.26 |
| parent | 101 | Finished `test` profile [optimized + debuginfo] target(s) in 0.56s; test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 27.12s | 27.96 |
| inventory | 0 | TEST-INVENTORY-DRIFT-CHECK REPORT; rows: 1139; discovered: 1139; unledgered: 0; stale: 0; TEST-INVENTORY-DRIFT-CHECK-VERDICT: PASS | 0.37 |
| check | 0 | Finished `dev` profile [optimized + debuginfo] target(s) in 1.06s | 1.14 |
| scan | 0 | DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED; AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=50s | 51.42 |

The final parent is **4 passed / 1 failed / 0 ignored / 0 measured / 0 filtered, exit 101**. The new admission checkpoint makes the owning RF test RED after its accepted energy proofs complete. Required semantics are NOT GREEN. Scanner PASS is not semantic PASS. Local AGENT-SCAN: hard 0, INSPECT 0, TEST-BUDGET PASS; embedded selftests skipped. Inventory is 1139/1139 with no drift; no new test ID was introduced. The native companion and inventory are byte-identical to the synchronized baseline.

Still incomplete: combined recurring **1 energy + 0.2 alloy per born corvette**; complete stocks under that full upkeep composition; full authored/declaration/order equivalence; identity/replay/fail-stop negative battery; post-birth save/restore. Existing partial negatives/order/cache proofs do not substitute. Structural departure, cancellation of enrolled born subtrees, reparent/fusion/tombstone/recycled-slot semantics remain untouched. Model 2 is CLOSED. No second agent, additional rebase, clearance invocation, merge, graduation or 2.3 dispatch.

The only executable continuation change is the owning workshop test. Prior result docs and the companion are preserved. Hosted Scan/Exec and final publish binding are recorded in the final PR/Board return after this evidence commit.

## Retained development diagnostics

Early witness diagnostics are retained in the raw-log manifest: local stock observation originally required an RF anchor (replaced with direct published GPU observation bound to the actual recipe input); a column API typo failed compilation; the off/on schedule initially assumed pre-hot-cycle boundary application (corrected from `step_once` ordering); recovery source matching initially omitted the refinery's authored upkeep flow; and the new refusal fixture initially assumed zero residual capacity (corrected to its actual two-row remainder without changing the exact companion). These are witness repairs, not runtime edits or relaxed economic assertions. The final canonical-output refusal remains RED.
