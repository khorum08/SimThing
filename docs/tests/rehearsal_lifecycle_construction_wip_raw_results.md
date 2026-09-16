# Construction Leaf A — tenth-roll raw observations

Base: `72d3c338ce91a47af17cfed5420b316a2484ede4`. Execution order: ingress, unchanged allocation, stock settlement. Commands, interpretation, fixture provenance and lifecycle limitations are in [the WIP results](rehearsal_lifecycle_construction_wip_results.md). Final committed-head reruns are attested by the PR validation and Board return.

## Canonical ingress

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
    semantic_kernel_bundle_hash: 8939920314109409953,
    workgroups: [
        32,
        64,
    ],
    subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority",
    abi_version: 1,
}
required_fingerprint: d5fcaf92eda4f724
observed_fingerprint: d5fcaf92eda4f724
bundle: 7c10fa28f2ca3aa1
ordinary session admitted; construction discriminator may proceed
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.23s
```

## Unchanged independent-resource discriminator

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

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.65s
```

## Canonical stock settlement

```text
running 3 tests
test leaf_residual_must_settle_as_owned_balance_for_each_resource ... component ["a"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["a"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
component ["b"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["b"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
component ["a", "b"] children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=false arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=true arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]
ordinary children_reversed=true arenas_reversed=true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]], [[0.0, 0.0, 0.0], [0.25, 0.25, 0.25], [0.75, 0.75, 0.75]]]

thread 'leaf_residual_must_settle_as_owned_balance_for_each_resource' (2844) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_wip.rs:315:5:
ordinary delivered flow never settled into project-owned WIP: ["component [\"a\"]/false/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]", "component [\"a\"]/false/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]", "component [\"a\"]/true/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]", "component [\"a\"]/true/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]]]", "component [\"b\"]/false/false: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"b\"]/false/true: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"b\"]/true/false: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"b\"]/true/true: [[[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"a\", \"b\"]/false/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"a\", \"b\"]/false/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"a\", \"b\"]/true/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "component [\"a\", \"b\"]/true/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "ordinary false/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "ordinary false/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "ordinary true/false: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]", "ordinary true/true: [[[0.0, 0.0, 0.0], [0.75, 0.0, 0.0], [0.25, 0.0, 0.0]], [[0.0, 0.0, 0.0], [0.25, 0.0, 0.0], [0.75, 0.0, 0.0]]]"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED
test parent_surplus_integrates_through_existing_balance_door ... parent surplus control ordinary=false: [[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]
parent surplus control ordinary=true: [[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]
ok
test seeded_leaf_rate_integrates_once_in_one_ordinary_generation ... seeded rate component: [[[0.0, 0.0, 0.0], [0.0, 0.5, 0.5], [0.0, 0.0, 0.0]]]
seeded rate ordinary: [[[0.0, 0.0, 0.0], [0.0, 0.5, 1.5], [0.0, 0.0, 0.0]]]; expected=[[[0.0, 0.0, 0.0], [0.0, 0.5, 0.5], [0.0, 0.0, 0.0]]]

thread 'seeded_leaf_rate_integrates_once_in_one_ordinary_generation' (32940) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_wip.rs:257:5:
assertion `left == right` failed: one dt=1 generation must integrate rate 0.5 exactly once
  left: [[[0.0, 0.0, 0.0], [0.0, 0.5, 1.5], [0.0, 0.0, 0.0]]]
 right: [[[0.0, 0.0, 0.0], [0.0, 0.5, 0.5], [0.0, 0.0, 0.0]]]
FAILED

failures:

failures:
    leaf_residual_must_settle_as_owned_balance_for_each_resource
    seeded_leaf_rate_integrates_once_in_one_ordinary_generation

test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.01s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_wip`
```
