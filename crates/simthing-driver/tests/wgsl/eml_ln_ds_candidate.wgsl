// EML-LN-PRIMITIVE-0 — standalone frozen candidate LND4 (double-single,
// table-driven, grid-exact reconstruction). DA 5186693435 algorithm authority
// with two measured refinements: the reduction residual is carried EXACTLY as
// a double-single pair (fma residual + Sterbenz), and the reconstruction's
// deciding sum k*LN2_HI + ln_c_hi is EXACT BY AUTHORED GRID (2^-16-aligned
// constants) because the certified tuple's shader compiler collapses two-sum
// error idioms (measured; the 5.11 eliminator class) — exact arithmetic is
// immune. No vendor transcendental, no f64, no DIV, no loop, no
// data-dependent branch. Authoritative entry:
//   ln_ds_bits(x_bits: u32) -> u32   (domain: positive normals 0x00800000..=0x7F7FFFFF)
// CPU twin: simthing_core::eml_ln::eml_ln_pinned_bits.
// naga 22.1 forbids dynamic indexing of module-const arrays; the table lives
// as a module-private var initialized from this const expression — still
// authored, frozen artifact text, no storage binding.
var<private> LN_TBL: array<vec3<u32>, 128> = array<vec3<u32>, 128>(
    vec3<u32>(0x3FB68000u, 0xBEB59E00u, 0x359BB94Du),
    vec3<u32>(0x3FB58000u, 0xBEB2CE00u, 0x3628CFC2u),
    vec3<u32>(0x3FB48000u, 0xBEAFFA00u, 0x3678F587u),
    vec3<u32>(0x3FB38000u, 0xBEAD2200u, 0x36AA785Au),
    vec3<u32>(0x3FB28000u, 0xBEAA4600u, 0x36E9E291u),
    vec3<u32>(0x3FB18000u, 0xBEA76400u, 0xB6B99300u),
    vec3<u32>(0x3FB08000u, 0xBEA48000u, 0xB5D00801u),
    vec3<u32>(0x3FAF8000u, 0xBEA19800u, 0x3686AFD9u),
    vec3<u32>(0x3FAF0000u, 0xBEA02200u, 0x36779796u),
    vec3<u32>(0x3FAE0000u, 0xBE9D3200u, 0xB6455694u),
    vec3<u32>(0x3FAD0000u, 0xBE9A3E00u, 0xB6ECD4C4u),
    vec3<u32>(0x3FAC0000u, 0xBE974800u, 0x36EA28F7u),
    vec3<u32>(0x3FAB0000u, 0xBE944A00u, 0xB6D09EF4u),
    vec3<u32>(0x3FAA0000u, 0xBE914A00u, 0xB4FDE7BDu),
    vec3<u32>(0x3FA98000u, 0xBE8FC800u, 0x33C36EBAu),
    vec3<u32>(0x3FA88000u, 0xBE8CC000u, 0xB652DD42u),
    vec3<u32>(0x3FA78000u, 0xBE89B400u, 0xB5E05275u),
    vec3<u32>(0x3FA70000u, 0xBE882C00u, 0xB63F9AE5u),
    vec3<u32>(0x3FA60000u, 0xBE851A00u, 0x36D8EC63u),
    vec3<u32>(0x3FA50000u, 0xBE820200u, 0x36D35A59u),
    vec3<u32>(0x3FA48000u, 0xBE807400u, 0x369DD295u),
    vec3<u32>(0x3FA38000u, 0xBE7AA800u, 0xB5A564B8u),
    vec3<u32>(0x3FA28000u, 0xBE746000u, 0xB494AA97u),
    vec3<u32>(0x3FA20000u, 0xBE713800u, 0xB56DC55Eu),
    vec3<u32>(0x3FA10000u, 0xBE6AE000u, 0xB685AD3Fu),
    vec3<u32>(0x3FA00000u, 0xBE648000u, 0x35838656u),
    vec3<u32>(0x3F9F8000u, 0xBE614C00u, 0x363D539Fu),
    vec3<u32>(0x3F9E8000u, 0xBE5ADC00u, 0x36B985C1u),
    vec3<u32>(0x3F9E0000u, 0xBE57A000u, 0x36DAC5FDu),
    vec3<u32>(0x3F9D0000u, 0xBE511C00u, 0xB6F07F8Bu),
    vec3<u32>(0x3F9C8000u, 0xBE4DD800u, 0xB6D8B9F8u),
    vec3<u32>(0x3F9B8000u, 0xBE474800u, 0xB6A37A22u),
    vec3<u32>(0x3F9B0000u, 0xBE43FC00u, 0xB6819483u),
    vec3<u32>(0x3F9A0000u, 0xBE3D5C00u, 0xB590210Eu),
    vec3<u32>(0x3F998000u, 0xBE3A0800u, 0x356157F9u),
    vec3<u32>(0x3F990000u, 0xBE36B000u, 0xB5FE719Du),
    vec3<u32>(0x3F980000u, 0xBE2FF800u, 0xB6C1C29Eu),
    vec3<u32>(0x3F978000u, 0xBE2C9800u, 0xB6E36661u),
    vec3<u32>(0x3F968000u, 0xBE25D000u, 0xB6DBCE69u),
    vec3<u32>(0x3F960000u, 0xBE226800u, 0xB6ADB32Eu),
    vec3<u32>(0x3F958000u, 0xBE1F0000u, 0x36F53C4Du),
    vec3<u32>(0x3F948000u, 0xBE182000u, 0x36A39C6Eu),
    vec3<u32>(0x3F940000u, 0xBE14AC00u, 0x36B41F80u),
    vec3<u32>(0x3F938000u, 0xBE113400u, 0x360737ADu),
    vec3<u32>(0x3F928000u, 0xBE0A3C00u, 0xB5308CE8u),
    vec3<u32>(0x3F920000u, 0xBE06BC00u, 0x344197B9u),
    vec3<u32>(0x3F918000u, 0xBE033800u, 0xB6289653u),
    vec3<u32>(0x3F908000u, 0xBDF85000u, 0xB6430046u),
    vec3<u32>(0x3F900000u, 0xBDF13800u, 0xB4EDC55Eu),
    vec3<u32>(0x3F8F8000u, 0xBDEA1800u, 0xB59EB366u),
    vec3<u32>(0x3F8F0000u, 0xBDE2F000u, 0xB6A91EB8u),
    vec3<u32>(0x3F8E0000u, 0xBDD49000u, 0xB6DA7496u),
    vec3<u32>(0x3F8D8000u, 0xBDCD5800u, 0xB6848C40u),
    vec3<u32>(0x3F8D0000u, 0xBDC61800u, 0xB68BAC63u),
    vec3<u32>(0x3F8C8000u, 0xBDBED000u, 0xB6ECDAF6u),
    vec3<u32>(0x3F8B8000u, 0xBDB03000u, 0xB6B1526Fu),
    vec3<u32>(0x3F8B0000u, 0xBDA8D800u, 0xB4E7E0C3u),
    vec3<u32>(0x3F8A8000u, 0xBDA17800u, 0x360D0567u),
    vec3<u32>(0x3F8A0000u, 0xBD9A1000u, 0x3621A791u),
    vec3<u32>(0x3F898000u, 0xBD92A000u, 0x351D0F5Fu),
    vec3<u32>(0x3F890000u, 0xBD8B2800u, 0xB65BBA8Eu),
    vec3<u32>(0x3F880000u, 0xBD785000u, 0xB5C30046u),
    vec3<u32>(0x3F878000u, 0xBD694000u, 0x36947525u),
    vec3<u32>(0x3F870000u, 0xBD5A1000u, 0xB6DD7119u),
    vec3<u32>(0x3F868000u, 0xBD4AE000u, 0xB6830EC9u),
    vec3<u32>(0x3F860000u, 0xBD3BA000u, 0xB631EC66u),
    vec3<u32>(0x3F858000u, 0xBD2C5000u, 0xB6375F92u),
    vec3<u32>(0x3F850000u, 0xBD1CF000u, 0xB687B9FFu),
    vec3<u32>(0x3F848000u, 0xBD0D8000u, 0xB6D98924u),
    vec3<u32>(0x3F840000u, 0xBCFC2000u, 0x36B278C4u),
    vec3<u32>(0x3F838000u, 0xBCDD0000u, 0x357F6142u),
    vec3<u32>(0x3F830000u, 0xBCBDC000u, 0xB68D83EBu),
    vec3<u32>(0x3F828000u, 0xBC9E8000u, 0x36ADDE5Du),
    vec3<u32>(0x3F820000u, 0xBC7E0000u, 0xB5A8B0FCu),
    vec3<u32>(0x3F818000u, 0xBC3F0000u, 0x36EE2820u),
    vec3<u32>(0x3F810000u, 0xBBFF0000u, 0xB429AC42u),
    vec3<u32>(0x3F800000u, 0x00000000u, 0x00000000u),
    vec3<u32>(0x3F800000u, 0x00000000u, 0x00000000u),
    vec3<u32>(0x3F7D0000u, 0x3C414000u, 0xB6EDD71Eu),
    vec3<u32>(0x3F7B0000u, 0x3CA1A000u, 0xB6AB6D34u),
    vec3<u32>(0x3F790000u, 0x3CE32000u, 0xB5344FADu),
    vec3<u32>(0x3F778000u, 0x3D0A5000u, 0xB560DFFDu),
    vec3<u32>(0x3F758000u, 0x3D2B9000u, 0xB6A3B3FCu),
    vec3<u32>(0x3F738000u, 0x3D4D1000u, 0xB6709518u),
    vec3<u32>(0x3F720000u, 0x3D666000u, 0xB68C3222u),
    vec3<u32>(0x3F700000u, 0x3D843000u, 0xB6CE94C4u),
    vec3<u32>(0x3F6E8000u, 0x3D910000u, 0x36F6B8F1u),
    vec3<u32>(0x3F6C8000u, 0x3DA24000u, 0x36BC07B8u),
    vec3<u32>(0x3F6B0000u, 0x3DAF4800u, 0x36B49B2Fu),
    vec3<u32>(0x3F690000u, 0x3DC0C800u, 0x36FC5E82u),
    vec3<u32>(0x3F678000u, 0x3DCE0800u, 0xB6734ACBu),
    vec3<u32>(0x3F660000u, 0x3DDB5800u, 0xB65DC94Bu),
    vec3<u32>(0x3F648000u, 0x3DE8C000u, 0xB6D0EFBDu),
    vec3<u32>(0x3F630000u, 0x3DF63800u, 0x36660C28u),
    vec3<u32>(0x3F610000u, 0x3E042C00u, 0x3645ACF2u),
    vec3<u32>(0x3F5F8000u, 0x3E0B0800u, 0xB6DFF6D3u),
    vec3<u32>(0x3F5E0000u, 0x3E11EC00u, 0xB5ED5B64u),
    vec3<u32>(0x3F5C8000u, 0x3E18DC00u, 0x364A69D2u),
    vec3<u32>(0x3F5B0000u, 0x3E1FDC00u, 0xB6E9699Bu),
    vec3<u32>(0x3F598000u, 0x3E26E400u, 0xB50ED088u),
    vec3<u32>(0x3F588000u, 0x3E2B9C00u, 0xB4CDBF9Du),
    vec3<u32>(0x3F570000u, 0x3E32BC00u, 0xB6C505D0u),
    vec3<u32>(0x3F558000u, 0x3E39E400u, 0x36E41D3Fu),
    vec3<u32>(0x3F540000u, 0x3E412000u, 0xB6FA6AB9u),
    vec3<u32>(0x3F528000u, 0x3E486400u, 0xB612301Au),
    vec3<u32>(0x3F518000u, 0x3E4D4400u, 0xB5872145u),
    vec3<u32>(0x3F500000u, 0x3E54A000u, 0xB6161BA9u),
    vec3<u32>(0x3F4E8000u, 0x3E5C0800u, 0x363985C1u),
    vec3<u32>(0x3F4D8000u, 0x3E610000u, 0x36A2AC60u),
    vec3<u32>(0x3F4C0000u, 0x3E688000u, 0x36DFC995u),
    vec3<u32>(0x3F4B0000u, 0x3E6D8800u, 0x36F6C352u),
    vec3<u32>(0x3F498000u, 0x3E752400u, 0xB6ED83EDu),
    vec3<u32>(0x3F488000u, 0x3E7A3C00u, 0xB6D3B2C8u),
    vec3<u32>(0x3F470000u, 0x3E80F600u, 0xB68D4ECAu),
    vec3<u32>(0x3F460000u, 0x3E838A00u, 0xB5F3F655u),
    vec3<u32>(0x3F450000u, 0x3E862200u, 0xB694C4F5u),
    vec3<u32>(0x3F438000u, 0x3E8A0C00u, 0xB6C0864Cu),
    vec3<u32>(0x3F428000u, 0x3E8CAC00u, 0xB6962322u),
    vec3<u32>(0x3F418000u, 0x3E8F5000u, 0xB6F4C3BBu),
    vec3<u32>(0x3F400000u, 0x3E934C00u, 0xB6EF7659u),
    vec3<u32>(0x3F3F0000u, 0x3E95F800u, 0xB6783237u),
    vec3<u32>(0x3F3E0000u, 0x3E98A800u, 0xB661E2CAu),
    vec3<u32>(0x3F3D0000u, 0x3E9B5C00u, 0xB6C44A0Fu),
    vec3<u32>(0x3F3C0000u, 0x3E9E1200u, 0x3693B99Au),
    vec3<u32>(0x3F3A8000u, 0x3EA22C00u, 0x368F029Du),
    vec3<u32>(0x3F398000u, 0x3EA4EE00u, 0xB6C0621Au),
    vec3<u32>(0x3F388000u, 0x3EA7B200u, 0xB6014456u),
    vec3<u32>(0x3F378000u, 0x3EAA7A00u, 0x3546DEF8u),
);

