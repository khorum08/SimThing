> Historical pre-tenth-roll evidence. #2064 repairs the independent-resource binding defect; the unchanged discriminator is now GREEN. Current Leaf A status and the next stock-settlement blocker are in [the WIP results](rehearsal_lifecycle_construction_wip_results.md). The evidence below is preserved as recorded.

# Construction Leaf A — resumed raw allocation evidence

Status: PROBATION / blocker-proof-present / BLOCKED / OPEN / UNMERGED.
Base: df5dd480b42dc224926c8bbab1d229b988b3f57c.
Captured on the reference GPU after synchronizing the same Leaf A branch.
Commands and interpretation: rehearsal_lifecycle_construction_allocation_results.md.
Final-head verification and hosted run references are in PR #2062 and the Board return.

## Canonical ingress (GREEN)

```text
adapter: NVIDIA GeForce RTX 4080 Laptop GPU; Vulkan; NVIDIA 595.79
compiler: rustc 1.95.0 (59807616e 2026-04-14); LLVM 22.1.2
cargo_features: EML_RESOURCE_PROFILING
required_fingerprint: 6fe1d809c05ee0f4
observed_fingerprint: 6fe1d809c05ee0f4
bundle: c6b6a70c64f7ae81
ordinary session admitted; construction discriminator may proceed
test construction_leaf_a_requires_qualified_ordinary_session ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Required independent-resource allocation (RED)

The following are actual GPU readbacks printed by the focused test, not CPU
replacement allocations. `expected` is only the test assertion oracle.

```text
single a: [[0.75, 0.25]]
single b: [[0.25, 0.75]]
component children=P,Q reverse_arenas=false: [[0.75, 0.25], [0.75, 0.25]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=P,Q reverse_arenas=true: [[0.25, 0.75], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=Q,P reverse_arenas=false: [[0.75, 0.25], [0.75, 0.25]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=Q,P reverse_arenas=true: [[0.25, 0.75], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
independent-resource policy binding failed: [("P,Q", false, [[0.75, 0.25], [0.75, 0.25]]), ("P,Q", true, [[0.25, 0.75], [0.25, 0.75]]), ("Q,P", false, [[0.75, 0.25], [0.75, 0.25]]), ("Q,P", true, [[0.25, 0.75], [0.25, 0.75]])]
ordinary reverse_children=false: [[0.75, 0.25], [0.75, 0.25]]; expected=[[0.75, 0.25], [0.25, 0.75]]
ordinary reverse_children=true: [[0.75, 0.25], [0.75, 0.25]]; expected=[[0.75, 0.25], [0.25, 0.75]]
ordinary two-input discriminator cannot lawfully allocate opposite policies: [(false, [[0.75, 0.25], [0.75, 0.25]]), (true, [[0.75, 0.25], [0.75, 0.25]])]
failures:
    each_resource_policy_works_alone_but_joint_plan_must_keep_both
    ordinary_resident_session_must_preserve_opposite_resource_policies
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
```

Exit 101. No expected-failure annotation, skip, CPU fallback, source patch,
qualification bypass, or synthetic correction of B's allocation.
