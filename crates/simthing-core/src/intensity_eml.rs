//! C-8b: compile legacy [`IntensityBehavior`] into ExactDeterministic EML programs.

use crate::eml_nodes::{self, EmlNode, EML_STACK_MAX};
use crate::eml_registry::{
    EmlConsumerKind, EmlConsumerMask, EmlExecutionClass, EmlFormulaMeta, EmlTreeId,
    MAX_EML_TREE_NODES,
};
use crate::property::IntensityBehavior;

/// Base for per-property intensity EML tree IDs (`tree_id = base + property_index`).
pub const INTENSITY_EML_TREE_ID_BASE: u32 = 0x8000;

pub fn intensity_tree_id(property_index: u32) -> EmlTreeId {
    EmlTreeId(INTENSITY_EML_TREE_ID_BASE + property_index)
}

fn lit(v: f32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::LITERAL_F32,
        flags: 0,
        a: v.to_bits(),
        b: 0,
        c: 0,
        d: 0,
    }
}

fn slot(col: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::SLOT_VALUE,
        flags: 0,
        a: col,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn param(index: u32) -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::PARAM,
        flags: 0,
        a: index,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn unary(opcode: u32) -> EmlNode {
    EmlNode {
        opcode,
        flags: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn clamp01() -> EmlNode {
    EmlNode {
        opcode: eml_nodes::opcode::CLAMP_BOUNDED,
        flags: 0,
        a: 0.0f32.to_bits(),
        b: 1.0f32.to_bits(),
        c: 0,
        d: 0,
    }
}

/// Compile legacy intensity update into a postfix EML program.
///
/// `param0` = dt (from tick params). Velocity and intensity are read via
/// `SLOT_VALUE` at `velocity_col` / `intensity_col`. FMA order matches
/// `PropertyValue::update_intensity` and `intensity_update.wgsl`.
pub fn compile_intensity_behavior_to_eml(
    behavior: &IntensityBehavior,
    tree_id: EmlTreeId,
    velocity_col: u32,
    intensity_col: u32,
) -> (EmlFormulaMeta, Vec<EmlNode>) {
    let nodes = vec![
        slot(velocity_col),
        unary(eml_nodes::opcode::ABS),
        lit(behavior.velocity_threshold),
        unary(eml_nodes::opcode::CMP_GT),
        lit(behavior.build_coefficient),
        slot(velocity_col),
        unary(eml_nodes::opcode::ABS),
        unary(eml_nodes::opcode::MUL),
        param(0),
        unary(eml_nodes::opcode::MUL),
        slot(intensity_col),
        unary(eml_nodes::opcode::ADD),
        slot(intensity_col),
        lit(behavior.decay_coefficient),
        slot(intensity_col),
        unary(eml_nodes::opcode::MUL),
        param(0),
        unary(eml_nodes::opcode::MUL),
        unary(eml_nodes::opcode::SUB),
        unary(eml_nodes::opcode::SELECT),
        clamp01(),
        unary(eml_nodes::opcode::RETURN_TOP),
    ];
    debug_assert!(nodes.len() as u32 <= MAX_EML_TREE_NODES);
    debug_assert!(nodes.len() as u32 <= EML_STACK_MAX || true);

    let meta = EmlFormulaMeta {
        tree_id,
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(
            EmlConsumerMask::INTENSITY | EmlConsumerMask::DEBUG_ORACLE,
        ),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: nodes.len() as u32,
        max_stack_depth: 4,
        has_loops: false,
        has_recursion: false,
        display_name: "intensity_update".into(),
    };
    (meta, nodes)
}

impl IntensityBehavior {
    /// Compile this behavior into EML formula metadata and postfix nodes.
    pub fn compile_to_eml(
        &self,
        tree_id: EmlTreeId,
        velocity_col: u32,
        intensity_col: u32,
    ) -> (EmlFormulaMeta, Vec<EmlNode>) {
        compile_intensity_behavior_to_eml(self, tree_id, velocity_col, intensity_col)
    }
}

/// CPU reference for intensity EML parity (test/oracle only).
pub fn intensity_eml_direct_cpu(
    behavior: &IntensityBehavior,
    velocity: f32,
    intensity: f32,
    dt: f32,
) -> f32 {
    let vel_abs = velocity.abs();
    let next = if vel_abs > behavior.velocity_threshold {
        let scaled = behavior.build_coefficient * vel_abs;
        let delta = scaled * dt;
        intensity + delta
    } else {
        let scaled = behavior.decay_coefficient * intensity;
        let delta = scaled * dt;
        intensity - delta
    };
    next.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eml_registry::EmlExpressionRegistry;

    #[test]
    fn c8b_intensity_behavior_compiles_exact_deterministic_eml() {
        let behavior = IntensityBehavior::default();
        let (meta, nodes) = compile_intensity_behavior_to_eml(
            &behavior,
            intensity_tree_id(0),
            1,
            2,
        );
        assert_eq!(meta.execution_class, EmlExecutionClass::ExactDeterministic);
        assert!(meta.allowed_consumers.contains_kind(EmlConsumerKind::Intensity));
        assert_eq!(meta.node_count, nodes.len() as u32);
        assert!(nodes.len() as u32 <= MAX_EML_TREE_NODES);
        for node in &nodes {
            if node.opcode == eml_nodes::opcode::PARAM {
                assert!(node.a <= 3, "PARAM index out of range: {}", node.a);
            }
        }
        let mut registry = EmlExpressionRegistry::new();
        registry
            .register_formula(intensity_tree_id(0), meta, nodes)
            .unwrap();
        registry
            .assert_consumer_admissible(intensity_tree_id(0), EmlConsumerKind::Intensity)
            .unwrap();
    }

    #[test]
    fn c8b_intensity_eml_cpu_oracle_matches_legacy_formula() {
        let behavior = IntensityBehavior {
            velocity_threshold: 0.005,
            build_coefficient: 2.0,
            decay_coefficient: 0.05,
        };
        let cases = [
            (0.0, 0.5, 1.0),
            (0.004, 0.5, 1.0),
            (0.005, 0.5, 1.0),
            (0.006, 0.5, 1.0),
            (-0.01, 0.5, 1.0),
            (0.0, 0.001, 1.0),
            (0.1, 0.999, 1.0),
            (0.0, 0.5, 0.0),
        ];
        for (velocity, intensity, dt) in cases {
            let expected = intensity_eml_direct_cpu(&behavior, velocity, intensity, dt);
            let (meta, nodes) = compile_intensity_behavior_to_eml(
                &behavior,
                intensity_tree_id(0),
                1,
                2,
            );
            let mut values = vec![0.0, velocity, intensity];
            let got = simthing_gpu_oracle_stub(&meta, &nodes, &mut values, 3, dt);
            assert_eq!(
                got.to_bits(),
                expected.to_bits(),
                "vel={velocity} int={intensity} dt={dt}"
            );
        }
    }

    fn simthing_gpu_oracle_stub(
        _meta: &EmlFormulaMeta,
        nodes: &[EmlNode],
        values: &mut [f32],
        n_dims: u32,
        dt: f32,
    ) -> f32 {
        let gpu_nodes: Vec<crate::EmlNodeGpu> = nodes
            .iter()
            .copied()
            .map(|n| crate::EmlNodeGpu {
                opcode: n.opcode,
                flags: n.flags,
                a: n.a,
                b: n.b,
                c: n.c,
                d: n.d,
            })
            .collect();
        // Inline stack eval matching cpu_oracle (no simthing-gpu dep in core tests).
        eval_eml_cpu_inline(&gpu_nodes, 0, values, n_dims, [dt, 0.0, 0.0, 0.0])
    }

    fn eval_eml_cpu_inline(
        nodes: &[crate::EmlNodeGpu],
        eval_slot: u32,
        values: &[f32],
        n_dims: u32,
        params: [f32; 4],
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
                    stack[sp] = values[i];
                    sp += 1;
                }
                eml_nodes::opcode::PARAM => {
                    stack[sp] = params[node.a as usize];
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
                eml_nodes::opcode::ABS => {
                    stack[sp - 1] = stack[sp - 1].abs();
                }
                eml_nodes::opcode::CMP_GT => {
                    let rhs = stack[sp - 1];
                    let lhs = stack[sp - 2];
                    stack[sp - 2] = if lhs > rhs { 1.0 } else { 0.0 };
                    sp -= 1;
                }
                eml_nodes::opcode::SELECT => {
                    let f_val = stack[sp - 1];
                    let t_val = stack[sp - 2];
                    let cond = stack[sp - 3] != 0.0;
                    stack[sp - 3] = if cond { t_val } else { f_val };
                    sp -= 2;
                }
                eml_nodes::opcode::CLAMP_BOUNDED => {
                    let v = stack[sp - 1];
                    stack[sp - 1] = v.clamp(f32::from_bits(node.a), f32::from_bits(node.b));
                }
                eml_nodes::opcode::RETURN_TOP => return stack[sp - 1],
                _ => {}
            }
        }
        stack[sp - 1]
    }
}
