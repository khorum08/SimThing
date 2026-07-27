---
rung: CAPABILITY-PREREQ-DAG-ADMISSION-0
kind: rung
track: 0.0.8.7
base_sha: 39eea3682915ded824b1d502feab7433fe142a0f
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 2.2 (P2 overlay law, capability-bestowal family). Std lane (Grok, pin -m grok-4.5). §1.2 doctrine applied to prerequisites: the prereq graph stops being spec-layer prose checked at boundary time and becomes admission-validated TYPED DATA. The runtime gate check stays boundary work — only its DATA is now proven-shaped."
surfaces: ["crates/simthing-spec/src", "crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-clausething/src", "crates/simthing-clausething/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/capability_tree_v1.md", "docs/tests"]
forbidden: ["changing unlock semantics (threshold crossing -> prereq gate -> ActivateOverlay flow unchanged; only DAG VALIDATION moves to admission)", "kernel/GPU/WGSL edits", "referee edits (capability install/activation tests green unmodified)", "a runtime prereq re-checker beyond the existing boundary handler (no second gate)", "new ColumnIndex mints outside the 0.1 doors"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "an-existing-authored-tree-fails-the-new-DAG-validation (live malformed prereq data — STOP, DA-route with evidence)", "atomicity-requires-kernel-changes"]
---
## BUILD
- Prerequisite DAG as admission-validated typed data (ANCHOR-ACK core-0087; contract:
  `capability_tree_v1.md` + 0.0.8.7 P2 family (1)): at spec admission, each capability tree's
  prereq graph is validated — **cycles, dangling entry references, cross-tree references, and
  self-prerequisites are spanned hard errors** naming tree/entry/span. Tiered AND mutual
  prerequisites both validate (tier ordering consistent with edges; `max_active` categories
  well-formed: bounds sane, members exist).
- **Mutual-exclusivity atomicity at the generation barrier:** `max_active` sibling suspension
  executes in the SAME boundary step as the activation (v1 §8 semantics) — add the referee
  proving activate+suspend land atomically at one generation barrier (no observable
  intermediate where both siblings are active across a generation).
- Generation-align the capability boundary vocabulary: `capability_tree_v1.md` day-language
  ("atomic at the day level", "the tick after") updated to generation terms (docs; serde
  untouched — the 1.2 wire-contract law stands).
- Existing-tree census: all authored capability trees in fixtures + scenarios pass the new
  validation (they should — if one does NOT, that is live malformed prereq data: STOP,
  DA-route). Negative fixtures: cycle, dangling ref, dup-tier conflict each span.
- Doctrine-CI co-evolution rides the PR; stamp 2.2 + advance posture row to
  `ORDER-WEIGHT-CLASS-0` in-diff; regen orientation.
## FENCES
- Admission-side validation + boundary-step atomicity proof only; unlock flow untouched;
  zero behavior deltas for valid trees (full battery green, referees unedited).
## EXIT-PROOF
- Negative fixtures span at admission (cycle/dangling/self/cross-tree/max_active-malformed);
  existing authored corpus admits unchanged (census); atomicity referee green on live GPU;
  workspace build + full driver battery green; v1 doc generation-aligned. Stamp + posture
  advance in-diff per the graduation ritual.
