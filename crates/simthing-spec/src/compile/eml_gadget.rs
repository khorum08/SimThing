//! Phase M EML-GADGET-1 — Tier-1 stateless gadget registry, compiler, and CPU oracles.
//!
//! Gadgets are node-template macros over the existing EvalEML opcode set.
//! Spec/admission/compiler only — no runtime execution path in this module.

use crate::error::SpecError;
use crate::spec::eml_gadget::{EmlGadgetInstanceSpec, EmlGadgetStackSpec};
use simthing_core::eml_nodes::{self, EmlNode};
use simthing_core::{EmlExecutionClass, MAX_EML_TREE_NODES};

/// Tier-1 + Tier-2 temporal gadget kind identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EmlGadgetKind {
    FieldSampler,
    WeightedAccumulator,
    SoftStep,
    // Tier-2 temporal (EML-GADGET-2B)
    VelocityMonitor,
    Decay,
    Ema,
    // Tier-2 temporal (EML-GADGET-2C)
    BoundedFeedback,
}

impl EmlGadgetKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::FieldSampler => "FieldSampler",
            Self::WeightedAccumulator => "WeightedAccumulator",
            Self::SoftStep => "SoftStep",
            Self::VelocityMonitor => "VelocityMonitor",
            Self::Decay => "Decay",
            Self::Ema => "Ema",
            Self::BoundedFeedback => "BoundedFeedback",
        }
    }

    pub fn execution_class(self) -> EmlExecutionClass {
        EmlExecutionClass::ExactDeterministic
    }

    pub fn requires_temporal_memory(self) -> bool {
        match self {
            Self::VelocityMonitor | Self::Decay | Self::Ema | Self::BoundedFeedback => true,
            _ => false,
        }
    }

    pub fn all_tier1() -> &'static [EmlGadgetKind] {
        &[
            EmlGadgetKind::FieldSampler,
            EmlGadgetKind::WeightedAccumulator,
            EmlGadgetKind::SoftStep,
        ]
    }

    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "FieldSampler" => Some(Self::FieldSampler),
            "WeightedAccumulator" => Some(Self::WeightedAccumulator),
            "SoftStep" => Some(Self::SoftStep),
            "VelocityMonitor" => Some(Self::VelocityMonitor),
            "Decay" => Some(Self::Decay),
            "Ema" => Some(Self::Ema),
            "BoundedFeedback" => Some(Self::BoundedFeedback),
            _ => None,
        }
    }
}

/// Still-deferred Tier-2+ kinds (EML-GADGET-2D and beyond).
/// BoundedFeedback is implemented in EML-GADGET-2C.
pub const DEFERRED_GADGET_KINDS: &[&str] = &[
    "Acceleration",
    "Hysteresis",
];

/// Admission/compile options for gadget stacks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmlGadgetCompileOptions {
    /// Maximum valid column index (exclusive upper bound is `max_col`).
    pub max_col: u32,
}

