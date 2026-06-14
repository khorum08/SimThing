//! MapGen PR5 — bounded hyperlane-to-link and lane-coupling authoring (M6).
//!
//! Lowers `add_hyperlane` declarations from the neutral-AST fixture into bounded scenario link metadata
//! (N4-adjacent lattice links) plus bounded lane-coupling authoring metadata (long-range gather edges).
//! Authored Stellaris positions remain inert render metadata. No pathfinding, movement, routes,
//! predecessors, border/frontline, Movement-Front, PALMA, FIELD_POLICY, or runtime surfaces.

use std::collections::{BTreeMap, BTreeSet};

use simthing_spec::PropertySpec;

use crate::hydrate_scenario::{
    HydratedScenarioGridPlacement, HydratedScenarioLink, HydratedScenarioPack, PR3_MAX_LINK_FANOUT,
};
use crate::mapgen_lattice::{assert_allowed_simthing_kinds, collect_gridcell_location_ids};
use crate::mapgen_neutral_ast::MapGenNeutralDocument;
use crate::mapgen_resource_flow::MapGenResourceFlowEnrollment;
use crate::raw::{RawBlock, RawProperty, RawValue};

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

/// Default maximum scenario links for the tiny pentad slice.
pub const MAPGEN_PR5_DEFAULT_MAX_LINKS: usize = 8;

/// Default maximum lane couplings for the tiny pentad slice.
pub const MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLINGS: usize = 8;

/// Default per-node lane-coupling fanout cap.
pub const MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLING_FANOUT: usize = 4;

/// Bounded hyperlane/link lowering options for MapGen PR5.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapGenLinksOptions {
    pub max_links: usize,
    pub max_lane_couplings: usize,
    pub max_lane_coupling_fanout: usize,
    pub max_per_node_fanout: usize,
}

impl Default for MapGenLinksOptions {
    fn default() -> Self {
        Self {
            max_links: MAPGEN_PR5_DEFAULT_MAX_LINKS,
            max_lane_couplings: MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLINGS,
            max_lane_coupling_fanout: MAPGEN_PR5_DEFAULT_MAX_LANE_COUPLING_FANOUT,
            max_per_node_fanout: PR3_MAX_LINK_FANOUT,
        }
    }
}

/// Authored long-range lane coupling between existing gridcell ids.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenLaneCoupling {
    pub from: String,
    pub to: String,
}

/// MapGen PR5 link/coupling expansion report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapGenLinksExpansionReport {
    pub link_count: u32,
    pub max_links: u32,
    pub per_node_fanout: BTreeMap<String, u32>,
    pub max_per_node_fanout: u32,
    pub lane_coupling_count: u32,
    pub max_lane_coupling_count: u32,
    pub max_lane_coupling_fanout: u32,
    pub unknown_endpoint_rejections: u32,
    pub self_link_rejections: u32,
    pub duplicate_link_rejections: u32,
    pub unsafe_expansion_flags: Vec<String>,
}

/// Scenario-container pack plus bounded link/lane-coupling enrollment.
#[derive(Debug, Clone)]
pub struct MapGenLinksEnrollment {
    pub pack: HydratedScenarioPack,
    pub lane_couplings: Vec<MapGenLaneCoupling>,
    pub expansion_report: MapGenLinksExpansionReport,
}

/// MapGen PR5 link lowering failure.
#[derive(Debug)]
pub struct MapGenLinksError {
    pub message: String,
}

impl MapGenLinksError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for MapGenLinksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MapGen links error: {}", self.message)
    }
}

impl std::error::Error for MapGenLinksError {}

/// Generate bounded link and lane-coupling metadata from PR4 enrollment plus neutral hyperlanes.
pub fn generate_mapgen_links(
    enrollment: &MapGenResourceFlowEnrollment,
    document: &MapGenNeutralDocument,
    options: MapGenLinksOptions,
) -> Result<MapGenLinksEnrollment, MapGenLinksError> {
    validate_options(&options)?;
    let hyperlanes = extract_hyperlane_declarations(document)?;
    lower_hyperlane_topology(&enrollment.pack, &hyperlanes, options)
}

/// Convenience: parse raw fixture → PR3 → PR4 → PR5 with defaults.
pub fn generate_default_mapgen_links_enrollment(
    document: &MapGenNeutralDocument,
) -> Result<MapGenLinksEnrollment, MapGenLinksError> {
    use crate::mapgen_resource_flow::generate_default_mapgen_resource_flow_enrollment;
    let enrollment = generate_default_mapgen_resource_flow_enrollment(document)
        .map_err(|err| MapGenLinksError::new(err.message))?;
    generate_mapgen_links(&enrollment, document, MapGenLinksOptions::default())
}

