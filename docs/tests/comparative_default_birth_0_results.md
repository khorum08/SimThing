# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **STOP — exact missing admitted facts (orchestrator remand `5153911298`)**
- HD-RECEIPT: `b17e36045daf`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA seam ruling: Board `5153818317`
- Prior DA-amendment head (withdrawn): `5b7a606438a338d3111cb87846f0df1658ace41f`
- STOP head / tested_code_sha: `5d7fa09ad9c77de25b182a8460e67cf821fddc8b`
- Orchestrator remand: Board `5153911298`
- Prior remand / landing: `5153845512` / `5153898093`

## Disposition

Coding **withdraws** the post-DA amendment that failed the orchestrator deep
check. Hosted Doctrine Scan / Exec / Clearance were green; the DA amendment was
not yet embodied. Useful 5.8 reuse is preserved. Only the remanding defects are
addressed — by refusal to invent, not by a new surface.

| Withdrawn surface | Remand defect |
|---|---|
| `Scenario.field_plan_admission` + fixture churn + install clone-from-scenario | **1** — S3 re-homed as explicit side-door / caller enrollment |
| Role-named `FieldPlanAdmissionReport` fields (`emitter_registrations`, `palma_d`, `guyang_conductance`, `guyang_value`) and `admit_field_plan_report(...)` pre-sort API | **2** — typed role taxonomy by struct/API shape |
| `class_id = value_col.raw_u32() as f32 + 1.0` | **3** — synthesized emitter identity convention |
| Independent `admit_field_plan_report(adjacency, neighbor_slots, ...)` with length-only neighbor validation | **4** — non-atomic S1; same-length neighbor substitution still constructible |
| Authored-order proof that bypasses default derivation (`ComparativeEmitterClass { authored_order }` hand-built) | **5** — not the HD default-path referee |

No kernel/allowlist widen. No parallel `compile_and_install_with_field_plan`. No role enum/tag/string/property grammar. Settled 5.8 explicit substrate untouched.

## STOP — exact missing already-admitted facts

Remand instruction: *If defects 1/2/3 cannot be closed using an already-admitted
producer/identity, return one narrow STOP with the exact missing fact rather
than inventing another surface.*

### S3 producer (defect 1) — blocks ordinary zero-wiring birth

**Missing fact:** an already-admitted field-plan / lowering **producer reachable
from ordinary `compile_and_install`** that yields the typed product DA granted
(`FieldAdjacency` + same-authority neighbor rows + registrations in authored
order) as an **install output**.

Measured at remand head:

- `compile_and_install` admits properties, overlays, capability trees, RF,
  resource economy, and `PropertyAdmissionReport` — **not** a sealed
  `FieldSweepRegistration` set for triad+emitters, and not a comparative
  field-plan binding.
- `GameModeSpec.region_fields` is a different substrate (RegionFieldSpec /
  mapping), not the FieldSweepRegistration comparative input surface.
- Putting the product on `Scenario` (or any other caller input envelope) is
  enrollment / explicit comparative wiring by another name — remand forbids
  re-homing the input door.
- A parallel install API is also forbidden (DA + HD).

Until ordinary install **owns or invokes** a real field-plan lowering that mints
the product, default birth cannot lawfully attach to the normal path.

### S2 triad target-column identities (defect 2) — blocks role derivation without taxonomy

**Missing fact:** already-admitted triad **target-column identities**
(`palma_d_col`, `guyang_value_col`, `guyang_conductance_col`) at the real
field-plan lowering/admission site.

DA `5153818317` correctly dissolves opcode/heuristic classification and
sanctions: *PALMA-D is the registration whose `output() == Matrix(palma_d_col)`*
(etc.), with remaining Matrix registrations as emitters in authored order.

That derivation **requires the triad columns as admitted facts**. They are not
present on:

- `FieldSweepRegistration` public API (only `output()`, programs, adjacency, …)
- ordinary install / `SpecSessionState` without a field-plan producer
- the DA product shape as stated (adjacency + neighbors + registrations in
  authored order only)

Without those column identities, callers must pre-sort registrations into
role-named slots (the withdrawn taxonomy) or invent a grammar. Coding stops.

### Emitter `class_id` identity (defect 3)

**Missing fact:** an already-admitted authored emitter identity value suitable
for `ComparativeEmitterClass::class_id`.

- 5.8 explicit path consumes caller-authored `class_id` (tests use `10.0` /
  `20.0`) plus durable `authored_order` — independent of column index.
- `FieldSweepRegistration::output() == Matrix(col)` supplies **value_col**, not
  an emitter identity.
- `class_id = col.raw_u32() + 1` is a new identity convention; remand
  `5153845512` already ordered STOP rather than invent one.

Authored order alone (enumerate over a registration vec) is also insufficient
for the HD default-path referee (defect 5): incidental vector reversal must not
change derived `authored_order` when the admitted identity/order is fixed.

### S1 atomic capture (defect 4) — implementable only once S3 producer exists

DA dissolved the "need a LinkGraph accessor" STOP. Capture-at-construction
alongside `link_graph()` remains the sanctioned 5.8 seam.

What is still required (and blocked on S3): the product must make
**adjacency + neighbor rows one same-authority admission artifact** so a caller
cannot keep the correct adjacency and substitute same-length neighbor membership.
That falsifier is named; it is not constructible without inventing a sealed
pair while the product has no producer.

## Focused proof (STOP witnesses)

```text
comparative_default_birth_0: 2 passed; 0 failed
```

1. `ordinary_install_does_not_invent_comparative_birth`
2. `scenario_has_no_field_plan_admission_side_door`

## 5.8 substrate

Settled 5.8 explicit fail-closed comparative admission remains valid and
untouched (`guyang_comparative_projections_0`). This STOP is 5.8b-only.

## Posture

**STOP.** Draft #1567. No `/clearance`, no pointer move, no 6.1+, no allowlist
widen, no Gu-Yang throughput work, no owner-channel rename residual.

Escalation to DA: name the ordinary-install field-plan **producer**, the admitted
triad **target-column** bindings, and the admitted emitter **class_id** /
durable **authored_order** facts — or amend the rung if those facts are not
meant to exist yet.
