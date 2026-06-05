struct GradientPair {
    dx: f32,
    dy: f32,
}

@group(0) @binding(0) var<storage, read> gradients: array<GradientPair>;
@group(0) @binding(1) var<storage, read_write> max_bits: atomic<u32>;
@group(0) @binding(2) var<uniform> row_count: u32;

const F_QNAN: u32 = 0x7FC00000u;
const F_PINF: u32 = 0x7F800000u;
const Q16_SCALE: f32 = 65536.0;
const U32_SCALE: f32 = 4294967296.0;

struct U64Pair {
    lo: u32,
    hi: u32,
}

fn add_u64(a: U64Pair, b: U64Pair) -> U64Pair {
    let lo = a.lo + b.lo;
    let carry = select(0u, 1u, lo < a.lo);
    return U64Pair(lo, a.hi + b.hi + carry);
}

fn mul_u32_to_u64(a: u32, b: u32) -> U64Pair {
    let a0 = a & 0xFFFFu;
    let a1 = a >> 16u;
    let b0 = b & 0xFFFFu;
    let b1 = b >> 16u;
    let p0 = a0 * b0;
    let p1 = a0 * b1;
    let p2 = a1 * b0;
    let p3 = a1 * b1;
    let carry = (p0 >> 16u) + (p1 & 0xFFFFu) + (p2 & 0xFFFFu);
    let lo = (p0 & 0xFFFFu) | (carry << 16u);
    let hi = p3 + (p1 >> 16u) + (p2 >> 16u) + (carry >> 16u);
    return U64Pair(lo, hi);
}

fn abs_i32_to_u32(v: i32) -> u32 {
    if (v < 0) {
        return u32(-v);
    }
    return u32(v);
}

fn mag2_bits_from_gradient(dx: f32, dy: f32) -> u32 {
    let dx_fixed = i32(round(dx * Q16_SCALE));
    let dy_fixed = i32(round(dy * Q16_SCALE));
    let dx2 = mul_u32_to_u64(abs_i32_to_u32(dx_fixed), abs_i32_to_u32(dx_fixed));
    let dy2 = mul_u32_to_u64(abs_i32_to_u32(dy_fixed), abs_i32_to_u32(dy_fixed));
    let sum = add_u64(dx2, dy2);
    return bitcast<u32>(f32(sum.hi) + (f32(sum.lo) / U32_SCALE));
}

fn sqrt_cr_f_core(m: f32) -> f32 {
    let y0 = sqrt(m);
    let y_hi = bitcast<f32>(bitcast<u32>(y0) & 0xFFFFF000u);
    let y_lo = y0 - y_hi;
    let p = y0 * y0;
    let yhi_yhi = y_hi * y_hi;
    let yhi_ylo = y_hi * y_lo;
    let two_yhi_ylo = yhi_ylo + yhi_ylo;
    let ylo_ylo = y_lo * y_lo;
    let e0 = yhi_yhi - p;
    let e1 = e0 + two_yhi_ylo;
    let e = e1 + ylo_ylo;
    let sp = m - p;
    let r = sp - e;
    let y_up = bitcast<f32>(bitcast<u32>(y0) + 1u);
    let y_dn = bitcast<f32>(bitcast<u32>(y0) - 1u);
    let u_up = y_up - y0;
    let u_dn = y0 - y_dn;
    let t_up = (y0 * u_up) + (0.25 * u_up * u_up);
    let t_dn = (y0 * u_dn) - (0.25 * u_dn * u_dn);
    if (r > t_up) { return y_up; }
    if (r < -t_dn) { return y_dn; }
    return y0;
}

fn sqrt_cr_f_bits(x_bits: u32) -> u32 {
    let sign = x_bits >> 31u;
    let exp = (x_bits >> 23u) & 0xFFu;
    let mant = x_bits & 0x7FFFFFu;
    if (exp == 0xFFu) {
        if (mant != 0u) { return F_QNAN; }
        if (sign == 0u) { return F_PINF; }
        return F_QNAN;
    }
    if (x_bits == 0x00000000u) { return 0x00000000u; }
    if (x_bits == 0x80000000u) { return 0x80000000u; }
    if (sign == 1u) { return F_QNAN; }

    var m2_bits: u32;
    var e2: i32;
    if (exp == 0u) {
        let lz = countLeadingZeros(mant);
        let sh = lz - 8u;
        let frac = (mant << sh) & 0x7FFFFFu;
        m2_bits = 0x3F800000u | frac;
        e2 = -118 - i32(lz);
    } else {
        m2_bits = 0x3F800000u | mant;
        e2 = i32(exp) - 127;
    }
    let k = e2 >> 1u;
    let parity = bitcast<u32>(e2) & 1u;
    let m = bitcast<f32>(m2_bits) * f32(1u << parity);
    let root = sqrt_cr_f_core(m);
    let root_bits = bitcast<u32>(root);
    let final_exp = i32((root_bits >> 23u) & 0xFFu) + k;
    return (u32(final_exp) << 23u) | (root_bits & 0x7FFFFFu);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= row_count) {
        return;
    }
    let g = gradients[idx];
    let mag_bits = sqrt_cr_f_bits(mag2_bits_from_gradient(g.dx, g.dy));
    atomicMax(&max_bits, mag_bits);
}
