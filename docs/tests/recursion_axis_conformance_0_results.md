# RECURSION-AXIS-CONFORMANCE-0 — E6 Owner-stop evidence

Status: **PROBATION / E6-OWNER-DECISION-PENDING / UNMERGED / NO SEMANTIC REMEDY / NO 15.6**.

Authority: Board handoff `5532779733`, binding clarification `5529092629`, and `handoffs/RECURSION-AXIS-CONFORMANCE-0.hd.md` (`HD-RECEIPT: bbecf03b4d88`). Coding orientation is `ORIENT-RECEIPT: 4266e4870c67` under rule stamp `240c816e9fe71dc1`.

## Stop verdict

The two mandatory E6 falsifiers were planted before any production change. Both reproduce the documented request-reserved stranding behavior on the production `ResidentClearingRuntime`, so the handoff's **E6 OWNER STOP** is active. This branch deliberately does not select work-conserving fallthrough, entitlement-first reservation, or a shared answer for both commitment classes.

## E6 transcript

| Falsifier | Input | Observed production-resident result | Stranded feasible supply |
|---|---|---|---:|
| zero-basis high band | available 4; source 8 requests 4 at precedence 0 with exact basis 0 | `(source=8, G=0, U=4)` | 4 |
| mixed bands | available 4; source 8 requests 4 at precedence 0 with basis 0; source 9 requests 4 at precedence 1 with nonzero basis | `(8, G=0, U=4), (9, G=0, U=4)` | 4 despite a serviceable lower band |

Focused execution:

```text
running 2 tests
E6 mixed-band stranding reproduced: available=4 products=[(8, 0, 4), (9, 0, 4)] stranded=4 despite serviceable lower band
E6 zero-basis-high-band reproduced: available=4 products=[(8, 0, 4)] stranded=4
test result: ok. 2 passed; 0 failed
```

## Archaeology map

| Surface | Current source truth | 15.5 disposition before Owner ruling |
|---|---|---|
| root continuous preparation | `ResidentClearingRuntime::prepare_root_continuous_allocation` installs total requested quantity as root flow and claimant pressure as `AllocatorWeight` | observed only; no change |
| CPU exact precedence | `settle_resident_apportionment_over_share_vector` computes `remaining = available - prior_requested` | E6 defect reproduced; no remedy selected |
| resident exact precedence | `resident_clearing_apportionment.wgsl::settle_partition` computes `remaining = supply - prior_total`, where `prior_total` sums requests | same E6 defect reproduced on the real GPU path; no remedy selected |
| canonical product | `ResidentConstrainedProduct` carries semantic row, claimant, G, U, generation, and integration band; settlement output and recursive intake remain type aliases | immutable ABI observed; no adapter added |
| spatial `dispatch(None)` | requires the template's original granter, advances via `for_recursive_intake_generation`, matches the prior semantic scope, and reuses product claimant/row identity | confirms E1 remediation remains outstanding; no source change |
| resident temporal path | the intake transform copies the whole `T_s` record, replaces its U field with `f(U)`, then recursive `read_claim` treats `G + f(U)` as the next request | confirms the forbidden pseudo-`T_s`/`G+f(U)` shape remains outstanding; no source change |
| CPU temporal authority | `produce_runtime_rf_next_generation_demands` consumes its one mint and adds optional `f(U)` to independently authored N+1 demand | already-correct reference remains untouched |
| prepared versus executes | resident `dispatch(None)` immediately evaluates the next economic clear from the pending intake/template without an N+1-authored datum | falsifier/remediation remains outstanding after Owner E6 ruling |

## Scope ledger

Changed scope is proof/evidence only: one new workshop integration referee, its two inventory rows, this report, the current-evidence row, the append-only anchor reach log, and the regenerated sanctioned-surface digest if required by freshness. There is no production Rust, WGSL, exact-Q, canonical `T_s`, persistence, consequence, qualification, canon, workflow, CI implementation, pointer, handoff, graduation, 15.6, 15.7, compression, or closeout change.

## Validation

- `cargo test -p simthing-workshop --test recursion_axis_conformance_0 -- --nocapture`: PASS, 2 passed on the real GPU resident path.
- `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet`: PASS.
- `cargo check -p simthing-workshop --all-targets`: PASS.
- `rustfmt --edition 2021 --check crates/simthing-workshop/tests/recursion_axis_conformance_0.rs`: PASS.
- anchor integrity, sanctioned-surface digest freshness, detachability, test inventory, lifecycle schema, and inventory drift checks: PASS.

The branch is based on exact admitted master `0796a95e2080c8ece6428685f971e9f8d60e859f`. These green checks establish the evidence packet; they do not discharge the active Owner decision or the remaining 15.5 proof obligations.

## Decision required

The Owner must rule commitment-class semantics for E6: whether immediate-flow work is work-conserving across an unserviceable higher band, whether explicit entitlement commitments reserve that capacity, and whether the two classes intentionally differ. Only an explicit Owner ruling can release this branch to implement E6 and complete the remaining 15.5 cross-product, semantic-scope, prepared-vs-executes, forbidden-shape, and parity proofs.
