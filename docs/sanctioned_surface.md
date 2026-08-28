# Sanctioned Surface Digest

> GENERATED FILE. Do not hand-edit. Regenerate with `bash scripts/ci/gen_digest.sh`.
> Source of truth: `scripts/ci/allow/*.txt` and `scripts/ci/scans.tsv`; optional track mode reads only the explicit track doc sibling addendum.

This digest is a derived context artifact for low-context agents. If it disagrees with CI data, the CI data wins and this file or generator is wrong.

## Source Manifest

| source | data rows | sha256 |
| --- | --- | --- |
| scripts/ci/allow/sealed_producers.txt | 25 | 6923796f3bacd3b27b29f778d5276c3f82a3a67c3f010c30f10b78de8a1866f6 |
| scripts/ci/allow/inert_buffer_handles.txt | 2 | 9e2069fa5730f17cacde1c671ebc17beb59f50738d2dcb914bceae13f9b8b3a4 |
| scripts/ci/allow/kernel_surface.txt | 303 | d6bc11a03b392def0fa6161e22bf010ab4024b22d90754e3db9bcdd427c0ed12 |
| scripts/ci/allow/sealed_types.txt | 19 | 9d427e3b41a586b06f24ca02cc45f78934237d2229b0c38d46e2a8a6ef5016be |
| scripts/ci/allow/contention_mechanisms.txt | 9 | 1f9e930b44197d4254f46ea96328f814d9ba0f2dad06881c454aebc01c01d44b |
| scripts/ci/scans.tsv | 24 | 36bf78ae4a81a9c49b570051a0f53cb10aa88ab5cf57faa0a05d83abfd068e79 |
| scripts/ci/constitutional_surfaces.tsv | 20 | 3309dc5da10e7a45f8fc3265c1d35bede8565c6440acfb6a65ed16cc80f30e3b |
| scripts/ci/constitutional_surface_check.sh | 512 | bada00e1929975c3d206a3cd1d62989f6998c9bd0c7d096c354dfa7684069d69 |

## Sanctioned Sealed Producers

