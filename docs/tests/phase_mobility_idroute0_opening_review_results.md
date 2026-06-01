# MOBILITY-IDROUTE-0-OPEN-0 — IDROUTE opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization only — not implementer self-acceptance.

## Verdict

**OPEN** (Option A).

`MOBILITY-IDROUTE-0` may open as the next v7.9 implementation ladder, limited to the **local D=2 identity-routing overlay substrate floor + performance bars** built on the MOBILITY-ALLOC-0 deterministic slab + bulk-accounting allocator substrate and the MOBILITY-REENROLL-0 bilateral re-enrollment substrate. The authorized scope is inherently first-slice-narrowed by the track definition (flat-star cell arenas only, local aligned relations, identity-as-column not tree structure, bounded `max_factions_per_cell`, no global faction vector, no ECON/OWNER, substrate-level only, no production `SimSession` wiring). This review **authorizes** IDROUTE substrate only; it does **not** implement the IDROUTE substrate code, and it opens **no** other ladder (ECON/OWNER remain proposed/parked).

## Reviewed files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md` (targeted scan — no identity-routing / columnar overlay / masked reduction terms; no conflict)
- `docs/design_v7_8.md` (§2 operating doctrine, §6 forward territory)
- `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md` (especially §11 identity routing as D=2 masked reduction + directed disburse; §12 Gallatin/NVIDIA substrate principles; §13 IDROUTE battery)
- `docs/workshop/mapping_current_guidance.md`
- `docs/workshop/sead_self_ai_track.md`
- `docs/tests/phase_mobility_scenario0_results.md`, `phase_mobility_scenario0_acceptance_review_results.md`
- `docs/tests/phase_mobility_owner_band_budget_audit_results.md`
- `docs/tests/phase_mobility_alloc0_opening_review_results.md`, `docs/tests/phase_mobility_alloc0_results.md`
- `docs/tests/phase_mobility_reenroll0_opening_review_results.md`, `docs/tests/phase_mobility_reenroll0_results.md`
- `crates/simthing-spec/src/designer_admission/mobility_scenario0.rs`, `mobility_audit0.rs`, `mobility_alloc0.rs`, `mobility_reenroll0.rs`
- `crates/simthing-spec/tests/mobility_scenario0_admission.rs`, `mobility_audit0_owner_band_budget.rs`, `mobility_alloc0_substrate.rs`, `mobility_reenroll0_substrate.rs`
- `docs/worklog.md`

## Accepted prerequisites (verified against the tree + by running the exact required commands)

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted (MOBILITY-SCENARIO-0-ACCEPT-0) | PASS — packet forces `ScenarioAdmissionProposed`, cannot self-promote; `mobility_scenario0_admission` **13 passed** |
| MOBILITY-AUDIT-0 passed | PASS — 13 required OrderBands under ceiling 16 (slack 3); `mobility_audit0_owner_band_budget` **8 passed** |
| MOBILITY-ALLOC-0 passed and usable as substrate | PASS — `mobility_alloc0_substrate` **15 passed** (deterministic per-parent/key slab + bulk accounting; exposes blocks/live slices for downstream routing math) |
| MOBILITY-REENROLL-0 passed and usable as substrate | PASS — `mobility_reenroll0_substrate` **16 passed** (bilateral re-enrollment on ALLOC; produces updated live slices/generations per cell block; no routing semantics) |
| v7.8 M/E/T closeout preserved | PASS — `c2_atlas_admission_relaxation` 15 + `clause_spec0_frontier_v2_admission` 25 + `v7_8_met_consumer_scenarios` 10 all green; `cargo check --workspace` clean (only pre-existing driver unused-import warnings) |
| IDROUTE isolable from ECON/OWNER | PASS — Workshop §11 explicitly designs IDROUTE as *local* D=2 masked gather + per-identity columns + directed disburse on cell arenas only. Global clearinghouse (ECON) and owner overlays (OWNER) are separate later tracks using the same cell blocks as input. No shared state or wiring required. |
| Testable at substrate level without production runtime wiring | PASS — Exact precedent set by ALLOC-0 and REENROLL-0 (pure `simthing-spec` designer-admission module + exhaustive tests; no `SimSession`, no driver, no gpu kernels) |
| Identity is a column, not tree structure | PASS — Workshop §11 + scenario0 packet + REENROLL substrate all treat `faction_id` / identity as a property column on the moving SimThing; political changes are column flips, spatial moves are reparenting events on cell blocks. |
| Owner-entity never modeled as spatial parent | PASS — Explicitly rejected in scenario0 + REENROLL forbidden paths; workshop §11.4–§11.6 reinforces session descendants only. |
| Capture remains owner-column flip, not reparenting | PASS — Explicit rejection in all prior mobility substrates + workshop doctrine. |
| Movement writes only the moving SimThing’s own authoritative columns | PASS — SEAD principle + REENROLL substrate (only the mover’s slot/parent changes; no side effects on siblings). |
| No global faction vector required for IDROUTE substrate | PASS — Local D=2 per-cell only (k ≤ `max_factions_per_cell`=4 in first slice); Hybrid Strata / dense N-wide is an ECON concern. |
| No semantic/raw WGSL needed | PASS — IDROUTE is masked reduction + disburse over existing AccumulatorOp / EML / OrderBand paths (workshop §11, §12). Generic non-semantic WGSL admissible only with parity (per doctrine). |
| No CPU planner / urgency / commitment emission | PASS — Preserved from v7.8 constitution + SEAD charter; IDROUTE is pure routing math feeding existing threshold machinery. |
| `simthing-sim` remains semantic-free | PASS — All prior mobility work + invariants enforce this; IDROUTE substrate adds no map/faction/AI awareness. |
| Expected IDROUTE test battery sufficient before implementation | PASS — Workshop §13 + production track §8 already define the complete, minimal substrate floor + guardrail rejections + performance bars (see below). One design-authority addition for atomic directed disburse parity may be warranted at implementation time. |

