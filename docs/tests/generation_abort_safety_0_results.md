# GENERATION-ABORT-SAFETY-0 — falsifier-first evidence

Status: **PROBATION / proof-present / DA-review-pending / OPEN / UNMERGED**. Outcome B implemented: one private seal faults a touched, unfinished generation. The committed RED precedes all production changes. The final PR/board certificate binds this packet to the tested implementation head and hosted runs; no graduation is claimed.

- Dispatch: board comment5560677371.
- Base: `90b20764dd134de431a7736ffb6bc65103421017`.
- Branch: `codex/generation-abort-safety-0`.
- Preserved RED commit: `91e1dc02f691e6314c8152894fee24c72e2b86bc` (cargo exit101 on this committed head).
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

No production file changed in the preserved RED commit. The final remedy uses only the handoff's ten listed paths. The core, driver, and GPU sources named above outside that list were read-only archaeology.

## Implemented seal and consumer ordering

| Operation | Existing seal transition | Effect ordering |
|---|---|---|
| begin_generation(N) | Ready -> Untouched | No economics; harmless validation leaves Untouched |
| authorize_economics | Untouched -> Touched; repeated Touched remains Touched | Before the first non-rollback-proven effect |
| unconsumed Drop | Untouched -> Ready; Touched -> Faulted | Never advances generation |
| begin/validation of faulted generation | typed GenerationFaulted | No permit can be minted for another economic door |
| successful finish_generation | Validates once, N -> N+1, consumes permit, Ready | Sole advance; next generation is clean |

`crates/simthing-core/src/tree_execution_context.rs:343` replaces the outstanding boolean with one atomic generation state on the same private seal. Begin is at539, finish at642, economic authorization at706, Drop at783. The state is not serialized or copied into scenario data. Migration shares the same seal; an old touched permit cannot clear a fault through a new incarnation. Existing consumed, foreign-capsule, stale-incarnation, wrong-generation, duplicate-outstanding and out-of-sequence guards remain.

`crates/simthing-driver/src/session.rs:630` validates the real permit, performs the existing placement/ingress checks, then authorizes before hot-cycle intents, GPU/RF/mapping writes and coordinator advance. Both step/run at1899 and recording at2030 take that same permit before the hot cycle. A non-boundary success retains the capability in an Option; the next call takes it back. The Option is not a second generation counter, fault state or authority. Any error during execution drops the local unconsumed capability immediately. Boundary validation still compares the physical day to the sealed generation; finish remains after the existing boundary/economy tail (2007 and2190). Recording's existing early empty-boundary commit and normal pre-finish replay frame order are unchanged.

`crates/simthing-driver/src/growth_entitlement.rs:209` covers both backends. Structural-only settlement authorizes before its effectful resolver. Nonempty ordinary settlement authorizes before taking the continuation at264, including CPU/vendorized clearing and grant publication. The resident and CPU continuation algorithms and departing-flow refusal are unchanged.

`crates/simthing-driver/src/resident_clearing_runtime.rs` validates permits at the immediate, commitment-partition, spatial, temporal-mint and temporal-execution doors. Temporal mint authorizes at1199 before mint bookkeeping/encoding/submission (1215). Common dispatch authorizes at1336 before schedule reservation (1338), exact encoding/live-head append and submission (1385). Spatial and temporal dispatch use this same common door. No second executor was added.

Materialization at1432 remains an asynchronous observer of products already submitted under the permit: it maps their live-head segment and fills the already-reserved canonical schedule rows. It cannot reserve a new economic row, recompute settlement, append a live head or mint demand. Keeping observation available does not authorize another generation. Proof-only readbacks likewise remain observations. Topology rebind preserves the existing live head and runs under the ordinary session's already-touched permit; it creates no new generation authority.

## GREEN witnesses and effect accounting

The primary referee extends the actual-session RED over ResidentRequired/CpuVendorizedOracle x one/three ticks per day x run/recording (eight real sessions). Each first attempt reaches the same existing typed late refresh failure. Resident sessions assert the real G4/U6 product; CPU sessions assert a real grant lifecycle publication. Each retries through step, recording, then step again without changing the session or its clock. All24 retries fault before another hot cycle.

```text
ResidentRequired, ticks_per_day=1, recording=false:
first: ResourceEconomy(RecipeExecutionMetadataMismatch { recipes:0, coefficients:1, order_bands:0 })
tick=1 day=1 resident_facts=[(generation1, claimant, G4, U6)]
retry: ExecutionIdentity("generation GenerationStamp(1) is faulted after an unfinished economic authorization")
tick=1 day=1 same resident_facts; changed_gpu_cells=0
```

The entire IntegrationSchedule (including its reservation state), GPU value vector, tick/day and canonical facts remain equal on retry. There is no duplicate or newly stranded reservation/product from a second attempt. **The first failed attempt's effects remain visible; this is fail-stop, not rollback.** No claim is made that poisoning erases those effects or provides durable crash recovery. Without a retry permit and before any new hot cycle, the second attempt cannot reach resident reserve/append/submission or CPU settlement.

