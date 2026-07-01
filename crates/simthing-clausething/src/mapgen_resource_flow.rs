//! MapGen PR4 — bounded Resource Flow enrollment from the PR3 lattice hierarchy (M3).
//!
//! Lowers deposit feedstock and a suppression/disruption arena onto closed CT-2c / ResourceFlowSpec
//! surfaces with explicit selector admission, arena caps, and an expansion report. No Movement-Front,
//! PALMA, FIELD_POLICY, hyperlane coupling, or runtime/GPU/driver/simthing-sim surfaces.

use simthing_core::{
    validate_and_mint_placed_participants_by_location_id,
    validate_location_ids_have_structural_placements, AccumulatorRole, AccumulatorSpec,
    ClampBehavior, LogTier, PlacedParticipantValidationError, SimThing, SimThingId,
    StructuralGridPlacement, SubFieldRole, SubFieldSpec,
};
use simthing_spec::spec::install_target::InstallTargetSpec;
use simthing_spec::spec::resource_flow::{
    BaseFlowDirectionSpec, BaseFlowObligationSpec, CouplingDelaySpec, CouplingSpec,
    EnrollmentSelectorSpec, ResourceFlowCapacityBudgetSpec, ResourceFlowOptInMode,
    ResourceFlowSpec,
};
use simthing_spec::spec::script::PropertyKey;
use simthing_spec::{
    effective_resource_flow_arena_caps, resolve_resource_flow_capacity_budget,
    spatial_arena_explicit_participants, ArenaSpec, ExplicitParticipantSpec, FissionPolicySpec,
    PropertySpec,
};

use crate::hydrate_scenario::HydratedScenarioPack;
use crate::mapgen_lattice::{
    assert_allowed_simthing_kinds, collect_gridcell_location_ids, MapGenLatticeHierarchy,
};
use crate::mapgen_neutral_ast::MapGenNeutralDocument;

const FORBIDDEN_GENERATED_PROPERTY_NAMES: &[&str] = &[
    "route",
    "path",
    "pathfinding",
    "predecessor",
    "movement",
    "movement_order",
    "border",
    "frontline",
    "cpu_planner",
    "fleet_path",
];

/// MapGen PR4 deposit minerals arena id.
pub const MAPGEN_RF_DEPOSIT_ARENA: &str = "mapgen_deposit_minerals";

/// MapGen PR4 suppression/disruption arena id (Movement-Front source later; no field in PR4).
pub const MAPGEN_RF_SUPPRESSION_ARENA: &str = "mapgen_suppression";

/// Property namespace for MapGen PR4 RF surfaces.
pub const MAPGEN_RF_PROPERTY_NAMESPACE: &str = "mapgen";

/// Default suppression arena participant cap for the tiny pentad slice.
pub const MAPGEN_RF_DEFAULT_SUPPRESSION_MAX_PARTICIPANTS: u32 = 8;

/// Default deposit arena participant cap.
pub const MAPGEN_RF_DEFAULT_DEPOSIT_MAX_PARTICIPANTS: u32 = 4;

/// Default arena coupling fanout cap.
pub const MAPGEN_RF_DEFAULT_MAX_COUPLING_FANOUT: u32 = 4;

/// Default arena orderband depth cap.
pub const MAPGEN_RF_DEFAULT_MAX_ORDERBAND_DEPTH: u32 = 8;

/// Bounded Resource Flow enrollment options for MapGen PR4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenResourceFlowOptions {
    pub suppression_max_participants: u32,
    pub deposit_max_participants: u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    pub capacity_budget: Option<ResourceFlowCapacityBudgetSpec>,
}

impl Default for MapGenResourceFlowOptions {
    fn default() -> Self {
        Self {
            suppression_max_participants: MAPGEN_RF_DEFAULT_SUPPRESSION_MAX_PARTICIPANTS,
            deposit_max_participants: MAPGEN_RF_DEFAULT_DEPOSIT_MAX_PARTICIPANTS,
            max_coupling_fanout: MAPGEN_RF_DEFAULT_MAX_COUPLING_FANOUT,
            max_orderband_depth: MAPGEN_RF_DEFAULT_MAX_ORDERBAND_DEPTH,
            capacity_budget: None,
        }
    }
}

