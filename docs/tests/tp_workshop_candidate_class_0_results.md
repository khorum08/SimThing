# TP-WORKSHOP-CANDIDATE-CLASS-0 Results

## Status

**DA-GRADUATED / COMPLETE** — merged [#1227](https://github.com/khorum08/SimThing/pull/1227) @ `0141f03c622bd0ec91976fdd59ddb95b0ecdb22e` (head `597cf030a7717c421d94d88674fe5ecce43360a6`). Gate-wiring DA stamp 2026-07-09.

## Identity

| Field | Value |
|---|---|
| Rung | `TP-WORKSHOP-CANDIDATE-CLASS-0` (workplan 8.6h harness adjacency) |
| PR | [#1227](https://github.com/khorum08/SimThing/pull/1227) |
| Merge SHA | `0141f03c622bd0ec91976fdd59ddb95b0ecdb22e` |
| tested_code_sha | `597cf030a7717c421d94d88674fe5ecce43360a6` |
| Kind | clearance-router class registration (gate-wiring) |

## Class

```text
class_id: tp-workshop-candidate-proof
envelope: 0.0.8.5-terran-pirate
requirements: tested_code_sha|coverage_basis|ci_green|workshop_only|no_engine_crate
```

Requires at least one `crates/simthing-workshop/{src,tests}/tp_*.rs` surface (docs-only multi-match rejected).

## Proof

`clearance_check.sh --selftest` PASS (34 fixtures), including clearable + mapeditor envelope + engine scope + missing proof fields.

## Non-claims

Not production mapeditor `.clause` API. Not UI picker. Not closeout. Not API admission.
