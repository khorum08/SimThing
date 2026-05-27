# D-1 — Discrete-Transaction Contention Memo

**Status:** Accepted (design review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–Phase T audit of discrete hard-currency transaction contention on AccumulatorOp v2  
**Audience:** Cursor implementation handoff, GPT review, production plan maintainers

---

## Executive summary

Phase T landed spec-owned transfer / recipe / emission / threshold-emit registrations with explicit scenario opt-in and default-off global flags. The production substrate already rejects the highest-risk class of discrete transfer contention — **same-band consumed-input double-debit** — at three layers: T-2 spec compile, C-8c GPU transfer planner, and bootstrap encode validation.

What remains unresolved is **cross-family and cross-stage ordering** at boundaries: multiple transaction families debiting the same source cell in the same tick, replay ordering when boundary events coincide, and registration refresh during structural mutation. These are policy and sequencing questions, not evidence that a new GPU allocator or AccumulatorOp primitive is required at current scales.

**Recommendation:** Keep D-2 deferred. Do not add WGSL, new AccumulatorOp variants, CPU production fallback, or Resource Flow substitution for hard-currency transfer. If implementation is warranted, prefer a **driver-only boundary transaction queue** (CPU compile + ordered AccumulatorOp upload) before any GPU allocator revival.

**Next gate:** Either **D-2 implementation handoff** (only if a concrete discrete workload proves need) or **E-11B nested hierarchy GPU** (optional/future).

---

## 1. Current-state audit

### 1.1 Discrete transaction classes (post–Phase T)

| Class | Builder / registration | Consume mode | Typical gate | Production path |
|-------|------------------------|--------------|--------------|-----------------|
| **Discrete transfer (E-2A)** | `resource_transfer_discrete` / `DiscreteTransferRegistration` | `SubtractFromSource` | Authoring `order_band` → `OrderBand(n)` | C-8c single-source transfer; exact source debit |
| **Conjunctive recipe (E-3)** | `conjunctive_recipe` / `ConjunctiveRecipeRegistration` | `SubtractFromAllInputs` | Fixed band 0 in C-8c planner | C-8c conjunctive crossing; min-across-inputs count |
| **Emission (C-8d)** | `EmissionRegistration` | `EmitEvent` (read-only source) | Emission band pipeline | GPU emission readback; IdentityFloor / Constant / EvalEML |
| **Emit on threshold (E-1)** | `EmitOnThresholdRegistration` | Values: threshold gate; Output: event buffer | Threshold / OrderBand | Separate threshold session upload |
| **Boundary-stage forms** | Driver `resource_economy_sync` after install / fission / boundary | N/A (upload + dispatch) | Session flags + `ResourceEconomyOptInMode` | Existing transfer/emission accumulator sessions |

Hard-currency movement is **exact discrete debit/credit** through AccumulatorOp transfer/recipe ops. Continuous Resource Flow (E-11 flat-star) is a separate substrate and does not substitute for discrete transfer.

### 1.2 Phase T production posture (unchanged by D-1)

- **Phase T complete.** Resource economy transfer/emission execution is **explicit opt-in only** (`ResourceEconomyOptInMode`).
- Global `use_accumulator_transfer` and `use_accumulator_emission` remain **default false**.
- Populated specs without opt-in still reject at boundary sync (T-4).
- No WGSL changes, no CPU production fallback, `simthing-sim` remains spec-free and semantic-free.

---

## 2. Existing guardrails

### 2.1 T-2 spec compile — same-band consumed-input contention

`simthing-spec::compile::resource_economy` maintains a `ContentionTracker` keyed by `(order_band, property_id, col)`:

- Discrete transfers record consumed source cells at their authored `order_band`.
- Recipe inputs record at **recipe band 0** (`RECIPE_ORDER_BAND`).
- Duplicate consumed cells in the same band fail compile with `ResourceEconomyConsumedInputContention`.

This catches authoring mistakes before materialization.

### 2.2 C-8c GPU transfer planner — same-band consumed-input contention (policy A)

`simthing-gpu::transfer_accumulator::plan_transfer_ops` rejects two registrations debiting the same `(slot, col)` in one plan:

- Error: `TransferPlanError::ContendedConsumedInput { slot, col }`.
- **Same-target writes remain allowed** (atomic add to target).
- Applies to the merged transfer + recipe registration vector uploaded at session sync.

### 2.3 Bootstrap encode validation

`validate_no_contention` in `accumulator_op::bootstrap_validate` rejects unsafe consume patterns in encoded GPU op sets (Always-band and OrderBand consumes on the same source cell).

### 2.4 Exact conservation invariants (not weakened)

- Discrete transfer: source debits exactly `amount`; target credits exactly `amount`.
- Recipe: inputs debit `unit_cost × fired_count`; target credits `fired_count` (Identity scale).
- `throttle_hint_max_per_tick` remains metadata only (E-3R).

### 2.5 What is explicitly *not* guarded today

| Gap | Notes |
|-----|-------|
| Cross-band same-cell debit | Transfer band 1 and band 2 may both debit the same source in one tick if authoring allows |
| Transfer vs recipe cross-family | T-2 tracks bands separately; recipe band 0 vs transfer band N may contend on same property if slots/cols overlap after materialization |
| Emission vs transfer | Emission reads source; does not consume — no debit contention, but replay ordering may affect read values |
| Multiple boundary hooks | Capability handlers + resource economy sync ordering is driver-defined, not a global transaction ledger |
| Fission / structural mutation | `ResourceEconomyRegistry.generation` bumps and reupload; in-flight tick vs boundary refresh ordering relies on session loop discipline |

---

## 3. Remaining contention classes

### 3.1 Boundary-time competing source debits

Realistic scenario: treaty payment (discrete transfer) and emergency construction spend (recipe) both debit the same treasury cell in one boundary window. Current defenses:

- If both map to the same `(order_band, property, col)` at compile time → **rejected at T-2**.
- If bands differ → **both may execute** in one tick; conservation holds per op, but **priority/ordering is undefined** across bands.

### 3.2 Same owner / same source / multiple transaction families

After live slot materialization, distinct authored properties may collocate on the same `(slot, col)` only if they are the same property — different properties on the same owner get distinct columns. Contention is therefore **same-property, multi-registration**, caught when bands and cells align at compile/plan time.

### 3.3 Replay ordering under simultaneous boundary events

Replay v3 restores spec snapshot + per-frame deltas. Resource economy registrations reinstall from spec; transfer/emission ops reupload on generation change. Determinism requires:

- Stable emission `reg_idx` by authoring id (T-3 — landed).
- Generation-keyed skip does not skip when generation changes (T-4/T-5 — tested).
- **Unresolved:** if future boundary transactions are ordered by event arrival rather than deterministic sort, replay could diverge — not observed in current T-5 replay tests, but D-2 should enforce a total order if multi-transaction boundaries expand.

### 3.4 Fission / structural mutation during registration refresh

`sync_resource_economy_if_enabled` runs after each boundary. Structural fission may change slot allocation; generation bump forces reupload. Gap: ticks between fission and sync where stale ops might reference old slots — mitigated by boundary-gated sync in `SimSession::run`, not mid-tick refresh.

---

## 4. Recommended policy boundary

**In scope for discrete hard-currency (AccumulatorOp transfer/recipe path):**

- Exact source debit conservation.
- Same-band consumed-input rejection (keep T-2 + C-8c + bootstrap).
- Explicit scenario opt-in (T-6).
- Driver-owned compile, materialization, upload, and boundary refresh.

**Out of scope (preserve stop conditions):**

- Continuous Resource Flow Balance semantics as a substitute for discrete transfer.
- Approximate allocation or probabilistic debit.
- Hard-currency routing through Resource Flow participant ops.
- CPU production fallback peer.
- New WGSL kernels or AccumulatorOp primitives without a later design review.
- `simthing-sim` ownership of spec or transaction registries.

**Policy recommendation:** Treat **cross-band multi-debit on the same cell** as an **authoring error** unless a documented priority total order exists. Extend T-2 contention tracking to optionally span all bands for production specs, or require distinct source properties — cheaper than D-2 GPU allocator.

---

## 5. Replay determinism requirements

Before any D-2 implementation:

1. Boundary transaction order must be **stable across record and replay** (sort key: authoring id + order_band + registration kind).
2. Resource economy generation and uploaded generation must match after reinstall (T-5 replay tests — baseline).
3. Emission readback records must be reproducible bit-exact under fixed seed and opt-in flags (T-5 burn-in — baseline).
4. Structural mutations must bump registry generation and force reupload before the next transfer/emission dispatch (T-4 boundary tests — baseline).

---

## 6. Stop conditions

Do **not** proceed to D-2 implementation if any of the following would be required:

- New WGSL transfer/recipe kernel or GPU allocator for workshop-scale O(10⁵) pool contention (dissolved for continuous flow; not observed for discrete Phase T workloads).
- New AccumulatorOp primitive for ordering (existing OrderBand + planner sufficient).
- CPU production fallback for transfer/recipe/emission.
- Weakening exact discrete conservation or allowing same-band double-debit.
- Routing hard-currency through Resource Flow.
- Importing `simthing-spec` into `simthing-sim`.

---

## 7. Candidate implementation ladder (if D-2 revives)

| Step | Scope | Rationale |
|------|-------|-----------|
| **D-2a (preferred)** | Driver-only **boundary transaction scheduler**: deterministic sort of pending discrete ops; single merged upload per boundary | Addresses cross-band ordering without GPU changes |
| **D-2b** | Extend T-2 `ContentionTracker` to **all-band union** mode for production specs | Catches authoring collisions earlier |
| **D-2c (defer)** | GPU allocator for discrete queues | Only if profiling shows boundary batches exceeding CPU compile + upload budget at target scale |
| **D-2d (reject)** | Workshop `transfer_contention_gpu.wgsl` v1 as production path | Workshop-local; not AccumulatorOp; policy mismatch |

---

## 8. Tests required before implementation

If D-2a/2b proceeds:

1. **Cross-band same-cell rejection** — spec compile fails when transfer band 1 and band 2 debit same property/role.
2. **Deterministic boundary ordering** — two transfers same band different ids; replay bit-exact.
3. **Transfer + recipe same tick** — oracle parity when bands differ but cells disjoint vs contended.
4. **Fission refresh** — generation bump mid-session; no stale op dispatch (extend T-4/T-5).
5. **Opt-in flag interaction** — Disabled mode never uploads; TransferOnly never uploads emission ops.

No new tests required for memo acceptance; T-5/T-6 suites remain the regression baseline.

---

## 9. Docs updates required

This memo satisfies the D-1 documentation gate. Related docs updated in the same PR:

- `docs/accumulator_op_v2_production_plan.md` — D-1 Done; Phase T complete posture
- `docs/todo.md` — D-1 landed; next gate D-2 or E-11B
- `docs/worklog.md` — D-1 entry
- `docs/workshop/workshop_current_state.md` — D-1 landed; Phase T complete

---

## Bottom line

Discrete hard-currency contention is **partially solved** by layered same-band consumed-input rejection. Phase T did not introduce new contention surfaces beyond authored registration upload at boundaries. **D-2 GPU allocator remains deferred.** The next productive handoff is either a narrow **D-2a driver scheduling** slice (if product needs cross-band priority) or **E-11B nested hierarchy GPU** (optional infrastructure). E-2B remains blocked unless enrollment compilation explicitly lands.
