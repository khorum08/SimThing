# TREE-EXECUTION-AUTHORITY-LIFETIME-0 — implementation evidence

Status: **PROBATION / proof-present / DA-review-pending / UNMERGED**.

Authority: handoff `handoffs/TREE-EXECUTION-AUTHORITY-LIFETIME-0.hd.md`, board dispatch `5547692843`, `HD-RECEIPT: ebdd6a0d00b4`, and coding `ORIENT-RECEIPT: 8b80bf8f3db9` under rule stamp `450b6a470c797a88` and orientation digest `58ed6da678f4e04c1138420c2e6fd9923ec9eb3b04449ec5da54e7651ce9fcb4`. The branch merge-base is `510162ae615ab38fb7ae8a882fbc012c422a72ca`, the merged handoff commit on `origin/master`; the handoff's authored projection base remains recorded as `f33366de377984280e12341c1782d5f329f2542b`.

## Result

One opaque `TreeExecutionLease` now survives for the complete session/runtime lifetime. The borrowed `TreeExecutionAuthority` exists only while the boundary seals that lease; later admission and topology rebind create short-lived `TreeExecutionBinding` views from the lease. The resident executor retains only a verifier when the session owns authority, while standalone proof runtimes retain their own lease. There is no global writer, lease, or executor registry.

Fresh session identity is minted from 128 bits of operating-system entropy and never from scenario bytes, paths, clone addresses, device facts, or a process-global counter. An explicit serde `PersistedTreeExecutionIdentity` is the durable realm source of truth. Restore validates that record, preserves its realm, and increments its incarnation. A recorded semantic fork derives a different realm and begins at incarnation one.

One non-cloneable, non-serde `TreeGenerationPermit(N)` now authorizes the whole generation. The session mints it once at the boundary, shares it by reference across every resident root/spatial edge, exact product sealing, and temporal N→N+1 preparation, then consumes it once at the barrier. Wrong-generation, reused, foreign-capsule, stale-incarnation, and second-outstanding-permit uses fail before economic work. Dropping an unconsumed permit releases only the reservation and does not advance generation, so failed work remains retryable and cannot partially commit authority.

## Authority-lifetime archaeology

| Production seam | Before this rung | 15.7 disposition |
|---|---|---|
| fresh `SimSession::open` | deterministic realm default derived outside the runtime lifetime | `mint_fresh_execution_identity` creates a fresh persisted record; private open validates it |
| restore/reopen | no explicit persisted execution-identity door | `open_restored(scenario, record)` preserves realm and mints `incarnation + 1` |
| semantic fork | raw realm reuse/derivation could be caller-shaped | `open_semantic_fork(scenario, source, recorded_fork)` produces a distinct realm and incarnation one |
| boundary admission | borrowed authority/context/binding could end after admission | `BoundaryProtocol::seal_tree_execution_lease` mints one owned lease retained by `SimSession` |
| resident admission | standalone capsule was constructed for admission | session passes a transient binding and executor retains a verifier; standalone/proof admission retains its owned lease |
| generation dispatch | raw generation stamp reached each edge | session calls `begin_generation` once; the same permit reference reaches every edge |
| product sealing | qualification/generation checks but no whole-generation capability | permit validation precedes exact settlement, immutable product sealing, and live-head append |
| temporal preparation | product generation shaped admission | the same N permit is required to prepare N+1 demand; actual N+1 dispatch requires a newly minted N+1 permit |
| topology rebind | new borrowed seal assembled around current state | same lease creates a transient binding; executor realm/incarnation/live head/pending temporal state remain intact |
| compiler/source qualification | production runtime spawned `rustc -Vv` and read source identity at runtime | build script captures compiler, Cargo features, lockfile, and named semantic source bundle into `OUT_DIR`; runtime only reads embedded constants |

Production creation census:

- session capsule mint: `BoundaryProtocol::seal_tree_execution_lease` → `TreeExecutionAuthority::seal` → `seal_lease`;
- standalone/proof capsule mint: `ResidentClearingRuntime::admit_market_with_persistence_deformations` → `seal` → `seal_lease`, with the resulting lease retained by the runtime;
- session generation mint doors: the ordinary and replay-recording boundary loops only; resident recursive calls accept `&TreeGenerationPermit` and cannot mint;
- raw session identity reuse door removed; explicit fresh, restore, and recorded-fork doors remain;
- global lease/writer/executor registry census: zero.

## Persisted identity and witness matrix

Canonical data record:

```text
PersistedTreeExecutionIdentity {
    realm_bytes: [u8; 16],
    incarnation: u64,
}
```

`TreeRealmId`, `ExecutionIncarnation`, `TreeExecutionContext`, `TreeExecutionLease`, and `TreeGenerationPermit` remain non-serde authority values. Deserialized record fields are inert until `realm()` and `incarnation()` validate nonzero values.

| Operation | Realm law | Incarnation law | Witness |
|---|---|---|---|
| ordinary open A | fresh OS-entropy realm A | `1` | focused double-open test |
| byte-identical ordinary open B | fresh realm B, `B != A` | `1` | focused double-open test |
| JSON round-trip | exact `realm_bytes` retained | exact recorded value retained | serde round-trip test |
| explicit restore(A record) | realm A retained | recorded incarnation + 1 | focused restore test |
| recorded semantic fork(A, key) | new realm, not A | `1` | focused fork test |

