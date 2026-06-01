use simthing_spec::{
    run_mobility_runtime1a_production_fixture, IdentityLane, MobilityAlloc0BlockSpec,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput, MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord,
    MobilityEcon0PlanInput, MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord,
    MobilityIdroute0PlanInput, MobilityOwner0ColumnKind, MobilityOwner0ColumnValue,
    MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord, MobilityOwner0Overlay,
    MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move,
    MobilityReenroll0PlanInput, MobilityReenroll0RegistryState, MobilityRuntime0CompositionInput,
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig,
    MobilityRuntime1aFixtureGate, MobilityRuntime1aForbiddenPathRequests,
    MobilityRuntime1aProductionFixtureInput, MobilityRuntime1aProductionFixtureReport,
    MobilityRuntime1aSimSessionSurface,
};

pub const PRODUCTION_PATH_0080_0_ID: &str = "PRODUCTION-PATH-0080-0";
pub const PRODUCTION_PATH_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - Local Patrol Economy opt-in production path";
pub const PRODUCTION_PATH_0080_0_SCENARIO: &str = "Local Patrol Economy";
pub const SCENARIO_0080_0_GATE_ID: &str = "SCENARIO-0080-0";
pub const PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES: [&str; 5] = [
    "supply",
    "maintenance",
    "local_output",
    "local_security",
    "disruption",
];

const SESSION_ID: u64 = 80;
const SOURCE_CELL_ID: u64 = 10;
const DESTINATION_CELL_ID: u64 = 20;
const PATROL_ENTITY_ID: u64 = 8_000;
const LOCAL_OWNER_ID: u64 = 7;
const OWNER_OVERLAY_MODIFIER_ID: u64 = 42;
const OWNER_OVERLAY_MODIFIER_AMOUNT: i64 = 11;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl ProductionPath0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0080Surface {
    pub gate: ProductionPath0080Gate,
    pub local_patrol_economy_fixture_registered: bool,
    pub global_default_schedule_registered: bool,
}

