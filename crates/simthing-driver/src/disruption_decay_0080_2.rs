//! `DISRUPTION-DECAY-0080-2` — rung 1 of the Pirate Gradient Pathfinding scenario.
//!
//! Accumulated-disruption field with bounded-feedback decay, on a sparse set of location
//! nodes. Opt-in/default-off. Pure CPU-deterministic integer/fixed-point recurrence — this
//! module is the CPU oracle a later GPU kernel is checked against (I8 bit-exact parity).
//!
//! Per node, per tick (single-writer-per-band on the `disruption` column):
//!
//! ```text
//! retained   = floor(disruption * retain_num / retain_den)   // base decay, composed with modifiers (each <= 1)
//! gained     = mover_presence_units * gain_per_presence_unit  // accumulation while a mover is present
//! suppressed = patrol_presence     * suppression_per_patrol   // patrols accelerate removal
//! disruption = clamp(retained + gained - suppressed, 0, MAX)
//! ```
//!
//! The decay coefficient is a *read-side parameter*: a base weight (game-session config)
//! composed multiplicatively with retention modifiers (faction tech / starsystem / fleet),
//! each `<= 1` so they can only *accelerate* decay — which keeps the recurrence bounded by
//! construction. The composed coefficient must stay in `[0, 1)` (bounded-feedback admission).
//! No global decay overlay *write*; no destructive root mutation; decay applies only to this
//! opt-in column. No gradient-follow movement (later rung); no GPU kernel (later rung); no
//! CPU planner; does not reopen the closed `0080-1` ladder.

pub const DISRUPTION_DECAY_0080_2_ID: &str = "DISRUPTION-DECAY-0080-2";
pub const DISRUPTION_DECAY_0080_2_SCENARIO: &str = "Pirate Gradient Pathfinding";
pub const DISRUPTION_DECAY_0080_2_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - scenario-scoped accumulated-disruption bounded-feedback decay field";

/// Fixed-point scale: one disruption unit = `DISRUPTION_SCALE` integer milliunits.
pub const DISRUPTION_SCALE: i64 = 1_000;
/// Bounded ceiling on accumulated disruption (clamp target).
pub const DISRUPTION_MAX: i64 = 100 * DISRUPTION_SCALE;

const DEFAULT_TICK_COUNT: u32 = 20;
const DEFAULT_NODE_COUNT: usize = 4;
const MAX_DECAY_MODIFIERS: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisruptionDecay0082Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl DisruptionDecay0082Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisruptionDecay0082Surface {
    pub gate: DisruptionDecay0082Gate,
    pub scenario_scoped_field_registered: bool,
    pub global_decay_overlay_write: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub gradient_follow_movement: bool,
    pub realtime_loop: bool,
    pub ui_framework: bool,
}

impl DisruptionDecay0082Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: DisruptionDecay0082Gate::explicit_opt_in(),
            scenario_scoped_field_registered: true,
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisruptionDecay0082ForbiddenRequests {
    pub global_decay_overlay_write: bool,
    pub unbounded_decay_coefficient: bool,
    pub decay_writes_non_subscribed_column: bool,
    pub second_writer_on_disruption_column: bool,
    pub gradient_follow_movement: bool,
    pub direct_movement_command: bool,
    pub external_boundary_request: bool,
    pub cpu_planner_urgency_commitment: bool,
    pub default_session_pass_graph_wiring: bool,
    pub global_default_schedule: bool,
    pub realtime_loop_or_ui: bool,
    pub semantic_or_raw_wgsl: bool,
    pub new_shader_or_gpu_kernel: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub clausething_dependency: bool,
    pub simthing_spec_alteration: bool,
    pub invariant_edit: bool,
    pub reopen_closed_0080_1_ladder: bool,
}

/// A multiplicative retention factor (`num/den`, with `num <= den` so it is `<= 1` and only
/// *accelerates* decay). Sourced from faction tech / starsystem natural bonus / patrol fleet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisruptionDecay0082RetentionFactor {
    pub label: &'static str,
    pub num: u64,
    pub den: u64,
}

