//! Funded structural products lowered into the EXISTING 2.1 ActionBand door
//! (DA, relay 5737649159).
//!
//! Session build only. Each declared product line lowers to the bounded
//! threshold / template / active-instance sequence 2.1 graduated on: unit `k`
//! crosses `k + 0.5` on the authored funding locus and dispatches one
//! `StructuralAuthorization(AddChild)` consequence carrying a freshly minted,
//! detached instance of the authored template. Nothing here executes a birth:
//! each unit's crossing is the ordinary values-plane threshold on the funding
//! cell (the boundary's band-crossing source), the frozen session is installed
//! at tick zero through [`crate::SimSession::install_action_band_commitments`],
//! and the ordinary boundary performs structural authorization and residency
//! placement.

use std::collections::{BTreeSet, HashMap};

use simthing_core::owner_channel::{bind_owner, OwnerRef, UNOWNED_OWNER_REF};
use simthing_core::{
    DimensionRegistry, Direction, EmitOnThresholdBuffer, EmitOnThresholdRegistration,
    EmlExpressionRegistry, SimPropertyId, SimThing, SimThingId, SimThingKind, SlotIndex,
    ThresholdDirection,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::SlotAllocator;
use simthing_sim::{CostBandSemantic, SimRuntimeTree, ThresholdRegistry, VelocityAlertRegistration};
use simthing_spec::{
    compile_overlay, is_owner_entity_kind, owner_entity_id, ActionBandAdmissionBudgetSpec, ActionBandBandSpec,
    ActionBandChannelBindingSpec, ActionBandChannelKind, ActionBandSessionBuildDoor,
    ActionBandSessionSpec, ActionBandTargetSpec, ActionBandTemplateSpec, ScalarBoundDirection,
    StructuralProductSpec, StructuralTemplateNodeSpec,
};

use crate::{
    compile_crossing_consequence_session, ActionBandActiveInstance,
    ActionBandNativeLaneAdmission, CrossingConsequenceSession, StructuralAuthorization,
};

/// Typed pre-activation refusal of a structural product declaration.
#[derive(Debug, thiserror::Error)]
pub enum StructuralProductError {
    #[error("structural product `{product}` is declared more than once (token {span:?})")]
    DuplicateProduct { product: String, span: Option<usize> },
    #[error("structural product `{product}`: {role} `{entity}` {reason} (token {span:?})")]
    UnresolvedEntity {
        product: String,
        role: &'static str,
        entity: String,
        reason: String,
        span: Option<usize>,
    },
    #[error(
        "structural product `{product}`: parent `{entity}` is an owner seat; ownership is \
         declared on the template, never by spatial parentage (token {span:?})"
    )]
    OwnerSeatParent {
        product: String,
        entity: String,
        span: Option<usize>,
    },
    #[error("structural product `{product}`: funding locus {reason} (token {span:?})")]
    FundingLocus {
        product: String,
        reason: String,
        span: Option<usize>,
    },
    #[error("structural product `{product}`: template {reason} (token {span:?})")]
    Template {
        product: String,
        reason: String,
        span: Option<usize>,
    },
    #[error("structural products: ActionBand admission refused: {0}")]
    Admission(String),
}

struct Resolved {
    parent: SimThingId,
    host: SimThingId,
    host_slot: SlotIndex,
    property: SimPropertyId,
    column: simthing_core::ColumnIndex,
}

/// The session-build product: the frozen consequence session plus the
/// values-plane crossing registrations its bands consume.
pub(crate) struct LoweredStructuralProducts {
    commitments: CrossingConsequenceSession,
    crossings: Vec<VelocityAlertRegistration>,
}

/// Lower and install every declared product at tick zero: register each
/// unit's ordinary values-plane crossing, sync, then bind the frozen session
/// through the one existing install door (which refuses any later bind).
pub(crate) fn install_structural_products(
    session: &mut crate::SimSession,
    products: &[StructuralProductSpec],
) -> Result<(), crate::SessionError> {
    let Some(lowered) = lower_structural_products(
        products,
        &session.proto.registry,
        &session.scenario.root,
        &session.proto.root,
        &session.proto.allocator,
        &session.scenario.install_targets,
    )?
    else {
        return Ok(());
    };
    for crossing in lowered.crossings {
        session.proto.register_velocity_alert(crossing);
    }
    session
        .proto
        .initial_gpu_sync(&session.coord, &mut session.state)?;
    session.install_action_band_commitments(lowered.commitments)
}

