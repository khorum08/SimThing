//! Property layouts, sub-field roles, and column-backed values.
//!
//! Ordinary code must not index `PropertyValue` lanes by raw integer — use
//! [`PropertyLayout::offset_of`] and role/column accessors instead.
//!
//! Direct field indexing is forbidden:
//!
//! ```compile_fail
//! use simthing_core::{PropertyLayout, PropertyValue};
//! let layout = PropertyLayout::standard(0);
//! let value = PropertyValue::from_layout(&layout);
//! let _ = value.data[0];
//! ```
//!
//! Bare integer lane indices are forbidden:
//!
//! ```compile_fail
//! use simthing_core::{PropertyLayout, PropertyValue};
//! let layout = PropertyLayout::standard(0);
//! let mut value = PropertyValue::from_layout(&layout);
//! value.set_lane_at_offset(0, 1.0);
//! ```
//!
//! `RoleOffset` cannot be forged from a bare integer:
//!
//! ```compile_fail
//! use simthing_core::RoleOffset;
//! let _off: RoleOffset = 0usize;
//! ```

use crate::accumulator_op::SoftAggregateGuard;
use crate::ids::SimPropertyId;
use crate::reduction::ReductionRule;
use serde::{Deserialize, Serialize};

// ── ClampBehavior ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClampBehavior {
    /// Hard floor and ceiling. For normalized 0–1 state properties.
    Bounded { min: f32, max: f32 },
    /// Floor only. For population, capacity, output — unbounded upward.
    Floored { min: f32 },
    /// No clamping. For signed pressures, vector components, aggregates.
    Unbounded,
}

impl ClampBehavior {
    pub fn apply(&self, value: f32) -> f32 {
        match self {
            Self::Bounded { min, max } => value.clamp(*min, *max),
            Self::Floored { min } => value.max(*min),
            Self::Unbounded => value,
        }
    }

    pub fn at_floor(&self, value: f32) -> bool {
        match self {
            Self::Bounded { min, .. } => value <= *min,
            Self::Floored { min } => value <= *min,
            Self::Unbounded => false,
        }
    }

    pub fn at_ceiling(&self, value: f32) -> bool {
        match self {
            Self::Bounded { max, .. } => value >= *max,
            _ => false,
        }
    }
}

// ── SubFieldRole ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubFieldRole {
    /// The primary scalar value on the property's spectrum.
    Amount,
    /// Rate of change per tick for a governed sub-field.
    Velocity,
    /// Expression strength (0–1). Drives fission secondary conditions.
    Intensity,
    /// Designer-named sub-field. Used for everything beyond the standard three:
    /// vector components, bonus vectors, population proportions, etc.
    Named(String),
    /// Mod-defined role. Evaluator treats as generic float.
    Custom(String),
}

// ── TransformOp ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransformOp {
    Add(f32),
    Multiply(f32),
    Set(f32),
}

impl TransformOp {
    pub fn apply(&self, current: f32) -> f32 {
        match self {
            Self::Add(v) => current + v,
            Self::Multiply(v) => current * v,
            Self::Set(v) => *v,
        }
    }
}

// ── SubFieldSpec ──────────────────────────────────────────────────────────────