impl Default for EmlGadgetCompileOptions {
    fn default() -> Self {
        Self { max_col: 64 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmlGadgetDiagnostic {
    pub code: &'static str,
    pub message: String,
}

/// Preview metadata for an admitted gadget stack.
#[derive(Clone, Debug, PartialEq)]
pub struct EmlGadgetPreviewReport {
    pub gadget_count: usize,
    pub gadget_ids: Vec<String>,
    pub gadget_kinds: Vec<String>,
    pub total_node_count: usize,
    pub flatten_preview_node_count: usize,
    pub execution_class: EmlExecutionClass,
    pub diagnostics: Vec<EmlGadgetDiagnostic>,
}

impl Default for EmlGadgetPreviewReport {
    fn default() -> Self {
        Self {
            gadget_count: 0,
            gadget_ids: Vec::new(),
            gadget_kinds: Vec::new(),
            total_node_count: 0,
            flatten_preview_node_count: 0,
            execution_class: EmlExecutionClass::ExactDeterministic,
            diagnostics: Vec::new(),
        }
    }
}

/// One compiled gadget instance.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledEmlGadget {
    pub id: String,
    pub kind: EmlGadgetKind,
    pub nodes: Vec<EmlNode>,
    pub execution_class: EmlExecutionClass,
    pub output_col: Option<u32>,
}

/// How a compiled gadget stack may be composed at runtime.
///
/// Per-gadget node templates are always executable. Multi-gadget inline flatten is never
/// claimed executable unless intermediate column wiring is proven (not implemented in V1).
#[derive(Clone, Debug, PartialEq)]
pub enum EmlGadgetCompositionPlan {
    /// Only per-gadget templates are valid output; chained runtime is deferred.
    PerGadgetOnly {
        reason: String,
    },
    /// Optional non-executable or single-gadget flatten preview for inspection.
    InlineFlattenPreview {
        nodes: Vec<EmlNode>,
        executable: bool,
        reason: String,
    },
}

impl EmlGadgetCompositionPlan {
    pub fn flatten_preview_executable(&self) -> bool {
        match self {
            Self::PerGadgetOnly { .. } => false,
            Self::InlineFlattenPreview { executable, .. } => *executable,
        }
    }

    pub fn flatten_preview_nodes(&self) -> Option<&[EmlNode]> {
        match self {
            Self::PerGadgetOnly { .. } => None,
            Self::InlineFlattenPreview { nodes, .. } => Some(nodes),
        }
    }
}

/// Compiled gadget stack (per-gadget templates + explicit composition plan).
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledEmlGadgetStack {
    pub gadgets: Vec<CompiledEmlGadget>,
    pub composition: EmlGadgetCompositionPlan,
    pub report: EmlGadgetPreviewReport,
}

/// Built-in Tier-1 gadget registry (descriptor names only in V1).
#[derive(Clone, Debug, Default)]
pub struct EmlGadgetRegistry;

impl EmlGadgetRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn tier1_kinds(&self) -> &'static [EmlGadgetKind] {
        EmlGadgetKind::all_tier1()
    }

    pub fn is_registered(&self, kind: EmlGadgetKind) -> bool {
        EmlGadgetKind::all_tier1().contains(&kind)
    }

    pub fn available_names(&self) -> Vec<&'static str> {
        EmlGadgetKind::all_tier1()
            .iter()
            .map(|k| k.name())
            .collect()
    }
}

/// Compile an authored gadget stack into EvalEML postfix node templates.
pub fn compile_eml_gadget_stack(
    spec: &EmlGadgetStackSpec,
    opts: EmlGadgetCompileOptions,
) -> Result<CompiledEmlGadgetStack, SpecError> {
    let _registry = EmlGadgetRegistry::new();
    let mut seen_ids = std::collections::HashSet::new();
    let mut compiled_gadgets = Vec::with_capacity(spec.gadgets.len());
    let mut report = EmlGadgetPreviewReport {
        execution_class: EmlExecutionClass::ExactDeterministic,
        ..Default::default()
    };

    for instance in &spec.gadgets {
        if instance.id().trim().is_empty() {
            return Err(SpecError::EmlGadgetAdmission {
                gadget: instance.id().to_string(),
                reason: "gadget id must be non-empty".into(),
            });
        }
        if !seen_ids.insert(instance.id().to_string()) {
            return Err(SpecError::EmlGadgetAdmission {
                gadget: instance.id().to_string(),
                reason: format!("duplicate gadget id `{}`", instance.id()),
            });
        }

        let kind = kind_from_instance(instance)?;
        if DEFERRED_GADGET_KINDS.contains(&kind.name()) {
            return Err(SpecError::EmlGadgetAdmission {
                gadget: instance.id().to_string(),
                reason: format!("gadget kind `{}` is deferred in EML-GADGET-1", kind.name()),
            });
        }

        let compiled = compile_gadget_instance(instance, kind, opts)?;
        report.gadget_ids.push(compiled.id.clone());
        report.gadget_kinds.push(compiled.kind.name().to_string());
        report.total_node_count += compiled.nodes.len();
        compiled_gadgets.push(compiled);
    }

    let composition = build_composition_plan(&compiled_gadgets, &spec.gadgets, &mut report);
    report.gadget_count = compiled_gadgets.len();

    apply_stack_node_cap_policy(&composition, &mut report)?;

    Ok(CompiledEmlGadgetStack {
        gadgets: compiled_gadgets,
        composition,
        report,
    })
}

