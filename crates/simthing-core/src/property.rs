use crate::ids::SimPropertyId;
use serde::{Deserialize, Serialize};

// ── Sub-field layout constants ────────────────────────────────────────────────
// Every PropertyValue flat vector is laid out as:
//   [amount, velocity, intensity, vec_0, vec_1, ..., vec_N]
pub const AMOUNT_IDX: usize = 0;
pub const VELOCITY_IDX: usize = 1;
pub const INTENSITY_IDX: usize = 2;
pub const VECTOR_START_IDX: usize = 3;

// ── PropertyValue ─────────────────────────────────────────────────────────────

/// Flat float vector for a single property instance on a single SimThing.
/// Layout is defined by the corresponding `SimProperty::layout` in the registry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyValue {
    pub data: Vec<f32>,
}

impl PropertyValue {
    pub fn zeroed(stride: usize) -> Self {
        Self { data: vec![0.0; stride] }
    }

    pub fn amount(&self) -> f32 {
        self.data[AMOUNT_IDX]
    }

    pub fn velocity(&self) -> f32 {
        self.data[VELOCITY_IDX]
    }

    pub fn intensity(&self) -> f32 {
        self.data[INTENSITY_IDX]
    }

    pub fn vector(&self) -> &[f32] {
        &self.data[VECTOR_START_IDX..]
    }

    pub fn vector_mut(&mut self) -> &mut [f32] {
        &mut self.data[VECTOR_START_IDX..]
    }

    pub fn vector_magnitude(&self) -> f32 {
        let v = self.vector();
        v.iter().map(|x| x * x).sum::<f32>().sqrt()
    }

    /// Integrate velocity into amount for `delta_time` days.
    ///
    /// Velocity is zeroed in the direction of a saturated boundary: a cohort pinned
    /// at the loyalty floor cannot accumulate negative velocity while suppressed and
    /// then resist recovery when conditions improve. That kind of inertia belongs in
    /// the vector components (e.g. grievance_inertia), where it is observable to the
    /// player and attributable by the AI. Hidden velocity debt is not the same thing.
    pub fn integrate(&mut self, delta_time: f32, valid_range: (f32, f32)) {
        let new_amount = self.data[AMOUNT_IDX] + self.data[VELOCITY_IDX] * delta_time;
        self.data[AMOUNT_IDX] = new_amount.clamp(valid_range.0, valid_range.1);

        // If we hit the floor, don't let velocity keep going negative.
        // If we hit the ceiling, don't let velocity keep going positive.
        // Recovery-direction velocity is always permitted through.
        if self.data[AMOUNT_IDX] <= valid_range.0 {
            self.data[VELOCITY_IDX] = self.data[VELOCITY_IDX].max(0.0);
        } else if self.data[AMOUNT_IDX] >= valid_range.1 {
            self.data[VELOCITY_IDX] = self.data[VELOCITY_IDX].min(0.0);
        }
    }

    /// Apply intensity build/decay based on current velocity magnitude.
    pub fn update_intensity(&mut self, behavior: &IntensityBehavior, delta_time: f32) {
        let vel_abs = self.data[VELOCITY_IDX].abs();
        let current = self.data[INTENSITY_IDX];
        let target = if vel_abs > behavior.velocity_threshold {
            current + behavior.build_coefficient * vel_abs * delta_time
        } else {
            current - behavior.decay_coefficient * current * delta_time
        };
        self.data[INTENSITY_IDX] = target.clamp(0.0, 1.0);
    }
}

// ── PropertyLayout ────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PropertyLayout {
    /// Total floats per instance = 3 (amount/velocity/intensity) + vector_len
    pub stride: usize,
    /// Number of directional vector components
    pub vector_len: usize,
}

impl PropertyLayout {
    pub fn new(vector_len: usize) -> Self {
        Self { stride: 3 + vector_len, vector_len }
    }
}

