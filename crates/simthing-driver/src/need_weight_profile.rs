//! RF-5: install + EvalEML transport for need / weight_profile bindings.
//!
//! Pattern mirrors CT-RF-EML-RATE-0 (`gated_rates`): resolve at session open,
//! seed authored weights onto the participant, register an ExactDeterministic
//! EvalEML WeightedAccumulator band writing the live need cell, and optionally
//! mirror need into AllocatorWeight. Threshold events use the existing
//! Accumulator Threshold + EmitEvent substrate.

use simthing_core::{
    eml_nodes, AccumulatorOp, ClampBehavior, ColumnIndex, CombineFn, ConsumeMode, DimensionRegistry,
    EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId,
    GateSpec, ScaleSpec, SimPropertyId, SimThing, SimThingId, SlotIndex, SourceSpec, SubFieldRole,
    SubFieldSpec,
};
use simthing_spec::{
    compile_property, EmlGadgetInstanceSpec, InstallTargetSpec, NeedWeightProfileBindingSpec,
    NeedWeightProfileInputSpec, NeedWeightProfileThresholdSpec, PropertySpec, ResourceFlowSpec,
    SpecError,
};

use crate::arena_hierarchy::resolve_node_columns;
use crate::arena_participant::ArenaParticipantScaffold;
use crate::install::{find_simthing_mut, resolve_install_target, InstallError};
use crate::scenario::Scenario;

/// Namespace for the RF-5 need host property materialised at install.
pub const NEED_PROPERTY_NAMESPACE: &str = "rf_need";
/// Amount role on the need host — live need value written by EvalEML.
pub const NEED_AMOUNT_ROLE: &str = "need";

const NEED_WEIGHT_TREE_BASE: u32 = 7_200_000;

/// Fully resolved binding ready for seeding + EvalEML op build.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedNeedWeightProfile {
    pub id: String,
    pub profile: String,
    pub participant_slot: u32,
    pub hosted_id: SimThingId,
    pub arena: String,
    pub arena_idx: u32,
    /// Global column of the need Amount cell.
    pub need_col: u32,
    /// Optional AllocatorWeight global column on the arena flow property.
    pub allocator_weight_col: Option<u32>,
    /// Compiled EvalEML nodes for WeightedAccumulator with remapped columns.
    pub nodes: Vec<EmlNodeGpu>,
    pub weight_seeds: Vec<f32>,
    pub weight_cols: Vec<u32>,
    pub input_cols: Vec<u32>,
    pub input_literals: Vec<Option<f32>>,
    pub threshold: Option<NeedWeightProfileThresholdSpec>,
}

/// Extract the single WeightedAccumulator from an authored stack (fail-closed).
pub fn extract_weighted_accumulator(
    binding: &NeedWeightProfileBindingSpec,
) -> Result<(Vec<u32>, Vec<u32>, Option<u32>), InstallError> {
    if binding.stack.gadgets.len() != 1 {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "stack must contain exactly one WeightedAccumulator gadget".into(),
        });
    }
    match &binding.stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            output_col,
            ..
        } => {
            if input_cols.is_empty() || input_cols.len() != weight_cols.len() {
                return Err(InstallError::NeedWeightProfileInvalid {
                    binding: binding.id.clone(),
                    reason: "WeightedAccumulator input_cols/weight_cols length mismatch".into(),
                });
            }
            Ok((input_cols.clone(), weight_cols.clone(), *output_col))
        }
        other => Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "expected WeightedAccumulator, got `{}`",
                other.kind_name()
            ),
        }),
    }
}

/// Resolve every authored need/weight_profile binding against the live install.
pub fn resolve_need_weight_profiles(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &mut DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
) -> Result<Vec<ResolvedNeedWeightProfile>, InstallError> {
    let mut out = Vec::with_capacity(spec.need_weight_profiles.len());
    for binding in &spec.need_weight_profiles {
        out.push(resolve_one(binding, spec, scenario, root, registry, scaffold)?);
    }
    Ok(out)
}