/// Composed decay weight: a base retention (game-session config, `< 1`) times any number of
/// `<= 1` retention modifiers broadcast down the flow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisruptionDecay0082DecayWeights {
    pub base_retain_num: u64,
    pub base_retain_den: u64,
    pub modifiers: Vec<DisruptionDecay0082RetentionFactor>,
}

impl DisruptionDecay0082DecayWeights {
    /// Base 0.9 retention (game-session default) with a -5% faction-tech retention modifier
    /// (0.95) — composed effective 0.855 = 171/200.
    pub fn canonical() -> Self {
        Self {
            base_retain_num: 9,
            base_retain_den: 10,
            modifiers: vec![DisruptionDecay0082RetentionFactor {
                label: "faction_tech_decay_bonus",
                num: 95,
                den: 100,
            }],
        }
    }
}

/// Scripted per-node presence for one tick (rung 1 drives presence externally; gradient-follow
/// movement is a later rung).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DisruptionDecay0082Presence {
    pub mover_presence_units: i64,
    pub patrol_presence: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisruptionDecay0082Input {
    pub surface: DisruptionDecay0082Surface,
    pub node_count: usize,
    pub tick_count: u32,
    pub decay_weights: DisruptionDecay0082DecayWeights,
    pub gain_per_presence_unit: i64,
    pub suppression_per_patrol: i64,
    /// `presence_schedule[tick][node]`.
    pub presence_schedule: Vec<Vec<DisruptionDecay0082Presence>>,
    pub forbidden: DisruptionDecay0082ForbiddenRequests,
}

impl DisruptionDecay0082Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: DisruptionDecay0082Surface::default_simsession(),
            node_count: 0,
            tick_count: 0,
            decay_weights: DisruptionDecay0082DecayWeights::canonical(),
            gain_per_presence_unit: 0,
            suppression_per_patrol: 0,
            presence_schedule: Vec::new(),
            forbidden: DisruptionDecay0082ForbiddenRequests::default(),
        }
    }

    /// Canonical 4-node, 20-tick trial demonstrating accumulate / base-decay / patrol-accelerated
    /// decay / saturation:
    /// - node 0: pirate present ticks 0..=5, then nothing until a patrol arrives ticks 12..=19.
    /// - node 1: never any presence (control — stays 0).
    /// - node 2: patrol present throughout, never a pirate (clean system — stays 0).
    /// - node 3: pirate present every tick (saturates against the clamp ceiling).
    pub fn explicit_opt_in() -> Self {
        let node_count = DEFAULT_NODE_COUNT;
        let tick_count = DEFAULT_TICK_COUNT;
        let mut presence_schedule = Vec::with_capacity(tick_count as usize);
        for tick in 0..tick_count {
            let mut row = vec![DisruptionDecay0082Presence::default(); node_count];
            // node 0
            if tick <= 5 {
                row[0].mover_presence_units = 10;
            } else if (12..=19).contains(&tick) {
                row[0].patrol_presence = 1;
            }
            // node 2: patrol present throughout
            row[2].patrol_presence = 1;
            // node 3: pirate present every tick
            row[3].mover_presence_units = 10;
            presence_schedule.push(row);
        }
        Self {
            surface: DisruptionDecay0082Surface::with_explicit_opt_in(),
            node_count,
            tick_count,
            decay_weights: DisruptionDecay0082DecayWeights::canonical(),
            gain_per_presence_unit: 2_000,
            suppression_per_patrol: 5_000,
            presence_schedule,
            forbidden: DisruptionDecay0082ForbiddenRequests::default(),
        }
    }
}

