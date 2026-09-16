# Construction Leaf A — cancellation raw evidence

[Interpretation, disposition and scope](rehearsal_lifecycle_construction_release_results.md).
All observations below are local GPU execution, not source-only proof.

## Identity

Base: 6dbab071be7afd3967c63663badcb9067e94ac22.
Unchanged-suite run at synchronization commit 6244bfc1d0ab5333447591230ea0af38c7173ac5.
New cancellation witness run on that commit plus the appended test/inventory row;
recipe file SHA-256: `bba04d8ff5eeec067c4dc981cc9ff73f381c9cba032b111ea877dbe629eabc7c`. Final committed-head rerun is recorded in PR/Board.
Command: `cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe funded_product_cancellation -- --nocapture --test-threads=1`.
Shared target: `C:/Users/mvorm/SimThing/target`. Exit 101.

## Unchanged prerequisite run before additions

### Unchanged ingress

```text
test construction_leaf_a_requires_qualified_ordinary_session ... adapter: AdapterInfo { name: "NVIDIA GeForce RTX 4080 Laptop GPU", vendor: 4318, device: 10144, device_type: DiscreteGpu, driver: "NVIDIA", driver_info: "595.79", backend: Vulkan }
required_fingerprint: 67ffbfdfe25a1d4e
observed_fingerprint: 67ffbfdfe25a1d4e
bundle: 754d2b205494c4e0
ordinary session admitted; construction discriminator may proceed
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.29s
```

### Unchanged discriminator

```text
test each_resource_policy_works_alone_but_joint_plan_must_keep_both ... single a: [[0.75, 0.25]]
test ordinary_resident_session_must_preserve_opposite_resource_policies ... ordinary reverse_children=false: [[0.75, 0.25], [0.25, 0.75]]; expected=[[0.75, 0.25], [0.25, 0.75]]
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.73s
```

### Unchanged wip

```text
test leaf_residual_must_settle_as_owned_balance_for_each_resource ... component ["a"] children_reversed=false arenas_reversed=false: [[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]; expected=[[[0.0, 0.0, 0.0], [0.75, 0.75, 0.75], [0.25, 0.25, 0.25]]]
test parent_surplus_integrates_through_existing_balance_door ... parent surplus control ordinary=false: [[[0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]]
test seeded_leaf_rate_integrates_once_in_one_ordinary_generation ... non-arena free-rate generation=1: [0.5, 0.5]
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.87s
```

### Unchanged recipe

```text
test ordinary_joint_lifecycle_matrix ... LIFECYCLE CASE START: joint material accounting
LIFECYCLE CASE PASS: joint material accounting
LIFECYCLE CASE START: partial cancellation and restart
LIFECYCLE CASE PASS: partial cancellation and restart
LIFECYCLE CASE START: external AddChild isolation control
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
test ordinary_recipe_preserves_incomplete_wip_and_consumes_funded_stock_once ... recipe host=p pulse=[1.0, 1.0] children_reversed=false arenas_reversed=false g=1: [[0.75, 0.25, 0.0], [0.25, 0.75, 0.0]]
test two_explicit_project_recipe_hosts_must_admit_in_one_ordinary_session ... joint hosts=["p", "q"] children=false arenas=false: materialized host slots=[1, 2]; ordinary=admitted
test unresolved_or_ambiguous_recipe_host_still_refuses_with_provenance ... negative ambiguous=false: economy host entity `p` is not in install_targets; span=Some(37)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 76.01s
```

## New cancellation witness — complete test output (compiler warnings omitted)

