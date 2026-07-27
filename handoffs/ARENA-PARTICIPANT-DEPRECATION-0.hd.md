---
rung: ARENA-PARTICIPANT-DEPRECATION-0
kind: rung
track: 0.0.8.7
base_sha: 426c7e4fc67ddffd95044f2bd7804bd4c1fe20dd
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "REWRITTEN per Owner elimination ruling + STOP 5091668622. Frontier lane (Codex 5.6). Owner: 'I MUCH prefer the elimination. The only evaluation that matters is which code gets blown up, whether it is crate or test, and whether/when to fix it.' The wrapper is ELIMINATED from the production path, not deprecated. The breakage ledger is a first-class deliverable."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-clausething/src", "crates/simthing-clausething/tests", "crates/simthing-workshop/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["warning-only theater (deprecating while production mints)", "papering over a consumer by faking wrapper-shaped state", "changing resolved per-participant flow VALUES (RF-1 + economics-level identity judge)", "editing referee ASSERTION LOGIC (re-anchoring topology fixtures is allowed and ledgered; weakening checks is not)", "new ColumnIndex mints outside the 0.1 doors"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "RF-1 conservation at head", "doctrine-scan", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening beyond the ledgered sites", "a-consumer-cannot-re-point-without-kernel/WGSL-semantics-changes", "per-participant resolved economics diverge pre/post"]
---
## BUILD
- ELIMINATE the wrapper from the production path (Owner ruling; DA census: ~85 refs, 14 src
  files + 5 test files, driver-concentrated). Replacement authority (P0: a SimThing IS a row):
  participants host their flow properties ON THEIR OWN ROWS — slot identity is the
  participant's own slot via the role pathway + the 1.1 `ResourceFlowDerivationReport`;
  `materialize_arena_participants` and `ArenaParticipantScaffold` are deleted; consumers
  (base obligations, `gated_rates`, `need_binding`, allocation sync, arena pressure/hierarchy/
  registry, conservation-oracle helpers) re-point at participant rows.
- Delete `SimThingKind::ArenaParticipant` LAST (variant, serde/exhaustive arms, kind-tag
  maps). Legacy carriers are fixtures only (pre-release): fix fixtures, no compat shim.
- **BREAKAGE LEDGER (first-class deliverable):** `docs/tests/arena_participant_elimination_ledger.tsv`
  — one row per blown-up site: `path | crate-or-test | disposition (fixed-now | deferred
  HORIZON-ENTRY(date) + reason)`. Every compile error and every re-anchored test appears;
  deferred rows need dated horizons. Greenfield charter applies to the re-hosting design.
## FENCES
- Identity judge moves to the ECONOMICS level (topology legitimately changes): RF-1
  conservation green; per-participant resolved flow values identical pre/post (capture a
  pre-elimination baseline of the TP + fixture economies and diff); replay bit-exact within
  the new topology. Referee assertion LOGIC unedited; topology-shaped fixtures re-anchored
  and ledgered.
## EXIT-PROOF
- Zero `ArenaParticipant` references tree-wide (grep-proven); ledger complete (every touched
  site rowed); workspace build + full driver battery green on live GPU; RF-1 green;
  per-participant economics baseline diff EMPTY. Stamp 1.3 exit-proof cell + advance posture
  row to `OVERLAY-EFFECT-HOST-ADMISSION-0` in-diff; regen orientation.
