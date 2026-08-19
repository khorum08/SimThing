//! EML-LN-PRIMITIVE-0 — pinned algorithm-as-spec for the `LN` exact primitive
//! (candidate LNDS: double-single, table-driven; DA order 5186693435 with the
//! exact-residual reduction refinement).
//!
//! **The sequence below IS the bit semantics**: exact sqrt(2)-folded bit
//! decomposition `x = m*2^k`; 128-cell const reciprocal/log table (every
//! `inv_c` exactly representable with <=9 significant bits; identity cells
//! j=76/77 bracket 1.0 so `ln(1.0)` is exactly `+0.0` and the near-1
//! neighborhood is pure-polynomial with a Sterbenz-exact argument); the
//! reduction residual is carried EXACTLY as a double-single pair (rounded
//! product + `fma` residual; `p_hi - 1` exact by Sterbenz — no reduction bit
//! is ever discarded); degree-5 `ln(1+s)` with two-sum/two-product error
//! carry; reconstruction `k*ln2 + ln_c + ln(1+s)` with the deciding hi-sum
//! EXACT BY AUTHORED GRID (2^-16-aligned LN2_HI and ln_c_hi make
//! `k*LN2_HI + ln_c_hi` exactly representable; the measured optimizer
//! collapse of two-sum error idioms on the certified tuple cannot touch
//! exact arithmetic); ONE final f32 add rounds `(hi, lo)`. Ops: f32 add/sub/mul, single-rounding
//! `fma` (`mul_add`), exact integer/bit steps. No vendor transcendental, no
//! f64, no DIV, no loop, no data-dependent branch.
//!
//! Domain: positive finite normals `[2^-126, f32::MAX]` (bits
//! `0x00800000..=0x7F7FFFFF`, 2,130,706,432 patterns). Out-of-domain inputs
//! are an ADMISSION error (5.10 call-site shapes); the sequence performs no
//! hidden clamping. Admitted semantics are append-only; the algorithm
//! identity below mechanizes invalidation-on-drift.
//!
//! Failure archaeology retained (never resurrect): LN1C
//! `0x108443cfaeeaadfe` RED at `0x008dcb6b` (1 ULP); LNCF
//! `0xbc2f8faa558bb920` (vendor-log seed + EXP-residual ±1 ULP snap) RED at
//! `0x00800000` (2 ULP, 1,478 probe mismatches) — the seed-accuracy
//! assumption this route removes entirely.

/// Sequence revision. Bumping this is minting a new primitive.
pub const EML_LN_SEQUENCE_VERSION: u32 = 3;

/// Domain endpoint bits (positive finite normals).
pub const EML_LN_DOMAIN_MIN_BITS: u32 = 0x0080_0000;
pub const EML_LN_DOMAIN_MAX_BITS: u32 = 0x7F7F_FFFF;
pub const EML_LN_DOMAIN_SIZE: u64 = 2_130_706_432;

/// ln(2) split: EML_LN_LN2_HI is authored ON THE 2^-16 GRID (exact f32,
/// low mantissa bits zero) so `k*LN2_HI` and `k*LN2_HI + ln_c_hi` are both
/// EXACTLY representable — the reconstruction's deciding sum is exact by
/// AUTHORED CONSTRUCTION, immune to compiler reassociation (the measured
/// two-sum-collapse eliminator on the certified tuple cannot break exact
/// arithmetic). EML_LN_LN2_MID carries the next 24 bits.
pub const EML_LN_LN2_HI: f32 = f32::from_bits(0x3F31_7200);
pub const EML_LN_LN2_MID: f32 = f32::from_bits(0x35BF_BE8E);
/// Polynomial tail coefficient 1/3 — bits `0x3EAAAAAB`.
pub const EML_LN_C3: f32 = f32::from_bits(0x3EAA_AAAB);

