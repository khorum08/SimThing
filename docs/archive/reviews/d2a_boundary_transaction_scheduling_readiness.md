# D-2a — Boundary Transaction Scheduling Readiness Review

**Status:** Accepted (design review, docs-only)  
**Date:** 2026-05-27  
**Scope:** Post–Phase T / post–D-1 audit of whether hard-currency discrete transfer ordering needs a driver-only boundary scheduling layer now  
**Audience:** Cursor implementation handoff, GPT review, production plan maintainers  
**Prior art:** [`d1_discrete_transaction_contention_memo.md`](d1_discrete_transaction_contention_memo.md)

---

## Executive summary

Phase T landed spec-owned discrete transfer / recipe / emission / threshold-emit registrations with explicit opt-in and default-off global flags. Layered **same-band consumed-input rejection** (T-2 compile, C-8c GPU planner, bootstrap encode) already protects the highest-risk double-debit class for current shipped workloads.

**Finding:** No valid **current product** hard-currency scenario is blocked by same-band collision rejection. Designer RON smoke, T-5 burn-in, and session-open tests use disjoint source debits or single-debit paths. However, **`order_band` is authored and tracked but not yet wired through the C-8c transfer planner** — all transfer/recipe ops are planned on `GateSpec::OrderBand(0)`. Cross-band same-source authoring therefore passes T-2 compile but would fail at boundary sync (`TransferPlanError::ContendedConsumedInput`) if attempted today.

**Recommendation: B — defer D-2a implementation.** Keep Phase T same-band rejection as sufficient for current production safety. Document the `order_band` wiring gap as technical debt. If product later needs sequential multi-debit on the same source within one boundary, implement the **narrow D-2a ladder** (driver scheduling + existing OrderBand wiring; no new WGSL, roles, or CPU fallback). **Do not send to Opus** unless requirements expand beyond driver-only scheduling.

| Option | Verdict |
|--------|---------|
| **A. Implement D-2a now** | **Not recommended** — no concrete product scenario blocked today |
| **B. Defer D-2a** | **Recommended** — current guardrails sufficient for Phase T workloads |
| **C. Send to Opus** | **Not required** — no WGSL/role/spec-redesign trigger |

**Next gate depends on product priority:** narrow D-2a implementation (only after a named multi-transaction scenario), E-11B nested hierarchy GPU, simthing-spec/RON rebuild, or continued soak.

---

## 1. Current-state audit

### 1.1 Hard-currency production path (post–Phase T)

| Stage | Owner | Responsibility |
|-------|-------|----------------|
| Authoring | `simthing-spec::ResourceEconomySpec` | Transfer `order_band`, recipe inputs, emission formulas, threshold emits |
| Compile | `simthing-spec::compile::resource_economy` | Property/EML resolution, same-band contention pre-check, expansion report |
| Materialize | `simthing-driver::resource_economy_compile` | Slot resolution, `DiscreteTransferRegistration` / recipe / emission structs, stable emission `reg_idx`, `transfer_order_band_by_id` diagnostics |
| Boundary sync | `simthing-driver::resource_economy_sync` | Flag-gated upload, generation-keyed skip, merge transfers + recipes → C-8c planner |
| GPU plan | `simthing-gpu::transfer_accumulator::plan_transfer_ops` | Encode AccumulatorOp transfer/recipe ops; reject same-plan consumed-input contention |
| Dispatch | Existing AccumulatorOp v2 OrderBand passes | Exact discrete debit/credit; emission read-only source |

Hard-currency movement remains **exact discrete** AccumulatorOp transfer/recipe/emission. Resource Flow (E-11 flat-star) is separate and must not substitute for discrete transfer ([`resource_flow_substrate.md`](../adr/resource_flow_substrate.md)).

### 1.2 Phase T + RF posture (unchanged)

- **Phase T complete.** Transfer/emission execution is explicit opt-in only (`ResourceEconomyOptInMode`).
- Global `use_accumulator_transfer` and `use_accumulator_emission` remain **default false**.
- **Phase T designer/RON smoke addendum** landed — designer-authored `resource_economy_smoke.ron` exercises deserialize → compile → install → `open_from_spec` without expanding runtime semantics.
- **Bounded `FlatStarResourceFlow`** posture unchanged; global Resource Flow default-on remains deferred.
- `simthing-sim` remains spec-free and semantic-free.

### 1.3 Critical wiring gap (D-2a relevance)

`ResourceTransferSpec::order_band` is:

