EML Phase 5 intensity spike rich report
n_slots: 100000
warm_runs: 10

Correctness:
EML vs CPU max_abs_error: 0
EML vs CPU mean_abs_error: 0
hardcoded vs CPU max_abs_error: 0
hardcoded vs CPU mean_abs_error: 0
EML vs hardcoded max_abs_error: 0
EML vs hardcoded mean_abs_error: 0
EML repeated runs identical: true
hardcoded repeated runs identical: true

Timing:
cpu_node_eval_us: 2839
cpu_direct_eval_us: 146
gpu_eml_cold_total_us: 880184
gpu_eml_warm_mean_us: 2314
gpu_eml_warm_min_us: 1610
gpu_eml_warm_max_us: 5962
gpu_hardcoded_warm_mean_us: 1370
gpu_hardcoded_warm_min_us: 1029
gpu_hardcoded_warm_max_us: 1988
dispatch_only_unavailable_reason: wgpu timestamp queries not implemented in this spike

Interpretation:
correctness_gate: PASS
determinism_gate: PASS
shader_performance_gate: INFORMATIVE_ONLY
note: GPU warm timings include buffer upload, dispatch, wait, and readback; not pure shader time.