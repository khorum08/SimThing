---
rung: EMBEDDER-GUIDE-EXEMPLARS-0
kind: rung
track: 0.0.8.7
base_sha: fc313a21e95d0853e93d2e3680ddc671f03bac4b
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "DA-authored because EXIT-PROOF-COVERAGE flagged 11.2 INSPECT (scope dated 2026-08-03 absent from the exit proof); the same delta widens that cell. DA finding the lane must not rediscover: POW is NOT an admitted opcode, EXP and LN are. A power law is authored as exp(k * ln x). Minting POW is an EML library addition and a STOP."
surfaces: ["docs/embedders_guide.md", "docs/tests", "crates/simthing-embedder", "scripts/ci", "scripts/ci/test_inventory.tsv", "scripts/ci/doc_budget_baseline.tsv"]
forbidden: ["new engine capability, authority, evaluator, service, or simulation path behind the guide; 11.2 is onboarding surface over the already-graduated five-verb door", "minting a POW opcode or any new EML primitive or vocabulary member; power laws compose from the admitted EXP and LN", "engine edits or scenario-side wiring required to run an exemplar; the cold reader needs neither", "finance or networking vocabulary landing in engine crates; SCENARIO-RESIDUE remains the vocabulary gate", "ClauseThing dependency, reverse dependency onto simthing-embedder, or a second front-end", "adding cargo build/test/check to any workflow; CI runs no cargo by standing Owner ruling", "11.3, 12.x, StemThing-B, Vector CostBand, or the queued ClauseThing debts"]
required_checks: ["before edits render/read this handoff; fresh Std coding ORIENT receipt at rule stamp 61818ff7d4adda84; ACK every rendered REQUIRED-ANCHOR", "FIRST STEP: read crates/simthing-embedder/src and tests/vendor_door_0.rs and write the guide from the door that EXISTS; every code block in the guide must appear verbatim in an exemplar so the two cannot drift", "guide lands DOC-BUDGET-capped with its baseline row; doc_budget_check green", "two exemplars as integration tests in crates/simthing-embedder/tests: a finance toy and a network-saturation full-Triad exercise; each uses only the five verbs and imports no engine crate the door does not already re-export", "at least one exemplar STATES its domain law through admitted EXP/LN rather than a staircase or piecewise ladder; power law is exp(k * ln x)", "new CI admission gate proves cited guide paths resolve, exemplars are door-only, and the authored-law exemplar carries the EML vocabulary; gate ships with a planted defect and a selftest", "inventory rows for both exemplars; lifecycle, detachability, DOC-BUDGET, orientation, sanctioned digest, Agent and Doctrine green; inspect every hosted Doctrine Scan step; return exact base/head/tested_code_sha plus workflow ids"]
stop_conditions: ["an exemplar cannot be written without an engine edit, new capability, or scenario-side wiring - the door is short of its claim, report the exact gap", "the authored-law example appears to need a POW opcode or any new EML member", "the CI admission gate can only be made to bite by matching a name or phrase rather than the law", "a guide claim cannot be backed by an exemplar that actually runs"]
---
## BUILD
- Land `docs/embedders_guide.md`: a DOC-BUDGET-capped Embedder Guide teaching the five verbs in anatomy order — **Derive, Populate, Overlay, Bind, Run** — as the vendoring and onboarding surface for the door graduated at 11.1 (#1800 @ `9929ac98`). Write it for a cold reader with no SimThing history.
- **The guide states no code it cannot run.** Every code block must appear verbatim in an exemplar, so guide and exemplar cannot drift; the guide cites exemplar paths rather than paraphrasing them.
- Land two exemplars as integration tests in `crates/simthing-embedder/tests/`, deliberately **non-game** because domain-neutrality is the door central claim and only a non-game witness tests it: (1) a **finance toy**; (2) a **network-saturation full-Triad exercise** driving need/corridor/front/chokepoint bands so Triad observables are born rather than hand-fed.
- At least one exemplar **authors its domain canonical law** through the completed vocabulary instead of approximating it: volume-delay as a power law, backoff or utility as exp/log-shaped. Compose the power law as `exp(k * ln x)` from the admitted `EXP` and `LN`.
- Add the CI admission gate for exemplars: cited guide paths resolve, exemplars import only the embedder door, and the authored-law exemplar carries the EML exp/ln vocabulary. Ship it with a planted defect and a selftest, per standing gate discipline.
## FENCES
- 11.2 is **onboarding surface, not simulation capability**. If the guide wants an engine change to be writable, the door is short of its claim and that is a STOP with the gap named, not an engine edit.
- **`POW` is not an admitted opcode; `EXP` and `LN` are.** Express power laws by composition. Minting `POW` is an EML library addition, gated by the `EML-OPCODE-LIBRARY` constitutional surface and routed DA-RESERVE — STOP and escalate instead.
- The cold reader stands up a running, serialized, observed tree with **zero engine edits and zero scenario-side wiring**. An exemplar needing either falsifies the rung premise.
- No finance or networking vocabulary in engine crates; `SCENARIO-RESIDUE` remains the vocabulary hard gate and this rung adds no scenario-word branch.
- `simthing-embedder` stays a leaf: nothing in the engine may depend on it, and the exemplars add no state to it.
- **Do not add cargo to CI.** CI admission-checks structure; execution greenness is proven at DA review. That split is a standing Owner ruling, not a gap to close.
## EXIT-PROOF
- A cold reader stands up a running, serialized, observed tree from the guide alone — zero engine edits, zero scenario-side wiring — and **both exemplars execute green** under the embedder batteries.
- **The authored-law falsifier bites the LAW, not a spelling:** replacing the `exp`/`ln` law with a staircase or piecewise ladder REDs; renaming a symbol does not. A gate that can only be tripped by a name change is the failure mode this rung must avoid.
- The CI admission gate REDs its planted defect for its own named reason and passes its selftest; the guide DOC-BUDGET row lands and `doc_budget_check` is green.
- Guide/exemplar drift is unrepresentable: every guide code block is present verbatim in an exemplar that runs.
- Evidence: `docs/tests/embedder_guide_exemplars_0_results.md`, inventory rows for both exemplars, ladder row stamped at merge with `merged #<PR> @ <sha>`.
