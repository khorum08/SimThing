# HU-CLEARANCE-DSL-0 Results

## Status

**PROOF-PRESENT / PROBATION** — data-driven class predicates + treeverify fold. Gate-wiring; not self-merged.

ORIENT-RECEIPT: `4921e84c2b89` · rule stamp `27baba147e3f156c` (coding).

## A. Requirement DSL

Sibling **`scripts/ci/class_predicates.tsv`** (not extra columns on `precedented_classes.tsv`):
scope + detection would overload the 6-col class registry and fixture TSV copies.

| Column | Meaning |
|---|---|
| `match_any_globs` | detection signal (≥1 path) |
| `scope_globs` | envelope paths |
| `forbidden_globs` | none may match → `class-envelope-violation` |
| `detect_mode` | `all_in_scope` \| `any_then_envelope` |
| `priority` | multi-shape winner (picker 20 > API 10) |

Generic engine in `clearance_check.sh` interprets rows. Migrated:

- `tp-admitted-clause-api-composition` (`all_in_scope`)
- `tp-studio-clause-picker` (`any_then_envelope` + post-match scope)

Deleted: `has_*_shape` detectors, all TP `check_*_field` / `check_picker_only`.

`requirements` for both classes: **only** `tested_code_sha|coverage_basis|ci_green`.
Banned attestations (DA review owns): `admitted_api`, `ui_file_picker`, `tp_defaults_in_production`,
`session_hydrate`, `studio_clause_picker`, `production_api_only`, `duplicate_parse_rebind`, closeout flags.

Body-only selftest fixtures → CLEARABLE (attestations no longer machine-gated).
Path envelope fixtures unchanged (reserve/unclassified). `closeout_yes` remapped to out-of-scope path.

## B. Treeverify fold

Every `DA-RESERVE(...)` also emits `DA-TREEVERIFY-PROFILE:` via `da_treeverify_lib.py profile`
on the same changed-file list. CLEARABLE/FAIL never emit it. CLI + lifecycle gate untouched.

## Exit proof

```text
bash scripts/ci/clearance_check.sh --selftest
→ CLEARANCE-SELFTEST: PASS (59 fixtures)

clearance_check.sh line count: 1413 (baseline 1470; net −57)
```

No new class rows · no scans.tsv · verdict lexicon unchanged · no self-merge.
