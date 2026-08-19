//! CONTENTION-CONSERVATION-JUDGE-0 — oracle-first conservation referee.
//!
//! Judges declared accounting snapshots only. Never allocates, never clears,
//! never chooses a winner. Reconstruction uses only `reduce_owner_channel_rf`
//! and `reconstruct_owner_channel_rf_map`. Distinct owners in one container
//! are normal. The production snapshot carries no field whose purpose is to
//! make this judge return a wrong answer.

use simthing_core::{cost_band::CostBandDraw, GenerationStamp, SimThing};

use super::channel_key::ResourceKey;
use super::owner_channel_rf::{
    reconstruct_owner_channel_rf_map, reduce_owner_channel_rf, OwnerChannelRfError,
    OwnerChannelRfOwnAggregate, OwnerChannelRfSeamBalance,
};

/// Named production-judge RED reasons on the ordinary accounting path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConservationJudgeReason {
    SeededOverAccounting,
    SeededUnderAccounting,
    ChildParentOnly,
    StemThingPartition,
    ActionBandOmission,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConservationVerdict {
    Green,
    Red(ConservationJudgeReason),
}

/// Per-resource bounded supply versus granted + remainder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelBound {
    pub resource: ResourceKey,
    pub supply: u32,
    pub remainder: u32,
}

/// Quantized input draw plus created output on a distinct channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuantizedChannelObservation {
    pub input: CostBandDraw,
    pub output_created: u32,
}

/// 6.2 seam observation. Conservation is `child + seam + parent == admitted`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeamObservation {
    pub balance: OwnerChannelRfSeamBalance,
}

/// StemThing-A partition numbers consumed as observations, not owned here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StemThingPartitionObservation {
    pub free: u64,
    pub in_flight: u64,
    pub occupied: u64,
    pub capacity: u64,
}

/// Declared conservation universe for one referee act.
pub struct ConservationSnapshot<'a> {
    pub root: &'a SimThing,
    pub own_aggregates: &'a [OwnerChannelRfOwnAggregate],
    pub channels: &'a [ChannelBound],
    pub quantized: Option<QuantizedChannelObservation>,
    pub seam: Option<SeamObservation>,
    pub stemthing: Option<StemThingPartitionObservation>,
    /// ActionBand-originated ordinary claims that must sit in `own_aggregates`.
    pub actionband_originated: &'a [OwnerChannelRfOwnAggregate],
}

/// Production conservation judge. Ordinary-path verdicts only.
pub fn judge_conservation(
    snapshot: &ConservationSnapshot<'_>,
) -> Result<ConservationVerdict, OwnerChannelRfError> {
    let stamped = reduce_owner_channel_rf(
        snapshot.root,
        snapshot.own_aggregates,
        GenerationStamp::new(0),
    )?;
    let report = stamped.product();
    let reconstructed = reconstruct_owner_channel_rf_map(snapshot.root, &report.stead)?;

    for originated in snapshot.actionband_originated {
        let present = snapshot.own_aggregates.iter().any(|row| {
            row.simthing_id == originated.simthing_id
                && row.resource_key == originated.resource_key
                && row.surplus == originated.surplus
                && row.deficit == originated.deficit
        });
        if !present {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::ActionBandOmission,
            ));
        }
    }

    for channel in snapshot.channels {
        let granted = reconstructed
            .iter()
            .filter(|bucket| bucket.scope.resource_key == channel.resource)
            .try_fold(0u32, |acc, bucket| acc.checked_add(bucket.surplus_total));
        let Some(granted) = granted else {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::SeededOverAccounting,
            ));
        };
        let Some(accounted) = granted.checked_add(channel.remainder) else {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::SeededOverAccounting,
            ));
        };
        if accounted > channel.supply {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::SeededOverAccounting,
            ));
        }
        if accounted < channel.supply {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::SeededUnderAccounting,
            ));
        }
    }

    if let Some(quantized) = snapshot.quantized {
        if !quantized.input.conserves_exactly() {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::SeededOverAccounting,
            ));
        }
    }

    if let Some(seam) = snapshot.seam {
        if !seam.balance.is_exact() {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::ChildParentOnly,
            ));
        }
    }

    if let Some(stemthing) = snapshot.stemthing {
        let sum = stemthing
            .free
            .checked_add(stemthing.in_flight)
            .and_then(|value| value.checked_add(stemthing.occupied));
        if sum != Some(stemthing.capacity) {
            return Ok(ConservationVerdict::Red(
                ConservationJudgeReason::StemThingPartition,
            ));
        }
    }

    Ok(ConservationVerdict::Green)
}
