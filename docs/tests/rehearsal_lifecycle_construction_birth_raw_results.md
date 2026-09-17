> HISTORICAL: the first-birth failure below is repaired by #2067 at 6dbab071be7afd3967c63663badcb9067e94ac22. All ten unchanged Leaf-A tests, including first birth and G4/G5 continuation, passed on the synchronized branch. Current funded-cancellation findings are in [release results](rehearsal_lifecycle_construction_release_results.md). The original packet below is preserved.

# Construction Leaf A — funded structural continuation raw packet

Authority and interpretation: [birth results](rehearsal_lifecycle_construction_birth_results.md).

Synchronized base 7720499d3c9c7a5e777b647952e27567eaf76639. Logs below come from the local profiled GPU executions during development; final exact-head aggregate results are in the PR/Board return. No GPU skip.


## ingress-resume12.txt

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.35s
```


## discriminator-resume12.txt

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

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.89s
```


## wip-resume12.txt

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

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.99s
```


## recipe-resume12.txt

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
test two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session ... joint hosts=["p", "q"] children=false arenas=false: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["q", "p"] children=false arenas=false: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["p", "q"] children=false arenas=true: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["q", "p"] children=false arenas=true: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["p", "q"] children=true arenas=false: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["q", "p"] children=true arenas=false: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["p", "q"] children=true arenas=true: materialized host slots=[1, 2]; ordinary=admitted
joint hosts=["q", "p"] children=true arenas=true: materialized host slots=[1, 2]; ordinary=admitted
ok
test unresolved_or_ambiguous_recipe_host_still_refuses_with_provenance ... negative ambiguous=false: economy host entity `p` is not in install_targets; span=Some(37)
negative ambiguous=true: entity `p` host is ambiguous (2 hosts) for economy property placement; span=Some(37)
ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 29.50s
```


## cancellation12-corrected.txt

```text
running 1 test
test cancelled_partial_wip_survives_recovery_reparent_and_restart ... cancel project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
cancel project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
restart project=0 reversed=false g=5: [[2.5, 1.5, 0.0], [1.5, 2.5, 0.0]]
restart project=0 reversed=false g=6: [[3.25, 1.75, 1.0], [1.75, 3.25, 1.0]]
restart project=0 reversed=false g=7: [[4.0, 2.0, 2.0], [2.0, 4.0, 2.0]]
restart project=0 reversed=false g=8: [[3.75, 1.25, 4.0], [1.25, 3.75, 4.0]]
cancel project=0 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
cancel project=0 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
restart project=0 reversed=true g=5: [[2.5, 1.5, 0.0], [1.5, 2.5, 0.0]]
restart project=0 reversed=true g=6: [[3.25, 1.75, 1.0], [1.75, 3.25, 1.0]]
restart project=0 reversed=true g=7: [[4.0, 2.0, 2.0], [2.0, 4.0, 2.0]]
restart project=0 reversed=true g=8: [[3.75, 1.25, 4.0], [1.25, 3.75, 4.0]]
cancel project=1 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
cancel project=1 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
restart project=1 reversed=false g=5: [[2.5, 1.5, 0.0], [1.5, 2.5, 0.0]]
restart project=1 reversed=false g=6: [[3.25, 1.75, 1.0], [1.75, 3.25, 1.0]]
restart project=1 reversed=false g=7: [[4.0, 2.0, 2.0], [2.0, 4.0, 2.0]]
restart project=1 reversed=false g=8: [[3.75, 1.25, 4.0], [1.25, 3.75, 4.0]]
cancel project=1 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
cancel project=1 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
restart project=1 reversed=true g=5: [[2.5, 1.5, 0.0], [1.5, 2.5, 0.0]]
restart project=1 reversed=true g=6: [[3.25, 1.75, 1.0], [1.75, 3.25, 1.0]]
restart project=1 reversed=true g=7: [[4.0, 2.0, 2.0], [2.0, 4.0, 2.0]]
restart project=1 reversed=true g=8: [[3.75, 1.25, 4.0], [1.25, 3.75, 4.0]]
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 5.09s
```


## birth12-corrected.txt

