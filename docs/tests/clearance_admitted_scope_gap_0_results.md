# CLEARANCE-ADMITTED-SCOPE-GAP-0 Results

## Status

**DONE** — machine `DA-RESERVE(admitted-scope-router-gap)` live in `clearance_check.sh`; 7/7 DA-required selftests PASS; full clearance selftest **49/49 PASS**.

## Policy source

| Field | Value |
|---|---|
| Rung | `CLEARANCE-ADMITTED-SCOPE-GAP-0` |
| Kind | Harness / router (gate-wiring) |
| DA policy | #1242 Option A (`CLEARANCE-UNCLASSIFIED-SCOPE-REDUCTION-0`) |
| Base | #1242 merged @ `f7be0dc2845e89afd24720c8f3d5df2da7bb81c1` |

DA ruling (binding):

```text
unclassified-scope is too broad for admitted implementation work. Future proof-present
PRs inside an already-admitted Owner/DA envelope must not be treated as fresh DA design
decisions merely because no precedented class exists. The clearance router should
distinguish true unclassified scope from admitted-scope router gaps.
```

## Implemented verdict

```text
CLEARANCE-VERDICT: DA-RESERVE(admitted-scope-router-gap)
```

Meaning (required wording):

```text
DA-RESERVE(admitted-scope-router-gap) means admitted envelope + proof-present + missing class.
It is router debt, not a fresh DA design question. Repeated admitted-scope router gaps
should be closed with class-hardening.
```

Orchestrator action: do **not** open a fresh DA design ruling; open/land class-hardening or require class-hardening follow-up.

## Router behavior

When no precedented class matches (after gate-wiring early-out and explicit novelty check):

1. `novelty_claim: YES` + valid `novelty_basis` → already emitted `DA-RESERVE(novelty)` (before class detect).
2. Else if `admitted_envelope: YES` and admitting_pr/admitting_rung + surfaces + proof fields present:
   - forbidden surface hit → `DA-RESERVE(class-envelope-violation)` (not clearable)
   - else → `DA-RESERVE(admitted-scope-router-gap)`
3. Else → `DA-RESERVE(unclassified-scope)` (narrowed: no class + no valid admitted claim)

Matched class paths preserve existing class-envelope / engine-scope / binding behavior.

Gate-wiring surfaces still emit `DA-RESERVE(gate-wiring)` before class/admitted-scope logic.

## Required body fields

When claiming admitted-scope gap:

```text
admitted_envelope: YES
admitting_pr: #<n>          # or admitting_rung: <RUNG-ID>
admitted_surfaces: <one-line>
forbidden_surfaces: <one-line>
tested_code_sha: <8+ hex>
coverage_basis: PASS
ci_green: PASS
```

Missing any required claim/proof field →:

```text
CLEARANCE-VERDICT: FAIL(missing-admitted-scope-router-gap-fields: <field list>)
```

Not a DA design reserve.

## Selftest matrix

| # | Fixture | Expected | Result |
|---|---|---|---|
| 1 | `clearance_selftest_admitted_scope_true_unknown` | `DA-RESERVE(unclassified-scope)` | PASS |
| 2 | `clearance_selftest_admitted_scope_api_gap` | `DA-RESERVE(admitted-scope-router-gap)` | PASS |
| 3 | `clearance_selftest_admitted_scope_picker_gap` | `DA-RESERVE(admitted-scope-router-gap)` | PASS |
| 4 | `clearance_selftest_admitted_scope_missing_proof_fields` | `FAIL(missing-admitted-scope-router-gap-fields: tested_code_sha, coverage_basis, ci_green)` | PASS |
| 5 | `clearance_selftest_admitted_scope_forbidden_surface` | `DA-RESERVE(class-envelope-violation)` | PASS |
| 6 | `clearance_selftest_admitted_scope_novelty_preserved` | `DA-RESERVE(novelty)` | PASS |
| 7 | `clearance_selftest_admitted_scope_gate_wiring` | `DA-RESERVE(gate-wiring)` | PASS |

Full suite: `bash scripts/ci/clearance_check.sh --selftest` → **PASS (49 fixtures)**.

## Unclassified-scope narrowing proof

Fixture #1: no class, no `admitted_envelope`, no novelty → `DA-RESERVE(unclassified-scope)` only. True unadmitted residue; not novelty.

## Admitted API gap proof

Fixture #2: synthetic mapeditor path **not** covered by `tp-admitted-clause-api-composition`, body claims #1229-shaped admitted API envelope + proofs → `DA-RESERVE(admitted-scope-router-gap)`.

## Admitted picker gap proof

Fixture #3: synthetic picker-shaped path with no class, body claims #1235-shaped admitted picker envelope + proofs → `DA-RESERVE(admitted-scope-router-gap)`.

## Missing proof fields proof

Fixture #4: `admitted_envelope: YES` + admitting_pr + surfaces, missing `tested_code_sha` / `coverage_basis` / `ci_green` → `FAIL(missing-admitted-scope-router-gap-fields: ...)`.

## Forbidden surface proof

Fixture #5: valid admitted claim but changed files include `crates/simthing-sim/src/engine_source_change.rs` (runtime/engine) → `DA-RESERVE(class-envelope-violation)`. **Never** `ORCHESTRATOR-CLEARABLE`.

## Novelty preservation proof

Fixture #6: body has both `novelty_claim: YES` + valid basis **and** admitted-envelope fields → `DA-RESERVE(novelty)` (novelty checked before class/admitted-scope). Not swallowed by admitted-scope-router-gap.

## Gate-wiring preservation proof

Fixture #7: touches `scripts/ci/clearance_check.sh` + `precedented_classes.tsv` even with admitted-envelope body → `DA-RESERVE(gate-wiring)`.

## Fixture lifecycle posture

Committed under `scripts/ci/fixtures/clearance/clearance_selftest_admitted_scope_*` (existing harness convention). Ledgered in `scripts/ci/test_inventory.tsv` as seal-proof fixtures for `CLEARANCE-ADMITTED-SCOPE-GAP-0`. **Not** scenario artifacts. `fixture_accretion: LEDGERED`.

## Commands

```bash
bash scripts/ci/clearance_check.sh --selftest
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_true_unknown
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_api_gap
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_picker_gap
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_missing_proof_fields
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_forbidden_surface
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_novelty_preserved
bash scripts/ci/clearance_check.sh --fixture clearance_selftest_admitted_scope_gate_wiring
bash scripts/ci/gen_orientation.sh --check
bash scripts/ci/test_inventory_drift_check.sh
bash scripts/ci/test_lifecycle_expiry_check.sh --schema
bash scripts/ci/doc_budget_check.sh --check
bash scripts/ci/doctrine_scan.sh
git diff --check
```

## Clearance routing

This PR changes router/gate surfaces → expected:

```text
CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)
```

Do **not** self-merge.

## Known gaps

- Forbidden-surface checks are textual/path heuristics (engine/GPU/kernel, GameMode/RF/live-run, closeout, optional picker keywords) — not full semantic parsing of `forbidden_surfaces` free text.
- Option C (admitted-envelope registry) remains a later hardening rung after Option A is live.
- Does not register the picker class (`TP-STUDIO-CLAUSE-PICKER-CLASS-0` is next).

## Recommended next rung

```text
TP-STUDIO-CLAUSE-PICKER-CLASS-0
```

**Not next:** closeout, product picker work, GameMode/RF attach, runtime/GPU/kernel.
