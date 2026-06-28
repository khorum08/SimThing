//! CT-3b+4a: arena-to-cell pressure projection.
//!
//! Projects admitted Resource Flow arena participant output (resolved GPU
//! session values) into region-field seed cells per an authored
//! [`ArenaPressureBindingSpec`]. Boundary-time *consumption* of resolved
//! state — never recomputation, never a side-channel map: every seed value
//! is read from the installed participant's flow column.

use simthing_core::DimensionRegistry;
use simthing_spec::{ArenaPressureBindingSpec, PressureSourceSpec};
use thiserror::Error;

use crate::arena_hierarchy::resolve_node_columns;
use crate::arena_participant::ArenaParticipantScaffold;
use crate::arena_registry::ArenaRegistry;
use crate::first_slice_mapping_runtime::FirstSliceSeed;
use crate::scenario::Scenario;

#[derive(Debug, Error)]
pub enum ArenaPressureError {
    #[error("pressure binding names unknown arena `{arena}`")]
    UnknownArena { arena: String },

    #[error("pressure binding target `{target_id}` is not a scenario install target")]
    UnknownTarget { target_id: String },

    #[error("pressure binding target `{target_id}` resolved to no admitted participant in arena `{arena}`")]
    TargetNotAdmitted { target_id: String, arena: String },

    #[error("pressure binding column resolution failed for arena `{arena}`: {reason}")]
    ColumnResolution { arena: String, reason: String },

    #[error("projected pressure for target `{target_id}` is non-finite")]
    NonFinitePressure { target_id: String },
}

/// Resolve every placement to its admitted participant slot + flow column and
/// read the projected pressure value out of `values`. A target id resolving
/// to multiple admitted participants sums their contributions
/// (deterministic install-target order). Bounds on `(row, col)` were already
/// admitted by `compile_region_field_preview`.
pub fn project_arena_pressure_seeds(
    binding: &ArenaPressureBindingSpec,
    scenario: &Scenario,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    scaffold: &ArenaParticipantScaffold,
    values: &[f32],
    n_dims: u32,
) -> Result<Vec<FirstSliceSeed>, ArenaPressureError> {
    let (arena_idx, descriptor) = arena_registry
        .arenas
        .iter()
        .enumerate()
        .find(|(_, arena)| arena.name == binding.arena)
        .ok_or_else(|| ArenaPressureError::UnknownArena {
            arena: binding.arena.clone(),
        })?;

    let layout = &registry.property(descriptor.flow_property_id).layout;
    let cols = resolve_node_columns(layout, &binding.arena).map_err(|e| {
        ArenaPressureError::ColumnResolution {
            arena: binding.arena.clone(),
            reason: format!("{e:?}"),
        }
    })?;
    let local_col = match &binding.source {
        PressureSourceSpec::IntrinsicFlow => cols.intrinsic_flow_col,
        PressureSourceSpec::AllocatedFlow => cols.allocated_flow_col,
        // The gadget composition hook: any named column an EML/gadget op
        // writes on the flow property is projectable heatmap feedstock.
        PressureSourceSpec::Named { sub_field } => layout
            .offset_of(&simthing_core::SubFieldRole::Named(sub_field.clone()))
            .ok_or_else(|| ArenaPressureError::ColumnResolution {
                arena: binding.arena.clone(),
                reason: format!("named sub-field `{sub_field}` not in flow layout"),
            })?
            .lane() as u32,
    };
    let global_col = registry.column_range(descriptor.flow_property_id).start as u32 + local_col;

    let mut seeds = Vec::with_capacity(binding.placements.len());
    for placement in &binding.placements {
        let hosted = scenario
            .install_targets
            .get(&placement.target_id)
            .filter(|ids| !ids.is_empty())
            .ok_or_else(|| ArenaPressureError::UnknownTarget {
                target_id: placement.target_id.clone(),
            })?;

        let mut pressure = 0.0f32;
        let mut resolved_any = false;
        for hosted_id in hosted {
            let Some(slot) = scaffold
                .index
                .participant_slot(*hosted_id, arena_idx as u32)
            else {
                continue;
            };
            pressure += values[(slot * n_dims + global_col) as usize];
            resolved_any = true;
        }
        if !resolved_any {
            return Err(ArenaPressureError::TargetNotAdmitted {
                target_id: placement.target_id.clone(),
                arena: binding.arena.clone(),
            });
        }
        if !pressure.is_finite() {
            return Err(ArenaPressureError::NonFinitePressure {
                target_id: placement.target_id.clone(),
            });
        }
        seeds.push(FirstSliceSeed {
            row: placement.row,
            col: placement.col,
            value: pressure,
        });
    }
    Ok(seeds)
}

