---
rung: EXECUTION-STATUS-TAXONOMY-0
kind: rung
track: 0.0.8.7
base_sha: cebead45d05d4af7e466828a95492944434427b8
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Rung 0.2, Std lane (Grok CLI, grok-4.5). QUEUED BEHIND 0.1 (one rung in flight; both touch scripts/ci — rebase on 0.1's merge before starting). Data + docs work only; no engine code."
surfaces: ["scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["engine crate changes (this is harness data, not code)", "new attestation files (data-driven classes only — HU doctrine)", "renaming/moving existing execution surfaces", "speculative taxonomy values beyond the four ruled classes"]
required_checks: ["doctrine-scan", "doctrine-selftest (scripts/ci changes)", "orientation-check", "doc-budget", "board-digest renders the classification"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "a-surface-resists-classification (escalate via orchestrator, do not invent a fifth class)"]
---
## BUILD
- The execution-status taxonomy as DATA (0.0.8.7 §3 Phase 0 row; RF-harness lineage — the
  audit's `executed | oracle | rehearsal | compile-plan` classes, exactly four): a TSV
  (scripts/ci) classifying every driver/kernel execution surface — the executed recursive
  arena, RF-1/cpu_oracle judges, dress-rehearsal R1–R6C family, compile-plan builders
  (arena_allocation_plan, resource_economy_compile, silo/link compiles), min_plus traversal,
  accumulator/threshold paths. One row per surface: path, class, one-line basis.
- Board surfacing: the board digest / orientation renders per-class counts (executed=N,
  oracle=N, rehearsal=N, compile-plan=N) so posture drift (a rehearsal quietly becoming
  executed, an oracle going dark) is visible at a glance. Reuse the existing digest/board
  generation path — no new generators.
- A cheap scan row (HEURISTIC) tripping when a NEW execution-flavored surface lands
  unclassified (delta-scoped, INSPECT-only), so the table cannot silently rot.
## FENCES
- Data + docs only; zero engine-crate edits. Exactly the four ruled classes. No attestation
  prose — classifications carry a one-line basis, disputes escalate to the DA.
## EXIT-PROOF
- Every current driver/kernel execution surface classified (spot-auditable against the
  exclusion-audit families); board digest renders the counts; unclassified-surface fixture
  fires INSPECT; doctrine-scan 0 hard failures; selftests green; stamp the 0.2 ladder cell
  PROBATION on merge; orientation regenerated.
