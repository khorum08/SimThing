# 0088 plain JSON admitted-loader convergence

PROBATION / proof-present / clearance-pending / OPEN / UNMERGED.
Dispatch: Board 5627721196. HD-RECEIPT 70df16e1babd;
ORIENT-RECEIPT 28f56884d309; 48-anchor ACK Board 5612017867.
Base: 9d2f5d9538442c011cd1f3663c1400502e42307a (#2034 merged).
Exact tested head and hosted results are recorded in the PR and Board return.

Studio's two plain-JSON file entry points now consume
`load_scenario_spec_from_json_str` and require its `ingestion_ready` report before
returning authority or constructing a Studio document. A successful parse alone
does not authorize adoption. Refusal identifies canonical JSON admission and the
source path; a non-ready report also names the scenario. The session loader reads
plain JSON once. Native cache routing still re-admits native source and carries
its authored live profile; structural JSON is not a new economic source.

## Falsifier and proof

RED commit 04414ff4f95ebe246ea7e787a988615449e20af5 contains the owning tests
and inventory rows before the adapter fix. A canonical document has a 2x2 local
frame and a child at column 3. Structural serde accepts it; the admitted spec
loader reports `ingestion_ready=false`. The old Studio loader nevertheless
returns authority, failing `nonadmitted must refuse`. The other test passes.

The same two tests pass after the adapter fix:

- `rehearsal_ingress_json_preserves_admitted_canonical_and_compatibility_records`
  compares admitted canonical serialization through authority loading, Studio
  loading and save/reload for canonical Scenario and legacy World records. It
  checks their distinct authority classifications and compatibility marker.
- `rehearsal_ingress_json_refuses_nonadmitted_document_before_replacement` checks
  the parseable-but-nonadmitted child and malformed JSON through both file-load
  paths. Both refuse with the source path before replacement; the prior valid
  document's canonical serialization and file path remain identical.

The identity witness uses the admitted deterministic canonical serializer, which
normalizes property tuple ordering, rather than the older history digest over
structural serialization. No spec implementation or historical test is changed.

Reproduce with `cargo test -p simthing-mapeditor --test rehearsal_ingress_json --
--test-threads=1`. Package verification uses `cargo test -p simthing-mapeditor
--no-fail-fast -- --test-threads=1`, retaining native/cache admission, atomic
replacement, portable source/dependency and installed native-RF regressions.

Cargo check and local delta agent scan pass (zero hard failures, zero INSPECT).
Inventory drift passes: 969 active / 1538 discovered / 569 parked, no stale or
unledgered identities. Lifecycle schema passes. The two new owning rows are
behavior-regression / AUDIT / ledger-only, birth 0.0.8.8-integrated-rehearsal,
DSU 0. No parked referee is renewed or weakened.

## Census transition readiness

AUTHORING-INGRESS-CANONICAL-JSON is READY for its pre-authorized B1 classification
tightening once this leaf lands: both Studio plain-JSON paths consume the
admitted loader, both rejection branches precede replacement, compatibility
records are retained, and merged #2033/#2034 provide native/cache economics and
dependency fidelity. Signal to Orchestration for the DA census increment; the
protected census is unchanged here. This is readiness for this surface only.

Historical-stage successor proofs, hydration/generation/preset convergence and
literal-install/runtime-vertical-seed retirement remain under the same contract.
The historical Board-pointer referee remains Orchestration-owned harness debt.
No engine/spec, gate, Cargo, census, triage or graduation edits; no self-merge.
