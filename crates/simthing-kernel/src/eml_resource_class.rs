use std::sync::atomic::{AtomicBool, Ordering};

use simthing_core::{eml_opcode, EmlNodeGpu, EmlResourceClass};

/// Production plant: uniqueness-fused JIT emits ordinary Sum fold (separate
/// map-MUL then fold-ADD) instead of the fused `fma` fold body.
static PLANT_SEAM_JIT_SEPARATE_ROUNDING: AtomicBool = AtomicBool::new(false);

/// Plant: SSA-JIT uniqueness-fused seam uses ordinary Sum fold (unfused).
pub(crate) fn plant_seam_jit_separate_rounding(on: bool) {
    PLANT_SEAM_JIT_SEPARATE_ROUNDING.store(on, Ordering::SeqCst);
}

const STACK_LIMIT_TOKEN: &str = "const EML_STACK_MAX: u32 = 32u;";

/// JIT-specialize the sole resource token in a canonical EML interpreter.
/// Algebra, opcodes, control flow, and bindings remain byte-identical.
pub(crate) fn specialize_eml_stack_limit(
    canonical_source: &str,
    resource_class: EmlResourceClass,
) -> String {
    assert_eq!(
        canonical_source.matches(STACK_LIMIT_TOKEN).count(),
        1,
        "canonical EML shader must contain exactly one stack-limit token"
    );
    canonical_source.replace(
        STACK_LIMIT_TOKEN,
        &format!(
            "const EML_STACK_MAX: u32 = {}u;",
            resource_class.stack_slots()
        ),
    )
}

const JIT_EVALUATOR_BEGIN: &str = "// EML-JIT-EVALUATOR-BEGIN";
const JIT_EVALUATOR_END: &str = "// EML-JIT-EVALUATOR-END";
const FNV_OFFSET: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CanonicalFieldProgramIdentity {
    words: Vec<u32>,
    digest: u64,
}

impl CanonicalFieldProgramIdentity {
    pub(crate) fn new(map: &[EmlNodeGpu], fold: &[EmlNodeGpu], post: &[EmlNodeGpu]) -> Self {
        let mut words = Vec::with_capacity(3 + 6 * (map.len() + fold.len() + post.len()));
        for program in [map, fold, post] {
            words.push(program.len() as u32);
            for node in program {
                words.extend_from_slice(&[node.opcode, node.flags, node.a, node.b, node.c, node.d]);
            }
        }
        let digest = words.iter().fold(FNV_OFFSET, |mut hash, word| {
            for byte in word.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            hash
        });
        Self { words, digest }
    }

    pub(crate) fn digest(&self) -> u64 {
        self.digest
    }

    pub(crate) fn word_count(&self) -> u32 {
        self.words.len() as u32
    }

    pub(crate) fn fused_pair(producer: &Self, consumer: &Self) -> Self {
        let mut words = Vec::with_capacity(2 + producer.words.len() + consumer.words.len());
        words.push(0x4655_5345);
        words.extend_from_slice(&producer.words);
        words.push(0x5041_4952);
        words.extend_from_slice(&consumer.words);
        Self::from_words(words)
    }

    fn from_words(words: Vec<u32>) -> Self {
        let digest = words.iter().fold(FNV_OFFSET, |mut hash, word| {
            for byte in word.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            hash
        });
        Self { words, digest }
    }
}

pub(crate) fn pipeline_cache_digest(
    resource_class: EmlResourceClass,
    identity: &CanonicalFieldProgramIdentity,
) -> u64 {
    let class_word = match resource_class {
        EmlResourceClass::CompactStack4 => 4u32,
        EmlResourceClass::LegacyFixed32 => 32u32,
    };
    [
        class_word,
        identity.digest() as u32,
        (identity.digest() >> 32) as u32,
    ]
    .iter()
    .fold(FNV_OFFSET, |mut hash, word| {
        for byte in word.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        hash
    })
}

