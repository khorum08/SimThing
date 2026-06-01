# MOBILITY-IDROUTE-0 - local D=2 identity-routing overlay substrate results

Date: 2026-06-02

## Verdict

**PASS**

MOBILITY-IDROUTE-0 implemented the authorized local D=2 identity-routing overlay substrate on top of ALLOC-0 + REENROLL-0. The implementation is confined to `simthing-spec` designer-admission/substrate modeling and tests; it does not wire production runtime behavior, ECON, OWNER, or global routing.

**R1 reconciliation:** MOBILITY-IDROUTE-0-R1 supersedes the original test-count reporting mismatch. The reconciled battery is 20 explicit tests (7 substrate floor + 10 guardrails + 3 performance bars) and is recorded in [`phase_mobility_idroute0_r1_results.md`](phase_mobility_idroute0_r1_results.md).

## Implemented Substrate Scope

- Local per-cell D=2 identity routing modeled as column-based masked gather + directed disburse.
- Identity represented strictly as `IdentityLane` column on local records.
- Deterministic per-identity masked `Sum` and packed-key `Max` / argmax.
- Directed disburse from identity columns back to local children, purely functional / immutable-by-construction.
- CPU/GPU-proxy layout checksums.
- Rejection coverage for global faction vectors, owner-as-spatial-parent, capture-as-reparenting, ECON/OWNER runtime, production `SimSession` wiring, default-on behavior, semantic/raw WGSL, and `max_factions_per_cell` violations.

## Commands Run

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget
cargo test -p simthing-spec --test mobility_alloc0_substrate
cargo test -p simthing-spec --test mobility_reenroll0_substrate
cargo test -p simthing-spec --test mobility_idroute0_substrate
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation
cargo check --workspace
```

## Posture Attestation

MOBILITY-IDROUTE-0 substrate is green. ECON and OWNER remain proposed/parked. No production runtime integration, GPU kernels, default-on flags, semantic/raw WGSL, `simthing-sim` semantic awareness, CPU planner, CPU urgency computation, CPU commitment emission, or invariant changes were introduced.
