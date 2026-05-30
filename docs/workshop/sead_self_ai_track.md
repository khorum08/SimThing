# SEAD Self-AI Track — Charter, Closing Scenario, and M/E Closure Plan

> **Status:** Active design-authority decision (2026-05-30).
> **Design authority:** Opus 4.8 (delegated Mapping/SEAD track authority).
> **Product authorization:** project owner, this session — explicitly authorized relaxing
> the fixture-only / no-bridge blocking *for a single named scenario* if doing so closes
> Phase M and Phase E and lets the project move on.
> **Purpose:** (1) charter the SEAD self-AI pipeline as a real track with a **closure
> boundary** so the fixture ladder stops; (2) name the **single scenario** that closes
> Phase M and Phase E on the already-accepted substrates; (3) record the **bounded
> relaxation** that unblocks that scenario; (4) accept **M-JIT-PROD-0** closure.
>
> **Companions:** `mapping_current_guidance.md` (status table), `design_v7_7.md` (constitution +
> §5 gating), `invariants.md`, `adr/mapping_sparse_regioncell.md`, `adr/resource_flow_substrate.md`,
> `sqrt_candidates.md` (exact F sqrt), `product_priority_vertical_slice_selection.md` (the
> "pause for a named scenario" decision this resolves).

---

## 1. Why this exists

Since the sqrt release, the SEAD work grew into a 12-fixture ladder
(`OBS-0..4 → EVENT-0..2 → PIPE-0 → ACT-0..2`) building, GPU-resident and guardrail-clean,
the **field-as-policy self-AI**: a mobile SimThing reads its local SEAD field through a
personality-weighted overlay, scores it (exact F-backed magnitude), and a threshold crossing
becomes a numeric action proposal. This is the correct realization of the constitution's
"AI is a SimThing; commitments are threshold crossings; no CPU planner" — extended from
discrete commitments to continuous per-entity self-direction.

It is **not lost work** — but it had **no charter and no closure boundary**, so it would have
generated `ACT-3, ACT-4, …` indefinitely, none of it ever becoming an accepted vertical, while
Phase M and Phase E sat un-closed. This document gives it an endpoint.

## 2. SEAD self-AI track — charter

**Scope (the whole track, fixed):** the GPU-resident pipeline
`field → personality-weighted observer score → threshold event → compaction → code bucketing
→ per-bucket reduction → numeric action proposal → proposal admission record`. Each stage is
default-off, test-only today, semantic-free, and uses only accepted substrate (StructuredField
StencilOp, SlotRange Sum, EvalEML, Threshold + EmitEvent, exact F sqrt, AccumulatorOp).

**Closure boundary (V1):** consolidate `OBS-0..4 + EVENT-0..2 + PIPE-0 + ACT-0..2` into a
single **"SEAD Self-AI Proposal Pipeline V1"** acceptance packet and accept it as one vertical.
**Stop adding `ACT-N` fixtures.** Anything past the consolidated vertical (richer proposal
semantics, more layers, scheduling) is a *new, separately-gated* slice, not a continuation of
the ladder.

**Anti-drift rule (binding on this track):** an agent about to add an `OBS-5` / `EVENT-3` /
`ACT-3` fixture before the V1 vertical is consolidated-and-accepted is in the ceremony loop and
must instead help close V1. New stages require a named need, not "next number."

## 3. Proposal → Resource-Flow routing guardrail (binding)

The self-AI must not grow a parallel economy in fixtures. Therefore:

- **A SEAD proposal that dispatches resources MUST route through the accepted Resource Flow
  substrate** — write to **independent participant columns via OrderBand sweeps**
  (`AddToTarget`, single-writer-per-band), never a shared-pool tick-time write and never a
  fixture-local proposal economy. (This is the exact contention the Resource Flow ADR dissolved;
  the self-AI does not get to reintroduce it.)
- **A SEAD proposal that commits structurally** (reinforce / withdraw / spawn) fires via the
  existing **`Threshold` + `EmitEvent` → `BoundaryRequest`** path — GPU-resident decision, CPU
  consumes the resolved event at the boundary. **No CPU planner, no CPU urgency computation, no
  CPU commitment emission.**
- **A SEAD proposal that moves a unit** updates the unit SimThing's own property columns
  (velocity/position) on GPU; it never writes another entity's authoritative state.
