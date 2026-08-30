# CLEARING-WEIGHT-DEFORMATION-LIFECYCLE-0 Results

- Status: **PROBATION / proof-present / DA-review-pending**
- Rung: `CLEARING-WEIGHT-DEFORMATION-LIFECYCLE-0`
- Handoff-authored provenance base: `8140206d66f3bfa6246227abd6c095a9d2ba22e3`
- Exact dispatch base: `8099af484250b51f416477c83fec8925993e9ea4`
- tested_code_sha / final_head_sha: PR- and Board-relay-bound after this evidence commit; this file does not self-hash
- ORIENT-RECEIPT: `343e3c133d7c`
- role: `coding`
- orientation_rule_stamp: `dd6572b07cbd2905`
- orientation_digest_sha: `eb52aa993e6c245dbdbbe2ebeda3625e653891b33a6bf733a361f8cf6f444fad`
- HD-RECEIPT: `d04e47a3972c`
- expected route: `DA-RESERVE(binding)`

## Pre-edit archaeology

The required dispatch-base inventory was published before production edits in
Board comment `5469846100`.

| Surface | Dispatch-base census |
|---|---|
| Empty dependency admission | One `DerivedDependencyIndex::admit(Vec::new())` at `clearing_weight_projection.rs:167`. |
| Existing 7.8a lifecycle | `ChangedLocus` -> frozen `DerivedDependencyIndex` / `DerivedDependencyTarget` -> `invalidate` -> `remap_range`; the overlay projection already exercised this exact path. |
| Panic indexing | One `Index<&SimThingId>` implementation; zero production consumers; ten test-only call sites across the 13.6 matrix (six) and germ (four). |
| Dynamic deformation | Zero production clearing-weight deformation/refresh callers. Resolver production references were definition/re-export only. |

Dispatch-base seal truth was captured before edits:

- `clearing_weight_span_unification_0.rs`: blob `f59e31eebc030ab7c67f30b5a184f43e0e228f15`, 18 assertion sites;
- `stemthing_b_flow_market_germ_0.rs`: blob `2837de9cea426dd69b911cef302f475456abb769`, 46 assertion sites;
- base matrix/germ execution: respectively 2 and 4 passed, 0 failed.

Post-edit the same files retain exactly 18 and 46 assertion sites. Their only
changes are source-locus plumbing required by dependency admission and
mechanical replacement of panic indexing with `effective_weight(...).unwrap()`
or an equivalent `Option` assertion. Authored scenarios, numeric expected
values, tie/remainder law, DecimalField `17:33:50`, generation authority, and
replay assertions are unchanged.

## Existing-vocabulary lifecycle

The clearing projection now admits these frozen rows using the existing 7.8a
types, without a second registry or source discriminator:

| Authored operand source | Existing dependency target | Lawful affected scope |
|---|---|---|
| default `ChangedLocus(root, property, role)` | `DerivedDependencyTarget::SpanRoot(root)` | all logical rows in the projection |
| override `ChangedLocus(source, property, role)` | `DerivedDependencyTarget::SpanRoot(override.simthing_id)` | only the overridden logical subtree |

Admission remains eager. A refresh accepts the current eager default/override
values plus changed loci and the existing `GenerationStamp`; it validates the
frozen override dependency shape, calls the existing `invalidate`, and invokes
the existing `remap_range` only for returned affected ranges. It creates no
cache, scheduler, dirty map, mutable registration surface, GPU clearing path,
or production runtime caller. `ChangedLocus` is re-exported intact so the
authored witness can name the already-graduated writer-blind key; its narrowing
vocabulary stays kernel-private.

The panic-bearing `Index<&SimThingId>` implementation is deleted. Search finds
zero implementation or consumer rows; `effective_weight(SimThingId) ->
Option<f32>` is the sole effective-weight participant lookup.

## Affected-only and end-to-end proof

The focused four-row root / ship / freighter / commodity witness admits exactly
two dependency bindings: default -> root and ship override -> ship subtree.

| Change | Affected ranges | Affected rows | Dirty spans | Rebuilt spans | Spans examined | Member rows scanned |
|---|---:|---:|---:|---:|---:|---:|
| default source | 1 | 4 | 3 | 3 | 3 | 0 |
| ship override source | 1 | 2 | 1 | 1 | 1 | 0 |

For the ship-only refresh, two unaffected profile segments are checked and zero
profile identities change; commodity remains bit-exact at `1.0`. Ship and its
freighter descendant change together from `0.5` to `2.0` at generation 8.

The witness then passes those effective weights into the existing constrained
claim and clearing path. With supply 3, the generation-7 clear is commodity 3 /
ship 0. After the authored ship operand emits its existing locus and refreshes
at generation 8, the clear is commodity 0 / ship 3. Repeating the identical
generation-8 inputs yields an identical result.

## Performance lease

Exactly one row was appended for
`crates/simthing-kernel/src/clearing_weight_projection.rs` with
`consumer=PERFORMANCE-TRACK`, debt `clearing-weight sparse-K build complexity`,
and `owed-measurement:2026-08-30`. No optimization or benchmark was performed.

- `track_closeout.sh --artifact-expiry`: PASS, `expired=0 cruft=0 malformed=0`;
- `track_closeout.sh --decommission --dry-run`: DRY, `reaped=0 files=0 manual=0`.

The new lease is non-reap cargo.

## Verification

| Command / proof | Result |
|---|---|
| Focused lifecycle witness | PASS — 1 passed, 0 failed |
| 13.6 matrix + germ | PASS — 2 + 4 passed, 0 failed |
| Generic derived-span tests | PASS — 4 passed, 0 failed |
| Constrained-clearing execution seal | PASS — 1 passed, 0 failed |
| `cargo check -p simthing-spec` | PASS |
| Artifact expiry / decommission dry-run | PASS / DRY, zero reap |
| `cargo test --workspace --all-targets --no-fail-fast -j 1 --quiet` | `STRUCTURAL-CERTIFICATE-SUMMARY suites=127 passed=479 failed=0 ignored=14 exit=0` |

Exact-head inventory, agent/doctrine/orientation/handoff/anchor/diff checks,
hosted Doctrine Scan/Exec, fresh `/clearance`, and `/relay-lint` are recorded in
the PR/Board relay after publication of the final head.

## Scope and routing

No constrained-clearing arithmetic, eligibility, epsilon, tie/remainder,
DecimalField, generation, or replay implementation changed. There is no
kernel-to-spec dependency, new Cargo dependency, production deformation caller,
cache, scheduler, GPU clearing path, sparse-K optimization, gate-code edit,
pointer movement, closeout action, next-track work, or engineering-review
ceremony.

Return posture is **PROBATION / proof-present / DA-review-pending** under
`DA-RESERVE(binding)`. Coding does not merge or self-graduate. The post-review
owner hold remains binding.

Owner hold: `5466933355`.
