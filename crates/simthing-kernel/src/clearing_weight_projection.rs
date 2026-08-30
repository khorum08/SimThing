//! Effective clearing weights projected onto the graduated logical span substrate.
//!
//! Tree topology is admitted once by [`OverlaySpanProjection`]. Clearing-weight
//! resolution consumes that frozen logical directory and partitions only at
//! sparse override boundaries; it neither walks the tree nor materializes a
//! participant-id map.

use std::collections::HashMap;

use simthing_core::{GenerationStamp, SimThingId, TransformOp};
use thiserror::Error;

use crate::derived_span_projection::{
    ChangedLocus, DerivedDependencyBinding, DerivedDependencyIndex, DerivedDependencyTarget,
    DerivedSpanAdmissionError, DerivedSpanProjection, EffectiveProfileId, EffectiveSpanSeed,
    LogicalRowRange, LogicalSubtreeDirectory,
};
use crate::OverlaySpanProjection;

/// One sparse inherited clearing-weight override on the ordinary tree.
#[derive(Clone, Debug, PartialEq)]
pub struct ClearingWeightOverrideSpec {
    /// Existing writer-blind 7.8a locus that owns this authored operand.
    pub source_locus: ChangedLocus,
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
    #[error("default clearing-weight source {0:?} is not the projection root")]
    DefaultSourceNotRoot(SimThingId),
    #[error("clearing-weight refresh changed the frozen operand dependency shape")]
    FrozenDependencyShapeChanged,
    #[error(transparent)]
    Projection(#[from] DerivedSpanAdmissionError),
}

/// Semantic work performed by one clearing-weight generation refresh.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClearingWeightProjectionRefresh {
    pub affected_ranges: u64,
    pub affected_logical_rows: u64,
    pub dirty_spans: u64,
    pub semantic_spans_rebuilt: u64,
    pub spans_examined: u64,
    pub logical_member_rows_scanned: u64,
    pub unaffected_profile_identities_checked: u64,
    pub unaffected_profile_identity_changes: u64,
}

/// Maximal effective clearing-weight spans over the canonical 7.8a logical
/// participant directory.
#[derive(Clone, Debug, PartialEq)]
pub struct ClearingWeightSpanProjection {
    projection: DerivedSpanProjection<f32>,
    override_dependency_shape: Vec<(SimThingId, ChangedLocus)>,
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

    /// Return the number of frozen source-locus to affected-subtree bindings.
    pub fn dependency_binding_count(&self) -> u64 {
        self.projection.dependency_index().binding_count() as u64
    }

    /// Consume authored source changes through the existing 7.8a dependency
    /// index, then eagerly rebuild only the affected semantic span ranges.
    pub fn refresh(
        &mut self,
        default_weight: f32,
        overrides: &[ClearingWeightOverrideSpec],
        changed_loci: &[ChangedLocus],
        generation: GenerationStamp,
    ) -> Result<ClearingWeightProjectionRefresh, ClearingWeightResolutionError> {
        validate_default(default_weight)?;
        let admitted = admit_overrides(self.projection.directory(), overrides)?;
        if override_dependency_shape(overrides) != self.override_dependency_shape {
            return Err(ClearingWeightResolutionError::FrozenDependencyShapeChanged);
        }

        let invalidation = self.projection.invalidate(changed_loci, generation)?;
        let affected_ranges = invalidation.affected_ranges.len() as u64;
        let affected_logical_rows = invalidation
            .affected_ranges
            .iter()
            .map(|range| range.len())
            .sum();
        let dirty_spans = invalidation.dirty_span_ranges.len() as u64;
        let spans_examined = invalidation.spans_examined;
        let logical_member_rows_scanned = invalidation.logical_member_rows_scanned;
        let unaffected_profiles =
            unaffected_profile_samples(&self.projection, invalidation.affected_ranges.as_slice());
        let mut semantic_spans_rebuilt = 0u64;
        for affected_range in invalidation.affected_ranges.iter().copied() {
            let mut replacements = HashMap::new();
            for span in self.projection.spans_in_range(affected_range) {
                let start = span.range().start().max(affected_range.start());
                let (weight, profile) = resolved_weight_at(default_weight, &admitted, start)?;
                replacements.insert(start, (weight, profile));
            }
            semantic_spans_rebuilt += self.projection.remap_range(
                affected_range,
                generation,
                |range, prior, prior_profile| {
                    replacements
                        .get(&range.start())
                        .copied()
                        .unwrap_or((*prior, prior_profile))
                },
            )?;
        }

        let unaffected_profile_identity_changes = unaffected_profiles
            .iter()
            .filter(|(row, profile)| self.projection.effective_profile_at(*row) != Some(*profile))
            .count() as u64;
        Ok(ClearingWeightProjectionRefresh {
            affected_ranges,
            affected_logical_rows,
            dirty_spans,
            semantic_spans_rebuilt,
            spans_examined,
            logical_member_rows_scanned,
            unaffected_profile_identities_checked: unaffected_profiles.len() as u64,
            unaffected_profile_identity_changes,
        })
    }
}

