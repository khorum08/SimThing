---
rung: EXACT-CAP-PROJECTION-0
kind: rung
track: 0.0.8.7
base_sha: 80e80d527f997425a7191a874a3dd6dd6419e6e9
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(binding)
owner_approved: true
owner_notes: "Owner ruling + DA mint 5548825933 binds the exact law verbatim: CAP COLLISION = SATURATE AND REDISTRIBUTE. DA cross-rung adjudication 5548783236 requires bounded active-set/water-filling inside each precedence equality band and freezes all no-collision outputs bit-identical. DA graduation 5552278732 closed 15.8, stamped master 80e80d527f997425a7191a874a3dd6dd6419e6e9, and opened 15.9 as the sole active pointer."
surfaces:
  - crates/simthing-kernel/src/resident_clearing_apportionment.rs
  - crates/simthing-kernel/src/shaders/resident_clearing_apportionment.wgsl
  - crates/simthing-kernel/tests/exact_cap_projection_0.rs
  - crates/simthing-workshop/tests/exact_cap_projection_0.rs
  - docs/tests/exact_cap_projection_0_results.md
  - scripts/ci/test_inventory.tsv
  - scripts/ci/anchor_reach_log.tsv
forbidden:
  - crates/simthing-kernel/src/clearing_weight_projection.rs is not the cap-settlement authority; do not move the remedy there
  - no new market, projection authority, settlement path, host floating-point lambda, compatibility fallback, scorer, or second rounding/tie law
  - no redistribution across hard-precedence equality-band boundaries
  - no clip-and-drop remainder behavior; a feasible cap-collision vector may not fail-close or leave allocatable supply stranded
  - no weakening or replacement of exact Q149 basis identity, Hamilton/largest-remainder, deterministic tie rotation, semantic row identity, or canonical product laws
  - no edits to 15.10 GENERATION-ABORT-SAFETY-0 semantics, failure/retry policy, generation poisoning, or rollback
  - no canon rewrite, pointer movement, graduation stamp, known-remand marker deletion, or successor DEPARTING-STREAM-DISPOSAL-0 work
  - no workflow or gate-code edits under .github/workflows/** or scripts/ci/**/*.sh|*.py without STOP and explicit DA/Owner authority
required_checks:
  - render/read this handoff first; obtain a fresh coding ORIENT under orientation_rule_stamp 53a4ada59778a8b5 and ACK every rendered REQUIRED-ANCHOR before implementation
  - ARCHAEOLOGY FIRST: census every current production caller of the exact resident apportionment CPU oracle and WGSL executor, every cap/error propagation surface, and the current frozen parity/permutation/workgroup/partition/tie/no-collision referees; return exact paths in the proof packet
  - FALSIFIER FIRST: before production edits, add and run the Owner two-row cap-collision witness [1,100] with S=101 and prove stamped-master behavior REDs by refusing/overflowing instead of yielding lawful (1,100); preserve the raw RED transcript
  - implement only the Owner law: CAP COLLISION = SATURATE AND REDISTRIBUTE
  - within each hard-precedence equality band compute the exact feasible projection g_i = min(r_i, lambda*b_i) with total equal to min(S_remaining, sum executable requests); implement it as a bounded active-set/water-filling projection over the existing exact basis representation
  - active-set loop law: proportional exact apportionment over active rows; any row whose candidate share exceeds request cap freezes exactly at request; subtract frozen exact caps from remaining band supply; remove frozen rows from active basis; repeat until no active row breaches cap
  - each non-final active-set iteration must make monotone progress by freezing at least one row; no unbounded retry/search loop
  - Hamilton/largest-remainder and the existing deterministic granter+generation tie rotation apply only to the final active set after all cap collisions are frozen; do not create a second remainder/tie authority
  - preserve hard precedence exactly: earlier equality bands consume only their lawful exact grant before later bands; excess within one band never crosses backward or bypasses precedence
  - preserve Q149 identity exactly: no host floating conversion of lambda, request caps, basis totals, quotients, or remainders may become economic authority
  - mirror one exact algorithm in CPU reference and WGSL production; CPU/GPU must agree for cap-collision and no-collision cases, with no posture-specific semantics
  - mandatory GREEN witness: Owner two-row [1,100], S=101 -> exactly (1,100)
  - mandatory GREEN witness: a three-row case with multiple capped rows that proves remaining supply is redistributed rather than clipped or stranded
  - include a case that requires more than one active-set freeze iteration if the exact chosen bases make such a case representable; otherwise mechanically explain why the three-row witness exhausts the bounded loop shape
  - prove row permutation invariance by semantic identity on the new cap-collision corpus
  - prove W32/W64 workgroup invariance and single-pass/partitioned-dispatch invariance on cap-collision corpus using the existing physical-dispatch controls
  - prove existing no-collision corpus BIT-IDENTICAL to stamped-master expected products, including grants, unresolved, canonical ordering, and status; do not update golden expectations merely because the implementation changed
  - rerun and preserve Q149 exact-basis witnesses, neutral-request identity, exact tie rotation across generation/granter, hard precedence, E6 work-conserving zero-basis/mixed-band behavior, semantic-row permutation, and resident CPU/GPU parity
  - prove total grant per precedence band equals min(remaining supply, sum executable request caps) whenever a feasible vector exists; request-cap collision alone is no longer a typed refusal
  - preserve typed refusal for genuinely invalid continuous input, arithmetic overflow outside the feasible projection domain, impossible indexing/admission, zero dispatch partition, and other pre-existing malformed inputs
  - authority census must not grow: one resident exact apportionment authority, one existing canonical product path, no new host/economic adapter or private solver
  - run touched-package checks, focused exact-cap referee, frozen resident apportionment/parity corpus, full structural battery, inventory/drift, constitutional/census checks, lifecycle/sanctioned-surface/anchor checks, detachability, Agent Scan, hosted Doctrine Scan and hosted Doctrine Exec
  - return PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED with exact base/head, HD/ORIENT receipts, archaeology table, RED-before/GREEN-after transcripts, CPU/GPU algorithm correspondence, cap-active-set termination argument, no-collision bit-identity certificate, frozen-invariant results, authority census, changed-file ledger, hosted run IDs, and every INSPECT result for orchestration triage
