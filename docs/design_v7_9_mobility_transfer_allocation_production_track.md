# SimThing — Design v7.9 Mobility / Transfer Allocation Production Track

> **Status:** Parked future production track (MOBILITY-TRACK-0 landed, 2026-06-01). No implementation gate is open until product/design authority names and accepts the scenario.
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

No implementation gate is open.

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

**Status:** Proposed first gate. Scenario/admission only. No implementation.

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
| SCENARIO | Product scenario / admission packet            | Product/design authority                          | Landed as admission metadata; awaiting design-authority/product acceptance | MOBILITY-SCENARIO-0 accepted                      |
| AUDIT    | Owner/OrderBand depth budget                   | Scenario accepted                                 | Proposed | `owner_band_budget_audit` passes or narrows scope |
| ALLOC    | Deterministic slab + bulk-accounting allocator | Scenario accepted; A-0 baseline                   | Proposed | ALLOC substrate floor + performance bars green    |
| REENROLL | Reparenting / bilateral arena re-enrollment    | ALLOC green                                       | Proposed | REENROLL substrate floor + performance bars green |
| IDROUTE  | D=2 identity-routing overlay                   | ALLOC + REENROLL green                            | Proposed | IDROUTE substrate floor + performance bars green  |
| ECON     | Session clearinghouse + subsidiarity economy   | ALLOC + REENROLL green; owner-band audit complete | Proposed | ECON substrate floor + performance bars green     |
| OWNER    | Owner-relations + latched modifier overlays    | ECON green                                        | Proposed | OWNER substrate floor + performance bars green    |

---

## 5. AUDIT — owner_band_budget_audit

**Status:** Proposed early audit. No runtime implementation.

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

### REENROLL substrate floor

| Test                                  | Must prove                                                                          |
| ------------------------------------- | ----------------------------------------------------------------------------------- |
| `reenroll_slice_migration_contiguous` | Migrating entity lands contiguously in destination block; origin remains valid.     |
| `reenroll_replay_determinism`         | Same movement sequence and seed produce identical layout and generation trajectory. |
| `reenroll_cpu_gpu_parity_post_move`   | GPU/CPU parity after movement sequences.                                            |

### REENROLL performance bars

| Test                               | Must prove                                                                   |
| ---------------------------------- | ---------------------------------------------------------------------------- |
| `reenroll_burst_moves_bulk`        | Burst of moves absorbed via bulk accounting in O(affected blocks).           |
| `reenroll_scale_soak_34k_mobility` | 34k entities with continuous inter-cell movement; bounded per-boundary cost. |

**Report:**

```text
docs/tests/phase_mobility_reenroll0_results.md
```

---

## 8. IDROUTE — identity-routing overlay

**Entry gate:** ALLOC + REENROLL green.

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

### IDROUTE performance bars

| Test                               | Must prove                                                                           |
| ---------------------------------- | ------------------------------------------------------------------------------------ |
| `idroute_d2_masked_dispatch_scale` | Many cells with k≈2 run within existing AO pipeline; masked gather adds fixed bands. |
| `idroute_concentration_one_cell`   | One highly contested cell maintains bounded cost.                                    |
| `idroute_scale_soak_34k`           | 34k entities across contested cells; bounded per-tick cost.                          |

**Report:**

```text
docs/tests/phase_mobility_idroute0_results.md
```

---

## 9. ECON — session clearinghouse + subsidiarity economy

**Entry gate:** ALLOC + REENROLL green; `owner_band_budget_audit` complete.

**Purpose:** Prove global/misaligned owner routing through the session clearinghouse and spatial spine.

**Core doctrine:**

* `GameSession` root has owner-entities, `SpeciesRegistry`, and `worldStateMap` as siblings.
* Owner-entities are never spatial parents.
* Capture is owner-column flip.
* Economy is a blockable per-tick Resource Flow.
* Modifiers are latched, blockade-immune overlays.
* Hybrid Strata: local anonymous channels → dense N-wide vector near root.
* Faction index is generational slab with Ghost-Node zeroing.
* Band Alpha hard fixed-point runs before Band Beta soft float.

### ECON substrate floor

