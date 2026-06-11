//! CT-1a admission/install snapshot via existing `simthing-spec` compile path.

use simthing_core::{DimensionRegistry, SubFieldRole};
use simthing_spec::compile::{compile_overlay, compile_property};
use simthing_spec::spec::domain_pack::DomainPackSpec;

use crate::error::HydrateError;
use crate::hydrate::HydratedEntityPack;

/// Canonical install snapshot for CT-1a parity.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralInstallSnapshot {
    pub property_keys: Vec<String>,
    pub overlay_specs: Vec<OverlaySpecFingerprint>,
    pub seeded_amount: f32,
    pub final_amount: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverlaySpecFingerprint {
    pub overlay_id: String,
    pub targets_property: String,
    pub sub_field_deltas: Vec<(SubFieldRole, simthing_core::TransformOp)>,
}

/// Admit and apply a hydrated pack through `compile_property` / `compile_overlay`.
pub fn admit_and_apply_pack(
    pack: &HydratedEntityPack,
) -> Result<LiteralInstallSnapshot, HydrateError> {
    admit_and_apply_domain_pack(&pack.domain_pack, pack.seed_amount)
}

pub fn admit_and_apply_domain_pack(
    pack: &DomainPackSpec,
    seed_amount: f32,
) -> Result<LiteralInstallSnapshot, HydrateError> {
    if pack.properties.is_empty() {
        return Err(HydrateError::new("domain pack has no properties"));
    }

    let mut registry = DimensionRegistry::new();
    let mut property_keys = Vec::new();

    for property in &pack.properties {
        let (_, diag) = compile_property(property, &mut registry)
            .map_err(|err| HydrateError::new(format!("compile_property failed: {err}")))?;
        if !diag.diagnostics.is_empty() {
            return Err(HydrateError::new(format!(
                "compile_property diagnostics: {:?}",
                diag.diagnostics
            )));
        }
        property_keys.push(format!("{}::{}", property.namespace, property.name));
    }

    let property = &pack.properties[0];
    let prop_id = registry
        .id_of(&property.namespace, &property.name)
        .ok_or_else(|| HydrateError::new("compiled property missing from registry"))?;
    let layout = &registry.property(prop_id).layout;
    let mut data = registry.property(prop_id).default_value().data;

    if let Some(idx) = layout.offset_of(&SubFieldRole::Amount) {
        if idx < data.len() {
            data[idx] = seed_amount;
        }
    }

    let mut final_amount = seed_amount;
    for overlay_spec in &pack.overlays {
        let (overlay, diag) = compile_overlay(overlay_spec, &registry)
            .map_err(|err| HydrateError::new(format!("compile_overlay failed: {err}")))?;
        if !diag.diagnostics.is_empty() {
            return Err(HydrateError::new(format!(
                "compile_overlay diagnostics: {:?}",
                diag.diagnostics
            )));
        }
        overlay.transform.apply_to_data(&mut data, layout);
        if let Some(idx) = layout.offset_of(&SubFieldRole::Amount) {
            if idx < data.len() {
                final_amount = data[idx];
            }
        }
    }

    let overlay_specs = pack
        .overlays
        .iter()
        .map(|spec| OverlaySpecFingerprint {
            overlay_id: spec.id.clone(),
            targets_property: spec.targets_property.clone(),
            sub_field_deltas: spec.sub_field_deltas.clone(),
        })
        .collect();

    Ok(LiteralInstallSnapshot {
        property_keys,
        overlay_specs,
        seeded_amount: seed_amount,
        final_amount,
    })
}