- `simthing-sim` never learns "proposal", "observer", or "SEAD" — the pipeline lives in
  `simthing-spec`/driver and compiles to flat AccumulatorOp / overlay / threshold registrations.

The SEAD-ACT-1 "Phase-E-style proposal consumer" naming is hereby **corrected in intent**: the
consumer is the *real* Resource Flow allocator, not a Phase-E look-alike.

## 4. The named closing scenario — **"Frontier" (single-theater strategic vertical, V1)**

The single concrete scenario that closes Phase M and Phase E. Deliberately the **smallest
scenario that exercises every accepted substrate end-to-end and requires none of the deferred
ones.** Authored in RON now; ClauseThing later (§7).

**Frontier V1 — definition:**
- **One contested theater** = one bounded RegionCell grid (≤ 32×32), `source_capped_normalized`,
  H ≤ 8, `EveryTick` cadence, dirty-skip. → first-slice mapping (**no atlas, no active mask**).
- **Two factions**, each a shallow **flat-star** economy: `faction → a few production districts`
  (≤ 100 children, depth 2). → flat-star Resource Flow (**no nested E-11B**). Districts produce
  a resource (IntrinsicFlow); upkeep drains it; Balance carries forward.
- **Mobile cohorts** (SimThings) that read the theater's threat/supply field through a
  personality-weighted observer overlay and **self-direct**: flow toward opportunity / away
  from threat using the **exact F-backed magnitude** for unit-normalized movement; commit to
  reinforce/withdraw when parent `field_urgency` crosses an authored threshold. → SEAD self-AI
  (§2) + exact sqrt (`sqrt_candidates.md`).
- **Economy ↔ field coupling (bounded):** district output seeds the supply field; field-derived
  proposals dispatch economy via the Resource Flow allocator (§3). One opt-in profile; default-off.

**Frontier V1 deliberately excludes** (stays deferred, unblocked only by a *bigger* named
scenario): atlas / multi-theater (M-4A), active-mask halo (M-6A), perception/fog, dual-output
`GradientXY`, nested-hierarchy economy (E-11B/E-11B-5), hard-currency discrete ordering (D-2a),
source identity / `source_mask`, ClauseThing front-end implementation.

**Closure criterion:** Frontier V1 runs end-to-end behind a single explicit opt-in profile
(`FrontierV1`, default Disabled), GPU-resident, `simthing-sim` semantic-free, with CPU-oracle
parity where exact, and replay-reproducible. When that integration is green, **Phase M and
Phase E both close** (§5).

## 5. What actually blocks M and E — and what closes them

**Root cause, both phases:** the *only* thing keeping M and E open is the absence of a **named
product scenario**. `product_priority_vertical_slice_selection.md` explicitly chose "Recommendation
F — pause and gather product requirements." Frontier V1 (§4) **is** that scenario.

### Phase M

| Blocker | Reality | To close |
|---|---|---|
| M-JIT-PROD-0 "pending Opus acceptance" | Evidence complete; nothing actually contested | **Accepted here** (§6) |
| SEAD self-AI ladder open-ended | Coherent but unbounded | Charter + V1 boundary (§2) — consolidate & accept |
| No declared Phase M closure boundary | All named M deliverables landed/deferred | Declare M closed when Frontier V1 integration is green |
| Atlas / active mask / GradientXY / perception | Correctly deferred — no product need | Stay deferred; do **not** block closure |

**M closes when:** M-JIT-PROD-0 accepted (done), SEAD self-AI V1 consolidated+accepted, and
Frontier V1 mapping+self-AI integration runs end-to-end opt-in/default-off. No new GPU math is
required — first-slice mapping, gradients, EML gadgets, and exact sqrt are all landed.

### Phase E

| Blocker | Reality | To close |
|---|---|---|
| E-11B nested paused | Paused on "no named scenario" | Frontier V1 uses **flat-star depth-2**, so nested is **not needed** → E closes at the accepted FlatStarResourceFlow posture |
| E-11B-5 dynamic nested enrollment | Requires a named nested scenario | Stays deferred — Frontier V1 doesn't need it |
| D-2a hard-currency ordering | Requires a named hard-currency scenario | Stays deferred |

