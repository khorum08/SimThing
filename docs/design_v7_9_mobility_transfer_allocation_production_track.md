# SimThing — Design v7.9 Mobility / Transfer Allocation Production Track

> **Status:** MOBILITY-SCENARIO-0 ACCEPTED; MOBILITY-AUDIT-0 PASS; MOBILITY-ALLOC-0 + REENROLL-0 PASS (substrate); MOBILITY-IDROUTE-0 PASS + R1 hardened; MOBILITY-ECON-0 PASS (substrate); MOBILITY-OWNER-0 PASS + R1 hardened (substrate). **The v7.9 mobility/transfer substrate ladder is complete at substrate level.** **MOBILITY-RUNTIME-0 PASS (test-only, default-off substrate-composition harness).** **MOBILITY-RUNTIME-1A PASS (CPU-only, default-off `simthing-spec` production-fixture model; no real runtime crate or GPU pass graph).** Actual production runtime crate fixture wiring (**MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE**) and RUNTIME-1B GPU pass-graph registration remain separate, currently-closed later gates. The Hybrid-Strata/faction-index ECON scaling layer remains a later ECON slice.
> **Purpose:** Sequence the next named-scenario territory after v7.8 M/E/T closeout: spatial mobility, reparenting-triggered arena re-enrollment, deterministic slab/bulk allocation, identity-routing overlays, session clearinghouse economy, and owner-relation overlays.
> **Authority:** This track consumes `docs/design_v7_8.md` §6 and `docs/workshop/mobility_and_transfer_allocation.md`. It does not supersede `docs/design_v7_8.md`, `docs/invariants.md`, or the v7.8 closeout.
> **Posture:** Parked until scenario acceptance. No implementation by default.

---

## 1. Current baseline

The project is at a clean post-v7.8 closeout state.

| Line                                    | State                                                                                                         |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| A / E — Nested Resource Flow            | A-0 ACCEPTED. Static nested Resource Flow closed at first slice. E-11B-5 remains parked.                      |
| B / T — Discrete hard-currency ordering | B-0 ACCEPTED. Closed at narrow smoke level.                                                                   |
| C / M — Atlas / multi-theater mapping   | C-0/C-1/C-2 ACCEPTED. Map batching closed at designer surface. Production atlas runtime remains a later gate. |
| AO-WGSL-0                               | ACCEPTED. Generic semantic-free AccumulatorOp WGSL performance option, default-off.                           |
| ClauseThing / L3                        | Parked.                                                                                                       |
| FrontierV2-5 / ACT / EVENT / OBS / PIPE | Rejected / no ladder reopen.                                                                                  |

MOBILITY-ALLOC-0, MOBILITY-REENROLL-0, MOBILITY-IDROUTE-0 (+R1), MOBILITY-ECON-0, and MOBILITY-OWNER-0 (+R1) are green at substrate level (deterministic slab + bulk-accounting allocator; bilateral re-enrollment; local D=2 identity routing; session-clearinghouse + subsidiarity economy clearinghouse-circulation first slice; owner-relations + latched modifier overlays, including isolated owned-record down-broadcast coverage). The v7.9 mobility/transfer substrate ladder is complete at substrate level. Production runtime integration remains a separate, currently-closed gate; no downstream implementation gate is open unless explicitly authorized by this or a future opening review.

---

## 2. Track doctrine

This track is **performance-led, scenario-gated, and designer-admission guarded**.

Implementation PRs should target concrete throughput and scale bars, not re-prove every invariant repeatedly. The implementation-owned substrate floor is limited to:

1. deterministic replay / I8 parity;
2. no live-slot compaction;
3. no owner-entity as a spatial parent.

All richer correctness rules should be enforced at designer/scenario admission as import-time rejections:

* capture is a column flip, not reparenting;
* owner-entities are session descendants, never spatial parents;
* semantic/raw WGSL from designer/spec admission remains rejected;
* `max_factions_per_cell`, block sizes, conservation class, routing policy, and owner-relation layering are admitted or rejected at spec import;
* hard and soft quantities must not silently mix in one pass;
* modifier overlays are blockade-immune and latched;
* blockable resource flows are per-tick and cut by blockade.

Strong guardrails live at the designer/scenario-facing layer.

---

## 3. Scenario gate

### MOBILITY-SCENARIO-0 — Named scenario and admission packet

**Status:** **ACCEPTED (MOBILITY-SCENARIO-0-ACCEPT-0, design authority + product, 2026-06-01).**
Scenario/admission only — no implementation authorized. The accepted packet is intrinsically
first-slice-narrowed: routing is `NarrowedAdversarialFirstSlice`, spatial depth 4, `max_factions_per_cell`
4, 48 cells, 34k soak. Acceptance opened only `MOBILITY-AUDIT-0 / owner_band_budget_audit`;
that audit now passes without narrowing. See
[`phase_mobility_scenario0_acceptance_review_results.md`](tests/phase_mobility_scenario0_acceptance_review_results.md).

**Purpose:** Define the product scenario that justifies opening the mobility/transfer allocation track.

The scenario must declare:

| Parameter                        | Required decision                                                                |
| -------------------------------- | -------------------------------------------------------------------------------- |
| Theater shape                    | Cells/systems/sectors involved in first scenario slice.                          |
| `max_factions_per_cell`          | Leaf identity-channel count and local routing EML node budget.                   |
| `max_fleet_density` / block size | Slot block reservation sizing for moving entities.                               |
| Entity identity boundary         | Which units are SimThing slots vs. aggregate `count` columns.                    |
| Owner columns                    | Which owner-relations exist: faction, species, blueprint, etc.                   |
| Resource quantity classes        | Hard fixed-point vs. soft float quantities.                                      |
| Supply/economy scope             | Whether sector/cell edges are Resource Flow couplings or only spatial structure. |
| Blockade semantics               | Which flows are cut; which overlays remain blockade-immune.                      |
| Routing mode                     | Adversarial, cooperative, directed, argmax/triage, or proportional.              |
| 34k soak profile                 | Entity count, churn rate, movement rate, capture/unlock cadence.                 |

**Admission outputs:**

