---
rung: STEMTHING-CENSUS-TIER2-SESSION
kind: transport
track: 0.0.8.7
base_sha: 380e8630
audience: da
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(novelty)
owner_notes: "DA SESSION HANDOFF, not a coding dispatch. You are the Design Authority for 0.0.8.7. Phase 6 is COMPLETE (7/7) and the pointer is PARKED at `none` because the constitutional pointer-flip is an OWNER act, not yours. Your job: finish the StemThing §3.1 slot-bearing-artifact census, author the Tier-2 amendment from what it finds, and integrate the StemThing doc into the workplan. OWNER PRE-APPROVAL (2026-08-03): core design + constitution were already updated for the EXP/LN EML additions and the StemThing unification; ANY FURTHER doc/constitution changes the amendment requires are pre-approved — author them, do not ask. DO NOT dispatch 7.1: it waits behind StemThing-A. 5.11/5.12 (EXP/LN) run in PARALLEL in the same Phase 6->7 gap on disjoint files and lanes; neither stream waits on the other."
surfaces: ["docs/stead_stemthing_unification.md", "docs/workshop/stemthing_slot_census_methodology.md", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/simthing_core_design.md", "docs/full_eml_unification.md", "scripts/ci/doctrine_anchors.tsv", "scripts/ci/anchor_triggers.tsv", "scripts/ci"]
forbidden: ["dispatching 7.1 or any Phase 7 row before StemThing-A lands", "flipping the constitutional pointer -- that is an OWNER act; park at `none` and ask", "publishing the census as exhaustive without the reconciliation line -- false completeness is the defect this track keeps catching", "treating the illustrative 3.1 artifact list as the census", "reinterpreting the core-design stability law instead of amending it", "blocking 5.11/5.12 on StemThing or vice versa -- disjoint files, disjoint lanes, no shared gate"]
required_checks: ["fresh da ORIENT-RECEIPT before edits", "census reconciliation line: row count vs type-walk consumer count vs grep residue", "every grep hit either a census row or recorded provably-non-slot WITH REASON", "one BLOCKER verdict => shape (b); do not force shape (a)", "rungclose PASS on any rung you stamp; both exit-proof and status cells", "exit_proof_coverage_check.sh clean on any row you touch"]
stop_conditions: ["a BLOCKER is found -- authoritative identity depends on physical permanence with no lawful rebind path; report it, do not engineer around it", "the amendment appears to require reinterpreting rather than amending standing law", "the census cannot be reconciled and would have to be published as curated", "any step appears to need the Owner's constitutional pointer-flip to proceed"]
---
## BUILD
- **Re-orient first.** `bash scripts/ci/orient.sh --role=da`, carry the receipt. At `380e8630` it is `b6c7c21eea9b`, rule stamp `5554b2613f8907ff` — recompute rather than trusting these. Resolve anchors via `anchor_query.sh`, never raw doctrine greps. `stead-rejected-shapes` is binding: read it before proposing anything.
- **Finish the census** per `docs/workshop/stemthing_slot_census_methodology.md` §2–4. Three findings are already established and re-verifiable; the remaining work is the exhaustiveness proof, not a restart.
- **Author the §3.1 Tier-2 amendment** from what the census actually finds — restating the stability law as *stable within an epoch; rebindable only at a recorded boundary remap*, plus the enumeration. Cite the EXISTING rebind path; do not propose a new one.
- **Integrate the StemThing doc into the workplan**: mint its rungs in §3b (blocked until the Tier-2 ruling lands, per §10), anchor the doc, and update core design where the amendment changes paradigm text.
## FENCES
- **The pointer stays `none` until the Owner flips it.** Phase 6 is complete; StemThing-A is DA-lane work followed by an Owner decision. `gen_orientation` will reject a pointer naming a rung absent from the ladder — it caught exactly that mistake at the 6.3 stamp.
- **CI runs no `cargo test`** by standing Owner ruling. Green checks prove nothing about test health; run crate batteries locally and `cargo test --doc` explicitly for compile-fail seals (all 93 doctests are compile-fail; only 2 are error-code pinned).
- `handoff_dispatch.sh --lint` does **not** check the 60-line coding-projection cap. Render the projection before dispatching anything.
- Reconciliation is enumeration, not test execution — the constitutional-change law (§4) requires a static blast-radius sweep across `.ron`/`.clause`/fixtures, not just `--include=*.md`.
## EXIT-PROOF
- Census TSV lands with the **reconciliation line**: row count vs type-walk consumer count vs grep-residue disposition, every residue hit either a row or justified out.
- §3.1 Tier-2 amendment lands with shape (a) or (b) ruled **from the census**, not asserted.
- StemThing rungs minted in §3b with exit-proofs that cover their scope (`exit_proof_coverage_check.sh` clean).
- `full_eml_unification.md` and `stead_stemthing_unification.md` anchored: rows in `doctrine_anchors.tsv` + wiring in `anchor_triggers.tsv`. Both are currently **governing but unanchored** — zero content hash, unreachable via `anchor_query.sh`.
- All gates green: anchor / orientation / doc-budget / detachability / residue / rungclose.