/// One node's state transition for one tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisruptionDecay0082Row {
    pub tick: u32,
    pub node: usize,
    pub mover_presence_units: i64,
    pub patrol_presence: i64,
    pub disruption_before: i64,
    pub retained: i64,
    pub gained: i64,
    pub suppressed: i64,
    pub disruption_after: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisruptionDecay0082Report {
    pub id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,

    // Guardrail confirmations.
    pub no_global_decay_overlay_write: bool,
    pub decay_coefficient_bounded: bool,
    pub single_writer_per_disruption_column: bool,
    pub no_gradient_follow_movement: bool,
    pub no_new_gpu_kernel: bool,
    pub no_cpu_planner: bool,
    pub no_default_session_pass_graph_wiring: bool,
    pub no_global_default_schedule: bool,
    pub does_not_reopen_closed_0080_1_ladder: bool,

    // Field results.
    pub node_count: usize,
    pub tick_count: u32,
    pub effective_retain_num: u64,
    pub effective_retain_den: u64,
    pub rows: Vec<DisruptionDecay0082Row>,
    pub final_disruption: Vec<i64>,
    pub peak_disruption: Vec<i64>,

    // Behavioral diagnostics.
    pub accumulates_with_presence: bool,
    pub decays_to_zero_without_input: bool,
    pub patrol_accelerates_decay: bool,
    pub saturates_at_ceiling: bool,

    pub text_export: String,
    pub deterministic_replay_checksum: u64,
}

/// Run the accumulated-disruption decay field for one opt-in input.
pub fn run_disruption_decay_0080_2(input: &DisruptionDecay0082Input) -> DisruptionDecay0082Report {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    // Compose + bound the decay coefficient.
    let composed = compose_retention(&input.decay_weights);
    if composed.is_none() {
        diagnostics.push("decay_coefficient_invalid");
    }
    let bounded = composed
        .map(|(num, den)| num < den && den != 0)
        .unwrap_or(false);
    if composed.is_some() && !bounded {
        diagnostics.push("unbounded_decay_coefficient");
    }
    validate_params(input, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let (retain_num, retain_den) = composed.expect("validated above");
    let run = run_ticks(input, retain_num, retain_den);
    admitted_report(input, retain_num, retain_den, run)
}

/// Two identical runs for deterministic-replay assertions.
pub fn replay_disruption_decay_0080_2() -> (DisruptionDecay0082Report, DisruptionDecay0082Report) {
    let input = DisruptionDecay0082Input::explicit_opt_in();
    (
        run_disruption_decay_0080_2(&input),
        run_disruption_decay_0080_2(&input),
    )
}

fn validate_surface(surface: &DisruptionDecay0082Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("disruption_decay_default_on_rejected");
    }
    if surface.global_decay_overlay_write {
        diagnostics.push("global_decay_overlay_write_rejected");
    }
    if surface.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if surface.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if surface.gradient_follow_movement {
        diagnostics.push("gradient_follow_movement_not_in_this_rung");
    }
    if surface.realtime_loop {
        diagnostics.push("realtime_loop");
    }
    if surface.ui_framework {
        diagnostics.push("ui_framework");
    }
}

