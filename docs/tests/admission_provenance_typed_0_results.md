# ADMISSION-PROVENANCE-TYPED-0 results

- Track: 0.0.8.7 RF arena modernization (rung 13.1)
- Status: **PROBATION / proof-present / DA-review-pending**
- Branch: `codex/admission-provenance-typed-0`
- dispatch_base_sha: `ebd1d5577b453a94739eff531ae944803aa7f51f`
- implementation_checkpoint_sha: `d6734c3f188e65f93d22ae45a12f387bd9593745`
- accepted_production_head / structural_tested_sha: `efe9557d2b457387597f1b926ac50ec78a357751`
- remand_base_sha: `efe9557d2b457387597f1b926ac50ec78a357751`
- final_head_sha: PR/Board-relay-bound after the evidence commit; this file does not self-hash
- dispatch_binding: Board `5462866469`; orchestration amendment `5462922116`; Phase-13 authority `5462798074`
- census_binding_remand: DA ruling `5463464560`; orchestration dispatch `5464053422`
- HD-RECEIPT: `fdcb3140ae93`
- ORIENT-RECEIPT: `65d1ff95529f`
- role: `coding`
- orientation_rule_stamp: `cf4a20680136ef8f`
- orientation_digest_sha: `54fffe1fab55b4d2b5e1ae5ed8399e708c581894b93df33cc2207f2087301d1e`
- expected_route: `DA-RESERVE(binding)`

## Outcome

The existing production root-admission seam now returns one domain-neutral
`SpecError::AdmissionRefused { law_id, element_path }` shape. Its payload is
exactly the already-enforced law identity and deterministic authored/admitted
element identity. The existing wrapper chain remains mechanical and typed:

```text
SpecError::AdmissionRefused
  -> InstallError::Spec
  -> SessionError::Install
  -> SimSession::open_from_spec
```

No companion law enum, diagnostic registry, facade result, application
vocabulary, or alternate admission entrypoint was added. The five censused
predicates are unchanged: accepted configurations still admit, and rejected
configurations still refuse at the same predicate. Only refusal provenance and
its error shape changed.

## First-step and post-change census

The exact amended dispatch base was censused before the transplant edits. A
tree-wide literal census found 128 `SpecError::ValidationFailedAt` constructors:
89 in `simthing-driver`, 37 in `simthing-spec`, and 2 in
`simthing-mapeditor`. The five authored root-admission collapses in
`crates/simthing-driver/src/install.rs` were the complete 13.1 target.

| Classification | Base | Tested code | Disposition |
|---|---:|---:|---|
| Whole-tree `SpecError::ValidationFailedAt` literals | 128 | 123 | Exactly the five target constructors were removed. |
| Targeted `install.rs` root-admission collapses | 5 | 0 | Promoted to `AdmissionRefused`. |
| Called-path internal invariant/error mappings (`gated_rates.rs` 2, `resource_flow_compile.rs` 1) | 3 | 3 | Pre-existing, outside the five authored 13.1 target; unchanged. |
| All other non-target literals | 120 | 120 | Unchanged; no silent reclassification or cleanup campaign. |

Thus the exit proof is zero *targeted* generic fallbacks, not a false claim
that the legacy variant disappeared globally. `ValidationFailedAt` remains for
the 123 non-root compilation/reporting constructors outside this handoff.

## Promoted law and element authority

| Targeted rejection site | `law_id` | Deterministic `element_path` authority |
|---|---|---|
| Standalone-overlay compile diagnostics are nonempty | `standalone-overlay-compile-diagnostics-empty` | `domain_packs.overlays[id="<authored overlay id>"]` |
| Qualified-host economy-property registry lookup | `resource-economy-property-registered` | `resource_economy.properties[key="<namespace>::<name>"]` |
| Economy-property materialization registry lookup | `resource-economy-property-registered` | Same authoritative property key; both passes share one constructor. |
| Admitted economy-property host is absent from the live tree | `resource-economy-property-host-live` | `simthings[id=<admitted raw id>].properties[key="<namespace>::<name>"]` |
| Base-flow-obligation participant lacks its admitted flow property | `base-flow-obligation-participant-property-live` | `resource_flow.base_obligations[id="<obligation id>"].participants[id=<admitted raw id>].properties[key="<namespace>::<name>"]` |

Every value comes from identity already in scope at the refusing predicate.
No filename, function name, invented Scenario field, or ClauseThing term is
used as element identity.

## Semantic-equivalence and wrapper proof

| Pre-existing predicate | Before | After | Acceptance frontier |
|---|---|---|---|
| Standalone overlay compiles with diagnostics | Refused at `standalone_overlay_compile` | Refused with overlay law/id path | Unchanged |
| Economy property key is absent from the admitted registry, hosted or unhosted | Refused at `resource_economy_property_placement` | Refused with registry law/property path | Unchanged |
| Admitted economy property host id is absent from the live tree | Refused at `resource_economy_property_host` | Refused with host law/SimThing+property path | Unchanged |
| Base-obligation participant lacks the admitted flow property | Refused at `base_flow_obligation_participant_property` | Refused with obligation law/participant+property path | Unchanged |
| Registered `core::loyalty` property on the ordinary root host | Admitted | Admitted through `SimSession::open_from_spec` | Unchanged positive witness |

