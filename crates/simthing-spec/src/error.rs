use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum SpecError {
    #[error("failed to parse RON: {0}")]
    RonParse(String),

    // ── Capability tree validation (PR 1) ────────────────────────────────────
    #[error("duplicate capability tree id `{0}`")]
    DuplicateTreeId(String),

    #[error("duplicate category `{0}` in tree `{1}`")]
    DuplicateCategory(String, String),

    #[error("duplicate entry `{0}` in tree `{1}`")]
    DuplicateEntry(String, String),

    #[error("negative research_cost on entry `{0}`")]
    NegativeResearchCost(String),

    #[error("Threshold activation requires research_cost > 0 on entry `{0}`")]
    ThresholdRequiresPositiveCost(String),

    #[error("validation failed")]
    ValidationFailed,

    // ── Event effect compilation (CT-1b) ─────────────────────────────────────
    #[error(
        "effect references overlay `{overlay_ref}` by authored id; install-time resolution did not run"
    )]
    UnresolvedOverlayRef { overlay_ref: String },

    // ── Property compilation (PR 2) ──────────────────────────────────────────
    #[error("duplicate property registration `{namespace}::{name}`")]
    DuplicateProperty { namespace: String, name: String },

    #[error("sub-field `{sub_field}` on property `{property}` declares governed_by `{governed_by}` which is not present in the same layout")]
    InvalidGovernedByRole {
        property: String,
        sub_field: String,
        governed_by: String,
    },

    // ── Overlay compilation (PR 2) ───────────────────────────────────────────
    #[error("overlay `{overlay}` targets unknown property `{namespace}::{name}`")]
    UnknownProperty {
        overlay: String,
        namespace: String,
        name: String,
    },

    #[error("overlay `{overlay}` references sub-field role `{role}` not present in property `{property}`'s layout")]
    InvalidSubFieldRole {
        overlay: String,
        property: String,
        role: String,
    },

    #[error("overlay `{overlay}` has malformed targets_property `{targets_property}` (expected `namespace::name`)")]
    InvalidPropertyReference {
        overlay: String,
        targets_property: String,
    },

    // ── Capability tree builder (PR 3) ───────────────────────────────────────
    #[error(
        "entry `{0}` was authored with ActivationMode::OnPrereqMet — that state is runtime-only"
    )]
    OnPrereqMetAuthoredDefault(String),

    #[error("entry `{entry_id}` in tree `{in_tree}` references unknown prereq category `{category}` (expected `namespace::name`)")]
    UnknownPrereqCategory {
        in_tree: String,
        entry_id: String,
        category: String,
    },

    #[error("entry `{entry_id}` in tree `{in_tree}` references unknown prereq entry `{prereq_entry_id}` in category `{category}`")]
    UnknownPrereqEntry {
        in_tree: String,
        entry_id: String,
        category: String,
        prereq_entry_id: String,
    },

    #[error("entry `{0}` declares itself as a prereq")]
    SelfReferentialPrereq(String),

    #[error("category `{category}` in tree `{in_tree}` sets max_active = {count}; v0 supports only Unlimited (None) or Limited(1)")]
    UnsupportedMaxActive {
        in_tree: String,
        category: String,
        count: usize,
    },

    #[error(
        "entry `{entry_id}` effect #{effect_index} targets property `{targets_property}`: {reason}"
    )]
    InvalidEffectTarget {
        entry_id: String,
        effect_index: usize,
        targets_property: String,
        reason: String,
    },

    // ── Scripted trigger/effect/event compiler (PR 8) ───────────────────────
    #[error("trigger references unknown property `{namespace}::{name}`")]
    InvalidTriggerProperty { namespace: String, name: String },

    #[error("trigger references role `{role}` not present in property `{property}`")]
    InvalidTriggerRole { property: String, role: String },

    // ── Resource Flow admission (E-10) ───────────────────────────────────────
    #[error("duplicate arena name `{name}`")]
    DuplicateArenaName { name: String },

    #[error("arena `{arena}` requires explicit admission; implicit participation is forbidden")]
    ImplicitParticipation { arena: String },

    #[error("property `{property}` sub-field `{sub_field}` references unknown arena `{arena}`")]
    UnknownArenaRoleReference {
        arena: String,
        property: String,
        sub_field: String,
    },

    #[error("unknown arena `{arena}` in {context}")]
    UnknownArenaReference { arena: String, context: String },

    #[error("property `{property}` is not registered for Resource Flow admission")]
    UnknownResourceFlowProperty { property: String },

    #[error("arena `{arena}` declares wildcard admission without a declared upper bound")]
    UnboundedWildcardAdmission { arena: String },

    #[error("arena `{arena}` wildcard expansion {computed} exceeds declared cap {declared}")]
    WildcardExpansionExceedsCap {
        arena: String,
        declared: u32,
        computed: u32,
    },

    #[error("arena `{arena}` flow property `{property}` conflicts with arena `{other_arena}`")]
    ConflictingArenaFlowProperty {
        arena: String,
        other_arena: String,
        property: String,
    },

    #[error("property `{property}` sub-field `{sub_field}` Balance num_count_source references unresolved property id {referenced_property_id}")]
    UnresolvedBalanceNumCountSource {
        property: String,
        sub_field: String,
        referenced_property_id: u32,
    },

    #[error("arena `{arena}` duplicate {kind} binding: `{first}` vs `{second}`")]
    DuplicateArenaRoleBinding {
        arena: String,
        kind: String,
        first: String,
        second: String,
    },

    #[error("property `{property}` sub-field `{sub_field}` possession does not admit to arena `{arena}`")]
    PropertyPossessionNotArenaAdmission {
        arena: String,
        property: String,
        sub_field: String,
    },

    #[error("arena `{arena}` exceeds max_participants ({declared} declared, {computed} computed)")]
    MaxParticipantsExceeded {
        arena: String,
        declared: u32,
        computed: u32,
    },

    #[error(
        "arena `{arena}` exceeds max_coupling_fanout ({declared} declared, {computed} computed)"
    )]
    MaxCouplingFanoutExceeded {
        arena: String,
        declared: u32,
        computed: u32,
    },

    #[error(
        "arena `{arena}` exceeds max_orderband_depth ({declared} declared, {computed} computed)"
    )]
    MaxOrderBandDepthExceeded {
        arena: String,
        declared: u32,
        computed: u32,
    },

    #[error("coupling graph contains an all-algebraic cycle")]
    AllAlgebraicCouplingCycle,

    #[error("arena `{arena}` hidden fanout {computed} exceeds declared budget {declared}")]
    HiddenFanoutExceeded {
        arena: String,
        declared: u32,
        computed: u32,
    },

    #[error("duplicate Resource Flow base obligation id `{id}`")]
    DuplicateBaseFlowObligation { id: String },

    #[error("Resource Flow base obligation `{id}` rate must be finite and >= 0")]
    InvalidBaseFlowObligationRate { id: String },

    // ── Resource Flow preflight (E-10R, driver-mapped) ─────────────────────────
    #[error("arena `{arena}` explicit participant subtree_root_id {subtree_root_id} is unknown in the live session")]
    UnknownExplicitParticipantSimThing { arena: String, subtree_root_id: u32 },

    #[error("arena `{arena}` explicit participant subtree_root_id {subtree_root_id} slot mismatch (declared {declared_slot}, actual {actual_slot})")]
    ExplicitParticipantSlotMismatch {
        arena: String,
        subtree_root_id: u32,
        declared_slot: u32,
        actual_slot: u32,
    },

    #[error("arena `{arena}` explicit participant subtree_root_id {subtree_root_id} slot {slot} is tombstoned")]
    ExplicitParticipantTombstoned {
        arena: String,
        subtree_root_id: u32,
        slot: u32,
    },

    #[error("arena `{arena}` reserved_gap_per_intermediate {reserved} is smaller than expected_max_children_per_intermediate {expected}")]
    ReservedGapTooSmall {
        arena: String,
        reserved: u32,
        expected: u32,
    },

    #[error("arena `{arena}` duplicate enrollment for hosted SimThing subtree_root_id {subtree_root_id}")]
    DuplicateEnrollmentHostedSimThing { arena: String, subtree_root_id: u32 },

    #[error("arena `{arena}` explicit participant parent_subtree_root_id {parent_subtree_root_id} is unknown in the same arena explicit list")]
    UnknownExplicitParticipantParent {
        arena: String,
        parent_subtree_root_id: u64,
    },

    #[error("arena `{arena}` explicit participant parent graph contains a cycle involving subtree_root_id {subtree_root_id}")]
    ExplicitParticipantParentCycle { arena: String, subtree_root_id: u32 },

    #[error("arena `{arena}` explicit participant depth-first allocation failed child contiguity")]
    ExplicitParticipantAllocationNonContiguous { arena: String },

    // ── Resource economy compile (Phase T-2) ───────────────────────────────────
    #[error("resource economy duplicate authoring id `{id}`")]
    DuplicateResourceEconomyId { id: String },

    #[error("resource economy transfer `{transfer}` references unknown source property `{namespace}::{name}`")]
    UnknownResourceEconomySourceProperty {
        transfer: String,
        namespace: String,
        name: String,
    },

    #[error("resource economy transfer `{transfer}` references unknown target property `{namespace}::{name}`")]
    UnknownResourceEconomyTargetProperty {
        transfer: String,
        namespace: String,
        name: String,
    },

    #[error("resource economy `{context}` references unknown property `{namespace}::{name}`")]
    UnknownResourceEconomyProperty {
        context: String,
        namespace: String,
        name: String,
    },

    #[error("resource economy `{context}` references role `{role}` not present in property `{property}`")]
    InvalidResourceEconomyRole {
        context: String,
        property: String,
        role: String,
    },

    #[error("resource economy transfer `{transfer}` amount must be finite and >= 0")]
    InvalidTransferAmount { transfer: String },

    #[error("resource economy recipe `{recipe}` input unit_cost must be finite and > 0")]
    InvalidRecipeUnitCost { recipe: String },

    #[error("resource economy recipe `{recipe}` requires at least one input")]
    EmptyRecipeInputs { recipe: String },

    #[error("resource economy recipe `{recipe}` throttle_hint_max_per_tick must be > 0")]
    InvalidRecipeThrottleHint { recipe: String },

    #[error(
        "resource economy emission `{emission}` references unknown EML formula key `{formula_key}`"
    )]
    UnknownEmissionFormulaKey {
        emission: String,
        formula_key: String,
    },

    #[error("resource economy emission `{emission}` EML formula `{formula_key}` is not ExactDeterministic for Emission")]
    EmissionEmlNotExactDeterministic {
        emission: String,
        formula_key: String,
    },

    #[error("resource economy emission `{emission}` formula shape is not supported")]
    UnsupportedEmissionFormula { emission: String },

    #[error("resource economy emission `{emission}` constant must be finite")]
    InvalidEmissionConstant { emission: String },

    #[error("resource economy threshold emit `{emit}` threshold must be finite")]
    InvalidThresholdEmitThreshold { emit: String },

    #[error(
        "resource economy consumed input (property {property_id}, col {col}) contended on order band {order_band} between `{first}` and `{second}`"
    )]
    ResourceEconomyConsumedInputContention {
        property_id: u32,
        col: u32,
        order_band: u32,
        first: String,
        second: String,
    },

    #[error("resource economy admission: {reason}")]
    ResourceEconomyAdmission { reason: String },

    // ── Region field mapping admission (Phase M-3) ───────────────────────────
    #[error("region field `{field}`: {reason}")]
    RegionFieldAdmission { field: String, reason: String },

    // ── W impedance compose admission (BH-2B) ────────────────────────────────
    #[error("W impedance compose admission: {reason}")]
    WImpedanceComposeAdmission { reason: String },

    // ── EML Gadget Library (Phase M EML-GADGET-1) ────────────────────────────
    #[error("EML gadget `{gadget}`: {reason}")]
    EmlGadgetAdmission { gadget: String, reason: String },

    // ── JIT kernel descriptor admission (Phase M-JIT-DESC-1) ─────────────────
    #[error("jit kernel descriptor `{kernel}`: {reason}")]
    JitKernelDescriptorAdmission { kernel: String, reason: String },
}
