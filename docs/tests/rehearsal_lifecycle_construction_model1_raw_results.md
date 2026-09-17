# Construction Leaf A — complete Model-1 raw execution

PASS-MODEL-1 / PROBATION / proof-present. No rung graduation or merge.
Base: 2e334ddd6718c332d8c29736c1c9166bbb4c97c5.
Executed code head: 4f2f1fccfcf44de43a995c5112c7cd65b4cd7a9c.
Recipe SHA-256: 5ab7655fd70d335906bb7000442278fa4b5d1b68df433289532157361539a805.

All runtime output below, in execution order; only compiler warnings/build progress are omitted.
The result report gives exact commands. Final documentation-bearing head reruns are bound in the PR/Board return.
Raw log hashes cover the original complete files, including compiler output.

## ingress

Raw log SHA-256: 6f88bfcf587dcd19b06e72da527cea72bff3e75b352c3863a9911467eda21f93.

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
    semantic_kernel_bundle_hash: 11250151406862512699,
    workgroups: [
        32,
        64,
    ],
    subgroup_assumption: "subgroup-independent:no-subgroup-builtins-or-size-authority",
    abi_version: 1,
}
required_fingerprint: ec5a2a30afaee795
observed_fingerprint: ec5a2a30afaee795
bundle: 9c2091194366b23b
ordinary session admitted; construction discriminator may proceed
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.50s
```

## discriminator

Raw log SHA-256: e6624aa573344a2037cfcb4366ab6aee942d47ddc79d65976b64e448fba6265d.

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

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.64s
```

## wip

