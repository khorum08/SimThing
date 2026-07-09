# HU-CLEARANCE-DSL-0 Results

**PROOF-PRESENT / PROBATION** — gate-wiring; not self-merged (Fable).
ORIENT-RECEIPT: `4921e84c2b89` · stamp `27baba147e3f156c`.

## A. Requirement DSL

Sibling `scripts/ci/class_predicates.tsv` (not extra cols on `precedented_classes.tsv` —
would overload 6-col registry + fixture TSV copies).

| Column | Meaning |
|---|---|
| `match_any_globs` | detection signal (≥1 path) |
| `scope_globs` | envelope |
| `forbidden_globs` | none may match → envelope reserve |
| `detect_mode` | `all_in_scope` \| `any_then_envelope` |
| `priority` | picker 20 > API 10 |

Migrated both TP admitted classes; deleted `has_*_shape` + all TP `check_*_field` / `check_picker_only`.
Body reqs: only `tested_code_sha|coverage_basis|ci_green`. DA owns banned attestations.

| Metric | Before | After |
|---|---|---|
| `clearance_check.sh` (`wc -l`) | 1470 | 1415 |
| picker `requirements` fields | 11 | 3 (proof-identity only) |

## B. Treeverify fold

Every `DA-RESERVE` emits `DA-TREEVERIFY-PROFILE:` via `da_treeverify_lib.py`.
CLEARABLE/FAIL never emit. CLI + lifecycle + `da_review_profile.tsv` untouched.

## New fixtures

| Fixture | Half |
|---|---|
| `clearance_selftest_dsl_forbidden_glob_hit` | (a) forbidden-glob → reserve |
| `clearance_selftest_dsl_treeverify_profile_on_reserve` | (b) reserve carries profile |

Migrated TP fixtures retained; body-only attestation cases → CLEARABLE; path envelopes unchanged.

## Exit proof

```text
clearance_check.sh --selftest → PASS (61 fixtures)
da_treeverify.sh --selftest → PASS
relay_lint.sh --selftest → green (logic untouched; §4B)
agent_scan.sh → PASS footer on this diff
```

Rider: unused `AGENT_SCAN_BASH` export removed from `agent_scan.sh`.
No new class · no scans.tsv · lexicon unchanged · no self-merge.

tested_code_sha: e17664d2d87da0836ccad1f6218ed85b6d6c2f99
coverage_basis: PASS - clearance_check 61 fixtures + da_treeverify + relay_lint + agent_scan PASS