/// Lower authored hyperlane endpoint pairs into bounded link and lane-coupling metadata.
pub fn lower_hyperlane_topology(
    pack: &HydratedScenarioPack,
    hyperlanes: &[(String, String)],
    options: MapGenLinksOptions,
) -> Result<MapGenLinksEnrollment, MapGenLinksError> {
    validate_options(&options)?;
    let gridcell_ids: BTreeSet<String> = collect_gridcell_location_ids(&pack.root_node)
        .into_iter()
        .collect();
    let placement_by_id = placement_map(&pack.grid_metadata.placements);

    let mut unknown_endpoint_rejections = 0u32;
    let mut self_link_rejections = 0u32;
    let mut duplicate_link_rejections = 0u32;

    let mut seen_pairs: BTreeSet<(String, String)> = BTreeSet::new();
    let mut accepted_pairs: Vec<(String, String)> = Vec::new();

    for (raw_from, raw_to) in hyperlanes {
        if raw_from == raw_to {
            self_link_rejections += 1;
            continue;
        }
        if !gridcell_ids.contains(raw_from) || !gridcell_ids.contains(raw_to) {
            unknown_endpoint_rejections += 1;
            continue;
        }
        let canonical = canonical_pair(raw_from, raw_to);
        if !seen_pairs.insert(canonical.clone()) {
            duplicate_link_rejections += 1;
            continue;
        }
        accepted_pairs.push(canonical);
    }

    if unknown_endpoint_rejections > 0 {
        return Err(MapGenLinksError::new(format!(
            "hyperlane references unknown gridcell endpoint ({unknown_endpoint_rejections} rejection(s))"
        )));
    }
    if self_link_rejections > 0 {
        return Err(MapGenLinksError::new(format!(
            "hyperlane self-link rejected ({self_link_rejections} rejection(s))"
        )));
    }

    let mut links = Vec::new();
    let mut lane_couplings = Vec::new();

    for (from, to) in accepted_pairs {
        let left = placement_by_id
            .get(&from)
            .ok_or_else(|| MapGenLinksError::new(format!("missing placement for `{from}`")))?;
        let right = placement_by_id
            .get(&to)
            .ok_or_else(|| MapGenLinksError::new(format!("missing placement for `{to}`")))?;
        if is_n4_neighbor(*left, *right) {
            links.push(HydratedScenarioLink { from, to });
        } else {
            lane_couplings.push(MapGenLaneCoupling { from, to });
        }
    }

    if links.len() > options.max_links {
        return Err(MapGenLinksError::new(format!(
            "link count {} exceeds PR5 max_links {}",
            links.len(),
            options.max_links
        )));
    }
    if lane_couplings.len() > options.max_lane_couplings {
        return Err(MapGenLinksError::new(format!(
            "lane coupling count {} exceeds PR5 max_lane_couplings {}",
            lane_couplings.len(),
            options.max_lane_couplings
        )));
    }

    let mut per_node_fanout: BTreeMap<String, u32> = BTreeMap::new();
    let mut lane_coupling_fanout: BTreeMap<String, u32> = BTreeMap::new();
    for link in &links {
        *per_node_fanout.entry(link.from.clone()).or_insert(0) += 1;
        *per_node_fanout.entry(link.to.clone()).or_insert(0) += 1;
    }
    for coupling in &lane_couplings {
        *per_node_fanout.entry(coupling.from.clone()).or_insert(0) += 1;
        *per_node_fanout.entry(coupling.to.clone()).or_insert(0) += 1;
        *lane_coupling_fanout
            .entry(coupling.from.clone())
            .or_insert(0) += 1;
        *lane_coupling_fanout.entry(coupling.to.clone()).or_insert(0) += 1;
    }

    for (node_id, fanout) in &per_node_fanout {
        if *fanout as usize > options.max_per_node_fanout {
            return Err(MapGenLinksError::new(format!(
                "topology fanout for `{node_id}` is {fanout}, above PR5 cap {}",
                options.max_per_node_fanout
            )));
        }
    }
    for (node_id, fanout) in &lane_coupling_fanout {
        if *fanout as usize > options.max_lane_coupling_fanout {
            return Err(MapGenLinksError::new(format!(
                "lane coupling fanout for `{node_id}` is {fanout}, above PR5 cap {}",
                options.max_lane_coupling_fanout
            )));
        }
    }

    let mut unsafe_expansion_flags = Vec::new();
    if links.len() >= options.max_links {
        unsafe_expansion_flags.push("link_count_at_cap".into());
    }
    if lane_couplings.len() >= options.max_lane_couplings {
        unsafe_expansion_flags.push("lane_coupling_count_at_cap".into());
    }
    let observed_max_per_node_fanout = per_node_fanout.values().copied().max().unwrap_or(0);
    if observed_max_per_node_fanout as usize >= options.max_per_node_fanout {
        unsafe_expansion_flags.push("per_node_fanout_at_cap".into());
    }
    let observed_max_lane_coupling_fanout =
        lane_coupling_fanout.values().copied().max().unwrap_or(0);
    if observed_max_lane_coupling_fanout as usize >= options.max_lane_coupling_fanout {
        unsafe_expansion_flags.push("lane_coupling_fanout_at_cap".into());
    }

    let expansion_report = MapGenLinksExpansionReport {
        link_count: links.len() as u32,
        max_links: options.max_links as u32,
        per_node_fanout: per_node_fanout.clone(),
        max_per_node_fanout: options.max_per_node_fanout as u32,
        lane_coupling_count: lane_couplings.len() as u32,
        max_lane_coupling_count: options.max_lane_couplings as u32,
        max_lane_coupling_fanout: options.max_lane_coupling_fanout as u32,
        unknown_endpoint_rejections,
        self_link_rejections,
        duplicate_link_rejections,
        unsafe_expansion_flags,
    };

    let mut out_pack = pack.clone();
    out_pack.grid_metadata.links = links;
    out_pack.grid_metadata.max_fanout = options.max_per_node_fanout;
    for coupling in &lane_couplings {
        out_pack
            .game_mode
            .properties
            .push(lane_coupling_property(coupling));
    }

    assert_no_deferred_pr5_surfaces(&out_pack)?;
    assert_no_forbidden_generated_properties(&out_pack)?;
    assert_allowed_simthing_kinds(&out_pack.root_node)
        .map_err(|err| MapGenLinksError::new(err.message))?;

    Ok(MapGenLinksEnrollment {
        pack: out_pack,
        lane_couplings,
        expansion_report,
    })
}

