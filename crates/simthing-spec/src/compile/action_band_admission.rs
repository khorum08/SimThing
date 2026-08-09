//! One session-build ActionBand template admission door.
//!
//! The product is descriptor data only. It binds existing columns, EML trees,
//! threshold registrations, and emission-table rows; it does not execute or
//! re-evaluate any numerical ActionBand decision.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    ColumnIndex, DimensionRegistry, EmitOnThresholdRegistration, EmlConsumerKind,
    EmlExpressionRegistry, EmlTreeId,
};
use thiserror::Error;

use crate::spec::action_band::{
    ActionBandAdmissionBudgetSpec, ActionBandBandSpec, ActionBandChannelKind,
    ActionBandRequirementSemantics, ActionBandSessionSpec, ActionBandTargetSpec,
    ActionBandTemplateSpec,
};

/// The sole stateful session-build door. Once admitted, the product cannot be
/// replaced or widened through this session object.
#[derive(Debug, Default)]
pub struct ActionBandSessionBuildDoor {
    product: Option<FrozenActionBandTemplates>,
}

impl ActionBandSessionBuildDoor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate and freeze all ActionBand templates for one session.
    pub fn admit_once_at_session_build(
        &mut self,
        spec: &ActionBandSessionSpec,
        registry: &DimensionRegistry,
        eml_registry: &EmlExpressionRegistry,
        threshold_registrations: &[EmitOnThresholdRegistration],
    ) -> Result<&FrozenActionBandTemplates, ActionBandAdmissionError> {
        if let Some(existing) = &self.product {
            return Err(ActionBandAdmissionError::MidSessionTemplateMintRefused {
                admitted: existing.templates.len(),
                attempted: spec.templates.len(),
            });
        }

        let product =
            compile_frozen_product(spec, registry, eml_registry, threshold_registrations)?;
        self.product = Some(product);
        Ok(self.product.as_ref().expect("just inserted"))
    }

    pub fn product(&self) -> Option<&FrozenActionBandTemplates> {
        self.product.as_ref()
    }
}

/// Stable numeric template identity in the frozen table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActionBandTemplateIndex(u32);

impl ActionBandTemplateIndex {
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Stable span into one of the product's flat side tables.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActionBandTableSpan {
    start: u32,
    len: u32,
}

impl ActionBandTableSpan {
    pub fn start(self) -> u32 {
        self.start
    }

    pub fn len(self) -> u32 {
        self.len
    }

    pub fn is_empty(self) -> bool {
        self.len == 0
    }
}

/// Index into the already-existing threshold registration table. There is no
/// ActionBand-local comparator, crossing record, or constructor for this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExistingThresholdRegistrationIndex(u32);

impl ExistingThresholdRegistrationIndex {
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Index into a pre-admitted generic emission-binding table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PreAdmittedEmissionBindingIndex(u32);

impl PreAdmittedEmissionBindingIndex {
    pub fn raw(self) -> u32 {
        self.0
    }
}

/// Immutable numeric template descriptor consumed by later GPU lowering.
#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedActionBandTemplate {
    index: ActionBandTemplateIndex,
    target: AdmittedActionBandTarget,
    velocity: Option<AdmittedActionBandVelocity>,
    channel_span: ActionBandTableSpan,
    band_span: ActionBandTableSpan,
    dependency_span: ActionBandTableSpan,
    max_active_subordinates: u32,
    reserved_instance_rows: u32,
}

impl AdmittedActionBandTemplate {
    pub fn index(&self) -> ActionBandTemplateIndex {
        self.index
    }

    pub fn target(&self) -> &AdmittedActionBandTarget {
        &self.target
    }

    pub fn velocity(&self) -> Option<AdmittedActionBandVelocity> {
        self.velocity
    }

    pub fn channel_span(&self) -> ActionBandTableSpan {
        self.channel_span
    }

    pub fn band_span(&self) -> ActionBandTableSpan {
        self.band_span
    }

