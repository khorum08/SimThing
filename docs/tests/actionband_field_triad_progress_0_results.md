# ACTIONBAND-FIELD-TRIAD-PROGRESS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5a)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/actionband-field-triad-progress-0`
- Base: `153093821f1cae83623f07b9cd0f1e1351cf5a50`
- HD-RECEIPT: `e0e8fa9d2164`
- ORIENT-RECEIPT: `98a916672d1a`
- Handoff: Board comment `5248238142`

## Authority binding

The same 7.1 session-build door admits the closed authored source vocabulary
`None | RfGrant | GuYangAvailable | GuYangRealized`. An explicitly conserved
binding must select exactly one non-`None` source. Its frozen product carries
only the band's existing `ExistingThresholdRegistrationIndex`; it does not mint
a FieldSweep reference, crossing surface, field solver, or private throughput
model.

Driver lowering accepts a conserved-progress binding only on an existing native
next-state lane and seals its signed value through the existing Phase-5
`BandCrossingDelta`/ActionBand path. Structural-only lowering fails closed.

## Load-bearing proof

`real_gu_yang_resident_output_bounds_rf_progress_without_duplicate_solve_or_cpu_mirror`
uses the production `compile_gu_yang_n4_field_sweeps` result, preserving its two
ordinary `FieldSweepRegistration` rows (conductance then flux). The field result
is copied between GPU-resident buffers, consumed by the ordinary Phase-5
threshold registration, dispatched by ActionBand on its existing native lane,
and consumed by the ordinary RF accumulator.

- capacity `1.0 -> 0.5 -> 1.0` produces progress down then bit-exact restoration;
- the signed negative fixture remains negative, proving no `abs(flux)` rewrite;
- each run dispatches exactly the two native Gu-Yang registrations;
- raw field readbacks: 0;
- CPU numerical interpositions: 0;
- duplicate/equivalent field solves: 0.

The RF result readback and sparse Phase-5 crossing readback are proof surfaces,
not raw-field CPU authority.

## Biting falsifiers

- explicit `None` and absent source fail admission;
- `GuYangRealized + RfGrant` on one conserved leg fails as a duplicate bound;
- a fifth `PrivateThroughput` source fails deserialization;
- a declared conserved leg cannot use structural-only compilation;
- an unbounded positive feedback gadget fails the existing bounded-feedback
  admission while its bounded/decaying counterpart admits;
- a planted PALMA-potential/CostBand-affordability private throughput
  reconstruction is capacity-insensitive and REDs against the native Gu-Yang
  capacity result.

## Schema preservation

The graduated 7.5 `BoundObservableIdentity` and
`FIELD_NEUTRALITY_OUTCOME == FieldNeutralityGate::FieldNeutral` schema is reused
unchanged. Human-readable labels do not select the bound source or numerical
dispatch path.

## Test budget

Four tests are admitted because each covers a distinct load-bearing contract:

1. `conserved_progress_source_is_closed_exactly_once_and_existing_threshold_bound`
   covers the closed source vocabulary, missing/double/private REDs, and binding
   to the existing threshold registration.
2. `real_gu_yang_resident_output_bounds_rf_progress_without_duplicate_solve_or_cpu_mirror`
   covers the real resident Gu-Yang-to-Phase5-to-ActionBand-to-RF chain, capacity
   monotonicity/restoration, signedness, and residency counters.
3. `field_or_rf_recurrence_reuses_existing_bounded_feedback_admission` covers the
   required existing bounded-feedback contract and generation-pacing falsifier.
4. `field_triad_identity_uses_the_graduated_field_neutral_semantic_schema` covers
   preservation of the already-graduated semantic/readback schema.

No pointer movement, merge, self-clearance, or successor-rung work is claimed.
