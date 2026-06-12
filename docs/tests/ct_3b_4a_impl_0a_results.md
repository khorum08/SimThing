# CT-3b+4a implementation results — RF-fed movement-front heatmap spine

Status: **IMPLEMENTED / PASS — NO OPEN DEFERRALS** (2026-06-12; 0A spine + 0B closures:
CT-RF-EML-RATE-0, `value:` lowering, GPU projection, Line 3 session-loop integration, and the
**commitment-effects closure** — final addendum below. Frontier agent + executive design
authority.)
**The headline spine runs end to end from ClauseScript, including the mandatory CT-4a leg:**

```
ClauseScript fixture (ct3b4a_headline.clause)
→ hydration → GameModeSpec (CT-2c economy + RegionFieldSpec + pressure binding + ai_will_do)
→ install: BaseFlowObligationSpec seeds IntrinsicFlow (static modifiers folded: 6 × 1.75 = 10.5)
→ GPU Resource Flow arena bands resolve flows
→ admitted arena-to-cell projection reads the farmer's resolved IntrinsicFlow (bit-exact 10.5 seed)
→ bounded stencil heatmap (Normalized, horizon-capped, seed-then-zero)
→ Layer-2 Sum reduce to parent summary
→ ai_will_do field_urgency EvalEML over reduced parent columns (authored weights)
→ authored threshold crossing fires with the authored event kind (GPU Pass-7 scan)
→ BoundaryRequest::AttachOverlay commitment applied through apply_structural_mutations
   onto the acting SimThing
```

CPU-oracle parity holds at every computed seam; the counterfactual run (no arena pressure)
stays below the threshold — **the commitment genuinely depends on Resource Flow pressure**.

## Named consumer

CT-3b+4a headline vertical (amended ORIENT-0): the SEAD/suppression-disruption heatmap spine over
Resource Flow arena pressure, plus the mandatory `ai_will_do → Layer-3 EML → threshold
commitment` leg. This 0A slice is the substrate + spine proof; the exact full-rung remainder is
listed under **Remaining exit blockers**.

## Spec/admission surfaces added

1. **`ArenaPressureBindingSpec`** on `RegionFieldSpec` (`pressure_binding`, serde-default):
   `{ arena, source: IntrinsicFlow|AllocatedFlow, placements: [{target_id, row, col}] }`.
   Admission (`compile_region_field_preview`): named arena, non-empty bounded placements
   (≤ cell count), in-bounds rows/cols, no duplicate cells — all spanned `SpecError`s; the
   out-of-bounds rejection is test-proven. Spec presence enables nothing.
2. **`ai_will_do` weights** on `RegionFieldFormulaBindingSpec` (`weight_pressure`,
   `weight_resource`, serde-default `None`) — the Layer-3 personality weights are spec-visible
   authored data, no longer harness constants.

## Driver surface added

**`arena_pressure::project_arena_pressure_seeds`** — resolves each placement through the live
install (scenario install target → admitted arena participant slot via the participant scaffold →
flow-property column via `resolve_node_columns` + `column_range`) and reads the projected value
from resolved session values. Boundary-time **consumption** of GPU-resolved state. Hard errors:
unknown arena, unknown target, target not admitted in the arena, non-finite pressure. Multiple
hosted participants per target sum deterministically.

## How RF arena pressure feeds the heatmap

Install-seeded folded obligations → GPU arena reduce/allocate bands → projection reads the bound
participant's `IntrinsicFlow` column (proven bit-exact: seed = 10.5 = the CT-2c folded effective
rate) → seeds the admitted region field under the caller-managed one-shot-seed-then-zero source
policy → `StructuredFieldStencilOp` propagation (existing WGSL kernel, bounded horizon) → Layer-2
Sum reduction → Layer-3 EML. No `contributions` side-channel, no hand seeds, no presentation map.

## The CT-4a leg (mandatory, included)

`ai_will_do` lowers at hydration to the `field_urgency` formula binding with authored weights;
the runtime's EvalEML (`ExactDeterministic`, 8 nodes ≤ 32) computes
`urgency = weight_pressure × reduced_pressure + weight_resource × reduced_resource` on GPU over
the parent slot; the authored `FirstSliceCommitmentSpec` threshold (16.4, event_kind 7) is
scanned GPU-side; the crossing event drives a `BoundaryRequest::AttachOverlay` commitment applied
through `simthing_sim::apply_structural_mutations` onto the acting SimThing (overlay verified in
the tree). Threshold discrimination is proven both ways (fires with RF pressure at urgency
≈ 16.80; counterfactual floor 16.0 stays silent).

## CPU/oracle parity & exactness classification

- **RF leg:** the folded-rate install path is the CT-2c-REMEDIAL-3 substrate (bit-exact oracle
  proof already standing); the projected seed is asserted bit-exact (`to_bits`).
- **Heatmap + urgency:** CPU oracle replays the runtime's exact sequence — seed write, one
  source-setup step (`cpu_stencil_step`), seed-then-zero, `cpu_horizon(horizon)` — then the
  Layer-3 formula. GPU vs oracle asserted within 1e-3 at the diagnostic readback seam
  (consistent with the standing first-slice commitment fixtures). Classification:
  **GpuVerified vs CPU recurrence oracle**; no new exact-authority claim is made at this seam.