| symbol | door-class | rationale | promotion-blocker | source |
| --- | --- | --- | --- | --- |
| apply_field_sweep_registration | apply | FIELD-SWEEP-N4-PARITY-0 sole admission door minting proof-present FieldSweepRegistration | retire when registration admission is an in-crate type boundary only | sealed_producers.txt |
| apply_canonical_order_proof | apply | FIELD-SWEEP-N4-PARITY-0 adjacency-bound canonical linear neighbor-order proof | retire when canonical order is intrinsic in the admitted adjacency type | sealed_producers.txt |
| apply_conductance_certificate | apply | FIELD-ADJACENCY-GENERATORS-0 adjacency-bound per-node chi times weighted-degree admission proof | retire when conductance stability is intrinsic in the admitted field-law type | sealed_producers.txt |
| apply_transient_certificate | apply | FIELD-ADJACENCY-GENERATORS-0 session-bound kernel-private transient producer witness | retire when transient field composition is intrinsic in an admitted chain type | sealed_producers.txt |
| apply_conservative | apply | FIELD-SWEEP-N4-PARITY-0 conservative FieldLawProof requires an undirected-symmetry certificate | retire when conservation law is intrinsic in the admitted program type | sealed_producers.txt |
| apply_non_conservative | apply | FIELD-SWEEP-N4-PARITY-0 explicit non-conservative FieldLawProof for ordinary authored folds | retire when field-law classification is intrinsic in the admitted program type | sealed_producers.txt |
| apply_undirected_symmetry_certificate | apply | FIELD-ADJACENCY-GENERATORS-0 adjacency-bound proof that every admitted weighted grid or LinkGraph edge has its reverse | retire when undirected symmetry is intrinsic in the admitted adjacency type | sealed_producers.txt |
| cpu_oracle_band_crossing_deltas | cpu_oracle | WRITE-DOOR-BAND-DELTA-0 CPU-oracle twin for sealed BandCrossingDelta; parity-only path | retire when CPU oracle is type-quarantined to in-crate parity harness | sealed_producers.txt |
| cpu_oracle_threshold_events | cpu_oracle | CPU-oracle twin for threshold events; parity-only path | retire when CPU oracle is type-quarantined to in-crate parity harness | sealed_producers.txt |
| cpu_oracle_emission_records | cpu_oracle | CPU-oracle twin for emission records; parity-only path | retire when CPU oracle is type-quarantined to in-crate parity harness | sealed_producers.txt |
| execute_ops_cpu_with_emissions | cpu_oracle | CPU-oracle batch path returning emission records; parity-only | retire when CPU oracle batch path is in-crate only | sealed_producers.txt |
| execute_threshold_ops_cpu | cpu_oracle | CPU-oracle batch path returning threshold emissions; parity-only | retire when CPU oracle batch path is in-crate only | sealed_producers.txt |
| apply_band_crossing_deltas_from_fused_emissions | apply | WRITE-DOOR-BAND-DELTA-0 sealed join mint from fused emissions + canonical Anchored registry | retire when band-crossing mint is an in-crate type boundary only | sealed_producers.txt |
| apply_band_crossing_deltas_from_threshold_events | apply | WRITE-DOOR-BAND-DELTA-0 sealed join mint from threshold events + canonical Anchored registry | retire when band-crossing mint is an in-crate type boundary only | sealed_producers.txt |
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
| CanonicalOrderProof | authority-export | FIELD-SWEEP-N4-PARITY-0 sealed authored-order proof consumed by field registration admission | retire when proof mint and admission are in-crate only | kernel_surface.txt |
| FIELD_SWEEP_LEGACY_PROGRAM_NODES | authority-export | FIELD-SWEEP-N4-PARITY-0 fixed legacy resource-class program-node ceiling | retire when EML-RESOURCE-CLASS-ADMISSION-0 replaces the single class | kernel_surface.txt |
| FIELD_SWEEP_LEGACY_STACK_SLOTS | authority-export | FIELD-SWEEP-N4-PARITY-0 fixed legacy resource-class stack ceiling | retire when EML-RESOURCE-CLASS-ADMISSION-0 replaces the single class | kernel_surface.txt |
| FIELD_SWEEP_WORKGROUP_SIZE | authority-export | FIELD-SWEEP-N4-PARITY-0 canonical generic sweep dispatch width | retire when dispatch geometry is admitted wholly inside the kernel | kernel_surface.txt |
| FieldAdjacency | authority-export | FIELD-ADJACENCY-GENERATORS-0 weighted grid or canonical LinkGraph adjacency over the existing input-list gather | retire when field authoring lowers through a narrower spec door | kernel_surface.txt |
| FieldConductanceCertificate | sealed-export | FIELD-ADJACENCY-GENERATORS-0 sealed per-node weighted-degree chi bound proof | retire when conductance stability is intrinsic in the admitted field-law type | kernel_surface.txt |
| FieldDegreeBucket | authority-export | FIELD-ADJACENCY-GENERATORS-0 read-only degree-homogeneous scheduling metadata | retire when scheduling is wholly kernel-private | kernel_surface.txt |
| FieldLawProof | authority-export | FIELD-SWEEP-N4-PARITY-0 sealed conservative or explicit non-conservative law proof | retire when field law is intrinsic in the admitted program type | kernel_surface.txt |
| FieldSweepAdmissionError | authority-export | FIELD-SWEEP-N4-PARITY-0 typed admission rejection surface | retire when field authoring lowers through a narrower spec door | kernel_surface.txt |
| FieldSweepExecutionError | authority-export | FIELD-SWEEP-N4-PARITY-0 typed CPU/GPU execution and parity-oracle error surface | retire when parity oracle and session are in-crate only | kernel_surface.txt |
| FieldSweepJitCacheIdentity | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 exact resource-class plus canonical-program cache identity evidence | retire when JIT profiling and evidence consumers are in-crate only | kernel_surface.txt |
| FieldSweepOutput | authority-export | FIELD-ADJACENCY-GENERATORS-0 typed matrix-or-kernel-private-transient destination | retire when field composition lowers through a narrower admitted chain door | kernel_surface.txt |
| FieldSweepProgramIdentity | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 canonical admitted-program identity evidence | retire when JIT profiling and evidence consumers are in-crate only | kernel_surface.txt |
| FieldSweepRegistration | authority-export | FIELD-SWEEP-N4-PARITY-0 immutable proof-present production field registration | retire when field registration consumers are in-crate only | kernel_surface.txt |
| FieldSweepRegistrationRequest | authority-export | FIELD-SWEEP-N4-PARITY-0 untrusted authored field admission request | retire when field authoring lowers through a narrower spec door | kernel_surface.txt |
| FieldSweepResourceClass | authority-export | FIELD-SWEEP-N4-PARITY-0 admitted fixed resource-class witness | retire when EML-RESOURCE-CLASS-ADMISSION-0 owns typed classes | kernel_surface.txt |
| FieldSweepSession | authority-export | FIELD-SWEEP-N4-PARITY-0 kernel-owned generic sweep residency/dispatch/readback session | retire when session construction is only reachable from the standard sim lifecycle | kernel_surface.txt |
| FieldTransientCertificate | sealed-export | FIELD-ADJACENCY-GENERATORS-0 sealed witness for compatible kernel-private transient reads | retire when transient composition is intrinsic in the admitted registration chain | kernel_surface.txt |
| GRID_N4_NSEW | authority-export | FIELD-SWEEP-N4-PARITY-0 Gu-Yang authored canonical N4 order | retire when order lowers from sealed authored adjacency specs | kernel_surface.txt |
| GRID_N4_WENS | authority-export | FIELD-SWEEP-N4-PARITY-0 PALMA authored canonical N4 order | retire when order lowers from sealed authored adjacency specs | kernel_surface.txt |
| GridN4Offset | authority-export | FIELD-SWEEP-N4-PARITY-0 exact unit cardinal offset value | retire when order lowers from sealed authored adjacency specs | kernel_surface.txt |
| GridOffset | authority-export | FIELD-ADJACENCY-GENERATORS-0 authored weighted grid-offset value | retire when adjacency authoring lowers through a narrower spec door | kernel_surface.txt |
| LinkGraphNeighbor | authority-export | FIELD-ADJACENCY-GENERATORS-0 authored canonical weighted LinkGraph row value | retire when link lowering is driver-private | kernel_surface.txt |
| UndirectedSymmetryCertificate | authority-export | FIELD-SWEEP-N4-PARITY-0 sealed conservative-fold adjacency symmetry witness | retire when undirected symmetry is intrinsic in the admitted adjacency type | kernel_surface.txt |
| apply_field_sweep_registration | authority-export | FIELD-SWEEP-N4-PARITY-0 sole admission door for proof-present field registrations | retire when field registration consumers are in-crate only | kernel_surface.txt |
| execute_field_sweep_cpu | authority-export | FIELD-SWEEP-N4-PARITY-0 independent CPU oracle for one generic sweep | retire only if another independent bit-exact parity judge replaces it | kernel_surface.txt |
| execute_field_sweep_cpu_chain | authority-export | FIELD-ADJACENCY-GENERATORS-0 CPU referee retaining kernel-private transient state across an admitted registration chain | retire only if another full-buffer chain oracle replaces it | kernel_surface.txt |
| execute_field_sweep_cpu_iterations | authority-export | FIELD-SWEEP-N4-PARITY-0 independent CPU oracle for iterative PALMA parity | retire only if another independent bit-exact parity judge replaces it | kernel_surface.txt |
| execute_field_sweep_cpu_natural_order | authority-export | FIELD-ADJACENCY-GENERATORS-0 independent unbucketed target-order parity judge | retire when scheduling equivalence is enforced by construction | kernel_surface.txt |
| field_param | authority-export | FIELD-SWEEP-N4-PARITY-0 fixed edge-context parameter vocabulary | retire when field authoring lowers through a sealed builder with no raw EML indices | kernel_surface.txt |
| field_sweep | authority-export | FIELD-SWEEP-N4-PARITY-0 authoritative generic field-sweep namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| grid_n4_offsets | authority-export | FIELD-ADJACENCY-GENERATORS-0 canonical authored-weight N4 preset | retire when grid adjacency authoring is driver-private | kernel_surface.txt |
| grid_n8_offsets | authority-export | FIELD-ADJACENCY-GENERATORS-0 N8 preset requiring an authored diagonal weight | retire when grid adjacency authoring is driver-private | kernel_surface.txt |
| grid_radius_offsets | authority-export | FIELD-ADJACENCY-GENERATORS-0 radius-r preset requiring authored shell weights | retire when grid adjacency authoring is driver-private | kernel_surface.txt |
| opcode_in_accumulator_vocabulary | authority-export | FIELD-SWEEP-N4-PARITY-0 separates field-only target/neighbor reads from ordinary EvalEML admission | retire when opcode context is encoded in distinct admitted opcode types | kernel_surface.txt |
| accumulator_op | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorInputGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorInputListTable | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpSession | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorOpSessionError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AccumulatorPipelineSessions | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AdmittedEvalEmlCombine | sealed-export | OC-K-EML-OPCODE-GATE-0 admitted combine token | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AdmittedEvalEmlOpcode | sealed-export | OC-K-EML-OPCODE-GATE-0 admitted opcode token | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ExactPrimitiveAdmission | sealed-export | EML-RESOURCE-CLASS-ADMISSION-0 proof token conjunctively binding determinism cost class and concrete consumer without opening opcode vocabulary | retire if exact primitive admission becomes wholly kernel-internal | kernel_surface.txt |
| ExactPrimitiveAdmissionDoor | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 stateful zero-or-one exact primitive admission gate | retire if exact primitive vocabulary expansion is permanently forbidden | kernel_surface.txt |
| ExactPrimitiveAdmissionRequest | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 untrusted conjunctive request requiring all three sealed keys | retire when no external generic primitive proposer remains | kernel_surface.txt |
| ExactPrimitiveBitSemantics | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 closed exact bit-semantics vocabulary | retire when determinism evidence is fixed to one non-authored policy | kernel_surface.txt |
| ExactPrimitiveConsumer | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 closed concrete consumer vocabulary excluding scenario labels | retire when consumer necessity is proven wholly inside the kernel | kernel_surface.txt |
| ExactPrimitiveConsumerEvidence | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 measured threshold-excess evidence for a concrete consumer | retire when no external measurement harness mints consumer keys | kernel_surface.txt |
| ExactPrimitiveConsumerKey | sealed-export | EML-RESOURCE-CLASS-ADMISSION-0 sealed concrete-consumer necessity key | retire if exact primitive admission becomes wholly kernel-internal | kernel_surface.txt |
| ExactPrimitiveCostEvidence | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 driver-originated compiled resource comparison bound to an EML class | retire when cost evidence is collected wholly inside the kernel | kernel_surface.txt |
| ExactPrimitiveCostKey | sealed-export | EML-RESOURCE-CLASS-ADMISSION-0 sealed non-regressing strict resource-improvement key | retire if exact primitive admission becomes wholly kernel-internal | kernel_surface.txt |
| ExactPrimitiveDeterminismEvidence | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 bit domain reference and supported-backend replay evidence | retire when determinism evidence is collected wholly inside the kernel | kernel_surface.txt |
| ExactPrimitiveDeterminismKey | sealed-export | EML-RESOURCE-CLASS-ADMISSION-0 sealed exhaustive reference and backend replay identity key | retire if exact primitive admission becomes wholly kernel-internal | kernel_surface.txt |
| ExactPrimitiveDomainPolicy | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 closed domain and special-value policy vocabulary | retire when determinism policy is fixed to one non-authored domain | kernel_surface.txt |
| ExactPrimitiveResourceEffect | authority-export | EML-RESOURCE-CLASS-ADMISSION-0 typed compiled registers binary and local-memory counter row | retire when cost evidence is collected wholly inside the kernel | kernel_surface.txt |
| AO_WGSL0_ENTRY_POINT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ao_wgsl0_fast_path_compatible | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0Compatibility | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0FallbackReason | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| AoWgsl0PlanShape | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ApproximateDecisionDiagnostic | authority-export | OC-K-DECISION-INGRESS-0 approximate decision diagnostic only | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ApproximateDiagnostic | authority-export | OC-K-EXACT-GATE-0 diagnostic-only magnitude; cannot mint ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| apply_band_crossing_deltas_from_fused_emissions | authority-export | WRITE-DOOR-BAND-DELTA-0 sealed join mint from fused emissions + canonical Anchored registry; xref sealed path | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| apply_band_crossing_deltas_from_threshold_events | authority-export | WRITE-DOOR-BAND-DELTA-0 sealed join mint from threshold events + canonical Anchored registry; xref sealed path | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| BandCrossingDelta | sealed-export | WRITE-DOOR-BAND-DELTA-0 sealed write-impact delta; minted only from fused threshold emissions | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| BandCrossingDirection | sealed-export | WRITE-DOOR-BAND-DELTA-0 sealed rising/falling direction for write-impact deltas | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
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
| column_from_wire | authority-export | PLAN-STRUCT-TYPING-0 WGSL-RAW-BOUNDARY rematerialize helper; sole production from_gpu_round_trip call site | RF-COLUMN-MINT-MIGRATE-0 retires residual raw doors | kernel_surface.txt |
| ColumnRuleDescriptor | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| combine_in_closed_vocabulary | authority-export | OC-K-EML-OPCODE-GATE-0 closed combine vocabulary predicate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CombineRegistrationRequest | sealed-export | OC-K-EML-OPCODE-GATE-0 combine registration request class | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CommitmentRegistration | sealed-export | OC-K-EXACT-GATE-0 commitment registration requiring ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| conjunctive_recipe_registration_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| conjunctive_recipe_registrations_to_transfer | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| context | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_oracle | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| cpu_oracle_band_crossing_deltas | authority-export | WRITE-DOOR-BAND-DELTA-0 CPU-oracle twin for sealed BandCrossingDelta mint; xref sealed path | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
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
| DerivedSpanAdmissionError | authority-export | DERIVED-SPAN-PROJECTION-INVALIDATION-0 typed span-admission rejection returned by the public OverlaySpanProjection::compile seam and carried through simthing-sim GpuSyncError into SessionError | retire when projection compile and its failure contract are in-crate only | kernel_surface.txt |
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
| emission_plan_signature_fields | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionFormula | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionOpPlanSignature | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionPlan | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionPlanError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRecord | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRecordGpu | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionRegistration | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionSyncError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmissionToken | sealed-export | OC-K-DECISION-INGRESS-0 emission token from sealed ThresholdEmission | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emit_on_threshold_registrations_to_gpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| emit_on_threshold_registrations_to_ops | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| eml_opcode_gate | authority-export | OC-K-EML-OPCODE-GATE-0 module surface; closed EvalEML opcode/combine gate | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlGpuProgramTable | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlTreeRangeGpu | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EmlUploadError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| encode_column | authority-export | PLAN-STRUCT-TYPING-0 WGSL-RAW-BOUNDARY drop helper; sole production ColumnIndex::raw_u32 encode path for plan columns | RF-COLUMN-MINT-MIGRATE-0 retires residual raw doors | kernel_surface.txt |
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
| LegacyOracleFamily | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| max_candidate_f_magnitude_bits | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| mint_exact_magnitude_proof_candidate_f | authority-export | OC-K-EXACT-GATE-0 GPU Candidate F mint of ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| mint_exact_magnitude_proof_candidate_f_cpu | authority-export | OC-K-EXACT-GATE-0 CPU Candidate F mint of ExactMagnitudeProof | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_CONSTANT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_MAX_EMIT | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| NO_TREE_ID | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| ObjectResidency | sealed-export | ROW-SLOT-OBJECT-SEMANTICS-0 kernel-minted object-to-row residency token with private fields | retire when object-derived residency is enforced by closed kernel type admission | kernel_surface.txt |
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
| OverlayProjectionHostChange | authority-export | DERIVED-SPAN-PROJECTION-INVALIDATION-0 authoritative changed-locus kind minted by simthing-sim boundary and consumed through gpu_sync; carries no writer-subsystem discriminant | retire when boundary change reporting is admitted wholly inside the kernel | kernel_surface.txt |
| OverlaySpanProjection | authority-export | DERIVED-SPAN-PROJECTION-INVALIDATION-0 compiled span/profile projection compiled and refreshed by simthing-sim gpu_sync and retained by OverlayCompileCache | retire when projection compile and retention are in-crate only | kernel_surface.txt |
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
| apply_epoch_rebind_to_values | authority-export | SLOT-LOGICAL-IDENTITY-0 boundary-upload baking of one EpochRebind into the slot-major values plane (zero per-access indirection between epochs) | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotAllocError | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotDeltaRange | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SlotSummary | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| CommittedResidencyCorruption | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 typed committed overlap/out-of-bounds diagnostic with owning granters and exact extents | retire when committed-corruption observation is wholly session-internal | kernel_surface.txt |
| CommittedResidencyPlacement | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 kernel-minted authoritative physical placement with private fields | retire when placement observation is wholly session-internal | kernel_surface.txt |
| ProvisionalResidencyEntitlement | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 typed physical input consumed by the kernel after the driver validates a graduated market grant | retire when the market grant and kernel boundary share one crate-private admission seam | kernel_surface.txt |
| ResidencyEntitlementError | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 typed provisional-entitlement shape refusal | retire when entitlement conversion is wholly driver-private | kernel_surface.txt |
| ResidencyExtent | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 checked half-open physical extent vocabulary explicitly authorized by A1 | retire when all physical placement authoring lowers through a narrower vendor door | kernel_surface.txt |
| ResidencyExtentError | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 typed zero-length and overflow extent refusal | retire when extent construction is wholly behind the vendor door | kernel_surface.txt |
| ResidencyPlacementDisposition | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 closed committed relocated unchanged outcome vocabulary | retire when placement observation is wholly session-internal | kernel_surface.txt |
| ResidencyPlacementError | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 typed ordinary refusal terminal corruption configuration and existing-remap refusal surface | retire when all placement consumers are in-crate | kernel_surface.txt |
| ResidencyPlacementIdentity | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 private-field stable granter grantee market relationship identity | retire when placement observation is wholly session-internal | kernel_surface.txt |
| ResidencyPlacementOutcome | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 kernel-minted committed placement and closed disposition | retire when placement observation is wholly session-internal | kernel_surface.txt |
| ResidencyPlacementRefusal | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 kernel-minted U-preserving ordinary physical infeasibility product | retire when refusal handling is wholly session-internal | kernel_surface.txt |
| ResidencyPlacementRefusalReason | authority-export | STEMTHING-B-VRAM-RESIDENCY-0 closed exact physical infeasibility vocabulary | retire when refusal handling is wholly session-internal | kernel_surface.txt |
| ResidencyRelocationOutcome | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 kernel-minted placement plus the existing epoch-rebind section | retire when relocation observation is wholly session-internal | kernel_surface.txt |
| ResidencySessionTermination | sealed-export | STEMTHING-B-VRAM-RESIDENCY-0 kernel-minted recorded hard-fault product with exact corruption cause and generation | retire when terminal handling is wholly session-internal | kernel_surface.txt |
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
| ThresholdEvent | sealed-export | Sealed record/type export; produced only through sanctioned doors | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
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
| wgsl_encode | authority-export | ROW-SLOT-OBJECT-SEMANTICS-0 single fenced raw WGSL encode boundary; no shader or opcode semantics | PLAN-STRUCT-TYPING-0 collapses round-trip mints into this boundary | kernel_surface.txt |
| WORKGROUP_SIZE | surface-inert | Inert public kernel constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| world_state | authority-export | Exported kernel module surface; authority-bearing namespace | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldAccumulatorRuntime | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldGpuState | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| WorldSummaryRuntime | authority-export | Exported kernel runtime/planning/oracle surface | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| EXP_PRIMITIVE_NAME | surface-inert | EML-EXP-PRIMITIVE-0 admitted primitive registry name constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| SoftmaxWeightGadget | authority-export | EML-EXP-PRIMITIVE-0 stabilized softmax weight gadget riding existing MAX and Sum bands | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| admit_exp_call_sites | authority-export | EML-EXP-PRIMITIVE-0 tree-scan wiring of the 5.10 call-site shapes for the admitted EXP primitive | retire if exact primitive call-site admission becomes wholly kernel-internal | kernel_surface.txt |
| eml_exp_qualification | authority-export | EML-EXP-PRIMITIVE-0 pinned exhaustive-qualification artifacts and certified toolchain roster | retire if qualification pinning becomes wholly kernel-internal | kernel_surface.txt |
| exp_primitive_domain | authority-export | EML-EXP-PRIMITIVE-0 canonical full-domain EXP interval constructor | retire if exact primitive call-site admission becomes wholly kernel-internal | kernel_surface.txt |
| LN_PRIMITIVE_NAME | surface-inert | EML-LN-PRIMITIVE-0 admitted primitive registry name constant | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| LnConsumerGadgets | authority-export | EML-LN-PRIMITIVE-0 guarded PowerLaw eml-operator entropy LogAccumulate authored-data builders | retire when kernel export set is closed by type-boundary admission | kernel_surface.txt |
| eml_ln_qualification | authority-export | EML-LN-PRIMITIVE-0 pinned exhaustive-qualification artifacts and certified roster | retire if qualification pinning becomes wholly kernel-internal | kernel_surface.txt |
| ln_primitive_domain | authority-export | EML-LN-PRIMITIVE-0 canonical positive-normal LN interval constructor | retire if exact primitive call-site admission becomes wholly kernel-internal | kernel_surface.txt |
| GrowthResidencyCommit | sealed-export | STEMTHING-B-GROWTH-ENTITLEMENT-SEAM-0 consumed product joining real 11.2a grant provenance to committed 11.2b placement before ordinary structural attachment | retire when the ordinary structural mutation boundary is wholly kernel-internal | kernel_surface.txt |

