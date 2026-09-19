# 2.2 RF release and repaired capacity proof — inventory scope STOP

Status: **STOP / PROBATION / proof-present / orchestration-review-pending**.
Same PR #2075, OPEN / DRAFT / UNMERGED. No merge or graduation. Model 2 CLOSED.
Unified handoff: [5744331095](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744331095).
DA discharge: [5744310551](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744310551), landed #2082.

## Binding and preservation

- Exact release base: `a14d4a57813952781ec475cfd32d7c4f80fadfda`.
- Rebased SAME branch ONCE: `ddf62c55caac534f7a82e9f3eada9d5cd698652c` -> `895803e4faae5c20ae998f7b0ab321ffbd7da588`. All ten picks completed cleanly. Reflog in the raw companion.
- Previous STOP preserved remotely at `codex/0088-economy-fleet-stop-ddf62c55`; earlier 366d7b60 / 3d00def2 / ec04a635 backup branches remain.
- Committed implementation head: `4d6f43a7a968698ea6a4da74426d039731c0938d`; tree `b28175a71c9aa14a69a95a39ce19f9550c1c7b1a`.
- Final evidence-only head, final-head reruns and hosted results will be bound in the current PR body / Board return; this document records the implementation-head evidence, not a self-referential final SHA.
- Eighteenth E8 `0xee37_12f2_ef18_6934` retained at both runtime/qualification literals. All 32 sealed component blobs (including Cargo.lock) match the release base.
- All eight historical economy/fleet result files are byte-identical to old STOP. No canonical scenario, dependency, production, parser, spec, driver, kernel, GPU, CI gate, anchor, workplan, orientation or handoff edit.
- Inventory is unchanged from the once-rebased head. The existing five parent-test ledger rows were carried through rebase; #2082's native RF ledger row comes from the release base.

## Ordered execution

Before new edits, `cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet` passed (59.53s), followed by AGENT-SCAN PASS (68s, hard 0 / INSPECT 0 / TEST-BUDGET PASS). Then the four gates ran in the mandated order:

| Check | Exact harness result (all exited 0) |
| --- | --- |
| Conjunction / canonical locus FIRST | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 5.06s` |
| Capped refinery + generator scarcity/recovery SECOND | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 2.63s` |
| Native funded structural birth THIRD | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 1.98s` |
| Born-energy RF upkeep FOURTH | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.02s` |
| RF boundary strengthening | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 3.39s` |
| Capacity focused | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.57s` |
| Native structural-product full target | `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 32.69s` |
| Committed code-head economy/fleet full target | `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 26.20s` |


All local Cargo commands use `--locked`. Tests use `-- --nocapture --test-threads=1`, and isolated tests also use `--exact`. No ignored/measured tests. Native target check passed (2.04s); combined two-target check on implementation head passed (1.54s). Implementation-head AGENT-SCAN PASS (58s), hard 0 / INSPECT 0 / TEST-BUDGET PASS. Embedded scanner selftest SKIPPED; inventory drift deliberately not included in this coding scan.

## RF admission and economic discriminator

The parent test now checks automatic Derived-arena membership at each successful birth boundary, exact structural-parent and allocator-slot binding, exclusion of every energy-less child, zero enrollment refusals, one admission per carrier root, and a single registry-generation bump. It supplies no enrollment/rebuild operation. The fixture's dyadic shares permit exact equality; removed the former magic 0.0001 tolerance.

In the strengthened isolated run, both cases birth at G6:

| Case | Born root -> RF parent | Slot | Energy arena | Registry generation | G8 faction surplus |
| --- | --- | --- | --- | --- | --- |
| flow 0, Terran | 262 -> 214 | 38 | 1 | 1 -> 2 | 1 refinery + 1 yard = **2** |
| flow 0, Pirate | 268 -> 220 | 41 | 1 | 1 -> 2 | 1 + 1 = **2** |
| flow -1, Terran | 329 -> 281 | 38 | 1 | 1 -> 2 | 0.5 + 0.5 = **1** |
| flow -1, Pirate | 335 -> 287 | 41 | 1 | 1 -> 2 | 0.5 + 0.5 = **1** |

IDs are exact-run observations; allocations in the full target differ. Non-carrier children never inherit membership. A born leaf's energy contribution settles at the host pool under #2082, as the expected +2 / +1 totals show. This discharges the previous post-birth RF enrollment RED, not the whole recurring 1 energy + 0.2 alloy contract.

Native funded-birth gate remains green: both first fund at G5. Terran roots 262/265 birth G6/G10 under A1=214; Pirate roots 268/271 birth G6/G11 under E1=220. Children are respectively 263,264 / 266,267 / 269,270 / 272,273. All twelve IDs are fresh, ordinary placed extents of three, explicitly owned. N0 live 38 -> 50 within fixed capacity 76. Each recipe pays exactly six alloys + four energy-work per funded scalar output, at most one output per generation. At G40: Terran mineral stock 60 / alloys 2 / yard energy 12; Pirate minerals 54 / alloys 1 / yard energy 12. Seven funded scalar outputs per faction do not imply seven births: authored count is two. No refund/cancellation disposition is claimed for the excess scalar output.

