# SimThing Repo Guide for Web AI Models

**Purpose:** This document provides curated, high-signal links to the SimThing GitHub repository. These links are intended to give web-based AI chat models (Claude, GPT, Gemini, etc.) direct access to accurate, up-to-date information about specific subsystems of SimThing without requiring them to hallucinate or rely on outdated training data.

All links point to the official repository:  
**https://github.com/khorum08/SimThing** (default branch: `master`)

---

## Quickstart for Low-Context AI Agents

Read these raw files (in order) for the fastest high-signal orientation. All links are raw.

**1. Current constitution (v7.8) + bounded posture** (mandatory first read for any current-state question)  
https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_8.md

**2. v7.8 Production Track (PR ladders & closure status)**  
https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_8_production_track.md

**3. v7.9 Mobility / Transfer Allocation Production Track** (currently the most active development area)  
https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_9_mobility_transfer_allocation_production_track.md

**4. (Historical baseline — v7.7 closed posture) Core concept, SEAD definition, and "SimThings-as-AI" philosophy**  
https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_7.md  
 Introduces the overall SimThing substrate, the three-layer spatial field model, SEAD (Spatiotemporal Evolution with Attractor Dynamics) as the field-intelligence system, and the rule that AI commitments emerge as GPU threshold crossings over parent EML pressure fields — with no CPU planner.

**5. Binding constitutional constraints**  
https://raw.githubusercontent.com/khorum08/SimThing/master/docs/invariants.md  
 The hard rules: simthing-sim stays semantic-free and map-free, everything is opt-in by default, no semantic WGSL, strong guardrails on what is allowed.

---

## v7.9 Mobility Track — Current Active Focus (Early June 2026)

Multiple mobility substrates have been implemented in the `simthing-spec` designer-admission layer (substrate modeling + tests only — no production runtime wiring):

- ALLOC-0: Deterministic slab + bulk-accounting allocator
- REENROLL-0: Bilateral arena re-enrollment for spatial movement
- IDROUTE-0 (+ R1 hardening): Local D-2 identity routing (masked sums, argmax, directed disburse)
- ECON-0: Session clearinghouse + subsidiarity economy

**Key principles demonstrated:**
- Spatial movement (reparenting) is orthogonal to political/owner control (column flips).
- Local identity routing uses bounded per-cell mechanisms rather than global vectors.
- All work is heavily guarded at the scenario/designer admission layer.

**Primary documents:**
- https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_9_mobility_transfer_allocation_production_track.md
- https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/mobility_and_transfer_allocation.md
- Mobility substrate implementations (ALLOC-0, REENROLL-0, IDROUTE-0, ECON-0)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-spec/src/designer_admission/
- Recent mobility test reports  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/tests/ (look for files starting with `phase_mobility_`)

---

## Resource Flow System

**Current concrete posture:**
- `FlatStarResourceFlow` (depth-2) remains the production posture — opt-in only; global default false.
- **A-0** (static nested Resource Flow D=3/D=4) is **ACCEPTED** (A-0-ACCEPT-0). Line A static nested Resource Flow is closed at the first slice. Dynamic E-11B-5 enrollment remains parked behind a future named product scenario.
- **B-0** (narrow D-2a hard-currency ordering) is ACCEPTED and closed at smoke level (B-0-ACCEPT-0). No B-1 is open.
- **C-2** is ACCEPTED — map batching closed at the designer surface. Production atlas runtime / sparse-residency scheduler is a separate later gate (not open).

All promoted v7.8 M/E/T lines are now closed for their current named scenarios. No implementation gate remains open. See `docs/design_v7_8_production_track.md` for the authoritative status.

See the v7.8 Production Track for the single authoritative status of all lines.

The Resource Flow system (AccumulatorOp v2 / E-11) is the GPU-native approach to economy, production, and hierarchical resource movement on the accepted flat-star substrate.

This section contains the most important documents and source files for understanding:

- The high-level architecture and commitments
- The constitutional guardrails
- The core driver-side implementation (arenas, enrollment, compilation)
- The GPU-side execution model (emission, transfer, accumulator operations)
- Relevant shader code

### 1. Foundational Architecture & Constitution

These three documents form the authoritative foundation. Any AI model attempting to reason about Resource Flow should read these first.

- **Resource Flow Substrate ADR** (Primary architectural decision record)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/resource_flow_substrate.md

- **SimThing Core Structural Invariants** (Constitutional rules — many directly constrain Resource Flow posture)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/invariants.md