/// 128-cell `(inv_c, ln_c_hi, ln_c_lo)` table (bit patterns; part of the
/// frozen algorithm identity). `ln_c_hi` on the shared 2^-16 grid (exact
/// sums with `k*LN2_HI`); `ln_c_mid` carries `-ln(inv_c) - ln_c_hi` (~40 bits).
pub const EML_LN_TABLE: [[u32; 3]; 128] = [
    [0x3FB68000, 0xBEB59E00, 0x359BB94D],
    [0x3FB58000, 0xBEB2CE00, 0x3628CFC2],
    [0x3FB48000, 0xBEAFFA00, 0x3678F587],
    [0x3FB38000, 0xBEAD2200, 0x36AA785A],
    [0x3FB28000, 0xBEAA4600, 0x36E9E291],
    [0x3FB18000, 0xBEA76400, 0xB6B99300],
    [0x3FB08000, 0xBEA48000, 0xB5D00801],
    [0x3FAF8000, 0xBEA19800, 0x3686AFD9],
    [0x3FAF0000, 0xBEA02200, 0x36779796],
    [0x3FAE0000, 0xBE9D3200, 0xB6455694],
    [0x3FAD0000, 0xBE9A3E00, 0xB6ECD4C4],
    [0x3FAC0000, 0xBE974800, 0x36EA28F7],
    [0x3FAB0000, 0xBE944A00, 0xB6D09EF4],
    [0x3FAA0000, 0xBE914A00, 0xB4FDE7BD],
    [0x3FA98000, 0xBE8FC800, 0x33C36EBA],
    [0x3FA88000, 0xBE8CC000, 0xB652DD42],
    [0x3FA78000, 0xBE89B400, 0xB5E05275],
    [0x3FA70000, 0xBE882C00, 0xB63F9AE5],
    [0x3FA60000, 0xBE851A00, 0x36D8EC63],
    [0x3FA50000, 0xBE820200, 0x36D35A59],
    [0x3FA48000, 0xBE807400, 0x369DD295],
    [0x3FA38000, 0xBE7AA800, 0xB5A564B8],
    [0x3FA28000, 0xBE746000, 0xB494AA97],
    [0x3FA20000, 0xBE713800, 0xB56DC55E],
    [0x3FA10000, 0xBE6AE000, 0xB685AD3F],
    [0x3FA00000, 0xBE648000, 0x35838656],
    [0x3F9F8000, 0xBE614C00, 0x363D539F],
    [0x3F9E8000, 0xBE5ADC00, 0x36B985C1],
    [0x3F9E0000, 0xBE57A000, 0x36DAC5FD],
    [0x3F9D0000, 0xBE511C00, 0xB6F07F8B],
    [0x3F9C8000, 0xBE4DD800, 0xB6D8B9F8],
    [0x3F9B8000, 0xBE474800, 0xB6A37A22],
    [0x3F9B0000, 0xBE43FC00, 0xB6819483],
    [0x3F9A0000, 0xBE3D5C00, 0xB590210E],
    [0x3F998000, 0xBE3A0800, 0x356157F9],
    [0x3F990000, 0xBE36B000, 0xB5FE719D],
    [0x3F980000, 0xBE2FF800, 0xB6C1C29E],
    [0x3F978000, 0xBE2C9800, 0xB6E36661],
    [0x3F968000, 0xBE25D000, 0xB6DBCE69],
    [0x3F960000, 0xBE226800, 0xB6ADB32E],
    [0x3F958000, 0xBE1F0000, 0x36F53C4D],
    [0x3F948000, 0xBE182000, 0x36A39C6E],
    [0x3F940000, 0xBE14AC00, 0x36B41F80],
    [0x3F938000, 0xBE113400, 0x360737AD],
    [0x3F928000, 0xBE0A3C00, 0xB5308CE8],
    [0x3F920000, 0xBE06BC00, 0x344197B9],
    [0x3F918000, 0xBE033800, 0xB6289653],
    [0x3F908000, 0xBDF85000, 0xB6430046],
    [0x3F900000, 0xBDF13800, 0xB4EDC55E],
    [0x3F8F8000, 0xBDEA1800, 0xB59EB366],
    [0x3F8F0000, 0xBDE2F000, 0xB6A91EB8],
    [0x3F8E0000, 0xBDD49000, 0xB6DA7496],
    [0x3F8D8000, 0xBDCD5800, 0xB6848C40],
    [0x3F8D0000, 0xBDC61800, 0xB68BAC63],
    [0x3F8C8000, 0xBDBED000, 0xB6ECDAF6],
    [0x3F8B8000, 0xBDB03000, 0xB6B1526F],
    [0x3F8B0000, 0xBDA8D800, 0xB4E7E0C3],
    [0x3F8A8000, 0xBDA17800, 0x360D0567],
    [0x3F8A0000, 0xBD9A1000, 0x3621A791],
    [0x3F898000, 0xBD92A000, 0x351D0F5F],
    [0x3F890000, 0xBD8B2800, 0xB65BBA8E],
    [0x3F880000, 0xBD785000, 0xB5C30046],
    [0x3F878000, 0xBD694000, 0x36947525],
    [0x3F870000, 0xBD5A1000, 0xB6DD7119],
    [0x3F868000, 0xBD4AE000, 0xB6830EC9],
    [0x3F860000, 0xBD3BA000, 0xB631EC66],
    [0x3F858000, 0xBD2C5000, 0xB6375F92],
    [0x3F850000, 0xBD1CF000, 0xB687B9FF],
    [0x3F848000, 0xBD0D8000, 0xB6D98924],
    [0x3F840000, 0xBCFC2000, 0x36B278C4],
    [0x3F838000, 0xBCDD0000, 0x357F6142],
    [0x3F830000, 0xBCBDC000, 0xB68D83EB],
    [0x3F828000, 0xBC9E8000, 0x36ADDE5D],
    [0x3F820000, 0xBC7E0000, 0xB5A8B0FC],
    [0x3F818000, 0xBC3F0000, 0x36EE2820],
    [0x3F810000, 0xBBFF0000, 0xB429AC42],
    [0x3F800000, 0x00000000, 0x00000000],
    [0x3F800000, 0x00000000, 0x00000000],
    [0x3F7D0000, 0x3C414000, 0xB6EDD71E],
    [0x3F7B0000, 0x3CA1A000, 0xB6AB6D34],
    [0x3F790000, 0x3CE32000, 0xB5344FAD],
    [0x3F778000, 0x3D0A5000, 0xB560DFFD],
    [0x3F758000, 0x3D2B9000, 0xB6A3B3FC],
    [0x3F738000, 0x3D4D1000, 0xB6709518],
    [0x3F720000, 0x3D666000, 0xB68C3222],
    [0x3F700000, 0x3D843000, 0xB6CE94C4],
    [0x3F6E8000, 0x3D910000, 0x36F6B8F1],
    [0x3F6C8000, 0x3DA24000, 0x36BC07B8],
    [0x3F6B0000, 0x3DAF4800, 0x36B49B2F],
    [0x3F690000, 0x3DC0C800, 0x36FC5E82],
    [0x3F678000, 0x3DCE0800, 0xB6734ACB],
    [0x3F660000, 0x3DDB5800, 0xB65DC94B],
    [0x3F648000, 0x3DE8C000, 0xB6D0EFBD],
    [0x3F630000, 0x3DF63800, 0x36660C28],
    [0x3F610000, 0x3E042C00, 0x3645ACF2],
    [0x3F5F8000, 0x3E0B0800, 0xB6DFF6D3],
    [0x3F5E0000, 0x3E11EC00, 0xB5ED5B64],
    [0x3F5C8000, 0x3E18DC00, 0x364A69D2],
    [0x3F5B0000, 0x3E1FDC00, 0xB6E9699B],
    [0x3F598000, 0x3E26E400, 0xB50ED088],
    [0x3F588000, 0x3E2B9C00, 0xB4CDBF9D],
    [0x3F570000, 0x3E32BC00, 0xB6C505D0],
    [0x3F558000, 0x3E39E400, 0x36E41D3F],
    [0x3F540000, 0x3E412000, 0xB6FA6AB9],
    [0x3F528000, 0x3E486400, 0xB612301A],
    [0x3F518000, 0x3E4D4400, 0xB5872145],
    [0x3F500000, 0x3E54A000, 0xB6161BA9],
    [0x3F4E8000, 0x3E5C0800, 0x363985C1],
    [0x3F4D8000, 0x3E610000, 0x36A2AC60],
    [0x3F4C0000, 0x3E688000, 0x36DFC995],
    [0x3F4B0000, 0x3E6D8800, 0x36F6C352],
    [0x3F498000, 0x3E752400, 0xB6ED83ED],
    [0x3F488000, 0x3E7A3C00, 0xB6D3B2C8],
    [0x3F470000, 0x3E80F600, 0xB68D4ECA],
    [0x3F460000, 0x3E838A00, 0xB5F3F655],
    [0x3F450000, 0x3E862200, 0xB694C4F5],
    [0x3F438000, 0x3E8A0C00, 0xB6C0864C],
    [0x3F428000, 0x3E8CAC00, 0xB6962322],
    [0x3F418000, 0x3E8F5000, 0xB6F4C3BB],
    [0x3F400000, 0x3E934C00, 0xB6EF7659],
    [0x3F3F0000, 0x3E95F800, 0xB6783237],
    [0x3F3E0000, 0x3E98A800, 0xB661E2CA],
    [0x3F3D0000, 0x3E9B5C00, 0xB6C44A0F],
    [0x3F3C0000, 0x3E9E1200, 0x3693B99A],
    [0x3F3A8000, 0x3EA22C00, 0x368F029D],
    [0x3F398000, 0x3EA4EE00, 0xB6C0621A],
    [0x3F388000, 0x3EA7B200, 0xB6014456],
    [0x3F378000, 0x3EAA7A00, 0x3546DEF8],
];

