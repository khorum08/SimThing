---
rung: STEMTHING-B-ALLOCATOR-RETIREMENT-0
kind: rung
track: 0.0.8.7
base_sha: d0c8c91b8fd0b5c1d341ca83011b748a842b5eb0
audience: coding
model_tier: frontier
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Authorized after DA graduation of 11.2b. Governing law is docs/stead_stemthing_unification.md §12; the 11.2c ladder row and this handoff are pointers. Follow the 11.1d seal shape: free-list ORDER as grant policy becomes unrepresentable, while physical free-range/index structures may survive only as downstream machinery after market-decided entitlement. Net deletion is the success signal; mint no new scan surface."
surfaces: ["crates/simthing-kernel", "crates/simthing-gpu", "crates/simthing-driver", "crates/simthing-core", "crates/simthing-sim", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests", "scripts/ci/allow/kernel_surface.txt", "scripts/ci/test_inventory.tsv"]
forbidden: ["any new allocator, allocation manager, clearing path, entitlement authority, or retry/convergence mechanism", "free-list or physical iteration order deciding entitlement or grant quantity", "removing physical free-range/index machinery that is still required downstream of market-decided entitlement", "new scan/gate surface invented solely to police this deletion", "registry column append or GPU buffer-sizing infrastructure unrelated to allocator policy", "11.2d Vendor Door or simthing-embedder work", "11.3 implementation, 11.4, 12.x, Vector CostBand, or the three ClauseThing baseline reds", "retired handoff receipts 890b845b5655 or aff35cbd509a", "adding cargo build/test/check to CI"]
required_checks: ["before edits render/read this handoff; perform fresh Frontier coding orientation on live master at rule stamp 9ee3f7649d1fc790 and ACK every rendered REQUIRED-ANCHOR", "resolve docs/stead_stemthing_unification.md §12 as governing law and implement 11.2c only", "map every surviving allocator-policy reach before edits: entitlement source -> placement boundary -> physical free-range/index machinery; distinguish policy from downstream geometry machinery and STOP on ambiguity", "identify and delete every legacy residency allocation POLICY path outside the 11.2a market plus 11.2b physical-placement boundary", "prove a planted attempt to grant by free-list or iteration order is unrepresentable: compile-fail or typed admission RED for its own named reason", "prove market-decided entitlement still reaches 11.2b level-local placement and relocation unchanged after policy deletion", "prove registry column append and GPU buffer sizing remain infrastructure and are not accidentally retired", "show net production deletion and no replacement manager/policy/scan surface", "run focused allocator-retirement falsifiers plus applicable residency, market-germ, inventory, Agent/Doctrine, orientation and lifecycle checks; return exact base/head/tested_code_sha and hosted workflow IDs", "structural certificate is owed at DA graduation because production paths are deleted"]
stop_conditions: ["a legacy allocation-policy path cannot be distinguished from required downstream physical placement machinery", "removing free-list order as policy would require a new allocator, manager, clearing path, or retry loop", "the 11.2a market entitlement can no longer reach 11.2b placement without restoring allocation policy", "registry column append or GPU buffer sizing must be semantically redesigned to complete the deletion", "the planted free-list-order policy attempt cannot be made unrepresentable without a new scan surface", "completion requires 11.2d, 11.3, 11.4, 12.x, Vector CostBand, or ClauseThing-red work"]
---
## BUILD
- Implement **11.2c `STEMTHING-B-ALLOCATOR-RETIREMENT-0` only**. This is the deletion rung. §12 is governing law; the row and handoff are pointers.
- Retire legacy residency allocation POLICY outside the graduated 11.2a market and 11.2b physical-placement boundary. Entitlement is already market-decided; no second allocator policy may survive beside it.
- Make free-list ORDER or any equivalent physical iteration order **unrepresentable as grant policy**. Do not merely make it non-default, discourage it in prose, or leave a callable compatibility path.
- Preserve physical free-range/index structures only where they are ordinary geometry machinery downstream of a cleared entitlement. Slot/extents remain kernel physics; registry column append and GPU buffer sizing remain infrastructure.
- Prefer deletion, narrowing, and ordinary-lane convergence over replacement abstractions. Net production deletion is a required success signal.
- Update one focused results artifact, current evidence index/inventory as required, and mark the 11.2c row PROBATION in the coding diff; no pointer movement.

## FENCES
- Follow the **11.1d seal shape**: bypass must be impossible by construction or typed admission, not detectable only by a new census/scan.
- Do not invent a new allocator, manager, policy registry, clearing engine, retry loop, validation framework, or scan surface to replace what is being deleted.
- Do not retire the 11.2b level-local placement book, extent vocabulary, placement oracle, canonical schedule rows, or epoch-rebind relocation authority.
- 11.2d remains fenced; no Vendor Door or `simthing-embedder` changes. 11.3 remains the pointer but implementation stays blocked until all four B rungs graduate.
- 11.4, 12.x, Vector CostBand, and the three ClauseThing baseline reds remain fenced. No cargo execution in CI. Retired receipts remain forbidden.

## EXIT-PROOF
- **Planted seal RED:** attempt to author or invoke free-list/iteration order as the source of entitlement or grant quantity must fail to compile or RED at admission for its own named reason. Renaming the policy must not evade the seal.
- **Positive survival witness:** a real 11.2a `MarketGrantRecord` still crosses the 11.2b provenance bridge and receives legal level-local physical placement using surviving free-range/index machinery only as downstream geometry.
- **Relocation survival witness:** the existing transactional epoch-rebind/remap route still preserves logical identity after deletion; no second remap/history authority appears.
- **Infrastructure fence:** registry column append and GPU buffer sizing remain intact and are shown not to decide entitlement.
- **Deletion signal:** report exact retired production paths/symbols and net production-line deletion; any replacement allocation-policy abstraction is a failure.
- Standing falsifiers RED surviving free-list-order policy, second clearing/allocator authority, policy hidden behind geometry order, new scan surface, accidental 11.2b physics deletion, and premature 11.2d/later work. Structural certificate is DA-side at graduation.