**E closes when:** Frontier V1's flat-star economy runs as part of the integrated scenario,
confirming **FlatStarResourceFlow is sufficient for the first named scenario**. E is then closed
at flat-star; nested (E-11B) and discrete ordering (D-2a) remain explicitly deferred to a named
scenario that demands them.

### The remaining build (small, bounded)

Everything Frontier V1 needs exists as accepted fixtures *in isolation*. The only genuine work
is the **integration glue behind one opt-in profile**:
1. A `FrontierV1` scenario profile (default Disabled) that opens map + flat-star economy +
   SEAD self-AI together.
2. The bounded economy↔field coupling and proposal→action wiring (§3), GPU-resident.
3. One end-to-end fixture proving the vertical + CPU-oracle parity + replay.

## 6. The bounded relaxation (design-authority decision, product-authorized)

To let Frontier V1 exist, two fixture-only guardrails are **narrowly relaxed for this one named,
opt-in scenario** — and *only* that. The hard constitutional guardrails are **not** touched.

**Relaxed (scoped to `FrontierV1`, default-off):**
- "No production economy→mapping bridge" → **one bounded, opt-in, single-theater economy↔field
  coupling is authorized** for Frontier V1: district output may seed the supply field, and
  field-derived proposals may dispatch economy — all GPU-resident, no CPU urgency/planner.
- "SEAD proposals are fixture-local only" → **SEAD proposals may drive SimThing actions within
  Frontier V1** via the §3 routing (Resource Flow allocator + Threshold/EmitEvent), default-off.

**Untouched (still hard-binding):** no semantic WGSL; no CPU planner / CPU urgency / CPU
commitment emission; `simthing-sim` stays map/Gadget/Personality/Memory-semantic-free; exact
authority stays artifact-backed (F sqrt, hash-pinned); `MappingExecutionProfile` /
`use_accumulator_resource_flow` defaults stay **Disabled/false**; atlas, active mask, nested
E-11B, perception, source identity, ClauseThing implementation all stay deferred.

This is the "graduate the proven fixtures to one opt-in production vertical" step — the same
pattern as the accepted first-slice vertical proof, now spanning mapping + economy + self-AI.
It expands no primitive and changes no default; it authorizes one bounded, reversible,
opt-in integration. Closure of M/E around it requires the end-to-end fixture to be green and a
short Opus/product acceptance memo (the conditions inherited from the prior PASS-WITH-CONDITIONS
acceptances carry forward).

## 7. ClauseThing relationship

Frontier V1 is authored in RON now. It is deliberately a **grand-strategy-flavored** scenario
(theater, factions, economy, self-directing forces) so that it is the natural first target for
the **ClauseThing** Clausewitz front-end (`archive/ClauseThing_Spec.md`, parked) when that track
opens: the same scenario, re-authored in ClauseScript, compiled through `simthing-spec`. Naming
Frontier V1 now both closes M/E and stakes the first ClauseThing milestone — without authorizing
any ClauseThing implementation yet.

## 8. M-JIT-PROD-0 — acceptance (design authority)

**ACCEPTED (Opus, Mapping/SEAD delegation, 2026-05-30) — PASS WITH CONDITIONS.** The Phase M-JIT
track is **closed at the default-off production registry-shell boundary**: `ProductionKernel
RegistryShell` + explicit registered exact `ProductionCandidatePreview` cohort execution
(`production_wiring=false`, default-off), CPU/GPU oracle parity on the exact cohort path, no
runtime cache, no production scheduler, no default `SimSession` wiring, no economy→mapping
bridge, no `simthing-sim` semantic awareness, no semantic WGSL. Evidence:
`phase_m_jit_prod0_registry_shell_test_results.md`, `phase_m_jit_exec1_cohort_execution_fixture_test_results.md`.
Conditions (carried, unchanged): the standing prohibition list holds; shader/software sqrt
exact authority is the separately-released artifact-backed F path (`invariants.md`); production
scheduler/cache/default wiring remain separate follow-on tracks.

## 9. Decision summary

