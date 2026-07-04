//! ClauseThing — ClauseScript front-end for SimThing.
//!
//! CT-0a: crate skeleton and vendored jomini text-path parser.
//! CT-0b: lossless raw model, JSON goldens, and safe synthetic round-trip.
//! CT-0c: expansion passes (@vars, inline_script, $PARAM$, [[PARAM]] conditionals)
//! with `@[ ]` inline math preserved symbolically and `value:` left symbolic.
//! CT-0d: symbolic scope-chain extraction and lab-only frequency evidence.
//! CT-1a: literal entity hydration into existing `simthing-spec` authoring structs.
//! CT-2a: literal `produces`/`upkeep` hydration into Resource Flow authoring.
//! CT-2c: category economy hydration into Resource Flow and ResourceEconomy authoring.
//! TP-SHIPSIZE-DECODER-0: shipsize / `ship_*` modifier decoder family.
//! TP-FLEETS-SHIPS-0: fleet/ship scenario-envelope authoring.
//! CT-PR2/3: scenario-container hydration into generic spec surfaces plus a SimThing tree and
//! bounded grid-link metadata.
//! MapGen PR2: neutral-AST parse-only adapter (M1) — no semantic mapping.
//! MapGen PR3: gridcell lattice hierarchy generator — scenario-container output only.
//! MapGen PR4: bounded Resource Flow enrollment/feedstock from PR3 hierarchy — front-end only.
//! MapGen PR5: bounded hyperlane-to-link and lane-coupling authoring from PR4 enrollment — front-end only.
//! MapGen PR6: Movement-Front L1/L2/L3 authoring feedstock from PR5 enrollment — front-end only.
//! MapGen PR7: PALMA W/D reach feedstock from PR6 Movement-Front enrollment — front-end only.
//! MapGen PR9: constitutional guard hardening (Candidate F, P1/horizon, one-system-per-cell) — tests/docs only.
//! No runtime wiring, default-off.

pub mod error;
pub mod expand;
pub mod jomini;
pub mod raw;
pub mod scope;

mod emit;
mod hydrate;
mod hydrate_category_economy;
mod hydrate_shipsize_decoder;
mod hydrate_field_operator;
mod hydrate_palma_feedstock;
mod hydrate_resource_flow;
mod hydrate_scenario;
mod hydrate_scenario_commitment;
mod json;
mod literal_install;
mod mapgen_lattice;
mod mapgen_links;
mod mapgen_movement_front;
mod mapgen_neutral_ast;
mod mapgen_palma;
mod mapgen_resource_flow;
mod parse;
mod scope_json;
mod scope_lab;
mod stellaris_names;

