# WORKSHOP-CORPUS-TRIAGE-0 Results

> **Status: PROBATION / proof-present / orchestration-review-pending.** Coding
> lane only; no merge, graduation, pointer movement, `--apply`, or 13.5+ work.

**Date:** 2026-08-30
**Dispatch:** Board `5466001952`
**HD-RECEIPT:** `d5ab65c8a047`
**ORIENT-RECEIPT:** `12660ec6def1` (`orientation_rule_stamp=4660c85703ef2f33`)
**Dispatch master:** `cf8c3f863d28e16ce30bd9092b3c633774182b5a`
**tested_code_sha:** `892a092c6678c65e900fbd6ed4af08a95ff0e7d0`

## Discovery

`track_closeout.sh --discover --track 0.0.8.7-rf-arena-modernization`: ripe=0
(track still open). `--build-manifest` scoped 603 track assets (596 inventory +
7 docs); 70 of those inventory rows are `simthing-workshop`. Workshop source,
reports, and A1 archive files are not inventory-scoped; they are the
default-delete candidate corpus and are ledgered in `closeout_artifacts.tsv`.

## Dispositions

Workshop-scoped resolved manifest
`docs/tests/workshop_corpus_triage_0_closeout_manifest.tsv`
(`CLOSEOUT-RECEIPT: a65962f29ab4`):

| disposition | count | meaning |
|---|---:|---|
| keep-durable | 61 | 60 durable-class inventory proofs + A1 memo already archived |
| elevate-class | 10 | `until-closeout:behavior-regression` standing EML/ActionBand proofs |

`scripts/ci/closeout_artifacts.tsv` 0→75 lease rows (header-only → disposition
rows only):

| family | rows | measurement |
|---|---:|---|
| `persistent_bench` (+WGSL/report) | 4 | `measured:2026-08-30@cf8c3f863d28e16ce30bd9092b3c633774182b5a` tiny 4p/32q/8t `pivot_mean_us=599` `env_us=26103` `cpu_us=2` `WEAK_PASS` |
| `multitarget_replay` / `transfer_contention` / `overlay_order` / `eml_phase5` / `weighted_mean` | 28 | `owed-measurement:2026-08-30` live instrument compiles; bounded window measured persistent_bench only |
| A1 Gu-Yang gather-vs-tiled archive | 19 | `instrument-stale:2026-08-30` 31 compile errors; **FIRST-RUNG PRECONDITION** |
| standing ActionBand/EML workshop proofs + crate scaffolding | 24 | dated `2026-08-30`; not default-delete |

Consumer for performance families and A1: **PERFORMANCE-TRACK**. No lease lacks
a named consumer/date. No delete lacks an owner (zero deletes).

## A1

Wildcard resolved against the live tree: `docs/workshop/archive/c1_perf_reframe_memo.md`
plus `docs/workshop/archive/field_policy/field_policy_tensor_stencil_*` (preserves,
prototypes, refinements, notes, README). Archive compiled as-is against current
kernel: **FAIL** (`StencilRefinementPrototype` gone; `u32` vs `SlotIndex` /
`ColumnIndex`). Not repaired. Perf-track opening may not cite this debt until
re-measurement runs.

## Closeout proof (non-mutating)

- `--check-eval` PASS receipt `a65962f29ab4`
- `--artifact-expiry` PASS expired=0
- `--decommission --dry-run` DRY reaped=0 files=0
- `--apply` not run

## Containment

Zero production engine/spec/kernel/gpu/sim/driver/embedder semantic change.
Zero gate-code edit. Zero pointer movement. Zero track closure.

## Certificate

`cargo test --workspace --offline`: **133 suites / 590 passed / 0 failed / 15
ignored**. `track_closeout.sh --prove` PASS. orientation/digest/doc-budget
`--check` PASS.
