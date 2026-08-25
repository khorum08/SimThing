//! AUTHORED-LAW-GADGET-0 — authored power law through the ordinary EvalEML consumer.

use simthing_core::{
    eml_nodes, AccumulatorOp, ColumnIndex, CombineFn, ConsumeMode, EmlConsumerMask,
    EmlExecutionClass, EmlExpressionRegistry, EmlFormulaMeta, EmlNodeGpu, EmlTreeId, GateSpec,
    ScaleSpec, SlotIndex, SourceSpec,
};
use simthing_gpu::{
    scoped_debug_readback_allowed, AccumulatorOpSession, EmlGpuProgramTable, GpuContext,
    PackedAccumulatorUpload,
};
use simthing_spec::{
    compile_eml_gadget_stack, deserialize_eml_gadget_stack_ron, oracle_power_law,
    EmlGadgetCompileOptions,
};

const TREE_ID: EmlTreeId = EmlTreeId(8_711);

fn gpu_context() -> Option<GpuContext> {
    match GpuContext::new_blocking() {
        Ok(context) => Some(context),
        Err(_) if std::env::var_os("SIMTHING_GPU_REQUIRE_ADAPTER_MATCH").is_some() => {
            panic!("GPU adapter required for AUTHORED-LAW-GADGET-0 production-consumer proof")
        }
        Err(_) => None,
    }
}

fn gpu_nodes(nodes: &[eml_nodes::EmlNode]) -> Vec<EmlNodeGpu> {
    nodes
        .iter()
        .map(|node| EmlNodeGpu {
            opcode: node.opcode,
            flags: node.flags,
            a: node.a,
            b: node.b,
            c: node.c,
            d: node.d,
        })
        .collect()
}

#[test]
fn authored_power_law_executes_through_production_accumulator_eml() {
    let authored = deserialize_eml_gadget_stack_ron(
        r#"(
            gadgets: [(
                kind: "PowerLaw",
                id: "vendor-square-law",
                input_col: 0,
                output_col: Some(0),
                exponent: 2.0,
                input_floor: 0.25,
            )],
        )"#,
    )
    .expect("vendor-authored PowerLaw RON must deserialize as data");
    let compiled = compile_eml_gadget_stack(&authored, EmlGadgetCompileOptions { max_col: 1 })
        .expect("authored law must admit before execution");
    assert_eq!(compiled.report.gadget_kinds, ["PowerLaw"]);
    assert!(compiled.composition.flatten_preview_executable());

    let host_nodes = compiled.gadgets[0].nodes.clone();
    assert_eq!(
        host_nodes
            .iter()
            .map(|node| node.opcode)
            .collect::<Vec<_>>(),
        vec![
            eml_nodes::opcode::SLOT_VALUE,
            eml_nodes::opcode::CLAMP_BOUNDED,
            eml_nodes::opcode::LN,
            eml_nodes::opcode::LITERAL_F32,
            eml_nodes::opcode::MUL,
            eml_nodes::opcode::CLAMP_BOUNDED,
            eml_nodes::opcode::EXP,
            eml_nodes::opcode::RETURN_TOP,
        ],
        "authored law must remain the canonical guarded EXP(k * LN(x)) composition",
    );

    let metadata = EmlFormulaMeta {
        tree_id: TREE_ID,
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask(EmlConsumerMask::ALL_PRODUCTION),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: host_nodes.len() as u32,
        max_stack_depth: 2,
        has_loops: false,
        has_recursion: false,
        display_name: "authored-power-law-production-consumer".into(),
    };
    let mut registry = EmlExpressionRegistry::new();
    registry
        .register_formula(TREE_ID, metadata.clone(), host_nodes.clone())
        .expect("the existing exact-primitive registry gate must admit the authored law");

    let Some(context) = gpu_context() else {
        return;
    };
    let _readback = scoped_debug_readback_allowed(true);
    let encoded_nodes = gpu_nodes(&host_nodes);
    let mut table = EmlGpuProgramTable::new(&context, 32, 4);
    let uploaded = table
        .upload_trees(&context, &[(TREE_ID, metadata, encoded_nodes)])
        .expect("existing production EML table must accept the authored law");
    for (tree_id, table_index) in uploaded {
        registry
            .mark_tree_uploaded(tree_id, table_index, table.generation)
            .expect("uploaded authored tree remains bound to its admitted identity");
    }

    let column = ColumnIndex::from_raw_for_oracle_or_rehearsal(0);
    let op = AccumulatorOp {
        source: SourceSpec::SlotValue {
            slot: SlotIndex::new(0),
            col: column,
        },
        combine: CombineFn::EvalEML { tree_id: TREE_ID.0 },
        gate: GateSpec::Always,
        scale: ScaleSpec::Constant(1.0),
        consume: ConsumeMode::ResetTarget,
        targets: vec![(SlotIndex::new(0), column)],
    };
    let upload = PackedAccumulatorUpload::from_ops_with_eml(&[op], Some(&registry))
        .expect("ordinary AccumulatorOp packing must consume the authored EML tree");
    let input = [4.0f32];
    let mut session = AccumulatorOpSession::new_attached(&context, 1, 1, 1);
    session.upload_values(&context, &input);
    session.copy_values_to_previous(&context);
    session
        .upload_packed_ops(&context, &upload)
        .expect("upload ordinary EvalEML op");
    session
        .tick_with_eml(&context, 0, Some(&table))
        .expect("ordinary production EvalEML dispatch");
    let observed = session.readback_full(&context).expect("proof readback")[0];
    assert_eq!(
        observed.to_bits(),
        oracle_power_law(input[0], 2.0, 0.25).to_bits(),
        "authored data must execute through the existing production EvalEML path",
    );
}

#[test]
fn opcode_census_is_unchanged_and_pow_remains_absent() {
    const CENSUS: &str = include_str!("../../../scripts/ci/constitutional_surfaces.tsv");
    const OPCODE_SOURCE: &str = include_str!("../../simthing-core/src/eml_nodes.rs");
    const BASELINE: &str = "LITERAL_F32,SLOT_VALUE,PARAM,TARGET_VALUE,NEIGHBOR_VALUE,ADD,SUB,MUL,NEG,DIV,MIN,MAX,CLAMP_BOUNDED,CLAMP_FLOORED,ABS,FLOOR,EXP,LN,CMP_LT,CMP_LE,CMP_GT,CMP_GE,CMP_EQ,SELECT,RETURN_TOP";

    let opcode_row = CENSUS
        .lines()
        .find(|line| line.starts_with("EML-OPCODE-LIBRARY\t"))
        .expect("constitutional opcode row");
    let admitted_members = opcode_row
        .split('\t')
        .nth(4)
        .expect("opcode row admitted_members column");
    assert_eq!(admitted_members, BASELINE);
    assert!(!admitted_members.split(',').any(|name| name == "POW"));
    assert!(!OPCODE_SOURCE.contains("pub const POW"));
}
