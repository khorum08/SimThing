use crate::boundary::CapabilityTreeError;
use crate::keys::{CapabilityEffectKey, CapabilityEntryKey};
use crate::runtime::{CapabilityTreeDefinition, CapabilityTreeState};
use simthing_core::{DimensionRegistry, OverlayId, SimPropertyId, SubFieldRole, TransformOp};

pub struct CapabilityPreviewInput<'a> {
    pub definition: &'a CapabilityTreeDefinition,
    pub state: &'a CapabilityTreeState,
    pub registry: &'a DimensionRegistry,
    pub shadow: &'a [f32],
    pub n_dims: usize,
    pub tree_slot: u32,
    pub entry: CapabilityEntryKey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewDelta {
    pub property_id: SimPropertyId,
    pub role: SubFieldRole,
    pub overlay_id: OverlayId,
    pub current: f32,
    pub after: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewOverlayBreakdown {
    pub overlay_id: OverlayId,
    pub effect_key: CapabilityEffectKey,
    pub deltas: Vec<CapabilityPreviewDelta>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CapabilityPreviewReport {
    pub per_overlay: Vec<CapabilityPreviewOverlayBreakdown>,
    pub combined: Vec<CapabilityPreviewDelta>,
}

pub fn preview_capability_effect(
    input: CapabilityPreviewInput<'_>,
) -> Result<CapabilityPreviewReport, CapabilityTreeError> {
    let entry = input
        .definition
        .entries
        .get(&input.entry)
        .ok_or_else(|| CapabilityTreeError::EntryNotInTree(input.entry.to_string()))?;

    let mut per_overlay = Vec::new();
    let mut combined = Vec::<CapabilityPreviewDelta>::new();

    for ((overlay_id, effect_key), transform) in entry
        .overlay_ids
        .iter()
        .zip(entry.effect_keys.iter())
        .zip(entry.effect_transforms.iter())
    {
        if !input.registry.is_active(transform.property_id) {
            continue;
        }
        let layout = &input.registry.property(transform.property_id).layout;
        let range = input.registry.column_range(transform.property_id);
        let mut deltas = Vec::new();

        for (role, op) in &transform.sub_field_deltas {
            let Some(col) = range.col_for_role(role, layout) else {
                continue;
            };
            let idx = input.tree_slot as usize * input.n_dims + col;
            let Some(current) = input.shadow.get(idx).copied() else {
                continue;
            };
            let after = op.apply(current);
            let delta = CapabilityPreviewDelta {
                property_id: transform.property_id,
                role: role.clone(),
                overlay_id: *overlay_id,
                current,
                after,
            };
            push_combined(&mut combined, delta.clone(), op);
            deltas.push(delta);
        }

        per_overlay.push(CapabilityPreviewOverlayBreakdown {
            overlay_id: *overlay_id,
            effect_key: effect_key.clone(),
            deltas,
        });
    }

    Ok(CapabilityPreviewReport {
        per_overlay,
        combined,
    })
}

fn push_combined(
    combined: &mut Vec<CapabilityPreviewDelta>,
    delta: CapabilityPreviewDelta,
    op: &TransformOp,
) {
    if let Some(existing) = combined
        .iter_mut()
        .find(|existing| existing.property_id == delta.property_id && existing.role == delta.role)
    {
        existing.after = op.apply(existing.after);
    } else {
        combined.push(delta);
    }
}
