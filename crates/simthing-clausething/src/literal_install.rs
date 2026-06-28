//! CT-1a CPU overlay/property parity via existing `simthing-spec` compile path.
//!
//! **Not** the driver installed-tree proof. `LiteralInstallSnapshot` exercises
//! `compile_property` / `compile_overlay` admission plus CPU
//! `PropertyTransformDelta::apply_to_data` only. Domain-pack standalone overlays
//! are not wired through `simthing-driver::install_atomic` today (see
//! `compile_pack_properties` in `crates/simthing-driver/src/install.rs`).

use simthing_core::{DimensionRegistry, SubFieldRole};
use simthing_spec::compile::{compile_overlay, compile_property};
use simthing_spec::spec::domain_pack::DomainPackSpec;

use crate::error::HydrateError;
use crate::hydrate::HydratedEntityPack;

/// CPU overlay/property parity snapshot for CT-1a (not installed-tree parity).
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
    let mut value = registry.property(prop_id).default_value();
    value.set_role(&SubFieldRole::Amount, layout, seed_amount);

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
        overlay.transform.apply_to_data(value.raw_lanes_mut(), layout);
        final_amount = value.get_role(&SubFieldRole::Amount, layout);
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
