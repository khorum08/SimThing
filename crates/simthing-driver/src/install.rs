//! Spec → session installation.
//!
//! Orchestrates compilation of a `GameModeSpec` against a live `Scenario`:
//! registers properties, builds capability trees, clones each tree per
//! resolved owner with fresh `OverlayId`s, and assembles a populated
//! `SpecSessionState` ready for `SimSession::install_spec_state`.
//!
//! See `docs/adr/game_mode_session_installation.md` for design rationale.

use simthing_core::DimensionRegistry;
use simthing_core::{
    kind_matches, AccumulatorRole, Overlay, OverlayId, PropertyAdmissionDisposition, PropertyValue,
    RoleOffset, SimPropertyId, SimThing, SimThingId, SimThingKind,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    compile_event, compile_overlay, compile_property, compile_resource_economy,
    resolve_resource_flow_capacity_budget, CapabilityEntryKey, CapabilityTreeBuildOutput,
    CapabilityTreeBuilder, CapabilityTreeInstance, CapabilityTreeSpec, CapabilityTreeState,
    CapabilityUnlockRegistration, DomainPackSpec, EffectSpec, EffectTarget, EventSpec,
    GameModeSpec, InstallTargetSpec, OverlaySpec, PropertyKey, ResourceEconomySpec,
    ResourceFlowSpec, SpecError,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use thiserror::Error;

use crate::arena_registry::ArenaRegistry;
use crate::resource_economy_compile::{
    materialize_resource_economy_registry_for_session, ResourceEconomyCompileError,
};
use crate::resource_flow_compile::compile_and_materialize_resource_flow;
use crate::resource_flow_derivation::{
    derive_resource_flow_admission, ResourceFlowDerivationError,
};
use crate::resource_flow_enrollment::resolve_resource_flow_enrollment;
use crate::resource_flow_preflight::validate_resource_flow_preflight;
use crate::scenario::Scenario;
use crate::spec_session::SpecSessionState;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("spec error: {0}")]
    Spec(#[from] SpecError),

    #[error("field-plan admission: {0}")]
    FieldPlan(#[from] crate::comparative_default_birth::FieldPlanAdmissionError),

    #[error("capability tree `{tree_id}` resolved to zero owners for target `{target:?}`")]
    NoMatchingOwners {
        tree_id: String,
        target: InstallTargetSpec,
    },

    #[error("scenario install_targets key `{key}` is not defined in the scenario")]
    UnknownInstallTarget { key: String },

    #[error("slot allocation overflow for owner {owner_id:?} (cloned tree exceeds scenario n_slots; raise n_slots)")]
    SlotOverflow { owner_id: SimThingId },

    #[error(
        "session root has no slot — allocator was not populated before install_targets resolution"
    )]
    RootHasNoSlot,

    #[error("slot allocation error: {0}")]
    SlotAlloc(#[from] simthing_gpu::SlotAllocError),

    #[error("resource flow materialization exceeds scenario n_slots ({capacity} > {cap})")]
    ResourceFlowSlotOverflow { capacity: usize, cap: usize },

    #[error("resource economy compile: {0}")]
    ResourceEconomy(#[from] ResourceEconomyCompileError),

    #[error("Resource Flow derivation: {0}")]
    ResourceFlowDerivation(#[from] ResourceFlowDerivationError),

    #[error("Specialization protocol: {0}")]
    Specialization(#[from] simthing_core::SpecializationError),

    #[error("Resource Flow base obligation `{obligation}` targets SimThing {subtree_root_id} which is not admitted to arena `{arena}`")]
    BaseFlowObligationTargetNotAdmitted {
        obligation: String,
        arena: String,
        subtree_root_id: u32,
    },

    #[error("Resource Flow base obligation `{obligation}` admitted participant slot {slot} in arena `{arena}` has no owner")]
    BaseFlowObligationParticipantSlotMissing {
        obligation: String,
        arena: String,
        slot: u32,
    },

    #[error("Resource Flow base obligation `{obligation}` arena `{arena}` flow property has no IntrinsicFlow sub-field")]
    BaseFlowObligationMissingIntrinsicFlow { obligation: String, arena: String },

    #[error("need_binding `{binding}` invalid: {reason} (span_token={span_token:?})")]
    NeedBindingInvalid {
        binding: String,
        reason: String,
        /// Clause token index for spanned admission diagnostics.
        span_token: Option<usize>,
    },

    #[error("event `{event_id}` references unknown overlay `{overlay_ref}` (no standalone pack overlay with that authored id)")]
    UnknownOverlayRef {
        event_id: String,
        overlay_ref: String,
    },

    #[error("event `{event_id}` overlay ref `{overlay_ref}` resolved to {installed} installed overlay instances; per-owner effect resolution needs the SCOPE-MEMO successor — install on a single owner")]
    AmbiguousOverlayRef {
        event_id: String,
        overlay_ref: String,
        installed: usize,
    },

    #[error("duplicate standalone overlay authored id `{overlay_ref}` across domain packs")]
    DuplicateOverlayRefId { overlay_ref: String },

    #[error(
        "capability overlay {overlay_id:?} resolved to host {resolved_host:?} for property `{property}` but failed admission: {reason} (source_span_token={source_span_token:?})"
    )]
    CapabilityOverlayHostAdmission {
        overlay_id: OverlayId,
        resolved_host: SimThingId,
        property: String,
        source_span_token: Option<usize>,
        reason: String,
    },

    #[error(
        "gated rate `{gated}` requires a `rate_base` sub-field on arena `{arena}`'s flow property"
    )]
    GatedRateMissingBaseColumn { gated: String, arena: String },

    #[error("gated rate `{gated}` references unresolvable trigger property `{property}`")]
    GatedRateUnknownTriggerProperty { gated: String, property: String },

    /// 5.3b observation-host materialization: missing/ambiguous/out-of-tree candidates.
    #[error(
        "observation-host materialization for `{property}` failed: {reason} (provenance={provenance})"
    )]
    ObservationHostMaterialization {
        property: String,
        reason: String,
        provenance: String,
    },
}

