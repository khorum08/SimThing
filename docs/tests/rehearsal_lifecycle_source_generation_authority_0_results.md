# Source-generation authority — sparse crossings are lawful

DA increment per Orchestration relay `5720895186` (Astra return `5715569466` accepted).
**Model 1 remains UNDECIDED; Model 2 remains CLOSED.** Ninth admitted substrate gap of
the track, and — as relayed — a *generic clock-domain* defect, not construction
semantics.

## The defect

`GenerationBoundCrossingDedupe` froze an affine `source = facility + offset` association
at the first observed crossing. The two clocks advance on different events **by
construction**:

- `ActionBandExecutionFacility::generation` advances once per successful **non-empty**
  dispatch — a dispatch ordinal / session facility clock;
- simulation generation advances every boundary, and `SimSession` dispatches consequences
  only when crossings exist, so quiet boundaries deliberately manufacture nothing.

A crossing at G2, quiet G3/G4, then a lawful crossing at G5 therefore refused with
`CrossingGenerationMismatch { expected: 2, actual: 5 }` — in all 8 of Astra's sessions,
including ample-capacity controls that never exercised placement refusal.

## The admitted successor law (all ten relay rules)

**The sealed crossing's own SOURCE generation is authoritative for crossing identity; the
facility ordinal is never consulted for identity** (rules 1–2). The dedupe keeps a
monotone watermark over accepted source generations:

- all keys in one dispatch must share one source generation — one dispatch settles exactly
  one evaluation generation; a mixed batch is a typed mismatch;
- **above** the watermark: a fresh exactly-once window opens. Sparse gaps are lawful and
  require no manufactured intermediate dispatch (rules 3–4);
- **equal** to the watermark: the retained window applies, so a genuine duplicate (same
  source generation + same admitted band slot/event identity) stays suppressed **even
  across separate dispatches** (rule 6);
- **below** the watermark: regression, refused fail-closed with the new typed
  `CrossingSourceGenerationRegressed` (rule 7).

`observe_boundary` is **deleted** rather than rebased onto the ordinal — under source
authority it has no work, and its removal forecloses re-synchronizing the window from the
facility clock. No source time is restamped or collapsed (rule 5); sealed provenance,
participant/band mapping, consequence authority, the outcome fence, the single installed
facility, and all foreign/malformed envelope refusals are untouched (rule 8); no
construction-specific map, second ledger, replay, or alternate facility (rule 9); no new
association had to be threaded into the facility at all, so the ordinal stays orthogonal
(rule 10).

## Witness floor (executed, reference machine)

`sparse_source_generations_execute_once_each_without_clock_pumping`, through the
**production** `bind_generation_authority` mint (EVENT-GENERATION-STAMP-0) — no synthetic
key, no test-only ingress:

- **G2 → quiet G3/G4 → G5**: both execute exactly once at their actual source
  generations; the dedupe window carries `Some(2)` then `Some(5)`, never the ordinal.
- **Gap-length sweep**: run for gap 3 (sparse) and gap 1 (contiguous G2→G3 control);
  authored order invariant in both.
- **Quiet boundaries**: `dispatch_sealed_and_apply` returns `None` and the facility
  generation is asserted *unchanged* across the quiet span — no clock pumping.
- **Duplicate** at the accepted source generation: `DuplicateCrossingConsumption`.
- **Regression** (accepted G5, then G2 replayed): `CrossingSourceGenerationRegressed
  { accepted: 5, actual: 2 }`, and the accepted window neither rewinds nor clears.
- **MUTANT RED**: restoring a next-ordinal-only admission rule reproduces the exact
  mismatch shape (`expected: 3, actual: 5`); restored, green.

**Existing batteries GREEN**: owning `actionband_overlay_actuation_0` 3/3,
`actionband_gpu_execution_0` 3/3, `actionband_recursive_composition_0` 2/2,
`actionband_semantic_shadow_0` 11/11.

### Disclosed referee update

The owning battery's replay referee previously asserted the affine model directly: after
one dispatch it expected the window restamped to the facility's next ordinal
(`Some(1), 0`) and the replay refused as a mismatch *against that ordinal*. Under the
successor law the window is keyed by the crossing's own source generation and retained, so
the identical replay is refused as what it actually is — a **duplicate consumption**. The
refusal is preserved and strictly better typed; the change is annotated in place at the
assertion. This is the same disclosure discipline applied to the score-and-bands referee
in the independent-resource increment.

## E8 accounting — no roll due

Per the relay's instruction not to infer exemption, seal membership was **verified**: the
31-entry `crates/simthing-gpu/build.rs::COMPONENTS` list contains no `action_band` file.
The diff touches exactly `crates/simthing-driver/src/action_band_consequence.rs` and its
owning test; `boundary.rs` and `session.rs` are untouched. **No sealed byte moves, so no
E8 roll is due** — the qualification pin remains `0xec5a_2a30_afae_e795` (thirteenth roll).
