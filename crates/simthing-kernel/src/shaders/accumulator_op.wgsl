// Pass B kernel — AccumulatorOp execution for intent, overlay, threshold,
// and C-5/C-6 reductions. Reduction sessions bind `output_vectors` as the
// values buffer and use linear SlotRange gathers for Mean, WeightedMean,
// Sum, Max, Min, and First.

struct AccumulatorOpGpu {
    source_kind: u32,
    source_slot: u32,
    source_col: u32,
    source_count: u32,
    combine_kind: u32,
    combine_a: u32,
    combine_b: u32,
    combine_c: u32,
    combine_d: u32,
    gate_kind: u32,
    gate_a: u32,
    gate_b: u32,
    scale_kind: u32,
    scale_a: u32,
    consume: u32,
    target0_slot: u32,
    target0_col: u32,
    target1_slot: u32,
    target1_col: u32,
    target2_slot: u32,
    target2_col: u32,
    target3_slot: u32,
    target3_col: u32,
    n_targets: u32,
    _pad: u32,
}

struct AccumulatorTickParams {
    n_ops: u32,
    current_band: u32,
    n_slots: u32,
    n_dims: u32,
    emission_capacity: u32,
    threshold_emission_capacity: u32,
    dt_bits: u32,
    _pad1: u32,
    generation: u32,
    execute_mode: u32,
    _pad2: u32,
    _pad3: u32,
}

struct AccumulatorSummaryParams {
    n_slots: u32,
    n_dims: u32,
    _pad0: u32,
    _pad1: u32,
}

struct SlotSummaryGpu {
    slot: u32,
    flags: u32,
    checksum_all: u32,
    _pad: u32,
    group_checksums: array<u32, 4>,
}

struct EmissionRecordGpu {
    reg_idx: u32,
    emit_count: u32,
}

