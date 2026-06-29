# AS-KIND-OUT-OF-TICK-0E Results

## Status

PASS — production kind-read audit over `crates/simthing-sim/src` is clean; AS-KIND-OUT-OF-TICK-0 promoted to PROBATION.

## PR / branch / merge

- Branch: `codex/as-kind-out-of-tick-0e`
- PR: (pending)
- Merge: (pending)

## What changed

- Ran targeted source audit over `crates/simthing-sim/src` (and `tests/` for classification); zero unclassified production kind-read residue after 0A–0D.
- Added load-bearing closure test `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` in `kind_production_audit.rs` (strips `#[cfg(test)]` modules + comments, scans production text).
- Promoted AS-KIND-OUT-OF-TICK-0 ladder row IN PROGRESS → PROBATION in `docs/design_0_0_8_4_admission_substrate.md`.

## Production kind-read audit

Audit commands (closure evidence for this rung only):

```text
rg -n "SimThingKind|SimThingKindTag|\.kind\b|kind_tag|match .*kind" crates/simthing-sim/src
rg -n "SimThingKind|SimThingKindTag|\.kind\b|kind_tag|match .*kind" crates/simthing-sim/tests || true
```

| File / line | Pattern | Classification |
|---|---|---|
| `boundary.rs:1270` | `match registry.get(event.event_kind)` | Not SimThing kind — threshold `event_kind` dispatch (production) |
| `threshold_registry.rs:228,255` | `match self.get(event.event_kind)` | Not SimThing kind — threshold registry (production) |
| `replay.rs:231-232` | `value.get("kind")` + `match kind` | Not SimThing kind — replay JSON record kind (production) |
| `fission_clone_source_view.rs:9,20` | `v.kind` / `v.inner().kind` in `//!` docs | Documentation — compile_fail proof examples only |
| `boundary.rs:1454+` | `SimThingKind`, `SimThingKindTag`, fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `fission.rs:643+` | `SimThingKind`, `.kind`, assertions | Test/fixture — `#[cfg(test)] mod tests` |
| `tree_mutation.rs:527+` | `SimThingKind`, `SimThingKindTag` | Test/fixture — `#[cfg(test)] mod tests` |
| `delta_log.rs:316+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `overlay_lifecycle.rs:301+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `property_expiry.rs:272+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `observability.rs:174+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `threshold_registry.rs:1033+` | `SimThingKind`, `SimThingKindTag` | Test/fixture — `#[cfg(test)] mod tests` |
| `tree_index.rs:63+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `replay.rs:453+` | `SimThingKind` fixtures | Test/fixture — `#[cfg(test)] mod tests` |
| `crates/simthing-sim/tests/*.rs` | `SimThingKind`, `.kind`, fixtures | Test/fixture construction — allowed |
| All other `src/*.rs` production paths | — | No `SimThingKind` / `SimThingKindTag` / `.kind` / `kind_tag_to_kind` hits |

**Production residue:** none. Fission clone-source selection remains marker/property-driven (0A–0B); child spawn routes through `ResolvedFissionChildBlueprint` in simthing-core (0C); `FissionCloneSourceView` exposes no kind backdoor (0D).

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `as_kind_out_of_tick_production_audit_has_no_runtime_kind_reads` | Accidental production `SimThingKind` / `SimThingKindTag` / `.kind` / `kind_tag_to_kind` reintroduction in `simthing-sim/src` |
| `fission_clone_source_view_hides_kind_compile_fail` | Direct `view.kind` access does not compile |
| `fission_clone_source_view_inner_kind_backdoor_compile_fail` | `view.inner().kind` does not compile (`inner()` removed) |
| `fission_child_blueprint_hides_kind_compile_fail` | Spawn blueprint kind-free at runtime-facing API (simthing-core) |
| `fission_clone_capability_subtrees_behavior_preserved` | 0A/0B/0C clone behavior intact |
| `fission_spawns_same_child_kind_after_core_spawn_refactor` | 0C core blueprint spawn parity |
| `projected_fission_slots_match_resolved_clone_sources` | Slot projection agrees with marker-resolved clone sources |

## Scope Ledger

| File | Reason |
|---|---|
| `crates/simthing-sim/src/kind_production_audit.rs` | Closure audit test module |
| `crates/simthing-sim/src/lib.rs` | Wire `#[cfg(test)] mod kind_production_audit` |
| `docs/tests/as_kind_out_of_tick_0e_results.md` | Results + audit table + Scope Ledger |
| `docs/tests/current_evidence_index.md` | One AS-KIND-OUT-OF-TICK-0E row |
| `docs/design_0_0_8_4_admission_substrate.md` | AS-KIND-OUT-OF-TICK-0 → PROBATION |

## Known gaps / next

- AS-4 (`AS-SIM-SEMANTIC-FREE-0`): seal public `simthing-sim` surface so game-concept names cannot cross the crate boundary; test/fixture kind reads remain until then.
- DA review for AS-KIND-OUT-OF-TICK-0 PROBATION → DA-APPROVED is out of scope for this slice.
