# FIELD-WORLD-MODEL Horizon — Adjudicated Charter

**Status:** ADJUDICATED HORIZON CHARTER (design authority, 2026-06-09). **Not an implementation
authorization.** No consumer below is open; opening one requires explicit product/design
authorization per the consumer-pulled doctrine.
**Adjudicates:** the Codex design-horizon handoff "LeWM JEPA Horizon, Field-Movie Dataset, and
ClauseThing Deferral Decision" (received via session, 2026-06-09; original retained in the
product's Codex session — drop into `workshop/archive/` if byte-level provenance is wanted).
**Read with:** `simthing_core_design.md` (paradigm), `design_0_0_8_1.md` §0/§5 (doctrine + scenario
gate contract), `invariants.md` (gates).

---

## 0. The dual-mission frame (the governing correction)

SimThing is, and has always been intended as, two things sharing one substrate:

1. **A grand-strategy videogame simulation engine** — the consumer-facing product.
2. **A general-purpose, GPU-resident simulation ontology engine** whose runs constitute
   **interactive training corpora for AI**: deterministic, replayable, causally-annotated field
   movies with full ground truth.

The Codex handoff evaluated everything under frame (1) only, treating the dataset as
instrumentation for a game AI. Under frame (2) the corpus is a **product line**: every rendered
pixel carries complete, queryable causal provenance (SimThing → property column → reduction band →
overlay), which observational video corpora structurally cannot have and which most synthetic
environments cannot pin bit-exactly. All sequencing and valuation rulings below follow from this
dual frame.

---

## 1. Adjudication of the Codex handoff

### 1.1 ACCEPTED (binding as written, with engine-vocabulary normalization)

- **Authority discipline.** The learned model is proposer/scorer/anticipator only. The simulator is
  authoritative. Forbidden outputs stand verbatim: no direct state mutation, fleet movement,
  resource creation, combat results, ownership changes, or bypass of
  `Threshold` + `EmitEvent` → `BoundaryRequest`. A neural planner that mutates state is a CPU
  planner with extra steps; the FIELD_POLICY closure posture in `invariants.md` applies to it
  unchanged.
- **Model role.** Strategic field imagination module / policy scorer / latent rollout engine —
  never the whole faction brain, never a replacement simulation.
- **VRAM analysis.** ~15M params ⇒ runtime inference plausibly under 400 MB at 64×64–96×96 field
  atlases with small rollout sets; training explicitly outside that budget. Accepted.
- **Visual-grammar versioning.** A model trained on drifting render conventions learns renderer
  drift. All visual transforms are version-pinned in the episode manifest.
- **Difficulty ladder.** ClauseThing 1.0× / dataset exporter 0.8–1.5× / metadata-only 0.2–0.4× /
  probe 2–4× / optimizer 5–8× / playable AI 8–15×+research-risk. Accepted as planning priors.
- **The product correction on bit-exactness.** Recorded: heatmap *images* are smooth, lossy,
  perceptual by design. See §1.2(e) for the binding restatement.
- **Paper claims — verified against arXiv 2603.19312 (v2) abstract, 2026-06-09:** ~15M parameters;
  end-to-end from raw pixels; exactly two loss terms (next-embedding prediction + Gaussian-latent
  regularizer); single-GPU, few-hours training; latent probing of physical quantities; surprise
  detection of physically implausible events. Authors: Maes, Le Lidec, Scieur, LeCun, Balestriero —
  the JEPA-orthodox line. Repo: `github.com/lucas-maes/le-wm` (delegates pipeline to
  stable-worldmodel/stable-pretraining/Hydra/Lightning).

### 1.2 CORRECTED

**(a) Frame.** The dataset is mission-bearing product, not instrumentation (§0).

**(b) Sequencing — the consumer-pulled inversion.** The handoff's "safe" recommendation
(CLAUSETHING-IMPORTER-0 carrying *inert* export-metadata hooks, with no exporter existing) is the
0.0.7.9 failure mode by definition: substrate/schema built ahead of any consumer, validated by
nothing. Constitution §1.1 exists to kill exactly this. **Ruling:** the export schema is proven the
consumer-pulled way — a deliberately narrow `FIELD-MOVIE-DATASET-0` runs against the *existing*
RR/0080 fixture first and validates the schema through a real exporter; `CLAUSETHING-IMPORTER-0`
then carries a schema that has survived contact with a consumer. If product elects ClauseThing
first anyway, the inert hooks are admissible only as an **openly recorded deviation** from
consumer-pulled posture — never as the "safe default."