/// Declares the semantics, clamping, and integration behavior of one
/// contiguous block of floats within a PropertyValue data vec.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubFieldSpec {
    /// Semantic role. Used by overlay transforms and the semantic layer to
    /// reference this sub-field by name rather than by index.
    pub role: SubFieldRole,

    /// Number of consecutive floats this sub-field occupies in the data vec.
    /// 1 = scalar, N > 1 = vector of N components.
    pub width: usize,

    /// Clamping applied after integration and after transform application.
    pub clamp: ClampBehavior,

    /// Optional cap on |velocity| before integration. None = uncapped.
    pub velocity_max: Option<f32>,

    /// Default value for each float in this sub-field at creation.
    pub default: f32,

    /// Human-readable name for UI display and observability tooling.
    pub display_name: String,

    /// Optional range hint for UI scaling. Does not affect simulation.
    pub display_range: Option<(f32, f32)>,

    /// Which sub-field role governs this one's rate of change.
    /// None  = this sub-field is not evolved by integration.
    /// Some  = integrate this sub-field using the named role's value as velocity.
    ///
    /// Example: Named("axis_position") governed_by Some(Named("axis_drift"))
    /// means axis_position advances by axis_drift * delta_time each tick.
    pub governed_by: Option<SubFieldRole>,

    /// Override the default reduction rule for this sub-field. When `None`,
    /// the rule is derived from `role` via `ReductionRule::default_for_role`.
    /// Reduction aggregates children's column values into the parent at the
    /// presentation tier (GPU Passes 4–6 / CPU reduction oracle).
    pub reduction_override: Option<ReductionRule>,

    /// Tolerance policy for soft-aggregate reductions. Required (must be
    /// `Some(Quantized { .. })` or `Some(Hysteresis { .. })`) when this
    /// sub-field's resolved reduction is `Mean` or `WeightedMean` AND the
    /// sub-field is registered as a hard structural threshold reading from
    /// the post-reduction buffer (`THRESH_BUF_OUTPUT`). `None` and
    /// `Some(Unguarded)` are equivalent: no guard applied.
    ///
    /// Enforced at threshold-registration time by
    /// `simthing_sim::threshold_registry::assert_no_hard_trigger_on_soft_aggregate`.
    /// Today no production threshold path satisfies the "post-reduction + hard
    /// trigger" combination, so this field is forward-protecting — it gates
    /// the C-5 WeightedMean migration and E0 economic substrate before they
    /// land. See `docs/workshop/soft_aggregate_tolerance_audit.md`.
    #[serde(default)]
    pub soft_aggregate_guard: Option<SoftAggregateGuard>,

    /// Resource Flow compile-time metadata (E-8). None for non-resource sub-fields.
    /// Compiles away before GPU upload; must not drive runtime branching in
    /// `simthing-sim`.
    #[serde(default)]
    pub accumulator_spec: Option<crate::accumulator_spec::AccumulatorSpec>,
}

impl SubFieldSpec {
    /// Resolve the reduction rule for this sub-field: override if set,
    /// otherwise the role's default.
    pub fn resolved_reduction(&self) -> ReductionRule {
        self.reduction_override
            .unwrap_or_else(|| ReductionRule::default_for_role(&self.role))
    }
}

// ── PropertyLayout ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyLayout {
    pub sub_fields: Vec<SubFieldSpec>,
}

impl PropertyLayout {
    /// Total GPU columns required = sum of all sub-field widths.
    pub fn stride(&self) -> usize {
        self.sub_fields.iter().map(|sf| sf.width).sum()
    }

    /// Local byte offset (index into data vec) of the first float in a
    /// sub-field with the given role. Returns None if role not present.
    pub fn offset_of(&self, role: &SubFieldRole) -> Option<RoleOffset> {
        let mut offset = 0;
        for sf in &self.sub_fields {
            if &sf.role == role {
                return Some(RoleOffset(offset));
            }
            offset += sf.width;
        }
        None
    }

    /// Width of a sub-field with the given role.
    pub fn width_of(&self, role: &SubFieldRole) -> Option<usize> {
        self.sub_fields
            .iter()
            .find(|sf| &sf.role == role)
            .map(|sf| sf.width)
    }

    /// Default data vec for a fresh PropertyValue of this layout.
    /// Each sub-field's floats are initialized to SubFieldSpec::default.
    pub fn default_data(&self) -> Vec<f32> {
        self.sub_fields
            .iter()
            .flat_map(|sf| std::iter::repeat(sf.default).take(sf.width))
            .collect()
    }