pub use emit::emit_text;
pub use error::{EmitError, ExpandError, HydrateError, ParseError};
pub use expand::{
    ExpansionInput, ExpansionOptions, expand_document, is_inline_math, is_value_reference,
};
pub use hydrate::{HydratedEntityPack, hydrate_entity_pack};
pub use hydrate_category_economy::{
    CategoryFlowContribution, DecodedEconomicKey, EconomicAxis, EconomicOp,
    HydratedCategoryEconomyPack, decode_economic_modifier_key, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode,
};
pub use hydrate_shipsize_decoder::{
    DecodedShipModifierKey, HydratedShipsizeDecoderPack, ShipModifierFamily, ShipModifierOp,
    SHIP_MODIFIER_ATTRIBUTES, MAX_SHIP_EML_NODES, compile_value_formula_eml,
    decode_ship_modifier_key, decode_ship_modifier_key_spanned, hydrate_shipsize_decoder_pack,
};
pub use hydrate_field_operator::{
    BH3_MAX_FIELD_IMPEDANCE_PROFILES, BH3_MAX_FIELD_STRESS_PROFILES,
    BH3_SATURATING_FLUX_CHI_CFL_MAX, HydratedFieldOperatorPack, hydrate_field_operator_pack,
    hydrate_field_operator_property,
};
pub use hydrate_palma_feedstock::{
    HydratedScenarioPalmaFeedstock, PR5_MAX_SCENARIO_PALMA_FEEDSTOCK,
    build_palma_feedstock_from_region_field,
};
pub use hydrate_resource_flow::{
    HydratedResourceFlowPack, hydrate_resource_flow_pack, net_intrinsic_flow,
};
pub use hydrate_scenario::{
    HydratedEmbeddedStaticGalaxyScenario, HydratedFleetPlacement, HydratedFleetShipPayload,
    HydratedOwnedSystem, HydratedOwnershipVolume, HydratedPlanetSurfacePayload,
    HydratedScenarioGridMetadata, HydratedScenarioGridPlacement, HydratedScenarioLink,
    HydratedScenarioNode, HydratedScenarioOwner, HydratedScenarioPack, PR3_MAX_LINK_FANOUT,
    PR4_MAX_SCENARIO_FIELD_OPERATORS, hydrate_scenario,
};
pub use hydrate_scenario_commitment::{HydratedScenarioCommitment, PR6_MAX_SCENARIO_COMMITMENT};
pub use jomini::{TextTape, TextToken};
pub use json::to_canonical_json;
pub use literal_install::{
    LiteralInstallSnapshot, OverlaySpecFingerprint, admit_and_apply_domain_pack,
    admit_and_apply_pack,
};
pub use mapgen_lattice::{
    MAPGEN_CANONICAL_LATTICE_EDGE, MAPGEN_DEFAULT_FIXTURE_LATTICE_EDGE, MapGenLatticeError,
    MapGenLatticeHierarchy, MapGenLatticeOptions, MapgenStructuralGridBudget,
    STRUCTURAL_BYTES_PER_LINK, STRUCTURAL_BYTES_PER_OCCUPIED_CELL, StructuralGridFrame,
    StructuralGridStats, admit_structural_grid, assert_allowed_simthing_kinds,
    collect_gridcell_location_ids, generate_mapgen_lattice_hierarchy,
    validate_fixture_lattice_edge, validate_one_system_per_gridcell,
};
pub use mapgen_links::{
    MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLING_FANOUT, MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLINGS,
    MAPGEN_PR5_DEFAULT_MAX_LINKS, MapGenLaneCoupling, MapGenLinksEnrollment, MapGenLinksError,
    MapGenLinksExpansionReport, MapGenLinksOptions, extract_hyperlane_declarations,
    generate_default_mapgen_links_enrollment, generate_mapgen_links, lower_hyperlane_topology,
};
pub use mapgen_movement_front::{
    MAPGEN_MF_CHOKE_OUTPUT_COL, MAPGEN_MF_COMMITMENT_ID, MAPGEN_MF_DEFAULT_HORIZON,
    MAPGEN_MF_FIELD_OPERATOR_ID, MAPGEN_MF_L2_REDUCTION_SCOPE, MAPGEN_MF_MAX_HORIZON,
    MAPGEN_MF_N_DIMS, MAPGEN_MF_SOURCE_COL, MapGenMovementFrontAuthoring,
    MapGenMovementFrontAuthoringReport, MapGenMovementFrontError, MapGenMovementFrontErrorKind,
    MapGenMovementFrontOptions, assert_no_palma_feedstock,
    generate_default_mapgen_movement_front_authoring, generate_mapgen_movement_front_authoring,
    validate_l1_operator_locality, validate_options,
};
pub use mapgen_neutral_ast::{MapGenNeutralDocument, parse_mapgen_neutral_document};
pub use mapgen_palma::{
    MAPGEN_PALMA_D_OUTPUT_COL, MAPGEN_PALMA_FEEDSTOCK_ID, MAPGEN_PALMA_W_OUTPUT_COL,
    MapGenPalmaAuthoringReport, MapGenPalmaError, MapGenPalmaFeedstockAuthoring,
    MapGenPalmaOptions, build_w_impedance_compose_from_palma,
    generate_default_mapgen_palma_feedstock, generate_mapgen_palma_feedstock,
    validate_palma_options,
};
pub use mapgen_resource_flow::{
    MAPGEN_RF_DEFAULT_DEPOSIT_MAX_PARTICIPANTS, MAPGEN_RF_DEFAULT_MAX_COUPLING_FANOUT,
    MAPGEN_RF_DEFAULT_MAX_ORDERBAND_DEPTH, MAPGEN_RF_DEFAULT_SUPPRESSION_MAX_PARTICIPANTS,
    MAPGEN_RF_DEPOSIT_ARENA, MAPGEN_RF_PROPERTY_NAMESPACE, MAPGEN_RF_SUPPRESSION_ARENA,
    MapGenResourceFlowArenaExpansion, MapGenResourceFlowEnrollment, MapGenResourceFlowError,
    MapGenResourceFlowExpansionReport, MapGenResourceFlowOptions, SpatialArenaBindingReport,
    SpatialBindingMode, generate_default_mapgen_resource_flow_enrollment,
    generate_mapgen_resource_flow_enrollment, validate_arena_caps, validate_explicit_enrollment,
    validate_resource_flow_enrollment, validate_spatial_binding,
};
pub use parse::parse_raw_document;
pub use raw::RawDocument;
pub use scope::{
    ScopeAtom, ScopeAtomKind, ScopeChain, ScopeDiagnostic, ScopeDiagnosticKind,
    ScopeExtractionReport, ScopeReference, ScopeReferenceRole, ScopeTable, extract_scopes,
    extract_scopes_validated, parse_scope_chain, synthetic_scope_table,
};
pub use scope_json::scope_report_to_json;
pub use scope_lab::{LabFrequencyReport, scan_lab_scopes};
pub use stellaris_names::{
    StellarisStarNameCatalog, StellarisStarNameError, parse_stellaris_star_name_catalog,
};
