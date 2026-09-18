# Construction 2.1 retention payer and canonical graduation carrier

**PROBATION / proof-present / DA DEEP-TREE review pending. OPEN / UNMERGED.**
Authority: DA [5723973467](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5723973467)
and handoff [5724136171](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5724136171).
This payer performs no 2.2 implementation. Model 2 stays CLOSED.

Base is exact master `9b6385ad9776cc43b0d79abfcd86cb94581c96a5`.
The payer merged canonical stamp `6fd3cb7f272bc854a5fd2c967f747152b50b670d`
from `da/0088-grad-2.1`. Both stamped files remain Git-blob-identical:
`docs/0_0_8_8_Integrated_SimThing_Rehearsal.md` and
`docs/orchestrator_orientation.md`. The stamp records DA-GRADUATED / merged
#2062 @ 9b6385ad, PASS-MODEL-1 / Model 2 CLOSED / COMPLETE, with pointer 2.2.
That stamp lands only through the subsequent reviewed payer merge.

## Exact disposition

| Population | Disposition | Count |
|---|---|---:|
| Live-consumed scripts-ci fixtures | Promote from pen to active harness-fixture ownership | 46 |
| Rust proofs in dedicated files | Survivor-safe automatic whole-file reap | 25 |
| Rust proofs in shared/inline files | Exact test-function removal | 293 |
| Rust total | Reaped | 318 |
| Precautionary renewals | None | 0 |
| Parked remainder | None | 0 |

The [364-row original pen](construction_retention_payer_checkpoint/original_parked.tsv)
and [disposition ledger](construction_retention_payer_checkpoint/dispositions.tsv)
preserve every identity and original birth/parking/DSU provenance. Active
inventory is **1037 unchanged rows +46 =1083**. The promoted rows retain their
fixture bytes, identity, class, boundary, verdict, promotion target and DSU count.
Their owning `birth_track` becomes the existing `harness-fixture` family; the
original birth track/date/closeout/DSU fields remain explicit in each note and the
original ledger. This is live-consumer promotion, not a date reset or renewal.

Each [promotion](construction_retention_payer_checkpoint/fixture_promotions.tsv)
names a concrete structured `downstream-utility:`: clearance selftests, doctrine
selftests, relay-lint selftests, artifact-provenance selftests, review-context
selftests, or the Embedder Guide selftest. All six consumers were executed.
The inherited review-context command was corrected to include its required
`--selftest` switch. Original fixture files and all gate code are unchanged.

The earlier closeout event, not a fabricated wall-clock date, triggers this
disposal. After the 46 promotions, the stock survivor-safe reaper ran with
`--decommission --all --dry-run`, its selected paths were checked against the
exact 318-proof population and surviving live rows, then the same stock command
ran without `--dry-run`. It removed **25 rows /25 files** and refused **293**
shared/inline proof identities. The additional **one** manual message is the
unrelated, non-reapable `docs/workshop/archive/field_policy/README.md` artifact
lease selected by `--all`; its row/date/file remain byte-identical and it is not
part of this payer. No `TRACK_CLOSEOUT_NOW` override or gate edit was used.

The stock `--prove` rehearsal passes, including survivor/shared-support refusal
and bounded-renewal closeout controls. See the full
[dry run](construction_retention_payer_checkpoint/decommission_dry_run.txt),
[apply](construction_retention_payer_checkpoint/decommission_apply.txt), and
[proof output](construction_retention_payer_checkpoint/reaper_prove.txt).

## Manual remainder and preservation

Only after the safe reaper/accounting, **293 exact test functions in 90 files**
were removed. Rust syntax-tree function spans include their attributes and doc
comments. The [function TSV](construction_retention_payer_checkpoint/function_removals.tsv)
records identity, original byte ranges, and SHA-256 of each removed item. Shared
helpers, imports and other functions outside these ranges remain unchanged,
apart from exposed EOF separators and the separately authorized E8 pin constant.

This left **53 zero-test shells**. Each has zero surviving inventory identities
and no module/include/script/config execution consumer. They were reaped under
the existing SCENARIO-RESIDUE DEAD-TARGET authority, with a
[separate manifest](construction_retention_payer_checkpoint/empty_targets.tsv).
They add no proof identities to the 318 account. Two textual mentions are
historical qualification/baseline provenance, not execution consumers; three
same-named `src` module exports resolve to production modules, not these test
targets. No shared support file was deleted.

The [preservation snapshot](construction_retention_payer_checkpoint/preservation_snapshot.tsv)
and [verification](construction_retention_payer_checkpoint/preservation_verification.txt)
establish:

- 895 surviving function bodies are byte-identical in 37 retained shared files;
- 877 original live/artifact files are byte-identical; twelve shared files differ
  only by the approved removals and pin constant;
- both canonical stamp blobs are exact; all 1037 original active rows and the
  unrelated artifact lease are unchanged;
- all 318 Rust identities are gone, all 46 fixture identities are active, and
  zero parked decisions or renewed leases remain.

## Pre-announced E8 overlap and fourteenth pin roll

Before scoping or deleting, the actual `build.rs::COMPONENTS` list was intersected
with the 364 parked rows. Exactly two proofs overlap one sealed file:

```text
crates/simthing-core/src/persistence_deformation.rs
  decay_and_saturation_admit_but_unbounded_escalation_refuses
  nonfinite_and_unbounded_shapes_refuse_at_admission
```

The [overlap record](construction_retention_payer_checkpoint/sealed_overlap.tsv)
was created before deletion. Every component was compared again afterward:
only that file's approved test-function bytes changed. No production function,
shader, ABI, Cargo.lock, qualification algorithm, or `COMPONENTS` list changed.

The unchanged canonical construction ingress test first produced the required
negative with the old pin: `UnqualifiedAdapter`, required `ec5a2a30afaee795`,
observed **ee0e9ac0af830bdf**, bundle **3e5297c713ea1ecd**. The one constant in
`simthing-gpu/src/resident_clearing_runtime.rs` was then rolled under standing
authority. The same ordinary GPU session test passes with required=observed
**ee0e9ac0af830bdf**. The complete qualification record and both runs are in
[E8 evidence](construction_retention_payer_checkpoint/e8_pin_proof.txt).

Reference tuple: NVIDIA GeForce RTX 4080 Laptop / Vulkan / NVIDIA 595.79,
rustc 1.95.0 / LLVM 22.1.2, `simthing-gpu/eml-resource-profiling`. The final
exact-head clean-checkout reproduction and hosted run identities are bound in
the PR body and Board return. Clean-checkout proof uses the same admitted tuple;
no cross-adapter qualification is claimed.

## Gates, advisory residue, and limits

Required local checks pass: rung-close 2.1, artifact expiry (expired=0, cruft=0,
malformed=0), lifecycle schema, test inventory drift (stale=0), orientation
freshness/pointer 2.2, and canonical stamp equality. Cargo check with `--tests`
covers all nine affected Rust crates using the admitted profiling feature.
The stock local agent scan reports zero hard failures and zero INSPECT flags.
Exact commands/results and fixture-consumer outputs are in the
[local validation packet](construction_retention_payer_checkpoint/local_validation.txt).

Two other diagnostics are reported distinctly:

1. The broader legacy `test_inventory_check.sh` reports **33 pre-existing
   judgment/class findings**, identical on base and payer, with zero new
   findings. Its discovery and inventory still agree at 1083, missing=0/extra=0,
   and its lifecycle-schema subcheck passes. Required inventory drift and schema
   pass; this auxiliary diagnostic is not misrepresented as green.
2. Standalone SCENARIO-RESIDUE has zero scenario/domain/dead-target failures and
   **52 advisory DEAD-EXPORT entries**, versus 46 at base. The six new entries
   are `action_band_semantic_shadow.rs`, `comparative_default_birth.rs`,
   `owner_channel_rf_compile.rs`, `resource_flow_convergence_burn_in.rs`,
   `resource_flow_dynamic_enrollment_soak.rs` (driver), and `resolution_site.rs`
   (sim). They are unreaped exported-source residue exposed by retiring their
   proof consumers. This 318-proof disposition does not authorize deleting those
   production modules; no future consumer is invented. Existing source and
   support residue remains unchanged. The complete before/after account is in
   [diagnostic accounting](construction_retention_payer_checkpoint/diagnostic_accounting.txt).

No exception, allowlist, triage or budget-exemption additions. No production
implementation, lifecycle gate change, precautionary renewal, or 2.2 work.
Return the open payer for the required single DA DEEP-TREE review. Fresh
final-body clearance and its actual route are recorded after hosted artifact
inspection; neither a green workflow nor a reserve is merge permission.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: b8d086a89850304da14d4f3d31a3f31c6cdd52f803b3e55e8baa83de359d2201
ANCHOR-ACK: admission-ladder-necessity-test@4bedf826f6f7
ANCHOR-ACK: workshop-candidate-homing@3e584f0ad175
