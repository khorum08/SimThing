# TEST-PARE-INVENTORY-0 Results

Status: **PROBATION** - inventory/classification rung complete; zero deletions; paring decisions deferred to Track D audit rungs.

## What Changed

- Added `scripts/ci/test_inventory.tsv`, a mechanical inventory of the current Rust test corpus and CI fixture proof surface.
- Added `scripts/ci/test_inventory_check.sh`, a self-check that re-enumerates the same mechanical surface and validates schema, taxonomy, never-pare rules, and exact row coverage.
- Documented Track D in the CI scaffolding design, operator screening guide, and current evidence index.

No runtime code, crate source, crate test source, workflow, scanner allowlist, or Track A blocking gate was changed.

## Inventory Method

The inventory was generated from the bounded D0 surface:

- `crates/*/src/**/*.rs`
- `crates/*/tests/**/*.rs`
- `crates/*/benches/**/*.rs`
- `scripts/ci/fixtures/**/*`

The enumerator records Rust test attributes (`#[test]`, `#[tokio::test]`, `#[async_std::test]`), `#[cfg(test)] mod ...` test modules, `compile_fail` documentation fences, trybuild compile-fail declarations, and CI fixture files. The checked-in inventory and the self-check both enumerate **6,300** rows.

## Taxonomy

Inventory columns:

`crate	file	test_name	kind	class	superseding_boundary	verdict	note`

Allowed `kind` values: `unit`, `integration`, `doc`, `compile_fail`, `trybuild`, `fixture`, `unknown`.

Allowed `class` values: `behavior-regression`, `oracle-parity`, `seal-proof`, `golden-byte`, `invariant-required`, `stead-required`, `admission-superseded`, `admission-adjacent`, `usecase-superseded`, `duplicate-battery`, `hygiene-theater`, `unknown`.

Allowed `verdict` values: `KEEP`, `PARE`, `AUDIT`, `COLLAPSE(n->1)`, `COLLAPSE(n→1)`.

## Row Counts

By kind:

- `integration`: 5,002
- `unit`: 1,193
- `compile_fail`: 77
- `fixture`: 28

By verdict:

- `AUDIT`: 5,476
- `KEEP`: 824
- `PARE`: 0
- `COLLAPSE`: 0

Selected classes:

- `behavior-regression`: 4,250
- `admission-adjacent`: 891
- `oracle-parity`: 484
- `hygiene-theater`: 147
- `unknown`: 138
- `stead-required`: 121
- `golden-byte`: 113
- `seal-proof`: 105
- `usecase-superseded`: 50
- `invariant-required`: 1

## Never-Pare Enforcement

The self-check rejects any non-`KEEP` verdict for:

- `compile_fail` and `trybuild` rows
- `seal-proof`, `oracle-parity`, `golden-byte`, `invariant-required`, and `stead-required` rows
- the named invariant test `custom_layout_ethics_axis`

Current never-pare proof rows are all `KEEP`, including the 77 `compile_fail` rows, 28 CI fixture rows, oracle/parity rows, golden-byte rows, STEAD-required rows, and the `custom_layout_ethics_axis` invariant row.

## Pare / Collapse Summary

D0 assigns **zero** rows to `PARE` or `COLLAPSE`. Any future deletion or collapse requires a later Track D audit rung with an explicit superseding boundary and proof that no seal, oracle, golden-byte, STEAD, invariant, or compile-fail coverage is being removed.

## Admission / Audit Summary

The inventory marks **891** rows as `admission-adjacent`, all with `AUDIT` verdict. They are not deletion candidates in D0; they are the queue for later admission-ladder review.

The inventory marks **5,476** rows as `AUDIT` overall, including **138** `unknown` rows. Unknown rows are deliberately not greenlit for removal.

## Self-Check

Command:

```bash
bash scripts/ci/test_inventory_check.sh
```

Result:

```text
TEST-INVENTORY-CHECK REPORT
  rows: 6300
  discovered: 6300
  missing: 0
  extra: 0
  inspect: none
TEST-INVENTORY-CHECK-VERDICT: PASS
```

## Scope Ledger

- Zero crate source edits.
- Zero crate test source edits.
- Zero test deletions.
- Zero workflow edits.
- Zero scanner or allowlist edits.
- Zero all-crates cargo tests and zero bare/full-crate cargo tests.
- Track B owner-deep cadence remains undecided pending paring data.
- Smoke PASS remains mechanics-only, not seal-proof.

## Graduation Routing

Recommended status: **PROBATION**.

Reason: D0 proves inventory coverage and classification mechanics only. It does not authorize deletion, collapse, or cadence changes. D1 must audit the `AUDIT` queue against explicit superseding boundaries before any test corpus reduction can graduate.

## Known Gaps / Next

- `TEST-PARE-AUDIT-1`: inspect `admission-adjacent`, `unknown`, `hygiene-theater`, and `usecase-superseded` rows against actual proof boundaries.
- `TEST-PARE-CRATE-2+`: make per-crate deletion/collapse proposals only after D1 has named safe superseding coverage.
- `TEST-PARE-CADENCE-DF`: decide whether owner-deep full batteries move to weekly/master-only only after paring data exists.