Raw log SHA-256: ba1e399d6a5fc62ebc6cd0944c6683c1246433cb2eb040e3e489a25b02ab1e7a.

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

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.72s
```

## recipe

Raw log SHA-256: 6a9b0f02a0204f231fc86307dea59d3f8da8f5bbc745dfcb74336f5ec0e34e86.

```text
running 8 tests
test canonical_multi_remove_must_preserve_requested_identities ... multi-remove indices=[0, 2] requested=[SimThingId(4), SimThingId(6)] actual_removed=[SimThingId(4), SimThingId(6)]
multi-remove indices=[0, 2]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }
multi-remove indices=[2, 0] requested=[SimThingId(16), SimThingId(14)] actual_removed=[SimThingId(14), SimThingId(16)]
multi-remove indices=[2, 0]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }
multi-remove indices=[3, 4] requested=[SimThingId(27), SimThingId(28)] actual_removed=[SimThingId(27), SimThingId(28)]
multi-remove indices=[3, 4]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }
multi-remove indices=[4, 3] requested=[SimThingId(38), SimThingId(37)] actual_removed=[SimThingId(37), SimThingId(38)]
multi-remove indices=[4, 3]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }
ok
test funded_product_cancellation_must_release_placement_and_continue ... cancel setup funded=false project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(41), grantee: SimThingId(44), market_grant_key: 11398904908351112381 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=false g=5: healthy
cancel continuation funded=false project=0 reversed=false g=6: healthy
cancel setup funded=false project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(48), grantee: SimThingId(51), market_grant_key: 3536011259070016897 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=true g=5: healthy
cancel continuation funded=false project=0 reversed=true g=6: healthy
cancel setup funded=false project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(55), grantee: SimThingId(58), market_grant_key: 18434364953901430701 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=false g=5: healthy
cancel continuation funded=false project=1 reversed=false g=6: healthy
cancel setup funded=false project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(62), grantee: SimThingId(65), market_grant_key: 5470639023723137953 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=true g=5: healthy
cancel continuation funded=false project=1 reversed=true g=6: healthy
cancel setup funded=true project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(69), grantee: SimThingId(72), market_grant_key: 17443327128941301813 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=0 reversed=false g=5: healthy
cancel continuation funded=true project=0 reversed=false g=6: healthy
cancel setup funded=true project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(76), grantee: SimThingId(79), market_grant_key: 17105437865553382337 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=0 reversed=true g=5: healthy
cancel continuation funded=true project=0 reversed=true g=6: healthy
cancel setup funded=true project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(83), grantee: SimThingId(86), market_grant_key: 4649988275375998457 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=1 reversed=false g=5: healthy
cancel continuation funded=true project=1 reversed=false g=6: healthy
cancel setup funded=true project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(90), grantee: SimThingId(93), market_grant_key: 2199242831400765245 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=1 reversed=true g=5: healthy
cancel continuation funded=true project=1 reversed=true g=6: healthy
ok
test ordinary_construction_terminal_matrix ... terminal negative project=0 case=stale-binding attempt=0: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=stale-binding attempt=1: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=stale-binding attempt=2: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=foreign-binding attempt=0: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=foreign-binding attempt=1: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=foreign-binding attempt=2: ActionBandIngress(BindingTableStale)
terminal negative project=0 case=remove-bound: ActionBandIngress(BoundIdentityRemapped { id: 119 })
terminal fail-stop remove-bound retry=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal fail-stop remove-bound retry=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal fail-stop remove-bound retry=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal negative project=0 case=duplicate-product g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, true] action_generation=Some(1) crossings=[[]]
terminal negative project=0 case=duplicate-product g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] products=[true, true] action_generation=Some(1) crossings=[[]]
terminal negative project=0 case=duplicate-product g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, true] action_generation=Some(2) crossings=[[BandCrossingDelta { generation: 5, reg_idx: 1, sim_thing_id: SimThingId(127), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 1.5, direction: Rising, post_value: 2.0, event_kind: 1 }]]
terminal negative project=0 case=duplicate-product g=7 result=Err(GpuSync(GrowthEntitlement("residency placement configuration refused: growth grantee 129 is already resident"))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, true] action_generation=Some(2) crossings=[]
terminal fail-stop duplicate-product retry=0: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal fail-stop duplicate-product retry=1: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal fail-stop duplicate-product retry=2: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal negative project=0 case=changed-crossing-threshold g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal negative project=0 case=changed-crossing-threshold g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal negative project=0 case=changed-crossing-threshold g=6 result=Err(ActionBandIngress(Dispatch(Gpu("sealed crossing at registration index 1 carries threshold bits 0x3fa00000, but band 1 froze admitted threshold bits 0x3fc00000: the bound threshold definition was redefined after ActionBand admission")))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, false] action_generation=Some(1) crossings=[[BandCrossingDelta { generation: 5, reg_idx: 1, sim_thing_id: SimThingId(134), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 1.25, direction: Rising, post_value: 2.0, event_kind: 1 }]]
terminal fail-stop changed-crossing-threshold retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal fail-stop changed-crossing-threshold retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal fail-stop changed-crossing-threshold retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal negative project=1 case=stale-binding attempt=0: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=stale-binding attempt=1: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=stale-binding attempt=2: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=foreign-binding attempt=0: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=foreign-binding attempt=1: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=foreign-binding attempt=2: ActionBandIngress(BindingTableStale)
terminal negative project=1 case=remove-bound: ActionBandIngress(BoundIdentityRemapped { id: 164 })
terminal fail-stop remove-bound retry=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal fail-stop remove-bound retry=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal fail-stop remove-bound retry=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
terminal negative project=1 case=duplicate-product g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, true] action_generation=Some(1) crossings=[[]]
terminal negative project=1 case=duplicate-product g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] products=[true, true] action_generation=Some(1) crossings=[[]]
terminal negative project=1 case=duplicate-product g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, true] action_generation=Some(2) crossings=[[BandCrossingDelta { generation: 5, reg_idx: 1, sim_thing_id: SimThingId(172), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 1.5, direction: Rising, post_value: 2.0, event_kind: 1 }]]
terminal negative project=1 case=duplicate-product g=7 result=Err(GpuSync(GrowthEntitlement("residency placement configuration refused: growth grantee 173 is already resident"))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, true] action_generation=Some(2) crossings=[]
terminal fail-stop duplicate-product retry=0: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal fail-stop duplicate-product retry=1: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal fail-stop duplicate-product retry=2: ExecutionIdentity("generation GenerationStamp(7) is faulted after an unfinished economic authorization")
terminal negative project=1 case=changed-crossing-threshold g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal negative project=1 case=changed-crossing-threshold g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal negative project=1 case=changed-crossing-threshold g=6 result=Err(ActionBandIngress(Dispatch(Gpu("sealed crossing at registration index 1 carries threshold bits 0x3fa00000, but band 1 froze admitted threshold bits 0x3fc00000: the bound threshold definition was redefined after ActionBand admission")))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] products=[true, false] action_generation=Some(1) crossings=[[BandCrossingDelta { generation: 5, reg_idx: 1, sim_thing_id: SimThingId(179), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 1.25, direction: Rising, post_value: 2.0, event_kind: 1 }]]
terminal fail-stop changed-crossing-threshold retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal fail-stop changed-crossing-threshold retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal fail-stop changed-crossing-threshold retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
terminal early crossing project=0 frozen=1.5 runtime=1.5 g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] products=[false, false] action_generation=Some(0) crossings=[[]]
terminal early crossing project=0 frozen=1.5 runtime=1.5 g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[false, false] action_generation=Some(1) crossings=[[BandCrossingDelta { generation: 1, reg_idx: 0, sim_thing_id: SimThingId(186), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.5, direction: Rising, post_value: 1.0, event_kind: 0 }]]
terminal early crossing project=0 frozen=1.5 runtime=1.5 g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal early product id=SimThingId(188) parent=Some(ChildOf(SimThingId(186))) slot=Some(SlotIndex(3)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(185), grantee: SimThingId(188), market_grant_key: 14472098009411547725 }, extent: ResidencyExtent { start: 3, length: 1 }, quantity: 1, committed_generation: GenerationStamp(3) }) added=true
terminal early product id=SimThingId(189) parent=None slot=None placement=None added=false
terminal early crossing project=0 frozen=1.5 runtime=1.5 g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal early crossing project=0 frozen=1.5 runtime=0.75 g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] products=[false, false] action_generation=Some(0) crossings=[[]]
terminal early crossing project=0 frozen=1.5 runtime=0.75 g=2 result=Err(ActionBandIngress(Dispatch(Gpu("sealed crossing at registration index 1 carries threshold bits 0x3f400000, but band 1 froze admitted threshold bits 0x3fc00000: the bound threshold definition was redefined after ActionBand admission")))) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[false, false] action_generation=Some(0) crossings=[[BandCrossingDelta { generation: 1, reg_idx: 0, sim_thing_id: SimThingId(193), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.5, direction: Rising, post_value: 1.0, event_kind: 0 }, BandCrossingDelta { generation: 1, reg_idx: 1, sim_thing_id: SimThingId(193), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.75, direction: Rising, post_value: 1.0, event_kind: 1 }]]
terminal fail-stop early-crossing retry=0: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal fail-stop early-crossing retry=1: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal fail-stop early-crossing retry=2: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal early crossing project=1 frozen=1.5 runtime=1.5 g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] products=[false, false] action_generation=Some(0) crossings=[[]]
terminal early crossing project=1 frozen=1.5 runtime=1.5 g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[false, false] action_generation=Some(1) crossings=[[BandCrossingDelta { generation: 1, reg_idx: 0, sim_thing_id: SimThingId(201), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.5, direction: Rising, post_value: 1.0, event_kind: 0 }]]
terminal early crossing project=1 frozen=1.5 runtime=1.5 g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal early product id=SimThingId(202) parent=Some(ChildOf(SimThingId(201))) slot=Some(SlotIndex(3)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(199), grantee: SimThingId(202), market_grant_key: 9472483962220602587 }, extent: ResidencyExtent { start: 3, length: 1 }, quantity: 1, committed_generation: GenerationStamp(3) }) added=true
terminal early product id=SimThingId(203) parent=None slot=None placement=None added=false
terminal early crossing project=1 frozen=1.5 runtime=1.5 g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[true, false] action_generation=Some(1) crossings=[[]]
terminal early crossing project=1 frozen=1.5 runtime=0.75 g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] products=[false, false] action_generation=Some(0) crossings=[[]]
terminal early crossing project=1 frozen=1.5 runtime=0.75 g=2 result=Err(ActionBandIngress(Dispatch(Gpu("sealed crossing at registration index 1 carries threshold bits 0x3f400000, but band 1 froze admitted threshold bits 0x3fc00000: the bound threshold definition was redefined after ActionBand admission")))) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] products=[false, false] action_generation=Some(0) crossings=[[BandCrossingDelta { generation: 1, reg_idx: 0, sim_thing_id: SimThingId(208), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.5, direction: Rising, post_value: 1.0, event_kind: 0 }, BandCrossingDelta { generation: 1, reg_idx: 1, sim_thing_id: SimThingId(208), property_id: SimPropertyId(2), role: Named("balance"), slot: SlotIndex(1), col: ColumnIndex(38), threshold: 0.75, direction: Rising, post_value: 1.0, event_kind: 1 }]]
terminal fail-stop early-crossing retry=0: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal fail-stop early-crossing retry=1: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal fail-stop early-crossing retry=2: ExecutionIdentity("generation GenerationStamp(2) is faulted after an unfinished economic authorization")
terminal independent order project=0 order=[false, false, false, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(220), SimThingId(222)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, false, false, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(233), SimThingId(235)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, true, false, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(246), SimThingId(248)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, true, false, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(259), SimThingId(261)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, false, true, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(272), SimThingId(274)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, false, true, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(285), SimThingId(287)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, true, true, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(298), SimThingId(300)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, true, true, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(311), SimThingId(313)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, false, false, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(324), SimThingId(326)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, false, false, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(337), SimThingId(339)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, true, false, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(350), SimThingId(352)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, true, false, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(363), SimThingId(365)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, false, true, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(376), SimThingId(378)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, false, true, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(389), SimThingId(391)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[false, true, true, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(402), SimThingId(404)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=0 order=[true, true, true, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(415), SimThingId(417)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, false, false, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(428), SimThingId(430)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, false, false, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(441), SimThingId(443)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, true, false, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(454), SimThingId(456)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, true, false, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(467), SimThingId(469)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, false, true, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(480), SimThingId(482)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, false, true, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(493), SimThingId(495)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, true, true, false] host_rows=[SlotIndex(1), SlotIndex(2)] births=[SimThingId(506), SimThingId(508)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, true, true, false] host_rows=[SlotIndex(2), SlotIndex(1)] births=[SimThingId(519), SimThingId(521)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, false, false, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(532), SimThingId(534)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, false, false, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(545), SimThingId(547)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, true, false, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(558), SimThingId(560)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, true, false, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(571), SimThingId(573)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, false, true, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(584), SimThingId(586)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, false, true, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(597), SimThingId(599)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[false, true, true, true] host_rows=[SlotIndex(5), SlotIndex(6)] births=[SimThingId(610), SimThingId(612)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
terminal independent order project=1 order=[true, true, true, true] host_rows=[SlotIndex(6), SlotIndex(5)] births=[SimThingId(623), SimThingId(625)] source_generations=[1, 5] trace=[([[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]], 3, 0), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 5, 1), ([[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]], 3, 1), ([[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]], 3, 1), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 3, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2), ([[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]], 5, 2)]
ok
test ordinary_funded_placement_and_restart_matrix ... placement/restart project=0 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(630), grantee: SimThingId(637), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(16227562193434212560), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(629), grantee: SimThingId(637), market_grant_key: 16227562193434212560 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=false g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh completion ordinal=1 id=SimThingId(639) component=SimThingId(640) overlay=OverlayId(326) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(629), grantee: SimThingId(639), market_grant_key: 8826717574082237810 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(7) }
placement/restart project=0 reversed=false g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh observation id=SimThingId(639) slot=SlotIndex(3) value=18
fresh observation id=SimThingId(640) slot=SlotIndex(4) value=18
placement/restart project=0 reversed=false g=9 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=false g=10 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[4.5, 1.5, 2.0], [1.5, 4.5, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=false g=11 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(3)
placement/restart project=0 reversed=false g=12 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh completion ordinal=2 id=SimThingId(641) component=SimThingId(642) overlay=OverlayId(327) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(629), grantee: SimThingId(641), market_grant_key: 120993717953264060 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(12) }
placement/restart project=0 reversed=false g=13 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(641) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(642) slot=SlotIndex(4) value=19
placement/restart project=0 reversed=false g=14 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(641) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(642) slot=SlotIndex(4) value=19
placement/restart project=0 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=0 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(646), grantee: SimThingId(653), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(13158955658966968492), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(645), grantee: SimThingId(653), market_grant_key: 13158955658966968492 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=true g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh completion ordinal=1 id=SimThingId(655) component=SimThingId(656) overlay=OverlayId(339) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(645), grantee: SimThingId(655), market_grant_key: 2300410821436212654 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(7) }
placement/restart project=0 reversed=true g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh observation id=SimThingId(655) slot=SlotIndex(3) value=18
fresh observation id=SimThingId(656) slot=SlotIndex(4) value=18
placement/restart project=0 reversed=true g=9 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=true g=10 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[4.5, 1.5, 2.0], [1.5, 4.5, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=0 reversed=true g=11 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(3)
placement/restart project=0 reversed=true g=12 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh completion ordinal=2 id=SimThingId(657) component=SimThingId(658) overlay=OverlayId(340) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(645), grantee: SimThingId(657), market_grant_key: 16796682242963382552 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(12) }
placement/restart project=0 reversed=true g=13 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(657) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(658) slot=SlotIndex(4) value=19
placement/restart project=0 reversed=true g=14 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(657) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(658) slot=SlotIndex(4) value=19
placement/restart project=1 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=1 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(663), grantee: SimThingId(669), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(7521633002871632888), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(661), grantee: SimThingId(669), market_grant_key: 7521633002871632888 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=false g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh completion ordinal=1 id=SimThingId(671) component=SimThingId(672) overlay=OverlayId(352) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(661), grantee: SimThingId(671), market_grant_key: 16978823939621648338 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(7) }
placement/restart project=1 reversed=false g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh observation id=SimThingId(671) slot=SlotIndex(3) value=18
fresh observation id=SimThingId(672) slot=SlotIndex(4) value=18
placement/restart project=1 reversed=false g=9 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=false g=10 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[4.5, 1.5, 2.0], [1.5, 4.5, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=false g=11 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(3)
placement/restart project=1 reversed=false g=12 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh completion ordinal=2 id=SimThingId(673) component=SimThingId(674) overlay=OverlayId(353) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(661), grantee: SimThingId(673), market_grant_key: 8034990223757269460 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(12) }
placement/restart project=1 reversed=false g=13 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(673) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(674) slot=SlotIndex(4) value=19
placement/restart project=1 reversed=false g=14 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(673) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(674) slot=SlotIndex(4) value=19
placement/restart project=1 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=1 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(679), grantee: SimThingId(685), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(5916601741025369666), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(677), grantee: SimThingId(685), market_grant_key: 5916601741025369666 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=true g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh completion ordinal=1 id=SimThingId(687) component=SimThingId(688) overlay=OverlayId(365) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(677), grantee: SimThingId(687), market_grant_key: 10205575438325575076 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(7) }
placement/restart project=1 reversed=true g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=3 live=5 present=[false, true, false] action_generation=Some(2)
fresh observation id=SimThingId(687) slot=SlotIndex(3) value=18
fresh observation id=SimThingId(688) slot=SlotIndex(4) value=18
placement/restart project=1 reversed=true g=9 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=true g=10 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[4.5, 1.5, 2.0], [1.5, 4.5, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(2)
placement/restart project=1 reversed=true g=11 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(3)
placement/restart project=1 reversed=true g=12 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh completion ordinal=2 id=SimThingId(689) component=SimThingId(690) overlay=OverlayId(366) placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(677), grantee: SimThingId(689), market_grant_key: 1969627884956512926 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(12) }
placement/restart project=1 reversed=true g=13 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(689) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(690) slot=SlotIndex(4) value=19
placement/restart project=1 reversed=true g=14 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.5, 0.5, 3.0], [0.5, 3.5, 3.0]] capacity=3 live=5 present=[false, false, true] action_generation=Some(3)
fresh observation id=SimThingId(689) slot=SlotIndex(3) value=19
fresh observation id=SimThingId(690) slot=SlotIndex(4) value=19
no-refusal control project=0 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=0 reversed=false g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=0 reversed=false g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=0 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=0 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=0 reversed=true g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=0 reversed=true g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=1 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=false g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=false g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=1 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=6 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=true g=7 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
no-refusal control project=1 reversed=true g=8 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(2)
ok
test ordinary_joint_lifecycle_matrix ... LIFECYCLE CASE START: joint material accounting
joint pulse=[1.0, 1.0] children=false arenas=false hosts=["p", "q"] g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
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
LIFECYCLE CASE PASS: joint material accounting
LIFECYCLE CASE START: partial cancellation and restart
cancel project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
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
LIFECYCLE CASE PASS: partial cancellation and restart
LIFECYCLE CASE START: external AddChild isolation control
external AddChild control g=3: Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
external AddChild control g=4: Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true })
LIFECYCLE CASE PASS: external AddChild isolation control
LIFECYCLE CASE START: funded structural boundary required success
birth pulse=[1.0, 1.0] reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]] born=false component=false placed=false action_generation=Some(0)
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
birth pulse=[4.0, 4.0] reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] born=false component=false placed=false action_generation=Some(0)
birth pulse=[4.0, 4.0] reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=false component=false placed=false action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
birth pulse=[4.0, 4.0] reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] born=true component=true placed=true action_generation=Some(1)
LIFECYCLE CASE PASS: funded structural boundary required success
ok
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

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 174.06s
```

## crossing

Raw log SHA-256: dace62760d7daa528c50ee9041106a93eb60b13bad993a376dd7ca6891785fa3.

```text
running 4 tests
test forbidden_overlay_and_state_plane_shapes_are_rejected_by_the_real_door ... ok
test frozen_threshold_definition_is_authority_not_registration_index ... ok
test one_real_gpu_door_executes_all_three_consequence_arms ... ok
test sparse_source_generations_execute_once_each_without_clock_pumping ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.34s
```
