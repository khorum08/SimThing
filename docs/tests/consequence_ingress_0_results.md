# CONSEQUENCE-INGRESS-0 results

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED**.
Coding has not merged, graduated the rung, moved the pointer, started 15.4,
begun compression, or closed the track.

## Provenance and adjudication

- Board handoff: `5528574555`
- exact branch base: `54a9e4db7df0976ef4a3a2cb7649d43fa5101caa`
- branch: `codex/consequence-ingress-0`
- `HD-RECEIPT: 914592a5c6e4`
- `ORIENT-RECEIPT: 9eb65c6f1220`
- orientation rule stamp: `44f61b2ae1a75dc7`
- orientation digest:
  `b1c49cdeaff3b38e3b373a13fc9372b409777e4600f05931878cdefc23db5f37`
- expected return route: `DA-RESERVE(binding)`

Engineering adjudication: **the presumption is supported**. The 15.0 chain is
a lawful secondary consequence of already-created U, has an existing admitted
EML/CostBand/Overlay implementation, and lacked only authored lowering plus a
production application consumer. It is not an oracle-only instrument. The
implementation therefore restores that consumer without altering exact Q,
canonical `T_s`, or the separate first-order Current-to-Next recurrence.

## Archaeology map

| Surface | Existing authority | 15.3 disposition |
|---|---|---|
| filter U | `ConstrainedGrant::unresolved` and `UnresolvedDemandObservation` after the one constrained clear | accepted only as an input; 15.3 cannot mint a grant or observation |
| valuation | `AuthoredPersistenceValuation` owns one admitted `TransformOp` plus CostBand unit cost | ordinary ClauseScript `script_value` now lowers to this exact type |
| funding | `fund_unresolved_persistence` evaluates U, enforces later generation, and calls the existing scalar `cost_band_quantize` | unchanged; now has one named non-test production consumer |
| binding | `PersistenceOverlayBinding` fixes origin, target, transform, and authored dissolution conditions | passed unchanged into the existing funding function |
| overlay birth | `dispatch_until_dissolved` plus `admit_dispatch_minted_overlay` mints the ordinary instruction Overlay | unchanged; zero draw remains a successful no-overlay consequence |
| runtime delivery | `RoutedOverlayDelivery` submits `BoundaryRequest::AttachOverlay` through `FeederSender`; the TreeMaintainer uses ordinary routed admission/activation | reused directly; no second overlay route |
| ClauseScript | `parse_script_value` produces `RateFormulaSpec`; `append_value_formula_ops` preserves ordered `add`/`mult`/`floor_at`/`ceil_at` modifiers | one shared private persistence EML builder now feeds both the 15.2 sealed port and the 15.3 consequence valuation |
| shared EML evaluator | `TransformOp::apply_with_params` uses `eval_overlay_eml` | archaeology-driven repair adds execution of already-admitted `MIN`/`MAX`; before repair a ClauseScript ceiling left the ceiling literal on top of the stack |
| 14.1 instrument | calls `fund_unresolved_persistence` only to measure and drops the overlay | remains an instrument; not counted as the production consumer |
| 15.2 deformation | `PersistenceDeformationProgram` is consumed only inside the one Current-to-Next mint | remains type-distinct and absent from every 15.3 production function |

## Authored lowering and consequence call graphs

```text
authored ClauseScript scenario / script_value
  -> ordinary parse_script_value
  -> RateFormulaSpec ordered modifier chain
  -> shared private persistence EML builder
  -> TransformOp::admit_eml
  -> AuthoredPersistenceValuation::new(unit_cost)
```

```text
ordinary recursive filter clear at generation N
  -> ConstrainedGrant.unresolved U
  -> UnresolvedDemandObservation
  -> submit_clause_persistence_consequence_script_value (Studio application door)
     -> compile_persistence_consequence_script_value
     -> submit_authored_persistence_consequence (driver consumer)
        -> fund_unresolved_persistence at N+k, k>0
        -> AuthoredPersistenceValuation(U) -> CostBand
        -> PersistenceOverlayBinding -> admitted Overlay born
        -> existing RoutedOverlayDelivery -> existing feeder boundary
        -> ordinary TreeMaintainer admission/activation
```

There is exactly one public Studio application door and exactly one public
driver consumer in their dedicated modules. ClauseThing lowers one-way into
the admitted spec type. Core, spec, kernel, sim, GPU, feeder, and driver retain
zero ClauseThing dependency arrows.

