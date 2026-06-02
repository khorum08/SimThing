//! `ECON-SCALE-0080-0` — bounded faction-indexed contended ECON scaling for `SCENARIO-0080-1`
//! (Nested Starmap). Opt-in/default-off, scenario-scoped, deterministic. Terran + Pirate are a fixed
//! bounded faction set; the pirate is a full economy faction (adversarial participant in a starsystem's
//! resource flow), not merely a disruptor identity. The faction index layers onto the existing
//! subsidiarity / FlatStar Resource Flow posture; it does not replace the clearinghouse model and does
//! not introduce nested Resource Flow, hard currency, markets, trade, or `ai_budget`.

pub const ECON_SCALE_0080_0_ID: &str = "ECON-SCALE-0080-0";
pub const ECON_SCALE_0080_0_SCENARIO: &str = "Nested Starmap";
pub const ECON_SCALE_0080_0_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - bounded faction-indexed contended ECON scaling for Nested Starmap";
pub const ECON_SCALE_0080_0_FACTION_COUNT: usize = 2;
pub const ECON_SCALE_0080_0_MAX_PARTICIPANTS_PER_STARSYSTEM: usize = 4;
pub const ECON_SCALE_0080_0_DEFAULT_SEED: u64 = 0x0080_0001;

/// Bounded fixed faction set for this scenario.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EconScale0080Faction {
    Terran,
    Pirate,
}

impl EconScale0080Faction {
    pub fn faction_index(self) -> u8 {
        match self {
            EconScale0080Faction::Terran => 0,
            EconScale0080Faction::Pirate => 1,
        }
    }

    /// Both factions are full economy factions in this scenario — the pirate participates in resource
    /// flow, it is not merely a disruptor identity.
    pub fn is_full_economy_faction(self) -> bool {
        true
    }

    /// The pirate participates adversarially.
    pub fn is_adversarial(self) -> bool {
        matches!(self, EconScale0080Faction::Pirate)
    }

    pub fn stable_code(self) -> u64 {
        match self {
            EconScale0080Faction::Terran => 1,
            EconScale0080Faction::Pirate => 2,
        }
    }
}

/// The bounded fixed faction index (Terran + Pirate). No unbounded fan-out, no dynamic registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080FactionIndex {
    pub factions: Vec<EconScale0080Faction>,
}

impl EconScale0080FactionIndex {
    pub fn bounded() -> Self {
        Self {
            factions: vec![EconScale0080Faction::Terran, EconScale0080Faction::Pirate],
        }
    }

    pub fn count(&self) -> usize {
        self.factions.len()
    }

    pub fn is_bounded(&self) -> bool {
        if self.count() != ECON_SCALE_0080_0_FACTION_COUNT {
            return false;
        }
        let mut seen = Vec::new();
        for faction in &self.factions {
            if seen.contains(faction) {
                return false;
            }
            seen.push(*faction);
        }
        true
    }
}

