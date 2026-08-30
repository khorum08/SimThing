//! Generic derived-span projection and source-blind invalidation (7.8a).
//!
//! This module owns physical projection metadata only. Consumer-specific
//! composition remains outside: callers provide an already-composed descriptor
//! and its stable profile identity. Logical subtree ranges are authoritative;
//! dense row materializations are optional caches built by consumers.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use simthing_core::{GenerationStamp, SimPropertyId, SimThingId, SubFieldRole};
use thiserror::Error;

/// Optional narrowing carried by a changed locus. The vocabulary is generic:
/// consumers may identify one admitted binding or one effective profile, but
/// may not name a writer subsystem.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum DerivedLocusNarrowing {
    Binding(u64),
    Profile(u64),
}

/// One authoritative changed locus. There is deliberately no writer/source
/// field: identical state changes invalidate identical work.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChangedLocus {
    logical_id: SimThingId,
    property_id: SimPropertyId,
    role: SubFieldRole,
    narrowing: Option<DerivedLocusNarrowing>,
}

impl ChangedLocus {
    pub fn new(logical_id: SimThingId, property_id: SimPropertyId, role: SubFieldRole) -> Self {
        Self {
            logical_id,
            property_id,
            role,
            narrowing: None,
        }
    }

    pub(crate) fn narrowed(mut self, narrowing: DerivedLocusNarrowing) -> Self {
        self.narrowing = Some(narrowing);
        self
    }

    pub fn logical_id(&self) -> SimThingId {
        self.logical_id
    }

    pub fn property_id(&self) -> SimPropertyId {
        self.property_id
    }

    pub fn role(&self) -> &SubFieldRole {
        &self.role
    }

    pub(crate) fn narrowing(&self) -> Option<DerivedLocusNarrowing> {
        self.narrowing
    }
}

/// Opaque identity of an already-composed effective descriptor. It derives
/// from semantic bindings, never a slot or physical row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct EffectiveProfileId(u64);

impl EffectiveProfileId {
    pub fn from_semantic_digest(digest: u64) -> Self {
        Self(digest)
    }

    pub fn digest(self) -> u64 {
        self.0
    }
}

/// Generic field registration target. The authority tag says which existing
/// Field-Triad registration owns the work; it does not alter key semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum FieldRegistrationAuthority {
    Stead,
    Palma,
    GuYang,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct FieldRegistrationRef {
    authority: FieldRegistrationAuthority,
    registration_id: u32,
}

impl FieldRegistrationRef {
    pub fn new(authority: FieldRegistrationAuthority, registration_id: u32) -> Self {
        Self {
            authority,
            registration_id,
        }
    }

    pub fn authority(self) -> FieldRegistrationAuthority {
        self.authority
    }

