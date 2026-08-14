# GPU-OVERLAY-LIFECYCLE-EXTRACTION-0 results

- Track: 0.0.8.7 RF arena modernization (rung 7.7)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `d37dd2a91292`
- ORIENT-RECEIPT: `8de008acfbdd`
- orientation_rule_stamp: `874354e66bc3ac81`
- ANCHOR-ACK: `orientation-harness-core@8a365d1c0864`
- ANCHOR-ACK: `scanner-selftest-delta-gate@34fb2662baae`
- ANCHOR-ACK: `overlay-germ@f0c8d2ebade9`
- ANCHOR-ACK: `overlay-promoted-laws@248c7893b462`
- ANCHOR-ACK: `core-overlays@f95c9376ee06`
- Board dispatch: comment `5290633692`
- expected_route: `DA-RESERVE(gate-wiring)`

## What landed

GPU owns overlay numerical lifecycle. `resolve_overlay_lifecycle` is the CPU oracle and is no longer the production boundary evaluator. `AfterTicks` is deadline comparison (`deadline_generation = g_activation + duration`) with zero CPU decrement. `OverrideReceived` is rejected at admission (no authored corpus uses it; matching/identity would have been a STOP).

Routed durations rebase at destination generation. Physical slots exist only in the upload row. Session templates freeze; mid-session mint REDs.

## Mutants (intended reasons)

| Mutant | Reason |
|---|---|
| GPU/oracle divergence | deadline compare mismatch |
| AfterTicks decrement | `*remaining -= 1` in production evaluator |
| OverrideReceived admit | variant still a dissolve arm |
| Mid-session template mint | `MidSessionTemplateMint` |
| Overlay-local EML table | `OverlayLocalEmlTable` |
| Durable-row capture | `DurableRowCapture` |
| Foreign absolute deadline | dest rebase ≠ origin+duration |
| Global clock | `GlobalClock` |
| Second crossing detector | `SecondCrossingDetector` — expiry stays on the existing Phase-5 / OverlayDissolved surface |
| Deadline overflow | fail-closed, no wrap |

Pre-extraction OverlayDissolved recording `{host, overlay}` replays post-extraction. Carry = `instances * 32` bytes measured before any compaction.

7.6 census: clean-tree `--harvest` universe unchanged (71 routes; pin not hand-edited). `--check` required four `# RESIDUE` justifications for overlay-named 7.7 helpers that are not harvested route names: `admit_overlay_lifecycle`, `resolve_overlay_lifecycle_oracle`, `bind_tree_overlays`, `refuse_overlay_local_eml_table`. Residue 49→53. No new harvested token.

## Fences

No OverlayHistory, no ladder/stamp edit, no `binding_conditions.tsv` close, no scan retirement, no 7.8/7.8a/7.9. `OVERLAY-PEER-AUTHORITY` stays live.
