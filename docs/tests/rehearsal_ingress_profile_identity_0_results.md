# 0088 Studio slice C0: deterministic profile provenance

PROBATION / proof-present / OPEN / UNMERGED. Return to Orchestration only.
Dispatch: Board 5646981503, following the profile-pin STOP at 5646969438.
Canonical HD: `handoffs/0088-STUDIO-SLICE-0.hd.md`.
HD-RECEIPT: b22c5f735522. ORIENT-RECEIPT: 28f56884d309.
Base: `a178f5b6f614afeb6f062de19266ac46600923e1`.
The final tested head, hosted scan and fresh clearance are bound in the PR/Board
return; this file does not claim its own commit hash.

## Problem and owning correction

The existing Mapeditor `authored_profile_content_identity` hashed the GameMode,
full session tree and install targets. Tree property HashMap iteration order
and materialized node IDs changed its bytes for equivalent authored profiles.
The three-process investigation on the exact base isolated property-pair order;
same-process reopen additionally changed node IDs through allocation history.
Orchestration classified their removal from workload identity as an admitted
provenance repair, retaining measurement law and reference structure.

The correction is confined to that existing function in
`crates/simthing-mapeditor/src/clause_scenario_ingest.rs`:

- Each node in the existing ordered tree receives a distinct serialization
  ordinal in preorder. A per-call lookup maps its original SimThingId to that
  ordinal. The lookup is discarded after hashing and never enters a session.
- Every property map contributes a list ordered by typed SimPropertyId, with
  each complete property value retained. Numeric values coincidentally equal
  to a node ID remain values; arbitrary JSON numbers are never remapped.
- The same lookup resolves tree IDs, resource-parent edges, overlay origins and
  affected-node references, and install-target memberships. GameMode's explicit
  RF participant `subtree_root_id` / `parent_subtree_root_id` fields are also
  mapped; slots, capacities and other numeric authoring remain unchanged.
- Complete GameMode data, every tree field, overlay field, child/edge/overlay
  sequence, target name and membership sequence remain in the digest domain.
  Exhaustive destructuring of SimThing, ResourceParentEdge and Overlay makes
  field additions visible to the compiler. OverlayId is retained unchanged;
  this repair concerns SimThingId allocation history, not a new overlay identity
  rule. The shipped profile has no preinstalled tree overlays.
- Duplicate node identities or a declared reference outside the profile tree
  return an error. They cannot silently collapse to a shared sentinel or an
  omitted reference. Valid distinct tree targets retain distinct ordinals.

The serialization view is hash input only. No runtime object, source/dependency
identity helper, core serializer/allocator, save format or persistence behavior
is changed. The private digest is bound and checked by the same existing
source-cache provenance paths. Existing in-memory provenance is not migrated;
newly loaded profiles bind the repaired digest.

## Before / after digest matrix

Before values are the exact base investigation at
[Board 5646969438](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5646969438).
After values come from the owning production-door witness. The final-head
rerun and process IDs are recorded in the PR/Board return.

| Qualifier | Before: existing digest on base | After: repaired digest |
| --- | --- | --- |
| Fresh process 1 | `fnv1a64:60ba4735951b0465:21477` | `fnv1a64:bfcbc44323b304bf:23530` |
| Fresh process 2 | `fnv1a64:c16ba6005adb87df:21477` | `fnv1a64:bfcbc44323b304bf:23530` |
| Fresh process 3 | `fnv1a64:56da5326f2a00d23:21477` | `fnv1a64:bfcbc44323b304bf:23530` |
| Same-process first open | `fnv1a64:c908aec26cbfe6e7:21477` | `fnv1a64:bfcbc44323b304bf:23530` |
| Same-process reopen | `fnv1a64:aa268ac1ec592020:21477` | `fnv1a64:bfcbc44323b304bf:23530` |

The after pair's actual profile root IDs are 279 and 379. Its first resident
bridge has dropped before reopen, while process allocation history remains.
All five opens use the original shipped `scenarios/stellaristhing_base.clause`
through `run_clause_picker_action` and the UI's
`StudioAppState::try_adopt_loaded_scenario_session`. Fresh-process children
run from foreign temporary working directories, with explicit temporary cache
destinations, no resolver entries and no profile hint. No native-dialog visual
automation is claimed. Each admitted bridge reports generation 0; its ordinary
material readout is A1 minerals/alloys 3/4 and E1 minerals/alloys 3/3.
All 13 install-target sets are present. No simulation timing or throughput
measurement is collected by this qualifier.

Unchanged native byte identities in every report:

| Input | Identity |
| --- | --- |
| stellaristhing_base.clause | `fnv1a64:ee4e4df9e8c9fbd9:5798` |
| stellaristhing_base.base.json | `fnv1a64:c49f9ca3c8c75e77:20370` |
| stellaristhing_base.dependencies.json | `fnv1a64:2f064bfb3e043aa0:72` |

