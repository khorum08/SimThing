# SIMTHING-AUTOMATON-INTRINSIC-0 results

- Track: 0.0.8.7 RF arena modernization (rung 6.0b)
- Status: **COMPLETE / DA-GRADUATED — merged #1591 @ af886019**
- HD-RECEIPT: `024c6a4ae906`
- Coding ORIENT-RECEIPT: `4a101ed6652d` · Orchestrator ORIENT-RECEIPT: `64dd5616eb47`
- Authoritative relay: Board `5161511025`
- implementation_code_sha: `73d87fc82abcf2d663bde4fcef3ad9367f68435f`
- tested_code_sha: `73d87fc82abcf2d663bde4fcef3ad9367f68435f`
- coverage_basis: PASS — DA re-ran every referee at the exact reviewed head and independently
  planted two defects to confirm the referees are load-bearing.
- **Merge provenance:** authored as PR #1590 on branch `codex/simthing-automaton-intrinsic-0`.
  The DA branched the asset repair (#1591) from the reviewed head rather than `master`, so the
  squash of #1591 carried this rung's implementation to `master` in the same commit. #1590 is the
  authoring PR; `af886019` is the merge that landed the code. Recorded here rather than repaired by
  reverting independently-verified working code.

## Exit proof

| Contract | Evidence |
|---|---|
| No new inbox | `SimThing` gains **no field**. The +21 lines in `simthing.rs` are `walk_inherited_until`, a shared function. `overlays: Vec<Overlay>` remains the reception surface. |
| Reception in three modes | Deficit-driven rides `apply_owner_silo_runtime_disburse_down_cpu` → `apply_runtime_local_allocations_from_disburse_down`; standing rides the shared inheritance walk; predicate-broadcast is one paid subtree walk. |
| No second transport | `automaton_reception.rs` **calls** the existing disburse-down seam with canonical `{OwnerRef, ResourceKey, ScopeId}` keys. It fails closed when either receiver or origin is out of tree. |
| Required origin | `Overlay.origin: SimThingId` — plain, no `Option`, no default. A `compile_fail,E0063` doctest proves omission is a type error, and `cargo test --doc -p simthing-core` **executes it** (23 passed). |
| Routed filtering | `deficit_directive_routes_origin_to_lca_to_target_and_policy_filters_it` — one delivered directive observes intermediate policy mutation, suspension and removal without re-delivery. |
| Conjunctive predicate composition | `allows_routed_predicate` accumulates `allowed &= …` over the stack. A descendant `Set(1.0)` moves the candidate value but cannot reopen admission an ancestor's `Set(0.0)` closed. |
| Value composition unchanged | The same test asserts the descendant's composed value **is** `1.0` — selection is conjunctive while ordinary value composition stays sequential. The two are proven distinct. |
| Shared inheritance walk | `walk_inherited_until` is called from `evaluate.rs` (standing) and `owner_channel.rs` (owner resolution). Descendants retain no copied standing state. |
| Inert by default | `overlays.capacity()` assertions prove an inert SimThing stores zero bytes for reception and origination. |
| Derived spatial location | No coordinate is stamped on the overlay or event path; location derives from origin. |
| Conservation untouched | Standing directives make no conservation claim. The Invariant Set is unchanged. |

## DA falsification — defects planted independently at the reviewed head

A referee that does not fail when its defect is planted is decoration. Both were planted by the DA,
not taken from the relay's claim.

| Planted defect | Result |
|---|---|
| `allowed &= …` → `allowed = …` (conjunctive AND-fold becomes a sequential last-wins selector) | **RED** — `predicate_broadcast_…_are_conjunctive` fails at `receipts.len()`, `left: 2, right: 1`. The descendant reopens admission and receives the broadcast. |
| Post-allocation delivery loop removed from `receive_command_deficits_from_disbursement` | **RED** — `command_deficit_rides_disbursement_and_arrives_with_live_route_policy` fails `left: 0, right: 1`. RF allocation still succeeds; nothing arrives. |

Both files were restored to the exact reviewed head and re-verified green before the ruling.

## Batteries

- `simthing-core` automaton referees: 3 passed · driver RF reception referees: 2 passed
- Full sweep `simthing-core` / `-sim` / `-driver` / `-kernel` / `-spec`: **0 failures**
- `cargo test --doc -p simthing-core`: 23 passed (carries the required-origin proof)
- `anchor_check` PASS · `doc_budget` PASS · `gen_orientation` PASS · `gen_digest` PASS
- `artifact_provenance` PASS · `detachability` PASS · residue `scenario=0 domain=0`

## Findings raised during review

- **A pre-existing red the relay did not disclose.** `existing_authored_ron_capability_trees_admit_unchanged`
  had been failing since the `Permanent` → `UntilDissolved` rename (`10d9d639`): nine authored RON
  assets named a variant that no longer existed. Verified to pre-date base `9ad91df5`, so **not this
  rung's defect**. Repaired in #1591. The relay attributed the workspace stop to ClauseThing debt
  alone; it also hit `simthing-spec` and `simthing-driver`.
- **No CI workflow runs `cargo test`.** The four workflows are grep/doctrine gates, which is why a
  red asset-admission test survived unnoticed across multiple rungs.
- **ClauseThing test-target compile errors are genuinely inherited** (`ColumnIndex`/`u32`, missing
  `admission_disposition`) and unrelated to overlay work — the relay's characterisation of those is
  accurate. Not fixed here.
- **Evidence deliverables were missing.** The rung landed its `test_inventory` rows but neither this
  results doc nor a `current_evidence_index.md` line; both authored by the DA at graduation.
