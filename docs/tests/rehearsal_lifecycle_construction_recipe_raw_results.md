# Construction Leaf A — eleventh-roll recipe/admission raw observations

Base: `ec078b3095b19218c2d50b0b9a7e607967f029c1`. Order: ingress → unchanged allocation → stock/authorized diagnostic → recipes. Interpretations and exact commands are in [the current results](rehearsal_lifecycle_construction_recipe_results.md); final-head reruns are attested by the PR and Board return.

## Ingress

```text
running 1 test
test construction_leaf_a_requires_qualified_ordinary_session ... adapter: AdapterInfo { name: "NVIDIA GeForce RTX 4080 Laptop GPU", vendor: 4318, device: 10144, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "595.79", backend: Vulkan }
qualification: ResidentClearingQualification {
    backend: "Vulkan",
    adapter: "NVIDIA GeForce RTX 4080 Laptop GPU",
    vendor: 4318,
    device: 10144,
    device_class: "DiscreteGpu",
    driver_runtime: "NVIDIA 595.79",
    features: "Features(DEPTH_CLIP_CONTROL | DEPTH32FLOAT_STENCIL8 | TEXTURE_COMPRESSION_BC | TIMESTAMP_QUERY | INDIRECT_FIRST_INSTANCE | SHADER_F16 | RG11B10UFLOAT_RENDERABLE | BGRA8UNORM_STORAGE | FLOAT32_FILTERABLE | TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | PIPELINE_STATISTICS_QUERY | TIMESTAMP_QUERY_INSIDE_ENCODERS | TIMESTAMP_QUERY_INSIDE_PASSES | MAPPABLE_PRIMARY_BUFFERS | TEXTURE_BINDING_ARRAY | BUFFER_BINDING_ARRAY | STORAGE_RESOURCE_BINDING_ARRAY | SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING | PARTIALLY_BOUND_BINDING_ARRAY | MULTI_DRAW_INDIRECT | MULTI_DRAW_INDIRECT_COUNT | PUSH_CONSTANTS | ADDRESS_MODE_CLAMP_TO_ZERO | ADDRESS_MODE_CLAMP_TO_BORDER | POLYGON_MODE_LINE | POLYGON_MODE_POINT | CONSERVATIVE_RASTERIZATION | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | SPIRV_SHADER_PASSTHROUGH | MULTIVIEW | TEXTURE_FORMAT_NV12 | RAY_TRACING_ACCELERATION_STRUCTURE | RAY_QUERY | SHADER_F64 | SHADER_I16 | SHADER_PRIMITIVE_INDEX | DUAL_SOURCE_BLENDING | SHADER_INT64 | SUBGROUP | SUBGROUP_VERTEX | SUBGROUP_BARRIER | PIPELINE_CACHE | SHADER_INT64_ATOMIC_MIN_MAX | SHADER_INT64_ATOMIC_ALL_OPS)",
    compiler: "rustc 1.95.0 (59807616e 2026-04-14)\nbinary: rustc\ncommit-hash: 59807616e1fa2540724bfbac14d7976d7e4a3860\ncommit-date: 2026-04-14\nhost: x86_64-pc-windows-msvc\nrelease: 1.95.0\nLLVM version: 22.1.2",
    cargo_features: "EML_RESOURCE_PROFILING",
    shader_compiler: "wgpu 22.1.0 / naga 22.1.0",
    cargo_lock_hash: 2005979115394712535,
    semantic_kernel_bundle_hash: 8120982973085176555,
    workgroups: [
        32,
        64,
    ],
    subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority",
    abi_version: 1,
}
required_fingerprint: f65dd84eae5040c6
observed_fingerprint: f65dd84eae5040c6
bundle: 70b386fcb9bcd2eb
ordinary session admitted; construction discriminator may proceed
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.27s
```

## Independent allocation

