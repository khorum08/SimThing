//! E-10 — Resource Flow admission compiler (spec layer).
//!
//! Validates authored `ResourceFlowSpec` plus property `accumulator_spec` bindings,
//! producing a driver-ready [`CompiledResourceFlowAdmission`]. Hard rejections only;
//! warnings do not satisfy admission.

use crate::error::SpecError;
use crate::spec::resource_flow::{
    CouplingDelaySpec, CouplingSpec, FissionPolicySpec, ResourceFlowSpec, WildcardAdmissionSpec,
};
use crate::spec::script::PropertyKey;
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, DimensionRegistry, NumCountSource,
    SimPropertyId, SubFieldRole,
};
use std::collections::{HashMap, HashSet};

/// Resolved arena admission plan — input to `simthing-driver` registry materialization.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledResourceFlowAdmission {
    pub arenas: Vec<CompiledArenaAdmission>,
    pub couplings: Vec<CompiledCouplingAdmission>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledArenaAdmission {
    pub name: String,
    pub flow_property_id: SimPropertyId,
    pub balance_property_id: Option<SimPropertyId>,
    pub max_participants: u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    pub fission_policy: FissionPolicySpec,
    pub reserved_orderband_depth: u32,
    pub explicit_participants: Vec<(u32, u32)>,
    pub wildcard_max_expansion: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledCouplingAdmission {
    pub from_arena: String,
    pub to_arena: String,
    pub delay: CompiledCouplingDelay,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledCouplingDelay {
    Algebraic,
    OneTickDelay,
    BoundaryStage { stage: u32 },
    AccumulatorState { property_id: SimPropertyId },
}

/// Deterministic expansion report produced at successful admission compile + materialize.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceFlowExpansionReport {
    pub arena_count: usize,
    pub participant_count: usize,
    pub coupling_count: usize,
    pub per_arena_participant_counts: Vec<(String, u32)>,
    pub per_arena_coupling_fanout: Vec<(String, u32)>,
    pub total_registration_estimate: Option<u32>,
    pub total_orderband_depth_reserved: u32,
    pub rejected: Vec<ResourceFlowDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceFlowDiagnostic {
    pub code: &'static str,
    pub message: String,
}

/// Compile Resource Flow admission from authored spec + live property registry.
pub fn compile_resource_flow_admission(
    spec: &ResourceFlowSpec,
    registry: &DimensionRegistry,
) -> Result<CompiledResourceFlowAdmission, SpecError> {
    if spec.arenas.is_empty() {
        return Ok(CompiledResourceFlowAdmission {
            arenas: Vec::new(),
            couplings: Vec::new(),
        });
    }

    let arena_names: HashSet<&str> = spec.arenas.iter().map(|a| a.name.as_str()).collect();
    if arena_names.len() != spec.arenas.len() {
        let mut seen = HashSet::new();
        for arena in &spec.arenas {
            if !seen.insert(arena.name.clone()) {
                return Err(SpecError::DuplicateArenaName {
                    name: arena.name.clone(),
                });
            }
        }
    }
    validate_base_obligations(spec, &arena_names)?;
    validate_gated_rates(spec, &arena_names)?;

    let property_bindings = scan_property_arena_bindings(registry)?;
    validate_property_arena_references(&property_bindings, &arena_names)?;
    validate_no_property_possession_admission(&property_bindings, spec)?;
    validate_balance_num_count_sources(registry)?;

    let mut flow_property_claims: HashMap<SimPropertyId, String> = HashMap::new();
    let mut compiled_arenas = Vec::with_capacity(spec.arenas.len());

    for arena in &spec.arenas {
        let flow_property_id = resolve_property(registry, &arena.flow_property)?;
        if let Some(existing) = flow_property_claims.insert(flow_property_id, arena.name.clone()) {
            return Err(SpecError::ConflictingArenaFlowProperty {
                arena: arena.name.clone(),
                other_arena: existing,
                property: format_property_key(&arena.flow_property),
            });
        }

        let balance_property_id = match &arena.balance_property {
            None => None,
            Some(key) => Some(resolve_property(registry, key)?),
        };

        validate_wildcard(&arena.name, arena.wildcard_admission.as_ref())?;
        validate_fission_policy(&arena.name, arena.fission_policy)?;
        if arena.explicit_participants.is_empty() && arena.wildcard_admission.is_none() {
            return Err(SpecError::ImplicitParticipation {
                arena: arena.name.clone(),
            });
        }

        let explicit_participants: Vec<(u32, u32)> = arena
            .explicit_participants
            .iter()
            .map(|p| (p.slot, p.subtree_root_id))
            .collect();

        let wildcard_max_expansion = arena
            .wildcard_admission
            .as_ref()
            .and_then(|w| w.max_expansion);

        compiled_arenas.push(CompiledArenaAdmission {
            name: arena.name.clone(),
            flow_property_id,
            balance_property_id,
            max_participants: arena.max_participants,
            max_coupling_fanout: arena.max_coupling_fanout,
            max_orderband_depth: arena.max_orderband_depth,
            fission_policy: arena.fission_policy,
            reserved_orderband_depth: arena.reserved_orderband_depth,
            explicit_participants,
            wildcard_max_expansion,
        });
    }

    validate_duplicate_role_bindings(registry, &arena_names)?;

    let mut compiled_couplings = Vec::with_capacity(spec.couplings.len());
    for coupling in &spec.couplings {
        validate_coupling_arena_refs(coupling, &arena_names)?;
        compiled_couplings.push(CompiledCouplingAdmission {
            from_arena: coupling.from_arena.clone(),
            to_arena: coupling.to_arena.clone(),
            delay: compile_coupling_delay(coupling, registry)?,
        });
    }

    Ok(CompiledResourceFlowAdmission {
        arenas: compiled_arenas,
        couplings: compiled_couplings,
    })
}

/// CT-RF-EML-RATE-0: gated-rate admission. The per-(arena, install-target)
/// term count is capped so each compiled effective-rate `EvalEML` tree stays
/// within the exact-class node budget.
fn validate_gated_rates(
    spec: &ResourceFlowSpec,
    arena_names: &HashSet<&str>,
) -> Result<(), SpecError> {
    let mut ids = HashSet::new();
    // Per-(arena, install-target) EML node budget: base read + combine
    // skeleton is 4 nodes; each term costs its estimate below. Cap with
    // headroom under the 32-node exact-class tree limit.
    let mut per_target_nodes: HashMap<(String, String), u32> = HashMap::new();
    for gated in &spec.gated_rates {
        if !ids.insert(gated.id.clone()) {
            return Err(SpecError::DuplicateBaseFlowObligation {
                id: gated.id.clone(),
            });
        }
        if !gated.rate.is_finite() || gated.rate < 0.0 {
            return Err(SpecError::InvalidBaseFlowObligationRate {
                id: gated.id.clone(),
            });
        }
        if let Some(trigger) = &gated.trigger {
            if !trigger.at_least.is_finite() {
                return Err(SpecError::InvalidBaseFlowObligationRate {
                    id: gated.id.clone(),
                });
            }
        }
        let mut term_nodes = 2u32; // magnitude push + combining ADD
        if let Some(formula) = &gated.rate_formula {
            if !formula.base.is_finite() {
                return Err(SpecError::InvalidBaseFlowObligationRate {
                    id: gated.id.clone(),
                });
            }
            for op in &formula.ops {
                if let crate::spec::resource_flow::RateFormulaOperandSpec::Literal(value) =
                    &op.operand
                {
                    if !value.is_finite() {
                        return Err(SpecError::InvalidBaseFlowObligationRate {
                            id: gated.id.clone(),
                        });
                    }
                }
            }
            term_nodes += 2 * formula.ops.len() as u32;
        }
        if gated.trigger.is_some() {
            term_nodes += 4; // trigger read + threshold + CMP_GE + gate MUL
        }
        if !arena_names.contains(gated.arena.as_str()) {
            return Err(SpecError::UnknownArenaReference {
                arena: gated.arena.clone(),
                context: format!("gated_rates.{}", gated.id),
            });
        }
        let key = (gated.arena.clone(), format!("{:?}", gated.install));
        let nodes = per_target_nodes.entry(key).or_insert(4);
        *nodes += term_nodes;
        if *nodes > 28 {
            return Err(SpecError::InvalidBaseFlowObligationRate {
                id: format!(
                    "{} (effective-rate tree exceeds the EML node budget for this arena target)",
                    gated.id
                ),
            });
        }
    }
    Ok(())
}

fn validate_base_obligations(
    spec: &ResourceFlowSpec,
    arena_names: &HashSet<&str>,
) -> Result<(), SpecError> {
    let mut ids = HashSet::new();
    for obligation in &spec.base_obligations {
        if !ids.insert(obligation.id.clone()) {
            return Err(SpecError::DuplicateBaseFlowObligation {
                id: obligation.id.clone(),
            });
        }
        if !obligation.rate.is_finite() || obligation.rate < 0.0 {
            return Err(SpecError::InvalidBaseFlowObligationRate {
                id: obligation.id.clone(),
            });
        }
        if !arena_names.contains(obligation.arena.as_str()) {
            return Err(SpecError::UnknownArenaReference {
                arena: obligation.arena.clone(),
                context: format!("base_obligations.{}", obligation.id),
            });
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct PropertyArenaBinding {
    property_key: String,
    sub_field: String,
    arena: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum BindingKind {
    AllocatedFlow,
    AllocatorWeight,
}

fn scan_property_arena_bindings(
    registry: &DimensionRegistry,
) -> Result<Vec<PropertyArenaBinding>, SpecError> {
    let mut out = Vec::new();
    for prop in &registry.properties {
        let property_key = format!("{}::{}", prop.namespace, prop.name);
        for sf in &prop.layout.sub_fields {
            let Some(AccumulatorSpec { role, .. }) = &sf.accumulator_spec else {
                continue;
            };
            match role {
                AccumulatorRole::AllocatedFlow { arena } => {
                    out.push(PropertyArenaBinding {
                        property_key: property_key.clone(),
                        sub_field: format_role(&sf.role),
                        arena: arena.clone(),
                    });
                }
                AccumulatorRole::AllocatorWeight { arena } => {
                    out.push(PropertyArenaBinding {
                        property_key: property_key.clone(),
                        sub_field: format_role(&sf.role),
                        arena: arena.clone(),
                    });
                }
                AccumulatorRole::IntrinsicFlow | AccumulatorRole::Balance(_) => {}
            }
        }
    }
    Ok(out)
}

fn validate_property_arena_references(
    bindings: &[PropertyArenaBinding],
    arena_names: &HashSet<&str>,
) -> Result<(), SpecError> {
    for binding in bindings {
        if !arena_names.contains(binding.arena.as_str()) {
            return Err(SpecError::UnknownArenaRoleReference {
                arena: binding.arena.clone(),
                property: binding.property_key.clone(),
                sub_field: binding.sub_field.clone(),
            });
        }
    }
    Ok(())
}

/// Property possession of arena-scoped columns does not enroll participants.
fn validate_no_property_possession_admission(
    bindings: &[PropertyArenaBinding],
    spec: &ResourceFlowSpec,
) -> Result<(), SpecError> {
    for binding in bindings {
        let Some(arena) = spec.arenas.iter().find(|a| a.name == binding.arena) else {
            continue;
        };
        let has_explicit = !arena.explicit_participants.is_empty();
        let has_wildcard = arena.wildcard_admission.is_some();
        if !has_explicit && !has_wildcard {
            return Err(SpecError::PropertyPossessionNotArenaAdmission {
                arena: binding.arena.clone(),
                property: binding.property_key.clone(),
                sub_field: binding.sub_field.clone(),
            });
        }
    }
    Ok(())
}

fn validate_balance_num_count_sources(registry: &DimensionRegistry) -> Result<(), SpecError> {
    for prop in &registry.properties {
        let property_key = format!("{}::{}", prop.namespace, prop.name);
        for sf in &prop.layout.sub_fields {
            let Some(AccumulatorSpec {
                role:
                    AccumulatorRole::Balance(BalanceSpec {
                        num_count_source, ..
                    }),
                ..
            }) = &sf.accumulator_spec
            else {
                continue;
            };
            let Some(source) = num_count_source else {
                continue;
            };
            if let NumCountSource::Column { property_id, role } = source {
                if registry.try_property(*property_id).is_none() {
                    return Err(SpecError::UnresolvedBalanceNumCountSource {
                        property: property_key.clone(),
                        sub_field: format_role(&sf.role),
                        referenced_property_id: property_id.0,
                    });
                }
                let referenced = registry.property(*property_id);
                if referenced.layout.offset_of(role).is_none() {
                    return Err(SpecError::UnresolvedBalanceNumCountSource {
                        property: property_key.clone(),
                        sub_field: format_role(&sf.role),
                        referenced_property_id: property_id.0,
                    });
                }
            }
        }
    }
    Ok(())
}

fn validate_duplicate_role_bindings(
    registry: &DimensionRegistry,
    arena_names: &HashSet<&str>,
) -> Result<(), SpecError> {
    for prop in &registry.properties {
        let property_key = format!("{}::{}", prop.namespace, prop.name);
        let mut seen: HashMap<(String, BindingKind), String> = HashMap::new();
        for sf in &prop.layout.sub_fields {
            let Some(AccumulatorSpec { role, .. }) = &sf.accumulator_spec else {
                continue;
            };
            let (arena, kind) = match role {
                AccumulatorRole::AllocatedFlow { arena } => {
                    (arena.clone(), BindingKind::AllocatedFlow)
                }
                AccumulatorRole::AllocatorWeight { arena } => {
                    (arena.clone(), BindingKind::AllocatorWeight)
                }
                _ => continue,
            };
            if !arena_names.contains(arena.as_str()) {
                continue;
            }
            let sub_field = format_role(&sf.role);
            let key = (arena.clone(), kind);
            if let Some(prev) = seen.insert(key, sub_field.clone()) {
                return Err(SpecError::DuplicateArenaRoleBinding {
                    arena,
                    kind: match kind {
                        BindingKind::AllocatedFlow => "AllocatedFlow",
                        BindingKind::AllocatorWeight => "AllocatorWeight",
                    }
                    .into(),
                    first: format!("{property_key}::{prev}"),
                    second: format!("{property_key}::{sub_field}"),
                });
            }
        }
    }
    Ok(())
}

fn validate_wildcard(
    arena: &str,
    wildcard: Option<&WildcardAdmissionSpec>,
) -> Result<(), SpecError> {
    let Some(w) = wildcard else {
        return Ok(());
    };
    let Some(max) = w.max_expansion else {
        return Err(SpecError::UnboundedWildcardAdmission {
            arena: arena.into(),
        });
    };
    if max == 0 {
        return Err(SpecError::UnboundedWildcardAdmission {
            arena: arena.into(),
        });
    }
    if w.expanded_count > max {
        return Err(SpecError::WildcardExpansionExceedsCap {
            arena: arena.into(),
            declared: max,
            computed: w.expanded_count,
        });
    }
    Ok(())
}

fn validate_fission_policy(_arena: &str, policy: FissionPolicySpec) -> Result<(), SpecError> {
    match policy {
        FissionPolicySpec::Inherit | FissionPolicySpec::Reevaluate | FissionPolicySpec::Reject => {
            Ok(())
        }
    }
}

fn validate_coupling_arena_refs(
    coupling: &CouplingSpec,
    arena_names: &HashSet<&str>,
) -> Result<(), SpecError> {
    if !arena_names.contains(coupling.from_arena.as_str()) {
        return Err(SpecError::UnknownArenaReference {
            arena: coupling.from_arena.clone(),
            context: "coupling.from_arena".into(),
        });
    }
    if !arena_names.contains(coupling.to_arena.as_str()) {
        return Err(SpecError::UnknownArenaReference {
            arena: coupling.to_arena.clone(),
            context: "coupling.to_arena".into(),
        });
    }
    Ok(())
}

fn compile_coupling_delay(
    coupling: &CouplingSpec,
    registry: &DimensionRegistry,
) -> Result<CompiledCouplingDelay, SpecError> {
    Ok(match &coupling.delay {
        CouplingDelaySpec::Algebraic => CompiledCouplingDelay::Algebraic,
        CouplingDelaySpec::OneTickDelay => CompiledCouplingDelay::OneTickDelay,
        CouplingDelaySpec::BoundaryStage { stage } => {
            CompiledCouplingDelay::BoundaryStage { stage: *stage }
        }
        CouplingDelaySpec::AccumulatorState { property } => {
            CompiledCouplingDelay::AccumulatorState {
                property_id: resolve_property(registry, property)?,
            }
        }
    })
}

fn resolve_property(
    registry: &DimensionRegistry,
    key: &PropertyKey,
) -> Result<SimPropertyId, SpecError> {
    registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
        SpecError::UnknownResourceFlowProperty {
            property: format_property_key(key),
        }
    })
}

fn format_property_key(key: &PropertyKey) -> String {
    format!("{}::{}", key.namespace, key.name)
}

fn format_role(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(n) => format!("Named({n})"),
        SubFieldRole::Custom(n) => format!("Custom({n})"),
    }
}