struct ThresholdEmissionGpu {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

struct OverlayLifecycleStateGpu {
    satisfied_mask: atomic<u32>,
    required_mask: u32,
    dissolved: atomic<u32>,
    generation: atomic<u32>,
}

struct AccumulatorInputGpu {
    slot: u32,
    col: u32,
    unit_cost_bits: u32,
    flags: u32,
}

const SOURCE_CONSTANT: u32 = 0u;
const SOURCE_SLOT_VALUE: u32 = 1u;
const SOURCE_SLOT_RANGE: u32 = 2u;
const SOURCE_INPUT_LIST: u32 = 3u;

const COMBINE_IDENTITY: u32 = 0u;
const COMBINE_SUM: u32 = 1u;
const COMBINE_MEAN: u32 = 2u;
const COMBINE_MAX: u32 = 3u;
const COMBINE_MIN: u32 = 4u;
const COMBINE_WEIGHTED_MEAN: u32 = 5u;
const COMBINE_AFFINE_INTENT: u32 = 6u;
const COMBINE_INTEGRATE_CLAMP: u32 = 9u;
const COMBINE_MIN_ACROSS_INPUTS: u32 = 11u;
const COMBINE_EVAL_EML: u32 = 12u;
const COMBINE_FIRST: u32 = 13u;

const CLAMP_BOUNDED: u32 = 0u;
const CLAMP_FLOORED: u32 = 1u;
const CLAMP_UNBOUNDED: u32 = 2u;

const GATE_ALWAYS: u32 = 0u;
const GATE_THRESHOLD: u32 = 1u;
const GATE_ORDER_BAND: u32 = 4u;

const CONSUME_NONE: u32 = 0u;
const CONSUME_SUBTRACT_FROM_SOURCE: u32 = 1u;
const CONSUME_SUBTRACT_FROM_ALL_INPUTS: u32 = 2u;
const CONSUME_RESET_TARGET: u32 = 3u;
const CONSUME_SCALE_TARGET: u32 = 4u;
const CONSUME_EMIT_EVENT: u32 = 5u;
const CONSUME_ADD_TO_TARGET: u32 = 6u;

const SCALE_IDENTITY: u32 = 0u;
const SCALE_CONSTANT: u32 = 1u;

const EXECUTE_MODE_COMPACT_VELOCITY: u32 = 1u;

const DIR_UPWARD: u32 = 0u;
const DIR_DOWNWARD: u32 = 1u;
const DIR_EITHER: u32 = 2u;
const DIR_LEVEL_AT_OR_ABOVE: u32 = 3u;
const DIR_LEVEL_BELOW: u32 = 4u;
const THRESH_BUF_OUTPUT: u32 = 1u;
const THRESH_BUF_OWNING_GENERATION: u32 = 2u;

@group(0) @binding(0) var<storage, read> ops: array<AccumulatorOpGpu>;
@group(0) @binding(1) var<storage, read_write> values: array<atomic<i32>>;
@group(0) @binding(2) var<storage, read_write> emissions: array<EmissionRecordGpu>;
@group(0) @binding(3) var<storage, read_write> emission_count: atomic<u32>;
@group(0) @binding(4) var<uniform> tick_params: AccumulatorTickParams;
@group(0) @binding(5) var<storage, read> previous_values: array<f32>;
@group(0) @binding(6) var<storage, read_write> threshold_emissions: array<ThresholdEmissionGpu>;
@group(0) @binding(7) var<storage, read_write> threshold_emission_count: atomic<u32>;
@group(0) @binding(8) var<storage, read> eml_nodes: array<EmlNodeGpu>;
@group(0) @binding(9) var<storage, read> eml_tree_ranges: array<EmlTreeRangeGpu>;
@group(0) @binding(10) var<storage, read> input_list: array<AccumulatorInputGpu>;
@group(0) @binding(11) var<storage, read> previous_output_values: array<f32>;
@group(0) @binding(12) var<storage, read> output_values: array<f32>;
@group(0) @binding(13) var<storage, read_write> overlay_lifecycle_next: array<OverlayLifecycleStateGpu>;

struct EmlNodeGpu {
    opcode: u32,
    flags: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct EmlTreeRangeGpu {
    node_offset: u32,
    node_count: u32,
    execution_class: u32,
    flags: u32,
}

struct EmlEvalCtx {
    range_idx: u32,
    eval_slot: u32,
    param0: f32,
    param1: f32,
    param2: f32,
    param3: f32,
}

const EML_OP_LITERAL_F32: u32 = 0u;
const EML_OP_SLOT_VALUE: u32 = 1u;
const EML_OP_PARAM: u32 = 2u;
const EML_OP_ADD: u32 = 10u;
const EML_OP_SUB: u32 = 11u;
const EML_OP_MUL: u32 = 12u;
const EML_OP_NEG: u32 = 13u;
const EML_OP_DIV: u32 = 14u;
const EML_OP_MIN: u32 = 20u;
const EML_OP_MAX: u32 = 21u;
const EML_OP_CLAMP_BOUNDED: u32 = 22u;
const EML_OP_CLAMP_FLOORED: u32 = 23u;
const EML_OP_ABS: u32 = 24u;
const EML_OP_FLOOR: u32 = 25u;
const EML_OP_EXP: u32 = 26u;
const EML_OP_LN: u32 = 27u;
const EML_OP_CMP_LT: u32 = 30u;
const EML_OP_CMP_LE: u32 = 31u;
const EML_OP_CMP_GT: u32 = 32u;
const EML_OP_CMP_GE: u32 = 33u;
const EML_OP_CMP_EQ: u32 = 34u;
const EML_OP_SELECT: u32 = 40u;
const EML_OP_RETURN_TOP: u32 = 50u;

const EML_STACK_MAX: u32 = 32u;

// EML-EXP-PRIMITIVE-0: pinned algorithm-as-spec for the EXP exact primitive.
// The step order IS the bit law; the CPU twin
// (simthing_core::eml_exp::eml_exp_pinned_f32) executes the identical
// sequence, and the exhaustive admitted-domain digest is the parity referee. Constants
// are bitcast-pinned to exact binary32 bits. Any edit here is a NEW primitive
// name, never a mutation of EXP. Byte-identical to the field_sweep.wgsl copy;
// a kernel referee holds the two copies and the Rust twin's constants aligned.
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

fn eml_param(ctx: EmlEvalCtx, idx: u32) -> f32 {
    if (idx == 0u) {
        return ctx.param0;
    }
    if (idx == 1u) {
        return ctx.param1;
    }
    if (idx == 2u) {
        return ctx.param2;
    }
    return ctx.param3;
}

fn eml_eval(ctx: EmlEvalCtx) -> f32 {
    let range = eml_tree_ranges[ctx.range_idx];
    var stack: array<f32, 32>;
    var mul_a: array<f32, 32>;
    var mul_b: array<f32, 32>;
    var is_mul: array<u32, 32>;
    var sp: u32 = 0u;

    for (var i: u32 = 0u; i < range.node_count; i = i + 1u) {
        let node = eml_nodes[range.node_offset + i];
        switch node.opcode {
            case EML_OP_LITERAL_F32: {
                stack[sp] = bitcast<f32>(node.a);
                is_mul[sp] = 0u;
                sp = sp + 1u;
            }
            case EML_OP_SLOT_VALUE: {
                stack[sp] = atomic_read_f32_at(linear_idx(ctx.eval_slot, node.a));
                is_mul[sp] = 0u;
                sp = sp + 1u;
            }
            case EML_OP_PARAM: {
                stack[sp] = eml_param(ctx, node.a);
                is_mul[sp] = 0u;
                sp = sp + 1u;
            }
            case EML_OP_ADD, EML_OP_SUB: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                let rhs_is = is_mul[sp - 1u];
                let lhs_is = is_mul[sp - 2u];
                var result = 0.0;
                if (lhs_is == 1u && rhs_is == 0u) {
                    let a = mul_a[sp - 2u];
                    let b = mul_b[sp - 2u];
                    if (node.opcode == EML_OP_SUB) {
                        result = fma(a, b, -rhs);
                    } else {
                        result = fma(a, b, rhs);
                    }
                } else if (lhs_is == 0u && rhs_is == 1u) {
                    let a = mul_a[sp - 1u];
                    let b = mul_b[sp - 1u];
                    if (node.opcode == EML_OP_SUB) {
                        result = fma(-a, b, lhs);
                    } else {
                        result = fma(a, b, lhs);
                    }
                } else if (node.opcode == EML_OP_SUB) {
                    result = lhs - rhs;
                } else {
                    result = lhs + rhs;
                }
                stack[sp - 2u] = result;
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_MUL: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs * rhs;
                mul_a[sp - 2u] = lhs;
                mul_b[sp - 2u] = rhs;
                is_mul[sp - 2u] = 1u;
                sp = sp - 1u;
            }
            case EML_OP_NEG: {
                stack[sp - 1u] = -stack[sp - 1u];
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_DIV: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs / rhs;
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_MIN: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = min(lhs, rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_MAX: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = max(lhs, rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_CLAMP_BOUNDED: {
                let v = stack[sp - 1u];
                stack[sp - 1u] = clamp(v, bitcast<f32>(node.a), bitcast<f32>(node.b));
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_CLAMP_FLOORED: {
                let v = stack[sp - 1u];
                stack[sp - 1u] = max(v, bitcast<f32>(node.a));
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_ABS: {
                stack[sp - 1u] = abs(stack[sp - 1u]);
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_FLOOR: {
                stack[sp - 1u] = floor(stack[sp - 1u]);
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_EXP: {
                stack[sp - 1u] = eml_exp_pinned(stack[sp - 1u]);
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_LN: {
                stack[sp - 1u] = eml_ln_pinned(stack[sp - 1u]);
                is_mul[sp - 1u] = 0u;
            }
            case EML_OP_CMP_LT: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs < rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_CMP_LE: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs <= rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_CMP_GT: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs > rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_CMP_GE: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs >= rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_CMP_EQ: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs == rhs);
                is_mul[sp - 2u] = 0u;
                sp = sp - 1u;
            }
            case EML_OP_SELECT: {
                let f_val = stack[sp - 1u];
                let t_val = stack[sp - 2u];
                let cond = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(f_val, t_val, cond);
                is_mul[sp - 3u] = 0u;
                sp = sp - 2u;
            }
            case EML_OP_RETURN_TOP: {
                return stack[sp - 1u];
            }
            default: {
                return 0.0;
            }
        }
    }
    return stack[sp - 1u];
}

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * tick_params.n_dims + col;
}

fn atomic_read_f32_at(idx: u32) -> f32 {
    return bitcast<f32>(atomicLoad(&values[idx]));
}

fn atomic_add_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    loop {
        let old_bits = atomicLoad(cell_ptr);
        let new_bits = bitcast<i32>(bitcast<f32>(old_bits) + val);
        let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
        if result.exchanged { break; }
    }
}

fn atomic_store_f32_at(idx: u32, val: f32) {
    atomicStore(&values[idx], bitcast<i32>(val));
}

// C-4 overlay OrderBands guarantee a single writer per (band, slot, col).
// These helpers are intentionally load+store rather than CAS loops.
fn atomic_add_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old + val));
}

fn atomic_mul_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old * val));
}