/// Lower every declared product into ONE frozen consequence session, or
/// `None` when the game mode declares none. All references resolve here,
/// before activation; any unresolved or ambiguous reference refuses typed.
pub(crate) fn lower_structural_products(
    products: &[StructuralProductSpec],
    registry: &DimensionRegistry,
    scenario_root: &SimThing,
    runtime_root: &SimRuntimeTree,
    allocator: &SlotAllocator,
    install_targets: &HashMap<String, Vec<SimThingId>>,
) -> Result<Option<LoweredStructuralProducts>, StructuralProductError> {
    if products.is_empty() {
        return Ok(None);
    }
    let mut seen = BTreeSet::new();
    for product in products {
        if !seen.insert(product.id.as_str()) {
            return Err(StructuralProductError::DuplicateProduct {
                product: product.id.clone(),
                span: product.source_span_token,
            });
        }
    }
    let owners = declared_owners(scenario_root);

    let mut thresholds = Vec::new();
    let mut crossings = Vec::new();
    let mut templates = Vec::new();
    let mut consequences = Vec::new();
    let mut instance_slots = Vec::new();
    let mut channels = BTreeSet::new();
    for product in products {
        let resolved = resolve(product, registry, scenario_root, runtime_root, allocator, install_targets)?;
        channels.insert(resolved.column.raw_u32());
        for unit in 0..product.count.get() {
            let row = u32::try_from(thresholds.len())
                .map_err(|_| StructuralProductError::Admission("too many product units".into()))?;
            let bound = unit as f32 + 0.5;
            thresholds.push(EmitOnThresholdRegistration {
                slot: resolved.host_slot,
                col: resolved.column,
                threshold: bound,
                direction: ThresholdDirection::Upward,
                event_kind: row,
                buffer: EmitOnThresholdBuffer::Values,
            });
            crossings.push(VelocityAlertRegistration {
                sim_thing_id: resolved.host,
                property_id: resolved.property,
                sub_field: product.funding.role.clone(),
                threshold: bound,
                direction: Direction::Rising,
                cost_band: CostBandSemantic::observation(),
            });
            templates.push(ActionBandTemplateSpec {
                id: format!("{}#{unit}", product.id),
                label: None,
                axis_channels: vec![ActionBandChannelBindingSpec {
                    column: resolved.column.raw_u32(),
                    kind: ActionBandChannelKind::Primitive,
                }],
                target: ActionBandTargetSpec::ScalarBound {
                    channel: resolved.column.raw_u32(),
                    bound,
                    direction: ScalarBoundDirection::AtLeast,
                },
                velocity: None,
                bands: vec![ActionBandBandSpec {
                    threshold_registration_index: row,
                    eml_program: None,
                    emission_binding_indices: vec![row],
                }],
                subordinate_template_ids: vec![],
                max_active_subordinates: 0,
                reserved_instance_rows: 1,
                requirement_semantics: Default::default(),
            });
            // Every unit is its own freshly minted instance of the template.
            let child = instantiate(&product.template, registry, &owners, product)?;
            let consequence = StructuralAuthorization::admit(BoundaryRequest::AddChild {
                parent: resolved.parent,
                child,
            })
            .map_err(|error| StructuralProductError::Template {
                product: product.id.clone(),
                reason: format!("is not an admissible structural request: {error:?}"),
                span: product.source_span_token,
            })?;
            consequences.push(consequence);
            instance_slots.push(resolved.host_slot);
        }
    }

    let rows = u32::try_from(thresholds.len())
        .map_err(|_| StructuralProductError::Admission("too many product units".into()))?;
    let spec = ActionBandSessionSpec {
        budget: ActionBandAdmissionBudgetSpec {
            axis_channel_count: channels.len() as u32,
            dependency_binding_count: 0,
            storage_rows: rows,
            eml_program_count: 0,
            emission_binding_count: rows,
        },
        templates,
    };
    let eml = EmlExpressionRegistry::new();
    let frozen = ActionBandSessionBuildDoor::new()
        .admit_once_at_session_build(&spec, registry, &eml, &thresholds)
        .map_err(|error| StructuralProductError::Admission(error.to_string()))?
        .clone();
    let lanes = ActionBandNativeLaneAdmission::from_existing_surfaces(
        registry,
        &[],
        &[],
        &thresholds,
        &ThresholdRegistry::new(),
    );
    // Template indices follow authored position, so each unit keeps its own
    // funding host as its active-instance slot.
    let active = frozen
        .templates()
        .iter()
        .map(|template| {
            ActionBandActiveInstance::new(
                template.index(),
                instance_slots[template.index().raw() as usize],
                [0.0; 4],
            )
        })
        .collect::<Vec<_>>();
    let commitments =
        compile_crossing_consequence_session(&frozen, &eml, &consequences, &active, &lanes)
            .map_err(|error| StructuralProductError::Admission(error.to_string()))?;
    Ok(Some(LoweredStructuralProducts {
        commitments,
        crossings,
    }))
}