The focused integration proof destructures
`SessionError::Install(InstallError::Spec(SpecError::AdmissionRefused { ... }))`
directly from `SimSession::open_from_spec`. It checks hosted and unhosted
registry failures, the admitted-host failure, and the paired accepted fixture.
No assertion parses `Display`.

The install-module proof additionally binds the standalone-overlay and
base-obligation cases to their exact laws and paths through the same wrapper
types.

## Load-bearing falsification

A temporary production mutant changed the economy-property law literal from
`resource-economy-property-registered` to `planted-wrong-law-id`. The exact
open-from-spec witness went RED with:

```text
left:  "planted-wrong-law-id"
right: "resource-economy-property-registered"
```

The mutant was restored before the tested checkpoint. The complete focused
battery was then rerun green at `tested_code_sha`. This proves the witness is
bound to machine-readable law identity rather than merely to rejection or
formatted text.

## Constitutional row

The single existing `ROOT-CONTRACT-ADMISSION-ERROR` row was updated in place.

| Field | Before | After |
|---|---|---|
| Admitted member | `ValidationFailedAt` | `crates/simthing-driver/src/install.rs::AdmissionRefused` |
| Production witness | `SpecError::ValidationFailedAt {` | `SpecError::AdmissionRefused {` |
| Blocker | Retire only when generic failure is unrepresentable and provenance is typed | DA approval for any production root-admission fallback not carrying `law_id` and `element_path` |
| Posture/deferral | Production / none | Production / none |

The row uses the existing `regex-symbol` parser over the spec declaration and
driver consumer. D1 extends its data declaration to admit only
`AdmissionRefused` while harvesting both `SpecError::ValidationFailedAt { ... }`
and the legacy generic nullary shapes as out-of-set symbols. The checker gains
exactly one DA-authorized selftest plant for the braced constructor; the
pre-existing `root-nullary` plant remains load-bearing. No sibling checker or
workflow was added.

### D1 census-binding remand

Final declaration regex:

```regex
(?:SpecError::(AdmissionRefused|ValidationFailedAt)(?=\s*\{)|\b(?:ValidationFailed|AdmissionFailed|InvalidSpec|InvalidConfiguration|InvalidState)(?=\s*,))
```

Fable's exact acceptance falsifier was replayed by temporarily adding a fresh
production `SpecError::ValidationFailedAt { site: "simthing-driver/install" }`
use to `install.rs`. The checker RED was:

```text
CONSTITUTIONAL-SURFACE-VERDICT: FAIL errors=1
  - ROOT-CONTRACT-ADMISSION-ERROR: registry drift added=['crates/simthing-driver/src/install.rs::ValidationFailedAt'] removed=[]
```

The mutant was restored byte-for-byte before the final head. The old nullary
plant was also replayed and still REDs the same row with
`added=['crates/simthing-spec/src/error.rs::None']`; the permanent full
selftest passes at exactly `planted=12 valid_binding=1`.

## Local evidence at the tested code identity

| Command | Result |
|---|---|
| `cargo check -p simthing-spec -p simthing-driver` | PASS |
| `cargo test -p simthing-spec --no-fail-fast -j 1` | PASS — 49 passed including doctests |
| `cargo test -p simthing-driver --lib --no-fail-fast -j 1` | PASS — 17 passed |
| `cargo test -p simthing-driver --test admission_provenance_typed_0 --no-fail-fast -j 1` | PASS — 3 passed |
| `cargo test -p simthing-embedder --tests --no-fail-fast -j 1` | PASS — 15 passed |
| `bash scripts/ci/constitutional_surface_check.sh --check` | PASS — `ROOT-CONTRACT-ADMISSION-ERROR=1` |
| `bash scripts/ci/constitutional_surface_check.sh --selftest` | PASS — `planted=12 valid_binding=1` |
| `bash scripts/ci/test_inventory_check.sh` | PASS — 1,354 discovered / 1,354 ledgered; lifecycle PASS |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS — unledgered 0, stale 0 |
| `bash scripts/ci/anchor_check.sh --check` | PASS — pending/curation PASS; standing coverage INSPECT retained |
| `bash scripts/ci/gen_orientation.sh --check` | PASS before the evidence/design update |
| `bash scripts/ci/orient.sh --selftest` | PASS — 4 fixtures |
| `bash scripts/ci/handoff_dispatch.sh --lint handoffs/ADMISSION-PROVENANCE-TYPED-0.hd.md` | PASS |
| `bash scripts/ci/handoff_dispatch.sh --receipt handoffs/ADMISSION-PROVENANCE-TYPED-0.hd.md` | PASS — `fdcb3140ae93` |
| `bash scripts/ci/doctrine_selftest.sh` | PASS |
| `bash scripts/ci/doctrine_scan.sh` | INSPECT — 0 hard failures; 417 standing whole-tree heuristic hits (`SIM-KIND-READ` 1, `SPEC-LOWERER-KIND-READ` 416) |
| `bash scripts/ci/overlay_germ_archaeology_census_check.sh --check` | PASS — `unclassified=0 open=0`; refusal helper kept domain-neutral and outside the overlay archaeology symbol census |
| `bash scripts/ci/gen_digest.sh --check` | PASS after deterministic sanctioned-surface regeneration |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |
| `bash scripts/ci/agent_scan.sh --base ebd1d557... --head d6734c3f...` | PASS — no hard or delta finding |
| `git diff --check ebd1d557... d6734c3f...` | PASS |
| `cargo fmt --all -- --check` | PASS |

