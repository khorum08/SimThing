# 0.0.8.7 — RF Arena Modernization (the SimThing unification)

> **Status: STUB / AUTHORED — NOT OPENED.** Owner-directed 2026-07-19. Develops while the
> 0.0.8.6 TP chain finishes; opens only after TP closes and the Owner commits the full plan
> (via `gen_orientation.sh --open`; pointer-lifecycle gate applies). Do not flip the pointer;
> do not dispatch from this doc. Phase ladders below are commitment-time placeholders — rung
> decomposition happens when the Owner commits.
>
> **CORE track.** At completion this is **built INTO THE CORE**: the canonized object model
> lands as constitutional sections of `simthing_core_design.md`, and the **SimThing object is
> complete and distinct** — one concrete, working, GPU-resident simulation kernel.

## 1. North star

Unify the SimThing principle: **a SimThing with resource properties, parented into a tree,
simulates — intrinsically, recursively, on the GPU — with zero scenario-side wiring.** The
recursive RF arena regulating resource economics and Wei's STEAD automata expressed as spatial
heatmaps ARE the Sim in SimThings; this track makes that a property of the *object*, not of
per-session configuration. Future scenarios — and other games — script onto SimThing trees and
**work**.

**The prize is emergence (Owner, 2026-07-20).** The unification is real and tonal — STEAD is
control economics: the same allocation kernel run over a non-conserved signal commodity, where
stocks are the plant, the field is the controller, the arena-pressure projection is the sensor,
banded commitments are the actuator, and authored weights are the prices. But the unification is
the *rail*, not the prize. **The value this track exists to capture is Wei's emergent behavior**:
fronts, borders, escalations, and strategy arising UNSCRIPTED from one shared local rule plus
authored prices (attractor dynamics — 12.10's `38/38/1` divergence from two weight scalars was an
attractor-relocation proof, the first captured instance). Every phase privileges that emergence
while aligning strictly to the SimThing recursive emission-band RF topology: mechanisms serve the
dynamics and must never script the outcomes they exist to let emerge.

Evidence base: the 0.0.8.6 RF sub-track (RF-1…RF-5A). Every expensive failure was the same
gap — the RF kernel living as session wiring instead of object semantics (engine disengaged
for three eras behind opt-in flags; RF-5A's manual plumbing; the cosplay era only possible
because there was a seam to fake). The substrate ADR's original intent ("one execution model;
allocation is intrinsic") is completed at the object level, now falsifiable because the
executed recursive default (RF-2), the conservation oracle (RF-1), and repaired kernel
contracts (RF-2A) exist.

## 2. Pillars (Owner requirements, binding at commitment)

- **P1 — Intrinsic RF kernel at the object.** Resource properties + a parent edge ⇒ arena
  participation **derived at admission** (typed, inspectable, spanned hard-errors;
  `DefaultDisabled` opt-out retained). Behavior when parented and as a parent is intrinsic,
  natural, recursively automatic. Collapse the config seams (ResourceFlowSpec / ArenaRegistry /
  execution profiles / registrations) into the derivation.
- **P2 — Overlays as THE data/settings/directives layer.** Canonize overlays with a full,
  **living API surface**: lightweight and versatile enough to span capability-tree overlays
  all the way to **user-action mechanisms**. One overlay law from authored data to operator input.
- **P3 — Specialization protocol.** Specialized SimThings (spatial, owner, session, …) get a
  protocol **richer than a kind enumeration** — but it **must not break existing kind code**.
  Promotion, not rewrite; every rung keeps the existing corpus green (compatibility falsifier:
  full existing test suite passes unmodified at every rung).
- **P4 — GPU-residency intrinsic to the object.** SimThings ARE dense-matrix rows: the
  row/column setup is as intrinsic as RF flow, property-value emission bands, and thresholds.
  **A SimThing knows how to slot into a parent, and how child rows are allocated relative to
  it.** Slot/column identity flows from the object model through the role pathway — never
  minted beside it (consumes the RF-harness column admission-gate).
- **P5 — Intrinsic STEAD influence.** Resource accumulation on spatial SimThings emits
  influence falloff as *object behavior* (not authored emitters); falloff superposition forms
  the heatmaps; their interactions feed **Gu-Yang pathfinding and PALMA borders** through the
  Structural Execution Convergence Contract (stead §10) — existing ops, never bespoke kernels.
- **P6 — Portability proof.** A deliberately non-TP domain (ADR's own arena examples: food /
  research / colony) scripted **purely onto SimThing trees, zero engine code** — and it works.
  The 12.6 second-synthetic-scenario discipline scaled to a whole game.

## 3. Phase ladder (stub; decomposed at commitment)