## Whole-generation permit proof

| Mutant or attempted use | Result before economic work |
|---|---|
| second permit for live N | `GenerationPermitAlreadyOutstanding` |
| N permit presented as N+1 | `PermitGenerationMismatch` |
| N permit used after the barrier | `GenerationPermitAlreadyConsumed` |
| equal raw realm/generation from another capsule | `AuthorityCapsuleMismatch` |
| old-incarnation permit after migration | `StaleIncarnation` |
| finish N as anything except N+1 | `GenerationAdvanceOutOfSequence` |
| failure/drop before finish | outstanding reservation released; live generation unchanged |

The 15.5 cross-product runs the root and changed-granter/changed-scope descendant edges under the same permit, retains `T_s=(G4,U6,N50)`, produces authored N+1 demand `2 + U6 = 8`, and produces half-deformed demand `2 + U6/2 = 5`. E6 remains work-conserving with reservation only from actual conserved in-flight holdings.

## E8 complete semantic-kernel qualification

Build provenance captured for the exercised production feature set:

| Field | Exact value |
|---|---|
| Rust compiler | `rustc 1.95.0 (59807616e 2026-04-14)`, full `-Vv` embedded at build time |
| Cargo features | `EML_RESOURCE_PROFILING` |
| `Cargo.lock` digest | `1bd6ab6097f443d7` |
| semantic-kernel bundle | `3a3753ffa5b15608` |
| exact qualified runtime tuple | `bea9102eb252a3ca` |
| workgroups / ABI | `[32, 64]` / `1` |

The named bundle covers `Cargo.lock`; child-share EML construction; EML nodes/opcodes; accumulator plan types, builder, encoding, CPU oracle, runtime session, and canonical WGSL interpreter; arena allocation/synchronization/hierarchy planner sources; resident clearing plan sources; persistence-deformation program and recursive-intake CPU/WGSL; exact apportionment CPU/WGSL; exact GPU projection ABI; and the production resident runtime. Paths, lengths, and bytes participate in the deterministic digest, and Cargo emits `rerun-if-changed` for every component.

| Independent mutation | Qualified | Mutant | Outcome |
|---|---:|---:|---|
| child-share EML source | `3a3753ffa5b15608` | `9eb6a5212bd6d5dd` | stale bundle differs |
| 15.2 recursive-intake shader | `3a3753ffa5b15608` | `204682e2a42690b7` | stale bundle differs |
| production arena planner | `3a3753ffa5b15608` | `9cbdd1e37b3d7639` | stale bundle differs |
| exact projection ABI in production feature tuple | `bea9102eb252a3ca` | `dda3d737bd41edeb` | typed `UnqualifiedAdapter` before execution |

The production runtime source has zero `Command::new`, `rustc -Vv`, `std::fs`, or `read_to_string` occurrences. Those operations exist only in `crates/simthing-gpu/build.rs` and execute during Cargo build.

## Proof transcript

Test-first compilation initially failed on the absent lease/permit APIs and permit-threaded dispatch signatures. After implementation, the exact source passed:

- `cargo test -p simthing-core tree_execution_context`: 3 passed;
- `cargo test -p simthing-gpu --features eml-resource-profiling resident_clearing_runtime`: 4 passed, including four independent E8 mutations;
- `cargo test -p simthing-workshop --test tree_execution_authority_lifetime_0`: 4 passed;
- frozen 14.5 plan: 3 passed; frozen 14.5 parity terminal referee: 1 passed;
- frozen 14.6 cutover: 3 passed;
- 15.0 formalization: 1 passed; 15.1 unification: 2 passed; 15.2 persistence: 3 passed; 15.3 consequence ingress: 2 passed; 15.4 oracle quarantine: 2 passed; 15.5 recursion axis: 5 passed; 15.6 substrate binding: 4 passed; E5 apportionment: 7 passed;
- `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet`: exit 0, every emitted test-result group green;
- inventory: 1426 discovered / 1426 registered, zero missing or extra;
- inventory drift prove, constitutional check and 12-fixture selftest, lifecycle schema/prove, sanctioned-surface digest, detachability and four-fixture selftest, anchor integrity and 12-fixture selftest, plan-typing, observation-bypass, slot, and overlay-germ censuses: PASS;
- Agent Scan: PASS, zero hard failures and zero inspect flags.

The full all-target run also exercises ordinary session generation pacing and replay-recording boundaries with the lease/permit layer active. All required mutants are red for their named reason; all required positive and frozen economic witnesses are green: **FULL ZERO-RED**.

## Scope and non-actions

The implementation changes core authority/identity types, the sim boundary, driver session/resident runtime, GPU qualification provenance, frozen permit-threaded referees, the focused 15.7 referee, test inventory, and this evidence/index. It does not edit workflow or CI gate implementation, decide the parked cap-collision question, rewrite/adopt canon, delete section 8.4 markers, start compression or post-15.7 work, move a pointer, graduate, merge, or close the track. Hosted Doctrine Scan/Exec, exact-head clearance, relay-lint, and review identifiers are carried by the PR and board relay for the immutable pushed head.