/// FNV-1a-64 identity over version, order tag, ln2 pair, poly coefficients,
/// domain, and every table word. Artifacts pin it; drift invalidates.
pub const EML_LN_ALGORITHM_IDENTITY: u64 = eml_ln_algorithm_identity();

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

const fn fnv_fold_word(mut hash: u64, word: u32) -> u64 {
    let mut b = 0;
    while b < 4 {
        hash ^= ((word >> (b * 8)) & 0xFF) as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
        b += 1;
    }
    hash
}

const fn eml_ln_algorithm_identity() -> u64 {
    let mut hash = FNV_OFFSET;
    hash = fnv_fold_word(hash, EML_LN_SEQUENCE_VERSION);
    hash = fnv_fold_word(hash, 0x4C4E_4434); // "LND4" (grid-exact, two-sum-free)
    hash = fnv_fold_word(hash, EML_LN_LN2_HI.to_bits());
    hash = fnv_fold_word(hash, EML_LN_LN2_MID.to_bits());
    hash = fnv_fold_word(hash, EML_LN_C3.to_bits());
    hash = fnv_fold_word(hash, 0x3E4C_CCCD); // 0.2 (poly c5)
    hash = fnv_fold_word(hash, 0xBE80_0000); // -0.25 (poly c4)
    hash = fnv_fold_word(
        hash,
        EML_LN_DOMAIN_MIN_BITS ^ EML_LN_DOMAIN_MAX_BITS.rotate_left(16),
    );
    let mut i = 0;
    while i < 128 {
        hash = fnv_fold_word(hash, EML_LN_TABLE[i][0]);
        hash = fnv_fold_word(hash, EML_LN_TABLE[i][1]);
        hash = fnv_fold_word(hash, EML_LN_TABLE[i][2]);
        i += 1;
    }
    hash
}