```text
running 1 test
test funded_product_cancellation_must_release_placement_and_continue ... cancel setup funded=false project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(4), market_grant_key: 15474471149597928219 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(4), market_grant_key: 15474471149597928219 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=false g=5: healthy
cancel continuation funded=false project=0 reversed=false g=6: healthy
cancel setup funded=false project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(8), grantee: SimThingId(11), market_grant_key: 11129623678524452045 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(8), grantee: SimThingId(11), market_grant_key: 11129623678524452045 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=true g=5: healthy
cancel continuation funded=false project=0 reversed=true g=6: healthy
cancel setup funded=false project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(15), grantee: SimThingId(18), market_grant_key: 10463037027549295077 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(15), grantee: SimThingId(18), market_grant_key: 10463037027549295077 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=false g=5: healthy
cancel continuation funded=false project=1 reversed=false g=6: healthy
cancel setup funded=false project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(22), grantee: SimThingId(25), market_grant_key: 6428043301543837737 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(22), grantee: SimThingId(25), market_grant_key: 6428043301543837737 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=true g=5: healthy
cancel continuation funded=false project=1 reversed=true g=6: healthy
cancel setup funded=true project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(29), grantee: SimThingId(32), market_grant_key: 5490804925521482069 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=false g=4 result=Err(ActionBandIngress(BindingTableStale)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(29), grantee: SimThingId(32), market_grant_key: 5490804925521482069 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel retry funded=true project=0 reversed=false attempt=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=0 reversed=false attempt=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=0 reversed=false attempt=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel setup funded=true project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(36), grantee: SimThingId(39), market_grant_key: 12713289762700408609 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=true g=4 result=Err(ActionBandIngress(BindingTableStale)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(36), grantee: SimThingId(39), market_grant_key: 12713289762700408609 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel retry funded=true project=0 reversed=true attempt=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=0 reversed=true attempt=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=0 reversed=true attempt=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel setup funded=true project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(43), grantee: SimThingId(46), market_grant_key: 18308302178867333649 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=false g=4 result=Err(ActionBandIngress(BindingTableStale)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(43), grantee: SimThingId(46), market_grant_key: 18308302178867333649 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel retry funded=true project=1 reversed=false attempt=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=1 reversed=false attempt=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=1 reversed=false attempt=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel setup funded=true project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(50), grantee: SimThingId(53), market_grant_key: 11231010971102938749 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=true g=4 result=Err(ActionBandIngress(BindingTableStale)) placement=Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(50), grantee: SimThingId(53), market_grant_key: 11231010971102938749 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel retry funded=true project=1 reversed=true attempt=0: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=1 reversed=true attempt=1: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")
cancel retry funded=true project=1 reversed=true attempt=2: ExecutionIdentity("generation GenerationStamp(4) is faulted after an unfinished economic authorization")

thread 'funded_product_cancellation_must_release_placement_and_continue' (36784) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_recipe.rs:1079:5:
funded cancellation/release required success: ["funded=false/project=0/reversed=false: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(4), market_grant_key: 15474471149597928219 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=false/project=0/reversed=true: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(8), grantee: SimThingId(11), market_grant_key: 11129623678524452045 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=false/project=1/reversed=false: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(15), grantee: SimThingId(18), market_grant_key: 10463037027549295077 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=false/project=1/reversed=true: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(22), grantee: SimThingId(25), market_grant_key: 6428043301543837737 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=true/project=0/reversed=false: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(29), grantee: SimThingId(32), market_grant_key: 5490804925521482069 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=true/project=0/reversed=false: canonical cancellation cannot finish: ActionBandIngress(BindingTableStale)", "funded=true/project=0/reversed=true: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(36), grantee: SimThingId(39), market_grant_key: 12713289762700408609 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=true/project=0/reversed=true: canonical cancellation cannot finish: ActionBandIngress(BindingTableStale)", "funded=true/project=1/reversed=false: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(43), grantee: SimThingId(46), market_grant_key: 18308302178867333649 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=true/project=1/reversed=false: canonical cancellation cannot finish: ActionBandIngress(BindingTableStale)", "funded=true/project=1/reversed=true: removed subtree retains committed placement Some(CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(50), grantee: SimThingId(53), market_grant_key: 11231010971102938749 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) })", "funded=true/project=1/reversed=true: canonical cancellation cannot finish: ActionBandIngress(BindingTableStale)"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    funded_product_cancellation_must_release_placement_and_continue

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 10.16s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`

```
