//! Sealed authored deformation for unresolved-demand persistence.
//!
//! The substrate owns creation of unresolved `U`. This type admits only a
//! bounded EML transformation of that already-created quantity; it does not
//! carry demand, choose a generation, or publish a second persistence lane.
//!
//! A planted shadow lane is not part of the engine vocabulary:
//!
//! ```compile_fail,E0432
//! use simthing_core::ShadowPersistence;
//! ```

use crate::{eml_opcode as opcode, TransformOp, EML_STACK_MAX};
use thiserror::Error;

/// Largest integer domain that ordinary binary32 EML can represent exactly.
///
/// An absent deformation never enters binary32 and therefore retains the full
/// native `u32` demand domain. This bound applies only when an author elects to
/// install a deformation program.
pub const MAX_EXACT_PERSISTENCE_DEFORMATION_CAP: u32 = 16_777_216;

/// One admitted persistence-policy program.
///
/// `cap` is both the admitted input domain and the sealed output envelope:
/// every `u` accepted by this program is in `0..=cap`, and admission proves
/// that the program result is finite and in that same interval. Runtime checks
/// remain as fail-closed witnesses; no result is ever silently clamped.
#[derive(Clone, Debug, PartialEq)]
pub struct PersistenceDeformationProgram {
    value_program: TransformOp,
    cap: u32,
}

// TransformOp equality is bitwise node equality; the wrapper therefore has a
// lawful total equality relation without widening TransformOp's public API.
impl Eq for PersistenceDeformationProgram {}

impl PersistenceDeformationProgram {
    /// Admit an ordinary EML value program for the closed `0..=cap` domain.
    pub fn admit(
        value_program: TransformOp,
        cap: u32,
    ) -> Result<Self, PersistenceDeformationAdmissionError> {
        if cap > MAX_EXACT_PERSISTENCE_DEFORMATION_CAP {
            return Err(
                PersistenceDeformationAdmissionError::CapNotExactlyRepresentable {
                    cap,
                    maximum: MAX_EXACT_PERSISTENCE_DEFORMATION_CAP,
                },
            );
        }
        let output = validate_bounded_program(&value_program, cap)?;
        if output.lo < 0.0 {
            return Err(PersistenceDeformationAdmissionError::MayProduceNegative {
                minimum_bits: output.lo.to_bits(),
            });
        }
        if output.hi > cap as f32 {
            return Err(PersistenceDeformationAdmissionError::MayExceedCap {
                maximum_bits: output.hi.to_bits(),
                cap,
            });
        }
        let zero_output = eval_deformation_eml(&value_program, 0.0);
        if zero_output != 0.0 {
            return Err(
                PersistenceDeformationAdmissionError::MayCreateWithoutUnresolved {
                    output_bits: zero_output.to_bits(),
                },
            );
        }
        Ok(Self { value_program, cap })
    }

    pub const fn cap(&self) -> u32 {
        self.cap
    }

    pub fn value_program(&self) -> &TransformOp {
        &self.value_program
    }

    /// Evaluate the sealed policy and project its finite non-negative value
    /// into the exact discrete demand domain by flooring once.
    ///
    /// Flooring is the declared type projection, not a bound repair: the
    /// unquantized value is checked against the admitted envelope first.
    pub fn deform(&self, unresolved: u32) -> Result<u32, PersistenceDeformationError> {
        if unresolved > self.cap {
            return Err(PersistenceDeformationError::InputExceedsCap {
                unresolved,
                cap: self.cap,
            });
        }
        let output = eval_deformation_eml(&self.value_program, unresolved as f32);
        if !output.is_finite() {
            return Err(PersistenceDeformationError::NonFiniteOutput);
        }
        if output < 0.0 {
            return Err(PersistenceDeformationError::NegativeOutput {
                output_bits: output.to_bits(),
            });
        }
        if output > self.cap as f32 {
            return Err(PersistenceDeformationError::OutputExceedsCap {
                output_bits: output.to_bits(),
                cap: self.cap,
            });
        }
        Ok(output.floor() as u32)
    }
}