- Required at authoring (`resource_economy.rs`).
- Preserved through T-2 compile (`CompiledResourceTransfer.order_band`).
- Recorded in T-3 materialization report (`transfer_order_band_by_id`).

But C-8c `plan_transfer_ops` assigns **`GateSpec::OrderBand(0)` to every registration** regardless of authored band. Recipe registrations are also planned at band 0. `n_bands` is therefore 0 or 1 today.

**Implication:** Cross-band sequential debit is **not available** at runtime even though T-2 contention tracking is band-scoped. Multi-transfer specs that debit the same `(property, col)` on different authored bands pass compile but **fail sync** when merged into one planner call.

---

## 2. Already-covered contention cases

### 2.1 T-2 spec compile — same-band consumed-input rejection

`ContentionTracker` keys `(order_band, property_id, col)`:

| Registration kind | Band used | Rejects |
|-------------------|-----------|---------|
| Discrete transfer | Authored `order_band` | Second transfer debiting same property/role in same band |
| Recipe input | Fixed `RECIPE_ORDER_BAND = 0` | Second recipe input on same property/role in band 0 |
| Cross-kind | Separate band keys | Transfer band 1 + recipe band 0 on same property **allowed at compile** |

Compile error: `SpecError::ResourceEconomyConsumedInputContention`.

**Test evidence:** `resource_economy_rejects_same_band_consumed_input_contention_when_detectable` in `resource_economy_compile_rejections.rs`.

### 2.2 C-8c GPU transfer planner — merged-plan consumed-input rejection

`plan_transfer_ops` rejects duplicate `(slot, col)` in the merged registration vector:

- Error: `TransferPlanError::ContendedConsumedInput`.
- **Same-target writes allowed** (atomic add).
- Applies to transfers + recipes uploaded together at boundary sync.

Because all ops are currently band 0, this effectively enforces **global single-debit-per-cell per boundary upload**, stricter than T-2's per-band model.

### 2.3 Bootstrap encode validation

`validate_no_contention` rejects unsafe Always/OrderBand consume patterns in encoded op sets.

### 2.4 Phase T boundary guardrails (landed)

- Populated spec + flag off → reject at sync (T-4).
- Generation-keyed skip + reupload on structural change (T-4/T-5).
- Stable emission `reg_idx` by authoring id (T-3/T-5).
- 100-tick transfer/recipe/emission conservation burn-in + replay (T-5).
- Designer RON session smoke: transfer/recipe/emission materialize; Resource Flow flag stays off.

### 2.5 Emission / threshold (non-debit)

Emissions use read-only source consume (`EmitEvent`); threshold emits gate on values. They do not participate in consumed-input debit contention. Ordering relative to debits follows AccumulatorOp band dispatch order once bands are wired.

---

## 3. Product scenarios still blocked

### 3.1 Not blocked today (current workloads)

| Scenario | Status |
|----------|--------|
| Single discrete transfer per source per tick | **Supported** — T-5 burn-in |
| Conjunctive recipe with disjoint inputs | **Supported** — T-5 recipe burn-in |
| Transfer + recipe + emission in one mode (disjoint debits) | **Supported** — designer RON smoke |
| Same-band double-debit (authoring mistake) | **Rejected** — T-2 + C-8c |
| Populated economy spec without opt-in | **Rejected** — T-4 flag-off |
| Hard-currency via Resource Flow | **Out of scope / rejected by policy** |

### 3.2 Latent / future scenarios (not valid product paths yet)

| Scenario | Current behavior | D-2a relevance |
|----------|------------------|----------------|
| Two transfers, same source, different `order_band` (priority sequencing) | T-2 **passes**; sync **fails** (both planned band 0) | Needs OrderBand wiring + deterministic schedule |
| Transfer band N + recipe band 0 debiting same treasury cell | T-2 **passes** if bands differ; sync **fails** | Same |
| Three+ sequential debits same source across bands in one boundary | Not representable safely today | D-2a ladder |
| Event-arrival-ordered debits without stable sort key | Not implemented; would threaten replay | D-2a must enforce total order |

**No shipped designer fixture or burn-in scenario requires these paths today.** The designer smoke transfer uses `order_band: 1` but debits a **different source property** than the recipe inputs.

---

## 4. Proposed D-2a scope (if product approves later)

D-2a should remain **narrow**:

1. **Wire authored `order_band`** from materialized registrations into C-8c `GateSpec::OrderBand(n)` (existing substrate).
2. **Driver boundary transaction schedule report** — deterministic sort of pending discrete ops before upload (key: `order_band`, registration kind, authoring id).
3. **Optional T-2 extension (D-2b)** — all-band union contention mode for production specs that must never double-debit a cell even across bands.
4. **Replay tests** — multi-band transfer/recipe/emission bit-exact under fixed seed.

**Out of scope for D-2a:**

- `ResourceEconomySpec` redesign.
- New transaction families or consume modes.
- Resource Flow substitution.
- Boundary-time slot compaction.
- Nondeterministic arbitration.

---

## 5. Driver-only scheduling feasibility

**Yes — if implemented, D-2a can remain driver-led with existing AccumulatorOp OrderBand execution.**

| Requirement | D-2a approach | New WGSL? | New role? | CPU fallback? |
|-------------|---------------|-----------|-----------|---------------|
| Cross-band ordering | Wire `order_band` + driver sort before upload | **No** | **No** | **No** |
| Boundary schedule visibility | Driver report struct (test/diagnostics) | **No** | **No** | **No** |
| Replay determinism | Stable sort key at compile/install | **No** | **No** | **No** |
| Same-band safety | Keep T-2 + C-8c rejection | **No** | **No** | **No** |

The only production touch beyond driver would be **using an existing field** in `simthing-gpu::plan_transfer_ops` — still AccumulatorOp v2, not a new primitive.

**simthing-sim** must remain unaware of spec, bands, or schedules.

---

## 6. Constitutional stop conditions

Do **not** implement D-2a (escalate to Opus / redesign) if any of the following become requirements:

- New WGSL transfer/recipe kernels or workshop-style GPU allocator revival.
- New `AccumulatorRole` variants for ordering.
- CPU production fallback peer for transfer/recipe/emission.
- `simthing-sim` spec or transaction registry awareness.
- `ResourceEconomySpec` schema redesign or global transfer/emission default-on.
- Hard-currency routing through Resource Flow.
- Boundary-time slot compaction or indirection-list SlotRange replacement.
- Nondeterministic transaction arbitration by event arrival time.
- Weakening exact discrete conservation or allowing same-band double-debit.

None of these triggers are active for the narrow D-2a path described above.

---

## 7. Replay and determinism requirements

Before any D-2a implementation lands:

1. **Total order key** — stable across record and replay: `(order_band, kind_rank, authoring_id)` where `kind_rank` is deterministic (e.g. transfer < recipe < emission upload phase).
2. **Generation match** — resource economy registry generation equals uploaded generation after reinstall (T-5 baseline).
3. **Emission reg_idx stability** — authoring id → `reg_idx` unchanged (T-3/T-5 baseline).
4. **Multi-band parity** — GPU vs CPU oracle bit-exact for 100-tick scenarios with bands 0..N-1 debiting disjoint cells; cross-band same-source sequential scenarios once wired.
5. **Flag-off invariants** — Disabled / TransferOnly / EmissionOnly modes unchanged (T-4/T-6).

Current T-5 replay tests cover single-band workloads; they do **not** yet prove multi-band ordering.

---

## 8. Required tests before implementation

If D-2a ladder is approved:

| Test | Purpose |
|------|---------|
| Cross-band same-source compile policy | Either reject at T-2 (all-band union) or accept with wired bands + oracle parity |
| `order_band` wiring | Transfer band 1 executes after band 0; conservation holds |
| Transfer + recipe multi-band | Disjoint vs contended cells; contended fails predictably |
| Deterministic boundary ordering | Two transfers same band, different ids — replay bit-exact |
| Designer-scale fixture extension | Optional multi-band treasury scenario through RON → session |
| Fission generation bump | Stale ops not dispatched after registry refresh (extend T-4/T-5) |
| Flag interaction matrix | Opt-in modes unchanged |

**For this review gate:** existing Phase T / D-1 regression suites are sufficient evidence. No new tests required for memo acceptance.

---

## 9. Review question answers

### Q1. What exact problem would D-2a solve that C-8c / T-2 / Phase T do not already solve?

**Deterministic sequential multi-debit on the same source cell across authored `order_band` values within one boundary**, plus driver-visible scheduling/reporting for replay. Phase T solves same-band rejection, opt-in, materialization, and single-band conservation. It does **not** wire authored bands into GPU planning or define a boundary total order across registration kinds.

### Q2. Which current same-band transfer / recipe / emission collisions are already rejected at compile time?

Same `(order_band, property_id, col)` for:

- Two discrete transfers (same band).
- Two recipe inputs (recipe band 0).
- Transfer + recipe when both map to the same band key (e.g. transfer band 0 + recipe input).

