# ACTIONBAND-EXECUTION-INGRESS-0 results

- Track: 0.0.8.7 RF arena modernization (remedial 11.1e)
- Status: **COMPLETE — DA-GRADUATED / merged #1822 @ `60654376`** (Fable deep review, graduation ruling on Board #1332)
- Branch: `codex/actionband-execution-ingress-0`
- Reconciled live-master base: `8c0c433ccb7c5119ad255412a6798dbc3be75442`
- Implementation / tested_code_sha: `6bbb98958bf18cd83f8e08cb40f9ad5a4bedec97`
- Evidence-only final head and hosted workflow IDs: bound in the PR body and Board return
- Board dispatch: `5400567405`; DA authority/A1: `5400536335`
- HD-RECEIPT: `8d6560239f55`
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- Orientation digest: `546353c5eec0d64647955ed4bdd8d4a287cef5dcfcfea8892621633b2d13f77c`
- Expected route: `DA-RESERVE(gate-wiring)`

## Archaeology and retained disposition

Before this rung, `compile_crossing_consequence_session` had one declaration
and zero true production callers. Its only downstream call to
`compile_action_band_gpu_execution_with_native_lanes` was inside that
declaration; every runtime consumer of the resulting
`CrossingConsequenceSession`/`CrossingConsequenceDispatch` lived in driver or
workshop tests. The Vendor Door merely aliased the compiler and could return a
detached product, while ordinary `SimSession` owned no ActionBand state.

The existing machinery was sufficient, so the advertised entry is retained.
The sole production caller is now
`simthing_embedder::bind::action_band_commitments`, which compiles and consumes
the product atomically into the supplied ordinary session. A standing census
separately proves one compiler declaration and exactly one true production
caller at that consuming facade; the declaration is never counted as execution
proof.

The lawful insertion seam is the ordinary boundary product already returned by
`BoundaryProtocol::execute_with_boundary_hook`:

`Phase-5 threshold event -> BandCrossingDelta -> immutable ActionBand plan join
-> existing CrossingConsequenceDispatch -> existing GPU facility boundary ->
existing feeder BoundaryRequest -> next ordinary tree boundary`.

No executor, dispatcher, comparator, listener, scheduler, clock, history, EML
opcode, target form, registry, or authored vocabulary was added.

## Lifetime, resident bind, staleness, and provenance

- Lifetime: a successful Vendor Door bind installs exactly one consuming
  `CrossingConsequenceDispatch` in `SimSession` for the session lifetime;
  duplicate installation is a typed `AlreadyInstalled` error.
- Bind point: installation is tick-zero only, after ordinary spec installation
  and GPU shape synchronization. The existing final `coord.shadow` seeds the
  facility-local resident plane; late installation fails closed.
- Staleness: installation snapshots the object binding table, registry width,
  property/activation shape, coordinator and GPU slot/dimension shape, and
  resident length. Every hot cycle and ActionBand boundary dispatch validates
  that snapshot. Binding, registry, or dimension drift is typed refusal; there
  is no silent recompile or rebind.
- Provenance: the existing consuming `bind_dispatch(self, ...)`, source-bound
  `ActionBandSessionOrigin`, sealed Phase-5 consumption keys, facility
  generation boundary, and generation dedupe remain authoritative. The new
  ingress can only join canonical sealed deltas and call the pre-existing sole
  dispatcher.
- History: both `run`/`step_once` and `record_to_path` use the same ordinary
  boundary insertion. Consequences remain feeder work applied no earlier than
  the next recorded structural boundary; no rival replay ledger exists.

## Production witness and falsifiers

`advertised_commitments_execute_and_stale_shape_refuses` opens a real ordinary
session, installs an ordinary velocity threshold, admits an embedder-authored
ActionBand and exact EML program through the advertised door, and runs three
ordinary generations on a real adapter. One canonical crossing advances the
existing ActionBand facility, emits one frozen structural authorization, and
the next ordinary boundary reparents the live child. Telemetry proves one
crossing batch, one crossing, one structural authorization, and facility
generation 1. Growing the registry afterward fails at the next hot-cycle door
with typed `RegistryStale`.

The planted dropped-product mutation deletes the ordinary
`dispatch_action_band_boundary` call while leaving compile/install intact. The
witness REDs for its own named reason,
`ACTIONBAND-EXECUTION-INGRESS-DROPPED-PRODUCT: the ordinary boundary must
consume the compiled ActionBand product`; the mutation was restored. The Vendor
Door source seal independently REDs a return-to-alias mutation and requires the
atomic install call. The caller census prevents the old declaration-only
tripwire wording from returning.

## Necessity Test and scope

No new mechanism was necessary. The change only exposes the existing immutable
sealed-crossing join on its owning GPU session, retains the already-graduated
dispatcher inside `SimSession`, and invokes it at the existing ordinary
boundary. The failed initial routed-overlay fixture was not evidence of a seam
gap: the feeder request reached the patcher, but its novel conditioned lifecycle
shape was correctly rejected by the frozen session-build lifecycle catalogue.
The final structural witness uses the already-admitted consequence vocabulary
without weakening that admission.

11.1f, #1803/11.2, 11.3+, 12.x, pointer state, peer-authority retirement,
`/clearance`, `/relay-lint`, merge, and graduation remain untouched. The
structural certificate is owed at graduation.

## Exact-code verification

All passing commands below ran with a clean worktree at tested code SHA
`6bbb98958bf18cd83f8e08cb40f9ad5a4bedec97`.

| Command | Result |
|---|---|
| `cargo check -p simthing-driver -p simthing-embedder` | PASS |
| `cargo test -p simthing-embedder -- --test-threads=1` | PASS — 1 ingress + 6 Vendor Door + 2 Triad surface + 2 compile-fail doctests |
| `cargo test -p simthing-driver --test actionband_overlay_actuation_0 -- --test-threads=1` | PASS — 2/2 |
| `cargo test -p simthing-driver --test actionband_gpu_execution_0 -- --test-threads=1` | PASS — 5/5 |
| `cargo test -p simthing-driver --test actionband_recursive_composition_0 -- --test-threads=1` | PASS — 5/5 |
| `cargo test -p simthing-sim --test gpu_overlay_lifecycle_oracle_parity_0 -- --test-threads=1` | PASS — 1/1 |
| `cargo test -p simthing-driver --lib -- --test-threads=1` | PASS — 16/16 |
| inventory check + drift prove + lifecycle schema | PASS — 1,314 rows / 1,314 discovered; zero expired |
| `agent_scan.sh` | PASS — zero hard failures and zero inspect flags at the implementation checkpoint; final-head scan recorded in the PR/Board return |

The complete rendered 51-anchor set was acknowledged before governed edits.
The reach ledger carries the session's projected queries; newly required
load-bearing receipts include
`admission-ladder-necessity-test@4bedf826f6f77b675a6080dc3289c17849fb825026d449472d44df324e2062dd`,
`eml-extension-ladder@7755bc72ffbe73411916f1d8aca76156ad45dcfd9570d345b1aec3adc26765a9`,
and
`seal-residue-cross-crate@49ee7c4ba6f40dd4c5254e895ba0c9b1ff12d44ab148f18ea82a7e9d83345f1c`.
The full ActionBand, overlay, RF, EML, field, slot-identity, orientation,
scanner, convergence, and workshop anchor family remains binding.
