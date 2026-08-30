//! Effective clearing weights projected onto the graduated logical span substrate.
//!
//! Tree topology is admitted once by [`OverlaySpanProjection`]. Clearing-weight
//! resolution consumes that frozen logical directory and partitions only at
//! sparse override boundaries; it neither walks the tree nor materializes a
//! participant-id map.

use simthing_core::{SimThingId, TransformOp};
use std::ops::Index;
use thiserror::Error;

use crate::derived_span_projection::{
    DerivedDependencyIndex, DerivedSpanAdmissionError, DerivedSpanProjection, EffectiveProfileId,
    EffectiveSpanSeed, LogicalRowRange,
};
use crate::OverlaySpanProjection;

/// One sparse inherited clearing-weight override on the ordinary tree.
#[derive(Clone, Debug, PartialEq)]
pub struct ClearingWeightOverrideSpec {
    pub simthing_id: SimThingId,
    pub value_program: TransformOp,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ClearingWeightResolutionError {
    #[error("default clearing weight must be finite and non-negative")]
    InvalidDefault,
    #[error("duplicate clearing-weight override for {0:?}")]
    DuplicateOverride(SimThingId),
    #[error("clearing-weight override names absent SimThing {0:?}")]
    UnknownSimThing(SimThingId),
    #[error("clearing-weight override at {0:?} produced a non-finite or negative value")]
    InvalidResolvedWeight(SimThingId),
    #[error(transparent)]
    Projection(#[from] DerivedSpanAdmissionError),
}

/// Maximal effective clearing-weight spans over the canonical 7.8a logical
/// participant directory.
#[derive(Clone, Debug, PartialEq)]
pub struct ClearingWeightSpanProjection {
    projection: DerivedSpanProjection<f32>,
}

impl ClearingWeightSpanProjection {
    /// Resolve one participant's effective weight by logical identity.
    pub fn effective_weight(&self, id: SimThingId) -> Option<f32> {
        let row = self.projection.directory().range(id)?.start();
        self.projection.descriptor_at(row).copied()
    }

    /// Return the number of interned weight profiles and maximal spans.
    pub fn profile_and_span_counts(&self) -> (u64, u64) {
        (
            self.projection.profile_count() as u64,
            self.projection.span_count() as u64,
        )
    }
}

impl Index<&SimThingId> for ClearingWeightSpanProjection {
    type Output = f32;

    fn index(&self, id: &SimThingId) -> &Self::Output {
        let row = self
            .projection
            .directory()
            .range(*id)
            .unwrap_or_else(|| panic!("absent clearing-weight participant {id:?}"))
            .start();
        self.projection
            .descriptor_at(row)
            .expect("admitted clearing-weight spans cover every logical row")
    }
}

/// Resolve sparse inherited clearing weights through an already-admitted 7.8a
/// participant projection. Resolution work is proportional to override
/// boundaries, not to members in the tree.
pub fn resolve_effective_clearing_weights(
    participants: &OverlaySpanProjection,
    default_weight: f32,
    overrides: &[ClearingWeightOverrideSpec],
) -> Result<ClearingWeightSpanProjection, ClearingWeightResolutionError> {
    if !default_weight.is_finite() || default_weight < 0.0 {
        return Err(ClearingWeightResolutionError::InvalidDefault);
    }

    for (index, row) in overrides.iter().enumerate() {
        if overrides[..index]
            .iter()
            .any(|prior| prior.simthing_id == row.simthing_id)
        {
            return Err(ClearingWeightResolutionError::DuplicateOverride(
                row.simthing_id,
            ));
        }
    }

    let directory = participants.logical_directory();
    let mut admitted = Vec::with_capacity(overrides.len());
    let mut boundaries = Vec::with_capacity(overrides.len() * 2 + 2);
    boundaries.push(0);
    boundaries.push(directory.total_rows());
    for row in overrides {
        let range = directory.range(row.simthing_id).ok_or(
            ClearingWeightResolutionError::UnknownSimThing(row.simthing_id),
        )?;
        admitted.push((range, row));
        boundaries.push(range.start());
        boundaries.push(range.end());
    }
    boundaries.sort_unstable();
    boundaries.dedup();

    let mut seeds: Vec<EffectiveSpanSeed<f32>> = Vec::new();
    for window in boundaries.windows(2) {
        let start = window[0];
        let end = window[1];
        if start == end {
            continue;
        }
        let mut active = admitted
            .iter()
            .filter(|(range, _)| range.contains(start))
            .collect::<Vec<_>>();
        active.sort_unstable_by(|(left, _), (right, _)| {
            right
                .len()
                .cmp(&left.len())
                .then_with(|| left.start().cmp(&right.start()))
        });

        let mut weight = default_weight;
        for (_, row) in active {
            weight = row.value_program.apply(weight);
            if !weight.is_finite() || weight < 0.0 {
                return Err(ClearingWeightResolutionError::InvalidResolvedWeight(
                    row.simthing_id,
                ));
            }
        }
        let weight_bits = if weight == 0.0 {
            0.0f32.to_bits()
        } else {
            weight.to_bits()
        };
        let canonical_weight = f32::from_bits(weight_bits);
        if let Some(previous) = seeds.last_mut() {
            if previous.descriptor().to_bits() == weight_bits {
                previous.extend_to(end)?;
                continue;
            }
        }
        let range = LogicalRowRange::new(start, end - start)?;
        seeds.push(EffectiveSpanSeed::new(
            range,
            EffectiveProfileId::from_semantic_digest(u64::from(weight_bits)),
            canonical_weight,
        ));
    }

    let projection = DerivedSpanProjection::admit(
        directory.clone(),
        seeds,
        DerivedDependencyIndex::admit(Vec::new())?,
    )?;
    Ok(ClearingWeightSpanProjection { projection })
}
