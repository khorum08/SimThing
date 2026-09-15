# simthing-workshop

This crate holds the current rehearsal measurement instruments and isolated
reference support. It remains a leaf consumer of the production crates.

The current owners are:

- `rehearsal_perf_guyang_gather_0` and `rehearsal_perf_tr_composite_0`: current
  workload-qualified field and resident-generation measurements.
- `actionband_*`: synthetic movement witness/support for `0088-MOVEMENT-0`.
- `generation_critical_path_baseline*` and `resident_clearing_plan`: CPU-door and
  resident-plan reference support for `0088-APPLICATION-PERFORMANCE-0` and
  `0088-CONTINUATION-RELIABILITY-0`.
- Qualification, arithmetic and field-probe integration-test containers: current
  numerical/measurement support; each proof keeps its own inventory lifecycle.

The exact ownership and disposal account is in
[`approved_dispositions.tsv`](../../docs/tests/artifact_expiry_repair_checkpoint/approved_dispositions.tsv).
Container ownership does not renew individual parked tests.

The historical EML Phase 5, WeightedMean, overlay-order, multi-target replay,
transfer-contention and persistent-buffer experiments are now non-runnable
[source/report evidence](../../docs/archive/0087-workshop-research/README.md).
Their old timings are historical observations, not current application baselines.
The current research inventory owns any subsequent comparison or requalification.

The typeface implementation and fixture font are owned by `simthing-tools`;
Studio consumes that font directly. The obsolete workshop re-export is removed.

Check the surviving crate with `cargo check -p simthing-workshop --tests`.
Run a named current test target when collecting a new measurement; retain its
workload, completion boundary, adapter and code identity with the result.