fn apply_stack_node_cap_policy(
    composition: &EmlGadgetCompositionPlan,
    report: &mut EmlGadgetPreviewReport,
) -> Result<(), SpecError> {
    match composition {
        EmlGadgetCompositionPlan::InlineFlattenPreview { nodes, .. } => {
            if nodes.len() > MAX_EML_TREE_NODES as usize {
                return Err(SpecError::EmlGadgetAdmission {
                    gadget: "stack".into(),
                    reason: format!(
                        "single-gadget inline tree node count {} exceeds EvalEML cap {MAX_EML_TREE_NODES}",
                        nodes.len()
                    ),
                });
            }
            if report.flatten_preview_node_count > MAX_EML_TREE_NODES as usize {
                report.diagnostics.push(EmlGadgetDiagnostic {
                    code: "flatten_preview_exceeds_cap",
                    message: format!(
                        "inline flatten preview would use {} nodes; chained OrderBand execution deferred",
                        report.flatten_preview_node_count
                    ),
                });
            }
        }
        EmlGadgetCompositionPlan::PerGadgetOnly { .. } => {
            if report.total_node_count > MAX_EML_TREE_NODES as usize {
                report.diagnostics.push(EmlGadgetDiagnostic {
                    code: "stack_total_exceeds_inline_cap",
                    message: "total per-gadget node count exceeds single EvalEML tree cap; \
                              composition remains PerGadgetOnly and chained runtime scheduling is deferred"
                        .into(),
                });
            }
        }
    }
    Ok(())
}

fn kind_from_instance(instance: &EmlGadgetInstanceSpec) -> Result<EmlGadgetKind, SpecError> {
    let kind = match instance {
        EmlGadgetInstanceSpec::FieldSampler { .. } => EmlGadgetKind::FieldSampler,
        EmlGadgetInstanceSpec::WeightedAccumulator { .. } => EmlGadgetKind::WeightedAccumulator,
        EmlGadgetInstanceSpec::SoftStep { .. } => EmlGadgetKind::SoftStep,
        EmlGadgetInstanceSpec::VelocityMonitor { .. } => EmlGadgetKind::VelocityMonitor,
        EmlGadgetInstanceSpec::Decay { .. } => EmlGadgetKind::Decay,
        EmlGadgetInstanceSpec::Ema { .. } => EmlGadgetKind::Ema,
        EmlGadgetInstanceSpec::BoundedFeedback { .. } => EmlGadgetKind::BoundedFeedback,
    };
    Ok(kind)
}

