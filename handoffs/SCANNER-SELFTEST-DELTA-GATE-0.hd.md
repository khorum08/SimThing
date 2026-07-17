---
rung: SCANNER-SELFTEST-DELTA-GATE-0
kind: rung
track: 0.0.8.6
base_sha: 5d0ad4850e1437400e0027e116095e66e11a6174
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "AUTONOMOUS execution authorized (run to merge; do not pause per-rung). Coder = Codex CLI (gpt-5.5 high, full-access) in a VISIBLE console window. Orchestrator (webchat) bears review + remedial drafting; DA rules only on the relay. OWNER MANDATE being mechanized: scanner self-tests must NOT run on every PR (lineage: R1-TEST-PURGE design_0_0_8_1 §408 — proof batteries are never default per-PR gates; whole-tree = CI/maintainer, not default). Deliverable BOTH the gate AND an anti-drift guard so it can never silently revert."
surfaces: [".github/workflows/doctrine-scan.yml", "scripts/ci/selftest_gate_guard.sh", "scripts/ci/doctrine_selftest.sh", "docs/ci_screening_surface.md", "scripts/ci/doctrine_anchors.tsv", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["gating/skipping the actual doctrine SCAN or the freshness/anchor-integrity/DOC-BUDGET-check/rule-expiry/lifecycle/triage steps (only the self-PROOF batteries are gated)", "weakening any selftest's logic", "making the anti-drift guard itself heavy (must be a cheap grep/parse run every PR)", "editing crates/** source", "kernel/WGSL/Studio/UI"]
required_checks: ["doctrine-selftest", "doctrine-scan", "gen-digest-check", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "gating-a-non-selftest-gate", "anti-drift-guard-not-mechanized"]
---
## BUILD
- In `.github/workflows/doctrine-scan.yml`, add an early `id: gate` step that sets output
  `run_selftests` = true iff the event diff touches scanner/harness files: `git diff --name-only`
  over `pull_request.base.sha..head.sha` (PRs), `before..sha` (push), grepping `^scripts/ci/|^\.github/workflows/`.
  `workflow_dispatch` → true. Fail-open only on a genuinely unresolvable diff (then true).
- Gate these SIX self-PROOF steps with `if: steps.gate.outputs.run_selftests == 'true'` (AND with any
  existing condition): Orientation receipt selftest, Relay lint selftest, DOC-BUDGET **selftest**,
  Track-closeout selftest (`--prove`), AGENTS stub **selftest**, and Doctrine self-test (`doctrine_selftest.sh`).
- DO NOT gate (these run every PR): the doctrine SCAN (PR-delta + whole-tree), sanctioned-surface /
  orientation freshness, anchor integrity, DOC-BUDGET **check**, rule-expiry, lifecycle schema gate,
  triage spam, AGENTS stub **check**. Only the self-PROOF batteries are gated.
- ANTI-DRIFT GUARD (the never-drifts-out mechanism): add `scripts/ci/selftest_gate_guard.sh` that parses
  the workflow YAML, finds every step whose `run:` invokes a scanner self-test (`--selftest`,
  `doctrine_selftest.sh`, or `track_closeout.sh --prove`), and FAILs (citing the mandate) if any such
  step lacks the `steps.gate.outputs.run_selftests` guard. Add a CHEAP doctrine-scan step that runs it
  UNCONDITIONALLY every PR (grep/parse only — no battery). Give it a `--selftest` with two fixtures
  (a gated step → PASS, an ungated selftest step → FAIL).
- Document + anchor the mandate: a short paragraph in `docs/ci_screening_surface.md` (scanner self-tests
  are delta-gated per the R1-TEST-PURGE / whole-tree-is-maintainer mandate, enforced by
  `selftest_gate_guard.sh`); `anchor_check.sh --resync` after the edit. Keep the doc net-neutral (it is at
  its DOC-BUDGET ceiling — trim redundancy to fit).
## FENCES
- The gate must never skip the actual scan or the freshness/integrity checks — self-PROOF batteries only.
- The anti-drift guard MUST be mechanized (a real FAIL on an ungated selftest), cheap, and run every PR;
  a prose-only note is NOT acceptable (that is the drift this rung closes).
- Selftests still run in FULL when scanner/workflow files change; do not weaken them.
## EXIT-PROOF
- Falsifiers, each observed: (a) a crate-only diff → gate `run_selftests=false` → the six steps SKIP;
  (b) a `scripts/ci/**` diff → gate true → they RUN; (c) `selftest_gate_guard.sh --selftest`: gated
  fixture PASS, ungated fixture FAIL; (d) removing the guard on any one selftest step → guard FAILs.
- doctrine-scan green on this PR (it touches scripts/ci + workflow → selftests run + pass); the guard
  passes on the gated workflow; gen_digest / orientation / doc-budget green; anchor resynced.
- PROBATION leads the 12.11 cell + the Active-open-rung row; orientation regenerated; DA stamps at merge.
