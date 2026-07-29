//! Derived STEAD anchor table (ANCHOR-TABLE-SURFACE-0).
//!
//! One observation surface for admitted Anchored loci. Writers are admission,
//! the fused write door (via sealed band-crossing evidence), and typed remaps.
//! Consumers read typed rows only — never reconstruct bands from raw matrices.
//!
//! ## Wire / replay fence
//!
//! This table is **derived and reconstructible**. It must never enter wire,
//! replay, or authored state. Serde derives exist only for in-process test
//! snapshots — not for `BoundaryDeltaEntry`, replay frames, or scenario bytes.

use crate::anchor_remap::{AnchorLocusRemap, AnchorRemapSection, AnchoredLocusMap};
use crate::column_index::ColumnIndex;
use crate::ids::{SimPropertyId, SimThingId};
use crate::property::{PropertyAdmissionDisposition, SubFieldRole};
use crate::registry::DimensionRegistry;
use crate::simthing::SimThing;
use crate::slot_index::SlotIndex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Ordered threshold-edge index for one locus (typed; no wire sentinel).
///
/// The i32 `ANCHOR_BAND_NONE` sentinel exists only on the GPU POD twin at the
/// governed encode boundary (4.2 typed-to-the-boundary law).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BandIndex(u32);

impl BandIndex {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Stable observation identity for one Anchored store locus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct AnchorIdentity {
    pub sim_thing_id: SimThingId,
    pub property_id: SimPropertyId,
}

impl AnchorIdentity {
    pub fn new(sim_thing_id: SimThingId, property_id: SimPropertyId) -> Self {
        Self {
            sim_thing_id,
            property_id,
        }
    }
}

/// One derived anchor-table row (typed; GPU POD encode is a separate boundary).
///
/// DA sharpening (comment 5120052669): no admitted falloff fields in this rung —
/// no in-rung consumer; 5.4+ carries that as authored EML map data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnchorTableRow {
    pub identity: AnchorIdentity,
    pub slot: SlotIndex,
    pub col: ColumnIndex,
    pub role: SubFieldRole,
    /// Current band after fused crossings; `None` until first crossing.
    pub band: Option<BandIndex>,
    /// Generation stamped on the last fused band crossing (`None` = never).
    pub last_crossing_generation: Option<u32>,
    /// Writer-side distance to nearest known threshold edge.
    pub urgency: f32,
    /// Magnitude stamped by admission / fused write door (not consumer matrix scan).
    pub observed_value: f32,
}

/// Deterministic ordered STEAD observation table.
///
/// Not wire/replay authority — reconstruct from admission + remaps + fused deltas.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AnchorTable {
    rows: Vec<AnchorTableRow>,
}

impl AnchorTable {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn rows(&self) -> &[AnchorTableRow] {
        &self.rows
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn get(&self, identity: AnchorIdentity) -> Option<&AnchorTableRow> {
        self.rows.iter().find(|r| r.identity == identity)
    }

    pub fn get_mut(&mut self, identity: AnchorIdentity) -> Option<&mut AnchorTableRow> {
        self.rows.iter_mut().find(|r| r.identity == identity)
    }

    pub fn get_by_identity_role(
        &self,
        identity: AnchorIdentity,
        role: &SubFieldRole,
    ) -> Option<&AnchorTableRow> {
        self.rows
            .iter()
            .find(|r| r.identity == identity && &r.role == role)
    }

    pub fn get_by_slot_col(&self, slot: SlotIndex, col: ColumnIndex) -> Option<&AnchorTableRow> {
        self.rows
            .iter()
            .find(|r| r.slot == slot && r.col == col)
    }

    /// Replace rows with a deterministically ordered mint from live Anchored loci.
    pub fn replace_rows(&mut self, rows: Vec<AnchorTableRow>) {
        self.rows = rows;
        self.sort_stable();
    }

