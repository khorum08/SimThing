//! ORACLE-SURFACE-QUARANTINE-0 — keep CPU reference doors out of ordinary Run vocabulary.
//! HD-RECEIPT: `2a1a98c9fcca`

use std::collections::BTreeSet;

const CPU_DOORS: [&str; 5] = [
    "clear_constrained_claims_at_generation",
    "clear_reduced_owner_channels",
    "clear_reduced_owner_channels_at_generation",
    "clear_stamped_owner_channels",
    "produce_runtime_rf_next_generation_demands",
];

#[test]
fn embedder_oracle_view_is_one_conversion_free_explicit_alias() {
    let run = include_str!("../src/run.rs");
    let alias = "pub use simthing_spec::clear_constrained_claims_at_generation;";

    assert_eq!(
        run.lines().filter(|line| line.trim() == alias).count(),
        1,
        "the retained comparator must be one item alias, never a wrapper or duplicate"
    );
    assert!(
        run.contains("pub mod cpu_filter_oracle {")
            && run.contains("This is a conversion-free item alias"),
        "the alias must be visibly quarantined as reference-oracle vocabulary"
    );
    assert!(
        !run.lines().any(|line| {
            let line = line.trim();
            line.starts_with("pub fn clear_constrained_claims_at_generation")
                || line.starts_with("pub fn clear_stamped_owner_channels")
        }),
        "ORACLE-SURFACE-QUARANTINE-WRAPPER: the embedder must not own CPU clearing"
    );
    assert!(
        !run.lines()
            .any(|line| { line.trim() == "pub use simthing_spec::clear_stamped_owner_channels;" }),
        "the stamped door had no embedder consumer and must remain unexported"
    );

    let _: fn(
        &[simthing_embedder::populate::ConstrainedSupply],
        &[simthing_embedder::run::ConstrainedClaim],
        &simthing_embedder::run::AuthoredClearingProgram,
        simthing_embedder::run::ClearingRemainderAuthority,
    ) -> Result<
        Vec<simthing_embedder::run::ConstrainedClearingResult>,
        simthing_embedder::run::ConstrainedClearingError,
    > = simthing_embedder::run::cpu_filter_oracle::clear_constrained_claims_at_generation;
}

#[test]
fn five_spec_doors_and_constitutional_authority_censuses_remain_exact() {
    let constrained = include_str!("../../simthing-spec/src/spec/constrained_clearing.rs");
    let temporal = include_str!("../../simthing-spec/src/spec/runtime_rf_tick.rs");
    let definitions: BTreeSet<_> = constrained
        .lines()
        .chain(temporal.lines())
        .filter_map(|line| line.trim().strip_prefix("pub fn "))
        .filter_map(|tail| tail.split('(').next())
        .filter(|symbol| CPU_DOORS.contains(symbol))
        .collect();
    let expected: BTreeSet<_> = CPU_DOORS.into_iter().collect();
    assert_eq!(
        definitions, expected,
        "the CpuVendorizedOracle surface must remain exactly five definitions"
    );

    let census = include_str!("../../../scripts/ci/constitutional_surfaces.tsv");
    let embedder_row = census
        .lines()
        .find(|line| line.starts_with("CPU-CLEARING-ORACLE-EMBEDDER-REEXPORTS\t"))
        .expect("explicit embedder re-export census row");
    assert!(
        embedder_row.contains("run.rs::clear_constrained_claims_at_generation")
            && !embedder_row.contains("clear_stamped_owner_channels"),
        "the re-export census must match the one retained explicit alias"
    );
    let peer_row = census
        .lines()
        .find(|line| line.starts_with("RECURSIVE-FILTER-PEER-RUNTIME-AUTHORITY-RESIDUE\t"))
        .expect("peer-authority residue row");
    assert!(
        peer_row.contains("apply_owner_silo_runtime_disburse_down_cpu")
            && peer_row.contains("evaluate_owner_silo_disburse_down_with_rf_source"),
        "the frozen 5-to-2 peer-authority disposition must remain intact"
    );
}
