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

    #[test]
    fn unique_mul_into_sub_rhs_matches_mul_add() {
        let a = 1.25f32;
        let b = 0.75f32;
        let lhs = 10.0f32;
        let expected = (-a).mul_add(b, lhs);
        let got = uniqueness_add_sub(true, lhs, a * b, None, Some((a, b)));
        assert_eq!(got.to_bits(), expected.to_bits());
    }

    #[test]
    fn two_mul_into_add_stays_unfused() {
        let a = 1.0e20f32;
        let b = 1.0f32;
        let c = 1.0e20f32;
        let d = 1.0f32;
        let lhs = a * b;
        let rhs = c * d;
        let got = uniqueness_add_sub(false, lhs, rhs, Some((a, b)), Some((c, d)));
        assert_eq!(got.to_bits(), (lhs + rhs).to_bits());
    }
}