pub(crate) fn generate_field_sweep_jit(
    canonical_source: &str,
    resource_class: EmlResourceClass,
    map: &[EmlNodeGpu],
    fold: &[EmlNodeGpu],
    post: &[EmlNodeGpu],
) -> String {
    let begin = canonical_source
        .find(JIT_EVALUATOR_BEGIN)
        .expect("canonical field shader has JIT evaluator begin marker");
    let end = canonical_source
        .find(JIT_EVALUATOR_END)
        .expect("canonical field shader has JIT evaluator end marker")
        + JIT_EVALUATOR_END.len();
    assert!(
        begin < end,
        "canonical field shader has ordered JIT markers"
    );

    // Uniqueness-rule instance (5.14): map ends in MUL + canonical Sum fold →
    // SPECIFIED FUSED (acc = fma(a, b, acc)). JIT emits explicit fma; CPU and
    // interpreted arms match via the registration-derived shape.
    // Plant: emit the ordinary Sum fold so MUL (eval_map) and ADD are separate
    // roundings — a real fusion-law defect, not a post-hoc bit corruption.
    let fold_body = if crate::field_sweep::seam_fused_shape(map, fold) {
        if PLANT_SEAM_JIT_SEPARATE_ROUNDING.load(Ordering::SeqCst) {
            emit_program("eval_fold", fold)
        } else {
            emit_seam_fused_fold(&map[..map.len() - 2])
        }
    } else {
        emit_program("eval_fold", fold)
    };
    let generated = format!(
        "// EML-JIT-EVALUATOR-BEGIN\n// Mechanically generated from admitted postfix IR.\nconst EML_JIT_RESOURCE_STACK_SLOTS: u32 = {}u;\n{}\n{}\n{}\n// EML-JIT-EVALUATOR-END",
        resource_class.stack_slots(),
        emit_program("eval_map", map),
        fold_body,
        emit_program("eval_post", post),
    );
    let mut source = String::with_capacity(canonical_source.len() + generated.len());
    source.push_str(&canonical_source[..begin]);
    source.push_str(&generated);
    source.push_str(&canonical_source[end..]);
    // The JIT collapses the interpreted uniqueness-fused branch into direct
    // generated calls (eval_fold is the fused form when the shape holds).
    replace_once(
        &mut source,
        "        if (params.pad1 == 1u) {
            // SEAM LAW: fused canonical-Sum fold (uniform flag, no divergence).
            let ab = eval_program_pair(params.map_offset, params.map_count - 2u, context);
            accumulator = fma(ab.x, ab.y, accumulator);
        } else {
            let mapped = eval_program(params.map_offset, params.map_count, context);
            context.mapped = mapped;
            accumulator = eval_program(params.fold_offset, params.fold_count, context);
        }",
        "        let mapped = eval_map(context);
        context.mapped = mapped;
        accumulator = eval_fold(context);",
    );
    replace_once(
        &mut source,
        "eval_program(params.post_offset, params.post_count, post_context)",
        "eval_post(post_context)",
    );
    source
}

pub(crate) fn generate_fused_transient_field_sweep_jit(
    canonical_source: &str,
    resource_class: EmlResourceClass,
    producer_map: &[EmlNodeGpu],
    producer_fold: &[EmlNodeGpu],
    producer_post: &[EmlNodeGpu],
    consumer_map: &[EmlNodeGpu],
    consumer_fold: &[EmlNodeGpu],
    consumer_post: &[EmlNodeGpu],
) -> String {
    let mut source = generate_field_sweep_jit(
        canonical_source,
        resource_class,
        consumer_map,
        consumer_fold,
        consumer_post,
    );
    let helper = format!(
        "{}\n{}\n{}\nfn eval_fused_transient(target_slot: u32) -> f32 {{\n    let range = ranges[target_slot];\n    var accumulator = bitcast<f32>(params.fused_identity_bits);\n    for (var edge_index = 0u; edge_index < range.count; edge_index = edge_index + 1u) {{\n        let input = inputs[range.offset + edge_index];\n        var context = FieldEmlContext(target_slot, input.slot, 1u, accumulator, bitcast<f32>(input.unit_cost_bits), bitcast<f32>(params.fused_dt_bits), 0.0, 0.0, 0.0, 0.0);\n        let mapped = eval_fused_map(context);\n        context.mapped = mapped;\n        accumulator = eval_fused_fold(context);\n    }}\n    let post_context = FieldEmlContext(target_slot, target_slot, 0u, accumulator, 0.0, bitcast<f32>(params.fused_dt_bits), 0.0, accumulator, 0.0, 0.0);\n    return eval_fused_post(post_context);\n}}\n",
        emit_program("eval_fused_map", producer_map),
        emit_program("eval_fused_fold", producer_fold),
        emit_program("eval_fused_post", producer_post),
    );
    let end = source
        .find(JIT_EVALUATOR_END)
        .expect("generated field shader retains JIT end marker");
    source.insert_str(end, &helper);

    assert_eq!(
        source.matches("transient_values[target_slot]").count(),
        3,
        "canonical field shader target-transient seams changed"
    );
    source = source.replacen("transient_values[target_slot]", "fused_target_transient", 2);
    replace_once(
        &mut source,
        "let target_slot = schedule[params.schedule_offset + gid.x];",
        "let target_slot = schedule[params.schedule_offset + gid.x];\n    let fused_target_transient = eval_fused_transient(target_slot);\n    transient_values[target_slot] = fused_target_transient;",
    );
    replace_once(
        &mut source,
        "transient_values[input.slot]",
        "eval_fused_transient(input.slot)",
    );
    source
}

