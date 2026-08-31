---
rung: RESIDENT-CLEARING-PLAN-0
kind: rung
track: 0.0.8.7
base_sha: 2ba6e28cd4ba83a52a5b0fa99a4a299ca31ae9d6
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(binding)
owner_approved: true
owner_notes: "Owner/DA authority Board 5478577994; canonical Phase-14 law Board 5471915320 rev 5. 14.1 is DA-GRADUATED in #1907 @ 9a036a44 and stamped by #1908 onto live master 2ba6e28cd4ba83a52a5b0fa99a4a299ca31ae9d6; pointer is 14.2. Build the candidate in its FINAL core/kernel/GPU home; workshop is only the parity/qualification consumer. The Owner GERM MANDATE is binding: this is the permanent base inherited by future specialist economic-resolution facilities, so germ-quality form is a first-class exit criterion. Clearing semantics are frozen; closeout and the unrelated performance seed remain held behind Phase 14."
surfaces:
  - "crates/simthing-core/src/tree_execution_context.rs"
  - "crates/simthing-core/src/lib.rs"
  - "crates/simthing-kernel/src/resident_clearing_plan.rs"
  - "crates/simthing-kernel/src/lib.rs"
  - "crates/simthing-gpu/src/resident_clearing_plan.rs"
  - "crates/simthing-gpu/src/lib.rs"
  - "crates/simthing-workshop/src/resident_clearing_plan.rs"
  - "crates/simthing-workshop/src/lib.rs"
  - "crates/simthing-workshop/tests/resident_clearing_plan_0.rs"
  - "docs/tests/resident_clearing_plan_0_results.md"
  - "docs/tests/current_evidence_index.md"
  - "scripts/ci/test_inventory.tsv"
  - "scripts/ci/closeout_artifacts.tsv"
  - "scripts/ci/anchor_reach_log.tsv"
forbidden:
  - "any change to constrained-clearing semantics, score bits, ordering, equality bands, grouping, requested totals, apportionment, grants, unresolved U, replay, generation authority, or structural-consequence law"
  - "any production consumer wiring, cutover, replacement or narrowing of the CPU oracle doors, GPU primary-authority claim, dispatch kernel, score/band implementation, apportionment implementation, or other 14.3-14.6 work"
  - "any source change in simthing-spec, simthing-driver, simthing-sim, simthing-feeder, simthing-clausething, or simthing-mapeditor"
  - "any field added to SimThing or any per-SimThing realm, host, process, device, transport, scheduler, incarnation, seam, physical-row, GPU-handle, or network identity"
  - "any SimThingId widening, ABI/schema migration, reliance on process-global raw-id uniqueness, foreign raw local id, or assumption that two trees share compact ordinals"
  - "any global mutable semantic registry, allocator, schedule, generation authority, session singleton, host-wide lock, all-tree barrier, or mutable kernel singleton; immutable ABI-compatible pipeline/artifact caches are not semantic authority"
  - "any test-only realm wrapper, raw-id injection escape, unsafe identity constructor, identity theatre, or workshop-local duplicate of the resident-plan algorithm"
  - "any host address, process/thread id, adapter/device/queue/endpoint, physical row, buffer offset, or GPU handle in canonical semantic serialization"
  - "any remote transport/networking implementation, distributed commit, async-streaming default, same-realm partition lease design, or detachment-shape selection"
  - "any Cargo.toml, Cargo.lock, WGSL, .github/workflows/**, scripts/ci/**/*.sh, scripts/ci/**/*.py, pointer, workplan graduation/stamp, orientation-guide, closeout apply/reap, or next-rung authoring change from coding"