fn apply_amount_clamp(kind: u32, lo: f32, hi: f32, x: f32) -> f32 {
    if (kind == CLAMP_BOUNDED) { return clamp(x, lo, hi); }
    if (kind == CLAMP_FLOORED) { return max(x, lo); }
    return x;
}

fn amount_at_floor(kind: u32, lo: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED || kind == CLAMP_FLOORED) { return x <= lo; }
    return false;
}

fn amount_at_ceiling(kind: u32, hi: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED) { return x >= hi; }
    return false;
}

fn gate_matches_for_band(op: AccumulatorOpGpu, current_band: u32) -> bool {
    // Band-wise gating only — threshold ops are handled by their own dispatch
    // path in `execute_ops`. Keeping the two gate families separate at the
    // dispatch level avoids the misleading "always-true" return for threshold
    // ops and lets the optimizer drop dead branches per dispatch.
    if (op.gate_kind == GATE_ALWAYS) {
        return true;
    }
    return op.gate_kind == GATE_ORDER_BAND && op.gate_a == current_band;
}

fn gate_matches_bandwise(op: AccumulatorOpGpu) -> bool {
    return gate_matches_for_band(op, tick_params.current_band);
}

fn threshold_crossed(prev: f32, curr: f32, threshold: f32, direction: u32) -> bool {
    // Overlay lifecycle property predicates are levels, not edges. They still
    // route through this sole Phase-5 comparator; the CPU only binds the
    // admitted direction mode and never evaluates the resident value.
    if (direction == DIR_LEVEL_AT_OR_ABOVE) {
        return curr >= threshold;
    }
    if (direction == DIR_LEVEL_BELOW) {
        return curr < threshold;
    }
    let up = (prev <= threshold) && (curr > threshold);
    let down = (prev >= threshold) && (curr < threshold);
    if (direction == DIR_UPWARD) {
        return up;
    }
    if (direction == DIR_DOWNWARD) {
        return down;
    }
    return up || down;
}

