// EML-LN-PRIMITIVE-0 Candidate F (LNCF) — standalone frozen WGSL artifact.
// DA 5186354130 / sqrt_candidates.md §§3–4,6–8 discipline:
//   - vendor `log` seeds only; must not decide
//   - already-admitted `eml_exp_pinned` decides via ±1 ULP neighbor snap
//   - never fold into EXP's artifact; this file is LN's own identity surface
// Authoritative production twins live in:
//   crates/simthing-kernel/src/shaders/field_sweep.wgsl
//   crates/simthing-kernel/src/shaders/accumulator_op.wgsl
//   crates/simthing-core/src/eml_ln.rs
// Any edit here or there is a NEW candidate identity and invalidates qualification.

fn eml_exp_pinned(x: f32) -> f32 {
    let a = x * bitcast<f32>(0x3FB8AA3Bu);
    let kf = round(a);
    let hi = fma(kf, bitcast<f32>(0xBF318000u), x);
    let r = fma(kf, bitcast<f32>(0x395E8083u), hi);
    let z = r * r;
    var p = bitcast<f32>(0x39506967u);
    p = fma(p, r, bitcast<f32>(0x3AB743CEu));
    p = fma(p, r, bitcast<f32>(0x3C088908u));
    p = fma(p, r, bitcast<f32>(0x3D2AA9C1u));
    p = fma(p, r, bitcast<f32>(0x3E2AAAAAu));
    p = fma(p, r, bitcast<f32>(0x3F000000u));
    let q = fma(z, p, r);
    let y = 1.0 + q;
    let k = i32(kf);
    let k1 = k >> 1u;
    let k2 = k - k1;
    let s1 = bitcast<f32>(u32(k1 + 127) << 23u);
    let s2 = bitcast<f32>(u32(k2 + 127) << 23u);
    let y1 = y * s1;
    return y1 * s2;
}

fn f32_next_up(y: f32) -> f32 {
    var bits = bitcast<u32>(y);
    if (y != y) { return y; }
    if (bits == 0x7F800000u) { return y; }
    if (bits == 0x00000000u) { return bitcast<f32>(0x00000001u); }
    if (bits == 0x80000000u) { return 0.0; }
    if ((bits & 0x80000000u) == 0u) { bits = bits + 1u; } else { bits = bits - 1u; }
    return bitcast<f32>(bits);
}
fn f32_next_down(y: f32) -> f32 {
    var bits = bitcast<u32>(y);
    if (y != y) { return y; }
    if (bits == 0xFF800000u) { return y; }
    if (bits == 0x00000000u) { return bitcast<f32>(0x80000000u); }
    if (bits == 0x80000000u) { return bitcast<f32>(0x80000001u); }
    if ((bits & 0x80000000u) == 0u) { bits = bits - 1u; } else { bits = bits + 1u; }
    return bitcast<f32>(bits);
}
fn eml_ln_snap_exp_domain(x: f32, y_dn: f32, e_dn: f32, y0: f32, e0: f32, y_up: f32, e_up: f32) -> f32 {
    let d0 = abs(x - e0);
    let d_up = abs(x - e_up);
    let d_dn = abs(x - e_dn);
    var best_y = y0;
    var best_d = d0;
    var best_bits = bitcast<u32>(y0);
    let up_bits = bitcast<u32>(y_up);
    if (d_up < best_d || (d_up == best_d && (up_bits & 1u) == 0u && (best_bits & 1u) == 1u)) {
        best_y = y_up;
        best_d = d_up;
        best_bits = up_bits;
    }
    let dn_bits = bitcast<u32>(y_dn);
    if (d_dn < best_d || (d_dn == best_d && (dn_bits & 1u) == 0u && (best_bits & 1u) == 1u)) {
        best_y = y_dn;
    }
    return best_y;
}

// Authoritative Candidate-F LN entry (f32).
fn eml_ln_cf(x: f32) -> f32 {
    if (bitcast<u32>(x) == 0x3F800000u) {
        return 0.0;
    }
    let y0 = log(x);
    let e0 = eml_exp_pinned(y0);
    let y_up = f32_next_up(y0);
    let y_dn = f32_next_down(y0);
    let e_up = eml_exp_pinned(y_up);
    let e_dn = eml_exp_pinned(y_dn);
    return eml_ln_snap_exp_domain(x, y_dn, e_dn, y0, e0, y_up, e_up);
}
