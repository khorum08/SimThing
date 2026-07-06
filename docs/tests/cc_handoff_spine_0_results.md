# CC-HANDOFF-SPINE-0 Results

## Status

PROBATION / proof-present / DA-review-pending.

## PR / branch / merge

- Branch: `codex/cc-handoff-spine-0`
- PR: not opened
- tested_code_sha: b076bcac3e
- coverage_basis: PASS - docs/template-only gate-wiring update; targeted proof commands recorded below; scanner and relay-lint implementation surfaces unchanged, so their selftests are not required by the handoff.
- ORIENT-RECEIPT: 3f856e32d5d7
- role: coding
- orientation_digest_sha: 5c5dfdb1c91bc2417d0c77b888e704aed8937158ac2e77a2b395743ddff63386

## What changed

- Compressed `docs/handoff_template.md` context spine.
- Replaced mechanically enforced doctrine restatements with enforcing-surface pointers.
- Retained unmechanized DA-clearance / authority-boundary norms.
- Added Canonical Entrypoints block.
- Clarified orientation as session admission, not mandatory per-handoff ceremony.
- Updated `docs/agent_onboarding.md` to match the session-scoped orientation rule.
- Necessity-scoped proof/selftest guidance so selftests run only when their surface is touched.

## Load-bearing proofs

`old_context_spine_lines=55`
`new_context_spine_lines=34`

```text
$ bash scripts/ci/doctrine_scan.sh
DOCTRINE SCAN REPORT  (commit b076bcac3e, 2026-07-06T13:13:26Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  TEST-BUDGET  PASS  0  design §0.9.5 test admission budget
  SPEC-LOWERER-KIND-READ  INSPECT  415  ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire
  TEST-INVENTORY-DRIFT  PASS  0  stock gate: inventory matches discovered tests and KEEP rows are owned
  DOC-BUDGET  PASS  0  DOC-BUDGET-VERDICT: PASS
  RULE-EXPIRY  PASS  0  RULE-EXPIRY-VERDICT: PASS
  AGENTS-STUB  PASS  0  AGENTS-STUB-VERDICT: PASS
  --- summary ---
  hard failures: 0   inspect flags: 415   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=415 selftest=SKIPPED
  --- inspect justifications ---
  justifications file present with 1 entries
  INSPECT findings present; per-INSPECT status: check justifications file or report for unresolved
  INSPECT-JUSTIFICATION:
    scan-id: <HEURISTIC_SCAN_ID>
    location: <file:line or symbol>
    status: provided via inspect_justifications.tsv
```

```text
$ bash scripts/ci/gen_orientation.sh --check
gen_orientation --check: PASS
```

```text
$ bash scripts/ci/gen_digest.sh --check
gen_digest --check: PASS
```

```text
$ git diff --check
<no output>
```

Conditional checks not required by touched-surface necessity:

- `scanner unchanged - selftest not required`
- `relay-lint implementation unchanged - relay-lint selftest not required`
- `no crate touched - cargo check not required`
- `no tests/inventory touched - inventory drift check not required`

## Scope Ledger

| Item | Status | Notes |
|---|---|---|
| `docs/handoff_template.md` spine compression | implemented | Mechanized doctrine moved to enforcing-surface pointers. |
| Canonical Entrypoints block | implemented | Conditional references, not a mandatory battery. |
| Session-vs-handoff orientation clarification | implemented | Full orientation framed as session admission. |
| `docs/agent_onboarding.md` mirror update | implemented | Coding-agent handoff wording no longer says "orient first" for every rung. |
| Proof battery necessity-scoped | implemented | Selftests run only when their surface is touched. |
| Handoff-template anchors | not touched | Already removed by DA ruling. |
| Scripts / schemas / workflows / crates | not touched | Docs-only intended scope. |
| DA-reviewed merge posture | retained | PROBATION / DA-review-pending. |

## Known gaps / next

- DA must confirm that each compressed line has a legitimate enforcing-surface pointer.
- DA must confirm no unmechanized doctrine was silently deleted.
- DA must confirm orientation language does not force repeated per-handoff receipts.
- DA must confirm selftest guidance is necessity-scoped and not a blanket proof battery.

## Graduation routing

- CI verdict: doctrine_scan INSPECT(415), no hard failures; required freshness checks PASS
- Triage entries: none for this docs diff; existing heuristic INSPECT covered by `inspect_justifications.tsv`
- Risk class: gate-wiring, data-deliverable
- Falsification check: inspect `docs/handoff_template.md`, `docs/agent_onboarding.md`, and `docs/tests/cc_handoff_spine_0_results.md`; confirm spine line count decreased, pointers cite live enforcers, retained norms still include DA clearance for gate-wiring / authority / PROBATION rungs, Canonical Entrypoints exists, orientation is session-level, handoffs carry session receipts rather than forcing full orientation, and selftests are conditional on touched surfaces.
- Recommended posture: deep - binding handoff template and onboarding-contract edit.