fn compile_gadget_instance(
    instance: &EmlGadgetInstanceSpec,
    kind: EmlGadgetKind,
    opts: EmlGadgetCompileOptions,
) -> Result<CompiledEmlGadget, SpecError> {
    let id = instance.id().to_string();
    let (nodes, output_col) = match instance {
        EmlGadgetInstanceSpec::FieldSampler {
            input_col,
            output_col,
            cap,
            ..
        } => {
            validate_col(*input_col, opts, &id, "input_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            validate_field_sampler_params(*cap, &id)?;
            (
                compile_field_sampler_nodes(*input_col, *cap),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::SoftStep {
            input_col,
            output_col,
            center,
            steepness,
            ..
        } => {
            validate_col(*input_col, opts, &id, "input_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            validate_soft_step_params(*center, *steepness, &id)?;
            (
                compile_soft_step_nodes(*input_col, *center, *steepness),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::WeightedAccumulator {
            input_cols,
            weight_cols,
            output_col,
            ..
        } => {
            validate_weighted_accumulator_params(input_cols, weight_cols, opts, &id)?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            (
                compile_weighted_accumulator_nodes(input_cols, weight_cols),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::VelocityMonitor {
            current_col,
            previous_col,
            output_col,
            dt,
            ..
        } => {
            validate_col(*current_col, opts, &id, "current_col")?;
            validate_col(*previous_col, opts, &id, "previous_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            if *current_col == *previous_col {
                return Err(SpecError::EmlGadgetAdmission {
                    gadget: id,
                    reason: "VelocityMonitor current_col and previous_col must be distinct".into(),
                });
            }
            validate_velocity_params(*dt, &id)?;
            (
                compile_velocity_monitor_nodes(*current_col, *previous_col, *dt),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::Decay {
            state_col,
            output_col,
            decay,
            ..
        } => {
            validate_col(*state_col, opts, &id, "state_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            validate_decay_params(*decay, &id)?;
            (
                compile_decay_nodes(*state_col, *decay),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::Ema {
            input_col,
            previous_col,
            output_col,
            decay,
            ..
        } => {
            validate_col(*input_col, opts, &id, "input_col")?;
            validate_col(*previous_col, opts, &id, "previous_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            if *input_col == *previous_col {
                return Err(SpecError::EmlGadgetAdmission {
                    gadget: id,
                    reason: "Ema input_col and previous_col must be distinct".into(),
                });
            }
            validate_decay_params(*decay, &id)?;
            (
                compile_ema_nodes(*input_col, *previous_col, *decay),
                *output_col,
            )
        }
        EmlGadgetInstanceSpec::BoundedFeedback {
            previous_col,
            input_col,
            output_col,
            decay,
            gain,
            min,
            max,
            ..
        } => {
            validate_col(*previous_col, opts, &id, "previous_col")?;
            validate_col(*input_col, opts, &id, "input_col")?;
            if let Some(col) = output_col {
                validate_col(*col, opts, &id, "output_col")?;
            }
            if *previous_col == *input_col {
                return Err(SpecError::EmlGadgetAdmission {
                    gadget: id,
                    reason: "BoundedFeedback previous_col and input_col must be distinct".into(),
                });
            }
            validate_bounded_feedback_params(*decay, *gain, *min, *max, &id)?;
            (
                compile_bounded_feedback_nodes(*previous_col, *input_col, *decay, *gain, *min, *max),
                *output_col,
            )
        }
    };

    if nodes.len() > MAX_EML_TREE_NODES as usize {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: id,
            reason: format!(
                "gadget node count {} exceeds EvalEML cap {MAX_EML_TREE_NODES}",
                nodes.len()
            ),
        });
    }

    Ok(CompiledEmlGadget {
        id,
        kind,
        nodes,
        execution_class: kind.execution_class(),
        output_col,
    })
}

fn build_composition_plan(
    gadgets: &[CompiledEmlGadget],
    instances: &[EmlGadgetInstanceSpec],
    report: &mut EmlGadgetPreviewReport,
) -> EmlGadgetCompositionPlan {
    if gadgets.is_empty() {
        return EmlGadgetCompositionPlan::PerGadgetOnly {
            reason: "empty gadget stack".into(),
        };
    }

    if gadgets.len() == 1 {
        let nodes = gadgets[0].nodes.clone();
        report.flatten_preview_node_count = nodes.len();
        return EmlGadgetCompositionPlan::InlineFlattenPreview {
            nodes,
            executable: true,
            reason: "single-gadget stack; per-gadget EvalEML template is directly executable"
                .into(),
        };
    }

    let chained = stack_uses_column_chaining(gadgets, instances);
    let reason: String = if chained {
        "multi-gadget output_col/input_col chaining requires per-gadget OrderBand execution; \
         inline flatten does not wire intermediate columns and runtime scheduling is deferred"
            .into()
    } else {
        "multi-gadget stack has no proven inline intermediate wiring; use per-gadget templates \
         only; chained runtime scheduling is deferred"
            .into()
    };

    report.diagnostics.push(EmlGadgetDiagnostic {
        code: "chained_runtime_deferred",
        message: reason.clone(),
    });

    EmlGadgetCompositionPlan::PerGadgetOnly { reason }
}

fn stack_uses_column_chaining(
    gadgets: &[CompiledEmlGadget],
    instances: &[EmlGadgetInstanceSpec],
) -> bool {
    let mut prior_outputs = std::collections::HashSet::new();
    for (compiled, instance) in gadgets.iter().zip(instances.iter()) {
        if instance
            .input_cols()
            .iter()
            .any(|col| prior_outputs.contains(col))
        {
            return true;
        }
        if let Some(out) = compiled.output_col {
            prior_outputs.insert(out);
        }
    }
    false
}

fn validate_col(col: u32, opts: EmlGadgetCompileOptions, gadget: &str, field: &str) -> Result<(), SpecError> {
    if col >= opts.max_col {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: format!("{field} {col} out of bounds (max_col {})", opts.max_col),
        });
    }
    Ok(())
}

fn validate_field_sampler_params(cap: f32, gadget: &str) -> Result<(), SpecError> {
    if !cap.is_finite() || cap <= 0.0 {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "FieldSampler cap must be finite and > 0".into(),
        });
    }
    Ok(())
}

fn validate_soft_step_params(center: f32, steepness: f32, gadget: &str) -> Result<(), SpecError> {
    if !center.is_finite() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "SoftStep center must be finite".into(),
        });
    }
    if !steepness.is_finite() || steepness <= 0.0 {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "SoftStep steepness must be finite and > 0".into(),
        });
    }
    Ok(())
}

fn validate_weighted_accumulator_params(
    input_cols: &[u32],
    weight_cols: &[u32],
    opts: EmlGadgetCompileOptions,
    gadget: &str,
) -> Result<(), SpecError> {
    if input_cols.is_empty() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "WeightedAccumulator requires at least one input".into(),
        });
    }
    if input_cols.len() != weight_cols.len() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: format!(
                "WeightedAccumulator input count {} != weight count {}",
                input_cols.len(),
                weight_cols.len()
            ),
        });
    }
    for (i, col) in input_cols.iter().enumerate() {
        validate_col(*col, opts, gadget, &format!("input_cols[{i}]"))?;
    }
    for (i, col) in weight_cols.iter().enumerate() {
        validate_col(*col, opts, gadget, &format!("weight_cols[{i}]"))?;
    }
    Ok(())
}

