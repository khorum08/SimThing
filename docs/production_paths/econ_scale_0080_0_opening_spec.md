# ECON-SCALE-0080-0 — Multi-Faction (Hybrid-Strata / Faction-Index) ECON Scaling Opening Spec

> **Status: OPENING SPEC / NO IMPLEMENTATION.**
> - Opened by **`SCENARIO-0080-1`** admitting the **pirate as a full economy faction** (product
>   decision, 2026-06-02), so the pirate entering a starsystem is an **adversarial participant in that
>   starsystem's resource flow**.
> - This is the previously-parked **Hybrid-Strata / faction-index ECON scaling**, opened by a named
>   consumer.
> - All wiring is **opt-in / default-off**. **This PR does not implement the scaling.**
> - **Implementation result (2026-06-02): IMPLEMENTED / PASS** — `ECON-SCALE-0080-0-IMPL-0`, 17 tests;
>   see [`../tests/phase_econ_scale_0080_0_impl_results.md`](../tests/phase_econ_scale_0080_0_impl_results.md).
>   Scope unchanged.

---

## 1. Why now

Local Patrol Economy deliberately kept the pirate a *disruptor identity, not a second economy owner*, to
avoid multi-faction contended economy — leaving faction-index ECON scaling parked *pending a consumer that
needs it*. `SCENARIO-0080-1` is that consumer: two factions (Terran, Pirate) genuinely contend in the same
starsystem's resource flow, which requires the economy substrate to index participation by faction.

---

## 2. Scope

- `SCENARIO-0080-1` only.
- A **bounded, fixed, small number of factions** (Terran + Pirate; design for `n` small, not unbounded).
- **Faction-indexed participation** in a starsystem's resource flow (the ECON clearinghouse becomes aware
  of which faction a participant belongs to), enabling **adversarial/contended** clearing within the
  existing subsidiarity model.
- The pirate's participation is **adversarial** (e.g. competing extraction/contention) — bounded, not a
  general market or trade engine.

---

## 3. ECON-scaling contract

- **Opt-in only**; default ECON path stays single-owner as today (no behavior change when disabled).
- **Bounded faction count**; faction index is a small fixed set, not unbounded fan-out.
- **CPU-oracle bit-exact parity (I8)** for the contended clearing; deterministic replay.
- **No hard currency, markets, trade, or `ai_budget`** — adversarial contention is modeled within the
  existing Resource Flow / subsidiarity clearinghouse, **not** a currency engine. (Player-order influence,
  if later, is a bounded *weighted overlay term*, not the currency mechanism — see scenario packet.)
- **No nested Resource Flow** beyond the established `FlatStarResourceFlow` posture unless a separate gate
  authorizes depth.
- **Subsidiarity preserved:** faction indexing layers onto the session-clearinghouse model; it does not
  replace it.
- **Reversible:** disabled ⇒ single-owner ECON identical to today.

---

## 4. Future implementation slice (not implemented here)

A future PR **may**: add opt-in faction-indexed participation to the ECON clearinghouse for
`SCENARIO-0080-1`; support a bounded, fixed faction set; clear contended/adversarial resource flow
deterministically with I8 parity; record deterministic per-step economy reports.

It **must not**: introduce hard currency / markets / trade / `ai_budget`; introduce nested Resource Flow
depth; generalize to unbounded factions; replace the subsidiarity clearinghouse; add a CPU planner; add
semantic/raw WGSL; or implement ClauseThing.

---

## 5. Future required tests (named, not implemented)

- `econ_scale_0080_0_explicit_opt_in_only`
- `econ_scale_0080_0_default_path_single_owner_unchanged`
- `econ_scale_0080_0_bounded_fixed_faction_count`
- `econ_scale_0080_0_faction_indexed_participation`
- `econ_scale_0080_0_adversarial_contended_clearing_deterministic`
- `econ_scale_0080_0_parity_bit_exact`
- `econ_scale_0080_0_replay_deterministic`
- `econ_scale_0080_0_no_hard_currency_markets_trade_aibudget`
- `econ_scale_0080_0_no_nested_resource_flow`
- `econ_scale_0080_0_subsidiarity_preserved`
- `econ_scale_0080_0_no_cpu_planner`
- `econ_scale_0080_0_no_semantic_or_raw_wgsl`
- `econ_scale_0080_0_no_clausething_dependency`
- `econ_scale_0080_0_docs_status_matches_gate`

---

## 6. Stop conditions

Stop if the gate would require: hard currency / markets / trade / `ai_budget`; nested Resource Flow depth;
unbounded faction fan-out; replacing the subsidiarity clearinghouse; a CPU planner / urgency / commitment;
semantic/raw WGSL; multi-faction *default-on* ECON (Tier-2); ClauseThing; `simthing-spec` alteration for
ClauseThing; invariant edits; or reopening any closed ladder.

---

## 7. Exit criteria (this opening PR)

- [x] Opening spec exists; `ECON-SCALE-0080-0` marked OPEN as a docs/design gate.
- [x] Scope is `SCENARIO-0080-1` only; bounded fixed faction set; opt-in/default-off.
- [x] Future implementation slice named, not implemented.
- [x] No hard currency / nested RF authorized; subsidiarity preserved.
- [x] No code changed; no invariant edit.

---

## 8. Pointers
- Scenario: [`../scenarios/scenario_0080_1_admission_packet.md`](../scenarios/scenario_0080_1_admission_packet.md)
- Production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Visibility report: [`../tests/phase_scenario_0080_1_opening_review_results.md`](../tests/phase_scenario_0080_1_opening_review_results.md)
