# Mapping Current Guidance

Current scenario status:

- `SCENARIO-0080-2` **COMPLETE (`SCENARIO-0080-2-COMPLETE-0`, 2026-06-04, design authority)** — closed *with caveats recorded as findings*: R6C ran the assembled R1→R6B stack over one mutable 100-tick session with write-back (detector table, race curve, checksum `1bba891c779190a4`), R7 reclosed with that evidence, and the mechanism chain holds. Findings (not defects): Terran patrols never crossed the movement threshold (one-sided contest), the production/attrition race is unresolved in 100 ticks, and front/self-sustaining-loop behaviors only partially emerged — these feed a *future* `SCENARIO-0080-3`, not opened now. Run is GPU-conformant + CPU-oracle verified; R6C whole-run remains `GPU-conformant; GPU execution not yet measured` at whole-run level (per-tick shapes GPU-dispatched via RUNTIME-0080-0-R0A). R6C report: [`../tests/scenario_0080_2_r6c_integrated_run_report.md`](../tests/scenario_0080_2_r6c_integrated_run_report.md); closeout: [`../tests/scenario_0080_2_r7_closeout_report.md`](../tests/scenario_0080_2_r7_closeout_report.md).
- **`GPU-MEASURE-0080-0` IMPLEMENTED / PASS** (report: [`../tests/gpu_measure_0080_0_results.md`](../tests/gpu_measure_0080_0_results.md)). The accepted rehearsal row/mask/threshold/emission-band shapes were measured on the discrete GPU: R1, R2, R6, and R6B are `GPU-measured (integer bit-exact)`; R4 is `GPU-measured (verified-approximate, within accepted f32 bound)`. No new semantic WGSL, no new op, no invariant edit, no pinned-number change, no default SimSession wiring, and no reopen of 0080-2. Constituent shapes GPU-measured via GPU-MEASURE-0080-0.
- **`RUNTIME-0080-0-R0` R0A corrected: PARTIAL CPU-authoritative dispatch** (report: [`../tests/runtime_0080_0_r0_results.md`](../tests/runtime_0080_0_r0_results.md)). Persistent GPU session + per-tick shape dispatches preserved; CPU remains tick authority; checksum `1bba891c779190a4`. **R0A is CLOSED as PARTIAL / informative.**
- **`RUNTIME-0080-0 R1a` IMPLEMENTED / PARTIAL (SCAFFOLD) — IMPL-0 PASS overclaimed; `RUNTIME-0080-0-R1a-REMEDIAL-0` OPEN (2026-06-05, Opus):** Audit found R1a-IMPL-0 did **not** earn the claim — the CPU recomputes the full Tier-A next-state and injects it into the GPU each tick (`COL_JOURNAL_DELTA`); the GPU "tick" is three `Identity` copies + swap, so `gpu_state_feeds_next_tick=true` holds only mechanically while the CPU stays the transition authority (R0A gap in a costume), and `inter_tick_tier_a_upload_count=0` is inaccurate. **Downgraded to PARTIAL (SCAFFOLD).** Report (corrected): [`../tests/runtime_0080_0_r1a_next_tick_authority_results.md`](../tests/runtime_0080_0_r1a_next_tick_authority_results.md). **Remedial re-scopes R1a onto the production substrate** (`WorldGpuState`/`Pipelines` Pass 0–7 + `OverlayDelta`/`IntentDelta`/`ThresholdEvent`): register the already-measured R6C Tier-A transforms as `AccumulatorOp`s/overlays so the GPU computes `state_N+1`, unifying player/AI overlays + SEAD thresholds + the resident transition on one substrate. Gated by an **anti-faking oversight protocol** (independence; negative control disabling the GPU transform must fail parity; measured counters; earned parity; source-shape guard). **Opcode/WGSL-gate clarification (Opus, 2026-06-05):** the "no new WGSL/opcode" stop-line was hygiene theater (stricter than `design_0_0_8_0.md` §2.3, "new generic WGSL is a Tier-2 gate, not a prohibition"); §2.4 + an invariants companion row now make a **generic, semantic-free** EML opcode / `AccumulatorOp` combine fn / kernel a Tier-2 **gate** (handoff §4a), not a ban — semantic ops stay banned, anti-faking unchanged. Handoff: [`../handoffs/runtime_0080_0_r1a_remedial_opening.md`](../handoffs/runtime_0080_0_r1a_remedial_opening.md); spec §14/§14.1: [`../production_paths/runtime_0080_0_r1_next_tick_authority_spec.md`](../production_paths/runtime_0080_0_r1_next_tick_authority_spec.md). **Tier-B** stays bounded boundary maintenance via `ThresholdEvent → BoundaryRequest`, not a CPU planner. **R1b/R1c remain parked:** R1b (`RESIDENT-EVENTLOG-0`) → R1c (`RESIDENT-REENROLL-0`). **Parked horizon:** multi-atlas/M-4A (§11 gate), richer emergence (`SCENARIO-0080-3`), multi-faction ECON, recursion.

Authoritative decision:

- [`../adr/mapping_sparse_regioncell.md`](../adr/mapping_sparse_regioncell.md)

Constitutional surfacing:

- [`../design_v7_7.md`](../design_v7_7.md)
- [`../design_v7_6.md`](../design_v7_6.md)
- [`../invariants.md`](../invariants.md) — Mapping (Sparse RegionCell) rows