## Focused end-to-end transcript

The focused referee parses an authored scenario containing an ordinary
`script_value` (`base=1`, `add=1`, floor 0, ceiling 64), clears a request of 5
against supply 2 at generation 10, and observes U=3. The authored valuation
returns 4; CostBand at unit cost 2 produces `n=2, r=0`; the binding mints one
UntilDissolvedWith overlay; the existing boundary attaches it with source
generation 11 and activation generation 12; the ordinary evaluator observes
the authored +2 target effect.

```text
running 2 tests
test authored_scenario_u_costband_binding_and_overlay_are_one_live_chain ... ok
test consequence_only_types_and_symbol_census_reject_every_feedback_route ... ok
test result: ok. 2 passed; 0 failed
```

The same-generation attempt returns the existing
`SameGenerationConsequence` and emits zero feeder work.

## Consequence-only negative matrix

| Forbidden route / plant | Mechanical result |
|---|---|
| valuation used as `RuntimeOwnerSiloDemandBucket` | compile-fail `E0308` |
| valuation used as `PersistenceDeformationProgram` | compile-fail `E0308` |
| `produce_runtime_rf_next_generation_demands*` in any of the three production functions | exact function-body census RED; planted call proven |
| Current-to-Next carry symbol in the consequence functions | exact function-body census RED; planted call proven |
| demand bucket, `ConstrainedClaim`, or `ConstrainedSupply` vocabulary in the consequence functions | exact function-body census RED |
| 15.2 deformation program/binding vocabulary in the consequence functions | exact function-body census RED; planted typed assignment proven |
| ClauseScript property operand / field-cache read | typed hydration refusal: persistence script values admit literal modifiers only |
| second public application or driver door | exact `pub fn` cardinality assertion RED |
| engine-to-ClauseThing dependency | seven-manifest zero-arrow assertion RED |
| same-generation reclear | existing typed `SameGenerationConsequence`; zero boundary work |

The result carrier has only observed generation, consequence generation,
CostBand draw, and optional Overlay. It has no demand, claim, carry,
deformation, PALMA/Gu-Yang, field-cache, descendant-walk, or response-plane
field.

## Symbol-keyed consumer census

Before 15.3, the constitutional registry had no persistence-consequence row
and `fund_unresolved_persistence` had only test and 14.1 instrument callers.
After 15.3, `PERSISTENCE-CONSEQUENCE-AUTHORING-INGRESS` pins exactly three
symbols:

1. ClauseThing `compile_persistence_consequence_script_value`;
2. driver `submit_authored_persistence_consequence`; and
3. Studio `submit_clause_persistence_consequence_script_value`.

The row is a production `overlay-ingress` route whose consumer evidence is the
Studio call into the named driver consumer. The existing legacy ClauseThing
lowerer census also increases from 23 to 24. Constitutional check and all 12
plants pass; the consumer cannot silently disappear or grow a sibling without
an explicit ledger change.

## Verification ledger

| Command / proof | Result |
|---|---|
| focused `consequence_ingress_0` | PASS — 2/2 |
| `simthing-spec` compile-fail docs | PASS — two new `E0308` refusals |
| frozen 15.0/15.1/15.2/14.5/14.6 and generation-critical-path batteries | PASS |
| touched-package checks and tests | PASS |
| test inventory / drift | PASS — 1403/1403, missing 0, unledgered 0, stale 0 |
| constitutional census / selftest | PASS — ingress 3, ClauseThing lowerers 24, planted 12 |
| anchors/lifecycle, sanctioned-surface freshness, detachability, Agent Scan | PASS |
| full structural certificate | PASS — ZERO RED |
| hosted Doctrine Scan/Exec, exact-head clearance, relay-lint | recorded in the immutable Board return packet |

## Changed-file census and fences

Production changes are limited to one ClauseThing lowerer/re-export, one
driver consumer/re-export, one Studio application door/re-export,
the two-op execution closure in the existing shared EML evaluator, and typed
compile-fail documentation on the existing valuation type. Proof changes are
one focused workshop referee, this report/current index, and the existing
constitutional/test/reach data ledgers plus regenerated sanctioned-surface
digest. No workflow, CI shell/Python, clearing shader, Q comparator, canonical
`T_s` carrier, Current-to-Next mint, 15.2 port, Triad authority, or tree/physical
identity mechanism changed.

Final exact head, PR, hosted run IDs, clearance, and relay-lint are recorded on
the Board because those identities exist only after this report is committed.
