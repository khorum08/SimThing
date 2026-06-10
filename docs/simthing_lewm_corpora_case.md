# SimThing–LeWM Corpora Case File

**Status:** HORIZON CASE FILE (design authority, 2026-06-09). **Non-binding, not an implementation
authorization, not mandatory reading.** Companion to
[`workshop/field_world_model_horizon.md`](workshop/field_world_model_horizon.md) (the adjudicated
charter; its §1.4 knee-event and §1.5 self-play-league rulings are assumed, not restated here).
This file records the evidentiary case, the prior-art audit, and the falsifiable hypothesis
developed in the 2026-06-09 design session. External claims marked [verified] were checked against
live sources on 2026-06-09; everything else is design-authority assessment.
**Amended 2026-06-10 (product decision):** the importer-slot instantiation is pivoted to the
**ClauseThing transpiler**, on the strength of a named potential consumer (the Stellaris /
Clausewitz-engine grand-strategy audience); the cliodynamics/cliometrics program is
**back-burnered** to the §4 footnote, revivable.

---

## 1. The instrumentation thesis — why JEPA scaling laws don't exist yet

The JEPA family's "bad rap" has two distinct halves with different cures:

1. **Collapse** (all inputs mapping to one embedding — the trivial minimum of latent prediction)
   was historically suppressed by heuristic scaffolds: EMA teachers, stop-gradients, DINO-style
   centering/sharpening, variance–covariance penalties. SIGReg (LeJEPA, Nov 2025) replaces the
   scaffolds with a principled distributional constraint — embeddings regularized toward an
   isotropic Gaussian — reducing the tunable loss hyperparameters to one. **Solved
   architecturally; SimThing is not needed for this half.**
2. **The scaling-law gap** has a deeper cause than instability: **autoregressive models are graded
   in a fixed external coordinate system** (cross-entropy against a frozen token vocabulary — the
   same gold standard at 10M and 100B parameters), while **a JEPA's loss is denominated in its own
   learned embedding space** — a currency the model itself prints, with different geometry at
   every scale. No fixed y-axis, no Kaplan/Chinchilla curve. The field's substitutes (downstream
   probing accuracy, planning success) are noisy, saturating, and task-confounded.

**SimThing supplies the missing coordinate system.** The simulator's ground-truth state columns
play, for a world-model JEPA, the role the token vocabulary plays for an LLM. This yields scaling
observables that are fixed, comparable, and non-gameable across model sizes:

- prediction error **decoded into ground-truth field space** (the cross-entropy analog);
- probe-recoverable information (linear-probe R² against known state columns) vs scale;
- surprise calibration against engine-generated impossible transitions (perturbed replays);
- planning regret against the FIELD_POLICY automaton;
- and a third controlled axis no natural corpus offers: **data complexity as an independent
  variable** (regime diversity, agent count, horizon depth, intrinsic dimensionality dialed by
  scenario authoring). Chinchilla had two axes; this substrate has three.

Plain statement of the thesis: natural data is an uncalibrated instrument; a deterministic,
ground-truthed, intervention-capable generator is a calibrated one. The corpus is not just
training material — it is **metrology for representation learning**.

---

## 2. The empirical-window audit (the "why hasn't anyone done this" check)

Run as an explicit anti-sycophancy audit on 2026-06-09, with searches. Findings:

**The study was ill-posed until ~7 months ago.** Pre-SIGReg JEPAs carried scale-sensitive
heuristic scaffolds that must be retuned per model size — any measured "exponent" confounds
scaffold tuning with architecture. The heuristic-free, scale-comparable object of study dates to
Nov 2025 (LeJEPA/SIGReg); its world-model instantiation (LeWM, arXiv:2603.19312 [verified: ~15M
params, two loss terms, single-GPU few-hours training, physical-quantity probing, surprise
detection; authors Maes, Le Lidec, Scieur, LeCun, Balestriero]) dates to Mar 2026.

**Internal falsification at Meta: undecidable from outside, with the layers split.**
JEPA-as-encoder is publicly validated at scale (DINOv2/v3, V-JEPA 2). JEPA-as-world-model-for-
planning stayed at proof-of-concept depth until LeWM. Meta's organizational pivot to LLM
superintelligence while LeCun's line was deprioritized is dual-interpretable (politics vs
discouraging internals) and indistinguishable from outside. Moderate evidence against a *known
devastating* negative result: AMI Labs' diligence-heavy syndicate (below). Weak evidence either
way; big labs bury negative results routinely.

