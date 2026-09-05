# 15.8 — ordinary-session demand authority: STOP packet

Historical STOP, resolved by binding DA ruling [5549583264](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549583264) and orchestration resume [5549605185](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549605185). The original evidence and anchor acknowledgments below are retained. Current implementation evidence is in [resident_session_integration_conformance_0_results.md](resident_session_integration_conformance_0_results.md).

Status: **STOP / Owner-DA adjudication pending / OPEN / UNMERGED**. This is pre-implementation archaeology, not the 15.8 exit proof. No production source, referee, gate implementation, pointer, or graduation state was changed.

## Ingress and identity

- Dispatch: [board comment 5549356007](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549356007).
- Coding STOP summary for orchestration: [board comment 5549449875](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5549449875), posted under the Owner's standing instruction to return every summary on the board with its comment ID.
- Binding adjudication: [5548783236](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5548783236); Owner mint: [5548825933](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5548825933).
- Branch: `codex/resident-session-integration-conformance-0`.
- Exact base and unchanged source HEAD: `79f0f6c84a5b0d66367edf9d253c6e7b9904e5fc`.
- Handoff projection `base_sha`: `89bb6ef0c33135fa7941d5f0eb22afe705ce182d`; actual branch base includes the subsequently merged handoff, as the dispatch requires.
- `HD-RECEIPT: cdb03115f4b9`.
- `ORIENT-RECEIPT: 8a4a068b3fe7`; `role: coding`; `orientation_rule_stamp: 1e2dcea8714428d2`.
- Orientation digest: `cefb68681891d3fafdc03ccdae0c16c7b224f83a34cf759bfe289d17c3d6e1e4`.
- The initial checkout was the completed 15.7 branch. Its orientation receipt was superseded during the dispatch-required fresh ingress on current master, before any implementation.

The coding projection was rendered and all 63 machine REQUIRED-ANCHORS are acknowledged below. Doctrine was retrieved through `anchor_query.sh`; its append-only reach records are retained. The local `.git/158-projection.txt`, `.git/158-anchors.txt`, and `.git/158-market-anchors.txt` retain the exact machine output.

## Decision requested

**Identify the intended existing ordinary recurring demand source, its claimant/full-scope continuity, and the consumer of its effective N+1 grant.** The only ordinary session caller of the exact resident operator currently consumes structural growth candidates. Its demand is the proposed child's entire subtree size, and its structural grant consumer verifies that same quantity. The source trace establishes when that structural batch exists, but does not establish a separate recurring demand authority for the requested `10 -> 2` economic witness.

If ordinary growth is the intended witness, Owner/DA needs to rule how an independently authored two-row candidate relates to the previous ten-row candidate's unresolved six units, and how effective demand eight/five relates to the unchanged structural placement quantity two. If an existing recurring RF property is intended instead, name its admitted ordinary-session binding and generation transition. The legacy owner-flow property reader is currently an oracle/compile-plan path, not such a live binding.

This is not a request to choose the already-ruled equation or implement a host carry. The equation remains `d_effective(N+1) = d_authored(N+1) + f(U(N))`, with U resident. It is a request to identify which existing authored datum and consequence have that meaning in an ordinary session.

The dispatch explicitly requires: **"If timing is ambiguous/contradictory or a new authoring authority would be required: STOP and return an Owner/DA adjudication packet before semantic implementation."** The handoff also stops when composition requires a peer authority. Accordingly, no new demand author, claimant remapping, implicit retry rule, structural grant conversion, or persistence lane was invented.

## Mechanically traced ordinary call graph

All references below are to the unchanged base. Paths and starting line numbers are provided for review.

