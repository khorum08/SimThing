# Digest — Mapping-System Suitability for FIELD-MOVIE-DATASET-0 (animated heatmap corpora)

**Status:** ENGINEERING-STUDY INPUT DIGEST (design authority, 2026-06-14). **Non-binding; not an
implementation authorization.** Produced for Grok / Gemini to author the engineering study for the
`FIELD-MOVIE-DATASET-0` horizon target. Grounded in the *current* mapping implementation, not prose.

**Read with (canonical sources, in order):**
1. `simthing_core_design.md` §1.1 (Anchor A — Wei), §3 (the cell-schema column model), §5–§8, **§7
   (the Movement-Front mapping system, end-to-end)**.
2. `workshop/field_world_model_horizon.md` (the adjudicated charter — §0 dual mission, §1.2 corrections,
   §1.3 ADDED, §2 `FIELD-MOVIE-DATASET-0` definition, §4 stop-lines).
3. `simthing_lewm_corpora_case.md` (the metrology thesis — why ground-truth columns are the JEPA
   coordinate system; the three-axis complexity control).
4. `clausething/ct_3b_4a_movement_front_heatmap_memo.md` (the canonical RF-pressure→heatmap vertical).
5. `clausething/ct_vertical_consumer_contract.md` (the live runtime pipeline + production APIs).
6. **Code (authoritative):** `crates/simthing-spec/src/spec/region_field.rs` (`RegionFieldSpec`,
   `ArenaPressureBindingSpec`); `crates/simthing-gpu/src/structured_field_stencil.rs`; the driver
   `FirstSliceMappingSession` / `open_from_spec` path; `invariants.md` §"Mapping (Sparse RegionCell)".

---

## 0. The horizon target, in one paragraph

`FIELD-MOVIE-DATASET-0` is the **schema-proving first slice** that exports a SimThing run as a
**field movie**: a per-tick animated heatmap (the Movement-Front field over a grid) plus raw field
layers, per-tick **events and actions**, causal ontology sidecars, and a manifest pinning
`seed + scenario + replay log + transform versions`, so the episode is **regenerable by construction**.
The corpus exists to be the **calibrated metrology instrument** for the LeWM/JEPA world-model scaling
study: the engine's ground-truth field columns play the role a token vocabulary plays for an LLM (a
fixed external coordinate system the model's loss can be decoded into). Export lives **outside
`simthing-sim`**; the sim stays semantic-free and export-ignorant.

**The question this digest answers for the study:** *is the mapping system, as built, the right source
for these animated heatmaps — and what must the engineering study design to turn its per-tick field
state into a corpus?*

---

## 1. What the mapping system already is (the field-movie generator, grounded)

The Movement-Front mapping system **is already an animated, ground-truthed, causally-annotated field
generator.** Per-tick, under opt-in (`MappingExecutionProfile::SparseRegionFieldV1`, default
`Disabled`), the pipeline is (consumer contract; `ct_3b_4a` memo):

```
RF arena bands (subtree reduces onto each Location/gridcell's flow column)
→ ArenaPressureBindingSpec on-device projection: (arena, sub_field) → (target_id, row, col)
   seeds the RegionField cell from resolved arena pressure (GPU-resident, NO readback)
→ StructuredFieldStencilOp / SaturatingFlux (Gu-Yang) propagates falloff across N4 neighbors
   (bounded horizon H ≤ 8/16; ping-pong; source_capped_normalized) ............ L1: the heat map
→ SlotRange Sum reduce cell → parent columns ............................ L2: strategic awareness
→ ai_will_do EvalEML (urgency = w_pressure·pressure + w_resource·resource) .. L3: interpretation
→ Threshold + EmitEvent → BoundaryRequest::AttachOverlay (CommitmentEffectSpec) . the decision
```

The whole `RegionFieldSpec` is the three-layer model in one struct: `pressure_binding` + operator +
`horizon`/`alpha_self`/`gamma_neighbor`/`source_col`/`target_col` (L1); `reduction` (L2);
`parent_formula` + `commitment` (L3).

**Why this is structurally ideal for a field-movie corpus — five assets the study should exploit:**

1. **The heatmap is the movie.** Each tick is one frame; the RegionField cell columns over the
   `(width, height)` grid are the frame's data. Multiple channels (suppression / threat / supply /
   `choke_output_col` / gradient) are co-resident columns — a multi-channel frame is a column bundle.
2. **The corpus ships its own probe labels (free).** The cell schema (`core §3`) splits **local causal
   state** (raw field values) from **inferred dynamics** (velocity / previous-value / pressure
   columns). These are *exactly* the LeWM latent-probe targets (pressure velocity, front position,
   convergence/attractor state, threshold-crossing proximity) — the engine computes the ground truth.