Emissions and threshold emits are not debit-contention tracked at T-2.

### Q3. Are any valid product hard-currency scenarios blocked by current same-band consumed-input collision rejection?

**No.** Current designer RON smoke and T-5 burn-in scenarios use disjoint debits or single-debit paths. Scenarios that **need** same-source multi-debit across bands are **future/latent**, not blocked by rejection — they are **unsupported** because bands are not wired.

### Q4. Would D-2a be driver-only boundary scheduling, or would it require new AccumulatorOp roles / WGSL / CPU fallback?

**Driver-only scheduling is feasible** with existing OrderBand AccumulatorOp execution. Wiring `order_band` in the C-8c planner uses existing gates — not new roles or WGSL. CPU fallback is **not** required.

### Q5. Can cross-band ordering be expressed by existing order_band fields and existing AccumulatorOp scheduling?

**In principle yes** — the authoring field and AccumulatorOp OrderBand dispatch already exist. **In practice not yet** — C-8c flattens to band 0. D-2a implementation must connect them.

### Q6. Does D-2a need to alter ResourceEconomySpec, or only compile/install scheduling?

**Only compile/install scheduling and optional T-2 contention policy.** No `ResourceEconomySpec` redesign required for the narrow path.

### Q7. How would D-2a interact with recipes, emissions, threshold emissions, and stable reg_idx?

- **Recipes:** Remain conjunctive band-0 contention at T-2 unless policy changes; GPU band assignment must align with driver schedule. Recipes debiting same cell as a transfer require explicit band ordering once wired.
- **Emissions:** Read-only; scheduled in emission upload phase; stable `reg_idx` unchanged.
- **Threshold emits:** Separate threshold session path; ordering relative to debits must be documented per band.
- **reg_idx:** Emission identity remains authoring-id stable; D-2a must not reorder emission identity assignment.

### Q8. How should replay determinism be proven?

Extend T-5-style replay tests with multi-band fixtures; assert bit-exact conservation and schedule report equality across record/replay; enforce stable sort keys at boundary sync.

### Q9. What tests would be needed before implementation?

See §8. Minimum: band wiring parity, cross-band ordering oracle, replay bit-exact, flag-off matrix, generation refresh.

### Q10. Should D-2a be implemented now, deferred, or sent to Opus?

**Defer.** No current product scenario requires it. **Do not send to Opus** unless requirements breach §6 stop conditions.

---

## 10. Recommendation

| Decision | **Defer D-2a implementation** |
|----------|----------------------------------|
| Rationale | Phase T same-band rejection is sufficient for shipped workloads; no product scenario blocked |
| Technical debt | Document and track `order_band` not wired to C-8c; cross-band same-source specs are latent footguns |
| If product prioritizes multi-transaction treasury | Approve narrow ladder D-2a-1…D-2a-5 (§11) |
| Opus | **Not required** at this gate |

---

## 11. Candidate implementation ladder (if product approves — not this PR)

| Step | Scope |
|------|-------|
| **D-2a-1** | Document allowed cross-band hard-currency ordering model |
| **D-2a-2** | Driver-side boundary transaction schedule report |
| **D-2a-3** | Wire authored `order_band` through C-8c planner; validate sequencing |
| **D-2a-4** | Deterministic replay tests for multi-band transfer/recipe/emission |
| **D-2a-5** | Docs + burn-in update |

Optional **D-2b:** T-2 all-band union contention for specs that forbid any same-cell multi-debit.

**Reject:** D-2 GPU allocator revival; workshop `transfer_contention_gpu.wgsl` as production path.

---

## 12. Docs update requirements

This review satisfies the D-2a readiness documentation gate. Related docs updated in the same PR:

- `docs/accumulator_op_v2_production_plan.md`
- `docs/todo.md`
- `docs/worklog.md`
- `docs/workshop/workshop_current_state.md`

---

## Bottom line

D-2a boundary transaction scheduling readiness review landed. **No production code changes.** Phase T remains complete. Phase T designer/RON smoke addendum remains landed. Hard-currency transfer remains exact discrete AccumulatorOp transfer/recipe/emission. Resource Flow remains separate. Bounded `FlatStarResourceFlow` posture unchanged. Global Resource Flow default-on remains deferred.

**Current same-band collision rejection is sufficient for production safety today.** Defer D-2a implementation until a named product scenario requires sequential multi-debit ordering; then implement the narrow driver-only ladder without new WGSL, roles, or CPU fallback.