```text
running 1 test
test first_funded_structural_birth_must_complete_and_leave_ordinary_session_usable ... birth pulse=[1.0, 1.0] reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[1.0, 1.0] reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[4.0, 4.0] reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[4.0, 4.0] reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=false component=false placed=false action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=false g=3 result=Err(ActionBandIngress(BindingTableStale)) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth retry 0: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")
birth retry 1: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")
birth retry 2: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")
birth pulse=[4.0, 4.0] reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[4.0, 4.0] reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=false component=false placed=false action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=true g=3 result=Err(ActionBandIngress(BindingTableStale)) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth retry 0: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")
birth retry 1: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")
birth retry 2: ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")

thread 'first_funded_structural_birth_must_complete_and_leave_ordinary_session_usable' (10152) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_recipe.rs:674:5:
funded structural boundary cannot finish: ["pulse=[4.0, 4.0]/reversed=false/g=3: ActionBandIngress(BindingTableStale); born=true; placed=true", "pulse=[4.0, 4.0]/reversed=true/g=3: ActionBandIngress(BindingTableStale); born=true; placed=true"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    first_funded_structural_birth_must_complete_and_leave_ordinary_session_usable

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.64s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`
```


## birth-control12.txt

```text
running 1 test
test ordinary_add_child_control_without_actionband_can_finish_and_continue ... external AddChild control g=3: Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
external AddChild control g=4: Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 1.31s
```


## Joint conservation matrix (120 generations)

```text
test joint_projects_preserve_partial_wip_and_consume_only_complete_owned_inputs ... joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["q", "p"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["q", "p"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["q", "p"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["q", "p"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["q", "p"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["p", "q"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["p", "q"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["p", "q"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["p", "q"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["p", "q"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["q", "p"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["q", "p"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["q", "p"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["q", "p"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=false arenas=true hosts=["q", "p"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["p", "q"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["p", "q"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["p", "q"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["p", "q"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["p", "q"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["q", "p"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["q", "p"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["q", "p"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["q", "p"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=false hosts=["q", "p"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["p", "q"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["p", "q"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["p", "q"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["p", "q"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["p", "q"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["q", "p"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["q", "p"] g=2: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["q", "p"] g=3: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["q", "p"] g=4: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 1.0] children=true arenas=true hosts=["q", "p"] g=5: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["p", "q"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["p", "q"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["p", "q"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["p", "q"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["p", "q"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["q", "p"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["q", "p"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["q", "p"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["q", "p"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=false hosts=["q", "p"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["p", "q"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["p", "q"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["p", "q"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["p", "q"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["p", "q"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["q", "p"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["q", "p"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["q", "p"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["q", "p"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=false arenas=true hosts=["q", "p"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["p", "q"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["p", "q"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["p", "q"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["p", "q"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["p", "q"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["q", "p"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["q", "p"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["q", "p"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["q", "p"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=false hosts=["q", "p"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["p", "q"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["p", "q"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["p", "q"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["p", "q"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["p", "q"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["q", "p"] g=1: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["q", "p"] g=2: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["q", "p"] g=3: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["q", "p"] g=4: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[1.0, 0.0] children=true arenas=true hosts=["q", "p"] g=5: [[0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["p", "q"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["p", "q"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["p", "q"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["p", "q"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["p", "q"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["q", "p"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["q", "p"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["q", "p"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["q", "p"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=false hosts=["q", "p"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["p", "q"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["p", "q"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["p", "q"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["p", "q"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["p", "q"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["q", "p"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["q", "p"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["q", "p"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["q", "p"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=false arenas=true hosts=["q", "p"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["p", "q"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["p", "q"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["p", "q"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["p", "q"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["p", "q"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["q", "p"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["q", "p"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["q", "p"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["q", "p"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=false hosts=["q", "p"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["p", "q"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["p", "q"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["p", "q"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["p", "q"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["p", "q"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["q", "p"] g=1: [[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["q", "p"] g=2: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["q", "p"] g=3: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["q", "p"] g=4: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
joint pulse=[4.0, 4.0] children=true arenas=true hosts=["q", "p"] g=5: [[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]]
ok
```


## Fixture corrections preserved

Exploratory runs exposed fixture assumptions, not production defects. They
were corrected without changing any prior prerequisite test or stock expectation:

1. Cancelling source flow with `multiply(0)` and later merely suspending that
   transform does not invert its historical write: G5 still had the initial WIP.
   Restart now explicitly authors source flow, never stock.
2. A root `set(1)` policy propagates to P/Q, so resumed supply is three units per
   resource per generation, not one. Observed G5 P(2.5,1.5), Q(1.5,2.5) is lawful.
   The declared restart profile and exact independent conservation oracle now
   account for all three producer flows. Both cancellation orders are GREEN.
3. Phase-5 upward crossing is `previous <= threshold && current > threshold`.
   Threshold 1 does not fire at integral output 1; threshold .5 now detects the
   first complete integral unit. The unchanged economic recipes produce integers,
   and the incomplete-stock structural negatives remain GREEN. The resulting
   actual boundary then exposes `BindingTableStale`; no substitute CPU trigger.

Original scratch captures remain `recipe-continuation12.txt`, `cancellation12.txt`
and `birth12.txt`; these intermediate fixture failures are not Model-1 verdicts.
The final required-success structural witness remains RED for the actual
production composition defect. No existing test was made ignored or expected-fail.

Final harness accounting: the four new functions above became four subcases of
`ordinary_joint_lifecycle_matrix` after the initial TEST-BUDGET INSPECT on seven
test attributes in the recipe file. All assertions and original tests remain.
Final recipe aggregate is 3 passed / 1 failed; whole focused suite 9 passed / 1 failed.
Development logs retain their original individual-test grouping.
