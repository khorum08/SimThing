//! Spec → session installation.
//!
//! Orchestrates compilation of a `GameModeSpec` against a live `Scenario`:
//! registers properties, builds capability trees, clones each tree per
//! resolved owner with fresh `OverlayId`s, and assembles a populated
//! `SpecSessionState` ready for `SimSession::install_spec_state`.
//!
//! See `docs/adr/game_mode_session_installation.md` for design rationale.

use simthing_core::{
    kind_matches, Overlay, OverlayId, SimThing, SimThingId, SimThingKind,
};
use simthing_core::DimensionRegistry;
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_event, compile_property, CapabilityEntryKey, CapabilityTreeBuildOutput,
    CapabilityTreeBuilder, CapabilityTreeInstance, CapabilityTreeState, CapabilityTreeSpec,
    CapabilityUnlockRegistration, DomainPackSpec, EventSpec, GameModeSpec, InstallTargetSpec,
    SpecError,
};
use std::collections::HashMap;
use thiserror::Error;

use crate::scenario::Scenario;
use crate::spec_session::SpecSessionState;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("spec error: {0}")]
    Spec(#[from] SpecError),

    #[error("capability tree `{tree_id}` resolved to zero owners for target `{target:?}`")]
    NoMatchingOwners {
        tree_id: String,
        target:   InstallTargetSpec,
    },

    #[error("scenario install_targets key `{key}` is not defined in the scenario")]
    UnknownInstallTarget { key: String },

    #[error("slot allocation overflow for owner {owner_id:?} (cloned tree exceeds scenario n_slots; raise n_slots)")]
    SlotOverflow { owner_id: SimThingId },

    #[error("session root has no slot — allocator was not populated before install_targets resolution")]
    RootHasNoSlot,
}

/// Compile a `GameModeSpec` against the supplied scenario state and return a
/// populated `SpecSessionState`.
///
/// Mutates `registry`, `root`, and `allocator` in place:
/// - New `SimProperty`s from the spec are registered with `registry`.
/// - Cloned capability tree `SimThing`s are attached as children of their
///   resolved owners under `root`.
/// - The allocator is re-populated to assign slots to every new node.
///
/// Caller is responsible for re-syncing GPU state after this returns (handled
/// by `SimSession::install_spec_state`).
pub fn compile_and_install(
    game_mode: &GameModeSpec,
    scenario:  &Scenario,
    registry:  &mut DimensionRegistry,
    root:      &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<SpecSessionState, InstallError> {
    let mut state = SpecSessionState::new();

    // ── 1. Compile properties (domain packs first, then game mode top-level).
    for pack in &game_mode.domain_packs {
        compile_pack_properties(pack, registry)?;
    }
    for prop_spec in &game_mode.properties {
        compile_property(prop_spec, registry)?;
    }

    // Global overlays from the game mode envelope are deferred per the ADR
    // (`docs/adr/game_mode_session_installation.md` §4). Capability tree
    // overlays compile inline through `CapabilityTreeBuilder::build` below.

    // ── 2. Build each capability tree once. Collect per-pack provenance so
    //      diagnostics can name the originating pack later (not used in v0).
    let mut compiled_trees: Vec<CompiledTree> = Vec::new();
    for tree_spec in &game_mode.capability_trees {
        compiled_trees.push(build_tree(tree_spec, registry)?);
    }
    for pack in &game_mode.domain_packs {
        for tree_spec in &pack.capability_trees {
            compiled_trees.push(build_tree(tree_spec, registry)?);
        }
    }

    // ── 3 + 4. Resolve install targets and clone trees per owner.
    let n_slots_cap = scenario.n_slots as usize;
    for compiled in &compiled_trees {
        let owners = resolve_install_target(&compiled.spec.install, scenario, root)?;
        if owners.is_empty() {
            return Err(InstallError::NoMatchingOwners {
                tree_id: compiled.spec.tree_id.clone(),
                target:  compiled.spec.install.clone(),
            });
        }
        for owner_id in owners {
            install_tree_for_owner(compiled, owner_id, root, allocator, &mut state)?;
        }
    }

    // After all cloned trees are attached, refuse to proceed if the allocator
    // outgrew the scenario's reserved slot capacity. Better a hard error here
    // than a silent GPU-buffer truncation later.
    if allocator.capacity() > n_slots_cap {
        // Find the first owner whose cloned tree overflowed for a useful
        // error payload. capacity() grew monotonically, so any cloned tree
        // can be cited — pick the most recently installed.
        let owner_id = state
            .capability_instances
            .values()
            .last()
            .map(|inst| inst.owner_id)
            .unwrap_or_else(SimThingId::new);
        return Err(InstallError::SlotOverflow { owner_id });
    }

    // ── 5. Scripted events: compile and install at session-global scope. The
    //      scope ADR (`docs/adr/scripted_event_scope_model.md`) will replace
    //      this with per-owner instances in O4.
    let root_slot = allocator
        .slot_of(root.id)
        .ok_or(InstallError::RootHasNoSlot)?;
    state.set_scripted_current_slot(root_slot);
    for event_spec in &game_mode.events {
        compile_and_install_event(event_spec, registry, &mut state)?;
    }
    for pack in &game_mode.domain_packs {
        for event_spec in &pack.events {
            compile_and_install_event(event_spec, registry, &mut state)?;
        }
    }

    Ok(state)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

struct CompiledTree<'spec> {
    spec:      &'spec CapabilityTreeSpec,
    build_out: CapabilityTreeBuildOutput,
}

fn compile_pack_properties(
    pack:     &DomainPackSpec,
    registry: &mut DimensionRegistry,
) -> Result<(), InstallError> {
    for prop_spec in &pack.properties {
        compile_property(prop_spec, registry)?;
    }
    Ok(())
}

fn build_tree<'spec>(
    spec:     &'spec CapabilityTreeSpec,
    registry: &mut DimensionRegistry,
) -> Result<CompiledTree<'spec>, InstallError> {
    let (build_out, _diag) = CapabilityTreeBuilder::build(spec, registry)?;
    Ok(CompiledTree { spec, build_out })
}