fn eval_deformation_eml(program: &TransformOp, unresolved: f32) -> f32 {
    let mut stack = [0.0f32; EML_STACK_MAX as usize];
    let mut sp = 0usize;
    for node in program.nodes() {
        match node.opcode {
            opcode::LITERAL_F32 => {
                stack[sp] = f32::from_bits(node.a);
                sp += 1;
            }
            opcode::PARAM => {
                stack[sp] = unresolved;
                sp += 1;
            }
            opcode::ADD
            | opcode::SUB
            | opcode::MUL
            | opcode::DIV
            | opcode::MIN
            | opcode::MAX
            | opcode::CMP_LT
            | opcode::CMP_LE
            | opcode::CMP_GT
            | opcode::CMP_GE
            | opcode::CMP_EQ => {
                let rhs = stack[sp - 1];
                let lhs = stack[sp - 2];
                stack[sp - 2] = match node.opcode {
                    opcode::ADD => lhs + rhs,
                    opcode::SUB => lhs - rhs,
                    opcode::MUL => lhs * rhs,
                    opcode::DIV => lhs / rhs,
                    opcode::MIN => lhs.min(rhs),
                    opcode::MAX => lhs.max(rhs),
                    opcode::CMP_LT => {
                        if lhs < rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    opcode::CMP_LE => {
                        if lhs <= rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    opcode::CMP_GT => {
                        if lhs > rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    opcode::CMP_GE => {
                        if lhs >= rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    opcode::CMP_EQ => {
                        if lhs == rhs {
                            1.0
                        } else {
                            0.0
                        }
                    }
                    _ => unreachable!(),
                };
                sp -= 1;
            }
            opcode::NEG => stack[sp - 1] = -stack[sp - 1],
            opcode::ABS => stack[sp - 1] = stack[sp - 1].abs(),
            opcode::FLOOR => stack[sp - 1] = stack[sp - 1].floor(),
            opcode::CLAMP_BOUNDED => {
                stack[sp - 1] = stack[sp - 1].clamp(f32::from_bits(node.a), f32::from_bits(node.b));
            }
            opcode::CLAMP_FLOORED => {
                stack[sp - 1] = stack[sp - 1].max(f32::from_bits(node.a));
            }
            opcode::SELECT => {
                let false_value = stack[sp - 1];
                let true_value = stack[sp - 2];
                let condition = stack[sp - 3] != 0.0;
                stack[sp - 3] = if condition { true_value } else { false_value };
                sp -= 2;
            }
            opcode::RETURN_TOP => return stack[sp - 1],
            _ => unreachable!("admission excludes unsupported deformation opcodes"),
        }
    }
    stack[sp - 1]
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PersistenceDeformationAdmissionError {
    #[error("persistence deformation cap {cap} exceeds exact binary32 integer maximum {maximum}")]
    CapNotExactlyRepresentable { cap: u32, maximum: u32 },
    #[error("persistence deformation EML is empty")]
    EmptyProgram,
    #[error("persistence deformation EML stack underflow at node {node}")]
    StackUnderflow { node: usize },
    #[error("persistence deformation EML stack exceeds {maximum} slots at node {node}")]
    StackOverflow { node: usize, maximum: u32 },
    #[error("persistence deformation EML leaves {observed} values; exactly one is required")]
    InvalidFinalStack { observed: usize },
    #[error("persistence deformation EML RETURN_TOP must be the final node")]
    NonTerminalReturn,
    #[error(
        "persistence deformation EML parameter {parameter} is not admitted; only U is PARAM(0)"
    )]
    UnsupportedParameter { parameter: u32 },
    #[error("persistence deformation EML opcode {opcode} at node {node} is not admitted")]
    UnsupportedOpcode { node: usize, opcode: u32 },
    #[error("persistence deformation EML literal at node {node} is non-finite")]
    NonFiniteLiteral { node: usize },
    #[error("persistence deformation EML clamp at node {node} has an invalid finite range")]
    InvalidClamp { node: usize },
    #[error("persistence deformation EML may produce a non-finite result at node {node}")]
    MayProduceNonFinite { node: usize },
    #[error("persistence deformation EML may divide by zero at node {node}")]
    MayDivideByZero { node: usize },
    #[error("persistence deformation EML may produce a negative result ({minimum_bits:#010x})")]
    MayProduceNegative { minimum_bits: u32 },
    #[error("persistence deformation EML may exceed admitted cap {cap} ({maximum_bits:#010x})")]
    MayExceedCap { maximum_bits: u32, cap: u32 },
    #[error(
        "persistence deformation EML produces {output_bits:#010x} at U=0; the substrate alone creates unresolved demand"
    )]
    MayCreateWithoutUnresolved { output_bits: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum PersistenceDeformationError {
    #[error("unresolved persistence input {unresolved} exceeds admitted cap {cap}")]
    InputExceedsCap { unresolved: u32, cap: u32 },
    #[error("persistence deformation produced a non-finite result")]
    NonFiniteOutput,
    #[error("persistence deformation produced a negative result ({output_bits:#010x})")]
    NegativeOutput { output_bits: u32 },
    #[error("persistence deformation result ({output_bits:#010x}) exceeds admitted cap {cap}")]
    OutputExceedsCap { output_bits: u32, cap: u32 },
}

#[derive(Clone, Copy, Debug)]
struct Interval {
    lo: f32,
    hi: f32,
}

impl Interval {
    fn finite(lo: f32, hi: f32, node: usize) -> Result<Self, PersistenceDeformationAdmissionError> {
        if !lo.is_finite() || !hi.is_finite() {
            return Err(PersistenceDeformationAdmissionError::MayProduceNonFinite { node });
        }
        Ok(Self { lo, hi })
    }

    fn endpoints_binary(
        self,
        rhs: Self,
        node: usize,
        op: impl Fn(f32, f32) -> f32,
    ) -> Result<Self, PersistenceDeformationAdmissionError> {
        let values = [
            op(self.lo, rhs.lo),
            op(self.lo, rhs.hi),
            op(self.hi, rhs.lo),
            op(self.hi, rhs.hi),
        ];
        if values.iter().any(|value| !value.is_finite()) {
            return Err(PersistenceDeformationAdmissionError::MayProduceNonFinite { node });
        }
        let lo = values.iter().copied().fold(f32::INFINITY, f32::min);
        let hi = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        Self::finite(lo, hi, node)
    }
}

fn validate_bounded_program(
    program: &TransformOp,
    cap: u32,
) -> Result<Interval, PersistenceDeformationAdmissionError> {
    let nodes = program.nodes();
    if nodes.is_empty() {
        return Err(PersistenceDeformationAdmissionError::EmptyProgram);
    }
    let mut stack = Vec::with_capacity(EML_STACK_MAX as usize);
    for (index, node) in nodes.iter().enumerate() {
        let binary = |stack: &mut Vec<Interval>| {
            let rhs = stack
                .pop()
                .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
            let lhs = stack
                .pop()
                .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
            Ok::<_, PersistenceDeformationAdmissionError>((lhs, rhs))
        };
        match node.opcode {
            opcode::LITERAL_F32 => {
                let value = f32::from_bits(node.a);
                if !value.is_finite() {
                    return Err(PersistenceDeformationAdmissionError::NonFiniteLiteral {
                        node: index,
                    });
                }
                stack.push(Interval {
                    lo: value,
                    hi: value,
                });
            }
            opcode::PARAM => {
                if node.a != 0 {
                    return Err(PersistenceDeformationAdmissionError::UnsupportedParameter {
                        parameter: node.a,
                    });
                }
                stack.push(Interval {
                    lo: 0.0,
                    hi: cap as f32,
                });
            }
            opcode::ADD => {
                let (lhs, rhs) = binary(&mut stack)?;
                stack.push(Interval::finite(lhs.lo + rhs.lo, lhs.hi + rhs.hi, index)?);
            }
            opcode::SUB => {
                let (lhs, rhs) = binary(&mut stack)?;
                stack.push(Interval::finite(lhs.lo - rhs.hi, lhs.hi - rhs.lo, index)?);
            }
            opcode::MUL => {
                let (lhs, rhs) = binary(&mut stack)?;
                stack.push(lhs.endpoints_binary(rhs, index, |left, right| left * right)?);
            }
            opcode::DIV => {
                let (lhs, rhs) = binary(&mut stack)?;
                if rhs.lo <= 0.0 && rhs.hi >= 0.0 {
                    return Err(PersistenceDeformationAdmissionError::MayDivideByZero {
                        node: index,
                    });
                }
                stack.push(lhs.endpoints_binary(rhs, index, |left, right| left / right)?);
            }
            opcode::MIN => {
                let (lhs, rhs) = binary(&mut stack)?;
                stack.push(Interval::finite(
                    lhs.lo.min(rhs.lo),
                    lhs.hi.min(rhs.hi),
                    index,
                )?);
            }
            opcode::MAX => {
                let (lhs, rhs) = binary(&mut stack)?;
                stack.push(Interval::finite(
                    lhs.lo.max(rhs.lo),
                    lhs.hi.max(rhs.hi),
                    index,
                )?);
            }
            opcode::NEG => {
                let value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                stack.push(Interval::finite(-value.hi, -value.lo, index)?);
            }
            opcode::ABS => {
                let value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                let lo = if value.lo <= 0.0 && value.hi >= 0.0 {
                    0.0
                } else {
                    value.lo.abs().min(value.hi.abs())
                };
                stack.push(Interval::finite(
                    lo,
                    value.lo.abs().max(value.hi.abs()),
                    index,
                )?);
            }
            opcode::FLOOR => {
                let value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                stack.push(Interval::finite(value.lo.floor(), value.hi.floor(), index)?);
            }
            opcode::CLAMP_BOUNDED => {
                let value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                let lo = f32::from_bits(node.a);
                let hi = f32::from_bits(node.b);
                if !lo.is_finite() || !hi.is_finite() || lo > hi {
                    return Err(PersistenceDeformationAdmissionError::InvalidClamp { node: index });
                }
                stack.push(Interval::finite(
                    value.lo.clamp(lo, hi),
                    value.hi.clamp(lo, hi),
                    index,
                )?);
            }
            opcode::CLAMP_FLOORED => {
                let value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                let floor = f32::from_bits(node.a);
                if !floor.is_finite() {
                    return Err(PersistenceDeformationAdmissionError::InvalidClamp { node: index });
                }
                stack.push(Interval::finite(
                    value.lo.max(floor),
                    value.hi.max(floor),
                    index,
                )?);
            }
            opcode::CMP_LT | opcode::CMP_LE | opcode::CMP_GT | opcode::CMP_GE | opcode::CMP_EQ => {
                let _ = binary(&mut stack)?;
                stack.push(Interval { lo: 0.0, hi: 1.0 });
            }
            opcode::SELECT => {
                let false_value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                let true_value = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                let _condition = stack
                    .pop()
                    .ok_or(PersistenceDeformationAdmissionError::StackUnderflow { node: index })?;
                stack.push(Interval::finite(
                    true_value.lo.min(false_value.lo),
                    true_value.hi.max(false_value.hi),
                    index,
                )?);
            }
            opcode::RETURN_TOP => {
                if index + 1 != nodes.len() {
                    return Err(PersistenceDeformationAdmissionError::NonTerminalReturn);
                }
            }
            _ => {
                return Err(PersistenceDeformationAdmissionError::UnsupportedOpcode {
                    node: index,
                    opcode: node.opcode,
                });
            }
        }
        if stack.len() > EML_STACK_MAX as usize {
            return Err(PersistenceDeformationAdmissionError::StackOverflow {
                node: index,
                maximum: EML_STACK_MAX,
            });
        }
    }
    if stack.len() != 1 {
        return Err(PersistenceDeformationAdmissionError::InvalidFinalStack {
            observed: stack.len(),
        });
    }
    Ok(stack[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eml_nodes::EmlNode, EmlPerProgramCap};

    fn node(opcode: u32, a: u32, b: u32) -> EmlNode {
        EmlNode {
            opcode,
            flags: 0,
            a,
            b,
            c: 0,
            d: 0,
        }
    }

    #[test]
    fn decay_and_saturation_admit_but_unbounded_escalation_refuses() {
        let decay = PersistenceDeformationProgram::admit(TransformOp::multiply(0.8), 100)
            .expect("bounded decay");
        assert_eq!(decay.deform(100), Ok(80));
        assert_eq!(decay.deform(81), Ok(64));

        let saturated = TransformOp::admit_eml(
            vec![
                node(opcode::PARAM, 0, 0),
                node(opcode::LITERAL_F32, 2.0f32.to_bits(), 0),
                node(opcode::MUL, 0, 0),
                node(opcode::CLAMP_BOUNDED, 0.0f32.to_bits(), 100.0f32.to_bits()),
            ],
            EmlPerProgramCap::DEFAULT,
        )
        .unwrap();
        let saturated = PersistenceDeformationProgram::admit(saturated, 100).unwrap();
        assert_eq!(saturated.deform(80), Ok(100));

        assert!(matches!(
            PersistenceDeformationProgram::admit(TransformOp::multiply(2.0), 100),
            Err(PersistenceDeformationAdmissionError::MayExceedCap { .. })
        ));
    }

    #[test]
    fn nonfinite_and_unbounded_shapes_refuse_at_admission() {
        assert!(matches!(
            PersistenceDeformationProgram::admit(TransformOp::set(f32::NAN), 100),
            Err(PersistenceDeformationAdmissionError::NonFiniteLiteral { .. })
        ));
        assert!(matches!(
            PersistenceDeformationProgram::admit(TransformOp::add(1.0), 100),
            Err(PersistenceDeformationAdmissionError::MayExceedCap { .. })
        ));
        assert!(matches!(
            PersistenceDeformationProgram::admit(TransformOp::set(1.0), 100),
            Err(PersistenceDeformationAdmissionError::MayCreateWithoutUnresolved { .. })
        ));
    }
}