fn compile_field_sampler_nodes(input_col: u32, cap: f32) -> Vec<EmlNode> {
    vec![
        node_slot(input_col),
        node_literal(cap),
        node_div_safe(),
        node_clamp_bounded(0.0, 1.0),
        node_return_top(),
    ]
}

fn compile_weighted_accumulator_nodes(input_cols: &[u32], weight_cols: &[u32]) -> Vec<EmlNode> {
    let mut nodes = Vec::new();
    for (input_col, weight_col) in input_cols.iter().zip(weight_cols.iter()) {
        nodes.push(node_slot(*input_col));
        nodes.push(node_slot(*weight_col));
        nodes.push(node_mul());
    }
    for _ in 1..input_cols.len() {
        nodes.push(node_add());
    }
    nodes.push(node_return_top());
    nodes
}

fn compile_soft_step_nodes(input_col: u32, center: f32, steepness: f32) -> Vec<EmlNode> {
    let mut nodes = Vec::new();
    // u = steepness * (x - center); keep first u on stack for the final division.
    nodes.extend(compute_u_nodes(input_col, center, steepness));
    // 1 + abs(u) using a second u recomputation.
    nodes.extend(compute_u_nodes(input_col, center, steepness));
    nodes.push(node_abs());
    nodes.push(node_literal(1.0));
    nodes.push(node_add());
    // Stack: [u, 1 + abs(u)] → u / (1 + abs(u))
    nodes.push(node_div_safe());
    nodes.push(node_literal(0.5));
    nodes.push(node_mul());
    nodes.push(node_literal(0.5));
    nodes.push(node_add());
    nodes.push(node_return_top());
    nodes
}

fn compute_u_nodes(input_col: u32, center: f32, steepness: f32) -> Vec<EmlNode> {
    vec![
        node_slot(input_col),
        node_literal(center),
        node_sub(),
        node_literal(steepness),
        node_mul(),
    ]
}

