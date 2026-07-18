//! RF-5 production composition: attach field-economy weight_profile stacks to
//! **already-authored** need-weight bindings by id equality only.
//!
//! Clause `weight_profile` carries only EML stack shape + profile kind.
//! Install / input_properties / weight_properties / threshold must appear as
//! complete `NeedWeightProfileBindingSpec` rows. No positional zip, name-stem,
//! or first-stockpile inventing.

use crate::hydrate_field_economy::HydratedFieldEconomy;
use crate::hydrate_scenario::HydratedScenarioPack;
use simthing_spec::{
    EmlGadgetInstanceSpec, NeedWeightProfileBindingSpec, ResourceFlowSpec,
};

/// Result of production need-weight composition.
#[derive(Debug, Clone, PartialEq)]
pub enum NeedWeightComposeOutcome {
    Bindings(Vec<NeedWeightProfileBindingSpec>),
    AdmissionGap {
        reason: String,
        missing_fields: Vec<&'static str>,
        weight_profile_ids: Vec<String>,
    },
}

/// Compose need-weight bindings for an arena from pack + optional pre-authored RF bindings.
pub fn compose_need_weight_bindings(
    pack: &HydratedScenarioPack,
    arena: &str,
    authored_bindings: &[NeedWeightProfileBindingSpec],
) -> NeedWeightComposeOutcome {
    let Some(economy) = pack.field_economy.as_ref() else {
        if authored_bindings.is_empty() {
            return NeedWeightComposeOutcome::Bindings(vec![]);
        }
        return attach_without_economy(arena, authored_bindings);
    };

    if authored_bindings.is_empty() {
        if economy.weight_profiles.is_empty() {
            return NeedWeightComposeOutcome::Bindings(vec![]);
        }
        return NeedWeightComposeOutcome::AdmissionGap {
            reason: "field_economy.weight_profile authors EML stack + profile kind only; \
                     install target, input_properties, weight_properties, and threshold are \
                     not present on the ClauseScript surface. Provide complete \
                     ResourceFlowSpec.need_weight_profiles (id-matched to weight_profile) \
                     before session open. Positional zip / name-stem join / first-stockpile \
                     input selection are not production authority."
                .into(),
            missing_fields: vec![
                "weight_profile.install (or companion NeedWeightProfileBindingSpec.install)",
                "weight_profile.input_properties (or companion binding.input_properties)",
                "weight_profile.weight_properties (or companion binding.weight_properties)",
                "weight_profile.threshold / event_kind (or companion binding.threshold)",
                "explicit join key weight_profile.id ↔ binding.id (not list index)",
            ],
            weight_profile_ids: economy
                .weight_profiles
                .iter()
                .map(|p| p.id.clone())
                .collect(),
        };
    }

    attach_stacks_from_economy(economy, arena, authored_bindings)
}

fn attach_without_economy(
    arena: &str,
    authored_bindings: &[NeedWeightProfileBindingSpec],
) -> NeedWeightComposeOutcome {
    let mut out = Vec::with_capacity(authored_bindings.len());
    for b in authored_bindings {
        if let Err(msg) = validate_binding_shape(b) {
            return NeedWeightComposeOutcome::AdmissionGap {
                reason: msg,
                missing_fields: vec!["complete NeedWeightProfileBindingSpec"],
                weight_profile_ids: vec![b.id.clone()],
            };
        }
        let mut b = b.clone();
        if b.arena.is_empty() {
            b.arena = arena.into();
        }
        out.push(b);
    }
    NeedWeightComposeOutcome::Bindings(out)
}