fn replace_once(source: &mut String, needle: &str, replacement: &str) {
    assert_eq!(
        source.matches(needle).count(),
        1,
        "canonical field shader JIT call seam changed: {needle}"
    );
    *source = source.replacen(needle, replacement, 1);
}

#[derive(Clone)]
struct EmitEntry {
    expr: String,
    /// Immediate MUL factors when `expr` is a MUL result (uniqueness tracking).
    mul: Option<(String, String)>,
}

impl EmitEntry {
    fn plain(expr: String) -> Self {
        Self { expr, mul: None }
    }
}

/// Seam-fused fold body (uniqueness instance): recompute the map final-MUL
/// operands and fuse them with the accumulator in ONE explicit fma.
fn emit_seam_fused_fold(mul_free_map: &[EmlNodeGpu]) -> String {
    let mut fused = String::from("fn eval_fold(context: FieldEmlContext) -> f32 {\n");
    let mut stack: Vec<EmitEntry> = Vec::new();
    let mut next_value = 0u32;
    let mut inner = String::new();
    for node in mul_free_map {
        if let Some(entry) = expression_for(node, &mut stack, &mut inner) {
            let value = format!("v{next_value}");
            next_value += 1;
            inner.push_str(&format!("    let {value}: f32 = {};\n", entry.expr));
            stack.push(EmitEntry {
                expr: value,
                mul: entry.mul,
            });
        }
    }
    let rhs = stack.pop().expect("seam mul rhs");
    let lhs = stack.pop().expect("seam mul lhs");
    fused.push_str(&inner);
    fused.push_str(&format!(
        "    return fma({}, {}, context.accumulator);\n}}\n",
        lhs.expr, rhs.expr
    ));
    fused
}

