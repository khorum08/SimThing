//! Sealed GPU POD encode for the derived STEAD anchor table (ANCHOR-TABLE-SURFACE-0).
//!
//! Typed rows live in `simthing-core` (`Option<BandIndex>` / `Option<u32>` generation).
//! i32 wire sentinels are minted only here at the governed upload boundary (4.2 law).
//! Dynamic field updates flow through admission / fused GPU writers / typed remaps —
//! never consumer reconstruction.
//!
//! The table is derived/reconstructible and must not enter wire/replay/authored
//! state; this module only encodes the live GPU observation twin.

use bytemuck::{Pod, Zeroable};
use simthing_core::{
    apply_band_crossings_to_anchor_table, AnchorIdentity, AnchorTable, AnchorTableRow, BandIndex,
    ColumnIndex, DimensionRegistry, SubFieldRole,
};

use crate::sealed::BandCrossingDelta;

/// Wire sentinel: no band crossed yet. Typed core uses `Option<BandIndex>`.
pub const ANCHOR_BAND_NONE_POD: i32 = -1;

/// Wire sentinel: no crossing generation yet. Typed core uses `Option<u32>`.
/// Distinguishes `None` from `Some(0)` (orch remand `5120847431`).
pub const ANCHOR_GENERATION_NONE_POD: i32 = -1;

/// GPU-resident POD twin of one [`AnchorTableRow`].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AnchorTableRowGpu {
    pub sim_thing_id: u32,
    pub property_id: u32,
    pub slot: u32,
    pub col: u32,
    pub band_idx: i32,
    pub last_crossing_generation: i32,
    pub urgency: f32,
    pub observed_value: f32,
}

impl AnchorTableRowGpu {
    /// Encode a typed row at the governed boundary only.
    pub fn encode(row: &AnchorTableRow) -> Self {
        Self {
            sim_thing_id: row.identity.sim_thing_id.raw(),
            property_id: row.identity.property_id.0,
            slot: row.slot.raw(),
            col: row.col.raw_u32(),
            band_idx: row
                .band
                .map(|b| b.raw() as i32)
                .unwrap_or(ANCHOR_BAND_NONE_POD),
            last_crossing_generation: row
                .last_crossing_generation
                .map(|g| g as i32)
                .unwrap_or(ANCHOR_GENERATION_NONE_POD),
            urgency: row.urgency,
            observed_value: row.observed_value,
        }
    }

    /// Decode a POD row into the typed observation row (governed boundary only).
    pub fn decode(&self, registry: &DimensionRegistry) -> AnchorTableRow {
        let property_id = simthing_core::SimPropertyId(self.property_id);
        let col = ColumnIndex::from_gpu_round_trip(self.col);
        let role = role_for_property_col(registry, property_id, col);
        let band = if self.band_idx == ANCHOR_BAND_NONE_POD {
            None
        } else {
            Some(BandIndex::new(self.band_idx as u32))
        };
        let last_crossing_generation = if self.last_crossing_generation < 0 {
            None
        } else {
            Some(self.last_crossing_generation as u32)
        };
        AnchorTableRow {
            identity: AnchorIdentity::new(
                simthing_core::SimThingId::from_session_raw(self.sim_thing_id),
                property_id,
            ),
            slot: simthing_core::SlotIndex::new(self.slot),
            col,
            role,
            band,
            last_crossing_generation,
            urgency: self.urgency,
            observed_value: self.observed_value,
        }
    }
}

fn role_for_property_col(
    registry: &DimensionRegistry,
    property_id: simthing_core::SimPropertyId,
    col: ColumnIndex,
) -> SubFieldRole {
    let Some(prop) = registry.try_property(property_id) else {
        return SubFieldRole::Amount;
    };
    let Some(range) = registry.try_column_range(property_id) else {
        return SubFieldRole::Amount;
    };
    for sf in &prop.layout.sub_fields {
        if range.col_for_role(&sf.role, &prop.layout) == Some(col) {
            return sf.role.clone();
        }
    }
    SubFieldRole::Amount
}

/// Encode the full table for GPU upload (deterministic row order preserved).
pub(crate) fn encode_anchor_table_gpu(table: &AnchorTable) -> Vec<AnchorTableRowGpu> {
    table.rows().iter().map(AnchorTableRowGpu::encode).collect()
}

/// Decode POD rows into the typed observation table.
pub(crate) fn decode_anchor_table_gpu(
    pods: &[AnchorTableRowGpu],
    registry: &DimensionRegistry,
) -> AnchorTable {
    let mut table = AnchorTable::new();
    let rows: Vec<AnchorTableRow> = pods.iter().map(|p| p.decode(registry)).collect();
    table.replace_rows(rows);
    table
}