* accepted scenario metadata;
* rejected scenario cases;
* test/battery selection;
* track entry/exit criteria;
* no production runtime change.

**Report:**

```text
docs/tests/phase_mobility_scenario0_results.md
```

**Required commands:**

```bash
cargo check --workspace
```

**Stop conditions:**

* scenario requires owner-entities as spatial parents;
* scenario models capture as reparenting;
* scenario requires indirection buffer before gap/slab model is attempted;
* scenario requires GPU-side semaphore / nondeterministic allocator;
* scenario requires semantic/raw WGSL from designer input;
* scenario opens ClauseThing/L3 implicitly;
* scenario reopens A/B/C or ACT/EVENT/OBS/PIPE.

---

## 4. Ladder index

| Ladder   | Capability                                     | Entry gate                                        | Status   | Advance condition                                 |
| -------- | ---------------------------------------------- | ------------------------------------------------- | -------- | ------------------------------------------------- |
| SCENARIO | Product scenario / admission packet            | Product/design authority                          | **ACCEPTED (MOBILITY-SCENARIO-0-ACCEPT-0, 2026-06-01)** | — (accepted) |
| AUDIT    | Owner/OrderBand depth budget                   | Scenario accepted                                 | **PASS (MOBILITY-AUDIT-0, 2026-06-01)** | Complete; first slice fits current ceiling |
| ALLOC    | Deterministic slab + bulk-accounting allocator | Scenario accepted; A-0 baseline                   | **PASS (MOBILITY-ALLOC-0, substrate only)** | Complete; substrate floor + performance bars green |
| REENROLL | Reparenting / bilateral arena re-enrollment    | ALLOC green                                       | **PASS (MOBILITY-REENROLL-0, substrate only)** | Complete; substrate floor + performance bars green |
| IDROUTE  | D=2 identity-routing overlay                   | ALLOC + REENROLL green                            | **PASS + R1 hardened (MOBILITY-IDROUTE-0-R1, 2026-06-02)** — local D=2 substrate floor + explicit battery green | ECON/OWNER remain parked  |
| ECON     | Session clearinghouse + subsidiarity economy   | ALLOC + REENROLL green; owner-band audit complete | **PASS (MOBILITY-ECON-0, substrate only)** | Complete; substrate floor + performance bars green |
| OWNER    | Owner-relations + latched modifier overlays    | ECON green                                        | **PASS + R1 hardened (MOBILITY-OWNER-0-R1, substrate only)** | Complete; substrate floor + performance bars green; isolated down-broadcast coverage explicit |
| RUNTIME  | Production runtime integration (post-substrate) | All substrates green                              | **PASS (MOBILITY-RUNTIME-0, test-only composition harness)** | Composition harness green |
| RUNTIME-1A | CPU-only `simthing-spec` production-fixture model (composition → fixture surface model) | RUNTIME-0 green | **PASS (MOBILITY-RUNTIME-1A, CPU-only default-off `simthing-spec` fixture model)** | Complete; floor + soak bars green; no runtime crate wiring |
| RUNTIME-1A-RUNTIME-FIXTURE | Actual production runtime crate `SimSession` fixture wiring | RUNTIME-1A green | **Closed (separate later gate)** | — |
| RUNTIME-1B | GPU pass-graph registration (opt-in, non-default) | RUNTIME-1A green | **Closed (separate later gate)** | — |

---

## 5. AUDIT — owner_band_budget_audit

**Status:** **PASS (MOBILITY-AUDIT-0, 2026-06-01).** No runtime implementation.

**Purpose:** Determine whether the interleaved circulations fit within `max_orderband_depth` at target spatial depth before ECON/OWNER implementation.

Audit these circulation families:

* modifier-down;
* economy-up;
* economy-down;
* research-up;
* thresholds;
* hard fixed-point Band Alpha;
* soft float Band Beta.

**Must prove:**

* target spatial depth fits the current OrderBand ceiling; or
* track must narrow scenario depth; or
* separate OrderBand-depth expansion scenario is required.

**Result:** Accepted scenario constants require 13 OrderBands under the audit model:
modifier-down (1), hard Band Alpha (1), economy-up (3), economy-down (3), research-up (3),
thresholds (1), and soft Band Beta (1). Current `max_orderband_depth` is 16, leaving slack 3.
Verdict: **PASS**. No narrowing or OrderBand-depth expansion scenario is required. This audit did not
open ALLOC or any runtime implementation gate; MOBILITY-ALLOC-0-OPEN-0 later opened only the
deterministic slab + bulk-accounting substrate gate, and MOBILITY-ALLOC-0 is now green at that layer.

**Report:**

```text
docs/tests/phase_mobility_owner_band_budget_audit_results.md
```

**Required commands:**

```bash
cargo check --workspace
```

---

## 6. ALLOC — deterministic slab + bulk-accounting allocator

**Status:** **PASS (MOBILITY-ALLOC-0, 2026-06-01).** Deterministic slab + bulk-accounting allocator substrate implemented and tested. No REENROLL, IDROUTE, ECON, OWNER, production `SimSession` wiring, default-on behavior, semantic/raw WGSL, GPU semaphore, or runtime gameplay integration is authorized.

**Entry gate:** MOBILITY-SCENARIO-0 accepted.

**Purpose:** Replace global LIFO slot reuse with deterministic per-parent/key slab allocation and two-stage bulk accounting.

**Scope:**

* parent/key owns a pre-formatted contiguous block;
* arrivals claim slices inside reserved headroom;
* whole-block reclaim on parent/key removal;
* no live-slice compaction;
* lowest-free-first deterministic allocation;
* net births/deaths handled in one boundary accounting pass;
* CPU/driver accounting only.

**Explicit non-goals:**

* no GPU-side semaphore;
* no CUDA-style atomics;
* no nondeterministic allocator;
* no live compaction;
* no indirection-list SlotRange;
* no semantic WGSL;
* no owner/economy semantics.

### ALLOC substrate floor

