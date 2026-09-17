# Frozen threshold-definition provenance — semantic definition is authority

DA increment per Orchestration relay `5721882717` (Astra terminal return `5721845969`
accepted). **Model 1 remains UNDECIDED; Model 2 remains CLOSED.** Tenth admitted substrate
gap of the track — and, as relayed, a *generic admission-provenance* gap, not construction
semantics.

## The defect

Tick-zero ActionBand commitments froze consequences against threshold **registration
indices**. `ActionBandCrossingBinding` recorded the bound registration's index, column and
`event_kind` — but **not the threshold value, slot, or direction**. The sole sealed-crossing
bridge, `ActionBandExecutionPlan::crossings_from_sealed`, then joined purely by index:

```rust
if band.threshold_registration != delta.reg_idx() { continue; }
```

Its own doc admitted the contract: *"this method performs joins only, never compares."*

So when the ordinary public threshold configuration path cleared and re-registered a bound
alert at a different value while the installed plan stayed frozen, production Phase-5
emitted real sealed crossings carrying the **rebuilt** threshold, and the frozen consequence
was routed through them. In Astra's terminal matrix that fired the second structural
consequence at receipt 1 instead of 1.5 — both product identities born, both receiving
committed placements.

## The admitted successor law (all ten relay rules)

**Semantic definition is authority; registration index is not** (rule 1). The admitted
threshold DEFINITION is frozen at tick-zero admission — value bits (identity, not
tolerance), slot, column, `event_kind`, and admitted direction — and carried beside the
band as CPU-only plan metadata. The GPU band row (`ActionBandBandGpu`, a `Pod` type) is
**unchanged**, so the GPU ABI does not move.

The bridge is now definition-first:

- a sealed crossing carrying the admitted definition **joins wherever the rebuild placed
  it** — index-order independent, so an identical rebuild or reorder stays lawful (rule 2);
- a crossing arriving at the frozen band's own registration index **without** that
  definition is the redefinition/remap case and **fails closed** with the typed
  `FrozenThresholdDefinitionStale`, before any consequence authority (rule 3);
- a crossing whose definition matches no frozen binding and whose index is not a bound
  index is simply not this plan's business — no consequence, no session-wide poisoning, so
  unbound/additive churn stays lawful without whole-registry byte equality (rule 4).

Direction is compared by *admissibility*, so an `Either` registration keeps accepting both
of its own crossings. Tick-zero commitment singularity stands — nothing recompiles,
rebinds, or reinstalls (rule 5). The canonical Phase-5 mint remains the only ingress; no
token, synthetic crossing, CPU comparator, restamp, or alternate facility (rule 6). The
#2071 source-generation/exactly-once law is untouched and orthogonal (rule 7); fail-stop
and pre-touch rejection semantics are unchanged (rule 8); one execution owner, no parallel
threshold→consequence map (rule 9).

## Witness floor (executed, reference machine)

`frozen_threshold_definition_is_authority_not_registration_index`, driven through the
**production** seal path with rebuilt registration sets (the ordinary configuration path
modelled honestly — clear and re-register while the plan stays frozen):

1. **Identical rebuild** of the bound definition → lawful, crossing joins.
2. **Redefinition before the crossing**, at both `0.75` and `1.25` (the relay's shapes) →
   typed `FrozenThresholdDefinitionStale` with the admitted and observed bits reported; no
   consequence, no birth.
3. **Reorder** — the bound definition re-registered behind an unrelated one, so it moves to
   a new registry index → still lawful and joined; the binding follows its definition.
4. **Additive unbound churn** at a non-bound index → zero crossings, no refusal, no
   poisoning.
5. **Remap** — a different definition occupying the frozen band's own index → fails closed.

**MUTANT RED**: disabling the gate routes a `threshold: 0.75` crossing through the band
that froze `1.0` and returns `Ok` with a live consequence batch — the premature-birth
mechanism reproduced verbatim. Restored, green.

**Existing batteries GREEN**: actuation, gpu_execution, recursive_composition,
semantic_shadow (20 tests); spec lib 36, kernel lib 6, sim lib 2.

## #2062 prerequisite evidence

Re-run of #2062's **UNCHANGED** suites at its tested head `aaf27a63`: **11/11 GREEN** —
the 8-test recipe suite including the terminal 46-session matrix, plus the 3-test WIP
suite. The four changed-threshold provenance negatives now refuse typed instead of
producing premature births. **Prerequisite evidence only, not a Model-1 verdict.**

## E8 accounting — no roll due, verified not inferred

Every changed file was checked against `crates/simthing-gpu/build.rs::COMPONENTS`
mechanically (each path grepped against the list): `action_band_admission.rs` (spec),
`action_band_execution.rs` (kernel), `action_band_execution_compile.rs` (driver),
`action_band_execution.rs` + `lib.rs` re-exports (gpu), and the owning test. **None is a
sealed component**; `session.rs` and `boundary.rs` are untouched, and the ingress shape was
deliberately NOT extended — the provenance law lives at the crossing bridge, so no sealed
byte moves. Pin remains `0xec5a_2a30_afae_e795` (thirteenth roll).