// ── CPU oracles (mandatory parity reference) ─────────────────────────────────

pub fn oracle_field_sampler(input: f32, cap: f32) -> f32 {
    (input / cap).clamp(0.0, 1.0)
}

pub fn oracle_weighted_accumulator(inputs: &[f32], weights: &[f32]) -> f32 {
    inputs
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| x * w)
        .sum()
}

pub fn oracle_soft_step(x: f32, center: f32, steepness: f32) -> f32 {
    let u = steepness * (x - center);
    0.5 + 0.5 * u / (1.0 + u.abs())
}

/// Evaluate a postfix EvalEML node program for spec-layer parity tests.
pub fn eval_eml_postfix(
    nodes: &[EmlNode],
    eval_slot: u32,
    values: &[f32],
    n_dims: u32,
) -> f32 {
    let mut stack = [0.0f32; 32];
    let mut sp: usize = 0;

    for node in nodes {
        match node.opcode {
            eml_nodes::opcode::LITERAL_F32 => {
                stack[sp] = f32::from_bits(node.a);
                sp += 1;
            }
            eml_nodes::opcode::SLOT_VALUE => {
                let i = (eval_slot * n_dims + node.a) as usize;
                stack[sp] = values.get(i).copied().unwrap_or(0.0);
                sp += 1;
            }
            eml_nodes::opcode::ADD => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs + rhs;
                sp -= 1;
            }
            eml_nodes::opcode::SUB => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs - rhs;
                sp -= 1;
            }
            eml_nodes::opcode::MUL => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = lhs * rhs;
                sp -= 1;
            }
            eml_nodes::opcode::DIV => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = if rhs == 0.0 { 0.0 } else { lhs / rhs };
                sp -= 1;
            }
            eml_nodes::opcode::ABS => {
                stack[sp - 1] = stack[sp - 1].abs();
            }
            eml_nodes::opcode::CLAMP_BOUNDED => {
                let v = stack[sp - 1];
                stack[sp - 1] = v.clamp(f32::from_bits(node.a), f32::from_bits(node.b));
            }
            eml_nodes::opcode::RETURN_TOP => {
                return stack[sp - 1];
            }
            _ => panic!("unsupported opcode in gadget parity eval: {}", node.opcode),
        }
    }
    stack[sp - 1]
}

// ── Node builders ────────────────────────────────────────────────────────────

fn node_literal(v: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_slot(col: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_add() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::ADD,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_sub() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SUB,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_mul() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::MUL,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_div_safe() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::DIV,
        flags: 1,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_abs() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::ABS,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn node_clamp_bounded(lo: f32, hi: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::CLAMP_BOUNDED,
        flags: 0,
        a: lo.to_bits(),
        b: hi.to_bits(),
        c: 0,
        d: 0,
    }
}

fn node_return_top() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::RETURN_TOP,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

/// Reject unknown deferred gadget kind strings at admission boundaries.
pub fn reject_unknown_gadget_kind(kind: &str, gadget_id: &str) -> Result<EmlGadgetKind, SpecError> {
    if DEFERRED_GADGET_KINDS.contains(&kind) {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget_id.to_string(),
            reason: format!("gadget kind `{kind}` is deferred in EML-GADGET-1"),
        });
    }
    EmlGadgetKind::parse(kind).ok_or_else(|| SpecError::EmlGadgetAdmission {
        gadget: gadget_id.to_string(),
        reason: format!(
            "unknown gadget kind `{kind}`; available: FieldSampler, WeightedAccumulator, SoftStep, VelocityMonitor, Decay, Ema"
        ),
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Tier-2 temporal gadget support (EML-GADGET-2B)
// VelocityMonitor, Decay, Ema — explicit-column, existing EvalEML nodes only.
// Stateful sequence CPU oracles. No runtime scheduling in this slice.
// ─────────────────────────────────────────────────────────────────────────────

fn validate_velocity_params(dt: Option<f32>, gadget: &str) -> Result<(), SpecError> {
    if let Some(d) = dt {
        if !d.is_finite() || d <= 0.0 {
            return Err(SpecError::EmlGadgetAdmission {
                gadget: gadget.to_string(),
                reason: "VelocityMonitor dt must be finite and > 0 when provided".into(),
            });
        }
    }
    Ok(())
}

fn validate_decay_params(decay: f32, gadget: &str) -> Result<(), SpecError> {
    if !decay.is_finite() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "decay must be finite".into(),
        });
    }
    if !(0.0..1.0).contains(&decay) {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "decay must satisfy 0 <= decay < 1 for EML-GADGET-2B".into(),
        });
    }
    Ok(())
}

