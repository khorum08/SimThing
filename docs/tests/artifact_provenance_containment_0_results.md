# ARTIFACT-PROVENANCE-CONTAINMENT-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.9c)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `97261e940e6b`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- Dispatch: Board comment `5157163643`
- Scope: 5.9c(b)+(c) only; accepted normalization-pump-2 and 6.0/6.1 remain untouched
- PR / tested code SHA: branch evidence pending publication

## Generated-artifact provenance

The live consumer census is source-derived from `gen_orientation.sh` and
`handoff_dispatch.sh`; it does not trust the historical count of 19 TSVs and
does not carry an artifact-name allowlist. Two currently consumed TSVs identify
themselves as generated and name their regenerators:

| Artifact | Consumers | Executable proof | Fixture authority |
|---|---|---|---|
| `property_admission_inventory.tsv` | orientation + handoff | `anchor_disposition_admission_0.rs` | crate-local `ct2a_micro_economy.clause` + `disposition_admission_minimal.clause` |
| `specialization_citizen_counts.tsv` | orientation + handoff | `first_citizen_specialists_0.rs` | crate-local `specialist_citizens_minimal.clause` |

The gate resolves each artifact header to its generator, resolves the Cargo
integration target invoked by that generator, follows local Rust modules, and
requires an existing fixture read. Active generator/proof code containing a
path into `scenarios/` hard-fails; comments and scenario content are not parsed.

The permanent planted fixture changes only the generator input reach:
crate-local `tests/fixtures/input.txt` is green, `scenarios/input.txt` is red,
and restoring the fixture reach returns green.

```text
ARTIFACT-PROVENANCE-VERDICT: PASS generated=2 consumers=2 errors=0
ARTIFACT-PROVENANCE-SELFTEST: PASS (3 checks, 1 planted defect)
```

## DEAD-EXPORT blind shapes and ruled cleanup

`DEAD-EXPORT` remains advisory and now reports:

- a direct Rust integration target with zero test attributes/functions;
- a `tests/support/` module unreachable from every Rust consumer outside its
  support directory, with internal support dependency reachability followed;
- the pre-existing crate-root module/symbol-reexport reachability rule.

Selftests plant both new dead shapes, keep an externally consumed support module
live, and retain the external-symbol re-export protection. The fresh census
confirmed the two ruled files carried no executable test/current external
consumer and no inventory or lifecycle row. They were deleted without
replacement:

- `crates/simthing-spec/tests/planet_child_location_admission.rs` — direct
  integration target, zero test functions;
- `crates/simthing-driver/tests/support/resource_economy_session.rs` — no
  consumer outside `tests/support/` (and syntactically stale scaffolding).

Historical result documents remain historical evidence and were not rewritten.
The live advisory census remains `INSPECT` for other pre-existing shapes; the
hard containment counts are green at `scenario=0 domain=0`.

```text
SCENARIO-RESIDUE-SELFTEST: PASS (10 checks, 4 planted defects)
SCENARIO-RESIDUE-VERDICT: INSPECT scenario=0 domain=0 dead_exports=58
```

## Focused proof

| Check | Result |
|---|---|
| `cargo test -p simthing-spec --tests --no-run` | PASS |
| `cargo test -p simthing-driver --tests --no-run` | PASS |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS — 1047/1047, unledgered=0, stale=0 |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --scheduled` | PASS — expired=0 |
| `bash scripts/ci/gen_orientation.sh --check` | PASS |
| `bash scripts/ci/handoff_dispatch.sh --selftest` | PASS |
| `python scripts/ci/detachability_check.sh` | PASS — 0/0/0 |

The accepted specialization artifact remains scenario-neutral and unchanged at
`spatial=2 owner-seat=2 session-root=1`.

## Posture

**PROBATION / proof-present / DA-review-pending.** Coder does not merge, advance
the pointer, reopen 5.9c(a), or begin 6.1+.
