//! TP-COMMITMENTS-0 — workshop-homed STEAD commitment surfaces over L3 field_urgency.
//!
//! Chains accepted fleet-movement theater (fronts + PALMA + 7×7) and authors per-faction
//! `ai_will_do` / `ai_weight` personality profiles. Commitments fire via Threshold +
//! EmitEvent only — no CPU planner, no scripted timers.

use simthing_clausething::HydratedScenarioPack;
use simthing_core::{ClampBehavior, SubFieldRole, SubFieldSpec, TransformOp};
use simthing_spec::spec::region_field::{
    CommitmentEffectSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    RegionFieldFormulaBindingSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::spec::PropertySpec;
use simthing_spec::{
    compile_region_field_preview, CompiledFirstSliceCommitmentThreshold, FIRST_SLICE_FIELD_URGENCY_COL,
};

use crate::fleet_movement_post_hydration::{
    apply_fleet_movement_post_hydration, FleetMovementHydrationError, TpFleetMovementAuthoringReport,
};
use crate::fronts_post_hydration::{TpFrontsTheaterCell, TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE};

/// Workshop commitment marker property namespace.
pub const TP_COMMITMENT_PROPERTY_NAMESPACE: &str = "tp_commitment";

/// Commitment type labels (authored event taxonomy — not route objects).
pub const TP_COMMITMENT_TYPE_ATTACK: &str = "attack";
pub const TP_COMMITMENT_TYPE_REINFORCE: &str = "reinforce";
pub const TP_COMMITMENT_TYPE_RAID: &str = "raid";
pub const TP_COMMITMENT_TYPE_WITHDRAW: &str = "withdraw";
pub const TP_COMMITMENT_TYPE_FORTIFY: &str = "fortify";

/// Terran personality: reinforce / fortify / attack from border threat + suppression.
pub const TP_TERRAN_WEIGHT_PRESSURE: f32 = 0.40;
pub const TP_TERRAN_WEIGHT_RESOURCE: f32 = 1.0;

/// Pirate personality: raid / withdraw / attack from disruption + threat opportunity.
pub const TP_PIRATE_WEIGHT_PRESSURE: f32 = 1.0;
pub const TP_PIRATE_WEIGHT_RESOURCE: f32 = 0.20;

/// Authored upward thresholds over parent `field_urgency` (calibrated to 7×7 theater seeds).
pub const TP_TERRAN_REINFORCE_THRESHOLD: f32 = 4.0;
pub const TP_PIRATE_RAID_THRESHOLD: f32 = 4.0;

/// Event kinds for commitment crossings (opaque u32 — consumed at boundary only).
pub const TP_TERRAN_REINFORCE_EVENT_KIND: u32 = 0x5245_4E46; // "RENF"
pub const TP_PIRATE_RAID_EVENT_KIND: u32 = 0x5241_4944; // "RAID"
pub const TP_COMMITMENT_ATTACK_EVENT_KIND: u32 = 0x4154_544B; // "ATTK"
pub const TP_COMMITMENT_WITHDRAW_EVENT_KIND: u32 = 0x5749_5448; // "WITH"
pub const TP_COMMITMENT_FORTIFY_EVENT_KIND: u32 = 0x464F_5254; // "FORT"

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TpPersonalityUrgencyProfile {
    pub weight_pressure: f32,
    pub weight_resource: f32,
}

#[derive(Debug, Clone)]
pub struct TpFactionCommitmentSpec {
    pub faction: String,
    pub commitment_type: &'static str,
    pub event_kind: u32,
    pub threshold: f32,
    pub profile: TpPersonalityUrgencyProfile,
    pub effect_target_id: String,
    pub effect: CommitmentEffectSpec,
}

#[derive(Debug, Clone)]
pub struct TpCommitmentsAuthoringReport {
    pub movement: TpFleetMovementAuthoringReport,
    pub terran: TpFactionCommitmentSpec,
    pub pirate: TpFactionCommitmentSpec,
    pub parent_slot: u32,
    pub urgency_col: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum CommitmentsHydrationError {
    #[error(transparent)]
    Movement(#[from] FleetMovementHydrationError),
    #[error("{0}")]
    Message(String),
}

/// Workshop-side STEAD commitment authoring over accepted fleet-movement theater.
pub fn apply_commitments_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<TpCommitmentsAuthoringReport, CommitmentsHydrationError> {
    let movement = apply_fleet_movement_post_hydration(pack)?;
    install_commitment_marker_property(pack)?;

    let terran_cell = select_faction_cell(&movement.palma.fronts.theater_cells, "terran")
        .ok_or_else(|| CommitmentsHydrationError::Message("terran theater cell required".into()))?;
    let pirate_cell = select_faction_cell(&movement.palma.fronts.theater_cells, "pirate")
        .ok_or_else(|| CommitmentsHydrationError::Message("pirate theater cell required".into()))?;

    let terran = build_faction_commitment(
        "terran",
        TP_COMMITMENT_TYPE_REINFORCE,
        TP_TERRAN_REINFORCE_EVENT_KIND,
        TP_TERRAN_REINFORCE_THRESHOLD,
        TpPersonalityUrgencyProfile {
            weight_pressure: TP_TERRAN_WEIGHT_PRESSURE,
            weight_resource: TP_TERRAN_WEIGHT_RESOURCE,
        },
        terran_cell,
    );
    let pirate = build_faction_commitment(
        "pirate",
        TP_COMMITMENT_TYPE_RAID,
        TP_PIRATE_RAID_EVENT_KIND,
        TP_PIRATE_RAID_THRESHOLD,
        TpPersonalityUrgencyProfile {
            weight_pressure: TP_PIRATE_WEIGHT_PRESSURE,
            weight_resource: TP_PIRATE_WEIGHT_RESOURCE,
        },
        pirate_cell,
    );

    let grid = movement.palma.fronts.grid_size;
    let parent_slot = grid.saturating_mul(grid);
    patch_region_field_for_commitments(pack, &terran)?;

    Ok(TpCommitmentsAuthoringReport {
        movement,
        terran,
        pirate,
        parent_slot,
        urgency_col: FIRST_SLICE_FIELD_URGENCY_COL,
    })
}

fn select_faction_cell<'a>(
    cells: &'a [TpFrontsTheaterCell],
    owner: &str,
) -> Option<&'a TpFrontsTheaterCell> {
    cells.iter().find(|cell| cell.owner == owner)
}

fn build_faction_commitment(
    faction: &str,
    commitment_type: &'static str,
    event_kind: u32,
    threshold: f32,
    profile: TpPersonalityUrgencyProfile,
    cell: &TpFrontsTheaterCell,
) -> TpFactionCommitmentSpec {
    let marker_property = format!("{faction}_commitment_marker");
    TpFactionCommitmentSpec {
        faction: faction.into(),
        commitment_type,
        event_kind,
        threshold,
        profile,
        effect_target_id: cell.target_id.clone(),
        effect: CommitmentEffectSpec {
            target_id: cell.target_id.clone(),
            targets_property: format!("{TP_COMMITMENT_PROPERTY_NAMESPACE}::{marker_property}"),
            sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
            lifecycle: Default::default(),
            once: true,
        },
    }
}

fn install_commitment_marker_property(pack: &mut HydratedScenarioPack) -> Result<(), CommitmentsHydrationError> {
    for (id, name) in [
        ("tp_terran_commitment_marker", "terran_commitment_marker"),
        ("tp_pirate_commitment_marker", "pirate_commitment_marker"),
    ] {
        let property = PropertySpec {
            id: id.into(),
            namespace: TP_COMMITMENT_PROPERTY_NAMESPACE.into(),
            name: name.into(),
            display_name: name.into(),
            description: String::new(),
            sub_fields: vec![SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: name.into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            }],
        };
        if !pack
            .game_mode
            .properties
            .iter()
            .any(|existing| existing.id == property.id)
        {
            pack.game_mode.properties.push(property);
        }
    }
    Ok(())
}