fn ln_ds_bits(x_bits: u32) -> u32 {
    let t = x_bits - 0x3F330000u;
    let k = i32(t) >> 23u;
    let m_bits = x_bits - (u32(k) << 23u);
    let j = (t >> 16u) & 0x7Fu;
    let m = bitcast<f32>(m_bits);
    let e = LN_TBL[j];
    let inv = bitcast<f32>(e.x);
    let lnc_hi = bitcast<f32>(e.y);
    let lnc_mid = bitcast<f32>(e.z);
    // Exact reduction residual: s carried as double-single.
    let p_hi = m * inv;
    let p_err = fma(m, inv, -p_hi);
    let s = p_hi - 1.0;
    let s_lo = p_err;
    // Degree-5 ln(1+s); no collapsible cancellation idiom anywhere.
    var poly = fma(s, 0.2, -0.25);
    poly = fma(s, poly, bitcast<f32>(0x3EAAAAABu));
    let z = s * s;
    let sp = s * poly;
    let r1 = fma(z, sp, -0.5 * z);
    let slo_term = fma(-s_lo, s, s_lo);
    // Grid-exact reconstruction + one final rounding add.
    let kf = f32(k);
    let t_hi = fma(kf, bitcast<f32>(0x3F317200u), lnc_hi);
    let mid = fma(kf, bitcast<f32>(0x35BFBE8Eu), lnc_mid);
    let low = mid + (slo_term + r1);
    let g1 = low + s;
    return bitcast<u32>(t_hi + g1);
}

@group(0) @binding(0) var<storage, read> input_bits: array<u32>;
@group(0) @binding(1) var<storage, read_write> output_bits: array<u32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= arrayLength(&input_bits)) { return; }
    output_bits[gid.x] = ln_ds_bits(input_bits[gid.x]);
}
