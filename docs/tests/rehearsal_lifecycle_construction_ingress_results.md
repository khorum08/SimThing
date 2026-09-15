# Construction Leaf A — qualification scope-gap return

**Historical packet — qualification blocker resolved by #2063.** On repaired base
`df5dd480b42dc224926c8bbab1d229b988b3f57c`, the same canonical ingress test passes.
The current return is [the allocation prerequisite packet](rehearsal_lifecycle_construction_allocation_results.md).
The following pre-roll evidence and claims are preserved as history, not current status.

Status: **PROBATION / ingress-proof-present / BLOCKED / OPEN / UNMERGED**.
Recipient: **ORCHESTRATION ONLY**, under [Board 5675549349](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5675549349).
Base: `ffa5b944928c3aa3fc66832700330dc43b2a0f78`.

**STOP before economics. Model 1 remains UNDECIDED; Model 2 remains CLOSED.**
This is a qualification scope gap, not `FAIL-MODEL-1`. Selecting either requested
model verdict would falsely claim an executed construction discriminator. The
handoff explicitly forbids the source/pin changes required to unblock admission.

## Reproduction and cause

The 49-line integration witness enters `SimSession::open` with the ordinary
resident posture and a tiny tree carrying the existing default RF property.
It does not step, clear materials, create a product, replace a pin, skip a missing
GPU, or fall back to the CPU oracle. Its required-success assertion stays RED.

```powershell
$env:CARGO_TARGET_DIR='C:/Users/mvorm/SimThing/target'
cargo check -p simthing-driver
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
```

`cargo check` passes. The focused test compiles and runs: **0 passed, 1 failed,
0 ignored**, exit 101, before economics. On the reference NVIDIA GeForce RTX 4080
Laptop GPU / Vulkan / NVIDIA 595.79, rustc 1.95.0 (LLVM 22.1.2), the required
`EML_RESOURCE_PROFILING` tuple reports:

| Quantity | Value |
|---|---|
| Required production fingerprint | `6f287da986750d29` |
| Observed fingerprint | `6fe1d809c05ee0f4` |
| Current semantic bundle | `c6b6a70c64f7ae81` |
| Last qualified bundle at `dae7ab1b` | `dfcb52ff6a412e2e` |
| Same captured adapter/compiler/features tuple, old Git bundle, offline hash only | `6f287da986750d29` |

Read-only forensic recomputation matches both the current captured fingerprint
and the prior pin by varying **only** the source-bundle hash. All 32 current local
component files match their canonical LF Git blobs at the supplied base. The
three changed components since the last qualification are:

- `crates/simthing-driver/src/arena_allocation_plan.rs`
- `crates/simthing-kernel/src/accumulator_op/session.rs`
- `crates/simthing-driver/src/session.rs`

The changes are the test removals in cleanup commit `1157066e`, subsequently
merged by #2061. `crates/simthing-gpu/build.rs` hashes full file bytes, including
test bodies. Removing those tests changes qualification even though production
functions are unchanged. Neither existing pin was rolled. This is inherited
baseline debt; this leaf changes none of the 32 components, Cargo inputs, compiler,
qualification comparator, or component list.

Raw capture and per-component SHA-256s are in
[the raw packet](rehearsal_lifecycle_construction_ingress_raw_results.md).
The earlier default-feature attempt reported `f250da9bec2d71dc`; it was the wrong
qualification tuple and is not used as the canonical reproduction.

## Current-path inventory (source inspection; not execution proof)