fn patch_region_field_for_commitments(
    pack: &mut HydratedScenarioPack,
    terran: &TpFactionCommitmentSpec,
) -> Result<(), CommitmentsHydrationError> {
    let field = pack
        .game_mode
        .region_fields
        .first_mut()
        .ok_or_else(|| CommitmentsHydrationError::Message("commitments require region field".into()))?;
    field.parent_formula = Some(RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: None,
        weight_pressure: Some(terran.profile.weight_pressure),
        weight_resource: Some(terran.profile.weight_resource),
    });
    let grid = field.grid_size;
    let parent_slot = grid.saturating_mul(grid);
    field.commitment = Some(FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot,
        urgency_col: FIRST_SLICE_FIELD_URGENCY_COL,
        threshold: terran.threshold,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: terran.event_kind,
        effect: Some(terran.effect.clone()),
    });
    compile_region_field_preview(field).map_err(|err| {
        CommitmentsHydrationError::Message(format!("commitment field admission rejected: {err}"))
    })?;
    Ok(())
}

/// Apply a faction personality profile to the authored region field (ai_will_do weights).
pub fn patch_personality_profile(
    pack: &mut HydratedScenarioPack,
    profile: TpPersonalityUrgencyProfile,
) {
    if let Some(field) = pack.game_mode.region_fields.first_mut() {
        if let Some(formula) = field.parent_formula.as_mut() {
            formula.weight_pressure = Some(profile.weight_pressure);
            formula.weight_resource = Some(profile.weight_resource);
        }
    }
}

/// Build a compiled commitment threshold for GPU scan from a faction spec.
pub fn compiled_faction_commitment(
    report: &TpCommitmentsAuthoringReport,
    spec: &TpFactionCommitmentSpec,
) -> CompiledFirstSliceCommitmentThreshold {
    CompiledFirstSliceCommitmentThreshold {
        source_formula_class: "field_urgency".into(),
        parent_slot: report.parent_slot,
        urgency_col: report.urgency_col,
        threshold: spec.threshold,
        direction: simthing_spec::CompiledFirstSliceCommitmentDirection::Upward,
        event_kind: spec.event_kind,
    }
}

/// EML weight tuple for mapping tick dispatch.
pub fn personality_eml_weights(profile: TpPersonalityUrgencyProfile) -> (f32, f32) {
    (profile.weight_pressure, profile.weight_resource)
}

/// Default fronts proof weights (pressure-dominant baseline).
pub fn default_fronts_eml_weights() -> (f32, f32) {
    (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE)
}

pub fn terran_personality_profile() -> TpPersonalityUrgencyProfile {
    TpPersonalityUrgencyProfile {
        weight_pressure: TP_TERRAN_WEIGHT_PRESSURE,
        weight_resource: TP_TERRAN_WEIGHT_RESOURCE,
    }
}

pub fn pirate_personality_profile() -> TpPersonalityUrgencyProfile {
    TpPersonalityUrgencyProfile {
        weight_pressure: TP_PIRATE_WEIGHT_PRESSURE,
        weight_resource: TP_PIRATE_WEIGHT_RESOURCE,
    }
}

pub fn commitment_property_key(faction: &str) -> PropertyKey {
    PropertyKey::new(
        TP_COMMITMENT_PROPERTY_NAMESPACE,
        &format!("{faction}_commitment_marker"),
    )
}