fn compile_and_install_event(
    spec:     &EventSpec,
    registry: &DimensionRegistry,
    state:    &mut SpecSessionState,
) -> Result<(), InstallError> {
    let (definition, _diag) = compile_event(spec, registry)?;
    state.add_scripted_event(definition);
    Ok(())
}

/// Resolve a `InstallTargetSpec` against the scenario's current root and the
/// `Scenario::install_targets` registry. Returns the matching owner ids.
pub(crate) fn resolve_install_target(
    target:   &InstallTargetSpec,
    scenario: &Scenario,
    root:     &SimThing,
) -> Result<Vec<SimThingId>, InstallError> {
    match target {
        InstallTargetSpec::AllOfKind { kind } => {
            let mut out = Vec::new();
            collect_matching_kind(root, kind, &mut out);
            Ok(out)
        }
        InstallTargetSpec::ScenarioListed { target_id } => {
            let owners = scenario
                .install_targets
                .get(target_id)
                .ok_or_else(|| InstallError::UnknownInstallTarget {
                    key: target_id.clone(),
                })?;
            Ok(owners.clone())
        }
        InstallTargetSpec::SessionRoot => Ok(vec![root.id]),
    }
}

fn collect_matching_kind(node: &SimThing, authored: &str, out: &mut Vec<SimThingId>) {
    if kind_matches(authored, &node.kind) {
        out.push(node.id);
    }
    for child in &node.children {
        collect_matching_kind(child, authored, out);
    }
}

