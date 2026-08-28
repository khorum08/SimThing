# UNIFIED-FACILITY-CONVERGENCE-WITNESS-0 — witness results

- Status: **PROBATION / proof-present / DA-review-pending**
- Rung: 11.3 `UNIFIED-FACILITY-CONVERGENCE-WITNESS-0`
- Live base: `365be02b8c2c11065d939bff7a089bcaf0856a11`
- Tested code SHA: `932431c846bebd5c227034032677b02d73ea510d`
- HD-RECEIPT: `21c52cc09eef`
- Expected route: `DA-RESERVE(gate-wiring)`
- Coding does not merge, self-graduate, or move the 11.3 pointer.

## Fresh ingress

- `ORIENT-RECEIPT: 1a6a00162374`
- `orientation_rule_stamp: 9ee3f7649d1fc790`
- `orientation_digest_sha: 1f9cb1f861c343da37b93624191dcc958bdef5553517f4334ef2102512fd7926`
- `ANCHOR-ACK: orientation-harness-core@8a365d1c0864`
- `ANCHOR-ACK: scanner-selftest-delta-gate@34fb2662baae`

The handoff was rendered for the coding role on the stamped master. The predecessor proof is the
standing mechanized witness
`six_real_doors_publish_conserved_sparse_lanes_and_cross_actionband_without_rebind`; this rung uses
the production surfaces that witness proves and does not copy a test-side lane seed.

## Live production seam map

| leg | landed production seam | load-bearing proof in this rung |
|---|---|---|
| StemThing-A residency | `SimSession::open` → `SlotAllocator::install_initial_tree` → root residency extent and stable pre-open binding table | the load-bearing source, alternate passthrough source, granter, authoritative terminal, and bypass terminal all have real allocator slots before ActionBand admission; admitted capacity is measured before the run |
| residency → B publication eligibility | `grant_disbursement::plan_live_updates` requires both the logical node and `allocator.slot_of(node)` before it can read the resident lane row | the real cleared grant targets the pre-open resident source; the residency passthrough sends the same real grant to the alternate pre-open resident, never a flag or fabricated identity |
| StemThing-B grant | `clear_constrained_claims_at_generation` → opaque `ConstrainedGrant` → `SimSession::record_cleared_market_grant` → canonical `IntegrationSchedule` fact | all control, emergence, and falsifier grants pass the graduated clearing/recording doors; no manual `ConstrainedGrant` construction exists |
| B grant → ActionBand lane | `ScheduledGrantLifecycleFacts::from_schedule` → `publish_scheduled_facts` → protected pre-open overlay transition → ordinary hot lane write | exact use of the 11.2f standing seam; occupied becomes `3` only from the N+1 fact publication, with no host shadow seed or direct lane write |
| ActionBand crossing | ordinary threshold scan → sealed `BandCrossingDelta` → `SimSession::dispatch_action_band_boundary` → `dispatch_sealed_and_apply` | the load-bearing rising threshold is `2`; the emergence run records exactly one real crossing and one routed delivery |
| OverlayThing consequence | `RoutedOverlayDelivery::admit` → the existing `FeederSender` boundary channel → ordinary next-boundary `deliver_routed_overlay` | the instruction overlay is actually attached at N+3; construction alone cannot change the terminal |
| terminal + history | authoritative `SimRuntimeTree::overlay_count(authoritative-terminal)` and the same `ReplayDriver.root` reconstructed from `BoundaryDeltaEntry` frames | the same existing tree surface supplies emergence, all four RED profiles, and bit-exact replay; no checkpoint or second history participates |

No post-open participant growth, binding refresh, or ActionBand reinstall is used. The participant is
load-bearing before the frozen ActionBand binding, so `BindingTableStale`, `LateInstall`, and ingress-shape
protections remain unchanged and green.

## One composed emergence witness

The single authored datum is the requested and cleared grant quantity:

```text
control:   quantity = 1
perturbed: quantity = 3
```

The authored market, resident identity, granter, threshold `2`, ActionBand templates, routed consequence,
terminal identity, scenario horizon, and replay path are unchanged. Quantity `1` publishes a real accepted
grant but remains below the ActionBand crossing; quantity `3` traverses every facility and changes the
authoritative terminal profile.

| generation | production event | terminal overlay count |
|---|---|---:|
| N = 0 | real grant clears and is recorded in the canonical schedule | 0 |
| N+1 | the sole 11.2f publisher consumes the due fact and publishes occupied `3` | 0 |
| N+2 | ordinary hot scan emits one sealed crossing; the sole ActionBand dispatcher submits one routed consequence | 0 |
| N+3 | the following ordinary boundary executes the routed OverlayThing attachment | 1 |
| N+4 | no second crossing or consequence; terminal remains authoritative and stable | 1 |

Thus the first terminal change is `N+k` with **k = 3**. The control profile is
`[0, 0, 0, 0, 0]`; the perturbed profile is `[0, 0, 0, 1, 1]`. There is no
same-generation collapse or consequence re-entry.

## A1 — four otherwise-green passthrough REDs