- **AccumulatorOp v2 Production Plan** (High-level roadmap and design for the current resource/economy execution model)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/accumulator_op_v2_production_plan.md

### 2. Core Driver Implementation

These files contain the main logic for how Resource Flow is authored, compiled, enrolled, and managed at the `simthing-driver` / `simthing-spec` layer.

- **Arena Registry** (Central registry and management of resource arenas — one of the most important files)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/src/arena_registry.rs

- **Resource Flow Compile** (How Resource Flow specs are compiled into runtime structures)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/src/resource_flow_compile.rs

- **Resource Flow Enrollment** (How participants are enrolled into the resource flow system)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/src/resource_flow_enrollment.rs

- **Arena Participant** (Core abstraction for things that participate in resource arenas)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/src/arena_participant.rs

- **Arena Hierarchy** (How hierarchical/nested resource flow structures are represented)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/src/arena_hierarchy.rs

### 3. GPU Execution Layer

These files show how Resource Flow actually executes on the GPU.

- **Emission Accumulator** (Handles non-conserving "EmitEvent" style resource operations)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-gpu/src/emission_accumulator.rs

- **Transfer Accumulator** (Core logic for resource transfers between participants)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-gpu/src/transfer_accumulator.rs

### 4. Shader Code (GPU Compute)

The actual parallel execution logic lives in WGSL shaders.

- **Accumulator Operation Shader** (The main shader implementing the generic AccumulatorOp model used by Resource Flow)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-gpu/src/shaders/accumulator_op.wgsl

---

## How to Use This Document

When feeding SimThing information to a web AI model, provide the **raw** links above. This gives the model clean, plain-text access to the actual source without HTML noise.

Recommended minimal context pack for Resource Flow questions:

1. `resource_flow_substrate.md` (ADR)
2. `invariants.md` (binding rules)
3. `accumulator_op_v2_production_plan.md`
4. `arena_registry.rs`
5. `accumulator_op.wgsl`

This combination gives both the "why" (architecture + constitution) and the "how" (implementation + GPU execution).

---

## Atlas / Map Batching (Line C) — Closed at the Designer Surface

**Status (as of C-2-ACCEPT-0):** Line C is now **closed at the designer/spec layer**.

C-0 proved a real packed-atlas GPU path (algebraic tile-local mask G=0, full-tile protocol-oracle parity).  
C-1 validated the 2000-star target envelope against the active budget.  
C-2 implemented bounded admission relaxation: homogeneous-square, algebraic-G=0, protocol-oracle-backed specs that fit the active `V78AtlasVramBudget` with mandatory multiplier reporting can now be admitted.

**Important clarification:**
- The **production atlas runtime + sparse-residency / cadence scheduler** is explicitly **not open**. It is a separate later gate.
- Atlas remains opt-in / default-off (`MappingExecutionProfile` default stays `Disabled`).
- Physical gutter, active mask (M-6A), and source identity (M-5) remain rejected or deferred.

### Two Reference Profiles (from C-1 / C-2 work)

- **TypicalHugeCommodityProfile** (~128×128 map, 1000 stars, 5×5 surfaces): algebraic G=0 ≈ 0.163 GiB — comfortably fits the 1.5 GiB default.
- **HorizonDedicatedServerStressProfile** (200×150 + 2000 stars + 10×10 surfaces): algebraic G=0 ≈ 0.862 GiB (fits); physical gutter ≈ 5.826 GiB (requires raised budget).

### Best Files for Understanding Atlas / Map Batching

- **C-2 Acceptance Review** (the authoritative closure document)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/tests/phase_m_c2_acceptance_review_results.md

- **C-1 2000-Star Scale Model Report** (budget math and profile modeling)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/tests/phase_m_c1_atlas_2000_star_scale_model_results.md

- **C-0 Implementation + Report** (real packed-atlas path with protocol oracle)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/tests/phase_m_c0_m4_atlas_protocol_oracle_results.md

- **V78AtlasVramBudget + Atlas Admission types** (the actual C-2 code surface)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-spec/src/designer_admission/v7_8_line_scenarios.rs  
  (and the new `atlas.rs` module added during C-2)

- **Current status table**  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/mapping_current_guidance.md

- **Isolation policy note** (why algebraic G=0 is preferred)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/mapping_sparse_regioncell.md

---

## SEAD (Spatiotemporal Evolution with Attractor Dynamics)

**Official expansion:** Spatiotemporal Evolution with Attractor Dynamics

