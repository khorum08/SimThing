fn is_non_finite_positive_or_nonpositive(x: f32) -> bool {
    if (!(x > 0.0)) { return true; }
    return (bitcast<u32>(x) & 0x7f800000u) >= 0x7f800000u;
}

fn pow2_i32(exp: i32) -> f32 {
    return bitcast<f32>(u32(exp + 127) << 23u);
}

fn snap_directional(y: f32, r: f32) -> f32 {
    let u_up = abs(bitcast<f32>(bitcast<u32>(y) + 1u) - y);
    let u_dn = abs(y - bitcast<f32>(bitcast<u32>(y) - 1u));
    if (r >  (y * u_up + 0.25 * u_up * u_up)) {
        return bitcast<f32>(bitcast<u32>(y) + 1u);
    }
    if (r < -(y * u_dn - 0.25 * u_dn * u_dn)) {
        return bitcast<f32>(bitcast<u32>(y) - 1u);
    }
    return y;
}

fn dekker_residual_hardened(y: f32, s: f32) -> f32 {
    let y_bits = bitcast<u32>(y);
    let y_hi = bitcast<f32>(y_bits & 0xFFFFF000u);
    let y_lo = y - y_hi;
    let p = y * y;
    let yhi_yhi = y_hi * y_hi;
    let term1 = yhi_yhi - p;
    let two_yhi_ylo = 2.0 * y_hi * y_lo;
    let ylo_ylo = y_lo * y_lo;
    let e_part1 = term1 + two_yhi_ylo;
    let e = e_part1 + ylo_ylo;
    let sp = s - p;
    return sp - e;
}

fn sqrt_positive_finite_normal(s: f32) -> f32 {
    let y = sqrt(s);
    let r = dekker_residual_hardened(y, s);
    return snap_directional(y, r);
}

fn sqrt_cr_d_subnormal_integer(x_bits: u32) -> f32 {
    var mant = x_bits & 0x007fffffu;
    var k = 0u;
    while (k < 24u && mant < 0x00800000u) {
        mant = mant << 1u;
        k = k + 1u;
    }
    var exp_field = 1u + k - 23u;
    var norm_mant = mant & 0x007fffffu;
    if (exp_field == 0u) {
        exp_field = 1u;
        norm_mant = (mant >> 1u) & 0x007fffffu;
    }
    let s_bits = (exp_field << 23u) | norm_mant;
    let s = bitcast<f32>(s_bits);
    let y = sqrt_positive_finite_normal(s);
    let half_k = i32(k >> 1u);
    var scale = pow2_i32(-half_k);
    if ((k & 1u) != 0u) {
        scale = scale * 0.7071067811865476;
    }
    return y * scale;
}

fn sqrt_cr_d(x: f32) -> f32 {
    if (is_non_finite_positive_or_nonpositive(x)) { return sqrt(x); }
    let x_bits = bitcast<u32>(x);
    let exp = x_bits >> 23u;
    let mant = x_bits & 0x007fffffu;
    if (exp == 0u && mant != 0u) {
        return sqrt_cr_d_subnormal_integer(x_bits);
    }
    return sqrt_positive_finite_normal(x);
}
