# Resource Flow Global/Default-On Readiness Re-Review

**Status:** Accepted (design/readiness review, docs-only)  
**Date:** 2026-05-19  
**Scope:** Post–RF-T3 audit for whether Resource Flow may move from explicit per-spec `FlatStarOptIn` toward **limited scenario-class default-on** (RF-T4) or **global default-on**  
**Authority:** [`resource_flow_default_on_readiness_review.md`](resource_flow_default_on_readiness_review.md) (prior review), [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md), [`accumulator_op_v2_production_plan.md`](../accumulator_op_v2_production_plan.md), RF-T1/T2/T3 gates (PRs #180, #181, #182), [`resource_flow_opt_in_product_soak_test_results.md`](../tests/resource_flow_opt_in_product_soak_test_results.md) (RF-T3 verification, inspected)

**Production code changes in this review:** **None.**

---

## Executive summary

Since the prior default-on readiness review (2026-05-19), the project completed **RF-T1** (authored scenario-class opt-in flagging), **RF-T2** (expanded opt-in burn-in), and **RF-T3** (product-like opt-in soak + telemetry). The narrow flat-star + static/dynamic enrollment slice is now **proven under explicit `FlatStarOptIn` opt-in** with observable flag-source attribution, 128/256-participant soak, multi-arena, dynamic fission cadence, replay determinism, and rejection telemetry.

**Recommendation: B — proceed to RF-T4 limited scenario-class default-on implementation.**

| Option | Verdict |
|--------|---------|
| **A.** Keep all Resource Flow execution explicit opt-in only | Safe baseline; acceptable if product defers scenario-class convenience |
| **B.** Proceed to RF-T4 limited scenario-class default-on | **Recommended next implementation gate** |
| **C.** Require additional RF-T3-style product soak before RF-T4 | **Not required** — RF-T3 closed the soak/telemetry gap identified in the prior review §5 |
| **D.** Approve global default-on readiness | **Rejected** — uncovered paths remain too broad |

**Global default-on remains unsafe.** `PipelineFlags::default().use_accumulator_resource_flow` must stay **false** until a future gate explicitly authorizes a global flip (not RF-T4). RF-T4 must **not** infer GPU execution from `ResourceFlowSpec` presence alone.

---

## 1. Current-state audit after RF-T3

| Layer | Post–RF-T3 posture | Default-on relevance |
|-------|-------------------|----------------------|
| **Runtime substrate** | AccumulatorOp v2; E-11 flat-star allocation via existing kernel | Unchanged; no new WGSL |
| **Global flag** | `PipelineFlags::default().use_accumulator_resource_flow == false` | RF-T4 does **not** change this |
| **Spec opt-in** | `ResourceFlowOptInMode::FlatStarOptIn` on `ResourceFlowSpec`; only mode that enables GPU via `open_from_spec` | Proven (RF-T1) |
| **Session application** | `SimSession::open_from_spec` → `apply_resource_flow_opt_in`; flat-star validation rejects wildcard (E-11B deferred) | Proven |
| **Flag-source attribution** | `ResourceFlowFlagSource`: `DefaultDisabled`, `SpecFlatStarOptIn`, `TestOverride` | Proven (RF-T3) |
| **Telemetry** | `ResourceFlowOptInTelemetryReport`: arenas, participants, ops, bands, generation, admissions/rejections, sync, max error, replay bit-exact | Proven (RF-T3) |
| **Static enrollment** | E-2B-1…4 selectors → explicit participants at install | Covered |
| **Dynamic enrollment** | E-2B-5 Policy A inherit + E-2B-5R atomic prepare/commit | Covered under opt-in soak |
| **Execution layout** | `build_flat_star_layout` only in production `build_execution_plan` | Nested D≥3 not production-wired |
| **simthing-sim** | Arena-ignorant; no `ArenaRegistry` / `ArenaParticipant` imports | Preserved |

**Constitutional posture preserved:** no WGSL changes; no new `AccumulatorRole` variants; no CPU production allocation fallback; no boundary-time slot compaction; no indirection-list SlotRange replacement; hard-currency transfer separate from Resource Flow; E-11B deferred by default; Policy B Reevaluate deferred.

---

## 2. Evidence gained since prior review

The prior review ([`resource_flow_default_on_readiness_review.md`](resource_flow_default_on_readiness_review.md)) recommended **B** (limited scenario-class readiness) and predicted an RF-T1…T5 ladder. Since then:

| Gate | Landed | Evidence |
|------|--------|----------|
| **RF-T1** | PR #180 | `ResourceFlowOptInMode { Disabled, FlatStarOptIn }` on `ResourceFlowSpec`; `open_from_spec` applies flag; wildcard rejected at flat-star validation; roundtrip + driver opt-in suites (13+2 tests) |
| **RF-T2** | PR #181 | Named burn-in fixtures via `open_from_spec` + `FlatStarOptIn`: static 10/64, skewed weights, dynamic single/multi fission, two-arena, disabled populated spec, wildcard reject, resync, replay (15 tests) |
| **RF-T3** | PR #182 | `ResourceFlowOptInTelemetryReport` + flag-source; product fixtures 128/256 static, dynamic fission cadence, multi-arena 1000-tick, multi-session replay, disabled diagnostics, rejection telemetry, repeated resync (6 + 13 tests); full regression green |

**What changed vs. prior review §5 (telemetry requirements):**

| Prior requirement | RF-T3 status |
|-------------------|--------------|
| Resource Flow sync report (arenas, ops, bands) | Surfaced in telemetry + burn-in reports |
| Dynamic enrollment report | Wired via `last_resource_flow_dynamic_enrollment_report` + telemetry |
| Flag source attribution | `ResourceFlowFlagSource` on session |
| Soak parity metrics (max_abs_error, replay_bit_exact) | Product soak + telemetry |
| Flat-star guard | `assert_flat_star_only_no_nested_claims` in RF-T2/T3 suites |

**What did not change:** global flag default; production nested GPU; Policy B; coupling-at-scale burn-in; wildcard product semantics.

---

## 3. Covered execution paths

| Path | Evidence | Opt-in required today |
|------|----------|----------------------|
| **`FlatStarOptIn` spec opt-in** | RF-T1 `resource_flow_opt_in`, roundtrip | Yes — `opt_in_mode: FlatStarOptIn` |
| **E-11 flat-star D=2 GPU allocation** | E-11 burn-in/soak + RF-T2/T3 product soak | Yes — flag true via opt-in |
| **Static E-2B enrollment** | `resource_flow_enrollment_compile`, `resource_flow_enrollment_session` | Compile always; GPU only when flag true |
| **E-2B-5 Policy A dynamic enrollment** | `e2b5_dynamic_fission_enrollment`, RF-T2/T3 dynamic fixtures | GPU sync when flag true |
| **E-2B-5R atomicity** | Rejection tests (cap, contiguity, registry); RF-T3 rejection telemetry | Proven |
| **Telemetry + flag-source** | RF-T3 telemetry suite | N/A (reporting) |
| **Disabled populated spec (no GPU)** | RF-T1/T2/T3 disabled fixtures | `opt_in_mode: Disabled` → flag false, spec staged |
| **Multi-arena flat-star (no coupling)** | RF-T2 two-arena, RF-T3 multi-arena 1000-tick | Opt-in |
| **Product-scale hosted counts** | RF-T3 128/256 participants, 1000 ticks | Opt-in; 256 uses finite-error tolerance |

---

## 4. Explicitly uncovered paths

| Path | Why uncovered | Blocks global default-on? | Blocks RF-T4? |
|------|---------------|---------------------------|---------------|
| **Global default-on** | Would enable GPU for all authored specs | **Yes — reject D** | N/A (RF-T4 avoids this) |
| **Inference from `ResourceFlowSpec` presence alone** | Spec stages registry/scaffold without execution intent | **Yes** | **Yes — RF-T4 must not do this** |
| **E-11B nested hierarchy GPU** | Production `build_execution_plan` flat-star only | Yes | Yes for nested scenarios |
| **Policy B Reevaluate selector re-run** | Runtime maps to inherit-only | Yes | Yes for Reevaluate-authored scenarios |
| **Wildcard / unbounded dynamic selector expansion at scale** | FlatStarOptIn rejects wildcard admission | Yes | Yes unless explicitly out of scope |
| **Coupling-heavy product graphs** | Two-arena no-coupling soaked; no coupling-delay burn-in at scale | Yes | Partial — RF-T4 should scope scenario classes without coupling until soaked |
| **Hard-currency via Resource Flow** | Constitutionally forbidden | Yes | N/A |
| **Gap-only nested fission allocation** | E-10R3 path; not Policy A flat-star | Yes | Yes |
| **Cross-session LDJSON replay of enrollment + GPU frames** | Partial replay determinism in soak; not full product replay pipeline | Moderate | Low for RF-T4 if scenario-class soak mirrors RF-T3 |
| **Bit-exact parity at 256+ participants** | RF-T3 accepts finite max_abs_error at scale | Moderate | Low — document tolerance in scenario class |

---

## 5. Telemetry and observability status

**Landed (RF-T3):**

- `ResourceFlowOptInTelemetryReport` — scenario name, opt_in_mode, flag_source, resource_flow_enabled, arenas/participants, total_ops, n_bands, generation start/end, dynamic admissions/rejections, sync_count, max_abs_error, replay_bit_exact
- `ResourceFlowFlagSource` — distinguishes default-disabled, spec opt-in, test override
- Product soak runners — `run_product_soak_with_telemetry`, `assert_telemetry_contract`

**Required before RF-T4 lands (implementation gate, not this review):**

| Telemetry | Purpose |
|-----------|---------|
| **`ScenarioClassDefaultOn` flag source** (or equivalent) | Distinguish RF-T4 auto-enable from spec `FlatStarOptIn` and test override |
| **Scenario-class attribution in telemetry** | Record which named scenario class / execution profile enabled GPU |
| **Negative path: populated spec + wrong/missing scenario class** | Assert flag false, GPU inactive, telemetry reports `DefaultDisabled` |
| **Regression carry-forward** | All RF-T1/T2/T3 + E-2B/E-11 suites remain green |

Prior review §5 telemetry gaps are **closed for explicit opt-in**. RF-T4 adds one new attribution dimension (scenario-class default-on source).

---

## 6. Default-on candidate definitions (re-affirmed)

| Candidate | Meaning | Post–RF-T3 verdict |
|-----------|---------|-------------------|
| **Global default-on (D)** | `PipelineFlags::default().use_accumulator_resource_flow == true` | **Rejected** — same rationale as prior review; uncovered paths unchanged |
| **RF-T4 limited scenario-class default-on (B)** | Named scenario class or authored **execution profile** auto-applies `FlatStarOptIn` (or equivalent flag enablement) at session open **without** author writing `opt_in_mode` on every spec | **Approved to implement** — narrow, explicit, mirrors Phase T scenario posture |
| **Opt-in-by-spec (`FlatStarOptIn`)** | Author sets `ResourceFlowOptInMode::FlatStarOptIn` on `ResourceFlowSpec` | **Landed (RF-T1)** — remains valid; RF-T4 is additive convenience |
| **Inference from spec presence** | GPU on whenever `resource_flow: Some(...)` | **Forbidden** — disabled populated spec proves staging ≠ execution |

**RF-T4 is not global default-on.** It is a **third enablement path**: scenario class declares execution intent; session open maps class → flag true + flat-star guard; spec may remain `opt_in_mode: Disabled` or omit explicit opt-in if scenario class profile says otherwise — **design detail for RF-T4 impl PR**, but must never flip `PipelineFlags::default()`.

---

## 7. Recommendation

**Chosen: B — proceed to RF-T4 limited scenario-class default-on implementation.**

- **Reject A** as unnecessary regression — RF-T1…T3 prove opt-in; product needs scenario-class convenience without global risk.
- **Reject C** — RF-T3 product soak (128/256, dynamic cadence, multi-arena, replay, rejection, resync) satisfies the prior review's pre-default-on soak intent for the flat-star slice.
- **Reject D** — global default-on still unsafe; E-11B, Policy B, couplings, wildcards, and spec-presence ambiguity remain.

**Is RF-T4 limited scenario-class default-on now safe?** **Yes**, under these constraints:

1. No `PipelineFlags::default()` change.
2. No GPU enablement from `ResourceFlowSpec` presence alone.
3. Scenario class / execution profile must **explicitly** declare flat-star Resource Flow execution.
4. Scope limited to burned-in topology: flat-star D=2, static E-2B, E-2B-5 Policy A, E-2B-5R.
5. New flag-source attribution for scenario-class enablement.
6. No new WGSL, roles, CPU fallback, simthing-sim awareness, E-11B, Policy B.

**Is global default-on still unsafe?** **Yes.**

---

## 8. RF-T4 candidate implementation ladder

**Not in this PR.** Recommended sequence after this re-review:

| Step | Scope | Notes |
|------|-------|-------|
| **RF-T4a** | Scenario-class / execution-profile type | e.g. `ResourceFlowScenarioClass` or `ResourceFlowExecutionProfile` on `Scenario` metadata or `GameModeSpec` — lists allowed topology (flat-star only) |
| **RF-T4b** | Session open mapping | `SimSession::open_from_spec` (or scenario open hook) applies flag when scenario class profile declares flat-star execution — **orthogonal to spec presence** |
| **RF-T4c** | Flag-source extension | Add `ScenarioClassDefaultOn` (or similar) to `ResourceFlowFlagSource`; telemetry records class name |
| **RF-T4d** | Guardrails | Reject scenario class + wildcard/nested claims; reject class + incompatible fission policy; flat-star-only assert |
| **RF-T4e** | Burn-in + regression | Mirror RF-T3 fixtures opened via scenario class instead of spec `FlatStarOptIn`; disabled populated spec + wrong class stays inactive |
| **RF-T4f** | Docs + re-review trigger | Update production plan; optional RF-T5 global re-review only if product requests global flip |

**Explicit RF-T4 non-goals:** global `PipelineFlags` default change; inferring from `ResourceFlowSpec` presence; new `ResourceFlowOptInMode` variants (e.g. `NestedOptIn`); E-11B; Policy B; WGSL; new roles; CPU fallback; simthing-sim arena imports; hard-currency via Resource Flow; slot compaction; indirection lists.

---

## 9. Required tests before RF-T4 implementation

| Test obligation | Target |
|-----------------|--------|
| Global flag default false | `PipelineFlags::default().use_accumulator_resource_flow == false` |
| Scenario class enables GPU | Named class profile → flag true, ops > 0, `ScenarioClassDefaultOn` attribution |
| Populated spec + no matching class | Registry staged, GPU inactive, `DefaultDisabled` |
| Populated spec + explicit `Disabled` opt-in + matching class | Define precedence (recommend: scenario class wins only when profile explicitly declares execution; spec `Disabled` wins if product requires author veto — **must be decided in RF-T4 ADR snippet**) |
| Flat-star-only guard | No nested GPU claims |
| Dynamic admission + resync under class default-on | Generation bump, sync count stable across resync |
| Replay determinism | Same seed → same telemetry + parity |
| Rejection atomicity | Cap-full rejection visible in telemetry |
| Transfer/emission flags unchanged | Resource Flow class does not enable transfer/emission |
| simthing-sim arena-ignorant | No new imports |
| No new WGSL | Static string guards in tests |
| Regression | RF-T1/T2/T3, E-2B, E-11, e2b5 soak suites green |

---

## 10. Stop conditions / Opus escalation triggers

Stop RF-T4 and escalate to Opus if implementation appears to require:

- Global `PipelineFlags::default().use_accumulator_resource_flow = true`
- GPU execution inferred merely from `ResourceFlowSpec` presence
- New WGSL or new `AccumulatorRole` variants
- CPU production allocation fallback
- `simthing-sim` Resource Flow / arena awareness
- Boundary-time slot compaction or indirection-list SlotRange replacement
- E-11B nested hierarchy GPU
- Policy B Reevaluate selector re-run
- Hard-currency transfer via Resource Flow
- Coupling-heavy or wildcard product semantics not yet designed
- Scenario-class default-on without flag-source attribution or negative-path tests

---

## 11. Docs update requirements

This review updates:

- `accumulator_op_v2_production_plan.md` — re-review status; next gate RF-T4
- `todo.md` — pivot next gate per recommendation B
- `worklog.md` — re-review landed entry
- `workshop_current_state.md` — re-review + preserved default false

---

## 12. Review question index

| # | Question | Answer |
|---|----------|--------|
| 1 | What evidence changed since the prior default-on readiness review? | RF-T1/T2/T3 landed: spec opt-in, expanded burn-in, product soak + telemetry; prior §5 telemetry gaps closed for opt-in slice |
| 2 | Did RF-T1 prove authored scenario-class opt-in? | **Yes** — `ResourceFlowOptInMode::FlatStarOptIn` on spec; `open_from_spec` applies flag; wildcard rejected |
| 3 | Did RF-T2 prove expanded opt-in burn-in? | **Yes** — 15 fixtures: static, dynamic, two-arena, disabled, resync, replay |
| 4 | Did RF-T3 prove product-like opt-in soak and telemetry? | **Yes** — 128/256 static, dynamic cadence, multi-arena, replay, rejection, resync; flag-source + telemetry report |
| 5 | Which paths are now covered? | FlatStarOptIn, E-11 flat-star D=2, static E-2B, E-2B-5 Policy A, E-2B-5R atomicity, telemetry/flag-source |
| 6 | Which paths remain uncovered? | Global default-on, E-11B, Policy B, wildcard/selector at scale, coupling-heavy graphs, hard-currency via RF (forbidden), spec-presence inference |
| 7 | Is RF-T4 limited scenario-class default-on now safe? | **Yes**, under §8 constraints — not global, not spec-presence inference |
| 8 | Is global default-on still unsafe? | **Yes** |
| 9 | What exact RF-T4 ladder should follow? | §8 — RF-T4a…f: scenario class type, session mapping, flag source, guards, burn-in, docs |
| 10 | What additional tests before RF-T4? | §9 table |

---

## References

- Prior review: [`resource_flow_default_on_readiness_review.md`](resource_flow_default_on_readiness_review.md)
- Substrate ADR: [`resource_flow_substrate.md`](../adr/resource_flow_substrate.md)
- RF-T3 verification: [`resource_flow_opt_in_product_soak_test_results.md`](../tests/resource_flow_opt_in_product_soak_test_results.md) (inspected; artifact retired in this PR)
- RF-T3 code: `resource_flow_opt_in_telemetry.rs`, `resource_flow_opt_in_product_soak.rs`
- Merged PRs: #180 (RF-T1), #181 (RF-T2), #182 (RF-T3)
