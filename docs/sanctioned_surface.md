# Sanctioned Surface Digest

> GENERATED FILE. Do not hand-edit. Regenerate with `bash scripts/ci/gen_digest.sh`.
> Source of truth: `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`; optional track mode reads only the explicit track doc sibling addendum.

This digest is a derived context artifact for low-context agents. If it disagrees with CI data, the CI data wins and this file or generator is wrong.

## Source Manifest

| source | data rows | sha256 |
| --- | --- | --- |
| scripts/ci/allow/sealed_producers.txt | 15 | 331dca91173adeaa5e78deb6e8a1999b44259aad46c0b0dd6c83b1a836bf46cf |
| scripts/ci/allow/inert_buffer_handles.txt | 2 | 9e2069fa5730f17cacde1c671ebc17beb59f50738d2dcb914bceae13f9b8b3a4 |
| scripts/ci/allow/kernel_surface.txt | 227 | 1a783371527a949bb870d724903143ad98e1c7c35818613c7dcc79e8c8746a5a |
| scripts/ci/allow/sealed_types.txt | 12 | 0465cdb467587a9fd44051ba281121b8bf5d718ac7e0ede1998856c6ded97a65 |
| scripts/ci/scans.tsv | 16 | 7a4176379668e9694346fbae63f630c74dc78828b55e5cee4ca30290e16e0490 |

## Sanctioned Sealed Producers

| symbol | door-class | rationale | promotion-blocker | source |
| --- | --- | --- | --- | --- |
| cpu_oracle_threshold_events | cpu_oracle | CPU-oracle twin for threshold events; parity-only path | retire when CPU oracle is type-quarantined to in-crate parity harness | sealed_producers.txt |
| cpu_oracle_emission_records | cpu_oracle | CPU-oracle twin for emission records; parity-only path | retire when CPU oracle is type-quarantined to in-crate parity harness | sealed_producers.txt |
| execute_ops_cpu_with_emissions | cpu_oracle | CPU-oracle batch path returning emission records; parity-only | retire when CPU oracle batch path is in-crate only | sealed_producers.txt |
| execute_threshold_ops_cpu | cpu_oracle | CPU-oracle batch path returning threshold emissions; parity-only | retire when CPU oracle batch path is in-crate only | sealed_producers.txt |
| apply_candidate_f_exact_magnitude | apply | Sanctioned exact-magnitude write door for Candidate F | retire when exact write is an in-crate AccumulatorOp type boundary only | sealed_producers.txt |
| read_event_candidates | read | Read sealed ThresholdEvent candidates from WorldGpuState | retire when threshold events are observed only via typed read view | sealed_producers.txt |
| read_records | read | Read sealed EmissionRecord values via EmissionRecordReadback | retire when emission readback is in-crate only | sealed_producers.txt |
| read_records_capped | read | Capped read of sealed EmissionRecord via readback helper | retire when emission readback is in-crate only | sealed_producers.txt |
| read_threshold_emissions | read | Read sealed ThresholdEmission via ThresholdEmissionReadback | retire when threshold emission readback is in-crate only | sealed_producers.txt |
| read_threshold_events | read | Read sealed ThresholdEvent via ThresholdEmissionReadback | retire when threshold event readback is in-crate only | sealed_producers.txt |
| read_events | read | Read threshold event candidates via ThresholdEventCandidatesReadback | retire when candidate readback is in-crate only | sealed_producers.txt |
| readback_threshold_emissions | read | Session readback door for sealed ThresholdEmission records | retire when threshold emission readback is in-crate only | sealed_producers.txt |
| readback_threshold_events | read | Session readback door for sealed ThresholdEvent records | retire when threshold event readback is in-crate only | sealed_producers.txt |
| readback_emissions | read | Session readback door for sealed EmissionRecord slice | retire when emission readback is in-crate only | sealed_producers.txt |
| readback_emissions_capped | read | Capped session readback door for sealed EmissionRecord records | retire when emission readback is in-crate only | sealed_producers.txt |

## Inert Buffer Handles

| symbol | door-class | rationale | promotion-blocker | source |
| --- | --- | --- | --- | --- |
| max_candidate_f_magnitude_bits | inert-util | Caller-owned ephemeral GPU buffers for exact-magnitude oracle probe | retire when oracle probe is in-crate only and not exported | inert_buffer_handles.txt |
| IndexedScatterOp::dispatch | inert-util | Indexed scatter dispatch with caller-owned src/dst buffers | retire when scatter buffers are session-sealed and dispatch is in-crate only | inert_buffer_handles.txt |

