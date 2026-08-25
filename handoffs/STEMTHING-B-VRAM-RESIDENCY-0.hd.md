---
rung: STEMTHING-B-VRAM-RESIDENCY-0
kind: rung
track: 0.0.8.7
base_sha: b6c5ca2e102559d7a9bbe88aab99472c1e34eddf
audience: coding
model_tier: frontier
owner_approved: true
expected_route: DA-RESERVE(gate-wiring)
owner_notes: "Authorized after DA graduation of 11.2a and explicit un-fencing of 11.2b. Governing law is docs/stead_stemthing_unification.md §12; the ladder row and this handoff are pointers. Entitlement comes from the graduated 11.2a market germ; this rung adds no second clearing path. No ORIENT-RECEIPT was staled by the 11.2a stamp."
surfaces: ["crates/simthing-kernel", "crates/simthing-gpu", "crates/simthing-spec", "crates/simthing-driver", "crates/simthing-sim", "docs/tests", "docs/design_0_0_8_7_rf_arena_modernization.md", "scripts/ci/test_inventory.tsv"]
forbidden: ["any second residency clearing path, retry loop, or same-generation re-clear", "treating free-list order or any physical-placement iteration order as entitlement policy", "disguising committed overlap or out-of-bounds corruption as ordinary unmet demand U", "crashing the sim for ordinary provisional placement infeasibility", "a second history, telemetry, remap, or generation-authority surface", "retiring allocator policy or free-list-order machinery reserved to 11.2c", "Vendor Door or simthing-embedder work reserved to 11.2d", "11.2c, 11.2d, 11.3 implementation, 11.4, 12.x, Vector CostBand, or the three fenced ClauseThing baseline reds", "retired handoff receipts 890b845b5655 or aff35cbd509a", "adding cargo build/test/check to CI"]
required_checks: ["before edits render/read this handoff; perform fresh Frontier coding orientation on live master at rule stamp 9ee3f7649d1fc790 and ACK every rendered REQUIRED-ANCHOR", "resolve docs/stead_stemthing_unification.md §12 as governing law and implement 11.2b only", "map the landed entitlement-to-placement chain before edits: graduated 11.2a grant product -> owning placement boundary -> placement oracle -> canonical 6.1 recorded schedule -> existing epoch-rebind/remap surface; STOP if any required link is absent", "prove residency entitlement is consumed from the graduated 11.2a market germ and no second clearing path, residency retry loop, or same-generation retry exists", "prove provisional cleared entitlement commits only after legal disjoint in-bounds placement succeeds", "plant infeasible provisional placement and require typed refusal, zero commit, quantity retained as ordinary U, refusal on the one recorded 6.1 schedule, and next-generation revaluation", "plant overlap or out-of-bounds in already-committed authoritative placement state and require a session hard fault, never U", "prove extent disjointness and bounds at every actual grant-placement or reallocation boundary while unchanged placements avoid global per-generation re-proof", "prove relocation uses the existing granter-owned epoch boundary remap/rebind surface and preserves stable logical identity; mint no second remap/history authority", "run focused residency placement/refusal/corruption/remap batteries plus Agent/Doctrine, inventory, orientation and applicable lifecycle checks; return exact base/head/tested_code_sha and hosted workflow IDs; structural certificate is owed at DA graduation"]
stop_conditions: ["the graduated 11.2a entitlement cannot reach the existing residency placement boundary without a second clearing path", "ordinary infeasibility cannot be represented as typed refusal plus U without committing geometry", "committed overlap/out-of-bounds cannot hard-fault at the authoritative placement judge", "legal disjoint in-bounds placement requires a new allocator manager, clearing engine, history, remap authority, or generation authority", "relocation cannot use the landed epoch-rebind/remap surface while preserving logical identity", "completion requires allocator-retirement work reserved to 11.2c, Vendor Door work reserved to 11.2d, or any later fenced rung"]
---
## BUILD
- Implement **11.2b `STEMTHING-B-VRAM-RESIDENCY-0` only** against live master. Governing law: `docs/stead_stemthing_unification.md` §12. The row and this handoff are pointers; §12 is the law.
- VRAM Residency remains the distinct engine-native market: **entitlement consumes the graduated 11.2a market germ; physical extent realization remains kernel physics behind the owning placement boundary.**
- Land the two-stage fail-closed placement law. A cleared residency entitlement is provisional until the owning boundary placement oracle proves a legal, disjoint, in-bounds realization. Only then may authoritative placement commit.
- Preserve the separation between market entitlement and physical geometry. Free-range/index structures may remain downstream placement machinery; their iteration/free-list order is never grant policy.
- Relocation remains granter-owned boundary structural work on the existing epoch-rebind/remap surface with recorded remap; stable logical identity survives physical relocation.
- Land one signal-only results doc, one current-evidence-index line, and the 11.2b row as PROBATION in the coding diff; no pointer movement.

## EXIT-PROOF
- **Planted RED 1 — provisional infeasibility:** a cleared entitlement that cannot be legally placed returns a typed placement refusal; nothing commits; the quantity remains ordinary `U`; the refusal rides the one canonical 6.1 recorded schedule; claimant re-values next generation. Ordinary infeasibility must not crash the sim.
- **Planted RED 2 — committed corruption:** overlap or out-of-bounds observed in already-committed authoritative placement state is an invariant breach and hard-faults the session. Committed corruption must not be disguised as `U`.
- Successful placement proves the minted extent is disjoint and bounded by the granter's authoritative extent at every actual grant-placement/reallocation boundary. Unchanged placements require no global per-generation re-proof.
- Residency entitlement demonstrably comes from the graduated 11.2a market germ; there is no residency-specific clearing path, retry loop, same-generation retry, or free-list-order entitlement policy.
- Relocation/rebind reuses the existing recorded boundary-remap surface and preserves stable logical slot identity; no second history/remap/generation authority is created.
- Focused falsifiers RED a second clearing door, ordinary-infeasibility crash, committed-corruption-as-U, free-list policy reach, duplicate remap/history authority, and premature 11.2c retirement work. Structural certificate is supplied by DA at graduation.

## FENCES
- 11.2c `STEMTHING-B-ALLOCATOR-RETIREMENT-0` remains fenced: do **not** retire allocator-policy/free-list-order machinery in this rung beyond proving it is downstream of market-decided entitlement.
- 11.2d `VENDOR-DOOR-GRANTING-SURFACE-0` remains fenced; no `simthing-embedder` changes.
- 11.3 remains the pointer rung but implementation stays blocked until all four StemThing-B predecessor rungs graduate. 11.4, 12.x, Vector CostBand, and the three ClauseThing baseline reds remain fenced.
- Do not add cargo execution to CI. Retired handoff receipts `890b845b5655` and `aff35cbd509a` must never be dispatched.