fn threshold_operands(op: AccumulatorOpGpu) -> vec2<f32> {
    if (op.source_count == THRESH_BUF_OWNING_GENERATION) {
        let curr_generation = tick_params.generation;
        let prev_generation = select(0u, curr_generation - 1u, curr_generation > 0u);
        return vec2<f32>(f32(prev_generation), f32(curr_generation));
    }
    let addr = linear_idx(op.source_slot, op.source_col);
    let use_output = op.source_count == THRESH_BUF_OUTPUT;
    return vec2<f32>(
        select(previous_values[addr], previous_output_values[addr], use_output),
        select(atomic_read_f32_at(addr), output_values[addr], use_output),
    );
}

fn project_overlay_lifecycle_crossing(op: AccumulatorOpGpu) {
    // Zero means that this ordinary Phase-5 registration has no lifecycle
    // projection. The sole threshold comparator remains `threshold_crossed`.
    if (op._pad == 0u) {
        return;
    }
    let row = (op._pad >> 5u) - 1u;
    let condition_bit = op._pad & 31u;
    let condition_mask = 1u << condition_bit;
    let prior = atomicOr(&overlay_lifecycle_next[row].satisfied_mask, condition_mask);
    let satisfied = prior | condition_mask;
    if ((satisfied & overlay_lifecycle_next[row].required_mask) == overlay_lifecycle_next[row].required_mask) {
        atomicStore(&overlay_lifecycle_next[row].dissolved, 1u);
        atomicStore(&overlay_lifecycle_next[row].generation, tick_params.generation);
    }
}

