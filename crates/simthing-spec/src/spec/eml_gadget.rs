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
}

impl EmlGadgetInstanceSpec {
    pub fn id(&self) -> &str {
        match self {
            Self::FieldSampler { id, .. }
            | Self::SoftStep { id, .. }
            | Self::WeightedAccumulator { id, .. } => id,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::FieldSampler { .. } => "FieldSampler",
            Self::SoftStep { .. } => "SoftStep",
            Self::WeightedAccumulator { .. } => "WeightedAccumulator",
        }
    }
}