/// Compile a `GameModeSpec` against the supplied scenario state and return a
/// populated `SpecSessionState`.
///
/// **In-place worker.** Mutates `registry`, `root`, and `allocator` directly
/// and **does not roll back on error**. If you need atomic-on-error
/// semantics (the usual case), prefer [`install_atomic`] or
/// [`preview_install`] — both wrap this function against scratch clones.
/// See `docs/adr/install_clone_then_commit.md`.
///
/// Mutations applied:
/// - New `SimProperty`s from the spec are registered with `registry`.
/// - Cloned capability tree `SimThing`s are attached as children of their
///   resolved owners under `root`.
/// - The allocator is re-populated to assign slots to every new node.
///
/// Caller is responsible for re-syncing GPU state after this returns (handled
/// by `SimSession::install_spec_state`).
pub fn compile_and_install(
    game_mode: &GameModeSpec,
    scenario: &Scenario,
    registry: &mut DimensionRegistry,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<SpecSessionState, InstallError> {
    let mut state = SpecSessionState::new();

    // ── 0. Order-weight class table (ORDER-WEIGHT-CLASS-0).
    simthing_spec::validate_order_weight_classes(&game_mode.order_weight_classes)
        .map_err(InstallError::Spec)?;
    for overlay_spec in &game_mode.overlays {
        simthing_spec::validate_order_weight_overlay(overlay_spec, &game_mode.order_weight_classes)
            .map_err(InstallError::Spec)?;
    }
    for pack in &game_mode.domain_packs {
        for overlay_spec in &pack.overlays {
            simthing_spec::validate_order_weight_overlay(
                overlay_spec,
                &game_mode.order_weight_classes,
            )
            .map_err(InstallError::Spec)?;
        }
    }
    // ── 1. Compile properties (domain packs first, then game mode top-level).
    for pack in &game_mode.domain_packs {
        compile_pack_properties(pack, registry)?;
    }
    for prop_spec in &game_mode.properties {
        compile_property(prop_spec, registry)?;
    }

    // ── 1b. Domain-pack standalone overlays (after properties are registered).
    //       The authored-id → installed OverlayId map feeds event effect
    //       resolution in step 5 (CT-1b `ActivateOverlayRef`).
    let mut overlay_ref_ids: HashMap<String, Vec<OverlayId>> = HashMap::new();
    for pack in &game_mode.domain_packs {
        install_pack_standalone_overlays(
            pack,
            registry,
            scenario,
            root,
            allocator,
            &mut overlay_ref_ids,
            &game_mode.order_weight_classes,
        )?;
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
    let resource_flow_capacity_budget = match &game_mode.resource_flow {
        Some(resource_flow) => {
            resolve_resource_flow_capacity_budget(resource_flow.capacity_budget.as_ref())?
        }
        None => None,
    };
    let n_slots_cap = resource_flow_capacity_budget
        .as_ref()
        .map(|budget| budget.gpu_slots as usize)
        .unwrap_or(scenario.n_slots as usize)
        .max(scenario.n_slots as usize);
    for compiled in &compiled_trees {
        let owners = resolve_install_target(&compiled.spec.install, scenario, root)?;
        if owners.is_empty() {
            return Err(InstallError::NoMatchingOwners {
                tree_id: compiled.spec.tree_id.clone(),
                target: compiled.spec.install.clone(),
            });
        }
        let root_id = root.id;
        for owner_id in owners {
            install_tree_for_owner(
                compiled, owner_id, root_id, registry, root, allocator, &mut state,
            )?;
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

    // ── 4b. Resource Flow admission: populated resource properties + typed
    //      parent edges derive the default arena. ResourceFlowSpec remains an
    //      override surface and is resolved onto the same downstream plan.
    let derived_resource_flow = derive_resource_flow_admission(
        game_mode.resource_flow.as_ref(),
        registry,
        root,
        allocator,
    )?;
    state.resource_flow_derivation = derived_resource_flow.report;

    // ── 4b′. Specialization protocol (3.1): derive structural conformance and
    //      validate declared profiles against admission artifacts. Observation
    //      + validation only; nothing downstream consults profiles yet (3.2).
    //      Structural placements are spec-side artifacts not visible to this
    //      install path; placement-gated profiles honestly do not derive here
    //      (artifact-complete callers assemble full observations themselves).
    // Observations derive from the INSTALLED TREE's authoritative hydration
    // stamps (remand 5098731168): structural col/row coordinate properties for
    // placement, and the typed policy/weight authority stamp for owner seats.
    // There is no caller-supplied observation input — conformance facts cannot
    // be fabricated by callers.
    let mut spec_observations = simthing_core::SpecializationObservations::default();
    collect_tree_observations(root, &mut spec_observations);
    state.specialization = simthing_core::derive_specializations(
        root,
        &simthing_core::seed_profiles(),
        &spec_observations,
    )?;
    if let Some(resource_flow) = derived_resource_flow.spec.as_ref() {
        let resolved = resolve_resource_flow_enrollment(resource_flow, scenario, root, allocator)?;
        let base_obligations = resolve_base_flow_obligation_targets(&resolved, scenario, root)?;
        validate_resource_flow_preflight(&resolved, allocator)?;
        let (arena_registry, report) = compile_and_materialize_resource_flow(&resolved, registry)?;
        seed_base_flow_obligations(
            &base_obligations,
            registry,
            root,
            allocator,
            &arena_registry,
        )?;
        // CT-RF-EML-RATE-0: resolve gated rates and copy the folded static
        // rate into the base column the per-tick EvalEML band reads.
        let gated = crate::gated_rates::resolve_gated_rates(
            &resolved,
            scenario,
            root,
            registry,
            &arena_registry,
        )?;
        crate::gated_rates::seed_gated_rate_base_columns(&gated, registry, root, allocator)?;
        state.resolved_gated_rates = gated;
        state.arena_registry = arena_registry;
        state.resource_flow_capacity_budget = report.capacity_budget;
    }
    if !game_mode.order_weight_classes.is_empty() {
        state.order_weight_classes = crate::order_directive::admit_order_weight_classes(
            &game_mode.order_weight_classes,
            registry,
            root,
            &state.arena_registry,
        )?;
    }

    // ── 4c. Resource economy (Phase T): compile + live-slot materialization.
    // Properties are placed on authored entity hosts (stockpile owner / install
    // target prefix), not invented by need_binding.
    if let Some(resource_economy) = &game_mode.resource_economy {
        ensure_resource_economy_properties(resource_economy, registry, root, scenario)?;
        let eml_registry = simthing_core::EmlExpressionRegistry::new();
        let compiled = compile_resource_economy(resource_economy, registry, &eml_registry)?;
        state.resource_economy_registry = Some(materialize_resource_economy_registry_for_session(
            &compiled,
            registry,
            &eml_registry,
            root,
            allocator,
            scenario,
        )?);
    }

    // RF-5A: resolve full-cell bindings only from already-authored property instances.
    if let Some(resource_flow) = derived_resource_flow.spec.as_ref() {
        if !resource_flow.need_bindings.is_empty() {
            let resolved = crate::need_binding::resolve_need_bindings(
                resource_flow,
                scenario,
                root,
                registry,
                &state.arena_registry,
                allocator,
            )?;
            crate::need_binding::prepare_need_binding_cells(&resolved, registry, root)?;
            if state.resource_economy_registry.is_none() {
                state.resource_economy_registry =
                    Some(crate::resource_economy_compile::ResourceEconomyRegistry {
                        registrations:
                            crate::resource_economy_compile::ResourceEconomyRegistrations {
                                transfers: vec![],
                                recipes: vec![],
                                emissions: vec![],
                                emit_on_threshold: vec![],
                                report: Default::default(),
                            },
                        generation: 1,
                    });
            }
            if let Some(economy) = state.resource_economy_registry.as_mut() {
                crate::need_binding::inject_need_binding_thresholds(&resolved, economy);
            }
            state.resolved_need_bindings = resolved;
        }
    }

    // ── 5. Scripted events: one definition + N per-owner instances per
    //      `EventSpec.install` (O4, `docs/adr/scripted_event_scope_model.md`).
    //      Default install is `SessionRoot` — pre-O4 behavior.
    let root_slot = allocator
        .slot_of(root.id)
        .ok_or(InstallError::RootHasNoSlot)?;
    state.set_session_root_owner(root.id);
    state.set_scripted_current_slot(root_slot);
    for event_spec in &game_mode.events {
        compile_and_install_event(
            event_spec,
            registry,
            scenario,
            root,
            allocator,
            &mut state,
            &overlay_ref_ids,
        )?;
    }
    for pack in &game_mode.domain_packs {
        for event_spec in &pack.events {
            compile_and_install_event(
                event_spec,
                registry,
                scenario,
                root,
                allocator,
                &mut state,
                &overlay_ref_ids,
            )?;
        }
    }

    // 5.3b: observation-host materialization for Anchored properties that still
    // lack live loci after economy/need/event resolution. Existing loci win.
    materialize_observation_hosts(game_mode, registry, root, scenario)?;

    state.property_admission = registry.property_admission_report();

    // 5.8b (DA 5154348081): ordinary install mints the field-plan product from
    // authored GameModeSpec.region_fields (S3 + default emitters). Triad columns
    // remain explicit 5.8 consumer inputs — not defaulted here.
    if let Some(report) = crate::comparative_default_birth::admit_field_plan_from_region_fields(
        &game_mode.region_fields,
    )? {
        state.field_plan_admission = Some(report);
    }

    Ok(state)
}

// ── Helpers ───────────────────────────────────────────────────────────────────

struct CompiledTree<'spec> {
    spec: &'spec CapabilityTreeSpec,
    build_out: CapabilityTreeBuildOutput,
}

fn compile_pack_properties(
    pack: &DomainPackSpec,
    registry: &mut DimensionRegistry,
) -> Result<(), InstallError> {
    for prop_spec in &pack.properties {
        compile_property(prop_spec, registry)?;
    }
    Ok(())
}

/// Install standalone `DomainPackSpec::overlays` through the same host/affects
/// semantics as capability-tree effect overlays: compile via `compile_overlay`,
/// resolve `OverlaySpec::install`, seed the target property on each owner host,
/// attach one re-stamped overlay per owner with `affects = [owner_id]`.
fn install_pack_standalone_overlays(
    pack: &DomainPackSpec,
    registry: &DimensionRegistry,
    scenario: &Scenario,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
    overlay_ref_ids: &mut HashMap<String, Vec<OverlayId>>,
    order_weight_classes: &[simthing_spec::OrderWeightClassSpec],
) -> Result<(), InstallError> {
    for overlay_spec in &pack.overlays {
        if overlay_ref_ids.contains_key(&overlay_spec.id) {
            return Err(InstallError::DuplicateOverlayRefId {
                overlay_ref: overlay_spec.id.clone(),
            });
        }
        let installed = install_standalone_overlay(
            overlay_spec,
            registry,
            scenario,
            root,
            order_weight_classes,
        )?;
        overlay_ref_ids.insert(overlay_spec.id.clone(), installed);
    }
    if !pack.overlays.is_empty() && allocator.slot_of(root.id).is_none() {
        allocator.populate_from_tree(root);
    }
    Ok(())
}

fn install_standalone_overlay(
    overlay_spec: &OverlaySpec,
    registry: &DimensionRegistry,
    scenario: &Scenario,
    root: &mut SimThing,
    order_weight_classes: &[simthing_spec::OrderWeightClassSpec],
) -> Result<Vec<OverlayId>, InstallError> {
    simthing_spec::validate_order_weight_overlay(overlay_spec, order_weight_classes)
        .map_err(InstallError::Spec)?;
    let (template, diag) =
        compile_overlay(overlay_spec, registry, scenario.root.id).map_err(InstallError::Spec)?;
    if !diag.diagnostics.is_empty() {
        return Err(InstallError::Spec(SpecError::ValidationFailedAt {
            site: "simthing-driver/install",
        }));
    }

    let owners = resolve_install_target(&overlay_spec.install, scenario, root)?;
    if owners.is_empty() {
        return Err(InstallError::NoMatchingOwners {
            tree_id: overlay_spec.id.clone(),
            target: overlay_spec.install.clone(),
        });
    }

    let prop_id = template.transform.property_id;
    let mut props_to_seed = HashSet::new();
    props_to_seed.insert(prop_id);

    let mut installed_ids = Vec::with_capacity(owners.len());
    for owner_id in owners {
        seed_effect_props_on(root, owner_id, &props_to_seed, registry);
        let overlay = Overlay {
            id: OverlayId::new(),
            kind: template.kind.clone(),
            source: template.source.clone(),
            origin: template.origin,
            affects: vec![owner_id],
            transform: template.transform.clone(),
            lifecycle: template.lifecycle.clone(),
        };
        if let Some(host) = find_simthing_mut(root, owner_id) {
            installed_ids.push(overlay.id);
            host.add_overlay(overlay);
        }
    }

    Ok(installed_ids)
}

/// Materialize resource-economy property instances onto **explicit** entity hosts.
///
/// Host authority comes only from authored `host_entity` / `*_host_entity` fields
/// (or World root when host is absent). No `{entity}_` name-prefix inference.
/// Constant emission formulas seed tree `PropertyValue` defaults (not dense GPU writes).
fn ensure_resource_economy_properties(
    spec: &ResourceEconomySpec,
    registry: &DimensionRegistry,
    root: &mut SimThing,
    scenario: &Scenario,
) -> Result<(), InstallError> {
    let placements = resource_economy_property_placements(spec);
    let mut qualified_hosts = HashMap::new();
    for placement in &placements {
        let Some(entity) = placement.host_entity.as_deref() else {
            continue;
        };
        let property_id = registry
            .id_of(&placement.key.namespace, &placement.key.name)
            .ok_or_else(|| SpecError::ValidationFailedAt {
                site: "simthing-driver/install",
            })?;
        let host_id = resolve_unique_install_host(scenario, entity, placement.host_span)?;
        if let Some(previous_host) = qualified_hosts.insert(property_id, host_id) {
            if previous_host != host_id {
                return Err(InstallError::NeedBindingInvalid {
                    binding: "resource_economy".into(),
                    reason: format!(
                        "property {}::{} has duplicate/conflicting economy host placement; PropertyKey is not row authority",
                        placement.key.namespace, placement.key.name
                    ),
                    span_token: placement.host_span,
                });
            }
        }
    }

    for placement in placements {
        let property_id = registry
            .id_of(&placement.key.namespace, &placement.key.name)
            .ok_or_else(|| SpecError::ValidationFailedAt {
                site: "simthing-driver/install",
            })?;
        let host_id = match &placement.host_entity {
            Some(entity) => resolve_unique_install_host(scenario, entity, placement.host_span)?,
            None => {
                // Unqualified: keep existing host if already placed; else World root.
                if let Some(existing) =
                    crate::resource_economy_compile::find_property_owner(root, property_id)
                {
                    existing
                } else {
                    root.id
                }
            }
        };
        let host = find_simthing_mut(root, host_id).ok_or_else(|| {
            InstallError::Spec(SpecError::ValidationFailedAt {
                site: "simthing-driver/install",
            })
        })?;
        if !host.properties.contains_key(&property_id) {
            let layout = registry.property(property_id).layout.clone();
            host.add_property(property_id, PropertyValue::from_layout(&layout));
        }
        if let Some((role, value)) = placement.seed {
            let layout = registry.property(property_id).layout.clone();
            if let Some(pv) = host.properties.get_mut(&property_id) {
                pv.set_role(&role, &layout, value);
            }
        }
    }
    Ok(())
}

fn resolve_unique_install_host(
    scenario: &Scenario,
    entity: &str,
    span: Option<usize>,
) -> Result<SimThingId, InstallError> {
    let Some(hosts) = scenario.install_targets.get(entity) else {
        return Err(InstallError::NeedBindingInvalid {
            binding: "resource_economy".into(),
            reason: format!("economy host entity `{entity}` is not in install_targets"),
            span_token: span,
        });
    };
    if hosts.len() != 1 {
        return Err(InstallError::NeedBindingInvalid {
            binding: "resource_economy".into(),
            reason: format!(
                "entity `{entity}` host is ambiguous ({} hosts) for economy property placement",
                hosts.len()
            ),
            span_token: span,
        });
    }
    Ok(hosts[0])
}

/// Place layout-default observation hosts for Anchored resource properties that
/// still have zero live loci, elected only from value-placing relations.
///
/// Vocabulary (DA residency law): economy emission/transfer/recipe hosts,
/// threshold hosts, need_binding loci, RF parent-edge child hosts. Governance
/// instruments (owner-policy overlays, policy-weight authority) never elect.
fn materialize_observation_hosts(
    game_mode: &GameModeSpec,
    registry: &DimensionRegistry,
    root: &mut SimThing,
    scenario: &Scenario,
) -> Result<(), InstallError> {
    // Disposition-only / micro installs have no value-placing vocabulary; the
    // materialization door stays inert so Anchored inventory can still report.
    if !game_mode_has_value_placing_vocabulary(game_mode, root) {
        return Ok(());
    }

    let mut live_counts: HashMap<SimPropertyId, usize> = HashMap::new();
    count_live_property_loci(root, &mut live_counts);

    let mut candidates: BTreeMap<SimPropertyId, BTreeMap<String, BTreeSet<String>>> =
        BTreeMap::new();
    let push = |out: &mut BTreeMap<SimPropertyId, BTreeMap<String, BTreeSet<String>>>,
                key: &PropertyKey,
                host: Option<&str>,
                provenance: String| {
        let Some(host) = host.filter(|h| !h.is_empty()) else {
            return;
        };
        let Some(property_id) = registry.id_of(&key.namespace, &key.name) else {
            return;
        };
        let prop = registry.property(property_id);
        if !prop.is_resource_bearing()
            || !matches!(
                prop.admission_disposition,
                PropertyAdmissionDisposition::Anchored
            )
        {
            return;
        }
        out.entry(property_id)
            .or_default()
            .entry(host.to_string())
            .or_default()
            .insert(provenance);
    };

    if let Some(economy) = &game_mode.resource_economy {
        for emission in &economy.emissions {
            // Hosted-observation locations reach production only as the lowered
            // presence-emission's typed host_entity (no id/name substring classing).
            push(
                &mut candidates,
                &emission.source,
                emission.host_entity.as_deref(),
                format!("economy.emission.host_entity id={}", emission.id),
            );
        }
        for thresh in &economy.emit_on_threshold {
            push(
                &mut candidates,
                &thresh.source,
                thresh.host_entity.as_deref(),
                format!("economy.emit_on_threshold.host_entity id={}", thresh.id),
            );
        }
        for transfer in &economy.transfers {
            push(
                &mut candidates,
                &transfer.source,
                transfer.source_host_entity.as_deref(),
                format!("economy.transfer.source_host_entity id={}", transfer.id),
            );
            push(
                &mut candidates,
                &transfer.target,
                transfer.target_host_entity.as_deref(),
                format!("economy.transfer.target_host_entity id={}", transfer.id),
            );
        }
        for recipe in &economy.recipes {
            push(
                &mut candidates,
                &recipe.target,
                recipe.target_host_entity.as_deref(),
                format!("economy.recipe.target_host_entity id={}", recipe.id),
            );
            for (idx, input) in recipe.inputs.iter().enumerate() {
                push(
                    &mut candidates,
                    &input.property,
                    input.host_entity.as_deref(),
                    format!(
                        "economy.recipe.input.host_entity id={} input[{idx}]",
                        recipe.id
                    ),
                );
            }
        }
    }
    if let Some(rf) = &game_mode.resource_flow {
        for binding in &rf.need_bindings {
            for locus in binding.inputs.iter().chain(binding.weights.iter()) {
                push(
                    &mut candidates,
                    &locus.property,
                    Some(locus.entity.as_str()),
                    format!(
                        "need_binding.locus.entity id={} role={:?}",
                        binding.id, locus.role
                    ),
                );
            }
        }
    }
    collect_rf_edge_observation_candidates(root, registry, &mut candidates);

    // Admission governs existence: every active admitted Anchored resource
    // property with zero live loci is derivation debt (no registry carve-outs).
    let mut anchored_ids: Vec<SimPropertyId> = registry
        .properties
        .iter()
        .enumerate()
        .filter(|(idx, prop)| {
            let property_id = SimPropertyId(*idx as u32);
            registry.is_active(property_id)
                && prop.is_resource_bearing()
                && matches!(
                    prop.admission_disposition,
                    PropertyAdmissionDisposition::Anchored
                )
        })
        .map(|(idx, _)| SimPropertyId(idx as u32))
        .collect();
    anchored_ids.sort_by_key(|id| id.0);

    for property_id in anchored_ids {
        if live_counts.get(&property_id).copied().unwrap_or(0) > 0 {
            continue;
        }
        let prop = registry.property(property_id);
        let identity = format!("{}::{}", prop.namespace, prop.name);
        let host_map = candidates.get(&property_id).cloned().unwrap_or_default();
        let hosts: BTreeSet<String> = host_map.keys().cloned().collect();
        let provenance = host_map
            .values()
            .flat_map(|set| set.iter().cloned())
            .collect::<Vec<_>>()
            .join("; ");
        let host_entity = match hosts.len() {
            0 => {
                return Err(InstallError::ObservationHostMaterialization {
                    property: identity,
                    reason: "zero value-placing candidates".into(),
                    provenance: if provenance.is_empty() {
                        "—".into()
                    } else {
                        provenance
                    },
                });
            }
            1 => hosts.into_iter().next().unwrap(),
            _ => {
                return Err(InstallError::ObservationHostMaterialization {
                    property: identity,
                    reason: format!("conflicting value-placing hosts {hosts:?}"),
                    provenance,
                });
            }
        };
        let host_id = resolve_observation_host_id(scenario, &host_entity, &identity, &provenance)?;
        let host = find_simthing_mut(root, host_id).ok_or_else(|| {
            InstallError::ObservationHostMaterialization {
                property: identity.clone(),
                reason: format!("elected host {host_entity} is out of tree"),
                provenance: provenance.clone(),
            }
        })?;
        if !host.properties.contains_key(&property_id) {
            let layout = registry.property(property_id).layout.clone();
            host.add_property(property_id, PropertyValue::from_layout(&layout));
        }
    }
    Ok(())
}

fn game_mode_has_value_placing_vocabulary(game_mode: &GameModeSpec, root: &SimThing) -> bool {
    if let Some(economy) = &game_mode.resource_economy {
        if !economy.emissions.is_empty()
            || !economy.emit_on_threshold.is_empty()
            || !economy.transfers.is_empty()
            || !economy.recipes.is_empty()
        {
            return true;
        }
    }
    if let Some(rf) = &game_mode.resource_flow {
        if rf
            .need_bindings
            .iter()
            .any(|b| !b.inputs.is_empty() || !b.weights.is_empty())
        {
            return true;
        }
    }
    tree_has_rf_parent_edges(root)
}

fn tree_has_rf_parent_edges(node: &SimThing) -> bool {
    if !node.resource_parent_edges.is_empty() {
        return true;
    }
    node.children.iter().any(tree_has_rf_parent_edges)
}

fn resolve_observation_host_id(
    scenario: &Scenario,
    host_entity: &str,
    property: &str,
    provenance: &str,
) -> Result<SimThingId, InstallError> {
    if let Some(raw) = host_entity.strip_prefix("simthing:") {
        let id = raw
            .parse::<u32>()
            .map_err(|_| InstallError::ObservationHostMaterialization {
                property: property.into(),
                reason: format!("malformed RF host key `{host_entity}`"),
                provenance: provenance.into(),
            })?;
        return Ok(SimThingId::from_session_raw(id));
    }
    let Some(hosts) = scenario.install_targets.get(host_entity) else {
        return Err(InstallError::ObservationHostMaterialization {
            property: property.into(),
            reason: format!("elected host `{host_entity}` is not in install_targets"),
            provenance: provenance.into(),
        });
    };
    if hosts.len() != 1 {
        return Err(InstallError::ObservationHostMaterialization {
            property: property.into(),
            reason: format!(
                "elected host `{host_entity}` is ambiguous ({} install_targets)",
                hosts.len()
            ),
            provenance: provenance.into(),
        });
    }
    Ok(hosts[0])
}

fn count_live_property_loci(node: &SimThing, counts: &mut HashMap<SimPropertyId, usize>) {
    for &pid in node.properties.keys() {
        *counts.entry(pid).or_default() += 1;
    }
    for child in &node.children {
        count_live_property_loci(child, counts);
    }
}

fn collect_rf_edge_observation_candidates(
    node: &SimThing,
    registry: &DimensionRegistry,
    out: &mut BTreeMap<SimPropertyId, BTreeMap<String, BTreeSet<String>>>,
) {
    for edge in &node.resource_parent_edges {
        if let Some(property_id) = registry.id_of(&edge.property_namespace, &edge.property_name) {
            let prop = registry.property(property_id);
            if prop.is_resource_bearing()
                && matches!(
                    prop.admission_disposition,
                    PropertyAdmissionDisposition::Anchored
                )
            {
                out.entry(property_id)
                    .or_default()
                    .entry(format!("simthing:{}", node.id.raw()))
                    .or_default()
                    .insert(format!(
                        "rf_parent_edge.child_host parent={} span={:?}",
                        edge.parent.raw(),
                        edge.source_span_token
                    ));
            }
        }
    }
    for child in &node.children {
        collect_rf_edge_observation_candidates(child, registry, out);
    }
}

struct EconomyPropertyPlacement {
    key: PropertyKey,
    host_entity: Option<String>,
    host_span: Option<usize>,
    /// Optional tree seed from authored Constant emission (role, value).
    seed: Option<(simthing_core::SubFieldRole, f32)>,
}

fn resource_economy_property_placements(
    spec: &ResourceEconomySpec,
) -> Vec<EconomyPropertyPlacement> {
    let mut out = Vec::new();
    for transfer in &spec.transfers {
        out.push(EconomyPropertyPlacement {
            key: transfer.source.clone(),
            host_entity: transfer.source_host_entity.clone(),
            host_span: transfer.source_host_span_token,
            seed: None,
        });
        out.push(EconomyPropertyPlacement {
            key: transfer.target.clone(),
            host_entity: transfer.target_host_entity.clone(),
            host_span: transfer.target_host_span_token,
            seed: None,
        });
    }
    for recipe in &spec.recipes {
        for input in &recipe.inputs {
            out.push(EconomyPropertyPlacement {
                key: input.property.clone(),
                host_entity: input.host_entity.clone(),
                host_span: input.host_span_token,
                seed: None,
            });
        }
        out.push(EconomyPropertyPlacement {
            key: recipe.target.clone(),
            host_entity: recipe.target_host_entity.clone(),
            host_span: recipe.target_host_span_token,
            seed: None,
        });
    }
    for emission in &spec.emissions {
        let seed = match &emission.formula {
            simthing_spec::EmissionFormulaSpec::Constant(v) if v.is_finite() => {
                Some((emission.source_role.clone(), *v))
            }
            _ => None,
        };
        out.push(EconomyPropertyPlacement {
            key: emission.source.clone(),
            host_entity: emission.host_entity.clone(),
            host_span: emission.host_span_token,
            seed,
        });
    }
    for emit in &spec.emit_on_threshold {
        out.push(EconomyPropertyPlacement {
            key: emit.source.clone(),
            host_entity: emit.host_entity.clone(),
            host_span: emit.host_span_token,
            seed: None,
        });
    }
    out
}

#[derive(Clone, Debug)]
struct ResolvedBaseFlowObligation {
    obligation: String,
    arena: String,
    arena_idx: u32,
    flow_property: PropertyKey,
    signed_rate: f32,
    hosted_ids: Vec<SimThingId>,
}

fn resolve_base_flow_obligation_targets(
    spec: &ResourceFlowSpec,
    scenario: &Scenario,
    root: &SimThing,
) -> Result<Vec<ResolvedBaseFlowObligation>, InstallError> {
    let mut out = Vec::with_capacity(spec.base_obligations.len());
    for obligation in &spec.base_obligations {
        let (arena_idx, arena) = spec
            .arenas
            .iter()
            .enumerate()
            .find(|(_, arena)| arena.name == obligation.arena)
            .ok_or_else(|| SpecError::UnknownArenaReference {
                arena: obligation.arena.clone(),
                context: format!("base_obligations.{}", obligation.id),
            })?;
        let hosted_ids = resolve_install_target(&obligation.install, scenario, root)?;
        if hosted_ids.is_empty() {
            return Err(InstallError::NoMatchingOwners {
                tree_id: obligation.id.clone(),
                target: obligation.install.clone(),
            });
        }
        for hosted_id in &hosted_ids {
            let raw = hosted_id.raw();
            let admitted = arena
                .explicit_participants
                .iter()
                .any(|participant| participant.subtree_root_id == raw);
            if !admitted {
                return Err(InstallError::BaseFlowObligationTargetNotAdmitted {
                    obligation: obligation.id.clone(),
                    arena: arena.name.clone(),
                    subtree_root_id: raw,
                });
            }
        }
        out.push(ResolvedBaseFlowObligation {
            obligation: obligation.id.clone(),
            arena: arena.name.clone(),
            arena_idx: arena_idx as u32,
            flow_property: arena.flow_property.clone(),
            signed_rate: obligation.signed_rate(),
            hosted_ids,
        });
    }
    Ok(out)
}

fn seed_base_flow_obligations(
    obligations: &[ResolvedBaseFlowObligation],
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &SlotAllocator,
    arena_registry: &ArenaRegistry,
) -> Result<(), InstallError> {
    for obligation in obligations {
        let flow_property_id = resolve_base_flow_property(registry, &obligation.flow_property)?;
        let intrinsic_offset =
            intrinsic_flow_offset(registry, flow_property_id).ok_or_else(|| {
                InstallError::BaseFlowObligationMissingIntrinsicFlow {
                    obligation: obligation.obligation.clone(),
                    arena: obligation.arena.clone(),
                }
            })?;
        for hosted_id in &obligation.hosted_ids {
            let participant_slot = arena_registry
                .participant_slot(*hosted_id, obligation.arena_idx)
                .ok_or_else(|| InstallError::BaseFlowObligationTargetNotAdmitted {
                    obligation: obligation.obligation.clone(),
                    arena: obligation.arena.clone(),
                    subtree_root_id: hosted_id.raw(),
                })?;
            let participant_id = allocator.owner_of(participant_slot).ok_or_else(|| {
                InstallError::BaseFlowObligationParticipantSlotMissing {
                    obligation: obligation.obligation.clone(),
                    arena: obligation.arena.clone(),
                    slot: participant_slot.raw(),
                }
            })?;
            let Some(participant_node) = find_simthing_mut(root, participant_id) else {
                return Err(InstallError::BaseFlowObligationParticipantSlotMissing {
                    obligation: obligation.obligation.clone(),
                    arena: obligation.arena.clone(),
                    slot: participant_slot.raw(),
                });
            };
            let Some(value) = participant_node.properties.get_mut(&flow_property_id) else {
                return Err(InstallError::Spec(SpecError::ValidationFailedAt {
                    site: "simthing-driver/install",
                }));
            };
            value.add_lane_at_offset(intrinsic_offset, obligation.signed_rate);
        }
    }
    Ok(())
}

fn resolve_base_flow_property(
    registry: &DimensionRegistry,
    key: &PropertyKey,
) -> Result<SimPropertyId, InstallError> {
    registry.id_of(&key.namespace, &key.name).ok_or_else(|| {
        InstallError::Spec(SpecError::UnknownResourceFlowProperty {
            property: format!("{}::{}", key.namespace, key.name),
        })
    })
}

pub(crate) fn intrinsic_flow_offset(
    registry: &DimensionRegistry,
    property_id: SimPropertyId,
) -> Option<RoleOffset> {
    // invariant: local index arithmetic has one home — resolve the role,
    // then go through `PropertyLayout::offset_of` (enumeration position is
    // only coincidentally correct while every sub-field has width 1).
    let layout = &registry.property(property_id).layout;
    let role = layout.sub_fields.iter().find_map(|sub_field| {
        sub_field
            .accumulator_spec
            .as_ref()
            .filter(|spec| matches!(spec.role, AccumulatorRole::IntrinsicFlow))
            .map(|_| sub_field.role.clone())
    })?;
    layout.offset_of(&role)
}

fn build_tree<'spec>(
    spec: &'spec CapabilityTreeSpec,
    registry: &mut DimensionRegistry,
) -> Result<CompiledTree<'spec>, InstallError> {
    let (build_out, _diag) = CapabilityTreeBuilder::build(spec, registry)?;
    Ok(CompiledTree { spec, build_out })
}

fn compile_and_install_event(
    spec: &EventSpec,
    registry: &DimensionRegistry,
    scenario: &Scenario,
    root: &SimThing,
    allocator: &SlotAllocator,
    state: &mut SpecSessionState,
    overlay_ref_ids: &HashMap<String, Vec<OverlayId>>,
) -> Result<(), InstallError> {
    let resolved = resolve_event_overlay_refs(spec, overlay_ref_ids)?;
    let (definition, _diag) = compile_event(&resolved, registry)?;
    let owners = resolve_install_target(&spec.install, scenario, root)?;
    if owners.is_empty() {
        return Err(InstallError::NoMatchingOwners {
            tree_id: spec.id.clone(),
            target: spec.install.clone(),
        });
    }
    // O4: one definition, N per-owner instances pointing at it.
    let event_id = definition.id.clone();
    let definition_id = state.register_scripted_event_definition(definition);
    for owner_id in owners {
        let slot = allocator
            .slot_of(owner_id)
            .ok_or(InstallError::RootHasNoSlot)?;
        let _ =
            state.attach_scripted_event_instance(definition_id, event_id.clone(), owner_id, slot);
    }
    Ok(())
}

/// Resolve `ActivateOverlayRef` effects against the standalone-overlay install
/// map. A ref must resolve to exactly one installed overlay instance —
/// per-owner resolution over shared event definitions is SCOPE-MEMO
/// SPEC-SCOPE-1 territory and is rejected here, not approximated.
fn resolve_event_overlay_refs(
    spec: &EventSpec,
    overlay_ref_ids: &HashMap<String, Vec<OverlayId>>,
) -> Result<EventSpec, InstallError> {
    if !spec
        .effects
        .iter()
        .any(|effect| matches!(effect, EffectSpec::ActivateOverlayRef { .. }))
    {
        return Ok(spec.clone());
    }
    let mut resolved = spec.clone();
    for effect in &mut resolved.effects {
        let EffectSpec::ActivateOverlayRef {
            target,
            overlay_ref,
        } = effect
        else {
            continue;
        };
        let installed =
            overlay_ref_ids
                .get(overlay_ref)
                .ok_or_else(|| InstallError::UnknownOverlayRef {
                    event_id: spec.id.clone(),
                    overlay_ref: overlay_ref.clone(),
                })?;
        let [overlay_id] = installed.as_slice() else {
            return Err(InstallError::AmbiguousOverlayRef {
                event_id: spec.id.clone(),
                overlay_ref: overlay_ref.clone(),
                installed: installed.len(),
            });
        };
        *effect = EffectSpec::ActivateOverlay {
            target: *target,
            overlay_id: *overlay_id,
        };
    }
    Ok(resolved)
}

/// Resolve a `InstallTargetSpec` against the scenario's current root and the
/// `Scenario::install_targets` registry. Returns the matching owner ids.
pub(crate) fn resolve_install_target(
    target: &InstallTargetSpec,
    scenario: &Scenario,
    root: &SimThing,
) -> Result<Vec<SimThingId>, InstallError> {
    match target {
        InstallTargetSpec::AllOfKind { kind } => {
            let mut out = Vec::new();
            collect_matching_kind(root, kind, &mut out);
            Ok(out)
        }
        InstallTargetSpec::ScenarioListed { target_id } => {
            let owners = scenario.install_targets.get(target_id).ok_or_else(|| {
                InstallError::UnknownInstallTarget {
                    key: target_id.clone(),
                }
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
    compiled: &CompiledTree<'_>,
    owner_id: SimThingId,
    root_id: SimThingId,
    registry: &DimensionRegistry,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
    state: &mut SpecSessionState,
) -> Result<(), InstallError> {
    let template = &compiled.build_out.tree;
    let definition = &compiled.build_out.definition;

    // 1. Clone the template with a fresh SimThingId. Properties carry over;
    //    overlays are re-stamped with new OverlayIds. Each cloned overlay's
    //    `affects` list is resolved from the authored `EffectTarget` on the
    //    corresponding `CapabilityEffectSpec` (see
    //    `docs/adr/capability_effect_target_scope.md`):
    //      - Owner          → vec![owner_id]
    //      - CapabilityTree → vec![cloned_tree_id]   (v0 behavior)
    //      - SessionRoot    → vec![root_id]
    let SimThingKind::Custom(tree_kind) = &template.kind else {
        unreachable!("CapabilityTreeBuilder always emits SimThingKind::Custom(tree_kind)");
    };
    let mut cloned = SimThing::new(
        SimThingKind::Custom(tree_kind.clone()),
        template.spawned_generation,
    );
    cloned.properties = template.properties.clone();

    let mut overlay_id_map: HashMap<OverlayId, OverlayId> = HashMap::new();
    let cloned_tree_id = cloned.id;
    // Per-effect overlay placement admission. GPU overlay-prep
    // walks the SimThing tree depth-first and applies each overlay's
    // transform to every node in its descendant subtree that carries the
    // target property. Therefore an overlay's HOST node must be an ancestor
    // of every affected slot — for `Owner`, host = owner (the clone's parent);
    // for `CapabilityTree`, host = clone; for `SessionRoot`, host = root.
    let mut owner_overlays: Vec<Overlay> = Vec::new();
    let mut root_overlays: Vec<Overlay> = Vec::new();
    let mut overlay_hosts: HashMap<OverlayId, SimThingId> = HashMap::new();
    for template_overlay in &template.overlays {
        let new_id = OverlayId::new();
        overlay_id_map.insert(template_overlay.id, new_id);
        let target = compiled
            .build_out
            .template_effect_targets
            .get(&template_overlay.id)
            .copied()
            .unwrap_or_default();
        let affects = resolve_effect_target(target, owner_id, cloned_tree_id, root_id);
        let host = match target {
            EffectTarget::Owner => owner_id,
            EffectTarget::CapabilityTree => cloned_tree_id,
            EffectTarget::SessionRoot => root_id,
        };
        let new_overlay = Overlay {
            id: new_id,
            kind: template_overlay.kind.clone(),
            source: template_overlay.source.clone(),
            origin: cloned_tree_id,
            affects,
            transform: template_overlay.transform.clone(),
            lifecycle: template_overlay.lifecycle.clone(),
        };
        let source_span_token = compiled
            .build_out
            .template_effect_source_spans
            .get(&template_overlay.id)
            .copied()
            .flatten();
        let host_node = match target {
            EffectTarget::CapabilityTree => &cloned,
            EffectTarget::Owner | EffectTarget::SessionRoot => find_simthing(root, host)
                .ok_or_else(|| {
                    capability_overlay_admission_error(
                        registry,
                        &new_overlay,
                        host,
                        source_span_token,
                        "resolved host is absent from the admitted SimThing tree",
                    )
                })?,
        };
        validate_capability_overlay_host(
            registry,
            host_node,
            &new_overlay,
            host,
            source_span_token,
        )?;
        overlay_hosts.insert(new_id, host);
        match target {
            EffectTarget::CapabilityTree => cloned.add_overlay(new_overlay),
            EffectTarget::Owner => owner_overlays.push(new_overlay),
            EffectTarget::SessionRoot => root_overlays.push(new_overlay),
        }
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

    // 2b. Attach owner/root overlays to their admitted host SimThings. The GPU
    //     ancestor walk requires the overlay to live on a node that is
    //     itself an ancestor of (or equal to) every affected slot.
    if !owner_overlays.is_empty() {
        if let Some(owner_node) = find_simthing_mut(root, owner_id) {
            for overlay in owner_overlays {
                owner_node.add_overlay(overlay);
            }
        }
    }
    if !root_overlays.is_empty() {
        for overlay in root_overlays {
            root.add_overlay(overlay);
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
            property_id: reg.property_id,
            sub_field: reg.sub_field.clone(),
            threshold: reg.threshold,
        })
        .collect();

    // 5. Per-instance by_overlay map, re-stamped via overlay_id_map.
    let by_overlay: HashMap<OverlayId, CapabilityEntryKey> = compiled
        .build_out
        .template_by_overlay
        .iter()
        .filter_map(|(old_id, key)| {
            overlay_id_map
                .get(old_id)
                .map(|new_id| (*new_id, key.clone()))
        })
        .collect();

    let instance = CapabilityTreeInstance {
        owner_id,
        definition_id: definition.id,
        tree_thing_id: cloned_tree_id,
        tree_slot: tree_slot.raw(),
        by_overlay,
        overlay_hosts,
    };
    let initial_state = CapabilityTreeState {
        owner_id,
        definition_id: definition.id,
        activation_mode_by_entry: HashMap::new(),
        active_by_category: HashMap::new(),
    };

    state.add_capability_tree_instance(
        definition.clone(),
        instance,
        initial_state,
        unlock_registrations,
    );

    Ok(())
}

/// Resolve a `CapabilityEffectSpec.effect_target` to the concrete
/// `affects: Vec<SimThingId>` list used at install time. Per the
/// EffectTarget ADR, `Owner` is the v1 default — install rewrites the
/// affects list rather than the v0 hard-coded clone target.
fn resolve_effect_target(
    target: EffectTarget,
    owner_id: SimThingId,
    clone_id: SimThingId,
    root_id: SimThingId,
) -> Vec<SimThingId> {
    match target {
        EffectTarget::Owner => vec![owner_id],
        EffectTarget::CapabilityTree => vec![clone_id],
        EffectTarget::SessionRoot => vec![root_id],
    }
}

fn validate_capability_overlay_host(
    registry: &DimensionRegistry,
    host: &SimThing,
    overlay: &Overlay,
    resolved_host: SimThingId,
    source_span_token: Option<usize>,
) -> Result<(), InstallError> {
    if overlay.affects.as_slice() != [resolved_host] {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "overlay affects metadata diverges from its resolved host",
        ));
    }

    let property_id = overlay.transform.property_id;
    if !registry.is_active(property_id) {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "target property is not active in the dimension registry",
        ));
    }
    let Some(host_value) = host.properties.get(&property_id) else {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "resolved host does not carry the target property",
        ));
    };

    let Some(property) = registry.try_property(property_id) else {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "target property is not registered",
        ));
    };
    let Some(columns) = registry.try_column_range(property_id) else {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "target property has no registered column range",
        ));
    };
    if host_value.raw_lanes().len() != property.layout.stride() {
        return Err(capability_overlay_admission_error(
            registry,
            overlay,
            resolved_host,
            source_span_token,
            "resolved host property value does not match the registered layout",
        ));
    }
    for (role, _) in &overlay.transform.sub_field_deltas {
        if columns.col_for_role(role, &property.layout).is_none() {
            return Err(capability_overlay_admission_error(
                registry,
                overlay,
                resolved_host,
                source_span_token,
                &format!("target role `{role:?}` is not layout-resolvable"),
            ));
        }
    }
    Ok(())
}