/// Per-arena expansion summary for the tiny MapGen slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenResourceFlowArenaExpansion {
    pub arena_id: String,
    pub participant_count: u32,
    pub max_participants: u32,
    pub coupling_fanout: u32,
    pub max_coupling_fanout: u32,
    pub max_orderband_depth: u32,
    pub source_properties_enrolled: Vec<String>,
    pub rejected_implicit_participants_count: u32,
    pub unsafe_expansion_flags: Vec<String>,
    /// STEAD/Mapping spatial binding (STEAD-CONTRACT-0): whether this arena is spatially bound to gridcell
    /// `Location`s and, if so, the structural grid frame it indexes through.
    pub spatial_binding: SpatialArenaBindingReport,
}

/// MapGen PR4 expansion report for bounded RF enrollment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenResourceFlowExpansionReport {
    pub arenas: Vec<MapGenResourceFlowArenaExpansion>,
}

/// Scenario-container pack plus bounded RF enrollment.
#[derive(Debug, Clone)]
pub struct MapGenResourceFlowEnrollment {
    pub pack: HydratedScenarioPack,
    pub expansion_report: MapGenResourceFlowExpansionReport,
}

/// MapGen PR4 Resource Flow enrollment failure.
#[derive(Debug)]
pub struct MapGenResourceFlowError {
    pub message: String,
}

impl MapGenResourceFlowError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MapGenResourceFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapGen resource flow error: {}", self.message)
    }
}

impl std::error::Error for MapGenResourceFlowError {}

/// How a Resource-Flow / Accumulator arena relates to the STEAD/Mapping spatial substrate
/// (`docs/stead_spatial_contract.md` §5). RF stays generic, **but an arena whose participants are gridcell
/// `Location`s is spatially indexed through STEAD** — it cannot ignore the structural grid. (Owner
/// correction, STEAD-CONTRACT-0: anything touching resource-flow code over Locations must confront this.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpatialBindingMode {
    /// Generic resource flow; participants are not gridcell Locations; no structural grid required.
    #[default]
    SpatiallyNeutral,
    /// Participants **are** gridcell `Location`s; each must have a structural grid placement (never render
    /// metadata), and the arena is indexed through the [`crate::StructuralGridFrame`].
    SpatiallyBoundToGridcellLocations,
}

/// Records an RF arena's relationship to the STEAD structural grid, so every spatially-bound arena
/// confronts STEAD/Mapping in its expansion report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpatialArenaBindingReport {
    pub binding_mode: SpatialBindingMode,
    pub grid_width: Option<u32>,
    pub grid_height: Option<u32>,
    pub occupied_cells: Option<u64>,
}

impl SpatialArenaBindingReport {
    pub fn neutral() -> Self {
        Self {
            binding_mode: SpatialBindingMode::SpatiallyNeutral,
            grid_width: None,
            grid_height: None,
            occupied_cells: None,
        }
    }
    pub fn bound(frame: crate::mapgen_lattice::StructuralGridFrame) -> Self {
        Self {
            binding_mode: SpatialBindingMode::SpatiallyBoundToGridcellLocations,
            grid_width: Some(frame.width),
            grid_height: Some(frame.height),
            occupied_cells: Some(frame.occupied_cells),
        }
    }
}

/// Validate gridcell-`Location` participants bind to STEAD **structural** placements, not render metadata.
/// `SpatiallyNeutral` arenas need no grid (generic RF). `SpatiallyBoundToGridcellLocations` requires every
/// participant node id to have a one-per-cell structural `(row,col)` placement in `grid_metadata`.
pub fn validate_spatial_binding(
    mode: SpatialBindingMode,
    participant_node_ids: &[String],
    grid_metadata: &crate::hydrate_scenario::HydratedScenarioGridMetadata,
) -> Result<(), MapGenResourceFlowError> {
    if mode == SpatialBindingMode::SpatiallyNeutral {
        return Ok(());
    }
    let placements: Vec<StructuralGridPlacement<'_>> = grid_metadata
        .placements
        .iter()
        .map(|placement| StructuralGridPlacement {
            location_id: placement.location_id.as_str(),
            coord: simthing_core::StructuralCoord::new(placement.col, placement.row),
        })
        .collect();
    let participant_refs: Vec<&str> = participant_node_ids.iter().map(String::as_str).collect();
    validate_location_ids_have_structural_placements(&participant_refs, &placements)
        .map_err(map_placed_participant_validation_error)
}

