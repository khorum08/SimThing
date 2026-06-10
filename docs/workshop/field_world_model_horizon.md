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
  (perturbed replays). The interactive/env form (observe, propose, admit, simulate, observe) is
  `FIELD-OPTIMIZER-0`'s loop generalized — the manifest schema therefore carries per-tick
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
- **Endogenous reflexivity (added 2026-06-10).** The rules of the simulation are themselves
  contested objects *inside* it: influence flows leaf→root→owner, crosses an aggregate threshold,
  and fissions the owner entity with an intensity-vector column partition (core design §2.1 —
  policy capture, secession, civil war; **subversion** as the fourth strategic vector). The corpus
  therefore contains **regime-shift episodes in which the policy layer is state, not constant** —
  the meta-dynamic a world model must master before any commercial (market/advertising) skin is
  credible, and prime material for surprise/OOD benchmarks.

### 1.4 Pareto-knee events and nonlinear decision cascades (added 2026-06-09)

Giovannelli, Raimundo & Vicente's *snee* approach (arXiv:2501.16993) formalizes the Pareto knee —
the point of **least maximal change**, where a small improvement in any objective forces a large
deterioration in another — via the maximal-change function MCF = max over objective pairs of the
sensitivity-norm ratio, minimized over the weight simplex, plus ellipsoidal most-changing
sub-fronts from the Jacobian pseudo-inverse. The mapping onto SimThing is exact: **their weight
simplex is the engine's policy-action space** (allocation weight columns + threshold biases,
§1.2(d)); their objectives are the Layer-3 personality-weighted pressure columns. A *knee event* is
the strategic state evolving into a sharpened trade-off geometry; a *nonlinear decision cascade* is
a threshold chain — i.e., the Pareto front developing a kink, where the MCF diverges.

**Two-tier detection architecture (both tiers inside existing doctrine):**

- **Tier 1 — engine-native, label-grade.** The paper's analytic machinery (implicit function
  theorem, Hessian inverses, an inner argmin per probe) exists because they cannot run the system;
  SimThing can. A counterfactual episode pair (§1.3) differenced on reduced objective columns **is
  a measured Jacobian column** — no smoothness or convexity assumptions. Over measured sensitivity
  columns the MCF is plain ratio/max algebra — **an EML gadget tree** — and a knee event is a
  `Threshold` + `EmitEvent` crossing: GPU-resident, FIELD_POLICY-compliant, no CPU planner. This
  tier is the certification authority and the **label generator** for Tier 2.
- **Tier 2 — LeWM-amortized, runtime-grade, `ApproximateDiagnostic` forever.** The differentiable
  JEPA predictor yields action-to-future-embedding sensitivity by one autograd call — a cheap
  surrogate for the MCF's derivative stack. Knee proximity has a learnable signature: local
  action-sensitivity blow-up plus rising disagreement across nearby latent rollouts; realized
  surprise spikes when the cascade fires. The sub-front machinery sharpens the planner: sample
  candidate policy vectors along measured max-change directions rather than isotropically.

**Caveat that cuts in the engine's favor:** the snee analytic guarantees require smoothness the
clamped/gated/thresholded dynamics do not have — but the verbal knee definition survives
nonsmoothness, and SimThing's interesting knees are precisely the nonsmooth ones (cascade
nucleation = front kink = diverging MCF), where the measured/learned route keeps working as the
analytic route stops.

**Visor closure:** the paper's decision-theoretic slot ("select the knee to be protected against
large trade-offs") is the faction visor. **Risk tolerance = distance-from-knee preference**: a
cautious faction responds to anticipated surprise by hedging toward the knee (the
worst-case-protected operating point); an opportunistic one deliberately operates on the steep face
of the front. Personality becomes a geometrically meaningful position on a measured trade-off
surface. The same geometry runs *inside* a polity: **owner-entity fission (core design §2.1) is
the intra-faction knee event** — the internal cooperation–defection front sharpening until the
polity kinks and splits — with the influence-velocity columns as its Tier-1 leading indicator.

Knee/cascade detection opens no consumer of its own: Tier-1 labels ride the dataset/intervention
rungs; Tier-2 probes join `LEWM-PROBE-0`; visor-conditioned knee response belongs to
`FIELD-OPTIMIZER-0`.

---

### 1.5 Interactive training and the self-play league (added 2026-06-09)

Product's proposed training paradigm, adjudicated: train LeWM/SIGReg on field movies with a mixed
diet — offline annotated episodes plus fully interactive episodes with the model "in the seat"
applying interventions as overlay pressure actions — culminating in a generational league of
lightweight models playing against each other with coalitions and betrayal, winners propagating up
a ladder. **Ruling: feasible and doubly useful, with three corrections and one architectural
split.**