**AMI Labs cannot build this instrument by thesis.** AMI launched 2026-03-10 with a $1.03B seed at
$3.5B pre-money (Cathay Innovation, Greycroft, Hiro Capital, HV Capital, Bezos Expeditions;
NVIDIA and Samsung strategic; Bezos/Cuban/Schmidt individual) [verified], CEO LeBrun framing it as
years-out fundamental research [verified]. Its differentiator is **world models trained on
real-world sensory data** — and reality has no ground-truth state vector. AMI faces the same
measurement problem as everyone (noisy downstream success rates); a synthetic metrology world is
off-thesis for a company whose pitch is "learn from the world, not simulations." **The labs with
the motive lack the instrument by thesis; the people with deterministic generators lack the
motive.**

**Robotics integrators (xAI, Figure, Unitree, Chinese unicorns) are structurally elsewhere.**
Their revealed bet is VLA stacks on LLM backbones (inheriting web-scale pretraining for free); the
JEPA path forfeits that inheritance — a research bet, not an integration bet. Their scarce
resource is real trajectories → research dollars go to data engines. Generative world models
(Cosmos/GAIA/World Labs class) are demoable and sellable; a JEPA's latent output is an embedding
you cannot show an investor. Cross-paradigm A/Bs are the genre industrial labs systematically
don't publish (partisan staffing; losses buried; wins dismissed on baseline quality).