/// Compile the binding to on-device scatter entries: session values buffer
/// index → stencil input buffer index. The GPU path never reads values back
/// to the host; the 0A CPU path above is its oracle. v1 limit: one admitted
/// participant per placement (summing multiple sources needs a staging
/// column written by a session EML op — the gadget hook — then a `Named`
/// projection of that column).
pub fn compile_arena_pressure_scatter(
    binding: &ArenaPressureBindingSpec,
    scenario: &Scenario,
    registry: &DimensionRegistry,
    arena_registry: &ArenaRegistry,
    scaffold: &ArenaParticipantScaffold,
    session_n_dims: u32,
    field: &simthing_spec::RegionFieldSpec,
) -> Result<(Vec<simthing_gpu::ScatterEntry>, Vec<(u32, u32)>), ArenaPressureError> {
    let (arena_idx, descriptor) = arena_registry
        .arenas
        .iter()
        .enumerate()
        .find(|(_, arena)| arena.name == binding.arena)
        .ok_or_else(|| ArenaPressureError::UnknownArena {
            arena: binding.arena.clone(),
        })?;
    let layout = &registry.property(descriptor.flow_property_id).layout;
    let cols = resolve_node_columns(layout, &binding.arena).map_err(|e| {
        ArenaPressureError::ColumnResolution {
            arena: binding.arena.clone(),
            reason: format!("{e:?}"),
        }
    })?;
    let local_col = match &binding.source {
        PressureSourceSpec::IntrinsicFlow => cols.intrinsic_flow_col,
        PressureSourceSpec::AllocatedFlow => cols.allocated_flow_col,
        PressureSourceSpec::Named { sub_field } => layout
            .offset_of(&simthing_core::SubFieldRole::Named(sub_field.clone()))
            .ok_or_else(|| ArenaPressureError::ColumnResolution {
                arena: binding.arena.clone(),
                reason: format!("named sub-field `{sub_field}` not in flow layout"),
            })?
            .lane() as u32,
    };
    let global_col = registry.column_range(descriptor.flow_property_id).start as u32 + local_col;

    let mut entries = Vec::with_capacity(binding.placements.len());
    let mut cells = Vec::with_capacity(binding.placements.len());
    for placement in &binding.placements {
        let hosted = scenario
            .install_targets
            .get(&placement.target_id)
            .filter(|ids| !ids.is_empty())
            .ok_or_else(|| ArenaPressureError::UnknownTarget {
                target_id: placement.target_id.clone(),
            })?;
        let mut slots = hosted
            .iter()
            .filter_map(|id| scaffold.index.participant_slot(*id, arena_idx as u32));
        let Some(slot) = slots.next() else {
            return Err(ArenaPressureError::TargetNotAdmitted {
                target_id: placement.target_id.clone(),
                arena: binding.arena.clone(),
            });
        };
        if slots.next().is_some() {
            return Err(ArenaPressureError::ColumnResolution {
                arena: binding.arena.clone(),
                reason: format!(
                    "target `{}` resolves to multiple participants; GPU scatter takes one source per cell — stage a summed column via a session EML op and project it as Named",
                    placement.target_id
                ),
            });
        }
        let cell = placement.row * field.grid_size + placement.col;
        entries.push(simthing_gpu::ScatterEntry {
            src_index: slot * session_n_dims + global_col,
            dst_index: cell * field.n_dims + field.source_col,
        });
        cells.push((placement.row, placement.col));
    }
    Ok((entries, cells))
}
