//! Generic field cadence scheduler and dirty macro-region skip (Phase M-2).
//!
//! Semantic-free driver capability for adopted mapping optimizations. Not wired
//! into the production pass graph; callers opt in explicitly.

use simthing_gpu::{
    GpuContext, StructuredFieldExecutionOptions, StructuredFieldExecutionReport,
    StructuredFieldStencilOp, StructuredFieldStencilError,
};
use thiserror::Error;

/// Opaque registered field identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FieldId(pub u32);

/// Opaque macro-region identifier local to a [`FieldId`].
///
/// Scheduler identity for a region is the pair `(FieldId, FieldRegionId)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldRegionId(pub u32);

/// Generic cadence tier for a registered field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldCadence {
    EveryTick,
    EveryN { n: u32 },
    OnEvent,
}

/// Per-field cadence/event state tracked across ticks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldScheduleState {
    pub field_id: FieldId,
    pub cadence: FieldCadence,
    pub event_pending: bool,
}

/// Dirty macro-region state used for conservative skip decisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirtyRegionState {
    pub dirty_source_present: bool,
    pub dirty_neighbor_present: bool,
    pub residual_present: bool,
    pub topology_generation: u64,
    pub last_topology_generation: u64,
    pub operator_generation: u64,
    pub last_operator_generation: u64,
}

impl Default for DirtyRegionState {
    fn default() -> Self {
        Self {
            dirty_source_present: false,
            dirty_neighbor_present: false,
            residual_present: false,
            topology_generation: 0,
            last_topology_generation: 0,
            operator_generation: 0,
            last_operator_generation: 0,
        }
    }
}

impl DirtyRegionState {
    pub fn is_clean(&self) -> bool {
        !self.dirty_source_present
            && !self.dirty_neighbor_present
            && !self.residual_present
            && self.topology_generation == self.last_topology_generation
            && self.operator_generation == self.last_operator_generation
    }
}

/// One registered macro-region belonging to a field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldRegionRegistration {
    pub region_id: FieldRegionId,
    pub field_id: FieldId,
    pub dirty: DirtyRegionState,
}

