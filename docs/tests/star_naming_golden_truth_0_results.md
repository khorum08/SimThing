# STAR-NAMING-GOLDEN-TRUTH-0 Results

> **Status: PROBATION / proof-present / orchestration-review-pending.** Coding
> lane only; no merge, graduation, pointer movement, or 13.4+ work is claimed.

**Date:** 2026-08-29
**Dispatch:** Board `5465510114`
**HD-RECEIPT:** `3e2e4cf9f3e0`
**ORIENT-RECEIPT:** `2557bebcf996` (`orientation_rule_stamp=5eaee638be917b8f`)
**Dispatch master / base:** `1f921d73b5a016622221d93f667b41c1b1e57b84`

## Adjudication

**LAWFUL canonical-save serialization drift. Re-blessed with provenance. Not a
naming-algorithm regression.**

| Field | Value |
|---|---|
| First divergent commit | `f4fd99109099dcd8d6082f843af604b15b48d9df` |
| Date | 2026-07-26 22:15:29 -0500 |
| Subject | `SESSION-WIRING-KILL-SWEEP-0 (1.2): toggle kill + generation rename` |
| Parent (GREEN) | `7986aea511288d936899f4fad90975dbe55cdbbb` — focused test `ok. 1 passed` |
| Hit (RED) | `f4fd9910` — `canonical star-name golden is stale` |
| Seam | `SimThing.spawned_day` → `spawned_generation` in `crates/simthing-core/src/simthing.rs`; serde aliases keep `spawned_day` loadable; canonical save emits the new field name |
| Naming files unchanged since origin | `star_names.rs` last `052cc192` (#1304, 2026-07-11); `rng.rs` last `e7f4fc55` (2026-06-14); golden last `052cc192` until this re-bless |

Leaf-path comparison of pre-bless golden vs current canonical save: 0 value
changes after normalizing `spawned_day`/`spawned_generation`. 3,002 spawn
fields renamed in place; 1,500 placements and 2,714 links identical; all 1,500
display names remain the `assign_star_names(TP_SEED=770421, sorted generated
system ids)` products.

Why the new bytes are canonical: P0 generation ruling made spawn calendar
vocabulary a front-end binding; 1.2 renamed the wire field. Load still accepts
the 2026-07-11 golden via `#[serde(alias = "spawned_day")]`. The byte-oracle
must track the current save representation, not freeze a retired field name.

## Blessing

```text
UPDATE_STUDIO_STAR_NAMING_GOLDEN=1 cargo test -p simthing-clausething --test studio_star_naming_pass_0 star_naming_canonical_tp_all_systems_have_display_names -- --exact
```

Next drift repeats the procedure in
`crates/simthing-clausething/tests/studio_star_naming_pass_0.rs`: name the first
divergent commit, then re-bless or fix. Do not weaken byte equality.

## Canonical source

`simthing_mapgenerator::star_names::assign_star_names` is the entity-name
authority: sorted/deduped generated system IDs; `MapGenSeed(seed ^
NAMING_SEED_DOMAIN)`; Fisher-Yates shuffle of the 4096-name catalog through
`MapGenRng`; prefix/core/suffix composition. No second seed, catalog, or
test-only name source.

## Containment

Zero kernel/core/gpu/sim/driver/embedder edits. Zero `.clause` rewrites. Zero
oracle weakening. `hydrate_scenario_with_source_base` path untouched.

## Certificate

`cargo test --workspace --offline`: **133 suites / 590 passed / 0 failed / 15
ignored**. Focused `studio_star_naming_pass_0` and
`studio_star_naming_repair_0` GREEN. `cargo check -p simthing-mapgenerator -p
simthing-clausething`, `agent_scan`, orientation/digest/doc-budget `--check`
PASS.
