# Phase M Product Scenario Selection Gate

**Date:** 2026-05-29  
**Base HEAD:** `c3728419a9af22ac2427c68f79c45b4d813db39c`  
**Final commit SHA:** *(pending merge — see PR)*  
**Lane classification:** Tier-2 product/readiness gate (V7.7 §5)  
**Decision:** **Candidate D — Existing M-5-gradient product fixture expansion**  
**Verdict:** **PASS — mapping scenario selected; no new substrate authorized**

---

## Files Inspected

| Surface | Finding |
|---|---|
| `docs/workshop/mapping_current_guidance.md` | M-5A/B/C/D+R1 landed; M-4A/M-6A deferred; no named atlas/sparse/source scenarios |
| `docs/accumulator_op_v2_production_plan.md` | Atlas not next; EML-GADGET-2 spec landed; runtime execution separately gated; product fixture chain accepted |
| `docs/tests/phase_m_m4a_atlas_readiness_gate_results.md` | DEFER — no multi-theater scenario, no VRAM budget |
| `docs/tests/phase_m_m6a_single_grid_active_mask_readiness_results.md` | DEFER — no sparse-grid scenario; NoHalo blocked; missing halo/parity |
| `docs/workshop/m5_gradient_extraction_design_note.md` | M-5 complete through M-5D; lateral use cases (scarcity, opportunity, logistics) documented; dual-output deferred |
| `docs/invariants.md` | No semantic WGSL; active mask requires halo; strict-sink binding; economy→mapping fixture-only |
| `docs/design_v7_7.md` | Mapping ADR provisional atlas; Phase M natives gated |
| `docs/workshop/phase_m_gating_and_doc_policy.md` | Tier-2 gate; compact doc updates only |

---

## Product-Scenario Candidates Considered

### Candidate A — Multi-theater atlas

**Example:** multiple independent region grids need VRAM/dispatch batching.

**Status:** **Not selected.** M-4A gate deferred for lack of named multi-theater scenario and approved VRAM budget. High GPU layout risk (tile boundaries, ping-pong, protocol oracle). No product has named this need since first-slice acceptance.

### Candidate B — Sparse-grid / irregular-geography active mask

**Example:** one grid with inactive cells (void, blocked terrain, sparse playable area).

**Status:** **Not selected.** M-6A gate deferred — no named sparse-grid scenario; `ActiveOnlyExperimentalNoHalo` constitutionally blocked; halo/frontier contract and CPU/GPU parity missing. Medium-high correctness risk at mask boundaries.

### Candidate C — Source identity / source-mask

**Example:** distinguish seed sources or behavioral diffusion sources.

**Status:** **Not selected.** Separate M-5 source-identity track; no named scenario; highest semantic-leakage risk (behavioral source policy, source identity buffer); overlaps with prohibited default-on wiring paths.

### Candidate D — Existing M-5-gradient product fixture expansion

**Example:** scarcity, opportunity, pressure, logistics routing signals on full-grid substrate.

**Status:** **SELECTED.** M-5A/B/C/D substrate is landed and green. M-5C proved need/routing pattern; design note §0 lists additional lateral compositions (price differential, labor opportunity, supply-reach/logistics) not yet individually fixture-proven. Tier-1 fast lane; zero new GPU/WGSL; fixture + CPU-oracle only.

### Candidate E — Return to non-mapping production-plan item

**Example:** EML-GADGET runtime execution gate; economy+SEAD non-mapping fixture; Resource Flow pause.

**Status:** **Not selected as primary.** Valid parallel track (product fixture chain acceptance recommends another tiny non-map-substrate + SEAD fixture; EML runtime execution needs separate Opus-gated handoff). However, mapping product need for gradient composition fixtures is concrete and lower risk than resuming substrate readiness loops for A/B/C.

---

## Risk Comparison

| Candidate | Product motivation | GPU correctness risk | Semantic leakage risk | Default-on wiring risk | Substrate change |
|---|---|---|---|---|---|
| A — Atlas | Low (unnamed) | **High** | Low (if generic) | Medium | Yes — packer + WGSL |
| B — Active mask | Low (unnamed) | **High** (NoHalo) | Low (if generic) | Low | Yes — halo + admission |
| C — Source-mask | Low (unnamed) | Medium | **High** | **High** | Yes — source identity |
| **D — M-5 fixture** | **Medium-High** (design note §0) | **None** (existing parity) | **None** (spec-layer meaning) | **None** | **No** |
| E — Non-mapping | Medium (plan authorized) | Varies by item | Low | Low | No mapping substrate |

### Evaluation questions

| Question | Answer |
|---|---|
| Strongest concrete product motivation among deferred substrate gates? | **None** — A/B/C all lack named scenarios |
| Strongest motivation among all candidates? | **D** — design note documents scarcity/opportunity/logistics compositions; M-5C established pattern |
| Least GPU correctness risk? | **D** — reuses M-5A parity suite; no new kernel paths |
| Useful product fixture without new substrate? | **D** — Tier-1 RON + integrated CPU-oracle test |
| Semantic leakage / default-on wiring risk? | **Highest for C**; **lowest for D** |
| Already-authorized non-mapping item to resume instead? | EML runtime execution gate; economy+SEAD fixture V2 — valid but does not address mapping product ladder |

