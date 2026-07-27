---
rung: ROOT-DERIVE-PARTICIPATION-0
kind: rung
track: 0.0.8.7
base_sha: 1b7e2432694b23dd31ea5dfe949aa1a1165803b5
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 1.1, first core-object rung. Frontier lane (Codex 5.6). Greenfield discretion charter applies. The judge is behavioral identity: derivation must produce the SAME resolved arena topology the explicit wiring produced — RF-1 + bit-exact replay are the referees, not new tests."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-spec/src", "crates/simthing-clausething/src", "crates/simthing-driver/tests", "scenarios", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/adr/resource_flow_substrate.md"]
forbidden: ["changing any resolved flow topology/value (derivation = same arena the wiring built; RF-1 + replay judge)", "removing DefaultDisabled (the single sanctioned authored opt-out stays)", "kind-branching (SimThingKind must not gate participation)", "new ColumnIndex mints outside the 0.1 doors", "deleting authored-override capability (overrides remain; only the mandatory wiring dies)"]
required_checks: ["cargo build --workspace", "full simthing-driver battery (RF-1 conservation + replay) on live GPU", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "derivation-cannot-reproduce-an-existing-arena-without-behavior-change", "a-seam-collapse-requires-kernel/WGSL-semantics-changes"]
---
## BUILD
- P0(a)/P1 (ANCHOR-ACK core-0087): admission DERIVES arena participation — a SimThing with
  resource properties + a parent edge participates in the recursive arena BY DERIVATION:
  typed at admission, inspectable, spanned hard-errors on ambiguity; `DefaultDisabled`
  retained as the authored opt-out. No wiring, no enrollment, no registry for the default
  path.
- Collapse the default-path config seams: mandatory `ResourceFlowSpec` / `ArenaRegistry` /
  execution-profile wiring for the DEFAULT recursive arena is deleted or reduced to
  authored OVERRIDES (overrides stay; the obligation dies). The derivation must construct
  the same participant set / arena topology those seams constructed — provably.
- The TP canonical scenario (`scenarios/terran_pirate_galaxy.clause`) is the live proof:
  strip its now-redundant arena wiring; hydration + admission derive the identical arena.
- Doctrine-CI co-evolution rides the PR: retire/adjust scans or anchors that watched the
  collapsed seams; update the 0.0.8.7 §3b stamp + orientation in-diff.
## FENCES
- Behavioral identity is the law: RF-1 conservation + bit-exact replay green BEFORE/AFTER
  with identical results on the TP scenario; any divergence = STOP, not a test edit.
- No kind-gating; participation derives from properties + topology only. Column identity
  only through the 0.1 doors. Deprecated-`new` count must not rise.
## EXIT-PROOF
- TP scenario runs with ZERO explicit arena wiring (diff shows the wiring deleted from the
  scenario/spec path); full driver battery green on live GPU; RF-1 + replay identical
  pre/post (the behavioral-identity proof); admission negative fixture: ambiguous
  participation spans at admission. Stamp 1.1 PROBATION in-diff; regen orientation.