fn maybe_emit_threshold(op_idx: u32, op: AccumulatorOpGpu) {
    // Caller guarantees op.gate_kind == GATE_THRESHOLD &&
    // op.consume == CONSUME_EMIT_EVENT. Read `curr` once and reuse for the
    // crossing test and the emission payload.
    if (op._pad != 0u &&
        (op.gate_a == DIR_LEVEL_AT_OR_ABOVE || op.gate_a == DIR_LEVEL_BELOW)) {
        let row = (op._pad >> 5u) - 1u;
        let condition_mask = 1u << (op._pad & 31u);
        if ((atomicLoad(&overlay_lifecycle_next[row].satisfied_mask) & condition_mask) != 0u) {
            return;
        }
    }
    let operands = threshold_operands(op);
    let prev = operands.x;
    let curr = operands.y;
    let threshold = bitcast<f32>(op.gate_b);
    if (!threshold_crossed(prev, curr, threshold, op.gate_a)) {
        return;
    }
    project_overlay_lifecycle_crossing(op);
    let out_idx = atomicAdd(&threshold_emission_count, 1u);
    if (out_idx < tick_params.threshold_emission_capacity) {
        threshold_emissions[out_idx].reg_idx = op_idx;
        threshold_emissions[out_idx].slot = op.source_slot;
        threshold_emissions[out_idx].col = op.source_col;
        threshold_emissions[out_idx].value = curr;
    }
}

fn apply_scale(value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.scale_kind == SCALE_CONSTANT) {
        return value * bitcast<f32>(op.scale_a);
    }
    return value;
}

fn clamped_transfer(op: AccumulatorOpGpu) -> f32 {
    let available = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    let requested = bitcast<f32>(op.scale_a);
    return min(max(requested, 0.0), max(available, 0.0));
}

fn gather_min_across_inputs(op: AccumulatorOpGpu) -> f32 {
    var amount = 3.402823466e38;
    for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
        let input = input_list[op.source_slot + i];
        let available = atomic_read_f32_at(linear_idx(input.slot, input.col));
        let unit_cost = bitcast<f32>(input.unit_cost_bits);
        if (unit_cost <= 0.0) {
            return 0.0;
        }
        let possible = available / unit_cost;
        amount = min(amount, possible);
    }
    if (op.source_count == 0u) {
        return 0.0;
    }
    return max(floor(amount), 0.0);
}

