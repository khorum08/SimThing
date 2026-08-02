# COMPARATIVE-DEFAULT-BIRTH-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8b)
- Status: **STOP — exact representation / role-identity residue after DA `5154066190` + orchestrator resume `5154078214`**
- HD-RECEIPT: `b17e36045daf`
- ORIENT-RECEIPT: `4992234cbe01` (orientation_rule_stamp `ff44072551872eb1`)
- DA seam rulings: `5153818317` (prior) · **`5154066190` (canonical source)**
- Orchestrator remands: `5153911298` · `5154032743` (publication) · **`5154078214` (resume)**
- Board STOP (prior publication): `5154016887` · publication landing `5154053278`
- Published tip at resume: `91470b125f051f25e4717928962a0a1c810dd688`
- PR: #1567 draft

## Disposition

DA `5154066190` correctly names the **producer call-site** gap and the authored
surface `GameModeSpec.region_fields`. Coding **accepts** that chain as real:

```text
GameModeSpec.region_fields
  → compile_region_field_preview / CompiledRegionFieldStencilSpec
  → compiled_stencil_to_gpu_config → StructuredFieldStencilConfig
  → compile_structured_field_sweeps → Vec<FieldSweepRegistration>
```

That path already runs at tick/mapping (`first_slice_mapping_runtime`,
`mapping_plan_tick`). Moving the **same** call earlier into ordinary
`compile_and_install` is lawful work **once** the comparative inputs can be
instantiated without inventing identity or role grammar.

Orchestrator `5154078214` binds a hard fence on emitter identity:

> DA states emitter `class_id = RegionFieldSpec::name`. … current
> `ComparativeEmitterClass::class_id` is `f32` and dominance writes that
> numeric value. **Do not invent a string→float hash/index/parse convention
> and do not silently change settled 5.8 explicit semantics.** If no
> already-admitted numeric identity corresponding exactly to the authored
> `name` exists, … **STOP** with that exact representation mismatch.

Coding **STOPs** on that exact fact (and two related role-surface facts that
block completing S2 even if class_id were fixed).

No reimplementation of Scenario side-door / role-named report / col+1 synthesis.

## STOP — exact missing / mismatched facts

### 1. Emitter `class_id` representation mismatch (binding; orchestrator-named)

| Surface | Type / meaning |
|---|---|
| `RegionFieldSpec::name` (DA identity) | `String` — designer text (`"fabric_hot_mapping"`, `"frontier_v1_theater"`, …) |
| `ComparativeEmitterClass::class_id` (settled 5.8) | `f32` — written into the **dominance** column as an EML `LITERAL_F32` (`comparative_projection.rs` dominance step / CPU oracle) |

**Fact unavailable:** an already-admitted **numeric** identity that *is* the
authored `name` (or is definitionally equal to it) without conversion.

Forbidden by remand if used to “satisfy” DA literally:
- string→float hash / index / parse convention
- silent change of settled 5.8 `class_id: f32` meaning or dominance payload type

`source_col` / `target_col` are admitted `u32`s, but DA names **`name`**, not
those columns, as emitter identity — and using column indices as class_id is
exactly the withdrawn col-identity synthesis.

### 2. Triad role uniqueness from authored region-fields (S2; blocks lawful default birth)

DA requires matching registrations by `output() == Matrix(target_col)` for
PALMA-D / Gu-Yang-U / Gu-Yang-C, with remaining Matrix registrations as emitters
in **authored `region_fields` list order**.

Measured on the live chain:

| Required comparative input | What region-field lowering actually admits |
|---|---|
| **PALMA-D** Matrix column | `RegionFieldOperatorSpec` has **no PALMA / min-plus operator**. Operators are `Normalized`, `SourceCappedNormalized`, `Gradient`, `SaturatingFlux` only. PALMA lives on the separate `compile_min_plus_field_sweep` / min-plus config path, not on `compile_structured_field_sweeps` for region fields. |
| **Gu-Yang-U** Matrix column | `SaturatingFlux` flux registration writes `Matrix(source_col)` (value), **not** `Matrix(target_col)`. |
| **Gu-Yang-C** Matrix column | `SaturatingFlux` conductance is **`FieldSweepOutput::Transient`**, not a Matrix target column. 5.8 comparative stall reads a **Matrix** `guyang_conductance_col`. |

So even with admitted `target_col` on the compiled stencil, the produced
registrations **do not expose** a unique `output() == Matrix(palma_d_col)` /
`Matrix(guyang_value_col)` / `Matrix(guyang_conductance_col)` triple that can be
matched without inventing an operator→role map or a second column convention.

Remand S2: if existing authored/compiled region-field facts do not uniquely
identify the three triad entries without forbidden heuristics → **STOP**.

### 3. Authored-order provenance is clear; birth still blocked on (1)+(2)

DA/orchestrator correctly bind **authored order** as index in
`GameModeSpec.region_fields` (designer list), not incidental registration-vector
order. That provenance rule is accepted and does not require a STOP by itself.
It cannot be exercised for default birth until (1) and (2) close.

### 4. S1 atomic capture — still blocked on a lawful product

Same-authority adjacency+neighbor artifact at the lowering site remains the
correct S1 shape (adopted). It is not implementable as a default-birth product
until install can mint a product that comparative birth can lawfully consume.

## What is *not* missing (accepted from DA)

- The **producer function** exists: `compile_structured_field_sweeps`.
- The **authored list** exists: `GameModeSpec.region_fields`.
- The **call-site move** (mapping/tick → ordinary install) is the right shape of
  work once identity/role facts are expressible.
- Scenario side-door / parallel install API remain forbidden (unchanged).

## Focused proof (STOP witnesses; no kabuki birth)

```text
comparative_default_birth_0: 2 passed; 0 failed
  ordinary_install_does_not_invent_comparative_birth
  scenario_has_no_field_plan_admission_side_door
guyang_comparative_projections_0: 5 passed  (5.8 explicit substrate untouched)
```

## Posture

**STOP.** Draft #1567. No `/clearance` rebind until a lawful head lands. No
pointer move, no 6.1+, no kernel/allowlist widen, no string→f32 convention, no
settled-5.8 class_id rewrite, no operator→role heuristic.

### Escalation to DA (exact)

1. **Representation:** how should authored `RegionFieldSpec::name: String` become
   the numeric identity written into dominance under settled 5.8 `class_id: f32`
   — or should 5.8 class_id/dominance be amended by DA design?
2. **Triad provenance:** which admitted region-field (or other authored) facts
   uniquely name PALMA-D / Gu-Yang-U / Gu-Yang-C Matrix columns when structured
   field sweeps do not emit PALMA and emit Gu-Yang conductance as Transient?
