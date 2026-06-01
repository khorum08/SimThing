# MOBILITY-ECON-0-OPEN-0 — ECON opening review

Date: 2026-06-02
Reviewer: Design authority (Opus 4.8 lane) + product. Gate-opening / production-track authorization
only — not implementer self-acceptance.

## Verdict

**OPEN** (Option A).

`MOBILITY-ECON-0` may open as the next v7.9 implementation ladder, limited to the
**session-clearinghouse + subsidiarity economy substrate floor + performance bars** built on the
MOBILITY-ALLOC-0 / REENROLL-0 / IDROUTE-0(+R1) substrates. Authorized scope is the **clearinghouse
circulation core**: local-cell → session up-aggregation, subsidiarity balance at the clearinghouse
boundary, session → local down-disburse, hard fixed-point Band Alpha before soft float Band Beta, and
conservation-class separation — all CPU/driver substrate accounting + parity proxy.

**Scoping note (design authority):** ECON-0's authorized battery is the clearinghouse-circulation +
subsidiarity + band-separation core below. The **Hybrid Strata** faction-channel partitioning
(`econ_shared_binding_merge_correct`, `econ_channel_binding_deterministic`,
`econ_dense_frontier_stays_local`, `econ_leaf_is_fixed_width_sum`) and the **generational faction-index
slab** (`econ_faction_index_static_during_tick`) are the broader ECON scaling architecture and are
**NOT part of the ECON-0 first slice** — they remain a later ECON slice, not authorized by this gate.
The circulation mechanism ECON-0 proves is independent of column partitioning, so this layering
carries no rework risk. This review **authorizes** ECON-0 only; it does **not** implement it, and it
opens **no** other ladder.

## Reviewed files

- `docs/workshop/phase_m_gating_and_doc_policy.md`
- `docs/invariants.md` (targeted scan — no clearinghouse/subsidiarity/band conflict; no conflict)
- `docs/design_v7_8.md`, `docs/design_v7_8_production_track.md`
- `docs/design_v7_9_mobility_transfer_allocation_production_track.md`
- `docs/workshop/mobility_and_transfer_allocation.md`, `mapping_current_guidance.md`, `sead_self_ai_track.md`
- `docs/tests/phase_mobility_scenario0_results.md`, `phase_mobility_scenario0_acceptance_review_results.md`
- `docs/tests/phase_mobility_owner_band_budget_audit_results.md`
- `docs/tests/phase_mobility_alloc0_opening_review_results.md`, `phase_mobility_alloc0_results.md`
- `docs/tests/phase_mobility_reenroll0_opening_review_results.md`, `phase_mobility_reenroll0_results.md`
- `docs/tests/phase_mobility_idroute0_opening_review_results.md`, `phase_mobility_idroute0_results.md`, `phase_mobility_idroute0_r1_results.md`
- `crates/simthing-spec/src/designer_admission/mobility_{scenario0,audit0,alloc0,reenroll0,idroute0}.rs`
- `crates/simthing-spec/tests/mobility_{scenario0_admission,audit0_owner_band_budget,alloc0_substrate,reenroll0_substrate,idroute0_substrate}.rs`
- `docs/worklog.md`

## Accepted prerequisites (verified against the tree, not only the reports)

| Prerequisite | Result |
| --- | --- |
| MOBILITY-SCENARIO-0 accepted | PASS — `mobility_scenario0_admission` **13 passed**; packet declares Band Alpha/Beta classes, rejects hard/soft mix and float structural gates |
| MOBILITY-AUDIT-0 passed; OrderBand ceiling sufficient for ECON-0 | PASS — `mobility_audit0_owner_band_budget` **8 passed**; audit budget 13 under ceiling 16. ECON-0's economy-up(3)+economy-down(3)+Band Alpha(1)+Band Beta(1)+thresholds(1)=**9 OrderBands**, comfortably within ceiling |
| MOBILITY-ALLOC-0 green | PASS — `mobility_alloc0_substrate` **15 passed** |
| MOBILITY-REENROLL-0 green | PASS — `mobility_reenroll0_substrate` **16 passed** (bilateral accounting, atomic-or-reject, parity) |
| MOBILITY-IDROUTE-0 green + R1 hardened | PASS — `mobility_idroute0_substrate` **20 passed** (7 floor + 10 guardrails + 3 perf); local D=2 routing produces parity-testable per-cell layouts ECON-0 consumes as inputs |
| v7.8 M/E/T closeout preserved | PASS — `c2_atlas_admission_relaxation` 15, `clause_spec0_frontier_v2_admission` 25, `v7_8_met_consumer_scenarios` 10; `cargo check --workspace` clean (pre-existing `simthing-core`/`simthing-driver` warnings only) |
| ECON isolable from OWNER | PASS — ECON-0 is clearinghouse circulation only; owner-relation overlays + latched modifiers are OWNER, explicitly excluded |
| Testable at substrate level without production runtime wiring | PASS — ALLOC/REENROLL/IDROUTE precedent (simthing-spec designer-admission/substrate module + tests + parity proxy, no `SimSession`) |

## Authorized ECON-0 scope (substrate only)

- Session-clearinghouse economy substrate only; local cell outputs from ALLOC/REENROLL/IDROUTE are
  admissible inputs.
- Subsidiarity balance tests at the session-clearinghouse boundary (self-sufficient subtrees do not
  escalate); **balance decisions in I64 fixed-point** (no float structural gate).
- Upward aggregation from local cell/block data into clearinghouse columns; downward disburse from
  clearinghouse columns back to local integration columns.