fn map_placed_participant_validation_error(
    err: PlacedParticipantValidationError,
) -> MapGenResourceFlowError {
    MapGenResourceFlowError::new(err.message)
}

fn mint_spatial_arena_participants(
    gridcells: &[GridcellEnrollment],
    hierarchy: &MapGenLatticeHierarchy,
) -> Result<Vec<ExplicitParticipantSpec>, MapGenResourceFlowError> {
    let placements: Vec<StructuralGridPlacement<'_>> = hierarchy
        .pack
        .grid_metadata
        .placements
        .iter()
        .map(|placement| StructuralGridPlacement {
            location_id: placement.location_id.as_str(),
            coord: simthing_core::StructuralCoord::new(placement.col, placement.row),
        })
        .collect();
    let participants: Vec<(SimThingId, &str)> = gridcells
        .iter()
        .map(|cell| (cell.simthing_id, cell.node_id.as_str()))
        .collect();
    let proofs = validate_and_mint_placed_participants_by_location_id(&participants, &placements)
        .map_err(map_placed_participant_validation_error)?;
    let slot_and_placed: Vec<(u32, _)> = gridcells
        .iter()
        .zip(proofs.iter())
        .map(|(gridcell, proof)| {
            let slot = install_slot_for_simthing(&hierarchy.pack.root, gridcell.simthing_id)
                .ok_or_else(|| {
                    MapGenResourceFlowError::new(format!(
                        "gridcell node `{}` missing from install slot map",
                        gridcell.node_id
                    ))
                })?;
            Ok((slot, *proof))
        })
        .collect::<Result<Vec<_>, MapGenResourceFlowError>>()?;
    Ok(spatial_arena_explicit_participants(&slot_and_placed))
}

