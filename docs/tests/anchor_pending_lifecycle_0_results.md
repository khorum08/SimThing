# ANCHOR-PENDING-LIFECYCLE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 11.5)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/anchor-pending-lifecycle-0`
- dispatch_base_sha: `174381095d6a9363c75c83a10e264d76fc878df8`
- tested_code_sha: `2d7e65b17fa93d84c30aab59bebe9750f76e3971`
- final_head_sha: evidence-bound successor commit; this file does not self-hash
- dispatch_binding: Board `5459050293`; schema ruling `5458992070`
- HD-RECEIPT: `26b7c27efe9d`
- ORIENT-RECEIPT: `eebbb5c1b2e0` (fresh after lifecycle/hash regeneration)
- role: `coding`
- orientation_rule_stamp: `616eeebb3d0ba711`
- orientation_digest_sha: `a09bc683b80bda4c798da08d1c6eef5e3988444786f857da62add8c344fa3849`
- expected_route: `DA-RESERVE(gate-wiring)`

## Outcome

The existing anchor table now carries one explicit lifecycle field. Its 64
rows are 62 `canonical` rows plus exactly two interim rows:
`unified-ingress-exclusivity` and `stemthing-b-market-grammar`, both tagged
`pending:UNIFIED-INGRESS-EXCLUSIVITY-0`. Their document targets and narrow
trigger-domain lists are otherwise byte-identical to the dispatch base.

`anchor_check.sh` joins those tags to the active design ladder through the
existing `gen_orientation.sh` `parse_rungs` authority. The read-only
`--rung-truth` projection uses the dedicated Status column where present and
the legacy Exit-proof cell otherwise; no second rung parser or lifecycle
registry was added. The resulting closed dispositions are:

- `PENDING-HEALTHY`: minting rung graduated and canonization remains open;
  advisory-only during ordinary checks.
- `ORPHANED`: minting rung absent, reverted, superseded, or not graduated;
  eligible for explicit reap.
- `STALE-PENDING`: `CORE-CANONIZATION-0` graduated while the row remained
  pending; ordinary integrity fails and librarian refuses auto-reap.

The live two-row worklist reports `PENDING-HEALTHY`; ordinary anchor integrity
passes. Anchor stamps now bind lifecycle as well as content hashes, resync
round-trips the six-column schema, and query output reports lifecycle without
changing selection or reach-log append behavior.

## Reap authority and deletion provenance

`librarian.sh --pending-anchors` lists dispositions without mutation.
`--confirm` removes only rows reported `ORPHANED`, refuses healthy and stale
rows, and appends exactly one provenance row for each removed anchor in the
same guarded transaction. An injected failure after the anchor-table replace
rolls both files back byte-exactly.

The deletion ledger schema is exactly:

```text
subject  scope  path  name  kind  authorizing_ruling  rung  hd_receipt
```

Its subject vocabulary is closed to `test` and `anchor`. The existing ledger
was mechanically migrated as 82 physical TSV rows (one header plus 81 test
entries): every legacy value is byte-identical after mapping
`crate/file/test_name` to `scope/path/name`. The inventory deletion guard
filters `subject=test` and receives its original seven-field shape, so legacy
authorization behavior is unchanged. Anchor reap rows use
`anchor / doctrine_anchors / <doc> / <anchor_id> / orphaned /
<confirm-comment-id> / <mint-rung> / n/a`.

`track_closeout.sh` now rejects closeout with stable
`PENDING-ANCHORS-REMAIN` evidence. It also rejects an anchor provenance row
whose `name` remains live with stable
`ANCHOR-DELETION-PROVENANCE-LIVE`; the valid-reap fixture passes after the
anchor is absent.

## Existing command transport

No workflow or mutation authority was added. The existing `/librarian` path
was extended at all four sanctioned sites:

1. command parser enumeration and truthful general/specific FORMAT strings;
2. malformed-librarian workflow reply;
3. librarian job action-to-flag case arm;
4. owner-review predicate and no-mutation route.

`/librarian pending-anchors --confirm` preserves `action=pending-anchors` and
`confirm=true`. A non-owner confirmation routes to owner review; an
unconfirmed list remains available to an authorized collaborator. Confirmed
workflow reaps are PR-branch-only and use the invoking comment/review ID as
the deletion ruling.

## Falsification and local evidence

| Command / fixture | Result |
|---|---|
| `bash scripts/ci/anchor_check.sh --check` | PASS; live `healthy=2 orphaned=0 stale=0` |
| `bash scripts/ci/anchor_check.sh --selftest` | PASS; 12 fixtures including healthy advisory, missing-rung orphan, and canonization-live stale failure |
| `bash scripts/ci/anchor_query.sh --selftest` | PASS; 12 fixtures including reach append and LF-clean prune |
| `bash scripts/ci/librarian.sh --pending-anchors` | DRY; two live healthy rows listed, no mutation |
| live `--pending-anchors --confirm` | PASS; both healthy rows refused, `reaped=0`, byte-identical tables |
| `bash scripts/ci/librarian.sh --selftest` | PASS; no-confirm refusal, orphan-only one-for-one reap, healthy/stale refusal, atomic rollback, parser/FORMAT, four-site transport, and owner-review assertions |
| `bash scripts/ci/track_closeout.sh --prove` | PASS; pending RED, ledgered-still-present RED, valid reap green, subject vocabulary closed, legacy test-deletion behavior green |
| `bash scripts/ci/gen_orientation.sh --selftest` | PASS; 29 fixtures |
| `bash scripts/ci/doctrine_selftest.sh` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | INSPECT; 0 hard failures and 417 standing whole-tree heuristic findings |
| `bash scripts/ci/test_inventory_check.sh` | PASS; 1,348 discovered / 1,348 ledgered |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS; unledgered 0 |
| workflow YAML parse | PASS |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |
| `git diff --check` | PASS |

The six legacy anchor-table fixtures were mechanically widened; no new test
fixture file or inventory row was added. The final diff is confined to
workflow/CI scripts, CI data, and documentation. No Rust, crate, runtime, GPU,
WGSL, simulation, admission, replay, history, or telemetry source changed, so
the Owner/DA ruling requires no structural certificate. The active pointer
remains on 11.5; no graduation or merge is claimed.