```text
running 2 tests
test each_resource_policy_works_alone_but_joint_plan_must_keep_both ... single a: [[0.75, 0.25]]
single b: [[0.25, 0.75]]
component children=P,Q reverse_arenas=false: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=P,Q reverse_arenas=true: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=Q,P reverse_arenas=false: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
component children=Q,P reverse_arenas=true: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
ok
test ordinary_resident_session_must_preserve_opposite_resource_policies ... ordinary reverse_children=false: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
ordinary reverse_children=true: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.54s
```

## Owned WIP and corrected non-arena rate

```text
running 3 tests
test leaf_residual_must_settle_as_owned_balance_for_each_resource ... component ["a"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["b"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ok
test parent_surplus_integrates_through_existing_balance_door ... parent surplus control ordinary=false: [[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]
parent surplus control ordinary=true: [[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]
ok
test seeded_leaf_rate_integrates_once_in_one_ordinary_generation ... non-arena free-rate generation=1: [0.5, 0.5]
non-arena free-rate generation=2: [0.5, 1.0]
ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.01s
```

## Recipe controls and combined admission refusal

```text
running 3 tests
test ordinary_recipe_preserves_incomplete_wip_and_consumes_funded_stock_once ... recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=q pulse=[1.0, 1.0] children_reversed=true arenas_reversed=true g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=false arenas_reversed=false g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=q pulse=[1.0, 0.0] children_reversed=true arenas_reversed=true g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=2: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=3: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=4: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=5: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=2: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=3: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=4: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=p pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=5: [[2.0, 0.0, 1.0], [1.0, 3.0, 0.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=2: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=3: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=4: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=false arenas_reversed=false g=5: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=2: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=3: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=4: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
recipe host=q pulse=[4.0, 4.0] children_reversed=true arenas_reversed=true g=5: [[3.0, 1.0, 0.0], [0.0, 2.0, 1.0]]
ok
test two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session ... joint hosts=["p", "q"] children=false arenas=false: materialized host slots=[1, 2]; ordinary=hosts=["p", "q"]/children=false/arenas=false: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["q", "p"] children=false arenas=false: materialized host slots=[1, 2]; ordinary=hosts=["q", "p"]/children=false/arenas=false: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["p", "q"] children=false arenas=true: materialized host slots=[1, 2]; ordinary=hosts=["p", "q"]/children=false/arenas=true: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["q", "p"] children=false arenas=true: materialized host slots=[1, 2]; ordinary=hosts=["q", "p"]/children=false/arenas=true: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["p", "q"] children=true arenas=false: materialized host slots=[1, 2]; ordinary=hosts=["p", "q"]/children=true/arenas=false: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["q", "p"] children=true arenas=false: materialized host slots=[1, 2]; ordinary=hosts=["q", "p"]/children=true/arenas=false: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["p", "q"] children=true arenas=true: materialized host slots=[1, 2]; ordinary=hosts=["p", "q"]/children=true/arenas=true: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })
joint hosts=["q", "p"] children=true arenas=true: materialized host slots=[1, 2]; ordinary=hosts=["q", "p"]/children=true/arenas=true: Install(NeedBindingInvalid { binding: "resource_economy", reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority", span_token: None })

thread 'two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session' (33120) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_recipe.rs:387:5:
distinct explicit recipe hosts were conflated: ["hosts=[\"p\", \"q\"]/children=false/arenas=false: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"q\", \"p\"]/children=false/arenas=false: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"p\", \"q\"]/children=false/arenas=true: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"q\", \"p\"]/children=false/arenas=true: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"p\", \"q\"]/children=true/arenas=false: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"q\", \"p\"]/children=true/arenas=false: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"p\", \"q\"]/children=true/arenas=true: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })", "hosts=[\"q\", \"p\"]/children=true/arenas=true: Install(NeedBindingInvalid { binding: \"resource_economy\", reason: \"property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority\", span_token: None })"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test unresolved_or_ambiguous_recipe_host_still_refuses_with_provenance ... negative ambiguous=false: economy host entity `p` is not in install_targets; span=Some(37)
negative ambiguous=true: entity `p` host is ambiguous (2 hosts) for economy property placement; span=Some(37)
ok

failures:

failures:
    two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.24s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`
```
