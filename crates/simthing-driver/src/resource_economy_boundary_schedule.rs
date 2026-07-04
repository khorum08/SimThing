//! Phase B-0 — deterministic boundary transfer schedule report (driver-only).

use crate::resource_economy_compile::ResourceEconomyRegistry;

/// Kind rank: transfer before recipe before emission/threshold upload phase.
pub const KIND_RANK_TRANSFER: u32 = 0;
pub const KIND_RANK_RECIPE: u32 = 1;

/// Stable ordering key: `(order_band, kind_rank, authoring_id)`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoundaryScheduleKey {
    pub order_band: u32,
    pub kind_rank: u32,
    pub authoring_id: String,
}

/// One boundary-scheduled registration row for diagnostics and oracle ordering.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundaryScheduleEntry {
    pub key: BoundaryScheduleKey,
    pub source_slot: u32,
    pub source_col: u32,
    pub target_slot: u32,
    pub target_col: u32,
    pub amount: f32,
}

/// Deterministic boundary schedule report sorted by [`BoundaryScheduleKey`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResourceEconomyBoundaryScheduleReport {
    pub entries: Vec<BoundaryScheduleEntry>,
}

impl ResourceEconomyBoundaryScheduleReport {
    /// Build a stable boundary schedule from a materialized registry.
    pub fn build(registry: &ResourceEconomyRegistry) -> Self {
        let report = &registry.registrations.report;
        let mut entries = Vec::new();

        for (idx, transfer) in registry.registrations.transfers.iter().enumerate() {
            let authoring_id = report
                .transfer_ids
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("transfer_{idx}"));
            entries.push(BoundaryScheduleEntry {
                key: BoundaryScheduleKey {
                    order_band: transfer.order_band,
                    kind_rank: KIND_RANK_TRANSFER,
                    authoring_id,
                },
                source_slot: transfer.source_slot.raw(),
                source_col: transfer.source_col.raw_u32(),
                target_slot: transfer.target_slot.raw(),
                target_col: transfer.target_col.raw_u32(),
                amount: transfer.amount,
            });
        }

        for (idx, _recipe) in registry.registrations.recipes.iter().enumerate() {
            let authoring_id = report
                .recipe_ids
                .get(idx)
                .cloned()
                .unwrap_or_else(|| format!("recipe_{idx}"));
            let recipe = &registry.registrations.recipes[idx];
            entries.push(BoundaryScheduleEntry {
                key: BoundaryScheduleKey {
                    order_band: 0,
                    kind_rank: KIND_RANK_RECIPE,
                    authoring_id,
                },
                source_slot: recipe.inputs.first().map(|i| i.slot.raw()).unwrap_or(0),
                source_col: recipe.inputs.first().map(|i| i.col.raw_u32()).unwrap_or(0),
                target_slot: recipe.target_slot.raw(),
                target_col: recipe.target_col.raw_u32(),
                amount: 0.0,
            });
        }

        entries.sort_by(|a, b| a.key.cmp(&b.key));
        Self { entries }
    }
}

#[cfg(test)]
mod tests {
    use simthing_core::{ColumnIndex, DiscreteTransferRegistration, SlotIndex};

    use super::*;
    use crate::resource_economy_compile::{
        ResourceEconomyMaterializationReport, ResourceEconomyRegistrations,
    };

    fn registry_with_transfers(
        transfers: Vec<DiscreteTransferRegistration>,
        ids: Vec<&str>,
    ) -> ResourceEconomyRegistry {
        ResourceEconomyRegistry {
            registrations: ResourceEconomyRegistrations {
                transfers,
                recipes: vec![],
                emissions: vec![],
                emit_on_threshold: vec![],
                report: ResourceEconomyMaterializationReport {
                    transfer_count: ids.len(),
                    transfer_ids: ids.into_iter().map(str::to_string).collect(),
                    ..Default::default()
                },
            },
            generation: 1,
        }
    }

}