All required commands executed and green on this branch (HEAD at origin/master + this review):

- `cargo test -p simthing-spec --test mobility_scenario0_admission` → 13 passed
- `cargo test -p simthing-spec --test mobility_audit0_owner_band_budget` → 8 passed
- `cargo test -p simthing-spec --test mobility_alloc0_substrate` → 15 passed
- `cargo test -p simthing-spec --test mobility_reenroll0_substrate` → 16 passed
- `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation` → 25 + 10 + 15 passed
- `cargo check --workspace` → clean (pre-existing driver warnings only)

## Authorized IDROUTE scope (substrate only — implementation in a later PR)

- Local D=2 identity routing on flat-star cell arenas (the only arenas modified by REENROLL).
- Identity represented as a **column** on the SimThing (read via masked gather), never as a tree parent or global vector for the substrate.
- Masked gather into per-identity parent columns within the bounded `max_factions_per_cell`.
- Deterministic per-identity masked `Sum` (exact for hard Band Alpha; approximate-deterministic for soft Band Beta).
- Deterministic multi-term routing `Sum` using fixed sorted op order (authoring_id tie-break).
- Deterministic packed-key `Max` / argmax for selective (triage) routing with unique winner by construction.
- Directed disburse from identity columns back to local children or integration columns.
- GPU-consumable, parity-testable layouts and reductions (reuses existing AO/EML/OrderBand machinery; CPU/driver accounting + proxy checksums).
- First-slice-narrowed by the accepted MOBILITY-SCENARIO-0 (k≤4, 48 cells, 34k soak).

## Explicit non-goals (enforced at designer/scenario admission; unchanged posture)

No ECON implementation; no OWNER implementation; no route/economy/owner-overlay runtime; no production `SimSession` wiring; no semantic/raw WGSL; no designer-authored shader code; no global faction vector; no owner-entity as spatial parent; no capture-as-reparenting; no nested arena reparenting; no default-on behavior; no default-on Resource Flow; no hard-currency through Resource Flow; no CPU planner; no CPU urgency computation; no CPU commitment emission; no invariant edits (none required).

## Authorized IDROUTE test battery (to implement in a later PR — none green in this opening review)

**Substrate floor**
- `idroute_masked_sum_correct`
- `idroute_multi_term_sum_determinism`
- `idroute_argmax_packed_key_unique`
- `idroute_directed_disburse_correct`
- `idroute_identity_column_not_tree_structure`
- `idroute_cpu_gpu_parity_layout`

**Guardrails (designer/scenario admission rejections)**
- `idroute_rejects_global_faction_vector`
- `idroute_rejects_owner_as_spatial_parent`
- `idroute_rejects_capture_as_reparenting`
- `idroute_rejects_econ_owner_runtime`
- `idroute_keeps_econ_owner_parked`
- `idroute_does_not_authorize_production_simsession_wiring`
- `idroute_does_not_enable_default_on_behavior`
- `idroute_rejects_semantic_or_raw_wgsl`

**Performance bars**
- `idroute_d2_masked_dispatch_scale`
- `idroute_concentration_one_cell`
- `idroute_scale_soak_34k`

## Posture attestation

MOBILITY-IDROUTE-0 is authorized **only** for local D=2 identity-routing overlay substrate in `simthing-spec` (designer-admission modeling + tests). ECON/OWNER remain proposed/parked and require separate opening reviews. No production runtime integration, no GPU kernels, no default-on flags, no semantic/raw WGSL, no `simthing-sim` semantic awareness, no CPU planner/urgency/commitment emission, no Resource Flow default-on, no hard-currency through RF, no invariant changes. v7.8 M/E/T closure (A-0/B-0/C-2), AO-WGSL-0 default-off, ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all unchanged. IDROUTE substrate, if implemented in a future PR, must follow the exact isolation and guardrail pattern established by ALLOC-0 and REENROLL-0.

Next gate (if this opening is accepted): MOBILITY-IDROUTE-0 implementation PR (substrate only, following the authorized battery above). Report of record: this file.
