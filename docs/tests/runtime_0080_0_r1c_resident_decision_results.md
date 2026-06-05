# RUNTIME-0080-0-R1c Resident Decision Results

**Status:** IMPLEMENTED / PARTIAL (STOP-LINE) - resident structural decisions require free-list scatter gate  
**Verdict:** PARTIAL  
**Date:** 2026-06-05  
**Primitive:** `RESIDENT-REENROLL-0`  
**Rung:** `RUNTIME-0080-0-R1c`  
**Scope:** resident structural decision authority gate: REENROLL/scatter/compact  
**Stable report checksum:** `570147cda47b5434`

## Verdict

R1c is intentionally PARTIAL. The full R1c goal is resident structural decision authority for
REENROLL/membership scatter, cohort birth/removal, and fusion lineage/compaction. The production spec
places that work behind the free-list-scatter / compaction stop-lines. This implementation therefore
does **not** set `structural_decisions_gpu_emitted = true` and does **not** claim resident structural
decision authority.

The smaller result landed here is an R1c gate/readiness harness that preserves R1b's earned journal
parity, proves the complete CPU-shadow contract, documents the backpressure policy, and names the next
smaller rung: `R1c-a resident free-list mark-only / no compaction`.

**Follow-on landed:** `R1c-a` is now implemented/pass for resident free-list mark-only, with allocation,
scatter, and compaction still deferred. See
[`runtime_0080_0_r1c_a_free_list_mark_results.md`](runtime_0080_0_r1c_a_free_list_mark_results.md).

## Predecessor R1b

| Field | Value |
| --- | ---: |
| R1b verdict | PARTIAL |
| R1b event journal parity | true |
| R1b GPU event rows | 247 |
| R1b oracle event rows | 247 |
| R1b structural decisions GPU emitted | false |
| R1b CPU boundary consumes event rows | true |
| R1b CPU boundary does not rederive decisions | true |
| R1b checksum matches | true |

R1b remains the earned substrate: all event kinds are journalized with exact oracle parity, but the CPU
decision witness still decides which structural rows exist.

## Complete CPU Shadow Contract

| Contract | Result |
| --- | --- |
| complete CPU shadow retained | true |
| includes Tier-A values | true |
| includes membership | true |
| includes positions | true |
| includes birth/removal state | true |
| includes fusion lineage | true |
| includes slot allocation | true |
| no structural reconstruction from value projection | true |
| serialize -> reload -> continue round-trip | true |
| reloaded from serialized snapshot | true |
| serialized snapshot hash | nonzero; included in stable report checksum |
| continue hash matches after reload | true |
| round-trip hash before | `1c13da017cfb496d` |
| round-trip hash after | `1c13da017cfb496d` |

The shadow snapshot covers systems, fleets, membership entities, stockpiles, construction progress,
blockade owners, Tier-A value arrays, fleet positions, birth/removal flags, lineage, and allocation
membership. It is a CPU-side whole-world mirror; resident GPU tables are an acceleration path, never the
only copy of structural state.

## R1c Stop Line

| Stop-line check | Result |
| --- | --- |
| stop line triggered | true |
| requires resident free-list scatter | true |
| requires resident compaction or lineage update | true |
| requires M-4A or multi-atlas now | false |
| semantic GPU code required | false |
| CPU planner required | false |
| `docs/invariants.md` edit required | false |
| pinned-number change required | false |
| scenario reopen required | false |
| next smaller rung | `R1c-a resident free-list mark-only / no compaction` |

## Authority Flags

| Flag | Value |
| --- | --- |
| structural decisions GPU emitted | false |
| resident REENROLL scatter authority | false |
| resident birth/removal authority | false |
| resident fusion compaction authority | false |
| CPU decision witness still authority | true |
| resident event journal remains only structural handoff | true |

## Backpressure Policy

| Field | Value |
| --- | --- |
| policy | bounded-lag journal drain at tick boundary; GPU value loop remains independent |
| GPU value loop may run ahead | true |
| CPU boundary consumer is not hot-path gate | true |
| max unserialized ticks documented | 1 |
| per-tick decision readback forbidden | true |

## R6C Checksum

- Expected: `1bba891c779190a4`
- Observed: `1bba891c779190a4`
- Matches: true

## Exact Command Results

```text
cargo test -p simthing-driver --test runtime_0080_0_r1c -> 11 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r1b -> 26 passed; 0 failed
cargo test -p simthing-driver --test runtime_0080_0_r1a -> 35 passed; 0 failed
cargo test -p simthing-driver --test dress_rehearsal_r6c_integrated_run -> 22 passed; 0 failed
cargo check --workspace -> passed
```
