//! SPECIALIZATION-PROTOCOL-0 (0.0.8.7 rung 3.1) referee — remand-2 `5098401165` form.
//!
//! Legs:
//! 1. ONE ordinary installed canonical-TP report (production
//!    `preview_install_with_observations` over the authority root with the
//!    hydrated placement artifact) derives ALL THREE seed populations, each
//!    checked against independent artifact oracles, row by row;
//! 2. owner-seat binds to the ADMITTED owner-silo policy/weight locus — an
//!    Owner with an unrelated accumulator-bearing property does NOT derive;
//! 3. session-root enforces the strict sole/direct-child invariant (three
//!    negatives + the admitted absolute-root posture proven separately);
//! 4. authored provenance for BOTH error classes (unknown profile + unmet
//!    requirement) with exact tokens from the parsed document;
//! 5. a concrete pre-3.1 serialized fixture loads, admits, and re-serializes
//!    without the new field.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use simthing_clausething::raw::RawValue;
use simthing_clausething::{
    hydrate_scenario_with_source_base, parse_raw_document, HydratedScenarioNode,
    HydratedScenarioPack, RawDocument,
};
use simthing_core::{
    derive_specializations, kind_identity, seed_profiles, AccumulatorRole, AccumulatorSpec,
    ClampBehavior, DimensionRegistry, KindIdentity, LogTier, SimProperty, SimThing, SimThingKind,
    SimThingKindTag, SpecializationError, SpecializationObservations, SpecializationRequirement,
    SubFieldRole, SubFieldSpec, PROFILE_OWNER_SEAT, PROFILE_SESSION_ROOT, PROFILE_SPATIAL,
};
use simthing_driver::{preview_install, InstallError, Scenario};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    apply_owner_policy_weight_authority, apply_owner_silo_metadata, compile_property,
    make_owner_entity, owner_hosts_policy_weight_authority, GameModeSpec, PropertySpec,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn hydrate_canonical() -> HydratedScenarioPack {
    let clause_path = repo_root().join("scenarios/terran_pirate_galaxy.clause");
    let text = std::fs::read_to_string(&clause_path).expect("read canonical clause");
    let document = parse_raw_document(text.as_bytes()).expect("parse canonical clause");
    let base = clause_path.parent().expect("clause parent").to_path_buf();
    hydrate_scenario_with_source_base(&document, Some(&base)).expect("hydrate canonical clause")
}

fn walk<'a>(node: &'a SimThing, f: &mut impl FnMut(&'a SimThing)) {
    f(node);
    for child in &node.children {
        walk(child, f);
    }
}

/// Authoritative placement artifact for the authority tree: hydration stamps
/// each embedded-grid gridcell with structural col/row properties written
/// FROM `embedded.namespaced_placements`. The stamped population is
/// cross-checked against the grid artifact count so the observation set is
/// tied to the spec-side grid, not to kinds.
fn authority_placed_set(pack: &HydratedScenarioPack, authority: &SimThing) -> BTreeSet<u32> {
    let mut placed = BTreeSet::new();
    walk(authority, &mut |n| {
        if simthing_spec::gridcell_structural_col(n).is_some()
            && simthing_spec::gridcell_structural_row(n).is_some()
        {
            placed.insert(n.id.raw());
        }
    });
    let mut artifact_pairs: Vec<(u32, u32)> = pack
        .embedded_static_galaxy_scenarios
        .iter()
        .flat_map(|e| e.namespaced_placements.iter().map(|p| (p.col, p.row)))
        .collect();
    artifact_pairs.sort_unstable();
    let mut stamped_pairs: Vec<(u32, u32)> = Vec::new();
    walk(authority, &mut |n| {
        if let (Some(col), Some(row)) = (
            simthing_spec::gridcell_structural_col(n),
            simthing_spec::gridcell_structural_row(n),
        ) {
            stamped_pairs.push((col, row));
        }
    });
    stamped_pairs.sort_unstable();
    assert_eq!(
        stamped_pairs, artifact_pairs,
        "structural stamps must match the embedded grid placement (col,row) artifact exactly"
    );
    placed
}

/// Hydrated placement artifact for the authored world tree: grid placements
/// keyed by authored location id; `root_node` maps authored ids to ids.
fn canonical_placed_set(pack: &HydratedScenarioPack) -> BTreeSet<u32> {
    fn index(node: &HydratedScenarioNode, map: &mut std::collections::HashMap<String, u32>) {
        map.insert(node.id.clone(), node.simthing_id.raw());
        for child in &node.children {
            index(child, map);
        }
    }
    let mut by_authored_id = std::collections::HashMap::new();
    index(&pack.root_node, &mut by_authored_id);
    pack.grid_metadata
        .placements
        .iter()
        .filter_map(|p| by_authored_id.get(&p.location_id).copied())
        .collect()
}

fn minimal_scenario(root: SimThing) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("_session", "seed", 0));
    Scenario {
        name: "specialization_protocol_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 8192,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: Default::default(),
    }
}

const CLAUSE_UNMET_PROOF: &str = r#"
scenario = span_proof {
    location = anchor_cell {
        display_name = "Anchor Cell"
    }
    owner = misdeclared {
        owner_key = "misdeclared"
        display_name = "Misdeclared"
        archetype = "expansionist"
        specialization = spatial
    }
}
"#;

const CLAUSE_UNKNOWN_PROOF: &str = r#"
scenario = span_proof_unknown {
    location = anchor_cell {
        display_name = "Anchor Cell"
    }
    owner = goblin_fan {
        owner_key = "goblin_fan"
        display_name = "Goblin Fan"
        archetype = "expansionist"
        specialization = warp_goblin
    }
}
"#;

fn declared_scalar_token(document: &RawDocument) -> usize {
    fn find(value: &RawValue, out: &mut Option<usize>) {
        match value {
            RawValue::Block(block) => {
                for property in &block.properties {
                    if property.key.text == "specialization" {
                        if let RawValue::Scalar(scalar) = &property.value {
                            *out = Some(scalar.span.token_index);
                            return;
                        }
                    }
                    find(&property.value, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Array(array) => {
                for item in &array.items {
                    find(item, out);
                    if out.is_some() {
                        return;
                    }
                }
            }
            RawValue::Header(header) => find(&header.payload, out),
            RawValue::Scalar(_) => {}
        }
    }
    let mut out = None;
    find(&document.root, &mut out);
    out.expect("authored specialization scalar present")
}

fn authored_owner_id(pack: &HydratedScenarioPack) -> u32 {
    pack.owners
        .first()
        .expect("authored owner")
        .simthing_id
        .raw()
}

/// Concrete pre-3.1 wire fixture: no `declared_specializations`, and the
/// pre-generation `spawned_day` field name (1.2 alias) — the oldest supported
/// wire shape this rung must not disturb.
const PRE_3_1_SIMTHING_JSON: &str = r#"{
    "id": 424242,
    "kind": "GameSession",
    "properties": [],
    "overlays": [],
    "children": [],
    "spawned_day": 0
}"#;