fn attach_stacks_from_economy(
    economy: &HydratedFieldEconomy,
    arena: &str,
    authored_bindings: &[NeedWeightProfileBindingSpec],
) -> NeedWeightComposeOutcome {
    let mut out = Vec::with_capacity(authored_bindings.len());
    for b in authored_bindings {
        if let Err(msg) = validate_binding_shape(b) {
            return NeedWeightComposeOutcome::AdmissionGap {
                reason: msg,
                missing_fields: vec!["input_properties", "weight_properties", "install"],
                weight_profile_ids: vec![b.id.clone()],
            };
        }
        let stack = if !b.stack.gadgets.is_empty() {
            b.stack.clone()
        } else {
            match economy.weight_profiles.iter().find(|p| p.id == b.id) {
                Some(p) => p.stack.clone(),
                None => {
                    return NeedWeightComposeOutcome::AdmissionGap {
                        reason: format!(
                            "need_weight_profiles id `{}` has empty stack and no field_economy.weight_profile with the same id",
                            b.id
                        ),
                        missing_fields: vec!["binding.id matching weight_profile.id"],
                        weight_profile_ids: economy
                            .weight_profiles
                            .iter()
                            .map(|p| p.id.clone())
                            .collect(),
                    };
                }
            }
        };
        if let Err(msg) = validate_stack_arity(&stack, b) {
            return NeedWeightComposeOutcome::AdmissionGap {
                reason: msg,
                missing_fields: vec!["input_properties/weight_properties arity vs stack"],
                weight_profile_ids: vec![b.id.clone()],
            };
        }
        let mut composed = b.clone();
        composed.stack = stack;
        if composed.arena.is_empty() {
            composed.arena = arena.into();
        }
        if composed.profile.is_empty() {
            if let Some(p) = economy.weight_profiles.iter().find(|p| p.id == b.id) {
                composed.profile = p.profile.clone();
            }
        }
        out.push(composed);
    }
    NeedWeightComposeOutcome::Bindings(out)
}

fn validate_binding_shape(b: &NeedWeightProfileBindingSpec) -> Result<(), String> {
    if b.id.trim().is_empty() {
        return Err("need_weight_profiles entry missing id".into());
    }
    if b.input_properties.is_empty() {
        return Err(format!(
            "need_weight_profiles `{}` missing input_properties",
            b.id
        ));
    }
    if b.weight_properties.is_empty() {
        return Err(format!(
            "need_weight_profiles `{}` missing weight_properties (no default-weight fallback)",
            b.id
        ));
    }
    match &b.install {
        simthing_spec::InstallTargetSpec::ScenarioListed { target_id }
            if !target_id.trim().is_empty() => {}
        other => {
            return Err(format!(
                "need_weight_profiles `{}` requires ScenarioListed install, got {other:?}",
                b.id
            ));
        }
    }
    Ok(())
}

fn validate_stack_arity(
    stack: &simthing_spec::EmlGadgetStackSpec,
    b: &NeedWeightProfileBindingSpec,
) -> Result<(), String> {
    if stack.gadgets.len() != 1 {
        return Err(format!(
            "need_weight_profiles `{}` stack must have exactly one WeightedAccumulator",
            b.id
        ));
    }
    match &stack.gadgets[0] {
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            ..
        } => {
            if input_cols.len() != b.input_properties.len()
                || weight_cols.len() != b.weight_properties.len()
            {
                return Err(format!(
                    "need_weight_profiles `{}` property arity mismatch: stack in/w={}/{} props in/w={}/{}",
                    b.id,
                    input_cols.len(),
                    weight_cols.len(),
                    b.input_properties.len(),
                    b.weight_properties.len()
                ));
            }
            Ok(())
        }
        other => Err(format!(
            "need_weight_profiles `{}` expected WeightedAccumulator, got {}",
            b.id,
            other.kind_name()
        )),
    }
}

/// Merge composed bindings into a ResourceFlowSpec.
pub fn merge_need_weight_bindings_into_resource_flow(
    flow: &mut ResourceFlowSpec,
    outcome: NeedWeightComposeOutcome,
) -> Result<(), NeedWeightComposeOutcome> {
    match outcome {
        NeedWeightComposeOutcome::Bindings(bindings) => {
            flow.need_weight_profiles = bindings;
            Ok(())
        }
        gap @ NeedWeightComposeOutcome::AdmissionGap { .. } => Err(gap),
    }
}

/// Human-readable gap line for Studio telemetry.
pub fn admission_gap_telemetry(gap: &NeedWeightComposeOutcome) -> Option<String> {
    match gap {
        NeedWeightComposeOutcome::AdmissionGap {
            reason,
            missing_fields,
            weight_profile_ids,
        } => Some(format!(
            "RF-5 admission gap: {reason} missing=[{}] profiles=[{}]",
            missing_fields.join(", "),
            weight_profile_ids.join(", ")
        )),
        NeedWeightComposeOutcome::Bindings(_) => None,
    }
}
