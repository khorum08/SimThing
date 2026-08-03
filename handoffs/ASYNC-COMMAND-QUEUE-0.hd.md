---
rung: ASYNC-COMMAND-QUEUE-0
kind: rung
track: 0.0.8.7
base_sha: dc15d484
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "DA/Owner posture 5172687124 authorizes 6.2 PROCEED and confirms Frontier — Codex 5.6 Sol MAX. PR #1610 @ dc15d484 delegates handoff authorship to orchestration when exit_proof_coverage_check.sh is clean; coverage is rows=0. DA retains PRE-DISPATCH scope/proof/fence review: do not implement until this handoff is reviewed, merged, and explicitly dispatched. The §3/6.2 row is authoritative; 6.2b/6.3 remain forbidden."
surfaces: ["crates/simthing-core/src/generation_stamp.rs", "crates/simthing-core/src/automaton.rs", "crates/simthing-core/tests", "crates/simthing-spec/src/spec/owner_channel_rf.rs", "crates/simthing-spec/src/spec/channel_key.rs", "crates/simthing-spec/tests", "crates/simthing-sim/src/boundary.rs", "crates/simthing-sim/tests", "crates/simthing-driver/src/session.rs", "crates/simthing-driver/src/spec_session.rs", "crates/simthing-driver/src/owner_channel_rf_compile.rs", "crates/simthing-driver/tests", "crates/simthing-feeder/src", "crates/simthing-kernel/src", "scripts/ci/test_inventory.tsv", "docs/tests", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/orchestrator_orientation.md"]
forbidden: ["drop/overwrite/throttle conserved pending products, slow-child waiting/lockstep, or eventual-conservation waiver; same-key pending values coalesce by exact sums and the seam is a holding account", "stamp summing, bucket-latest replay collapse, or a second injection/replay recorder/sequence authority beside the 6.1 IntegrationSchedule", "a second directive transport, inferred tree-distance routing, or inferred staleness tolerance beside existing RF + authored order/max-disbursement bands", "session-local interned owner ids across a seam, unstamped live cross-site standing reads, mid-generation queue application, or torn snapshots", "scenario/domain vocabulary in engine src/tests, shipped-scenario witnesses, gate/invariant weakening, or 6.2b/6.3/downstream implementation"]
required_checks: ["fresh coding ORIENT-RECEIPT + current ANCHOR-ACKs + HD-RECEIPT quoted before edits", "same-key N-product burst coalesces every OwnerChannelRfBucket sum field exactly and queue cardinality is distinct buckets; child+seam+parent is exact in both directions; dropped-product and in-flight-escape mutants RED", "slow child never blocks; authored seam tolerance hard-errors; coalesced carrier stamp=max while the ONE IntegrationSchedule preserves the full generation set; bucket-latest-collapse and second-recorder mutants RED", "CommandDeficit stays on owner_silo_disburse_down/runtime_local_allocation_from_disbursement with authored bands; downward ancestor view is barrier-stamped; only canonical OwnerRef crosses; double-buffer snapshot proof is torn-free", "single-log replay including standing reads is bit-exact; synthetic-only affected crate batteries + inventory/residue/detachability/lifecycle/orientation/anchor gates green"]
stop_conditions: ["exact coalescing or instantaneous child+seam+parent conservation appears to require dropping, information loss, blocking, or eventual-consistency waiver", "full coalesced generation membership appears to require a second recorder/replay path rather than extending IntegrationSchedule", "bidirectional standing view or seam identity appears to require unstamped live cross-site reads, a second transport/owner authority, or session-local intern ids", "completion requires 6.2b/6.3 work, Invariant Set change, gate loosening, inherited-red weakening, or shipped-scenario proof"]
---
## BUILD
- Land one async CPU action/seam queue applied only at generation barriers; a slow child never blocks. Coalesce upward products by canonical `{owner,resource,scope}` with exact field sums and at most one pending value per distinct bucket; the seam is the holding account.
- Extend the 6.1 `IntegrationSchedule` as the ONE replay recorder with a queue/injection row kind. Values coalesce; stamps do not: carrier stamp=`max`, recorder retains the FULL contributing generation set per product.
- Admit authored staleness tolerance per seam. Preserve 6.0b directive transport and authored `order_band`/`max_disbursement_band`; no second command path or distance inference.
- Make the seam bidirectional: upward products and downward ancestor standing/policy view are barrier-stamped; canonical `OwnerRef` crosses; generation-consistent shadow/snapshot reads are double-buffered/torn-free.
## FENCES
- Live within a resolution site; stamped across a seam. One transport, one recorder, one owner identity.
- Conservation is instantaneous `child + seam + parent`; never drop conserved products and never wait to avoid accounting them.
- Preserve 6.0/6.0b/6.1/6.1b landed semantics. No 6.2b resolution-site split, 6.3 STEAD staleness field/soak, or downstream work.
## EXIT-PROOF
- Single-log replay is bit-exact; same-key bursts coalesce exact sums at distinct-bucket cardinality; dropped-product mutant REDs.
- `child + seam + parent` is exact in both directions; in-flight escape REDs; lag never blocks and authored tolerance breach hard-errors.
- Carrier stamp is newest=max while schedule keeps FULL contributing generations; bucket-latest collapse and second-recorder paths RED. Command deficits remain on RF disburse/local-allocation with authored bands; interned owner ids never cross.
- Downward standing/policy state replays bit-exactly and double-buffer proof is torn-free. Synthetic-only focused + affected crate/gate batteries green. Return PROBATION / DA-review-pending; coder does not `/clearance`, merge, move pointer, or begin 6.2b/6.3.