## Kernel Surface

| symbol/signature | door-class | rationale | promotion-blocker | source |
| --- | --- | --- | --- | --- |
| accumulator_op | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorInputGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorInputListTable | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpSession | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpSessionError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorPipelineSessions | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AdmittedEvalEmlCombine | sealed-export | OC-K-EML-OPCODE-GATE-0 admitted combine token | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AdmittedEvalEmlOpcode | sealed-export | OC-K-EML-OPCODE-GATE-0 admitted opcode token | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AO_WGSL0_ENTRY_POINT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ao_wgsl0_fast_path_compatible | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0Compatibility | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0FallbackReason | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0PlanShape | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ApproximateDecisionDiagnostic | authority-export | OC-K-DECISION-INGRESS-0 approximate decision diagnostic only | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ApproximateDiagnostic | authority-export | OC-K-EXACT-GATE-0 diagnostic-only magnitude; cannot mint ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| BoundaryEmissionToken | sealed-export | OC-K-DECISION-INGRESS-0 boundary token for commitment mint | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_column_rule_descriptors | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_column_rules | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_governed_pairs | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_intensity_eml_entries | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_overlay_deltas | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| build_topology | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| candidate_f_magnitude | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CandidateFMagnitudeError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CandidateFMagnitudeReport | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CandidateFMagnitudeRequest | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CLAMP_BOUNDED | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CLAMP_FLOORED | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CLAMP_UNBOUNDED | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| classify_ao_wgsl0_plan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ColumnRuleDescriptor | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| combine_in_closed_vocabulary | authority-export | OC-K-EML-OPCODE-GATE-0 closed combine vocabulary predicate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CombineRegistrationRequest | sealed-export | OC-K-EML-OPCODE-GATE-0 combine registration request class | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CommitmentRegistration | sealed-export | OC-K-EXACT-GATE-0 commitment registration requiring ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| conjunctive_recipe_registration_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| conjunctive_recipe_registrations_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| context | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_oracle | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_oracle_emission_records | authority-export | CPU-oracle authority surface; xref sealed_producers:cpu_oracle_emission_records | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_oracle_threshold_events | authority-export | CPU-oracle authority surface; xref sealed_producers:cpu_oracle_threshold_events | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_reduce_oracle | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_reduce_oracle_call_count | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_scatter_indexed | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CpuDiagnosticDecision | authority-export | OC-K-DECISION-INGRESS-0 CPU diagnostic decision only | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CpuOracleError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CpuOracleParityProof | sealed-export | OC-K-EML-OPCODE-GATE-0 Tier-2 bit-exact CPU-oracle parity proof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| debug_readback_allowed | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DebugReadbackGuard | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| decision_ingress | authority-export | OC-K-DECISION-INGRESS-0 sealed decision ingress module | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DecisionIngressError | authority-export | OC-K-DECISION-INGRESS-0 decision ingress error type | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DEFAULT_EMISSION_CAPACITY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DEFAULT_EML_NODE_CAPACITY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DEFAULT_EML_TREE_CAPACITY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DEFAULT_INPUT_LIST_CAPACITY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DEFAULT_THRESHOLD_EMISSION_CAPACITY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DIR_DOWNWARD | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DIR_EITHER | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| DIR_UPWARD | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| discrete_transfer_registration_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| discrete_transfer_registrations_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emission_accumulator | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emission_oracle | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emission_plan_signature_fields | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionFormula | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionOpPlanSignature | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionOracleError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionOracleFormula | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionOracleRegistration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionPlanError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRecord | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRecordGpu | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRecordReadback | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRegistration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionSyncError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionToken | sealed-export | OC-K-DECISION-INGRESS-0 emission token from sealed ThresholdEmission | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emit_on_threshold_registrations_to_gpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emit_on_threshold_registrations_to_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| eml_opcode_gate | authority-export | OC-K-EML-OPCODE-GATE-0 module surface; closed EvalEML opcode/combine gate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlGpuProgramTable | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlTreeRangeGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlUploadError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| encode_column_rules | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| encode_emission_plan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| encode_rule | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| encode_transfer_plan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EncodeError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| eval_eml_cpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EvalEmlCombine | sealed-export | OC-K-EML-OPCODE-GATE-0 closed EvalEML combine newtype | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EvalEmlOpcode | sealed-export | OC-K-EML-OPCODE-GATE-0 closed EvalEML opcode newtype | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EvalEmlVocabulary | sealed-export | OC-K-EML-OPCODE-GATE-0 closed vocabulary snapshot | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| exact_mag2_bits_q16 | authority-export | OC-K-EXACT-GATE-0 CPU twin Q16 mag2 for Candidate F parity | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| exact_magnitude_gate | authority-export | OC-K-EXACT-GATE-0 exact magnitude proof token module | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ExactMagnitudeProof | sealed-export | OC-K-EXACT-GATE-0 Candidate F magnitude proof token; private bits | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ExactnessClass | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| execute_intent_deltas_cpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| execute_ops_cpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| execute_ops_cpu_with_emissions | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| execute_threshold_ops_cpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| FORMULA_KIND_CONSTANT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| FORMULA_KIND_EVAL_EML | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| FORMULA_KIND_IDENTITY_FLOOR | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GenericPrimitiveRegistration | sealed-export | OC-K-EML-OPCODE-GATE-0 Tier-2 generic primitive registration request | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| governed_pairs_for_property | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GovernedIntegrationPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GovernedPair | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| gpu_readback | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GpuContext | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GpuInitError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| GradientPairGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| indexed_scatter | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IndexedScatterError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IndexedScatterOp | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| InputListRange | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| InputListUploadError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| intensity_accumulator | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IntensityEmlEntry | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IntensityEmlOpPlanSignature | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IntensityEmlPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| IntentDelta | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| KernelReadbackError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| LegacyOracleFamily | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| max_candidate_f_magnitude_bits | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| mint_exact_magnitude_proof_candidate_f | authority-export | OC-K-EXACT-GATE-0 GPU Candidate F mint of ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| mint_exact_magnitude_proof_candidate_f_cpu | authority-export | OC-K-EXACT-GATE-0 CPU Candidate F mint of ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_CONSTANT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_MAX_EMIT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_TREE_ID | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OP_ADD | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OP_MULTIPLY | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OP_SET | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| opcode_in_closed_vocabulary | authority-export | OC-K-EML-OPCODE-GATE-0 closed opcode vocabulary predicate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OpcodeGateError | authority-export | OC-K-EML-OPCODE-GATE-0 opcode/combine gate error | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OpcodeRegistrationGate | sealed-export | OC-K-EML-OPCODE-GATE-0 EvalEML opcode/combine admission gate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OpcodeRegistrationRequest | sealed-export | OC-K-EML-OPCODE-GATE-0 opcode registration request class | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OperationFamily | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OpSetHandle | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| overlay_orderband | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| overlay_prep | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OverlayCompileCache | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OverlayDelta | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| OverlayOrderBandPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PackedAccumulatorUpload | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PackedIntentUpload | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PackedThresholdUpload | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| participation | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| passes | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| Pipelines | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PlacedParticipant | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PlacedParticipantValidationError | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_emission_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_governed_integration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_governed_integration_at_band | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_intensity_eml_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_overlay_orderband | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_reduction_orderband | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_transfer_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| plan_velocity_integration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| PlannerError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| project_tree_to_values | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| projection | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| readback | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| reduction | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| reduction_orderband | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| reduction_soft_band_for_depth_bucket | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ReductionOrderBandPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ReductionPlanError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| register_intensity_eml_formulas | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| registration | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| reset_cpu_reduce_oracle_call_count | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| resolved | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ResolvedGpuBuffers | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ResolvedWriteAuthority | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_FIRST | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_MAX | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_MEAN | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_MIN | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_SUM | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| RULE_WEIGHTED_MEAN | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ScatterEntry | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| scoped_debug_readback_allowed | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| sealed | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SemanticOpcodeRegistration | sealed-export | OC-K-EML-OPCODE-GATE-0 semantic opcode request (hard-reject) | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| set_debug_readback_allowed | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| slot | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotAllocator | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotAllocError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotDeltaRange | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotSummary | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SoftStepPolicyConditional | authority-export | OC-K-EML-OPCODE-GATE-0 SoftStep branchless policy conditional gadget | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| sqrt_cr_f_bits | authority-export | OC-K-EXACT-GATE-0 Candidate F CR-F sqrt bits CPU twin | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| StructuralCommitment | sealed-export | OC-K-DECISION-INGRESS-0 sealed structural commitment effect | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| StructuralGridPlacement | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| summaries_from_values | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| THRESH_BUF_OUTPUT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| THRESH_BUF_VALUES | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| threshold_registrations_to_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdCrossingToken | sealed-export | OC-K-DECISION-INGRESS-0 threshold crossing token from sealed ThresholdEvent | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEmission | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEmissionGpu | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEmissionReadback | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEvent | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEventCandidatesReadback | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdEventGpu | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ThresholdRegistration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| Topology | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TopologyState | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| transfer_accumulator | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferInputRef | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferOpPlanSignature | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferPlanError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferRegistration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| TransferSyncError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| validate_and_mint_placed_participants_by_location_id | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| validate_intent_deltas_no_duplicate_cells | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| validate_location_ids_have_structural_placements | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| validate_scatter_entries | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| velocity_accumulator | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| VelocityAccumulatorPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WEIGHT_COL_NONE | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WORKGROUP_SIZE | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| world_state | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldAccumulatorRuntime | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldGpuState | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldSummaryRuntime | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |

