# 2.2 inventory correction and native bounded-storage ingress STOP

**STOP / PROBATION / proof-present / orchestration-review-pending. Same #2075 OPEN / DRAFT / UNMERGED.**

Operative handoff: [Board 5744331095](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744331095), revision **2026-09-19T20:22:54Z**. All earlier versions/instructions are provenance only. This continuation discharges the exact inventory fence error, preserves the accepted RF/birth/capacity evidence, then exposes the next native authoring gap. One coding agent only; no new agent/branch/rebase, no merge/graduation. Model 2 CLOSED.

## Binding

- Exact release base: `a14d4a57813952781ec475cfd32d7c4f80fadfda` (#2082).
- Continuation starts at accepted head `ce632646400067d4de494ee6c488cc2c0e7a4c68`, tree `252b0418482e5d2bff597448376a70b6eb5379f8`.
- Exact one-row ledger commit: `f975a647ca243f94199919d1410fd7f08d40a60b`.
- Implementation/tested code head: `510d4cd3045d3e876e9dce67f4ba9787e4b8d9f6`, tree `9cdde7d866c4372c83c046cada45355a3dd41fde`. Final evidence-only head/reruns and hosted artifacts will be bound in the final PR body and Board return.
- Previous once-only rebase onto release is unchanged. No second rebase. Existing remote STOP heads and all **ten** prior result docs remain unchanged.
- E8 18: `0xee37_12f2_ef18_6934`; both literals and all 32 sealed components including Cargo.lock match release. No canonical scenario/dependency, production/parser/spec/driver/kernel/GPU, gate/class, handoff or workplan edits.
- Current router class debt is separately routed at Board 5744867970. Prior-head fresh clearance was `DA-RESERVE(class-envelope-violation)`; it is not a current-head clearance. No class/gate table edit made here; no self-rebase to pick up any independent hardening.

## Exact inventory correction

Only one new TSV row relative to ce632646; every prior row remains byte-equivalent and in order:

```tsv
simthing-workshop	crates/simthing-workshop/tests/native_structural_products_session_0.rs	sequential_capacity_exhaustion_preserves_prior_births	integration	invariant-required	0088-ECONOMY-FLEET-0	AUDIT	catches: failure of the exact N+0.5 funded-candidate crossing, loss of full committed placement across exact zero-grant capacity refusal, or fabricated birth/identity/live-count/free-capacity mutation during sequential ordinary exhaustion	ledger-only	0.0.8.8-integrated-rehearsal	0
```

Both immediate and post-probe `test_inventory_drift_check.sh` exit **0**, report **1120 rows / 1120 discovered / zero unledgered / zero stale / zero parked**. No other inventory mutation. The added stock-bound probe extends the existing owning stock test; no new test identifier/ledger row is introduced.

The required correction checks ran in order: inventory -> exact capacity -> full native -> full parent -> combined cargo check -> AGENT-SCAN. The already accepted capacity proof was not rewritten.

| Act | Exit | Exact harness result |
| --- | --- | --- |
| Ledger correction: capacity exact focus | 0 | `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.57s` |
| Ledger correction: native full | 0 | `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 39.80s` |
| Ledger correction: parent full | 0 | `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 23.00s` |
| New stock-bound ingress focus | 101 | `test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 4.37s` |
| Implementation-head parent full | 101 | `test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 22.26s` |


Combined ledger-head `cargo check --locked -p simthing-workshop --test rehearsal_lifecycle_economy_fleet --test native_structural_products_session_0`: exit 0, 0.66s. Ledger-head AGENT-SCAN: PASS, hard 0 / INSPECT 0 / TEST-BUDGET PASS, 61s. After adding the ingress probe, check of the parent target: exit 0, 1.87s; AGENT-SCAN: PASS, hard 0 / INSPECT 0 / TEST-BUDGET PASS, 57s. Embedded scanner selftests SKIPPED, not claimed as executed. All tests use `--locked`, `--nocapture --test-threads=1`; focused acts use `--exact`.

## First remaining-floor blocker: bounded stock cannot reach the native property schema

The next required floor is canonical **bounded storage**. The existing two stock cases still execute before the new assertion, unchanged: mine +3 minerals, refinery consumes 2 minerals +1 energy to make 1 alloy, cap one batch; explicit endowments TD20/10/4 and PC14/8/3; fixed generator restoration recovers without banked throughput. On the unbounded positive control TD minerals are **24 at G4 -> 25 at G5 -> 28 at G8**; PC minerals 18 -> 19 -> 22. These are valid unbounded results, not a regression.

The new probe uses that SAME source and unchanged declared JSON dependencies. It changes only the first (mineral) Balance sub-field declaration to request the existing core `ClampBehavior::Bounded` representation:

```clause
sub_field = {
  role = balance
  governed_by = balance_rate
  accumulator = Balance
  clamp = Bounded { min = 0 max = 24 }
}
```

The diagnostic bound 24 fits both mineral endowments; it is **not** a new frozen specimen capacity decision. The `clamp` spelling is a proposed native projection of existing core data, **not a claim that the current grammar already admits it**. No complete storage/overflow policy is inferred from a clamp. This minimal probe asks whether the actual spendable cell can carry a finite bound from native source before attempting runtime saturation/overflow/recovery accounting.

Exact candidate source identity (LF bytes written by the fixture): **`fnv1a64:a04e385ae7fa0df5:7527`**. Full candidate source is in the raw companion. No hydrated pack, profile, registry, matrix, or session is patched.

Ordinary native `ingest_clause_scenario_path` refuses **before activation**, at token **272**:

```text
SourceResolution("0088-INGRESS-FIDELITY-0 refusal before activation; ... ClauseThing hydration error at token 272: unsupported sub_field field `clamp`")
```

The assertion retains this required next-floor RED. If native hydration admits it later, the probe also requires exact retention of `Bounded { min: 0, max: 24 }` on `meridian::minerals / balance`; admission that silently drops the bound is not accepted. Both existing stock cases and all other parent tests retain their original assertions. There is no invalid canonical asset write, test-side stock clamp, or fabricated overflow/refund mechanism.

## Source archaeology and alternatives checked

All pointers below refer to this implementation head; production files are read-only.

| Surface | Observed contract / limit |
| --- | --- |
| `crates/simthing-core/src/property.rs:39` | Existing `ClampBehavior` offers Bounded/Floored/Unbounded. No new runtime variant is needed merely to name a bound, but its full economic storage sufficiency is not proven here. |
| `crates/simthing-clausething/src/hydrate_scenario.rs:2638` | Scenario property sub-fields delegate to the native ingress parser. |
| `crates/simthing-clausething/src/rehearsal_ingress_fields.rs:20` | Closed fields are role/default/governed_by/accumulator. Unknown fields are refused; returned layout hardcodes Unbounded at line 87. Balance takes its default metadata; arena-bearing forms only allow AllocatedFlow/AllocatorWeight. |
| `crates/simthing-clausething/src/hydrate_field_economy.rs:605` | `stockpile_silo` accepts id/owner/resource/current, explicitly rejects capacity. Existing `field_economy_grammar_0::stockpile_capacity_is_spanned_unsupported_authoring_error` protects that refusal; it was read, not weakened or newly rerun. |
| `crates/simthing-clausething/src/hydrate_field_economy.rs:1782` | Silo lowers an owner-local current -> stockpile transfer plus Constant(current) emission. That is not a finite bound on the existing mine/refinery/yard Balance cells and is not substituted as a second resource source. |
| `crates/simthing-clausething/src/hydrate_scenario.rs:869,1183` and `crates/simthing-spec/src/spec/scenario.rs:1029` | Owner stockpile_capacity requires a seed and becomes separate owner metadata marker/current/capacity. It is not an arbitrary `(host, property, role)` capacity declaration. |
| `crates/simthing-driver/src/session_resource_flow_silos.rs:28` | Its separate owner_silo arena targets session::owner_silo_flow with no balance_property; this is not the existing meridian energy/mineral stock locus. No authority swap attempted. |
| `crates/simthing-clausething/src/hydrate_scenario.rs:2751` | Native modifiers expose amount_add or amount_mult, not an arbitrary EML bound program. A test-side current-dependent clamp would supply the missing economy authority and is forbidden. |
| `crates/simthing-clausething/src/hydrate_field_economy.rs:1699,1953,1998` | Auto-generated local material outputs target Amount and use unbounded stock fields; not an alternate native bounded Balance definition. Duplicate authored property IDs are refused by hydrate_scenario at line 720, not merged as overrides. |
| `crates/simthing-clausething/src/hydrate_category_economy.rs:631,1451` | Separate CT-2c GameMode hydrator, not a native scenario composition door; it does not supply bounded hosted stocks for this loaded bundle. No profile splice or second IR used. |

**Precise return:** identify an existing admitted native representation for a finite bound on the same spendable mineral/energy/material cell, or admit the minimal protected authoring mapping needed for that representation. A generic runtime storage/overflow/saturation claim still needs a subsequent executed proof; this return does not prejudge its lawful disposition. Directly editing the closed ClauseThing/parser/spec surfaces is outside this coding release, so work stops at that boundary.

## Accepted evidence preserved and limits

[Accepted return 5744836766](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5744836766) and the ten immutable prior docs remain the detailed RF/birth/capacity record. Required ledger-head reruns reproduce native **6/6** and parent **5/5** before the new storage assertion. Capacity: 12 fresh three-row births, G17 exact MarketUnresolved zero grant, N+0.5=12.5 crossed 12 -> 13 at the causal boundary, no identity/live/free mutation, every full committed placement preserved; later refusals fabricate nothing. No source change to that witness this continuation.

RF proof remains automatic admission at G6, correct parent/slot, excluded non-carriers, one registry bump, +2 control / +1 one-corvette energy. Native funded products retain both owners, fresh identities, ordinary placement and exact six-alloy/four-energy recipe cost. Neither this accepted energy proof nor the prior refinery recovery proves full recurring **1 energy +0.2 alloy** upkeep or the two separate fleet-input recovery cases.

Remaining floor is still unproved: canonical bounded-storage integration and runtime overflow/stock accounting; separate insufficient-alloy and insufficient-energy-work fleet recovery; funding/reservation disposition around lawful refusal; full recurring upkeep and post-birth stock reconciliation; complete authored/order equivalence; identity/replay/fail-stop negatives; post-birth save/restore. No enrolled subtree removed/canceled/reparented; no departure/fusion/tombstone/recycled-slot semantics implemented. Model 2 CLOSED. No 2.3.

## Routing

**New semantic STOP: native bounded-stock authoring**, not the now-discharged inventory fence. Keep #2075 DRAFT/UNMERGED. Final-head tests and hosted Scan/Exec artifacts will be added to PR/Board return. Hosted green does not override the local semantic RED. Independent router class-hardening stays with DA/Orchestration, and Orchestration owns the required fresh final-body `/clearance`; Astra does not invoke it.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: rehearsal-0088-charter@056bfe9205e7
ANCHOR-ACK: rehearsal-0088-acceptances@3d95d334fafe
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: rehearsal-0088-binding-laws@25af224d1deb
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b
HD-RECEIPT (accepted capacity provenance only): c0c0bf05feb2
