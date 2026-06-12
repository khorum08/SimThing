//! Fast default-workspace sentinels for RUNTIME-0080-0-R1* rungs.
//! Historical proof-ledger/report/checksum batteries live in `docs/archive/superseded_tests/` only.

use simthing_driver::{
    run_runtime_0080_0_r1a, run_runtime_0080_0_r1b, run_runtime_0080_0_r1c,
    run_runtime_0080_0_r1c_a, run_runtime_0080_0_r1c_b, run_runtime_0080_0_r1c_c,
    run_runtime_0080_0_r1c_d, run_runtime_0080_0_r1c_e, run_runtime_0080_0_r1c_f,
    Runtime0080R1aInput, Runtime0080R1bInput, Runtime0080R1cAInput, Runtime0080R1cBInput,
    Runtime0080R1cCInput, Runtime0080R1cDInput, Runtime0080R1cEInput, Runtime0080R1cFInput,
    Runtime0080R1cInput, RUNTIME_0080_0_R1A_ID, RUNTIME_0080_0_R1B_ID, RUNTIME_0080_0_R1C_A_ID,
    RUNTIME_0080_0_R1C_B_ID, RUNTIME_0080_0_R1C_C_ID, RUNTIME_0080_0_R1C_D_ID,
    RUNTIME_0080_0_R1C_E_ID, RUNTIME_0080_0_R1C_F_ID, RUNTIME_0080_0_R1C_ID,
};

#[test]
fn r1_fast_default_off_or_opt_in_contract() {
    let a = run_runtime_0080_0_r1a(&Runtime0080R1aInput::default_simsession());
    assert!(!a.explicit_opt_in);
    assert!(a.default_off);
    assert!(a.disabled_no_op);
    assert_eq!(a.id, RUNTIME_0080_0_R1A_ID);

    let b = run_runtime_0080_0_r1b(&Runtime0080R1bInput::default_simsession());
    assert!(!b.explicit_opt_in);
    assert!(b.default_off);
    assert!(b.disabled_no_op);
    assert_eq!(b.id, RUNTIME_0080_0_R1B_ID);
    assert!(!b.resident_event_journal_created);

    let c = run_runtime_0080_0_r1c(&Runtime0080R1cInput::default_simsession());
    assert!(!c.explicit_opt_in);
    assert!(c.default_off);
    assert!(c.disabled_no_op);
    assert_eq!(c.id, RUNTIME_0080_0_R1C_ID);

    let c_a = run_runtime_0080_0_r1c_a(&Runtime0080R1cAInput::default_simsession());
    assert!(!c_a.explicit_opt_in);
    assert!(c_a.default_off);
    assert!(c_a.disabled_no_op);
    assert_eq!(c_a.id, RUNTIME_0080_0_R1C_A_ID);

    let c_b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::default_simsession());
    assert!(!c_b.explicit_opt_in);
    assert!(c_b.default_off);
    assert!(c_b.disabled_no_op);
    assert_eq!(c_b.id, RUNTIME_0080_0_R1C_B_ID);
    assert_eq!(c_b.allocation_rows_written, 0);

    let c_c = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::default_simsession());
    assert!(!c_c.explicit_opt_in);
    assert!(c_c.default_off);
    assert!(c_c.disabled_no_op);
    assert_eq!(c_c.id, RUNTIME_0080_0_R1C_C_ID);
    assert!(c_c.membership_delta_rows.is_empty());

    let c_d = run_runtime_0080_0_r1c_d(&Runtime0080R1cDInput::default_simsession());
    assert!(!c_d.explicit_opt_in);
    assert!(c_d.default_off);
    assert!(c_d.disabled_no_op);
    assert_eq!(c_d.id, RUNTIME_0080_0_R1C_D_ID);
    assert_eq!(c_d.compaction_rows_written, 0);

    let c_e = run_runtime_0080_0_r1c_e(&Runtime0080R1cEInput::default_simsession());
    assert!(!c_e.explicit_opt_in);
    assert!(c_e.default_off);
    assert!(c_e.disabled_no_op);
    assert_eq!(c_e.id, RUNTIME_0080_0_R1C_E_ID);
    assert_eq!(c_e.slot_remap_rows_written, 0);

    let c_f = run_runtime_0080_0_r1c_f(&Runtime0080R1cFInput::default_simsession());
    assert!(!c_f.explicit_opt_in);
    assert!(c_f.default_off);
    assert!(c_f.disabled_no_op);
    assert_eq!(c_f.id, RUNTIME_0080_0_R1C_F_ID);
    assert_eq!(c_f.zero_cohort_row_count, 0);
}

#[test]
fn r1_fast_no_authority_or_cpu_redecision_on_default_off() {
    let b = run_runtime_0080_0_r1c_b(&Runtime0080R1cBInput::default_simsession());
    assert!(!b.resident_compaction_authority);
    assert!(!b.resident_lineage_rewrite_authority);
    assert!(!b.resident_fusion_compaction_authority);
    assert!(!b.resident_reenroll_scatter_authority);
    assert!(!b.cpu_selected_any_slot);

    let c = run_runtime_0080_0_r1c_c(&Runtime0080R1cCInput::default_simsession());
    assert!(!c.resident_compaction_authority);
    assert!(!c.resident_lineage_rewrite_authority);
    assert!(!c.resident_fusion_compaction_authority);
    assert!(!c.resident_reenroll_scatter_authority);
    assert!(!c.resident_m4a_authority);
    assert!(!c.cpu_shadow.cpu_selected_membership_effects);

    let d = run_runtime_0080_0_r1c_d(&Runtime0080R1cDInput::default_simsession());
    assert!(!d.cpu_shadow.cpu_decided_any_compaction_row);
    assert!(!d.cpu_shadow.cpu_decided_any_lineage_row);
    assert!(!d.resident_m4a_authority);
    assert!(!d.default_session_wiring);

    let e = run_runtime_0080_0_r1c_e(&Runtime0080R1cEInput::default_simsession());
    assert!(!e.cpu_shadow.cpu_decided_any_slot_remap);
    assert!(!e.cpu_shadow.cpu_decided_any_compacted_table_row);
    assert!(!e.cpu_shadow.cpu_decided_any_lineage_application);
    assert!(!e.default_session_wiring);
}