/// Generate bounded Resource Flow enrollment from a PR3 lattice hierarchy.
pub fn generate_mapgen_resource_flow_enrollment(
    hierarchy: &MapGenLatticeHierarchy,
    options: MapGenResourceFlowOptions,
) -> Result<MapGenResourceFlowEnrollment, MapGenResourceFlowError> {
    validate_options(&options)?;
    let deposits = collect_deposit_feedstock(&hierarchy.pack)?;
    let gridcells = collect_gridcell_enrollment(&hierarchy.pack)?;
    if deposits.is_empty() {
        return Err(MapGenResourceFlowError::new(
            "PR4 requires at least one deposit feedstock node in the PR3 hierarchy",
        ));
    }
    if gridcells.is_empty() {
        return Err(MapGenResourceFlowError::new(
            "PR4 requires at least one gridcell participant in the PR3 hierarchy",
        ));
    }
    let effective_suppression_max_participants = options
        .capacity_budget
        .as_ref()
        .map(|budget| budget.participants_per_arena)
        .unwrap_or(0)
        .max(options.suppression_max_participants);
    let effective_deposit_max_participants = options
        .capacity_budget
        .as_ref()
        .map(|budget| budget.participants_per_arena)
        .unwrap_or(0)
        .max(options.deposit_max_participants);
    if gridcells.len() as u32 > effective_suppression_max_participants {
        return Err(MapGenResourceFlowError::new(format!(
            "gridcell participant count {} exceeds suppression max_participants {}",
            gridcells.len(),
            effective_suppression_max_participants
        )));
    }
    if deposits.len() as u32 > effective_deposit_max_participants {
        return Err(MapGenResourceFlowError::new(format!(
            "deposit participant count {} exceeds deposit max_participants {}",
            deposits.len(),
            effective_deposit_max_participants
        )));
    }

    // STEAD-CONTRACT-0 owner correction: this MapGen RF substrate is SPATIALLY BOUND to gridcell
    // `Location`s. Validate every Location participant has a STRUCTURAL grid placement (`grid_metadata`,
    // not render metadata) and capture the structural frame for the expansion report.
    let grid_frame = crate::mapgen_lattice::StructuralGridFrame::from_grid_metadata(
        &hierarchy.pack.grid_metadata,
    );
    let gridcell_participant_ids: Vec<String> =
        gridcells.iter().map(|cell| cell.node_id.clone()).collect();
    validate_spatial_binding(
        SpatialBindingMode::SpatiallyBoundToGridcellLocations,
        &gridcell_participant_ids,
        &hierarchy.pack.grid_metadata,
    )?;

    let deposit_flow_key = PropertyKey::new(MAPGEN_RF_PROPERTY_NAMESPACE, "deposit_minerals_flow");
    let suppression_flow_key = PropertyKey::new(MAPGEN_RF_PROPERTY_NAMESPACE, "suppression_flow");
    let deposit_property = build_flow_property_spec(
        "mapgen_deposit_minerals_flow",
        MAPGEN_RF_PROPERTY_NAMESPACE,
        "deposit_minerals_flow",
        MAPGEN_RF_DEPOSIT_ARENA,
    );
    let suppression_property = build_flow_property_spec(
        "mapgen_suppression_flow",
        MAPGEN_RF_PROPERTY_NAMESPACE,
        "suppression_flow",
        MAPGEN_RF_SUPPRESSION_ARENA,
    );

    let deposit_participants: Vec<ExplicitParticipantSpec> = deposits
        .iter()
        .map(|deposit| {
            let slot = install_slot_for_simthing(&hierarchy.pack.root, deposit.simthing_id)
                .ok_or_else(|| {
                    MapGenResourceFlowError::new(format!(
                        "deposit node `{}` missing from install slot map",
                        deposit.node_id
                    ))
                })?;
            Ok(ExplicitParticipantSpec::flat(
                slot,
                deposit.simthing_id.raw(),
            ))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let suppression_participants: Vec<ExplicitParticipantSpec> =
        mint_spatial_arena_participants(&gridcells, hierarchy)?;

    let deposit_arena = ArenaSpec {
        name: MAPGEN_RF_DEPOSIT_ARENA.into(),
        flow_property: deposit_flow_key.clone(),
        balance_property: None,
        max_participants: options.deposit_max_participants,
        max_coupling_fanout: options.max_coupling_fanout,
        max_orderband_depth: options.max_orderband_depth,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: deposit_participants.clone(),
        // DA repair (PR4): `explicit_participants` authoritatively lists EVERY deposit, so the deposit
        // arena enrolls via `ExplicitOnly` (matching the suppression arena). The earlier
        // `InstallTarget(ScenarioListed { deposits[0] })` named only the first deposit — harmless for
        // the single-deposit pentad fixture but a latent multi-deposit generalization bug, since a
        // later slice with N deposits would have a selector implying only deposit 0 is the admission
        // source. `ExplicitOnly` is multi-deposit-safe: admission is the full participant list.
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    };
    let suppression_arena = ArenaSpec {
        name: MAPGEN_RF_SUPPRESSION_ARENA.into(),
        flow_property: suppression_flow_key.clone(),
        balance_property: None,
        max_participants: options.suppression_max_participants,
        max_coupling_fanout: options.max_coupling_fanout,
        max_orderband_depth: options.max_orderband_depth,
        fission_policy: FissionPolicySpec::Reject,
        reserved_orderband_depth: 0,
        reserved_gap_per_intermediate: 0,
        expected_max_children_per_intermediate: 0,
        explicit_participants: suppression_participants.clone(),
        enrollment: Some(EnrollmentSelectorSpec::ExplicitOnly),
        wildcard_admission: None,
    };

    validate_arena_caps(&deposit_arena)?;
    validate_arena_caps(&suppression_arena)?;
    validate_explicit_enrollment_with_max(&deposit_arena, effective_deposit_max_participants)?;
    validate_explicit_enrollment_with_max(
        &suppression_arena,
        effective_suppression_max_participants,
    )?;

    let couplings = vec![CouplingSpec {
        from_arena: MAPGEN_RF_DEPOSIT_ARENA.into(),
        to_arena: MAPGEN_RF_SUPPRESSION_ARENA.into(),
        delay: CouplingDelaySpec::OneTickDelay,
    }];
    let resolved_capacity_budget =
        resolve_resource_flow_capacity_budget(options.capacity_budget.as_ref()).map_err(|err| {
            MapGenResourceFlowError::new(format!("RF capacity budget rejected: {err}"))
        })?;
    validate_coupling_fanout_with_budget(
        &[deposit_arena.clone(), suppression_arena.clone()],
        &couplings,
        resolved_capacity_budget.as_ref(),
    )?;

    let base_obligations: Vec<BaseFlowObligationSpec> = deposits
        .iter()
        .enumerate()
        .map(|(index, deposit)| BaseFlowObligationSpec {
            id: format!("mapgen_deposit_minerals_produce_{index}"),
            arena: MAPGEN_RF_DEPOSIT_ARENA.into(),
            install: InstallTargetSpec::ScenarioListed {
                target_id: deposit.node_id.clone(),
            },
            direction: BaseFlowDirectionSpec::Produce,
            rate: deposit.minerals_rate,
        })
        .collect();

    let resource_flow = ResourceFlowSpec {
        opt_in_mode: ResourceFlowOptInMode::Disabled,
        arenas: vec![deposit_arena.clone(), suppression_arena.clone()],
        couplings,
        base_obligations,
        capacity_budget: options.capacity_budget.clone(),
        gated_rates: vec![],
    };
    validate_resource_flow_enrollment(&resource_flow)?;

    let mut pack = hierarchy.pack.clone();
    pack.game_mode.properties.push(deposit_property);
    pack.game_mode.properties.push(suppression_property);
    pack.game_mode.resource_flow = Some(resource_flow);

    assert_no_deferred_pr4_surfaces(&pack)?;
    assert_no_forbidden_generated_properties(&pack)?;
    assert_allowed_simthing_kinds(&pack.root_node)
        .map_err(|err| MapGenResourceFlowError::new(err.message))?;

    let expansion_report = build_expansion_report(
        &[deposit_arena, suppression_arena],
        &deposits,
        &gridcells,
        options,
        grid_frame,
    );

    Ok(MapGenResourceFlowEnrollment {
        pack,
        expansion_report,
    })
}

/// Convenience: parse raw fixture → PR3 hierarchy → PR4 RF enrollment with defaults.
pub fn generate_default_mapgen_resource_flow_enrollment(
    document: &MapGenNeutralDocument,
) -> Result<MapGenResourceFlowEnrollment, MapGenResourceFlowError> {
    use crate::mapgen_lattice::{generate_mapgen_lattice_hierarchy, MapGenLatticeOptions};
    let hierarchy = generate_mapgen_lattice_hierarchy(document, MapGenLatticeOptions::default())
        .map_err(|err| MapGenResourceFlowError::new(err.message))?;
    generate_mapgen_resource_flow_enrollment(&hierarchy, MapGenResourceFlowOptions::default())
}

#[derive(Debug, Clone)]
struct DepositFeedstock {
    node_id: String,
    simthing_id: simthing_core::SimThingId,
    minerals_rate: f32,
}

#[derive(Debug, Clone)]
struct GridcellEnrollment {
    node_id: String,
    simthing_id: simthing_core::SimThingId,
}

fn validate_options(options: &MapGenResourceFlowOptions) -> Result<(), MapGenResourceFlowError> {
    if options.suppression_max_participants == 0
        || options.deposit_max_participants == 0
        || options.max_coupling_fanout == 0
        || options.max_orderband_depth == 0
    {
        return Err(MapGenResourceFlowError::new("PR4 RF caps must be positive"));
    }
    Ok(())
}

/// Validate arena caps are present and positive.
pub fn validate_arena_caps(arena: &ArenaSpec) -> Result<(), MapGenResourceFlowError> {
    if arena.max_participants == 0 {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing max_participants",
            arena.name
        )));
    }
    if arena.max_coupling_fanout == 0 {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing max_coupling_fanout",
            arena.name
        )));
    }
    if arena.max_orderband_depth == 0 {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing max_orderband_depth",
            arena.name
        )));
    }
    Ok(())
}

