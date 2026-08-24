---
rung: AUTHORED-LAW-GADGET-0
kind: remedial
track: 0.0.8.7
base_sha: f707af605e12859ee12ec02420ba27b2f8969537
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "11.1e ACTIONBAND-EXECUTION-INGRESS-0 graduated under DA ruling 5401443746. This is the last substrate rung before #1803/11.2 may resume. DA mint #1807 fixed the semantic choice: vendors state a power law as authored data through one law-stating EmlGadgetInstanceSpec member compiling exp(k*ln(x)) with the already-admitted EXP/LN primitives. Intrinsic mechanism wins over prose. No POW opcode, no staircase compatibility form, and no runtime-domain surprise. The vendor-facing gadget vocabulary becomes a censused constitutional surface in this same rung. 11.2, 11.3+, 12.x and pointer movement remain fenced."
surfaces: ["crates/simthing-spec", "crates/simthing-core", "crates/simthing-driver", "crates/simthing-embedder", "scripts/ci/constitutional_surfaces.tsv", "docs/tests", "scripts/ci/test_inventory.tsv"]
forbidden: ["new EML opcode, POW opcode, vendor transcendental, bespoke evaluator, kernel, WGSL path, or second execution path", "staircase, piecewise ladder, threshold table, or renamed equivalent accepted as the law-stating gadget", "runtime repair/clamp that hides an authored input capable of reaching forbidden LN domain", "uncensused extension of EmlGadgetInstanceSpec or weakening existing EML opcode/stack constitutional gates", "changes to #1803/11.2 guide/exemplars, 11.3+, 11.4, 12.x, or pointer state"]
required_checks: ["before edits render/read this handoff; carry a fresh Frontier coding ORIENT receipt at the current rule stamp and ACK every rendered REQUIRED-ANCHOR", "FIRST STEP archaeology: locate EmlGadgetInstanceSpec, compile_eml_gadget, its production consumers, the admitted EXP/LN node/opcode surfaces, and current constitutional-surface census; distinguish authored vocabulary from opcode vocabulary", "add exactly one law-stating authored gadget member whose canonical compilation is EXP(k * LN(x)); compose only already-admitted primitives and keep POW absent", "define admission metadata sufficient to prove the input domain cannot reach LN-invalid values; unsafe or uncertified authored law REDs at admission with a named diagnostic, never at runtime", "make staircase/piecewise substitution for the law-stating role fail semantically at admission for its own named reason; the falsifier must bite the law rather than variant spelling", "exercise the gadget through an existing real production consumer of EmlGadgetStackSpec/EmlGadgetInstanceSpec, not a helper-only compile test; prove authored data reaches the ordinary EML execution path", "add EmlGadgetInstanceSpec to the constitutional-surface census in the same delta and plant an unlisted-variant mutant that REDs; preserve existing EML-OPCODE-LIBRARY and EML-STACK-LIBRARY counts", "prove POW remains absent and EXP/LN semantics, identities, admission shapes, and exactness evidence are unchanged", "run focused spec/compiler/consumer batteries plus ordinary-session blast radius; structural certificate at graduation", "evidence follows handoff_template: one docs/tests/authored_law_gadget_0_results.md, one current_evidence_index.md line, one 11.1f status-row edit, plus mechanically required inventory/lifecycle/gate rows; return exact base/head/tested_code_sha and hosted workflow IDs"]
stop_conditions: ["the law requires a third opcode or new EML execution mechanism rather than composition of admitted EXP/LN", "domain safety cannot be established at admission without weakening the existing LN domain contract", "the only proof path is helper/test-only and no existing production consumer can execute the authored gadget", "censusing EmlGadgetInstanceSpec requires weakening or bypassing the constitutional-surface mechanism", "the change requires touching #1803/11.2, 11.3+, 11.4, 12.x, or moving the pointer"]
---
## BUILD
- Add one **law-stating authored gadget** to `EmlGadgetInstanceSpec`: a power law whose canonical emitted EML is `EXP(k * LN(x))`.
- Compose only the already-admitted EXP/LN primitives. **Do not add POW** or any third opcode/executor.
- Make domain safety part of authoring admission: a law whose admitted input can reach forbidden LN inputs is rejected before runtime.
- Census `EmlGadgetInstanceSpec` as a constitutional vendor-facing vocabulary in the same delta.
## FENCES
- Existing EXP/LN semantics, exact identities, opcode/stack libraries, EML execution paths, and production consumers remain authoritative.
- A staircase or piecewise approximation is not a compatibility form for this gadget; intrinsic law semantics are the API.
- One rung only: #1803/11.2, 11.3+, 11.4, 12.x and pointer movement stay untouched.
## EXIT-PROOF
- Authored data using the new gadget reaches a **real existing production consumer** and executes the canonical `EXP(k * LN(x))` composition through ordinary EML machinery.
- An LN-unsafe authored law REDs at admission; a staircase/piecewise substitute REDs for violating the law-stating role; neither falsifier depends on the variant name.
- A planted unlisted gadget variant REDs against the new constitutional census; existing opcode/stack census remains unchanged and POW stays absent.
- Focused compiler/consumer tests, ordinary-session blast radius, and existing EXP/LN exactness/admission batteries are green at one exact head. Structural certificate runs at graduation.
- Evidence: `docs/tests/authored_law_gadget_0_results.md`, one current-evidence-index line, one 11.1f status-row edit, and only mechanically required inventory/lifecycle/gate rows.