/// Shared per-node expression emitter (factored from emit_program).
fn expression_for(
    node: &EmlNodeGpu,
    stack: &mut Vec<EmitEntry>,
    output: &mut String,
) -> Option<EmitEntry> {
    match node.opcode {
        eml_opcode::LITERAL_F32 => {
            Some(EmitEntry::plain(format!("bitcast<f32>(0x{:08x}u)", node.a)))
        }
        eml_opcode::TARGET_VALUE => Some(EmitEntry::plain(format!(
            "values_in[context.target_slot * params.n_dims + {}u]",
            node.a
        ))),
        eml_opcode::NEIGHBOR_VALUE => Some(EmitEntry::plain(format!(
            "values_in[context.neighbor_slot * params.n_dims + {}u]",
            node.a
        ))),
        eml_opcode::PARAM => Some(EmitEntry::plain(field_param_expression(node.a).to_owned())),
        eml_opcode::NEG
        | eml_opcode::CLAMP_BOUNDED
        | eml_opcode::CLAMP_FLOORED
        | eml_opcode::ABS
        | eml_opcode::FLOOR
        | eml_opcode::EXP
        | eml_opcode::LN => {
            let operand = stack.pop().expect("admitted unary postfix operand");
            Some(EmitEntry::plain(match node.opcode {
                eml_opcode::NEG => format!("-{}", operand.expr),
                eml_opcode::CLAMP_BOUNDED => format!(
                    "clamp({}, bitcast<f32>(0x{:08x}u), bitcast<f32>(0x{:08x}u))",
                    operand.expr, node.a, node.b
                ),
                eml_opcode::CLAMP_FLOORED => {
                    format!("max({}, bitcast<f32>(0x{:08x}u))", operand.expr, node.a)
                }
                eml_opcode::ABS => format!("abs({})", operand.expr),
                eml_opcode::FLOOR => format!("floor({})", operand.expr),
                eml_opcode::EXP => format!("eml_exp_pinned({})", operand.expr),
                eml_opcode::LN => format!("eml_ln_pinned({})", operand.expr),
                _ => unreachable!(),
            }))
        }
        eml_opcode::SELECT => {
            let false_value = stack.pop().expect("admitted select false operand");
            let true_value = stack.pop().expect("admitted select true operand");
            let condition = stack.pop().expect("admitted select condition operand");
            Some(EmitEntry::plain(format!(
                "select({}, {}, {} != 0.0)",
                false_value.expr, true_value.expr, condition.expr
            )))
        }
        eml_opcode::RETURN_TOP => {
            let value = stack.last().expect("admitted return operand");
            output.push_str(&format!("    return {};\n", value.expr));
            None
        }
        eml_opcode::ADD | eml_opcode::SUB => {
            let rhs = stack.pop().expect("admitted binary rhs");
            let lhs = stack.pop().expect("admitted binary lhs");
            let is_sub = node.opcode == eml_opcode::SUB;
            let expr = match (&lhs.mul, &rhs.mul) {
                (Some((a, b)), None) => {
                    if is_sub {
                        format!("fma({a}, {b}, -({}))", rhs.expr)
                    } else {
                        format!("fma({a}, {b}, {})", rhs.expr)
                    }
                }
                (None, Some((a, b))) => {
                    if is_sub {
                        format!("fma(-({a}), {b}, {})", lhs.expr)
                    } else {
                        format!("fma({a}, {b}, {})", lhs.expr)
                    }
                }
                _ => {
                    if is_sub {
                        format!("{} - {}", lhs.expr, rhs.expr)
                    } else {
                        format!("{} + {}", lhs.expr, rhs.expr)
                    }
                }
            };
            Some(EmitEntry::plain(expr))
        }
        eml_opcode::MUL => {
            let rhs = stack.pop().expect("admitted binary rhs");
            let lhs = stack.pop().expect("admitted binary lhs");
            Some(EmitEntry {
                expr: format!("{} * {}", lhs.expr, rhs.expr),
                mul: Some((lhs.expr, rhs.expr)),
            })
        }
        opcode => {
            let rhs = stack.pop().expect("admitted binary rhs");
            let lhs = stack.pop().expect("admitted binary lhs");
            Some(EmitEntry::plain(binary_expression(
                opcode, &lhs.expr, &rhs.expr,
            )))
        }
    }
}

fn emit_program(name: &str, nodes: &[EmlNodeGpu]) -> String {
    let mut output = format!("fn {name}(context: FieldEmlContext) -> f32 {{\n");
    let mut stack: Vec<EmitEntry> = Vec::new();
    let mut next_value = 0u32;
    for node in nodes {
        if let Some(entry) = expression_for(node, &mut stack, &mut output) {
            let value = format!("v{next_value}");
            next_value += 1;
            output.push_str(&format!("    let {value}: f32 = {};\n", entry.expr));
            stack.push(EmitEntry {
                expr: value,
                mul: entry.mul,
            });
        }
    }
    output.push_str("}\n");
    output
}

fn field_param_expression(index: u32) -> &'static str {
    match index {
        0 => "f32(context.target_slot)",
        1 => "f32(context.neighbor_slot)",
        2 => "context.accumulator",
        3 => "context.edge_scalar",
        4 => "context.dt",
        5 => "context.mapped",
        6 => "context.folded",
        7 => "context.target_transient",
        8 => "context.neighbor_transient",
        _ => unreachable!("field program admission seals PARAM indices"),
    }
}

fn binary_expression(opcode: u32, lhs: &str, rhs: &str) -> String {
    match opcode {
        eml_opcode::ADD => format!("{lhs} + {rhs}"),
        eml_opcode::SUB => format!("{lhs} - {rhs}"),
        eml_opcode::MUL => format!("{lhs} * {rhs}"),
        eml_opcode::DIV => format!("{lhs} / {rhs}"),
        eml_opcode::MIN => format!("min({lhs}, {rhs})"),
        eml_opcode::MAX => format!("max({lhs}, {rhs})"),
        eml_opcode::CMP_LT => format!("select(0.0, 1.0, {lhs} < {rhs})"),
        eml_opcode::CMP_LE => format!("select(0.0, 1.0, {lhs} <= {rhs})"),
        eml_opcode::CMP_GT => format!("select(0.0, 1.0, {lhs} > {rhs})"),
        eml_opcode::CMP_GE => format!("select(0.0, 1.0, {lhs} >= {rhs})"),
        eml_opcode::CMP_EQ => format!("select(0.0, 1.0, {lhs} == {rhs})"),
        _ => unreachable!("field program admission seals binary opcodes"),
    }
}