/// The pinned `LN` sequence over raw domain bits — the CPU twin every GPU arm
/// must match bit-for-bit. Step order is load-bearing.
#[inline]
pub fn eml_ln_pinned_bits(x_bits: u32) -> u32 {
    let t = x_bits.wrapping_sub(0x3F33_0000);
    let k = (t as i32) >> 23;
    let m_bits = x_bits.wrapping_sub((k as u32) << 23);
    let j = ((t >> 16) & 0x7F) as usize;
    let m = f32::from_bits(m_bits);
    let inv = f32::from_bits(EML_LN_TABLE[j][0]);
    let lnc_hi = f32::from_bits(EML_LN_TABLE[j][1]);
    let lnc_mid = f32::from_bits(EML_LN_TABLE[j][2]);
    // Exact reduction residual: s carried as double-single (fma survives the
    // certified tuple's optimizer; p_hi - 1 exact by Sterbenz).
    let p_hi = m * inv;
    let p_err = m.mul_add(inv, -p_hi);
    let s = p_hi - 1.0;
    let s_lo = p_err;
    // Degree-5 ln(1+s); every product either exact, inside an fma, or a
    // small-magnitude term (no collapsible cancellation idiom anywhere).
    let poly = s.mul_add(0.2_f32, -0.25_f32);
    let poly = s.mul_add(poly, EML_LN_C3);
    let z = s * s;
    let sp = s * poly;
    let r1 = z.mul_add(sp, -0.5 * z);
    let slo_term = (-s_lo).mul_add(s, s_lo);
    // Reconstruction: t_hi = k*LN2_HI + ln_c_hi is EXACT (grid law); the
    // 40-bit tails ride one fma + small adds; ONE final add rounds (hi, lo).
    let kf = k as f32;
    let t_hi = kf.mul_add(EML_LN_LN2_HI, lnc_hi);
    let mid = kf.mul_add(EML_LN_LN2_MID, lnc_mid);
    let low = mid + (slo_term + r1);
    let g1 = low + s;
    (t_hi + g1).to_bits()
}