// ── Sub-field roles and transform semantics ───────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubFieldRole {
    Amount,
    Velocity,
    Intensity,
    VectorComponent(usize),
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransformSemantics {
    Additive,
    Multiplicative,
    Affine,
    VelocityBias,
    IntensityScale,
    Clamped(f32, f32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransformOp {
    Add(f32),
    Multiply(f32),
    Set(f32),
}

impl TransformOp {
    pub fn apply(&self, current: f32) -> f32 {
        match self {
            Self::Add(v)      => current + v,
            Self::Multiply(v) => current * v,
            Self::Set(v)      => *v,
        }
    }

    /// Returns the identity for composition: no-op.
    pub fn identity(semantics: &TransformSemantics) -> Self {
        match semantics {
            TransformSemantics::Multiplicative
            | TransformSemantics::IntensityScale
            | TransformSemantics::Affine    => Self::Multiply(1.0),
            _ => Self::Add(0.0),
        }
    }
}

// ── Decay and intensity behaviors ─────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Direction {
    Rising,
    Falling,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecayBehavior {
    TowardZero        { rate: f32 },
    OnThreshold       { threshold: f32, direction: Direction },
    AfterTicks        { remaining: u32 },
    WhenProperty      { other: SimPropertyId, threshold: f32 },
    /// Intensity-driven: when intensity falls below a floor after being active.
    IntensityGated    { intensity_floor: f32 },
}

/// Controls how intensity builds on rapid change and decays on stability.
/// Linear model: gain = build_coefficient * |velocity|, decay = decay_coefficient * intensity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntensityBehavior {
    /// Velocity magnitude below which intensity starts decaying.
    pub velocity_threshold:  f32,
    /// Multiplier on |velocity| → intensity gain per day.
    pub build_coefficient:   f32,
    /// Fraction of current intensity shed per day when velocity is below threshold.
    pub decay_coefficient:   f32,
}

impl Default for IntensityBehavior {
    fn default() -> Self {
        Self {
            velocity_threshold: 0.005,
            build_coefficient:  2.0,
            decay_coefficient:  0.05,
        }
    }
}

// ── Fission / Fusion thresholds ───────────────────────────────────────────────

/// Registered on a SimThing; GPU threshold scanner watches it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FissionThreshold {
    pub dimension:  SimPropertyId,
    pub sub_field:  SubFieldRole,
    pub threshold:  f32,
    pub direction:  Direction,
    pub template:   FissionTemplate,
    /// Optional secondary condition on a second sub-field.
    pub secondary:  Option<SecondaryCondition>,
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
    pub child_kind:                SimThingKindTag,
    /// Intensity below which the child fuses back into the parent.
    pub fusion_intensity_threshold: f32,
    /// Added to the parent's activating property vector[0] on fusion (scar).
    pub fusion_scar_coefficient:    f32,
    pub resolution_label:          String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FusionThreshold {
    pub dimension:  SimPropertyId,
    pub sub_field:  SubFieldRole,
    pub threshold:  f32,
    pub direction:  Direction,
}

/// Lightweight kind tag used inside FissionTemplate to avoid circular deps.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimThingKindTag {
    World,
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
    pub write_back:     Vec<ExpireEffect>,
    pub record_history: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExpireEffect {
    AddVelocity { property: SimPropertyId, sub_field: SubFieldRole, delta: f32 },
    SetIntensity { property: SimPropertyId, value: f32 },
}

// ── IntensityRange (semantic labels) ─────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IntensityRange {
    pub amount_min:    f32,
    pub amount_max:    f32,
    pub intensity_min: f32,
    pub intensity_max: f32,
    pub label:         String,
}

impl IntensityRange {
    pub fn matches(&self, amount: f32, intensity: f32) -> bool {
        amount    >= self.amount_min    && amount    < self.amount_max
            && intensity >= self.intensity_min && intensity < self.intensity_max
    }
}

// ── SimProperty ───────────────────────────────────────────────────────────────

