//! Uniqueness contraction for EvalEML ADD/SUB (5.14).
//!
//! DA `5192270934` / exit-proof `5193244394`: a `MUL` result fuses into its
//! consuming `ADD` or `SUB` iff that fusion is unique. Exactly one immediate
//! `MUL` producer → one-rounding `fma` / `mul_add` (SUB via negated factor or
//! addend). Two or more immediate `MUL` producers → unfused (`U`): each `MUL`
//! rounds to f32 before the consumer. No tie-break.

/// Apply uniqueness contraction at an ADD or SUB consumer.
///
/// `lhs_mul` / `rhs_mul` are `Some((a, b))` when that stack operand is the
/// immediate result of `MUL(a, b)`. Both `Some` or both `None` → unfused.
pub fn uniqueness_add_sub(
    is_sub: bool,
    lhs: f32,
    rhs: f32,
    lhs_mul: Option<(f32, f32)>,
    rhs_mul: Option<(f32, f32)>,
) -> f32 {
    match (lhs_mul, rhs_mul) {
        (Some((a, b)), None) => {
            if is_sub {
                a.mul_add(b, -rhs)
            } else {
                a.mul_add(b, rhs)
            }
        }
        (None, Some((a, b))) => {
            if is_sub {
                (-a).mul_add(b, lhs)
            } else {
                a.mul_add(b, lhs)
            }
        }
        _ => {
            if is_sub {
                lhs - rhs
            } else {
                lhs + rhs
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