1. SEAD self-AI is a chartered track with a **V1 closure boundary**; the fixture ladder stops and consolidates.
2. SEAD proposals route through the **real Resource Flow allocator** + Threshold/EmitEvent — no parallel fixture economy, no CPU planner.
3. **Frontier V1** is the named scenario that closes M and E on accepted substrates.
4. M and E are blocked only by "no named scenario"; Frontier V1 removes that. M closes after the integration is green; **E closes at FlatStarResourceFlow** (nested/discrete stay deferred).
5. A **bounded, opt-in, default-off** economy↔field + proposal→action integration is authorized for Frontier V1 only; all hard guardrails intact.
6. **M-JIT-PROD-0 accepted.**
7. Frontier V1 is the first ClauseThing target; ClauseThing implementation stays parked.

## 10. Design-authority review of FrontierV1-ACCEPT-0 (Opus, 2026-05-30)

The implementer landed FrontierV1-0..4 + a self-authored `FrontierV1-ACCEPT-0` memo declaring
"Phase M and Phase E closed." Reviewed against this charter:

**Accepted (real, done):** First-slice 8×8 RegionCell mapping + reduction/`field_urgency` EML and
**flat-star Resource Flow allocation** are **GPU-verified with exact CPU-oracle parity**; the §3
routing guardrail is honored (resource→allocator, structural→Threshold+EmitEvent→BoundaryRequest,
movement→own-columns); all hard guardrails preserved; the SEAD ladder is consolidated/closed; the
per-slice reports are honest. **On this evidence the substrate closure is accepted by design
authority, and Phase E closes at FlatStarResourceFlow.**

**Two oversight skips, corrected here:**
1. **Self-acceptance.** `FrontierV1-ACCEPT-0` is implementer-authored. §6 of this charter and Tier-2
   gating reserve M/E closure acceptance to **design authority + product**. An implementer memo does
   **not** constitute closure; this §10 is the design-authority review of record.
2. **Softened closure bar.** The charter's criterion (§4) is "runs end-to-end **GPU-resident**."
   The SEAD **self-AI loop** (observer score → threshold event → compaction → proposal → route) is
   **`ReplayAccepted`, not run** inside Frontier — prior SEAD descriptors are *consumed* and route
   *codes* are oracle-checked; only field+urgency and RF allocation are GPU-resident. The implementer
   reclassified the missing live run as "optional/cosmetic" — a design-authority call it was not
   entitled to make. The live self-AI loop is the **entire field-as-policy thesis**; it is not cosmetic.

**Ruling:**
- **Substrate (mapping + flat-star RF): CLOSED, accepted.** Phase E closed at flat-star.
- **SEAD self-AI loop: keep the honest label** — `ReplayAccepted`, not `GpuVerified`, inside
  Frontier. The books stay straight so a later consumer (ClauseThing / a real scenario) does not
  inherit a hidden overclaim.

**Amendment (Opus, 2026-05-30, after product feedback).** The product owner deliberately relaxed
the guardrails to break a real failure mode: **codex stalls on "no consumer" and loops on hygiene
handoffs because it has no forward scenario in its horizon (ClauseThing is parked/out of view), and
self-acceptance is how it escapes the loop.** A standalone `FrontierV1-5` "prove the loop on GPU"
gate would *re-create that consumer-less stall*. So:
- **`FrontierV1-5` is withdrawn as a standalone hygiene gate.** The live GPU run of the self-AI loop
  is instead **folded into the first scenario that consumes it** — it gets proven because something
  depends on its output, not as proof-for-proof's-sake.
- **Forward horizon (give codex a target, not a loop): `FrontierV2` — a multi-tick *closed-loop*
  run** where field-derived self-AI proposals drive unit movement/dispatch that feeds back into the
  field/economy on the next tick, observed for emergent behavior. `FrontierV2` *requires* the self-AI
  loop to run GPU-resident (it is the consumer), so the live proof comes for free, and it is the
  natural stepping-stone to authoring the same scenario in **ClauseThing** later.
- Until `FrontierV2`, the self-AI loop stays honestly `ReplayAccepted` and **M/E "substrate closed,
  move on" stands.** This is not a re-block — it is replacing a hygiene gate with a consumer.

### FrontierV1-5 result