required_checks:
  - "before edits render/read this handoff; obtain a fresh coding-role ORIENT-RECEIPT against the exact merged dispatch master, carry the HD-RECEIPT, and ACK every rendered REQUIRED-ANCHOR"
  - "FIRST STEP publish a final-home and coupling census from the exact base: existing local identity, generation, schedule, registry/residency, GPU buffer, and global-state surfaces; name the final owner of every new type before edits"
  - "prove TreeRealmId, ExecutionIncarnation, TreeExecutionContext, RealmQualified<TLocalId>, and SeamFactId are O(1) per tree/seam, canonically serializable, host-agnostic, and do not alter SimThing or SimThingId layout"
  - "prove TreeExecutionContext binds exactly one existing local root, generation authority, caller-owned schedule, registry/residency attachment, realm, and active incarnation without minting a second authority; stale-incarnation seam admission fails closed"
  - "prove compact owner/resource/scope/draw dictionaries assign ordinals from canonical total order, not hash-map iteration or registration order; replay reconstruction and migration recreation produce byte-identical semantic plans"
  - "prove cross-tree seam integration remaps canonical realm-qualified identities into destination-local ordinal space and never consumes a foreign ordinal"
  - "prove all count, offset, byte-size, alignment, and scratch calculations use checked arithmetic and hard-error before allocation when admitted budgets are exceeded"
  - "prove the concrete resident plan and GPU buffer/ABI representation live in final kernel/GPU modules and are consumed by the workshop harness without a second implementation or production dispatch"
  - "exercise Tree A realm A local id 7 and Tree B realm B local id 7 simultaneously through real TreeExecutionContext doors with different incarnations and generations; prove no alias, shared authority, shared schedule, shared dictionary, or buffer overlap"
  - "prove migration preserves realm and local semantic identities while changing incarnation and recreating the same plan; prove a speculative fork mints a distinct realm; prove stale-incarnation facts are rejected"
  - "prove SeamFactId carries source realm, seam id, source generation, and source ordinal so retry identity is distinct from lawful multiplicity"
  - "run cargo check for simthing-core, simthing-kernel, simthing-gpu, and simthing-workshop; run the focused resident_clearing_plan_0 witness plus frozen predecessor clearing/generation/neutrality seals"
  - "run test inventory, closeout-ledger lifecycle/expiry, Agent Scan, doc-budget, applicable doctrine/orientation/anchor checks, hosted Doctrine Scan/Exec, then fresh exact-head /clearance and /relay-lint; any red is a STOP"
  - "return PROBATION / proof-present / DA-review-pending with exact base/head, receipts and anchor ACKs, changed-file census, final-home map, canonical-byte/digest proofs, dictionary permutation/replay/migration proofs, A/B local-id-7 witness, budget table, no-global-state/no-consumer proof, commands and hosted workflow/job IDs"
stop_conditions:
  - "the required final-home dependency direction cannot be achieved without a crate cycle, manifest change, lower-layer dependency inversion, or workshop-local substitute"
  - "any implementation choice changes or must reinterpret frozen clearing semantics or advances score/bands/apportionment beyond a data/layout placeholder"
  - "TreeExecutionContext would own or duplicate a schedule, registry, residency authority, generation authority, or root rather than bind the existing per-tree authority"
  - "canonical identity or plan serialization would contain transient host/device/physical coordinates or depend on map/registration iteration order"
  - "deterministic dictionary reconstruction, migration recreation, or destination-side seam remapping cannot be proven without foreign ordinals or process-global uniqueness"
  - "a stale incarnation can be accepted, or overlapping local id 7 can be proved only through a test-only wrapper/raw-id fiction"
  - "buffer/scratch sizes or offset arithmetic can overflow, truncate, narrow admitted ranges, or allocate before a hard budget check"
  - "the final GPU-resident plan cannot be instantiated through the existing adapter/test posture; report the exact blocker rather than relocating the candidate"
  - "any new public horizon API lacks a concrete workshop consumer or a dated HORIZON-ENTRY naming its 14.3-14.6 consumer"
  - "any focused, predecessor, inventory, lifecycle, doctrine, hosted, clearance, or relay-lint check becomes red"
