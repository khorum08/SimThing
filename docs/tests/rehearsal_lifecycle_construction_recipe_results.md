# Construction Leaf A — explicit recipe-host admission blocker

**PROBATION / blocker-proof-present / BLOCKED / OPEN / UNMERGED.**
ORCHESTRATION ONLY. Authority: Board
[5691203267](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5691203267)
and followup [5691204610](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5691204610),
including DA settlement ruling 5690839905.
Same branch `codex/0088-construction-leaf-a`, draft PR #2062, synchronized by merge
onto exact master `ec078b3095b19218c2d50b0b9a7e607967f029c1` (#2065).
Historical RED packets and commits remain preserved.

**STOP: ordinary economy admission rejects the two explicitly hosted recipes.**
P and Q both own A/B Balance instances. Their recipe inputs and outputs explicitly
name their respective host. Lower-level compilation/materialization resolves
distinct rows correctly, but ordinary session install treats the second host for
a property ID as conflicting. This is an admission/precondition gap before the
combined lifecycle, not a Model-1 semantic falsifier. **Model 1 UNDECIDED;
Model 2 CLOSED.** No full lifecycle acceptance or graduation is claimed.

## Sequence and repaired prerequisites

| Required prerequisite | Executed result |
|---|---|
| Canonical ordinary ingress FIRST | 1 passed / 0 failed / 0 ignored; eleventh-roll required/observed `f65dd84eae5040c6`, bundle `70b386fcb9bcd2eb`. |
| Independent allocation, original test byte-for-byte unchanged | 2/0/0; single-resource controls, both joint arena orders, physical child reversal, ordinary sessions. |
| Canonical owned WIP settlement | GREEN across the existing component/ordinary stock matrix and parent-surplus controls. |
| Corrected free-rate diagnostic | Non-arena host rate=.5; G1 balance=.5, G2 balance=1.0. Exactly one integration per generation. |
| WIP test file total | 3/0/0. |
| New recipe/admission file | 2 passed / 1 failed / 0 ignored, exit 101. |

The only prior-test correction is the explicitly authorized seeded diagnostic.
The host is added AFTER default residency RF installation and owns a property
with Balance/governed_by, but no RF roles. The admitted execution plan is checked
to exclude its ID from EVERY arena. It then advances through two ordinary
`step_once` calls. The test name remains stable for inventory/history continuity;
the host is a containment leaf, not an arena leaf. No rate is seeded on a
settlement-owned resource participant. The exactly-once expectation is preserved.

Reference tuple: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79,
rustc 1.95.0, LLVM 22.1.2, `simthing-gpu/eml-resource-profiling` enabled.

## Executed recipe controls and refusal

The fixture retains World→P/Q and separate A/B RF properties with opposed
weights A=(3,1), B=(1,3). Recipe inputs use the actual canonical Balance cells,
cost 1A+1B, with scalar output coefficient 1. Explicit install-target names p/q
each resolve to ONE different live project. P's recipe uses band 0 and Q's band 1;
this authored ordering avoids claiming a same-band concurrency capability.
Scalar recipe output is never called a structural birth.

An authored pair of ordinary boundary overlays stops root A/B production after
G1. There is no CPU stock patch, readback-driven economic choice, spec reinstall,
or second stock ledger. The source pulse is 1A+1B, 1A+0B, or 4A+4B.

Each recipe tested ALONE in the same two-project world succeeds: two hosts ×
three pulse cases × two storage/arena-order shapes × five generations = **60
ordinary generations**. These positive controls have a recipe on only one host
per session and are explicitly not a substitute for the required joint case.

| Pulse / recipe | G1 stock at P; Q (A,B,output) | G2 through G5 |
|---|---|---|
| 1A+1B, P alone or Q alone | (.75,.25,0); (.25,.75,0) | Identical; partial WIP retained, zero irreversible credit. |
| 1A+0B, P alone or Q alone | (.75,0,0); (.25,0,0) | Identical; starvation creates no credit. |
| 4A+4B, P alone | (3,1,0); (1,3,0) | (2,0,1); (1,3,0), stable. |
| 4A+4B, Q alone | (3,1,0); (1,3,0) | (3,1,0); (0,2,1), stable. |

Exact per-host states and material totals (remaining A/B plus consumed units
represented by the recipe output) are asserted at EVERY generation. This proves
the scoped RF→WIP→recipe timing: RF settles G1, recipe consumes on G2, no further
credit through G5 after supply stops. It does not prove placement/creation timing.

For the JOINT fixture, both recipes compile and the ordinary session materializer
maps every input to the same slot as its respective target, with P/Q on distinct
slots. This is a lower-level binding control, not a substitute runtime. Eight
permutations (recipe vector order × physical child order × arena order) all
then refuse `SimSession::open_from_spec` before any joint generation:

```text
Install(NeedBindingInvalid {
  binding: "resource_economy",
  reason: "property leaf_a::a has duplicate/conflicting economy host placement; PropertyKey is not row authority",
  span_token: None
})
```

The final required-success assertion remains RED. Unknown p and ambiguous p
(two IDs under one name) are separately tested and still refuse with typed
`NeedBindingInvalid` and the authored span `Some(37)`. Those legitimate admission
checks must survive any repair; distinct explicit names are not ambiguity.

## Source restriction and scope boundary

`ensure_resource_economy_properties` in `crates/simthing-driver/src/install.rs`
collects all placements into `qualified_hosts: HashMap<PropertyId, HostId>`.
Its preliminary loop rejects any later use of that property on another host,
even when every input/target has an explicit, unique host name and both instances
already exist. The error comes before resource-economy compilation and actual
host-specific materialization in the ordinary install path.

`RecipeInputSpec::host_entity` is documented as the explicit entity host for
the consumed property INSTANCE. `CompiledResourceRecipeInput` retains it, and
`materialize_resource_economy_registry_for_session` resolves its slot. The
reproduction verifies this downstream distinction directly. Source archaeology
traces the global preliminary restriction to commit `995865812` (RF-5A remand);
this leaf neither deletes the deliberate refusal nor invents an exception.

Orchestration must route that admission restriction for adjudication/repair.
Start with `install.rs` host-placement collection and preserve the downstream
host identity, uniqueness, missing-host, and provenance contracts. Any additional
production surface requires its own evidence/scope; any sealed component touch
requires the standing E8 procedure. No new allocator or atomic Model-2 API is
warranted by a refusal before economic execution.

Omitting a project's recipe, inventing a distinct property schema per project,
or using the lower-level materializer to bypass ordinary ingress would change
the proof. Those are not adopted as a joint-lifecycle solution.

## Full §5.3 accounting

| Obligation | Evidence / remaining work |
|---|---|
| Ordinary qualified ingress; opposite A/B allocation | GREEN, unchanged discriminator. |
| Owned WIP / once-only governed integration | GREEN after #2065 and authorized diagnostic correction. |
| Incomplete inputs create no irreversible credit | GREEN single-recipe ordinary controls; joint application cannot admit. |
| Consume complete inputs once, retain surplus | GREEN single-recipe controls over five generations, exact material totals. |
| Cancel P and Q, stock/capacity/placement disposition, restart | NOT PROVED in the required joint lifecycle. |
| Residency/material timing; unavailable placement/refusal | Recipe timing above only; composed placement lifecycle NOT PROVED. |
| Touched fault stays fail-stop | NOT PROVED in the composed construction lifecycle; no rollback invented. |
| Two fresh funded structural creations and all bindings | NOT PROVED; scalar recipe output is not a birth. |
| Determinism / negatives | Prerequisite permutations, scarcity, missing/ambiguous host tested; full lifecycle remains unproved. |

STOP follows the remand's production/precondition-defect fence. Keep the same PR
OPEN/UNMERGED. After an admitted repair, rerun the unchanged joint-host success
assertion and all controls, then continue the remaining construction matrix.

## Reproduction / final validation

```powershell
$env:CARGO_TARGET_DIR='C:/Users/mvorm/SimThing/target'
cargo check -p simthing-driver --features simthing-gpu/eml-resource-profiling
bash scripts/ci/agent_scan.sh --base ec078b3095b19218c2d50b0b9a7e607967f029c1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_discriminator -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_wip -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe -- --nocapture --test-threads=1
```

[Raw packet](rehearsal_lifecycle_construction_recipe_raw_results.md). Exact final
head, local/hosted validation and fresh post-body clearance are in the PR/Board
return. `coverage_basis: FAIL`, `ci_green: NO`; no expected-failure camouflage.
Scope: rehearsal tests, evidence docs and inventory only; no production/Cargo/
scenario/UI/sealed/gate/anchor/router changes relative to repaired master.

ORIENT-RECEIPT: 28f56884d309, carried; governance unchanged. Construction contract
anchor re-resolved unchanged (`bdd51c7c4ae275ce76e645847539dc86701068cfe6cf23f23b59a55478413745`).
Direct Board authority; no HD receipt invented.