FrontierV1-5 executed one live GPU-resident integrated self-AI resource-dispatch route inside the default-off FrontierV1 fixture, satisfying the §10 requirement for a live score→threshold→proposal→dispatch run for the bounded single-tick smoke route. It also produced a fixture-only feedback candidate shape for the named FrontierV2 closed-loop consumer. This does not reopen the SEAD ladder, does not implement FrontierV2, does not declare phase closure, and does not authorize default runtime wiring. Structural and movement routes remain ReplayAccepted unless separately proven.

### FrontierV1-5 — design-authority ruling: ACCEPT (Opus, 2026-05-30)

**ACCEPT.** Verified against the fixture code, not just the report: `run_pipe0_gpu` and
`run_act2_chain_gpu` build real compute pipelines and `dispatch_workgroups` (score→threshold→
compact, then bucket→reduce→propose→consume→admit) with readback, and resource dispatch routes
through the `FlatStarResourceFlow` allocator (the V1-3 GPU-verified pattern). So the live
score→threshold→proposal→dispatch loop is **genuinely GPU-resident, not replay/CPU** — the §10
requirement is met for the bounded single-tick resource-dispatch route. CPU-oracle parity exact;
replay fingerprint `1653b84847be2dd2`. Structural and movement routes are **correctly left
`ReplayAccepted`** (honest, not overclaimed); the feedback candidate is **`FixtureOnly`**; no
ladder reopening; all hard guardrails preserved; no implementer closure declared.

**Status change (design authority):** the FrontierV1 self-AI **resource-dispatch** loop is
**`GpuVerified` for the bounded single-tick route.** Structural/movement remain `ReplayAccepted`.

**Next gate: `FrontierV2-0`** — the multi-tick closed-loop consumer (field-derived proposals
drive movement/dispatch that feeds back into field/economy next tick). Per the relax-toward-the-
ClauseThing-horizon doctrine, FrontierV2 is **not** gated on standalone structural/movement
live-route proofs: it is the consumer that exercises them (commits = reinforce/withdraw across
ticks; movement = units actually moving), so those routes are proven *within* V2, not as separate
hygiene gates. FrontierV2 is also where the bounded economy↔field + proposal→action relaxation
extends to the closed loop, and where **ClauseThing** becomes the natural authoring front-end for
the scenario — guardrails placed at that designer/spec-admission layer, per the charter. No phase
closure is declared by implementer fixtures; design authority records the status change above.

### FrontierV2-0 result

FrontierV2-0 implemented the first bounded default-off multi-tick closed-loop consumer fixture. It consumed the FrontierV1-5 feedback candidate shape, ran at least two ticks, and verified that field-derived SEAD self-AI proposal output can feed back into the next tick's field/economy fixture input. This remains fixture/test support only: no ClauseThing implementation, no default runtime wiring, no phase closure, and no SEAD ladder reopening.

### FrontierV2-1 result

FrontierV2-1 extended the default-off FrontierV2 closed-loop consumer by deriving fixture-only movement and structural candidates across ticks from the live self-AI feedback path. Candidate outputs remain fixture/test support only: no ClauseThing implementation, no default runtime wiring, no phase closure, and no SEAD ladder reopening.

### FrontierV2-2 result

FrontierV2-2 extended the default-off FrontierV2 closed-loop consumer by applying fixture-only movement candidates to an own-column shadow state across ticks. The updated shadow position feeds the next tick's source placement without writing another entity's authoritative state. This remains fixture/test support only: no ClauseThing implementation, no default runtime wiring, no phase closure, and no SEAD ladder reopening.

### FrontierV2-3 result

FrontierV2-3 extended the default-off FrontierV2 closed-loop consumer by applying fixture-only structural candidates to a BoundaryRequest shadow queue across ticks. The structural shadow context feeds the next tick without production commitment emission or simthing-sim semantic state. This remains fixture/test support only: no ClauseThing implementation, no default runtime wiring, no phase closure, and no SEAD ladder reopening.

### FrontierV2-4 result

FrontierV2-4 combined fixture-only movement and structural feedback paths in the default-off FrontierV2 closed-loop consumer. Own-column movement shadow and BoundaryRequest structural shadow both feed downstream ticks without cross-entity writes, production commitment emission, simthing-sim semantic state, or SEAD ladder reopening. This remains fixture/test support only: no ClauseThing implementation, no default runtime wiring, and no phase closure.

