# Track Closeout Protocol

Standing operating procedure for closing a workplan/track. One script —
[`scripts/ci/track_closeout.sh`](../scripts/ci/track_closeout.sh) — run **identically by the DA and
the Orchestrator**, replaces the hand-run, multi-PR, multi-window closeout that the 0.0.8.4.8
corpus-clearance sweep needed (seven PRs, ~13 rows each, two-TSV manual sync, 466-line row-dump
reports, and DA/Orchestrator scope divergence). Closing a track should cost about what a docs cleanup
pass costs.

This is **operational tooling, not a doctrine anchor.** It consumes the existing lifecycle harness; it
adds no new invariant surface.

**DA review profile rows:** track-local rows in `scripts/ci/da_review_profile.tsv` must set `core=NO` and
`expires_on`. On track closeout, **delete or retire** those rows (lifecycle gate fails if expired `active`
non-core rows remain). Core rows (`core=YES`) are permanent global surface weights — never closeout-delete.

**Reach-log decay:** during closeout, prune or report `scripts/ci/anchor_reach_log.tsv` entries older than
30 days via `bash scripts/ci/anchor_query.sh --prune 30` (observability only — never a clearance gate).

**Anchor/trigger table decay (OC-CLOSEOUT-0 durable rule):** future growth of
`scripts/ci/doctrine_anchors.tsv` or `scripts/ci/anchor_triggers.tsv` must be **(1)** trigger-backed
(path/domain adjacency that agents hit), **(2)** explicitly declined in a reach-log / closeout report
with a greppable reason, or **(3)** paired with retirement or elevation of another row so live-table
growth is not unpaired. Unbounded catalogue accretion without one of those three is a fail state.

## The one rule that governs everything

**TSV row growth is the primary fail state of the rustification harness.** A track-scoped asset is
therefore only ever one of:

- **`delete`** — not worth keeping. Removed now (inventory **and** boundary row, in lockstep). The
  Necessity Test applies: a delete must name the higher-rung owner (type boundary / admission
  hard-error / scan / integration path, or an auto-clear rule) that makes the test redundant.
- **`elevate`** — worth keeping, so it must not linger as a deletion-bound TSV row. Two targets:
  `elevate-code` (relocate the source/capability into a destination crate) or `elevate-class`
  (promote a genuine proof into a `permanent-residue:*` class in `test_residue_classes.tsv`).
- **`keep-durable`** — already carries a durable class; retained, no mutation.
- **`lease`** — undecided. The row is **relocated out of live `test_inventory.tsv` into the parking pen**
  (`test_lifecycle_parked.tsv`). Boundary-row audit ledger retired (HU-INVENTORY-ONEWRITE-0); inventory is the sole
  survivor table. The pen is on a **wall-clock clock** (not a survival count): cruft-flag at **3 days**, hard
  delete/elevate wall at **7 days** (`--artifact-expiry` detects; `--decommission` reaps). A lease is a grace
  period, never a resting state.

If an asset is worth keeping it is elevated; if it is not, it is deleted; if it is undecided it is parked
out of sight on a 7-day fuse. Nothing accretes in the live tables. The drift gate treats a parked test as
accounted-for (not "unledgered") so quarantine does not break the ledger.

Docs/artifacts are scoped too. `--build-manifest` includes the track source doc when it is a repo-local
`docs/*.md` / `docs/*.tsv` file, auto-discovers track-shaped result/review/manifest artifacts under
`docs/tests/`, and accepts explicit `--docs <glob>` additions. Source/design docs may be
`keep-durable` or `elevate-code` into `docs/archive/`; automatic delete/lease is reserved for narrow
`docs/tests/*_results.md`, `*_review.tsv`, and `*_manifest.tsv` artifacts. Closeout reports stay durable;
closeout manifests are leased for later reaping. Explicit `--docs` additions must still exist at apply
time; vanished explicit docs fail as stale manifests before mutation.

## Stages (same script, staged subcommands)