/// Validate explicit selector/enrollment is present and participants are declared.
pub fn validate_explicit_enrollment(arena: &ArenaSpec) -> Result<(), MapGenResourceFlowError> {
    if arena.enrollment.is_none() {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing explicit enrollment selector",
            arena.name
        )));
    }
    if arena.explicit_participants.is_empty() && arena.wildcard_admission.is_none() {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing explicit participants",
            arena.name
        )));
    }
    if arena.explicit_participants.len() as u32 > arena.max_participants {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` participant count {} exceeds max_participants {}",
            arena.name,
            arena.explicit_participants.len(),
            arena.max_participants
        )));
    }
    Ok(())
}

/// Validate every arena in a ResourceFlowSpec declares caps and explicit enrollment.
pub fn validate_resource_flow_enrollment(
    spec: &ResourceFlowSpec,
) -> Result<(), MapGenResourceFlowError> {
    if spec.arenas.is_empty() {
        return Err(MapGenResourceFlowError::new(
            "PR4 requires at least one RF arena",
        ));
    }
    let capacity_budget = resolve_resource_flow_capacity_budget(spec.capacity_budget.as_ref())
        .map_err(|err| {
            MapGenResourceFlowError::new(format!("RF capacity budget rejected: {err}"))
        })?;
    for arena in &spec.arenas {
        validate_arena_caps(arena)?;
        let (max_participants, _, _) =
            effective_resource_flow_arena_caps(arena, capacity_budget.as_ref());
        validate_explicit_enrollment_with_max(arena, max_participants)?;
    }
    validate_coupling_fanout_with_budget(&spec.arenas, &spec.couplings, capacity_budget.as_ref())?;
    Ok(())
}

fn validate_explicit_enrollment_with_max(
    arena: &ArenaSpec,
    max_participants: u32,
) -> Result<(), MapGenResourceFlowError> {
    if arena.enrollment.is_none() {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing explicit enrollment selector",
            arena.name
        )));
    }
    if arena.explicit_participants.is_empty() && arena.wildcard_admission.is_none() {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` missing explicit participants",
            arena.name
        )));
    }
    if arena.explicit_participants.len() as u32 > max_participants {
        return Err(MapGenResourceFlowError::new(format!(
            "arena `{}` participant count {} exceeds max_participants {}",
            arena.name,
            arena.explicit_participants.len(),
            max_participants
        )));
    }
    Ok(())
}