fn resolve_one(
    binding: &NeedWeightProfileBindingSpec,
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
    registry: &mut DimensionRegistry,
    scaffold: &ArenaParticipantScaffold,
) -> Result<ResolvedNeedWeightProfile, InstallError> {
    let (input_cols_local, weight_cols_local, _output_local) =
        extract_weighted_accumulator(binding)?;

    if binding.weight_seeds.is_empty() {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "weight_seeds empty — no default-weight fallback".into(),
        });
    }
    if binding.weight_seeds.len() != weight_cols_local.len() {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "weight_seeds len {} != weight_cols len {}",
                binding.weight_seeds.len(),
                weight_cols_local.len()
            ),
        });
    }
    if binding.inputs.len() != input_cols_local.len() {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: format!(
                "inputs len {} != input_cols len {}",
                binding.inputs.len(),
                input_cols_local.len()
            ),
        });
    }
    if binding.weight_seeds.iter().any(|w| !w.is_finite())
        || binding.inputs.iter().any(|i| match i {
            NeedWeightProfileInputSpec::Literal(v) => !v.is_finite(),
            NeedWeightProfileInputSpec::Property(_) => false,
        })
    {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "non-finite weight_seed or input literal".into(),
        });
    }

    let (arena_idx, arena) = spec
        .arenas
        .iter()
        .enumerate()
        .find(|(_, arena)| arena.name == binding.arena)
        .ok_or_else(|| SpecError::UnknownArenaReference {
            arena: binding.arena.clone(),
            context: format!("need_weight_profiles.{}", binding.id),
        })?;

    let hosted_ids = resolve_install_target(&binding.install, scenario, root)?;
    if hosted_ids.is_empty() {
        return Err(InstallError::NoMatchingOwners {
            tree_id: binding.id.clone(),
            target: binding.install.clone(),
        });
    }
    if hosted_ids.len() != 1 {
        return Err(InstallError::NeedWeightProfileInvalid {
            binding: binding.id.clone(),
            reason: "install target must resolve to exactly one participant".into(),
        });
    }
    let hosted_id = hosted_ids[0];
    let raw = hosted_id.raw();
    let admitted = arena
        .explicit_participants
        .iter()
        .any(|p| p.subtree_root_id == raw);
    if !admitted {
        return Err(InstallError::NeedWeightProfileTargetNotAdmitted {
            binding: binding.id.clone(),
            arena: arena.name.clone(),
            subtree_root_id: raw,
        });
    }
    let participant_slot = scaffold
        .index
        .participant_slot(hosted_id, arena_idx as u32)
        .ok_or_else(|| InstallError::NeedWeightProfileTargetNotAdmitted {
            binding: binding.id.clone(),
            arena: arena.name.clone(),
            subtree_root_id: raw,
        })?
        .raw();

    // Materialise a dedicated need host property (Amount + weight + input columns).
    let need_property_id = ensure_need_host_property(binding, registry)?;
    let layout = registry.property(need_property_id).layout.clone();
    let need_offset = layout
        .offset_of(&SubFieldRole::Amount)
        .ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
    let need_start = registry.column_range(need_property_id).start as u32;
    let need_col = need_start + need_offset.lane() as u32;

    // Remap authored local weight/input cols onto the need host's Named columns.
    let mut weight_cols = Vec::with_capacity(weight_cols_local.len());
    for (i, _) in weight_cols_local.iter().enumerate() {
        let role = SubFieldRole::Named(format!("w_{i}"));
        let off = layout.offset_of(&role).ok_or_else(|| {
            InstallError::NeedWeightProfileInvalid {
                binding: binding.id.clone(),
                reason: format!("missing weight subfield w_{i}"),
            }
        })?;
        weight_cols.push(need_start + off.lane() as u32);
    }
    let mut input_cols = Vec::with_capacity(input_cols_local.len());
    let mut input_literals = Vec::with_capacity(input_cols_local.len());
    for (i, input) in binding.inputs.iter().enumerate() {
        match input {
            NeedWeightProfileInputSpec::Literal(v) => {
                // Still allocate a column so the WeightedAccumulator shape is uniform;
                // the EvalEML tree uses LITERAL for these.
                let role = SubFieldRole::Named(format!("in_{i}"));
                let off = layout.offset_of(&role).ok_or_else(|| {
                    InstallError::NeedWeightProfileInvalid {
                        binding: binding.id.clone(),
                        reason: format!("missing input subfield in_{i}"),
                    }
                })?;
                input_cols.push(need_start + off.lane() as u32);
                input_literals.push(Some(*v));
            }
            NeedWeightProfileInputSpec::Property(key) => {
                let prop_id = registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
                    InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                        property: format!("{}::{}", key.namespace, key.name),
                    })
                })?;
                let prop_layout = &registry.property(prop_id).layout;
                let off = prop_layout.offset_of(&SubFieldRole::Amount).ok_or_else(|| {
                    InstallError::NeedWeightProfileInvalid {
                        binding: binding.id.clone(),
                        reason: format!(
                            "input property {}::{} missing Amount",
                            key.namespace, key.name
                        ),
                    }
                })?;
                let start = registry.column_range(prop_id).start as u32;
                input_cols.push(start + off.lane() as u32);
                input_literals.push(None);
            }
        }
    }

    let flow_property_id = registry
        .id_of(&arena.flow_property.namespace, &arena.flow_property.name)
        .ok_or_else(|| {
            InstallError::Spec(SpecError::UnknownResourceFlowProperty {
                property: format!(
                    "{}::{}",
                    arena.flow_property.namespace, arena.flow_property.name
                ),
            })
        })?;
    let flow_layout = &registry.property(flow_property_id).layout;
    let cols = resolve_node_columns(flow_layout, &arena.name).map_err(|_| {
        InstallError::Spec(SpecError::UnknownResourceFlowProperty {
            property: format!("{} flow columns", arena.name),
        })
    })?;
    let allocator_weight_col = Some(cols.weight_col);

    let nodes = build_weighted_need_nodes(&input_cols, &input_literals, &weight_cols);

    Ok(ResolvedNeedWeightProfile {
        id: binding.id.clone(),
        profile: binding.profile.clone(),
        participant_slot,
        hosted_id,
        arena: arena.name.clone(),
        arena_idx: arena_idx as u32,
        need_col,
        allocator_weight_col,
        nodes,
        weight_seeds: binding.weight_seeds.clone(),
        weight_cols,
        input_cols,
        input_literals,
        threshold: binding.threshold.clone(),
    })
}