    pub fn registration_id(self) -> u32 {
        self.registration_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct DerivedWorkId(u32);

impl DerivedWorkId {
    pub fn new(raw: u32) -> Self {
        Self(raw)
    }

    pub fn raw(self) -> u32 {
        self.0
    }
}

/// A frozen dependency target. `SpanRoot` names a logical subtree and is
/// resolved against the current span partition at invalidation time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum DerivedDependencyTarget {
    SpanRoot(SimThingId),
    LogicalMember(SimThingId),
    FieldRegistration(FieldRegistrationRef),
    Work(DerivedWorkId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DerivedDependencyBinding {
    locus: ChangedLocus,
    target: DerivedDependencyTarget,
}

impl DerivedDependencyBinding {
    pub fn new(locus: ChangedLocus, target: DerivedDependencyTarget) -> Self {
        Self { locus, target }
    }
}

/// Session-frozen source-blind dependency index. Admission consumes the rows;
/// there is no runtime registration or mutation surface.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct DerivedDependencyIndex {
    by_locus: HashMap<ChangedLocus, Vec<DerivedDependencyTarget>>,
}

impl DerivedDependencyIndex {
    pub fn admit(
        bindings: Vec<DerivedDependencyBinding>,
    ) -> Result<Self, DerivedSpanAdmissionError> {
        let mut by_locus: HashMap<ChangedLocus, Vec<DerivedDependencyTarget>> = HashMap::new();
        for binding in bindings {
            let targets = by_locus.entry(binding.locus).or_default();
            if targets.contains(&binding.target) {
                return Err(DerivedSpanAdmissionError::DuplicateDependency);
            }
            targets.push(binding.target);
        }
        Ok(Self { by_locus })
    }

    pub fn dependents(&self, locus: &ChangedLocus) -> &[DerivedDependencyTarget] {
        self.by_locus.get(locus).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn binding_count(&self) -> usize {
        self.by_locus.values().map(Vec::len).sum()
    }
}

/// Half-open logical preorder range. This is structural range vocabulary,
/// never a physical row range.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LogicalRowRange {
    start: u64,
    len: u64,
}

impl LogicalRowRange {
    pub fn new(start: u64, len: u64) -> Result<Self, DerivedSpanAdmissionError> {
        if len == 0 || start.checked_add(len).is_none() {
            return Err(DerivedSpanAdmissionError::InvalidLogicalRange { start, len });
        }
        Ok(Self { start, len })
    }

    pub fn start(self) -> u64 {
        self.start
    }

    pub fn len(self) -> u64 {
        self.len
    }

    pub fn end(self) -> u64 {
        self.start + self.len
    }

    pub fn contains(self, row: u64) -> bool {
        self.start <= row && row < self.end()
    }

    pub fn intersects(self, other: Self) -> bool {
        self.start < other.end() && other.start < self.end()
    }
}

/// Frozen logical-id -> subtree-range directory. It is independent of slot
/// bindings and survives a physical epoch remap unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LogicalSubtreeDirectory {
    total_rows: u64,
    ranges: HashMap<SimThingId, LogicalRowRange>,
}

impl LogicalSubtreeDirectory {
    pub fn admit(
        total_rows: u64,
        rows: Vec<(SimThingId, LogicalRowRange)>,
    ) -> Result<Self, DerivedSpanAdmissionError> {
        if total_rows == 0 {
            return Err(DerivedSpanAdmissionError::EmptyProjection);
        }
        let mut ranges = HashMap::with_capacity(rows.len());
        for (id, range) in rows {
            if range.end() > total_rows {
                return Err(DerivedSpanAdmissionError::RangeBeyondProjection {
                    end: range.end(),
                    total_rows,
                });
            }
            if ranges.insert(id, range).is_some() {
                return Err(DerivedSpanAdmissionError::DuplicateLogicalIdentity(id));
            }
        }
        Ok(Self { total_rows, ranges })
    }

    pub fn total_rows(&self) -> u64 {
        self.total_rows
    }

