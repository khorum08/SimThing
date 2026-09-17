> HISTORICAL: the first-birth failure below is repaired by #2067 at 6dbab071be7afd3967c63663badcb9067e94ac22. All ten unchanged Leaf-A tests, including first birth and G4/G5 continuation, passed on the synchronized branch. Current funded-cancellation findings are in [release results](rehearsal_lifecycle_construction_release_results.md). The original packet below is preserved.

# Construction Leaf A — funded birth invalidates its own session

**PROBATION / proof-present / BLOCKED / OPEN / UNMERGED — ORCHESTRATION ONLY.**
Authority: Board [5697992261](https://github.com/khorum08/SimThing/issues/1332#issuecomment-5697992261),
PR followup [5697996664](https://github.com/khorum08/SimThing/pull/2062#issuecomment-5697996664),
and DA repair ruling 5697918471. Same draft #2062 / `codex/0088-construction-leaf-a`,
synchronized by merge onto `7720499d3c9c7a5e777b647952e27567eaf76639` (#2066).
All historical RED packets remain in history and prior evidence documents.

**STOP — the first funded structural boundary commits its placement and subtree,
then fails `ActionBandIngress(BindingTableStale)`.** The ordinary session cannot
finish that generation or execute the next completion. This is a production
composition/precondition defect before the full Model-1 decision, not a semantic
falsification of settled WIP. **Model 1 UNDECIDED; Model 2 CLOSED.**

## Executed evidence

The required sequence ran on the repaired branch before new tests: canonical
qualified ingress (1/0/0), byte-unchanged independent allocation (2/0/0), WIP and
exactly-once integration (3/0/0), then the unchanged recipe file (3/0/0). All
**9 prerequisite tests passed**, including all eight joint-host admission cases.
The original three recipe tests and all earlier prerequisite test files retain
their semantics and source bytes. The new table-driven lifecycle test is appended to the recipe file.

Reference tuple: NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79,
rustc 1.95.0, LLVM 22.1.2, `simthing-gpu/eml-resource-profiling`.
Canonical ingress still reports required/observed `f65dd84eae5040c6`, bundle
`70b386fcb9bcd2eb`; #2066 requires no E8 change.

### Both projects execute and conserve material

A/B are canonical RF Balance properties on World→P/Q with opposing weights
A=(3,1), B=(1,3). Both explicit-host recipes consume 1A+1B from their own host.
P/Q authored bands remain 0/1; reversing the recipe vector does not reverse the
admitted phase policy. The new joint test executes three supply pulses × two
physical orders × two arena orders × two recipe-vector orders × five ordinary
generations = **120 generations**. Boundary overlays stop supply after G1.

| Supply pulse | G1 P; Q as (A,B,recipe output) | G2–G5 |
|---|---|---|
| 1A+1B | (.75,.25,0); (.25,.75,0) | Unchanged, no incomplete-input credit |
| 1A+0B | (.75,0,0); (.25,0,0) | Unchanged, no starvation credit |
| 4A+4B | (3,1,0); (1,3,0) | (2,0,1); (0,2,1), no repeated spend |

Every generation asserts exact host states AND remaining stock + consumed recipe
units = delivered material, independently for both resources. Scalar output is
not claimed to be a structural product.

### Scoped cancellation / recovery / restart control

Authored early-cancellation disposition: retain the entire partially funded WIP
host, with its identity, stock and existing residency, in a recovery depot via
ordinary `Reparent`. No product placement has yet been acquired. Supply is stopped
while cancelled. G2 moves P (or Q) to the depot; G3 holds; G4 reparents it back and
authors restart supply. The two original slots and all partial balances remain
unchanged throughout G1–G4. Both P and Q are cancelled independently, in original
and simultaneously reversed physical/arena/recipe orders: **four eight-generation
sessions**. No tree reinstall, stock patch, CPU refund, or allocator is used.

Restart explicitly authors flow=1 on the root policy subtree: root AND P/Q each
produce one unit per resource. This is **3A+3B new supply per generation**, not a
restoration or refund of stock. RF adds (1.75,1.25) to P and (1.25,1.75) to Q after
recipe execution. Exact G5–G8 P states are (2.5,1.5,0), (3.25,1.75,1), (4,2,2),
(3.75,1.25,4); Q swaps A/B. Every generation asserts delivered material
`1 + 3*(generation-4)` equals remaining stock plus consumed units. The recipe's
throttle hint is metadata, not an enforced cap: two units consume at G8.

This proves partial-WIP retention and restart for this declared storage policy.
It does NOT prove release of an acquired product-placement commitment, cancellation
of funded work, or recovery after placement refusal. Those remain full-matrix work.

### First funded birth: required-success RED

A canonical threshold observes P's integral recipe output. The ordinary Phase-5
crossing is strict `>`; threshold .5 detects the first whole completed recipe unit.
A tick-zero admitted ActionBand structural consequence selects one pre-admitted
`AddChild` candidate under P: a new node, one child component, and a property on
the candidate. The CPU never inspects stock to decide whether to submit the birth.
Ordinary GPU recipe output → sealed crossing → admitted consequence → feeder →
ordinary boundary is the only decision/application path.

This is a first-birth prerequisite, NOT proof of a reusable fresh-instance factory.
The candidate is authored at admission; no template counter is substituted for the
remaining two-completion structural obligation.

| Point | Observed ordinary behavior |
|---|---|
| G1, 4A+4B | P(3,1,0), Q(1,3,0); no subtree, no placement, ActionBand generation 0 |
| G2 | Both consume 1A+1B, P(2,0,1), Q(0,2,1); ActionBand generation 1; birth request queued; no placement yet |
| G3 | Placement commits, candidate AND component attach; fixed dimensions/capacity unchanged; then `Err(ActionBandIngress(BindingTableStale))` |
| Three retries | `ExecutionIdentity("generation GenerationStamp(3) is faulted after an unfinished economic authorization")`; GPU values, tick/day, schedule length and binding table unchanged |

The funded case reproduces in both original and reversed child/arena/recipe orders.
The two insufficient-input controls run five generations each with no birth,
placement or ActionBand dispatch, retaining the original partial stock exactly.
An external fixed `AddChild` control with the same joint recipe world and same
candidate shape, but no installed ActionBand, completes G3 AND G4 normally.
That control isolates the frozen ActionBand check; it is not offered as an
alternative construction mechanism.

The required-success assertion stays **RED** (no ignore/expected-failure wrapper).
The four continuation cases run in one table-driven test: joint accounting,
cancellation/restart and the external AddChild control pass; funded birth remains
RED. The full recipe file has four tests: three GREEN, one RED. Together with the
six preceding ingress/allocation/WIP tests, the focused total is 9 GREEN / 1 RED.
An initial seven-test grouping caused TEST-BUDGET INSPECT; consolidation preserves
every assertion and prior test, without a scanner exception or file split.

## Exact seam and routing

`crates/simthing-driver/src/session.rs` snapshots the whole allocator binding table
in `current_action_band_ingress_shape` at installation. `ensure_action_band_ingress_current`
compares that whole table to the frozen one and returns `BindingTableStale` on ANY
change. The ordinary boundary applies the authorized AddChild first, then
`dispatch_action_band_boundary` calls this check, even when no new crossing needs
dispatch. The sanctioned birth itself changes the table. The check predates this
leaf (commit `6bbb98958bf18cd83f8e08cb40f9ad5a4bedec97`, ordinary ActionBand ingress).
The generation has already been economically touched, so its refusal correctly
poisons the generation. No rollback is invented; attached objects and consumed
material remain as born state of the faulted generation.

Route the bound-session structural lifecycle/identity admission seam for separate
adjudication. Preserve sealed provenance, stale/foreign binding refusals, single
execution ownership and fail-stop law. A repair must define a lawful relationship
between an admitted structural consequence and the affected bindings; simply
turning off the stale check, late reinstalling ActionBand, or updating its snapshot
from the test would evade the contract. Any sealed-source change requires its own
standing E8 accounting. This leaf changes no production source or seal.

After that repair, rerun the SAME required-success first-birth witness, then finish
all remaining obligations. No direct DA relay or merge by this coding agent.

## Full §5.3 accounting

| Obligation | Current evidence / limit |
|---|---|
| Qualified ingress, independent opposite A/B allocation, WIP, exactly-once | GREEN unchanged prerequisites |
| Joint contention, incomplete-input credit, complete owned-input consumption | GREEN 120-generation joint matrix; scalar output only |
| Cancel P and Q, explicit WIP disposition, restart without duplicate spend/loss | GREEN scoped early-cancellation recovery-host policy, four sessions; retained stock residency |
| Capacity/residency release and recovery after acquired commitment | NOT PROVED; recovery control intentionally retains the live stock host |
| Residency/material ordering | Executed G2 material consumption, G3 placement+attach then fault; successful composed lifecycle NOT PROVED |
| Placement unavailable/refusal fabricates nothing and loses no reservation/material | NOT PROVED in funded construction |
| Touched-generation failure is fail-stop | GREEN at the actual funded-birth failure; three immutable retries per order, no rollback |
| At least two repeated funded fresh structural completions, properties/overlays/membership/observation/child bindings | NOT PROVED; first candidate and component attach but boundary fails; remaining binding/full factory obligations not inferred |
| Physical/recipe order determinism | Joint economic matrix independently permuted; cancellation/birth use original and combined reversal; full structural lifecycle NOT PROVED |
| Typed negative provenance | Existing unknown/ambiguous hosts retain span Some(37); birth has typed stale-binding error and faulted-generation refusal |

## Reproduction and validation

```powershell
$env:CARGO_TARGET_DIR='C:/Users/mvorm/SimThing/target'
cargo check -p simthing-driver --features simthing-gpu/eml-resource-profiling
bash scripts/ci/agent_scan.sh --base 7720499d3c9c7a5e777b647952e27567eaf76639
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_ingress -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_discriminator -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_wip -- --nocapture --test-threads=1
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rehearsal_lifecycle_construction_recipe -- --nocapture --test-threads=1
```

[Raw packet](rehearsal_lifecycle_construction_birth_raw_results.md). Exact final
head, local/hosted reports and fresh post-final-body clearance are recorded on
PR #2062 and the Board return. `coverage_basis: FAIL`, `ci_green: NO` preserve the
real required-success failure even when hosted Doctrine passes.

This continuation appends one table-driven test with four cases, adds this evidence pair, marks old recipe
admission evidence historical, and updates the test inventory. Relative to repaired
master the leaf remains rehearsal tests / evidence docs / inventory only. No
production, Cargo, scenario, UI, kernel/WGSL, sealed-source, gate or router edits.

ORIENT-RECEIPT: 28f56884d309; role coding; rule stamp 73e54d6b56b7266b, carried.
Governance unchanged by #2066. Construction anchor re-resolved unchanged:
`rehearsal-0088-construction-contract@bdd51c7c4ae2`.
Direct Board authority; no HD receipt invented.