fn validate_forbidden(
    forbidden: &DisruptionDecay0082ForbiddenRequests,
    diagnostics: &mut Vec<&'static str>,
) {
    if forbidden.global_decay_overlay_write {
        diagnostics.push("global_decay_overlay_write_rejected");
    }
    if forbidden.unbounded_decay_coefficient {
        diagnostics.push("unbounded_decay_coefficient");
    }
    if forbidden.decay_writes_non_subscribed_column {
        diagnostics.push("decay_writes_non_subscribed_column");
    }
    if forbidden.second_writer_on_disruption_column {
        diagnostics.push("second_writer_on_disruption_column");
    }
    if forbidden.gradient_follow_movement {
        diagnostics.push("gradient_follow_movement_not_in_this_rung");
    }
    if forbidden.direct_movement_command {
        diagnostics.push("direct_movement_command");
    }
    if forbidden.external_boundary_request {
        diagnostics.push("external_boundary_request");
    }
    if forbidden.cpu_planner_urgency_commitment {
        diagnostics.push("cpu_planner_urgency_commitment");
    }
    if forbidden.default_session_pass_graph_wiring {
        diagnostics.push("default_session_pass_graph_wiring");
    }
    if forbidden.global_default_schedule {
        diagnostics.push("global_default_schedule");
    }
    if forbidden.realtime_loop_or_ui {
        diagnostics.push("realtime_loop_or_ui");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.new_shader_or_gpu_kernel {
        diagnostics.push("new_shader_or_gpu_kernel");
    }
    if forbidden.hard_currency_markets_trade_aibudget {
        diagnostics.push("hard_currency_markets_trade_aibudget");
    }
    if forbidden.nested_resource_flow {
        diagnostics.push("nested_resource_flow");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.simthing_spec_alteration {
        diagnostics.push("simthing_spec_alteration");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
    if forbidden.reopen_closed_0080_1_ladder {
        diagnostics.push("reopen_closed_0080_1_ladder");
    }
}

fn validate_params(input: &DisruptionDecay0082Input, diagnostics: &mut Vec<&'static str>) {
    if input.decay_weights.modifiers.len() > MAX_DECAY_MODIFIERS {
        diagnostics.push("too_many_decay_modifiers");
    }
    for modifier in &input.decay_weights.modifiers {
        if modifier.den == 0 {
            diagnostics.push("decay_modifier_denominator_zero");
        } else if modifier.num > modifier.den {
            // A modifier > 1 would *increase* retention; rung 1 admits acceleration-only.
            diagnostics.push("decay_modifier_increases_retention");
        }
    }
    if input.gain_per_presence_unit < 0 {
        diagnostics.push("negative_gain_rate");
    }
    if input.suppression_per_patrol < 0 {
        diagnostics.push("negative_suppression_rate");
    }
    if input.surface.gate.explicit_opt_in {
        if input.node_count == 0 {
            diagnostics.push("empty_node_set");
        }
        if input.tick_count == 0 {
            diagnostics.push("empty_tick_set");
        }
        if input.presence_schedule.len() != input.tick_count as usize {
            diagnostics.push("presence_schedule_tick_shape_mismatch");
        }
        if input
            .presence_schedule
            .iter()
            .any(|row| row.len() != input.node_count)
        {
            diagnostics.push("presence_schedule_node_shape_mismatch");
        }
        if input.presence_schedule.iter().flatten().any(|p| {
            p.mover_presence_units < 0 || p.patrol_presence < 0
        }) {
            diagnostics.push("negative_presence");
        }
    }
}

struct RunResult {
    rows: Vec<DisruptionDecay0082Row>,
    final_disruption: Vec<i64>,
    peak_disruption: Vec<i64>,
}

fn run_ticks(input: &DisruptionDecay0082Input, retain_num: u64, retain_den: u64) -> RunResult {
    let node_count = input.node_count;
    let mut disruption = vec![0i64; node_count];
    let mut peak = vec![0i64; node_count];
    let mut rows = Vec::with_capacity(input.tick_count as usize * node_count);

    for tick in 0..input.tick_count {
        let presence_row = &input.presence_schedule[tick as usize];
        for node in 0..node_count {
            let presence = presence_row[node];
            let before = disruption[node];
            // Single writer per band: this update is the sole producer of disruption[node].
            let retained =
                ((before as u128 * retain_num as u128) / retain_den as u128) as i64;
            let gained = presence
                .mover_presence_units
                .saturating_mul(input.gain_per_presence_unit);
            let suppressed = presence
                .patrol_presence
                .saturating_mul(input.suppression_per_patrol);
            let after = (retained + gained - suppressed).clamp(0, DISRUPTION_MAX);
            disruption[node] = after;
            if after > peak[node] {
                peak[node] = after;
            }
            rows.push(DisruptionDecay0082Row {
                tick,
                node,
                mover_presence_units: presence.mover_presence_units,
                patrol_presence: presence.patrol_presence,
                disruption_before: before,
                retained,
                gained,
                suppressed,
                disruption_after: after,
            });
        }
    }

    RunResult {
        final_disruption: disruption,
        peak_disruption: peak,
        rows,
    }
}

/// Compose the base retention with all modifiers (multiplicatively), reduced by gcd. Returns
/// `None` if any denominator is zero or the reduced fraction does not fit `u64`.
fn compose_retention(weights: &DisruptionDecay0082DecayWeights) -> Option<(u64, u64)> {
    if weights.base_retain_den == 0 {
        return None;
    }
    let mut num: u128 = weights.base_retain_num as u128;
    let mut den: u128 = weights.base_retain_den as u128;
    for modifier in &weights.modifiers {
        if modifier.den == 0 {
            return None;
        }
        num = num.checked_mul(modifier.num as u128)?;
        den = den.checked_mul(modifier.den as u128)?;
    }
    let g = gcd_u128(num, den);
    num /= g;
    den /= g;
    if num > u64::MAX as u128 || den > u64::MAX as u128 {
        return None;
    }
    Some((num as u64, den as u64))
}

fn gcd_u128(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

fn disabled_no_op_report(input: &DisruptionDecay0082Input) -> DisruptionDecay0082Report {
    base_report(input, true, Vec::new(), 0, 0, None)
}

fn rejected_report(
    input: &DisruptionDecay0082Input,
    diagnostics: Vec<&'static str>,
) -> DisruptionDecay0082Report {
    let mut report = base_report(input, false, diagnostics, 0, 0, None);
    report.admitted = false;
    report
}

fn admitted_report(
    input: &DisruptionDecay0082Input,
    retain_num: u64,
    retain_den: u64,
    run: RunResult,
) -> DisruptionDecay0082Report {
    base_report(input, false, Vec::new(), retain_num, retain_den, Some(run))
}

fn base_report(
    input: &DisruptionDecay0082Input,
    disabled_no_op: bool,
    diagnostics: Vec<&'static str>,
    retain_num: u64,
    retain_den: u64,
    run: Option<RunResult>,
) -> DisruptionDecay0082Report {
    let opt_in = input.surface.gate.explicit_opt_in;
    let (rows, final_disruption, peak_disruption) = match run {
        Some(r) => (r.rows, r.final_disruption, r.peak_disruption),
        None => (Vec::new(), Vec::new(), Vec::new()),
    };

    let accumulates_with_presence = rows
        .iter()
        .any(|r| r.gained > 0 && r.disruption_after > r.disruption_before);
    let decays_to_zero_without_input = detect_decay_to_zero(&rows);
    let patrol_accelerates_decay = detect_patrol_acceleration(&rows);
    let saturates_at_ceiling = peak_disruption.iter().any(|&v| v >= DISRUPTION_MAX);

    let text_export = if !disabled_no_op && opt_in {
        render_export(input, retain_num, retain_den, &rows, &final_disruption)
    } else {
        String::new()
    };

    let mut report = DisruptionDecay0082Report {
        id: DISRUPTION_DECAY_0080_2_ID,
        status: DISRUPTION_DECAY_0080_2_STATUS_PASS,
        scenario_name: DISRUPTION_DECAY_0080_2_SCENARIO,
        admitted: true,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        no_global_decay_overlay_write: !input.surface.global_decay_overlay_write
            && !input.forbidden.global_decay_overlay_write,
        decay_coefficient_bounded: retain_den != 0 && retain_num < retain_den,
        single_writer_per_disruption_column: !input.forbidden.second_writer_on_disruption_column,
        no_gradient_follow_movement: !input.surface.gradient_follow_movement
            && !input.forbidden.gradient_follow_movement,
        no_new_gpu_kernel: !input.forbidden.new_shader_or_gpu_kernel,
        no_cpu_planner: !input.forbidden.cpu_planner_urgency_commitment,
        no_default_session_pass_graph_wiring: !input.surface.default_session_pass_graph_wiring
            && !input.forbidden.default_session_pass_graph_wiring,
        no_global_default_schedule: !input.surface.global_default_schedule
            && !input.forbidden.global_default_schedule,
        does_not_reopen_closed_0080_1_ladder: !input.forbidden.reopen_closed_0080_1_ladder,
        node_count: if disabled_no_op { 0 } else { input.node_count },
        tick_count: if disabled_no_op { 0 } else { input.tick_count },
        effective_retain_num: retain_num,
        effective_retain_den: retain_den,
        rows,
        final_disruption,
        peak_disruption,
        accumulates_with_presence,
        decays_to_zero_without_input,
        patrol_accelerates_decay,
        saturates_at_ceiling,
        text_export,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

/// "Gravity to zero without participation": a node holding disruption with no mover and no
/// patrol present must strictly decrease that tick (pure base decay), and never increase under
/// those conditions.
fn detect_decay_to_zero(rows: &[DisruptionDecay0082Row]) -> bool {
    let mut found = false;
    for r in rows {
        if r.mover_presence_units == 0 && r.patrol_presence == 0 && r.disruption_before > 0 {
            if r.disruption_after >= r.disruption_before {
                return false;
            }
            found = true;
        }
    }
    found
}

/// A tick with patrol presence (and no mover input) must remove more disruption than pure base
/// decay would: `suppressed > 0` and the after-value is strictly below the retained-only value.
fn detect_patrol_acceleration(rows: &[DisruptionDecay0082Row]) -> bool {
    rows.iter().any(|r| {
        r.patrol_presence > 0
            && r.mover_presence_units == 0
            && r.suppressed > 0
            && r.disruption_before > 0
            && r.disruption_after < r.retained
    })
}

fn render_export(
    input: &DisruptionDecay0082Input,
    retain_num: u64,
    retain_den: u64,
    rows: &[DisruptionDecay0082Row],
    final_disruption: &[i64],
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "DISRUPTION-DECAY-0080-2|scenario={}|nodes={}|ticks={}|retain={}/{}|gain_per_unit={}|suppression_per_patrol={}",
        DISRUPTION_DECAY_0080_2_SCENARIO,
        input.node_count,
        input.tick_count,
        retain_num,
        retain_den,
        input.gain_per_presence_unit,
        input.suppression_per_patrol,
    ));
    for row in rows {
        lines.push(format!(
            "TICK|t={}|node={}|mover={}|patrol={}|before={}|retained={}|gained={}|suppressed={}|after={}",
            row.tick,
            row.node,
            row.mover_presence_units,
            row.patrol_presence,
            row.disruption_before,
            row.retained,
            row.gained,
            row.suppressed,
            row.disruption_after,
        ));
    }
    for (node, value) in final_disruption.iter().enumerate() {
        lines.push(format!("FINAL|node={}|disruption={}", node, value));
    }
    lines.join("\n")
}

fn checksum_report(report: &DisruptionDecay0082Report) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325;
    hash = fnv_append_u64(hash, report.node_count as u64);
    hash = fnv_append_u64(hash, report.tick_count as u64);
    hash = fnv_append_u64(hash, report.effective_retain_num);
    hash = fnv_append_u64(hash, report.effective_retain_den);
    for row in &report.rows {
        hash = fnv_append_u64(hash, row.tick as u64);
        hash = fnv_append_u64(hash, row.node as u64);
        hash = fnv_append_u64(hash, row.disruption_after as u64);
        hash = fnv_append_u64(hash, row.retained as u64);
        hash = fnv_append_u64(hash, row.suppressed as u64);
    }
    for value in &report.final_disruption {
        hash = fnv_append_u64(hash, *value as u64);
    }
    for value in &report.peak_disruption {
        hash = fnv_append_u64(hash, *value as u64);
    }
    hash
}

fn fnv_append_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
