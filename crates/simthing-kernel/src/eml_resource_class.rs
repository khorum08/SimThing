use simthing_core::{eml_opcode, EmlNodeGpu, EmlResourceClass};

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

    let generated = format!(
        "// EML-JIT-EVALUATOR-BEGIN\n// Mechanically generated from admitted postfix IR.\nconst EML_JIT_RESOURCE_STACK_SLOTS: u32 = {}u;\n{}\n{}\n{}\n// EML-JIT-EVALUATOR-END",
        resource_class.stack_slots(),
        emit_program("eval_map", map),
        emit_program("eval_fold", fold),
        emit_program("eval_post", post),
    );
    let mut source = String::with_capacity(canonical_source.len() + generated.len());
    source.push_str(&canonical_source[..begin]);
    source.push_str(&generated);
    source.push_str(&canonical_source[end..]);
    replace_once(
        &mut source,
        "eval_program(params.map_offset, params.map_count, context)",
        "eval_map(context)",
    );
    replace_once(
        &mut source,
        "eval_program(params.fold_offset, params.fold_count, context)",
        "eval_fold(context)",
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

fn emit_program(name: &str, nodes: &[EmlNodeGpu]) -> String {
    let mut output = format!("fn {name}(context: FieldEmlContext) -> f32 {{\n");
    let mut stack: Vec<String> = Vec::new();
    let mut next_value = 0u32;
    for node in nodes {
        let expression = match node.opcode {
            eml_opcode::LITERAL_F32 => Some(format!("bitcast<f32>(0x{:08x}u)", node.a)),
            eml_opcode::TARGET_VALUE => Some(format!(
                "values_in[context.target_slot * params.n_dims + {}u]",
                node.a
            )),
            eml_opcode::NEIGHBOR_VALUE => Some(format!(
                "values_in[context.neighbor_slot * params.n_dims + {}u]",
                node.a
            )),
            eml_opcode::PARAM => Some(field_param_expression(node.a).to_owned()),
            eml_opcode::NEG
            | eml_opcode::CLAMP_BOUNDED
            | eml_opcode::CLAMP_FLOORED
            | eml_opcode::ABS
            | eml_opcode::FLOOR
            | eml_opcode::EXP
            | eml_opcode::LN => {
                let operand = stack.pop().expect("admitted unary postfix operand");
                Some(match node.opcode {
                    eml_opcode::NEG => format!("-{operand}"),
                    eml_opcode::CLAMP_BOUNDED => format!(
                        "clamp({operand}, bitcast<f32>(0x{:08x}u), bitcast<f32>(0x{:08x}u))",
                        node.a, node.b
                    ),
                    eml_opcode::CLAMP_FLOORED => {
                        format!("max({operand}, bitcast<f32>(0x{:08x}u))", node.a)
                    }
                    eml_opcode::ABS => format!("abs({operand})"),
                    eml_opcode::FLOOR => format!("floor({operand})"),
                    // EML-EXP-PRIMITIVE-0: the JIT block calls the ONE pinned
                    // helper defined in the canonical shader outside the
                    // excised evaluator region — same definition as the
                    // interpreted arm, bit-identical by construction.
                    eml_opcode::EXP => format!("eml_exp_pinned({operand})"),
                    // EML-LN-PRIMITIVE-0: same single-helper lowering pattern as EXP.
                    eml_opcode::LN => format!("eml_ln_pinned({operand})"),
                    _ => unreachable!(),
                })
            }
            eml_opcode::SELECT => {
                let false_value = stack.pop().expect("admitted select false operand");
                let true_value = stack.pop().expect("admitted select true operand");
                let condition = stack.pop().expect("admitted select condition operand");
                Some(format!(
                    "select({false_value}, {true_value}, {condition} != 0.0)"
                ))
            }
            eml_opcode::RETURN_TOP => {
                let value = stack.last().expect("admitted return operand");
                output.push_str(&format!("    return {value};\n"));
                None
            }
            opcode => {
                let rhs = stack.pop().expect("admitted binary rhs");
                let lhs = stack.pop().expect("admitted binary lhs");
                Some(binary_expression(opcode, &lhs, &rhs))
            }
        };
        if let Some(expression) = expression {
            let value = format!("v{next_value}");
            next_value += 1;
            output.push_str(&format!("    let {value}: f32 = {expression};\n"));
            stack.push(value);
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
            let end = start
                + source[start..]
                    .find("\n}")
                    .expect("pinned helper closes")
                + 2;
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

#[cfg(test)]
mod eml_ln_lowering_tests {
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

    fn ln_post_program() -> Vec<EmlNodeGpu> {
        vec![
            node(eml_opcode::TARGET_VALUE, 0, 0),
            node(
                eml_opcode::CLAMP_BOUNDED,
                simthing_core::EML_LN_DOMAIN_MIN_BITS,
                simthing_core::EML_LN_DOMAIN_MAX_BITS,
            ),
            node(eml_opcode::LN, 0, 0),
            node(eml_opcode::RETURN_TOP, 0, 0),
        ]
    }

    #[test]
    fn eml_ln_primitive_0_jit_lowering_calls_the_single_pinned_helper() {
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
            &ln_post_program(),
        );
        assert_eq!(
            generated.matches("fn eml_ln_pinned(").count(),
            1,
            "exactly one pinned LN helper definition survives excision"
        );
        assert!(
            generated.contains("eml_ln_pinned(v"),
            "the generated straight-line block calls the pinned LN helper"
        );
        assert!(
            !generated.contains("fn eval_program"),
            "interpreted evaluator is excised from the JIT source"
        );
    }

    #[test]
    fn eml_ln_primitive_0_wgsl_helper_copies_and_pinned_constants_agree() {
        fn helper_block(source: &str) -> &str {
            let start = source
                .find("fn f32_next_up(")
                .expect("LNCF next_up helper present");
            let pinned = source
                .find("fn eml_ln_pinned(")
                .expect("pinned LN helper present");
            let end = pinned
                + source[pinned..]
                    .find("\n}")
                    .expect("pinned LN helper closes")
                + 2;
            &source[start..end]
        }
        let field = helper_block(include_str!("shaders/field_sweep.wgsl"));
        let accumulator = helper_block(include_str!("shaders/accumulator_op.wgsl"));
        assert_eq!(
            field, accumulator,
            "one pinned LNCF sequence, two shader homes, zero drift"
        );
        assert!(
            field.contains("let y0 = log(x);"),
            "WGSL LNCF seeds with vendor log"
        );
        assert_eq!(
            field.matches("eml_exp_pinned(").count(),
            3,
            "exactly three EXP evaluations decide the ±1 ULP snap"
        );
        assert!(
            field.contains("0x3F800000u"),
            "WGSL LNCF pins the 1.0 -> +0.0 edge"
        );
    }
}
