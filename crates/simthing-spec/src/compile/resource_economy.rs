//! Phase T-2 — Resource economy spec compile / validation.
//!
//! Resolves authored [`ResourceEconomySpec`] against the live property and EML
//! registries. Produces a driver-ready compiled artifact without materializing
//! `simthing-core` / `simthing-gpu` registration vectors (T-3).

use std::collections::HashMap;

use crate::error::SpecError;
use crate::spec::resource_economy::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, RecipeInputSpec, ResourceEconomySpec,
    ResourceEmissionSpec, ResourceRecipeSpec, ResourceTransferSpec,
};
use crate::spec::script::PropertyKey;
use crate::spec::trigger::TriggerDirection;
use simthing_core::{
    DimensionRegistry, EmlConsumerKind, EmlExecutionClass, EmlExpressionRegistry, EmlTreeId,
    SimPropertyId, SubFieldRole, ThresholdDirection,
};

/// Compiled resource economy artifact (spec-owned; not GPU registration vectors).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompiledResourceEconomy {
    pub transfers: Vec<CompiledResourceTransfer>,
    pub recipes: Vec<CompiledResourceRecipe>,
    pub emissions: Vec<CompiledResourceEmission>,
    pub emit_on_threshold: Vec<CompiledEmitOnThreshold>,
    pub report: ResourceEconomyExpansionReport,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledResourceTransfer {
    pub id: String,
    pub source_property: SimPropertyId,
    pub source_role: SubFieldRole,
    pub source_col: u32,
    pub target_property: SimPropertyId,
    pub target_role: SubFieldRole,
    pub target_col: u32,
    pub amount: f32,
    pub order_band: u32,
    /// Explicit install_targets entity for source slot (RF-5A host-qualified).
    pub source_host_entity: Option<String>,
    /// Explicit install_targets entity for target slot.
    pub target_host_entity: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledResourceRecipeInput {
    pub property: SimPropertyId,
    pub role: SubFieldRole,
    pub col: u32,
    pub unit_cost: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledResourceRecipe {
    pub id: String,
    pub inputs: Vec<CompiledResourceRecipeInput>,
    pub target_property: SimPropertyId,
    pub target_role: SubFieldRole,
    pub target_col: u32,
    pub throttle_hint_max_per_tick: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledEmissionFormula {
    IdentityFloor,
    Constant(f32),
    EvalEml {
        formula_key: String,
        tree_id: EmlTreeId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledResourceEmission {
    pub id: String,
    pub source_property: SimPropertyId,
    pub source_role: SubFieldRole,
    pub source_col: u32,
    pub formula: CompiledEmissionFormula,
    /// Explicit install_targets entity for emission source slot.
    pub host_entity: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledEmitOnThreshold {
    pub id: String,
    pub source_property: SimPropertyId,
    pub source_role: SubFieldRole,
    pub source_col: u32,
    pub threshold: f32,
    pub direction: ThresholdDirection,
    pub event_kind: u32,
    pub buffer: EmitBufferSpec,
    /// Explicit install_targets entity for the observed source slot.
    pub host_entity: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceEconomyExpansionReport {
    pub transfer_count: usize,
    pub recipe_count: usize,
    pub recipe_input_count: usize,
    pub emission_count: usize,
    pub threshold_emit_count: usize,
    pub eval_eml_emission_count: usize,
    pub diagnostics: Vec<ResourceEconomyDiagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceEconomyDiagnostic {
    pub code: &'static str,
    pub message: String,
}

type ConsumedCellKey = (u32, SimPropertyId, u32);

struct ContentionTracker {
    cells: HashMap<ConsumedCellKey, String>,
}

impl ContentionTracker {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    fn record_consumed(
        &mut self,
        order_band: u32,
        property_id: SimPropertyId,
        col: u32,
        owner: &str,
    ) -> Result<(), SpecError> {
        let key = (order_band, property_id, col);
        if let Some(first) = self.cells.get(&key) {
            return Err(SpecError::ResourceEconomyConsumedInputContention {
                property_id: property_id.0,
                col,
                order_band,
                first: first.clone(),
                second: owner.to_string(),
            });
        }
        self.cells.insert(key, owner.to_string());
        Ok(())
    }
}

/// Compile authored resource economy content against live property + EML registries.
pub fn compile_resource_economy(
    spec: &ResourceEconomySpec,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
) -> Result<CompiledResourceEconomy, SpecError> {
    validate_unique_authoring_ids(spec)?;

    let mut contention = ContentionTracker::new();
    let mut transfers = Vec::with_capacity(spec.transfers.len());
    for transfer in &spec.transfers {
        transfers.push(compile_transfer(transfer, registry, &mut contention)?);
    }

    let mut recipes = Vec::with_capacity(spec.recipes.len());
    for recipe in &spec.recipes {
        recipes.push(compile_recipe(recipe, registry, &mut contention)?);
    }

    let mut emissions = Vec::with_capacity(spec.emissions.len());
    for emission in &spec.emissions {
        emissions.push(compile_emission(emission, registry, eml_registry)?);
    }

    let mut emit_on_threshold = Vec::with_capacity(spec.emit_on_threshold.len());
    for emit in &spec.emit_on_threshold {
        emit_on_threshold.push(compile_emit_on_threshold(emit, registry)?);
    }

    let eval_eml_emission_count = emissions
        .iter()
        .filter(|e| matches!(e.formula, CompiledEmissionFormula::EvalEml { .. }))
        .count();
    let recipe_input_count: usize = recipes.iter().map(|r| r.inputs.len()).sum();

    let report = ResourceEconomyExpansionReport {
        transfer_count: transfers.len(),
        recipe_count: recipes.len(),
        recipe_input_count,
        emission_count: emissions.len(),
        threshold_emit_count: emit_on_threshold.len(),
        eval_eml_emission_count,
        diagnostics: Vec::new(),
    };

    Ok(CompiledResourceEconomy {
        transfers,
        recipes,
        emissions,
        emit_on_threshold,
        report,
    })
}

fn validate_unique_authoring_ids(spec: &ResourceEconomySpec) -> Result<(), SpecError> {
    let mut seen = HashMap::<&str, &str>::new();
    for (kind, id) in spec
        .transfers
        .iter()
        .map(|t| ("transfer", t.id.as_str()))
        .chain(spec.recipes.iter().map(|r| ("recipe", r.id.as_str())))
        .chain(spec.emissions.iter().map(|e| ("emission", e.id.as_str())))
        .chain(
            spec.emit_on_threshold
                .iter()
                .map(|e| ("emit_on_threshold", e.id.as_str())),
        )
    {
        if let Some(first) = seen.insert(id, kind) {
            let _ = first;
            return Err(SpecError::DuplicateResourceEconomyId { id: id.to_string() });
        }
    }
    Ok(())
}

fn compile_transfer(
    transfer: &ResourceTransferSpec,
    registry: &DimensionRegistry,
    contention: &mut ContentionTracker,
) -> Result<CompiledResourceTransfer, SpecError> {
    if !transfer.amount.is_finite() || transfer.amount < 0.0 {
        return Err(SpecError::InvalidTransferAmount {
            transfer: transfer.id.clone(),
        });
    }

    let (source_property, source_col) = resolve_property_col(
        registry,
        &transfer.source,
        &transfer.source_role,
        &format!("transfer `{}` source", transfer.id),
        |ns, name| SpecError::UnknownResourceEconomySourceProperty {
            transfer: transfer.id.clone(),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    let (target_property, target_col) = resolve_property_col(
        registry,
        &transfer.target,
        &transfer.target_role,
        &format!("transfer `{}` target", transfer.id),
        |ns, name| SpecError::UnknownResourceEconomyTargetProperty {
            transfer: transfer.id.clone(),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    contention.record_consumed(
        transfer.order_band,
        source_property,
        source_col,
        &transfer.id,
    )?;

    Ok(CompiledResourceTransfer {
        id: transfer.id.clone(),
        source_property,
        source_role: transfer.source_role.clone(),
        source_col,
        target_property,
        target_role: transfer.target_role.clone(),
        target_col,
        amount: transfer.amount,
        order_band: transfer.order_band,
        source_host_entity: transfer.source_host_entity.clone(),
        target_host_entity: transfer.target_host_entity.clone(),
    })
}

fn compile_recipe(
    recipe: &ResourceRecipeSpec,
    registry: &DimensionRegistry,
    contention: &mut ContentionTracker,
) -> Result<CompiledResourceRecipe, SpecError> {
    if recipe.inputs.is_empty() {
        return Err(SpecError::EmptyRecipeInputs {
            recipe: recipe.id.clone(),
        });
    }
    if recipe.throttle_hint_max_per_tick == 0 {
        return Err(SpecError::InvalidRecipeThrottleHint {
            recipe: recipe.id.clone(),
        });
    }

    let (target_property, target_col) = resolve_property_col(
        registry,
        &recipe.target,
        &recipe.target_role,
        &format!("recipe `{}` target", recipe.id),
        |ns, name| SpecError::UnknownResourceEconomyProperty {
            context: format!("recipe `{}` target", recipe.id),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    let mut inputs = Vec::with_capacity(recipe.inputs.len());
    for (idx, input) in recipe.inputs.iter().enumerate() {
        inputs.push(compile_recipe_input(
            recipe, idx, input, registry, contention,
        )?);
    }

    Ok(CompiledResourceRecipe {
        id: recipe.id.clone(),
        inputs,
        target_property,
        target_role: recipe.target_role.clone(),
        target_col,
        throttle_hint_max_per_tick: recipe.throttle_hint_max_per_tick,
    })
}

fn compile_recipe_input(
    recipe: &ResourceRecipeSpec,
    idx: usize,
    input: &RecipeInputSpec,
    registry: &DimensionRegistry,
    contention: &mut ContentionTracker,
) -> Result<CompiledResourceRecipeInput, SpecError> {
    if !input.unit_cost.is_finite() || input.unit_cost <= 0.0 {
        return Err(SpecError::InvalidRecipeUnitCost {
            recipe: recipe.id.clone(),
        });
    }

    let context = format!("recipe `{}` input[{idx}]", recipe.id);
    let (property, col) = resolve_property_col(
        registry,
        &input.property,
        &input.role,
        &context,
        |ns, name| SpecError::UnknownResourceEconomyProperty {
            context: context.clone(),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    const RECIPE_ORDER_BAND: u32 = 0;
    contention.record_consumed(RECIPE_ORDER_BAND, property, col, &recipe.id)?;

    Ok(CompiledResourceRecipeInput {
        property,
        role: input.role.clone(),
        col,
        unit_cost: input.unit_cost,
    })
}

fn compile_emission(
    emission: &ResourceEmissionSpec,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
) -> Result<CompiledResourceEmission, SpecError> {
    let (source_property, source_col) = resolve_property_col(
        registry,
        &emission.source,
        &emission.source_role,
        &format!("emission `{}` source", emission.id),
        |ns, name| SpecError::UnknownResourceEconomyProperty {
            context: format!("emission `{}` source", emission.id),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    let formula = match &emission.formula {
        EmissionFormulaSpec::IdentityFloor => CompiledEmissionFormula::IdentityFloor,
        EmissionFormulaSpec::Constant(value) => {
            if !value.is_finite() {
                return Err(SpecError::InvalidEmissionConstant {
                    emission: emission.id.clone(),
                });
            }
            CompiledEmissionFormula::Constant(*value)
        }
        EmissionFormulaSpec::EvalEml { formula_key } => {
            let tree_id = eml_registry
                .tree_id_by_display_name(formula_key)
                .ok_or_else(|| SpecError::UnknownEmissionFormulaKey {
                    emission: emission.id.clone(),
                    formula_key: formula_key.clone(),
                })?;
            let meta =
                eml_registry
                    .get(tree_id)
                    .ok_or_else(|| SpecError::UnknownEmissionFormulaKey {
                        emission: emission.id.clone(),
                        formula_key: formula_key.clone(),
                    })?;
            if meta.execution_class != EmlExecutionClass::ExactDeterministic {
                return Err(SpecError::EmissionEmlNotExactDeterministic {
                    emission: emission.id.clone(),
                    formula_key: formula_key.clone(),
                });
            }
            eml_registry
                .assert_consumer_admissible(tree_id, EmlConsumerKind::Emission)
                .map_err(|_| SpecError::EmissionEmlNotExactDeterministic {
                    emission: emission.id.clone(),
                    formula_key: formula_key.clone(),
                })?;
            CompiledEmissionFormula::EvalEml {
                formula_key: formula_key.clone(),
                tree_id,
            }
        }
    };

    Ok(CompiledResourceEmission {
        id: emission.id.clone(),
        source_property,
        source_role: emission.source_role.clone(),
        source_col,
        formula,
        host_entity: emission.host_entity.clone(),
    })
}

fn compile_emit_on_threshold(
    emit: &EmitOnThresholdSpec,
    registry: &DimensionRegistry,
) -> Result<CompiledEmitOnThreshold, SpecError> {
    if !emit.threshold.is_finite() {
        return Err(SpecError::InvalidThresholdEmitThreshold {
            emit: emit.id.clone(),
        });
    }

    let (source_property, source_col) = resolve_property_col(
        registry,
        &emit.source,
        &emit.source_role,
        &format!("threshold emit `{}` source", emit.id),
        |ns, name| SpecError::UnknownResourceEconomyProperty {
            context: format!("threshold emit `{}` source", emit.id),
            namespace: ns,
            name,
        },
        |ctx, prop, role| SpecError::InvalidResourceEconomyRole {
            context: ctx,
            property: prop,
            role,
        },
    )?;

    Ok(CompiledEmitOnThreshold {
        id: emit.id.clone(),
        source_property,
        source_role: emit.source_role.clone(),
        source_col,
        threshold: emit.threshold,
        direction: map_threshold_direction(emit.direction),
        event_kind: emit.event_kind,
        buffer: emit.buffer,
        host_entity: emit.host_entity.clone(),
    })
}

fn map_threshold_direction(direction: TriggerDirection) -> ThresholdDirection {
    match direction {
        TriggerDirection::Rising => ThresholdDirection::Upward,
        TriggerDirection::Falling => ThresholdDirection::Downward,
    }
}

fn resolve_property_col(
    registry: &DimensionRegistry,
    key: &PropertyKey,
    role: &SubFieldRole,
    context: &str,
    unknown_property: impl FnOnce(String, String) -> SpecError,
    invalid_role: impl FnOnce(String, String, String) -> SpecError,
) -> Result<(SimPropertyId, u32), SpecError> {
    let property_id = registry
        .id_of(&key.namespace, &key.name)
        .ok_or_else(|| unknown_property(key.namespace.clone(), key.name.clone()))?;
    let layout = &registry.property(property_id).layout;
    let range = registry.column_range(property_id);
    let col = range
        .col_for_role(role, layout)
        .ok_or_else(|| {
            invalid_role(
                context.to_string(),
                format!("{}::{}", key.namespace, key.name),
                format_role(role),
            )
        })?
        .raw_u32();
    Ok((property_id, col))
}

fn format_role(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(s) => format!("Named({s})"),
        SubFieldRole::Custom(s) => format!("Custom({s})"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        ClampBehavior, EmlConsumerMask, EmlFormulaMeta, EmlNodeGpu, PropertyLayout, SimProperty,
        SubFieldSpec,
    };

    fn register_amount_property(
        reg: &mut DimensionRegistry,
        ns: &str,
        name: &str,
    ) -> SimPropertyId {
        reg.register(SimProperty {
            namespace: ns.into(),
            name: name.into(),
            layout: PropertyLayout {
                sub_fields: vec![SubFieldSpec {
                    role: SubFieldRole::Named("amount".into()),
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "amount".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                }],
            },
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description: String::new(),
            intensity_labels: vec![],
        })
    }

    fn exact_eml(name: &str, id: u32) -> (EmlTreeId, EmlFormulaMeta, Vec<EmlNodeGpu>) {
        (
            EmlTreeId(id),
            EmlFormulaMeta {
                tree_id: EmlTreeId(id),
                execution_class: EmlExecutionClass::ExactDeterministic,
                allowed_consumers: EmlConsumerMask(
                    EmlConsumerMask::ALL_PRODUCTION | EmlConsumerMask::DEBUG_ORACLE,
                ),
                max_abs_error: None,
                deterministic_gpu: true,
                requires_guard_for_hard_threshold: false,
                node_count: 1,
                max_stack_depth: 1,
                has_loops: false,
                has_recursion: false,
                display_name: name.into(),
            },
            vec![EmlNodeGpu {
                opcode: simthing_core::eml_opcode::LITERAL_F32,
                flags: 0,
                a: 1.0_f32.to_bits(),
                b: 0,
                c: 0,
                d: 0,
            }],
        )
    }
}