**(c) Two AIs, not one.** The engine-native, GPU-resident faction AI **already exists in
doctrine and does not wait on ML**: the FIELD_POLICY threshold automaton over Movement-Front
fields, with faction personality as authored sub-field properties applied as EML weighting overlays
(`simthing_core_design.md` §6–§8). That is faction AI **v1**. The LeWM-class module is a separate,
external, advisory **imagination layer** (anticipation the threshold automaton cannot do). Any
framing in which the game's AI is "pending the neural model" is rejected.

**(d) Vocabulary compiles down to existing mechanisms — no new mechanism class anywhere:**
- *Faction visor* = the existing personality-overlay doctrine (authored sub-fields conditioning
  EML weighting), optionally extended with a small adapter head on the external model side.
  Faction goals/memory/grudges/treaties stay explicit SimThing state and overlays — never weights.
- *Action space* = the allocation **weight columns + threshold biases that already exist**. A
  "policy pressure" action **is** an overlay transform parameter vector — giving the
  action-conditioned model a native, legal, low-dimensional action encoding for free.
- *Policy compiler* = **spec admission run in reverse**: model proposals enter as ordinary
  RON-shaped overlay/threshold candidates through the existing admission layer, then are simulated
  authoritatively. The handoff's hardest-rated component is mostly already built.

**(e) Bit-exactness split, stated bindingly.** Simulation truth and raw field layers keep their
existing exactness claims where claimed. Visual layers are **deterministic-but-lossy,
version-pinned, `ApproximateDiagnostic`-class by construction**; the manifest records every
transform's identity and parameters. "Smooth and perceptual" never means "unversioned or
unreproducible"; no exactness claim ever attaches to rendered pixels.

### 1.3 ADDED (material the handoff missed)

- **The interventional superpower.** Bit-exact determinism + overlay-as-intervention = native
  **counterfactual / do-operator datasets**: paired episodes identical except one admitted overlay,
  divergence measured per tick. Observational corpora cannot produce this; it is the corpus's
  primary differentiation and should be a first-class episode type
  (`episode_pair/{base,intervention}` with a shared seed + intervention record).
- **Replay substrate is the provenance backbone.** The existing `record`/`replay` (ldjson) path
  means an episode manifest pins `seed + scenario + replay log + transform versions`, making every
  episode **reproducible by construction**. Cross-machine reproducibility is claimed via
  regeneration from the pinned manifest — never via cross-GPU pixel identity.
- **Probe labels for free.** The Movement-Front cell schema (reduced causal-state columns +
  inferred attractor-dynamics columns) **is the probe target list**: the engine already computes
  the ground-truth quantities (pressure velocity, front position, convergence state,
  threshold-crossing proximity) that LeWM-style latent probing tests for. The corpus ships its own
  probe labels.
- **Corpus beyond training.** The same episodes are **evaluation suites**: world-model benchmarks,
  causal-probe batteries, surprise/OOD tests against engine-generated impossible transitions
  (perturbed replays). The interactive/env form (observe → propose → admit → simulate → observe)
  is `FIELD-OPTIMIZER-0`'s loop generalized — the manifest schema therefore carries per-tick
  **action/policy records** (an `actions/` stream beside `events/`), which the handoff's layout
  lacked and which an action-conditioned model requires anyway.
- **Provenance purity.** Fully synthetic, no third-party IP or personal data, complete causal
  ground truth: cleanly licensable corpus output by design.
- **ClauseThing's second mission.** Under frame (2), the importer is the **curriculum language**:
  distribution control over the corpus (geography, factions, doctrine, shocks, regime variation)
  *is* scenario authoring at scale. The two "rival" paths in the handoff are sequential stages of
  one pipeline.
- **Physics-clean corpus (theoretical note).** The generator is an automaton satisfying locality /
  symmetry / stability by construction (`simthing_core_design.md` §1.1, §7) — the structure causal
  world models must discover. Training a JEPA on these field movies is learning a system that
  provably has learnable causal structure, with the visual layer as the only noise source.

