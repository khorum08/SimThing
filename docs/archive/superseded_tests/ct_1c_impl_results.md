# CT-1c implementation results — capability tree from ClauseScript

Status: **IMPLEMENTED / PASS** (2026-06-11, frontier agent; rung design authority by product
delegation). **The first "designer writes Clausewitz, SimThing runs it" proof is green:** a
ClauseScript-authored tradition set hydrates onto the `capability_tree_v1` pattern verbatim and
runs through a real GPU session — prereq DAG ordering enforced, payloads activated, faction
values transformed.

## Scope ledger

| Specified element | Status |
|---|---|
| One capability tree (small tradition set) | **implemented** — `adaptability` tree: adopt → recycling → finisher prereq chain, 3 entries, 1 category |
| → `capability_tree_v1` pattern | **implemented** — hydrates to the existing `CapabilityTreeSpec`/builder/handler machinery, zero substrate changes |
| Prereq DAG → threshold ordering | **proven on GPU** — entry firing without its prereq waits in `OnPrereqMet`; activation sweep activates it in dependency order after the prereq lands; threshold AND prereq (finisher stays suspended with prereq met but threshold unfired) |
| Payload activation | **proven on GPU** — Owner-targeted `Permanent` payloads flip from `Suspended` and transform the owning faction's `ct1c::potency` |
| Parity green | **yes** — hydrated pack canonically identical to the hand-authored RON baseline; per-faction install shape asserted via `preview_install` |
| Scope model | same-scope/install-owner only, exactly as the accepted SCOPE-MEMO §8 predicted for this rung — **no SPEC-SCOPE ticket pulled** |

## What was built

**Hydration only** (`crates/simthing-clausething/src/hydrate.rs`): a `tradition_tree` block in
the safe synthetic dialect →

| ClauseScript | `simthing-spec` |
|---|---|
| `tradition_tree { id, kind, owner }` | `CapabilityTreeSpec { tree_id, tree_kind, owner_kind, install: AllOfKind{owner} }` |
| `category { namespace, name, display_name }` | `CapabilityCategorySpec` (one `SimProperty` per category) |
| `tradition { id, display_name, cost }` | `CapabilitySpec` (`cost` → `research_cost`, activation `Threshold`) |
| `possible { has_tradition = X }` | `prereqs: [CapabilityPrereqSpec { category: "ns::name", entry_id: X }]` (same-category, source order) |
| `modifier { targets_property, amount_mult\|amount_add }` | `CapabilityEffectSpec` (Amount delta, `when_activated: Permanent`, `effect_target: Owner`) |

No `simthing-spec`, `simthing-sim`, `simthing-gpu`, driver, or WGSL changes — the rung consumed
the accepted capability substrate as-is.

## Doc correction (source-is-ground-truth)

`docs/capability_tree_v1.md` §4–§7 examples authored prereq `category` as a bare name
(`"propulsion"`); the builder's `parse_category_ref` requires `namespace::name` and rejects bare
names with `UnknownPrereqCategory`. All seven example occurrences corrected, with a note at the
§4 definition. Discovered when the bare form failed install in this rung's first test run.

## Test-hygiene fix (same pattern, both rungs)

The CT-1b/CT-1c GPU smokes treated *any* `open_from_spec` error as "no GPU — skip", which
masked a real install error in this rung's first run. Both now probe GPU availability with a
bare `SimSession::open` and `expect()` the spec install, so install regressions fail loudly.

## The consumer ran (real reduction, GPU)

`tradition_prereq_dag_orders_activation_on_gpu`:

1. **Stage 1** — research progress seeded on *recycling* only (cost 15, seeded 16): its GPU
   threshold fires, but its prereq (adopt) is unresearched → `activation_mode == OnPrereqMet`,
   zero active entries, all 3 payloads still `Suspended`.
2. **Stage 2** — progress seeded on *adopt* (cost 10, seeded 11): adopt's threshold fires, the
   handler activates adopt, and the OnPrereqMet sweep activates recycling **in the same
   boundary, in dependency order**. Final state: `active = [adopt, recycling]`, finisher still
   `Suspended` (its prereq is now met but its threshold never fired — threshold AND prereqs).
3. **Payload proof** — the two activated Owner-targeted payloads (Add 5 + Add 7 on
   `ct1c::potency`) transform the **faction's** values on subsequent GPU ticks
   (read back ≥ 12.0 from the faction slot; was 0). No handler errors, no diagnostics.

CPU-side: `preview_install` asserts the install shape (1 instance owned by the faction, 3 GPU
threshold registrations, 3 suspended payloads, cloned `Custom("tradition_tree")` child under
the faction, both properties registered).

## Files changed

- `crates/simthing-clausething/src/hydrate.rs` — `tradition_tree` lowering
- `crates/simthing-clausething/tests/ct_1c_tradition.rs` — parity, install shape, GPU DAG proof
- `crates/simthing-clausething/tests/fixtures/ct1c_tradition_set.clause` + `..._baseline.ron`
- `crates/simthing-clausething/tests/ct_1b_recalc.rs` — GPU-skip masking fix (shared pattern)
- `docs/capability_tree_v1.md` — prereq category-ref examples corrected
- `docs/design_0_0_8_1_clausething_production_track.md` — §11 ledger row
- `docs/worklog.md`, this report

## Commands run

```text
cargo test -p simthing-clausething                          # all green (41 tests incl. 3 CT-1c, GPU proof ran)
cargo test -p simthing-spec --test pr2_compile              # 11 passed
cargo test -p simthing-driver --test session_integration    # 19 passed (GPU)
cargo fmt --all -- --check                                  # clean
```

`cargo test --workspace` — **not run**.

## Closure answers

1. **Designer writes Clausewitz, SimThing runs it?** Yes — the fixture is authored ClauseScript,
   hydrated through the real parse→expand→hydrate path, installed per-faction, and the GPU
   session enforces the tradition semantics end to end.
2. **Parity green?** Yes — canonical authoring identity vs the hand-authored RON baseline; the
   numeric path is the existing pass-3/threshold substrate already under the standing
   `pass3_overlay_matches_evaluator` parity guard; no new compute was added.
3. **Prereq DAG → threshold ordering?** Proven: OnPrereqMet wait, dependency-ordered sweep
   activation, threshold-AND-prereq conjunction.
4. **Payload activation?** Proven: `Suspended → Permanent` flips with Owner targeting; faction
   values transformed on GPU.
5. **Scope successor needed?** No — install-owner targeting sufficed, as the accepted SCOPE-MEMO
   §8 ruled; the first cross-entity capability payload pulls SPEC-SCOPE-1.
6. **Substrate changes?** None; `simthing-sim`/GPU/WGSL untouched.
7. **Paradox/lab corpus?** None — fixture is original SimThing-authored ClauseScript.
8. **Artifacts?** This report; no scratch retained.
9. **`cargo test --workspace` avoided?** Yes.
