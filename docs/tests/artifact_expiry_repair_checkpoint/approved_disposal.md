# Approved artifact-expiry disposal

Status: **PROBATION / proof-present / DA-review-pending**. Authority: Board
5674704988, applied on `codex/artifact-expiry-repair` after checkpoint
`8d93a387ffc754049d066b752311b420b4e9ed10`. Base:
`c26f02270ed16749170c780c4d9cadb856f51a2b`. The original proposal and
reclassification artifacts remain unchanged historical review records; this
report and [applied table](approved_dispositions.tsv) record the final decision.

| Approved manual population | DELETE | ELEVATE | Total |
| --- | ---: | ---: | ---: |
| Exact parked-test identities | 191 | 4 | 195 |
| Whole-file lease identities | 17 | 67 | 84 |
| Total | 208 | 71 | 279 |

The 82 former STOP rows were deleted by the explicit lifecycle-expiry decision.
They receive no invented replacement, renewed lease or permanent exemption.
Each removal is confined to the exact function and its attributes/doc comment;
shared files, helpers, imports and surviving functions remain. The
[removal manifest](approved_function_removals.tsv) records the 191 identities
and hashes of their removed source items.

M057/M112 retain their exact Embedder Guide-consumed function paths. M120/M141
retain their exact ignored generator tests and existing script callers. These
four identities moved from the old parking pen into finite current
`0.0.8.8-integrated-rehearsal` proof ownership. Both generator scripts are
byte-identical; any later generator rehome remains separate D-scope.

The finance correction restores
`crates/simthing-embedder/tests/finance_toy_0.rs` byte-for-byte from the base
commit. Its exact `finance_toy_five_verbs_observe_and_serialize` identity is a
current-track behavior/exemplar proof consumed by the Embedder Guide and hosted
checker. The previously reaped file lease stays discharged. This additional
restoration is separate from the unchanged 279-row manual population.

The 67 file elevations comprise 32 byte-identical research archive moves, one
byte-identical font move and 34 existing-path owner/container discharges.
The workshop shell removes only the archived module exports and obsolete
typeface shim. Three literal font include paths now point at `simthing-tools`.
Same-path numerical production sources and shader bytes remain unchanged.
Historical measurements retain their old meaning and do not become current
baselines. Every file has an explicit current owner in the applied table.

The live inventory is 995 unchanged rows plus five exact current consumers.
The parking pen is 559 minus 195: all 364 remaining rows retain their original
2026-09-10 dates and fields. The artifact ledger is 85 minus 84: its one
remaining row is unchanged. No lease date is minted.

[Preservation verification](approved_disposal_verification.txt) checks all 365
saved non-expired rows and 162 referenced files: 123 whole files are byte-identical;
39 shared files differ only by approved expired-function removals. Across all
68 files with function removals, 1,140 other function bodies remain byte-identical.
All remaining source bytes outside the exact removed item ranges are preserved,
apart from empty EOF separators exposed by deletion and removed for diff hygiene.

The carried reaper repair is exactly
`cecb22588ad9d5c8663eff0429f22a2b09bbcb82`. Its source is unchanged by disposal.
The cleanup PR remains open and unmerged for the existing DA gate-wiring route;
#2059 remains untouched and #2060 remains the canonical repair record.

Validation and hosted routing results are recorded in the final PR and Board
return. Local expiry has reached `expired=0`, with the 364 protected CRUFT rows
still reported as INSPECT; the restored guide check and all five planted-defect
selftests pass. Cargo checks cover the ten affected crates with `--tests`.
