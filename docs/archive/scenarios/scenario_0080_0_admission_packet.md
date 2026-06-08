# SCENARIO-0080-0 — Local Patrol Economy Admission Packet

> **Status: ACCEPTED (SCENARIO-0080-0-ACCEPTANCE-0, design authority + product, 2026-06-02)** — with a
> design-authority enrichment: the patrol relocate/patrol decision is sourced from the **accepted
> GPU-resident FIELD_POLICY posture** (`Threshold`+`EmitEvent`→`BoundaryRequest`), not an externally-scripted
> `move_request` and not a CPU planner — so the scenario exercises **FIELD_POLICY + Ownership + Flow** together.
> This pulls no new substrate (FIELD_POLICY V1 is an accepted decision mechanism; mobility/transfer remains the
> single substrate wired). `PRODUCTION-PATH-0080-0` is now OPEN, scoped to this scenario on the 0.0.7.9
> mobility/transfer substrate. Acceptance review:
> [`../tests/phase_scenario_0080_0_acceptance_review_results.md`](../tests/phase_scenario_0080_0_acceptance_review_results.md).
> Acceptance does **not** itself implement the production path.

> **Original admission status:** PROPOSED / ADMISSION ONLY — no runtime implementation, no production wiring.
>
> **0.0.8.0** is the active constitution ([`../design_0_0_8_0.md`](../design_0_0_8_0.md)).
> This packet is the **first consumer-pulled scenario gate** on the 0.0.8.0 production track
> ([`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)).
>
> **Acceptance** opens only the named substrate's already-defined production-path gate
> (`PRODUCTION-PATH-0080-0`). Acceptance does **not** itself implement the production path.

---

## 1. Scenario name and product purpose

| Field | Value |
|---|---|
| **Gate ID** | `SCENARIO-0080-0` |
| **Scenario name** | **Local Patrol Economy** |
| **Product purpose** | A small local scenario where an owned patrol unit moves from one location to another, keeps identity and owner relation, and participates in a simple local economy after relocation. |

**Consumer need:** A product scenario needs visible local movement plus basic economy coherence — not another substrate proof.

---

## 2. User-facing / product behavior

In product terms:

- The scenario contains **two or more local locations**.
- Each location has a **small local economy**.
- A **patrol unit** belongs to an **owner**.
- The patrol **consumes upkeep or supply** from the location where it is currently stationed.
- The patrol may provide a **local economic/security effect**, such as reducing local disruption or increasing local route safety.
- **Decision source (design-authority enrichment, accepted 2026-06-02):** the patrol's relocate/patrol
  decision is **GPU-resident** — a `disruption`/`local_security` `Threshold` crossing → `EmitEvent` →
  `BoundaryRequest` (the accepted FIELD_POLICY Field agent Proposal Pipeline V1 posture). The `move_request` is the
  materialized form of that proposal. **Not** a CPU planner; **not** an externally-scripted request.
  This makes the scenario exercise FIELD_POLICY (decision) + mobility (move) + Ownership/Flow (coherence)
  together. It pulls no new substrate — FIELD_POLICY V1 is an accepted mechanism, not a newly-pulled substrate.

- When the patrol moves from **Location A** to **Location B**:

  - its **entity identity is preserved**;
  - **source membership** updates;
  - **destination membership** updates;
  - **owner overlays** still apply;
  - the local economy **stops counting it at the source**;
  - the local economy **starts counting it at the destination**;
  - **no capture or spatial reparenting** semantics are introduced.

- **No gameplay UI** is implemented in this gate or this PR.

---

## 3. Basic economy scope

Bounded local economy only — enough to make ECON and OWNER relevant without reopening economy substrate design.

**Resources** (simple local values only):

- `supply`
- `maintenance`
- `local_output`
- `local_security`
- `disruption`

**Each location may have:**

- a supply balance;
- a maintenance burden;
- a simple output/security modifier affected by patrol presence.

**The patrol may:**

- consume upkeep/supply;
- contribute a local security/output effect;
- shift its economic participation when it moves.

**Explicitly excluded from this scenario:**

- hard currency
- markets
- trade routes
- nested Resource Flow
- multi-level economic fanout
- `ai_budget`
- policy overlays
- multi-faction economy
- Hybrid-Strata/faction-index scaling
- ClauseThing economic categories
- gameplay UI

---

## 4. Parked substrate consumed

Exactly **one** parked substrate:

| Substrate | State |
|---|---|
| **0.0.7.9 mobility/transfer substrate** | COMPLETE + PARKED, opt-in/default-off |

**Why this substrate:**

- It already contains **ALLOC, REENROLL, IDROUTE, ECON, OWNER**, RUNTIME-0/1A/1B, and the semantic-free GPU kernel substrate.
- It is **complete, parked, opt-in/default-off**, and mapped for the first non-test-support default `SimSession` path.
- **Local Patrol Economy** requires this substrate because movement, identity preservation, owner overlays, and local economy accounting must remain coherent after transfer.

---

## 5. Why the substrate is required now

Consumer-pulled rationale:

- This scenario is **not** "more mobility testing."
- The scenario needs **production-path eligibility** for local patrol relocation with local economy effects.
- The patrol's movement changes **both location membership and economic participation**.
- A scenario without the parked mobility/transfer substrate would either **fake movement**, **fake economy reassociation**, or **lose owner/economy coherence**.
- The next gate after acceptance should be the already-mapped production-path gate for the named substrate (`PRODUCTION-PATH-0080-0`).
- **No additional substrate expansion** is requested.

---

## 6. Bounds and scale

| Dimension | Bound |
|---|---|
| Scope | Local scenario only |
| Locations | Two or a few |
| Owners | One |
| Patrol units | One, or a very small fixed number |
| Economy | Basic local economy only (see §3) |
| Movement | Spatial movement only; no nested movement |
| Capture | No capture-as-reparenting |
| Hard currency | No hard-currency flow through Resource Flow |
| Markets | No markets or trade |
| UI | No gameplay UI |
| Shaders | No semantic WGSL |
| Planner | No AI planner |
| Authoring | No ClauseThing requirement |
| Scaling | No Hybrid-Strata/faction-index scaling |
| Evidence | Small deterministic scenario scale sufficient for admission — **no new soak** |

---

## 7. Designer/spec admission vocabulary

Scenario-facing vocabulary compatible with the current accepted `simthing-spec` admission surface (L0/L1/L2 / CLAUSE-SPEC). **Do not require ClauseScript; do not alter `simthing-spec`.**

| Term | Role |
|---|---|
| `scenario_id` / scenario name | Identifies Local Patrol Economy |
| `location` | A local place with its own economy |
| `local_economy` | Per-location economic state |
| `local_supply` | Supply balance at a location |
| `local_maintenance` | Maintenance burden at a location |
| `local_output` | Local output modifier |
| `local_security` | Local security modifier |
| `disruption` | Local disruption level |
| `patrol_entity` | The movable patrol unit |
| `patrol_owner` | Owner of the patrol |
| `current_patrol_location` | Where the patrol is stationed now |
| `source_location` | Origin of a move request |
| `destination_location` | Target of a move request |
| `move_request` | Request to relocate the patrol |
| `owner_relation` / `owner_overlay` | Owner linkage and latched modifiers |
| `local_economy_participation` | Whether/how the patrol counts in a location's economy |
| `acceptance_assertion` | Expected post-admission behavior check |
| `rejection_diagnostic` | Deterministic admission failure code |

---

## 8. Rejection vocabulary and diagnostics

Deterministic rejection cases (admission must reject):

| Rejection | Diagnostic intent |
|---|---|
| Owner-entity as spatial parent | `SCENARIO-0080-0-OWNER-ENTITY-SPATIAL-PARENT-REJECTED` |
| Capture-as-reparenting | `SCENARIO-0080-0-CAPTURE-AS-REPARENT-REJECTED` |
| Nested transfer | `SCENARIO-0080-0-NESTED-TRANSFER-REJECTED` |
| Hard-currency through Resource Flow | `SCENARIO-0080-0-HARD-CURRENCY-RF-REJECTED` |
| Market / trade / `ai_budget` | `SCENARIO-0080-0-MARKET-TRADE-AIBUDGET-REJECTED` |
| Semantic / raw WGSL | `SCENARIO-0080-0-SEMANTIC-WGSL-REJECTED` |
| CPU planner / urgency / commitment emission | `SCENARIO-0080-0-CPU-PLANNER-REJECTED` |
| Default-on without production gate | `SCENARIO-0080-0-DEFAULT-ON-WITHOUT-GATE-REJECTED` |
| Passive proof wrapper request | `SCENARIO-0080-0-PASSIVE-PROOF-WRAPPER-REJECTED` |
| Reopening closed ladder | `SCENARIO-0080-0-CLOSED-LADDER-REOPEN-REJECTED` |
| ClauseThing requirement | `SCENARIO-0080-0-CLAUSETHING-REQUIRED-REJECTED` (out of active scope) |

---

## 9. Runtime path requested

| Field | Value |
|---|---|
| **Next gate after acceptance** | `PRODUCTION-PATH-0080-0` |
| **Requested future gate scope** | First non-test-support default `SimSession` path for the named 0.0.7.9 mobility/transfer substrate, scoped to **Local Patrol Economy** |
| **This PR** | Does **not** implement it |

---

## 10. Default path / default schedule / gameplay

| Question | Answer |
|---|---|
| Production default path requested **now**? | **No** — only after admission acceptance opens `PRODUCTION-PATH-0080-0` |
| Default schedule requested **now**? | **No** |
| Gameplay-facing integration requested **now**? | **No** |
| Semantic WGSL requested **now**? | **No** |

---

## 11. Evidence needed for admission

Admission-only — evidence is **documentary**:

- [x] Scenario packet completeness (this document)
- [x] Exactly one parked substrate named (0.0.7.9 mobility/transfer)
- [x] Basic local economy scope is bounded (§3)
- [x] Patrol movement is the consumer requiring identity, owner, and economy coherence
- [x] Stop conditions listed (§15)
- [x] No implementation/code changes in this PR
- [x] No passive proof-wrapper request
- [x] Production-path gate not implemented (`PRODUCTION-PATH-0080-0` remains CLOSED)
- [x] ClauseThing remains horizon-only (§16)

Visibility report: [`../tests/phase_scenario_0080_0_admission_results.md`](../tests/phase_scenario_0080_0_admission_results.md).

---

## 12. Explicit non-goals

- no runtime implementation
- no production `SimSession` wiring
- no default schedule
- no gameplay UI
- no semantic WGSL
- no new GPU kernel
- no KERNEL successor
- no substrate expansion
- no nested Resource Flow
- no hard-currency economy
- no markets / trade / `ai_budget`
- no Hybrid-Strata/faction-index scaling
- no ClauseThing implementation
- no `simthing-spec` alteration for ClauseThing
- no invariant edits

---

## 13. Exit criteria for acceptance

Acceptance requires **design authority + product agreement** that:

1. the scenario is a **real product consumer** (not another substrate proof);
2. **exactly one** parked substrate is pulled (0.0.7.9 mobility/transfer);
3. the basic economy is **bounded** and does not reopen economy architecture;
4. patrol movement **requires** mobility/identity/owner/economy coherence;
5. the requested next gate is **correctly scoped** to Local Patrol Economy;
6. all **stop conditions** are preserved;
7. the next PR may open **`PRODUCTION-PATH-0080-0`** but may not implement beyond its authorized scope.

---

## 14. Stop conditions

From the 0.0.8.0 production track, plus economy-specific additions. Reject if the scenario requires:

- owner-entity as spatial parent
- capture-as-reparenting
- semantic/raw WGSL
- default-on behavior without a production gate
- hard-currency through Resource Flow
- nested Resource Flow
- market / trade / `ai_budget`
- CPU planner / urgency / commitment emission
- reopening atlas runtime, E-11B-5, B-1, ClauseThing/L3 front-end, FrontierV2-5, ACT/EVENT/OBS/PIPE, Hybrid-Strata/faction-index scaling, or any closed ladder without its own product authorization
- passive proof wrappers that do not unlock execution

---

## 15. ClauseThing horizon note

- **ClauseThing / ClauseScript** remains a horizontal future authoring surface — not active scope.
- This scenario does **not** assume ClauseThing support.
- This scenario targets the **current accepted `simthing-spec` admission surface**.
- It should avoid choices that make a future ClauseThing front-end impossible, but it does **not** design for ClauseScript now.

---

## 16. Pointers

- Active constitution: [`../design_0_0_8_0.md`](../design_0_0_8_0.md)
- 0.0.8.0 production track: [`../design_0_0_8_0_consumer_pulled_production_track.md`](../design_0_0_8_0_consumer_pulled_production_track.md)
- Parked substrate track: [`../design_v7_9_mobility_transfer_allocation_production_track.md`](../design_v7_9_mobility_transfer_allocation_production_track.md)
- Gating policy: [`../workshop/phase_m_gating_and_doc_policy.md`](../workshop/phase_m_gating_and_doc_policy.md)
- Binding rules: [`../invariants.md`](../invariants.md)
- Admission visibility: [`../tests/phase_scenario_0080_0_admission_results.md`](../tests/phase_scenario_0080_0_admission_results.md)
