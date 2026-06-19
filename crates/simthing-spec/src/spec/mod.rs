pub mod capability;
pub mod domain_pack;
pub mod effect;
pub mod eml_gadget;
pub mod event;
pub mod first_slice_scenario;
pub mod game_mode;
pub mod install_target;
pub mod overlay;
pub mod property;
pub mod region_field;
pub mod resource_economy;
pub mod resource_flow;
pub mod scenario;
pub mod scenario_ingestion;
pub mod script;
pub mod stress_compose;
pub mod trigger;
pub mod w_impedance_compose;

pub use capability::{
    ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilityPrereqSpec,
    CapabilitySpec, CapabilityTreeSpec, MaxActivePolicy,
};
pub use domain_pack::DomainPackSpec;
pub use effect::EffectSpec;
pub use eml_gadget::{EmlGadgetInstanceSpec, EmlGadgetStackSpec};
pub use event::{CooldownSpec, EventKey, EventPriority, EventSpec};
pub use first_slice_scenario::FirstSliceScenarioSpec;
pub use game_mode::GameModeSpec;
pub use install_target::InstallTargetSpec;
pub use overlay::OverlaySpec;
pub use property::PropertySpec;
pub use region_field::{
    ArenaPressureBindingSpec, CommitmentEffectLifecycleSpec, CommitmentEffectSpec,
    CompiledRegionFieldSummaryPolicy, FirstSliceCommitmentDirectionSpec, FirstSliceCommitmentSpec,
    GradientAxisSpec, MappingExecutionProfile, PressurePlacementSpec, PressureSourceSpec,
    RegionFieldCadenceSpec, RegionFieldFormulaBindingSpec, RegionFieldGridProfile,
    RegionFieldOperatorSpec, RegionFieldReductionSpec, RegionFieldSourcePolicySpec,
    RegionFieldSpec, RegionFieldSummaryPolicySpec,
};
pub use resource_economy::{
    EmissionFormulaSpec, EmitBufferSpec, EmitOnThresholdSpec, RecipeInputSpec,
    ResourceEconomyOptInMode, ResourceEconomySpec, ResourceEmissionSpec, ResourceRecipeSpec,
    ResourceTransferSpec,
};
pub use resource_flow::{
    ArenaSpec, BaseFlowDirectionSpec, BaseFlowObligationSpec, CouplingDelaySpec, CouplingSpec,
    EnrollmentSelectorSpec, ExplicitParticipantSpec, FissionPolicySpec, GatedRateOpSpec,
    GatedRateSpec, GatedRateTriggerSpec, RateFormulaOp, RateFormulaOpSpec, RateFormulaOperandSpec,
    RateFormulaSpec, ResourceFlowExecutionProfile, ResourceFlowOptInMode, ResourceFlowSpec,
    WildcardAdmissionSpec,
};
pub use scenario::{
    apply_galaxy_map_metadata, apply_gridcell_property_edit, apply_gridcell_role_metadata,
    apply_owner_entity_metadata, apply_scenario_metadata_to_root, canonical_scenario_link_key,
    canonical_scenario_link_pair, deserialize_scenario_authority, galaxy_map_display_name,
    galaxy_map_id, galaxy_map_role, game_session_child, game_session_galaxy_map,
    game_session_galaxy_maps, game_session_owners, gridcell_generated_system_id, gridcell_role,
    gridcell_structural_col, gridcell_structural_row, is_galaxy_map_entity, is_owner_entity_kind,
    make_galaxy_map, make_owner_entity, owner_archetype, owner_color_index, owner_display_name,
    owner_entity_id, owner_silo_marker, property_u32, reserve_simthing_ids_from_scenario,
    resolve_map_container, resolve_map_container_mut, scenario_metadata_seed,
    scenario_metadata_seed_value, scenario_metadata_string, scenario_metadata_string_value,
    scenario_metadata_u32, scenario_metadata_u32_value, serialize_scenario_authority,
    set_galaxy_map_display_name, set_owner_display_name, spatial_authority_root,
    structural_property_value_u32, sync_root_metadata_from_sidecar,
    sync_sidecar_from_root_metadata, validate_legacy_world_root_compatibility,
    validate_scenario_game_session_child, validate_scenario_links,
    validate_scenario_root_authority, validate_session_galaxy_map, validate_session_owner_entities,
    validate_stead_mapping_consistency, ScenarioEditError, ScenarioLinkError, ScenarioRootError,
    ScenarioRootValidationMode, ScenarioSerdeError, SimThingScenarioGrid, SimThingScenarioLink,
    SimThingScenarioProvenance, SimThingScenarioSpec, SimThingStructuralGridFrame,
    SimThingStructuralGridPlacement, SteadMappingError, GALAXY_GRIDCELL_ROLE_INERT,
    GALAXY_GRIDCELL_ROLE_PROPERTY_ID, GALAXY_GRIDCELL_ROLE_STAR_SYSTEM,
    GALAXY_MAP_DISPLAY_NAME_PROPERTY_ID, GALAXY_MAP_ID_PROPERTY_ID, GALAXY_MAP_ROLE_CANONICAL,
    GALAXY_MAP_ROLE_PROPERTY_ID, OWNER_ARCHETYPE_PROPERTY_ID, OWNER_COLOR_INDEX_PROPERTY_ID,
    OWNER_DISPLAY_NAME_PROPERTY_ID, OWNER_ID_PROPERTY_ID, OWNER_SILO_MARKER_PROPERTY_ID,
    SCENARIO_GENERATED_SYSTEM_ID_PROPERTY_ID, SCENARIO_GENERATOR_SEED_PROPERTY_ID,
    SCENARIO_GENERATOR_SHAPE_PROPERTY_ID, SCENARIO_ID_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_X_PROPERTY_ID, SCENARIO_RENDER_WORLD_Y_PROPERTY_ID,
    SCENARIO_RENDER_WORLD_Z_PROPERTY_ID, SCENARIO_SCHEMA_VERSION,
    SCENARIO_SCHEMA_VERSION_PROPERTY_ID, SCENARIO_SOURCE_LABEL_PROPERTY_ID,
    SCENARIO_STRUCTURAL_COL_PROPERTY_ID, SCENARIO_STRUCTURAL_INTEGER_MAX,
    SCENARIO_STRUCTURAL_ROW_PROPERTY_ID, SIMTHING_SCENARIO_AUTHORITY_LABEL,
};
pub use scenario_ingestion::{
    ingest_scenario, ingest_scenario_from_str, ingestion_error_from_root,
    ingestion_error_from_serde, GalaxyMapAdmissionReport, OwnerAdmissionReport,
    ScenarioCompileReadinessReport, ScenarioDeferral, ScenarioDeferralKind, ScenarioFingerprint,
    ScenarioIngestionClassification, ScenarioIngestionError, ScenarioIngestionProfile,
    ScenarioIngestionResult, ScenarioTreeAdmissionReport, ScenarioValidationReport,
    StructuralAdmissionReport,
};
pub use script::{
    PropertyKey, ScopeRef, ScriptEvalContext, ScriptEvalError, ScriptExpr, ScriptPredicate,
};
pub use stress_compose::{
    StressComposeProfileSpec, StressComposeSpec, StressOperatorSpec,
    STRESS_COMPOSE_MAX_INPUT_FIELDS, STRESS_COMPOSE_MAX_PROFILES, STRESS_OP_MISMATCH,
    STRESS_OP_OVERLAP, STRESS_OP_VELOCITY, STRESS_OP_WEIGHTED,
};
pub use trigger::{TriggerDirection, TriggerSpec};
pub use w_impedance_compose::{
    WImpedanceComposeProfileSpec, WImpedanceComposeSpec, CT_4B_LOCAL_AUTOMATA_W_FEEDSTOCK,
    W_IMPEDANCE_COMPOSE_MAX_PROFILES,
};
