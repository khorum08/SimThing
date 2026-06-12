use simthing_driver::{
    run_runtime_0080_0_r1c_b, run_runtime_0080_0_r1c_c, Runtime0080R1cBInput, Runtime0080R1cCInput,
    RUNTIME_0080_0_R1C_B_ID, RUNTIME_0080_0_R1C_C_ID,
};

#[test]
fn r1c_default_opt_in_default_off_is_preserved() {
    let b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::default_simsession());
    assert!(!b.explicit_opt_in);
    assert!(b.default_off);
    assert!(b.disabled_no_op);
    assert_eq!(b.id, RUNTIME_0080_0_R1C_B_ID);
    assert!(!b.admitted);
    assert_eq!(b.allocation_rows_written, 0);

    let c = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::default_simsession());
    assert!(!c.explicit_opt_in);
    assert!(c.default_off);
    assert!(c.disabled_no_op);
    assert_eq!(c.id, RUNTIME_0080_0_R1C_C_ID);
    assert!(!c.admitted);
    assert_eq!(c.membership_delta_rows.len(), 0);
}

#[test]
fn r1c_fast_no_compaction_or_lineage_rewrite() {
    let b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::default_simsession());
    assert!(!b.resident_compaction_authority);
    assert!(!b.resident_lineage_rewrite_authority);
    assert!(!b.resident_fusion_compaction_authority);
    assert!(!b.resident_reenroll_scatter_authority);

    let c = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::default_simsession());
    assert!(!c.resident_compaction_authority);
    assert!(!c.resident_lineage_rewrite_authority);
    assert!(!c.resident_fusion_compaction_authority);
    assert!(!c.resident_reenroll_scatter_authority);
    assert!(!c.resident_m4a_authority);
}