fn validate_coupling_fanout_with_budget(
    arenas: &[ArenaSpec],
    couplings: &[CouplingSpec],
    capacity_budget: Option<&simthing_spec::ResolvedResourceFlowCapacityBudget>,
) -> Result<(), MapGenResourceFlowError> {
    let mut out_fanout = std::collections::BTreeMap::<&str, u32>::new();
    let mut in_fanout = std::collections::BTreeMap::<&str, u32>::new();
    for coupling in couplings {
        *out_fanout.entry(coupling.from_arena.as_str()).or_insert(0) += 1;
        *in_fanout.entry(coupling.to_arena.as_str()).or_insert(0) += 1;
    }
    for arena in arenas {
        let fanout = out_fanout
            .get(arena.name.as_str())
            .copied()
            .unwrap_or(0)
            .max(in_fanout.get(arena.name.as_str()).copied().unwrap_or(0));
        let (_, max_coupling_fanout, _) =
            effective_resource_flow_arena_caps(arena, capacity_budget);
        if fanout > max_coupling_fanout {
            return Err(MapGenResourceFlowError::new(format!(
                "arena `{}` coupling fanout {fanout} exceeds max_coupling_fanout {}",
                arena.name, max_coupling_fanout
            )));
        }
    }
    Ok(())
}

