// Candidate F — correctly-rounded f32 sqrt. Loop-free, fixed op count, hardware-sqrt seed.
//
// Built for the 34k-entity hot path: NO data-dependent loop (uniform cost across all lanes,
// no warp divergence), one hardware `sqrt` + straight-line ops. Correctness strategy: decode
// bits in the integer domain; fold the exponent so the work runs on a significand m in [1,4)
// (sqrt(x) = sqrt(m) * 2^k, k integer); seed with the hardware sqrt; get the EXACT residual
// via a bitmask-split Dekker TwoProduct (no fma); snap to correctly-rounded with a directional
// Markstein test; reconstruct the result exponent with integer ops and return the BIT PATTERN.
// Every live f32 value stays in [1,4) or [1,2) — always normal, so FTZ never touches a
// residual term — and the result is emitted as u32 so no subnormal f32 store/load can flush.
//
// Authoritative entry:  sqrt_cr_f_bits(x_bits: u32) -> u32
// CPU oracle for parity: f32::sqrt (already correctly rounded).
//
// Contraction safety: the split is a bitwise AND the FP optimizer cannot rewrite, and the
// residual has NO `a*b + c` fma candidates — every product is materialized into a `let` and
// `p` is read twice, so naga/DXC cannot fuse it. The snap thresholds may be contracted
// harmlessly (a 1-ULP error there cannot flip a decision; sqrt has no exact ties). The one
// open question the standalone exhaustive sweep must settle is whether DXC reassociates the
// error-term SUM despite the let-sequencing; if it does, the fix is a contraction barrier on
// those ~4 lines — NOT a switch to an integer loop (which would forfeit the 34k budget).

const F_QNAN: u32 = 0x7FC00000u;
const F_PINF: u32 = 0x7F800000u;

// Correctly-rounded sqrt of m in [1,4); result in [1,2]. All intermediates stay normal.
fn sqrt_cr_f_core(m: f32) -> f32 {
    let y0 = sqrt(m);                                       // hardware seed, ~1 ULP, in [1,2)

    // Bitmask split: clear the low 12 mantissa bits. y_hi has 12 significant bits, so
    // y_hi*y_hi is <=24 bits and EXACT in f32. (Integer AND — optimizer cannot touch it.)
    let y_hi = bitcast<f32>(bitcast<u32>(y0) & 0xFFFFF000u);
    let y_lo = y0 - y_hi;

    // Exact residual r = m - y0*y0 (Dekker TwoProduct). No fma candidates anywhere.
    let p           = y0 * y0;          // read twice below -> compiler must materialize it
    let yhi_yhi     = y_hi * y_hi;      // exact
    let yhi_ylo     = y_hi * y_lo;      // exact
    let two_yhi_ylo = yhi_ylo + yhi_ylo;
    let ylo_ylo     = y_lo * y_lo;      // exact
    let e0 = yhi_yhi - p;               // exact (Sterbenz)
    let e1 = e0 + two_yhi_ylo;
    let e  = e1 + ylo_ylo;              // e == y0*y0 - p exactly
    let sp = m - p;                     // exact (Sterbenz)
    let r  = sp - e;                    // EXACT residual m - y0*y0

    // Directional Markstein snap (sqrt has no exact ties -> strict comparison is safe).
    let y_up = bitcast<f32>(bitcast<u32>(y0) + 1u);
    let y_dn = bitcast<f32>(bitcast<u32>(y0) - 1u);
    let u_up = y_up - y0;               // gap above
    let u_dn = y0 - y_dn;               // gap below (half of u_up at a power of two)
    let t_up = (y0 * u_up) + (0.25 * u_up * u_up);
    let t_dn = (y0 * u_dn) - (0.25 * u_dn * u_dn);
    if (r >  t_up) { return y_up; }
    if (r < -t_dn) { return y_dn; }
    return y0;
}

fn sqrt_cr_f_bits(x_bits: u32) -> u32 {
    let sign = x_bits >> 31u;
    let exp  = (x_bits >> 23u) & 0xFFu;
    let mant = x_bits & 0x7FFFFFu;

    // Special values, bit-exact with f32::sqrt.
    if (exp == 0xFFu) {
        if (mant != 0u) { return F_QNAN; }          // NaN -> NaN
        if (sign == 0u) { return F_PINF; }          // +inf -> +inf
        return F_QNAN;                              // -inf -> NaN
    }
    if (x_bits == 0x00000000u) { return 0x00000000u; }   // +0 -> +0
    if (x_bits == 0x80000000u) { return 0x80000000u; }   // -0 -> -0
    if (sign == 1u) { return F_QNAN; }                   // negative finite -> NaN

    // Recover significand M2 in [1,2) (as bits) and unbiased exponent E2. (countLeadingZeros
    // is a fixed-latency intrinsic, not a loop — no divergence.)
    var m2_bits: u32;
    var e2: i32;
    if (exp == 0u) {
        // Subnormal: normalize the leading 1 to the hidden-bit position via integer shift.
        let lz   = countLeadingZeros(mant);          // mant in [1, 2^23-1] -> lz in [9, 31]
        let sh   = lz - 8u;                           // move leading 1 to bit 23
        let frac = (mant << sh) & 0x7FFFFFu;
        m2_bits = 0x3F800000u | frac;
        e2 = -118 - i32(lz);                          // E2 = (31 - lz) - 149
    } else {
        m2_bits = 0x3F800000u | mant;
        e2 = i32(exp) - 127;
    }

    // Fold to an EVEN exponent: x = m * 2^(2k), m in [1,4). sqrt(x) = sqrt(m) * 2^k.
    let k      = e2 >> 1u;                            // floor(E2/2), arithmetic shift
    let parity = bitcast<u32>(e2) & 1u;              // 0 -> m in [1,2); 1 -> m in [2,4)
    let m      = bitcast<f32>(m2_bits) * f32(1u << parity);   // *1 or *2, exact

    let root = sqrt_cr_f_core(m);                     // correctly rounded, in [1,2]

    // Reconstruct: final exponent = root's exponent field + k. Result is always normal
    // (sqrt domain [2^-74.5, 2^64)), so this never overflows/underflows.
    let root_bits = bitcast<u32>(root);
    let final_exp = i32((root_bits >> 23u) & 0xFFu) + k;
    return (u32(final_exp) << 23u) | (root_bits & 0x7FFFFFu);
}

// Opcode-boundary wrapper. PRODUCTION RULE: feed the input column's RAW u32 bits to
// sqrt_cr_f_bits — do NOT load the column as an f32 register first, because DAZ can flush a
// subnormal input on load. Prefer wiring sqrt_cr_f_bits directly to the raw column bits.
fn sqrt_cr_f(x: f32) -> f32 {
    return bitcast<f32>(sqrt_cr_f_bits(bitcast<u32>(x)));
}
