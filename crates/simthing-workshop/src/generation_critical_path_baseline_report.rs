//! Lossless text projection of the 14.1 generation-critical-path baseline packet.

use crate::generation_critical_path_baseline::{BaselinePacket, LegSamples, WorkloadReport};

pub fn format_baseline_report(packet: &BaselinePacket) -> String {
    let mut out = String::new();
    out.push_str("GENERATION-CRITICAL-PATH-BASELINE-0\n");
    out.push_str("comparator only; not a go/no-go gate; no portable timing law\n\n");
    out.push_str("## Envelope\n");
    let env = &packet.envelope;
    out.push_str(&format!("tested_commit: {}\n", env.tested_commit));
    out.push_str(&format!("utc_date: {}\n", env.utc_date));
    out.push_str(&format!("cpu: {}\n", env.cpu));
    out.push_str(&format!("gpu: {}\n", env.gpu));
    out.push_str(&format!("adapter_backend: {}\n", env.adapter_backend));
    out.push_str(&format!("driver: {}\n", env.driver));
    out.push_str(&format!("os: {}\n", env.os));
    out.push_str(&format!("compiler_toolchain: {}\n", env.compiler_toolchain));
    out.push_str(&format!("profile: {}\n", env.profile));
    out.push_str(&format!("deterministic_seed: {}\n", env.deterministic_seed));
    out.push_str(&format!("exact_command: {}\n\n", env.exact_command));

    out.push_str("## Host clearing-door census\n");
    for door in &packet.door_census {
        out.push_str(&format!(
            "- {} @ {}:{} authority={} posture={} 14.6={} reexports={} callers={}\n",
            door.symbol,
            door.path,
            door.line,
            door.generation_authority_form,
            door.ordinary_or_oracle_posture,
            door.disposition_14_6,
            door.reexports,
            door.callers
        ));
    }
    out.push('\n');
    out.push_str("## Path diagram\n");
    out.push_str(&packet.path_diagram);
    out.push_str("\n\n## Leg definitions\n");
    for leg in &packet.leg_definitions {
        out.push_str(&format!("- {}: {}\n", leg.name, leg.boundary));
    }
    out.push('\n');
    out.push_str(&format!(
        "## Disclaimer\n{}\n\n",
        packet.comparator_disclaimer
    ));
    out.push_str(&format!(
        "## D1 signed remainder\n{}\n\n",
        packet.d1_signed_remainder_note
    ));
    out.push_str(&format!(
        "## D2 envelope shape\n{}\n\n",
        packet.d2_envelope_shape
    ));
    out.push_str(&format!(
        "## D3 N+1 boundary\n{}\n\n",
        packet.d3_nplus_boundary
    ));
    out.push_str(&format!(
        "## D6 samplewise residual\n{}\n\n",
        packet.d6_residual_definition
    ));

    for workload in &packet.workloads {
        out.push_str(&format_workload(workload));
        out.push('\n');
    }
    out.push_str("## Lossless JSON follows\n");
    match serde_json::to_string_pretty(packet) {
        Ok(json) => {
            out.push_str(&json);
            out.push('\n');
        }
        Err(err) => {
            out.push_str(&format!("json_error: {err}\n"));
        }
    }
    out
}

fn format_workload(workload: &WorkloadReport) -> String {
    let c = &workload.cardinalities;
    let mut out = format!("## Workload {}\n", c.name);
    out.push_str(&format!(
        "trees={} claims_per_tree={} claims_total={} scopes={} supplies={} claimants={} granters={} generations={:?} overlapping={} door={}\n",
        c.tree_count,
        c.claims_per_tree,
        c.claims_total,
        c.scopes_per_tree,
        c.supplies_per_tree,
        c.claimants_per_tree,
        c.granters_per_tree,
        c.generations,
        c.overlapping_raw_local_ids,
        c.overlapping_construction_door
    ));
    out.push_str(&format!(
        "warm_ups={} samples={} setup_allocation_ns={} grants_total={} unresolved_total={}\n",
        workload.warm_up_count,
        workload.sample_count,
        workload.setup_allocation_ns,
        workload.grants_total,
        workload.unresolved_total
    ));
    out.push_str(&format!(
        "isolation_matches_production={} neutrality_clears_identical={}\n",
        workload.isolation_matches_production, workload.neutrality_clears_identical
    ));
    out.push_str(&format_leg(&workload.enclosing_clear_ns));
    out.push_str(&format_leg(&workload.end_to_end_ns));
    for leg in &workload.legs {
        out.push_str(&format_leg(leg));
    }
    out.push_str(&format_leg(&workload.observation_overhead_residual));
    out.push_str(&format!(
        "difference_of_medians_ns={} (derived figure; not the residual)\nreconciliation: {}\n",
        workload.difference_of_medians_ns, workload.reconciliation_note
    ));
    if let Some(raw) = workload.overlapping_raw_value {
        out.push_str(&format!(
            "overlapping_raw={} contexts={:?}\n",
            raw, workload.overlapping_tree_contexts
        ));
    }
    let k = &workload.coupling;
    out.push_str(&format!(
        "coupling: simthing_allocator={} overlay_allocator={} host_lock={} shared_generation={} shared_schedule={} all_tree_sync={}\n",
        k.process_global_simthing_allocator,
        k.process_global_overlay_allocator,
        k.host_wide_clearing_lock,
        k.shared_generation_authority,
        k.shared_integration_schedule,
        k.all_tree_synchronization
    ));
    out
}

fn format_leg(leg: &LegSamples) -> String {
    format!(
        "leg {} isolation={} bytes_read_back={} bytes_uploaded={} median_ns={} p95_ns={} variance_ns2={:.3} min_ns={} max_ns={} mean_ns={:.3} samples={:?}\n",
        leg.name,
        leg.isolation,
        leg.bytes_read_back,
        leg.bytes_uploaded,
        leg.median_ns,
        leg.p95_ns,
        leg.variance_ns2,
        leg.min_ns,
        leg.max_ns,
        leg.mean_ns,
        leg.sample_ns
    )
}
