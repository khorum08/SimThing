# HD-CATALOG-LIBRARY-0 Results

Status: PROBATION / proof-present / DA-review-pending

## Identity
- PR: #1347
- Branch: `coder/hd-catalog-library-0`
- Handoff: `handoffs/HD-CATALOG-LIBRARY-0.hd.md`
- `HD-RECEIPT: 55089254496b`
- `tested_code_sha`: final PR head is mirrored in the PR body after the remand commit; the committed file cannot contain its own hash.
- `coverage_basis: PASS`
- `ci_green: PASS` for local required battery; GitHub exact-head gates are refreshed after push.

## Changed File Envelope
- Implementation: `scripts/ci/librarian.sh`
- Ladder/generated: `docs/design_0_0_8_4_8_4_hd_board.md`, `docs/orchestrator_orientation.md`
- Evidence: `docs/tests/hd_catalog_library_0_results.md`, `docs/tests/current_evidence_index.md`
- Forbidden surfaces untouched: `crates/**`, Studio/UI, new tables, second trigger engine, `gen_orientation` gate logic, `handoff_dispatch` logic.

## Catalog Proof
- Coding catalog: 23 lines; payload sections `required_checks,forbidden_surfaces,BUILD,FENCES,EXIT-PROOF`.
- Orchestrator catalog: 23 lines; payload section `routing`.
- DA catalog: 23 lines; payload sections `audit_targets,risk_class,expected_residue,forbidden_surfaces`.
- All roles enumerate the same anchor library from `doctrine_anchors.tsv` x `anchor_triggers.tsv`: 17 anchors and 38 trigger domains.
- Always-on spine derives from orientation: `field-policy-time-decisions`, `spec-fidelity-anti-ceremony`, `founding-ontology-invariants`, `drift-detectors-six-line`.

## Falsifiers
- `per-role-catalogs-differ`: coding/orchestrator/da outputs differ and assert their role payload surfaces.
- `catalog-cap-complete-or-fail`: a complete coding report passes at its exact boundary; one line below cap exits nonzero with only `LIBRARIAN-CATALOG-VERDICT: FAIL(report-line-cap role=coding ...)`, no partial body and no role PASS.
- `catalog-readonly-reach-log`: live reach-log bytes stay unchanged and a missing fixture reach log is not created.
- `staleness-harness-fixture-count`: `--staleness` emits `harness-fixture-count: 650`.

## Battery
- `bash scripts/ci/librarian.sh --selftest` PASS.
- `bash scripts/ci/agent_scan.sh` PASS.
- `bash scripts/ci/gen_orientation.sh --check` PASS.
- `bash scripts/ci/doc_budget_check.sh --check` PASS.

## Known Residue
- Local `--staleness` remains `INSPECT` because pre-existing leased artifacts are aging; this rung preserves that signal.
- Final `/clearance` and `/relay-lint` are refreshed on the remand head before returning.
- DA authors any graduation stamp at merge; this PR remains draft, PROBATION, and unmerged.
