//! TP-FRONTS-AUTHORING-0 — workshop-homed Movement-Front L1/L2/L3 over the contested border.
//!
//! Applied after generic `hydrate_scenario`; threat / suppression / disruption semantics live here,
//! not in `simthing-clausething/src`.

use std::collections::BTreeSet;

use simthing_clausething::{
    HydratedScenarioGridMetadata, HydratedScenarioGridPlacement, HydratedScenarioPack,
};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, ClampBehavior, LogTier, SimThing, SimThingId,
    SimThingKind, SubFieldRole, SubFieldSpec,
};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::region_field::{
    ArenaPressureBindingSpec, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec, RegionFieldCadenceSpec,
    RegionFieldFormulaBindingSpec, RegionFieldGridProfile, RegionFieldOperatorSpec,
    RegionFieldReductionSpec, RegionFieldSourcePolicySpec, RegionFieldSpec,
};
use simthing_spec::spec::resource_flow::{
    BaseFlowDirectionSpec, BaseFlowObligationSpec, ResourceFlowOptInMode, ResourceFlowSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::spec::PropertySpec;
use simthing_spec::{
    compile_region_field_preview, is_galaxy_map_entity, ArenaSpec, EnrollmentSelectorSpec,
    ExplicitParticipantSpec, FissionPolicySpec, FIRST_SLICE_FIELD_URGENCY_COL,
    REGION_FIELD_STANDARD_MAX_GRID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_ROW_PROPERTY_ID,
};

/// Workshop-local threat-front RF arena id (pirate raid pressure).
pub const TP_THREAT_ARENA: &str = "tp_front_threat";

/// Workshop-local suppression-front RF arena id (terran patrol / fleet presence).
pub const TP_SUPPRESSION_ARENA: &str = "tp_front_suppression";

/// Workshop-local disruption-front RF arena id (pirate posture feedstock).
pub const TP_DISRUPTION_ARENA: &str = "tp_front_disruption";

/// Property namespace for front RF flow columns.
pub const TP_FRONTS_PROPERTY_NAMESPACE: &str = "tp_front";

/// SaturatingFlux field-operator id stamped on the authored region field.
pub const TP_FRONTS_FIELD_OPERATOR_ID: &str = "tp_contested_border_front";

/// Default bounded L1 horizon for the border theater (STEAD §7 P1).
pub const TP_FRONTS_DEFAULT_HORIZON: u32 = 3;

/// SaturatingFlux source column for the contested-border slice.
pub const TP_FRONTS_SOURCE_COL: u32 = 0;

/// Choke output column for SaturatingFlux in the border slice.
pub const TP_FRONTS_CHOKE_OUTPUT_COL: u32 = 2;

/// Column width for the border theater field operator.
pub const TP_FRONTS_N_DIMS: u32 = 6;

/// Maximum border systems enrolled per faction in the bounded theater.
pub const TP_FRONTS_MAX_SYSTEMS_PER_SIDE: usize = 1;

/// Fallback intrinsic-flow seed when fleet payloads are absent.
pub const DEFAULT_THREAT_INTRINSIC_RATE: f32 = 30.0;
pub const DEFAULT_SUPPRESSION_INTRINSIC_RATE: f32 = 40.0;
pub const DEFAULT_DISRUPTION_INTRINSIC_RATE: f32 = 28.0;

/// L3 `field_urgency` formula weights (pressure-dominant for this proof slice).
pub const TP_FRONTS_WEIGHT_PRESSURE: f32 = 1.0;
pub const TP_FRONTS_WEIGHT_RESOURCE: f32 = 0.25;

/// High probe threshold — satisfies session-loop admission without Phase 7 semantics.
pub const TP_FRONTS_PROBE_THRESHOLD: f32 = 10_000.0;

/// Workshop-local probe event kind for commitment admission only (not Phase 7).
pub const TP_FRONTS_PROBE_EVENT_KIND: u32 = 0x4652_4F4E; // "FRON"

#[derive(Debug, Clone)]
pub struct TpFrontsTheaterCell {
    pub target_id: String,
    pub theater_row: u32,
    pub theater_col: u32,
    pub owner: String,
    pub simthing_id: SimThingId,
    pub threat_rate: f32,
    pub suppression_rate: f32,
    pub disruption_rate: f32,
}

#[derive(Debug, Clone)]
pub struct TpFrontsAuthoringReport {
    pub grid_size: u32,
    pub theater_cells: Vec<TpFrontsTheaterCell>,
    pub region_field: RegionFieldSpec,
    pub threat_binding: ArenaPressureBindingSpec,
    pub suppression_binding: ArenaPressureBindingSpec,
    pub disruption_binding: ArenaPressureBindingSpec,
}

#[derive(Debug, thiserror::Error)]
pub enum FrontsHydrationError {
    #[error("fronts post-hydration requires authority_root")]
    MissingAuthorityRoot,
    #[error("fronts post-hydration requires terran and pirate ownership volumes")]
    MissingOwnershipVolumes,
    #[error("fronts post-hydration requires at least one contested border system")]
    MissingContestedBorder,
    #[error("{0}")]
    Message(String),
}

/// Workshop-side Movement-Front authoring over a generic hydrated TP pack.
pub fn apply_fronts_post_hydration(
    pack: &mut HydratedScenarioPack,
) -> Result<TpFrontsAuthoringReport, FrontsHydrationError> {
    if pack.authority_root.is_none() {
        return Err(FrontsHydrationError::MissingAuthorityRoot);
    }
    let terran = pack
        .ownership_volumes
        .iter()
        .find(|volume| volume.id == "terran_core")
        .ok_or(FrontsHydrationError::MissingOwnershipVolumes)?;
    let pirate = pack
        .ownership_volumes
        .iter()
        .find(|volume| volume.id == "pirate_border")
        .ok_or(FrontsHydrationError::MissingOwnershipVolumes)?;

    let root = pack
        .authority_root
        .as_ref()
        .expect("authority root checked above");
    let theater_cells = build_theater_cells(root, terran, pirate, pack)?;
    if theater_cells.is_empty() {
        return Err(FrontsHydrationError::MissingContestedBorder);
    }

    let terran_rows = theater_cells
        .iter()
        .filter(|cell| cell.owner == "terran")
        .count();
    let pirate_rows = theater_cells
        .iter()
        .filter(|cell| cell.owner == "pirate")
        .count();
    let grid_size = theater_grid_size(terran_rows.max(pirate_rows))?;
    install_front_resource_flow(pack, &theater_cells)?;
    let surfaces = build_front_surfaces(&theater_cells, grid_size)?;
    compile_region_field_preview(&surfaces.region_field).map_err(|err| {
        FrontsHydrationError::Message(format!("region field admission rejected: {err}"))
    })?;

    pack.grid_metadata = build_theater_grid_metadata(&theater_cells, grid_size);
    pack.game_mode.region_fields = vec![surfaces.region_field.clone()];
    pack.game_mode.mapping_execution_profile = MappingExecutionProfile::SparseRegionFieldV1;
    pack.w_impedance_compose = None;
    pack.stress_compose = None;
    pack.palma_feedstock = None;
    pack.commitment = None;

    for cell in &theater_cells {
        pack.install_targets
            .entry(cell.target_id.clone())
            .or_default()
            .push(cell.simthing_id);
    }

    Ok(TpFrontsAuthoringReport {
        grid_size,
        theater_cells,
        region_field: surfaces.region_field,
        threat_binding: surfaces.threat_binding,
        suppression_binding: surfaces.suppression_binding,
        disruption_binding: surfaces.disruption_binding,
    })
}

struct BuiltFrontSurfaces {
    region_field: RegionFieldSpec,
    threat_binding: ArenaPressureBindingSpec,
    suppression_binding: ArenaPressureBindingSpec,
    disruption_binding: ArenaPressureBindingSpec,
}

fn build_front_surfaces(
    theater_cells: &[TpFrontsTheaterCell],
    grid_size: u32,
) -> Result<BuiltFrontSurfaces, FrontsHydrationError> {
    let cell_count = grid_size.saturating_mul(grid_size);
    let placements: Vec<PressurePlacementSpec> = theater_cells
        .iter()
        .map(|cell| PressurePlacementSpec {
            target_id: cell.target_id.clone(),
            row: cell.theater_row,
            col: cell.theater_col,
        })
        .collect();

    let threat_binding = ArenaPressureBindingSpec {
        arena: TP_THREAT_ARENA.into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements: placements.clone(),
    };
    let suppression_binding = ArenaPressureBindingSpec {
        arena: TP_SUPPRESSION_ARENA.into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements: placements.clone(),
    };
    let disruption_binding = ArenaPressureBindingSpec {
        arena: TP_DISRUPTION_ARENA.into(),
        source: PressureSourceSpec::IntrinsicFlow,
        placements,
    };

    let parent_formula = RegionFieldFormulaBindingSpec {
        formula_class: "field_urgency".into(),
        tree_id: None,
        weight_pressure: Some(TP_FRONTS_WEIGHT_PRESSURE),
        weight_resource: Some(TP_FRONTS_WEIGHT_RESOURCE),
    };
    let reduction = RegionFieldReductionSpec {
        child_slot_start: 0,
        child_slot_count: cell_count,
        child_col: TP_FRONTS_SOURCE_COL,
        parent_slot: cell_count,
        parent_col: TP_FRONTS_SOURCE_COL,
        order_band: 0,
    };
    let commitment = FirstSliceCommitmentSpec {
        source_formula_class: "field_urgency".into(),
        parent_slot: cell_count,
        urgency_col: FIRST_SLICE_FIELD_URGENCY_COL,
        threshold: TP_FRONTS_PROBE_THRESHOLD,
        direction: FirstSliceCommitmentDirectionSpec::Upward,
        event_kind: TP_FRONTS_PROBE_EVENT_KIND,
        effect: None,
    };

    let region_field = RegionFieldSpec {
        name: format!("{TP_FRONTS_FIELD_OPERATOR_ID}_field"),
        grid_size,
        n_dims: TP_FRONTS_N_DIMS,
        source_col: TP_FRONTS_SOURCE_COL,
        target_col: TP_FRONTS_SOURCE_COL,
        operator: RegionFieldOperatorSpec::SaturatingFlux {
            u_sat: 1.0,
            chi: 0.25,
            choke_output_col: Some(TP_FRONTS_CHOKE_OUTPUT_COL),
        },
        horizon: TP_FRONTS_DEFAULT_HORIZON,
        allow_extended_horizon: false,
        alpha_self: 1.0,
        gamma_neighbor: 0.5,
        source_cap: None,
        source_policy: RegionFieldSourcePolicySpec::CallerManagedOneShotSeedThenZero,
        cadence: RegionFieldCadenceSpec::EveryTick,
        grid_profile: RegionFieldGridProfile::StandardSquare,
        reduction: Some(reduction),
        parent_formula: Some(parent_formula),
        commitment: Some(commitment),
        request_atlas_batching: false,
        max_region_field_vram_bytes: None,
        summary_policy: Default::default(),
        pressure_binding: Some(suppression_binding.clone()),
    };

    Ok(BuiltFrontSurfaces {
        region_field,
        threat_binding,
        suppression_binding,
        disruption_binding,
    })
}

fn install_front_resource_flow(
    pack: &mut HydratedScenarioPack,
    theater_cells: &[TpFrontsTheaterCell],
) -> Result<(), FrontsHydrationError> {
    let participants: Vec<ExplicitParticipantSpec> = theater_cells
        .iter()
        .map(|cell| ExplicitParticipantSpec::flat(0, cell.simthing_id.raw()))
        .collect();

    let threat_key = PropertyKey::new(TP_FRONTS_PROPERTY_NAMESPACE, "threat_flow");
    let suppression_key = PropertyKey::new(TP_FRONTS_PROPERTY_NAMESPACE, "suppression_flow");
    let disruption_key = PropertyKey::new(TP_FRONTS_PROPERTY_NAMESPACE, "disruption_flow");

    for property in [
        flow_property_spec("tp_front_threat_flow", "threat_flow", TP_THREAT_ARENA),
        flow_property_spec("tp_front_suppression_flow", "suppression_flow", TP_SUPPRESSION_ARENA),
        flow_property_spec("tp_front_disruption_flow", "disruption_flow", TP_DISRUPTION_ARENA),
    ] {
        if !pack
            .game_mode
            .properties
            .iter()
            .any(|existing| existing.id == property.id)
        {
            pack.game_mode.properties.push(property);
        }
    }

    let arenas = vec![
        front_arena_spec(TP_THREAT_ARENA, threat_key, participants.clone()),
        front_arena_spec(TP_SUPPRESSION_ARENA, suppression_key, participants.clone()),
        front_arena_spec(TP_DISRUPTION_ARENA, disruption_key, participants),
    ];
    for arena in &arenas {
        validate_front_arena_caps(arena)?;
    }

    let mut base_obligations = Vec::new();
    for (index, cell) in theater_cells.iter().enumerate() {
        if cell.threat_rate > 0.0 {
            base_obligations.push(base_obligation(
                format!("tp_front_threat_{index}"),
                TP_THREAT_ARENA,
                &cell.target_id,
                cell.threat_rate,
            ));
        }
        if cell.suppression_rate > 0.0 {
            base_obligations.push(base_obligation(
                format!("tp_front_suppression_{index}"),
                TP_SUPPRESSION_ARENA,
                &cell.target_id,
                cell.suppression_rate,
            ));
        }
        if cell.disruption_rate > 0.0 {
            base_obligations.push(base_obligation(
                format!("tp_front_disruption_{index}"),
                TP_DISRUPTION_ARENA,
                &cell.target_id,
                cell.disruption_rate,
            ));
        }
    }

    pack.game_mode.resource_flow = Some(ResourceFlowSpec {
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        arenas,
        couplings: vec![],
        base_obligations,
        capacity_budget: None,
        gated_rates: vec![],
    });
    Ok(())
}

fn build_theater_cells(
    root: &SimThing,
    terran: &simthing_clausething::HydratedOwnershipVolume,
    pirate: &simthing_clausething::HydratedOwnershipVolume,
    pack: &HydratedScenarioPack,
) -> Result<Vec<TpFrontsTheaterCell>, FrontsHydrationError> {
    let terran_coords: BTreeSet<_> = terran
        .assigned_systems
        .iter()
        .map(|system| (system.row, system.col))
        .collect();
    let pirate_coords: BTreeSet<_> = pirate
        .assigned_systems
        .iter()
        .map(|system| (system.row, system.col))
        .collect();

    let mut terran_border = Vec::new();
    let mut pirate_border = Vec::new();
    for system in &terran.assigned_systems {
        if chebyshev_adjacent_to_any(system.row, system.col, &pirate_coords) {
            terran_border.push(system.clone());
        }
    }
    for system in &pirate.assigned_systems {
        if chebyshev_adjacent_to_any(system.row, system.col, &terran_coords) {
            pirate_border.push(system.clone());
        }
    }
    terran_border.sort_by_key(|system| (system.row, system.col, system.target_id.clone()));
    pirate_border.sort_by_key(|system| (system.row, system.col, system.target_id.clone()));
    terran_border.truncate(TP_FRONTS_MAX_SYSTEMS_PER_SIDE);
    pirate_border.truncate(TP_FRONTS_MAX_SYSTEMS_PER_SIDE);

    let threat_fallback = fleet_weapon_rate(pack, "pirate").unwrap_or(DEFAULT_THREAT_INTRINSIC_RATE);
    let suppression_fallback =
        fleet_weapon_rate(pack, "terran").unwrap_or(DEFAULT_SUPPRESSION_INTRINSIC_RATE);
    let disruption_fallback =
        fleet_weapon_rate(pack, "pirate").unwrap_or(DEFAULT_DISRUPTION_INTRINSIC_RATE);

    let mut cells = Vec::new();
    for (index, system) in terran_border.iter().enumerate() {
        let simthing_id = resolve_system_id(root, system.row, system.col).ok_or_else(|| {
            FrontsHydrationError::Message(format!(
                "contested terran border system `{}` missing from authority tree",
                system.target_id
            ))
        })?;
        cells.push(TpFrontsTheaterCell {
            target_id: system.target_id.clone(),
            theater_row: index as u32,
            theater_col: 0,
            owner: system.owner_ref.clone(),
            simthing_id,
            threat_rate: 0.0,
            suppression_rate: suppression_fallback,
            disruption_rate: 0.0,
        });
    }
    for (index, system) in pirate_border.iter().enumerate() {
        let simthing_id = resolve_system_id(root, system.row, system.col).ok_or_else(|| {
            FrontsHydrationError::Message(format!(
                "contested pirate border system `{}` missing from authority tree",
                system.target_id
            ))
        })?;
        cells.push(TpFrontsTheaterCell {
            target_id: system.target_id.clone(),
            theater_row: index as u32,
            theater_col: 2,
            owner: system.owner_ref.clone(),
            simthing_id,
            threat_rate: threat_fallback,
            suppression_rate: 0.0,
            disruption_rate: disruption_fallback,
        });
    }

    Ok(cells)
}

fn build_theater_grid_metadata(
    theater_cells: &[TpFrontsTheaterCell],
    grid_size: u32,
) -> HydratedScenarioGridMetadata {
    let placements: Vec<HydratedScenarioGridPlacement> = theater_cells
        .iter()
        .map(|cell| HydratedScenarioGridPlacement {
            location_id: cell.target_id.clone(),
            target_id: cell.target_id.clone(),
            row: cell.theater_row,
            col: cell.theater_col,
        })
        .collect();
    HydratedScenarioGridMetadata {
        grid_size,
        max_fanout: 4,
        placements,
        links: vec![],
    }
}

fn theater_grid_size(max_rows: usize) -> Result<u32, FrontsHydrationError> {
    if max_rows == 0 {
        return Err(FrontsHydrationError::MissingContestedBorder);
    }
    let edge = (max_rows as u32).max(3);
    if edge > REGION_FIELD_STANDARD_MAX_GRID {
        return Err(FrontsHydrationError::Message(format!(
            "contested border theater requires atlas deferral: {max_rows} rows exceed \
             bounded theater cap {REGION_FIELD_STANDARD_MAX_GRID}"
        )));
    }
    Ok(edge)
}

fn chebyshev_adjacent_to_any(row: u32, col: u32, coords: &BTreeSet<(u32, u32)>) -> bool {
    coords.iter().any(|(other_row, other_col)| {
        row.abs_diff(*other_row) <= 1 && col.abs_diff(*other_col) <= 1 && (row, col) != (*other_row, *other_col)
    })
}

fn fleet_weapon_rate(pack: &HydratedScenarioPack, owner: &str) -> Option<f32> {
    pack.fleet_ship_payloads
        .iter()
        .find(|payload| payload.owner == owner)
        .map(|payload| payload.weapon_damage_seed * payload.ships_per_fleet as f32)
}

fn resolve_system_id(root: &SimThing, row: u32, col: u32) -> Option<SimThingId> {
    let session = root
        .children
        .iter()
        .find(|child| child.kind == SimThingKind::GameSession)?;
    let galaxy_map = session.children.iter().find(|child| is_galaxy_map_entity(child))?;
    for star_system in &galaxy_map.children {
        if system_structural_coord(star_system) == (row, col) {
            return Some(star_system.id);
        }
    }
    None
}

fn system_structural_coord(system: &SimThing) -> (u32, u32) {
    let row = system
        .properties
        .get(&SCENARIO_STRUCTURAL_ROW_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    let col = system
        .properties
        .get(&SCENARIO_STRUCTURAL_COL_PROPERTY_ID)
        .and_then(|value| value.raw_lanes().first().copied())
        .unwrap_or(0.0) as u32;
    (row, col)
}

fn front_arena_spec(
    arena_name: &str,
    flow_property: PropertyKey,
    participants: Vec<ExplicitParticipantSpec>,
) -> ArenaSpec {
    ArenaSpec {
        name: arena_name.into(),
        flow_property,
        balance_property: None,
        max_participants: (TP_FRONTS_MAX_SYSTEMS_PER_SIDE * 2) as u32,
        max_coupling_fanout: 4,
        max_orderband_depth: 8,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: participants,
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    }
}

fn base_obligation(
    id: String,
    arena: &str,
    target_id: &str,
    rate: f32,
) -> BaseFlowObligationSpec {
    BaseFlowObligationSpec {
        id,
        arena: arena.into(),
        install: InstallTargetSpec::ScenarioListed {
            target_id: target_id.into(),
        },
        direction: BaseFlowDirectionSpec::Produce,
        rate,
    }
}

fn flow_property_spec(id: &str, name: &str, arena_name: &str) -> PropertySpec {
    PropertySpec {
        id: id.into(),
        namespace: TP_FRONTS_PROPERTY_NAMESPACE.into(),
        name: name.into(),
        display_name: name.into(),
        description: format!("TP front RF flow property for arena `{arena_name}`"),
        sub_fields: vec![
            flow_subfield("flow", AccumulatorRole::IntrinsicFlow),
            flow_subfield(
                "allocated",
                AccumulatorRole::AllocatedFlow {
                    arena: arena_name.into(),
                },
            ),
            flow_subfield(
                "weight",
                AccumulatorRole::AllocatorWeight {
                    arena: arena_name.into(),
                },
            ),
        ],
    }
}

fn flow_subfield(name: &str, role: AccumulatorRole) -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named(name.into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: name.into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role,
            log_tier: LogTier::Summary,
        }),
    }
}