```text
SimSession::step_once / run
  -> run_hot_cycle                                  session.rs:626,1804,1851
  -> ordinary RF bands                             simulation_fabric.rs:238,284
  -> boundary reached; begin_generation(N)          session.rs:1864
  -> BoundaryProtocol::execute_with_boundary_hook_and_growth
       bind current generation; read current GPU values for structural work
       run spec boundary handlers                  boundary.rs:694,704,810
       drain BoundaryRequest queue                 boundary.rs:877
       prepare current fission children            boundary.rs:901; fission.rs:164
       AddChild/fission -> complete candidates      boundary.rs:912-926
       quantity = subtree_size(proposed child)
       grantee = proposed child.id
       -> growth resolver under the existing N permit
          -> GrowthEntitlementMarketBinding::resolve_batch_resident
             rows.requested = candidate.quantity() growth_entitlement.rs:372
             rows.source = candidate.grantee()
             rows.rf_participant = structural_parent()
             supply = allocator growth capacity
             -> runtime.dispatch                    growth_entitlement.rs:385
                validate N permit + qualification
                exact apportionment on real arena   resident_clearing_runtime.rs:1270
                canonical T_s live-head append      resident_clearing_runtime.rs:1352
             -> materialize(root_ticket)            growth_entitlement.rs:390
             -> structural grant / growth decision
       -> validate decision quantity                sim/growth_entitlement.rs:316
       -> placement / attachment or refusal fact    boundary.rs:1024
  -> re-enrollment/rebind if topology changed
  -> finish_generation(N, N+1)                      session.rs:1961

SimSession::record_to_path repeats this boundary composition at
session.rs:2010-2136 with its own per-boundary permit variable.
```

The current resident branch does **not** call `market.authorize_draw`: that call occurs in the CPU oracle branch at `growth_entitlement.rs:238-262`. Resident admission does carry the frozen offering/Draw identities into market qualification (`growth_entitlement.rs:163-201`), but that is distinct from generating a per-generation authored demand.

There is no ordinary `dispatch_spatial`, `prepare_temporal_demands`, or `dispatch_temporal` call. Source search over `crates/**/*.rs` finds temporal door definitions and workshop callers only. The root ticket is consumed by `materialize`; it is not retained by the ordinary growth caller for continuation. Materialization reads resident U into schedule facts, but no ordinary temporal economic consumer exists. This absence is not a successful host-U-zero-use continuation proof.

## Authored-demand authoritativeness table

| Candidate source | Existing authority and timing | What remains unestablished for 15.8 |
|---|---|---|
| Explicit `BoundaryRequest::AddChild` | Feeder owns a complete authored child payload. At the consuming boundary, `subtree_size(child)` and `child.id` enter the complete batch, which is stamped with that boundary's generation. `take_boundary_requests` empties the queue (`feeder/patcher.rs:436`). A payload can have been queued earlier; it is not an independently generation-addressed N+1 demand product. | No recurring same-claimant demand binding or rule saying a later two-row payload is additive new demand for the unresolved ten-row payload. |
| Fission | Current-generation threshold events, current tree/property state and secondary predicates create a proposed child at the boundary (`sim/fission.rs:164-244`). Its new child identity and subtree size become a candidate before clearing. These data cannot in general be forced into existence at N. | No next-generation same-claimant source is retained from that prepared child. A later fission prepares another child; the runtime temporal matcher requires the original source identities. |
| Growth refusal `revalue_generation = N+1` | Metadata in `OrdinaryGrowthRefusal`; recorded as a structural/schedule fact. `rg` of its accessor finds constructors, the accessor and proof consumers, not an ordinary demand producer. | The date is not an executable reauthoring or retry authority. Replaying the refusal to create demand would introduce a host recurrence. |
| `OWNER_FLOW_DEMAND_PROPERTY_ID` | `owner_silo_demand_buckets_from_owner_view` reads exact integer property metadata from `SimThingScenarioSpec` (`spec/owner_silo_disburse_down.rs:130,203,232`). | Its reachable driver consumers are compile-plan/oracle functions. It has no ordinary `SimSession` current-generation update/authoring binding that can be identified as the required source. Promoting it requires an explicit authority disposition. |
| Caller-supplied `next_demands` to `produce_runtime_rf_next_generation_demands_for_tick` | Caller-provided vector consumed by the frozen vendorized CPU once-mint (`driver/runtime_rf_tick_compile.rs:47-73`). | Explicitly has no ordinary session caller. It cannot be used to create a second production temporal path. |

