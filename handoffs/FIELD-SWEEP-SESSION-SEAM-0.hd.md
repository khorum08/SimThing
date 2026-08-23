---
rung: FIELD-SWEEP-SESSION-SEAM-0
kind: remedial
track: 0.0.8.7
base_sha: b1b09d92ebbe18e3638e3efb78e401158db3f3e5
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Minted after a SECOND lawful STOP on the 11.2 arc; Owner directed 2026-08-23 to build the seam rather than descope the exemplar. The PALMA and Gu-Yang producers exist and return admitted FieldSweepRegistration products, but no ordinary session consumes them, so re-export cannot make Run execute a Triad. The execution primitive already exists and is already used in production: FirstSliceMappingSession builds FieldSweepSession instances and runs them. This rung is WIRING, not a new engine. 5.8b (DA 5154348081) holds: Triad columns remain explicit consumer inputs and must NOT be defaulted at install."
surfaces: ["crates/simthing-driver/src", "docs/tests", "scripts/ci/test_inventory.tsv"]
forbidden: ["a second field execution path, bespoke field kernel, or any new evaluator, authority, scheduler, registry, or service; stead section 10 Structural Execution Convergence Contract routes PALMA and Gu-Yang through EXISTING ops", "defaulting Triad columns at install; 5.8b keeps them explicit consumer inputs and this rung only ACCEPTS them", "minting a public chokepoint, corridor, front, dominance, or contest API; those stay born observables", "CPU-side decisions derived from comparative observation; observation is read-only and decisions stay on-device", "Vendor Door or facade work; that is 11.1b VENDOR-DOOR-TRIAD-SURFACE-0 and lands after this rung", "the 11.2 guide, exemplars, or admission gate", "11.3, 12.x, StemThing-B, Vector CostBand, ClauseThing debts"]
required_checks: ["before edits render/read this handoff; fresh Frontier coding ORIENT receipt at rule stamp 61818ff7d4adda84; ACK every rendered REQUIRED-ANCHOR", "FIRST STEP archaeology: map the existing field execution path end to end - compile_structured_field_sweeps, FieldSweepSession, FirstSliceMappingSession - and state exactly where the PALMA and Gu-Yang registration products must attach, before writing any seam", "an ordinary production session accepts admitted PALMA and Gu-Yang FieldSweepRegistration products and executes them in the ordinary tick", "admit_comparative_from_field_plan reaches SpecSessionState.comparative_projection on the production path; that field is declared but never assigned today", "convergence proof by diff and grep: no new field execution is authored and the seam routes through the existing FieldSweepSession", "planted second-execution-path defect REDs for its own named reason on the production path", "planted install-time Triad-column default REDs, proving 5.8b is preserved", "shape test: chokepoint, corridor, front and dominance still have ZERO public production surfaces after the rung", "inventory rows for new tests; lifecycle, detachability, DOC-BUDGET, orientation, sanctioned digest, Agent and Doctrine green; inspect every hosted Doctrine Scan step; return exact base/head/tested_code_sha plus workflow ids"]
stop_conditions: ["the seam cannot attach without authoring a second field execution path or a bespoke kernel", "executing admitted registrations appears to require a new authority, evaluator, scheduler, or admission vocabulary", "the only way to make the tick execute them is to default Triad columns at install", "a needed product or execution primitive is genuinely absent rather than merely unwired - name the exact missing surface and stop"]
---
## BUILD
- Land the **production session seam** that lets already-admitted PALMA and Gu-Yang `FieldSweepRegistration` products be **executed by an ordinary session tick**. This is the object 11.1b needs and cannot re-export its way to.
- **Wire, do not build.** The execution primitive exists and is already production: `FieldSweepSession` is constructed and run today by `FirstSliceMappingSession`, which compiles its own registrations from `gpu_config` and is gated on `formula_class == "field_urgency"`. Reuse that execution; attach the admitted registration products to it.
- Assign `SpecSessionState.comparative_projection` on the production path via `admit_comparative_from_field_plan`. The field is **declared but never assigned in production today**, so comparative admission currently reaches nothing.
- Keep the consumer in charge of Triad columns, per 5.8b (DA `5154348081`): the seam **accepts** registrations and columns from the consumer; ordinary install continues not to default them.
## FENCES
- **stead section 10, Structural Execution Convergence Contract, is the governing law:** PALMA and Gu-Yang route through EXISTING ops via driver/sim. A bespoke field kernel or a second execution path is the defect this rung exists to avoid, not a shortcut to green.
- **Wiring is not capability.** If attaching the products requires a new authority, evaluator, scheduler, or admission vocabulary, that is a substrate gap beyond this rung - STOP and name it.
- `chokepoint`, `corridor`, `front` and `dominance` have zero public production surfaces today and must still have zero afterwards. They are born observables; nothing here mints one.
- Observation stays read-only: comparative output may be observed, never used to make a CPU-side decision. Decisions stay on-device.
- No Vendor Door or facade work. 11.1b lands the re-export **after** this rung, and 11.2 lands the guide and exemplars after that.
## EXIT-PROOF
- A driver test drives the **ordinary production path**: admitted PALMA and Gu-Yang registrations are accepted, executed in the tick, and comparative output is observed.
- **`SpecSessionState.comparative_projection` is assigned on that path**, with the falsifier that removing the assignment REDs rather than silently observing nothing — the declared-but-dead field becoming live is the point.
- **Convergence proven, not asserted:** diff and grep show no new field execution authored and the seam routing through the existing `FieldSweepSession`; a planted second-execution-path defect REDs for its own named reason.
- **5.8b preserved:** a planted install-time Triad-column default REDs.
- **Shape test green:** the four observable names still carry zero public production surfaces.
- Evidence: `docs/tests/field_sweep_session_seam_0_results.md`, inventory rows for new tests, ladder row stamped at merge with `merged #<PR> @ <sha>`.