fn validate_front_arena_caps(arena: &ArenaSpec) -> Result<(), FrontsHydrationError> {
    if arena.max_participants == 0
        || arena.max_coupling_fanout == 0
        || arena.max_orderband_depth == 0
    {
        return Err(FrontsHydrationError::Message(format!(
            "arena `{}` missing positive RF caps",
            arena.name
        )));
    }
    if arena.explicit_participants.is_empty() {
        return Err(FrontsHydrationError::Message(format!(
            "arena `{}` missing explicit participants",
            arena.name
        )));
    }
    if arena.explicit_participants.len() as u32 > arena.max_participants {
        return Err(FrontsHydrationError::Message(format!(
            "arena `{}` participant count {} exceeds max_participants {}",
            arena.name,
            arena.explicit_participants.len(),
            arena.max_participants
        )));
    }
    Ok(())
}

/// Deterministic oracle for contested-border settling: mid-column pressure must be
/// strictly between the terran suppression side and the pirate disruption side.
pub fn contested_border_settling_oracle(
    field_values: &[f32],
    grid_size: u32,
    n_dims: u32,
    source_col: u32,
    theater_cells: &[TpFrontsTheaterCell],
) -> Result<(), String> {
    let idx = |slot: u32, col: u32| (slot * n_dims + col) as usize;
    let mut suppression_authored = 0.0f32;
    let mut disruption_authored = 0.0f32;
    let mut contested_mass = 0.0f32;
    let mut paired_rows = BTreeSet::new();
    for cell in theater_cells {
        match cell.theater_col {
            0 if cell.owner == "terran" => {
                suppression_authored += cell.suppression_rate;
                paired_rows.insert(cell.theater_row);
            }
            2 if cell.owner == "pirate" => {
                disruption_authored += cell.disruption_rate;
                paired_rows.insert(cell.theater_row);
            }
            _ => {}
        }
    }
    for row in paired_rows {
        let contested_slot = row * grid_size + 1;
        let value = field_values
            .get(idx(contested_slot, source_col))
            .copied()
            .ok_or_else(|| format!("missing contested slot {contested_slot}"))?;
        if !value.is_finite() {
            return Err(format!("non-finite contested value at slot {contested_slot}"));
        }
        contested_mass += value;
    }
    if suppression_authored <= disruption_authored {
        return Err(format!(
            "authored suppression pressure must exceed authored disruption pressure: suppression={suppression_authored} disruption={disruption_authored}"
        ));
    }
    if contested_mass <= 0.0 {
        return Err(format!(
            "contested middle column must carry non-zero settling pressure: contested={contested_mass}"
        ));
    }
    Ok(())
}

/// Expose the L3 urgency column index for workshop proofs.
pub fn fronts_l3_urgency_col() -> u32 {
    FIRST_SLICE_FIELD_URGENCY_COL
}