fn gather_value(op: AccumulatorOpGpu) -> f32 {
    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
        }
        return sum;
    }

    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_INPUT_LIST) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let input = input_list[op.source_slot + i];
            sum = sum + atomic_read_f32_at(linear_idx(input.slot, input.col));
        }
        return sum;
    }

    // C-5 intentionally uses linear-loop gather for deterministic soft aggregate
    // migration. Do not replace with shared-memory tree reduction in C-5.
    if (op.combine_kind == COMBINE_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
        }
        if (op.source_count == 0u) {
            return 0.0;
        }
        return sum / f32(op.source_count);
    }

    if (op.combine_kind == COMBINE_WEIGHTED_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
        let weight_col = op.combine_a;
        var weighted_sum = 0.0;
        var weight_total = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let child_slot = op.source_slot + i;
            let v = atomic_read_f32_at(linear_idx(child_slot, op.source_col));
            let w = atomic_read_f32_at(linear_idx(child_slot, weight_col));
            weighted_sum = weighted_sum + v * w;
            weight_total = weight_total + w;
        }
        if (weight_total == 0.0) {
            return 0.0;
        }
        return weighted_sum / weight_total;
    }

    if (op.combine_kind == COMBINE_MAX && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        var acc = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        for (var i: u32 = 1u; i < op.source_count; i = i + 1u) {
            let v = atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
            if (v > acc) {
                acc = v;
            }
        }
        return acc;
    }

    if (op.combine_kind == COMBINE_MIN && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        var acc = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        for (var i: u32 = 1u; i < op.source_count; i = i + 1u) {
            let v = atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
            if (v < acc) {
                acc = v;
            }
        }
        return acc;
    }

    if (op.combine_kind == COMBINE_FIRST && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        return atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    }

    if (op.combine_kind == COMBINE_EVAL_EML) {
        let ctx = EmlEvalCtx(
            op.combine_a,
            op.source_slot,
            bitcast<f32>(tick_params.dt_bits),
            0.0,
            0.0,
            0.0,
        );
        return eml_eval(ctx);
    }

    if (op.combine_kind == COMBINE_MIN_ACROSS_INPUTS
        && op.source_kind == SOURCE_INPUT_LIST) {
        return gather_min_across_inputs(op);
    }

    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE
        && op.source_kind == SOURCE_SLOT_VALUE
        && op.scale_kind == SCALE_CONSTANT) {
        return clamped_transfer(op);
    }

    var raw = 0.0;
    if (op.source_kind == SOURCE_CONSTANT) {
        raw = bitcast<f32>(op.source_slot);
    } else if (op.source_kind == SOURCE_SLOT_VALUE) {
        raw = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    }

    return apply_scale(raw, op);
}

fn clamp_transfer(write_value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.source_kind == SOURCE_SLOT_VALUE) {
        let available = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        return min(max(write_value, 0.0), max(available, 0.0));
    }
    return write_value;
}

fn write_target(slot: u32, col: u32, write_value: f32, op: AccumulatorOpGpu) {
    let idx = linear_idx(slot, col);
    switch op.consume {
        case CONSUME_ADD_TO_TARGET: {
            if (op.gate_kind == GATE_ORDER_BAND) {
                atomic_add_single_writer_f32_at(idx, write_value);
            } else {
                atomic_add_f32_at(idx, write_value);
            }
        }
        case CONSUME_SCALE_TARGET: {
            atomic_mul_single_writer_f32_at(idx, write_value);
        }
        case CONSUME_RESET_TARGET: {
            atomic_store_f32_at(idx, write_value);
        }
        case CONSUME_SUBTRACT_FROM_SOURCE, CONSUME_SUBTRACT_FROM_ALL_INPUTS: {
            atomic_add_f32_at(idx, write_value);
        }
        default: {
            atomic_store_f32_at(idx, write_value);
        }
    }
}

fn apply_targets(write_value: f32, op: AccumulatorOpGpu) {
    if (op.n_targets >= 1u) {
        write_target(op.target0_slot, op.target0_col, write_value, op);
    }
    if (op.n_targets >= 2u) {
        write_target(op.target1_slot, op.target1_col, write_value, op);
    }
    if (op.n_targets >= 3u) {
        write_target(op.target2_slot, op.target2_col, write_value, op);
    }
    if (op.n_targets >= 4u) {
        write_target(op.target3_slot, op.target3_col, write_value, op);
    }
}

