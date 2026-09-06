# GENERATION-ABORT-SAFETY-0 — falsifier-first evidence

Status: **PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED**. This initial packet preserves the actual-session RED before any production remedy. Implementation and final validation remain pending.

- Dispatch: board comment5560677371.
- Base: `90b20764dd134de431a7736ffb6bc65103421017`.
- Branch: `codex/generation-abort-safety-0`.
- HD-RECEIPT: 836f90b31f09
- ORIENT-RECEIPT: 96a2fe34ba3d
- orientation_rule_stamp: 0a0fe2267e3cc6a9
- orientation_digest_sha: b87026dc8420460649d5d413a53a91025fe0652ab7190bf35a030a25a776e51c

The merged handoff was rendered before the explicitly requested fresh coding orientation. All 55 required anchors were queried; 49 retain the hashes already read in this session and the six additional anchors were read in full. ACKs follow below.

## Observed baseline and adjudication

The actual ordinary resident-required SimSession uses an admitted authored market, real TreeExecutionLease permit and real resident executor. An existing malformed resource-economy refresh (zero recipes but one recipe coefficient) creates a deterministic typed `ResourceEconomySyncError::RecipeExecutionMetadataMismatch` after resident dispatch/materialization and before finish_generation. A normal authored per-tick Add(0.25) on a bounded property makes any subsequent hot-cycle write observable. There is no new production failpoint, forged permit, host reconstruction or alternate economic path.

```text
first step_once: ResourceEconomy(RecipeExecutionMetadataMismatch { recipes:0, coefficients:1, order_bands:0 })
tick=1 day=1 resident_facts=[(generation1, claimant2, G4, U6)]
same-session retry: ExecutionIdentity("generation permit mismatch: expected GenerationStamp(1), observed GenerationStamp(2)")
tick=2 day=2 resident_facts unchanged; changed_gpu_cells=1
FAILED: touched abort must refuse before another hot cycle (2 != 1)
0 passed / 1 failed / 0 ignored
```

This distinction is load-bearing: ordinary retry does **not** duplicate the exact resident row in this fixture. The physical coordinator has already advanced, so the next boundary asks for generation2 while the lease remains at uncommitted generation1. Its mismatch refusal comes **after another real hot GPU cycle**. The first canonical G/U record is retained; a second GPU value write occurs without finishing generation1. No clock rewind or direct runtime harness is used to manufacture same-generation resident replay.

**Outcome B: rollback is absent/incomplete.** The state-machine remedy must reject the retry before the hot path as well as close permit-Drop reopening. The session permit must cover the whole generation's hot cycles and boundary, not be minted only after hot work. This remains one capability on the existing private seal, not a second authority or rollback design.

## Ordering census before edits

- `crates/simthing-core/src/tree_execution_context.rs`: private seal has live_generation and outstanding bool. Lease begin validates identity/generation, swaps outstanding; verifier validates consumed/capsule/incarnation/generation; finish checks N+1, advances once, consumes and clears outstanding. Unconsumed Drop clears outstanding without distinguishing effects.
- `crates/simthing-driver/src/session.rs`: both step_once/run and record_to_path call run_hot_cycle before begin_generation. Hot work includes feeder mutation, GPU tick/RF/mapping and coordinator advance. After permit mint: commitment publication; ordinary boundary hook; growth/flow settlement; action-band dispatch; fission enrollment/resync; economy refresh; finish_generation. Recording additionally drains replay/spec records and writes its frame before normal finish.
- `crates/simthing-feeder/src/dispatcher.rs`: the real GPU pipeline precedes tick/day counter increments. Day rollover precedes the ordinary session's boundary permit mint. This source is read-only and outside remedy scope.
- `crates/simthing-driver/src/growth_entitlement.rs`: current-flow authorization; continuation is taken before the resident/CPU branch; optional temporal mint; structural growth clear; ordinary flow dispatch; materialization; replacement continuation. CPU/vendorized branch performs its existing clear and schedule grant records, then replaces CPU continuation. Both posture paths must consume the same sealed authorization.
- `crates/simthing-driver/src/resident_clearing_runtime.rs`: dispatch, commitment-partition, spatial, temporal-preparation and temporal-dispatch all validate the permit. The common dispatch_market builds the exact plan, reserves schedule rows, encodes exact work and live-head append, then queue-submits. Temporal preparation encodes/submits a mint and mutates live-head mint state. Materialization reads submitted products and fills their reserved schedule rows.
- `crates/simthing-gpu/src/resident_clearing_runtime.rs`: existing append/mint bookkeeping and real command encoding are below the driver authorization boundary. This file's only conditionally admitted edit is its qualification pin.
- `crates/simthing-sim/src/boundary.rs`: readback; scheduled lifecycle publication; spec hook; overlay lifecycle and structural processing; growth resolver; admitted mutation and GPU resync. Its errors propagate without rollback. Read-only archaeology; no change planned here.
- `crates/simthing-driver/src/resource_economy_sync.rs`: malformed recipe metadata is an existing typed failure in the ordinary late refresh. Read-only; used by the real-session falsifier.

No production file has changed in this initial RED packet. Scope for the later remedy is the handoff's exact listed files; 15.7/15.8/15.9 semantics, E8 comparison/record/build script, departing-stream disposal, recovery, canon and graduation remain fenced.

## Required anchor acknowledgments

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
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: overlay-closure-thesis@241cc54c5706
ANCHOR-ACK: overlay-designer-closure@4a047b29243d
ANCHOR-ACK: overlay-germ@f0c8d2ebade9
ANCHOR-ACK: overlay-promoted-laws@248c7893b462
ANCHOR-ACK: overlay-scale-laws@c2ffb2826df7
ANCHOR-ACK: rf-arena-allocation-invariants@82864469489b
ANCHOR-ACK: rf-arena-substrate@17b5f1e5c2ba
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
