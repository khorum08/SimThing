//! EML Gadget Library — RON-authored reusable EvalEML node-template macros (Phase M EML-GADGET-1).

use serde::{Deserialize, Serialize};

/// Authored stack of gadget instances (designer/RON surface).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct EmlGadgetStackSpec {
    #[serde(default)]
    pub gadgets: Vec<EmlGadgetInstanceSpec>,
}

/// One gadget instance in an authored stack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum EmlGadgetInstanceSpec {
    FieldSampler {
        id: String,
        input_col: u32,
        #[serde(default)]
        output_col: Option<u32>,
        cap: f32,
    },
    SoftStep {
        id: String,
        input_col: u32,
        #[serde(default)]
        output_col: Option<u32>,
        center: f32,
        steepness: f32,
    },
    WeightedAccumulator {
        id: String,
        input_cols: Vec<u32>,
        weight_cols: Vec<u32>,
        #[serde(default)]
        output_col: Option<u32>,
    },
    /// Tier-2 temporal: velocity = (current - previous) [ / dt ]
    VelocityMonitor {
        id: String,
        current_col: u32,
        previous_col: u32,
        #[serde(default)]
        output_col: Option<u32>,
        /// Optional positive finite dt for scaling. If None or 1.0, no division is emitted.
        #[serde(default)]
        dt: Option<f32>,
    },
    /// Tier-2 temporal (pure in-place decay form): state_next = state * decay
    /// The state column is its own memory; no separate previous_col required.
    Decay {
        id: String,
        state_col: u32,
        #[serde(default)]
        output_col: Option<u32>,
        decay: f32,
    },
    /// Tier-2 temporal (EMA / exponential smoothing): next = previous * decay + input * (1 - decay)
    Ema {
        id: String,
        input_col: u32,
        previous_col: u32,
        #[serde(default)]
        output_col: Option<u32>,
        decay: f32,
    },
}

impl EmlGadgetInstanceSpec {
    pub fn id(&self) -> &str {
        match self {
            Self::FieldSampler { id, .. }
            | Self::SoftStep { id, .. }
            | Self::WeightedAccumulator { id, .. }
            | Self::VelocityMonitor { id, .. }
            | Self::Decay { id, .. }
            | Self::Ema { id, .. } => id,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::FieldSampler { .. } => "FieldSampler",
            Self::SoftStep { .. } => "SoftStep",
            Self::WeightedAccumulator { .. } => "WeightedAccumulator",
            Self::VelocityMonitor { .. } => "VelocityMonitor",
            Self::Decay { .. } => "Decay",
            Self::Ema { .. } => "Ema",
        }
    }

    pub fn input_cols(&self) -> Vec<u32> {
        match self {
            Self::FieldSampler { input_col, .. } => vec![*input_col],
            Self::SoftStep { input_col, .. } => vec![*input_col],
            Self::WeightedAccumulator { input_cols, .. } => input_cols.clone(),
            Self::VelocityMonitor { current_col, previous_col, .. } => vec![*current_col, *previous_col],
            Self::Decay { state_col, .. } => vec![*state_col],
            Self::Ema { input_col, previous_col, .. } => vec![*input_col, *previous_col],
        }
    }

    pub fn output_col(&self) -> Option<u32> {
        match self {
            Self::FieldSampler { output_col, .. }
            | Self::SoftStep { output_col, .. }
            | Self::WeightedAccumulator { output_col, .. }
            | Self::VelocityMonitor { output_col, .. }
            | Self::Decay { output_col, .. }
            | Self::Ema { output_col, .. } => *output_col,
        }
    }
}