The auxiliary production-runtime witness is explicitly supplemental, not a substitute for the ordinary-session falsifier. For each axis it mints N7, rejects a wrong-generation dispatch before economics, drops the untouched permit, remints N7, actually dispatches/materializes G4/U6 and finishes N7->N8. Another untouched N8 drop proves a clean next generation. Separate N8 aborts cover immediate execution, spatial child execution, temporal mint alone, and temporal execution alone. Temporal execution uses a demand minted under the prior successfully finished generation, so its current permit starts untouched. Spatial descent uses the required same-generation parent under the same permit. Each abort refuses repeated begin requests at N8 and N9 with GenerationFaulted(N8), without advancing or changing the schedule.

Healthy actual sessions stop between hot cycles and alternate step/run/record across three real generations in both backends. A seal-level supplemental check proves successful validation remains untouched, invalid finish does not advance, consumed authorization refuses, and migration cannot launder a touched abort.

## Conditional E8 qualification

The final admitted driver source changes the existing bundle. Before changing either pin, the amended ordinary session refused at admission:

```text
ResidentClearing(LiveHead(UnqualifiedAdapter {
  required: 9297387406934488982,
  observed: 17379237397841828533
}))
0 passed; 1 failed; cargo exit101
```

The old required pin is `0x8106_f496_4185_3796`. The actually observed replacement is **`0xf12f_7455_8d8b_dab5`**. Only QUALIFIED_RESIDENT_CLEARING_FINGERPRINT and the independent QUALIFIED_RECORD_FINGERPRINT change to that identical literal, in the same implementation commit as the final source delta. No subsequent bundled production source edits occurred after capture. The comparator, qualification record, component list, ABI, build.rs and old numerical goldens are unchanged.

Qualification matrix: four passed. Actual ABI mutant: qualified=f12f74558d8bdab5, mutant=d234ad4c829c9094. Independent semantic-component mutations refuse: child-share 64ee276d0238ffaa; planner 2b716ef5bab71ea6; temporal15.2 0889516b5eb5ecae (qualified component bundle89df223f64d94729). Ordinary session admission and the independent parity referee pass with the fixed replacement pin.

## Validation and exact-head certificate

- Touched-package cargo check: PASS (core + driver; GPU/workshop compiled by the focused and qualification runs).
- New referee: **3 passed /0 failed /0 ignored**; eight aborted sessions,24 retries, four resident axes, healthy multi-tick sessions and seal migration.
- Frozen referees unchanged except the expressly authorized independent pin: **31 passed /0 failed /0 ignored** across eight groups. 15.7=4,15.8=6,15.9=3,parity=1,apportionment=7,axes=5,substrate=4,recursive formalization=1.
- Core permit/context unit corpus: **3 passed**; qualified GPU mutation corpus: **4 passed**.
- Structural battery: **15/15 exit0**: inventory, inventory-drift --prove, constitutional surface and selftest, lifecycle schema and prove, digest, detachability and selftest, anchor and selftest, plan typing, observation bypass, slot census and overlay census.
- Lifecycle: expired=0/audit=0. Detachability: production/proof coupling=0/0. Overlay census: routes77/discovery73/residue87/unclassified0/open0. Slot census: universe51/assigned51/dup0/missing0/blockers0.
- Authority census unchanged: resident production authority1; duplicate settlement/economic adapter/global coupling/private solver all0; CPU oracle definitions5/call sites2/embedder reexports1; frozen recursive-filter peer residue2.

The final PR/Board return supplies tested_code_sha, full workspace/all-targets totals, Agent Scan verdict and every INSPECT, hosted Doctrine Scan/Exec run IDs and actual artifact verdicts. This external certificate avoids editing source after pin capture or putting a self-referential commit hash in this file. Full command: `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet`. Generated baseline report output is restored after the run; it is not an authorized source change.

Local raw evidence is retained in `.git/1510-owner-red-head.log`, `1510-e8-old-pin-refusal.log`, `1510-focused-new.log`, `1510-frozen.log`, `1510-core-authority.log`, `1510-qualification.log`, `1510-workspace.log` and `1510-final-*.log`. The load-bearing RED/GREEN/qualification details are reproduced above for remote review.

## Changed-file ledger and fences

| Admitted path | Final change |
|---|---|
| crates/simthing-core/src/tree_execution_context.rs | Single seal state machine and typed fault |
| crates/simthing-driver/src/session.rs | Same permit spans hot cycles and both loops |
| crates/simthing-driver/src/resident_clearing_runtime.rs | Touch before reservation/append and temporal mint |
| crates/simthing-driver/src/growth_entitlement.rs | Touch before continuation/CPU and resident settlement |
| crates/simthing-gpu/src/resident_clearing_runtime.rs | One qualification literal only |
| crates/simthing-workshop/tests/resident_clearing_parity_0.rs | One independent qualification literal only |
| crates/simthing-workshop/tests/generation_abort_safety_0.rs | Preserved first RED plus three final referees |
| docs/tests/generation_abort_safety_0_results.md | Evidence, ordering, ACKs and review limits |
| scripts/ci/test_inventory.tsv | Three append-only owned test rows |
| scripts/ci/anchor_reach_log.tsv | One append-only reach row |

No new production failpoint, alternate settlement, rollback/recovery/replay mechanism, global fault registry, authority object, departing-stream disposal, gate/workflow edit, canon rewrite, pointer movement, graduation or closeout. Orchestration retains INSPECT triage, exact-head clearance and relay-lint; DA retains graduation review.

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