This concept and naming originates from the paper ["On the Spatiotemporal Dynamics of Generalization in Neural Networks"](https://arxiv.org/abs/2602.01651) by Zichao Wei, which derives SEAD as a neural cellular automaton architecture that respects physical postulates of computation (locality, symmetry, and stability via attractors) to achieve true length generalization and causal reasoning. The SimThing implementation draws inspiration from these ideas in its field-based, attractor-driven approach to spatial awareness and decision-making.

SEAD is the project codename for SimThing’s field-intelligence / spatial awareness and commitment system. It is the primary mechanism through which AI-like decision making currently emerges in the engine.

### Core Concept
SEAD implements a three-layer spatial awareness model:
- **Layer 1**: Dense local state fields (via StructuredFieldStencilOp / RegionCell grids)
- **Layer 2**: Hierarchical reduction across the spatial tree
- **Layer 3**: Parent-level EvalEML composition that produces “urgency” or pressure signals

These urgency signals are monitored by thresholds. When a threshold is crossed, the system emits an event (`EmitEvent` / `ConsumeMode::EmitEvent`). This event represents a **commitment** — the point at which the simulation “decides” to act on the detected pressure.

### Key Characteristics
- No traditional CPU-side AI planner exists in this path.
- Behavior is driven by authored spatial fields and gradient extraction (M-5 series operators).
- Economic state can influence SEAD by changing which EML weight profiles are active (see the Economy + SEAD product fixtures).
- The system is explicitly designed around **attractor dynamics** — pressures in the field landscape naturally pull the simulation toward certain states or actions.

### Relevance
SEAD represents SimThing’s field-intelligence approach to spatial awareness and decision-making (the “AI-as-SimThing” pattern). 

**Important clarification on placement (v7.8):**
The SEAD self-AI work (field → personality-weighted observer score → threshold → compaction → bucketing/reductions → numeric proposal → admission) was chartered and closed as **SEAD Self-AI Proposal Pipeline V1** (OBS-0..4 + EVENT-0..2 + PIPE-0 + ACT-0..2 consolidated). M-JIT closed at PROD-0. Per the charter (`sead_self_ai_track.md` §10–§11, Opus authority 2026-05-30): no further `ACT-N` or ladder extension; the vertical is accepted at fixture level. SEAD remains a **Phase M parallel self-AI / field-intelligence track**, not part of the E-phase Resource Flow effort and not a dependency for it. Next gate for richer self-AI or spatial work is the simthing-spec / ClauseThing admission layer (L1+), not more fixtures.

### Best Files for Understanding SEAD
- `docs/design_v7_7.md` (section 3 — SEAD field-intelligence surfacing)
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_7.md
- Economy + SEAD product fixture support code (Phase M orchestration example)
  https://raw.githubusercontent.com/khorum08/SimThing/master/crates/simthing-driver/tests/support/economy_sead_product_fixture.rs
- Various SEAD-OBS-* and SEAD-EVENT-* test results (Phase M sandbox probes)
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/tests/

---

*This document is intended to grow over time with additional sections for other major subsystems (Mapping / Structured Fields, EML Gadgets, AI / SEAD, Events, etc.).*

**Repository:** https://github.com/khorum08/SimThing

---

## Current Closed & Concrete Posture (Early June 2026)

**v7.7 + AccumulatorOp v2 production plan: CLOSED.** v7.8 M/E/T lines (A-0, B-0, C-0/1/2) are closed at their named scenarios. The v7.9 Mobility track has seen multiple substrate implementations (ALLOC, REENROLL, IDROUTE, ECON) land at the designer-admission / `simthing-spec` layer. OWNER remains proposed/parked. No production runtime integration is open.

**Concrete & closed (production-usable when explicitly opted in):**
- FlatStarResourceFlow (depth-2 only) — opt-in via profile or limited ScenarioClassDefaultOn; global default false.
- EML-GADGET full Tier-1 stateless + Tier-2 temporal substrate — landed with CPU-oracle parity and designer-layer admission.
- First-slice mapping + gradients (M-5), boundary doctrine, exact F sqrt, and core invariants/guardrails.
- v7.9 Mobility substrates (ALLOC-0, REENROLL-0, IDROUTE-0, ECON-0) — implemented and green at the `simthing-spec` designer-admission layer (non-production).

**Current major line status (early June 2026):**
- **v7.8 M/E/T lines**: All closed at their named scenarios (A-0 static nested RF, B-0 narrow hard-currency, C-0/1/2 atlas at designer surface).
- **v7.9 Mobility track**: ALLOC-0, REENROLL-0, IDROUTE-0 (with R1 hardening), and ECON-0 substrates are implemented and green at the `simthing-spec` designer-admission / substrate layer. OWNER remains proposed/parked.
- FlatStarResourceFlow (depth-2) remains the active production posture for Resource Flow (opt-in only).
- Production runtime integration for mobility features is **not open**. All work stays behind explicit scenario admission guardrails.

Richer ClauseThing-authored scenarios are still the expected source of named needs for any remaining deferred capability.

**Key raw links for the current posture (add these to any context pack):**
- v7.8 Constitution + Production Track: https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_8.md and https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_8_production_track.md
- **v7.9 Mobility Production Track** (most important for current active work): https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_9_mobility_transfer_allocation_production_track.md
- Mobility Workshop Findings (spatial vs political model, IDROUTE local routing, ECON subsidiarity, etc.): https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/mobility_and_transfer_allocation.md
- Mapping Current Guidance (status table): https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/mapping_current_guidance.md
- C-2 Acceptance, B-0 Acceptance, and other v7.8 closure reports remain relevant for historical context.

---

## EML Opcodes + JIT WGSL Tooling for Clausewitz-style Modifiers

The primary mechanism in SimThing for expressing the rich modifier systems described in `ClauseThing.md` (triggered modifiers, value: calculations, complex_trigger_modifier, economic category formulas, stacking, conditional effects, etc.) is the **EML gadget system** (composable node templates over existing EvalEML opcodes) combined with **JIT WGSL compilation** of those graphs.

This allows designer-authored complex modifier logic to be expressed at the spec/RON layer, compiled to efficient GPU kernels, while preserving determinism, CPU-oracle parity, and the "no semantic WGSL" constitutional rule.

### Key Files (raw links)

- **EML Gadget Library Design Note** (core explanation of how gadgets compose EML for modifier-like calculations)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/eml_gadget_library_design_note.md

- **Design v7.7 Amendment** (high-level integration of EML, mapping fields, SEAD urgency, and modifier-like pressure systems)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/design_v7_7.md

- **C8 EML Transfer / Intensity Design** (detailed use of EML for economic transfer and intensity calculations — directly analogous to resource category modifiers)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/c8_eml_transfer_intensity_design.md

- **M5 Gradient Extraction Design Note** (advanced EML usage for spatial "modifier" effects like scarcity/opportunity gradients)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/m5_gradient_extraction_design_note.md

- **SEAD Self-AI Track** (how EML + field machinery enables AI personalities via modifier-style urgency/pressure formulas)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/sead_self_ai_track.md

- **Overlay OrderBand Compiler Design** (how modifier application / overlay stacking is compiled to OrderBands using EML)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/workshop/c4_overlay_orderband_compiler_design.md

For the JIT side (compilation of EML graphs to WGSL kernels for performance):

- **M-JIT / Kernel Descriptor and Production Registry work** is documented across the M-JIT ladder in `docs/tests/phase_m_jit_*` and consolidated in the SEAD self-AI and workshop notes above. The core idea is that complex modifier formulas (as EML gadgets) can be JIT-compiled into specialized kernels while remaining within the ExactDeterministic / bounded execution classes.

These tools together provide the bridge for transpiling Clausewitz modifier concepts into SimThing's substrate without violating the core invariants (semantic-free substrate, CPU parity, opt-in only).

---

## Architecture Decision Records (ADRs)

These are the primary architectural decisions. All links are raw for direct ingestion by AI models.

### Core System ADRs
- **Resource Flow Substrate** (foundational for arenas, flows, and the E-phase model)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/resource_flow_substrate.md

- **Mapping Sparse RegionCell** (three-layer field model, Phase M field-intelligence, and optimization doctrine — central to SEAD work)  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/mapping_sparse_regioncell.md

### Supporting ADRs
- Install Clone Then Commit  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/install_clone_then_commit.md

- Scripted Event Scope Model  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/scripted_event_scope_model.md

- Spec Session State Replay  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/spec_session_state_replay.md

- Capability Effect Target Scope  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/capability_effect_target_scope.md

- Game Mode Session Installation  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/game_mode_session_installation.md

- PR11 Track A Session Assembly  
  https://raw.githubusercontent.com/khorum08/SimThing/master/docs/adr/pr11_track_a_session_assembly.md

**Note:** The Mapping Sparse RegionCell ADR is especially important for understanding the current SEAD / field-intelligence work (Phase M). It is distinct from the Resource Flow E-phase track. For the complete current set of ADRs, use the GitHub connector or browse the `docs/adr/` directory on the repo.
