---
rung: DIMENSION-FINALIZATION-SEAM-0
kind: remedial
track: 0.0.8.7
base_sha: b6e4837d23eed1a708f6671b350b29bcc7279adc
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Minted after a THIRD lawful STOP on this arc. Sol was right to stop rather than launder a scenario rewrite through the facade. The seam 11.1a landed executes, but only when the caller first rewrites GameModeSpec.region_fields[*].n_dims to the post-admission width - see field_sweep_session_seam_0.rs:236. A vendor cannot do that through the five verbs and should not have to predict the final width. The falsifier for this rung is a DELETION: that line comes out of the 11.1a witness and the witness still passes."
surfaces: ["crates/simthing-driver/src", "docs/tests", "scripts/ci/test_inventory.tsv"]
forbidden: ["a second execution path, bespoke field kernel, new evaluator, authority, scheduler, or registry; stead section 10 routes through EXISTING ops", "requiring the caller to mutate GameModeSpec or any authored spec to make widths agree; that is the defect, not the fix", "requiring the caller to predict the final width; a prediction helper that survives as REQUIRED means the rung has not landed", "defaulting Triad columns at install; 5.8b keeps them explicit consumer inputs", "more than one production surface owning the final width; single authority is the point", "Vendor Door or facade work - that is 11.1c; and the 11.2 guide, exemplars or admission gate", "11.1d, 11.1e, 11.1f, 11.3, 11.4, 12.x, StemThing-B, Vector CostBand, ClauseThing debts"]
required_checks: ["before edits render/read this handoff; fresh Frontier coding ORIENT receipt at rule stamp 61818ff7d4adda84; ACK every rendered REQUIRED-ANCHOR", "FIRST STEP archaeology: trace the exact production order in session.rs - install_atomic at the authored width, admit_comparative_from_field_plan growing the registry, install_session_mapping compiling from the original GameModeSpec - and state precisely where the finalization stage attaches", "land ONE production surface that binds admitted registrations and the ordinary mapping to the same final registry width, after derived-column admission and before ordinary region-field and caller registration compilation", "the 11.1a witness drops field.n_dims = final_n_dims and still passes; that deletion is the rung's primary falsifier", "no caller-supplied width prediction is required anywhere; any projected-dimensions helper becomes optional or is deleted", "planted second-finalization-site defect REDs for its own named reason", "planted install-time Triad-column default REDs, proving 5.8b preserved", "convergence re-proven by diff: no new executor, no second execution path", "real-adapter witness that an ordinary session executes admitted sweeps end to end without touching authored spec", "inventory rows for new tests; lifecycle, detachability, DOC-BUDGET, orientation, sanctioned digest, Agent and Doctrine green; inspect every hosted Doctrine Scan step; return exact base/head/tested_code_sha plus workflow ids"]
stop_conditions: ["binding both sides to the final width appears to require a new authority, evaluator, or a second execution path", "the final width cannot be known at the finalization point without caller input - name exactly what is missing and stop", "preserving 5.8b and removing the caller rewrite appear mutually exclusive", "the fix would require more than one production surface to own the final width"]
---
## BUILD
- Land the missing **dimension-finalization stage** in the ordinary admitted-sweep seam. Today the widths disagree by construction: `install_atomic` compiles the ordinary field plan at the **authored** `RegionFieldSpec.n_dims`, `admit_comparative_from_field_plan` then **grows** the live registry, and `install_session_mapping` compiles the ordinary mapping from the **original** `GameModeSpec.region_fields`. Attachment REDs with `AdmittedFieldSweepBindingMismatch { actual_dims: 57, expected_dims: 25 }`.
- **One production surface owns the final width**, placed after derived-column admission and before ordinary region-field and caller registration compilation, binding both sides with **no caller mutation of authored spec** and **no width prediction**.
- Remove the workaround at its source: `field_sweep_session_seam_0.rs:236` (`field.n_dims = final_n_dims`) comes out, and the 11.1a witness still passes.
## FENCES
- **The caller never rewrites authored spec to make widths agree.** That rewrite is the defect this rung exists to delete, not a pattern to relocate. Hiding it behind Bind, rebuilding `GameModeSpec`, pre-padding, or hardcoding a width are all the same defect wearing a different coat.
- **The caller never predicts the final width.** If a projected-dimensions helper is still *required* after this rung, the finalization stage did not land.
- **Single authority.** Exactly one production surface owns the final width; a second finalization site is the failure mode and must RED.
- **stead section 10 convergence unchanged:** route through existing ops. No new executor, no second execution path, no bespoke kernel.
- **5.8b (DA `5154348081`) unchanged:** ordinary install still defaults no Triad column; they remain explicit consumer inputs.
- No door or facade work. 11.1c re-exports once this seam is actually reachable.
## EXIT-PROOF
- **The primary falsifier is a deletion:** the 11.1a witness drops `field.n_dims = final_n_dims` and passes. Today removing that line REDs with `AdmittedFieldSweepBindingMismatch`; after this rung it must succeed, which is the whole claim in one line.
- **No prediction required:** a caller supplies no width; any projected-dimensions helper is optional or deleted.
- **Single authority proven:** a planted second finalization site REDs for its own named reason on the production path.
- **Convergence and 5.8b re-proven:** no new executor by diff; a planted install-time Triad default REDs.
- **Real-adapter witness:** an ordinary session executes admitted PALMA and Gu-Yang sweeps end to end and observes comparative output **without touching authored spec**.
- Evidence: `docs/tests/dimension_finalization_seam_0_results.md`, inventory rows for new tests, ladder row stamped at merge with `merged #<PR> @ <sha>`.
