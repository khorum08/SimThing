# CLEARANCE-DOCS-LADDER-STUDIO-STAMPS-0 Results

## Status
**PROBATION** — class-hardening; expected sticky `DA-RESERVE(gate-wiring)`.

## What changed
Widen `docs-ladder-pointer-correction` so Studio docs-only exit stamps self-clear when they only touch:

```text
docs/design_*.md
docs/orchestrator_orientation.md
docs/tests/*_readiness_0_results.md   # pre-existing
docs/tests/studio_*_results.md        # NEW (this rung)
```

### Surfaces
- `scripts/ci/precedented_classes.tsv` — scope glob `docs/tests/studio_*_results.md`
- `scripts/ci/clearance_check.sh` — `has_docs_ladder_shape` recognizes `docs/tests/studio_*_results.md`
- Fixtures:
  - `clearance_selftest_docs_ladder_studio_stamp_clearable` → ORCHESTRATOR-CLEARABLE
  - `clearance_selftest_docs_ladder_studio_stamp_rejects_crates` → DA-RESERVE(unclassified-scope)
  - `clearance_selftest_docs_ladder_studio_stamp_rejects_workflows` → DA-RESERVE(gate-wiring)

## Guardrails
Widened class still requires **every** changed path to match the docs-ladder globs. Mixing in `crates/**`, `.github/**`, or gate harness paths fails shape / hits gate-wiring. No production Studio behavior change.

## Proofs
```text
bash scripts/ci/clearance_check.sh --selftest
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_docs_ladder_studio_stamp_clearable
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_docs_ladder_studio_stamp_rejects_crates
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_docs_ladder_studio_stamp_rejects_workflows
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/gen_orientation.sh --check
```

## Follow-on
After this lands: re-run clearance on #1310 → expect ORCHESTRATOR-CLEARABLE → orchestrator merge exit stamp.
