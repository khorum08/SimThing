---
rung: 0088-STUDIO-SLICE-0
kind: rung
track: 0.0.8.8
base_sha: 5652ba2c1f4d68695543888f6e01338156c29d1f
audience: coding
model_tier: frontier
expected_route: ORCHESTRATOR-CLEARABLE(rehearsal-authored-scenario)
owner_approved: true
owner_notes: "DA handoff/ruling 5646203717 makes rung 1.2 O*: Orchestration selects the lane, reviews/merges conforming leaves, and graduates without DA unless a genuine reserve fires. Orchestration selected Astra / Frontier at Board 5646222623. Leaf A merged #2045 @ 5652ba2c. Orchestration correction after Astra persistence probe 5646740516: authoritative §3b 1.2 requires an initial persistence probe, not successful generation-N continuation. A truthful RED is valid probe evidence and is carried forward to 2.3, whose row owns checkpoint continuation/save-restore semantics. Do not mint a new persistence authority in 1.2 to turn that RED green."
surfaces: ["scenarios/**","docs/tests/rehearsal_scenario_*_results.md","crates/simthing-mapeditor/src/**","crates/simthing-mapeditor/tests/**","docs/tests/rehearsal_studio_*_results.md","crates/simthing-workshop/src/rehearsal_*","crates/simthing-workshop/tests/rehearsal_perf_*","docs/tests/rehearsal_perf_*_results.md","scripts/ci/test_inventory.tsv","scripts/ci/inspect_justifications.tsv","scripts/ci/triage_log.tsv","scripts/ci/closeout_artifacts.tsv","scripts/ci/anchor_reach_log.tsv","docs/tests/current_evidence_index.md"]
forbidden: ["no engine/spec/ClauseThing authority change: crates/simthing-{core,kernel,gpu,driver,sim,spec,feeder}/src/** and crates/simthing-clausething/src/** are OUT; if the slice needs new executable vocabulary, lowering authority, persistence authority, or refusal-law change, STOP and reserve","no protected/gate mutation: constitutional/census/anchor/class/router tables, scripts/ci gate code (*.sh/*.py), .github/workflows/**, allow/**, Cargo.toml, Cargo.lock are OUT; ordinary data ledgers named in surfaces remain lawful","no second scenario, observation, economy, save, replay, or UI authority; author THROUGH the canonical StemThing ingress and existing Studio resident-session doors","UI may display/operate admitted state but never make an economic decision or mint a new observation authority","sealed bundles are untouched: any build.rs COMPONENTS plan/sync edit or other E8 contact is an immediate STOP","scenario-born candidate code, if any, homes only in simthing-workshop rehearsal surfaces and is mortal; no engine homing","never weaken or bypass a refusal, historical referee, persistence check, timing denominator, or lifecycle rule to make the slice green","no self-triage: every scan-id-bearing INSPECT returns to Orchestration"]
required_checks: ["before edits render/read this HD, obtain a fresh coding ORIENT, run bash scripts/ci/anchor_query.sh --domain rehearsal-0088, and ACK every emitted REQUIRED-ANCHOR","author scenarios/stellaristhing_base.clause as the first two-faction live slice with spatial containment, RF parent/scope transport, and faction ownership/policy kept as three distinct relations; owner seats are never spatial containers","prove the ordinary Studio picker opens the authored scenario portably with no absolute or machine-local dependency assumptions","prove generation-stamped values change on the real resident session through a live source/sink resource path; a static map is RED","prove failed load/admission before effects leaves the prior valid Studio document/session intact","perform and record an initial persistence probe on the running slice; PASS or RED is valid 1.2 evidence if reported truthfully. Rebuilding source and replaying N steps is not generation-N continuation; do not invent new persistence authority in 1.2. Carry any RED seam forward to 2.3 checkpoint-continuation work","capture Studio baseline timings for the pinned slice workload and record source/profile digest without inventing a threshold or changing the denominator","split leaves by existing rehearsal class envelope as needed; every PR carries the proper scenarios/** or rehearsal_* trigger plus tested_code_sha, coverage_basis, ci_green, novelty_claim NO (or exact reserve basis), lifecycle-legal rows for tests/fixtures actually added, and complete INSPECT list","run touched-package/local semantic batteries plus hosted Doctrine Scan and fresh Clearance on each final head; never merge on red","return every implementation/evidence leaf PROBATION / proof-present / OPEN / UNMERGED to ORCHESTRATION ONLY with exact base/head and evidence paths; no direct DA contact unless Orchestration relays a genuine reserve"]
stop_conditions: ["any required PASSING 1.2 exit proof other than the diagnostic persistence probe requires engine/spec/ClauseThing production semantics, new executable vocabulary, a new lowering/observation/persistence authority, or changed ingress refusal law","the two-faction live slice cannot be authored through existing canonical StemThing semantics without a second IR, registry, scenario authority, economy manager, or UI-owned state","portable ordinary-picker loading cannot be achieved without a machine-local resolver token or protected/gate change","Studio timing evidence would require a changed denominator, workload shaping, threshold invention, or weakened measurement referee","any sealed-bundle/E8, protected census/anchor, class/router/gate, Cargo, or forbidden engine surface is required","a required historical or current referee would need weakening/deletion rather than a stronger successor"]
---
## BUILD
Author the first real 0.0.8.8 Studio slice through the ingress converged by 1.1. Create
`scenarios/stellaristhing_base.clause` with two factions and a viable live source/sink
resource path, then open/run/step/inspect it through the ordinary Studio resident-session
path. Keep structural residency, RF transport/scope, and faction ownership/policy as three
separate authored relations. Use only necessary generic observation bindings. The §7 growth
allowance is open inside the admitted rehearsal classes, but it does not authorize a new
simulation authority.

Implementation may land as multiple bounded O* leaves: authored scenario, Studio
presentation/operation proof, and measurement. Each leaf stays inside its matched class
envelope and returns to Orchestration for exact-head clearance and merge.

## FENCES
- Canonical StemThing semantics only; no legacy ingress or substitute model.
- No engine/spec/ClauseThing production changes, new executable vocabulary, or new authority.
- UI observes/operates admitted state; it never decides economics.
- Candidate proof code is workshop-homed and mortal; no sealed-bundle/E8 or gate touch.
- Failed pre-effect load preserves the prior valid session; do not weaken refusal behavior.
- Simulation speed changes wall-clock scheduling only, never generation-stamped economics.
- The 1.2 persistence obligation is diagnostic: record the real save/reload result; do not
  widen authority to force continuation. Successful checkpoint continuation belongs to 2.3.
- Preserve lifecycle law: no permanent proof, no bulk renewal, no silent lease extension.

## EXIT-PROOF
The ordinary Studio picker opens `stellaristhing_base.clause` portably; the real resident
session advances generation-stamped values on a live source/sink path; a failed load leaves
the prior valid session intact before effects; an initial persistence probe is recorded
truthfully, including any RED and owning authority seam; and Studio baseline timings are
captured against a pinned source/profile digest. The authored slice contains two factions
with the three relation families kept distinct and a viable initial economy. All owning
rehearsal tests/results and lifecycle rows are committed, every implementation/evidence leaf
is green and O*-merged by Orchestration, and no reserved surface or alternate authority was
needed to satisfy the passing 1.2 obligations.