| Test                                | Must prove                                                                                         |
| ----------------------------------- | -------------------------------------------------------------------------------------------------- |
| `alloc_no_live_slice_moves`         | No live slice changes slot address mid-session.                                                    |
| `alloc_bulk_accounting_determinism` | Same boundary event multiset produces identical slot assignment regardless of event arrival order. |
| `alloc_cpu_gpu_parity`              | Post-allocation arena layout produces bit-exact GPU/CPU oracle results.                            |

### ALLOC performance bars

| Test                                 | Must prove                                                       |
| ------------------------------------ | ---------------------------------------------------------------- |
| `alloc_burst_absorption_O_blocks`    | N simultaneous arrivals resolved in O(blocks), not O(arrivals).  |
| `alloc_high_water_bound`             | Buffer growth remains within declared bound of live-set peak.    |
| `alloc_collapse_fragmentation_ratio` | Collapse/regrow cycles do not monotonically grow wasted slots.   |
| `alloc_scale_soak_34k`               | 34k entities, sustained churn, bounded buffer, no resize thrash. |

**Report:**

```text
docs/tests/phase_mobility_alloc0_results.md
```

---

## 7. REENROLL — reparenting / bilateral arena re-enrollment

**Status:** **PASS (MOBILITY-REENROLL-0, 2026-06-01).** Bilateral arena re-enrollment substrate
implemented and tested on MOBILITY-ALLOC-0. Flat-star cell arenas, spatial movement only, atomic
commit-or-reject, no live compaction, arrival order not replay-significant. IDROUTE/ECON/OWNER remain
proposed/parked; no production runtime / `SimSession` / default-on. See
[`phase_mobility_reenroll0_results.md`](tests/phase_mobility_reenroll0_results.md).

**Entry gate:** ALLOC green.

**Purpose:** Make spatial reparenting a first-class arena operation.

When an entity moves from Cell A to Cell B:

1. deregister from origin cell arena;
2. register in destination cell arena;
3. rebuild both plans in the same boundary pass;
4. commit atomically or reject with no partial mutation.

**Scope:**

* flat-star cell arenas first;
* spatial movement only;
* identity/political ownership changes are column writes, not reparenting;
* destination uses slab/block reserved slices;
* origin/destination registry generation bumps only on successful commit.

**Non-goals:**

* no nested arena reparenting;
* no Policy B;
* no selector rerun;
* no slot compaction;
* no indirection buffer;
* no capture-as-reparenting.

### REENROLL substrate floor (authorized by MOBILITY-REENROLL-0-OPEN-0)

| Test                                          | Must prove                                                                                 |
| --------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `reenroll_bilateral_origin_destination_accounting` | Deregister origin + register destination in one boundary accounting pass.             |
| `reenroll_atomic_or_reject_no_partial_mutation`    | Both sides commit or neither; generation bumps only on successful commit.              |
| `reenroll_preserves_entity_identity`          | Entity id stable across origin→destination transfer.                                       |
| `reenroll_uses_alloc0_destination_assignment` | Destination slot comes from the MOBILITY-ALLOC-0 deterministic slab substrate.             |
| `reenroll_no_live_slice_compaction`           | No live slice changes address during re-enrollment.                                        |
| `reenroll_arrival_order_independent`          | Arrival order is not replay-significant.                                                   |
| `reenroll_cpu_gpu_parity_layout`              | GPU/CPU parity of the post-move layout.                                                    |

### REENROLL guardrails (designer/scenario admission)

| Test                                                | Must prove                                                       |
| --------------------------------------------------- | ---------------------------------------------------------------- |
| `reenroll_rejects_capture_as_reparenting`           | Capture stays an owner-column flip, never reparenting.           |
| `reenroll_rejects_owner_as_spatial_parent`          | No owner-entity becomes a spatial parent.                        |
| `reenroll_rejects_nested_arena_reparenting_without_gate` | Nested arena reparenting needs a separate gate.             |
| `reenroll_keeps_idroute_econ_owner_parked`          | IDROUTE/ECON/OWNER remain parked.                                |
| `reenroll_does_not_authorize_production_simsession_wiring` | No production `SimSession` wiring.                        |
| `reenroll_does_not_enable_default_on_behavior`      | No default-on behavior.                                          |

### REENROLL performance bars

| Test                                     | Must prove                                                                   |
| ---------------------------------------- | ---------------------------------------------------------------------------- |
| `reenroll_burst_transfer_O_blocks`       | Burst of moves absorbed via bulk accounting in O(affected blocks).           |
| `reenroll_origin_destination_high_water_bound` | Origin/destination buffer growth bounded under sustained transfer.     |
| `reenroll_scale_soak_34k_movement_churn` | 34k entities with continuous inter-cell movement; bounded per-boundary cost. |

**Report:**

```text
docs/tests/phase_mobility_reenroll0_results.md
```

---

## 8. IDROUTE — identity-routing overlay

**Entry gate:** ALLOC + REENROLL green.

**Status:** **PASS + R1 hardened (MOBILITY-IDROUTE-0-R1, 2026-06-02).** Local D=2 identity-routing substrate is green with explicit per-cell `max_factions_per_cell` admission, explicit global-vector rejection, immutable/report-only directed disburse, and 20/20 explicit substrate/guardrail/performance tests. ECON/OWNER remain proposed/parked and require separate opening.

**Purpose:** Prove local D=2 identity routing using masked reduction + directed disburse.

Identity is a column, not tree structure.

Combat, cooperation, and directed flows use the same mechanism:

```text
masked gather → per-identity parent columns → directed disburse → integration / threshold
```

**Scope:**

* local aligned relations only;
* D=2 cell arena;
* `max_factions_per_cell` bounded;
* no global faction vector;
* no owner-entity spatial parent.

### IDROUTE substrate floor

| Test                                 | Must prove                                               |
| ------------------------------------ | -------------------------------------------------------- |
| `idroute_masked_sum_correct`         | Per-identity masked Sum equals exact per-identity total. |
| `idroute_multi_term_sum_determinism` | Multi-term routing Sum uses fixed sorted op order.       |
| `idroute_argmax_packed_key_unique`   | Packed-key Max has deterministic unique winner.          |
| `idroute_directed_disburse_correct`  | Directed disburse output is complete and deterministic.  |
| `idroute_directed_disburse_atomic_or_immutable` | Disburse is report-only immutable or rejected with no partial target state. |
| `idroute_identity_column_not_tree_structure` | Identity remains a local column, not tree structure. |
| `idroute_cpu_gpu_parity_layout`      | Local layout remains CPU/GPU-proxy parity-testable.      |