/// Extract hyperlane endpoint pairs from a neutral MapGen document.
pub fn extract_hyperlane_declarations(
    document: &MapGenNeutralDocument,
) -> Result<Vec<(String, String)>, MapGenLinksError> {
    let root = root_block(&document.document)?;
    if root.properties.len() != 1 {
        return Err(MapGenLinksError::new(
            "MapGen fixture expects exactly one top-level slice block",
        ));
    }
    let RawValue::Block(slice_block) = &root.properties[0].value else {
        return Err(MapGenLinksError::new("MapGen slice root must be a block"));
    };
    let static_galaxy = require_block(slice_block, "static_galaxy_scenario")?;
    let mut hyperlanes = Vec::new();
    for property in property_values_matching(static_galaxy, "add_hyperlane") {
        hyperlanes.push(parse_hyperlane(property)?);
    }
    Ok(hyperlanes)
}

fn validate_options(options: &MapGenLinksOptions) -> Result<(), MapGenLinksError> {
    if options.max_links == 0
        || options.max_lane_couplings == 0
        || options.max_lane_coupling_fanout == 0
        || options.max_per_node_fanout == 0
    {
        return Err(MapGenLinksError::new("PR5 link caps must be positive"));
    }
    Ok(())
}

fn placement_map(placements: &[HydratedScenarioGridPlacement]) -> BTreeMap<String, (u32, u32)> {
    placements
        .iter()
        .map(|placement| {
            (
                placement.location_id.clone(),
                (placement.row, placement.col),
            )
        })
        .collect()
}

fn is_n4_neighbor(left: (u32, u32), right: (u32, u32)) -> bool {
    (left.0 == right.0 && left.1.abs_diff(right.1) == 1)
        || (left.1 == right.1 && left.0.abs_diff(right.0) == 1)
}

fn canonical_pair(from: &str, to: &str) -> (String, String) {
    if from < to {
        (from.to_string(), to.to_string())
    } else {
        (to.to_string(), from.to_string())
    }
}

fn lane_coupling_property(coupling: &MapGenLaneCoupling) -> PropertySpec {
    let name = format!("lane_coupling_{}_{}", coupling.from, coupling.to);
    PropertySpec {
        id: format!("mapgen_{name}"),
        namespace: "mapgen".into(),
        name: name.clone(),
        display_name: name,
        description: format!("inert={}:{}", coupling.from, coupling.to),
        sub_fields: vec![],
    }
}

