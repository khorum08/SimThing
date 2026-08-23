# FIELD-SWEEP-SESSION-SEAM-0 results

- Track: 0.0.8.7 RF arena modernization (remedial 11.1a)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/field-sweep-session-seam-0`
- branch base: `6b012e1377d8d292bff3e8237179053654b470f1`
- implementation / tested / final head: PR-body-bound after the evidence commit; this file does not self-hash
- HD-RECEIPT: `d08d00d27308`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation rule stamp: `61818ff7d4adda84`
- orientation digest: `4e5f9cf2623ca69114e7fbfc9eeec9e25157dc1f12e1cc35145a6fec42eb30c9`
- expected route: `DA-RESERVE(gate-wiring)`
- hosted workflow IDs and exact tested head: bound in the PR body and board return after hosted inspection

## Pre-code archaeology and attach point

The existing production route was mapped before edits:

```text
compile_structured_field_sweeps
    -> FirstSliceMappingSession::open_preview_with_budget
    -> FieldSweepSession binding(s)
    -> FirstSliceMappingSession::dispatch_logical_field_step
    -> dispatch_chain / resident export
    -> ordinary SimSession hot tick
```

The attach point is the `field_registrations` vector in
`open_preview_with_budget`, before the existing `FieldSweepSession`
construction. Caller-admitted products are appended there. Registrations with
different admitted adjacency bindings are partitioned into contiguous binding
groups, preserving transient producer/consumer adjacency while every group
still executes through the one `dispatch_logical_field_step` path.

`SimSession::open_from_spec_with_admitted_field_sweeps` is the explicit
consumer seam. It accepts admitted generic registrations and the three Triad
columns, calls `admit_comparative_from_field_plan`, assigns the admission to
`SpecSessionState.comparative_projection`, and passes the combined producer +
comparative chain into `FirstSliceMappingSession`. `install.rs` remains a
field-plan producer only and contains no Triad-column default.

## Load-bearing proof

| Test | Defect caught |
|---|---|
| `ordinary_session_executes_admitted_palma_guyang_and_observes_comparative` | Opens the ordinary production session with real admitted PALMA and Gu-Yang registrations, runs `step_once`, observes existing `FieldSweepSession` dispatch telemetry, and reads a non-zero comparative margin. It also requires the live `SpecSessionState.comparative_projection` assignment. |
| `session_seam_mutant_and_shape_seals_remain_closed` / assignment seal | Removing the production assignment REDs with `FIELD-SWEEP-SESSION-SEAM-ASSIGNMENT-REMOVAL`. |
| `session_seam_mutant_and_shape_seals_remain_closed` / execution-path seal | A second production `FieldSweepSession` constructor or CPU proof executor REDs with `FIELD-SWEEP-SESSION-SEAM-SECOND-EXECUTION-PATH`. |
| `session_seam_mutant_and_shape_seals_remain_closed` / install seal | An install-time Triad/default-comparative plant REDs with `FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT`; an ordinary preview still has a field plan and no comparative admission. |
| `session_seam_mutant_and_shape_seals_remain_closed` / public-shape seal | `chokepoint`, `corridor`, `front`, and `dominance` remain zero public driver-function surfaces. |

The production test uses the existing generic `FieldSweepRegistration` and
`FieldSweepSession` types. No kernel, evaluator, scheduler, registry, service,
CPU decision path, Vendor Door, facade, or successor-rung surface was added.
Comparative readback is observation only.

## Local verification

| Command | Result |
|---|---|
| `cargo check -p simthing-driver` | PASS (pre-existing warnings only) |
| `cargo test -p simthing-driver --test field_sweep_session_seam_0` | PASS — 2 passed, 0 failed |
| `cargo test -p simthing-driver --test comparative_default_birth_0` | PASS — 11 passed, 0 failed; live NVIDIA RTX 4080 Vulkan adapter |
| assignment-removal planted mutant | RED as required — `FIELD-SWEEP-SESSION-SEAM-ASSIGNMENT-REMOVAL` |
| second-execution-path planted mutant | RED as required — `FIELD-SWEEP-SESSION-SEAM-SECOND-EXECUTION-PATH` reported two constructor sites instead of one |
| install-time Triad-default planted mutant | RED as required — `FIELD-SWEEP-SESSION-SEAM-INSTALL-TRIAD-DEFAULT` |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS — 1305 inventory rows / 1305 discovered tests |
| `python scripts/ci/detachability_check.sh` | PASS — production = 0, proof = 0, ceiling = 0 |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |
| `bash scripts/ci/gen_orientation.sh --check` | PASS |
| `bash scripts/ci/gen_digest.sh --check` | PASS |
| `bash scripts/ci/anchor_check.sh --check` | PASS (37/57 unanchored diagnostics remain INSPECT) |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS |
| `bash scripts/ci/agent_scan.sh` | PASS — embedded PR-delta Doctrine PASS, 0 failures, 0 inspection findings |
| `bash scripts/ci/doctrine_scan.sh --pr-delta <base> <head>` | PASS — 0 failures, 0 inspection findings |
| `bash scripts/ci/doctrine_scan.sh` (ambient whole tree) | INSPECT — 0 failures, 419 pre-existing whole-tree inspection findings; test-budget and guard-kabuki checks PASS |

All 47 handoff-projected required anchors were read through
`scripts/ci/anchor_query.sh` before edits; exact reach records and content
hashes are carried in `scripts/ci/anchor_reach_log.tsv`. The load-bearing
constitutional acknowledgements include:

- `structural-execution-convergence@6b4cedec482b`
- `simthing-0087-binding-laws@8f13cba4aa7a`
- `simthing-0087-pillars@61487cba1f9e`
- `actionband-field-triad-authority@56cf5cdf2d2c`
- `stead-spatial-contract-core@8585db4ac631`
- `field-policy-time-decisions@993c7d0560e8`
- `workshop-candidate-homing@3e584f0ad175`

## Scope and routing

- No Vendor Door/facade work (11.1b), guide/exemplar/admission-gate work
  (11.2), successor work, merge, graduation, or pointer move.
- No `/clearance` or `/relay-lint` invocation by coding.
- Expected return: **PROBATION / proof-present / DA-review-pending**.
- Expected review route: **DA-RESERVE(gate-wiring)**.