---

## Decision

**Select Candidate D — Existing M-5-gradient product fixture expansion.**

### Named product scenario

**Full-grid scarcity/opportunity/logistics pressure composite** — a designer-authored multi-field RegionField frame using landed M-5 gradient substrate to compose routing signals from:

- scarcity / unmet-demand scalar fields (extends M-5C pattern)
- price or labor-opportunity gradients (Gradient X/Y on authored columns)
- optional supply-reach or logistics-cost gradient contribution
- L3 EMA + WeightedAccumulator composite over parent reductions
- strict-sink frame validation via `compile_region_field_frame_preview`

This scenario uses **full grids only** — no inactive cells, no multi-theater batching, no source identity. Meaning stays at spec/RON layer; shader sees floats only.

### Why M-4A remains deferred

No named multi-theater product scenario. No approved VRAM budget. Atlas packer remains high GPU-layout risk. First-slice posture is single 10×10 grid. **Not justified** by the selected scenario.

### Why M-6A remains deferred

Selected scenario explicitly uses **full-grid** execution. Irregular geography / sparse playable area is **not** in scope. Active mask remains blocked pending halo contract + parity. **Not justified** by the selected scenario.

### Why source-mask remains deferred

Selected scenario uses `CallerManagedOneShotSeedThenZero` and existing diffusion operators. No behavioral source policy or source identity buffer required. **Separate gate** if product names source-differentiation need later.

### Distinction summary

| Track | Selected scenario uses it? |
|---|---|
| M-5-gradient substrate (Gradient, L3, strict-sink) | **Yes** — core |
| Atlas/M-4A | No — deferred |
| Active mask/M-6A | No — deferred |
| Source-mask/M-5 source-identity | No — deferred |
| New WGSL / semantic shader | No — prohibited |

---

## Next Implementation Handoff

**Title:** `Phase M-5E-gradient — Scarcity/Opportunity Composite Product Fixture`

**Lane:** Tier-1 (fixture + CPU-oracle; no substrate change)

**Boundaries:**

- RON fixture + integrated CPU-oracle test over M-5A/B/C/D landed substrate
- Multi-field frame: scalar + Gradient X/Y fields → SlotRange Sum → L3 EMA + WeightedAccumulator composite
- Use `compile_region_field_frame_preview` for strict-sink validation
- Full-grid only; no atlas, no active mask, no source-mask
- No semantic WGSL; no default mapping wiring; no `simthing-sim` changes
- No production economy→mapping bridge; no ResourceEconomySpec→mapping coupling
- Optional GPU-resident threshold commitment in fixture only (M-5B/C precedent)

**Stop conditions:**

- Requires new WGSL, atlas, active-mask admission, or source-mask → stop
- Requires weakening M-5D strict-sink validation → stop
- Requires default SimSession mapping wiring → stop
- Requires production economy→mapping bridge → stop

**Parallel non-mapping track (not authorized by this gate):**

- EML-GADGET runtime execution gate — remains separately Opus-gated
- Economy + SEAD non-mapping product fixture V2 — authorized by product fixture chain acceptance but out of scope for this mapping scenario selection

---

## Required Scans

**Scan 1:**
```bash
rg "M-4A|atlas|active mask|ActiveOnly|source_mask|source identity|M-5-gradient|M-6A|product scenario|named scenario|VRAM budget" docs/workshop docs/accumulator_op_v2_production_plan.md docs/tests
```
**Result:** M-4A and M-6A readiness reports show DEFER without named scenarios; M-5-gradient A/B/C/D/R1 landed with test reports; atlas/active-mask/source-mask references are guardrail/deferred context; production plan lists atlas not next and ActiveOnly rejection at admission.

**Scan 2:**
```bash
rg "semantic WGSL|default SimSession mapping|production economy→mapping bridge|CPU urgency|CPU-side AI planner|simthing-sim|GradientXY|sqrt|L1 cross-field|dense per-cell" docs/workshop docs/accumulator_op_v2_production_plan.md docs/invariants.md
```
**Result:** Guardrail/deferred context only; no violations.

---

## Sanity Tests

```bash
cargo check --workspace
```
**Result:** **PASS**

No code changes beyond docs/report in this pass.

---

## Transient Log Cleanup

```bash
find docs/tests -maxdepth 1 -type f \( -name "*.log" -o -name "*tmp*" -o -name "*scratch*" \) -print
```
**Result:** 11 intentional historical `*_full.log` files; **no scratch/tmp artifacts deleted.**

---

## Posture Attestation

No implementation in this pass. No semantic WGSL, no new WGSL, no atlas/M-4A, no active-mask admission, no source-mask/source-identity, no default mapping wiring, no production economy→mapping bridge, no `simthing-sim` changes, no L1 coupling, no sqrt/new opcode; M-5D strict-sink validation unchanged; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.

---

**PASS** — Phase M Product Scenario Selection Gate completed; the next mapping work is justified by the named full-grid scarcity/opportunity/logistics composite scenario (Candidate D), routing to M-5E-gradient fixture on existing substrate; M-4A, M-6A, and source-mask remain deferred; active production guidance updated; no implementation performed; V7.7 / Mapping ADR / SEAD GPU-resident default-off posture intact.