fn apply_consume(write_value: f32, op: AccumulatorOpGpu) {
    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.source_kind == SOURCE_SLOT_VALUE) {
        // C-8c planner rejects same-band consumed-input contention. This clamp is
        // defensive only; it is not a transactional reservation mechanism.
        let idx = linear_idx(op.source_slot, op.source_col);
        let cell_ptr = &values[idx];
        loop {
            let old_bits = atomicLoad(cell_ptr);
            let old = bitcast<f32>(old_bits);
            let debit = min(max(write_value, 0.0), max(old, 0.0));
            let new_val = old - debit;
            let new_bits = bitcast<i32>(new_val);
            let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
            if result.exchanged { break; }
        }
    }
    if (op.consume == CONSUME_SUBTRACT_FROM_ALL_INPUTS
        && op.source_kind == SOURCE_INPUT_LIST) {
        let unit_count = write_value;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let input = input_list[op.source_slot + i];
            let unit_cost = bitcast<f32>(input.unit_cost_bits);
            let subtract = unit_count * unit_cost;
            let idx = linear_idx(input.slot, input.col);
            let cell_ptr = &values[idx];
            loop {
                let old_bits = atomicLoad(cell_ptr);
                let old = bitcast<f32>(old_bits);
                let new_val = max(old - subtract, 0.0);
                let new_bits = bitcast<i32>(new_val);
                let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
                if result.exchanged { break; }
            }
        }
    }
}

fn maybe_emit_event(op_idx: u32, write_value: f32, op: AccumulatorOpGpu) {
    // Threshold-gate emissions are handled by `maybe_emit_threshold`; this
    // path is reached only when `gate_kind != GATE_THRESHOLD` (see dispatch).
    if (op.consume != CONSUME_EMIT_EVENT) {
        return;
    }
    let emit_count = u32(floor(max(write_value, 0.0)));
    if (emit_count == 0u) {
        return;
    }
    let idx = atomicAdd(&emission_count, 1u);
    if (idx < tick_params.emission_capacity) {
        // C-8d: stable registration id encoded in combine_b by the emission planner.
        emissions[idx].reg_idx = op.combine_b;
        emissions[idx].emit_count = emit_count;
    }
}

fn integrate_clamp_at_slots(op: AccumulatorOpGpu, amount_slot: u32, velocity_slot: u32) {
    let amount_idx = linear_idx(amount_slot, op.target0_col);
    let velocity_idx = linear_idx(velocity_slot, op.target1_col);

    let amount0 = atomic_read_f32_at(amount_idx);
    let raw_vel = atomic_read_f32_at(velocity_idx);

    let dt = bitcast<f32>(tick_params.dt_bits);
    let vel_max = bitcast<f32>(op.combine_a);
    let clamp_min = bitcast<f32>(op.combine_b);
    let clamp_max = bitcast<f32>(op.combine_c);
    let clamp_kind = op.combine_d;

    let effective_vel = clamp(raw_vel, -vel_max, vel_max);
    let delta = effective_vel * dt;
    let new_val = amount0 + delta;
    let clamped = apply_amount_clamp(clamp_kind, clamp_min, clamp_max, new_val);

    atomic_store_f32_at(amount_idx, clamped);

    if (amount_at_floor(clamp_kind, clamp_min, clamped)) {
        atomic_store_f32_at(velocity_idx, max(raw_vel, 0.0));
    } else if (amount_at_ceiling(clamp_kind, clamp_max, clamped)) {
        atomic_store_f32_at(velocity_idx, min(raw_vel, 0.0));
    }
}

