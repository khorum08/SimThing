# ASYNC-COMMAND-QUEUE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `cf2d1a88db3198ba811ffce89871a1dd9876eba6`
- ORIENT-RECEIPT: `6af1884543b0`
- orientation_rule_stamp: `5554b2613f8907ff`
- HD-RECEIPT: `a84a5dce9ebf`
- Dispatch: `5172950744`
- expected_route: `DA-RESERVE(gate-wiring)`
- Scope: **6.2 only**; no 6.2b/6.3 work

## Landed surface

- One admitted `AsyncOwnerChannelRfSeam` owns a canonical-scope `BTreeMap` holding queue. A burst retains at most one pending value per `{OwnerRef, ResourceKey, ScopeId}` while every numeric bucket field sums exactly in widened `u64` state.
- Queue ingress moves conserved value child -> seam and generation-barrier application moves seam -> parent. The accounting oracle checks both `child + seam + parent == admitted` and pending-carrier value equals the seam holding account.
- `IntegrationScheduleRowKind::{QueueInjection, StandingView}` extends the existing 6.1 append-only recorder. Queue rows remain one per source product; values coalesce but generation membership does not. Coalesced carrier stamps are the maximum source stamp.
- `AuthoredSeamStaleness` has no default. Every barrier preflights each coalesced carrier's newest/max stamp and hard-errors atomically on breach; historical source stamps remain full replay evidence and never become admission blockers.
- Downward ancestor policy is captured site-locally, crosses only as `GenerationStamped<AncestorStandingPolicyView>` with canonical `OwnerRef`, and publishes through a two-slot generation/value buffer. Both directions replay from the same schedule.
- Existing `CommandDeficit` delivery remains on `owner_silo_disburse_down` -> `runtime_local_allocation_from_disbursement`; the rung adds no directive transport, route-distance inference, or disbursement-band authority.

## Biting proofs

| Proof | Result |
|---|---|
| Five-product same-scope burst | exact sum of all five `OwnerChannelRfBucket` numeric fields; pending cardinality equals distinct buckets; carrier stamp=max |
| Holding account | exact before and after the barrier; dropped-pending and all-three-account escape mutants RED |
| Full generation membership | five `QueueInjection` rows preserve generations 1..5; latest-only/drop replay differs from live |
| Coalesced lag and tolerance | Same-key generations 1..5 at parent 8 admit from carrier stamp 5 when authored max=3 while retaining all five schedule generations; max=2 hard-errors with queue/parent/log unchanged; historical-contributor staleness mutant RED |
| Out-of-order max-stamp referee | Same-key arrival 5 then 3 retains carrier stamp 5, admits at parent 8/authored max 3, and preserves schedule order [5,3]; exact last-wins assignment mutant exposes stamp 3 and REDs on staleness |
| One recorder | reversed ambient products replay bit-exact; empty second-recorder mutant hard-errors |
| Bidirectional snapshot | two standing-policy publications share the upward log, replay bit-exact, and staging cannot tear or republish an old slot |
| Directive path preservation | existing `simthing_automaton_rf_reception_0` battery remains green with live route policy and standing inheritance |

## Local evidence

```text
cargo test -p simthing-spec --test async_command_queue_0: 3/0
cargo test -p simthing-spec async_queue_accounting_mutant_proof --lib: 3/0
cargo test -p simthing-spec --test event_generation_stamp_reduce_up_0: 4/0
cargo test -p simthing-driver --test simthing_automaton_rf_reception_0: 2/0
```

## Posture

**PROBATION / proof-present / DA-review-pending.** No clearance, merge, pointer movement, or successor-rung work is claimed.
