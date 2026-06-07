//! R2 structural substrate orchestration over rehearsal journal rows.

use simthing_gpu::GpuContext;

use crate::dress_rehearsal_r6c_integrated_run::{
    DressRehearsalR6cWorld, R1bStructuralEvent, R1bStructuralEventKind,
};
use crate::runtime_0080_0_r1b::{
    Runtime0080R1bFreeSlotMarkSource, Runtime0080R1bLocalBirthRequestSource,
};
use crate::runtime_0080_0_r1c_a::run_mark_session;
use crate::runtime_0080_0_r1c_b::run_allocation_session;
use crate::runtime_0080_0_r1c_c::run_membership_for_rehearsal_journal;
use crate::runtime_0080_0_r1c_d::run_staging_for_rehearsal_journal;
use crate::runtime_0080_0_r1c_e::run_apply_for_rehearsal_staging;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R2SubstrateOutcome {
    pub r1c_a_ok: bool,
    pub r1c_b_ok: bool,
    pub r1c_c_ok: bool,
    pub r1c_d_ok: bool,
    pub r1c_e_ok: bool,
    pub mark_rows: u32,
    pub allocation_rows: u32,
    pub membership_delta_rows: u32,
    pub compaction_rows: u32,
    pub lineage_rows: u32,
    pub slot_remap_rows: u32,
    pub compacted_slot_rows: u32,
    pub membership_remap_rows: u32,
    pub diagnostics: Vec<String>,
}

pub(crate) fn run_r2_structural_substrates(
    ctx: &GpuContext,
    world: &DressRehearsalR6cWorld,
    journal_events: &[R1bStructuralEvent],
    fleet_ids: &[String],
    system_indices: &[usize],
) -> R2SubstrateOutcome {
    let mut outcome = R2SubstrateOutcome {
        r1c_a_ok: false,
        r1c_b_ok: false,
        r1c_c_ok: false,
        r1c_d_ok: false,
        r1c_e_ok: false,
        mark_rows: 0,
        allocation_rows: 0,
        membership_delta_rows: 0,
        compaction_rows: 0,
        lineage_rows: 0,
        slot_remap_rows: 0,
        compacted_slot_rows: 0,
        membership_remap_rows: 0,
        diagnostics: Vec::new(),
    };

    let mark_sources = free_slot_mark_sources_from_events(journal_events);
    outcome.mark_rows = mark_sources.len() as u32;
    let fleet_slots = fleet_ids.len() as u32;

    let marker = match run_mark_session(ctx, fleet_slots, &mark_sources, true) {
        Ok(marker) => marker,
        Err(diagnostic) => {
            outcome.diagnostics.push(diagnostic.to_string());
            return outcome;
        }
    };
    outcome.r1c_a_ok = marker.mark_parity_measured_from_gpu_values && marker.mark_source_rows > 0;

    let local_birth_requests = local_birth_request_sources_from_events(journal_events);
    let allocation = match run_allocation_session(
        ctx,
        fleet_slots,
        &marker.gpu_marked_slots,
        &local_birth_requests,
        true,
    ) {
        Ok(allocation) => allocation,
        Err(diagnostic) => {
            outcome.diagnostics.push(diagnostic.to_string());
            return outcome;
        }
    };
    outcome.allocation_rows = allocation.rows.len() as u32;
    outcome.r1c_b_ok = allocation.allocation_parity_measured_from_gpu_values
        && allocation.allocation_rows_written_from_gpu_values;

    let membership = match run_membership_for_rehearsal_journal(
        ctx,
        world,
        fleet_ids,
        system_indices,
        journal_events,
        &allocation.rows,
    ) {
        Ok(membership) => membership,
        Err(diagnostic) => {
            outcome.diagnostics.push(diagnostic.to_string());
            return outcome;
        }
    };
    outcome.membership_delta_rows = membership.delta_rows.len() as u32;
    outcome.r1c_c_ok = membership.membership_parity_measured_from_gpu_values;

    let staging = match run_staging_for_rehearsal_journal(
        ctx,
        journal_events,
        &allocation.rows,
        &membership.delta_rows,
    ) {
        Ok(staging) => staging,
        Err(diagnostic) => {
            outcome.diagnostics.push(diagnostic.to_string());
            return outcome;
        }
    };
    outcome.compaction_rows = staging.compaction_rows.len() as u32;
    outcome.lineage_rows = staging.lineage_rows.len() as u32;
    outcome.r1c_d_ok = staging.compaction_parity && staging.lineage_parity;

    let apply = match run_apply_for_rehearsal_staging(
        ctx,
        &staging.compaction_rows,
        &staging.lineage_rows,
        &membership.delta_rows,
    ) {
        Ok(apply) => apply,
        Err(diagnostic) => {
            outcome.diagnostics.push(diagnostic.to_string());
            return outcome;
        }
    };
    outcome.slot_remap_rows = apply.slot_remap_rows.len() as u32;
    outcome.compacted_slot_rows = apply.compacted_slot_rows.len() as u32;
    outcome.membership_remap_rows = apply.membership_remap_rows.len() as u32;
    outcome.r1c_e_ok = apply.remap_parity && apply.compacted_parity && apply.membership_parity;
    outcome
}

fn free_slot_mark_sources_from_events(
    events: &[R1bStructuralEvent],
) -> Vec<Runtime0080R1bFreeSlotMarkSource> {
    let mut rows = events
        .iter()
        .filter_map(|event| match event.event_kind {
            R1bStructuralEventKind::ZeroCohort => Some(Runtime0080R1bFreeSlotMarkSource {
                tick: event.tick,
                slot: event.source_slot,
                reason: "zero_cohort_departure",
                source_event_kind: event_kind_name(event.event_kind),
            }),
            R1bStructuralEventKind::FusionRequest => Some(Runtime0080R1bFreeSlotMarkSource {
                tick: event.tick,
                slot: event.target_slot,
                reason: "fusion_absorbed_slot",
                source_event_kind: event_kind_name(event.event_kind),
            }),
            _ => None,
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| (row.tick, row.slot, row.reason));
    rows
}

fn local_birth_request_sources_from_events(
    events: &[R1bStructuralEvent],
) -> Vec<Runtime0080R1bLocalBirthRequestSource> {
    let mut rows = Vec::new();
    for event in events
        .iter()
        .filter(|event| event.event_kind == R1bStructuralEventKind::LocalBirthRequest)
    {
        rows.push(Runtime0080R1bLocalBirthRequestSource {
            tick: event.tick,
            request_index: rows.len() as u32,
            owner_code: event.owner_code,
            source_cell: event.source_cell,
            requested_ships: event.amount_or_delta,
            source_event_kind: event_kind_name(event.event_kind),
        });
    }
    rows.sort_by_key(|row| (row.tick, row.request_index, row.source_cell));
    rows
}

fn event_kind_name(kind: R1bStructuralEventKind) -> &'static str {
    match kind {
        R1bStructuralEventKind::MoveRequest => "MoveRequest",
        R1bStructuralEventKind::DamageDelta => "DamageDelta",
        R1bStructuralEventKind::ShipCountDelta => "ShipCountDelta",
        R1bStructuralEventKind::OwnerCodeFlip => "OwnerCodeFlip",
        R1bStructuralEventKind::LocalBirthRequest => "LocalBirthRequest",
        R1bStructuralEventKind::FusionRequest => "FusionRequest",
        R1bStructuralEventKind::ZeroCohort => "ZeroCohort",
    }
}
