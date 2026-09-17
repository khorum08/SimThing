# Construction Leaf A — interim local multi-remove raw evidence

[Interpretation and limits](rehearsal_lifecycle_construction_multi_remove_results.md).
Local composition 93b2e1efa87deca76c37aa8b03f8f7d212ab6316 (Leaf-A 1cd598bd + #2068 8a4165cb).
Separate target/worktree; no push or final semantic verdict. New recipe file SHA-256:
`5d95ec0c54d0c6efe818fa9752a7c41ce2427a1ec4f9592e33bbc15002b14a9c`.
The committed-head rerun is recorded in the interim Board return.

Common command prefix:
`cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe`.
Each named filter below is followed by `-- --nocapture --test-threads=1`.

## funded_product_cancellation — unchanged, GREEN

```text
running 1 test
test funded_product_cancellation_must_release_placement_and_continue ... cancel setup funded=false project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(4), market_grant_key: 15474471149597928219 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=false g=5: healthy
cancel continuation funded=false project=0 reversed=false g=6: healthy
cancel setup funded=false project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(8), grantee: SimThingId(11), market_grant_key: 11129623678524452045 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=0 reversed=true g=5: healthy
cancel continuation funded=false project=0 reversed=true g=6: healthy
cancel setup funded=false project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(15), grantee: SimThingId(18), market_grant_key: 10463037027549295077 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=false g=5: healthy
cancel continuation funded=false project=1 reversed=false g=6: healthy
cancel setup funded=false project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(22), grantee: SimThingId(25), market_grant_key: 6428043301543837737 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=false project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=false project=1 reversed=true g=5: healthy
cancel continuation funded=false project=1 reversed=true g=6: healthy
cancel setup funded=true project=0 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(29), grantee: SimThingId(32), market_grant_key: 5490804925521482069 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=0 reversed=false g=5: healthy
cancel continuation funded=true project=0 reversed=false g=6: healthy
cancel setup funded=true project=0 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(36), grantee: SimThingId(39), market_grant_key: 12713289762700408609 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=0 reversed=true g=5: healthy
cancel continuation funded=true project=0 reversed=true g=6: healthy
cancel setup funded=true project=1 reversed=false g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(43), grantee: SimThingId(46), market_grant_key: 18308302178867333649 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=1 reversed=false g=5: healthy
cancel continuation funded=true project=1 reversed=false g=6: healthy
cancel setup funded=true project=1 reversed=true g=3 placement=CommittedResidencyPlacement { identity: ResidencyPlacementIdentity { granter: SimThingId(50), grantee: SimThingId(53), market_grant_key: 11231010971102938749 }, extent: ResidencyExtent { start: 3, length: 2 }, quantity: 2, committed_generation: GenerationStamp(3) } slots=SlotIndex(3)/SlotIndex(4)
cancel funded=true project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) placement=None stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] live=3
cancel continuation funded=true project=1 reversed=true g=5: healthy
cancel continuation funded=true project=1 reversed=true g=6: healthy
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 11.20s


```

## ordinary_funded_placement — exploratory, INVALIDATED by earlier wrong identity removal; no clean placement proof

```text
running 1 test
test ordinary_funded_placement_and_restart_matrix ... placement/restart project=0 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false]
placement/restart project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false]
placement/restart project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false]
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(2), grantee: SimThingId(9), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(1651361584794299392), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(9), market_grant_key: 1651361584794299392 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }

thread 'ordinary_funded_placement_and_restart_matrix' (32932) panicked at crates\simthing-sim\src\tree_index.rs:50:26:
removal index (is 4) should be < len (is 4)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    ordinary_funded_placement_and_restart_matrix

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 1.53s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`

```

## canonical_multi_remove — required-success RED

```text
running 1 test
test canonical_multi_remove_must_preserve_requested_identities ... multi-remove indices=[0, 2] requested=[SimThingId(4), SimThingId(6)] actual_removed=[SimThingId(4), SimThingId(7)]
multi-remove indices=[0, 2]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }
multi-remove indices=[2, 0] requested=[SimThingId(16), SimThingId(14)] actual_removed=[SimThingId(14), SimThingId(16)]
multi-remove indices=[2, 0]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }

thread 'canonical_multi_remove_must_preserve_requested_identities' (38272) panicked at crates\simthing-sim\src\tree_index.rs:50:26:
removal index (is 6) should be < len (is 6)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
multi-remove indices=[3, 4] requested=[SimThingId(27), SimThingId(28)] actual_removed=[SimThingId(27)]
multi-remove indices=[3, 4]: panicked: removal index (is 6) should be < len (is 6)
multi-remove retry indices=[3, 4] attempt=0: ExecutionIdentity("generation GenerationStamp(1) is faulted after an unfinished economic authorization")
multi-remove retry indices=[3, 4] attempt=1: ExecutionIdentity("generation GenerationStamp(1) is faulted after an unfinished economic authorization")
multi-remove retry indices=[3, 4] attempt=2: ExecutionIdentity("generation GenerationStamp(1) is faulted after an unfinished economic authorization")
multi-remove indices=[4, 3] requested=[SimThingId(38), SimThingId(37)] actual_removed=[SimThingId(37), SimThingId(38)]
multi-remove indices=[4, 3]: completed StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }

thread 'canonical_multi_remove_must_preserve_requested_identities' (38272) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_recipe.rs:1194:5:
canonical identity-based cancellation required success: ["indices=[0, 2]: wrong identity disposition id=SimThingId(6), requested=[SimThingId(4), SimThingId(6)], actual_removed=[SimThingId(4), SimThingId(7)]", "indices=[0, 2]: wrong identity disposition id=SimThingId(7), requested=[SimThingId(4), SimThingId(6)], actual_removed=[SimThingId(4), SimThingId(7)]", "indices=[3, 4]: wrong identity disposition id=SimThingId(28), requested=[SimThingId(27), SimThingId(28)], actual_removed=[SimThingId(27)]", "indices=[3, 4]: panicked: removal index (is 6) should be < len (is 6)"]
FAILED

failures:

failures:
    canonical_multi_remove_must_preserve_requested_identities

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.68s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`

```