## Sealed Types

| sealed type | source |
| --- | --- |
| BandCrossingDelta | sealed_types.txt |
| CanonicalOrderProof | sealed_types.txt |
| CandidateFMagnitudeReport | sealed_types.txt |
| EmissionRecord | sealed_types.txt |
| EmissionRecordGpu | sealed_types.txt |
| EmissionRecordReadback | sealed_types.txt |
| FieldLawProof | sealed_types.txt |
| FieldConductanceCertificate | sealed_types.txt |
| FieldSweepRegistration | sealed_types.txt |
| FieldTransientCertificate | sealed_types.txt |
| PlacedParticipant | sealed_types.txt |
| ResolvedWriteAuthority | sealed_types.txt |
| ThresholdEmission | sealed_types.txt |
| ThresholdEmissionGpu | sealed_types.txt |
| ThresholdEmissionReadback | sealed_types.txt |
| ThresholdEvent | sealed_types.txt |
| ThresholdEventCandidatesReadback | sealed_types.txt |
| ThresholdEventGpu | sealed_types.txt |
| UndirectedSymmetryCertificate | sealed_types.txt |

## Contention Mechanisms

| need | mechanism | ready-surface | promotion-blocker | source |
| --- | --- | --- | --- | --- |
| resolve-ownership | owner resolution | simthing_core::owner_channel::resolve_owner -- total for valid in-tree members, pure, never materialized; absence means inherit, unbound resolves neutral, and foreign/malformed authority fails closed | retire when ownership is intrinsic to an admitted type and cannot be asked for separately | contention_mechanisms.txt |
| rebind-ownership | ownership fission | simthing_core::owner_channel::bind_owner at a subtree root -- ONE write re-parents the whole subtree; unbind_owner reverses it | retire when subtree ownership is expressed structurally rather than by property binding | contention_mechanisms.txt |
| flip-ownership | ownership rebind | bind_owner from the neutral owner to a named one -- a flip is an ordinary rebind, never a None-to-Some transition with its own code path | retire when movement authority owns the flip end to end | contention_mechanisms.txt |
| segregate-flows-by-owner | owner-keyed bucket | simthing_spec::OwnerChannelScopeKey -- canonical {OwnerRef, ResourceKey, ScopeId} ordering makes same-owner and different-owner behaviour emergent without an owner-equality branch | retire when bucketing is intrinsic to the admitted channel type | contention_mechanisms.txt |
| detect-ownership-crossing | crossing predicate | simthing_core::owner_channel::is_ownership_crossing -- only crossing flow deltas are retained in addition to ordinary own aggregates; identity-edge flows are reconstructible | retire when crossing detection is intrinsic to reduce-up emission | contention_mechanisms.txt |
| reduce-owner-channel | generalized reduce-up | simthing_spec::reduce_owner_channel_rf -- canonical {OwnerRef, ResourceKey, ScopeId} buckets admit N owners and prove conservation without an owner-equality branch | retire when this reduction is intrinsic to the resident RF execution type | contention_mechanisms.txt |
| reconstruct-owner-channel | bounded STEAD reconstruction | simthing_spec::reconstruct_owner_channel_rf_map -- ordinary active node/resource aggregates plus exactly one retained flow record per ownership crossing reconstruct the RF map | retire when resident STEAD execution consumes the crossing rows directly | contention_mechanisms.txt |
| compute-overlay-value-from-runtime-state | EML overlay program (singular TransformOp value path) | simthing_core::TransformOp::to_eml_nodes + eval_overlay_eml / admit_overlay_eml_program(EmlPerProgramCap) -- PARAM(0)=current PARAM(1)=N(CostBand depth); LITERAL_F32/SELECT/CMP_GE/arithmetic only; no Static/Computed discriminant | retire when overlay value is unconstructible except as an admitted EML program at the type tier | contention_mechanisms.txt |
| unresolved-demand-eml-costband-persistence | mechanism | UnresolvedDemandObservation + fund_unresolved_persistence (CONTENTION-ARENA-EXECUTED-0): unresolved demand U is valued by authored EML over elapsed generations, quantized by the existing scalar CostBand, and funded as an ordinary UntilDissolvedWith overlay; same-generation consequence is rejected | retire when persistence valuation is admitted wholly inside the authored clearing program | contention_mechanisms.txt |

