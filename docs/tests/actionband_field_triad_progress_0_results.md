# ACTIONBAND-FIELD-TRIAD-PROGRESS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.5a)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/actionband-field-triad-progress-0`
- Base: `153093821f1cae83623f07b9cd0f1e1351cf5a50`
- HD-RECEIPT: `1d744631ed8b`
- ORIENT-RECEIPT: `98a916672d1a`
- Handoff: Board comment `5248238142`
- Remand: Board comment `5248615928`
- Option-B remand: Board comment `5254001647`; DA adjudication `5248796666`

## Authority binding

The same 7.1 session-build door admits the closed authored source vocabulary
`None | RfGrant | GuYangAvailable | GuYangRealized`. An explicitly conserved
binding must select exactly one non-`None` source. Its frozen product carries
only the band's existing `ExistingThresholdRegistrationIndex`; it does not mint
a FieldSweep reference, crossing surface, field solver, or private throughput
model. Conserved EML remains lawful as `q_desired`. Driver lowering carries the
selected closed source only in the existing emission row's reserved
`auxiliary1`; the existing emission write clamps `q_exec` to the selected
signed native `post_value` after EML.

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

- the unchanged `2*x` EML produces an observed `q_desired == 2*q_flux`, while
  conserved progress remains bit-equal to native `q_flux`;
- capacity `1.0 -> 0.5 -> 1.0` produces progress down then bit-exact restoration
  with the target and EML unchanged;
- the signed negative fixture remains negative, proving no `abs(flux)` rewrite;
- actual `FieldSweepSession` observations report exactly two native Gu-Yang
  registration dispatches and one resident export per clean run;
- actual FieldSweep host readbacks: 0;
- duplicate/equivalent field solves: 0.

The RF result readback and sparse Phase-5 crossing readback are proof surfaces,
not raw-field CPU authority.

## Biting falsifiers

- explicit `None` and absent source fail admission;
- `GuYangRealized + RfGrant` on one conserved leg fails as a duplicate bound;
- a fifth `PrivateThroughput` source fails deserialization and wire code `4`
  fails the existing table-shape validation;
- a declared conserved leg cannot use structural-only compilation;
- reapplying a second source code to a conserved emission row fails lowering;
- an otherwise valid `2*x` EML payload amplifier on `GuYangRealized` admits and
  executes, but its conserved emission is clamped to signed native `q_flux`;
- an unbounded positive feedback gadget fails the existing bounded-feedback
  admission while its bounded/decaying counterpart admits on an ordinary,
  non-conserved ActionBand band;
- a planted rival invokes the real FieldSweep dispatch seam twice and its real
  host-readback seam once; actual observations change from `(2 dispatches,
  1 resident export, 0 readbacks)` to `(4, 0, 1)` and RED the clean witness.

## Schema preservation

The graduated 7.5 `BoundObservableIdentity` and
`FIELD_NEUTRALITY_OUTCOME == FieldNeutralityGate::FieldNeutral` schema is reused
unchanged. Human-readable labels do not select the bound source or numerical
dispatch path.

## Test budget

Four tests are admitted because each covers a distinct load-bearing contract:

1. `conserved_progress_source_is_closed_exactly_once_and_existing_threshold_bound`
   covers the closed source vocabulary, missing/double/private REDs, and binding
   to the existing threshold registration, including lawful conserved EML,
   closed wire codes, and rejection of a reapplied source.
2. `real_gu_yang_resident_output_bounds_rf_progress_without_duplicate_solve_or_cpu_mirror`
   covers the real resident Gu-Yang-to-Phase5-to-ActionBand-to-RF chain, capacity
   monotonicity/restoration, signedness, observed `2*x` desired payload versus
   native-clamped execution, actual execution observations, and the
   duplicate-solve/readback seam mutant.
3. `field_or_rf_recurrence_reuses_existing_bounded_feedback_admission` covers the
   required existing bounded-feedback contract and generation-pacing falsifier.
4. `field_triad_identity_uses_the_graduated_field_neutral_semantic_schema` covers
   preservation of the already-graduated semantic/readback schema.

No pointer movement, merge, self-clearance, or successor-rung work is claimed.