### IDROUTE guardrails

| Test | Must prove |
| --- | --- |
| `idroute_rejects_global_faction_vector` | Explicit global dense faction vectors are rejected. |
| `idroute_accepts_many_cells_with_local_k_bound` | Many cells are accepted when each cell respects local `k <= 4`. |
| `idroute_rejects_one_cell_exceeding_max_factions_per_cell` | Any single over-wide cell is rejected. |
| `idroute_rejects_owner_as_spatial_parent` | Owner-entities are not spatial parents. |
| `idroute_rejects_capture_as_reparenting` | Capture is not reparenting. |
| `idroute_rejects_econ_owner_runtime` | ECON/OWNER runtime is not part of IDROUTE. |
| `idroute_keeps_econ_owner_parked` | ECON/OWNER remain parked. |
| `idroute_does_not_authorize_production_simsession_wiring` | No production `SimSession` wiring is authorized. |
| `idroute_does_not_enable_default_on_behavior` | No default-on behavior is authorized. |
| `idroute_rejects_semantic_or_raw_wgsl` | Semantic/raw WGSL remains rejected. |

### IDROUTE performance bars

| Test                               | Must prove                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------------ |
| `idroute_d2_masked_dispatch_scale` | Many cells with k≈2 run within existing AO pipeline; masked gather adds fixed bands. |
| `idroute_concentration_one_cell`   | One highly contested cell maintains bounded cost.                                    |
| `idroute_scale_soak_34k`           | 34k entities across contested cells; bounded per-tick cost.                          |

**Report:**

```text
docs/tests/phase_mobility_idroute0_results.md
docs/tests/phase_mobility_idroute0_r1_results.md
```

---

## 9. ECON — session clearinghouse + subsidiarity economy

**Status:** **PASS (MOBILITY-ECON-0, substrate only).** ECON-0 implements the
clearinghouse-circulation + subsidiarity + Band-Alpha/Beta-separation core (battery below). The
**Hybrid-Strata channel partitioning** and **generational faction-index slab** are the broader ECON
scaling architecture and are **NOT part of ECON-0** — they are a later ECON slice. OWNER remains
proposed/parked; no production runtime / `SimSession` / default-on. See [`phase_mobility_econ0_results.md`](tests/phase_mobility_econ0_results.md).

**Entry gate:** ALLOC + REENROLL green; `owner_band_budget_audit` complete (all PASS; ECON-0 ≈ 9
OrderBands ≤ ceiling 16).

**Purpose:** Prove the session-clearinghouse circulation (local-cell up-aggregation → subsidiarity
balance → down-disburse) at substrate level, with hard/soft band separation.

**Core doctrine (ECON-0 slice):**

* `GameSession` root; owner-entities are never spatial parents; capture is owner-column flip.
* Local cell outputs from ALLOC/REENROLL/IDROUTE are admissible inputs.
* Subsidiarity balance at the clearinghouse boundary; self-sufficient subtrees do not escalate.
* Balance decisions in I64 fixed-point (no float structural gate).
* Band Alpha (hard fixed-point) runs before Band Beta (soft float); one-directional Alpha→Beta;
  explicit conservation-class separation; no hard/soft silent mix.
* Deterministic multi-cell up/down ordering; GPU-consumable parity-testable layouts.

### ECON-0 substrate floor (authorized by MOBILITY-ECON-0-OPEN-0)

| Test                                          | Must prove                                                                        |
| --------------------------------------------- | --------------------------------------------------------------------------------- |
| `econ_session_clearinghouse_aggregates_local_cells` | Local cell/block outputs aggregate into clearinghouse columns.              |
| `econ_subsidiarity_balance_conservation`      | Balance at the clearinghouse boundary conserves; self-sufficient subtrees do not escalate. |
| `econ_hard_band_alpha_before_soft_band_beta`  | Hard fixed-point Band Alpha runs before soft float Band Beta (Beta reads finalized Alpha). |
| `econ_rejects_hard_soft_silent_mix`           | Hard and soft quantities never mix in one pass.                                   |
| `econ_deterministic_up_down_disburse`         | Up-aggregation + down-disburse ordering is deterministic; arrival order not significant. |
| `econ_cpu_gpu_parity_layout`                  | Post-circulation layout is bit-exact GPU/CPU.                                     |

### ECON-0 guardrails (designer/scenario admission)

| Test                                          | Must prove                                              |
| --------------------------------------------- | ------------------------------------------------------- |
| `econ_rejects_owner_overlay_runtime`          | No owner-relation/latched-modifier overlay runtime.     |
| `econ_keeps_owner_parked`                     | OWNER remains parked.                                   |
| `econ_rejects_default_on_resource_flow`       | No default-on Resource Flow.                            |
| `econ_rejects_hard_currency_through_resource_flow` | Hard currency never routes through soft Resource Flow. |
| `econ_rejects_float_structural_gate`          | No float value gates a structural transition.           |
| `econ_rejects_production_simsession_wiring`   | No production `SimSession` wiring.                      |
| `econ_rejects_semantic_or_raw_wgsl`           | Semantic/raw WGSL remains rejected.                     |
| `econ_rejects_cpu_planner_urgency_commitment` | No CPU planner / urgency / commitment emission.         |
| `econ_rejects_owner_as_spatial_parent`        | No owner-entity becomes a spatial parent.               |
| `econ_rejects_capture_as_reparenting`         | Capture stays an owner-column flip, never reparenting.  |
| `econ_rejects_hybrid_strata_or_faction_index_scaling_layer` | Hybrid-Strata/faction-index scaling remains a later ECON slice. |

### ECON-0 performance bars

