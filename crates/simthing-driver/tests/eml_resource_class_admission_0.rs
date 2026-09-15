use simthing_core::{
    eml_opcode, ColumnIndex, EmlConsumerMask, EmlExecutionClass, EmlExpressionRegistry,
    EmlFormulaMeta, EmlNodeGpu, EmlRegistryError, EmlResourceClass, EmlTreeId,
};
use simthing_driver::{
    compile_gu_yang_n4_field_sweeps, compile_palma_n4_field_sweep, GuYangN4FieldSweepSpec,
    PalmaN4FieldSweepSpec,
};
use simthing_gpu::{
    apply_field_sweep_registration, field_param, CanonicalOrderProof, FieldAdjacency,
    FieldLawProof, FieldSweepOutput, FieldSweepRegistrationRequest,
};

fn admitted_col(raw: u32, n_dims: u32) -> ColumnIndex {
    ColumnIndex::try_from_admitted_authored(raw, n_dims).expect("bounded column")
}

fn node(opcode: u32, a: u32) -> EmlNodeGpu {
    EmlNodeGpu {
        opcode,
        flags: 0,
        a,
        b: 0,
        c: 0,
        d: 0,
    }
}

fn literal(value: f32) -> EmlNodeGpu {
    node(eml_opcode::LITERAL_F32, value.to_bits())
}

fn exact_meta(tree_id: EmlTreeId) -> EmlFormulaMeta {
    EmlFormulaMeta {
        tree_id,
        execution_class: EmlExecutionClass::ExactDeterministic,
        allowed_consumers: EmlConsumerMask::default(),
        max_abs_error: None,
        deterministic_gpu: true,
        requires_guard_for_hard_threshold: false,
        node_count: 0,
        max_stack_depth: 0,
        has_loops: false,
        has_recursion: false,
        display_name: "resource-class-boundary".to_owned(),
    }
}