    /// Build the standard amount + velocity + intensity + N named vec layout.
    ///
    /// Clamping: amount and intensity Bounded(0,1), velocity Bounded(-1,1),
    /// vec components Unbounded.
    /// Integration: amount governed by velocity; intensity updated by
    /// IntensityBehavior separately (governed_by: None here).
    pub fn standard(vector_len: usize) -> Self {
        let mut sub_fields = vec![
            SubFieldSpec {
                role: SubFieldRole::Amount,
                width: 1,
                clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
                velocity_max: None,
                default: 0.0,
                display_name: "amount".into(),
                display_range: Some((0.0, 1.0)),
                governed_by: Some(SubFieldRole::Velocity),
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Velocity,
                width: 1,
                clamp: ClampBehavior::Bounded {
                    min: -1.0,
                    max: 1.0,
                },
                velocity_max: None,
                default: 0.0,
                display_name: "velocity".into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Intensity,
                width: 1,
                clamp: ClampBehavior::Bounded { min: 0.0, max: 1.0 },
                velocity_max: None,
                default: 0.0,
                display_name: "intensity".into(),
                display_range: Some((0.0, 1.0)),
                governed_by: None, // updated by IntensityBehavior, not integration
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
        ];
        for i in 0..vector_len {
            sub_fields.push(SubFieldSpec {
                role: SubFieldRole::Named(format!("vec_{i}")),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 0.0,
                display_name: format!("vec_{i}"),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            });
        }
        Self { sub_fields }
    }
}

// ── Column access indices (layout-resolved) ───────────────────────────────────

/// Local data-lane index resolved from [`PropertyLayout::offset_of`]. Not
/// constructible from a bare integer — ordinary code must resolve roles through
/// the layout API.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RoleOffset(usize);

impl RoleOffset {
    /// Numeric lane index after layout resolution. For narrow post-resolution
    /// uses (e.g. logging); not for constructing offsets outside `offset_of`.
    pub fn lane(self) -> usize {
        self.0
    }
}

// ── PropertyValue ─────────────────────────────────────────────────────────────

/// Flat float vector for a single property instance on a single SimThing.
/// Layout is defined by the corresponding `SimProperty::layout` in the registry.
/// Indices are never hardcoded outside of `PropertyLayout`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyValue {
    data: Vec<f32>,
}

impl PropertyValue {
    pub fn from_layout(layout: &PropertyLayout) -> Self {
        Self {
            data: layout.default_data(),
        }
    }

    /// Explicit escape hatch for serialization byte-lanes and lossless metadata
    /// encoding. Not for simulation logic — use role/layout accessors instead.
    pub fn from_raw_lanes(data: Vec<f32>) -> Self {
        Self { data }
    }

    /// Read-only view of raw float lanes (serialization / GPU projection only).
    pub fn raw_lanes(&self) -> &[f32] {
        &self.data
    }

    /// Mutable view of raw float lanes (serialization byte-lanes only).
    pub fn raw_lanes_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    /// Alias for callers that copy property bytes into GPU or RON buffers.
    pub fn raw_lanes_for_serialization(&self) -> &[f32] {
        self.raw_lanes()
    }

    pub fn lane_count(&self) -> usize {
        self.data.len()
    }

    /// Scalar read at a layout-resolved offset (`PropertyLayout::offset_of`).
    pub fn lane_at_offset(&self, offset: RoleOffset) -> f32 {
        self.data[offset.0]
    }

    /// Scalar write at a layout-resolved offset.
    pub fn set_lane_at_offset(&mut self, offset: RoleOffset, value: f32) {
        self.data[offset.0] = value;
    }

    /// In-place add at a layout-resolved offset.
    pub fn add_lane_at_offset(&mut self, offset: RoleOffset, delta: f32) {
        self.data[offset.0] += delta;
    }

    /// Read a contiguous lane slice at a layout-resolved offset and width.
    pub fn lanes_at_offset(&self, offset: RoleOffset, width: usize) -> &[f32] {
        let lane = offset.0;
        &self.data[lane..lane + width]
    }

    /// Write a contiguous lane slice at a layout-resolved offset.
    pub fn set_lanes_at_offset(&mut self, offset: RoleOffset, values: &[f32]) {
        let lane = offset.0;
        self.data[lane..lane + values.len()].copy_from_slice(values);
    }

    /// Read a scalar sub-field value by role. Panics if role not found.
    pub fn get_role(&self, role: &SubFieldRole, layout: &PropertyLayout) -> f32 {
        let offset = layout
            .offset_of(role)
            .unwrap_or_else(|| panic!("role {role:?} not in layout"));
        self.lane_at_offset(offset)
    }

    /// Write a scalar sub-field value by role. Panics if role not found.
    pub fn set_role(&mut self, role: &SubFieldRole, layout: &PropertyLayout, value: f32) {
        let offset = layout
            .offset_of(role)
            .unwrap_or_else(|| panic!("role {role:?} not in layout"));
        self.set_lane_at_offset(offset, value);
    }