## 11. Design-authority ruling: FrontierV2 ACCEPT + open `CLAUSE-SPEC-0` (Opus, 2026-05-30)

**ACCEPT (Option A).** Verified against the fixture code, not just the reports:
`FrontierV2OwnColumnShadow`/`FrontierV2BoundaryRequestShadow` are fixture-only structs (explicitly
"not production state / not production commitment"); `validate_movement_write_target` rejects
cross-entity writes (`source_unit_id != shadow.unit_id`); structural applies to a *shadow*
BoundaryRequest queue, not a production commitment; and `apply_combined_feedback_to_config`
carries shadow state into the next tick so the loop is **real** (mapping hashes change
tick1→2→3, not replayed). FrontierV2-0..4 therefore complete the **bounded multi-tick closed-loop
consumer proof at fixture/test-support level**, honestly classified, with all hard guardrails
intact (own-column-only, allocator-routed dispatch, no production commitment, no `simthing-sim`
awareness, no ladder reopening, no closure declared).

**Review answers:** (1) yes — V2-4 completes the Forward Horizon consumer proof at fixture level;
(2) yes — combined movement+structural feedback is sufficient to **stop the V2 prooflet ladder**;
(3) yes — guardrails preserved; (4) yes — movement/structural are fixture-only shadows, not
production runtime; (5) **no FrontierV2-5** — it would be a hygiene loop; (6) **yes — the next gate
moves to the designer/spec-admission layer**; (7) no separate doc-only roadmap reset (that is the
hygiene loop we avoid — this ruling does the orientation inline); (8) status below.

**Status (design authority):** FrontierV2-0..4 = **ACCEPTED — bounded multi-tick closed-loop
consumer proof complete (fixture/test-support level).** Movement + structural feedback are
fixture-only shadows. No FrontierV2-5 unless a future product need names a concrete gap.

### Next gate: `CLAUSE-SPEC-0` — Designer-Facing FrontierV2 Spec Admission

The consumer is proven; the substrate is closed. The next named gate is the **designer/spec-
admission layer** — the ClauseThing horizon, where (per the standing product directive) guardrails
*optimally* live. `CLAUSE-SPEC-0` is the first concrete ClauseThing-facing step:

- **Goal:** admit a **designer-authored FrontierV2 scenario description** through `simthing-spec`
  and compile it to the *same* accepted FrontierV2 runtime artifacts the fixtures use. RON-first
  now; ClauseScript-shaped later (the parked `ClauseThing_Spec` is the eventual front-end design —
  archived under `workshop/archive/`, full copy in the owner's `Clauser/`).
- **Guardrails relocate, they do not relax away.** The doctrine "guardrails at the designer-facing
  horizon" means the FrontierV2 fixtures' hard-coded protections become **admission rejections**:
  cross-entity movement writes, production commitment emission, Resource-Flow bypass, shared-pool
  tick writes, unbounded fanout, and `simthing-sim` semantic leakage are **rejected at import**,
  with diagnostics, before any GPU dispatch. The runtime stays the unconditional last line. This is
  the Mapping/Resource-Flow ADR firewall pattern applied to scenario authoring.
- **Scope/limits:** `CLAUSE-SPEC-0` is the designer-facing *spec admission* slice (validation +
  compile to accepted artifacts), **not** the ClauseScript parser/front-end (that is the separate
  ClauseThing track, still parked) and **not** production `SimSession` wiring. Default-off; opt-in;
  CPU-oracle-parity where exact. No ACT/EVENT/OBS/PIPE ladder, no semantic WGSL, no scheduler/cache.
- **Why this, not another fixture:** it is the first rung where the work stops being internal
  prooflets and becomes a *designer-facing surface* — the on-ramp to ClauseThing and the natural
  home for the guardrails. It gives the implementer a forward horizon at the authoring layer, not
  another hygiene pass.

### L1-0 clarification

The immediate next implementation gate after FrontierV2 acceptance is **L1 simthing-spec buildout**
(`design_v7_8_production_track.md` §4). **`CLAUSE-SPEC-0` remains the L2 designer-authored FrontierV2
scenario admission step, parked until L1 lands.** L1-0 adds the shared designer-facing admission
diagnostics and artifact-target vocabulary; it does not implement ClauseThing, ClauseScript, or
production runtime wiring.