| Test                                | Must prove                                                              |
| ----------------------------------- | ----------------------------------------------------------------------- |
| `econ_multi_cell_clearinghouse_scale` | Many cells aggregate/disburse through the clearinghouse within budget. |
| `econ_concentration_one_session`    | One heavily-loaded session maintains bounded cost.                      |
| `econ_scale_soak_34k`               | 34k entities with blockades/shortages; bounded per-tick cost.           |

### Broader ECON architecture (later slice — NOT authorized by ECON-0-OPEN-0)

These come from the workshop Hybrid-Strata / faction-index scaling layer and are a separate later
ECON slice; do not implement under the ECON-0 gate: `econ_shared_binding_merge_correct`,
`econ_channel_binding_deterministic`, `econ_dense_frontier_stays_local`, `econ_leaf_is_fixed_width_sum`,
`econ_faction_index_static_during_tick`, `econ_local_clears_cheap`.

**Report:**

```text
docs/tests/phase_mobility_econ0_results.md
```

---

## 10. OWNER — owner-relations + latched modifier overlays

**Status:** **PASS + R1 hardened (MOBILITY-OWNER-0-R1, substrate only).** This is the **final v7.9 substrate ladder**
and is green with explicit isolated owned-record down-broadcast coverage; the v7.9 mobility/transfer substrate ladder is complete at substrate level.
Implemented scope is the owner-overlay substrate: owner relations as columns/overlays (never spatial parents), capture
as owner-column flip (never reparenting), latched blockade-immune modifier overlays down-broadcast to
local records **without spawning arena columns**, deterministic application order, generation/resync
on owner-column change with no-silent-rebind. **Production runtime integration remains a separate,
currently-closed gate; the Hybrid-Strata/faction-index ECON scaling layer remains a later ECON slice
— both out of OWNER-0.** See
[`phase_mobility_owner0_results.md`](tests/phase_mobility_owner0_results.md) and
[`phase_mobility_owner0_r1_results.md`](tests/phase_mobility_owner0_r1_results.md).

**Entry gate:** ECON green (all prior substrates PASS; OWNER-0's modifier-down band is already inside
the audited 13 ≤ ceiling 16).

**Purpose:** Prove latched, blockade-immune owner overlays applied by owner-column over the
ALLOC/REENROLL/IDROUTE/ECON substrates.

**Scope:**

* owner relations as explicit columns/overlays, never spatial parents (faction/species/blueprint/tech
  per MOBILITY-SCENARIO-0); subscription by owner-column presence;
* capture = owner-column flip, never reparenting;
* latched, DirtyOnly, blockade-immune modifier overlays down-broadcast to local records;
* down-broadcast overlays never spawn arena/aggregation columns (only flow-pooling relations do —
  proven separate in ECON-0);
* deterministic overlay application order; generation/resync only on owner-column change with explicit
  no-silent-rebind;
* CPU/driver substrate accounting + parity proxy.

### OWNER-0 substrate floor (authorized by MOBILITY-OWNER-0-OPEN-0)

| Test                                          | Must prove                                                              |
| --------------------------------------------- | ----------------------------------------------------------------------- |
| `owner_column_overlay_applies_deterministically` | Owner-column overlays apply in a deterministic order.                |
| `owner_capture_is_column_flip_not_reparenting`| Capture flips an owner column; no spatial reparenting.                  |
| `owner_latched_modifier_overlay_persists`     | A latched modifier persists until its owner-set changes (DirtyOnly).    |
| `owner_blockade_immune_modifier_stays_latched`| Blockade does not drop a latched modifier (knowledge ≠ goods).          |
| `owner_down_broadcast_does_not_spawn_arena_columns` | Down-broadcast overlays never create aggregation/arena columns.   |
| `owner_down_broadcast_reaches_every_owned_including_isolated` | Latched overlays reach every matching owner-column record, including isolated/sparse owned SimThings. |
| `owner_generation_resync_on_owner_column_change` | Owner-column change resyncs deterministically; no silent rebind.     |
| `owner_cpu_gpu_parity_layout`                 | Post-overlay layout is bit-exact GPU/CPU.                               |
| `owner_cohort_homogeneity_via_fission`        | Partial defection/assimilation fissions a new cohort; no mixed cohort.  |

### OWNER-0 guardrails (designer/scenario admission)

| Test                                          | Must prove                                              |
| --------------------------------------------- | ------------------------------------------------------- |
| `owner_rejects_owner_as_spatial_parent`       | No owner-entity becomes a spatial parent.               |
| `owner_rejects_capture_as_reparenting`        | Capture stays an owner-column flip.                     |
| `owner_rejects_nested_arena_reparenting`      | No nested arena reparenting.                            |
| `owner_rejects_default_on_resource_flow`      | No default-on Resource Flow.                            |
| `owner_rejects_hard_currency_through_resource_flow` | Hard currency never routes through soft Resource Flow. |
| `owner_rejects_production_simsession_wiring`  | No production `SimSession` wiring.                      |
| `owner_rejects_semantic_or_raw_wgsl`          | Semantic/raw WGSL remains rejected.                     |
| `owner_rejects_cpu_planner_urgency_commitment`| No CPU planner / urgency / commitment emission.         |
| `owner_rejects_hybrid_strata_or_faction_index_scaling_layer` | Hybrid-Strata/faction-index scaling stays a later ECON slice. |
| `owner_keeps_production_runtime_integration_parked` | Production runtime integration stays closed.      |

### OWNER-0 performance bars

| Test                            | Must prove                                                                |
| ------------------------------- | ------------------------------------------------------------------------- |
| `owner_overlay_multi_cell_scale`| Owner overlays apply across many cells within budget.                     |
| `owner_concentration_one_owner` | One heavily-loaded owner maintains bounded cost.                          |
| `owner_dirtyonly_amortized`     | No owner-set changes → zero modifier dispersal cost.                      |
| `owner_down_broadcast_reaches_every_owned_including_isolated` | Dirty owner/modifier tick may touch all owned records, while steady no-change stays a deterministic no-op. |
| `owner_band_budget_audit`       | Interleaved circulations (incl. modifier-down) fit `max_orderband_depth`. |
| `owner_scale_soak_34k`          | 34k entities with faction+species owners, unlocks/captures; bounded cost. |

**Report:**