    /// Read a multi-float sub-field as a slice.
    pub fn get_role_slice(&self, role: &SubFieldRole, layout: &PropertyLayout) -> &[f32] {
        let offset = layout
            .offset_of(role)
            .unwrap_or_else(|| panic!("role {role:?} not in layout"));
        let width = layout.width_of(role).unwrap();
        self.lanes_at_offset(offset, width)
    }

    /// Write a multi-float sub-field from a slice.
    pub fn set_role_slice(&mut self, role: &SubFieldRole, layout: &PropertyLayout, values: &[f32]) {
        let offset = layout
            .offset_of(role)
            .unwrap_or_else(|| panic!("role {role:?} not in layout"));
        let width = layout.width_of(role).unwrap();
        assert_eq!(
            values.len(),
            width,
            "role {role:?} expects width {width}, got {}",
            values.len()
        );
        self.set_lanes_at_offset(offset, values);
    }

    /// Integrate all sub-fields that have a `governed_by` relationship.
    ///
    /// For each governed sub-field:
    ///   1. Read the governing sub-field's current value as velocity.
    ///   2. Optionally clamp it to velocity_max.
    ///   3. Advance: new = current + velocity * dt.
    ///   4. Apply the governed sub-field's ClampBehavior.
    ///   5. Pin the governing velocity to zero in the saturated direction.
    ///      Prevents hidden velocity debt at floor/ceiling — inertia that
    ///      should persist after conditions improve belongs in Named vector
    ///      components (e.g. grievance_inertia), where it is observable.
    pub fn integrate(&mut self, layout: &PropertyLayout, delta_time: f32) {
        // Collect (governed_offset, governing_offset, spec) before mutating data.
        // Both offsets go through layout.offset_of — no raw index arithmetic here. (I1)
        let pairs: Vec<(RoleOffset, RoleOffset, SubFieldSpec)> = layout
            .sub_fields
            .iter()
            .filter_map(|sf| {
                let gov_role = sf.governed_by.as_ref()?;
                let governed_off = layout.offset_of(&sf.role)?;
                let governing_off = layout.offset_of(gov_role)?;
                Some((governed_off, governing_off, sf.clone()))
            })
            .collect();

        for (governed_off, governing_off, spec) in pairs {
            let governed_lane = governed_off.lane();
            let governing_lane = governing_off.lane();
            let raw_vel = self.data[governing_lane];
            let effective_vel = match spec.velocity_max {
                Some(max) => raw_vel.clamp(-max, max),
                None => raw_vel,
            };
            let new_val = self.data[governed_lane] + effective_vel * delta_time;
            let clamped = spec.clamp.apply(new_val);
            self.data[governed_lane] = clamped;

            if spec.clamp.at_floor(clamped) {
                self.data[governing_lane] = self.data[governing_lane].max(0.0);
            } else if spec.clamp.at_ceiling(clamped) {
                self.data[governing_lane] = self.data[governing_lane].min(0.0);
            }
        }
    }

    /// Update intensity sub-field using IntensityBehavior. No-op if layout
    /// has no Intensity or Velocity role.
    pub fn update_intensity(
        &mut self,
        behavior: &IntensityBehavior,
        layout: &PropertyLayout,
        delta_time: f32,
    ) {
        let vel_offset = match layout.offset_of(&SubFieldRole::Velocity) {
            Some(o) => o.lane(),
            None => return,
        };
        let int_offset = match layout.offset_of(&SubFieldRole::Intensity) {
            Some(o) => o.lane(),
            None => return,
        };
        let vel_abs = self.data[vel_offset].abs();
        let current = self.data[int_offset];
        let target = if vel_abs > behavior.velocity_threshold {
            current + behavior.build_coefficient * vel_abs * delta_time
        } else {
            current - behavior.decay_coefficient * current * delta_time
        };
        self.data[int_offset] = target.clamp(0.0, 1.0);
    }

    /// Magnitude of the first Named sub-field in this value's layout.
    /// Returns 0.0 if no Named sub-fields are present.
    pub fn vector_magnitude(&self, layout: &PropertyLayout) -> f32 {
        for sf in &layout.sub_fields {
            if matches!(&sf.role, SubFieldRole::Named(_)) {
                // offset_of is the one place local index arithmetic lives. (I1)
                let offset = layout.offset_of(&sf.role).unwrap().lane();
                let slice = &self.data[offset..offset + sf.width];
                return slice.iter().map(|x| x * x).sum::<f32>().sqrt();
            }
        }
        0.0
    }
}