**Adjudication outcome:** current structural candidate timing is established; the requested independent recurring N+1 demand authority and its compatible ordinary economic consumer are not. If Owner/DA binds a source available only at Current N+1, the handoff already settles timing: retain U resident and once-mint at N+1 before allocation. No earlier authoring has been proposed.

## Structural consumer conflict

`GrowthEntitlementMarketBinding::resolve_batch_resident` passes `candidate.quantity()` to `record_resident_structural_grant` in both its complete and nonzero partial grant branches (`driver/growth_entitlement.rs:404-435`).

The admitted spec recorder enforces at `spec/flow_market.rs:371`:

```rust
if product.generation() != generation
    || product.granted().checked_add(product.unresolved()) != Some(requested)
{
    return Err(GrantLifecycleError::ResidentProductMismatch);
}
```

Therefore merely substituting temporal dispatch into this caller cannot carry a successful nonzero N+1 product totaling eight or five while its requested structural quantity remains two. This contradiction is a source-derived result, not a newly executed canonical witness. The boundary separately requires a granted residency entitlement's quantity to equal the candidate quantity (`sim/growth_entitlement.rs:341`). Rewriting the candidate to eight/five, changing the recorder's quantity contract, splitting a product into new grant kinds, or selecting a new numerical consumer would require semantics not established by this archaeology.

## Existing persistence and permit ports

```text
ClauseThing compile_persistence_deformation_script_value / _eml
  -> core PersistenceDeformationProgram::admit
  -> existing spec PersistenceDeformationBinding(s)
     [no carrier in the ordinary SpecSessionState or growth market binding]

SimSession::admit_resident_clearing_for_market        session.rs:1091
  -> sealed binding from the existing execution lease and integration schedule
  -> admit_sealed_market_with_persistence_deformations(..., &[])
                                                    session.rs:1113-1119
  -> resident claimant->program map                  resident_clearing_runtime.rs:856
  -> plan.with_persistence_deformations              resident_clearing_runtime.rs:1294
  -> existing resident temporal once-mint            gpu/resident_clearing_runtime.rs:303
```

The program type, admission and resident evaluation exist. The ordinary session carrier is absent; persistence consequences/overlays are a separate consequence-only path and cannot substitute for it. Carrying a ruled authored binding through session admission can reuse these ports after the source/scope identity is resolved.

`TreeExecutionLease::begin_generation` admits the current live generation only (`core/tree_execution_context.rs:533`). `finish_generation` consumes N and advances authority to N+1 (`:626`). Current `prepare_temporal_demands` validates the **product's N permit** (`driver/resident_clearing_runtime.rs:1164`); it also requires authored source identities to match the N product rows. `dispatch_temporal` independently validates N+1 (`:1217`). For a source born at Current N+1, late mint wiring must be changed under the handoff's law; retaining or reusing a consumed N permit is not a solution. No permit law was changed here.

## Arena selection archaeology

| Path | Existing result | Disposition |
|---|---|---|
| Ordinary authored growth market | `resident_market_admission` passes `preferred_arena = None` (`growth_entitlement.rs:192`). | Reported defect confirmed. |
| `ResidentRfArenaBinding::admit`, absent preference | Nonempty arena registry selects physical index 0 (`resident_clearing_runtime.rs:444-449`). | Registry order is economic selection; unlawful under the handoff. |
| Explicit preference | First matching arena name, then its admitted execution layout, flow property and participant mapping. | Existing semantic identity evidence; uniqueness must be established or admission must refuse. |
| Qualification | Binds selected arena, property, topology, registry, policy, exact basis and ABI. | Seals the selection after it happened; cannot make physical-first selection semantic. |

For semantically distinct A and B and an absent preference, the source deterministically selects A under `[A,B]` and B under `[B,A]`. This is a source-derived falsifier prediction, **not an executed A/B economic transcript**. No selector or fallback was added. The admitted remedy remains unique semantic resolution or symmetric typed refusal, once implementation resumes.

## Verification and limits

The unchanged source was checked locally:

- `cargo check -p simthing-driver`: PASS (existing warnings).
- `cargo test -p simthing-workshop --test recursion_axis_conformance_0 --test resident_filter_substrate_binding_0 --test tree_execution_authority_lifetime_0 -- --nocapture --test-threads=1`: PASS, respectively 5/5, 4/4 and 4/4. Local raw transcript: `.git/158-frozen-baseline.log`.
- The frozen direct-runtime transcript still reports `T_s=(G4,U6,N50)`, authored N+1 `2`, effective `8`; the half-deformation parity witness passes. E6 zero-basis and mixed-band work-conservation, E5 exact-basis qualification, E7 rebind, and the lease/permit refusal matrix pass. These remain baseline proofs and do not discharge 15.8.

The first isolated ordinary-growth test command without the workshop's `simthing-gpu/eml-resource-profiling` feature refused at session open with `UnqualifiedAdapter { required: 13738529931016709066, observed: 715583756781095106 }`. It performed no ordinary economic execution. No qualification seal was changed. The feature-matched rerun passed:

```text
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling \
  --test stemthing_b_growth_entitlement_seam_0 \
  implicit_root_market_add_child_refusal_and_replay_use_one_authority_chain \
  -- --exact --nocapture --test-threads=1
test result: ok. 1 passed; 0 failed; 4 filtered out
```

Its transcript is `.git/158-ordinary-qualified-baseline.log`. This existing actual-session test verifies the current AddChild, refusal and replay chain; it is not the new 15.8 temporal witness.

`bash scripts/ci/agent_scan.sh` also returned `AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=33s`, with zero hard failures. Its default comparison was unchanged base-to-HEAD (`79f0f6c8..79f0f6c8`); this is baseline screening, not a committed-delta certificate for this uncommitted packet. Raw output: `.git/158-agent-scan.log`. `git diff --check` passed for the tracked reach-log delta. The packet carries all 63 required acknowledgments.

No actual-session 10/2 identity/half temporal transcript, post-remedy arena permutation transcript, full structural certificate, hosted Doctrine Scan/Exec, `/clearance`, or `/relay-lint` is claimed. Those are implementation-exit obligations, pending the explicit STOP adjudication. Source authority counts are unchanged because source and census files are unchanged; this is not a claim that baseline production satisfies the requested continuation law.

## Changed-file census and review boundary

The intended working-tree delta is this STOP packet plus the append-only `scripts/ci/anchor_reach_log.tsv` records emitted by the prescribed queries. A tracked Python bytecode cache changed when the harness imported its existing module; that incidental generated change was restored. All production source, frozen tests, doctrine/anchor definitions, workflow and gate code, authority census, handoff, and ladder remain at the base. The branch is local and unmerged. No implementation PR was opened. The STOP summary was posted to the board as comment `5549449875` under the Owner's subsequent standing instruction.

15.9, 15.10, canon adoption, marker deletion, pointer movement, graduation and closeout remain fenced. The next step is the source/consumer ruling stated above, followed by implementation within the same handoff or its explicitly amended projection. Final graduation routing remains orchestration-owned exact-head clearance and relay lint after substantive review.

## Required-anchor acknowledgments

Acknowledgments bind to the hashes emitted by the current anchor queries; they do not constitute a graduation or proof certificate.