```text
docs/tests/phase_mobility_owner0_results.md
docs/tests/phase_mobility_owner0_r1_results.md
```

---

## 10A. RUNTIME — production runtime integration (post-substrate gate)

**Status:** **PASS (MOBILITY-RUNTIME-0, test-only composition harness).** Implemented as a **test-only, default-off substrate-composition harness only** — **not**
production `SimSession`/GPU pass-graph wiring, which remains a separate, currently-closed later gate
(per `invariants.md` lines 108/128/161/184). See
[`phase_mobility_runtime0_opening_review_results.md`](tests/phase_mobility_runtime0_opening_review_results.md) and
[`phase_mobility_runtime0_results.md`](tests/phase_mobility_runtime0_results.md).

**Entry gate:** all v7.9 substrates green (ALLOC/REENROLL/IDROUTE(+R1)/ECON/OWNER(+R1)).

**Why narrowed:** the substrates are pure `simthing-spec` metadata/proxy models validated in
isolation; their **ordered composition is the unproven step**, and the invariants gate production
`SimSession` wiring separately. RUNTIME-0 proves the composition; real production wiring is a later
gate.

**Authorized scope (the narrowing):** deterministically compose ALLOC → REENROLL → IDROUTE → ECON →
OWNER outputs as a CPU/driver composition with existing parity proxies; explicit opt-in /
default-off; **no default `SimSession` pass-graph wiring, no GPU runtime hook, no default-on**;
invoked only from test/fixture paths; composition preserves every substrate invariant (deterministic
replay; parity proxy; movement writes only the mover's own columns; capture = owner-column flip; ECON
flow separate from OWNER overlay; hard Band Alpha before soft Band Beta; no hard/soft silent mix;
isolated owned units receive owner overlays by owner-column presence).

### RUNTIME-0 substrate-integration floor (authorized by MOBILITY-RUNTIME-0-OPEN-0)

| Test | Must prove |
| --- | --- |
| `runtime0_opt_in_only_default_off` | Composition is opt-in; default-off. |
| `runtime0_no_simsession_passgraph_wiring` | Harness does **not** wire into the default session pass graph (real wiring is a separate later gate). |
| `runtime0_integrates_alloc_reenroll_idroute_econ_owner_in_order` | Substrates compose in the documented order. |
| `runtime0_preserves_deterministic_replay` | Composition preserves deterministic replay. |
| `runtime0_preserves_cpu_gpu_parity_proxy` | Composed parity proxy is bit-exact CPU/GPU-proxy. |
| `runtime0_movement_writes_only_moving_simthing_columns` | Movement touches only the mover's own columns. |
| `runtime0_capture_remains_owner_column_flip` | Capture is an owner-column flip, not reparenting. |
| `runtime0_owner_overlay_reaches_isolated_owned_unit` | Owner overlays reach every owned unit incl. an isolated one (by owner-column presence). |
| `runtime0_econ_resource_flow_separate_from_owner_modifier_overlay` | ECON per-tick flow stays distinct from latched OWNER overlays. |
| `runtime0_no_hard_soft_silent_mix` | Hard Band Alpha and soft Band Beta never silently mix. |

### RUNTIME-0 guardrails (designer/scenario admission)

| Test | Must prove |
| --- | --- |
| `runtime0_rejects_default_on_behavior` | No default-on behavior. |
| `runtime0_rejects_semantic_or_raw_wgsl` | No semantic/raw WGSL. |
| `runtime0_rejects_cpu_planner_urgency_commitment` | No CPU planner / urgency / commitment emission. |
| `runtime0_rejects_owner_as_spatial_parent` | No owner-entity as a spatial parent. |
| `runtime0_rejects_capture_as_reparenting` | Capture is not reparenting. |
| `runtime0_rejects_nested_arena_reparenting` | No nested arena reparenting. |
| `runtime0_rejects_default_on_resource_flow` | No default-on Resource Flow. |
| `runtime0_rejects_hard_currency_through_resource_flow` | No hard currency through Resource Flow. |
| `runtime0_rejects_hybrid_strata_or_faction_index_scaling` | Hybrid-Strata/faction-index scaling stays parked. |
| `runtime0_rejects_closed_ladder_reopen` | No reopen of atlas runtime / E-11B-5 / B-1 / ClauseThing-L3 / FrontierV2-5 / ACT-EVENT-OBS-PIPE. |

### RUNTIME-0 performance / soak bars

| Test | Must prove |
| --- | --- |
| `runtime0_34k_integrated_scenario_soak` | 34k-entity composed scenario; bounded cost; bit-stable replay. |
| `runtime0_dirty_owner_modifier_steady_state_zero_redisperse` | No owner-set change → zero modifier re-dispersal in the composed pipeline. |
| `runtime0_mobility_churn_with_owner_overlay_and_econ_clearinghouse` | Movement churn + owner overlays + ECON clearinghouse compose deterministically under load. |

**Report:**

```text
docs/tests/phase_mobility_runtime0_results.md
```

### 10A.1 RUNTIME-1A — CPU-only `simthing-spec` production-fixture model

**Status:** **PASS (MOBILITY-RUNTIME-1A, 2026-06-02; boundary clarified MOBILITY-RUNTIME-1A-R1, 2026-06-02).**
CPU-only, default-off **`simthing-spec` production-fixture model** wiring the green RUNTIME-0
composition into `MobilityRuntime1aSimSessionSurface` behind an explicit named gate. This is **not**
actual production runtime crate wiring — `simthing-driver`, `simthing-gpu`, and GPU pass-graph
registration are untouched. No default schedule, no gameplay-facing integration.
**MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE** (actual production runtime crate `SimSession` fixture wiring)
and **RUNTIME-1B** (GPU pass-graph registration) remain separate, currently-closed later gates. See
[`phase_mobility_runtime1_results.md`](tests/phase_mobility_runtime1_results.md) and
[`phase_mobility_runtime1a_r1_results.md`](tests/phase_mobility_runtime1a_r1_results.md).