/// Resolve sparse inherited clearing weights through an already-admitted 7.8a
/// participant projection. Resolution work is proportional to override
/// boundaries, not to members in the tree.
pub fn resolve_effective_clearing_weights(
    participants: &OverlaySpanProjection,
    default_weight: f32,
    default_source_locus: ChangedLocus,
    overrides: &[ClearingWeightOverrideSpec],
) -> Result<ClearingWeightSpanProjection, ClearingWeightResolutionError> {
    validate_default(default_weight)?;
    let directory = participants.logical_directory();
    let default_range = directory.range(default_source_locus.logical_id()).ok_or(
        ClearingWeightResolutionError::Projection(
            DerivedSpanAdmissionError::UnknownLogicalIdentity(default_source_locus.logical_id()),
        ),
    )?;
    if default_range.start() != 0 || default_range.len() != directory.total_rows() {
        return Err(ClearingWeightResolutionError::DefaultSourceNotRoot(
            default_source_locus.logical_id(),
        ));
    }
    let admitted = admit_overrides(directory, overrides)?;
    let mut boundaries = Vec::with_capacity(overrides.len() * 2 + 2);
    boundaries.push(0);
    boundaries.push(directory.total_rows());
    for (range, _) in &admitted {
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
        let (canonical_weight, profile) = resolved_weight_at(default_weight, &admitted, start)?;
        let weight_bits = canonical_weight.to_bits();
        if let Some(previous) = seeds.last_mut() {
            if previous.descriptor().to_bits() == weight_bits {
                previous.extend_to(end)?;
                continue;
            }
        }
        let range = LogicalRowRange::new(start, end - start)?;
        seeds.push(EffectiveSpanSeed::new(range, profile, canonical_weight));
    }

    let root_id = default_source_locus.logical_id();
    let mut dependencies = Vec::with_capacity(overrides.len() + 1);
    dependencies.push(DerivedDependencyBinding::new(
        default_source_locus,
        DerivedDependencyTarget::SpanRoot(root_id),
    ));
    dependencies.extend(overrides.iter().map(|row| {
        DerivedDependencyBinding::new(
            row.source_locus.clone(),
            DerivedDependencyTarget::SpanRoot(row.simthing_id),
        )
    }));
    let projection = DerivedSpanProjection::admit(
        directory.clone(),
        seeds,
        DerivedDependencyIndex::admit(dependencies)?,
    )?;
    Ok(ClearingWeightSpanProjection {
        projection,
        override_dependency_shape: override_dependency_shape(overrides),
    })
}

fn validate_default(default_weight: f32) -> Result<(), ClearingWeightResolutionError> {
    if !default_weight.is_finite() || default_weight < 0.0 {
        return Err(ClearingWeightResolutionError::InvalidDefault);
    }
    Ok(())
}

fn admit_overrides<'a>(
    directory: &LogicalSubtreeDirectory,
    overrides: &'a [ClearingWeightOverrideSpec],
) -> Result<Vec<(LogicalRowRange, &'a ClearingWeightOverrideSpec)>, ClearingWeightResolutionError> {
    let mut admitted = Vec::with_capacity(overrides.len());
    for (index, row) in overrides.iter().enumerate() {
        if overrides[..index]
            .iter()
            .any(|prior| prior.simthing_id == row.simthing_id)
        {
            return Err(ClearingWeightResolutionError::DuplicateOverride(
                row.simthing_id,
            ));
        }
        directory.range(row.source_locus.logical_id()).ok_or(
            ClearingWeightResolutionError::Projection(
                DerivedSpanAdmissionError::UnknownLogicalIdentity(row.source_locus.logical_id()),
            ),
        )?;
        let range = directory.range(row.simthing_id).ok_or(
            ClearingWeightResolutionError::UnknownSimThing(row.simthing_id),
        )?;
        admitted.push((range, row));
    }
    Ok(admitted)
}

fn resolved_weight_at(
    default_weight: f32,
    admitted: &[(LogicalRowRange, &ClearingWeightOverrideSpec)],
    logical_row: u64,
) -> Result<(f32, EffectiveProfileId), ClearingWeightResolutionError> {
    let mut active = admitted
        .iter()
        .filter(|(range, _)| range.contains(logical_row))
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
    Ok((
        f32::from_bits(weight_bits),
        EffectiveProfileId::from_semantic_digest(u64::from(weight_bits)),
    ))
}

fn override_dependency_shape(
    overrides: &[ClearingWeightOverrideSpec],
) -> Vec<(SimThingId, ChangedLocus)> {
    let mut shape = overrides
        .iter()
        .map(|row| (row.simthing_id, row.source_locus.clone()))
        .collect::<Vec<_>>();
    shape.sort_unstable_by_key(|(id, locus)| {
        (id.raw(), locus.logical_id().raw(), locus.property_id().0)
    });
    shape
}

fn unaffected_profile_samples(
    projection: &DerivedSpanProjection<f32>,
    affected_ranges: &[LogicalRowRange],
) -> Vec<(u64, EffectiveProfileId)> {
    let mut samples = Vec::new();
    for span in projection.iter_spans() {
        let mut cursor = span.range().start();
        for affected in affected_ranges
            .iter()
            .copied()
            .filter(|affected| affected.intersects(span.range()))
        {
            let unaffected_end = affected.start().min(span.range().end());
            if cursor < unaffected_end {
                samples.push((
                    cursor,
                    projection
                        .effective_profile_at(cursor)
                        .expect("sample comes from an admitted span"),
                ));
            }
            cursor = cursor.max(affected.end());
            if cursor >= span.range().end() {
                break;
            }
        }
        if cursor < span.range().end() {
            samples.push((
                cursor,
                projection
                    .effective_profile_at(cursor)
                    .expect("sample comes from an admitted span"),
            ));
        }
    }
    samples
}