```text
ANCHOR-ACK: accumulator-exact-vs-soft-semantics@0efceafc77cf
ANCHOR-ACK: accumulator-op-v2-invariants@32fb4fc36080
ANCHOR-ACK: actionband-8x-sequencing@067ef8ace1e0
ANCHOR-ACK: actionband-axis-budget@52275c538689
ANCHOR-ACK: actionband-binding-laws@d6a8b1b2d673
ANCHOR-ACK: actionband-constitutional-placement@d56d9a04a620
ANCHOR-ACK: actionband-crossing-surface@623db585f145
ANCHOR-ACK: actionband-determinism-lifecycle@6306c484732c
ANCHOR-ACK: actionband-eml-payload-purity@2a1d981f3958
ANCHOR-ACK: actionband-executive@9c7e004e213b
ANCHOR-ACK: actionband-fenced-questions@c40674d92d18
ANCHOR-ACK: actionband-field-triad-authority@56cf5cdf2d2c
ANCHOR-ACK: actionband-gpu-physical-model@3252c1b3c3b5
ANCHOR-ACK: actionband-native-authority-table@541a03cb00a1
ANCHOR-ACK: actionband-performance-model@8d93f06d4bae
ANCHOR-ACK: actionband-target-forms@c3b7bce99f1f
ANCHOR-ACK: actionband-vendorization-direction@20336db0d366
ANCHOR-ACK: admission-ladder-necessity-test@4bedf826f6f7
ANCHOR-ACK: candidate-f-exhaustive-proof-method@7c5ce0b93dab
ANCHOR-ACK: clausescript-compatibility@a483808213fb
ANCHOR-ACK: core-gpu-residency@f9b19479262a
ANCHOR-ACK: core-overlays@94a8955e46f2
ANCHOR-ACK: core-property-value-model@1be54f2e4803
ANCHOR-ACK: core-rf-arenas@5dd14f66897b
ANCHOR-ACK: eml-admission-shapes@bdcc0b9512f7
ANCHOR-ACK: eml-extension-ladder@7755bc72ffbe
ANCHOR-ACK: eml-integration-plan@8eba54b02320
ANCHOR-ACK: eml-triad-integration@dada7d680557
ANCHOR-ACK: evaluation-identity-invariants@64ad30392930
ANCHOR-ACK: exact-numeric-candidate-f@6938a2efadb5
ANCHOR-ACK: field-policy-time-decisions@4309cdd821fe
ANCHOR-ACK: field-sweep-preservation@acc521a5a361
ANCHOR-ACK: founding-ontology-invariants@46802793fba7
ANCHOR-ACK: intrinsic-constrained-clearing@957b7c81b756
ANCHOR-ACK: movement-front-adjudications@5af6a29acb75
ANCHOR-ACK: one-tree-owners-never-spatial@9a10c1be61ee
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: overlay-closure-thesis@241cc54c5706
ANCHOR-ACK: overlay-designer-closure@4a047b29243d
ANCHOR-ACK: overlay-germ@f0c8d2ebade9
ANCHOR-ACK: overlay-promoted-laws@248c7893b462
ANCHOR-ACK: overlay-scale-laws@c2ffb2826df7
ANCHOR-ACK: rf-arena-allocation-invariants@82864469489b
ANCHOR-ACK: rf-arena-substrate@17b5f1e5c2ba
ANCHOR-ACK: rf-market-candidate-laws@357f6c986fff
ANCHOR-ACK: rf-market-falsifiers@d40df3102410
ANCHOR-ACK: rf-market-mirror-cycle@1a1aca57e5f6
ANCHOR-ACK: rf-market-port-census@3bc7792c27e3
ANCHOR-ACK: rf-market-receive-not-recompute@a9856f7f3bc1
ANCHOR-ACK: rf-market-settled-code-census@3bc7792c27e3
ANCHOR-ACK: scanner-selftest-delta-gate@34fb2662baae
ANCHOR-ACK: seal-residue-cross-crate@c61c33d90efc
ANCHOR-ACK: simthing-0087-binding-laws@567370293add
ANCHOR-ACK: simthing-0087-pillars@61487cba1f9e
ANCHOR-ACK: stead-events-are-rf@1f3bdde23cee
ANCHOR-ACK: stead-rejected-shapes@7f75f8b55271
ANCHOR-ACK: stead-shared-surface-ledger@2d7062067214
ANCHOR-ACK: stead-spatial-contract-core@8585db4ac631
ANCHOR-ACK: stemthing-binding-laws@6787a118c3ca
ANCHOR-ACK: stemthing-lane-not-leg@9a1d443b7981
ANCHOR-ACK: stemthing-slot-identity-ruling@02c87b9126e1
ANCHOR-ACK: structural-execution-convergence@6b4cedec482b
ANCHOR-ACK: workshop-candidate-homing@3e584f0ad175
```
