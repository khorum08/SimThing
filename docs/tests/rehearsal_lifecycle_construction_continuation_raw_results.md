# Construction Leaf A — separated crossing raw evidence

Authority: Board 5707765404. Base `308adb53ece34b725b1c2e164d3399d1fe2d64c4`.
Diagnostic execution with synchronized `fd502250` plus the exact appended test below.
Recipe source SHA-256: `883ac82f7f02956763f0ab16e785093dde73a04d38e2eb97e7238f4b2a8cab8d`.
Retained prefix SHA-256: `5d95ec0c54d0c6efe818fa9752a7c41ce2427a1ec4f9592e33bbc15002b14a9c`.
Final-head repetition and hosted receipts are in PR #2062 / Board return; these
logs remain the original diagnostic capture (8 required-success failures).

Reference: Windows / NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79;
rustc 1.95.0 / LLVM 22.1.2 / wgpu 22.1.0 / naga 22.1.0.
Target: `C:/Users/mvorm/SimThing-0088-construction-local-2068/target`.

```text
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe ordinary_funded_placement_and_restart_matrix -- --exact --nocapture --test-threads=1
running 1 test
test ordinary_funded_placement_and_restart_matrix ... placement/restart project=0 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(2), grantee: SimThingId(9), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(1651361584794299392), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(1), grantee: SimThingId(9), market_grant_key: 1651361584794299392 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=false g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart project=0 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=0 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(18), grantee: SimThingId(25), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(7690163033792438842), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(17), grantee: SimThingId(25), market_grant_key: 7690163033792438842 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=0 reversed=true g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart project=1 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=1 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(35), grantee: SimThingId(41), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(16149235034885463082), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(33), grantee: SimThingId(41), market_grant_key: 16149235034885463082 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=false g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart project=1 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(0)
placement/restart project=1 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=2 live=6 present=[false, false, false] action_generation=Some(1)
typed placement refusal: OrdinaryGrowthRefusal { candidate: OrdinaryGrowthCandidate { structural_parent: SimThingId(51), grantee: SimThingId(57), quantity: 2, origin: AddChild }, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4), market_grant_key: Some(13913329182249817922), reason: Placement(ResidencyPlacementRefusal { identity: ResidencyPlacementIdentity { granter: SimThingId(49), grantee: SimThingId(57), market_grant_key: 13913329182249817922 }, proposed: ResidencyExtent { start: 0, length: 8 }, reason: NoContiguousExtent { containing: ResidencyExtent { start: 0, length: 8 }, quantity: 2 }, retained_unmet_quantity: 2, attempted_generation: GenerationStamp(3), revalue_generation: GenerationStamp(4) }) }
placement/restart project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart project=1 reversed=true g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] capacity=5 live=3 present=[false, false, false] action_generation=Some(1)
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
placement/restart fault=ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 })) retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control project=0 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=0 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=false g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(1)
no-refusal control retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control project=0 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=0 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=0 reversed=true g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(1)
no-refusal control retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control project=1 reversed=false g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=1 reversed=false g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=false g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(1)
no-refusal control retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control project=1 reversed=true g=1 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.0, 1.0, 0.0], [1.0, 3.0, 0.0]] action_generation=Some(0)
no-refusal control project=1 reversed=true g=2 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=3 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=4 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[2.0, 0.0, 1.0], [0.0, 2.0, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=5 result=Ok(StepOnceOutcome { ticks_run: 1, boundaries_run: 1, boundary_reached: true }) stock=[[3.75, 1.25, 1.0], [1.25, 3.75, 1.0]] action_generation=Some(1)
no-refusal control project=1 reversed=true g=6 result=Err(ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))) stock=[[2.75, 0.25, 2.0], [0.25, 2.75, 2.0]] action_generation=Some(1)
no-refusal control retry=0: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=1: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")
no-refusal control retry=2: ExecutionIdentity("generation GenerationStamp(6) is faulted after an unfinished economic authorization")

thread 'ordinary_funded_placement_and_restart_matrix' (36188) panicked at crates\simthing-driver\tests\rehearsal_lifecycle_construction_recipe.rs:1680:5:
ordinary funded continuation cannot finish: ["placement/restart project=0 reversed=false g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "placement/restart project=0 reversed=true g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "placement/restart project=1 reversed=false g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "placement/restart project=1 reversed=true g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "no-refusal control project=0 reversed=false g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "no-refusal control project=0 reversed=true g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "no-refusal control project=1 reversed=false g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))", "no-refusal control project=1 reversed=true g=6: ActionBandIngress(Dispatch(CrossingGenerationMismatch { expected: 2, actual: 5 }))"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
FAILED

failures:

failures:
    ordinary_funded_placement_and_restart_matrix

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 11.83s

error: test failed, to rerun pass `-p simthing-driver --test rehearsal_lifecycle_construction_recipe`

```

Original prerequisite summaries (in execution order):

```text
ingress: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.81s
required_fingerprint: ec5a2a30afaee795
observed_fingerprint: ec5a2a30afaee795
bundle: 9c2091194366b23b
discriminator: test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.40s
wip: test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.06s
recipe: test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 80.84s
cancellation: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 11.67s
reuse: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.00s
multi: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 4.83s
structural: test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s
```

Boundary acceptance law negatives:

```text
running 9 tests
test session::bound_lifecycle_acceptance_proofs::additive_row_from_accepted_growth_is_lawful_and_without_growth_is_stale ... ok
test session::bound_lifecycle_acceptance_proofs::bound_identity_remap_or_removal_refuses_with_its_own_type_even_during_growth ... ok
test session::bound_lifecycle_acceptance_proofs::bound_identity_removal_stays_typed_even_with_removal_evidence ... ok
test session::bound_lifecycle_acceptance_proofs::canonical_removal_evidence_admits_unbound_shrink_and_its_absence_is_stale ... ok
test session::bound_lifecycle_acceptance_proofs::identical_shape_is_always_lawful ... ok
test session::bound_lifecycle_acceptance_proofs::mixed_add_remove_needs_both_evidence_classes ... ok
test session::bound_lifecycle_acceptance_proofs::registry_and_dimension_fences_refuse_typed_even_with_growth_evidence ... ok
test session::bound_lifecycle_acceptance_proofs::surviving_remap_stays_stale_even_with_removal_evidence_present ... ok
test session::bound_lifecycle_acceptance_proofs::unbound_preexisting_row_change_is_stale_even_during_growth ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.00s


```