// ── Decay and intensity behaviors ─────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Rising,
    Falling,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecayBehavior {
    TowardZero {
        rate: f32,
    },
    OnThreshold {
        threshold: f32,
        direction: Direction,
    },
    AfterTicks {
        remaining: u32,
    },
    WhenProperty {
        other: SimPropertyId,
        threshold: f32,
    },
    IntensityGated {
        intensity_floor: f32,
    },
}

/// Controls how intensity builds on rapid change and decays on stability.
/// Linear model: gain = build_coefficient * |velocity|, decay = decay_coefficient * intensity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntensityBehavior {
    pub velocity_threshold: f32,
    pub build_coefficient: f32,
    pub decay_coefficient: f32,
}

impl Default for IntensityBehavior {
    fn default() -> Self {
        Self {
            velocity_threshold: 0.005,
            build_coefficient: 2.0,
            decay_coefficient: 0.05,
        }
    }
}

// ── Fission / Fusion thresholds ───────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FissionThreshold {
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
    pub template: FissionTemplate,
    pub secondary: Option<SecondaryCondition>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SecondaryCondition {
    IntensityAbove(f32),
    IntensityBelow(f32),
    AmountAbove(f32),
    AmountBelow(f32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FissionTemplate {
    pub child_kind: SimThingKindTag,
    pub fusion_intensity_threshold: f32,
    pub fusion_scar_coefficient: f32,
    pub resolution_label: String,
    #[serde(default)]
    pub clone_capability_children: bool,
    #[serde(default)]
    pub capability_container_kinds: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FusionThreshold {
    pub dimension: SimPropertyId,
    pub sub_field: SubFieldRole,
    pub threshold: f32,
    pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimThingKindTag {
    Scenario,
    GameSession,
    World,
    Owner,
    /// Legacy alias for [`Owner`]; retained for serialized fission templates.
    Faction,
    StarSystem,
    Location,
    Cohort,
    Fleet,
    Station,
    Custom(String),
}

// ── On-expire handler ─────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpireHandler {
    pub write_back: Vec<ExpireEffect>,
    pub record_history: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExpireEffect {
    AddVelocity {
        property: SimPropertyId,
        sub_field: SubFieldRole,
        delta: f32,
    },
    SetIntensity {
        property: SimPropertyId,
        value: f32,
    },
}

// ── IntensityRange (semantic labels) ─────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntensityRange {
    pub amount_min: f32,
    pub amount_max: f32,
    pub intensity_min: f32,
    pub intensity_max: f32,
    pub label: String,
}

impl IntensityRange {
    pub fn matches(&self, amount: f32, intensity: f32) -> bool {
        amount >= self.amount_min
            && amount < self.amount_max
            && intensity >= self.intensity_min
            && intensity < self.intensity_max
    }
}

// ── SimProperty ───────────────────────────────────────────────────────────────

/// The schema for a property dimension. Equality and hashing are defined on
/// `namespace + name` only — metadata fields do not participate.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimProperty {
    // identity
    pub namespace: String,
    pub name: String,

    // layout — fully declarative, designer-controlled
    pub layout: PropertyLayout,

    // behavior — all optional
    pub decay: Option<DecayBehavior>,
    pub intensity_behavior: Option<IntensityBehavior>,
    pub fission_templates: Vec<FissionThreshold>,
    pub fusion_templates: Vec<FusionThreshold>,
    pub on_expire: Option<ExpireHandler>,

    // metadata
    pub description: String,
    pub intensity_labels: Vec<IntensityRange>,
}

impl SimProperty {
    /// Minimal property for tests. Uses PropertyLayout::standard(vector_len).
    pub fn simple(namespace: &str, name: &str, vector_len: usize) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            layout: PropertyLayout::standard(vector_len),
            decay: None,
            intensity_behavior: None,
            fission_templates: vec![],
            fusion_templates: vec![],
            on_expire: None,
            description: String::new(),
            intensity_labels: vec![],
        }
    }

    pub fn default_value(&self) -> PropertyValue {
        PropertyValue::from_layout(&self.layout)
    }

    pub fn interpret_intensity(&self, amount: f32, intensity: f32) -> Option<&str> {
        self.intensity_labels
            .iter()
            .find(|r| r.matches(amount, intensity))
            .map(|r| r.label.as_str())
    }
}