    pub fn range(&self, id: SimThingId) -> Option<LogicalRowRange> {
        self.ranges.get(&id).copied()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EffectiveSpanSeed<D> {
    range: LogicalRowRange,
    profile_id: EffectiveProfileId,
    descriptor: D,
}

impl<D> EffectiveSpanSeed<D> {
    pub fn new(range: LogicalRowRange, profile_id: EffectiveProfileId, descriptor: D) -> Self {
        Self {
            range,
            profile_id,
            descriptor,
        }
    }

    pub(crate) fn descriptor(&self) -> &D {
        &self.descriptor
    }

    pub(crate) fn extend_to(&mut self, end: u64) -> Result<(), DerivedSpanAdmissionError> {
        self.range = LogicalRowRange::new(self.range.start(), end - self.range.start())?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EffectiveSpan<D> {
    range: LogicalRowRange,
    profile_id: EffectiveProfileId,
    descriptor: Arc<D>,
    dirty_at: Option<GenerationStamp>,
}

impl<D> EffectiveSpan<D> {
    pub(crate) fn range(&self) -> LogicalRowRange {
        self.range
    }

    pub(crate) fn descriptor(&self) -> &D {
        &self.descriptor
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct DerivedInvalidation {
    /// Exact logical ranges named by the frozen dependency index.
    pub affected_ranges: Vec<LogicalRowRange>,
    pub dirty_span_ranges: Vec<LogicalRowRange>,
    pub field_registrations: Vec<FieldRegistrationRef>,
    pub work: Vec<DerivedWorkId>,
    /// Semantic work metric. Physical member rows are never inspected by the
    /// invalidator; a dense consumer cache reports its own separate work.
    pub spans_examined: u64,
    pub logical_member_rows_scanned: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DerivedSpanProjection<D> {
    directory: LogicalSubtreeDirectory,
    spans: BTreeMap<u64, EffectiveSpan<D>>,
    profiles: HashMap<EffectiveProfileId, Arc<D>>,
    profile_uses: HashMap<EffectiveProfileId, usize>,
    dependencies: DerivedDependencyIndex,
}

impl<D: Clone + PartialEq> DerivedSpanProjection<D> {
    pub fn admit(
        directory: LogicalSubtreeDirectory,
        seeds: Vec<EffectiveSpanSeed<D>>,
        dependencies: DerivedDependencyIndex,
    ) -> Result<Self, DerivedSpanAdmissionError> {
        if seeds.is_empty() {
            return Err(DerivedSpanAdmissionError::EmptyProjection);
        }
        let mut expected_start = 0u64;
        let mut prior: Option<&EffectiveSpanSeed<D>> = None;
        for seed in &seeds {
            if seed.range.start() != expected_start {
                return Err(DerivedSpanAdmissionError::NonContiguousCoverage {
                    expected_start,
                    actual_start: seed.range.start(),
                });
            }
            if let Some(previous) = prior {
                if previous.profile_id == seed.profile_id && previous.descriptor == seed.descriptor
                {
                    return Err(DerivedSpanAdmissionError::DescendantScaleProfileExplosion {
                        at_row: seed.range.start(),
                    });
                }
            }
            expected_start = seed.range.end();
            prior = Some(seed);
        }
        if expected_start != directory.total_rows() {
            return Err(DerivedSpanAdmissionError::IncompleteCoverage {
                covered: expected_start,
                total_rows: directory.total_rows(),
            });
        }
        let mut profiles: HashMap<EffectiveProfileId, Arc<D>> = HashMap::new();
        let mut spans = BTreeMap::new();
        let mut profile_uses = HashMap::new();
        for seed in seeds {
            let descriptor = match profiles.get(&seed.profile_id) {
                Some(admitted) if admitted.as_ref() != &seed.descriptor => {
                    return Err(DerivedSpanAdmissionError::ProfileIdentityCollision {
                        profile_digest: seed.profile_id.digest(),
                    });
                }
                Some(admitted) => admitted.clone(),
                None => {
                    let admitted = Arc::new(seed.descriptor);
                    profiles.insert(seed.profile_id, admitted.clone());
                    admitted
                }
            };
            *profile_uses.entry(seed.profile_id).or_insert(0) += 1;
            spans.insert(
                seed.range.start(),
                EffectiveSpan {
                    range: seed.range,
                    profile_id: seed.profile_id,
                    descriptor,
                    dirty_at: None,
                },
            );
        }
        Ok(Self {
            directory,
            spans,
            profiles,
            profile_uses,
            dependencies,
        })
    }

    pub(crate) fn iter_spans(&self) -> impl Iterator<Item = &EffectiveSpan<D>> {
        self.spans.values()
    }

    pub(crate) fn spans_in_range(
        &self,
        range: LogicalRowRange,
    ) -> impl Iterator<Item = &EffectiveSpan<D>> {
        let first = self
            .spans
            .range(..=range.start())
            .next_back()
            .map(|(&start, _)| start)
            .unwrap_or(range.start());
        self.spans
            .range(first..range.end())
            .map(|(_, span)| span)
            .filter(move |span| span.range.intersects(range))
    }

    pub fn span_count(&self) -> usize {
        self.spans.len()
    }

    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn directory(&self) -> &LogicalSubtreeDirectory {
        &self.directory
    }

    pub fn dependency_index(&self) -> &DerivedDependencyIndex {
        &self.dependencies
    }

    pub fn effective_profile_at(&self, logical_row: u64) -> Option<EffectiveProfileId> {
        self.spans
            .range(..=logical_row)
            .next_back()
            .map(|(_, span)| span)
            .filter(|span| span.range.contains(logical_row))
            .map(|span| span.profile_id)
    }

    pub(crate) fn descriptor_at(&self, logical_row: u64) -> Option<&D> {
        self.spans
            .range(..=logical_row)
            .next_back()
            .map(|(_, span)| span)
            .filter(|span| span.range.contains(logical_row))
            .map(EffectiveSpan::descriptor)
    }

    pub fn invalidate(
        &mut self,
        loci: &[ChangedLocus],
        generation: GenerationStamp,
    ) -> Result<DerivedInvalidation, DerivedSpanAdmissionError> {
        let mut ranges = Vec::new();
        let mut field_registrations = HashSet::new();
        let mut work = HashSet::new();
        for locus in loci {
            self.directory.range(locus.logical_id()).ok_or(
                DerivedSpanAdmissionError::UnknownLogicalIdentity(locus.logical_id()),
            )?;
            for target in self.dependencies.dependents(locus) {
                match *target {
                    DerivedDependencyTarget::SpanRoot(id) => ranges.push(
                        self.directory
                            .range(id)
                            .ok_or(DerivedSpanAdmissionError::UnknownLogicalIdentity(id))?,
                    ),
                    DerivedDependencyTarget::LogicalMember(id) => {
                        let range = self
                            .directory
                            .range(id)
                            .ok_or(DerivedSpanAdmissionError::UnknownLogicalIdentity(id))?;
                        ranges.push(LogicalRowRange::new(range.start(), 1)?);
                    }
                    DerivedDependencyTarget::FieldRegistration(registration) => {
                        field_registrations.insert(registration);
                    }
                    DerivedDependencyTarget::Work(id) => {
                        work.insert(id);
                    }
                }
            }
        }
        let ranges = coalesce_ranges(ranges);
        let mut dirty_span_ranges = Vec::new();
        let mut candidate_starts = HashSet::new();
        for range in &ranges {
            candidate_starts.extend(self.spans_in_range(*range).map(|span| span.range.start()));
        }
        let mut candidate_starts = candidate_starts.into_iter().collect::<Vec<_>>();
        candidate_starts.sort_unstable();
        for start in &candidate_starts {
            let span = self
                .spans
                .get_mut(start)
                .expect("candidate came from span index");
            span.dirty_at = Some(generation);
            dirty_span_ranges.push(span.range);
        }
        let mut field_registrations = field_registrations.into_iter().collect::<Vec<_>>();
        field_registrations.sort_unstable();
        let mut work = work.into_iter().collect::<Vec<_>>();
        work.sort_unstable();
        Ok(DerivedInvalidation {
            affected_ranges: ranges,
            dirty_span_ranges,
            field_registrations,
            work,
            spans_examined: candidate_starts.len() as u64,
            logical_member_rows_scanned: 0,
        })
    }

    /// Rewrite only spans intersecting `range`, splitting at its boundaries.
    /// The closure receives one already-composed descriptor and returns the
    /// replacement descriptor/profile identity. It is called per span, never
    /// per descendant row.
    pub fn remap_range(
        &mut self,
        range: LogicalRowRange,
        generation: GenerationStamp,
        mut remap: impl FnMut(LogicalRowRange, &D, EffectiveProfileId) -> (D, EffectiveProfileId),
    ) -> Result<u64, DerivedSpanAdmissionError> {
        let affected_keys = self.span_keys_in_range(range);
        if affected_keys.is_empty() {
            return Ok(0);
        }
        let first = affected_keys[0];
        let last = *affected_keys.last().expect("non-empty checked above");
        let neighbor_before = self.spans.range(..first).next_back().map(|(&key, _)| key);
        let neighbor_after = self
            .spans
            .range((std::ops::Bound::Excluded(last), std::ops::Bound::Unbounded))
            .next()
            .map(|(&key, _)| key);
        let mut window_keys = affected_keys.clone();
        if let Some(key) = neighbor_before {
            window_keys.push(key);
        }
        if let Some(key) = neighbor_after {
            window_keys.push(key);
        }
        window_keys.sort_unstable();
        window_keys.dedup();

        let mut window = BTreeMap::new();
        for key in window_keys {
            let span = self
                .remove_span(key)
                .expect("window key came from span index");
            window.insert(key, span);
        }
        let mut rewritten = Vec::with_capacity(window.len() + 2);
        let mut changed_spans = 0u64;
        for (_, span) in window {
            if !span.range.intersects(range) {
                rewritten.push(span);
                continue;
            }
            let overlap_start = span.range.start().max(range.start());
            let overlap_end = span.range.end().min(range.end());
            if span.range.start() < overlap_start {
                rewritten.push(EffectiveSpan {
                    range: LogicalRowRange {
                        start: span.range.start(),
                        len: overlap_start - span.range.start(),
                    },
                    profile_id: span.profile_id,
                    descriptor: span.descriptor.clone(),
                    dirty_at: span.dirty_at,
                });
            }
            let overlap = LogicalRowRange {
                start: overlap_start,
                len: overlap_end - overlap_start,
            };
            let (descriptor, profile_id) =
                remap(overlap, span.descriptor.as_ref(), span.profile_id);
            let descriptor = self.intern_profile(profile_id, descriptor)?;
            rewritten.push(EffectiveSpan {
                range: overlap,
                profile_id,
                descriptor,
                dirty_at: Some(generation),
            });
            changed_spans += 1;
            if overlap_end < span.range.end() {
                rewritten.push(EffectiveSpan {
                    range: LogicalRowRange {
                        start: overlap_end,
                        len: span.range.end() - overlap_end,
                    },
                    profile_id: span.profile_id,
                    descriptor: span.descriptor,
                    dirty_at: span.dirty_at,
                });
            }
        }
        for span in merge_adjacent(rewritten) {
            self.insert_span(span)?;
        }
        Ok(changed_spans)
    }

    fn span_keys_in_range(&self, range: LogicalRowRange) -> Vec<u64> {
        self.spans_in_range(range)
            .map(|span| span.range.start())
            .collect()
    }

    fn remove_span(&mut self, start: u64) -> Option<EffectiveSpan<D>> {
        let span = self.spans.remove(&start)?;
        let uses = self
            .profile_uses
            .get_mut(&span.profile_id)
            .expect("every span profile has a use count");
        *uses -= 1;
        if *uses == 0 {
            self.profile_uses.remove(&span.profile_id);
            self.profiles.remove(&span.profile_id);
        }
        Some(span)
    }

    fn intern_profile(
        &mut self,
        profile_id: EffectiveProfileId,
        descriptor: D,
    ) -> Result<Arc<D>, DerivedSpanAdmissionError> {
        match self.profiles.get(&profile_id) {
            Some(admitted) if admitted.as_ref() == &descriptor => Ok(admitted.clone()),
            Some(_) => Err(DerivedSpanAdmissionError::ProfileIdentityCollision {
                profile_digest: profile_id.digest(),
            }),
            None => {
                let descriptor = Arc::new(descriptor);
                self.profiles.insert(profile_id, descriptor.clone());
                Ok(descriptor)
            }
        }
    }

    fn insert_span(&mut self, span: EffectiveSpan<D>) -> Result<(), DerivedSpanAdmissionError> {
        match self.profiles.get(&span.profile_id) {
            Some(admitted) if admitted.as_ref() != span.descriptor.as_ref() => {
                return Err(DerivedSpanAdmissionError::ProfileIdentityCollision {
                    profile_digest: span.profile_id.digest(),
                });
            }
            Some(_) => {}
            None => {
                self.profiles
                    .insert(span.profile_id, span.descriptor.clone());
            }
        }
        *self.profile_uses.entry(span.profile_id).or_insert(0) += 1;
        let prior = self.spans.insert(span.range.start(), span);
        debug_assert!(prior.is_none(), "span starts remain unique");
        Ok(())
    }
}

fn merge_adjacent<D: PartialEq>(spans: Vec<EffectiveSpan<D>>) -> Vec<EffectiveSpan<D>> {
    let mut merged: Vec<EffectiveSpan<D>> = Vec::with_capacity(spans.len());
    for span in spans {
        if let Some(previous) = merged.last_mut() {
            if previous.range.end() == span.range.start()
                && previous.profile_id == span.profile_id
                && previous.descriptor == span.descriptor
                && previous.dirty_at == span.dirty_at
            {
                previous.range.len += span.range.len;
                continue;
            }
        }
        merged.push(span);
    }
    merged
}

fn coalesce_ranges(mut ranges: Vec<LogicalRowRange>) -> Vec<LogicalRowRange> {
    ranges.sort_unstable_by_key(|range| (range.start(), range.end()));
    let mut merged: Vec<LogicalRowRange> = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let Some(previous) = merged.last_mut() {
            if range.start() <= previous.end() {
                let end = previous.end().max(range.end());
                previous.len = end - previous.start;
                continue;
            }
        }
        merged.push(range);
    }
    merged
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DerivedSpanAdmissionError {
    #[error("derived projection requires at least one logical row and one span")]
    EmptyProjection,
    #[error("logical range start {start} length {len} is empty or overflows")]
    InvalidLogicalRange { start: u64, len: u64 },
    #[error("logical range end {end} exceeds projection row count {total_rows}")]
    RangeBeyondProjection { end: u64, total_rows: u64 },
    #[error("logical identity {0:?} appears more than once in the frozen range directory")]
    DuplicateLogicalIdentity(SimThingId),
    #[error("logical identity {0:?} is outside the frozen range directory")]
    UnknownLogicalIdentity(SimThingId),
    #[error("span coverage expected row {expected_start}, found row {actual_start}")]
    NonContiguousCoverage {
        expected_start: u64,
        actual_start: u64,
    },
    #[error("span coverage stops at {covered}, projection contains {total_rows} rows")]
    IncompleteCoverage { covered: u64, total_rows: u64 },
    #[error("homogeneous profile was split into descendant-scale adjacent spans at row {at_row}")]
    DescendantScaleProfileExplosion { at_row: u64 },
    #[error("effective profile id {profile_digest:#018x} names more than one semantic descriptor")]
    ProfileIdentityCollision { profile_digest: u64 },
    #[error("the frozen dependency index contains a duplicate locus/target row")]
    DuplicateDependency,
    #[error("frozen dependency shape changed at {0:?}; admit a fresh projection")]
    FrozenDependencyShapeChanged(SimThingId),
}

#[cfg(test)]
#[path = "derived_span_projection_tests.rs"]
mod tests;