Every RED executes the same four-boundary horizon, returns `Ok`, replays successfully, records exactly one
real ActionBand crossing, one routed delivery, and an executed overlay consequence at N+3 on the secondary
route. Only the authoritative terminal remains `[0, 0, 0, 0, 0]`.

| RED | real production neutralization | why the authoritative terminal stays static | other-facility liveness |
|---|---|---|---|
| residency | route the quantity-3 real grant to the alternate pre-open resident | the named load-bearing resident does not receive the published B lane | alternate resident publishes, crosses, and routes an overlay at N+3 |
| granting/disbursement | source receives a real quantity-1 accepted grant; alternate resident receives the quantity-3 liveness grant | source grant is real but cannot cross threshold `2` | two canonical grant facts; alternate lane crosses and executes the overlay |
| ActionBand crossing | source receives quantity `3`, but its real admitted threshold is `4`; alternate resident retains threshold `2` | source occupied lane moves to `3`, but no source crossing is sealed | two canonical grant facts; alternate crossing and overlay execute normally |
| OverlayThing consequence | source quantity `3` crosses threshold `2`, while its admitted routed consequence targets the bypass terminal | upstream resident, grant, and source crossing all execute, but the authoritative terminal is not the consequence receiver | one source crossing and one ordinary overlay attachment execute at the bypass target |

No RED faults, wedges, terminates early, stales the binding table, fails admission, or uses a helper refusal
as its causal reason.

## A2 / replay seal

The sole terminal observable is the existing canonical runtime-tree overlay state of the node authored as
`authoritative-terminal`, read through `SimRuntimeTree::overlay_count`. `ReplayDriver.root` is the same
surface after canonical delta-log reconstruction. All six live profiles (control, emergence, and four
passthroughs) equal their replay profiles bit-for-bit.

Every `ReplayFrame` carries `shadow_values: None`; `ReplayDriver.shadow_values` remains `None`. Replay
applies recorded grant facts and protected publications, recorded sealed band deltas, and the recorded
ordinary overlay attachment. It does not re-clear, re-submit a consequence, restore a shadow checkpoint,
or consult a second history/telemetry surface.

## Carry before compaction

The witness records these admitted cardinalities immediately after session and ActionBand admission and
before any run or summary collapse:

- allocator capacity: **6** (root + five real resident participants);
- registry columns: **7** (four conserved grant lanes + the existing simple-property three-column shape);
- frozen ActionBand templates / active instances / routed consequence rows: **2 / 2 / 2**;
- ActionBand facility generation at admission: **0**;
- market offerings / Draw envelopes: **1 / 1**;
- compaction steps: **0**.

## No-new-authority / no-new-semantics census

Executable delta is one integration witness only:
`crates/simthing-driver/tests/unified_facility_convergence_witness_0.rs`. Supporting deltas are the test
inventory row, current-evidence row, required anchor reach rows, and this results record. Production
`src`, engine/facade semantics, workflow/gate code, scenario runtime data, handoff, and orientation sources
are untouched.

The witness has no direct boundary overlay submission, host shadow seed/write, fabricated grant struct,
direct OverlayThing call, alternate executor, ActionBand rebind, generation writer, checkpoint restore,
or second replay/history surface. Typed construction makes the chain enter through the real clearing,
session schedule, sealed crossing, routed consequence, and replay doors.

## Local proof on tested code SHA

| command | result |
|---|---|
| `cargo check -p simthing-driver` | PASS (inherited warnings only) |
| `cargo test -q -p simthing-driver --test unified_facility_convergence_witness_0 -- --test-threads=1` | PASS — 1 passed |
| `cargo test -q -p simthing-driver --test grant_disbursement_lane_0 -- --test-threads=1` | PASS — 3 passed |
| `cargo test -q -p simthing-driver --test residency_tier_vocabulary_0 -- --test-threads=1` | PASS — 4 passed |
| `cargo test -q -p simthing-driver --test stemthing_b_vram_residency_0 -- --test-threads=1` | PASS — 1 passed |
| `cargo test -q -p simthing-driver --test actionband_overlay_actuation_0 -- --test-threads=1` | PASS — 2 passed |
| `cargo fmt --all -- --check` | PASS |
| `bash scripts/ci/agent_scan.sh` | PASS — hard failures 0, inspect 0 |
| `bash scripts/ci/test_inventory_check.sh` | PASS — 1,348 rows / 1,348 discovered; missing 0, extra 0 |
| `bash scripts/ci/test_inventory_drift_check.sh` | PASS — unledgered 0, parked 0, stale 0 |
| `bash scripts/ci/test_lifecycle_expiry_check.sh --schema` | PASS — expired 0, audit 0 |
| `bash scripts/ci/anchor_check.sh` | PASS — curation 62 rows; inherited coverage INSPECT only |
| `bash scripts/ci/doc_budget_check.sh --check` | PASS |

Hosted workflow IDs and the exact final evidence head are returned in the PR/Board relay so the documented
tested code SHA remains immutable.

## Return

**PROBATION / proof-present / DA-review-pending.** Structural certification and graduation remain DA-side.
11.4, 12.x, Vector CostBand, and the three known ClauseThing baseline REDs remain fenced.
