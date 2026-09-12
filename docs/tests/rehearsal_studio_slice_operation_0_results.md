# 0088 Studio slice, Leaf B: operation and initial persistence probe

PROBATION / proof-present / OPEN / UNMERGED. Return to Orchestration only.
This evidence-only leaf of `0088-STUDIO-SLICE-0` records ordinary Studio
controller operation of the merged Meridian Arm slice. Initial persistence
is RED: Save Scenario reconstructs authored source at generation 0. Per
[Orchestration disposition 5646763251](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5646763251),
that truthful diagnostic satisfies the 1.2 initial-probe obligation; successful
checkpoint continuation belongs to 2.3 `0088-POLICY-CONTINUATION-0`.

Canonical HD: `handoffs/0088-STUDIO-SLICE-0.hd.md`.
HD-RECEIPT: b22c5f735522. ORIENT-RECEIPT: 28f56884d309.
Fresh orientation and all 48 unchanged required anchors ACKed at Board
5646821273. Exact evidence-leaf base:
`aa0efae745016b31ea26a35bb9a72373d6cf4524`.

## Evidence provenance and bounds

The observations and complete disposable Rust probe were preserved in
[Board 5646740516](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5646740516)
at `5652ba2c1f4d68695543888f6e01338156c29d1f`, the merged Leaf A #2045.
The only intervening change to this leaf's base is the corrected HD in
#2046; production code and the three scenario inputs are byte-identical.
The old STOP/energy-display posture in that report is superseded by the
disposition above; the observations are preserved unchanged here.

Retained probe SHA256:
`b948918c5f446946ad59e6fd058ad44b4c02ff0bedbdfc1e4c8922d131969dd0`.
Original output SHA256:
`34ff678bf7bdf270e0f798cefb6248133b806173d08aa8967ed2f5322ea3a8d5`.
Local copies are `.git/0088-studio-b-save-probe.rs` and
`.git/0088-studio-b-probe/continuation-red.log`; the cited Board report
also contains the probe and observed output so review does not depend on
those local files. The probe deliberately exits 1 on continuation mismatch.
It is an investigation adapter, not a registered or renewed regression test.

The existing injectable picker boundary supplied selected paths through
`FakeClauseFilePicker` / `FakeScenarioFilePicker`. All loading, admission,
stepping, observation, saving and reopening used production controllers.
This is controller-level proof; the native OS dialog and rendered window
were not visually automated. No simulated economics or copied resident
values supplied any observation.

## Ordinary picker portability: PASS

The three shipped files were copied into `relocated bundle/`, a directory
with spaces. From sibling `different working directory/`, the probe selected
`../relocated bundle/stellaristhing_base.clause` through
`open_clause_scenario_with_picker_state`. Its production picker action ran
native parse, expansion, source-relative hydration, structural rebind,
source-cache creation/re-ingest and Studio session construction. It reported
`StructuralRebindReady session hydrate PASS`.

`StudioAppState::try_adopt_loaded_scenario_session` then admitted the candidate
through `StudioLiveSessionBridge` before committing the document, using the
same transaction as the UI. Resolver entries were `{}`; source/dependency
identities matched the shipped bundle:

| Input | Native content identity |
| --- | --- |
| stellaristhing_base.clause | `fnv1a64:ee4e4df9e8c9fbd9:5798` |
| stellaristhing_base.base.json | `fnv1a64:c49f9ca3c8c75e77:20370` |
| stellaristhing_base.dependencies.json | `fnv1a64:2f064bfb3e043aa0:72` |

Absolute paths in diagnostic messages are resolved display paths, not
authored dependencies or machine-local resolver tokens. The later save
retained source path `../relocated bundle/stellaristhing_base.clause` and an
empty resolver map.

## Real resident run, step and inspect: PASS

Ticks ran through `StudioLiveSessionBridge::consume_scheduled_ticks`: three
ticks to N=3, then one additional tick on the retained original session.
The probe printed `bridge.readout().field_accretion_samples`, restricted to
the readout's current `executed_ticks`. These are the generation-stamped
samples already rendered by Studio's material gauges/field-accretion table.
Bridge generation and `SimSession.coord.day_index()` agreed at every row.

The table below shows real changes along both sites' native mineral-source
and mineral-to-alloy sink/recipe paths. Leaf A's
`rehearsal_scenario_studio_slice_0_results.md` records the authored relations
and source/upkeep attribution. This leaf uses the existing material readout;
it adds no economic selection or calculation to the UI and makes no new
energy-display requirement. The unchanged RF summary's zero aggregate/balance
readout is not evidence of the facility allocation changes observed in Leaf A.

## Failed pre-effect load preserves the running session: PASS

At generation 3 the same ClauseScript picker/controller received
`invalid.clause` containing:

```text
scenario = rejected { unsupported_leaf_b_probe = yes }
```

It refused before activation with the source file, token 3, and unsupported
scenario field `unsupported_leaf_b_probe` in the diagnostic. Exact checks:

```text
FAILURE PRESERVATION: rejected=true document_equal=true resident_identity_equal=true anchor_rows_equal=true generation=3
```

The document comparison serialized the prior/current `scenario_authority`;
resident identity used `persisted_execution_identity()`; anchor comparison
covered all `AnchorTableSnapshot::from_session(...).rows()`. The original
resident session subsequently advanced to generation 4, shown below as the
uninterrupted control. This proves the exercised pre-effect refusal boundary,
not rollback after an economic effect.

## Initial persistence probe: RED, carried to 2.3