Capped refinery and fixed generator-restoration cases retain their prior exact generation-by-generation stock checks. These prove refinery scarcity/recovery, not the separate remaining fleet-input recovery cases.

## Bonsai salvage — source material reviewed, three claims repaired

After RF GREEN, rendered `handoffs/0088-BONSAI-CAPACITY-INTEGRATION-0.hd.md` as coding; HD-RECEIPT `c0c0bf05feb2`. Historical base/HOLD is superseded by the unified exact release. Compared source `64af8ebcfce2d9ac196f0e371e8eac39fd439438` against `60830eef4907b2bc9a8bf94325f9b50f8891d528`; did not accept its old completion claim or cherry-pick its verbosity/scaffolding.

Only the named native test file changed for this subtask. Preserved all five preexisting tests, including the landed DA RF witness. The source's 40-generation sequential native-session structure survives: one source-authored count-50 product at A1, three rows per subtree, ordinary funding and residency only. No synthetic funding, market, allocator, AddChild injection, structural removal or RF departure.

1. **Funding causality:** for N=12 prior births, exact next threshold **12.5**; consecutive pre-boundary shadow stamps/values **(16,12) -> (17,13)**. G16 settlement crosses the threshold; the one newly funded candidate attempts G17. Assert current shadow and typed attempted-generation stamp both equal 17. This replaces merely `pool > 0`.
2. **Placement preservation:** capture actual `CommittedResidencyPlacement` objects for every previously born root immediately before/after each boundary, including first refusal; exact equality covers private fields `identity`, `extent`, `quantity`, `committed_generation`. This replaces reduced shape/owner/quantity facts. Root and child identity sets are checked at each boundary; no prior identity vanishes.
3. **Specific refusal:** only exact `MarketUnresolved { granted: 0 }` for parent A1, quantity 3, remaining capacity <3. Removed the `Placement(_)` alternate. Any changed semantics fail rather than widening.

Observed: N0 live 36 / free capacity 36. Twelve successes at G2,3,4,5,6,8,9,10,12,13,14,16, each +3 live / -3 capacity. First refusal G17 has zero capacity, no birth, no new identity, unchanged live count and capacity, and all twelve full placements unchanged. Seventeen additional refusal facts through G40, no later births. Final live 72 / free capacity 0. Full initial focused log includes every placement identity, extent, quantity and committed generation. The focused witness passes 1/0/0/0/5; whole native target passes 6/0/0/0/0.

Cleanup replaced 380 appended trial lines with 147 focused proof lines (plus two import changes), removing duplicate histories/diagnostic vectors and trial-only prose. Existing tests were not reformatted. Astra performed all repairs solo; no Bonsai agent/session resumed, no parallel exception retained. No inference about Bonsai tool/runtime failures beyond reviewed source material.

## Exact STOP: inventory fence prevents CI admission

`bash scripts/ci/test_inventory_drift_check.sh` exits **1**:

```text
rows: 1119
discovered: 1120
unledgered: 1
stale: 0
TEST-INVENTORY-DRIFT-CHECK-VERDICT: FAIL
unledgered: simthing-workshop / crates/simthing-workshop/tests/native_structural_products_session_0.rs / sequential_capacity_exhaustion_preserves_prior_births / integration
```

The mandatory named focused test is new and is not ledgered on the released base. Handoff **§3E explicitly forbids inventory edits**, and **§6 requires STOP when hardening needs another file**. This is the concrete scope gap; no new generic runtime defect is claimed. Removing/renaming/hiding the mandated test would not discharge the handoff. The patch is preserved in the same draft lineage with this truthful RED. The local coding scan's PASS does not override the stock inventory gate.

Return to Orchestration for a narrow inventory-surface correction (or an authoritative landed ledger row) covering exactly this named test. Suggested ordinary classification follows the adjacent native witnesses: `integration / invariant-required / 0088-ECONOMY-FLEET-0 / AUDIT / ledger-only / 0.0.8.8-integrated-rehearsal / 0`, with a note naming exact threshold, placement and refusal invariants. No gate change or waiver is requested. No inventory row was written.

## Remaining 2.2 obligations, not discharged

Work stops at this required scope conflict before the mandatory capacity integration can be CI-green. Canonical bounded-storage specimen integration; separate insufficient-alloy and energy-work fleet recovery; funding/reservation disposition around refusal/cancellation; recurring 0.2-alloy upkeep coupled with energy upkeep; complete post-birth stock reconciliation; full authored/order equivalence; identity/replay/fail-stop negatives; post-birth save/restore remain unproved. The existing native target covers declaration-order equivalence and malformed ingress, but is not claimed as the complete parent battery.

No enrolled born subtree was canceled/removed. The STRUCTURAL DEPARTURE follow-on remains absent; no removal/reparent/fusion/tombstone implementation or test shortcut was attempted. Model 2 remains CLOSED. No 2.3 dispatch.

Final current-head hosted Scan/Exec results, skips, artifacts, exact tested SHA/tree and final reruns go in the PR/Board return. Orchestration, not Astra, owns the fresh final-body `/clearance` and literal routing. Recommended posture: **scope-gap STOP, no merge/graduation**.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b
HD-RECEIPT: c0c0bf05feb2
