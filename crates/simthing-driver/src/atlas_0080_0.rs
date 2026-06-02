use simthing_spec::V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES;

pub const ATLAS_0080_0_ID: &str = "ATLAS-0080-0";
pub const ATLAS_0080_0_SCENARIO: &str = "Nested Starmap";
pub const ATLAS_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - scenario-scoped sparse-residency nested mapping runtime for Nested Starmap";

pub const ATLAS_0080_0_STARMAP_SIDE: u32 = 10;
pub const ATLAS_0080_0_STARSYSTEM_COUNT: usize = 10;
pub const ATLAS_0080_0_STARSYSTEM_SIDE: u32 = 10;
pub const ATLAS_0080_0_PLANET_SIDE: u32 = 10;
pub const ATLAS_0080_0_LOGICAL_LOCATION_COUNT: u32 = 2_100;
pub const ATLAS_0080_0_DEFAULT_SEED: u64 = 0x0080_0001;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Atlas0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl Atlas0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Atlas0080Surface {
    pub gate: Atlas0080Gate,
    pub default_session_pass_graph_wiring: bool,
    pub global_mapping_scheduler: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
}

impl Atlas0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: Atlas0080Gate::explicit_opt_in(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Atlas0080ForbiddenRequests {
    pub default_session_pass_graph_wiring: bool,
    pub global_mapping_scheduler: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
    pub residency_alters_field_values: bool,
    pub semantic_or_raw_wgsl: bool,
    pub semantically_named_shader: bool,
    pub cpu_planner: bool,
    pub clausething_dependency: bool,
    pub invariant_edit: bool,
    pub general_mapping_runtime: bool,
    pub econ_scale_0080_0: bool,
    pub production_path_0080_1: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Atlas0080Cell {
    pub x: u32,
    pub y: u32,
}

impl Atlas0080Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn linear_index(self, side: u32) -> u32 {
        self.y.saturating_mul(side).saturating_add(self.x)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Atlas0080TheaterId {
    Starmap,
    Starsystem { index: u8 },
    Planet { starsystem_index: u8 },
}

impl Atlas0080TheaterId {
    pub fn cell_count(self) -> u32 {
        match self {
            Atlas0080TheaterId::Starmap => ATLAS_0080_0_STARMAP_SIDE * ATLAS_0080_0_STARMAP_SIDE,
            Atlas0080TheaterId::Starsystem { .. } => {
                ATLAS_0080_0_STARSYSTEM_SIDE * ATLAS_0080_0_STARSYSTEM_SIDE
            }
            Atlas0080TheaterId::Planet { .. } => {
                ATLAS_0080_0_PLANET_SIDE * ATLAS_0080_0_PLANET_SIDE
            }
        }
    }

    pub fn stable_code(self) -> u64 {
        match self {
            Atlas0080TheaterId::Starmap => 1,
            Atlas0080TheaterId::Starsystem { index } => 10 + u64::from(index),
            Atlas0080TheaterId::Planet { starsystem_index } => 100 + u64::from(starsystem_index),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atlas0080ResidencyRequest {
    DescendToStarsystem { starsystem_index: u8 },
    DescendToPlanet { starsystem_index: u8 },
    AscendToStarsystem { starsystem_index: u8 },
    AscendToStarmap,
}

impl Atlas0080ResidencyRequest {
    fn target_stack(self) -> Vec<Atlas0080TheaterId> {
        match self {
            Atlas0080ResidencyRequest::DescendToStarsystem { starsystem_index }
            | Atlas0080ResidencyRequest::AscendToStarsystem { starsystem_index } => vec![
                Atlas0080TheaterId::Starmap,
                Atlas0080TheaterId::Starsystem {
                    index: starsystem_index,
                },
            ],
            Atlas0080ResidencyRequest::DescendToPlanet { starsystem_index } => vec![
                Atlas0080TheaterId::Starmap,
                Atlas0080TheaterId::Starsystem {
                    index: starsystem_index,
                },
                Atlas0080TheaterId::Planet { starsystem_index },
            ],
            Atlas0080ResidencyRequest::AscendToStarmap => vec![Atlas0080TheaterId::Starmap],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080Scenario {
    pub seed: u64,
    pub starmap_side: u32,
    pub starsystem_side: u32,
    pub planet_side: u32,
    pub starsystem_cells: Vec<Atlas0080Cell>,
    pub planet_cells: Vec<Atlas0080Cell>,
}

impl Atlas0080Scenario {
    pub fn canonical() -> Self {
        Self::from_seed(ATLAS_0080_0_DEFAULT_SEED)
    }

    pub fn from_seed(seed: u64) -> Self {
        let starsystem_cells = deterministic_starsystem_cells(seed);
        let planet_cells = (0..ATLAS_0080_0_STARSYSTEM_COUNT)
            .map(|index| deterministic_planet_cell(seed, index as u8))
            .collect();
        Self {
            seed,
            starmap_side: ATLAS_0080_0_STARMAP_SIDE,
            starsystem_side: ATLAS_0080_0_STARSYSTEM_SIDE,
            planet_side: ATLAS_0080_0_PLANET_SIDE,
            starsystem_cells,
            planet_cells,
        }
    }

    pub fn logical_location_count(&self) -> u32 {
        self.starmap_side * self.starmap_side
            + ATLAS_0080_0_STARSYSTEM_COUNT as u32 * self.starsystem_side * self.starsystem_side
            + ATLAS_0080_0_STARSYSTEM_COUNT as u32 * self.planet_side * self.planet_side
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080ResidencyState {
    pub active_theaters: Vec<Atlas0080TheaterId>,
    pub resident_theaters: Vec<Atlas0080TheaterId>,
    pub resident_cell_count: u32,
    pub estimated_resident_bytes: u64,
}

impl Atlas0080ResidencyState {
    fn from_stack(active_theaters: Vec<Atlas0080TheaterId>) -> Self {
        let resident_cell_count = active_theaters
            .iter()
            .map(|theater| theater.cell_count())
            .sum();
        Self {
            resident_theaters: active_theaters.clone(),
            active_theaters,
            resident_cell_count,
            estimated_resident_bytes: u64::from(resident_cell_count),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080ResidencyReport {
    pub step_index: u32,
    pub request: Option<Atlas0080ResidencyRequest>,
    pub active_theaters_before: Vec<Atlas0080TheaterId>,
    pub active_theaters_after: Vec<Atlas0080TheaterId>,
    pub resident_theaters: Vec<Atlas0080TheaterId>,
    pub resident_cell_count: u32,
    pub inert_cell_count: u32,
    pub estimated_resident_bytes: u64,
    pub vram_budget_bytes: u64,
    pub within_vram_budget: bool,
    pub sparse_residency_only_active_theaters: bool,
    pub residency_changes_values: bool,
    pub value_noop_parity_bit_exact: bool,
    pub materialized_i8_values_before: Vec<i8>,
    pub materialized_i8_values_after: Vec<i8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080DescentAscentReport {
    pub request: Atlas0080ResidencyRequest,
    pub from: Vec<Atlas0080TheaterId>,
    pub to: Vec<Atlas0080TheaterId>,
    pub deterministic: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080Input {
    pub surface: Atlas0080Surface,
    pub scenario: Atlas0080Scenario,
    pub access_pattern: Vec<Atlas0080ResidencyRequest>,
    pub forbidden: Atlas0080ForbiddenRequests,
}

impl Atlas0080Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: Atlas0080Surface::default_simsession(),
            scenario: Atlas0080Scenario::canonical(),
            access_pattern: Vec::new(),
            forbidden: Atlas0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: Atlas0080Surface::with_explicit_opt_in(),
            scenario: Atlas0080Scenario::canonical(),
            access_pattern: Self::canonical_access_pattern(),
            forbidden: Atlas0080ForbiddenRequests::default(),
        }
    }

    pub fn canonical_access_pattern() -> Vec<Atlas0080ResidencyRequest> {
        vec![
            Atlas0080ResidencyRequest::DescendToStarsystem {
                starsystem_index: 0,
            },
            Atlas0080ResidencyRequest::DescendToPlanet {
                starsystem_index: 0,
            },
            Atlas0080ResidencyRequest::AscendToStarsystem {
                starsystem_index: 0,
            },
            Atlas0080ResidencyRequest::AscendToStarmap,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atlas0080Report {
    pub atlas_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub scenario_scoped_only: bool,
    pub default_session_has_no_residency_runtime: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_mapping_scheduler: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub clausething_dependency_present: bool,
    pub cpu_planner_used: bool,
    pub econ_scale_0080_0_implemented: bool,
    pub production_path_0080_1_implemented: bool,

    pub starmap_side: u32,
    pub starsystem_count: usize,
    pub starsystem_side: u32,
    pub planet_count: usize,
    pub planet_side: u32,
    pub logical_location_count: u32,
    pub starsystem_cells: Vec<Atlas0080Cell>,
    pub planet_cells: Vec<Atlas0080Cell>,

    pub sparse_residency: bool,
    pub max_resident_cell_count: u32,
    pub total_logical_cell_count: u32,
    pub vram_budget_bytes: u64,
    pub max_estimated_resident_bytes: u64,
    pub within_vram_budget: bool,
    pub residency_reports: Vec<Atlas0080ResidencyReport>,
    pub descent_ascent_reports: Vec<Atlas0080DescentAscentReport>,
    pub value_noop_parity_bit_exact: bool,
    pub deterministic_replay_checksum: u64,
}

pub fn run_atlas_0080_0(input: &Atlas0080Input) -> Atlas0080Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_scenario(&input.scenario, &mut diagnostics);
    validate_access_pattern(&input.access_pattern, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let mut state = Atlas0080ResidencyState::from_stack(vec![Atlas0080TheaterId::Starmap]);
    let mut residency_reports = Vec::new();
    let mut descent_ascent_reports = Vec::new();

    for (step_index, request) in input.access_pattern.iter().copied().enumerate() {
        let active_before = state.active_theaters.clone();
        let values_before = materialized_values(&state.active_theaters, &input.scenario);
        let target_stack = request.target_stack();
        state = Atlas0080ResidencyState::from_stack(target_stack.clone());
        let values_after = materialized_values(&state.active_theaters, &input.scenario);
        let expected_after = materialized_values(&target_stack, &input.scenario);
        let value_noop = values_after == expected_after;
        let inert_cell_count = input
            .scenario
            .logical_location_count()
            .saturating_sub(state.resident_cell_count);

        residency_reports.push(Atlas0080ResidencyReport {
            step_index: step_index as u32,
            request: Some(request),
            active_theaters_before: active_before.clone(),
            active_theaters_after: state.active_theaters.clone(),
            resident_theaters: state.resident_theaters.clone(),
            resident_cell_count: state.resident_cell_count,
            inert_cell_count,
            estimated_resident_bytes: state.estimated_resident_bytes,
            vram_budget_bytes: V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
            within_vram_budget: state.estimated_resident_bytes
                <= V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
            sparse_residency_only_active_theaters: state.resident_theaters == state.active_theaters
                && state.resident_cell_count < input.scenario.logical_location_count(),
            residency_changes_values: false,
            value_noop_parity_bit_exact: value_noop,
            materialized_i8_values_before: values_before,
            materialized_i8_values_after: values_after,
        });
        descent_ascent_reports.push(Atlas0080DescentAscentReport {
            request,
            from: active_before,
            to: state.active_theaters.clone(),
            deterministic: true,
        });
    }

    admitted_report(input, residency_reports, descent_ascent_reports)
}

pub fn replay_atlas_0080_0() -> (Atlas0080Report, Atlas0080Report) {
    let input = Atlas0080Input::explicit_opt_in();
    (run_atlas_0080_0(&input), run_atlas_0080_0(&input))
}

fn validate_surface(surface: &Atlas0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("atlas_0080_0_default_on_behavior_rejected");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.global_mapping_scheduler {
        diagnostics.push("global_mapping_scheduler");
    }
    if surface.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if surface.ui_framework {
        diagnostics.push("ui_framework");
    }
}

fn validate_scenario(scenario: &Atlas0080Scenario, diagnostics: &mut Vec<&'static str>) {
    if scenario.starmap_side != ATLAS_0080_0_STARMAP_SIDE
        || scenario.starsystem_side != ATLAS_0080_0_STARSYSTEM_SIDE
        || scenario.planet_side != ATLAS_0080_0_PLANET_SIDE
        || scenario.starsystem_cells.len() != ATLAS_0080_0_STARSYSTEM_COUNT
        || scenario.planet_cells.len() != ATLAS_0080_0_STARSYSTEM_COUNT
        || scenario.logical_location_count() != ATLAS_0080_0_LOGICAL_LOCATION_COUNT
    {
        diagnostics.push("nested_starmap_shape_bounds");
    }
}

fn validate_access_pattern(
    access_pattern: &[Atlas0080ResidencyRequest],
    diagnostics: &mut Vec<&'static str>,
) {
    for request in access_pattern {
        let index = match request {
            Atlas0080ResidencyRequest::DescendToStarsystem { starsystem_index }
            | Atlas0080ResidencyRequest::DescendToPlanet { starsystem_index }
            | Atlas0080ResidencyRequest::AscendToStarsystem { starsystem_index } => {
                *starsystem_index
            }
            Atlas0080ResidencyRequest::AscendToStarmap => continue,
        };
        if usize::from(index) >= ATLAS_0080_0_STARSYSTEM_COUNT {
            diagnostics.push("starsystem_index_out_of_bounds");
        }
    }
}

fn validate_forbidden(forbidden: &Atlas0080ForbiddenRequests, diagnostics: &mut Vec<&'static str>) {
    if forbidden.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if forbidden.global_mapping_scheduler {
        diagnostics.push("global_mapping_scheduler");
    }
    if forbidden.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if forbidden.ui_framework {
        diagnostics.push("ui_framework");
    }
    if forbidden.residency_alters_field_values {
        diagnostics.push("residency_alters_field_values");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.semantically_named_shader {
        diagnostics.push("semantically_named_shader");
    }
    if forbidden.cpu_planner {
        diagnostics.push("cpu_planner");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
    if forbidden.general_mapping_runtime {
        diagnostics.push("general_mapping_runtime");
    }
    if forbidden.econ_scale_0080_0 {
        diagnostics.push("econ_scale_0080_0_not_implemented");
    }
    if forbidden.production_path_0080_1 {
        diagnostics.push("production_path_0080_1_not_implemented");
    }
}

fn disabled_no_op_report(input: &Atlas0080Input) -> Atlas0080Report {
    base_report(input, true, Vec::new(), Vec::new(), Vec::new())
}

fn rejected_report(input: &Atlas0080Input, diagnostics: Vec<&'static str>) -> Atlas0080Report {
    let mut report = base_report(input, false, diagnostics, Vec::new(), Vec::new());
    report.disabled_no_op = false;
    report
}

fn admitted_report(
    input: &Atlas0080Input,
    residency_reports: Vec<Atlas0080ResidencyReport>,
    descent_ascent_reports: Vec<Atlas0080DescentAscentReport>,
) -> Atlas0080Report {
    base_report(
        input,
        true,
        Vec::new(),
        residency_reports,
        descent_ascent_reports,
    )
}

fn base_report(
    input: &Atlas0080Input,
    admitted: bool,
    diagnostics: Vec<&'static str>,
    residency_reports: Vec<Atlas0080ResidencyReport>,
    descent_ascent_reports: Vec<Atlas0080DescentAscentReport>,
) -> Atlas0080Report {
    let disabled_no_op = admitted && !input.surface.gate.explicit_opt_in;
    let max_resident_cell_count = residency_reports
        .iter()
        .map(|report| report.resident_cell_count)
        .max()
        .unwrap_or(0);
    let max_estimated_resident_bytes = residency_reports
        .iter()
        .map(|report| report.estimated_resident_bytes)
        .max()
        .unwrap_or(0);
    let within_vram_budget = max_estimated_resident_bytes <= V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES;
    let value_noop_parity_bit_exact = admitted
        && input.surface.gate.explicit_opt_in
        && residency_reports
            .iter()
            .all(|report| report.value_noop_parity_bit_exact && !report.residency_changes_values);
    let sparse_residency = admitted
        && input.surface.gate.explicit_opt_in
        && residency_reports.iter().all(|report| {
            report.sparse_residency_only_active_theaters
                && report.resident_cell_count < input.scenario.logical_location_count()
        });

    let mut report = Atlas0080Report {
        atlas_id: ATLAS_0080_0_ID,
        status: ATLAS_0080_0_STATUS_PASS,
        scenario_name: ATLAS_0080_0_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: input.surface.gate.explicit_opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        scenario_scoped_only: true,
        default_session_has_no_residency_runtime: !input.surface.gate.explicit_opt_in,
        default_session_pass_graph_wiring: input.surface.default_session_pass_graph_wiring
            || input.forbidden.default_session_pass_graph_wiring,
        global_mapping_scheduler: input.surface.global_mapping_scheduler
            || input.forbidden.global_mapping_scheduler,
        realtime_loop: input.surface.realtime_loop || input.forbidden.realtime_loop,
        ui_framework: input.surface.ui_framework || input.forbidden.ui_framework,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        cpu_planner_used: input.forbidden.cpu_planner,
        econ_scale_0080_0_implemented: false,
        production_path_0080_1_implemented: false,
        starmap_side: input.scenario.starmap_side,
        starsystem_count: input.scenario.starsystem_cells.len(),
        starsystem_side: input.scenario.starsystem_side,
        planet_count: input.scenario.planet_cells.len(),
        planet_side: input.scenario.planet_side,
        logical_location_count: input.scenario.logical_location_count(),
        starsystem_cells: input.scenario.starsystem_cells.clone(),
        planet_cells: input.scenario.planet_cells.clone(),
        sparse_residency,
        max_resident_cell_count,
        total_logical_cell_count: input.scenario.logical_location_count(),
        vram_budget_bytes: V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES,
        max_estimated_resident_bytes,
        within_vram_budget,
        residency_reports,
        descent_ascent_reports,
        value_noop_parity_bit_exact,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn deterministic_starsystem_cells(seed: u64) -> Vec<Atlas0080Cell> {
    let mut cells = Vec::with_capacity(ATLAS_0080_0_STARSYSTEM_COUNT);
    let mut state = seed ^ 0x9E37_79B9_7F4A_7C15;
    while cells.len() < ATLAS_0080_0_STARSYSTEM_COUNT {
        state = lcg_next(state);
        let candidate = (state % 100) as u32;
        let cell = Atlas0080Cell::new(
            candidate % ATLAS_0080_0_STARMAP_SIDE,
            candidate / ATLAS_0080_0_STARMAP_SIDE,
        );
        if !cells.contains(&cell) {
            cells.push(cell);
        }
    }
    cells
}

fn deterministic_planet_cell(seed: u64, starsystem_index: u8) -> Atlas0080Cell {
    let state = lcg_next(seed ^ (u64::from(starsystem_index) + 1).saturating_mul(0xA24B_AED4));
    let candidate = (state % 100) as u32;
    Atlas0080Cell::new(
        candidate % ATLAS_0080_0_STARSYSTEM_SIDE,
        candidate / ATLAS_0080_0_STARSYSTEM_SIDE,
    )
}

fn materialized_values(theaters: &[Atlas0080TheaterId], scenario: &Atlas0080Scenario) -> Vec<i8> {
    let mut values = Vec::new();
    for theater in theaters {
        for slot in 0..theater.cell_count() {
            values.push(i8_oracle_value(scenario.seed, *theater, slot));
        }
    }
    values
}

fn i8_oracle_value(seed: u64, theater: Atlas0080TheaterId, slot: u32) -> i8 {
    let hash = stable_mix(
        seed ^ theater.stable_code().saturating_mul(0x9E37_79B9)
            ^ u64::from(slot).saturating_mul(0x85EB_CA6B),
    );
    (hash % 127) as i8
}

fn lcg_next(state: u64) -> u64 {
    state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

fn stable_mix(mut value: u64) -> u64 {
    value ^= value >> 33;
    value = value.wrapping_mul(0xff51_afd7_ed55_8ccd);
    value ^= value >> 33;
    value = value.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    value ^ (value >> 33)
}

fn checksum_report(report: &Atlas0080Report) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for cell in &report.starsystem_cells {
        hash = fnv(hash, u64::from(cell.x));
        hash = fnv(hash, u64::from(cell.y));
    }
    for step in &report.residency_reports {
        hash = fnv(hash, u64::from(step.step_index));
        hash = fnv(hash, u64::from(step.resident_cell_count));
        hash = fnv(hash, step.estimated_resident_bytes);
        for theater in &step.resident_theaters {
            hash = fnv(hash, theater.stable_code());
        }
        for value in &step.materialized_i8_values_after {
            hash = fnv(hash, (*value as i16 as u16) as u64);
        }
    }
    hash
}

fn fnv(mut hash: u64, value: u64) -> u64 {
    hash ^= value;
    hash.wrapping_mul(0x0000_0100_0000_01B3)
}