- **GPU/JIT/WGSL used:** the existing `StructuredFieldStencilOp` WGSL kernel, the existing
  `EvalEML` interpreter (field_urgency tree), the existing arena reduce/allocate kernels, and the
  GPU threshold scan — all bounded arithmetic over admitted buffers. **No new kernel-side code
  was needed for 0A**; the lifted-WGSL allowance was exercised through the existing kernels.
- **Exact GPU sqrt rule:** not applicable — no sqrt, magnitude, distance, or gradient norm
  anywhere in this slice. A future gradient-magnitude consumer must route exactness through
  `m_jit_sqrt_f_exact`.

## Addendum — CT-RF-EML-RATE-0 IMPLEMENTED (same day, follow-up PR)

The largest 0B blocker is closed. Trigger-gated produces/upkeep now lower to a **per-tick
`EvalEML` effective-rate band at OrderBand 0, with every arena reduce/allocate band shifted up
one** — the binding ordering ("rate before reduce") is structural, not scheduled by convention.

- **Authoring:** `gated { trigger { property = ns::name at_least = T } <economic_key> = N }`
  inside `produces`/`upkeep` → `GatedRateSpec` on `ResourceFlowSpec.gated_rates` (admission:
  finite rates/thresholds, named arenas, ≤4 terms per arena target to hold the ≤32-node budget);
  `trigger_property` blocks register the watched property. Gated pairs carry an immutable
  `rate_base` sub-field.
- **Semantics:** `intrinsic = (folded_base + Σ add×gate) × (1 + Σ mult×gate)`,
  `gate = trigger ≥ at_least` computed **inside the EML tree** (`SLOT_VALUE`, `LITERAL_F32`,
  `CMP_GE`, `MUL`, `ADD` — all exact-class opcodes). Gated terms compose after the static
  CT-2c fold; the folded base column is written once at install and never mutated.
- **Proof (GPU, bit-exact `to_bits` equality):** gate off holds the folded base (10.5); rising
  edge lands `(10.5 + 4) = 14.5`; a **held gate does not compound** across ticks; falling edge
  returns exactly to base; the base column is asserted immutable. Registered
  `ExactDeterministic`; no per-tick Add/Multiply overlay touches any rate column anywhere.
- **`value:` formula trees** remain spanned hard errors: the EML band they lower onto now
  exists, so the remaining work is pure formula-to-nodes lowering — a mechanical consumer-pulled
  extension, no longer a substrate gap.

## Remaining exit blockers (exact, for the full CT-3b+4a 0B closure)

1. ~~CT-RF-EML-RATE-0~~ — **implemented**, including `value:` tree lowering: `script_value`
   formulas (base + ordered add/mult/floor_at/ceil_at over literals and live property-column
   reads) hydrate from `value:NAME` references at both rate consumption points (gated
   magnitudes and always-on dynamic terms) and lower to formula subtrees on the same band.
   GPU proof bit-exact incl. the ceiling clamp and live input reads. Flat formulas only;
   recursion rejected at hydration.
2. ~~GPU-side projection copy~~ — **implemented** (same day): a generic indexed gather-scatter
   WGSL kernel (`simthing_gpu::IndexedScatterOp`, one dispatch, host-validated bounds, duplicate
   destinations rejected, CPU oracle) moves session values buffer → stencil input buffer
   on-device; the mapping runtime gained a GPU-seed path that runs the one-shot
   seed-then-zero sequence with **zero host value writes**. Proven **bit-identical** against the
   0A CPU projection through the full heatmap → ai_will_do → commitment chain (threat, urgency,
   and threshold events all `to_bits`-equal). **The gadget composition hook:**
   `PressureSourceSpec::Named { sub_field }` projects *any* named flow-property column — a
   session EML/gadget op writing a named column makes that column heatmap feedstock, so arena
   state shapes the spatial field through authored formulas with no new interpreter: behavior
   stays data over EvalEML (Anchor B), and the new WGSL is pure bounded data movement.
3. ~~Per-tick session integration~~ — **implemented (Line 3, final addendum).**

## Addendum — Line 3 session-loop integration IMPLEMENTED (closes the rung)

`SimSession::run()` (and `record_to_path`) now execute the opt-in GPU work inside the
production tick loop:

1. **RF arena bands dispatch per tick** when `use_accumulator_resource_flow` is on — the bands
   (including the CT-RF-EML-RATE-0 effective-rate band at OrderBand 0) were previously only ever
   driven by test harnesses. `RunSummary.resource_flow_band_dispatches` counts them.
2. **The mapping chain runs per tick** under `SessionMappingState`, installed by
   `open_from_spec` iff the game mode authored `SparseRegionFieldV1` + exactly one region field
   + a pressure binding + ai_will_do weights + a commitment threshold: on-device indexed
   scatter (session values → stencil input), GPU seed-then-zero, bounded stencil propagation,
   Layer-2 reduce, ai_will_do EML, GPU commitment scan. **No value readback anywhere on this
   path.** Half-authored configurations are hard open errors naming the missing surface; the
   profile default stays `Disabled` and presence of `region_fields` alone wires nothing
   (test-proven).