/// Clone the template capability tree for one owner, attach it under that
/// owner in `root`, allocate slots, and register the instance in `state`.
fn install_tree_for_owner(
    compiled:  &CompiledTree<'_>,
    owner_id:  SimThingId,
    root:      &mut SimThing,
    allocator: &mut SlotAllocator,
    state:     &mut SpecSessionState,
) -> Result<(), InstallError> {
    let template = &compiled.build_out.tree;
    let definition = &compiled.build_out.definition;

    // 1. Clone the template with a fresh SimThingId. Properties carry over;
    //    overlays are re-stamped with new OverlayIds and have their `affects`
    //    pointed at the new clone so the handler emits the correct activation
    //    target later.
    let SimThingKind::Custom(tree_kind) = &template.kind else {
        unreachable!("CapabilityTreeBuilder always emits SimThingKind::Custom(tree_kind)");
    };
    let mut cloned = SimThing::new(SimThingKind::Custom(tree_kind.clone()), template.spawned_day);
    cloned.properties = template.properties.clone();

    let mut overlay_id_map: HashMap<OverlayId, OverlayId> = HashMap::new();
    let cloned_tree_id = cloned.id;
    for template_overlay in &template.overlays {
        let new_id = OverlayId::new();
        overlay_id_map.insert(template_overlay.id, new_id);
        let cloned_overlay = Overlay {
            id:        new_id,
            kind:      template_overlay.kind.clone(),
            source:    template_overlay.source.clone(),
            affects:   vec![cloned_tree_id],
            transform: template_overlay.transform.clone(),
            lifecycle: template_overlay.lifecycle.clone(),
        };
        cloned.add_overlay(cloned_overlay);
    }

    // 2. Attach as a child of the owner. If the owner is the root itself,
    //    attach there; otherwise walk the tree.
    if owner_id == root.id {
        root.add_child(cloned);
    } else {
        let attached = attach_child(root, owner_id, cloned);
        if !attached {
            return Err(InstallError::UnknownInstallTarget {
                key: format!("owner {:?} (not found in scenario root)", owner_id),
            });
        }
    }

    // 3. Re-populate slots so the cloned subtree gets allocations.
    allocator.populate_from_tree(root);
    let tree_slot = allocator
        .slot_of(cloned_tree_id)
        .ok_or(InstallError::SlotOverflow { owner_id })?;

    // 4. Per-owner unlock registrations point at the cloned tree id (not the
    //    template id). Re-map by zipping with the template's registrations.
    let unlock_registrations: Vec<CapabilityUnlockRegistration> = compiled
        .build_out
        .unlock_registrations
        .iter()
        .map(|reg| CapabilityUnlockRegistration {
            sim_thing_id: cloned_tree_id,
            property_id:  reg.property_id,
            sub_field:    reg.sub_field.clone(),
            threshold:    reg.threshold,
        })
        .collect();

    // 5. Per-instance by_overlay map, re-stamped via overlay_id_map.
    let by_overlay: HashMap<OverlayId, CapabilityEntryKey> = compiled
        .build_out
        .template_by_overlay
        .iter()
        .filter_map(|(old_id, key)| {
            overlay_id_map.get(old_id).map(|new_id| (*new_id, key.clone()))
        })
        .collect();

    let instance = CapabilityTreeInstance {
        owner_id,
        definition_id: definition.id,
        tree_thing_id: cloned_tree_id,
        tree_slot,
        by_overlay,
    };
    let initial_state = CapabilityTreeState {
        owner_id,
        definition_id: definition.id,
        activation_mode_by_entry: HashMap::new(),
        active_by_category:       HashMap::new(),
    };

    state.add_capability_tree_instance(
        definition.clone(),
        instance,
        initial_state,
        unlock_registrations,
    );

    Ok(())
}

/// Depth-first search for `owner_id` and attach `child` underneath. Returns
/// `true` on success and consumes `child`; returns `false` and hands `child`
/// back through the `Option` when the owner is not present.
fn attach_child(node: &mut SimThing, owner_id: SimThingId, child: SimThing) -> bool {
    if !contains(node, owner_id) {
        return false;
    }
    attach_child_known_present(node, owner_id, child)
}

fn contains(node: &SimThing, target: SimThingId) -> bool {
    node.id == target || node.children.iter().any(|c| contains(c, target))
}

/// Precondition: `contains(node, owner_id)` is true.
fn attach_child_known_present(
    node:     &mut SimThing,
    owner_id: SimThingId,
    child:    SimThing,
) -> bool {
    if node.id == owner_id {
        node.add_child(child);
        return true;
    }
    let target_idx = node
        .children
        .iter()
        .position(|c| contains(c, owner_id))
        .expect("contains() guaranteed at least one matching subtree");
    attach_child_known_present(&mut node.children[target_idx], owner_id, child)
}