**Floor:** `runtime1_explicit_opt_in_only`, `runtime1_default_simsession_behavior_unchanged`,
`runtime1_registers_named_mobility_composition_fixture`, `runtime1_no_default_passgraph_schedule`,
`runtime1_cpu_only_no_gpu_passgraph` *(design-authority addition — enforces the 1A/1B split)*,
`runtime1_preserves_runtime0_composition_order`, `runtime1_preserves_deterministic_replay`,
`runtime1_preserves_cpu_gpu_parity_proxy`, `runtime1_preserves_owner_overlay_isolated_unit`,
`runtime1_preserves_econ_owner_separation`, `runtime1_no_hard_soft_silent_mix`.
**Guardrails:** `runtime1_rejects_{default_on_behavior, semantic_or_raw_wgsl,
cpu_planner_urgency_commitment, owner_as_spatial_parent, capture_as_reparenting,
nested_arena_reparenting, default_on_resource_flow, hard_currency_through_resource_flow,
hybrid_strata_or_faction_index_scaling, closed_ladder_reopen, unscoped_gpu_passgraph_wiring}`.
**Perf/soak:** `runtime1_34k_production_fixture_soak`,
`runtime1_dirty_owner_modifier_steady_state_zero_redisperse`,
`runtime1_mobility_churn_with_owner_overlay_and_econ_clearinghouse`,
`runtime1_no_default_runtime_cost_when_disabled`.

**Report:** `docs/tests/phase_mobility_runtime1_results.md`.

