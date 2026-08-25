# DIMENSION-FINALIZATION-SEAM-0 results

- Track: 0.0.8.7 RF arena modernization (remedial 11.1b)
- Status: **COMPLETE — DA-GRADUATED / merged #1813 @ `5d7000bd`** (Fable deep review, graduation ruling on Board #1332)
- Branch: `codex/dimension-finalization-seam-0`
- Branch base: `979dcb0e51902587476d8b58d7830352792f434d`
- Implementation / tested / final head: PR-body-bound after the evidence commit; this file does not self-hash
- HD-RECEIPT: `4ed070f7ace4`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- Orientation digest: `d1a0f5d9edf11e082136736c2abda5ae8604c381e7400acbe83e52c7159448bc`
- Expected route: `DA-RESERVE(gate-wiring)`
- Hosted workflow IDs and exact tested head: bound in the PR body and board return after hosted inspection

## Archaeology and attach point

The pre-code route was confirmed as:

```text
install_atomic(GameModeSpec at authored RegionFieldSpec.n_dims)
    -> admit_comparative_from_field_plan(grows live DimensionRegistry)
    -> install_spec_state(resizes ordinary session state)
    -> install_session_mapping(original GameModeSpec)
    -> FirstSliceMappingSession
```

Before this rung, the 11.1a witness projected the registry growth itself,
compiled caller registrations at that predicted width, and rewrote every
authored `RegionFieldSpec.n_dims`. Deleting only the rewrite produced the
required pre-code RED:

```text
AdmittedFieldSweepBindingMismatch {
    actual_dims: 57,
    expected_dims: 128,
}
```

The finalization stage attaches in
`FirstSliceMappingSession::open_with_finalized_field_sweeps`, reached after
comparative admission and before the ordinary preview or caller registrations
are compiled. That one surface reads `DimensionRegistry.total_columns`, binds
the compiled ordinary preview to it, invokes the deferred caller compiler with
it, and appends both caller and comparative registrations to the existing
field-sweep chain. Authored `GameModeSpec` data is only borrowed and remains
unchanged.

## Load-bearing proof

| Proof | Result |
|---|---|
| Primary deletion | The witness contains neither the `field.n_dims = final_n_dims` rewrite nor its projected-registry helper and passes. |
| No prediction | The caller supplies a compiler, not a width. The witness records the compiler argument and proves it equals the live post-admission registry width. |
| Same final width | The ordinary mapping preview width equals the width received by the caller compiler; all registrations pass the existing binding check. |
| Authored input unchanged | The witness snapshots both authored region-field widths and proves they are byte-for-byte unchanged after session open. |
| Real adapter | The ordinary session compiles PALMA, Gu-Yang, and the resident continuation at the final width, executes `step_once`, and observes comparative output on `NVIDIA GeForce RTX 4080 Laptop GPU` / Vulkan with adapter match required. |
| Single authority | `dimension_finalization_single_authority_and_no_prediction_seals` requires exactly one production `DIMENSION-FINALIZATION-SEAM-0-AUTHORITY` site. A planted second site REDs with its named reason and reports both sites. |
| 5.8b preserved | A planted install-time `triad_columns` default REDs with `FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT`; `install.rs` remains free of comparative admission/defaults. |
| Convergence | Diff from the branch base adds no `FieldSweepSession::new`, CPU proof executor, scheduler, kernel, evaluator, or registry. The only constructor remains the existing contiguous binding in `mapping_runtime.rs`. |

## Verification

| Command / mutant | Result |
|---|---|
| deletion-only test before implementation | RED as required — caller registration width 57 versus ordinary authored width 128 |
| `SIMTHING_GPU_REQUIRE_ADAPTER_MATCH=1 cargo test -p simthing-driver --test field_sweep_session_seam_0 ordinary_session_executes_admitted_palma_guyang_and_observes_comparative -- --exact --nocapture` | PASS — live NVIDIA RTX 4080 Laptop GPU / Vulkan |
| `cargo test -p simthing-driver --test field_sweep_session_seam_0` | PASS — 3 passed, 0 failed |
| planted second-finalization-site mutant | RED as required — `DIMENSION-FINALIZATION-SEAM-0-SECOND-AUTHORITY` reported 2 sites |
| planted install-time Triad-default mutant | RED as required — `FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT` |
| required CI / doctrine checks | Recorded after final code-head verification in the PR body and board return |

All 47 handoff-projected required anchors were acknowledged through
`scripts/ci/anchor_query.sh` before edits. The two query receipts (the
`gate-wiring` domain and all five projected paths) are recorded in
`scripts/ci/anchor_reach_log.tsv`.

## Scope and routing

- No Vendor Door/facade, guide, exemplar, admission-gate, successor-rung, merge,
  graduation, or pointer work was performed.
- No `/clearance` or `/relay-lint` was invoked by coding.
- Expected return: **PROBATION / proof-present / DA-review-pending**.
- Expected review route: **DA-RESERVE(gate-wiring)**.