fn capability_overlay_admission_error(
    registry: &DimensionRegistry,
    overlay: &Overlay,
    resolved_host: SimThingId,
    source_span_token: Option<usize>,
    reason: &str,
) -> InstallError {
    let property_id = overlay.transform.property_id;
    let property = registry
        .try_property(property_id)
        .map(|property| format!("{}::{}", property.namespace, property.name))
        .unwrap_or_else(|| format!("{property_id:?}"));
    InstallError::CapabilityOverlayHostAdmission {
        overlay_id: overlay.id,
        resolved_host,
        property,
        source_span_token,
        reason: reason.to_owned(),
    }
}

fn find_simthing(node: &SimThing, target: SimThingId) -> Option<&SimThing> {
    if node.id == target {
        return Some(node);
    }
    node.children
        .iter()
        .find_map(|child| find_simthing(child, target))
}

/// Find `target_id` inside `root` (depth-first) and add `props` to its
/// `properties` map with registry defaults if not already present.
/// Standalone domain-pack overlays retain this explicit materialization
/// behavior. Capability effects use `validate_capability_overlay_host`
/// instead and never invent a missing target property.
pub(crate) fn seed_effect_props_on(
    root: &mut SimThing,
    target_id: SimThingId,
    props: &HashSet<simthing_core::SimPropertyId>,
    registry: &DimensionRegistry,
) {
    if props.is_empty() {
        return;
    }
    if let Some(node) = find_simthing_mut(root, target_id) {
        for prop_id in props {
            if !node.properties.contains_key(prop_id) && registry.is_active(*prop_id) {
                node.add_property(*prop_id, registry.property(*prop_id).default_value());
            }
        }
    }
}