The profile digest remains distinct from these source/dependency digests. Its
new byte length reflects the canonical serialization view; source/cache bytes
retain the original format and content identities.

## Owning witness and refusal strength

One bounded owning test is added:
`crates/simthing-mapeditor/tests/rehearsal_ingress_profile_identity.rs`,
`rehearsal_ingress_profile_identity_preserves_workload_and_references`.
It reads the already-bound private provenance through Debug; it does not expose
a new public identity API or duplicate the production hash implementation.

The witness first performs the five ordinary opens above. It then exercises
the existing save-provenance boundary:

| Falsifier | Result |
| --- | --- |
| Deliberately rebuilt property maps, demonstrably different raw serialization | Same digest; save succeeds with identical cache bytes |
| Complete bijective renaming using reversed, distant numeric IDs | Same digest; save succeeds with identical cache bytes |
| Property addition / existing numeric value change | Different digest; save refuses before writing |
| Authored Owner policy multiplier change | Different digest; save refuses before writing |
| Install-target reference change | Different digest; save refuses before writing |
| Two distinct target references conflated | Different digest; save refuses before writing |
| Resource-parent target change | Different digest; save refuses before writing |
| Ordered tree topology change | Different digest; save refuses before writing |
| Overlay origin or affected-node reference change | Different digest; save refuses before writing |
| GameMode explicit RF participant or parent reference change | Different digest; save refuses before writing |
| Duplicate node ID or dangling target | Identity error; save refuses before writing |

The reference-form cases absent from the shipped bundle use an enriched
HydratedScenarioPack through the existing pack-to-profile/provenance door.
They exercise serialization/save validation without executing new economics or
claiming those additions are in the shipped slice. Their tree overlay, explicit
RF references and two-target membership distinguish alias preservation from
erasure. A property holding a numeric value equal to a node ID ensures that
typed references and ordinary numbers cannot be conflated by the repair.

For edited-profile cases, the witness verifies the Scenario authority is
unchanged and requires the exact existing "session differs from its native
source" refusal. That branch, after successful digest computation, proves the
new profile digest differs from its bound provenance. A serialization or
admission error cannot masquerade as that successful negative comparison.

## Reference-erasing mutant

A disposable mutation changed the canonical reference lookup from returning
its distinct ordinal to returning zero for every found reference. All five
opens still produced the same mutant digest
`fnv1a64:458aa4fb9aa2f4be:23493`; mere repeatability was insufficient.

The owning witness then failed at the edited install-target reference: save
incorrectly succeeded, so the required refusal assertion was RED (cargo exit
101). The exact good source was restored after the run. This demonstrates
that a stable hash which loses reference distinctions cannot pass this witness.
The mutant is not committed or registered as another test.

## Verification, lifecycle and routing

Commands for the final-head proof:

```text
cargo check -p simthing-mapeditor
cargo test -p simthing-mapeditor --test rehearsal_ingress_profile_identity -- --nocapture
cargo test -p simthing-mapeditor --test rehearsal_ingress_native_rf --test rehearsal_ingress_source_cache --test rehearsal_ingress_literal_successor --test rehearsal_ingress_fidelity
cargo test -p simthing-clausething --test field_economy_grammar_0 --test rehearsal_ingress_records --test ct_0b_raw_model --test ct_0c_expansion --test ct_0d_scope --test ct_scenario_container
bash scripts/ci/agent_scan.sh --base a178f5b6f614afeb6f062de19266ac46600923e1 --head HEAD
```

The new test has one behavior-regression / AUDIT / ledger-only inventory row,
owned by `0088-STUDIO-SLICE-0`, birth track `0.0.8.8-integrated-rehearsal`, DSU 0.
It is a scoped borrow under the track's ordinary closure lifecycle. No old test,
fixture or lease is renewed. The observed escaped nondeterminism and lost-reference
mutant are not caught by the compiler or an existing source-cache witness.

Scope: the existing Mapeditor identity function, one owning test, its inventory
row, and this packet. Class `rehearsal-ingress-mapping`; novelty_claim: NO.
All scan-id INSPECT findings return to Orchestration verbatim; no self-triage.
Hosted Doctrine Scan artifact/step conclusions and fresh final-head Clearance
are reported on the PR/Board. No engine/spec/ClauseThing, core serializer,
allocator, Cargo/E8, protected/gate, binding-law or source asset changes.

Leaf C1 baseline measurement remains for Orchestration's dispatch after C0
merges. The accepted generation-N persistence RED remains carried to
2.3 `0088-POLICY-CONTINUATION-0`; this repair makes no continuation claim.
