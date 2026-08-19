# CONTENTION-ARENA-EXECUTED-0 results

- Track: 0.0.8.7 RF arena modernization (rung 8.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/contention-arena-executed-0`
- Canonical live base: `dc24ddde4f3048d6a103196c74ac7b21e47c0a41`
- HD-RECEIPT: `5adea81721be`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- Dispatch: Board comment `5337657106`; DA ruling `5337607437`
- Expected route: `DA-RESERVE(gate-wiring)`

## Landed surface

`simthing_spec::clear_constrained_claims` is one scenario-neutral bounded
claim→clear→disburse mechanism for both fitting and oversubscribed postures.
Claims are constructible only from the existing
`RuntimeOwnerSiloDemandBucket`, so priority enters through the landed
`CommandDeficit` seam. Full `OwnerChannelScopeKey` values segregate supply and
claims; the executor neither compares owners nor reconstructs ownership.

`AuthoredClearingProgram` is the single numerical policy surface. Its admitted
EML reads existing order weight and landed priority. Constant, priority, and
price programs therefore produce proportional, priority-ordered, and
price-driven allocations with identical executor code and claim data. Physical
input order is non-semantic; exact proportional remainder placement uses stable
logical SimThing identity.

`UnresolvedDemandObservation` records `U = requested - granted` at clearing
generation N. `fund_unresolved_persistence` refuses N, evaluates U through an
authored EML valuation only at a later generation, passes that value through the
existing scalar CostBand, and produces an ordinary routed instruction overlay
only through `UntilDissolvedWith` carrying an authored condition. It emits no
claim and cannot re-clear in the same generation. CostBand remainder `R` stays
an independently observed quantity.

ActionBand-originated demand is deliberately represented as the same ordinary
RF aggregate and runtime demand shape. There is no ActionBand claim kind,
allocator, executor, or local clearing branch. The graduated 8.1 conservation
judge is unchanged and judges the executed grant rows.

## Focused exit proof

One focused integration test is admitted because it exercises the whole 8.2
constitutional boundary rather than separate helper facts:

| Test | Admission reason |
|---|---|
| `generic_constrained_clearing_is_authored_generation_paced_and_conserved` | Proves fitting and oversubscribed execution through one mechanism; program-data-only priority/price emergence; landed priority and full-key owner segregation; order-independent production versus a row-order mutant; ordinary ActionBand routing versus a local-shortcut mutant; distinct U and CostBand R; same-generation persistence rejection; authored `UntilDissolvedWith` delivery; and unchanged 8.1 GREEN/RED adjudication. |

Measured inline allocation with supply 6 and requests 4/1/4:

| Authored law | claim A | ordinary ActionBand claim | claim B |
|---|---:|---:|---:|
| landed priority | 4 | 1 | 1 |
| price/order weight | 1 | 1 | 4 |

The same claims with supply 9 receive 4/1/4 and report a fitting posture. The
proportional program is invariant under reversed input rows, while the
test-side sequential-row mutant changes recipients. Persistence observes
`U=3`, then at generation N+1 produces CostBand `V=3, C=2, N=1, R=1`; `U` and
`R` are visibly distinct.

## Persistence / attrition mechanism return

| Landed mechanism | Ready surface | Promotion blocker |
|---|---|---|
| unresolved-demand EML→CostBand overlay persistence | `UnresolvedDemandObservation` + `fund_unresolved_persistence` | DA must author the companion `scripts/ci/allow/contention_mechanisms.txt` row at rung close; governing DA ruling is Board comment `5337607437`; coding intentionally did not edit that registry. |

No other persistence or attrition mechanism landed.

## Fences retained

- No contention/combat service, participant taxonomy, resolution enum,
  arena-local owner plane, owner equality, second reconstruction, or parallel
  priority mechanism.
- No physical row-order policy, ActionBand-local clearing, vector/atomic
  multi-lane holding, StemThing-B, Phase 9+, pointer movement, or
  `OVERLAY-PEER-AUTHORITY` retirement.
- No CI implementation, workflow, scanner, allowlist, or contention-mechanism
  registry edit. DA owns the graduation companion row.

## Local evidence

The exact tested head and final command tails are recorded in the Board/PR
relay after the implementation commit. The focused test is
`cargo test -p simthing-driver --test contention_arena_executed_0`.

## Posture

Return **PROBATION / proof-present / DA-review-pending**. Coding does not merge,
advance the pointer, invoke graduation commands, or begin Vector CostBand,
StemThing-B, or Phase 9 work.
