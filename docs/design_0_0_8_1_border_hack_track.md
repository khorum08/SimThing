# SimThing 0.0.8.1 — Border Hack Track (`BH-`): the C_u saturating-flux stencil operator

> **Status: BH-0…BH-2S CLOSED; BH-2S-API-DOC handoff (2026-06-11).** Named consumer
> `CT-4b_Local_Automata_W_Feedstock` opens BH-2 W composition and BH-2S stress feedstock.
> BH-2C/BH-2D remain deferred. Seated as a generic GPU utility, PALMA-style. Product
> authorization: borders, frontlines, and choke topology as **free-ish side effects of the
> heatmap pass** on late-game crowded maps — no border service, no segmentation pass, no border
> objects. Adjudicated by executive design authority from the C_u proposition digest
> (Gu & Yang 2025, arXiv:2509.20797). Consumer anchor: the closed CT-3b+4a movement-front
> vertical ([`clausething/ct_vertical_consumer_contract.md`](clausething/ct_vertical_consumer_contract.md))
> and the SEAD horizon. One rung in flight; rung = one PR + one report + one status row.
>
> **Implementer note (binding): this track will be executed by Codex/Cursor-class agents under
> Gemini/Codex orchestration, without a frontier agent.** §2–§4 are therefore written as an
> exact implementation contract: pinned math, pinned evaluation order, pinned file map, pinned
> test names, and an explicit voiding list. Where this document and an implementer's instinct
> disagree, this document wins. Where this document is silent and the gap is load-bearing,
> **stop and leave a PARTIAL status row** (§6) — do not improvise.

---

## 1. Adjudication (binding)