stop_conditions:
  - the Owner [1,100]/S=101 witness cannot be made to exercise the existing exact resident apportionment authority without introducing a new economic author or test-only semantics
  - lawful saturation+redistribution would change any no-collision product bit, Q149 basis identity, tie winner, hard-precedence result, E6 work-conservation result, semantic row ordering, or existing canonical product meaning
  - a correct implementation would require host floating-point lambda/projection, a second solver/rounding path, a new settlement authority, or redistribution across precedence bands
  - CPU and WGSL cannot express the same exact bounded active-set law under the existing admitted integer/Q149 machinery
  - implementation requires touching a production surface outside the two bound apportionment files or changing gate code; STOP with the exact source/consumer reason before broadening
  - any required frozen witness, authority census, structural certificate, hosted gate, inventory, or anchor check is red after the remedy
---

## BUILD

The sole semantic change is the cap-collision disposition inside the already-admitted exact resident apportionment authority.

**OWNER LAW (binding, verbatim): CAP COLLISION = SATURATE AND REDISTRIBUTE.**

Current stamped source already owns the correct economic inputs and exact arithmetic. `resident_clearing_apportionment.rs` converts the admitted continuous basis to exact common Q149, partitions by hard precedence, computes exact quotient/remainder shares, and applies deterministic Hamilton tie rotation. Its present post-share guard rejects a row whose computed grant exceeds `requested`; that is the 15.9 falsifier edge. The WGSL file is the production counterpart and must implement the same semantics.

For each precedence equality band, with active exact bases `b_i`, request caps `r_i`, and remaining supply `S_remaining`, the lawful result is the unique bounded water-filling form `g_i = min(r_i, lambda*b_i)` whose exact total is `min(S_remaining, sum executable requests)`. Implement by repeatedly freezing any cap-breaching rows at exact request, subtracting those caps, removing their basis from the active denominator, and re-apportioning the remainder. Only the final active set receives the existing Hamilton/largest-remainder residue assignment and existing deterministic tie rotation.

The active-set machinery is an internal refinement of the one graduated projection; it is not a new market, solver, demand authority, or settlement lane. No-collision execution must take the same numerical path/result as stamped master at the externally visible product boundary.

## FENCES

15.8 is graduated and may be consumed but not reopened. 15.10 is fenced. `DEPARTING-STREAM-DISPOSAL-0` is proposed-only and gates nothing. Canon rewrite/adoption and closeout remain later work.

Do not use `clearing_weight_projection.rs` as a substitute authority: that file governs weight-span projection, not this exact cap-constrained settlement. Do not repair cap collisions by clipping grants after Hamilton and discarding excess, by iterating host floats, by widening precedence, or by adding an alternate CPU-only/GPU-only algorithm.

If the existing exact arithmetic cannot express the Owner law while preserving the frozen no-collision outputs, STOP for DA rather than changing the law.

## EXIT-PROOF

Exit requires all of the following on one tested implementation head:

1. stamped-master RED for the Owner `[1,100]`, `S=101` cap collision;
2. GREEN exact `(1,100)` on CPU and GPU;
3. GREEN three-row multiple-cap redistribution, plus bounded-loop progress evidence;
4. cap-collision row-permutation, W32/W64, and dispatch-partition invariance;
5. no-collision outputs proven bit-identical to stamped master;
6. Q149 identity, Hamilton tie rotation, hard precedence, E6 work-conservation, semantic ordering, and canonical product laws unchanged;
7. no new economic/settlement authority and unchanged authority census;
8. structural/inventory/anchor/hosted certificates zero-red;
9. coding return remains PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED for independent orchestration triage, exact-head clearance, relay-lint, and DA graduation review.