pub(crate) fn find_simthing_mut(node: &mut SimThing, target: SimThingId) -> Option<&mut SimThing> {
    if node.id == target {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_simthing_mut(child, target) {
            return Some(found);
        }
    }
    None
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
fn attach_child_known_present(node: &mut SimThing, owner_id: SimThingId, child: SimThing) -> bool {
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

// ── I1: clone-then-commit wrappers ────────────────────────────────────────────
//
// See `docs/adr/install_clone_then_commit.md`.

/// Staged output of a `preview_install` — the registry / root / allocator /
/// spec state that **would** be produced if the install were committed. The
/// caller inspects this (Studio "preview" panel, hot-reload verification),
/// then either commits via `SimSession::apply_install_preview` or discards.
///
/// All four fields are owned values (not references), so the preview can
/// outlive the inputs it was generated from.
#[derive(Debug)]
pub struct InstallPreview {
    pub registry: DimensionRegistry,
    pub root: SimThing,
    pub allocator: SlotAllocator,
    pub state: SpecSessionState,
}

/// Run a full `compile_and_install` against scratch copies of the caller's
/// state. On success, returns an `InstallPreview` carrying the populated
/// scratch. On error, the caller's `registry` / `root` / `allocator` are
/// completely untouched — useful for Studio preview workflows or any caller
/// that wants "try install, possibly discard."
///
/// Memory: peaks at roughly 2× the registry + root + allocator size for the
/// duration of the call. All three are small in practice.

/// SPECIALIZATION-PROTOCOL-0 (remand 5098731168): derive specialization
/// observations from the installed tree's authoritative hydration stamps.
/// Placement = the structural col AND row coordinate properties the grid
/// hydration writes; policy/weight seat = the typed authority stamp applied
/// ONLY to field-economy-referenced Owners (`owner_hosts_policy_weight_authority`
/// — the inert default silo marker never qualifies).
fn collect_tree_observations(
    node: &simthing_core::SimThing,
    observations: &mut simthing_core::SpecializationObservations,
) {
    if simthing_spec::gridcell_structural_col(node).is_some()
        && simthing_spec::gridcell_structural_row(node).is_some()
    {
        observations.structurally_placed.insert(node.id.raw());
    }
    if simthing_spec::owner_hosts_policy_weight_authority(node) {
        observations.policy_weight_hosts.insert(node.id.raw());
    }
    for child in &node.children {
        collect_tree_observations(child, observations);
    }
}

pub fn preview_install(
    game_mode: &GameModeSpec,
    scenario: &Scenario,
    registry: &DimensionRegistry,
    root: &SimThing,
    allocator: &SlotAllocator,
) -> Result<InstallPreview, InstallError> {
    let mut scratch_registry = registry.clone();
    let mut scratch_root = root.clone();
    let mut scratch_allocator = allocator.clone();
    let state = compile_and_install(
        game_mode,
        scenario,
        &mut scratch_registry,
        &mut scratch_root,
        &mut scratch_allocator,
    )?;
    Ok(InstallPreview {
        registry: scratch_registry,
        root: scratch_root,
        allocator: scratch_allocator,
        state,
    })
}

/// Atomic-on-error install: clones caller state, runs `compile_and_install`
/// against the clones, and commits the result back to the caller on success.
/// On error, caller state is unchanged. Drop-in replacement for
/// `compile_and_install` when atomicity is desired (which is the usual case).
///
/// Used by `SimSession::open_from_spec` so a failed install on a brand-new
/// session leaves the just-built `BoundaryProtocol` untouched, and by any
/// future caller that wants the same guarantee.
pub fn install_atomic(
    game_mode: &GameModeSpec,
    scenario: &Scenario,
    registry: &mut DimensionRegistry,
    root: &mut SimThing,
    allocator: &mut SlotAllocator,
) -> Result<SpecSessionState, InstallError> {
    let preview = preview_install(game_mode, scenario, registry, root, allocator)?;
    *registry = preview.registry;
    *root = preview.root;
    *allocator = preview.allocator;
    Ok(preview.state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        OverlayLifecycle, SimProperty, SimThing, SimThingKind, SubFieldRole, TransformOp,
    };
    use simthing_spec::{
        ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec,
        CapabilityTreeSpec, SpecVersion,
    };

    fn empty_scenario(world: SimThing) -> Scenario {
        let mut registry = DimensionRegistry::new();
        let _ = registry.register(SimProperty::simple("_placeholder", "seed", 0));
        Scenario {
            name: "i1_test".into(),
            ticks_per_day: 1,
            max_days: 1,
            dt: 0.0,
            n_slots: 16,
            registry,
            root: world,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets: HashMap::new(),
        }
    }

    fn stub_capability_spec() -> CapabilitySpec {
        CapabilitySpec {
            id: "stub".into(),
            display_name: "Stub".into(),
            description: String::new(),
            flavor_text: String::new(),
            research_cost: 1.0,
            activation: ActivationMode::Threshold,
            icon: String::new(),
            thumbnail: String::new(),
            card_image: String::new(),
            unlock_video: None,
            model_preview: None,
            prereqs: Vec::new(),
            unlocks_ship_components: Vec::new(),
            unlocks_buildings: Vec::new(),
            unlocks_units: Vec::new(),
            unlocks_weapons: Vec::new(),
            effects: Vec::new(),
        }
    }

    /// Game mode that attempts to install a capability tree on an owner
    /// kind that doesn't exist in the scenario. `CapabilityTreeBuilder`
    /// registers the category property during step 2 (build), then step 3
    /// fails with `NoMatchingOwners` — leaving the category property
    /// registered in the in-place worker. This is the partial-mutation
    /// footgun the ADR fixes.
    fn failing_game_mode() -> GameModeSpec {
        GameModeSpec {
            id: "i1_failure".into(),
            display_name: "I1 Failure Fixture".into(),
            description: String::new(),
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: Vec::new(),
            properties: Vec::new(),
            overlays: Vec::new(),
            order_weight_classes: vec![],
            capability_trees: vec![CapabilityTreeSpec {
                tree_id: "doomed_tree".into(),
                tree_kind: "doomed_tree".into(),
                owner_kind: "NonexistentKind".into(),
                install: InstallTargetSpec::AllOfKind {
                    kind: "NonexistentKind".into(),
                },
                categories: vec![CapabilityCategorySpec {
                    property_namespace: "i1_test".into(),
                    property_name: "marker".into(),
                    display_name: "Marker".into(),
                    tier: 0,
                    max_active: None,
                    entries: vec![stub_capability_spec()],
                    source_span_token: None,
                }],
            }],
            events: Vec::new(),
            resource_flow: None,
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![],
            mapping_execution_profile: Default::default(),
        }
    }

    /// Game mode that succeeds — installs one tree on the World root via
    /// `InstallTargetSpec::SessionRoot`.
    fn succeeding_game_mode() -> GameModeSpec {
        GameModeSpec {
            id: "i1_success".into(),
            display_name: "I1 Success Fixture".into(),
            description: String::new(),
            spec_version: SpecVersion::default(),
            metadata: Default::default(),
            domain_packs: Vec::new(),
            properties: Vec::new(),
            overlays: Vec::new(),
            order_weight_classes: vec![],
            capability_trees: vec![CapabilityTreeSpec {
                tree_id: "root_tree".into(),
                tree_kind: "root_tree".into(),
                owner_kind: "World".into(),
                install: InstallTargetSpec::SessionRoot,
                categories: vec![CapabilityCategorySpec {
                    property_namespace: "i1_test".into(),
                    property_name: "marker".into(),
                    display_name: "Marker".into(),
                    tier: 0,
                    max_active: None,
                    entries: vec![stub_capability_spec()],
                    source_span_token: None,
                }],
            }],
            events: Vec::new(),
            resource_flow: None,
            resource_economy: None,
            resource_flow_execution_profile: Default::default(),
            region_fields: vec![],
            mapping_execution_profile: Default::default(),
        }
    }

    fn fresh_caller_state(scenario: &Scenario) -> (DimensionRegistry, SimThing, SlotAllocator) {
        let mut allocator = SlotAllocator::new();
        allocator.populate_from_tree(&scenario.root);
        (scenario.registry.clone(), scenario.root.clone(), allocator)
    }

    fn effect_host_game_mode(source_span_token: usize) -> GameModeSpec {
        let mut game_mode = succeeding_game_mode();
        game_mode.capability_trees[0].categories[0].entries[0]
            .effects
            .push(CapabilityEffectSpec {
                targets_property: "effect_host::pressure".into(),
                sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::multiply(3.0))],
                when_activated: OverlayLifecycle::UntilDissolved,
                effect_target: EffectTarget::Owner,
                source_span_token: Some(source_span_token),
            });
        game_mode
    }

    #[test]
    fn overlay_effect_host_admission_rejects_missing_property_with_source_span() {
        let mut scenario = empty_scenario(SimThing::new(SimThingKind::World, 0));
        let _ = scenario
            .registry
            .register(SimProperty::simple("effect_host", "pressure", 0));
        let root_id = scenario.root.id;
        let (mut registry, mut root, mut allocator) = fresh_caller_state(&scenario);

        let error = match preview_install(
            &effect_host_game_mode(5094),
            &scenario,
            &mut registry,
            &mut root,
            &mut allocator,
        ) {
            Ok(_) => panic!("missing effect-host property must fail admission"),
            Err(error) => error,
        };

        match error {
            InstallError::CapabilityOverlayHostAdmission {
                resolved_host,
                property,
                source_span_token,
                reason,
                ..
            } => {
                assert_eq!(resolved_host, root_id);
                assert_eq!(property, "effect_host::pressure");
                assert_eq!(source_span_token, Some(5094));
                assert!(reason.contains("does not carry"));
            }
            other => panic!("unexpected admission error: {other}"),
        }
    }

    #[test]
    fn overlay_effect_host_admission_accepts_and_transforms_correct_host() {
        let mut scenario = empty_scenario(SimThing::new(SimThingKind::World, 0));
        let property_id =
            scenario
                .registry
                .register(SimProperty::simple("effect_host", "pressure", 0));
        let layout = scenario.registry.property(property_id).layout.clone();
        let mut value = scenario.registry.property(property_id).default_value();
        value.set_role(&SubFieldRole::Amount, &layout, 2.0);
        scenario.root.add_property(property_id, value);

        let root_id = scenario.root.id;
        let (mut registry, mut root, mut allocator) = fresh_caller_state(&scenario);
        let preview = preview_install(
            &effect_host_game_mode(5095),
            &scenario,
            &mut registry,
            &mut root,
            &mut allocator,
        )
        .expect("correctly hosted capability effect must admit");

        let instance = preview
            .state
            .capability_instances
            .values()
            .next()
            .expect("installed capability instance");
        let overlay_id = *instance
            .by_overlay
            .keys()
            .next()
            .expect("installed capability overlay");
        assert_eq!(instance.overlay_hosts.get(&overlay_id), Some(&root_id));

        let overlay = preview
            .root
            .overlays
            .iter()
            .find(|overlay| overlay.id == overlay_id)
            .expect("effect overlay lives on its resolved host");
        assert_eq!(overlay.affects, vec![root_id]);

        let mut transformed = preview.root.properties[&property_id].clone();
        overlay
            .transform
            .apply_to_data(transformed.raw_lanes_mut(), &layout);
        assert_eq!(
            transformed
                .get_role(&SubFieldRole::Amount, &layout)
                .to_bits(),
            6.0_f32.to_bits()
        );
    }
}
