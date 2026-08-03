//! SIMTHING-AUTOMATON-INTRINSIC-0 command-deficit reception.
//!
//! This is an arrival hook on the existing owner-silo disburse-down -> runtime
//! local-allocation seam. A command deficit is one conserved allocation unit;
//! only a supplied unit delivers its attributable overlay to the consumer.

use std::collections::BTreeMap;

use simthing_core::{
    deliver_deficit_directive, DirectiveDeliveryReceipt, Overlay, OverlayDeliveryError, SimThing,
    SimThingId,
};
use simthing_spec::{
    apply_owner_silo_runtime_disburse_down_cpu, apply_runtime_local_allocations_from_disburse_down,
    OwnerRef, ResourceKey, RuntimeLocalAllocationApplicationReport, RuntimeOwnerSiloDemandBucket,
    RuntimeOwnerSiloDisburseDownResult, RuntimeOwnerSiloWritebackResult, ScopeId,
};
use thiserror::Error;

/// One single-use directive requested by a receiving SimThing.
///
/// The unit amount is structural: one deficit consumes one disbursement unit.
/// Standing directives do not use this type and make no conservation claim.
#[derive(Clone, Debug)]
pub struct CommandDeficit {
    pub receiver: SimThingId,
    pub owner_ref: OwnerRef,
    pub resource_key: ResourceKey,
    pub scope_id: ScopeId,
    pub priority: u32,
    pub directive: Overlay,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandDeficitReceptionReport {
    pub disburse_down_results: Vec<RuntimeOwnerSiloDisburseDownResult>,
    pub local_allocation: RuntimeLocalAllocationApplicationReport,
    pub deliveries: Vec<DirectiveDeliveryReceipt>,
    pub unmet_command_count: u32,
}

#[derive(Debug, Error)]
pub enum CommandDeficitReceptionError {
    #[error("duplicate command deficit for receiver {receiver:?}")]
    DuplicateDeficit { receiver: SimThingId },
    #[error("command-deficit receiver {receiver:?} is not in the supplied tree")]
    ReceiverNotInTree { receiver: SimThingId },
    #[error("command-deficit directive origin {origin:?} is not in the supplied tree")]
    OriginNotInTree { origin: SimThingId },
    #[error("owner-silo disburse-down rejected command deficits: {0}")]
    DisburseDown(String),
    #[error("runtime local allocation rejected disbursement: {0}")]
    LocalAllocation(String),
    #[error("allocated command has no matching admitted deficit for receiver {receiver:?}")]
    MissingDeficit { receiver: SimThingId },
    #[error(transparent)]
    Delivery(#[from] OverlayDeliveryError),
}

type DeficitKey = (OwnerRef, ResourceKey, ScopeId, u32);

/// Run command deficits through the existing conserved disburse-down and local
/// allocation path, then terminate supplied commands in `SimThing.overlays`.
///
/// Removing the final delivery loop is the production-seam planted defect: RF
/// allocation still succeeds, but no SimThing receives its directive.
pub fn receive_command_deficits_from_disbursement(
    root: &mut SimThing,
    writeback_results: &[RuntimeOwnerSiloWritebackResult],
    deficits: &[CommandDeficit],
) -> Result<CommandDeficitReceptionReport, CommandDeficitReceptionError> {
    let mut admitted = BTreeMap::<DeficitKey, &CommandDeficit>::new();
    let mut demand_buckets = Vec::with_capacity(deficits.len());
    for deficit in deficits {
        if !contains(root, deficit.receiver) {
            return Err(CommandDeficitReceptionError::ReceiverNotInTree {
                receiver: deficit.receiver,
            });
        }
        if !contains(root, deficit.directive.origin) {
            return Err(CommandDeficitReceptionError::OriginNotInTree {
                origin: deficit.directive.origin,
            });
        }
        let key = (
            deficit.owner_ref.clone(),
            deficit.resource_key.clone(),
            deficit.scope_id.clone(),
            deficit.receiver.raw(),
        );
        if admitted.insert(key, deficit).is_some() {
            return Err(CommandDeficitReceptionError::DuplicateDeficit {
                receiver: deficit.receiver,
            });
        }
        demand_buckets.push(RuntimeOwnerSiloDemandBucket {
            owner_ref: deficit.owner_ref.clone(),
            resource_key: deficit.resource_key.clone(),
            scope_id: deficit.scope_id.clone(),
            requested: 1,
            priority: deficit.priority,
            source_simthing_id_raw: Some(deficit.receiver.raw()),
        });
    }

    let disburse_down_results = if demand_buckets.is_empty() {
        Vec::new()
    } else {
        apply_owner_silo_runtime_disburse_down_cpu(writeback_results, &demand_buckets)
            .map_err(|error| CommandDeficitReceptionError::DisburseDown(error.message))?
    };
    let local_allocation =
        apply_runtime_local_allocations_from_disburse_down(&disburse_down_results)
            .map_err(|error| CommandDeficitReceptionError::LocalAllocation(error.message))?;

    let mut deliveries = Vec::new();
    for allocation in &local_allocation.states {
        if allocation.allocated == 0 {
            continue;
        }
        let key = (
            allocation.owner_ref.clone(),
            allocation.resource_key.clone(),
            allocation.scope_id.clone(),
            allocation.source_simthing_id_raw,
        );
        let deficit =
            admitted
                .get(&key)
                .ok_or_else(|| CommandDeficitReceptionError::MissingDeficit {
                    receiver: SimThingId::from_session_raw(allocation.source_simthing_id_raw),
                })?;
        deliveries.push(deliver_deficit_directive(
            root,
            deficit.receiver,
            deficit.directive.clone(),
        )?);
    }

    Ok(CommandDeficitReceptionReport {
        disburse_down_results,
        unmet_command_count: local_allocation.unmet_total,
        local_allocation,
        deliveries,
    })
}

fn contains(node: &SimThing, target: SimThingId) -> bool {
    node.id == target || node.children.iter().any(|child| contains(child, target))
}