| stage | command | does |
| --- | --- | --- |
| discover | `--discover [--track <id>]` | read-only: lists rows at end-of-lifecycle not yet dispositioned + aging leased artifacts. "What's ripe for closeout." |
| build | `--build-manifest <workplan.md \| --track <id>> [--out <path>] [--docs <glob>]...` | deterministic scope discovery → one disposition manifest TSV + a **CLOSEOUT-RECEIPT**. Auto-clears known-shape residue (rules table); marks durable rows `keep-durable`; includes track docs/artifacts; everything else `needs-disposition`. |
| eval | `--check-eval <manifest>` | validates every disposition is resolved and every `delete` has a named owner; refuses `needs-disposition`; rewrites the header receipt to the resolved value. |
| apply | `--apply <manifest>` | one batched mutation: deletes (both TSVs), class stamps, code/doc moves, doc leases, **parks (rows relocated to the pen)**; stamps the `birth_track` **closed**; if `scripts/ci/active_track.txt` still points at the closing source/doc scope, retires it to `none` and regenerates orientation; runs the gate battery, including `cargo check -p <crate>` for code move targets; emits a compact, size-first report. |
| clock | `--artifact-expiry` | wall-clock gate over the parking pen (`test_lifecycle_parked.tsv`) and staged-file leases (`closeout_artifacts.tsv`): INSPECT at ≥3d, FAIL at ≥7d. Standing CI gate — detects, does not delete. |
| reap | `--decommission [--dry-run] [--all]` | actually deletes expired parked/leased assets — but **only the unambiguously safe ones**: ledger-only `cfg_test_mod::*` markers (drop the pen row), dedicated unshared test files under `crates/*/tests/**`, and narrow result/review/manifest docs under `docs/tests/`. Refuses and reports anything risky — inline/src unit tests, shared test files, non-reapable docs, code awaiting rehome — for manual handling. `--all` reaps every parked row, not just past-the-wall ones. |
| guard | `--deletion-guard <base> <head>` | a removed inventory row is lawful only if its `birth_track` was `closed` **at base**, or this PR is itself the closeout (closed at head **and** the diff carries both `docs/tests/<track>_closeout_report.md` and `docs/tests/<track>_closeout_manifest.tsv`) → otherwise FAIL. Hand-flipping `status=closed` in the same PR as the deletion is a bypass and is blocked. cfg-marker ledger sweeps exempt. |
| prove | `--prove` | self-tests all of the above. |

## Anti-divergence: receipts, not SHA-matching

There is **no per-asset SHA pinning anywhere.** The write/churn/update-delay of SHA-matching cost more
than the failures it caught. Agreement between the DA and the Orchestrator flows from the
**CLOSEOUT-RECEIPT**: a 12-hex stamp over the manifest disposition body only (comment/prose lines
excluded, so cosmetic edits don't churn it). Same manifest ⇒ same receipt ⇒ provably identical scope.
`--apply` refuses a manifest whose header receipt does not match its body (i.e. someone edited without
re-running `--check-eval`).

## Where it fits

Not every PR ladder needs closeout. But **every rung that declares a definitive closure point must run
`--apply` to rubber-stamp it** (the `birth_track` → `closed` transition is that stamp), and any PR that
deletes existing tests is checked by `--deletion-guard`. Invoke `--discover` any time to surface
closure debt without opening a track. Windows/CRLF-safe: all TSV I/O is BOM/`\r\n`-normalized.

Active orientation is local-agent-owned: `--apply` mutates `active_track.txt` only when the current pointer
belongs to the closing track's source/doc scope; unrelated active tracks are left alone. GHA/webchat verify the
committed result with `gen_orientation.sh --check`; choosing the next active production mission remains DA/operator work.

## Report shape

The committed report (`docs/tests/<track>_closeout_report.md`) leads with the **TSV table sizes
before→after and their delta** (a grown table is a FAIL), then disposition tallies, then a complete
**"NOT deleted"** survivor table naming each survivor's new lifecycle. The full per-asset list lives in
the committed manifest, not inlined as a wall of rows.
