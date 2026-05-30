fn canonical_qnan() -> u32 {
    return 0x7fc00000u;
}

fn sq24_u48(x: u32) -> vec2<u32> {
    let x_lo = x & 0xffffu;
    let x_hi = x >> 16u;
    let lo_sq = x_lo * x_lo;
    let mid = (x_lo * x_hi) << 1u;
    let hi_sq = x_hi * x_hi;

    let add_low = mid << 16u;
    let lo = lo_sq + add_low;
    let carry = select(0u, 1u, lo < lo_sq);
    let hi = hi_sq + (mid >> 16u) + carry;
    return vec2<u32>(hi, lo);
}

fn shl_u48(v: vec2<u32>, n: u32) -> vec2<u32> {
    if (n == 0u) {
        return v;
    }
    if (n < 32u) {
        return vec2<u32>((v.x << n) | (v.y >> (32u - n)), v.y << n);
    }
    if (n == 32u) {
        return vec2<u32>(v.y, 0u);
    }
    if (n < 64u) {
        return vec2<u32>(v.y << (n - 32u), 0u);
    }
    return vec2<u32>(0u, 0u);
}

fn cmp_u48(a: vec2<u32>, b: vec2<u32>) -> i32 {
    if (a.x > b.x) {
        return 1;
    }
    if (a.x < b.x) {
        return -1;
    }
    if (a.y > b.y) {
        return 1;
    }
    if (a.y < b.y) {
        return -1;
    }
    return 0;
}

fn sub_u48(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let borrow = select(0u, 1u, a.y < b.y);
    let lo = a.y - b.y;
    let hi = a.x - b.x - borrow;
    return vec2<u32>(hi, lo);
}

fn compare_sq_to_x(sy: u32, sx: u32, exu: i32, eyu: i32) -> i32 {
    let sq = sq24_u48(sy);
    let shift = (2 * eyu) - exu - 23;
    if (shift >= 0) {
        let sq_shifted = shl_u48(sq, u32(shift));
        return cmp_u48(sq_shifted, vec2<u32>(0u, sx));
    }
    let x_scaled = shl_u48(vec2<u32>(0u, sx), u32(-shift));
    return cmp_u48(sq, x_scaled);
}

fn isqrt_floor_sig(sx: u32, exu: i32, eyu: i32) -> u32 {
    var lo = 0x00800000u;
    var hi = 0x00ffffffu;
    var best = lo;
    loop {
        if (lo > hi) {
            break;
        }
        let mid = lo + ((hi - lo) >> 1u);
        let cmp = compare_sq_to_x(mid, sx, exu, eyu);
        if (cmp <= 0) {
            best = mid;
            lo = mid + 1u;
        } else {
            if (mid == 0u) {
                break;
            }
            hi = mid - 1u;
        }
    }
    return best;
}

fn round_sig_nearest_even(floor_sig: u32, sx: u32, exu: i32, eyu: i32) -> u32 {
    let shift = (2 * eyu) - exu - 23;
    let x_scaled = shl_u48(vec2<u32>(0u, sx), u32(-shift));
    let sq_lo = sq24_u48(floor_sig);
    let diff_lo = sub_u48(x_scaled, sq_lo);

    if (floor_sig == 0x00ffffffu) {
        return floor_sig;
    }

    let up_sig = floor_sig + 1u;
    let sq_up = sq24_u48(up_sig);
    let diff_up = sub_u48(sq_up, x_scaled);
    let cmp = cmp_u48(diff_up, diff_lo);
    if (cmp < 0) {
        return up_sig;
    }
    if (cmp > 0) {
        return floor_sig;
    }
    if ((floor_sig & 1u) != 0u) {
        return up_sig;
    }
    return floor_sig;
}

fn sqrt_cr_e_bits(x_bits: u32) -> u32 {
    let sign = x_bits >> 31u;
    let exp = (x_bits >> 23u) & 0xffu;
    let mant = x_bits & 0x007fffffu;

    if (exp == 0xffu) {
        if (mant != 0u) {
            return x_bits | 0x00400000u;
        }
        if (sign == 0u) {
            return 0x7f800000u;
        }
        return canonical_qnan();
    }

    if (sign == 1u) {
        if (exp == 0u && mant == 0u) {
            return 0x80000000u;
        }
        return canonical_qnan();
    }

    if (exp == 0u && mant == 0u) {
        return 0x00000000u;
    }

    var sx = mant;
    var exu = i32(exp) - 127;
    if (exp == 0u) {
        exu = -126;
        while ((sx & 0x00800000u) == 0u) {
            sx = sx << 1u;
            exu = exu - 1;
        }
    } else {
        sx = sx | 0x00800000u;
    }

    var eyu = exu >> 1;
    let floor_sig = isqrt_floor_sig(sx, exu, eyu);
    var sig = round_sig_nearest_even(floor_sig, sx, exu, eyu);
    if (sig == 0x01000000u) {
        sig = 0x00800000u;
        eyu = eyu + 1;
    }

    let exp_out = u32(eyu + 127);
    return (exp_out << 23u) | (sig & 0x007fffffu);
}