/// The schema for a property dimension. This is the HashMap key type used at
/// registration time. After registration, callers use `SimPropertyId` exclusively.
///
/// Equality and hashing are defined on `namespace + name` only — metadata fields
/// do not participate. See `impl PartialEq` below.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimProperty {
    // identity
    pub namespace: String,
    pub name:      String,

    // layout
    pub layout: PropertyLayout,

    // behavior — all optional
    pub decay:              Option<DecayBehavior>,
    pub intensity_behavior: Option<IntensityBehavior>,
    pub fission_templates:  Vec<FissionThreshold>,
    pub fusion_templates:   Vec<FusionThreshold>,
    pub on_expire:          Option<ExpireHandler>,

    // metadata
    pub description:       String,
    pub valid_range:       (f32, f32),
    pub default_velocity:  f32,
    pub default_intensity: f32,
    pub intensity_labels:  Vec<IntensityRange>,
}

impl SimProperty {
    /// Construct a minimal property for tests / bootstrap.
    pub fn simple(namespace: &str, name: &str, vector_len: usize) -> Self {
        Self {
            namespace:          namespace.into(),
            name:               name.into(),
            layout:             PropertyLayout::new(vector_len),
            decay:              None,
            intensity_behavior: None,
            fission_templates:  vec![],
            fusion_templates:   vec![],
            on_expire:          None,
            description:        String::new(),
            valid_range:        (0.0, 1.0),
            default_velocity:   0.0,
            default_intensity:  0.0,
            intensity_labels:   vec![],
        }
    }

    /// Return a fresh zero-initialized PropertyValue sized to this property's layout.
    pub fn default_value(&self) -> PropertyValue {
        let mut pv = PropertyValue::zeroed(self.layout.stride);
        pv.data[VELOCITY_IDX]  = self.default_velocity;
        pv.data[INTENSITY_IDX] = self.default_intensity;
        pv
    }

    /// Look up the semantic label for a given (amount, intensity) pair.
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

    fn loyalty(amount: f32, velocity: f32) -> PropertyValue {
        let mut pv = PropertyValue::zeroed(6); // stride 6: amount/vel/intensity + 3 vec
        pv.data[AMOUNT_IDX]   = amount;
        pv.data[VELOCITY_IDX] = velocity;
        pv
    }

    /// Velocity pinned at floor must not go further negative; recovering velocity passes.
    #[test]
    fn velocity_clamped_at_floor() {
        let range = (0.0_f32, 1.0_f32);

        let mut suppressed = loyalty(0.0, -0.03);
        suppressed.integrate(1.0, range);
        // amount stays at floor
        assert_eq!(suppressed.amount(), 0.0);
        // velocity must not remain negative — it would resist recovery
        assert!(suppressed.velocity() >= 0.0, "velocity was {}", suppressed.velocity());

        let mut recovering = loyalty(0.0, 0.05);
        recovering.integrate(1.0, range);
        // amount climbs off the floor
        assert!((recovering.amount() - 0.05).abs() < 1e-5);
        // positive velocity is untouched
        assert!(recovering.velocity() > 0.0);
    }

    /// Velocity pinned at ceiling must not go further positive; falling velocity passes.
    #[test]
    fn velocity_clamped_at_ceiling() {
        let range = (0.0_f32, 1.0_f32);

        let mut maxed = loyalty(1.0, 0.05);
        maxed.integrate(1.0, range);
        assert_eq!(maxed.amount(), 1.0);
        assert!(maxed.velocity() <= 0.0, "velocity was {}", maxed.velocity());

        let mut declining = loyalty(1.0, -0.02);
        declining.integrate(1.0, range);
        assert!((declining.amount() - 0.98).abs() < 1e-5);
        assert!(declining.velocity() < 0.0);
    }

    /// Mid-range: velocity and amount both change normally, no clamping.
    #[test]
    fn integrate_mid_range_unchanged() {
        let mut pv = loyalty(0.5, -0.03);
        pv.integrate(1.0, (0.0, 1.0));
        assert!((pv.amount() - 0.47).abs() < 1e-5);
        assert!((pv.velocity() - (-0.03)).abs() < 1e-5);
    }
}