**Corrections:**
1. **The offline half is never action-free.** LeWM is action-conditioned; the offline corpus
   carries logged behavior-policy actions in the `actions/` stream — FIELD_POLICY v1 and scripted
   doctrines ARE the behavior policy (the two-AIs ruling §1.2(c) paying off). The real offline/
   interactive distinction is whose actions: automaton's vs the model's own.
2. **50/50 is a curriculum endpoint, not a starting recipe.** Begin near-100% offline (breadth
   across ClauseThing-authored regimes); ramp interactive share as planning competence appears
   (the DAgger-style cure for offline distribution shift / compounding rollout delusion); hold a
   replay mixture against forgetting. The interactive half structurally requires
   `FIELD-OPTIMIZER-0`-class machinery — this paradigm is stages 3–5 of §3 matured, never a
   reordering of the ladder.
3. **Action-space dimensionality discipline.** Freeform full-resolution heatmap actions defeat
   short-horizon latent planning (CEM dies above tens of dimensions) and exceed admissible
   authority. Actions are **parameterized pressure primitives** (channel, center, radius,
   magnitude, falloff) or a coarse sparse grid — each compiling to an admissible overlay.

**The league split — visors propagate, gradients learn physics.** Agents = shared world model +
visor + planning budget. Per generation: **one shared world model trained by SGD on pooled league
data; a population of visors evolved by tournament selection** (PBT). Selection does preferences;
gradients do dynamics. A 12-faction match is one network with twelve conditioning vectors —
VRAM-trivial; "generations" are retained checkpoints (AlphaStar-league structure: past checkpoints
+ exploiter agents prevent strategy collapse and meta-cycling). Known hard problem, stated
honestly: self-play makes the environment non-stationary (physics + current meta); the fields
partially rescue this because opponents are observed *through* the heatmaps, and short horizons
are meta-robust — but continual retraining is expected, and league diversity is a data
requirement.

**Diplomacy: behavioral, not linguistic.** Coalitions and betrayal emerge from selfish planners in
n-player payoff structures given a small **discrete diplomatic action vocabulary** (alliance
overlays, tribute transfers, joint-defense weight-sharing — all already SimThing-expressible).
Betrayal is the tick where predicted defection value crosses predicted cooperation value — a §1.4
knee event in the cooperation–defection trade-off. Linguistic negotiation (Cicero-class) is out of
scope for 15M-class models and is not promised. **Subversion joins the league's action vocabulary
as a fourth verb** (influence emission per core design §2.1): visors gain a subversion-preference
axis, and the league's two knee events pair — betrayal (external, §1.4) and civil war (internal,
core design §2.1) — both predictable from the same sensitivity machinery.

**Why it is doubly useful:** (a) the agent path; (b) **the league is the corpus's diversity
engine** — an auto-curriculum generating an open-ended strategy ecology no authored scenario list
can match, with every episode Elo-stamped (skill-stratified strategic trajectories + causal
sidecars). Free bonus with teeth: a population selected for winning is an **adversarial fuzzer for
the simulator** — exploits become bug reports or OOD surprise benchmarks, made safe by the
admission-only authority discipline.

Eventual consumer name reserved (not opened, not authorized): `FIELD-LEAGUE-0`, downstream of
`FIELD-OPTIMIZER-0`.

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
at scale. **Product note (2026-06-10):** this slot now has a **named potential consumer** — the
Stellaris / Clausewitz-engine grand-strategy audience (players and modders) — giving the importer
genuine consumer-pulled standing; the cliodynamic instantiation evaluated 2026-06-09 is
back-burnered (see `../simthing_lewm_corpora_case.md` §4 footnote) and revivable on the same
lowering pipeline.

**`LEWM-PROBE-0`** — external Python harness, tiny model, small corpus; probes against
Movement-Front ground-truth columns **and Tier-1 measured Pareto-sensitivity / knee-event labels
(§1.4)**; surprise evaluation against perturbed replays. No SimThing runtime dependency, no
gameplay role.

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
- Odrzywolek — *All elementary functions from a single operator* (arXiv:2603.21852) — the EML
  discipline through which model proposals compile back to legal overlay/threshold data.
- Giovannelli, Raimundo, Vicente — *Pareto sensitivity, most-changing sub-fronts, and knee
  solutions* (arXiv:2501.16993) — the snee/MCF formalization underlying §1.4's knee-event and
  cascade-detection architecture; read 2026-06-09.