fn install_slot_for_simthing(root: &SimThing, target: SimThingId) -> Option<u32> {
    let mut next_slot = 0u32;
    install_slot_walk(root, target, &mut next_slot)
}

fn install_slot_walk(node: &SimThing, target: SimThingId, next_slot: &mut u32) -> Option<u32> {
    let slot = *next_slot;
    if node.id == target {
        return Some(slot);
    }
    *next_slot += 1;
    for child in &node.children {
        if let Some(found) = install_slot_walk(child, target, next_slot) {
            return Some(found);
        }
    }
    None
}

fn collect_deposit_feedstock(
    pack: &HydratedScenarioPack,
) -> Result<Vec<DepositFeedstock>, MapGenResourceFlowError> {
    let mut deposits = Vec::new();
    collect_deposit_feedstock_inner(&pack.root_node, &mut deposits)?;
    Ok(deposits)
}

fn collect_deposit_feedstock_inner(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
    deposits: &mut Vec<DepositFeedstock>,
) -> Result<(), MapGenResourceFlowError> {
    if let Some(rate) = parse_inert_rate(node, "deposit_minerals_authored")? {
        deposits.push(DepositFeedstock {
            node_id: node.id.clone(),
            simthing_id: node.simthing_id,
            minerals_rate: rate,
        });
    }
    for child in &node.children {
        collect_deposit_feedstock_inner(child, deposits)?;
    }
    Ok(())
}

fn collect_gridcell_enrollment(
    pack: &HydratedScenarioPack,
) -> Result<Vec<GridcellEnrollment>, MapGenResourceFlowError> {
    let ids = collect_gridcell_location_ids(&pack.root_node);
    let mut gridcells = Vec::with_capacity(ids.len());
    for id in ids {
        let node = find_node(&pack.root_node, &id).ok_or_else(|| {
            MapGenResourceFlowError::new(format!("gridcell node `{id}` missing from hierarchy"))
        })?;
        gridcells.push(GridcellEnrollment {
            node_id: node.id.clone(),
            simthing_id: node.simthing_id,
        });
    }
    Ok(gridcells)
}

fn parse_inert_rate(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
    property_name: &str,
) -> Result<Option<f32>, MapGenResourceFlowError> {
    let Some(property) = node.properties.iter().find(|property| {
        property.namespace == "mapgen"
            && (property.name == property_name
                || property.name.starts_with(&format!("{property_name}_")))
    }) else {
        return Ok(None);
    };
    let Some(value) = property.description.strip_prefix("inert=") else {
        return Err(MapGenResourceFlowError::new(format!(
            "node `{}` property `{property_name}` must use inert= metadata",
            node.id
        )));
    };
    value.parse::<f32>().map(Some).map_err(|_| {
        MapGenResourceFlowError::new(format!(
            "node `{}` property `{property_name}` has non-numeric inert value `{value}`",
            node.id
        ))
    })
}