| Test                                    | Must prove                                                                       |
| --------------------------------------- | -------------------------------------------------------------------------------- |
| `econ_rooting_no_spatial_owner`         | No owner-entity is a spatial-containment parent.                                 |
| `econ_circulation_parity`               | Up-sweep + hub clearing + down-sweep is bit-exact GPU/CPU at spatial depth.      |
| `econ_shared_binding_merge_correct`     | Elementwise channel Sum only under parent-imposed shared binding.                |
| `econ_channel_binding_deterministic`    | Binding sorted by `faction_id`, not arrival order.                               |
| `econ_balance_test_fixed_point`         | Balance decisions use I64 fixed-point.                                           |
| `econ_band_alpha_before_beta`           | Hard exact band precedes soft float band.                                        |
| `econ_faction_index_static_during_tick` | Faction index immutable during GPU tick; Ghost Node zeroing preserves alignment. |

### ECON performance bars

| Test                              | Must prove                                                                           |
| --------------------------------- | ------------------------------------------------------------------------------------ |
| `econ_local_clears_cheap`         | Self-sufficient subtrees do not escalate to full root.                               |
| `econ_dense_frontier_stays_local` | Dense N-wide frontier remains near high nodes under spatial-local contestation.      |
| `econ_leaf_is_fixed_width_sum`    | Leaf aggregation is fixed-width `SlotRange Sum`; no GPU indirection.                 |
| `econ_scale_soak_34k`             | 34k owned entities with blockades/shortages; bounded per-tick cost; stable M5 field. |

**Report:**

```text
docs/tests/phase_mobility_econ0_results.md
```

---

## 10. OWNER — owner-relations + modifier overlays

**Entry gate:** ECON green.

**Purpose:** Prove latched, blockade-immune owner overlays and multi-owner cohort filtering.

**Scope:**

* owner subscription by owner-column presence;
* species and faction are structurally the same owner-entity mechanism;
* SpeciesRegistry is a session-peer grouping node;
* capability trees resolve to overlays and instantiation gates;
* instantiation is gated fission;
* modifier overlays are DirtyOnly and blockade-immune;
* pop cohorts remain homogeneous; partial changes fission a new cohort.

### OWNER substrate floor

| Test                                   | Must prove                                                           |
| -------------------------------------- | -------------------------------------------------------------------- |
| `owner_cohort_homogeneity_via_fission` | Partial defection/assimilation splits a new cohort; no mixed cohort. |

### OWNER performance bars

| Test                        | Must prove                                                                |
| --------------------------- | ------------------------------------------------------------------------- |
| `owner_dirtyonly_amortized` | No owner-set changes → zero modifier dispersal cost.                      |
| `owner_band_budget_audit`   | Interleaved circulations fit `max_orderband_depth` at target depth.       |
| `owner_scale_soak_34k`      | 34k entities with faction+species owners, unlocks/captures; bounded cost. |

**Report:**

```text
docs/tests/phase_mobility_owner0_results.md
```

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

## 13. Suggested first actionable PR

The first PR should not be ALLOC implementation. It should be:

```text
MOBILITY-SCENARIO-0 — Scenario/admission packet for mobility and transfer allocation
```

**Why first:** the workshop resolved substrate architecture, but scenario/product parameters still determine bounds: `max_factions_per_cell`, block size, fleet density, 34k soak shape, entity identity boundary, quantity classes, and economy topology.

**Deliverable:** scenario/admission doc + maybe `simthing-spec` metadata/rejection vocabulary if needed.

**Report:**

```text
docs/tests/phase_mobility_scenario0_results.md
```

**Status after PR:**

```text
Scenario accepted or scenario rejected/narrowed.
No implementation opened until design-authority acceptance.
```

---

## 14. Final track posture

This production track is landed as a parked future track (MOBILITY-TRACK-0). It should not be handed to Cursor as implementation until product/design authority explicitly opens `MOBILITY-SCENARIO-0`.

Expected initial row:

| Step             | Intent                                                                       | Status               | Report                             |
| ---------------- | ---------------------------------------------------------------------------- | -------------------- | ---------------------------------- |
| MOBILITY-TRACK-0 | Create parked v7.9 mobility/transfer production track from workshop findings | **Done / docs-only** | — |
| V7.8/V7.9-DOC-R1 | Reconcile stale v7.8 Line C “pending/remaining gate” language in active docs   | **Done / docs-only** | [`phase_v7_8_v7_9_doc_r1_results.md`](tests/phase_v7_8_v7_9_doc_r1_results.md) |
| MOBILITY-SCENARIO-0 | Add typed scenario/admission metadata and rejection coverage for the first v7.9 mobility/transfer scenario packet; no runtime implementation or implementation gate opened | **Landed / awaiting design-authority + product acceptance** | [`phase_mobility_scenario0_results.md`](tests/phase_mobility_scenario0_results.md) |