## Sealed Types

| sealed type | source |
| --- | --- |
| ThresholdEvent | sealed_types.txt |
| ThresholdEventGpu | sealed_types.txt |
| ThresholdEventCandidatesReadback | sealed_types.txt |
| EmissionRecord | sealed_types.txt |
| EmissionRecordGpu | sealed_types.txt |
| EmissionRecordReadback | sealed_types.txt |
| ThresholdEmission | sealed_types.txt |
| ThresholdEmissionGpu | sealed_types.txt |
| ThresholdEmissionReadback | sealed_types.txt |
| PlacedParticipant | sealed_types.txt |
| ResolvedWriteAuthority | sealed_types.txt |
| CandidateFMagnitudeReport | sealed_types.txt |

## Forbidden / Screened Patterns

| scan-id | reliability | why | target | pattern/source | exclude | promotion-blocker | source |
| --- | --- | --- | --- | --- | --- | --- | --- |
| B3-BUFFER-ESCAPE | RELIABLE | design §5 B3 buffer escape | crates/simthing-kernel/src/** | pub fn [a-z_]+\\(&self\\) *-> *&(wgpu::)?Buffer\|^\\s*pub [a-z_]+ *: *Buffer\|-> *BindingResource | pub\\(crate\\);compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when buffer accessors are crate-private type boundary only | scans.tsv |
| FORGE-MINTERS | RELIABLE | design §5 forge minters | crates/simthing-kernel/src/** | pub fn (from_boundary_delivery\|for_kernel_readback\|for_boundary_install)\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when sealed-producer allowlist scan (CI-A-ALLOWLIST-SCANS-0) subsumes explicit forge names | scans.tsv |
| UNSAFE-FN | RELIABLE | design §5 unsafe fn | crates/simthing-{kernel,sim}/src/** | \\bunsafe fn\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when unsafe is unrepresentable at kernel/sim type boundary | scans.tsv |
| UNSAFE-ALLOW-ATTR | RELIABLE | design §5 allow unsafe attr | crates/simthing-{kernel,sim}/src/lib.rs | #!\\[allow\\(unsafe_code\\)\\] | (none) | retire when crate attributes are admission-typed not prose | scans.tsv |
| UNSAFE-FORBID-ATTR | RELIABLE | design §5 forbid unsafe attr | crates/simthing-{kernel,sim}/src/lib.rs | @REQUIRE:#!\\[forbid\\(unsafe_code\\)\\] | (none) | retire when semantic-free crate template enforces forbid at type boundary | scans.tsv |
| DENY-TOML-STUB | RELIABLE | design §0.6.6 deny.toml stub | deny.toml | . | (none) | retire when dependency policy is compile-time typed not file-shaped | scans.tsv |
| COLUMN-INDEX-MINT | HEURISTIC | design §5 ColumnIndex::new residual tripwire (OC-K-COLUMN-ROLE-0) | crates/**/src/** | ColumnIndex::new | column_index\\.rs;registry\\.rs;.*accumulator_op;cpu_oracle\\.rs;runtime_0080.;dress_rehearsal.;arena_allocation_plan\\.rs;resource_economy_compile\\.rs;first_slice_mapping_runtime\\.rs;gated_rates\\.rs;owner_silo_accumulator.;structural_link_accumulator.;gpu_measure_0080.;transfer_accumulator\\.rs;emission_accumulator\\.rs;intensity_accumulator\\.rs;region_field_admission\\.rs;accumulator_op_builder\\.rs;compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when ColumnIndex::new is admission-gated (pub(crate) + layout-derived constructors only) — OC-K2.1 backlog | scans.tsv |
| SIM-KIND-READ | HEURISTIC | design §5 sim .kind read | crates/simthing-sim/src/** | match .*\\.kind\|\\.kind\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*//;delta_log;sim_runtime_tree;kind_production_audit;^\\s*assert_\|^\\s*#\\[test\\] | promote when runtime tree view is kind-free at type boundary | scans.tsv |
| SEMANTIC-WORDS | HEURISTIC | design §5 semantic words below spec | crates/simthing-{sim,kernel}/src/** | faction\|combat\|terran\|pirate\|diplomacy | compile_fail;^\\s*//!;^\\s*///;^\\s*//;^\\s*assert_\|^\\s*#\\[test\\]\|SimThingKind:: | promote when game semantics are spec-boundary typed only | scans.tsv |
| SPEC-STRING-CHANNEL | HEURISTIC | design §5 stringly channel identity | crates/simthing-spec/src/** | owner_ref *: *(Option<)?String\|resource_key *: *(Option<)?String | compile_fail;^\\s*//!;^\\s*///;^\\s*//;channel_key\\.rs | promote when channel identity is newtyped in spec admission | scans.tsv |
| ALLOW-SEALED-PRODUCERS | RELIABLE | design §5 sealed producer allowlist | crates/simthing-kernel/src/** | @ALLOWLIST:sealed-producers | (none) | retire when sealed producers are type-boundary admitted | scans.tsv |
| ALLOW-BUFFER-HANDLES | RELIABLE | design §5 buffer handle allowlist | crates/simthing-kernel/src/** | @ALLOWLIST:buffer-handles | (none) | retire when buffer handles are crate-private type boundary only | scans.tsv |
| ALLOW-KERNEL-SURFACE | RELIABLE | design §5 kernel surface allowlist | crates/simthing-kernel/src/lib.rs | @ALLOWLIST:kernel-surface | (none) | retire when kernel exports are type-boundary closed | scans.tsv |
| TEST-BUDGET | HEURISTIC | design §0.9.5 test admission budget | crates/** | @TEST_BUDGET | (none) | retire if test admission becomes a typed ledger gate end-to-end | scans.tsv |
| SPEC-LOWERER-KIND-READ | HEURISTIC | ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but closed-lowerer hits are higher suspicion because lowerers are constitutionally closed unless a DA-authorized amendment names them | crates/simthing-{spec,clausething}/src/** | match .*\\.kind\|\\.kind\\s*(==\|!=)\|match\\s+(?:&)?kind\\s*\\{[\\s\\S]*?SimThingKind:: | compile_fail;^\\s*//!;^\\s*///;^\\s*//;^\\s*assert_\|^\\s*#\\[test\\];planet_non_grid_child_kind_label;is_admitted_planet_non_grid_child;scenario_deferral_kind_label;planet_child_location_error_kind_label;simthing_kind_label;location_participant_kind_label;non_location_participant_kind_label | retire when spec-layer role resolution is role-keyed by SubFieldRole/column admission boundaries rather than SimThingKind branching | scans.tsv |
| GUARD-KABUKI-TRIPWIRE | HEURISTIC | handoff_template section H + ci_screening_surface section 4; HEURISTIC tripwire for bespoke source-scanning guards and test-side include_str source greps; HC-6: symbol with well-formed FRESH HORIZON-ENTRY(iso-date): consumer/ref is EXEMPT (dated+assessable; unmarked/stale stay FLAGGED; never bare-token forever-pass); HC-8 accepted evasion residue: PRIVATE fn source-scanner or var-bound include_str! evades the pub-fn-anchored arms (DA review is the backstop; regex intentionally NOT widened — would false-fire on legit parsers); legitimate cases route to INSPECT triage, never FAIL | crates/**/{src,tests}/**/*.rs | pub fn [A-Za-z0-9_]+\\([^)]*source:\\s*&str[\\s\\S]{0,1200}source\\.(contains\|find\|matches\|lines\|to_ascii_lowercase)\\(\|pub fn [A-Za-z0-9_]+\\([^)]*path:\\s*&Path[\\s\\S]{0,800}read_to_string\\(path\\)[\\s\\S]{0,800}\\.(contains\|find\|matches\|lines)\\(\|include_str!\\("../src/[^"\\n]*"\\)\\.(contains\|find\|matches\|lines)\\( | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when anti-kabuki source-scan guard review is admission-typed or no production/test guard uses source text as a proof surrogate | scans.tsv |