3. **Commitment crossings are journaled** per tick (`SimSession.mapping_commitments`:
   tick + `ThresholdEvent` with the authored event kind) and counted in `RunSummary`.

Proof (`ct_3b_4a_session_loop.rs`, GPU): a 3-boundary `session.run(3)` over the ClauseScript
headline fixture dispatches RF bands and the mapping chain on every tick and journals authored
event-kind-7 crossings; the disabled-profile run installs nothing and journals nothing; the
stripped-binding open fails loudly.

~~Named deferral~~ — **closed by the commitment-effects addendum below.** PALMA/min-plus was
deliberately not touched — Line 3 needed no traversal utility.

## Addendum — authored commitment effects (track-closing PR, 2026-06-12)

The last deferral is implemented: `FirstSliceCommitmentSpec.effect: Option<CommitmentEffectSpec>`
authors the structural consequence of a crossing — `{ target_id, targets_property,
sub_field_deltas, lifecycle (closed set, v1 Permanent), once (default true — a decision latch) }`.
ClauseScript: `effect { target targets_property amount_add|amount_mult }` inside the `urgency`
block. Admission rejects unnamed targets, malformed property refs, and empty deltas.

At install, the session resolves the target (exactly one SimThing), the property column, and
seeds the effect property on the host (the overlay-compile contract). In the loop, when a
boundary arrives with new journaled crossings, the session builds the authored overlay
(`Custom("mapping_commitment")`, System source) and submits
`BoundaryRequest::AttachOverlay` through `tx.submit_boundary` — **the ordinary feeder channel,
drained and applied by the existing boundary structural machinery**, with the empty-boundary
fast path suppressed for that boundary. The once-latch consumes the journal watermark so held
crossings never re-fire.

Proof (GPU, in `ct_3b_4a_session_loop.rs`): across a 3-boundary run with crossings every tick,
the effect applies **exactly once**; exactly one commitment overlay sits on the acting farmer;
and the overlay's `Permanent` transform raises the authored `simthing::alarm` column on
subsequent GPU ticks (read back > 0). `RunSummary.mapping_commitment_effects_applied` counts
applications. The full authored loop is now closed: ClauseScript economy → RF pressure →
heatmap → ai_will_do → threshold → **authored structural consequence on the world tree** —
GPU decides, the boundary consumes, nothing recomputes.

## Files changed

- `crates/simthing-spec/src/spec/region_field.rs` — binding + weights specs
- `crates/simthing-spec/src/compile/region_field_admission.rs` — binding admission
- `crates/simthing-spec/src/lib.rs`, `src/spec/mod.rs` — exports
- `crates/simthing-driver/src/arena_pressure.rs` (new) + `src/lib.rs` — projection
- `crates/simthing-clausething/src/hydrate_category_economy.rs` — `region_field`/`mapping`
  dialect (urgency, pressure_binding, derived slot/column layout)
- `crates/simthing-clausething/tests/ct_3b_4a_headline.rs` + `fixtures/ct3b4a_headline.clause`
- Test-literal updates for the new optional fields (spec + driver test files, incl. the
  pre-existing `resource_flow_opt_in.rs` breakage from #592, fixed)
- This report; production ledger row

## Confirmations

No semantic GPU code (kernels see floats/indices; bindings resolve at admission/install). No
`simthing-sim` changes — it remains ClauseThing-, map-, and arena-blind (the commitment applies
through its existing generic mutation path). No runtime category dispatch (categories folded
away at hydration). No noun engines, no global mover registry, no CPU planner (the commitment is
a GPU threshold crossing; the CPU applies the structural result). Participants explicit and
bounded; Resource Flow and mapping both authored opt-in; `ResourceFlowSpec` presence alone
inactive (standing tests re-run green). Mobile concepts remain ordinary SimThings. No
Paradox/lab corpus. Stale CT-2c memo semantics not resurrected (no dead overlays; folded rates
consumed). No side-channel heatmap seeds.

## Tests run

```text
cargo test -p simthing-clausething --test ct_3b_4a_headline      # 2 passed (GPU spine proof ran)
cargo test -p simthing-clausething                               # all 11 suites green
cargo test -p simthing-spec --test region_field_spec_admission   # 26 passed
cargo test -p simthing-driver --test phase_m_first_slice_runtime                     # 28 passed
cargo test -p simthing-driver --test phase_m_first_slice_product_commitment_fixture # 7 passed
cargo test -p simthing-driver --test phase_m_first_slice_scenario_spec              # 9 passed
cargo test -p simthing-driver --test phase_m_first_slice_summary_validity           # 11 passed
cargo test -p simthing-driver --test resource_flow_opt_in                           # 13 passed
cargo test -p simthing-driver --test resource_flow_base_intrinsic                   # 3 passed
cargo fmt --all -- --check                                       # clean
```

`cargo test --workspace` — **not run**. Lab scans — not run (decoder corpus obligation remains
standing, deliberately not mixed into this PR).
