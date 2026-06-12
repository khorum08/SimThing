# SimThing 0.0.8.1 — Border Hack Track (`BH-`): the C_u saturating-flux stencil operator

> **Status: OPEN (2026-06-12) — seated as a generic GPU utility, PALMA-style.** Product
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
| **BH-1** | T1 | §3 contract | Choke readout column + GPU compact reduce/threshold consumer (BH-1R); parity extended |
| **BH-2** | deferred (named-consumer gate) | `1−C/χ` choke column as PALMA min-plus impedance feedstock `W` (gradient-valley coupling) | Opens when a movement consumer names it; D stays a field |
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
CPU readback is oracle/diagnostic only. PALMA untouched until BH-2 opens. Fixtures original and
synthetic. No sqrt anywhere in this track (exact-sqrt rule untriggered).

## 8. Status ledger

| Rung | Status | Report |
|---|---|---|
| BH-0 saturating-flux operator | IMPLEMENTED / PASS | [`tests/bh0_saturating_flux_results.md`](tests/bh0_saturating_flux_results.md) |
| BH-1 choke readout | IMPLEMENTED / PASS | [`tests/bh1_choke_readout_results.md`](tests/bh1_choke_readout_results.md) |
| BH-1R choke GPU consumption | IMPLEMENTED / PASS | [`tests/bh1r_choke_consumption_results.md`](tests/bh1r_choke_consumption_results.md) |
| BH-2 PALMA impedance coupling | DEFERRED (named-consumer gate) | — |
| BH-3 ClauseThing authoring | DEFERRED (consumer-pulled) | — |

*Opened 2026-06-12; §2–§6 expanded same day by executive design authority for unsupervised
Codex/Cursor execution after re-evaluation found the 2-hop gather and boundary semantics
unpinned — the two seams where an unsupervised pass would most plausibly have gone wrong.*