/// Map ordered sealed band-crossing deltas onto table update triples.
pub(crate) fn band_crossing_updates_from_deltas(
    deltas: &[BandCrossingDelta],
) -> Vec<(
    AnchorIdentity,
    BandIndex,
    f32,
    Option<simthing_core::ColumnIndex>,
)> {
    let mut updates = Vec::new();
    for delta in deltas {
        let identity = AnchorIdentity::new(delta.sim_thing_id(), delta.property_id());
        let band = BandIndex::new(delta.reg_idx());
        let col = Some(delta.col());
        if let Some(existing) = updates
            .iter_mut()
            .find(|(id, _, _, c)| *id == identity && *c == col)
        {
            *existing = (identity, band, delta.post_value(), col);
        } else {
            updates.push((identity, band, delta.post_value(), col));
        }
    }
    updates
}

/// Apply sealed fused deltas to the table at the given dispatch generation.
pub(crate) fn apply_sealed_band_crossings_to_anchor_table(
    table: &mut AnchorTable,
    deltas: &[BandCrossingDelta],
    generation: u32,
) {
    let updates = band_crossing_updates_from_deltas(deltas);
    apply_band_crossings_to_anchor_table(table, &updates, generation);
}

/// Independent oracle: expected band/generation/value after applying deltas.
pub(crate) fn oracle_anchor_table_after_deltas(
    before: &AnchorTable,
    deltas: &[BandCrossingDelta],
    generation: u32,
) -> AnchorTable {
    let mut after = before.clone();
    apply_sealed_band_crossings_to_anchor_table(&mut after, deltas, generation);
    after
}

/// Mint birth-only POD seeds from registry (never from live-table readback).
pub(crate) fn birth_anchor_rows_gpu(
    sim_thing_id: simthing_core::SimThingId,
    property_id: simthing_core::SimPropertyId,
    slot: simthing_core::SlotIndex,
    to_col: ColumnIndex,
    registry: &DimensionRegistry,
) -> Vec<AnchorTableRowGpu> {
    let identity = AnchorIdentity::new(sim_thing_id, property_id);
    let mut born = Vec::new();
    let Some(prop) = registry.try_property(property_id) else {
        return born;
    };
    let Some(range) = registry.try_column_range(property_id) else {
        return born;
    };
    for sf in &prop.layout.sub_fields {
        let Some(col) = range.col_for_role(&sf.role, &prop.layout) else {
            continue;
        };
        born.push(AnchorTableRowGpu::encode(&AnchorTableRow {
            identity,
            slot,
            col,
            role: sf.role.clone(),
            band: None,
            last_crossing_generation: None,
            urgency: 0.0,
            observed_value: 0.0,
        }));
    }
    if born.is_empty() {
        born.push(AnchorTableRowGpu::encode(&AnchorTableRow {
            identity,
            slot,
            col: to_col,
            role: SubFieldRole::Amount,
            band: None,
            last_crossing_generation: None,
            urgency: 0.0,
            observed_value: 0.0,
        }));
    }
    born
}

/// Remap op kinds for the GPU-resident structural path.
pub(crate) const ANCHOR_REMAP_KIND_MOVE: u32 = 0;
pub(crate) const ANCHOR_REMAP_KIND_RETIRE: u32 = 1;

/// POD twin of one typed locus remap (move/retire). Births are separate seed rows.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub(crate) struct AnchorRemapOpGpu {
    pub sim_thing_id: u32,
    pub property_id: u32,
    pub kind: u32,
    pub from_slot: u32,
    pub from_col: u32,
    pub to_slot: u32,
    pub to_col: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub(crate) struct AnchorRemapParams {
    pub n_src_rows: u32,
    pub n_ops: u32,
    pub n_births: u32,
    pub _pad: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        AnchorIdentity, AnchorTableRow, ColumnIndex, SimPropertyId, SimThingId, SlotIndex,
        SubFieldRole,
    };

    #[test]
    fn encode_mints_sentinels_only_at_pod_boundary() {
        let row = AnchorTableRow {
            identity: AnchorIdentity::new(SimThingId::from_session_raw(3), SimPropertyId(7)),
            slot: SlotIndex::new(1),
            col: ColumnIndex::from_raw_for_oracle_or_rehearsal(2),
            role: SubFieldRole::Amount,
            band: None,
            last_crossing_generation: None,
            urgency: 0.25,
            observed_value: 3.5,
        };
        let gpu = AnchorTableRowGpu::encode(&row);
        assert_eq!(gpu.band_idx, ANCHOR_BAND_NONE_POD);
        assert_eq!(gpu.last_crossing_generation, ANCHOR_GENERATION_NONE_POD);
        assert_eq!(gpu.observed_value, 3.5);

        let gen_zero = AnchorTableRow {
            last_crossing_generation: Some(0),
            ..row.clone()
        };
        let gpu_zero = AnchorTableRowGpu::encode(&gen_zero);
        assert_eq!(gpu_zero.last_crossing_generation, 0);
        assert_ne!(gpu_zero.last_crossing_generation, ANCHOR_GENERATION_NONE_POD);

        let crossed = AnchorTableRow {
            band: Some(BandIndex::new(4)),
            last_crossing_generation: Some(9),
            ..row
        };
        let gpu2 = AnchorTableRowGpu::encode(&crossed);
        assert_eq!(gpu2.band_idx, 4);
        assert_eq!(gpu2.last_crossing_generation, 9);
    }
}