---
## BUILD
- **Land the real execution/seam vocabulary in core.** Introduce the minimal semantic types named by Phase 14 and one real `TreeExecutionContext` binding. Durable identity is realm-qualified and host-agnostic; transient execution handles stay outside canonical form. Migration preserves realm/local identity and changes incarnation; forks mint realms; stale incarnation fails closed.
- **Build a concrete germ-quality resident plan in kernel.** The first constrained-clearing composition owns deterministic typed owner/resource/scope/draw dictionaries, checked dense ranges, generation/context binding, semantic plan digest, and admitted buffer/scratch budgets. Assignment derives from canonical ordering. This is a concrete reusable economic-resolution germ, not a host-algorithm port, trait forest, or empty extension scaffold.
- **Build its final GPU representation now.** Define the stable ABI/buffer descriptors and per-tree owned resident buffer set in `simthing-gpu`; make widths, alignments, ranges, and checked byte budgets explicit. Two trees may share a physical adapter/cache, never semantic state, ordinals, schedules, ranges, or authority. Do not add a dispatch or WGSL kernel yet.
- **Use workshop only as consumer and parity harness.** Construct the final core/kernel/GPU objects through their public doors; never restate plan construction in workshop. Exercise admission-order permutations, replay reconstruction, migration recreation, stale-incarnation rejection, destination ordinal remap, budget refusals, and simultaneous realm-A/realm-B local-id-7 plans with different generations.
- **Record signal-only evidence.** Publish the final-home/type/layout table, canonical bytes or digest, dictionary ordering law, budget formula/table, A/B witness, migration/fork/seam facts, no-global-state/no-production-consumer proof, focused commands, and changed-file census. Lease only the new 14.2 artifacts in the existing closeout ledger.

## FENCES
- **Frozen semantics and rung boundary.** 14.2 lands identity, context, plan, dictionaries, budgets, and resident ABI/storage only. Scoring/equality bands begin at 14.3; exact apportionment at 14.4; full parity at 14.5; production cutover/egress at 14.6.
- **Kernel containment and one-way dependencies.** Core owns semantic identity/context; kernel owns the reusable semantic plan; GPU owns physical resident storage; workshop consumes upward. No application/spec type enters core/kernel/GPU and no final algorithm lives in workshop.
- **Tree-local truth.** Realm/incarnation/seam metadata is O(1) per tree/seam. Base SimThing and local IDs remain compact. Foreign facts cross only in canonical realm-qualified form and are remapped on receipt; raw ids and ordinals never cross as identity.
- **No hidden global authority.** New state is instance-owned and per tree. Sharing an immutable compatible pipeline/artifact cache is lawful; sharing mutable semantic state, schedules, allocators, generations, or dictionaries is not.
- **Germ mandate.** Prefer a small concrete composable substrate whose typed axes and checked layout can support future facilities. Reject translation-shaped host containers, constrained-clearing-only names in generic substrate, gratuitous abstractions, inert APIs, or specialization hooks without a present consumer/horizon marker.

## EXIT-PROOF
- **Realm/seam law:** final core types serialize canonically; layout of `SimThing`/`SimThingId` is unchanged; migration/fork/incarnation laws and stale rejection are executable; `SeamFactId` distinguishes retry identity from lawful multiplicity.
- **Deterministic plan:** identical admitted semantic inputs under reversed/permuted registration produce identical dictionaries, ranges, serialized plan, and digest; replay and migration recreate it; receiving trees remap seam identities without foreign ordinals.
- **Resident independence:** final GPU buffers are per-tree, budget-checked, disjoint, and simultaneously instantiable for realm A/id 7 and realm B/id 7 at different generations with no shared mutable kernel state.
- **Honest scope:** workshop contains only the consumer/harness; no production caller, dispatch, WGSL, CPU-door change, score/band/apportionment implementation, or 14.3+ delta exists.
- **Preservation and routing:** frozen 14.1/clearing/generation/neutrality seals stay green; evidence and ledgers are current; exact-head hosted checks, `/clearance`, and `/relay-lint` are green. Return PROBATION for DA deep-tree review; coding does not merge or move the pointer.
