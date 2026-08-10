# ACTIONBAND-SEMANTIC-SHADOW-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `coding/actionband-semantic-shadow-0`
- Base: `80e61a7062598486be1caf929617ac2037af2c34` (A1 amendment merge #1718)
- HD-RECEIPT: `edeb59f58239`
- ORIENT-RECEIPT: `98a916672d1a`
- orientation_rule_stamp: `3e2afa381d2aea10`
- expected_route: `DA-RESERVE(gate-wiring)`
- Dispatch: `5246978127`
- DA pre-dispatch: `5246937280`

## Field-neutrality gate

**Outcome: `FIELD-NEUTRAL`**

Inspection of existing opaque products (`StructuralCommitment`, GPU numeric
fingerprints, admission-time `ActionBandSemanticShadow` labels, closed target
forms) found no PALMA-only public/readback semantics (no
progress==PALMA-progress, no CostBand==throughput, no sole-field-PALMA
requirement). The new post-authority projection types are field-class-neutral
opaque keys only.

**A1 positive proof:** synthetic bound-observable identity
`synthetic-rf-grant-axis-v1` (not PALMA-derived) is carried, reported, and
round-tripped through `project_semantic_readback` without special-casing. No
Gu-Yang/7.5a implementation.

## Product

`simthing-driver::action_band_semantic_shadow` projects sealed ActionBand
structural products into CPU semantic readback **after** GPU authority:

- designation from admission-time semantic shadow (post-authority metadata)
- `resolve_owner` with exact `OwnerResolutionError` propagation (no alias to `unowned`)
- `GenerationStamp` travels beside owner/designation; stale stamps fail closed
- field-neutral `BoundObservableIdentity` provenance
- `ActionBandTransitProjection` for presentation consumers (12.5 icon obligation)
  without modifying icon-layer sources

## Tests

| Test | Obligation |
|---|---|
| `field_neutrality_gate_is_field_neutral` | Gate outcome + existing shadow schema inspection |
| `a1_synthetic_non_palma_bound_observable_round_trips` | A1 positive non-PALMA identity through readback |
| `identity_blindness_labels_do_not_change_numerical_or_sealed_products` | Bit-identical numeric fingerprint + sealed products under label change |
| `readback_resolves_owner_with_generation_stamp_and_rejects_stale` | Owner + stamp; stale RED |
| `foreign_and_malformed_owner_do_not_alias_to_unowned` | Foreign owner error retained |
| `readback_reports_designation_after_authority_and_transit_projection` | Designation + transit projection after authority |
| `production_icon_layer_source_is_untouched` | Zero icon-layer change; no movement facility vocabulary |

Lib unit: field-neutrality constant + bound-observable round-trip.

## Batteries (local)

| Command | Result |
|---|---|
| focused 7.5 | 7 integration PASS |
| lib units | 2 PASS |
| inherited 7.1/7.2/7.3/7.4 | bound in relay |

## ANCHOR-ACK (load-bearing)

- `actionband-field-triad-authority@56cf5cdf2d2c`
- `actionband-native-authority-table@541a03cb00a1`
- Full REQUIRED-ANCHORS set carried from coding orient `98a916672d1a`

## Scope

Only 7.5 PROBATION. No 7.5a/7.5b/7.5c, 8.x, Vector CostBand, StemThing-B,
gate edits, or icon-layer source changes. Coding does not merge or move pointer.
