//! Phase M — resource economy authoring preview and admission diagnostics.
//!
//! Spec/admission/reporting only: summarizes admitted `ResourceEconomySpec` content
//! for designers and agents. Does not execute economy at runtime.

use crate::compile::resource_economy::{
    compile_resource_economy, CompiledResourceEconomy, ResourceEconomyDiagnostic,
};
use crate::error::SpecError;
use crate::spec::game_mode::GameModeSpec;
use crate::spec::resource_economy::{ResourceEconomyOptInMode, ResourceEconomySpec};
use crate::spec::resource_flow::ResourceFlowOptInMode;
use crate::spec::script::PropertyKey;
use simthing_core::{DimensionRegistry, EmlExpressionRegistry, SubFieldRole};
use std::collections::{BTreeMap, BTreeSet};

/// Diagnostic-only preview of an admitted resource economy spec.
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceEconomyAuthoringPreview {
    pub report: ResourceEconomyPreviewReport,
    pub compiled: CompiledResourceEconomy,
}

/// Human/agent-readable summary of authored resource economy content.
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceEconomyPreviewReport {
    pub opt_in_mode: ResourceEconomyOptInMode,
    pub transfer_count: usize,
    pub recipe_count: usize,
    pub emission_count: usize,
    pub threshold_emit_count: usize,
    pub order_bands: Vec<u32>,
    pub resources_bound: Vec<ResourceBindingPreview>,
    pub transfers: Vec<TransferPreview>,
    pub recipes: Vec<RecipePreview>,
    pub threshold_emits: Vec<ThresholdEmitPreview>,
    pub warnings: Vec<ResourceEconomyDiagnostic>,
    pub resource_flow_enabled: bool,
    pub simple_static_nets: Vec<StaticPropertyNetPreview>,
    /// Designer-facing one-line schedule descriptions for every authored transfer,
    /// recipe, and threshold emission. Pure preview ergonomics helper (R2);
    /// derived only from admitted authoring data. No runtime semantics.
    pub schedule_lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceBindingPreview {
    pub namespace: String,
    pub name: String,
    pub role: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransferPreview {
    pub id: String,
    pub source: PropertyKey,
    pub source_role: SubFieldRole,
    pub target: PropertyKey,
    pub target_role: SubFieldRole,
    pub amount: f32,
    pub order_band: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecipePreview {
    pub id: String,
    pub target: PropertyKey,
    pub target_role: SubFieldRole,
    pub input_count: usize,
    pub throttle_hint_max_per_tick: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThresholdEmitPreview {
    pub id: String,
    pub source: PropertyKey,
    pub source_role: SubFieldRole,
    pub threshold: f32,
    pub event_kind: u32,
}

/// Static transfer-only net per property/role for one boundary (diagnostic metadata).
#[derive(Clone, Debug, PartialEq)]
pub struct StaticPropertyNetPreview {
    pub namespace: String,
    pub name: String,
    pub role: String,
    pub net_per_boundary: f32,
}

/// Compile and preview an authored resource economy spec against live registries.
pub fn compile_resource_economy_authoring_preview(
    spec: &ResourceEconomySpec,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
    resource_flow_enabled: bool,
) -> Result<ResourceEconomyAuthoringPreview, SpecError> {
    let compiled = compile_resource_economy(spec, registry, eml_registry)?;
    let report = build_preview_report(spec, &compiled, resource_flow_enabled);
    Ok(ResourceEconomyAuthoringPreview { report, compiled })
}

/// Compile properties + resource economy from a game mode into an authoring preview.
pub fn compile_game_mode_resource_economy_authoring_preview(
    game_mode: &GameModeSpec,
    eml_registry: &EmlExpressionRegistry,
) -> Result<ResourceEconomyAuthoringPreview, SpecError> {
    let economy =
        game_mode
            .resource_economy
            .as_ref()
            .ok_or_else(|| SpecError::ResourceEconomyAdmission {
                reason: format!("game mode `{}` has no resource_economy block", game_mode.id),
            })?;

    let mut registry = DimensionRegistry::new();
    for property in &game_mode.properties {
        crate::compile::property::compile_property(property, &mut registry)?;
    }

    let resource_flow_enabled = resource_flow_enabled_for_game_mode(game_mode);
    compile_resource_economy_authoring_preview(
        economy,
        &registry,
        eml_registry,
        resource_flow_enabled,
    )
}

fn resource_flow_enabled_for_game_mode(game_mode: &GameModeSpec) -> bool {
    let spec_opt_in = game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.opt_in_mode)
        .unwrap_or(ResourceFlowOptInMode::Disabled);
    if spec_opt_in == ResourceFlowOptInMode::FlatStarOptIn {
        return true;
    }
    game_mode
        .resource_flow_execution_profile
        .enables_flat_star_resource_flow()
}

fn build_preview_report(
    spec: &ResourceEconomySpec,
    compiled: &CompiledResourceEconomy,
    resource_flow_enabled: bool,
) -> ResourceEconomyPreviewReport {
    let mut order_bands = BTreeSet::new();
    let mut bindings = BTreeSet::new();

    let transfers: Vec<TransferPreview> = spec
        .transfers
        .iter()
        .map(|t| {
            order_bands.insert(t.order_band);
            record_binding(&mut bindings, &t.source, &t.source_role);
            record_binding(&mut bindings, &t.target, &t.target_role);
            TransferPreview {
                id: t.id.clone(),
                source: t.source.clone(),
                source_role: t.source_role.clone(),
                target: t.target.clone(),
                target_role: t.target_role.clone(),
                amount: t.amount,
                order_band: t.order_band,
            }
        })
        .collect();

    let recipes: Vec<RecipePreview> = spec
        .recipes
        .iter()
        .map(|r| {
            record_binding(&mut bindings, &r.target, &r.target_role);
            for input in &r.inputs {
                record_binding(&mut bindings, &input.property, &input.role);
            }
            RecipePreview {
                id: r.id.clone(),
                target: r.target.clone(),
                target_role: r.target_role.clone(),
                input_count: r.inputs.len(),
                throttle_hint_max_per_tick: r.throttle_hint_max_per_tick,
            }
        })
        .collect();

    for emission in &spec.emissions {
        record_binding(&mut bindings, &emission.source, &emission.source_role);
    }

    let threshold_emits: Vec<ThresholdEmitPreview> = spec
        .emit_on_threshold
        .iter()
        .map(|e| {
            record_binding(&mut bindings, &e.source, &e.source_role);
            ThresholdEmitPreview {
                id: e.id.clone(),
                source: e.source.clone(),
                source_role: e.source_role.clone(),
                threshold: e.threshold,
                event_kind: e.event_kind,
            }
        })
        .collect();

    let simple_static_nets = compute_simple_static_nets(&transfers);

    let mut schedule_lines: Vec<String> = Vec::new();
    for t in &transfers {
        let role_str = role_label(&t.target_role);
        schedule_lines.push(format!(
            "{}: {:+.1} {}/{} @ order_band {} (transfer)",
            t.id, t.amount, t.target.namespace, role_str, t.order_band
        ));
    }
    for r in &recipes {
        let role_str = role_label(&r.target_role);
        schedule_lines.push(format!(
            "{}: recipe -> {}/{} ({} inputs, throttle_hint={})",
            r.id, r.target.namespace, role_str, r.input_count, r.throttle_hint_max_per_tick
        ));
    }
    for e in &threshold_emits {
        let role_str = role_label(&e.source_role);
        schedule_lines.push(format!(
            "{}: threshold {:.1} -> event_kind {} (source {}/{})",
            e.id, e.threshold, e.event_kind, e.source.namespace, role_str
        ));
    }

    ResourceEconomyPreviewReport {
        opt_in_mode: spec.opt_in_mode,
        transfer_count: compiled.transfers.len(),
        recipe_count: compiled.recipes.len(),
        emission_count: compiled.emissions.len(),
        threshold_emit_count: compiled.emit_on_threshold.len(),
        order_bands: order_bands.into_iter().collect(),
        resources_bound: bindings.into_iter().collect(),
        transfers,
        recipes,
        threshold_emits,
        warnings: compiled.report.diagnostics.clone(),
        resource_flow_enabled,
        simple_static_nets,
        schedule_lines,
    }
}

fn record_binding(
    set: &mut BTreeSet<ResourceBindingPreview>,
    key: &PropertyKey,
    role: &SubFieldRole,
) {
    set.insert(ResourceBindingPreview {
        namespace: key.namespace.clone(),
        name: key.name.clone(),
        role: role_label(role),
    });
}

fn role_label(role: &SubFieldRole) -> String {
    match role {
        SubFieldRole::Amount => "Amount".into(),
        SubFieldRole::Velocity => "Velocity".into(),
        SubFieldRole::Intensity => "Intensity".into(),
        SubFieldRole::Named(s) => s.clone(),
        SubFieldRole::Custom(s) => s.clone(),
    }
}

fn compute_simple_static_nets(transfers: &[TransferPreview]) -> Vec<StaticPropertyNetPreview> {
    let mut nets: BTreeMap<(String, String, String), f32> = BTreeMap::new();

    for transfer in transfers {
        let source_key = (
            transfer.source.namespace.clone(),
            transfer.source.name.clone(),
            role_label(&transfer.source_role),
        );
        let target_key = (
            transfer.target.namespace.clone(),
            transfer.target.name.clone(),
            role_label(&transfer.target_role),
        );
        *nets.entry(source_key).or_default() -= transfer.amount;
        *nets.entry(target_key).or_default() += transfer.amount;
    }

    nets.into_iter()
        .filter(|(_, net)| *net != 0.0)
        .map(
            |((namespace, name, role), net_per_boundary)| StaticPropertyNetPreview {
                namespace,
                name,
                role,
                net_per_boundary,
            },
        )
        .collect()
}