fn build_flow_property_spec(
    id: &str,
    namespace: &str,
    name: &str,
    arena_name: &str,
) -> PropertySpec {
    PropertySpec {
        id: id.into(),
        namespace: namespace.into(),
        name: name.into(),
        display_name: name.into(),
        description: format!("MapGen PR4 RF flow property for arena `{arena_name}`"),
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

fn build_expansion_report(
    arenas: &[ArenaSpec],
    deposits: &[DepositFeedstock],
    gridcells: &[GridcellEnrollment],
    options: MapGenResourceFlowOptions,
    grid_frame: crate::mapgen_lattice::StructuralGridFrame,
) -> MapGenResourceFlowExpansionReport {
    let mut out_fanout = std::collections::BTreeMap::<&str, u32>::new();
    let mut in_fanout = std::collections::BTreeMap::<&str, u32>::new();
    out_fanout.insert(MAPGEN_RF_DEPOSIT_ARENA, 1);
    in_fanout.insert(MAPGEN_RF_SUPPRESSION_ARENA, 1);

    let arena_reports = arenas
        .iter()
        .map(|arena| {
            let coupling_fanout = out_fanout
                .get(arena.name.as_str())
                .copied()
                .unwrap_or(0)
                .max(in_fanout.get(arena.name.as_str()).copied().unwrap_or(0));
            let source_properties_enrolled = vec![format!(
                "{}::{}",
                arena.flow_property.namespace, arena.flow_property.name
            )];
            let rejected_implicit_participants_count = match arena.name.as_str() {
                MAPGEN_RF_DEPOSIT_ARENA => deposits
                    .len()
                    .saturating_sub(arena.explicit_participants.len()),
                MAPGEN_RF_SUPPRESSION_ARENA => gridcells
                    .len()
                    .saturating_sub(arena.explicit_participants.len()),
                _ => 0,
            } as u32;
            let mut unsafe_expansion_flags = Vec::new();
            if arena.explicit_participants.len() as u32 >= arena.max_participants {
                unsafe_expansion_flags.push("participant_at_cap".into());
            }
            if coupling_fanout >= arena.max_coupling_fanout {
                unsafe_expansion_flags.push("coupling_fanout_at_cap".into());
            }
            if arena.max_orderband_depth > options.max_orderband_depth {
                unsafe_expansion_flags.push("orderband_depth_exceeds_fixture_default".into());
            }
            MapGenResourceFlowArenaExpansion {
                arena_id: arena.name.clone(),
                participant_count: arena.explicit_participants.len() as u32,
                max_participants: arena.max_participants,
                coupling_fanout,
                max_coupling_fanout: arena.max_coupling_fanout,
                max_orderband_depth: arena.max_orderband_depth,
                source_properties_enrolled,
                rejected_implicit_participants_count,
                unsafe_expansion_flags,
                // The MapGen RF substrate (deposit feedstock + suppression front) is spatially anchored to
                // the gridcell `Location` lattice; both arenas record the structural frame they index through.
                spatial_binding: SpatialArenaBindingReport::bound(grid_frame),
            }
        })
        .collect();

    MapGenResourceFlowExpansionReport {
        arenas: arena_reports,
    }
}

fn assert_no_deferred_pr4_surfaces(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenResourceFlowError> {
    if pack.w_impedance_compose.is_some() || pack.stress_compose.is_some() {
        return Err(MapGenResourceFlowError::new(
            "PR4 generator must not emit field_operator surfaces",
        ));
    }
    if pack.palma_feedstock.is_some() {
        return Err(MapGenResourceFlowError::new(
            "PR4 generator must not emit PALMA feedstock",
        ));
    }
    if pack.commitment.is_some() {
        return Err(MapGenResourceFlowError::new(
            "PR4 generator must not emit FIELD_POLICY commitment",
        ));
    }
    if !pack.grid_metadata.links.is_empty() {
        return Err(MapGenResourceFlowError::new(
            "PR4 generator must not emit hyperlane/link topology",
        ));
    }
    Ok(())
}

fn assert_no_forbidden_generated_properties(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenResourceFlowError> {
    for property in &pack.game_mode.properties {
        reject_forbidden_property_name(property)?;
    }
    walk_forbidden_properties(&pack.root_node)?;
    Ok(())
}

fn walk_forbidden_properties(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
) -> Result<(), MapGenResourceFlowError> {
    for property in &node.properties {
        reject_forbidden_property_name(property)?;
    }
    for child in &node.children {
        walk_forbidden_properties(child)?;
    }
    Ok(())
}

fn reject_forbidden_property_name(property: &PropertySpec) -> Result<(), MapGenResourceFlowError> {
    let haystack = format!(
        "{} {} {} {}",
        property.id, property.namespace, property.name, property.description
    );
    for forbidden in FORBIDDEN_GENERATED_PROPERTY_NAMES {
        if haystack.contains(forbidden) {
            return Err(MapGenResourceFlowError::new(format!(
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            )));
        }
    }
    Ok(())
}

fn find_node<'a>(
    node: &'a crate::hydrate_scenario::HydratedScenarioNode,
    id: &str,
) -> Option<&'a crate::hydrate_scenario::HydratedScenarioNode> {
    if node.id == id {
        return Some(node);
    }
    for child in &node.children {
        if let Some(found) = find_node(child, id) {
            return Some(found);
        }
    }
    None
}