1. **Not already expressible.** The admitted stencil operators (`Normalized`,
   `SourceCappedNormalized`, `Gradient`) carry admission-time constant weights. EvalEML cannot
   gather N4 neighbors (`SLOT_VALUE` reads the evaluated slot's own row). State-dependent
   weights require a **new generic stencil operator variant**. No prior EML hack covers this.
2. **Admissible under the standing Tier-2 gate** (invariants, EML Gadget Library row): the
   operator is semantic-free (WGSL sees floats and caps; no faction/border/map nouns),
   CPU-oracle bit-exact, meaning pinned at admission, reusable by any RegionField. Same
   admission shape as `source_capped_normalized`; same landing pattern as the dual-output
   `GradientXY` rung (SCENARIO-0080-2 rung 3, PR #451).
3. **Provenance caveat (binding on every BH rung).** The local product form below is an
   **engineering ansatz inspired by** Gu–Yang's hydrodynamic-limit results. It is **not** a
   theorem of the paper, and **no BH rung may claim paper fidelity in a PASS row.** What we
   claim — and test exactly — are the operator's own properties: conservation, bounded
   stability, saturation choking, fixed-point convergence.
4. **Decision doctrine.** Gradient descent over fields remains the primary decision-emergence
   arena. C_u decides nothing; it shapes the field so its gradients already encode crowding.
   Border positions are *readouts* of where flux froze — never objects, never a service.
5. **The conservation bonus.** The symmetric pairwise flux is antisymmetric under `i↔j`, so the
   operator conserves total mass by construction (closed grid, §2.3) — unlike the attenuating
   `Normalized` operator. This makes it the natural evolution operator for conservative RF-fed
   pressure. Conservation is property-tested to float-order determinism, never claimed
   exact-authoritative.

## 2. BH-0 implementation contract (pinned — implement exactly this)

### 2.1 The math, in the exact evaluation order both kernel and oracle must use

Reads come **only from the input (ping-pong read) buffer**; the write goes to the output
buffer. For cell `i` at `(row, col)`, neighbor order is fixed: **N, S, E, W**
(`row−1`, `row+1`, `col+1`, `col−1` — match the existing kernel's directional order).

```
σ(u)  = clamp(u / u_sat, 0.0, 1.0)

C(p)  = χ * (1−σ(u_N(p))) * (1−σ(u_S(p))) * (1−σ(u_E(p))) * (1−σ(u_W(p)))
        // product accumulated left-to-right in exactly this N,S,E,W order
        // an out-of-bounds neighbor of p contributes factor 1.0 (no saturation)

u'_i  = u_i + dt * [ fN + fS + fE + fW ]
        // summed left-to-right in exactly this N,S,E,W order, where for each
        // in-bounds neighbor j:
        //   f_j = ((C_i + C_j) * 0.5) * (u_j − u_i)
        // and for each OUT-OF-BOUNDS neighbor: f_j = 0.0  (zero-FLUX boundary)
```

**Float discipline:** no `fma`, no `mix`, no reassociation — plain `*`, `+`, `clamp`, in the
written order, identically in WGSL and the CPU oracle. This is what bit-exact parity means here.

### 2.2 The 13-point diamond (the part the digest glossed — do not "simplify" it)

`f_j` needs `C_j`, and `C_j` reads **j's own four neighbors**. The kernel therefore gathers the
**2-hop N4 diamond**: 13 cells total — `i`, its 4 neighbors, and the 8 distinct cells at
Manhattan distance 2 (`row±2`, `col±2` on-axis, and the four `row±1,col±1` diagonals). All 13
reads are from the input buffer; all five `C` values (`C_i`, `C_N`, `C_S`, `C_E`, `C_W`) are
computed in registers and discarded. **No scratch column, no second pass, no stored C field.**

> **Voiding tripwire:** replacing `(C_i + C_j)/2` with `C_i` (or any non-symmetric weight) to
> avoid the 2-hop gather destroys the antisymmetry that makes the operator conservative. A PR
> that does this is VOID regardless of how many tests it passes.

### 2.3 Boundary semantics (pinned)

Unlike the existing variants' zero-*value* boundary, `SaturatingFlux` uses **zero-flux**
boundaries: an out-of-bounds neighbor contributes no flux term, and contributes factor `1.0`
(σ = 0) inside any `C` product. The grid is a closed system; total mass is exactly conserved up
to float summation order. Do not reuse the existing boundary handling blindly — this divergence
is intentional and must be explicit in both kernel and oracle.

### 2.4 Admission bounds (hard, validated at `compile_region_field_preview`)

- Spec shape: `RegionFieldOperatorSpec::SaturatingFlux { u_sat: f32, chi: f32 }`.
- `u_sat` finite and `> 0`; `chi` finite, `0 < chi ≤ 1`.
- Stability (CFL): `dt * chi ≤ 0.25` — but `dt` is runtime; admission pins `chi ≤ 0.25 / dt_max`
  is NOT available, so v1 rule: the stencil applies the update with `dt` **fixed at 1.0** and
  `chi ≤ 0.25` enforced at admission (the effective rate is `chi`). Reject `chi > 0.25` with a
  spanned error naming the CFL bound. (`χ ≤ 1` in §1 math is therefore tightened to `≤ 0.25`.)
- `source_cap` not allowed with this operator (mirror the `Normalized` rejection).
- `source_col == target_col` (single-column field, like `Normalized`); gradient-style
  `output_col` is BH-1's business, not BH-0's.

### 2.5 File map (where every change goes — nothing else moves)

| File | Change |
|---|---|
| `crates/simthing-gpu/src/structured_field_stencil.rs` | New `StructuredFieldStencilOperator::SaturatingFlux` variant; new `VARIANT_SATURATING_FLUX` constant; `FieldStencilParamsGpu` gains `u_sat: f32` and `chi: f32` **appended at the end of the struct with matching `_pad` to keep 16-byte alignment** (it is `#[repr(C)] Pod` — the WGSL struct must be extended in the identical field order); `from_config` maps them; `validate()` enforces §2.4; the WGSL gains the variant branch implementing §2.1–2.3; `cpu_stencil_step` gains the matching arm (same evaluation order). |
| `crates/simthing-spec/src/spec/region_field.rs` | `RegionFieldOperatorSpec::SaturatingFlux { u_sat, chi }` (serde `PascalCase` like siblings). |
| `crates/simthing-spec/src/compile/region_field_admission.rs` | Compile/validation arm per §2.4; `CompiledRegionFieldOperator::SaturatingFlux { u_sat, chi }`; plumb through `compile_region_field_stencil_config`. |
| `crates/simthing-driver/src/first_slice_mapping_runtime.rs` | `compiled_stencil_to_gpu_config` match arm only. |
| Test files | New `crates/simthing-gpu/tests/` or driver test file `bh0_saturating_flux.rs` (§2.6); existing operator tests MUST remain byte-untouched. |

Struct-literal fallout: adding the spec enum variant is non-breaking (enums); adding params
fields breaks any `FieldStencilParamsGpu { .. }` literal — fix only by appending the new fields,
never by reordering.

### 2.6 Acceptance tests (all required; these names or near variants)

1. `saturating_flux_gpu_matches_cpu_oracle_bit_exact` — grids 4×4 and 8×8, horizons 1, 2, 4
   (ping-pong both parities), mixed seed patterns including boundary-adjacent cells; assert
   `to_bits()` equality on every cell, every horizon.
2. `saturating_flux_conserves_total_mass` — random-ish fixed seed pattern, 16 steps; assert
   total sum equals initial sum within `1e-4` absolute (float order), and assert **exact**
   conservation per step on the CPU oracle when summed in a fixed order.
3. `saturating_flux_clear_field_reduces_to_symmetric_diffusion` — `u_sat = 1e9` (σ≈0 ⇒ C≈χ
   everywhere): assert the update equals the hand-computed plain flux
   `u_i + χ·Σ(u_j−u_i)` bit-exactly (closed-form 3×3 single-seed case, hand-verifiable).
4. `saturating_flux_chokes_at_saturation` — a wall of cells at `u ≥ u_sat` splitting the grid:
   assert zero flux crosses the wall over N steps (the two sides' partial sums each conserve),
   and cells adjacent to the wall have `C = 0` (oracle-introspected).
5. `saturating_flux_recovers_when_saturation_clears` — drain the wall below `u_sat`
   (overwrite values), step again: flux resumes (partial sums change).
6. `saturating_flux_admission_bounds` — reject `u_sat ≤ 0`, non-finite, `chi ≤ 0`,
   `chi > 0.25`, `source_cap` present, `source_col != target_col`; accept the valid shape.
7. `saturating_flux_existing_operators_unchanged` — run the standing `Normalized` /
   `SourceCappedNormalized` / gradient parity tests untouched (no edits to those files) and
   green in the same PR.

### 2.7 Numeric mini-case (hand-checkable; put it in test 3)

3×3 grid, center seed `u = 8.0`, all else `0`, `u_sat = 1e9`, `χ = 0.25`:
`C = 0.25` everywhere (σ≈0). Each of the center's four fluxes is
`f = ((0.25+0.25)/2)·(0−8) = −2.0`, so `u'_center = 8 + (−2·4) = 0.0`; each N4 neighbor gains
exactly `+2.0`; corners stay `0`. Total mass `8.0` before and after ✓ (note this seed sits
exactly at the CFL edge — full drain in one step, no overshoot). The test must assert these
exact values (`to_bits`).

## 3. BH-1 implementation contract (opens only after BH-0 PASS)

- Optional choke readout: spec gains `choke_output_col: Option<u32>` **on the SaturatingFlux
  variant only**; when authored, the kernel additionally writes
  `1 − C(i)/χ` (∈ [0,1], 0 = clear, 1 = fully choked) to that column of the output buffer in
  the same dispatch — the `GradientXY` dual-output precedent is the model.
- Admission: `choke_output_col < n_dims`, distinct from `source_col`; the choke column is a
  **strict sink** (may not be the `source_col` of any same-frame field — reuse the
  frame-gradient-sink validation pattern verbatim).
- Consumption proof: a synthetic crowded fixture reduces the choke column through a **compact GPU sum/threshold consumer** (`SaturatingFluxChokeThresholdOp`, BH-1R). Same-frame Layer-2 admission wiring remains deferred; strict sink admission unchanged.
- Oracle: `cpu_stencil_step` writes the same choke values; parity test extends test 1.

## 4. Voiding list (any one of these voids the rung regardless of green tests)

1. Non-symmetric flux weights (the §2.2 tripwire).
2. Zero-value boundary semantics (mass drain) instead of §2.3 zero-flux.
3. A stored C field, scratch column, second dispatch, or new buffer for BH-0.
4. Approximate parity (tolerance compares) where §2.6 demands `to_bits`.
5. CPU-only implementation with the WGSL "deferred".
6. Touching existing operator branches, their tests, or their outputs.
7. Any border object, border service, segmentation pass, or per-border state.
8. Paper-fidelity language in a status row or report (§1.3).
9. `cargo test --workspace`.
10. Semantic names in WGSL (no "border", "front", "territory" — `saturating_flux`, `u_sat`,
    `chi`, `choke` are the approved vocabulary).

## 5. The ladder

| Rung | Gate | Scope | Exit criteria |
|---|---|---|---|
| **BH-0** | T2 (substrate gate) | §2 contract, complete | All seven §2.6 tests green; report + status row |
| **BH-1** | T1 | §3 contract | Choke readout column; parity extended |
| **BH-1R** | T1 | compact GPU choke threshold consumer | Compact 4-float GPU readback; CPU oracle test-only |
| **BH-1R-SCALE** | T1 | staged parallel GPU reduction | No single-lane full-grid scan; multi-workgroup + partial fold |
| **BH-2** | named consumer (`CT-4b_Local_Automata_W_Feedstock`) | GPU W composition from base W + choke columns | BH-2A contract + BH-2B kernel; no movement/pathfinding |
| **BH-2C** | deferred | PALMA feedstock proof | PALMA consumes composed W generically |
| **BH-2D** | deferred | CT-4b fixture proof | 200×200 two-field fixture; test names only |
| **BH-3** | deferred (consumer-pulled) | ClauseThing authoring surface for the operator | Opens with the first ClauseScript-authored consumer |

BH-0 was frontier-gated at seating; with §2 pinned to file-and-formula level it is
**downgraded to mechanical (Codex/Cursor-eligible) under §4 and §6 discipline** — the design
judgment is spent; what remains is faithful transcription plus the listed tests.

## 6. Orchestrator stop conditions (leave PARTIAL, do not improvise)

Stop, record `PARTIAL / BLOCKED` with the exact gap, and end the rung if: bit-exact parity
cannot be reached after aligning evaluation order (suspect fast-math/fma — report the
divergent cell and bits); the params-struct extension breaks alignment or existing parity
tests; the 13-point gather exceeds an addressing assumption in the existing kernel scaffolding;
any §4 item seems necessary to pass a test; or the spec/oracle disagree with this document.
A PARTIAL row with a precise blocker is a success state; a creative reinterpretation is not.

## 7. Guardrails (restated)

No semantic WGSL. No border objects/services/graphs — borders are field readouts.
`simthing-sim` stays map-free and BH-blind. Opt-in, default-off; spec presence enables nothing.
CPU readback is oracle/diagnostic only. PALMA coupling deferred until BH-2C. Fixtures original and
synthetic. No sqrt anywhere in this track (exact-sqrt rule untriggered).

## 8. Status ledger

| Rung | Status | Report |
|---|---|---|
| BH-0 saturating-flux operator | IMPLEMENTED / PASS | [`tests/bh0_saturating_flux_results.md`](tests/bh0_saturating_flux_results.md) |
| BH-1 choke readout | IMPLEMENTED / PASS | [`tests/bh1_choke_readout_results.md`](tests/bh1_choke_readout_results.md) |
| BH-1R compact choke threshold consumer | IMPLEMENTED / PASS | [`tests/bh1r_choke_consumption_results.md`](tests/bh1r_choke_consumption_results.md) |
| BH-1R-SCALE staged parallel GPU reduction | IMPLEMENTED / PASS | [`tests/bh1r_scale_parallel_reduction_results.md`](tests/bh1r_scale_parallel_reduction_results.md) |
| BH-2A named consumer contract | IMPLEMENTED / PASS | §9 addendum (this doc) |
| BH-2B W composition kernel | IMPLEMENTED / PASS | [`tests/bh2_w_composition_results.md`](tests/bh2_w_composition_results.md) |
| BH-2S multi-field overlap stress | IMPLEMENTED / PASS | [`tests/bh2s_overlap_stress_results.md`](tests/bh2s_overlap_stress_results.md) |
| BH-2S-API-DOC consumer service surface | DOCUMENTED / PASS | §11 (this doc) |
| BH-2C PALMA feedstock proof | DEFERRED | — |
| BH-2D CT-4b fixture proof | DEFERRED | — |
| BH-3 ClauseThing authoring | DEFERRED (consumer-pulled) | — |

**Track-forward (2026-06-11):** Named consumer `CT-4b_Local_Automata_W_Feedstock` opens BH-2.
BH-2A/B land generic GPU W composition (`WImpedanceComposeOp`) — linear weighted feedstock only;
no movement policy, pathfinding engine, route/predecessor objects, or semantic WGSL. Bridge:
`compiled_w_impedance_compose_to_gpu_config` in `w_impedance_compose_bridge.rs` (not
`first_slice_mapping_runtime.rs`). Candidate-F rule applies to every future BH/PALMA handoff.

*Opened 2026-06-12; §2–§6 expanded same day by executive design authority for unsupervised
Codex/Cursor execution after re-evaluation found the 2-hop gather and boundary semantics
unpinned — the two seams where an unsupervised pass would most plausibly have gone wrong.*

## 9. BH-2 Named Consumer Contract: `CT-4b_Local_Automata_W_Feedstock`

BH-2 is opened by named traversal consumer **`CT-4b_Local_Automata_W_Feedstock`**.

**Consumer need:** local fleet/movement automata need an admitted numeric impedance field `W`
that reflects live choke pressure without consulting raw coordinates, high-level faction border
lines, CPU segmentation, route objects, or a pathfinding service. The consumer evaluates local
vector steps by gradient descent / min-plus traversal over resident PALMA fields. **BH-2 only
supplies numeric `W` feedstock.** BH-2 does not implement movement policy.

### 9.1 Conceptual fixture (BH-2D scope — not implemented in BH-2A/B)

200×200 grid; 100 star/source points; 150 local automata (75 source family A, 75 source family B).
Fixture docs/tests may say Terran/Pirate; production substrate must not encode those semantics.

### 9.2 Two-field topology

Two independent semantic-free `SaturatingFlux` fields (`choke_a`, `choke_b` readouts). Each uses
BH-0/BH-1; no stored C field; no CPU border service.

### 9.3 BH-2 W composition (BH-2B — pinned)

Per cell, for each admitted profile `p`:

```text
output_w[p] = base_w + weight_a[p] * choke_a + weight_b[p] * choke_b
```

Allowed vocabulary: `base_w`, `choke_a`, `choke_b`, `weight_a`, `weight_b`, `output_w`,
`profile`, `impedance`, `compose`.

Forbidden production vocabulary: `Terran`, `Pirate`, `border`, `frontline`, `ambush`,
`fleet_ai`, `pathfinding`, `movement_engine`, `route`, `predecessor`.

### 9.4 Architectural boundary

| Layer | Role |
|---|---|
| `simthing-spec` | author/admit `WImpedanceComposeSpec` |
| `simthing-gpu` | `WImpedanceComposeOp` GPU-resident kernel |
| `simthing-driver` | `compiled_w_impedance_compose_to_gpu_config` bridge only |
| `first_slice_mapping_runtime.rs` | **must not** host W composition semantics |

### 9.5 Candidate-F sqrt rule

GPU-resident sqrt/magnitude/norm paths route through `m_jit_sqrt_f_exact`. BH-2 uses linear
weighted composition only.

### 9.6 BH-2 ladder

| Rung | Status | Scope |
|---|---|---|
| BH-2A | IMPLEMENTED / PASS | This addendum + status rows |
| BH-2B | IMPLEMENTED / PASS | Generic GPU W composition operator + admission |
| BH-2S | IMPLEMENTED / PASS | Generic GPU stress field algebra (overlap/mismatch/weighted/velocity) |
| BH-2C | DEFERRED | PALMA consumes composed W; D stays resident |
| BH-2D | DEFERRED | CT-4b 200×200 fixture proof |

## 10. BH-2S: Multi-Field Overlap Stress (scenario-track addendum)

**Purpose:** extend the BH-2 / CT-4b scenario track so multiple SaturatingFlux-derived choke
fields produce resident stress/motivation feedstock for FIELD_POLICY movement-front behavior
without semantic GPU code, border objects, AI planners, or CPU segmentation.

Each admitted pressure field: `field_k → BH-0 → BH-1 choke_k` where `choke_k = 1 − C_k/χ_k`.
Use `choke_k` for stress maps (not raw stored C).

**Minimal field algebra (pinned):**

```text
stress_overlap  = choke_a * choke_b
stress_mismatch = abs(choke_a - choke_b)
stress_weighted = weight_a * choke_a + weight_b * choke_b
stress_velocity = abs(choke_now - choke_prev)
```

**Resource-flow spine:** RF → pressure columns → BH-0/BH-1 → BH-2/BH-2S stress → FIELD_POLICY
threshold columns → threshold crossing → EmitEvent / BoundaryRequest. No CPU planner.

**Stowaway budget:** single-pass GPU field algebra over resident columns; admission-capped
`max_input_fields` (4) and `max_profiles` (8); CPU oracle test-only; no full-field readback.

**Architectural boundary:**

| Layer | Role |
|---|---|
| `simthing-spec` | `StressComposeSpec` + admission |
| `simthing-gpu` | `StressComposeOp` |
| `simthing-driver` | `compiled_stress_compose_to_gpu_config` bridge only |

Forbidden production vocabulary: `border`, `frontline`, `culture`, `Terran`, `Pirate`,
`ambush`, `hegemony`, `fleet_ai`, `pathfinding`, `movement_engine`, `route`, `predecessor`.

## 11. BH service surfaces exposed to consumers

These are admitted field-operation surfaces, not semantic services. **“Service surface”** means
admitted API / driver / GPU field-operation surface — not a semantic game service. The GPU must
remain semantic-free.

The runtime sees:

- field columns;
- profile weights;
- scalar composition operators;
- compact reductions;
- thresholds;
- `BoundaryRequest` / `EmitEvent`.

The runtime must **not** see: Terran, Pirate, culture, border object, frontline, ambush, fleet AI,
route, predecessor, movement engine, or pathfinding engine.

### 11.1 `SaturatingFlux` (BH-0)

| Layer | Surface |
|---|---|
| Spec | `RegionFieldOperatorSpec::SaturatingFlux { u_sat, chi, choke_output_col }` |
| GPU | `StructuredFieldStencilOperator::SaturatingFlux` in `structured_field_stencil.rs` |
| Driver | `compiled_stencil_to_gpu_config` (bridge only) |

Conservative field relaxation with symmetric `(C_i + C_j) * 0.5` flux and zero-flux boundaries.
**Emits no border object.** **Stores no C field** — C is register-transient in the 13-point diamond
gather. Mass-conserving by antisymmetric pairwise flux construction.

### 11.2 `ChokeReadout` (BH-1)

Optional `choke_output_col` on the `SaturatingFlux` variant. Same dispatch writes resident scalar
column:

```text
choke = 1 − C/χ     (0 = clear, 1 = fully choked)
```

Generic pressure/choke feedstock. Not a border, frontline, or segmentation artifact.

### 11.3 `ChokeThresholdConsumer` (BH-1R / BH-1R-SCALE)

| Layer | Surface |
|---|---|
| GPU | `SaturatingFluxChokeThresholdOp` — compact GPU reduction over resident choke column |
| Scale | BH-1R-SCALE: staged parallel reduction (256-thread pass 1 + partial fold pass 2) |

GPU-resident compact reduction. Outputs **compact aggregate values only** (e.g. four-float
readback). **CPU oracle test-only.** **No CPU-side classification.** No full-field CPU readback
for production decisions.

### 11.4 `WComposition` (BH-2B)

| Layer | Surface |
|---|---|
| Spec | `WImpedanceComposeSpec` + `compile_w_impedance_compose_preview` |
| GPU | `WImpedanceComposeOp` |
| Driver | `compiled_w_impedance_compose_to_gpu_config` |

Composes one or more choke columns into impedance `W`:

```text
output_w[p] = base_w + weight_a[p] * choke_a + weight_b[p] * choke_b
```

Generic numeric profile weights. **No faction/movement semantics in production code.**
Admission cap: **`max_profiles = 8`**. Two input choke columns (`choke_a`, `choke_b`) per
operator — columnar storage-buffer layout; no one-texture-per-resource design.

### 11.5 `OverlapStressComposition` (BH-2S)

| Layer | Surface |
|---|---|
| Spec | `StressComposeSpec` + `compile_stress_compose_preview` |
| GPU | `StressComposeOp` |
| Driver | `compiled_stress_compose_to_gpu_config` |

Exposes BH-2S stress field algebra over **already-produced resident choke columns**. Supported
forms (single-pass, semantic-free scalar ops):

```text
stress_overlap  = choke_a * choke_b
stress_mismatch = abs(choke_a - choke_b)
stress_weighted = weight_a * choke_a + weight_b * choke_b
stress_velocity = abs(choke_now - choke_prev)
```

Resident scalar fields, **not borders**. **No sqrt required** — linear, absolute, product,
min/max, clamp, sum, and threshold logic only.

Admission caps (binding budget):

- **`max_input_fields = 4`** distinct input field columns referenced per operator;
- **`max_profiles = 8`** stress profiles per operator;
- column indices and weights passed via storage-buffer / profile table — **not** one texture
  binding per resource;
- column aliasing rejected at admission.

### 11.6 `FIELD_POLICY` feedstock

Stress, W, and choke columns may feed:

- threshold rules;
- urgency columns;
- commitment gates;
- compact GPU reductions;
- movement-front local sampling (downstream consumer);
- PALMA/min-plus traversal over resident W/D fields (BH-2C — deferred).

Motivation emerges from field pressure crossing thresholds — **no AI planner.** Stress/W/choke
columns feed thresholds and events only through admitted properties / `AccumulatorOp`
registrations.

**Resource-flow spine (unchanged):**

```text
RF arena / overlays / admitted sources
→ resolved pressure columns
→ BH-0 SaturatingFlux per admitted field
→ BH-1 choke readout columns
→ BH-2 W composition and/or BH-2S stress composition
→ FIELD_POLICY threshold columns
→ threshold crossing
→ EmitEvent / BoundaryRequest
```

### 11.7 Scenario-level use-cases (examples only — not production semantics)

These use-cases are authored scenario interpretations of generic fields. **They are not GPU
runtime semantics.** The GPU sees only field columns, profile weights, scalar composition ops,
reductions, and thresholds.

| Scenario interpretation (fixture/docs only) | Generic field surface consumed |
|---|---|
| Contested fleet projection vs disruption overlap | `stress_overlap` over two choke columns |
| Patrol-gap detection | high `stress_mismatch` or low `stress_overlap` thresholds |
| Imperial overextension signal | weighted choke sum crossing commitment threshold |
| Military overreach signal | velocity + weighted stress crossing urgency gate |
| Economic-flow / security-flow mismatch | `stress_mismatch` between two admitted fields |
| Multi-system chokepoint score | compact GPU reduction over weighted stress column |
| Border-velocity / instability signal | `stress_velocity` from prev/current choke columns |
| W impedance shaping for PALMA/min-plus traversal | composed `output_w` column (BH-2B) |
| Convoy, patrol, raider, or fleet automata sampling resident W/D | local FIELD_POLICY sampling over resident columns — not a movement engine |

Fixture naming may use Terran/Pirate, culture, economy, or fleet language in docs/tests only.
Production code must see only numeric field/profile identifiers.

### 11.8 Forbidden interpretation (binding)

The following are **explicitly forbidden** as production paths on this track:

- border service;
- frontline service;
- pathfinding engine;
- movement engine;
- route object;
- predecessor table;
- CPU segmentation pass;
- CPU border classifier;
- semantic WGSL branches;
- faction-specific production code;
- full-field CPU readback for decisions.

### 11.9 Candidate-F sqrt rule (every BH/PALMA handoff)

Any GPU-resident sqrt, magnitude, distance, gradient norm, movement-front norm, threshold path,
or parity-sensitive exact path must route through:

```text
m_jit_sqrt_f_exact
```

Native WGSL `sqrt`, Rust `sqrt`, `length`, `distance`, `normalize`, `hypot`, magnitude, or norm
is **forbidden** in authoritative BH / BH-2S / PALMA-adjacent paths. Existing BH-2S
overlap/mismatch/weighted/velocity stress operators **do not require sqrt**.

### 11.10 Stowaway budget rule

BH-2S is accepted only while it remains nearly-free resident field algebra over already-produced
choke columns. If it requires full-field CPU readback, CPU border analysis, graph segmentation,
unbounded field fan-in, or extra semantic passes, it must stop as **PARTIAL**.

Allowed: single-pass per-cell GPU field algebra; fused/batched composition with W where practical;
compact GPU reduction; existing prev/current column machinery; admission-capped input fields and
profiles; CPU oracle for tests only.

Not allowed: extra BH-0 passes solely for flavor; N² all-field pairwise matrices without an
admission cap; full-field CPU readback; CPU border detection; CPU route planning; semantic WGSL;
persistent border objects; unbounded resource-field fan-in; storing raw C fields globally;
native sqrt/magnitude/norm.

### 11.11 Binding / fan-in discipline

- Prefer existing **columnar storage-buffer layout**; pass column indices and weights.
- Cap `max_input_fields` and `max_profiles` at admission (`StressComposeSpec`: **4** input
  fields, **8** profiles; `WImpedanceComposeSpec`: **8** profiles).
- Batch profiles when needed; compose in chunks if binding pressure grows.
- **Do not** add one texture binding per resource — use packed columns / admitted profile tables.
- If the harness cannot support fan-in cleanly, stop and report **PARTIAL**.