| Phase | Theme | Note |
|---|---|---|
| 0 | Hygiene prerequisites | The 0.0.8.4.8.4.2 RF-harness interventions (column admission-gate + migration; execution-status taxonomy `executed / oracle / rehearsal / compile-plan`, board-surfaced; deferred candidates). Fold-in vs. precede = Owner decision at commitment. |
| 1 | Intrinsic RF kernel (P1) | Derived-at-admission participation; config-seam collapse; RF-1 oracle + determinism judge every rung. |
| 2 | Overlay API canonization (P2) | Data → settings → directives → capability trees → user actions; one living surface. |
| 3 | Specialization protocol (P3) | Richer-than-kind, kind-compatible; spatial/owner/session first citizens. |
| 4 | Intrinsic GPU residency (P4) | Row/col + parent-slotting + child-row allocation as object semantics. |
| 5 | Intrinsic STEAD influence (P5) | Falloff emission + heatmap interaction + Gu-Yang/PALMA consumption. **STEAD-as-emission-bands (Owner correction 2026-07-20):** STEAD influence on a parent SimThing is READ as **emission-band thresholds** — a banded ladder of threshold registrations on the accumulated cell (existing machinery composed: N band edges = N threshold registrations, each with its own event kind/band id; the RF-5A append-scan proved multi-scan sequencing). Bands bind **1:1 to the falloff influence bands in the heatmap**; intensity (band), velocity (banded thresholds on the GovernedPair rate cell), and magnitude (value) are all observable/measurable through existing property mechanisms. **Quantize the READING, never the FIELD**: falloff superposition + conservation math stay continuous; bands are the observation/decision/coupling surface only — band edges must not enter accumulation math (replay determinism + RF-1 envelope). |
| 6 | **Movement-Front execution** (Owner-added 2026-07-20) | Fleet movement AUTHORITY: spatial SimThings move along STEAD need/falloff gradients (core §7 automaton, Gu-Yang pathfinding, PALMA borders) under the ordinary tick — decisions stay threshold crossings, movement state is sim authority. **Decision ingress = sealed band-crossing events from Phase 5's emission bands (OC-K-DECISION-INGRESS-0 pattern) — never CPU branching on raw field/heatmap reads** (the naive-Phase-6 temptation is pre-empted by design). Fills the 12.4 `InTransit` readback seam ("test-private until authoritative movement readback exists"); the 12.5 icon descriptors start expressing real transit with zero icon-layer change (the renderer-seam design anticipates this). |
| 7 | **Combat as RF arena** (Owner-added 2026-07-20) | The axiom made mechanical: combat = RF arena resolution (HP/damage as resource flows). Promote the R6/R6B/R6C dress rehearsals from falsification-oracle to executed default via the proven RF promotion pattern (oracle-first → executed → legacy/rehearsal re-anchored); RF-1-style conservation judging; rehearsals retain oracle role against the executed path. |
| 8 | Portability proof + CORE canonization (P6) | Non-TP domain works untouched; object model written into `simthing_core_design.md`; SimThing complete and distinct. |

## 4. Binding laws (carry from day one)

**The Consumer Law (Owner mandate, 2026-07-20).** STEAD, PALMA, Gu-Yang, and Studio observation
are STANDING CONSUMERS of all data accumulated at SimThings — accumulated data is never walled
off as hygiene ("waiting for a consumer" is not a state that exists; STEAD is the consumer).
Hygiene disciplines govern PROOF (source loci cannot close an OVL or prove a flow) and MUTATION
(no side-channel writes) — never READ ACCESS. CPU-side consumer reads route through the one
canonical generic read seam (hosted SimThingId + typed PropertyKey/role, role-pathway loci,
coherent per-tick snapshots); observation never becomes CPU decision branching — decisions stay
on-device threshold/band crossings. Born from the 12.3 stop: the 12.2→12.8 horizon seam was
dropped and the disruption map sat walled at 0.0 while macro disruption ran ~8446.
**Emergence-first (the prize law).** Every phase carries a 12.10-style emergence falsifier:
authored-parameter changes must produce qualitatively different, UNSCRIPTED macro outcomes; a
phase that leaves the mechanism sound but the dynamics inert — over-quantized bands, over-
constrained admission, scripted outcomes, dead attractors — FAILS its exit-proof regardless of
green checks. P1 bounded-horizon and P2 one-shared-rule are preserved as the conditions under
which Wei’s emergence generalizes (per the paper); band ladders quantize the READING never the
FIELD, so the continuous dynamics that carry the emergence are never discretized away.
Promotion, not rewrite — existing code keeps working at every rung. Intrinsic ≠ implicit:
all automatic behavior is derived at admission, typed, spanned-erroring — never runtime magic.
Oracle-judged: RF-1 conservation + bit-exact replay at every phase. Deferred capabilities from
the RF-5A rulings (mid-session authored refresh; authored complete arena composition) land
here, not as scenario patches.

## 5. Open conditions

1. 0.0.8.6 TP chain closed (RF-5A → RF-5 → 12.10) and 0.0.8.6 CLOSED/PARKED.
2. RF-harness (0.0.8.4.8.4.2) disposition decided: folded as Phase 0 or run as pre-track.
3. Owner commits the full plan (rung decomposition of §3); then `gen_orientation.sh --open`.
