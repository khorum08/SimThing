---
rung: ORDER-WEIGHT-CLASS-0
kind: rung
track: 0.0.8.7
base_sha: 7766f709b8b84517f899f831eb7a6daf854eedbc
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 2.3, closes Phase 2 (P2 operator-directive family). Std lane (Grok, pin -m grok-4.5). The Owner's canonical example is BINDING: a user-selected destination = a dominant weight overlay (e.g. +10000) on the fleet's need columns — the fleet still behaves entirely by STEAD banded commitments, but price dominance makes the outcome undeniable. ORDERS ARE PRICE INJECTIONS, NEVER COMMAND CHANNELS."
surfaces: ["crates/simthing-core/src", "crates/simthing-spec/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-clausething/src", "crates/simthing-clausething/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["a command channel / direct state write / movement setter (the directive is an OVERLAY riding the ordinary allocation sweep — nothing else)", "infinite or non-finite weights (RF-1 envelope + replay must hold)", "kernel/GPU/WGSL edits", "referee edits", "new ColumnIndex mints outside the 0.1 doors"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "dominance-cannot-be-guaranteed-with-finite-weights-under-existing-normalization (pricing-model gap — STOP, DA-route)", "directive-requires-a-non-overlay-mechanism"]
---
## BUILD
- The authored ORDER-WEIGHT CLASS (ANCHOR-ACK core-0087; P2 family (4)): a typed, authored,
  FINITE dominance magnitude class for operator directives — declared as data (spec-level
  class with its magnitude), not a scattered literal. Dominance contract: an order-class
  weight dominates ambient authored prices in the same arena under the existing allocation
  normalization; finite always (RF-1 envelopes + bit-exact replay preserved).
- `OverlaySource::Player` directive path admitted end-to-end: an operator directive is an
  ordinary overlay ({kind, source: Player, affects, transform, lifecycle}) writing weight
  columns via the standard machinery — structurally IDENTICAL to an AI policy overlay
  (core design §6). Lifecycle: `Transient` with declarative dissolve (arrival threshold /
  override / timer) so completed orders retire themselves at generation boundaries.
- **The canonical exemplar (exit-proof-bound, live GPU):** in a TP-derived fixture, a
  destination order = order-class weight overlay on the target's need/weight columns; the
  fleet's banded commitments select the destination (decision ingress unchanged, on-device);
  arrival dissolves the overlay; the fleet resumes ambient STEAD behavior. Assert: outcome
  undeniable (allocation flips to the ordered target while the order is live), lawful
  (RF-1 green, replay bit-exact), reversible (post-dissolve behavior identical to a
  never-ordered twin from the same state).
- Directive latency assertion: the order takes effect at the next generation boundary
  (decision-ingress latency — the responsive-order feel, by construction).
- Doctrine-CI co-evolution rides the PR; stamp 2.3 + advance posture row to
  `SPECIALIZATION-PROTOCOL-0` (Phase 3) in-diff; regen orientation.
## FENCES
- One modification law: the directive is an overlay, full stop — any second mechanism is a
  constitutional violation (§6). Finite weights only. Zero behavior deltas absent an order
  (full battery green, referees unedited).
## EXIT-PROOF
- Canonical exemplar green on live GPU (dominance + lawfulness + reversibility + latency);
  order-class declared as typed data; negative fixture: a non-finite or class-less dominant
  weight is a spanned admission error; workspace build + full battery green. Stamp + posture
  advance in-diff per the graduation ritual.