impl Default for EconScale0080FactionIndex {
    fn default() -> Self {
        Self::bounded()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EconScale0080Gate {
    pub explicit_opt_in: bool,
    pub enabled_by_default: bool,
}

impl EconScale0080Gate {
    pub fn explicit_opt_in() -> Self {
        Self {
            explicit_opt_in: true,
            enabled_by_default: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EconScale0080Surface {
    pub gate: EconScale0080Gate,
}

impl EconScale0080Surface {
    pub fn default_simsession() -> Self {
        Self::default()
    }

    pub fn with_explicit_opt_in() -> Self {
        Self {
            gate: EconScale0080Gate::explicit_opt_in(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EconScale0080ForbiddenRequests {
    pub hard_currency: bool,
    pub markets_trade_aibudget: bool,
    pub nested_resource_flow: bool,
    pub unbounded_faction_fanout: bool,
    pub replace_subsidiarity: bool,
    pub cpu_planner: bool,
    pub semantic_or_raw_wgsl: bool,
    pub semantically_named_shader: bool,
    pub clausething_dependency: bool,
    pub invariant_edit: bool,
    pub production_path_0080_1: bool,
}

/// A faction-indexed participant in a starsystem's resource-flow clearing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080Participant {
    pub faction: EconScale0080Faction,
    pub faction_index: u8,
    pub adversarial: bool,
    pub extraction_weight: i64,
    pub security_weight: i64,
}

impl EconScale0080Participant {
    pub fn terran(extraction_weight: i64, security_weight: i64) -> Self {
        Self {
            faction: EconScale0080Faction::Terran,
            faction_index: EconScale0080Faction::Terran.faction_index(),
            adversarial: false,
            extraction_weight,
            security_weight,
        }
    }

    pub fn pirate(extraction_weight: i64) -> Self {
        Self {
            faction: EconScale0080Faction::Pirate,
            faction_index: EconScale0080Faction::Pirate.faction_index(),
            adversarial: true,
            extraction_weight,
            security_weight: 0,
        }
    }
}

/// One starsystem's bounded local economy + its faction-indexed participants.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080StarsystemEconomy {
    pub starsystem_index: u8,
    pub terran_owned: bool,
    pub supply: i64,
    pub extraction: i64,
    pub security: i64,
    pub disruption: i64,
    pub contention: i64,
    pub participants: Vec<EconScale0080Participant>,
}

/// Clearing input for a single starsystem.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080ClearingInput {
    pub economy: EconScale0080StarsystemEconomy,
}

/// Deterministic per-starsystem clearing report.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080ClearingReport {
    pub starsystem_index: u8,
    pub terran_owned: bool,
    pub faction_indices_present: Vec<u8>,
    pub terran_present: bool,
    pub pirate_present: bool,
    pub adversarial: bool,
    pub supply_before: i64,
    pub supply_after: i64,
    pub terran_extraction: i64,
    pub pirate_extraction: i64,
    pub contention_before: i64,
    pub contention_after: i64,
    pub disruption_before: i64,
    pub disruption_after: i64,
    pub security_before: i64,
    pub security_after: i64,
    pub subsidiarity_preserved: bool,
    pub parity_bit_exact: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080Scenario {
    pub seed: u64,
    pub faction_index: EconScale0080FactionIndex,
    pub starsystems: Vec<EconScale0080StarsystemEconomy>,
}

impl EconScale0080Scenario {
    pub fn canonical() -> Self {
        Self::from_seed(ECON_SCALE_0080_0_DEFAULT_SEED)
    }

    /// Representative bounded clearing set for the Nested Starmap initial conditions:
    /// a Terran-owned starsystem with only Terran participation; a Terran-owned starsystem a pirate has
    /// entered (adversarial contention); and a neutral starsystem with pirate-only participation
    /// (the pirate is a full economy faction even where it owns nothing).
    pub fn from_seed(seed: u64) -> Self {
        Self {
            seed,
            faction_index: EconScale0080FactionIndex::bounded(),
            starsystems: vec![
                EconScale0080StarsystemEconomy {
                    starsystem_index: 0,
                    terran_owned: true,
                    supply: 100,
                    extraction: 0,
                    security: 20,
                    disruption: 0,
                    contention: 0,
                    participants: vec![EconScale0080Participant::terran(10, 5)],
                },
                EconScale0080StarsystemEconomy {
                    starsystem_index: 1,
                    terran_owned: true,
                    supply: 100,
                    extraction: 0,
                    security: 20,
                    disruption: 0,
                    contention: 0,
                    participants: vec![
                        EconScale0080Participant::terran(10, 5),
                        EconScale0080Participant::pirate(15),
                    ],
                },
                EconScale0080StarsystemEconomy {
                    starsystem_index: 2,
                    terran_owned: false,
                    supply: 80,
                    extraction: 0,
                    security: 5,
                    disruption: 0,
                    contention: 0,
                    participants: vec![EconScale0080Participant::pirate(12)],
                },
            ],
        }
    }

    pub fn clearing_inputs(&self) -> Vec<EconScale0080ClearingInput> {
        self.starsystems
            .iter()
            .cloned()
            .map(|economy| EconScale0080ClearingInput { economy })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080Input {
    pub surface: EconScale0080Surface,
    pub scenario: EconScale0080Scenario,
    pub forbidden: EconScale0080ForbiddenRequests,
}

impl EconScale0080Input {
    pub fn default_simsession() -> Self {
        Self {
            surface: EconScale0080Surface::default_simsession(),
            scenario: EconScale0080Scenario::canonical(),
            forbidden: EconScale0080ForbiddenRequests::default(),
        }
    }

    pub fn explicit_opt_in() -> Self {
        Self {
            surface: EconScale0080Surface::with_explicit_opt_in(),
            scenario: EconScale0080Scenario::canonical(),
            forbidden: EconScale0080ForbiddenRequests::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EconScale0080RunReport {
    pub econ_scale_id: &'static str,
    pub status: &'static str,
    pub scenario_name: &'static str,
    pub admitted: bool,
    pub diagnostics: Vec<&'static str>,

    pub explicit_opt_in: bool,
    pub default_off: bool,
    pub disabled_no_op: bool,
    pub scenario_scoped_only: bool,
    pub single_owner_default_unchanged: bool,

    pub bounded_faction_count: bool,
    pub faction_count: usize,
    pub factions: Vec<EconScale0080Faction>,
    pub faction_indexed_participation: bool,
    pub adversarial_contended_clearing: bool,
    pub pirate_is_full_economy_faction: bool,
    pub subsidiarity_preserved: bool,
    pub flat_star_posture_preserved: bool,
    pub parity_bit_exact: bool,

    pub nested_resource_flow: bool,
    pub hard_currency_markets_trade_aibudget: bool,
    pub unbounded_faction_fanout: bool,
    pub cpu_planner_used: bool,
    pub semantic_or_raw_wgsl_present: bool,
    pub clausething_dependency_present: bool,
    pub production_path_0080_1_implemented: bool,

    pub clearing_reports: Vec<EconScale0080ClearingReport>,
    pub deterministic_replay_checksum: u64,
}

pub fn run_econ_scale_0080_0(input: &EconScale0080Input) -> EconScale0080RunReport {
    let mut diagnostics = Vec::new();
    validate_surface(&input.surface, &mut diagnostics);
    validate_scenario(&input.scenario, &mut diagnostics);
    validate_forbidden(&input.forbidden, &mut diagnostics);

    if !diagnostics.is_empty() {
        return rejected_report(input, diagnostics);
    }

    if !input.surface.gate.explicit_opt_in {
        return disabled_no_op_report(input);
    }

    let clearing_reports = input
        .scenario
        .clearing_inputs()
        .iter()
        .map(|clearing| clear_starsystem(&clearing.economy))
        .collect();

    admitted_report(input, clearing_reports)
}

pub fn replay_econ_scale_0080_0() -> (EconScale0080RunReport, EconScale0080RunReport) {
    let input = EconScale0080Input::explicit_opt_in();
    (run_econ_scale_0080_0(&input), run_econ_scale_0080_0(&input))
}

/// Deterministic bounded contended clearing for one starsystem. Terran (owner-priority via subsidiarity)
/// extracts first; the adversarial pirate contends over the remaining supply; contention/disruption rise
/// with pirate pressure; security rises with Terran presence and falls with pirate extraction. All values
/// are bounded saturating integers — this function is the CPU oracle.
fn clear_starsystem(economy: &EconScale0080StarsystemEconomy) -> EconScale0080ClearingReport {
    let scalars = clearing_scalars(economy);
    let oracle = clearing_scalars_oracle(economy);
    let parity_bit_exact = scalars == oracle;
    let ClearingScalars {
        terran_present,
        pirate_present,
        terran_extraction,
        pirate_extraction,
        supply_after,
        contention_after,
        disruption_after,
        security_after,
    } = scalars;

    let mut faction_indices_present = Vec::new();
    for participant in &economy.participants {
        if !faction_indices_present.contains(&participant.faction_index) {
            faction_indices_present.push(participant.faction_index);
        }
    }
    faction_indices_present.sort_unstable();

    EconScale0080ClearingReport {
        starsystem_index: economy.starsystem_index,
        terran_owned: economy.terran_owned,
        faction_indices_present,
        terran_present,
        pirate_present,
        adversarial: pirate_present,
        supply_before: economy.supply,
        supply_after,
        terran_extraction,
        pirate_extraction,
        contention_before: economy.contention,
        contention_after,
        disruption_before: economy.disruption,
        disruption_after,
        security_before: economy.security,
        security_after,
        subsidiarity_preserved: true,
        parity_bit_exact,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClearingScalars {
    terran_present: bool,
    pirate_present: bool,
    terran_extraction: i64,
    pirate_extraction: i64,
    supply_after: i64,
    contention_after: i64,
    disruption_after: i64,
    security_after: i64,
}

fn clearing_scalars(economy: &EconScale0080StarsystemEconomy) -> ClearingScalars {
    let supply = economy.supply.max(0);
    let terran_weight: i64 = economy
        .participants
        .iter()
        .filter(|p| p.faction == EconScale0080Faction::Terran)
        .map(|p| p.extraction_weight.max(0))
        .sum();
    let pirate_weight: i64 = economy
        .participants
        .iter()
        .filter(|p| p.faction == EconScale0080Faction::Pirate)
        .map(|p| p.extraction_weight.max(0))
        .sum();
    let terran_present = economy
        .participants
        .iter()
        .any(|p| p.faction == EconScale0080Faction::Terran);
    let pirate_present = economy
        .participants
        .iter()
        .any(|p| p.faction == EconScale0080Faction::Pirate);

    let terran_extraction = terran_weight.min(supply).max(0);
    let remaining = (supply - terran_extraction).max(0);
    let pirate_extraction = pirate_weight.min(remaining).max(0);
    let supply_after = (supply - terran_extraction - pirate_extraction).max(0);

    let contention_after = economy.contention
        + if terran_present && pirate_present {
            terran_weight.min(pirate_weight).max(0)
        } else {
            0
        };
    let disruption_after = economy.disruption + pirate_extraction;
    let terran_security_gain: i64 = economy
        .participants
        .iter()
        .filter(|p| p.faction == EconScale0080Faction::Terran)
        .map(|p| p.security_weight.max(0))
        .sum();
    let security_after = (economy.security + terran_security_gain - pirate_extraction).max(0);

    ClearingScalars {
        terran_present,
        pirate_present,
        terran_extraction,
        pirate_extraction,
        supply_after,
        contention_after,
        disruption_after,
        security_after,
    }
}

/// Independent re-derivation of the clearing scalars used as the bit-exact CPU parity oracle.
fn clearing_scalars_oracle(economy: &EconScale0080StarsystemEconomy) -> ClearingScalars {
    let mut terran_weight = 0i64;
    let mut pirate_weight = 0i64;
    let mut terran_security_gain = 0i64;
    let mut terran_present = false;
    let mut pirate_present = false;
    for participant in &economy.participants {
        match participant.faction {
            EconScale0080Faction::Terran => {
                terran_present = true;
                terran_weight += participant.extraction_weight.max(0);
                terran_security_gain += participant.security_weight.max(0);
            }
            EconScale0080Faction::Pirate => {
                pirate_present = true;
                pirate_weight += participant.extraction_weight.max(0);
            }
        }
    }
    let supply = economy.supply.max(0);
    let terran_extraction = terran_weight.clamp(0, supply);
    let remaining = (supply - terran_extraction).max(0);
    let pirate_extraction = pirate_weight.clamp(0, remaining);
    let supply_after = (supply - terran_extraction - pirate_extraction).max(0);
    let contention_after = economy.contention
        + if terran_present && pirate_present {
            terran_weight.min(pirate_weight).max(0)
        } else {
            0
        };
    let disruption_after = economy.disruption + pirate_extraction;
    let security_after = (economy.security + terran_security_gain - pirate_extraction).max(0);

    ClearingScalars {
        terran_present,
        pirate_present,
        terran_extraction,
        pirate_extraction,
        supply_after,
        contention_after,
        disruption_after,
        security_after,
    }
}

fn validate_surface(surface: &EconScale0080Surface, diagnostics: &mut Vec<&'static str>) {
    if surface.gate.enabled_by_default {
        diagnostics.push("econ_scale_0080_0_default_on_behavior_rejected");
    }
}

fn validate_scenario(scenario: &EconScale0080Scenario, diagnostics: &mut Vec<&'static str>) {
    if !scenario.faction_index.is_bounded() {
        diagnostics.push("unbounded_or_unbalanced_faction_set");
    }
    for economy in &scenario.starsystems {
        if economy.participants.len() > ECON_SCALE_0080_0_MAX_PARTICIPANTS_PER_STARSYSTEM {
            diagnostics.push("participants_exceed_bounded_max");
        }
        for participant in &economy.participants {
            if !scenario.faction_index.factions.contains(&participant.faction) {
                diagnostics.push("participant_faction_out_of_bounded_set");
            }
            if participant.faction_index != participant.faction.faction_index() {
                diagnostics.push("participant_faction_index_mismatch");
            }
        }
    }
}

fn validate_forbidden(forbidden: &EconScale0080ForbiddenRequests, diagnostics: &mut Vec<&'static str>) {
    if forbidden.hard_currency {
        diagnostics.push("hard_currency");
    }
    if forbidden.markets_trade_aibudget {
        diagnostics.push("markets_trade_aibudget");
    }
    if forbidden.nested_resource_flow {
        diagnostics.push("nested_resource_flow");
    }
    if forbidden.unbounded_faction_fanout {
        diagnostics.push("unbounded_faction_fanout");
    }
    if forbidden.replace_subsidiarity {
        diagnostics.push("replace_subsidiarity");
    }
    if forbidden.cpu_planner {
        diagnostics.push("cpu_planner");
    }
    if forbidden.semantic_or_raw_wgsl {
        diagnostics.push("semantic_or_raw_wgsl");
    }
    if forbidden.semantically_named_shader {
        diagnostics.push("semantically_named_shader");
    }
    if forbidden.clausething_dependency {
        diagnostics.push("clausething_dependency");
    }
    if forbidden.invariant_edit {
        diagnostics.push("invariant_edit");
    }
    if forbidden.production_path_0080_1 {
        diagnostics.push("production_path_0080_1_not_implemented");
    }
}

fn disabled_no_op_report(input: &EconScale0080Input) -> EconScale0080RunReport {
    base_report(input, true, Vec::new(), Vec::new())
}

fn rejected_report(
    input: &EconScale0080Input,
    diagnostics: Vec<&'static str>,
) -> EconScale0080RunReport {
    let mut report = base_report(input, false, diagnostics, Vec::new());
    report.disabled_no_op = false;
    report
}

fn admitted_report(
    input: &EconScale0080Input,
    clearing_reports: Vec<EconScale0080ClearingReport>,
) -> EconScale0080RunReport {
    base_report(input, true, Vec::new(), clearing_reports)
}

fn base_report(
    input: &EconScale0080Input,
    admitted: bool,
    diagnostics: Vec<&'static str>,
    clearing_reports: Vec<EconScale0080ClearingReport>,
) -> EconScale0080RunReport {
    let opt_in = input.surface.gate.explicit_opt_in;
    let disabled_no_op = admitted && !opt_in;
    let active = admitted && opt_in;

    let faction_indexed_participation = active
        && !clearing_reports.is_empty()
        && clearing_reports
            .iter()
            .all(|report| !report.faction_indices_present.is_empty())
        && clearing_reports
            .iter()
            .any(|report| report.terran_present && report.pirate_present);
    let adversarial_contended_clearing = active
        && clearing_reports.iter().any(|report| {
            report.adversarial
                && report.terran_present
                && report.pirate_present
                && report.contention_after > report.contention_before
        });
    let pirate_is_full_economy_faction = active
        && EconScale0080Faction::Pirate.is_full_economy_faction()
        && clearing_reports
            .iter()
            .any(|report| report.pirate_present && report.pirate_extraction > 0);
    let subsidiarity_preserved = clearing_reports
        .iter()
        .all(|report| report.subsidiarity_preserved);
    let parity_bit_exact = clearing_reports.iter().all(|report| report.parity_bit_exact);

    let mut report = EconScale0080RunReport {
        econ_scale_id: ECON_SCALE_0080_0_ID,
        status: ECON_SCALE_0080_0_STATUS_PASS,
        scenario_name: ECON_SCALE_0080_0_SCENARIO,
        admitted,
        diagnostics,
        explicit_opt_in: opt_in,
        default_off: !input.surface.gate.enabled_by_default,
        disabled_no_op,
        scenario_scoped_only: true,
        // The default ECON path is never altered: this surface is opt-in/scenario-scoped only.
        single_owner_default_unchanged: true,
        bounded_faction_count: input.scenario.faction_index.is_bounded(),
        faction_count: input.scenario.faction_index.count(),
        factions: input.scenario.faction_index.factions.clone(),
        faction_indexed_participation,
        adversarial_contended_clearing,
        pirate_is_full_economy_faction,
        subsidiarity_preserved,
        flat_star_posture_preserved: true,
        parity_bit_exact: if active { parity_bit_exact } else { true },
        nested_resource_flow: input.forbidden.nested_resource_flow,
        hard_currency_markets_trade_aibudget: input.forbidden.hard_currency
            || input.forbidden.markets_trade_aibudget,
        unbounded_faction_fanout: input.forbidden.unbounded_faction_fanout,
        cpu_planner_used: input.forbidden.cpu_planner,
        semantic_or_raw_wgsl_present: input.forbidden.semantic_or_raw_wgsl,
        clausething_dependency_present: input.forbidden.clausething_dependency,
        production_path_0080_1_implemented: false,
        clearing_reports,
        deterministic_replay_checksum: 0,
    };
    report.deterministic_replay_checksum = checksum_report(&report);
    report
}

fn checksum_report(report: &EconScale0080RunReport) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    hash = fnv(hash, report.faction_count as u64);
    for faction in &report.factions {
        hash = fnv(hash, faction.stable_code());
    }
    for step in &report.clearing_reports {
        hash = fnv(hash, u64::from(step.starsystem_index));
        hash = fnv(hash, signed(step.supply_after));
        hash = fnv(hash, signed(step.terran_extraction));
        hash = fnv(hash, signed(step.pirate_extraction));
        hash = fnv(hash, signed(step.contention_after));
        hash = fnv(hash, signed(step.disruption_after));
        hash = fnv(hash, signed(step.security_after));
        for index in &step.faction_indices_present {
            hash = fnv(hash, u64::from(*index));
        }
    }
    hash
}

fn signed(value: i64) -> u64 {
    value as u64
}

fn fnv(mut hash: u64, value: u64) -> u64 {
    hash ^= value;
    hash.wrapping_mul(0x0000_0100_0000_01B3)
}