## Forbidden / Screened Patterns

| scan-id | reliability | why | target | pattern/source | exclude | promotion-blocker | source |
| --- | --- | --- | --- | --- | --- | --- | --- |
| FIELD-SWEEP-SINGLE-PATH-ALGEBRA | RELIABLE | design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; algebra is authored EML data and never an enum/tag/operator match in the sweep path | crates/simthing-{kernel,gpu,driver}/src/** | enum\\s+(?:FieldAlgebra\|FieldKind\|SemiringKind\|SweepOperator)\\b\|match\\s+[^;\\n]*(?:field_kind\|semiring\|algebra\|operator_identity)\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire only when algebra identity is unrepresentable outside sealed EML program data | scans.tsv |
| FIELD-SWEEP-SINGLE-PATH-SHADERS | RELIABLE | design 0.0.8.7 Phase 5 FIELD-SWEEP-SINGLE-PATH; no eighth bespoke field shader in either production shader home beside the exact canonical generic interpreter | crates/simthing-{gpu,kernel}/src/shaders/**/*.wgsl | @ALLOWLIST:field-sweep-shaders | (none) | retire at DOCTRINE-CI-RECONCILE-0 when the seven-file migration-oracle catalogue is empty and absence is type/admission enforced | scans.tsv |
| FIELD-SWEEP-LEGACY-CALLERS | RELIABLE | design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; the seven retiring operators are migration referees only and have zero compiled production callers | crates/simthing-*/src/**/*.rs | @ALLOWLIST:field-sweep-legacy-callers | (none) | retire when the seven legacy operator implementations are deleted at their authorized removal rung | scans.tsv |
| FIELD-SWEEP-DENSE-CAP-CROSSING | RELIABLE | design 0.0.8.7 Phase 5 FIELD-ADJACENCY-GENERATORS-0; generic and sparse LinkGraph adjacency cannot inherit dense REGION_FIELD theater caps | crates/simthing-{kernel,driver}/src/**/*.rs | @ALLOWLIST:field-sweep-dense-caps | (none) | retire when dense and sparse adjacency admission are separated by unforgeable types | scans.tsv |
| B3-BUFFER-ESCAPE | RELIABLE | design §5 B3 buffer escape | crates/simthing-kernel/src/** | pub fn [a-z_]+\\(&self\\) *-> *&(wgpu::)?Buffer\|^\\s*pub [a-z_]+ *: *Buffer\|-> *BindingResource | pub\\(crate\\);compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when buffer accessors are crate-private type boundary only | scans.tsv |
| FORGE-MINTERS | RELIABLE | design §5 forge minters | crates/simthing-kernel/src/** | pub fn (from_boundary_delivery\|for_kernel_readback\|for_boundary_install)\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when sealed-producer allowlist scan (CI-A-ALLOWLIST-SCANS-0) subsumes explicit forge names | scans.tsv |
| UNSAFE-FN | RELIABLE | design §5 unsafe fn | crates/simthing-{kernel,sim}/src/** | \\bunsafe fn\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when unsafe is unrepresentable at kernel/sim type boundary | scans.tsv |
| UNSAFE-ALLOW-ATTR | RELIABLE | design §5 allow unsafe attr | crates/simthing-{kernel,sim}/src/lib.rs | #!\\[allow\\(unsafe_code\\)\\] | (none) | retire when crate attributes are admission-typed not prose | scans.tsv |
| UNSAFE-FORBID-ATTR | RELIABLE | design §5 forbid unsafe attr | crates/simthing-{kernel,sim}/src/lib.rs | @REQUIRE:#!\\[forbid\\(unsafe_code\\)\\] | (none) | retire when semantic-free crate template enforces forbid at type boundary | scans.tsv |
| DENY-TOML-STUB | RELIABLE | design §0.6.6 deny.toml stub | deny.toml | . | (none) | retire when dependency policy is compile-time typed not file-shaped | scans.tsv |
| SIM-KIND-READ | HEURISTIC | design §5 sim .kind read | crates/simthing-sim/src/** | match .*\\.kind\|\\.kind\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*//;delta_log;sim_runtime_tree;kind_production_audit;^\\s*assert_\|^\\s*#\\[test\\] | promote when runtime tree view is kind-free at type boundary | scans.tsv |
| SEMANTIC-WORDS | HEURISTIC | design §5 semantic words below spec | crates/simthing-{sim,kernel}/src/** | faction\|combat\|terran\|pirate\|diplomacy | compile_fail;^\\s*//!;^\\s*///;^\\s*//;^\\s*assert_\|^\\s*#\\[test\\]\|SimThingKind:: | promote when game semantics are spec-boundary typed only | scans.tsv |
| SPEC-STRING-CHANNEL | HEURISTIC | design §5 stringly channel identity | crates/simthing-spec/src/** | owner_ref *: *(Option<)?String\|resource_key *: *(Option<)?String | compile_fail;^\\s*//!;^\\s*///;^\\s*//;channel_key\\.rs | promote when channel identity is newtyped in spec admission | scans.tsv |
| ALLOW-SEALED-PRODUCERS | RELIABLE | design §5 sealed producer allowlist | crates/simthing-kernel/src/** | @ALLOWLIST:sealed-producers | (none) | retire when sealed producers are type-boundary admitted | scans.tsv |
| ALLOW-BUFFER-HANDLES | RELIABLE | design §5 buffer handle allowlist | crates/simthing-kernel/src/** | @ALLOWLIST:buffer-handles | (none) | retire when buffer handles are crate-private type boundary only | scans.tsv |
| ALLOW-KERNEL-SURFACE | RELIABLE | design §5 kernel surface allowlist | crates/simthing-kernel/src/lib.rs | @ALLOWLIST:kernel-surface | (none) | retire when kernel exports are type-boundary closed | scans.tsv |
| TEST-BUDGET | HEURISTIC | design §0.9.5 test admission budget | crates/** | @TEST_BUDGET | (none) | retire if test admission becomes a typed ledger gate end-to-end | scans.tsv |
| SPEC-LOWERER-KIND-READ | HEURISTIC | ci_screening_surface §12 + design §0A.1; HEURISTIC tripwire: spec/lowering kind read may be legitimate role-resolution, but closed-lowerer hits are higher suspicion because lowerers are constitutionally closed unless a DA-authorized amendment names them | crates/simthing-{spec,clausething}/src/** | match .*\\.kind\|\\.kind\\s*(==\|!=)\|match\\s+(?:&)?kind\\s*\\{[\\s\\S]*?SimThingKind:: | compile_fail;^\\s*//!;^\\s*///;^\\s*//;^\\s*assert_\|^\\s*#\\[test\\];planet_non_grid_child_kind_label;is_admitted_planet_non_grid_child;scenario_deferral_kind_label;planet_child_location_error_kind_label;simthing_kind_label;location_participant_kind_label;non_location_participant_kind_label | retire when spec-layer role resolution is role-keyed by SubFieldRole/column admission boundaries rather than SimThingKind branching | scans.tsv |
| GUARD-KABUKI-TRIPWIRE | HEURISTIC | handoff_template section H + ci_screening_surface section 4; HEURISTIC tripwire for bespoke source-scanning guards and test-side include_str source greps; HC-6: symbol with well-formed FRESH HORIZON-ENTRY(iso-date): consumer/ref is EXEMPT (dated+assessable; unmarked/stale stay FLAGGED; never bare-token forever-pass); HC-8 accepted evasion residue: PRIVATE fn source-scanner or var-bound include_str! evades the pub-fn-anchored arms (DA review is the backstop; regex intentionally NOT widened — would false-fire on legit parsers); legitimate cases route to INSPECT triage, never FAIL | crates/**/{src,tests}/**/*.rs | pub fn [A-Za-z0-9_]+\\([^)]*source:\\s*&str[\\s\\S]{0,1200}source\\.(contains\|find\|matches\|lines\|to_ascii_lowercase)\\(\|pub fn [A-Za-z0-9_]+\\([^)]*path:\\s*&Path[\\s\\S]{0,800}read_to_string\\(path\\)[\\s\\S]{0,800}\\.(contains\|find\|matches\|lines)\\(\|include_str!\\("../src/[^"\\n]*"\\)\\.(contains\|find\|matches\|lines)\\( | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when anti-kabuki source-scan guard review is admission-typed or no production/test guard uses source text as a proof surrogate | scans.tsv |
| EXECUTION-STATUS-UNCLASSIFIED | HEURISTIC | design 0.0.8.7 §3 Phase 0 EXECUTION-STATUS-TAXONOMY-0; HEURISTIC: execution-flavored driver/kernel surface missing from scripts/ci/execution_status_taxonomy.tsv (delta-scoped on PR) | crates/simthing-{driver,kernel}/src/** | @EXECUTION_STATUS_TAXONOMY | (none) | retire when execution posture is type-admitted at the object model and the TSV is no longer the live registry | scans.tsv |
| CELL-STORAGE-POLYMORPHISM | HEURISTIC | design 0.0.8.7 §2 P0(e) fence (i) CELL-STORAGE-POLYMORPHISM; HEURISTIC reach detector for tagged/templated/heterogeneous matrix-cell storage across production crates (workshop excluded) | crates/simthing-{kernel,core,gpu,sim,driver,mapgenerator,mapeditor,feeder,tools,clausething,spec}/src/** | enum\\s+\\w*(?:Matrix\|Lane)?Cell\\w*\\s*\\{[\\s\\S]{0,400}(?:Box\\s*<\|dyn\\s+\|tagged.?union\|Any\\b)\|heterogeneous\\s+(?:matrix\\s+)?cell\|CellStorage\\s*[<=]\|type\\s+\\w*CellValue\\s*=\\s*enum | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when homogeneous-lane storage is type-admitted and heterogeneous cell reaches cannot compile | scans.tsv |
| BESPOKE-PATHFINDER | HEURISTIC | design 0.0.8.7 §4 TRIAD DOORS / P5 PALMA BESPOKE-PATHFINDER; HEURISTIC: A* (BinaryHeap+came_from/g_score/open_set) OR ordinary Dijkstra (dist/distance+prev/predecessor with dijkstra/shortest_path/relax_edge) in production crates | crates/simthing-{kernel,core,gpu,sim,driver,mapgenerator,mapeditor,feeder,tools,clausething,spec}/src/** | BinaryHeap[\\s\\S]{0,1200}(?:came_from\|g_score\|open_set)\|(?:came_from\|g_score\|open_set)[\\s\\S]{0,1200}BinaryHeap\|(?:dijkstra\|shortest_path)[\\s\\S]{0,1000}(?:\\bdist\\b\|\\bdistance\\b)[\\s\\S]{0,1000}(?:\\bprev\\b\|\\bpredecessor\\b)\|(?:\\bdist\\b\|\\bdistance\\b)[\\s\\S]{0,1000}(?:\\bprev\\b\|\\bpredecessor\\b)[\\s\\S]{0,1000}(?:dijkstra\|shortest_path\|relax_edge) | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when pathing is exclusively the PALMA door (type-admitted) and bespoke graph-search cannot compile in production crates | scans.tsv |
| BORDER-SERVICE | HEURISTIC | design 0.0.8.7 §4 TRIAD DOORS / P5 Gu-Yang BORDER-SERVICE; HEURISTIC: border/frontline semantic service machinery (not mere presentation polyline projection/cache); mapeditor included for service-layer reaches | crates/simthing-{kernel,core,gpu,sim,driver,mapgenerator,mapeditor,feeder,tools,clausething,spec}/src/** | border_service\|FrontlineTracer\|BorderPolylineEngine\|marching_squares\|contour_extract(?:ion)?\|trace_border(?:line)?\\s*\\(\|fn\\s+\\w*border\\w*_service\\b\|struct\\s+\\w*BorderService\\b | compile_fail;^\\s*//!;^\\s*///;^\\s*// | retire when border observation is exclusively the Gu-Yang door and bespoke border services cannot compile in production crates | scans.tsv |
| OWNER-POLICY-WEIGHT-AUTHORITY-MINT | HEURISTIC | design 0.0.8.7 §3 Phase 3 FIRST-CITIZEN-SPECIALISTS-0; HEURISTIC: authored clause/scenario-JSON sources minting OWNER_POLICY_WEIGHT_AUTHORITY property id 8_300_318 outside hydration field-economy derivation (hydration-derived dumps excluded) | **/*.{clause,simthing-scenario.json} | 8_300_318\|8300318 | fixtures[/\\\\]known_bad[/\\\\].*authority_mint;fixtures[/\\\\]hydration_derived[/\\\\];from-clause\\.simthing-scenario\\.json;^\\s*//\|^\\s*#\|_comment | retire when scenario admission type-rejects out-of-band authority stamps | scans.tsv |
