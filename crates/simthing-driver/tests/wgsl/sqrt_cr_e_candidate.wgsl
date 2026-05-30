fn isqrt_u32(n: u32) -> u32 {
    var rem = n;
    var root = 0u;
    var bit = 1u << 30u;
    while (bit > rem) {
        bit = bit >> 2u;
    }
    while (bit != 0u) {
        if (rem >= root + bit) {
            rem = rem - (root + bit);
            root = (root >> 1u) + bit;
        } else {
            root = root >> 1u;
        }
        bit = bit >> 2u;
    }
    return root;
}

fn canonical_qnan() -> u32 {
    return 0x7fc00000u;
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

    var m = mant;
    var e2 = i32(exp) - 127;
    if (exp == 0u) {
        var shift: i32 = 0;
        while ((m & 0x00800000u) == 0u) {
            m = m << 1u;
            shift = shift + 1;
        }
        e2 = -126 - shift;
    } else {
        m = m | 0x00800000u;
    }

    if ((e2 & 1) != 0) {
        m = m << 1u;
        e2 = e2 - 1;
    }

    let rad = m << 7u;
    var root = isqrt_u32(rad);
    let sq = root * root;
    let rem = rad - sq;
    let tie = (root << 1u) + 1u;
    if (rem > (tie >> 1u) || (rem == (tie >> 1u) && (root & 1u) != 0u)) {
        root = root + 1u;
    }

    var out_m = root << 8u;
    var out_e = (e2 >> 1) + 127;
    if (out_m >= 0x01000000u) {
        out_m = out_m >> 1u;
        out_e = out_e + 1;
    }

    if (out_e >= 255) {
        return 0x7f800000u;
    }
    if (out_e <= 0) {
        let shift = u32(1 - out_e);
        if (shift >= 25u) {
            return 0u;
        }
        var sub = out_m >> shift;
        if ((out_m & ((1u << shift) - 1u)) != 0u) {
            sub = sub + 1u;
        }
        return sub & 0x007fffffu;
    }

    let out_exp = u32(out_e) << 23u;
    let out_mant = out_m & 0x007fffffu;
    return out_exp | out_mant;
}