- **Hard fixed-point Band Alpha runs before soft float Band Beta**; Beta reads finalized Alpha;
  one-directional (Alpha→Beta), never the same pass; explicit conservation-class separation.
- Deterministic ordering for multi-cell aggregation and disburse (no arrival-order significance).
- GPU-consumable, parity-testable layouts/reductions; CPU/driver accounting + parity proxy following
  the ALLOC/REENROLL/IDROUTE pattern.

## Explicit non-goals (enforced at designer/scenario admission)

No OWNER; no owner-relation overlays; no latched modifier overlay runtime; **no Hybrid-Strata
channel-binding or generational faction-index in ECON-0 (later ECON slice)**; no production gameplay
integration; no production `SimSession` wiring; no semantic/raw WGSL; no designer-authored shader; no
global/CPU AI planner; no CPU urgency; no CPU commitment emission; no default-on behavior; no
default-on Resource Flow; no hard-currency through Resource Flow; no capture-as-reparenting; no
owner-entity as spatial parent; no nested arena reparenting; no invariant edits.

## Opening checks

| Check | Result |
| --- | --- |
| SCENARIO accepted | PASS |
| AUDIT passed; OrderBand ceiling sufficient for ECON-0 first slice | PASS (ECON-0 ≈ 9 OrderBands ≤ 16) |
| ALLOC substrate green | PASS |
| REENROLL substrate green | PASS |
| IDROUTE substrate green + R1 hardened | PASS |
| ECON isolable from OWNER | PASS |
| ECON testable at substrate level without runtime wiring | PASS |
| Hard Band Alpha / soft Band Beta separated | PASS (scenario0 class lists; ECON-0 floor `econ_hard_band_alpha_before_soft_band_beta`) |
| Hard values never route through Resource Flow as soft | PASS (guardrail `econ_rejects_hard_currency_through_resource_flow`) |
| No hard/soft silent mixing | PASS (floor `econ_rejects_hard_soft_silent_mix`) |
| No float value gates structural transition | PASS (guardrail `econ_rejects_float_structural_gate`; fixed-point balance) |
| No owner overlay / latched modifier required for ECON-0 | PASS (those are OWNER) |
| No semantic/raw WGSL needed | PASS |
| No CPU planner / urgency / commitment emission | PASS |
| `simthing-sim` remains semantic-free | PASS |
| Expected ECON test battery sufficient before implementation | PASS — see below |

## Authorized ECON-0 test battery (to implement in a later PR — none green yet)

**Substrate floor**

- `econ_session_clearinghouse_aggregates_local_cells`
- `econ_subsidiarity_balance_conservation`
- `econ_hard_band_alpha_before_soft_band_beta`
- `econ_rejects_hard_soft_silent_mix`
- `econ_deterministic_up_down_disburse`
- `econ_cpu_gpu_parity_layout`

**Guardrails**

- `econ_rejects_owner_overlay_runtime`
- `econ_keeps_owner_parked`
- `econ_rejects_default_on_resource_flow`
- `econ_rejects_hard_currency_through_resource_flow`
- `econ_rejects_float_structural_gate`
- `econ_rejects_production_simsession_wiring`
- `econ_rejects_semantic_or_raw_wgsl`
- `econ_rejects_cpu_planner_urgency_commitment`
- `econ_rejects_owner_as_spatial_parent`
- `econ_rejects_capture_as_reparenting`

**Performance bars**

- `econ_multi_cell_clearinghouse_scale`
- `econ_concentration_one_session`
- `econ_scale_soak_34k`

This battery is **authorized, not implemented**; no ECON test is marked green in this PR. The broader
ECON architecture tests (Hybrid Strata channel binding, generational faction-index slab,
dense-frontier locality, fixed-width leaf sum) are **not** part of ECON-0 and remain a later slice.

## Commands

```bash
cargo test -p simthing-spec --test mobility_scenario0_admission                # 13 passed
cargo test -p simthing-spec --test mobility_audit0_owner_band_budget           # 8 passed
cargo test -p simthing-spec --test mobility_alloc0_substrate                   # 15 passed
cargo test -p simthing-spec --test mobility_reenroll0_substrate                # 16 passed
cargo test -p simthing-spec --test mobility_idroute0_substrate                 # 20 passed
cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission --test v7_8_met_consumer_scenarios --test c2_atlas_admission_relaxation  # 25 + 10 + 15 passed
cargo check --workspace                                                        # Finished (pre-existing simthing-core/simthing-driver warnings only)
```

## Posture attestation

Opening review only — no ECON implementation, no clearinghouse/economy code, no GPU kernels, no
production `SimSession` wiring, no default-on flags, no semantic/raw WGSL, no `simthing-sim` semantic
awareness, no CPU planner/urgency/commitment emission, no Resource Flow default-on, no hard-currency
through Resource Flow, no invariant changes. Owner-entities remain non-spatial; capture remains an
owner-column flip; hard Band Alpha stays separated from soft Band Beta; balance stays fixed-point;
SEAD decisions stay GPU-resident threshold/event outputs. v7.8 M/E/T closure (A-0/B-0/C-2), AO-WGSL-0
default-off, ClauseThing/L3 parked, FrontierV2-5 rejected, ACT/EVENT/OBS/PIPE no reopen — all
unchanged. OWNER remains proposed/parked.

## Next gate

**`MOBILITY-ECON-0`** — implement the authorized clearinghouse-circulation substrate floor +
performance bars above (later PR, substrate-only). The Hybrid-Strata / faction-index ECON scaling
layer and **OWNER** remain proposed/parked.
