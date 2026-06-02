# ATLAS-0080-0 — Atlas Production Runtime / Sparse-Residency Nested Mapping Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - Opened by the named multi-theater scenario **`SCENARIO-0080-1`** (Nested Starmap).
> - This is the *named first slice* the invariant *"No production mapping runtime without first-slice
>   gating"* contemplates — previously parked (M-4A territory).
> - All wiring is **opt-in / default-off**; **no default session pass-graph wiring** (that is Tier-2).
> - **This PR does not implement the atlas runtime.**

---

## 1. Why now

The atlas designer surface (C-0/C-1/C-2: `request_atlas_batching`, scale model, VRAM budget) was CLOSED;
the **production runtime / sparse-residency scheduler** was held parked *pending a named multi-theater
scenario*. `SCENARIO-0080-1` is that scenario: a nested `session → starmap → starsystem → planet → submap`
structure (~2,100 location simthings) that genuinely needs **sparse residency** (most cells inert) and
**nested theater management** (descend/ascend between theaters). The consumer now exists.

---

## 2. Scope

- `SCENARIO-0080-1` only.
- A **sparse-residency nested mapping runtime**: materialize/reside only the active cells of each theater
  (the 10 starsystems of the starmap; the active subfield of an entered starsystem; the planet submap when
  engaged), within the established VRAM budget posture.
- **Nested theater descent/ascent** as a bounded, deterministic operation (starmap ↔ starsystem ↔ planet).
- Consumes the existing atlas designer-surface artifacts (batching request, scale model, VRAM budget);
  it does **not** redefine them.

---

## 3. Atlas runtime contract

- **Opt-in residency only.** No default-on residency; no global mapping scheduler.
- **Deterministic** materialization/eviction given the same seed + access pattern; **deterministic
  replay**.
- **Bounded** residency footprint within the configured VRAM budget (default posture; configurable; no
  hard cap as per C-2).
- **Sparse:** inert cells are not resident; residency tracks the active theater set only.
- **CPU-oracle bit-exact parity (I8)** for any GPU-resident field the runtime materializes; residency
  must be a **strict no-op on field values** (it changes *where/whether* a field is resident, never the
  field's computed values).
- **Reversible:** disabled ⇒ no residency runtime, identical to today's test/opt-in-driven fields.

---

## 4. Future implementation slice (not implemented here)

A future PR **may**: add an opt-in sparse-residency nested-mapping runtime for `SCENARIO-0080-1`;
materialize/evict theater cells deterministically; support bounded descent/ascent between
starmap/starsystem/planet; preserve I8 parity; record deterministic per-step residency reports.

It **must not**: wire the starmap into the **default** session pass graph at session open (Tier-2); add a
real-time loop; add a global mapping scheduler; introduce semantic/raw WGSL or a semantically-named
shader; alter field *values* via residency; exceed the bounded scenario; reopen the (closed) atlas
designer surface as if unsettled; or implement ClauseThing.

**WGSL discipline.** Any residency/eviction kernel work stays semantic-free straight-line WGSL over the
existing kernel substrate (invariants row 169/194); semantic map/faction concepts never enter shader
text. New semantic shader = stop-and-escalate.

---

## 5. Future required tests (named, not implemented)

- `atlas_0080_0_explicit_opt_in_only`
- `atlas_0080_0_default_session_has_no_residency_runtime`
- `atlas_0080_0_sparse_residency_only_active_theaters`
- `atlas_0080_0_nested_descent_ascent_deterministic`
- `atlas_0080_0_residency_is_value_noop_parity_bit_exact`
- `atlas_0080_0_residency_within_vram_budget`
- `atlas_0080_0_replay_deterministic`
- `atlas_0080_0_no_default_session_pass_graph_wiring`
- `atlas_0080_0_no_realtime_loop`
- `atlas_0080_0_no_global_mapping_scheduler`
- `atlas_0080_0_no_semantic_or_raw_wgsl`
- `atlas_0080_0_no_clausething_dependency`
- `atlas_0080_0_docs_status_matches_gate`

---

## 6. Stop conditions

Stop if the gate would require: default session pass-graph wiring at session open (Tier-2); a real-time
loop; a global mapping scheduler; residency that alters field values; semantic/raw WGSL or a
semantically-named shader; CPU planner; unbounded residency / breaking the VRAM budget posture into a
hard cap; reopening the closed atlas designer surface; ClauseThing; `simthing-spec` alteration for
ClauseThing; invariant edits; or a general mapping runtime beyond this scenario.

---

## 7. Exit criteria (this opening PR)

- [x] Opening spec exists; `ATLAS-0080-0` marked OPEN as a docs/design gate.
- [x] Scope is `SCENARIO-0080-1` only; sparse-residency nested mapping; opt-in/default-off.
- [x] Future implementation slice named, not implemented.
- [x] Invariant *"first-slice gating"* satisfied by this named opening; no invariant edit.
- [x] No code changed.

---

## 8. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_scenario_0080_1_opening_review_results.md`](../tests/phase_scenario_0080_1_opening_review_results.md)
