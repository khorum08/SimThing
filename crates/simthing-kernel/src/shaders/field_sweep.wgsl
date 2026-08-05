// FIELD-SWEEP-N4-PARITY-0: one generic EML map/fixed-linear-fold/post sweep.

const EML_STACK_MAX: u32 = 32u;

struct EmlNode {
    opcode: u32,
    flags: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct FieldRange {
    offset: u32,
    count: u32,
}

// Byte-identical to AccumulatorInputGpu: the existing input-list gather row.
struct AccumulatorInput {
    slot: u32,
    col: u32,
    unit_cost_bits: u32,
    flags: u32,
}

struct FieldSweepParams {
    n_slots: u32,
    n_dims: u32,
    output_col: u32,
    map_offset: u32,
    map_count: u32,
    fold_offset: u32,
    fold_count: u32,
    post_offset: u32,
    post_count: u32,
    identity_bits: u32,
    dt_bits: u32,
    schedule_offset: u32,
    schedule_count: u32,
    output_mode: u32,
    pad1: u32,
    fused_identity_bits: u32,
    fused_dt_bits: u32,
    pad2: u32,
    pad3: u32,
}

struct FieldEmlContext {
    target_slot: u32,
    neighbor_slot: u32,
    has_neighbor: u32,
    accumulator: f32,
    edge_scalar: f32,
    dt: f32,
    mapped: f32,
    folded: f32,
    target_transient: f32,
    neighbor_transient: f32,
}

@group(0) @binding(0) var<storage, read> values_in: array<f32>;
@group(0) @binding(1) var<storage, read_write> values_out: array<f32>;
@group(0) @binding(2) var<storage, read> ranges: array<FieldRange>;
@group(0) @binding(3) var<storage, read> inputs: array<AccumulatorInput>;
@group(0) @binding(4) var<storage, read> nodes: array<EmlNode>;
@group(0) @binding(5) var<storage, read> schedule: array<u32>;
@group(0) @binding(6) var<uniform> params: FieldSweepParams;
@group(0) @binding(7) var<storage, read_write> transient_values: array<f32>;

// EML-EXP-PRIMITIVE-0: pinned algorithm-as-spec for the EXP exact primitive.
// The step order IS the bit law; the CPU twin
// (simthing_core::eml_exp::eml_exp_pinned_f32) executes the identical
// sequence, and the exhaustive admitted-domain digest is the parity referee. Constants
// are bitcast-pinned to exact binary32 bits. Any edit here is a NEW primitive
// name, never a mutation of EXP. Placed OUTSIDE the JIT evaluator markers so
// the interpreted arm and every JIT-generated straight-line block call this
// one definition.
//
// The specified operations are one product, one round-ties-even, eight
// EXPLICIT fused multiply-adds (`fma` builtin — single-rounding IEEE, the
// CPU twin uses `f32::mul_add`), one add, and exact integer/bit scale steps.
// The fused/intrinsic shape is deliberate: the certified toolchain's shader
// compiler eliminates magic-shifter rounding and freely contracts separate
// mul+add chains (even across bitcast fences — measured), so the sequence
// pins the semantics the hardware executes instead of fencing against them.
fn eml_exp_pinned(x: f32) -> f32 {
    let a = x * bitcast<f32>(0x3FB8AA3Bu);              // x * log2(e)
    let kf = round(a);                                  // RNE intrinsic
    let hi = fma(kf, bitcast<f32>(0xBF318000u), x);     // x - kf*ln2_hi
    let r = fma(kf, bitcast<f32>(0x395E8083u), hi);     // hi + kf*2.1219444e-4
    let z = r * r;
    var p = bitcast<f32>(0x39506967u);                  // P5
    p = fma(p, r, bitcast<f32>(0x3AB743CEu));           // P4
    p = fma(p, r, bitcast<f32>(0x3C088908u));           // P3
    p = fma(p, r, bitcast<f32>(0x3D2AA9C1u));           // P2
    p = fma(p, r, bitcast<f32>(0x3E2AAAAAu));           // P1
    p = fma(p, r, bitcast<f32>(0x3F000000u));           // P0
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

// EML-LN-PRIMITIVE-0: pinned algorithm-as-spec for the LN exact primitive
// (candidate LND4). The step order IS the bit law; the CPU twin
// (simthing_core::eml_ln::eml_ln_pinned_f32) executes the identical sequence,
// and the exhaustive admitted-domain digest is the parity referee. The
// reconstruction's deciding sum k*LN2_HI + ln_c_hi is EXACT BY AUTHORED GRID
// (2^-16-aligned constants) — immune to the measured two-sum-collapse
// eliminator on the certified tuple; every inexact product rides an explicit
// fma. Any edit is a NEW primitive name. Byte-identical helper + table across
// both shader homes; a kernel referee holds them and the Rust twin aligned.
var<private> EML_LN_TBL: array<vec3<u32>, 128> = array<vec3<u32>, 128>(
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

fn eml_ln_pinned(x: f32) -> f32 {
    let x_bits = bitcast<u32>(x);
    let t = x_bits - 0x3F330000u;
    let k = i32(t) >> 23u;
    let m_bits = x_bits - (u32(k) << 23u);
    let j = (t >> 16u) & 0x7Fu;
    let m = bitcast<f32>(m_bits);
    let e = EML_LN_TBL[j];
    let inv = bitcast<f32>(e.x);
    let lnc_hi = bitcast<f32>(e.y);
    let lnc_mid = bitcast<f32>(e.z);
    let p_hi = m * inv;
    let p_err = fma(m, inv, -p_hi);
    let s = p_hi - 1.0;
    let s_lo = p_err;
    var poly = fma(s, 0.2, -0.25);
    poly = fma(s, poly, bitcast<f32>(0x3EAAAAABu));
    let z = s * s;
    let sp = s * poly;
    let r1 = fma(z, sp, -0.5 * z);
    let slo_term = fma(-s_lo, s, s_lo);
    let kf = f32(k);
    let t_hi = fma(kf, bitcast<f32>(0x3F317200u), lnc_hi);
    let mid = fma(kf, bitcast<f32>(0x35BFBE8Eu), lnc_mid);
    let low = mid + (slo_term + r1);
    let g1 = low + s;
    return t_hi + g1;
}

// EML-JIT-EVALUATOR-BEGIN
const OP_LITERAL_F32: u32 = 0u;
const OP_PARAM: u32 = 2u;
const OP_TARGET_VALUE: u32 = 3u;
const OP_NEIGHBOR_VALUE: u32 = 4u;
const OP_ADD: u32 = 10u;
const OP_SUB: u32 = 11u;
const OP_MUL: u32 = 12u;
const OP_NEG: u32 = 13u;
const OP_DIV: u32 = 14u;
const OP_MIN: u32 = 20u;
const OP_MAX: u32 = 21u;
const OP_CLAMP_BOUNDED: u32 = 22u;
const OP_CLAMP_FLOORED: u32 = 23u;
const OP_ABS: u32 = 24u;
const OP_FLOOR: u32 = 25u;
const OP_EXP: u32 = 26u;
const OP_LN: u32 = 27u;
const OP_CMP_LT: u32 = 30u;
const OP_CMP_LE: u32 = 31u;
const OP_CMP_GT: u32 = 32u;
const OP_CMP_GE: u32 = 33u;
const OP_CMP_EQ: u32 = 34u;
const OP_SELECT: u32 = 40u;
const OP_RETURN_TOP: u32 = 50u;

fn field_param(index: u32, context: FieldEmlContext) -> f32 {
    if index == 0u { return f32(context.target_slot); }
    if index == 1u { return f32(context.neighbor_slot); }
    if index == 2u { return context.accumulator; }
    if index == 3u { return context.edge_scalar; }
    if index == 4u { return context.dt; }
    if index == 5u { return context.mapped; }
    if index == 6u { return context.folded; }
    if index == 7u { return context.target_transient; }
    return context.neighbor_transient;
}

fn eval_program(offset: u32, count: u32, context: FieldEmlContext) -> f32 {
    var stack: array<f32, EML_STACK_MAX>;
    var sp = 0u;
    for (var local = 0u; local < count; local = local + 1u) {
        let node = nodes[offset + local];
        switch node.opcode {
            case OP_LITERAL_F32: {
                stack[sp] = bitcast<f32>(node.a);
                sp = sp + 1u;
            }
            case OP_TARGET_VALUE: {
                stack[sp] = values_in[context.target_slot * params.n_dims + node.a];
                sp = sp + 1u;
            }
            case OP_NEIGHBOR_VALUE: {
                stack[sp] = values_in[context.neighbor_slot * params.n_dims + node.a];
                sp = sp + 1u;
            }
            case OP_PARAM: {
                stack[sp] = field_param(node.a, context);
                sp = sp + 1u;
            }
            case OP_NEG: { stack[sp - 1u] = -stack[sp - 1u]; }
            case OP_CLAMP_BOUNDED: {
                stack[sp - 1u] = clamp(
                    stack[sp - 1u],
                    bitcast<f32>(node.a),
                    bitcast<f32>(node.b),
                );
            }
            case OP_CLAMP_FLOORED: {
                stack[sp - 1u] = max(stack[sp - 1u], bitcast<f32>(node.a));
            }
            case OP_ABS: { stack[sp - 1u] = abs(stack[sp - 1u]); }
            case OP_FLOOR: { stack[sp - 1u] = floor(stack[sp - 1u]); }
            case OP_EXP: { stack[sp - 1u] = eml_exp_pinned(stack[sp - 1u]); }
            case OP_LN: { stack[sp - 1u] = eml_ln_pinned(stack[sp - 1u]); }
            case OP_SELECT: {
                let false_value = stack[sp - 1u];
                let true_value = stack[sp - 2u];
                let condition = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(false_value, true_value, condition);
                sp = sp - 2u;
            }
            case OP_RETURN_TOP: { return stack[sp - 1u]; }
            default: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                var result = 0.0;
                switch node.opcode {
                    case OP_ADD: { result = lhs + rhs; }
                    case OP_SUB: { result = lhs - rhs; }
                    case OP_MUL: { result = lhs * rhs; }
                    case OP_DIV: { result = lhs / rhs; }
                    case OP_MIN: { result = min(lhs, rhs); }
                    case OP_MAX: { result = max(lhs, rhs); }
                    case OP_CMP_LT: { result = select(0.0, 1.0, lhs < rhs); }
                    case OP_CMP_LE: { result = select(0.0, 1.0, lhs <= rhs); }
                    case OP_CMP_GT: { result = select(0.0, 1.0, lhs > rhs); }
                    case OP_CMP_GE: { result = select(0.0, 1.0, lhs >= rhs); }
                    case OP_CMP_EQ: { result = select(0.0, 1.0, lhs == rhs); }
                    default: {}
                }
                stack[sp - 2u] = result;
                sp = sp - 1u;
            }
        }
    }
    return stack[sp - 1u];
}

// SEAM LAW: evaluate the first `count` nodes and return the top two stack
// values (the map's final-MUL operands) for the fused canonical-Sum fold.
fn eval_program_pair(offset: u32, count: u32, context: FieldEmlContext) -> vec2<f32> {
    var stack: array<f32, EML_STACK_MAX>;
    var sp = 0u;
    for (var local = 0u; local < count; local = local + 1u) {
        let node = nodes[offset + local];
        switch node.opcode {
            case OP_LITERAL_F32: { stack[sp] = bitcast<f32>(node.a); sp = sp + 1u; }
            case OP_TARGET_VALUE: { stack[sp] = values_in[context.target_slot * params.n_dims + node.a]; sp = sp + 1u; }
            case OP_NEIGHBOR_VALUE: { stack[sp] = values_in[context.neighbor_slot * params.n_dims + node.a]; sp = sp + 1u; }
            case OP_PARAM: { stack[sp] = field_param(node.a, context); sp = sp + 1u; }
            case OP_NEG: { stack[sp - 1u] = -stack[sp - 1u]; }
            case OP_CLAMP_BOUNDED: { stack[sp - 1u] = clamp(stack[sp - 1u], bitcast<f32>(node.a), bitcast<f32>(node.b)); }
            case OP_CLAMP_FLOORED: { stack[sp - 1u] = max(stack[sp - 1u], bitcast<f32>(node.a)); }
            case OP_ABS: { stack[sp - 1u] = abs(stack[sp - 1u]); }
            case OP_FLOOR: { stack[sp - 1u] = floor(stack[sp - 1u]); }
            case OP_EXP: { stack[sp - 1u] = eml_exp_pinned(stack[sp - 1u]); }
            case OP_LN: { stack[sp - 1u] = eml_ln_pinned(stack[sp - 1u]); }
            case OP_SELECT: {
                let false_value = stack[sp - 1u];
                let true_value = stack[sp - 2u];
                let condition = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(false_value, true_value, condition);
                sp = sp - 2u;
            }
            default: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                var result = 0.0;
                switch node.opcode {
                    case OP_ADD: { result = lhs + rhs; }
                    case OP_SUB: { result = lhs - rhs; }
                    case OP_MUL: { result = lhs * rhs; }
                    case OP_DIV: { result = lhs / rhs; }
                    case OP_MIN: { result = min(lhs, rhs); }
                    case OP_MAX: { result = max(lhs, rhs); }
                    case OP_CMP_LT: { result = select(0.0, 1.0, lhs < rhs); }
                    case OP_CMP_LE: { result = select(0.0, 1.0, lhs <= rhs); }
                    case OP_CMP_GT: { result = select(0.0, 1.0, lhs > rhs); }
                    case OP_CMP_GE: { result = select(0.0, 1.0, lhs >= rhs); }
                    case OP_CMP_EQ: { result = select(0.0, 1.0, lhs == rhs); }
                    default: {}
                }
                stack[sp - 2u] = result;
                sp = sp - 1u;
            }
        }
    }
    return vec2<f32>(stack[sp - 2u], stack[sp - 1u]);
}
// EML-JIT-EVALUATOR-END

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if gid.x >= params.schedule_count {
        return;
    }
    let target_slot = schedule[params.schedule_offset + gid.x];

