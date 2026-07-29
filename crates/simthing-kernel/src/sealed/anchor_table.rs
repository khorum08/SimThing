//! Sealed GPU POD encode for the derived STEAD anchor table (ANCHOR-TABLE-SURFACE-0).
//!
//! Typed rows live in `simthing-core` (`Option<BandIndex>`). The i32 wire
//! sentinel is minted only here at the governed upload boundary (4.2 law).
//! Dynamic field updates flow through admission / fused band deltas / typed
//! remaps — never consumer reconstruction.
//!
//! The table is derived/reconstructible and must not enter wire/replay/authored
//! state; this module only encodes the live GPU observation twin.

use bytemuck::{Pod, Zeroable};
use simthing_core::{
    apply_band_crossings_to_anchor_table, AnchorIdentity, AnchorTable, AnchorTableRow, BandIndex,
};

use crate::sealed::BandCrossingDelta;

/// Wire sentinel: no band crossed yet. Typed core uses `Option<BandIndex>`.
pub const ANCHOR_BAND_NONE_POD: i32 = -1;

/// GPU-resident POD twin of one [`AnchorTableRow`].
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct AnchorTableRowGpu {
    pub sim_thing_id: u32,
    pub property_id: u32,
    pub slot: u32,
    pub col: u32,
    pub band_idx: i32,
    pub last_crossing_generation: u32,
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
            last_crossing_generation: row.last_crossing_generation.unwrap_or(0),
            urgency: row.urgency,
            observed_value: row.observed_value,
        }
    }
}

/// Encode the full table for GPU upload (deterministic row order preserved).
pub fn encode_anchor_table_gpu(table: &AnchorTable) -> Vec<AnchorTableRowGpu> {
    table.rows().iter().map(AnchorTableRowGpu::encode).collect()
}

/// Map ordered sealed band-crossing deltas onto table update triples.
///
/// Band index = `reg_idx` of the winning (last) delta for each identity+col.
/// Multi-edge jumps resolve deterministically in emission order.
pub fn band_crossing_updates_from_deltas(
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
pub fn apply_sealed_band_crossings_to_anchor_table(
    table: &mut AnchorTable,
    deltas: &[BandCrossingDelta],
    generation: u32,
) {
    let updates = band_crossing_updates_from_deltas(deltas);
    apply_band_crossings_to_anchor_table(table, &updates, generation);
}

/// Independent oracle: expected band/generation/value after applying deltas.
pub fn oracle_anchor_table_after_deltas(
    before: &AnchorTable,
    deltas: &[BandCrossingDelta],
    generation: u32,
) -> AnchorTable {
    let mut after = before.clone();
    apply_sealed_band_crossings_to_anchor_table(&mut after, deltas, generation);
    after
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        AnchorIdentity, AnchorTableRow, ColumnIndex, SimPropertyId, SimThingId, SlotIndex,
        SubFieldRole,
    };

    #[test]
    fn encode_mints_sentinel_only_at_pod_boundary() {
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
        assert_eq!(gpu.last_crossing_generation, 0);
        assert_eq!(gpu.observed_value, 3.5);

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