**Deferred (separate, currently-closed gates):** **MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE** — actual
`simthing-driver` / production runtime crate `SimSession` fixture wiring (openable only after RUNTIME-1A
is green and per the invariants' production-wiring rails). **RUNTIME-1B** — GPU pass-graph registration
(opt-in/non-default). A default production schedule and gameplay-facing integration remain beyond that,
unopened.

---

## 11. Cross-track guardrails

These are enforced at designer/scenario admission, not re-proven per PR unless marked as substrate floor.

Reject scenarios that:

* model owner-entities as spatial parents;
* model capture as reparenting;
* require semantic/raw WGSL from designer input;
* require GPU-side allocator semaphore / nondeterministic atomics;
* require indirection buffer before slab/block path is attempted;
* mix hard and soft quantity classes in one pass;
* let float values gate structural transitions;
* exceed `max_factions_per_cell` or routing EML node budget;
* use arrival order as a replay-significant ordering source;
* silently rebind Hybrid Strata channels without generation bump/resync;
* require default-on Resource Flow;
* route hard-currency through Resource Flow;
* reopen FrontierV2-5 or ACT/EVENT/OBS/PIPE.

---

## 12. Required hygiene for every implementation PR

Every implementation PR must:

1. create exactly one test report in `docs/tests`;
2. save relevant test results there if visibility is required;
3. delete scratch/tmp/log outputs when no longer needed;
4. not commit `target/`, `.claude/worktrees`, replay LDJSON, or local benchmark dumps;
5. update this production track compactly;
6. update worklog compactly;
7. not edit `invariants.md` without Tier-2 design-authority process;
8. not perform SHA/fingerprint hygiene unless the step itself requires a deterministic fixture fingerprint;
9. honestly classify status;
10. avoid self-acceptance.

---

## 13. Historical first actionable PR

The first PR was not ALLOC implementation. It was:

```text
MOBILITY-SCENARIO-0 — Scenario/admission packet for mobility and transfer allocation
```

**Why first:** the workshop resolved substrate architecture, but scenario/product parameters still determined bounds: `max_factions_per_cell`, block size, fleet density, 34k soak shape, entity identity boundary, quantity classes, and economy topology.

**Deliverable:** scenario/admission doc + maybe `simthing-spec` metadata/rejection vocabulary if needed.

**Report:**

```text
docs/tests/phase_mobility_scenario0_results.md
```

**Status after PR:**

```text
Scenario accepted by MOBILITY-SCENARIO-0-ACCEPT-0. MOBILITY-AUDIT-0 passed, MOBILITY-ALLOC-0-OPEN-0 opened only the deterministic slab + bulk-accounting substrate, and MOBILITY-ALLOC-0 is now green.
```

---

## 14. Final track posture

This production track is landed as a parked future track (MOBILITY-TRACK-0). MOBILITY-SCENARIO-0 is accepted, MOBILITY-AUDIT-0 passes, and MOBILITY-ALLOC-0 / REENROLL-0 / IDROUTE-0(+R1) / ECON-0 / OWNER-0(+R1) are all green at substrate level. **The v7.9 mobility/transfer substrate ladder is complete at substrate level.** **MOBILITY-RUNTIME-0 is PASS (test-only composition harness); MOBILITY-RUNTIME-1A is PASS (CPU-only, default-off `simthing-spec` production-fixture model — no real runtime crate or GPU pass graph).** Actual production runtime crate fixture wiring (MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE) and GPU pass-graph wiring (RUNTIME-1B) remain separate, currently-closed later gates. The Hybrid-Strata/faction-index ECON scaling layer also remains a later, separately-gated ECON slice.

Expected initial row:

| Step             | Intent                                                                       | Status               | Report                             |
| ---------------- | ---------------------------------------------------------------------------- | -------------------- | ---------------------------------- |
| MOBILITY-TRACK-0 | Create parked v7.9 mobility/transfer production track from workshop findings | **Done / docs-only** | — |
| V7.8/V7.9-DOC-R1 | Reconcile stale v7.8 Line C “pending/remaining gate” language in active docs   | **Done / docs-only** | [`phase_v7_8_v7_9_doc_r1_results.md`](tests/phase_v7_8_v7_9_doc_r1_results.md) |
| MOBILITY-SCENARIO-0 | Add typed scenario/admission metadata and rejection coverage for the first v7.9 mobility/transfer scenario packet; no runtime implementation or implementation gate opened | **Accepted by MOBILITY-SCENARIO-0-ACCEPT-0; implementation closed** | [`phase_mobility_scenario0_results.md`](tests/phase_mobility_scenario0_results.md) |
| MOBILITY-SCENARIO-0-ACCEPT-0 | Design-authority/product acceptance of the v7.9 mobility scenario; accept Option A and open only `MOBILITY-AUDIT-0`; docs-only | **Accepted / docs-only** | [`phase_mobility_scenario0_acceptance_review_results.md`](tests/phase_mobility_scenario0_acceptance_review_results.md) |
| MOBILITY-AUDIT-0 | Audit accepted v7.9 mobility owner/OrderBand depth budget; no runtime implementation or implementation gate opened | **PASS / audit-only** | [`phase_mobility_owner_band_budget_audit_results.md`](tests/phase_mobility_owner_band_budget_audit_results.md) |
| MOBILITY-ALLOC-0-OPEN-0 | Design-authority/product opening review for deterministic slab + bulk-accounting allocator substrate | **OPEN / docs-only authorization** | [`phase_mobility_alloc0_opening_review_results.md`](tests/phase_mobility_alloc0_opening_review_results.md) |
| MOBILITY-ALLOC-0 | Deterministic per-parent/key slab allocation + bulk accounting substrate; no downstream runtime integration | **PASS / substrate-only** | [`phase_mobility_alloc0_results.md`](tests/phase_mobility_alloc0_results.md) |
| MOBILITY-REENROLL-0-OPEN-0 | Design-authority/product opening review for bilateral arena re-enrollment substrate | **OPEN / docs-only authorization** | [`phase_mobility_reenroll0_opening_review_results.md`](tests/phase_mobility_reenroll0_opening_review_results.md) |
| MOBILITY-REENROLL-0 | Bilateral arena re-enrollment substrate on MOBILITY-ALLOC-0; no downstream runtime integration | **PASS / substrate-only** | [`phase_mobility_reenroll0_results.md`](tests/phase_mobility_reenroll0_results.md) |
| MOBILITY-IDROUTE-0-OPEN-0 | Design-authority/product opening review for local D=2 identity-routing substrate | **OPEN / docs-only authorization** | [`phase_mobility_idroute0_opening_review_results.md`](tests/phase_mobility_idroute0_opening_review_results.md) |
| MOBILITY-IDROUTE-0 | Local D=2 identity-routing substrate; no ECON/OWNER or downstream runtime integration | **PASS / substrate-only** | [`phase_mobility_idroute0_results.md`](tests/phase_mobility_idroute0_results.md) |
| MOBILITY-IDROUTE-0-R1 | Remedial hardening for local-k admission, global-vector rejection, directed disburse atomic-or-immutable coverage, and exact battery reporting | **PASS / substrate-only hardening** | [`phase_mobility_idroute0_r1_results.md`](tests/phase_mobility_idroute0_r1_results.md) |
| MOBILITY-ECON-0-OPEN-0 | Design-authority/product opening review for session-clearinghouse + subsidiarity economy substrate (clearinghouse-circulation first slice) | **OPEN / docs-only authorization** | [`phase_mobility_econ0_opening_review_results.md`](tests/phase_mobility_econ0_opening_review_results.md) |
| MOBILITY-ECON-0 | Session-clearinghouse + subsidiarity economy substrate; no OWNER, Hybrid-Strata/faction-index scaling, or downstream runtime integration | **PASS / substrate-only** | [`phase_mobility_econ0_results.md`](tests/phase_mobility_econ0_results.md) |
| MOBILITY-OWNER-0-OPEN-0 | Design-authority/product opening review for owner-relations + latched modifier overlay substrate (final v7.9 substrate ladder) | **OPEN / docs-only authorization** | [`phase_mobility_owner0_opening_review_results.md`](tests/phase_mobility_owner0_opening_review_results.md) |
| MOBILITY-OWNER-0 | Owner-relations + latched modifier overlay substrate; final v7.9 substrate ladder; no production runtime integration | **PASS / substrate-only** | [`phase_mobility_owner0_results.md`](tests/phase_mobility_owner0_results.md) |
| MOBILITY-OWNER-0-R1 | Hardening for isolated owner-overlay down-broadcast completeness and dirty/no-op cost decomposition | **PASS / substrate-only hardening** | [`phase_mobility_owner0_r1_results.md`](tests/phase_mobility_owner0_r1_results.md) |
| MOBILITY-RUNTIME-0-OPEN-0 | Design-authority/product opening review for production runtime integration of the completed substrate ladder | **OPEN WITH NARROWING / docs-only authorization** (test-only default-off composition harness; real `SimSession`/GPU wiring deferred) | [`phase_mobility_runtime0_opening_review_results.md`](tests/phase_mobility_runtime0_opening_review_results.md) |
| MOBILITY-RUNTIME-0 | Default-off substrate-composition harness for ALLOC -> REENROLL -> IDROUTE -> ECON -> OWNER; no production `SimSession`/GPU wiring | **PASS / test-only composition harness** | [`phase_mobility_runtime0_results.md`](tests/phase_mobility_runtime0_results.md) |
| MOBILITY-RUNTIME-1-OPEN-0 | Design-authority/product opening review for production `SimSession`/GPU pass-graph wiring | **OPEN WITH NARROWING / docs-only authorization** → RUNTIME-1A only (CPU-only default-off named-gate fixture; GPU pass-graph split out as RUNTIME-1B, closed) | [`phase_mobility_runtime1_opening_review_results.md`](tests/phase_mobility_runtime1_opening_review_results.md) |
| MOBILITY-RUNTIME-1A | CPU-only default-off `simthing-spec` production-fixture model wiring RUNTIME-0 composition into `MobilityRuntime1aSimSessionSurface`; no runtime crate or GPU pass-graph | **PASS / CPU-only `simthing-spec` fixture model** | [`phase_mobility_runtime1_results.md`](tests/phase_mobility_runtime1_results.md); boundary: [`phase_mobility_runtime1a_r1_results.md`](tests/phase_mobility_runtime1a_r1_results.md) |
| MOBILITY-RUNTIME-1A-R1 | Verify production-fixture boundary; reconcile RUNTIME-1A status language | **PASS WITH NARROWING / docs + test hardening** | [`phase_mobility_runtime1a_r1_results.md`](tests/phase_mobility_runtime1a_r1_results.md) |
| MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE | Actual production runtime crate `SimSession` fixture wiring | **Closed (separate later gate)** | — |