| Concern | Actual owner and call chain | Limit still requiring the discriminator |
|---|---|---|
| Owned numerical stock | Core `SimThing.properties` + `PropertyValue`; `DimensionRegistry` resolves roles; driver `derive_resource_flow_admission` derives participants/parent edges; `sync_resource_flow_accumulator` lowers allocation and governed Balance integration | Balance is resident WIP, not a second inventory ledger. Explicit host identity must accompany shared property roles. |
| Material delivery | Driver `arena_allocation_plan` + `child_share_eml` lower the recursive RF operation; kernel runs admitted bands; `Balance` carries residual via `governed_by` | Flow/allocation rates must not be counted again as owned stock or irreversible progress. Ordinary-session pulse/stock witness has not executed. |
| Exact recipe | Spec `compile_resource_economy` → driver `materialize_resource_economy_registry_for_session` → `sync_resource_economy_accumulator` → kernel `plan_transfer_ops` / `SubtractFromAllInputs` | Existing conjunctive recipe uses complete affordable quanta. Its numerical output alone is not structural completion. |
| Stock release | Existing `ResourceTransferSpec` / `DiscreteTransferRegistration` uses source-debit transfer; single-input `max_transfer` is a clamped transfer, not conjunctive floor | Cancellation needs a lawful binding that stops the project and routes its actual stock. Host-side replacement of plans is not proof of that binding. |
| Grants/holdings | Spec `MarketGrantRecord` lifecycle; session `release_market_grant`, `revoke_market_grant`, partition/transfer doors | Grants are not stock balances. ActionBand's `PersistentScarceGrantHolding` and `AtomicCommonDepthCommitment` still reject at `reject_deferred_requirement`; this does not establish that Model 1 needs either. |
| Capacity/placement | Driver `GrowthEntitlementMarketBinding::settle_boundary_claims`; sim boundary validates provenance, calls allocator `realize_unattached_growth_residency`, then creates `VerifiedGrowthResidencyCommit` before attach | Capacity quantity and contiguous placement have separate judges. A fabricated scalar recipe input is not a residency commitment. |
| Structural application | Sim `apply_add_child` requires verified growth; `Remove` tombstones rows; `Reparent` retains identity/rows. Driver `StructuralAuthorization` and `FrozenActionBandStructuralRequests` clone pre-admitted requests | Frozen `AddChild` retains its child identity. Fresh recurring completion must be proved, not inferred from a counter. Existing fission has `clone_subtree_with_fresh_ids` and clone enrollment, but has not been shown to satisfy this handoff's completion profile. |
| Failure | `step_once_into_summary` takes the existing generation permit before the hot cycle and restores/finishes it only on the lawful path; boundary placement refusal is recorded separately from fatal errors | No rollback or new retry path is implemented. Touched-generation behavior remains inherited and must be exercised after ingress is admitted. |

## Required proof accounting

| Handoff item | Status |
|---|---|
| 1. Current-path inventory | Preliminary source inventory above; no semantic PASS inferred from names. |
| 2. Two-project/two-input discriminator | NOT PROVED. Ordinary resident session ingress refuses before economics. |
| 3. No irreversible partial spend | NOT PROVED. Recipe source inspected; executable lifecycle proof remains blocked. |
| 4. Cancel P/Q, release, restart | NOT PROVED. No automatic cancellation/stock disposition claimed. |
| 5. Capacity/placement timing and fault | Source chain identified; ordinary-session scenario blocked. |
| 6. Two fresh funded completions and full bindings | NOT PROVED. No frozen-template/counter substitution. |
| 7. Determinism/storage invariance | NOT PROVED. Existing proportional RF weight expression and explicit transfer OrderBands identified; no new tie rule proposed. |
| 8. Negatives | Actual qualification refusal reproduced. Material/placement/cancellation negatives still pending. |

An exploratory four-case draft was compiled and attempted before isolating this
ingress blocker. Three cases stopped at qualification; the separate low-level RF
fixture did not produce its expected stock and is **unverified fixture work**, not
a substrate falsification. That draft and transcripts remain in the external
scratch directory; they are not committed, enrolled, or claimed as evidence.

## Remand requested

Orchestration should route the inherited E8 qualification repair separately. The
existing candidate repair surfaces are the production fingerprint literal in
`crates/simthing-gpu/src/resident_clearing_runtime.rs` and its independent referee
literal in `crates/simthing-workshop/tests/resident_clearing_parity_0.rs`, subject to
the standing DA qualification/mutation/clean-checkout proof requirements. This
packet supplies an observed value, **not authority to copy it into either pin**.
Do not restore expired proofs, weaken qualification, or amend build.rs to evade
the source-byte change. A repair handoff must own its exact surfaces and proofs.

After admitted repair, resume Leaf A at the repaired master and execute the full
matrix before choosing a model. No Model-2 API is proposed from an ingress failure.

## Surface and lifecycle ledger

| Changed surface | Purpose |
|---|---|
| `crates/simthing-driver/tests/rehearsal_lifecycle_construction_ingress.rs` | Minimal RED ordinary-session reproduction with raw provenance |
| `docs/tests/rehearsal_lifecycle_construction_ingress_results.md` | Scope gap, source inventory, honest proof accounting |
| `docs/tests/rehearsal_lifecycle_construction_ingress_raw_results.md` | Captured error and byte-hash evidence |
| `scripts/ci/test_inventory.tsv` | One new admission-adjacent AUDIT row, consumed by this Leaf A, delete at closeout |

No production, scenarios, UI, Cargo, sealed bundle, gate, anchor, or router edits.
The draft PR and Board return carry final head, tested_code_sha, hosted Doctrine,
the fresh clearance outcome, and complete INSPECT accounting. A green hosted
Doctrine run must not be represented as a passing GPU/construction test.

ORIENT-RECEIPT: 28f56884d309
orientation_rule_stamp: 73e54d6b56b7266b
orientation_digest_sha: e5b941f898d53f86a8be2e19afba51d52a41dda06f69dff4d3b52817ffed89c6

All 14 current `rehearsal-0088-*` anchors were resolved, read and ACKed before
edits; exact ACKs are retained with the raw packet. The direct Board instruction
is this leaf's dispatch authority; no construction HD receipt was fabricated.