### Structural/full-workspace certificate

At the exact tested code identity:

```text
cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet
STRUCTURAL-CERTIFICATE-SUMMARY suites=124 passed=472 failed=3 ignored=14 exit=101
```

The three failures are the inherited pre-13.2 ClauseThing baseline, not a new
13.1 acceptance failure:

1. `gpu_category_micro_economy_matches_arena_allocation_oracle` remains RED at
   `open_from_spec`; its formerly site-only refusal is now the exact typed
   `base-flow-obligation-participant-property-live` law with obligation,
   participant `id=2`, and `simthing::settlement_food_flow` property path.
2. `gpu_scatter_projection_matches_cpu_oracle_through_commitment` remains RED
   at the same predicate with the same typed law/path.
3. `star_naming_canonical_tp_all_systems_have_display_names` remains RED for
   its pre-existing stale canonical star-name golden.

No ClauseThing test, fixture, assertion, golden, lowering, or production file
was edited. The two named admission reds are the diagnostic witness handed to
13.2; coding did not make them green or weaken their oracles.

## D1 remand diff census

Exactly six files differ from accepted production head `efe9557d`:

| Path | D1 change |
|---|---|
| `scripts/ci/constitutional_surfaces.tsv` | Extend the existing ROOT row's data declaration to harvest the braced legacy constructor. |
| `scripts/ci/constitutional_surface_check.sh` | Add exactly one authorized braced-root plant and assert the existing named registry-drift reason. |
| `scripts/ci/anchor_reach_log.tsv` | Append the remand's scanner-selftest anchor query. |
| `docs/tests/admission_provenance_typed_0_results.md` | Record the D1 falsifiers and inherited certificate posture. |
| `docs/tests/current_evidence_index.md` | Keep the current evidence summary truthful. |
| `docs/sanctioned_surface.md` | Regenerate the deterministic checker/data digest. |

There is no Rust production or test-file delta from `efe9557d`; the two
temporary acceptance mutants were restored byte-for-byte.

## Exact changed-file census and containment

The final evidence head contains twelve changed files relative to the amended
dispatch base:

| Path | Change |
|---|---|
| `crates/simthing-spec/src/error.rs` | Add the single two-field domain-neutral typed refusal; retain the legacy non-root shape. |
| `crates/simthing-driver/src/install.rs` | Promote exactly five root-admission constructors without changing predicates. |
| `crates/simthing-driver/tests/admission_provenance_typed_0.rs` | Public open-from-spec negative/positive proof. |
| `scripts/ci/constitutional_surfaces.tsv` | Update the one existing ROOT row in place. |
| `scripts/ci/constitutional_surface_check.sh` | Add exactly one DA-authorized braced-root selftest plant and named-reason assertion. |
| `scripts/ci/test_inventory.tsv` | Ledger the four authored tests. |
| `scripts/ci/anchor_reach_log.tsv` | Append the exact-base orientation anchor ACKs. |
| `docs/sanctioned_surface.md` | Deterministically regenerate the digest for the changed constitutional data row. |
| `docs/tests/admission_provenance_typed_0_results.md` | This evidence packet. |
| `docs/tests/current_evidence_index.md` | One current-evidence line. |
| `docs/design_0_0_8_7_rf_arena_modernization.md` | Rung 13.1 only: TODO to PROBATION; pointer unchanged. |
| `docs/orchestrator_orientation.md` | Deterministically regenerated active-rung projection. |

There is zero delta in `simthing-core`, `simthing-kernel`, `simthing-gpu`,
`simthing-sim`, `simthing-clausething`, ClauseThing hydration,
`Scenario.field_plan_admission`, FieldPlan/comparative/default-birth/triad
grammar, budgets, Vector CostBand, or workflows. The only checker-code delta is
the one authorized selftest plant and its named-reason assertion; production
Rust is byte-identical to accepted head `efe9557d`. The active pointer remains
on 13.1, and the new 13.4-A1 surface is untouched.

## Routing

Hosted Doctrine Scan, final run identities, the exact evidence head, and fresh
body/head-bound `/clearance` plus `/relay-lint` are attached to the draft PR
and Board return after push. Coding returns **PROBATION / proof-present /
DA-review-pending** to `DA-RESERVE(binding)` and does not merge, graduate, move
the pointer, or begin 13.2.