At N=3 the ordinary UI controller `save_scenario_action` reported
`Saved { message: "Scenario saved: running-n3.simthing-scenario.json" }`.
`load_scenario_with_picker` reopened that file and the ordinary UI admission
transaction adopted it into a separate fresh bridge. The original bridge
remained alive as the control. Neither replaying N ticks nor injecting saved
observations was used to recreate the running state.

| Checkpoint | Bridge/runtime generation | A1 minerals | A1 alloys | E1 minerals | E1 alloys |
| --- | --- | --- | --- | --- | --- |
| Ordinary open | 0 / 0 | 3 | 4 | 3 | 3 |
| Save at N | 3 / 3 | 4 | 8 | 4 | 7 |
| Reopen before any step | 0 / 0 | 3 | 4 | 3 | 3 |
| Uninterrupted control N+1 | 4 / 4 | 3 | 10 | 3 | 9 |
| Reopen plus one step | 1 / 1 | 4 | 5 | 4 | 4 |

```text
SAVE FORMAT="simthing-clause-cache-v1"
KEYS=["dependencies", "document", "format", "resolver_entries", "source_identity", "source_path"]
RESIDENT_N_RESTORED: false
REALM_RESTORED: false
CONTINUATION_MATCH: false
Error: "RED: Save Scenario reopens initial source state, not resident generation N"
exit code: 1
```

The owning production seam at the evidence base is:

1. `crates/simthing-mapeditor/src/app/scenario_io.rs:94`,
   `save_scenario_action`: passes the authored `StudioSession` to the writer;
   it does not receive/capture the live bridge or `SimSession`.
2. `crates/simthing-mapeditor/src/scenario_io.rs:66`,
   `save_current_session_scenario_to_path`: verifies source/profile provenance
   then calls `write_clause_source_cache`. The object above contains raw
   source and dependency identities, with no resident generation, holdings
   or execution checkpoint. Its load counterpart at line 94 re-ingests
   native source to construct a fresh StudioSession.
3. `crates/simthing-mapeditor/src/app/ui.rs:2662`: Save Candidate explicitly
   refuses native authored programs and directs the operator to Save
   Scenario's source cache. The ScenarioSpec runtime save/reopen adapters
   declare persistent history/GPU dispatch deferred.
4. `crates/simthing-driver/src/session.rs:717`, `SimSession::open_restored`:
   restores the durable execution realm with a new incarnation via fresh
   open; it allocates new `WorldGpuState`, projects the supplied initial tree,
   and initializes ordinary-flow continuation to default. Identity restoration
   alone is not a complete generation-N state restore.
5. `crates/simthing-driver/src/spec_replay.rs:756`,
   `open_replay_with_spec`: opens a fresh spec session and returns a separate
   ReplayDriver plus frames, not this Studio session's resident checkpoint.

The missing capture/restore of resident state and generation carries to
2.3 `0088-POLICY-CONTINUATION-0`. No new persistence authority, replay substitute
or engine change was introduced to force a PASS in 1.2. The diagnostic remains
RED; its recording is the accepted 1.2 initial persistence evidence.

## Profile identity: separate unresolved measurement boundary

The opening profile identity was `fnv1a64:b4a3495aedb4706f:21477`; reopening
produced `fnv1a64:9c71859289076130:21477`. Source/dependency identities above
remained stable. The same-process reopen also rematerialized different node
IDs, so this pair does not isolate property ordering as its only difference.

Leaf A separately isolated fresh-process profile variability:
`fnv1a64:735cf4b0078d87b9:21477` versus `fnv1a64:def2ee608b11e06f:21477`.
That diagnostic comparison found equivalent game modes/trees except for
property-pair ordering. `SimThing.properties` is a HashMap serialized as a
pair list (`crates/simthing-core/src/simthing.rs:73`);
`authored_profile_content_identity` sorts install targets but serializes
the tree without canonicalizing those pairs
(`crates/simthing-mapeditor/src/clause_scenario_ingest.rs:120`).

Digest instability is not the cause of the persistence RED: Save's comparison
against the loaded profile provenance succeeds; the resulting format omits
resident state. Reopening binds a fresh profile rather than restoring that
state. Sorting JSON cannot add the missing checkpoint. No digest was normalized
or substituted, and stable source identity is not called a profile identity.

Leaf C must decide whether the unstable profile digest permits the required
reproducible workload pin before claiming source/profile-pinned timings. This
packet claims neither reproducible profile pinning nor a timing baseline.
Any required serializer/authority change remains a boundary for Orchestration.

## Existing verification and scope

The final PR/Board return binds `tested_code_sha`, `coverage_basis`, local
test/scan results, hosted Doctrine Scan artifact/step conclusions, and fresh
Clearance to the final evidence head. Existing focused batteries are:

```text
cargo test -p simthing-clausething --test field_economy_grammar_0 --test rehearsal_ingress_records --test ct_0b_raw_model --test ct_0c_expansion --test ct_0d_scope --test ct_scenario_container
cargo test -p simthing-mapeditor --test rehearsal_ingress_native_rf --test rehearsal_ingress_source_cache --test rehearsal_ingress_literal_successor --test rehearsal_ingress_fidelity
bash scripts/ci/agent_scan.sh --base aa0efae745016b31ea26a35bb9a72373d6cf4524 --head HEAD
```

Scope is this packet alone, within `rehearsal-studio-presentation`;
novelty_claim: NO. No production code, scenario asset, registered test,
fixture, inventory row, lifecycle lease, existing referee, gate or anchored
doctrine changes. The disposable probe receives no test renewal. Every
scan-id-bearing INSPECT returns verbatim to Orchestration, with no self-triage.