3. **Full causal provenance per cell.** Every heatmap value traces SimThing → property column → arena
   (the `pressure_binding` projection record) → reduction band → overlay → threshold. The pipeline
   *is* the sidecar; provenance is queryable, not reconstructed.
4. **Bit-exact determinism + replay = the provenance backbone.** The `record`/`replay` (ldjson) path
   means an episode manifest pins `seed + scenario + replay log + transform versions`; the episode is
   regenerable. Reproducibility is claimed via *regeneration from manifest*, **never** cross-GPU pixel
   identity.
5. **Interventions are native.** Overlays are the universal modifier; a do-operator counterfactual
   pair = two episodes identical except one admitted overlay, divergence measured per tick. This is the
   corpus's primary differentiator (observational corpora cannot produce it).

**Physics-cleanliness (the metrology payoff):** the generator satisfies P1/P2/P3 by construction (Wei,
core §1.1/§7). The field movies are rollouts of a system that *provably has learnable causal structure*
(P3 dissipation-to-attractors = LeCun's abstraction argument as world-physics; P2 symmetry = the
small-law license for small-model sufficiency). The render/visual layer is the only noise source.

---

## 2. Suitability verdict and the real gaps (export-side and scale-side, not paradigm-side)

**Verdict: the mapping system is the correct and well-suited source.** It is already per-tick,
ground-truthed, causally-annotated, deterministic, replayable, and intervention-native. There is **no
paradigm gap** — the heatmap the study wants is the heatmap the engine evolves. The work is **export
tooling and scale**, and the engineering study should be scoped around the following concrete gaps,
each with the constraint that binds it.

| # | Gap | Why it exists (grounded) | Binding constraint for the study |
|---|---|---|---|
| **G1** | **No exporter exists.** | `FIELD-MOVIE-DATASET-0` *is* the exporter; the mapping system produces field state, not episodes. | Exporter is a **tool/crate outside `simthing-sim`**; the sim never learns the words export/dataset/episode/frame. Export metadata is spec-layer, compiles away. |
| **G2** | **Grid resolution: implemented ≤ 10/32 per edge; corpus target 64×64–96×96 atlases** (charter §1.1 VRAM analysis). | `RegionFieldGridProfile` caps at `StandardSquare` (≤10) / `ExtendedSquare` (≤32); `request_atlas_batching` is **rejected at admission** (M-4 atlas Provisional/unimplemented). The canonical 200×200 is the *design* target, not admittable today. | Resolution to 64–96 needs **either** a widened single-grid cap **or** the deferred **atlas batching** (M-4) — its isolation policy (algebraic G=0 mask preferred; physical gutter fallback) and VRAM-multiplier reporting are pre-adjudicated in the mapping ADR. The study must pick and cost one. |
| **G3** | **Smoothed perceptual frames are a new transform.** | The engine produces **raw field columns**; the JEPA sees lossy heatmap *images*. | The render/smoothing transform is **`ApproximateDiagnostic`-class, deterministic-but-lossy, version-pinned**; every transform's identity + params recorded in the manifest. No exactness claim ever attaches to pixels; raw + annotation layers preserve source values losslessly. |
| **G4** | **Export readback vs the runtime no-readback discipline.** | The runtime path is **compact-evidence-only / no full-field CPU readback** (decisions never read values back). The exporter must read the *whole* field grid each tick. | Export readback is an **offline/diagnostic seam**, explicitly separate from the runtime decision path; it never feeds a decision and never becomes a default. Keep it `debug_readback`-class, opt-in, version-pinned. |
| **G5** | **Frame on skipped/clean ticks.** | Cadence (`EveryTick`/`EveryN`/`OnEvent`) + dirty-skipping = compute follows the wavefront; quiet regions don't recompute. | A movie needs a frame **every export tick** even when the field is clean: use `summary_policy = CachedUntilDirtyWithZeroInitial`; the export frame is the cached resident value. Define an explicit **export cadence** decoupled from compute cadence. |
| **G6** | **Multi-channel / multi-field composition.** | One `RegionFieldSpec` is one field; a corpus frame wants several channels (pressure, choke, gradient, supply…). | Compose co-resident RegionField/arena columns into a typed multi-channel frame; the schema carries `export_layer_id` / `export_semantic_group` / `annotation_fields` / `visual_default` / `tier_projection` / `training_visibility` / `policy_action_tags` (charter §2). |
| **G7** | **Scale for *agency* (corpus value lives here).** | The differentiator is agency (actors, economies, adversaries), not texture. The mapgen pentad fixture is 3×3/5 systems — toy. | Corpus-grade episodes need many gridcell Locations + many arena participants + factions; this couples to G2 (resolution) and to the §1.5 self-play league as the diversity engine (out of scope for `-0`, named downstream). |

