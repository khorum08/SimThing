# TEST-ADMISSION-REGIME-0 Results

## Status

**PROBATION** - test admission is now standing law, stacked on `TEST-PARE-STANDARD-DA-0` / PR #1087. This rung adds gates and ledger structure; it deletes no tests.

## Fable ruling embodied

Binding principle:

```text
A test is rung-3/4 ladder residue like a scan. Test admission requires naming the regression nothing higher on the ladder owns. Promotion retires redundant tests in the same PR.
```

`KEEP` without a permanent-residue class or promotion target is illegal.

## What changed

- Landed test-admission doctrine in `docs/simthing_core_design.md`, `docs/design_0_0_8_3.md`, and `docs/handoff_template.md`.
- Added `promotion_target` to `scripts/ci/test_inventory.tsv`.
- Added `TEST-BUDGET` to `scripts/ci/scans.tsv`.
- Added `scripts/ci/test_inventory_drift_check.sh`.
- Wired inventory drift into `scripts/ci/doctrine_scan.sh`.
- Extended `scripts/ci/doctrine_selftest.sh` with TEST-BUDGET and drift proofs.
- Added TEST-BUDGET and drift fixtures under `scripts/ci/fixtures/**`.
- Added `docs/tests/test_promotion_wave_plan.md`.

## Doctrine landings

- Core §1.2: tests are ladder residue like scans.
- Constitution §0.9.5: promotion retires redundant scans or tests in the same PR; kernel/sim strict tier is binding.
- Handoff template §6: new KEEP-class tests must name a promotion target or permanent-residue class.

## Survivor trichotomy

Inventory `KEEP` rows are legal only with:

- `permanent-residue:*`
- `promotion-target:*`

Current inventory:

```text
live inventory rows: 6301
KEEP rows: 829
KEEP rows with legal promotion_target: 829
promotion-target rows: 122
```

## TEST-BUDGET flow gate

`TEST-BUDGET` is a HEURISTIC, delta-scoped scan:

- More than 3 added `#[test]` functions in one changed file without table-driven form: INSPECT.
- Table-driven growth remains quiet.

Selftest proof:

- `enumeration_burst.rs`: TEST-BUDGET fires.
- `table_driven_trap.rs`: TEST-BUDGET stays quiet.

## Inventory drift stock gate

`test_inventory_drift_check.sh` mechanically enumerates tests/fixtures and compares them to the ledger.

It fails:

- unledgered tests
- ledgered-but-deleted stale rows
- unowned KEEP rows
- kernel/sim KEEP rows that are not permanent residue

## Kernel/sim strict tier

`simthing-kernel` and `simthing-sim` KEEP rows must be permanent-residue rows. Admission enumeration and hygiene KEEP rows in those crates are drift-check failures.

## Never-pare set

Preserved: compile_fail / trybuild seal proofs, CPU-oracle parity, bit-exact determinism, golden-byte/exact artifacts, doc-named invariants, STEAD-required suite, `custom_layout_ethics_axis`, escaped-bug regressions, and active live rung suites while open.

## Promotion-wave plan

`docs/tests/test_promotion_wave_plan.md` reconciles with the inventory:

```text
promotion-target rows: 122
target groups: 2
```

Follow-on `TEST-PROMOTE-<boundary>-0` rungs must land the higher boundary and retire redundant tests in the same PR.

## Validation

Commands:

```bash
bash scripts/ci/doctrine_scan.sh
bash scripts/ci/doctrine_selftest.sh
bash scripts/ci/gen_digest.sh --check
bash scripts/ci/test_inventory_check.sh
bash scripts/ci/test_pare_boundary_check.sh
bash scripts/ci/test_inventory_drift_check.sh --prove
```

Observed:

```text
DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0
DOCTRINE-SELFTEST-VERDICT: PASS
gen_digest --check: PASS
TEST-INVENTORY-CHECK-VERDICT: PASS
TEST-PARE-BOUNDARY-CHECK-VERDICT: PASS
TEST-INVENTORY-DRIFT-PROVE-VERDICT: PASS
```

## Scope Ledger

- No test deletions.
- No crate source edits.
- No crate test edits outside `scripts/ci/fixtures/**`.
- No workflow edits.
- No `allow/*.txt` edits.
- No cargo tests or full batteries run.

## Graduation routing

Recommended status: **PROBATION**.

Why: this turns test discipline into enforced admission law and prevents re-bloating. DA/orchestrator should deep-review doctrine landings, TEST-BUDGET proof, drift stock gate, kernel/sim strict tier, promotion-wave reconciliation, and no-deletion/no-cargo scope.

## Known gaps / next

Next highest-impact material reduction remains `TEST-PARE-SPEC-0`, followed by scan-duplicate, Tier 1 type-seal, or classifier-consolidation waves depending on DA routing.