/// f32-boundary wrapper (opcode arm form).
#[inline]
pub fn eml_ln_pinned_f32(x: f32) -> f32 {
    f32::from_bits(eml_ln_pinned_bits(x.to_bits()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eml_ln_primitive_0_unity_is_exactly_positive_zero() {
        assert_eq!(eml_ln_pinned_bits(0x3F80_0000), 0x0000_0000);
    }

    #[test]
    fn eml_ln_primitive_0_pinned_edges_hold_their_bits() {
        // min_normal: the exact input that killed LNCF at 2 ULP.
        assert_eq!(eml_ln_pinned_bits(0x0080_0000), 0xC2AE_AC50);
        assert_eq!(eml_ln_pinned_bits(0x7F7F_FFFF), 0x42B1_7218);
        assert_eq!(eml_ln_pinned_bits(0x3F80_0001), 0x33FF_FFFF);
        assert_eq!(eml_ln_pinned_bits(0x3F7F_FFFF), 0xB380_0000);
    }

    #[test]
    fn eml_ln_primitive_0_identity_cells_and_table_shape_hold() {
        assert_eq!(EML_LN_TABLE[76][0], 1.0_f32.to_bits());
        assert_eq!(EML_LN_TABLE[77][0], 1.0_f32.to_bits());
        assert_eq!(EML_LN_TABLE[76][1], 0);
        assert_eq!(EML_LN_TABLE[77][2], 0);
        for row in EML_LN_TABLE {
            let mant = (row[0] & 0x7F_FFFF) | 0x80_0000;
            let significant = 24 - mant.trailing_zeros();
            assert!(
                significant <= 9,
                "inv_c bits {:#010X} exceeds 9 significant bits",
                row[0]
            );
        }
    }

    #[test]
    fn eml_ln_primitive_0_algorithm_identity_is_nonzero_and_table_bound() {
        assert_ne!(EML_LN_ALGORITHM_IDENTITY, 0);
        // recompute with word 0 perturbed by one ULP: identity must move
        let mut hash = FNV_OFFSET;
        hash = fnv_fold_word(hash, EML_LN_SEQUENCE_VERSION);
        hash = fnv_fold_word(hash, 0x4C4E_4434);
        hash = fnv_fold_word(hash, EML_LN_LN2_HI.to_bits());
        hash = fnv_fold_word(hash, EML_LN_LN2_MID.to_bits());
        hash = fnv_fold_word(hash, EML_LN_C3.to_bits());
        hash = fnv_fold_word(hash, 0x3E4C_CCCD);
        hash = fnv_fold_word(hash, 0xBE80_0000);
        hash = fnv_fold_word(
            hash,
            EML_LN_DOMAIN_MIN_BITS ^ EML_LN_DOMAIN_MAX_BITS.rotate_left(16),
        );
        let mut i = 0;
        while i < 128 {
            let drift = u32::from(i == 0);
            hash = fnv_fold_word(hash, EML_LN_TABLE[i][0] + drift);
            hash = fnv_fold_word(hash, EML_LN_TABLE[i][1]);
            hash = fnv_fold_word(hash, EML_LN_TABLE[i][2]);
            i += 1;
        }
        assert_ne!(
            EML_LN_ALGORITHM_IDENTITY, hash,
            "planted table drift must move identity"
        );
    }
}