**One non-gap to state explicitly so the study doesn't re-litigate it:** the engine is *not* a pure Wei
automaton — hierarchical reduce-up/disburse-down moves information across scales faster than any stencil
light cone, by design. The honest framing (case-file §3) is **"automaton core with hierarchical
aggregation, all ground-truthed."** This is a *feature* for the corpus (it can test cross-scale
causation that pure-CA corpora cannot), not a defect — but the sidecar must label which transport is
stencil (L1) vs hierarchy (L2) so a probe can distinguish them.

---

## 3. What the engineering study must design (scope of the requested study)

For `FIELD-MOVIE-DATASET-0` specifically (the narrow first slice: one existing RR/0080-class fixture,
one ≥100-tick episode — **no model, no optimizer, no default wiring**):

1. **Episode + manifest schema.** Directory shape (`frames/`, raw `layers/`, `events/`, **`actions/`**,
   `sidecars/`, `manifest`); the manifest fields pinning `seed + scenario + replay log + transform
   versions`; the channel/annotation schema fields (G6); episode-pair (`episode_pair/{base,intervention}`)
   support if cheap, else named for the next rung (charter §2/§1.3).
2. **The export-readback seam (G4).** How the exporter reads GPU-resident RegionField + parent-reduced +
   event/commitment columns each export tick without touching the runtime decision path; the
   `debug_readback`/diagnostic API surface; determinism of the read.
3. **The render transform (G3).** The version-pinned raw→smoothed-frame pipeline; its parameterization;
   how the manifest records it; the raw-layer pass-through that preserves source values.
4. **Frame/channel composition (G5/G6).** Export cadence vs compute cadence; cached frames on clean
   ticks; multi-channel frame assembly from co-resident columns; the L1-vs-L2 transport label (§2 note).
5. **Probe-label emission (asset #2).** Which cell-schema columns ship as ground-truth labels
   (pressure velocity, front position, convergence/attractor state, threshold-crossing proximity); the
   per-tick label stream format; Tier-1 measured Pareto-sensitivity / knee-event labels (charter §1.4)
   if cheap.
6. **Resolution decision (G2).** Quantify the single-grid-widen vs atlas-batching tradeoff for 64–96
   atlases, with the VRAM-multiplier reporting the mapping ADR already mandates; state which is in scope
   for `-0` vs deferred. (For `-0` a ≤32 grid may suffice to prove the schema; corpus-grade resolution
   is a named follow-on.)
7. **Provenance sidecar format.** How SimThing → column → arena-projection → reduction-band → overlay →
   threshold provenance is serialized per cell/per event so the corpus is queryably causal.

---

## 4. Hard constraints / stop-lines the study must honor (carried from the charter §4)

- `simthing-sim` **never learns** the words export, dataset, episode, frame, visor, or model; export is
  a spec-layer/tool concern that compiles away.
- **Bit-exactness split (binding):** simulation truth + raw field layers keep their exactness claims;
  **visual/rendered layers are deterministic-but-lossy, version-pinned, `ApproximateDiagnostic`-class.**
  No exactness claim attaches to pixels; reproducibility = regeneration from manifest, never cross-GPU
  pixel identity.
- **Opt-in / default-off** preserved (`MappingExecutionProfile`, `ResourceFlowOptInMode`); export adds
  no default wiring and no GPU decision-path change.
- **No model authority anywhere downstream:** the corpus is for a *proposer/scorer/anticipator*; the
  simulator stays authoritative; nothing bypasses `Threshold` + `EmitEvent` → `BoundaryRequest`.
- **Candidate F / exact sqrt:** any export-side gradient magnitude that ever feeds an exact gate routes
  through `m_jit_sqrt_f_exact`; native WGSL `sqrt` is diagnostic-only. (Heatmap seeding itself needs no
  sqrt.)
- **Consumer-pulled sequencing:** `FIELD-MOVIE-DATASET-0` proves the schema against an *existing*
  fixture *before* `CLAUSETHING-IMPORTER-0` carries it. Inert export hooks ahead of an exporter are the
  0.0.7.9 anti-pattern — admissible only as an openly recorded deviation, never the "safe default."

---

## 5. Bottom line for Grok / Gemini

The mapping system is **the right instrument and most of the apparatus is already built**: a
per-tick-animated, multi-channel, ground-truthed, causally-annotated, deterministic, replayable,
intervention-native field generator whose cell schema already *is* the probe-label list. The
engineering study is therefore **not a "can the engine do this" study** — it is an **export-and-scale
engineering study**: design the episode/manifest schema, the offline export-readback seam, the
version-pinned render transform, the frame/channel/cadence composition, the provenance sidecars, and
the resolution path (single-grid-widen vs deferred atlas) — all outside `simthing-sim`, all honoring the
bit-exactness split and the consumer-pulled, no-model-authority stop-lines above.