**The race has quietly started — uniqueness claim trimmed accordingly.** Apple's SALT draws real
JEPA-class scaling instrumentation (accuracy-vs-FLOPs Pareto frontiers dominating V-JEPA's curve)
[verified] — within-paradigm, proxy-anchored. An OpenReview submission ("Semantic Tube
Prediction") claims a JEPA-style geometric prior yields 16× data-efficiency vs Chinchilla's data
term [verified; contested]. **The unoccupied slot is narrower than "first to measure JEPA scaling":
it is the externally-anchored, same-substrate, cross-paradigm exponent comparison with controlled
data complexity** — the one configuration that requires a deterministic ground-truth generator.

**Addendum (2026-06-10) — DeepMind buys into the asset class [verified].** On 2026-05-01 CCP Games
bought itself back from Pearl Abyss for $120M (cash + crypto), rebranded as **Fenris Creations**,
and **Google DeepMind took a minority stake ("millions") plus a research partnership**:
experiments on an *offline EVE Online running on a local server* to "test and evaluate models in a
controlled setting," with stated focus on long-horizon planning, memory, and continual learning;
Hassabis on record calling games the "perfect training ground," a DeepMind senior director saying
EVE demands skills AI "has not yet fully mastered." **Adjudication:** a frontier lab just put a
price on exactly this asset class — a player-driven adversarial macro-simulation as AI research
substrate — the strongest third-party validation of the dual-mission thesis to date. **What they
bought is an interactive evaluation arena, not a metrology lab:** an offline EVE offers full state
observability and agent testing, but no bit-exact replay, no do-operator counterfactual pairs, no
complexity dialing, and no causal ground truth (EVE's causal layer is famously out-of-band — the
server logs the kill, never the con). The unoccupied slot above survives, narrowed once more, now
with a market price on the adjacent asset. Two consequences: (a) the window estimate below
**tightens** — DeepMind now holds motive, an adjacent asset, and *both* world-model lineages
(Genie-class generative and the MuZero latent tradition); treat ~12 months as the planning
horizon, not 24. (b) EVE's two-decade **aggregate** series (publicly published Monthly Economic
Reports; alliance rise/collapse cycles) is a denser, genre-native candidate plug for the
back-burnered second-anchor program (§4 footnote) — empirical calibration without leaving the
Clausewitz consumer domain.

**Window estimate:** 12–24 months before AMI (which will eventually need exactly this budgeting
instrument for its own board) or Apple (extending Pareto work) publishes adjacent results.

---

## 3. The Movement-Front alignment — prior-art status and the technical core

**Prior-art search verdict (2026-06-09):** the field is circling the composition from three
directions without assembling it [verified]:

- **MIT (Lee, Han, Kumar, Agrawal; arXiv:2603.10055):** pre-pre-training LLMs on neural-cellular-
  automata-generated synthetic data **outperforms from-scratch and C4 pre-pre-training at matched
  token budgets** — the proposition "automaton corpora teach networks something transferable" is
  now empirical. But: autoregressive, tokenized 1D, *random* neural rules, no ground truth, no
  annotations, no interventions.
- **Causal-JEPA (arXiv 2026.02):** interventions entering the JEPA conversation — object-level
  *latent* interventions, not simulator-grounded do-operations.
- **Agent World Model (arXiv 2026.02):** synthetic-environment generation becoming a product
  category for agentic RL.

Nobody composes the full stack: **postulate-derived attractor automaton as generator +
ground-truth-annotated field movies as observation + latent prediction as learner + external
anchor as metrology.** Nobody in these threads cites Wei (arXiv:2602.01651).

**Why the alignment stays invisible:** (a) community siloing — Wei's paper lives in
generalization-dynamics theory, unread by the world-model/SSL crowd and the ALife/CA crowd alike;
(b) **inverted theorem reading** — the field reads "generalizing systems must be automata" as an
architecture prescription; the converse reading (an automaton's outputs are precisely the corpus
on which causal generalization is possible *and verifiable*) treats an architecture paper as a
corpus-design theorem, which nobody does; (c) **the substrate prerequisite** — the insight is only
actionable by someone who already owns a GPU-resident attractor-dynamics simulator with ground
truth and export. Discovery direction here was substrate→observer ("what observer does my world
deserve"), the inverse of the field's model→data direction.

**The technical core (why attractor dynamics are the JEPA-favoring regime):**

- **P3 (dissipation to attractors) is LeCun's abstraction argument formalized as world-physics.**
  In an attractor system the future is predictable in the abstract (which basin) and unpredictable
  in detail (which micro-trajectory), and the unpredictable part is exactly what dissipates. **The
  world itself performs the relevance partition** JEPA's loss bets on: AR burns capacity on
  transients the attractor erases; a latent predictor that learns basin geometry captures
  everything that persists — and whether it did is checkable, because the attractors are known.
- **P2 (symmetry: one shared rule everywhere) means the law is small even when the world is
  large.** Parameter demand scales with rule complexity, not map size; convolutional weight
  sharing *is* the symmetry postulate. P2 is the theoretical license for small-model sufficiency
  (the ~15M / sub-400MB regime) on arbitrarily large maps.

**Honest counterweights:** (1) the MIT result cuts both ways — random, meaning-free rules already
transfer, so attractor structure may not be *necessary* for the corpus effect; the differentiators
that survive are ground truth, causal sidecars, **agency** (actors, economies, adversaries —
meaningful causal structure, not just statistical texture), legal interventions, deterministic
replay: structure *with verifiable semantics* — a corpus you can measure with, not merely train
on. (2) SimThing is **not a pure Wei automaton**: hierarchical reduce-up/disburse-down transports
information across scales faster than any stencil light cone, by design. Claim "automaton core
with hierarchical aggregation, all ground-truthed" — which is more interesting, since pure CA
corpora cannot test cross-scale causation and this one can. (3) Four months of an unconnected idea
is weak evidence of profundity; the base rate is near-total.

---

## 4. Importer-slot instantiation — ClauseThing first; cliodynamics back-burnered

**Product decision (2026-06-10):** the importer slot (`CLAUSETHING-IMPORTER-0`, charter §2)
proceeds as the **ClauseThing transpiler**, targeting the **Stellaris / Clausewitz-engine
grand-strategy audience** — players and modders of Clausewitz-class titles — as its named
potential consumer. This gives the importer genuine consumer-pulled standing (a market that
already authors scenarios and mods in exactly this genre) rather than speculative authoring
capability. The charter's curriculum framing (§1.3) stands unchanged: the importer remains the
corpus's distribution-control language, and ClauseThing-authored regime diversity feeds the
dataset rungs exactly as any alternative instantiation would have.

**Epistemic consequence, stated plainly:** the §5 scaling instrument proceeds on its **first
anchor only** — internal ground truth, the columns. The *second anchor* (external validity of the
dynamics class against real historical series) is **deferred, not lost**; until revived, the "toy
world" objection to measured exponents is answered by controlled-complexity design and honest
scoping rather than by historical calibration.

**The back-burner is cheap by construction:** both importer instantiations share one shape
(external formal source → parse → validate → lower through CLAUSE-SPEC admission), so the
ClauseThing build lays most of the road a later cliodynamic importer would drive.

**Generalization (2026-06-10): the importer slot is a semantic port.** The cliodynamics appeal was
largely semantic — the prestige of the historical domain, not a property of the dynamics. From the
substrate's view, history, market intelligence, and advertising performance are interchangeable
plugs into one calibration interface (fit flow/decay/threshold parameters to any external series at
the spec layer), and the commercial domains offer calibration data *denser by orders of magnitude*
than century-coded polities. A Clausewitz-class game is no less complex than any of them at the
only level the substrate recognizes — adversarial accumulation over a topology with threshold
events. One hazard the commercial skins import that the game does not: market adversaries adapt to
whatever observes them, so the rules themselves are the contested object — and SimThing
**endogenizes** exactly that contest (core design §2.1: policy capture → owner-entity fission).
The game corpus therefore contains ground-truthed **rule-capture and regime-shift trajectories** —
the pretraining a capture-detection model needs in domains that will never expose their influence
columns. If a market/advertising skin is ever pursued, the aggregate-not-pixel boundary above binds
**doubly**: pixel-level market-prediction claims are commercially irresistible and epistemically
fatal.

> **Footnote — the back-burnered cliodynamics case (evaluated 2026-06-09; BACK-BURNERED
> 2026-06-10 by product; revivable).** A transpiler for Turchin-class structural-demographic
> models was adjudicated, prior to the consumer decision, as an epistemically superior importer
> instantiation. Compressed record: transpilation is near-trivial — coupled ODEs (population,
> elites, fiscal health, instability) map to `governed_by` integration; elite overproduction to a
> cohort property outrunning a capacity column; state fiscal crisis to a stockpile/`Balance`
> trajectory; political-stress outbreaks to threshold cascades (charter §1.4 knee events at
> civilizational scale); the metaethnic frontier/imperiogenesis models are already spatial and
> Movement-Front-shaped. Acceptance tests come pre-written by the literature (reproduce a
> published secular cycle; reproduce the *Ages of Discord* US political-stress curve). Revival
> would give the corpus the epistemic status of climate-model/epidemiological synthetic data
> (rollouts of a calibrated scientific model class) and enable the out-of-sample test: train on
> transpiled polities to a cutoff, test surprise/instability forecasts against real later crises
> (the domain-randomization pattern). Boundaries if revived: empiricism anchors **aggregates, not
> pixels** (spatial disaggregation adds degrees of freedom the originals never constrained);
> calibration is loose by nature (Seshat uncertainty flags and live disputes; sparse, proxy-laden
> cliometric series) — the looseness becomes the domain-randomization range, not a defect. The
> gift runs both directions: cliodynamics gets the experimental apparatus (interventional
> counterfactual pairs at scale) that one-run history denies it. The back-burnering is itself an
> application of the consumer-pulled principle: the gamer consumer is named and real; the
> historical-science consumer is not yet.

---

## 5. The testable hypothesis — can the latent-predictive paradigm overcome AR primacy?

**Category fix:** LeWM contains transformer blocks. The contest is not transformers vs something
else; it is **autoregressive generative prediction in observation space vs joint-embedding
prediction in latent space** — same blocks, same data, different training objective. State it this
way and the "you built a worse transformer" dodge is closed in advance.

**The constant-vs-exponent correction (the load-bearing quantitative point):** no constant
efficiency multiplier — 2×, 3.5×, 20× — overcomes *frontier* primacy. A constant is paid once and
absorbed: ~3.5× is half an order of magnitude — one GPU generation, one year of compute growth —
and incumbents co-opt constant-factor wins into hybrids (the RWKV/SSM/Mamba precedent: real
measured multiples; primacy untouched). The only quantity that overcomes frontier primacy is an
**exponent gap**: with α_J > α_AR the efficiency ratio grows as a power of compute and the
multiple is re-earned for free at every order of magnitude; with equal exponents no intercept
advantage ever closes a frontier scaling six orders of magnitude overhead. **But primacy is
domain-relative:** *edge primacy* (robotics, wearables, the sub-400MB faction AI — every
budget-capped deployment) is decided by intercepts, because capped domains never ride the curve
far enough for exponents to matter. A measured constant ~3.5× at fixed small scale is decisive
*there*. Two claims, separately testable.

**Formal hypothesis.** Fit, per paradigm, on the ground-truth anchor:

```
L(C) = L∞ + A · C^(−α)
```

- **H1:** α_J ≥ α_AR everywhere, with strict inequality that **widens as observation entropy
  rises** relative to latent-dynamics entropy.
- **H0:** α_J ≤ α_AR at matched anchoring — the small-model wins were task mismatch; the
  paradigm's ceiling is an edge-intercept play.

Three distinguishable outcomes, all informative: **exponent gap** (strong — extrapolate the fitted
laws for a *measured crossover compute*, the first quantitative answer to the primacy question);
**equal exponents + intercept advantage** (edge primacy demonstrated, frontier primacy
mechanism-falsified); **JEPA worse on equal anchoring** (the negative result — arguably the most
publishable of the three, given the capital currently bet against it).

**Design:** one fixed corpus (Movement-Front episodes from ClauseThing-authored regimes;
optionally the back-burnered cliodynamic class — §4 footnote — if external validity is later
revived). Two *championed* families: (a) decoder transformer over tokenized field patches,
next-token cross-entropy; (b) LeWM/SIGReg latent predictor. **Both decoded into ground-truth field
space at horizon k for evaluation** (neither graded in its home currency), plus probe-R² on state
columns, plus planning regret. Five–six model sizes per family across ~3 orders of magnitude of
compute (≈2M–500M params), **each family at its own compute-optimal data ratio** (imposing AR's
Chinchilla allocation on the JEPA is a buried thumb on the scale). Power-law fits with bootstrap
CIs; decision rule = exponent-interval separation. **Mechanism axis:** rerun at three
render-entropy levels with latent dynamics fixed; the abstraction thesis predicts α_AR degrades
with observation entropy while α_J holds — one plot that isolates *why*, or kills the explanation.

**Cost:** small rungs are workstation work; full study ≈ low-thousands of GPU-hours (weeks on a
small box or a modest cloud bill). Not lab-scale.

**Honest limits (state before hostile reviewers do):** exponents measured to ~0.5B parameters and
extrapolated to frontier compute is a long extrapolation, and scaling laws have broken outside
their measured range — the study yields a measured crossover *estimate*, never a proof of primacy.
Smooth-loss laws do not capture emergent-capability discontinuities; ground-truth anchoring
partially defuses the emergence objection, not entirely. The result's credibility equals the AR
baseline's credibility: equal data, equal tuning sweat, strong tokenization, or the headline dies
in one dismissal.

---

## 6. Disposition

Nothing in this file opens, authorizes, or reorders anything. The charter's consumer ladder and
sequencing (§3 of `workshop/field_world_model_horizon.md`) stand: exporter first, importer second
(**ClauseThing instantiation with the named Stellaris/Clausewitz-audience consumer; cliodynamic
instantiation back-burnered, §4 footnote**), probes and optimizer downstream; the scaling study of
§5 rides on `LEWM-PROBE-0`-class external harness work and the corpus rungs. The dual-mission
identity this file operationalizes is recorded in `simthing_core_design.md` §1. Falsification
paths are built in throughout: H0 in §5, the probe failures in §3, and the out-of-sample
historical forecast test (deferred with the §4 cliodynamics option) — each cheap, each decisive,
each survivable.

## References

- Maes, Le Lidec, Scieur, LeCun, Balestriero — *LeWorldModel* (arXiv:2603.19312; repo
  `github.com/lucas-maes/le-wm`) [claims verified against v2 abstract, 2026-06-09].
- Balestriero, LeCun — LeJEPA / SIGReg (Nov 2025) — heuristic-free anti-collapse regularization.
- Wei — *On the Spatiotemporal Dynamics of Generalization in Neural Networks* (arXiv:2602.01651) —
  the locality/symmetry/stability postulates; engine realization in `simthing_core_design.md` §1.1, §7.
- Lee, Han, Kumar, Agrawal — *Training Language Models via Neural Cellular Automata*
  (arXiv:2603.10055) [verified] — automaton corpora transfer, AR side.
- Causal-JEPA (arXiv 2026.02); Agent World Model (arXiv 2026.02) — adjacent fragments [verified
  via survey listing].
- Apple SALT — frozen-teacher video SSL, accuracy-vs-FLOPs Pareto frontiers vs V-JEPA [verified].
- Giovannelli, Raimundo, Vicente — *Pareto sensitivity / snee* (arXiv:2501.16993) — charter §1.4.
- Turchin — *Historical Dynamics* (2003); Turchin & Nefedov — *Secular Cycles* (2009); Turchin —
  *Ages of Discord* (2016); the Seshat Global History Databank — sources for the **back-burnered**
  §4-footnote option.
- AMI Labs launch and $1.03B seed coverage (Crunchbase News, TechCrunch, Built In, 2026-03-09/10)
  [verified].
- Bakhtin et al. — CICERO (*Science*, 2022) — the LLM-as-human-interface boundary marker for
  diplomacy (charter §1.5).