fn dispatch_one_op_for_band(op_idx: u32, op: AccumulatorOpGpu, current_band: u32) {
    // C-2 folded intent deltas: direct affine update on one cell, no targets.
    if (op.combine_kind == COMBINE_AFFINE_INTENT) {
        let idx = linear_idx(op.source_slot, op.source_col);
        let cell_ptr = &values[idx];
        let mul = bitcast<f32>(op.combine_a);
        let add = bitcast<f32>(op.combine_b);
        loop {
            let old_bits = atomicLoad(cell_ptr);
            let old = bitcast<f32>(old_bits);
            let new_bits = bitcast<i32>(old * mul + add);
            let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
            if result.exchanged { break; }
        }
        return;
    }

    // C-7 GovernedPair velocity integration — multi-target write with legacy
    // semantics (amount integrate + optional velocity pinning at floor/ceiling).
    if (op.combine_kind == COMBINE_INTEGRATE_CLAMP) {
        if (gate_matches_for_band(op, current_band)) {
            integrate_clamp_at_slots(op, op.target0_slot, op.target1_slot);
        }
        return;
    }

    // Threshold ops dispatch on consume mode:
    //   CONSUME_EMIT_EVENT: detect crossing, write compact threshold record.
    //   CONSUME_NONE:       detect crossing, write to targets (no record).
    //                       Used by E-1 debt-band preconditions.
    // Both paths return early — threshold ops are disjoint from band-gated ops.
    if (op.gate_kind == GATE_THRESHOLD) {
        if (op.consume == CONSUME_EMIT_EVENT) {
            maybe_emit_threshold(op_idx, op);
        } else if (op.consume == CONSUME_NONE && op.source_kind == SOURCE_SLOT_VALUE) {
            let operands = threshold_operands(op);
            let prev = operands.x;
            let curr = operands.y;
            let threshold = bitcast<f32>(op.gate_b);
            if (threshold_crossed(prev, curr, threshold, op.gate_a)) {
                var write_value = gather_value(op);
                apply_targets(write_value, op);
            }
        }
        return;
    }

    if (!gate_matches_for_band(op, current_band)) {
        return;
    }

    var write_value = gather_value(op);
    write_value = clamp_transfer(write_value, op);
    var target_value = write_value;
    if (op.combine_kind == COMBINE_MIN_ACROSS_INPUTS) {
        target_value = apply_scale(write_value, op);
    }
    apply_targets(target_value, op);
    apply_consume(write_value, op);
    maybe_emit_event(op_idx, write_value, op);
}

@compute @workgroup_size(64)
fn execute_ops(@builtin(global_invocation_id) gid: vec3<u32>) {
    var op_idx = gid.x;
    if (tick_params.execute_mode == EXECUTE_MODE_COMPACT_VELOCITY) {
        let op_count = tick_params.n_ops;
        if (op_count == 0u) {
            return;
        }
        op_idx = op_idx + tick_params.current_band;
        let total_invocations = op_count * tick_params.n_slots;
        if (op_idx >= total_invocations) {
            return;
        }

        let packed_op_idx = op_idx % op_count;
        let slot_offset = op_idx / op_count;
        let op = ops[packed_op_idx];
        if (slot_offset >= op.source_count) {
            return;
        }
        let slot = op.source_slot + slot_offset;
        integrate_clamp_at_slots(op, slot, slot);
        return;
    }

    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];
    dispatch_one_op_for_band(op_idx, op, tick_params.current_band);
}

// AO-WGSL-0: semantic-free generic OrderBand entry (single band per dispatch).
// Multi-band sequences are driven from Rust with preserved global band order.
// Band count for batching lives in `tick_params._pad1` for harness reporting only.
@compute @workgroup_size(64)
fn execute_orderband_bands(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];
    dispatch_one_op_for_band(op_idx, op, tick_params.current_band);
}

@group(0) @binding(0) var<storage, read_write> summary_values: array<atomic<i32>>;
@group(0) @binding(1) var<storage, read_write> summaries: array<SlotSummaryGpu>;
@group(0) @binding(2) var<uniform> summary_params: AccumulatorSummaryParams;

@compute @workgroup_size(64)
fn write_summaries(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= summary_params.n_slots) {
        return;
    }

    var checksum_all = 0u;
    var group_checksums = array<u32, 4>(0u, 0u, 0u, 0u);
    let group_size = (summary_params.n_dims + 3u) / 4u;

    for (var col: u32 = 0u; col < summary_params.n_dims; col = col + 1u) {
        let idx = slot * summary_params.n_dims + col;
        let bits = bitcast<u32>(atomicLoad(&summary_values[idx]));
        checksum_all = checksum_all ^ bits;
        let g = col / group_size;
        if (g < 4u) {
            group_checksums[g] = group_checksums[g] ^ bits;
        }
    }

    summaries[slot].slot = slot;
    summaries[slot].flags = 0u;
    summaries[slot].checksum_all = checksum_all;
    summaries[slot]._pad = 0u;
    summaries[slot].group_checksums = group_checksums;
}