fn parse_hyperlane(value: &RawValue) -> Result<(String, String), MapGenLinksError> {
    let RawValue::Block(block) = value else {
        return Err(MapGenLinksError::new("add_hyperlane must be a block"));
    };
    let from = require_scalar(block, "from")?;
    let to = require_scalar(block, "to")?;
    Ok((from, to))
}

fn assert_no_deferred_pr5_surfaces(pack: &HydratedScenarioPack) -> Result<(), MapGenLinksError> {
    if pack.w_impedance_compose.is_some() || pack.stress_compose.is_some() {
        return Err(MapGenLinksError::new(
            "PR5 generator must not emit field_operator surfaces",
        ));
    }
    if pack.palma_feedstock.is_some() {
        return Err(MapGenLinksError::new(
            "PR5 generator must not emit PALMA feedstock",
        ));
    }
    if pack.commitment.is_some() {
        return Err(MapGenLinksError::new(
            "PR5 generator must not emit FIELD_POLICY commitment",
        ));
    }
    Ok(())
}

fn assert_no_forbidden_generated_properties(
    pack: &HydratedScenarioPack,
) -> Result<(), MapGenLinksError> {
    for property in &pack.game_mode.properties {
        reject_forbidden_property_name(property)?;
    }
    walk_forbidden_properties(&pack.root_node)?;
    Ok(())
}

fn walk_forbidden_properties(
    node: &crate::hydrate_scenario::HydratedScenarioNode,
) -> Result<(), MapGenLinksError> {
    for property in &node.properties {
        reject_forbidden_property_name(property)?;
    }
    for child in &node.children {
        walk_forbidden_properties(child)?;
    }
    Ok(())
}

fn reject_forbidden_property_name(property: &PropertySpec) -> Result<(), MapGenLinksError> {
    let haystack = format!(
        "{} {} {} {}",
        property.id, property.namespace, property.name, property.description
    );
    for forbidden in FORBIDDEN_GENERATED_PROPERTY_NAMES {
        if haystack.contains(forbidden) {
            return Err(MapGenLinksError::new(format!(
                "generated property must not reference forbidden vocabulary `{forbidden}`"
            )));
        }
    }
    Ok(())
}

fn root_block(document: &crate::raw::RawDocument) -> Result<&RawBlock, MapGenLinksError> {
    let RawValue::Block(block) = &document.root else {
        return Err(MapGenLinksError::new("document root must be a block"));
    };
    Ok(block)
}

fn require_block<'a>(block: &'a RawBlock, key: &str) -> Result<&'a RawBlock, MapGenLinksError> {
    let RawValue::Block(nested) = block_value(block, key)? else {
        return Err(MapGenLinksError::new(format!("`{key}` must be a block")));
    };
    Ok(nested)
}

fn block_value<'a>(block: &'a RawBlock, key: &str) -> Result<&'a RawValue, MapGenLinksError> {
    Ok(&require_property(block, key)?.value)
}

fn require_property<'a>(
    block: &'a RawBlock,
    key: &str,
) -> Result<&'a RawProperty, MapGenLinksError> {
    block
        .properties
        .iter()
        .find(|property| property.key.text == key)
        .ok_or_else(|| MapGenLinksError::new(format!("missing property `{key}`")))
}

fn property_values_matching<'a>(block: &'a RawBlock, key: &str) -> Vec<&'a RawValue> {
    block
        .properties
        .iter()
        .filter(|property| property.key.text == key)
        .map(|property| &property.value)
        .collect()
}

fn require_scalar(block: &RawBlock, key: &str) -> Result<String, MapGenLinksError> {
    optional_scalar(block, key)?.ok_or_else(|| MapGenLinksError::new(format!("missing `{key}`")))
}

fn optional_scalar(block: &RawBlock, key: &str) -> Result<Option<String>, MapGenLinksError> {
    match block
        .properties
        .iter()
        .find(|property| property.key.text == key)
    {
        Some(property) => Ok(Some(scalar_text(&property.value)?.to_string())),
        None => Ok(None),
    }
}

fn scalar_text(value: &RawValue) -> Result<&str, MapGenLinksError> {
    let RawValue::Scalar(scalar) = value else {
        return Err(MapGenLinksError::new("expected scalar value"));
    };
    Ok(scalar.text.as_str())
}