fn resolve(
    product: &StructuralProductSpec,
    registry: &DimensionRegistry,
    scenario_root: &SimThing,
    runtime_root: &SimRuntimeTree,
    allocator: &SlotAllocator,
    install_targets: &HashMap<String, Vec<SimThingId>>,
) -> Result<Resolved, StructuralProductError> {
    let span = product.source_span_token;
    let entity = |role: &'static str, name: &str| -> Result<SimThingId, StructuralProductError> {
        let unresolved = |reason: String| StructuralProductError::UnresolvedEntity {
            product: product.id.clone(),
            role,
            entity: name.to_string(),
            reason,
            span,
        };
        match install_targets.get(name).map(Vec::as_slice) {
            Some([id]) if runtime_root.contains_id(*id) => Ok(*id),
            Some([_]) => Err(unresolved("is absent from the installed tree".into())),
            Some(ids) if ids.len() > 1 => Err(unresolved(format!("is ambiguous ({} hosts)", ids.len()))),
            _ => Err(unresolved("is not in install_targets".into())),
        }
    };
    let parent = entity("parent", &product.parent_entity)?;
    if find(scenario_root, parent)
        .is_some_and(|node| is_owner_entity_kind(&node.kind) || owner_entity_id(node).is_some())
    {
        return Err(StructuralProductError::OwnerSeatParent {
            product: product.id.clone(),
            entity: product.parent_entity.clone(),
            span,
        });
    }
    let host = entity("funding host", &product.funding.host_entity)?;
    let funding = &product.funding;
    let key = format!("{}::{}", funding.property.namespace, funding.property.name);
    let locus = |reason: String| StructuralProductError::FundingLocus {
        product: product.id.clone(),
        reason,
        span,
    };
    let property = registry
        .id_of(&funding.property.namespace, &funding.property.name)
        .ok_or_else(|| locus(format!("property `{key}` is not registered")))?;
    let layout = &registry.property(property).layout;
    if !layout
        .sub_fields
        .iter()
        .any(|sub| sub.role == funding.role && sub.width == 1)
    {
        return Err(locus(format!(
            "role {:?} is not a scalar sub-field of `{key}`",
            funding.role
        )));
    }
    let column = registry
        .column_range(property)
        .col_for_role(&funding.role, layout)
        .ok_or_else(|| locus(format!("role {:?} has no column in `{key}`", funding.role)))?;
    let hosts_property = runtime_root
        .snapshot_node(host)
        .is_some_and(|node| node.property_ids.contains(&property));
    if !hosts_property {
        return Err(locus(format!(
            "host `{}` does not carry `{key}`",
            funding.host_entity
        )));
    }
    let host_slot = allocator.slot_of(host).ok_or_else(|| {
        locus(format!("host `{}` has no residency slot", funding.host_entity))
    })?;
    Ok(Resolved {
        parent,
        host,
        host_slot,
        property,
        column,
    })
}

/// Mint one detached instance: fresh identities, authored properties,
/// overlays owned by the node itself, explicit owner binding, and children.
fn instantiate(
    template: &StructuralTemplateNodeSpec,
    registry: &DimensionRegistry,
    owners: &BTreeSet<String>,
    product: &StructuralProductSpec,
) -> Result<SimThing, StructuralProductError> {
    let refuse = |reason: String| StructuralProductError::Template {
        product: product.id.clone(),
        reason,
        span: product.source_span_token,
    };
    if is_owner_entity_kind(&template.kind)
        || matches!(
            template.kind,
            SimThingKind::Scenario | SimThingKind::GameSession | SimThingKind::World
        )
    {
        return Err(refuse(format!("kind {:?} cannot be born as a product", template.kind)));
    }
    let mut node = SimThing::new(template.kind.clone(), 0);
    if let Some(owner) = &template.owner_ref {
        if owner != UNOWNED_OWNER_REF && !owners.contains(owner) {
            return Err(refuse(format!("owner `{owner}` is not a declared owner")));
        }
        bind_owner(&mut node, &OwnerRef::new(owner.clone()));
    }
    for cell in &template.property_values {
        let key = format!("{}::{}", cell.property.namespace, cell.property.name);
        let property_id = registry
            .id_of(&cell.property.namespace, &cell.property.name)
            .ok_or_else(|| refuse(format!("property `{key}` is not registered")))?;
        if node.properties.contains_key(&property_id) {
            return Err(refuse(format!("property `{key}` is authored twice on one node")));
        }
        let property = registry.property(property_id);
        let mut value = property.default_value();
        let mut roles = BTreeSet::new();
        for (role, amount) in &cell.values {
            if !amount.is_finite() {
                return Err(refuse(format!("`{key}` {role:?} value must be finite")));
            }
            if !roles.insert(format!("{role:?}"))
                || !property
                    .layout
                    .sub_fields
                    .iter()
                    .any(|sub| &sub.role == role && sub.width == 1)
            {
                return Err(refuse(format!(
                    "`{key}` {role:?} is duplicated or not a scalar sub-field"
                )));
            }
            value.set_role(role, &property.layout, *amount);
        }
        node.add_property(property_id, value);
    }
    for overlay in &template.overlays {
        let (mut compiled, _) = compile_overlay(overlay, registry, node.id)
            .map_err(|error| refuse(format!("overlay `{}`: {error}", overlay.id)))?;
        compiled.affects = vec![node.id];
        node.overlays.push(compiled);
    }
    for child in &template.children {
        node.add_child(instantiate(child, registry, owners, product)?);
    }
    Ok(node)
}

fn declared_owners(root: &SimThing) -> BTreeSet<String> {
    let mut owners = BTreeSet::new();
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        if let Some(owner) = owner_entity_id(node) {
            owners.insert(owner);
        }
        pending.extend(node.children.iter());
    }
    owners
}

fn find(node: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if node.id == id {
        return Some(node);
    }
    node.children.iter().find_map(|child| find(child, id))
}