    pub fn dependency_span(&self) -> ActionBandTableSpan {
        self.dependency_span
    }

    pub fn max_active_subordinates(&self) -> u32 {
        self.max_active_subordinates
    }

    pub fn reserved_instance_rows(&self) -> u32 {
        self.reserved_instance_rows
    }
}

/// Frozen binding for an admitted previous-plane velocity observable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedActionBandVelocity {
    current_channel: ColumnIndex,
    previous_generation_channel: ColumnIndex,
}

impl AdmittedActionBandVelocity {
    pub fn current_channel(self) -> ColumnIndex {
        self.current_channel
    }

    pub fn previous_generation_channel(self) -> ColumnIndex {
        self.previous_generation_channel
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedActionBandChannel {
    column: ColumnIndex,
    kind: ActionBandChannelKind,
}

impl AdmittedActionBandChannel {
    pub fn column(self) -> ColumnIndex {
        self.column
    }

    pub fn kind(self) -> ActionBandChannelKind {
        self.kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedActionBandBand {
    threshold_registration: ExistingThresholdRegistrationIndex,
    eml_program: Option<EmlTreeId>,
    emission_binding_span: ActionBandTableSpan,
}

impl AdmittedActionBandBand {
    pub fn threshold_registration(&self) -> ExistingThresholdRegistrationIndex {
        self.threshold_registration
    }

    pub fn eml_program(&self) -> Option<EmlTreeId> {
        self.eml_program
    }

    pub fn emission_binding_span(&self) -> ActionBandTableSpan {
        self.emission_binding_span
    }
}

/// Closed, total GPU-lowering descriptors. No predicate-only variant exists.
#[derive(Clone, Debug, PartialEq)]
pub enum AdmittedActionBandTarget {
    Point {
        current_channels: Vec<ColumnIndex>,
        target: Vec<f32>,
    },
    ScalarBound {
        channel: ColumnIndex,
        bound: f32,
        direction: crate::spec::action_band::ScalarBoundDirection,
    },
    Interval {
        channel: ColumnIndex,
        lo: f32,
        hi: f32,
    },
    AxisAlignedBox {
        channels: Vec<ColumnIndex>,
        lo: Vec<f32>,
        hi: Vec<f32>,
    },
    LocusRadius {
        distance_channel: ColumnIndex,
        radius: f32,
    },
    PalmaReachableSet {
        distance_channel: ColumnIndex,
        maximum_distance: f32,
    },
    EmlProjectedSet {
        input_channels: Vec<ColumnIndex>,
        membership_program: EmlTreeId,
        projection_program: EmlTreeId,
        projection_width: u32,
    },
}

/// Human-readable authoring metadata, physically separate from numeric tables.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBandSemanticShadow {
    template: ActionBandTemplateIndex,
    authored_id: String,
    label: Option<String>,
}

impl ActionBandSemanticShadow {
    pub fn template(&self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn authored_id(&self) -> &str {
        &self.authored_id
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Metadata joining a sealed crossing's existing registration index to an
/// opaque ActionBand template/band. This carries no crossing authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActionBandCrossingBinding {
    threshold_registration: ExistingThresholdRegistrationIndex,
    template: ActionBandTemplateIndex,
    band_table_index: u32,
}

impl ActionBandCrossingBinding {
    pub fn threshold_registration(self) -> ExistingThresholdRegistrationIndex {
        self.threshold_registration
    }

    pub fn template(self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn band_table_index(self) -> u32 {
        self.band_table_index
    }
}

/// Frozen session product. All tables are private and expose read-only slices.
#[derive(Clone, Debug, PartialEq)]
pub struct FrozenActionBandTemplates {
    budget: ActionBandAdmissionBudgetSpec,
    templates: Vec<AdmittedActionBandTemplate>,
    channels: Vec<AdmittedActionBandChannel>,
    bands: Vec<AdmittedActionBandBand>,
    dependencies: Vec<ActionBandTemplateIndex>,
    emission_bindings: Vec<PreAdmittedEmissionBindingIndex>,
    crossing_bindings: Vec<ActionBandCrossingBinding>,
    semantic_shadow: Vec<ActionBandSemanticShadow>,
}

impl FrozenActionBandTemplates {
    pub fn budget(&self) -> ActionBandAdmissionBudgetSpec {
        self.budget
    }

    pub fn templates(&self) -> &[AdmittedActionBandTemplate] {
        &self.templates
    }

    pub fn channels(&self) -> &[AdmittedActionBandChannel] {
        &self.channels
    }

    pub fn bands(&self) -> &[AdmittedActionBandBand] {
        &self.bands
    }

    pub fn dependencies(&self) -> &[ActionBandTemplateIndex] {
        &self.dependencies
    }

    pub fn emission_bindings(&self) -> &[PreAdmittedEmissionBindingIndex] {
        &self.emission_bindings
    }

    pub fn semantic_shadow(&self) -> &[ActionBandSemanticShadow] {
        &self.semantic_shadow
    }

    /// Resolve metadata only after the caller has obtained an existing sealed
    /// crossing and read its `reg_idx`. Supplying an integer here cannot mint a
    /// crossing or authorize an effect.
    pub fn bindings_for_existing_threshold(
        &self,
        threshold_registration_index: u32,
    ) -> impl Iterator<Item = &ActionBandCrossingBinding> {
        self.crossing_bindings.iter().filter(move |binding| {
            binding.threshold_registration.raw() == threshold_registration_index
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ActionBandAdmissionError {
    #[error("mid-session ActionBand template mint refused: {admitted} templates are already frozen; attempted {attempted}")]
    MidSessionTemplateMintRefused { admitted: usize, attempted: usize },
    #[error("ActionBand template row {template_index} has an empty authored id")]
    EmptyTemplateId { template_index: usize },
    #[error("ActionBand template rows {first_template_index} and {template_index} both use authored id `{id}`")]
    DuplicateTemplateId {
        first_template_index: usize,
        template_index: usize,
        id: String,
    },
    #[error("ActionBand template `{template_id}` references unknown subordinate template `{dependency_id}`")]
    UnknownDependency {
        template_id: String,
        dependency_id: String,
    },
    #[error("ActionBand template `{template_id}` declares max_active_subordinates={declared}, but its admitted dependency span is {span}")]
    MaxActiveSubordinatesExceedsSpan {
        template_id: String,
        declared: u32,
        span: usize,
    },
    #[error("ActionBand dependency bindings {computed} exceed admitted budget {declared}")]
    DependencyBudgetExceeded { declared: u32, computed: usize },
    #[error("ActionBand distinct axis/channels {computed} exceed admitted budget {declared}")]
    AxisChannelBudgetExceeded { declared: u32, computed: usize },
    #[error("ActionBand template `{template_id}` repeats axis channel {column}")]
    DuplicateTemplateChannel { template_id: String, column: u32 },
    #[error("ActionBand template `{template_id}` references column {column} outside admitted registry width {bound}")]
    UnknownColumn {
        template_id: String,
        column: u32,
        bound: u32,
    },
    #[error("ActionBand template `{template_id}` references column {column}, which is not a live Anchored property column")]
    UnanchoredColumn { template_id: String, column: u32 },
    #[error("ActionBand template `{template_id}` uses column {column} without declaring it in its axis/channel span")]
    ChannelOutsideTemplateSpan { template_id: String, column: u32 },
    #[error("ActionBand storage rows {computed} exceed admitted budget {declared}")]
    StorageBudgetExceeded { declared: u32, computed: u64 },
    #[error("ActionBand template `{template_id}` target is invalid: {reason}")]
    InvalidTarget { template_id: String, reason: String },
    #[error("ActionBand template `{template_id}` EmlProjectedSet is predicate-only; a projection/distance program is required")]
    PredicateOnlyTarget { template_id: String },
    #[error("ActionBand template `{template_id}` requests velocity without an explicitly admitted previous-generation plane")]
    PreviousGenerationPlaneRequired { template_id: String },
    #[error("ActionBand template `{template_id}` references unknown EML program {tree_id}")]
    UnknownEmlProgram { template_id: String, tree_id: u32 },
    #[error(
        "ActionBand template `{template_id}` EML program {tree_id} is not admitted for {consumer}"
    )]
    EmlProgramNotAdmissible {
        template_id: String,
        tree_id: u32,
        consumer: &'static str,
    },
    #[error("ActionBand distinct EML programs {computed} exceed admitted budget {declared}")]
    EmlProgramBudgetExceeded { declared: u32, computed: usize },
    #[error("ActionBand template `{template_id}` band {band_index} references threshold registration {registration_index}, but only {available} exist")]
    UnknownThresholdRegistration {
        template_id: String,
        band_index: usize,
        registration_index: u32,
        available: usize,
    },
    #[error("ActionBand template `{template_id}` band {band_index} binds threshold registration {registration_index} on a non-Anchored column")]
    ThresholdRegistrationNotAnchored {
        template_id: String,
        band_index: usize,
        registration_index: u32,
    },
    #[error("ActionBand template `{template_id}` band {band_index} references emission binding {binding_index}, but admitted table width is {available}")]
    UnknownEmissionBinding {
        template_id: String,
        band_index: usize,
        binding_index: u32,
        available: u32,
    },
    #[error("ActionBand template `{template_id}` requires pre-8.x scarce-lane semantics `{requirement}`; admission fails/defer-closes")]
    Pre8xScarceLaneSemanticsUnsupported {
        template_id: String,
        requirement: &'static str,
    },
    #[error("ActionBand table width exceeds the u32 GPU index space")]
    TableWidthOverflow,
}

fn compile_frozen_product(
    spec: &ActionBandSessionSpec,
    registry: &DimensionRegistry,
    eml_registry: &EmlExpressionRegistry,
    threshold_registrations: &[EmitOnThresholdRegistration],
) -> Result<FrozenActionBandTemplates, ActionBandAdmissionError> {
    let mut template_ids = BTreeMap::<&str, (usize, ActionBandTemplateIndex)>::new();
    for (template_index, template) in spec.templates.iter().enumerate() {
        if template.id.trim().is_empty() {
            return Err(ActionBandAdmissionError::EmptyTemplateId { template_index });
        }
        let index = ActionBandTemplateIndex(to_u32(template_index)?);
        if let Some((first_template_index, _)) =
            template_ids.insert(&template.id, (template_index, index))
        {
            return Err(ActionBandAdmissionError::DuplicateTemplateId {
                first_template_index,
                template_index,
                id: template.id.clone(),
            });
        }
    }

    let registry_width = u32::try_from(registry.total_columns)
        .map_err(|_| ActionBandAdmissionError::TableWidthOverflow)?;
    let mut templates = Vec::with_capacity(spec.templates.len());
    let mut channels = Vec::new();
    let mut bands = Vec::new();
    let mut dependencies = Vec::new();
    let mut emission_bindings = Vec::new();
    let mut crossing_bindings = Vec::new();
    let mut semantic_shadow = Vec::with_capacity(spec.templates.len());
    let mut session_channels = BTreeSet::<u32>::new();
    let mut session_eml_programs = BTreeSet::<u32>::new();
    let mut storage_rows = 0u64;

    for (template_index, template) in spec.templates.iter().enumerate() {
        reject_deferred_requirement(template)?;
        let index = ActionBandTemplateIndex(to_u32(template_index)?);
        storage_rows = storage_rows
            .checked_add(u64::from(template.reserved_instance_rows))
            .ok_or(ActionBandAdmissionError::TableWidthOverflow)?;
        if storage_rows > u64::from(spec.budget.storage_rows) {
            return Err(ActionBandAdmissionError::StorageBudgetExceeded {
                declared: spec.budget.storage_rows,
                computed: storage_rows,
            });
        }

        let channel_start = to_u32(channels.len())?;
        let mut declared_channels = BTreeSet::<u32>::new();
        for channel in &template.axis_channels {
            if !declared_channels.insert(channel.column) {
                return Err(ActionBandAdmissionError::DuplicateTemplateChannel {
                    template_id: template.id.clone(),
                    column: channel.column,
                });
            }
            let column = seal_anchored_column(template, channel.column, registry, registry_width)?;
            session_channels.insert(channel.column);
            channels.push(AdmittedActionBandChannel {
                column,
                kind: channel.kind,
            });
        }
        if session_channels.len() > spec.budget.axis_channel_count as usize {
            return Err(ActionBandAdmissionError::AxisChannelBudgetExceeded {
                declared: spec.budget.axis_channel_count,
                computed: session_channels.len(),
            });
        }

        let target = compile_target(
            template,
            &declared_channels,
            registry,
            registry_width,
            eml_registry,
            &mut session_eml_programs,
        )?;
        let velocity = compile_velocity(template, &declared_channels, registry, registry_width)?;

        let band_start = to_u32(bands.len())?;
        for (band_index, band) in template.bands.iter().enumerate() {
            let admitted_band = compile_band(
                template,
                band_index,
                band,
                &declared_channels,
                registry,
                threshold_registrations,
                eml_registry,
                spec.budget.emission_binding_count,
                &mut session_eml_programs,
                &mut emission_bindings,
            )?;
            let flat_band_index = to_u32(bands.len())?;
            crossing_bindings.push(ActionBandCrossingBinding {
                threshold_registration: admitted_band.threshold_registration,
                template: index,
                band_table_index: flat_band_index,
            });
            bands.push(admitted_band);
        }

        let dependency_start = to_u32(dependencies.len())?;
        for dependency_id in &template.subordinate_template_ids {
            let Some(&(_, dependency_index)) = template_ids.get(dependency_id.as_str()) else {
                return Err(ActionBandAdmissionError::UnknownDependency {
                    template_id: template.id.clone(),
                    dependency_id: dependency_id.clone(),
                });
            };
            dependencies.push(dependency_index);
        }
        if template.max_active_subordinates > template.subordinate_template_ids.len() as u32 {
            return Err(ActionBandAdmissionError::MaxActiveSubordinatesExceedsSpan {
                template_id: template.id.clone(),
                declared: template.max_active_subordinates,
                span: template.subordinate_template_ids.len(),
            });
        }
        if dependencies.len() > spec.budget.dependency_binding_count as usize {
            return Err(ActionBandAdmissionError::DependencyBudgetExceeded {
                declared: spec.budget.dependency_binding_count,
                computed: dependencies.len(),
            });
        }

        templates.push(AdmittedActionBandTemplate {
            index,
            target,
            velocity,
            channel_span: span(channel_start, channels.len())?,
            band_span: span(band_start, bands.len())?,
            dependency_span: span(dependency_start, dependencies.len())?,
            max_active_subordinates: template.max_active_subordinates,
            reserved_instance_rows: template.reserved_instance_rows,
        });
        semantic_shadow.push(ActionBandSemanticShadow {
            template: index,
            authored_id: template.id.clone(),
            label: template.label.clone(),
        });
    }

    if session_eml_programs.len() > spec.budget.eml_program_count as usize {
        return Err(ActionBandAdmissionError::EmlProgramBudgetExceeded {
            declared: spec.budget.eml_program_count,
            computed: session_eml_programs.len(),
        });
    }

    Ok(FrozenActionBandTemplates {
        budget: spec.budget,
        templates,
        channels,
        bands,
        dependencies,
        emission_bindings,
        crossing_bindings,
        semantic_shadow,
    })
}

fn reject_deferred_requirement(
    template: &ActionBandTemplateSpec,
) -> Result<(), ActionBandAdmissionError> {
    let requirement = match template.requirement_semantics {
        ActionBandRequirementSemantics::Ordinary => return Ok(()),
        ActionBandRequirementSemantics::AtomicCommonDepthCommitment => {
            "atomic-common-depth-commitment"
        }
        ActionBandRequirementSemantics::PersistentScarceGrantHolding => {
            "persistent-scarce-grant-holding"
        }
    };
    Err(
        ActionBandAdmissionError::Pre8xScarceLaneSemanticsUnsupported {
            template_id: template.id.clone(),
            requirement,
        },
    )
}

fn compile_target(
    template: &ActionBandTemplateSpec,
    declared_channels: &BTreeSet<u32>,
    registry: &DimensionRegistry,
    registry_width: u32,
    eml_registry: &EmlExpressionRegistry,
    session_eml_programs: &mut BTreeSet<u32>,
) -> Result<AdmittedActionBandTarget, ActionBandAdmissionError> {
    let invalid = |reason: &str| ActionBandAdmissionError::InvalidTarget {
        template_id: template.id.clone(),
        reason: reason.into(),
    };
    let channel = |raw: u32| {
        require_declared_column(template, raw, declared_channels)?;
        seal_anchored_column(template, raw, registry, registry_width)
    };

    Ok(match &template.target {
        ActionBandTargetSpec::Point {
            current_channels,
            target,
        } => {
            if current_channels.is_empty() || current_channels.len() != target.len() {
                return Err(invalid(
                    "Point requires equal non-zero channel and target widths",
                ));
            }
            if target.iter().any(|value| !value.is_finite()) {
                return Err(invalid("Point target values must be finite"));
            }
            AdmittedActionBandTarget::Point {
                current_channels: current_channels
                    .iter()
                    .map(|raw| channel(*raw))
                    .collect::<Result<_, _>>()?,
                target: target.clone(),
            }
        }
        ActionBandTargetSpec::ScalarBound {
            channel: raw,
            bound,
            direction,
        } => {
            if !bound.is_finite() {
                return Err(invalid("ScalarBound bound must be finite"));
            }
            AdmittedActionBandTarget::ScalarBound {
                channel: channel(*raw)?,
                bound: *bound,
                direction: *direction,
            }
        }
        ActionBandTargetSpec::Interval {
            channel: raw,
            lo,
            hi,
        } => {
            if !lo.is_finite() || !hi.is_finite() || lo > hi {
                return Err(invalid("Interval requires finite lo <= hi"));
            }
            AdmittedActionBandTarget::Interval {
                channel: channel(*raw)?,
                lo: *lo,
                hi: *hi,
            }
        }
        ActionBandTargetSpec::AxisAlignedBox { channels, lo, hi } => {
            if channels.is_empty() || channels.len() != lo.len() || lo.len() != hi.len() {
                return Err(invalid(
                    "AxisAlignedBox requires equal non-zero channel/lo/hi widths",
                ));
            }
            if lo
                .iter()
                .zip(hi)
                .any(|(lo, hi)| !lo.is_finite() || !hi.is_finite() || lo > hi)
            {
                return Err(invalid(
                    "AxisAlignedBox requires finite componentwise lo <= hi",
                ));
            }
            AdmittedActionBandTarget::AxisAlignedBox {
                channels: channels
                    .iter()
                    .map(|raw| channel(*raw))
                    .collect::<Result<_, _>>()?,
                lo: lo.clone(),
                hi: hi.clone(),
            }
        }
        ActionBandTargetSpec::LocusRadius {
            distance_channel,
            radius,
        } => {
            if !radius.is_finite() || *radius < 0.0 {
                return Err(invalid("LocusRadius requires a finite non-negative radius"));
            }
            AdmittedActionBandTarget::LocusRadius {
                distance_channel: channel(*distance_channel)?,
                radius: *radius,
            }
        }
        ActionBandTargetSpec::PalmaReachableSet {
            distance_channel,
            maximum_distance,
        } => {
            if !maximum_distance.is_finite() || *maximum_distance < 0.0 {
                return Err(invalid(
                    "PalmaReachableSet requires a finite non-negative maximum distance",
                ));
            }
            AdmittedActionBandTarget::PalmaReachableSet {
                distance_channel: channel(*distance_channel)?,
                maximum_distance: *maximum_distance,
            }
        }
        ActionBandTargetSpec::EmlProjectedSet {
            input_channels,
            membership_program,
            projection_program,
            projection_width,
        } => {
            if input_channels.is_empty() || *projection_width == 0 {
                return Err(invalid(
                    "EmlProjectedSet requires input channels and non-zero projection width",
                ));
            }
            let Some(projection_program) = projection_program else {
                return Err(ActionBandAdmissionError::PredicateOnlyTarget {
                    template_id: template.id.clone(),
                });
            };
            let membership = admit_eml_program(
                template,
                *membership_program,
                EmlConsumerKind::HardThreshold,
                "hard-threshold membership",
                eml_registry,
                session_eml_programs,
            )?;
            let projection = admit_eml_program(
                template,
                *projection_program,
                EmlConsumerKind::Emission,
                "projection/emission",
                eml_registry,
                session_eml_programs,
            )?;
            AdmittedActionBandTarget::EmlProjectedSet {
                input_channels: input_channels
                    .iter()
                    .map(|raw| channel(*raw))
                    .collect::<Result<_, _>>()?,
                membership_program: membership,
                projection_program: projection,
                projection_width: *projection_width,
            }
        }
    })
}

fn compile_velocity(
    template: &ActionBandTemplateSpec,
    declared_channels: &BTreeSet<u32>,
    registry: &DimensionRegistry,
    registry_width: u32,
) -> Result<Option<AdmittedActionBandVelocity>, ActionBandAdmissionError> {
    let Some(velocity) = template.velocity else {
        return Ok(None);
    };
    let Some(previous) = velocity.previous_generation_channel else {
        return Err(ActionBandAdmissionError::PreviousGenerationPlaneRequired {
            template_id: template.id.clone(),
        });
    };
    require_declared_column(template, velocity.current_channel, declared_channels)?;
    require_declared_column(template, previous, declared_channels)?;
    let current_channel =
        seal_anchored_column(template, velocity.current_channel, registry, registry_width)?;
    let previous_generation_channel =
        seal_anchored_column(template, previous, registry, registry_width)?;
    Ok(Some(AdmittedActionBandVelocity {
        current_channel,
        previous_generation_channel,
    }))
}

#[allow(clippy::too_many_arguments)]
fn compile_band(
    template: &ActionBandTemplateSpec,
    band_index: usize,
    band: &ActionBandBandSpec,
    declared_channels: &BTreeSet<u32>,
    registry: &DimensionRegistry,
    threshold_registrations: &[EmitOnThresholdRegistration],
    eml_registry: &EmlExpressionRegistry,
    emission_binding_count: u32,
    session_eml_programs: &mut BTreeSet<u32>,
    emission_bindings: &mut Vec<PreAdmittedEmissionBindingIndex>,
) -> Result<AdmittedActionBandBand, ActionBandAdmissionError> {
    let Some(registration) =
        threshold_registrations.get(band.threshold_registration_index as usize)
    else {
        return Err(ActionBandAdmissionError::UnknownThresholdRegistration {
            template_id: template.id.clone(),
            band_index,
            registration_index: band.threshold_registration_index,
            available: threshold_registrations.len(),
        });
    };
    let registration_column = registration.col.raw_u32();
    if !declared_channels.contains(&registration_column) {
        return Err(ActionBandAdmissionError::ChannelOutsideTemplateSpan {
            template_id: template.id.clone(),
            column: registration_column,
        });
    }
    if !is_live_anchored_column(registry, registration.col) {
        return Err(ActionBandAdmissionError::ThresholdRegistrationNotAnchored {
            template_id: template.id.clone(),
            band_index,
            registration_index: band.threshold_registration_index,
        });
    }

    let eml_program = band
        .eml_program
        .map(|tree_id| {
            admit_eml_program(
                template,
                tree_id,
                EmlConsumerKind::Emission,
                "emission",
                eml_registry,
                session_eml_programs,
            )
        })
        .transpose()?;

    let emission_start = to_u32(emission_bindings.len())?;
    for binding_index in &band.emission_binding_indices {
        if *binding_index >= emission_binding_count {
            return Err(ActionBandAdmissionError::UnknownEmissionBinding {
                template_id: template.id.clone(),
                band_index,
                binding_index: *binding_index,
                available: emission_binding_count,
            });
        }
        emission_bindings.push(PreAdmittedEmissionBindingIndex(*binding_index));
    }

    Ok(AdmittedActionBandBand {
        threshold_registration: ExistingThresholdRegistrationIndex(
            band.threshold_registration_index,
        ),
        eml_program,
        emission_binding_span: span(emission_start, emission_bindings.len())?,
    })
}

fn admit_eml_program(
    template: &ActionBandTemplateSpec,
    raw: u32,
    consumer: EmlConsumerKind,
    consumer_name: &'static str,
    eml_registry: &EmlExpressionRegistry,
    session_eml_programs: &mut BTreeSet<u32>,
) -> Result<EmlTreeId, ActionBandAdmissionError> {
    let tree_id = EmlTreeId(raw);
    if eml_registry.get(tree_id).is_none() {
        return Err(ActionBandAdmissionError::UnknownEmlProgram {
            template_id: template.id.clone(),
            tree_id: raw,
        });
    }
    if eml_registry
        .assert_consumer_admissible(tree_id, consumer)
        .is_err()
    {
        return Err(ActionBandAdmissionError::EmlProgramNotAdmissible {
            template_id: template.id.clone(),
            tree_id: raw,
            consumer: consumer_name,
        });
    }
    session_eml_programs.insert(raw);
    Ok(tree_id)
}

fn require_declared_column(
    template: &ActionBandTemplateSpec,
    raw: u32,
    declared_channels: &BTreeSet<u32>,
) -> Result<(), ActionBandAdmissionError> {
    if declared_channels.contains(&raw) {
        Ok(())
    } else {
        Err(ActionBandAdmissionError::ChannelOutsideTemplateSpan {
            template_id: template.id.clone(),
            column: raw,
        })
    }
}

fn seal_anchored_column(
    template: &ActionBandTemplateSpec,
    raw: u32,
    registry: &DimensionRegistry,
    registry_width: u32,
) -> Result<ColumnIndex, ActionBandAdmissionError> {
    let column = ColumnIndex::try_from_admitted_authored(raw, registry_width).map_err(|_| {
        ActionBandAdmissionError::UnknownColumn {
            template_id: template.id.clone(),
            column: raw,
            bound: registry_width,
        }
    })?;
    if !is_live_anchored_column(registry, column) {
        return Err(ActionBandAdmissionError::UnanchoredColumn {
            template_id: template.id.clone(),
            column: raw,
        });
    }
    Ok(column)
}

fn is_live_anchored_column(registry: &DimensionRegistry, column: ColumnIndex) -> bool {
    let Some(&(property_id, _)) = registry.column_owners.get(column.raw()) else {
        return false;
    };
    registry.is_active(property_id)
        && registry
            .try_property(property_id)
            .is_some_and(|property| property.admission_disposition.is_anchored())
}

fn span(start: u32, end: usize) -> Result<ActionBandTableSpan, ActionBandAdmissionError> {
    let end = to_u32(end)?;
    Ok(ActionBandTableSpan {
        start,
        len: end
            .checked_sub(start)
            .ok_or(ActionBandAdmissionError::TableWidthOverflow)?,
    })
}

fn to_u32(value: usize) -> Result<u32, ActionBandAdmissionError> {
    u32::try_from(value).map_err(|_| ActionBandAdmissionError::TableWidthOverflow)
}
