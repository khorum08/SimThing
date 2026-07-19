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
| 5 | Intrinsic STEAD influence (P5) | Falloff emission + heatmap interaction + Gu-Yang/PALMA consumption. |
| 6 | Portability proof + CORE canonization (P6) | Non-TP domain works untouched; object model written into `simthing_core_design.md`; SimThing complete and distinct. |

## 4. Binding laws (carry from day one)

Promotion, not rewrite — existing code keeps working at every rung. Intrinsic ≠ implicit:
all automatic behavior is derived at admission, typed, spanned-erroring — never runtime magic.
Oracle-judged: RF-1 conservation + bit-exact replay at every phase. Deferred capabilities from
the RF-5A rulings (mid-session authored refresh; authored complete arena composition) land
here, not as scenario patches.

## 5. Open conditions

1. 0.0.8.6 TP chain closed (RF-5A → RF-5 → 12.10) and 0.0.8.6 CLOSED/PARKED.
2. RF-harness (0.0.8.4.8.4.2) disposition decided: folded as Phase 0 or run as pre-track.
3. Owner commits the full plan (rung decomposition of §3); then `gen_orientation.sh --open`.