fn compile_velocity_monitor_nodes(current_col: u32, previous_col: u32, dt: Option<f32>) -> Vec<EmlNode> {
    let mut nodes = vec![
        node_slot(current_col),
        node_slot(previous_col),
        node_sub(),
    ];
    if let Some(d) = dt {
        if (d - 1.0).abs() > 1e-9 {
            nodes.push(node_literal(d));
            nodes.push(node_div_safe());
        }
    }
    nodes.push(node_return_top());
    nodes
}

fn compile_decay_nodes(state_col: u32, decay: f32) -> Vec<EmlNode> {
    vec![
        node_slot(state_col),
        node_literal(decay),
        node_mul(),
        node_return_top(),
    ]
}

fn compile_ema_nodes(input_col: u32, previous_col: u32, decay: f32) -> Vec<EmlNode> {
    let one_minus_decay = 1.0 - decay;
    vec![
        node_slot(previous_col),
        node_literal(decay),
        node_mul(),
        node_slot(input_col),
        node_literal(one_minus_decay),
        node_mul(),
        node_add(),
        node_return_top(),
    ]
}

// Public CPU oracles for stateful sequence parity tests (exported from spec crate)
pub fn oracle_velocity_monitor(current: f32, previous: f32, dt: f32) -> f32 {
    (current - previous) / dt
}

pub fn oracle_decay(state: f32, decay: f32) -> f32 {
    state * decay
}

pub fn oracle_ema(input: f32, previous: f32, decay: f32) -> f32 {
    previous * decay + input * (1.0 - decay)
}

// ─────────────────────────────────────────────────────────────────────────────
// Tier-2 BoundedFeedback (EML-GADGET-2C)
// Requires explicit clamp (min < max). Strict admission rejects unbounded forms.
// Uses existing CLAMP_BOUNDED support (already present for FieldSampler).
// ─────────────────────────────────────────────────────────────────────────────

fn validate_bounded_feedback_params(decay: f32, gain: f32, min: f32, max: f32, gadget: &str) -> Result<(), SpecError> {
    if !decay.is_finite() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "BoundedFeedback decay must be finite".into(),
        });
    }
    if !(0.0..1.0).contains(&decay) {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "BoundedFeedback decay must satisfy 0 <= decay < 1".into(),
        });
    }
    if !gain.is_finite() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "BoundedFeedback gain must be finite".into(),
        });
    }
    if !min.is_finite() || !max.is_finite() {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "BoundedFeedback min and max must be finite".into(),
        });
    }
    if min >= max {
        return Err(SpecError::EmlGadgetAdmission {
            gadget: gadget.to_string(),
            reason: "BoundedFeedback min must be < max".into(),
        });
    }
    Ok(())
}

fn compile_bounded_feedback_nodes(previous_col: u32, input_col: u32, decay: f32, gain: f32, min: f32, max: f32) -> Vec<EmlNode> {
    vec![
        node_slot(previous_col),
        node_literal(decay),
        node_mul(),
        node_slot(input_col),
        node_literal(gain),
        node_mul(),
        node_add(),
        node_clamp_bounded(min, max),
        node_return_top(),
    ]
}

pub fn oracle_bounded_feedback(previous: f32, input: f32, decay: f32, gain: f32, min: f32, max: f32) -> f32 {
    (previous * decay + input * gain).clamp(min, max)
}

// 2B node emission re-uses the existing private node_* helpers defined earlier in this file
// (node_slot, node_literal, node_sub, node_mul, node_add, node_div_safe, node_return_top).
// No duplication.
