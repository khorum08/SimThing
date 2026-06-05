# RUNTIME-0080-0-R1c-a Free-List Mark Results

**Status:** IMPLEMENTED / PASS - resident free-list mark-only; no allocation or compaction  
**Verdict:** PASS  
**Date:** 2026-06-05  
**Primitive:** `RESIDENT-FREELIST-MARK-ONLY-0`  
**Rung:** `RUNTIME-0080-0-R1c-a`  
**Scope:** resident free-list mark-only from R1b journal rows; no scatter/compact  
**Stable report checksum:** `58d70988e436777b`

## Verdict

R1c-a earns the smaller rung named by R1c: a resident GPU slot bitmap marks cohort slots made free by
R1b's already GPU-read structural journal rows. The mark source is the R1b journal projection, not a new
CPU oracle reconstruction lane.

This pass does **not** claim allocation into free slots, REENROLL scatter, birth/removal authority,
fusion compaction, or GPU-emitted structural decisions. Those remain future gates.

## Predecessors

| Field | Value |
| --- | --- |
| R1c verdict | PARTIAL |
| R1b event journal parity | true |
| R1b event rows read from GPU values | true |
| R1b free-slot mark source rows | nonzero |

## Resident Mark-Only Evidence

| Field | Value |
| --- | --- |
| resident marker session created | true |
| mark sources from R1b GPU journal | true |
| mark parity measured from GPU values | true |
| disabled-marker negative control detected | true |
| marker dispatch count | 1 |
| marker readback count | 1 |

The disabled-marker control clears the resident bitmap and fails mark parity; re-enabling mark writers
restores GPU-vs-oracle slot parity.

## Authority Flags

| Flag | Value |
| --- | --- |
| resident free-list mark authority | true |
| resident free-list allocation authority | false |
| resident REENROLL scatter authority | false |
| resident birth/removal authority | false |
| resident fusion compaction authority | false |
| structural decisions GPU emitted | false |

## Remaining Gates

| Gate | Value |
| --- | --- |
| requires compaction for next rung | true |
| requires allocation for birth rung | true |
| semantic GPU code required | false |
| CPU planner required | false |
| `docs/invariants.md` edit required | false |
| pinned-number change required | false |
| scenario reopen required | false |
| next horizon | `R1c-b resident allocation into marked free slots / no compaction` |

## R6C Checksum

- Expected: `1bba891c779190a4`
- Observed: `1bba891c779190a4`
- Matches: true

## Exact Command Results

```text
cargo test -p simthing-driver --test runtime_0080_0_r1c_a -> 9 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r1b -> 26 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r1c -> 11 passed; 0 failed
cargo check --workspace -> passed
```