    fn sort_stable(&mut self) {
        self.rows.sort_by(|a, b| {
            (
                a.identity.sim_thing_id,
                a.identity.property_id,
                a.slot,
                a.col.raw(),
            )
                .cmp(&(
                    b.identity.sim_thing_id,
                    b.identity.property_id,
                    b.slot,
                    b.col.raw(),
                ))
        });
    }
}

/// Mint one row per live Anchored property column (all layout roles).
///
/// Unobserved properties are excluded. Ordering is deterministic.
/// Remap identity remains `(SimThingId, SimPropertyId)`; every role row for that
/// pair moves/retires together.
pub fn mint_anchor_table_from_admission(
    root: &SimThing,
    registry: &DimensionRegistry,
    loci: &AnchoredLocusMap,
    values: &[f32],
    n_dims: usize,
) -> AnchorTable {
    let mut rows = Vec::new();
    for (&(sim_thing_id, property_id), &(slot, _primary_col)) in loci.iter() {
        let Some(prop) = registry.try_property(property_id) else {
            continue;
        };
        if !matches!(
            prop.admission_disposition,
            PropertyAdmissionDisposition::Anchored
        ) {
            continue;
        }
        let Some(range) = registry.try_column_range(property_id) else {
            continue;
        };
        for sf in &prop.layout.sub_fields {
            let Some(col) = range.col_for_role(&sf.role, &prop.layout) else {
                continue;
            };
            let observed_value = value_at(values, n_dims, slot, col);
            rows.push(AnchorTableRow {
                identity: AnchorIdentity::new(sim_thing_id, property_id),
                slot,
                col,
                role: sf.role.clone(),
                band: None,
                last_crossing_generation: None,
                urgency: 0.0,
                observed_value,
            });
        }
    }
    let mut table = AnchorTable::new();
    table.replace_rows(rows);
    let _ = root;
    table
}

fn value_at(values: &[f32], n_dims: usize, slot: SlotIndex, col: ColumnIndex) -> f32 {
    if n_dims == 0 {
        return 0.0;
    }
    let idx = usize::from(slot) * n_dims + col.raw();
    values.get(idx).copied().unwrap_or(0.0)
}

/// Apply typed remaps: birth / move / retire while preserving dynamic fields.
pub fn apply_anchor_remaps_to_table(
    table: &mut AnchorTable,
    section: &AnchorRemapSection,
    registry: &DimensionRegistry,
) {
    if section.remap_not_required {
        return;
    }
    let mut by_id: BTreeMap<AnchorIdentity, Vec<AnchorTableRow>> = BTreeMap::new();
    for row in table.rows.drain(..) {
        by_id.entry(row.identity).or_default().push(row);
    }

    for remap in &section.remaps {
        apply_one_remap(&mut by_id, remap, registry);
    }

    let rows: Vec<AnchorTableRow> = by_id.into_values().flatten().collect();
    table.replace_rows(rows);
}

fn apply_one_remap(
    by_id: &mut BTreeMap<AnchorIdentity, Vec<AnchorTableRow>>,
    remap: &AnchorLocusRemap,
    registry: &DimensionRegistry,
) {
    let identity = AnchorIdentity::new(remap.sim_thing_id, remap.property_id);
    match (remap.to_slot, remap.to_col, remap.from_col) {
        (None, None, _) => {
            by_id.remove(&identity);
        }
        (Some(to_slot), Some(to_col), from_col) => {
            if let Some(rows) = by_id.get_mut(&identity) {
                let primary_delta = match from_col {
                    Some(from) => to_col.raw() as i64 - from.raw() as i64,
                    None => 0,
                };
                for row in rows.iter_mut() {
                    row.slot = to_slot;
                    // Prefer role→column resolution from the live registry. Fall
                    // back to the typed remap endpoint when this row was the
                    // primary locus (no oracle/rehearsal ColumnIndex mint).
                    if let Some(resolved) =
                        registry
                            .try_column_range(remap.property_id)
                            .and_then(|range| {
                                registry.try_property(remap.property_id).and_then(|prop| {
                                    range.col_for_role(&row.role, &prop.layout)
                                })
                            })
                    {
                        row.col = resolved;
                    } else if from_col == Some(row.col) {
                        row.col = to_col;
                    } else if primary_delta != 0 {
                        // Non-primary role without registry resolution: leave
                        // the prior typed column (do not mint via raw doors).
                        let _ = primary_delta;
                    }
                }
            } else {
                // Birth: seed all role rows for the property.
                let Some(prop) = registry.try_property(remap.property_id) else {
                    return;
                };
                let Some(range) = registry.try_column_range(remap.property_id) else {
                    return;
                };
                let mut born = Vec::new();
                for sf in &prop.layout.sub_fields {
                    let Some(col) = range.col_for_role(&sf.role, &prop.layout) else {
                        continue;
                    };
                    born.push(AnchorTableRow {
                        identity,
                        slot: to_slot,
                        col,
                        role: sf.role.clone(),
                        band: None,
                        last_crossing_generation: None,
                        urgency: 0.0,
                        observed_value: 0.0,
                    });
                }
                if born.is_empty() {
                    born.push(AnchorTableRow {
                        identity,
                        slot: to_slot,
                        col: to_col,
                        role: SubFieldRole::Amount,
                        band: None,
                        last_crossing_generation: None,
                        urgency: 0.0,
                        observed_value: 0.0,
                    });
                }
                by_id.insert(identity, born);
            }
        }
        _ => {}
    }
}

/// Refresh observed magnitudes from the writer-owned value plane and recompute urgency.
///
/// Cost model (DA watch-item): O(rows × matching edges) per boundary. Fine at the
/// dispatch baseline (~25 Anchored properties / modest live loci); revisit before
/// spatial-scale corpora without changing the consumer door.
///
/// `edge_thresholds` are `(slot_raw, col_raw, threshold)` triples known to the
/// fused write door (registration sidecars). Callers must not use this as a
/// consumer observation path.
pub fn refresh_anchor_table_magnitudes(
    table: &mut AnchorTable,
    values: &[f32],
    n_dims: usize,
    edge_thresholds: &[(u32, u32, f32)],
) {
    for row in &mut table.rows {
        row.observed_value = value_at(values, n_dims, row.slot, row.col);
        row.urgency = urgency_for(
            row.observed_value,
            row.slot.raw(),
            row.col.raw() as u32,
            edge_thresholds,
        );
    }
}

fn urgency_for(value: f32, slot: u32, col: u32, edges: &[(u32, u32, f32)]) -> f32 {
    let mut best: Option<f32> = None;
    for &(e_slot, e_col, threshold) in edges {
        if e_slot != slot || e_col != col {
            continue;
        }
        let d = (value - threshold).abs();
        best = Some(match best {
            Some(b) => b.min(d),
            None => d,
        });
    }
    best.unwrap_or(0.0)
}

/// Apply ordered fused band-crossing evidence to matching rows.
///
/// Multi-edge jumps: last delta in order wins for band / generation / value.
/// Matching uses identity + column when present.
pub fn apply_band_crossings_to_anchor_table(
    table: &mut AnchorTable,
    crossings: &[(AnchorIdentity, BandIndex, f32, Option<ColumnIndex>)],
    generation: u32,
) {
    for &(identity, band, post_value, col) in crossings {
        for row in table.rows.iter_mut().filter(|r| r.identity == identity) {
            if let Some(c) = col {
                if row.col != c {
                    continue;
                }
            }
            row.band = Some(band);
            row.last_crossing_generation = Some(generation);
            row.observed_value = post_value;
            if col.is_some() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::property::SimProperty;
    use crate::simthing::SimThingKind;

    #[test]
    fn mint_excludes_unobserved_and_orders_deterministically() {
        let mut registry = DimensionRegistry::new();
        let anchored = registry.register(SimProperty::simple("ns", "a", 1));
        let mut unobs = SimProperty::simple("ns", "u", 1);
        unobs.admission_disposition = PropertyAdmissionDisposition::Unobserved {
            reason: "dark".into(),
            source_span_token: 1,
        };
        let _unobserved = registry.register(unobs);

        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        root.properties.insert(
            anchored,
            crate::property::PropertyValue::from_raw_lanes(vec![0.0]),
        );

        let mut loci = AnchoredLocusMap::new();
        loci.insert(
            (root.id, anchored),
            (
                SlotIndex::new(0),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
        );
        let table = mint_anchor_table_from_admission(&root, &registry, &loci, &[0.0], 1);
        assert!(!table.is_empty());
        assert!(table
            .rows()
            .iter()
            .all(|r| r.identity.property_id == anchored));
        assert!(table.rows().iter().all(|r| r.band.is_none()));
    }

    #[test]
    fn unobserved_fixture_locus_gets_no_row() {
        let mut registry = DimensionRegistry::new();
        let mut unobs = SimProperty::simple("ns", "dark", 1);
        unobs.admission_disposition = PropertyAdmissionDisposition::Unobserved {
            reason: "fixture-dark".into(),
            source_span_token: 9,
        };
        let dark_id = registry.register(unobs);
        let mut root = SimThing::new(SimThingKind::GameSession, 0);
        root.properties.insert(
            dark_id,
            crate::property::PropertyValue::from_raw_lanes(vec![42.0]),
        );
        // Even if a caller fabricates a locus entry, disposition excludes the row.
        let mut loci = AnchoredLocusMap::new();
        loci.insert(
            (root.id, dark_id),
            (
                SlotIndex::new(0),
                ColumnIndex::from_raw_for_oracle_or_rehearsal(0),
            ),
        );
        let table = mint_anchor_table_from_admission(&root, &registry, &loci, &[42.0], 1);
        assert!(table.is_empty(), "Unobserved locus must mint zero rows");
        assert!(table.get(AnchorIdentity::new(root.id, dark_id)).is_none());
    }
}