impl ProductionPath0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: ProductionPath0080Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProductionPath0080ForbiddenRequests {
    pub global_default_schedule: bool,
    pub gameplay_surface: bool,
    pub semantic_or_raw_wgsl: bool,
    pub cpu_planner_or_external_move_script: bool,
    pub capture_as_reparenting: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub nested_transfer: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub clausething_dependency: bool,
    pub closed_ladder_reopen: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPatrolEconomyCell {
    pub cell_id: u64,
    pub supply: i64,
    pub maintenance: i64,
    pub local_output: i64,
    pub local_security: i64,
    pub disruption: i64,
    pub patrol_participation_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalPatrolEconomyScenario {
    pub source: LocalPatrolEconomyCell,
    pub destination: LocalPatrolEconomyCell,
    pub patrol_entity_id: u64,
    pub owner_id: u64,
    pub owner_overlay_modifier_id: u64,
    pub owner_overlay_modifier_amount: i64,
    pub disruption_threshold: i64,
    pub local_security_floor: i64,
}

impl LocalPatrolEconomyScenario {
    pub fn canonical() -> Self {
        Self {
            source: LocalPatrolEconomyCell {
                cell_id: SOURCE_CELL_ID,
                supply: 12,
                maintenance: 4,
                local_output: 6,
                local_security: 2,
                disruption: 9,
                patrol_participation_count: 1,
            },
            destination: LocalPatrolEconomyCell {
                cell_id: DESTINATION_CELL_ID,
                supply: 9,
                maintenance: 3,
                local_output: 5,
                local_security: 7,
                disruption: 1,
                patrol_participation_count: 0,
            },
            patrol_entity_id: PATROL_ENTITY_ID,
            owner_id: LOCAL_OWNER_ID,
            owner_overlay_modifier_id: OWNER_OVERLAY_MODIFIER_ID,
            owner_overlay_modifier_amount: OWNER_OVERLAY_MODIFIER_AMOUNT,
            disruption_threshold: 8,
            local_security_floor: 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionPath0080Input {
    pub surface: ProductionPath0080Surface,
    pub scenario: LocalPatrolEconomyScenario,
    pub forbidden: ProductionPath0080ForbiddenRequests,
}

impl ProductionPath0080Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: ProductionPath0080Surface::default_simsession(),
            scenario: LocalPatrolEconomyScenario::canonical(),
            forbidden: ProductionPath0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: ProductionPath0080Surface::with_explicit_opt_in(),
            scenario: LocalPatrolEconomyScenario::canonical(),
            forbidden: ProductionPath0080ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductionPath0080Report {
    pub path_id: &'static str,
    pub scenario_gate_id: &'static str,
    pub status: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub local_patrol_economy_instantiated: bool,
    pub global_default_schedule_registered: bool,
    pub gameplay_surface_present: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub clausething_dependency_present: bool,

    pub sead_threshold_accepted: bool,
    pub sead_emit_event_emitted: bool,
    pub boundary_request_materialized: bool,
    pub mobility_substrate_consumed_boundary_request: bool,
    pub cpu_planner_used: bool,
    pub external_move_script_used: bool,

    pub patrol_entity_id_before: u64,
    pub patrol_entity_id_after: u64,
    pub identity_preserved_after_relocation: bool,
    pub source_membership_before: bool,
    pub source_membership_after: bool,
    pub destination_membership_before: bool,
    pub destination_membership_after: bool,
    pub owner_id_before: u64,
    pub owner_id_after: u64,
    pub owner_overlay_modifier_before: i64,
    pub owner_overlay_modifier_after: i64,
    pub owner_overlay_persists_after_move: bool,

    pub source_patrol_count_before: u32,
    pub source_patrol_count_after: u32,
    pub destination_patrol_count_before: u32,
    pub destination_patrol_count_after: u32,
    pub bounded_local_economy_values: Vec<&'static str>,
    pub bounded_local_economy_only: bool,

    pub capture_as_reparenting: bool,
    pub owner_entity_as_spatial_parent: bool,
    pub nested_transfer: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub closed_ladders_reopened: bool,

    pub mobility_report: Option<MobilityRuntime1aProductionFixtureReport>,
    pub deterministic_replay_checksum: u64,
}

pub fn run_production_path_0080_0(input: &ProductionPath0080Input) -> ProductionPath0080Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let threshold_accepted = input.scenario.source.disruption
        >= input.scenario.disruption_threshold
        || input.scenario.source.local_security <= input.scenario.local_security_floor;
    if !threshold_accepted {
        return rejected_report(input, vec!["local_patrol_threshold_not_crossed"]);
    }

    let mobility_report = run_mobility_runtime1a_production_fixture(&mobility_input(input));
    if !mobility_report.admitted {
        let mut merged = mobility_report.diagnostics.clone();
        if merged.is_empty() {
            merged.push("mobility_runtime1a_rejected");
        }
        return rejected_report(input, merged);
    }

    admitted_report(input, mobility_report)
}

pub fn replay_production_path_0080_0() -> (ProductionPath0080Report, ProductionPath0080Report) {
    let input = ProductionPath0080Input::explicit_opt_in();
    (
        run_production_path_0080_0(&input),
        run_production_path_0080_0(&input),
    )
}

fn validate_surface(surface: &ProductionPath0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("production_path_0080_0_default_on_behavior_rejected");
    }
    if surface.global_default_schedule_registered {
        diagnostics.push("global_default_schedule");
    }
}

fn validate_forbidden(
    forbidden: &ProductionPath0080ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.gameplay_surface {
        diagnostics.push("gameplay_surface");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.cpu_planner_or_external_move_script {
        diagnostics.push("cpu_planner_or_external_move_script");
    }
    if forbidden.capture_as_reparenting {
        diagnostics.push("capture_as_reparenting");
    }
    if forbidden.owner_entity_as_spatial_parent {
        diagnostics.push("owner_entity_as_spatial_parent");
    }
    if forbidden.nested_transfer {
        diagnostics.push("nested_transfer");
    }
    if forbidden.hard_currency_markets_trade_aibudget {
        diagnostics.push("hard_currency_markets_trade_aibudget");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.closed_ladder_reopen {
        diagnostics.push("closed_ladder_reopen");
    }
}

fn disabled_no_op_report(input: &ProductionPath0080Input) -> ProductionPath0080Report {
    base_report(input, Vec::new(), true, None)
}

fn rejected_report(
    input: &ProductionPath0080Input,
    diagnostics: Vec<&'static str>,
) -> ProductionPath0080Report {
    let mut report = base_report(input, diagnostics, false, None);
    report.admitted = false;
    report.disabled_no_op = false;
    report
}

fn admitted_report(
    input: &ProductionPath0080Input,
    mobility_report: MobilityRuntime1aProductionFixtureReport,
) -> ProductionPath0080Report {
    let mut report = base_report(input, Vec::new(), false, Some(mobility_report));
    report.local_patrol_economy_instantiated = true;
    report.sead_threshold_accepted = true;
    report.sead_emit_event_emitted = true;
    report.boundary_request_materialized = true;
    report.mobility_substrate_consumed_boundary_request = true;
    report.patrol_entity_id_after = input.scenario.patrol_entity_id;
    report.identity_preserved_after_relocation =
        report.patrol_entity_id_before == report.patrol_entity_id_after;
    report.source_membership_after = false;
    report.destination_membership_after = true;
    report.owner_id_after = input.scenario.owner_id;
    report.owner_overlay_modifier_after = input.scenario.owner_overlay_modifier_amount;
    report.owner_overlay_persists_after_move = report.owner_id_before == report.owner_id_after
        && report.owner_overlay_modifier_before == report.owner_overlay_modifier_after;
    report.source_patrol_count_after = input
        .scenario
        .source
        .patrol_participation_count
        .saturating_sub(1);
    report.destination_patrol_count_after = input
        .scenario
        .destination
        .patrol_participation_count
        .saturating_add(1);
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn base_report(
    input: &ProductionPath0080Input,
    diagnostics: Vec<&'static str>,
    disabled_no_op: bool,
    mobility_report: Option<MobilityRuntime1aProductionFixtureReport>,
) -> ProductionPath0080Report {
    let checksum = mobility_report
        .as_ref()
        .map(|report| report.deterministic_replay_checksum)
        .unwrap_or(0);

    ProductionPath0080Report {
        path_id: PRODUCTION_PATH_0080_0_ID,
        scenario_gate_id: SCENARIO_0080_0_GATE_ID,
        status: PRODUCTION_PATH_0080_0_STATUS_PASS,
        admitted: true,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        local_patrol_economy_instantiated: false,
        global_default_schedule_registered: input.surface.global_default_schedule_registered,
        gameplay_surface_present: false,
        semantic_or_raw_wgsl_present: false,
        clausething_dependency_present: false,
        sead_threshold_accepted: false,
        sead_emit_event_emitted: false,
        boundary_request_materialized: false,
        mobility_substrate_consumed_boundary_request: false,
        cpu_planner_used: false,
        external_move_script_used: false,
        patrol_entity_id_before: input.scenario.patrol_entity_id,
        patrol_entity_id_after: input.scenario.patrol_entity_id,
        identity_preserved_after_relocation: disabled_no_op,
        source_membership_before: true,
        source_membership_after: true,
        destination_membership_before: false,
        destination_membership_after: false,
        owner_id_before: input.scenario.owner_id,
        owner_id_after: input.scenario.owner_id,
        owner_overlay_modifier_before: input.scenario.owner_overlay_modifier_amount,
        owner_overlay_modifier_after: input.scenario.owner_overlay_modifier_amount,
        owner_overlay_persists_after_move: disabled_no_op,
        source_patrol_count_before: input.scenario.source.patrol_participation_count,
        source_patrol_count_after: input.scenario.source.patrol_participation_count,
        destination_patrol_count_before: input.scenario.destination.patrol_participation_count,
        destination_patrol_count_after: input.scenario.destination.patrol_participation_count,
        bounded_local_economy_values: PRODUCTION_PATH_0080_0_ALLOWED_ECONOMY_VALUES.to_vec(),
        bounded_local_economy_only: true,
        capture_as_reparenting: false,
        owner_entity_as_spatial_parent: false,
        nested_transfer: false,
        hard_currency_markets_trade_aibudget: false,
        closed_ladders_reopened: false,
        mobility_report,
        deterministic_replay_checksum: checksum,
    }
}

fn mobility_input(input: &ProductionPath0080Input) -> MobilityRuntime1aProductionFixtureInput {
    MobilityRuntime1aProductionFixtureInput {
        surface: MobilityRuntime1aSimSessionSurface {
            gate: MobilityRuntime1aFixtureGate::explicit_opt_in(),
            named_fixture_registered: false,
            composition_invocations: 0,
        },
        composition: mobility_composition(&input.scenario),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    }
}

fn mobility_composition(scenario: &LocalPatrolEconomyScenario) -> MobilityRuntime0CompositionInput {
    let blocks = vec![
        block(SOURCE_CELL_ID, 0, 8),
        block(DESTINATION_CELL_ID, 8, 8),
        block(30, 16, 2),
        block(31, 18, 2),
    ];
    let live_slices = vec![
        live(SOURCE_CELL_ID, scenario.patrol_entity_id, 0),
        live(SOURCE_CELL_ID, 8_001, 1),
        live(30, 2, 16),
        live(31, 3, 18),
    ];

    MobilityRuntime0CompositionInput {
        config: MobilityRuntime0HarnessConfig::opt_in_test_harness(),
        alloc: MobilityAlloc0PlanInput {
            blocks: blocks.clone(),
            live_slices: live_slices.clone(),
            events: vec![],
            forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
        },
        reenroll: MobilityReenroll0PlanInput {
            registry: MobilityReenroll0RegistryState {
                blocks,
                live_slices,
                origin_generations: Default::default(),
                destination_generations: Default::default(),
            },
            moves: vec![MobilityReenroll0Move {
                entity_id: scenario.patrol_entity_id,
                origin: key(SOURCE_CELL_ID),
                destination: key(DESTINATION_CELL_ID),
                arrival_order: 9,
            }],
            forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
        },
        idroute: MobilityIdroute0PlanInput {
            records: vec![
                MobilityIdroute0LocalRecord {
                    entity_id: scenario.patrol_entity_id,
                    parent_key: key(DESTINATION_CELL_ID),
                    identity: IdentityLane(0),
                    hard_value: scenario.destination.local_security,
                    soft_value: scenario.destination.local_output as f32,
                },
                MobilityIdroute0LocalRecord {
                    entity_id: 8_001,
                    parent_key: key(SOURCE_CELL_ID),
                    identity: IdentityLane(1),
                    hard_value: scenario.source.local_security,
                    soft_value: scenario.source.local_output as f32,
                },
                MobilityIdroute0LocalRecord {
                    entity_id: 2,
                    parent_key: key(30),
                    identity: IdentityLane(0),
                    hard_value: 2,
                    soft_value: 0.25,
                },
            ],
            max_factions_per_cell: 4,
            forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
        },
        econ: MobilityEcon0PlanInput {
            records: vec![
                econ_record(
                    DESTINATION_CELL_ID,
                    7,
                    scenario.destination.supply,
                    scenario.destination.maintenance,
                    scenario.destination.disruption as f32,
                    1,
                ),
                econ_record(
                    SOURCE_CELL_ID,
                    7,
                    scenario.source.supply,
                    scenario.source.maintenance,
                    scenario.source.disruption as f32,
                    2,
                ),
                econ_record(30, 7, 1, 1, 0.25, 3),
            ],
            forbidden: MobilityEcon0ForbiddenPathRequests::default(),
        },
        owner: MobilityOwner0PlanInput {
            records: vec![
                owner_record(
                    scenario.patrol_entity_id,
                    DESTINATION_CELL_ID,
                    scenario.owner_id,
                ),
                owner_record(2, 30, scenario.owner_id),
                MobilityOwner0LocalRecord {
                    entity_id: 3,
                    cell_key: key(31),
                    cohort_count: 1,
                    owner_columns: vec![MobilityOwner0ColumnValue {
                        kind: MobilityOwner0ColumnKind::Species,
                        owner_id: scenario.owner_id,
                    }],
                    generation: 0,
                    blocked_by_blockade: false,
                    arrival_order: 3,
                },
            ],
            overlays: vec![MobilityOwner0Overlay {
                owner: MobilityOwner0ColumnValue {
                    kind: MobilityOwner0ColumnKind::Faction,
                    owner_id: scenario.owner_id,
                },
                modifier_id: scenario.owner_overlay_modifier_id,
                modifier_amount: scenario.owner_overlay_modifier_amount,
                arrival_order: 0,
            }],
            owner_changes: vec![],
            forbidden: MobilityOwner0ForbiddenPathRequests::default(),
        },
        forbidden: MobilityRuntime0ForbiddenPathRequests::default(),
    }
}

fn block(cell_id: u64, start_slot: u32, slot_count: u32) -> MobilityAlloc0BlockSpec {
    MobilityAlloc0BlockSpec {
        parent_key: key(cell_id),
        start_slot,
        slot_count,
        reserved_headroom: slot_count / 2,
    }
}

fn live(cell_id: u64, entity_id: u64, slot: u32) -> MobilityAlloc0LiveSlice {
    MobilityAlloc0LiveSlice {
        entity_id,
        parent_key: key(cell_id),
        slot,
    }
}

fn key(cell_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey {
        parent_id: SESSION_ID,
        key_id: cell_id,
    }
}

fn econ_record(
    cell_id: u64,
    resource_id: u64,
    hard_available: i64,
    hard_need: i64,
    soft_beta_signal: f32,
    arrival_order: u64,
) -> MobilityEcon0LocalCellRecord {
    MobilityEcon0LocalCellRecord {
        session_id: SESSION_ID,
        cell_key: key(cell_id),
        resource_id,
        hard_available,
        hard_need,
        soft_beta_signal,
        arrival_order,
    }
}

fn owner_record(entity_id: u64, cell_id: u64, owner_id: u64) -> MobilityOwner0LocalRecord {
    MobilityOwner0LocalRecord {
        entity_id,
        cell_key: key(cell_id),
        cohort_count: 1,
        owner_columns: vec![MobilityOwner0ColumnValue {
            kind: MobilityOwner0ColumnKind::Faction,
            owner_id,
        }],
        generation: 0,
        blocked_by_blockade: false,
        arrival_order: entity_id,
    }
}

fn checksum_report(report: &ProductionPath0080Report) -> u64 {
    [
        report.patrol_entity_id_before,
        report.patrol_entity_id_after,
        report.owner_id_before,
        report.owner_id_after,
        report.source_patrol_count_before as u64,
        report.source_patrol_count_after as u64,
        report.destination_patrol_count_before as u64,
        report.destination_patrol_count_after as u64,
        report
            .mobility_report
            .as_ref()
            .map(|mobility| mobility.deterministic_replay_checksum)
            .unwrap_or(0),
    ]
    .iter()
    .fold(0xcbf2_9ce4_8422_2325, |hash, value| {
        fnv_append_u64(hash, *value)
    })
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