---

## 2. Consumer definitions (named; none open)

**`FIELD-MOVIE-DATASET-0`** — *the schema-proving first slice; recommended first.*
Scope: one existing fixture (RR/0080 class), one ≥100-tick episode; smoothed frames + raw layers +
ontology sidecars + per-tick events **and actions** + manifest pinning seed/replay/transform
versions; episode-pair (intervention) support MAY be included if cheap, else named for the next
rung. No model, no optimizer, no default wiring. Export machinery lives outside `simthing-sim`
(exporter tool/crate); the sim stays semantic-free and export-ignorant — export metadata is
spec-layer, compiles away like all semantics.
Success: deterministic episode structure; tick-aligned annotations; visuals lossy-by-design with
manifest-recorded transforms; raw/annotation layers preserve source values; episode regenerable
from manifest.

**`CLAUSETHING-IMPORTER-0` (export-aware)** — *the authoring + curriculum surface; second.*
As the handoff scoped it (limited syntax → parse → validate → lower through existing CLAUSE-SPEC
admission; no runtime semantic change; no ML), **plus** carrying the schema fields proven by
FIELD-MOVIE-DATASET-0: `export_layer_id`, `export_semantic_group`, `annotation_fields`,
`visual_default`, `tier_projection`, `training_visibility`, `policy_action_tags`. Curriculum
framing: the importer is how corpus diversity (seeds × regimes × doctrines × shocks) gets authored
at scale.

**`LEWM-PROBE-0`** — external Python harness, tiny model, small corpus; probes against
Movement-Front ground-truth columns; surprise evaluation against perturbed replays. No SimThing
runtime dependency, no gameplay role.

**`FIELD-OPTIMIZER-0`** — candidate policy-pressure vectors → latent rollouts → visor-conditioned
scoring → proposal → **existing admission** → authoritative validation run. No direct mutation, no
model authority.

**Never to be opened as named:** `LEWM-PLAYING-AI-0` (too broad), `NEURAL-PLANNER-0` (the name is
the failure mode). The path to a playing AI is the four consumers above plus the already-existing
FIELD_POLICY automaton — never one monolithic track.

## 3. Sequencing ruling

```
1. FIELD-MOVIE-DATASET-0        (narrow; proves the export schema via a real consumer)
2. CLAUSETHING-IMPORTER-0       (export-aware with the proven schema; curriculum surface)
3. corpus diversity generation  (ClauseThing-authored regimes; episode pairs/interventions)
4. LEWM-PROBE-0                 (external harness)
5. FIELD-OPTIMIZER-0 → policy proposals through admission
```

Product may elect 2-before-1 for product-surface reasons; doing so requires the §1.2(b) recorded
deviation and accepts speculative-schema risk knowingly. Nothing opens without explicit
authorization; faction AI v1 (FIELD_POLICY + personality overlays) proceeds independently of this
entire ladder.

## 4. Standing stop-lines (carried from the handoff, binding for any future rung)

Model output never mutates state, moves units, creates resources, resolves combat, flips ownership,
or bypasses `Threshold`+`EmitEvent`→`BoundaryRequest`. Visors never contain faction memory.
Smoothing/rendering transforms are export-layer tools, never gameplay mechanics, unless separately
admitted. `simthing-sim` never learns the words export, dataset, episode, frame, visor, or model.
Every closure answers *specified vs implemented*; every rung answers *which named consumer does
this unblock*.

## 5. References

- Maes, Le Lidec, Scieur, LeCun, Balestriero — *LeWorldModel: Stable End-to-End Joint-Embedding
  Predictive Architecture from Pixels* (arXiv:2603.19312; repo `github.com/lucas-maes/le-wm`).
  Claims verified against the v2 abstract 2026-06-09.
- Wei — *On the Spatiotemporal Dynamics of Generalization in Neural Networks* (arXiv:2602.01651) —
  why the Movement-Front substrate yields a physics-clean corpus (`simthing_core_design.md` §1.1).
- Odrzywołek — *All elementary functions from a single operator* (arXiv:2603.21852) — the EML
  discipline through which model proposals compile back to legal overlay/threshold data.