    let target_base = target_slot * params.n_dims;
    for (var col = 0u; col < params.n_dims; col = col + 1u) {
        values_out[target_base + col] = values_in[target_base + col];
    }

    let range = ranges[target_slot];
    var accumulator = bitcast<f32>(params.identity_bits);
    for (var edge_index = 0u; edge_index < range.count; edge_index = edge_index + 1u) {
        let input = inputs[range.offset + edge_index];
        var context = FieldEmlContext(
            target_slot,
            input.slot,
            1u,
            accumulator,
            bitcast<f32>(input.unit_cost_bits),
            bitcast<f32>(params.dt_bits),
            0.0,
            0.0,
            transient_values[target_slot],
            transient_values[input.slot],
        );
        if (params.pad1 == 1u) {
            // SEAM LAW: fused canonical-Sum fold (uniform flag, no divergence).
            let ab = eval_program_pair(params.map_offset, params.map_count - 2u, context);
            accumulator = fma(ab.x, ab.y, accumulator);
        } else {
            let mapped = eval_program(params.map_offset, params.map_count, context);
            context.mapped = mapped;
            accumulator = eval_program(params.fold_offset, params.fold_count, context);
        }
    }
    let post_context = FieldEmlContext(
        target_slot,
        target_slot,
        0u,
        accumulator,
        0.0,
        bitcast<f32>(params.dt_bits),
        0.0,
        accumulator,
        transient_values[target_slot],
        0.0,
    );
    let written = eval_program(params.post_offset, params.post_count, post_context);
    if params.output_mode == 0u {
        values_out[target_base + params.output_col] = written;
    } else {
        transient_values[target_slot] = written;
    }
}