impl PartialEq for SimProperty {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.name == other.name
    }
}

impl Eq for SimProperty {}

impl std::hash::Hash for SimProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.namespace.hash(state);
        self.name.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standard_layout() -> PropertyLayout {
        PropertyLayout::standard(3)
    }

    fn loyalty(layout: &PropertyLayout, amount: f32, velocity: f32) -> PropertyValue {
        let mut pv = PropertyValue::from_layout(layout);
        pv.set_role(&SubFieldRole::Amount, layout, amount);
        pv.set_role(&SubFieldRole::Velocity, layout, velocity);
        pv
    }

    /// Custom layout: ethics axis with signed position, drift governor, and
    /// a 3-wide bonus vector. Verifies stride, offsets, defaults, integration.
    #[test]
    fn custom_layout_ethics_axis() {
        let layout = PropertyLayout {
            sub_fields: vec![
                SubFieldSpec {
                    role: SubFieldRole::Named("axis_position".into()),
                    width: 1,
                    clamp: ClampBehavior::Bounded {
                        min: -10.0,
                        max: 10.0,
                    },
                    velocity_max: Some(0.5),
                    default: 0.0,
                    display_name: "axis_position".into(),
                    display_range: Some((-10.0, 10.0)),
                    governed_by: Some(SubFieldRole::Named("axis_drift".into())),
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
                SubFieldSpec {
                    role: SubFieldRole::Named("axis_drift".into()),
                    width: 1,
                    clamp: ClampBehavior::Bounded {
                        min: -1.0,
                        max: 1.0,
                    },
                    velocity_max: None,
                    default: 0.0,
                    display_name: "axis_drift".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
                SubFieldSpec {
                    role: SubFieldRole::Named("ethics_bonus".into()),
                    width: 3, // [research, stability, unity]
                    clamp: ClampBehavior::Bounded { min: 0.0, max: 2.0 },
                    velocity_max: None,
                    default: 1.0, // neutral
                    display_name: "ethics_bonus".into(),
                    display_range: Some((0.0, 2.0)),
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
            ],
        };

        assert_eq!(layout.stride(), 5);
        assert_eq!(
            layout
                .offset_of(&SubFieldRole::Named("axis_position".into()))
                .map(|o| o.lane()),
            Some(0)
        );
        assert_eq!(
            layout
                .offset_of(&SubFieldRole::Named("axis_drift".into()))
                .map(|o| o.lane()),
            Some(1)
        );
        assert_eq!(
            layout
                .offset_of(&SubFieldRole::Named("ethics_bonus".into()))
                .map(|o| o.lane()),
            Some(2)
        );
        assert_eq!(
            layout.width_of(&SubFieldRole::Named("ethics_bonus".into())),
            Some(3)
        );

        let defaults = layout.default_data();
        assert_eq!(defaults, vec![0.0, 0.0, 1.0, 1.0, 1.0]);

        // axis_position governed by axis_drift
        let pos_off = layout
            .offset_of(&SubFieldRole::Named("axis_position".into()))
            .unwrap();
        let drift_off = layout
            .offset_of(&SubFieldRole::Named("axis_drift".into()))
            .unwrap();
        let bonus_off = layout
            .offset_of(&SubFieldRole::Named("ethics_bonus".into()))
            .unwrap();
        let bonus_w = layout
            .width_of(&SubFieldRole::Named("ethics_bonus".into()))
            .unwrap();

        let mut pv = PropertyValue::from_layout(&layout);
        pv.set_role(&SubFieldRole::Named("axis_drift".into()), &layout, 0.2);
        pv.integrate(&layout, 1.0);
        assert!(
            (pv.get_role(&SubFieldRole::Named("axis_position".into()), &layout) - 0.2).abs() < 1e-5,
            "position was {}",
            pv.get_role(&SubFieldRole::Named("axis_position".into()), &layout)
        );
        assert!(
            (pv.get_role(&SubFieldRole::Named("axis_drift".into()), &layout) - 0.2).abs() < 1e-5,
            "drift unchanged"
        );
        assert_eq!(
            pv.get_role_slice(&SubFieldRole::Named("ethics_bonus".into()), &layout),
            &[1.0, 1.0, 1.0],
            "bonus vector unchanged"
        );
    }

}
