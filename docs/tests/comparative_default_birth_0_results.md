# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **STOP — exact admitted-surface impossibilities (remand `5153670027`)**
- HD-RECEIPT: `b17e36045daf`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA seam authorization: Board `5151136145`
- Remand: Board `5153670027`
- Prior kabuki head (withdrawn): `da456707f44091dc27fab753ec151a771e35f7eb`

## Disposition

Coding **withdraws** the prior 5.8b implementation that crossed HD fences:

| Withdrawn | Why |
|---|---|
| Kernel public doors `FieldAdjacency::order_fingerprint`, `FieldLawProof::is_conservative`, `FieldSweepRegistration::field_law_proof` | New kernel/GPU public door; Scan `ALLOW-SEALED-PRODUCERS` hard-fail; HD forbids |
| `admit_field_plan_binding_with_neighbors` caller-supplied LinkGraph rows | Second topology authority; length-only validation |
| Role derivation via MIN/MUL/conservative fold heuristics | Semantic guesswork over generic registrations |
| `durable_emitter_class_id((order+1)*1000+col)` | Synthesized identity, not admitted authored identity |
| `compile_and_install_with_field_plan(..., Some(plan))` as the birth path | Parallel install side-door; not ordinary install with zero comparative wiring |

No allowlist row was added. Sealed-producer doors are removed rather than sanctioned.

## STOP — exact missing already-admitted facts

### S1 — LinkGraph neighbor transport (blocks seam A for LinkGraph)

**Fact unavailable:** a public, transportable copy of the **exact** LinkGraph neighbor order bound inside an admitted `FieldAdjacency`.

- `FieldAdjacency` keeps neighbor lists private (`lists` is not a public surface).
- Public surface is `grid_shape` / `grid_offsets_data` / `slots` / `degree_buckets` only.
- Grid can seal neighbor rows from public offsets (no new door). LinkGraph cannot, without either:
  - a **new kernel public accessor** for private link rows (forbidden new door), or
  - **caller-reauthored neighbor rows** beside the adjacency (forbidden second topology authority).

`FieldAdjacency: PartialEq` can prove two adjacencies are identical when both already exist, but it does not export neighbor rows for comparative oracle/border construction.

**Planted-proof requirement (LinkGraph topology substitute red)** cannot be discharged lawfully until S1 is closed by DA design of a transport artifact that is the same admission product as the FieldAdjacency, not a parallel payload.

### S2 — Unique role identity on field registrations (blocks seam B)

**Fact unavailable:** an already-admitted, unique binding that distinguishes emitter / PALMA-D / Gu-Yang-U / Gu-Yang-C without new grammar.

- `FieldLawProof` is sealed; production consumers are not meant to inspect it (kernel design).
- Exposing `is_conservative` / `field_law_proof` is a sealed-producer door (Scan FAIL).
- Classifying roles by generic fold opcodes (MIN/MUL) or residual matrix outputs is **heuristic inference**, not unique role identity — any unrelated min-plus or product-fold registration would be mislabeled.
- No existing `GameModeSpec` / install surface admits “emitter class” or triad column roles as typed registration identity distinct from generic field-sweep programs.
- Synthesizing `class_id` from order×column is a new identity convention, not an admitted authored identity.

Missing/ambiguous role evidence must reject. Without a unique admitted fact per required role, derivation must STOP rather than guess.

### S3 — Ordinary install has no field-plan admission product (blocks default birth on normal path)

**Fact unavailable:** an already-admitted field-plan registration binding produced **inside** ordinary `compile_and_install` that default birth can consume with zero comparative wiring.

- `compile_and_install` today admits properties, overlays, capability trees, RF, resource economy — not a comparative field-plan / `FieldSweepRegistration` set for triad+emitters.
- `GameModeSpec.region_fields` is a different substrate (RegionFieldSpec / mapping), not the sealed FieldSweepRegistration comparative input surface.
- A parallel install API that only births when the caller supplies `Some(plan)` is explicit comparative wiring by another name — remand forbids it as the 5.8b success path.

Until install (or an existing admission step it already owns) produces the field-plan artifact as an ordinary install product, default birth cannot lawfully attach to the normal path.

## What would be required (not designed here)

Per remand: **no speculative replacement design**. These three missing facts are escalated to DA as the exact seam question. Any solution that invents a second topology payload, a role enum/tag/string namespace, a sealed-producer door, or a parallel install enrollment is out of coding authority.

## 5.8 substrate

Settled 5.8 explicit fail-closed comparative admission remains valid and untouched. This STOP is 5.8b-only.

## Posture

**STOP.** Draft #1567. No `/clearance`, no pointer move, no 6.1+, no allowlist widen, no Gu-Yang throughput work.
