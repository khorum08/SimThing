use crate::error::CapabilityTreeError;
use crate::keys::CapabilityEntryKey;
use crate::runtime::{CapabilityTreeDefinition, CapabilityTreeState};
use simthing_core::{DimensionRegistry, OverlayId, SimPropertyId, SubFieldRole};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewDelta {
    pub property_id: SimPropertyId,
    pub role:        SubFieldRole,
    pub current:     f32,
    pub after:       f32,
    pub overlay_id:  OverlayId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewOverlayBreakdown {
    pub overlay_id: OverlayId,
    pub deltas:     Vec<CapabilityPreviewDelta>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewReport {
    pub per_overlay: Vec<CapabilityPreviewOverlayBreakdown>,
    pub combined:    Vec<CapabilityPreviewDelta>,
}

pub struct CapabilityPreviewInput<'a> {
    pub definition: &'a CapabilityTreeDefinition,
    pub state:      &'a CapabilityTreeState,
    pub registry:   &'a DimensionRegistry,
    pub shadow:     &'a [f32],
    pub n_dims:     usize,
    pub tree_slot:  u32,
    pub owner_slot: u32,
    pub entry:      CapabilityEntryKey,
}

pub fn preview_capability_effect(
    input: CapabilityPreviewInput<'_>,
) -> Result<CapabilityPreviewReport, CapabilityTreeError> {
    let entry_def = input
        .definition
        .entry(&input.entry)
        .ok_or(CapabilityTreeError::UnknownEntry(input.entry.clone()))?;

    let mut per_overlay = Vec::new();
    let mut combined_map: HashMap<(SimPropertyId, SubFieldRole), CapabilityPreviewDelta> =
        HashMap::new();

    for (overlay_id, transform) in entry_def.overlay_ids.iter().zip(&entry_def.overlay_transforms) {
        let layout = &input.registry.property(transform.property_id).layout;
        let range = input.registry.column_range(transform.property_id);
        let mut deltas = Vec::new();

        for (role, op) in &transform.sub_field_deltas {
            let Some(local) = layout.offset_of(role) else { continue };
            let col = range.start + local;
            let current = read_shadow(input.shadow, input.owner_slot, input.n_dims, col);
            let after = op.apply(current);
            let delta = CapabilityPreviewDelta {
                property_id: transform.property_id,
                role:        role.clone(),
                current,
                after,
                overlay_id:  *overlay_id,
            };
            deltas.push(delta.clone());
            combined_map
                .entry((transform.property_id, role.clone()))
                .and_modify(|existing| {
                    existing.after = op.apply(existing.after);
                    existing.overlay_id = *overlay_id;
                })
                .or_insert(delta);
        }

        per_overlay.push(CapabilityPreviewOverlayBreakdown {
            overlay_id: *overlay_id,
            deltas,
        });
    }

    let combined = combined_map.into_values().collect();
    let _ = (&input.state, input.tree_slot);
    Ok(CapabilityPreviewReport { per_overlay, combined })
}

fn read_shadow(shadow: &[f32], slot: u32, n_dims: usize, col: usize) -> f32 {
    shadow
        .get(slot as usize * n_dims + col)
        .copied()
        .unwrap_or(0.0)
}