fn ensure_need_host_property(
    binding: &NeedWeightProfileBindingSpec,
    registry: &mut DimensionRegistry,
) -> Result<SimPropertyId, InstallError> {
    let name = format!("need_{}", binding.id);
    if let Some(id) = registry.id_of(NEED_PROPERTY_NAMESPACE, &name) {
        return Ok(id);
    }
    let n_inputs = binding.inputs.len();
    let n_weights = binding.weight_seeds.len();
    let mut sub_fields = vec![SubFieldSpec {
        role: SubFieldRole::Amount,
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: NEED_AMOUNT_ROLE.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }];
    for i in 0..n_inputs {
        sub_fields.push(SubFieldSpec {
            role: SubFieldRole::Named(format!("in_{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("in_{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        });
    }
    for i in 0..n_weights {
        sub_fields.push(SubFieldSpec {
            role: SubFieldRole::Named(format!("w_{i}")),
            width: 1,
            clamp: ClampBehavior::Unbounded,
            velocity_max: None,
            default: 0.0,
            display_name: format!("w_{i}"),
            display_range: None,
            governed_by: None,
            reduction_override: None,
            soft_aggregate_guard: None,
            accumulator_spec: None,
        });
    }
    let spec = PropertySpec {
        id: format!("{NEED_PROPERTY_NAMESPACE}_{name}"),
        namespace: NEED_PROPERTY_NAMESPACE.into(),
        name,
        display_name: format!("need {}", binding.id),
        description: format!("RF-5 need host for weight_profile `{}`", binding.profile),
        sub_fields,
    };
    let (id, _) = compile_property(&spec, registry)?;
    Ok(id)
}

fn build_weighted_need_nodes(
    input_cols: &[u32],
    input_literals: &[Option<f32>],
    weight_cols: &[u32],
) -> Vec<EmlNodeGpu> {
    let mut nodes = Vec::new();
    for (i, (in_col, w_col)) in input_cols.iter().zip(weight_cols.iter()).enumerate() {
        match input_literals.get(i).and_then(|x| *x) {
            Some(lit) => nodes.push(literal(lit)),
            None => nodes.push(slot_value(*in_col)),
        }
        nodes.push(slot_value(*w_col));
        nodes.push(op_node(eml_nodes::opcode::MUL));
    }
    for _ in 1..input_cols.len().max(1) {
        if input_cols.len() > 1 {
            nodes.push(op_node(eml_nodes::opcode::ADD));
        }
    }
    // Single input: no ADD needed. Zero inputs impossible (validated earlier).
    if input_cols.len() > 1 {
        // ADDs already folded above for pairs; for n inputs we need n-1 ADDs.
        // The loop above runs 1..n which is n-1 times — correct.
    }
    nodes.push(op_node(eml_nodes::opcode::RETURN_TOP));
    nodes
}

/// Seed authored weight (and literal input) columns on the need host; attach
/// the need property to the participant if missing.
pub fn seed_need_weight_profiles(
    resolved: &[ResolvedNeedWeightProfile],
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &simthing_gpu::SlotAllocator,
) -> Result<(), InstallError> {
    for binding in resolved {
        let participant_id = allocator
            .owner_of(SlotIndex::new(binding.participant_slot))
            .ok_or_else(|| InstallError::NeedWeightProfileParticipantSlotMissing {
                binding: binding.id.clone(),
                slot: binding.participant_slot,
            })?;
        let Some(node) = find_simthing_mut(root, participant_id) else {
            return Err(InstallError::NeedWeightProfileParticipantSlotMissing {
                binding: binding.id.clone(),
                slot: binding.participant_slot,
            });
        };
        // Ensure the need host property instance exists on the participant.
        let need_pid = registry
            .column_owners
            .get(binding.need_col as usize)
            .map(|(pid, _)| *pid)
            .ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
        if !node.properties.contains_key(&need_pid) {
            let prop = registry.property(need_pid);
            node.properties.insert(need_pid, prop.default_value());
        }
        let value = node
            .properties
            .get_mut(&need_pid)
            .ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
        let layout = registry.property(need_pid).layout.clone();
        for (i, seed) in binding.weight_seeds.iter().enumerate() {
            let role = SubFieldRole::Named(format!("w_{i}"));
            let off = layout.offset_of(&role).ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
            value.set_lane_at_offset(off, *seed);
        }
        for (i, lit) in binding.input_literals.iter().enumerate() {
            if let Some(v) = lit {
                let role = SubFieldRole::Named(format!("in_{i}"));
                let off = layout.offset_of(&role).ok_or(InstallError::Spec(SpecError::ValidationFailed))?;
                value.set_lane_at_offset(off, *v);
            }
        }
    }
    Ok(())
}

/// Build EvalEML AccumulatorOps: need = Σ input×weight; optionally mirror to AllocatorWeight.
pub fn build_need_weight_profile_ops(
    resolved: &[ResolvedNeedWeightProfile],
    eml_registry: &mut EmlExpressionRegistry,
) -> Vec<AccumulatorOp> {
    let mut ops = Vec::with_capacity(resolved.len() * 2);
    for (idx, binding) in resolved.iter().enumerate() {
        let tree_id = EmlTreeId(NEED_WEIGHT_TREE_BASE + idx as u32);
        eml_registry
            .register_formula(
                tree_id,
                EmlFormulaMeta {
                    tree_id,
                    execution_class: EmlExecutionClass::ExactDeterministic,
                    allowed_consumers: EmlConsumerMask(
                        EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
                    ),
                    max_abs_error: None,
                    deterministic_gpu: true,
                    requires_guard_for_hard_threshold: false,
                    node_count: binding.nodes.len() as u32,
                    max_stack_depth: 0,
                    has_loops: false,
                    has_recursion: false,
                    display_name: format!("need_weight_profile_{}", binding.id),
                },
                binding.nodes.clone(),
            )
            .expect("need weight profile formula registers");

        // Source is a weight column (any slot value works as EvalEML entry);
        // combine EvalEML writes need.
        let source_col = binding.weight_cols.first().copied().unwrap_or(binding.need_col);
        ops.push(AccumulatorOp {
            source: SourceSpec::SlotValue {
                slot: SlotIndex::new(binding.participant_slot),
                col: ColumnIndex::new(source_col as usize),
            },
            combine: CombineFn::EvalEML { tree_id: tree_id.0 },
            gate: GateSpec::OrderBand(0),
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::ResetTarget,
            targets: vec![(
                SlotIndex::new(binding.participant_slot),
                ColumnIndex::new(binding.need_col as usize),
            )],
        });

        // Mirror need → AllocatorWeight so RF disburse sees the transported weight.
        if let Some(weight_col) = binding.allocator_weight_col {
            ops.push(AccumulatorOp {
                source: SourceSpec::SlotValue {
                    slot: SlotIndex::new(binding.participant_slot),
                    col: ColumnIndex::new(binding.need_col as usize),
                },
                combine: CombineFn::Identity,
                gate: GateSpec::OrderBand(0),
                scale: ScaleSpec::Identity,
                consume: ConsumeMode::ResetTarget,
                targets: vec![(
                    SlotIndex::new(binding.participant_slot),
                    ColumnIndex::new(weight_col as usize),
                )],
            });
        }
    }
    ops
}

/// Build threshold registrations for bindings that author a need threshold.
pub fn need_weight_threshold_registrations(
    resolved: &[ResolvedNeedWeightProfile],
) -> Vec<simthing_gpu::ThresholdRegistration> {
    use simthing_gpu::{DIR_UPWARD, THRESH_BUF_VALUES, ThresholdRegistration};
    let mut out = Vec::new();
    for binding in resolved {
        let Some(th) = &binding.threshold else {
            continue;
        };
        out.push(ThresholdRegistration {
            slot: binding.participant_slot,
            col: binding.need_col,
            threshold: th.threshold,
            direction: DIR_UPWARD,
            event_kind: th.event_kind,
            buffer: THRESH_BUF_VALUES,
        });
    }
    out
}

fn literal(value: f32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: value.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot_value(col: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn op_node(opcode: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Build a binding from a hydrated weight_profile + authored seeds (workshop/Studio helper).
pub fn binding_from_hydrated_stack(
    id: impl Into<String>,
    profile: impl Into<String>,
    stack: simthing_spec::EmlGadgetStackSpec,
    arena: impl Into<String>,
    install: InstallTargetSpec,
    weight_seeds: Vec<f32>,
    inputs: Vec<NeedWeightProfileInputSpec>,
    threshold: Option<NeedWeightProfileThresholdSpec>,
) -> NeedWeightProfileBindingSpec {
    NeedWeightProfileBindingSpec {
        id: id.into(),
        profile: profile.into(),
        stack,
        arena: arena.into(),
        install,
        weight_seeds,
        inputs,
        threshold,
    }
}