/// Evidence-only grid descriptor; scheduler decisions do not depend on grid size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldGridDescriptor {
    pub field_id: FieldId,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldDispatchSchedule {
    Dispatch,
    Skip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldDispatchReason {
    CadenceDue,
    DirtySource,
    DirtyNeighbor,
    Residual,
    TopologyChanged,
    OperatorChanged,
    EventTriggered,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldDispatchDecision {
    pub region_id: FieldRegionId,
    pub field_id: FieldId,
    pub schedule: FieldDispatchSchedule,
    pub reasons: Vec<FieldDispatchReason>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldSchedulerReport {
    pub total_regions: u32,
    pub scheduled_regions: u32,
    pub skipped_regions: u32,
    pub skip_ratio: f32,
    pub false_skip_count: u32,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum FieldSchedulerError {
    #[error("EveryN cadence requires n > 0")]
    InvalidEveryNZero,
    #[error("unknown field id {0:?} for region {1:?}")]
    UnknownField(FieldId, FieldRegionId),
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum ScheduledStencilExecutionError {
    #[error("multiple scheduled regions ({count}) cannot share one StructuredFieldStencilOp")]
    MultipleScheduledRegionsForSingleOp { count: u32 },
    #[error(transparent)]
    Stencil(#[from] StructuredFieldStencilError),
}

/// Generic cadence + dirty-region scheduler.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldScheduler {
    fields: Vec<FieldScheduleState>,
    regions: Vec<FieldRegionRegistration>,
}

impl FieldCadence {
    pub fn validate(&self) -> Result<(), FieldSchedulerError> {
        if let Self::EveryN { n: 0 } = self {
            return Err(FieldSchedulerError::InvalidEveryNZero);
        }
        Ok(())
    }

    /// Returns whether this cadence tier is due at `tick`.
    pub fn is_due(&self, tick: u32, event_pending: bool) -> Result<bool, FieldSchedulerError> {
        self.validate()?;
        Ok(match self {
            Self::EveryTick => true,
            Self::EveryN { n } => tick % n == 0,
            Self::OnEvent => event_pending,
        })
    }
}

impl FieldScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_field(&mut self, state: FieldScheduleState) {
        if let Some(existing) = self.fields.iter_mut().find(|f| f.field_id == state.field_id) {
            *existing = state;
        } else {
            self.fields.push(state);
        }
    }

    pub fn register_region(&mut self, region: FieldRegionRegistration) {
        if let Some(existing) = self.regions.iter_mut().find(|r| {
            r.field_id == region.field_id && r.region_id == region.region_id
        }) {
            *existing = region;
        } else {
            self.regions.push(region);
        }
    }

    pub fn fields(&self) -> &[FieldScheduleState] {
        &self.fields
    }

    pub fn regions(&self) -> &[FieldRegionRegistration] {
        &self.regions
    }

    pub fn regions_mut(&mut self) -> &mut [FieldRegionRegistration] {
        &mut self.regions
    }

    fn field_state(&self, field_id: FieldId) -> Option<&FieldScheduleState> {
        self.fields.iter().find(|f| f.field_id == field_id)
    }

    fn collect_reasons(
        dirty: &DirtyRegionState,
        cadence_due: bool,
        event_triggered: bool,
    ) -> Vec<FieldDispatchReason> {
        let mut reasons = Vec::new();
        if cadence_due && !event_triggered {
            reasons.push(FieldDispatchReason::CadenceDue);
        }
        if event_triggered {
            reasons.push(FieldDispatchReason::EventTriggered);
        }
        if dirty.dirty_source_present {
            reasons.push(FieldDispatchReason::DirtySource);
        }
        if dirty.dirty_neighbor_present {
            reasons.push(FieldDispatchReason::DirtyNeighbor);
        }
        if dirty.residual_present {
            reasons.push(FieldDispatchReason::Residual);
        }
        if dirty.topology_generation != dirty.last_topology_generation {
            reasons.push(FieldDispatchReason::TopologyChanged);
        }
        if dirty.operator_generation != dirty.last_operator_generation {
            reasons.push(FieldDispatchReason::OperatorChanged);
        }
        reasons
    }

    fn must_schedule(dirty: &DirtyRegionState, cadence_due: bool) -> bool {
        cadence_due
            || dirty.dirty_source_present
            || dirty.dirty_neighbor_present
            || dirty.residual_present
            || dirty.topology_generation != dirty.last_topology_generation
            || dirty.operator_generation != dirty.last_operator_generation
    }

    /// Oracle helper for tests: true when a region must not be skipped.
    pub fn region_must_schedule(
        dirty: &DirtyRegionState,
        cadence: FieldCadence,
        tick: u32,
        event_pending: bool,
    ) -> Result<bool, FieldSchedulerError> {
        let cadence_due = cadence.is_due(tick, event_pending)?;
        Ok(Self::must_schedule(dirty, cadence_due))
    }

    /// Decide dispatch vs skip for every registered region at `tick`.
    pub fn decide_tick(
        &self,
        tick: u32,
    ) -> Result<(Vec<FieldDispatchDecision>, FieldSchedulerReport), FieldSchedulerError> {
        let mut decisions = Vec::with_capacity(self.regions.len());
        let mut scheduled = 0u32;
        let mut false_skips = 0u32;

        for region in &self.regions {
            let field = self
                .field_state(region.field_id)
                .ok_or(FieldSchedulerError::UnknownField(
                    region.field_id,
                    region.region_id,
                ))?;
            field.cadence.validate()?;
            let cadence_due = field.cadence.is_due(tick, field.event_pending)?;
            let event_triggered =
                matches!(field.cadence, FieldCadence::OnEvent) && field.event_pending;
            let must = Self::must_schedule(&region.dirty, cadence_due);
            let reasons = Self::collect_reasons(&region.dirty, cadence_due, event_triggered);
            let schedule = if must {
                FieldDispatchSchedule::Dispatch
            } else {
                FieldDispatchSchedule::Skip
            };
            if matches!(schedule, FieldDispatchSchedule::Skip)
                && Self::must_schedule(&region.dirty, cadence_due)
            {
                false_skips += 1;
            }
            if matches!(schedule, FieldDispatchSchedule::Dispatch) {
                scheduled += 1;
            }
            decisions.push(FieldDispatchDecision {
                region_id: region.region_id,
                field_id: region.field_id,
                schedule,
                reasons,
            });
        }

        let total = self.regions.len() as u32;
        let skipped = total.saturating_sub(scheduled);
        let skip_ratio = if total == 0 {
            0.0
        } else {
            skipped as f32 / total as f32
        };

        Ok((
            decisions,
            FieldSchedulerReport {
                total_regions: total,
                scheduled_regions: scheduled,
                skipped_regions: skipped,
                skip_ratio,
                false_skip_count: false_skips,
            },
        ))
    }
}

/// Count due ticks for one cadence over `[0, tick_count)`.
pub fn count_cadence_due_ticks(
    cadence: FieldCadence,
    tick_count: u32,
    event_ticks: &[u32],
) -> Result<u32, FieldSchedulerError> {
    cadence.validate()?;
    let mut due = 0u32;
    for tick in 0..tick_count {
        let event_pending = event_ticks.contains(&tick);
        if cadence.is_due(tick, event_pending)? {
            due += 1;
        }
    }
    Ok(due)
}

/// Invoke `visit` for each scheduled decision; skipped decisions are not visited.
pub fn visit_scheduled_regions<F, E>(
    decisions: &[FieldDispatchDecision],
    mut visit: F,
) -> Result<Vec<(FieldId, FieldRegionId)>, E>
where
    F: FnMut(&FieldDispatchDecision) -> Result<(), E>,
{
    let mut executed = Vec::new();
    for decision in decisions {
        if matches!(decision.schedule, FieldDispatchSchedule::Skip) {
            continue;
        }
        visit(decision)?;
        executed.push((decision.field_id, decision.region_id));
    }
    Ok(executed)
}

/// Run caller-provided execution for each scheduled decision.
///
/// The scheduler does not map regions to GPU ops; the caller supplies execution
/// behavior per `(FieldId, FieldRegionId)`.
pub fn execute_scheduled_regions_with<F, E, T>(
    decisions: &[FieldDispatchDecision],
    mut execute_one: F,
) -> Result<ScheduledRegionsExecutionSummary<T>, E>
where
    F: FnMut(&FieldDispatchDecision) -> Result<T, E>,
{
    let mut executed = Vec::new();
    let mut results = Vec::new();
    for decision in decisions {
        if matches!(decision.schedule, FieldDispatchSchedule::Skip) {
            continue;
        }
        results.push(execute_one(decision)?);
        executed.push((decision.field_id, decision.region_id));
    }
    Ok(ScheduledRegionsExecutionSummary { executed, results })
}

/// Execute at most one scheduled region against a single `StructuredFieldStencilOp`.
///
/// Returns `None` when no region is scheduled. Errors when more than one scheduled
/// region would advance the same op/buffer pair.
pub fn execute_single_scheduled_stencil_region(
    ctx: &GpuContext,
    op: &StructuredFieldStencilOp,
    decisions: &[FieldDispatchDecision],
    options: StructuredFieldExecutionOptions,
) -> Result<Option<ScheduledSingleStencilExecution>, ScheduledStencilExecutionError> {
    let scheduled: Vec<_> = decisions
        .iter()
        .filter(|d| matches!(d.schedule, FieldDispatchSchedule::Dispatch))
        .collect();
    if scheduled.is_empty() {
        return Ok(None);
    }
    if scheduled.len() > 1 {
        return Err(ScheduledStencilExecutionError::MultipleScheduledRegionsForSingleOp {
            count: scheduled.len() as u32,
        });
    }
    let decision = scheduled[0];
    let report = op.execute_configured(ctx, options)?;
    Ok(Some(ScheduledSingleStencilExecution {
        field_id: decision.field_id,
        region_id: decision.region_id,
        report,
    }))
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScheduledRegionsExecutionSummary<T> {
    pub executed: Vec<(FieldId, FieldRegionId)>,
    pub results: Vec<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScheduledSingleStencilExecution {
    pub field_id: FieldId,
    pub region_id: FieldRegionId,
    pub report: StructuredFieldExecutionReport,
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn every_n_zero_rejected() {
        assert_eq!(
            FieldCadence::EveryN { n: 0 }.validate(),
            Err(FieldSchedulerError::InvalidEveryNZero)
        );
    }

    #[test]
    fn cadence_counts_over_120_ticks() {
        let ticks = 120;
        assert_eq!(
            count_cadence_due_ticks(FieldCadence::EveryTick, ticks, &[]).unwrap(),
            120
        );
        assert_eq!(
            count_cadence_due_ticks(FieldCadence::EveryN { n: 4 }, ticks, &[]).unwrap(),
            30
        );
        assert_eq!(
            count_cadence_due_ticks(FieldCadence::EveryN { n: 10 }, ticks, &[]).unwrap(),
            12
        );
        assert_eq!(
            count_cadence_due_ticks(FieldCadence::EveryN { n: 60 }, ticks, &[]).unwrap(),
            2
        );
        assert_eq!(
            count_cadence_due_ticks(FieldCadence::OnEvent, ticks, &[5, 17, 42]).unwrap(),
            3
        );
    }

    #[test]
    fn region_identity_is_field_scoped() {
        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: FieldId(1),
            cadence: FieldCadence::EveryTick,
            event_pending: false,
        });
        scheduler.register_field(FieldScheduleState {
            field_id: FieldId(2),
            cadence: FieldCadence::EveryTick,
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(1),
            dirty: DirtyRegionState::default(),
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(2),
            dirty: DirtyRegionState::default(),
        });
        assert_eq!(scheduler.regions().len(), 2);
        let (decisions, _) = scheduler.decide_tick(0).unwrap();
        assert_eq!(decisions.len(), 2);
    }

    #[test]
    fn same_field_region_replacement_updates_state() {
        let mut scheduler = FieldScheduler::new();
        scheduler.register_field(FieldScheduleState {
            field_id: FieldId(1),
            cadence: FieldCadence::EveryN { n: 100 },
            event_pending: false,
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(1),
            dirty: DirtyRegionState::default(),
        });
        scheduler.register_region(FieldRegionRegistration {
            region_id: FieldRegionId(0),
            field_id: FieldId(1),
            dirty: DirtyRegionState {
                dirty_source_present: true,
                ..Default::default()
            },
        });
        assert_eq!(scheduler.regions().len(), 1);
        let (decisions, _) = scheduler.decide_tick(1).unwrap();
        assert_eq!(decisions.len(), 1);
        assert!(matches!(
            decisions[0].schedule,
            FieldDispatchSchedule::Dispatch
        ));
    }

    #[test]
    fn visit_scheduled_regions_skips_and_counts() {
        let decisions = [
            FieldDispatchDecision {
                region_id: FieldRegionId(1),
                field_id: FieldId(0),
                schedule: FieldDispatchSchedule::Skip,
                reasons: vec![],
            },
            FieldDispatchDecision {
                region_id: FieldRegionId(2),
                field_id: FieldId(0),
                schedule: FieldDispatchSchedule::Dispatch,
                reasons: vec![FieldDispatchReason::DirtySource],
            },
            FieldDispatchDecision {
                region_id: FieldRegionId(3),
                field_id: FieldId(1),
                schedule: FieldDispatchSchedule::Dispatch,
                reasons: vec![FieldDispatchReason::CadenceDue],
            },
        ];
        let mut calls = 0u32;
        let executed = visit_scheduled_regions(&decisions, |_d| {
            calls += 1;
            Ok::<(), ()>(())
        })
        .unwrap();
        assert_eq!(calls, 2);
        assert_eq!(
            executed,
            vec![(FieldId(0), FieldRegionId(2)), (FieldId(1), FieldRegionId(3))]
        );
    }
}