#[cfg(test)]
mod eml_exp_lowering_tests {
    use super::*;
    use simthing_core::EmlResourceClass;

    fn node(opcode: u32, a: u32, b: u32) -> EmlNodeGpu {
        EmlNodeGpu {
            opcode,
            flags: 0,
            a,
            b,
            c: 0,
            d: 0,
        }
    }

    fn exp_post_program() -> Vec<EmlNodeGpu> {
        vec![
            node(eml_opcode::TARGET_VALUE, 0, 0),
            node(
                eml_opcode::CLAMP_BOUNDED,
                simthing_core::EML_EXP_DOMAIN_MIN_BITS,
                simthing_core::EML_EXP_DOMAIN_MAX_BITS,
            ),
            node(eml_opcode::EXP, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ]
    }

    /// EML-EXP-PRIMITIVE-0: the JIT lowers EXP as a call to the ONE pinned
    /// helper, and that helper survives evaluator excision because it lives
    /// outside the markers — no second definition, no bespoke shader.
    #[test]
    fn eml_exp_primitive_0_jit_lowering_calls_the_single_pinned_helper() {
        let canonical = include_str!("shaders/field_sweep.wgsl");
        let trivial = vec![
            node(eml_opcode::LITERAL_F32, 0.0f32.to_bits(), 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ];
        let generated = generate_field_sweep_jit(
            canonical,
            EmlResourceClass::CompactStack4,
            &trivial,
            &trivial,
            &exp_post_program(),
        );
        assert_eq!(
            generated.matches("fn eml_exp_pinned(").count(),
            1,
            "exactly one pinned helper definition survives excision"
        );
        assert!(
            generated.contains("eml_exp_pinned(v"),
            "the generated straight-line block calls the pinned helper"
        );
        assert!(
            !generated.contains("fn eval_program"),
            "interpreted evaluator is excised from the JIT source"
        );
    }

    /// The two hand-written shader homes carry a byte-identical pinned helper,
    /// and its bitcast constants are exactly the CPU twin's pinned bits — the
    /// referee that keeps the three sequence copies from drifting apart.
    #[test]
    fn eml_exp_primitive_0_wgsl_helper_copies_and_pinned_constants_agree() {
        fn helper_block(source: &str) -> &str {
            let start = source
                .find("fn eml_exp_pinned(")
                .expect("pinned helper present");
            let end = start + source[start..].find("\n}").expect("pinned helper closes") + 2;
            &source[start..end]
        }
        let field = helper_block(include_str!("shaders/field_sweep.wgsl"));
        let accumulator = helper_block(include_str!("shaders/accumulator_op.wgsl"));
        assert_eq!(
            field, accumulator,
            "one pinned sequence, two shader homes, zero drift"
        );
        for bits in [
            simthing_core::eml_exp::EML_EXP_LOG2E.to_bits(),
            simthing_core::eml_exp::EML_EXP_NEG_LN2_HI.to_bits(),
            simthing_core::eml_exp::EML_EXP_NEG_LN2_LO.to_bits(),
            simthing_core::eml_exp::EML_EXP_P5.to_bits(),
            simthing_core::eml_exp::EML_EXP_P4.to_bits(),
            simthing_core::eml_exp::EML_EXP_P3.to_bits(),
            simthing_core::eml_exp::EML_EXP_P2.to_bits(),
            simthing_core::eml_exp::EML_EXP_P1.to_bits(),
            simthing_core::eml_exp::EML_EXP_P0.to_bits(),
        ] {
            let token = format!("bitcast<f32>(0x{bits:08X}u)");
            assert!(
                field.contains(&token),
                "WGSL helper carries pinned constant {token}"
            );
        }
        assert_eq!(
            field.matches("fma(").count(),
            8,
            "exactly eight fused multiply-adds in the pinned sequence"
        );
        assert_eq!(
            field.matches("round(").count(),
            1,
            "exactly one round-ties-even in the pinned sequence"
        );
    }
}