> **▶ FORWARD HORIZON (updated 2026-05-30):** **v7.7 is CLOSED** and the **AccumulatorOp v2
> production plan is CLOSED** — Phase M and Phase E are closed at their accepted bounded postures
> (Frontier substrate + SEAD Self-AI Proposal Pipeline V1; `FlatStarResourceFlow`). FrontierV2-0..4
> (multi-tick closed-loop consumer) are accepted at fixture level. **The `simthing-spec` buildout
> (L1) is now ACCEPTED (L1-ACCEPT-0, 2026-05-30):** L1-0 diagnostics + L1-1 RON preflight manifest +
> accepted FrontierV2 artifact targets are the designer/spec admission substrate. **L2 implementation
> has landed and is pending design-authority review: `CLAUSE-SPEC-0` — Designer-Facing FrontierV2
> Spec Admission** — admits a designer-authored FrontierV2 scenario through `simthing-spec` and
> compiles it metadata-only to the *same* accepted runtime artifacts, with guardrails **relocated to
> spec admission** (cross-entity writes, production commitment, Resource-Flow bypass, unbounded
> fanout, `simthing-sim` leakage rejected at import; runtime is the last line). **L2 / CLAUSE-SPEC-0
> is now ACCEPTED (Opus design authority, 2026-05-30; code-verified — [`../tests/phase_m_clause_spec0_acceptance_review_results.md`](../tests/phase_m_clause_spec0_acceptance_review_results.md)).**
> **C-2-ACCEPT-0 (2026-05-30): C-0 + C-1 + C-2 ACCEPTED — MAP BATCHING CLOSED at the designer surface.**
> Atlas proof (C-0: real packed-atlas GPU path, algebraic tile-local mask G=0, full-tile
> protocol-oracle parity `GpuVerifiedApproximate`, fingerprint `a974fe44e20620f3`) + 2000-star scale
> model (C-1) + **bounded algebraic-G=0 atlas admission relaxation (C-2)**. `request_atlas_batching`
> now admits **only** bounded algebraic-G=0, homogeneous-square, protocol-oracle-backed specs that fit
> the active `V78AtlasVramBudget` (1.5 GiB default, configurable, no hard cap) with mandatory
> multiplier reporting; physical gutter / active mask / source identity / production runtime /
> default-on all stay rejected. `MappingExecutionProfile` default stays `Disabled`. **The atlas
> production runtime / sparse-residency scheduler is a SEPARATE LATER GATE — not open. There is no
> open Line C implementation gate.** **B-0 (hard-currency ordering) is ACCEPTED (B-0-ACCEPT-0) — Line B is CLOSED at the narrow smoke level; no B-1.** **A-0 (nested RF) is ACCEPTED (A-0-ACCEPT-0, 2026-05-30) — Line A static nested Resource Flow CLOSED at the first nested slice:** static nested D=3/D=4 arena materialization, per-parent contiguous SlotRange enforcement, reserved-gap exclusion, and bit-exact GPU/CPU oracle parity over the existing AccumulatorOp OrderBand path; Resource Flow stays opt-in/default-off; hard-currency stays Phase T; **E-11B-5 dynamic enrollment stays parked behind a separate named scenario.** E-11B-5 dynamic enrollment is not a blocker to v7.8 M/E/T closure. It remains parked behind a future named product scenario (explicit nested admission only under already-enrolled parent preserving per-parent contiguous SlotRange; no Policy B/selector rerun/wildcard/gap-child promotion/slot compaction/indirection-list/default-on RF/hard-currency reroute/CPU fallback/simthing-sim awareness). **All promoted v7.8 M/E/T lines (A-0 + B-0 + C-2) are now closed for their current named scenarios; no implementation gate remains open.** **AO-WGSL-0: ACCEPTED (AO-WGSL-0-ACCEPT, 2026-05-30) — generic semantic-free AccumulatorOp WGSL performance option.** Default-off OrderBand fast path for compatible AO plans (now O(1) per-tick allocations via dynamic-offset uniform + single bind group); unsupported shapes fall back to the existing accepted path; designer-authored raw/semantic WGSL remains rejected; A-0/B-0/C-2 semantics unchanged. **L3 — ClauseThing / ClauseScript — is parked pending separate product authorization;
> do NOT start the ClauseScript parser/front-end or production `SimSession` wiring.** Do not reopen closed phases, self-accept, or spawn `ACT-N`/`FrontierV2-5`/atlas/nested
> prooflets. Closure/acceptance memos are **design-authority + product only**. **Forward workshop
> territory (ALLOC + REENROLL substrate green; downstream ladders parked):** [`mobility_and_transfer_allocation.md`](mobility_and_transfer_allocation.md)
> is the resolved architectural record for spatial mobility, faction-ownership topology, subsidiarity
> economy, and related mechanisms — the territory any future named M/E/T expansion scenario will
> draw from. All six architectural gaps resolved; `owner_band_budget_audit` is PASS for the accepted
> MOBILITY-SCENARIO-0 first slice; MOBILITY-ALLOC-0 is PASS for deterministic slab + bulk-accounting
> allocator substrate only; **MOBILITY-REENROLL-0 is PASS; MOBILITY-IDROUTE-0 is PASS + R1 hardened** for local D=2 identity-routing overlay substrate; **MOBILITY-ECON-0 is PASS** for the session-clearinghouse/subsidiarity economy clearinghouse-circulation substrate; **MOBILITY-OWNER-0 is PASS + R1 hardened** for owner-relations + latched modifier overlays, including isolated owned-record down-broadcast coverage. **0.0.7.9 mobility/transfer substrate is COMPLETE and PARKED** (MOBILITY-GPU-SUBSTRATE-DIRECTION-0 → Option A, 2026-06-02): all substrates (ALLOC/REENROLL/IDROUTE+R1/ECON/OWNER+R1) + RUNTIME-0/1A/1A-fixture/1B + the semantic-free GPU kernel substrate (GPU-EXEC-0 + RUNTIME-1B-DISPATCH-0 + KERNEL-0..6) are PASS and opt-in/default-off. **0.0.8.0 is ACTIVE** ([`../design_0_0_8_0.md`](../design_0_0_8_0.md)); **SCENARIO-FIRST** is the production track ([`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)); first gate `SCENARIO-0080-0` - **Local Patrol Economy ACCEPTED (2026-06-02); `PRODUCTION-PATH-0080-0` IMPLEMENTED / PASS** (opt-in/default-off, scoped to Local Patrol Economy on the 0.0.7.9 mobility/transfer substrate; patrol relocate decision sourced from the accepted GPU-resident SEAD threshold->event->BoundaryRequest path - SEAD + Ownership + Flow; no CPU planner, default schedule, gameplay, semantic WGSL, hard currency/markets/trade/`ai_budget`, nested Resource Flow, ClauseThing dependency, or invariant edit). **ClauseThing / ClauseScript** is a horizontal future authoring aspiration only — not active scope. Gameplay, semantic/default mobility shader, and Hybrid-Strata/faction-index remain closed/parked later gates.
> Closed track (0.0.7.9): [`../design_v7_9_mobility_transfer_allocation_production_track.md`](../design_v7_9_mobility_transfer_allocation_production_track.md) (Charter:
> [`sead_self_ai_track.md`](sead_self_ai_track.md) §11; track: [`../design_v7_8_production_track.md`](../design_v7_8_production_track.md); closure: [`../design_v7_7.md`](../design_v7_7.md).)

> `PRODUCTION-PATH-0080-0` implemented / PASS for **Local Patrol Economy**. The path is
> opt-in/default-off, scoped only to Local Patrol Economy, consumes the 0.0.7.9 mobility/transfer
> substrate, and sources patrol relocation from accepted SEAD `Threshold` + `EmitEvent` ->
> `BoundaryRequest`. No global default schedule, CPU planner, gameplay surface, semantic WGSL, hard
> currency/markets/trade/`ai_budget`, nested Resource Flow, ClauseThing dependency, or invariant edit.
> Spec: [`../production_paths/production_path_0080_0_opening_spec.md`](../production_paths/production_path_0080_0_opening_spec.md);
> report: [`../tests/phase_production_path_0080_0_impl_results.md`](../tests/phase_production_path_0080_0_impl_results.md).
> `DEFAULT-SCHEDULE-0080-0` implemented / PASS: 1A deterministic opt-in scenario-scoped schedule +
> patrol loop and 1B bounded pirate loop for Local Patrol Economy. Pirate is a second IDROUTE
> identity, not a second economy owner; relocation remains threshold/event/`BoundaryRequest` driven
> through `run_production_path_0080_0`; `local_security` evasion and deterministic cat-and-mouse are
> included. No global default schedule. Spec:
> [`../production_paths/default_schedule_0080_0_opening_spec.md`](../production_paths/default_schedule_0080_0_opening_spec.md);
> reports: [`../tests/phase_default_schedule_0080_0_impl_1a_results.md`](../tests/phase_default_schedule_0080_0_impl_1a_results.md),
> [`../tests/phase_default_schedule_0080_0_impl_1b_results.md`](../tests/phase_default_schedule_0080_0_impl_1b_results.md).
> `GAMEPLAY-0080-0` **IMPLEMENTED / PASS** as read-only Local Patrol Economy observation export
> (consumes `DefaultSchedule0080RunReport`; deterministic tick transcript/summary via
> `observe_gameplay_0080_0`). Player control/command input, UI framework, real-time loop, and global
> default schedule remain CLOSED. Impl:
> [`../tests/phase_gameplay_0080_0_impl_results.md`](../tests/phase_gameplay_0080_0_impl_results.md).
> Spec: [`../gameplay/gameplay_0080_0_opening_spec.md`](../gameplay/gameplay_0080_0_opening_spec.md).
> `GAMEPLAY-0080-1` implemented/pass as read-only Nested Starmap observation/export; consumes
> `DEFAULT-SCHEDULE-0080-1`, exports atlas residency, faction-index ECON, owner-overlay/up-aggregation,
> SEAD movement trace, and Terran/Pirate movement rows. Control/demo for `0080-1` not opened. Impl:
> [`../tests/phase_gameplay_0080_1_impl_results.md`](../tests/phase_gameplay_0080_1_impl_results.md).
> `CONTROL-0080-0` **IMPLEMENTED / PASS** as bounded Local Patrol Economy command admission
> (`admit_control_0080_0`; writes only `DefaultSchedule0080Input` bounded values/config, then
> schedule→observation path; movement remains SEAD-sourced). Direct movement/control/UI/realtime remain
> CLOSED. Impl: [`../tests/phase_control_0080_0_impl_results.md`](../tests/phase_control_0080_0_impl_results.md).
> Spec: [`../gameplay/control_0080_0_opening_spec.md`](../gameplay/control_0080_0_opening_spec.md).
> `DEMO-0080-0` **IMPLEMENTED / PASS** as headless Local Patrol Economy demo/export library helper
> (`run_demo_0080_0`; canonical control batch; control→schedule→observation/export path; day-to-day
> patrol/pirate movement record in impl report). **No CLI binary.** Direct movement/control/UI/realtime
> remain CLOSED. Impl: [`../tests/phase_demo_0080_0_impl_results.md`](../tests/phase_demo_0080_0_impl_results.md).
> Spec: [`../gameplay/demo_0080_0_opening_spec.md`](../gameplay/demo_0080_0_opening_spec.md).
> **0.0.8.0 Local Patrol Economy COMPLETE / PARKED** (scenario → production path → schedule → observation
> → control → demo/export all IMPLEMENTED / PASS); next work requires a **new named scenario or explicit
> product authorization**. Closed concerns (CLI/UI/realtime/player-control/global-schedule/semantic-WGSL/
> ClauseThing/broader-economy) remain closed. Closeout:
> [`../tests/phase_local_patrol_economy_0080_closeout_results.md`](../tests/phase_local_patrol_economy_0080_closeout_results.md).
> **Second scenario `SCENARIO-0080-1` (Nested Starmap, Terran/Pirate multi-theater) ACCEPTED**
> (2026-06-02): nested session→starmap(10×10)→10 starsystems(10×10)→planet(10×10) with owner overlays
> inheriting personality/policy weights broadcast from faction-owner simthings; ownership up-aggregation
> (planet→starsystem) as a derived overlay; pirate admitted as a **full economy faction**. Deliberately
> opens two parked substrate gates: **`ATLAS-0080-0`** implemented/pass as scenario-scoped sparse-residency
> nested mapping runtime (the named multi-theater first-slice) and **`ECON-SCALE-0080-0`** **implemented/pass**
> as bounded Terran/Pirate faction-indexed contended ECON scaling (pirate is a full economy faction; deterministic
> clearing with CPU parity oracle; subsidiarity preserved; no hard currency/nested RF/unbounded factions; default
> single-owner ECON unchanged). **`PRODUCTION-PATH-0080-1` is implemented/pass** as the opt-in Nested
> Starmap composition of `ATLAS-0080-0` + `ECON-SCALE-0080-0`; schedule/movement not opened. Opt-in/default-off; no default
> session pass-graph wiring; SEAD-sourced decisions. Packet:
> [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md);
> specs: [`../production_paths/atlas_0080_0_opening_spec.md`](../production_paths/atlas_0080_0_opening_spec.md),
> [`../production_paths/econ_scale_0080_0_opening_spec.md`](../production_paths/econ_scale_0080_0_opening_spec.md).
> ATLAS report: [`../tests/phase_atlas_0080_0_impl_results.md`](../tests/phase_atlas_0080_0_impl_results.md).
> **Codex development sequence:** see the `SCENARIO-0080-1` **PR ladder** (production track §11) — 10
> rungs, Opus authors/adjudicates every OPEN + ACCEPT gate, Codex develops IMPL rungs and stop-and-escalates
> on any stop-condition. Initial conditions pinned in the scenario packet §4.1 (6/10 stars Terran, 3 Terran
> ships + 3 pirate ships; pirate owns only its ships). `PRODUCTION-PATH-0080-1` **opened as a docs/design
> gate** for the Nested Starmap composition of `ATLAS-0080-0` + `ECON-SCALE-0080-0` (owner-overlay
> inheritance + ownership up-aggregation as numeric summaries; read-only SEAD composite-gap terms; no
> schedule/movement; no new substrate); implementation report:
> [`../tests/phase_production_path_0080_1_impl_results.md`](../tests/phase_production_path_0080_1_impl_results.md).
> `DEFAULT-SCHEDULE-0080-1` **implemented/pass** as scenario-scoped Nested Starmap SEAD-sourced
> schedule/movement (consumes `run_production_path_0080_1`; read-only composite-gap terms →
> `Threshold + EmitEvent → BoundaryRequest` → existing mobility/transfer substrate; opt-in/default-off;
> preserves identity + owner overlays and updates membership without reparenting; no observation/control/demo;
> no global default schedule; no new substrate). Spec: [`../production_paths/default_schedule_0080_1_opening_spec.md`](../production_paths/default_schedule_0080_1_opening_spec.md);
> report: [`../tests/phase_default_schedule_0080_1_impl_results.md`](../tests/phase_default_schedule_0080_1_impl_results.md).
> `GAMEPLAY-0080-1` **implemented/pass** as read-only Nested Starmap observation/export
> (`observe_gameplay_0080_1`; read-only consumer of `DefaultSchedule0081RunReport` → deterministic
> transcript: atlas residency, faction-index ECON, owner-overlay + up-aggregation summaries, SEAD movement
> trace, Terran/Pirate movement rows; opt-in/default-off; mutates nothing). Observation only.
> Spec: [`../gameplay/gameplay_0080_1_opening_spec.md`](../gameplay/gameplay_0080_1_opening_spec.md);
> report: [`../tests/phase_gameplay_0080_1_impl_results.md`](../tests/phase_gameplay_0080_1_impl_results.md).
> `CONTROL-0080-1` **implemented/pass** as bounded Nested Starmap command admission
> (`admit_control_0080_1`; opt-in deterministic vocabulary writes only existing
> `DefaultSchedule0081Input` bounded schedule values plus bounded Nested Starmap control config,
> then runs the existing `DEFAULT-SCHEDULE-0080-1` -> `GAMEPLAY-0080-1` path; commands never move
> a ship, emit an external `BoundaryRequest`, or bypass SEAD). Direct movement control, player
> command loop, UI, real-time loop, demo, and global default schedule remain CLOSED. Spec:
> [`../gameplay/control_0080_1_opening_spec.md`](../gameplay/control_0080_1_opening_spec.md);
> report: [`../tests/phase_control_0080_1_impl_results.md`](../tests/phase_control_0080_1_impl_results.md).
> `DEMO-0080-1` **IMPLEMENTED / PASS** — headless Nested Starmap demo/export library helper
> (`run_demo_0080_1`; opt-in/default-off; canonical `Control0081CommandBatch::canonical_run()`; existing
> control→schedule→observation/export path; compact demo report with atlas residency, faction-index ECON,
> owner-overlay/up-aggregation, SEAD movement trace, Terran/Pirate movement rows, command transcript, and
> replay checksum; **No CLI binary**; 24/24 tests PASS). Spec:
> [`../gameplay/demo_0080_1_opening_spec.md`](../gameplay/demo_0080_1_opening_spec.md); report:
> [`../tests/phase_demo_0080_1_impl_results.md`](../tests/phase_demo_0080_1_impl_results.md).
> **`SCENARIO-0080-1` is COMPLETE / PARKED** after `SCENARIO-0080-1-CLOSE-0` (2026-06-02) — the full
> Nested Starmap vertical slice (scenario → atlas → econ-scale → production path → schedule → observation →
> control → demo/export) is IMPLEMENTED / PASS with a 155-test chain green at close. **No further work
> opens on this slice without a new named product scenario or explicit product authorization.** Closeout:
> [`../tests/phase_scenario_0080_1_closeout_results.md`](../tests/phase_scenario_0080_1_closeout_results.md).

Active read order (authoritative path for agents):

1. [`../design_0_0_8_0.md`](../design_0_0_8_0.md) — **ACTIVE constitution (consumer-pulled phase): operating doctrine, parked inventory, next track**
2. [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md) — **0.0.8.0 SCENARIO-FIRST production track; first gate SCENARIO-0080-0 (OPEN, admission only); ClauseThing horizon-only**
3. [`phase_m_gating_and_doc_policy.md`](phase_m_gating_and_doc_policy.md) — **which lane is your change (Tier-1/Tier-2), and how much doc does it need; §6 proven-capability stop rule**
4. `docs/invariants.md` (binding constraints)
5. The compact status table below (where each slice is)
6. [`../design_v7_8_production_track.md`](../design_v7_8_production_track.md) (historical M/E/T PR ladder)
7. [`../accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md) (closed — archive pointer only)
8. `docs/workshop/eml_gadget_library_design_note.md`; `docs/workshop/m5_gradient_extraction_design_note.md`
9. The single test report for the slice you're touching

Historical/superseded artifacts live under [`../archive/`](../archive/) and `docs/workshop/archive/`. Do not treat archived files as active authority.

## Phase M — compact status (single source of truth)

`Lane`: T1 = fast-lane (one PR + one test report + one row), T2 = gated (design-review → acceptance → impl). See the gating policy.

| Slice | Lane | Status | Notes |
|---|---|---|---|
| First-slice runtime R1/R2/R3 | T2 | accepted (numeric; consumption-proof pending **R7**) | GPU-resident stencil→reduction→EML→threshold; opt-in, default-off. **Proven on a hand-seeded field at parity — not a gameplay-produced/demoed heatmap; reclassified per `invariants.md` "Scenario Proof", consumption-proof pending dress-rehearsal R7 (production track §12.5).** |
| Product fixture chain (economy + SEAD) | T2 | accepted | economy→SEAD link stays `tests/support` fixture-only |
| Boundary resolution doctrine (tick/boundary/day) | T2 | accepted | legible names preferred; no calendar/pause sim semantics |
| EML-GADGET-1 (Tier-1 stateless gadgets) | T2 | accepted | FieldSampler / WeightedAccumulator / SoftStep |
| EML-GADGET-2 (temporal: 2A–2E) | T2 | landed | explicit-column memory; bounded-feedback admission; Acceleration via explicit velocity col |
| **M-5A-gradient** (single-target Gradient op + per-direction weights) | **T1** | **landed** | [`phase_m_m5a_gradient_single_target_test_results.md`](../tests/phase_m_m5a_gradient_single_target_test_results.md) |
| **M-5B-gradient** (L3 composition RON fixture) | **T1** | **landed** | [`phase_m_m5b_gradient_l3_composition_test_results.md`](../tests/phase_m_m5b_gradient_l3_composition_test_results.md); R1 integrated evidence: [`phase_m_m5b_gradient_l3_composition_r1_test_results.md`](../tests/phase_m_m5b_gradient_l3_composition_r1_test_results.md) |
| **M-5C-gradient** (need/routing signal product fixture) | **T1** | **landed** | [`phase_m_m5c_gradient_need_signal_test_results.md`](../tests/phase_m_m5c_gradient_need_signal_test_results.md) |
| **M-5D-gradient + R1** (gradient strict-sink admission + grouped frame compile helper) | **T1** | **landed** | strict-sink validator: [`phase_m_m5d_gradient_sink_admission_test_results.md`](../tests/phase_m_m5d_gradient_sink_admission_test_results.md); grouped helper: [`phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md`](../tests/phase_m_m5d_r1_gradient_frame_compile_helper_test_results.md) |
| **M-5E-gradient** (scarcity/opportunity/logistics composite product fixture) | **T1** | **landed** | [`phase_m_m5e_gradient_scarcity_opportunity_test_results.md`](../tests/phase_m_m5e_gradient_scarcity_opportunity_test_results.md) |
| Dual-output `GradientXY` | T2 | **landed** | SCENARIO-0080-2 rung 3 (PR #451); `StructuredFieldStencilOperator::GradientXY` in `simthing-gpu`; dual-output in one dispatch, CPU-oracle parity verified, aliasing rejected at admission. |
| L1 coupling; dense per-cell temporal | T2 | deferred | separate gate each |
| **Shader/software deterministic sqrt / SEAD observer** | **T2** | **SEAD-V1 consolidated** | SEAD-OBS-1: [`phase_m_sead_obs1_overlay_score_admission_results.md`](../tests/phase_m_sead_obs1_overlay_score_admission_results.md). SEAD-OBS-2: [`phase_m_sead_obs2_multilayer_overlay_score_results.md`](../tests/phase_m_sead_obs2_multilayer_overlay_score_results.md). SEAD-OBS-3: [`phase_m_sead_obs3_fixed_point_score_results.md`](../tests/phase_m_sead_obs3_fixed_point_score_results.md). SEAD-OBS-4: [`phase_m_sead_obs4_threshold_event_results.md`](../tests/phase_m_sead_obs4_threshold_event_results.md). SEAD-EVENT-0: [`phase_m_sead_event0_compaction_results.md`](../tests/phase_m_sead_event0_compaction_results.md). SEAD-PIPE-0: [`phase_m_sead_pipe0_observer_event_pipeline_results.md`](../tests/phase_m_sead_pipe0_observer_event_pipeline_results.md). SEAD-EVENT-1: [`phase_m_sead_event1_code_bucketing_results.md`](../tests/phase_m_sead_event1_code_bucketing_results.md). SEAD-EVENT-2: [`phase_m_sead_event2_bucket_reductions_results.md`](../tests/phase_m_sead_event2_bucket_reductions_results.md). SEAD-ACT-0: [`phase_m_sead_act0_numeric_proposals_results.md`](../tests/phase_m_sead_act0_numeric_proposals_results.md). SEAD-ACT-1: [`phase_m_sead_act1_phase_e_proposal_consumer_results.md`](../tests/phase_m_sead_act1_phase_e_proposal_consumer_results.md). SEAD-ACT-2: [`phase_m_sead_act2_proposal_admission_records_results.md`](../tests/phase_m_sead_act2_proposal_admission_records_results.md). SEAD-ACT-3: [`phase_m_sead_act3_economic_fixture_records_results.md`](../tests/phase_m_sead_act3_economic_fixture_records_results.md). SEAD-ACT-4: [`phase_m_sead_act4_economic_fixture_validation_corpus_results.md`](../tests/phase_m_sead_act4_economic_fixture_validation_corpus_results.md). SEAD-V1: [`phase_m_sead_v1_consolidation_results.md`](../tests/phase_m_sead_v1_consolidation_results.md). **SEAD-V1-CONSOLIDATE-0:** SEAD self-AI fixture ladder consolidated into Proposal Pipeline V1. OBS/EVENT/PIPE/ACT evidence accepted through ACT-2 as V1 core; ACT-3/ACT-4 retained as supporting Economic V1 fixture evidence. Stop ACT-N/EVENT-N expansion. FrontierV1 is the next opt-in/default-off M/E closure scenario. Charter: [`sead_self_ai_track.md`](sead_self_ai_track.md). F artifact hash `e2e9e27601ee2e13`. **Consumption note (2026-06-03): SEAD-OBS scores an entity's own overlays — it has never consumed a heatmap to derive pathing/critical-path; the field→SEAD→action loop is consumption-proof pending dress-rehearsal R4 (production track §12.5). Candidate F is the named consumer in R4.** |
| **Frontier V1 — named closing scenario (M + E)** | **T2** | **FrontierV1-5 ACCEPTED (numeric/registration; consumption-proof pending R4/R7)** | Reports: v1-0..v1-5 + acceptance review in `docs/tests/`. Superseded post-acceptance roadmap archived under [`../archive/superseded_tests/`](../archive/superseded_tests/). Charter: [`sead_self_ai_track.md`](sead_self_ai_track.md). **2026-06-03 audit: the "SEAD route" only registers descriptors — SEAD never consumed the field to act; field→SEAD→action loop pending dress-rehearsal R4 (production track §12.5).** |
| **L1-ACCEPT-0 — simthing-spec buildout closure / L2 gate** | **T2** | **L1 ACCEPTED** | Design authority accepted L1 (L1-0 + L1-1) as sufficient designer/spec admission substrate and opened L2 / CLAUSE-SPEC-0. ClauseThing/ClauseScript parked; FrontierV2-5 rejected; one non-blocking preview.rs diagnostic-code nit carried to L2 and resolved there. Report: [`phase_m_l1_acceptance_review_results.md`](../tests/phase_m_l1_acceptance_review_results.md) |
| **CLAUSE-SPEC-0 — designer-authored FrontierV2 scenario admission** | **T2** | **ACCEPTED** | RON-first, default-off FrontierV2 scenario admission through `simthing-spec`; lowers to accepted FrontierV2 fixture artifact targets and enforces guardrails at admission. ClauseThing/ClauseScript and production runtime remain parked. Impl: [`phase_m_clause_spec0_frontier_v2_admission_results.md`](../tests/phase_m_clause_spec0_frontier_v2_admission_results.md); acceptance: [`phase_m_clause_spec0_acceptance_review_results.md`](../tests/phase_m_clause_spec0_acceptance_review_results.md). |
| **V7.8-MET-SCENARIO-0 — consumer scenario pack for M/E/T promoted lines** | **T2** | **proposed; no implementation authorization** | Uses accepted CLAUSE-SPEC layer only to name the scenarios needed for Line A nested Resource Flow, Line B hard-currency ordering, and Line C atlas mapping. No E-11B/E-11B-5, D-2/D-2a, M-4/M-4A, ClauseThing, ClauseScript, or runtime widening is authorized. Report: [`phase_m_v7_8_met_consumer_scenarios_results.md`](../tests/phase_m_v7_8_met_consumer_scenarios_results.md). |
| **SCENARIO-0080-0 — Local Patrol Economy (0.0.8.0)** | **T2** | **ACCEPTED; full vertical slice IMPLEMENTED / PASS through DEMO-0080-0** | Production path, schedule, gameplay observation, control admission, headless demo/export helper. No CLI binary; direct movement/UI/realtime closed. Demo movement record: [`../tests/phase_demo_0080_0_impl_results.md`](../tests/phase_demo_0080_0_impl_results.md). |
| **MOBILITY-SCENARIO-0 — 0.0.7.9 mobility/transfer substrate (formerly v7.9)** | **T2** | **COMPLETE + PARKED (Option A, 2026-06-02)** | Full 0.0.7.9 substrate (all ladders + semantic-free GPU kernel substrate KERNEL-0..6) proven, opt-in/default-off, parked. No open gate. 0.0.8.0 SCENARIO-FIRST track open; `SCENARIO-0080-0` Local Patrol Economy proposed. Track: [`../design_v7_9_mobility_transfer_allocation_production_track.md`](../design_v7_9_mobility_transfer_allocation_production_track.md); 0.0.8.0 track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md). |
| **L1-1 — designer admission RON preflight manifest + diagnostic preview** | **T2** | **landed** | RON-first preflight manifest + preview report exercising L1-0 guardrail diagnostics from shallow designer input; CLAUSE-SPEC-0 parked downstream; no ClauseThing/ClauseScript or production runtime. Report: [`phase_m_l1_1_designer_preflight_manifest_results.md`](../tests/phase_m_l1_1_designer_preflight_manifest_results.md) |
| **L1-0 — simthing-spec designer admission substrate preflight** | **T2** | **landed** | Shared guardrail diagnostics + artifact-target vocabulary for future CLAUSE-SPEC-0; no ClauseThing/ClauseScript, FrontierV2-5, or production runtime. Report: [`phase_m_l1_0_designer_admission_substrate_results.md`](../tests/phase_m_l1_0_designer_admission_substrate_results.md) |
| **Frontier V2 — multi-tick closed-loop consumer** | **T2** | **FrontierV2-0..4 ACCEPTED (Opus design authority, 2026-05-30; code-verified) — bounded multi-tick closed-loop consumer proof complete at fixture/test-support level; movement+structural are fixture-only shadows; no FrontierV2-5; next gate = L1 simthing-spec buildout (L2 CLAUSE-SPEC parked downstream — `sead_self_ai_track.md` §11)** | **FrontierV2-0:** first default-off multi-tick closed-loop consumer. Report: [`phase_m_frontier_v2_0_closed_loop_consumer_results.md`](../tests/phase_m_frontier_v2_0_closed_loop_consumer_results.md). **FrontierV2-1:** movement/structural FixtureCandidate evolution. Report: [`phase_m_frontier_v2_1_candidate_evolution_results.md`](../tests/phase_m_frontier_v2_1_candidate_evolution_results.md). **FrontierV2-2:** own-column movement feedback. Report: [`phase_m_frontier_v2_2_movement_feedback_application_results.md`](../tests/phase_m_frontier_v2_2_movement_feedback_application_results.md). **FrontierV2-3:** BoundaryRequest structural feedback. Report: [`phase_m_frontier_v2_3_structural_feedback_application_results.md`](../tests/phase_m_frontier_v2_3_structural_feedback_application_results.md). **FrontierV2-4:** combined movement + structural feedback loop across ticks. Builds on V2-2 and V2-3; own-column shadow and BoundaryRequest shadow both feed downstream ticks. ClauseThing remains unimplemented; no phase closure declared. Report: [`phase_m_frontier_v2_4_combined_feedback_loop_results.md`](../tests/phase_m_frontier_v2_4_combined_feedback_loop_results.md). |
| Atlas / M-4A; source-mask (`M-5`); economy→mapping bridge | T2 | deferred/prohibited | see prohibition list in gating policy |
| **M-4A Atlas Readiness Gate** | **T2** | **deferred → promoted to v7.8 track** | atlas (M-4/M-4A) now tracked in [`../design_v7_8.md`](../design_v7_8.md) Line C; isolation policy ratified, unimplemented, gated on a named multi-theater scenario + VRAM budget + §11 PR; [`phase_m_m4a_atlas_readiness_gate_results.md`](../tests/phase_m_m4a_atlas_readiness_gate_results.md) |
| **M-6A Single-Grid Active Mask Readiness Gate** | **T2** | **deferred** | missing halo contract + CPU/GPU parity; [`phase_m_m6a_single_grid_active_mask_readiness_results.md`](../tests/phase_m_m6a_single_grid_active_mask_readiness_results.md) |
| **Product Scenario Selection Gate** | **T2** | **selected → M-5E** | full-grid scarcity/opportunity/logistics composite; no new substrate; [`phase_m_product_scenario_selection_gate_results.md`](../tests/phase_m_product_scenario_selection_gate_results.md) |
| **EML-GADGET Runtime Execution Gate** | **T2** | **landed (fixture)** | per-gadget EvalEML runtime via existing AccumulatorOp; [`phase_m_eml_gadget_runtime_execution_gate_test_results.md`](../tests/phase_m_eml_gadget_runtime_execution_gate_test_results.md) |
| **M-JIT track (closed at PROD-0)** | **T2** | **CLOSED — accepted (Opus/Mapping-SEAD authority 2026-05-30, PASS WITH CONDITIONS; [`sead_self_ai_track.md`](sead_self_ai_track.md) §8)** | Default-off `ProductionKernelRegistryShell` + explicit registered exact `ProductionCandidatePreview` cohort execution; intermediate ladder reports deleted — retained evidence: [`phase_m_jit_prod0_registry_shell_test_results.md`](../tests/phase_m_jit_prod0_registry_shell_test_results.md), [`phase_m_jit_exec1_cohort_execution_fixture_test_results.md`](../tests/phase_m_jit_exec1_cohort_execution_fixture_test_results.md), [`phase_m_jit_sqrt_candidate_battery_r1_test_results.md`](../tests/phase_m_jit_sqrt_candidate_battery_r1_test_results.md), [`phase_m_jit_grad0_spatial_observer_r1_test_results.md`](../tests/phase_m_jit_grad0_spatial_observer_r1_test_results.md), [`phase_m_jit_grad1_observer_formula_fusion_test_results.md`](../tests/phase_m_jit_grad1_observer_formula_fusion_test_results.md) |

**M-JIT status:** Track **closed at M-JIT-PROD-0** (`d62b09d`) pending/after Opus acceptance. Explicit registered exact `ProductionCandidatePreview` cohort execution is available only via test-invoked `ProductionKernelRegistryShell` calls (`production_wiring=false`, default-off). **Follow-on tracks remain gated:** shader/software sqrt exact path ([`sqrt_candidates.md`](sqrt_candidates.md)); production scheduler; runtime kernel cache; default SimSession wiring; production economy→mapping bridge; atlas/active mask/source identity; dual-output `GradientXY`; native sqrt exact authority; approximate `mag2` feeding exact score inputs; semantic WGSL; ClauseThing implementation (proposal-only).

**E-phase / E11 / Resource Flow:** Reports documenting stalled or review-blocked E-phase work are intentionally retained on `master`. Restart evidence: [`e11_implementation_handoff.md`](e11_implementation_handoff.md), [`e11_readiness_review.md`](e11_readiness_review.md), [`e11_hierarchical_allocation_design.md`](e11_hierarchical_allocation_design.md). JIT doc closeout did not delete E-phase stalled evidence.

> Per-slice landing history (EML-GADGET-2A…2E, boundary/economy, etc.) now lives in the status table above and in `docs/worklog.md`. The accepted designs and binding rules are in the design notes and `docs/invariants.md`. Standing posture ("no semantic WGSL / no default wiring / `simthing-sim` map-free / defaults unchanged") is binding from `invariants.md` and asserted once per PR test report — not restated per slice here.

| **C-0 — first §11-gate M-4 atlas slice (Line C/M)** | **T2** | **landed / Pending Opus Review** | Tests algebraic tile-local mask G=0 against exact full-tile protocol CPU oracle and reports VRAM multiplier against active configurable budget (1.5 GiB default). No production mapping runtime or default-on atlas. Fingerprint `a974fe44e20620f3`. Report: [`phase_m_c0_m4_atlas_protocol_oracle_results.md`](../tests/phase_m_c0_m4_atlas_protocol_oracle_results.md) |
| **C-1 — 2000-star atlas scale model (Line C/M)** | **T2** | **Done / Pending Opus Review** | Pure model of 200×150 starmap + 2000 10×10 star grids + 10k planet-system grids + 60k surfaces (7.23 M dense cells). Algebraic G=0 fits 1.5 GiB default (~0.862 GiB); physical gutter (~5.826 GiB) requires raised active budget. No production runtime, no default-on, no M-6A/M-5. Report: [`phase_m_c1_atlas_2000_star_scale_model_results.md`](../tests/phase_m_c1_atlas_2000_star_scale_model_results.md) |
| **V7.8-CLEAN-0 — active-docs slimming / archive cleanup** | **T2** | **landed** | Archived closed/superseded design/workshop/production docs and stale evidence; preserved L0/L1 and E-phase evidence; [phase_m_v7_8_cleanup_track_prune_results.md](../tests/phase_m_v7_8_cleanup_track_prune_results.md) |

## Historical narrative (archived)

Verbose per-slice Phase M narrative blocks were moved to
[../archive/superseded_workshop/mapping_current_guidance_historical_narrative.md](../archive/superseded_workshop/mapping_current_guidance_historical_narrative.md)
by V7.8-CLEAN-0. **Do not treat archived narrative as active guidance.** Per-slice history lives in
this status table, [../design_v7_8_production_track.md](../design_v7_8_production_track.md), and
[../worklog.md](../worklog.md).

Other archives: [../archive/README.md](../archive/README.md), [rchive/](archive/).
