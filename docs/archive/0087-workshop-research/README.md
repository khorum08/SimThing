# Historical workshop research evidence

Board disposition 5674704988 moved these exact 0.0.8.7 files out of the live
workshop crate. Source, shaders and reports are byte-identical to the approved
checkpoint. No module in this directory is exported by Cargo.

The families are `persistent_bench`, `multitarget_replay`, `transfer_contention`,
`overlay_order`, `eml_phase5` (including its report formatter), and `weighted_mean`.
Their historical source retains the original relative includes and report
interpretation. Paths mentioned inside old reports describe the historical run.

Current owner: `0088-APPLICATION-PERFORMANCE-0`, M13, through
[the research inventory](../../tests/rehearsal_perf_research_inventory_0_results.md).
The current workload-qualified instruments are `rehearsal_perf_guyang_gather_0`
and `rehearsal_perf_tr_composite_0`. Historical timings do not become current
baselines through this archive move; current-path validity remains unresolved.

[The approved disposition table](../../tests/artifact_expiry_repair_checkpoint/approved_dispositions.tsv)
records each original path, destination and owner. This is evidence preservation,
with no new lease or executable proof exemption.
