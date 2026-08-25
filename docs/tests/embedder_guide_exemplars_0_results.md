# EMBEDDER-GUIDE-EXEMPLARS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 11.2)
- Status: **COMPLETE — DA-GRADUATED / merged #1803 @ `9a883cc3`** (Fable deep review, graduation ruling on Board #1332)
- Resume dispatch: board/PR comment `5403241597`
- DA authorization: `5403211098`
- Canonical handoff: `handoffs/EMBEDDER-GUIDE-EXEMPLARS-0.hd.md`
- HD-RECEIPT: `99644421d2bd` (retired `890b845b5655` / `aff35cbd509a` never dispatched)
- ORIENT-RECEIPT: `a5dc59920dd4`
- Orientation rule stamp: `61818ff7d4adda84`
- ANCHOR-ACK: `orientation-harness-core@8a365d1c0864`
- ANCHOR-ACK: `scanner-selftest-delta-gate@34fb2662baae`
- Expected route: `DA-RESERVE(gate-wiring)`
- Coding branch: `coder/embedder-guide-exemplars-0` (resume draft #1803)
- Handoff `base_sha`: `c0f8a1de46245b4479ae08abea88e389be2fdd1e`
- Synchronized live master: `0bf9810d072d634f344b1fe4c663c6d9031fb426`
- tested_code_sha: `cf48009df1a4e36d395adc0c1192fc749141dc7f`
- Pointer: 11.2 only. No merge, no 11.3/11.4/12.x.

## Master sync

FIRST STEP merged `origin/master` into #1803. Stale branch-side Vendor Door
re-exports (`eml_exp_pinned_f32` / `eml_ln_pinned_f32` / `Direction` on
populate/bind, `TransformOp` on overlay) were surrendered to the graduated
master door. One onboarding re-export remains: `populate::SimPropertyId`,
required to name Overlay's property id from the frozen finance exemplar.

## Census (current five-verb door)

| Verb | Graduated surface used by 11.2 |
|---|---|
| Derive | `owner_seat`, `ComparativeEmitterClass`, `EmlGadgetInstanceSpec::PowerLaw`, `compile_eml_gadget_stack` |
| Populate | tree/RF authoring, `compile_property`, `RegionFieldSpec`, `SlotAllocator` |
| Overlay | `authored` finite-horizon overlay |
| Bind | `compile_palma_n4_field_sweep`, `compile_gu_yang_n4_field_sweeps`, `observe_gu_yang_stall`, `authored_column` |
| Run | `initialize`, `initialize_with_admitted_field_sweeps`, `start`, `tick`, `serialize` |

No direct engine imports in the two exemplars.

## Authored law

Volume-delay is admitted `PowerLaw { exponent: 4.0, input_floor: 0.25 }`.
An LN-unsafe floor (`0.0`) REDs at compile. CI detects `EmlGadgetInstanceSpec::PowerLaw`
plus `input_floor` after type-alias resolution. A staircase plant REDs
`FAIL(authored-law-staircase)`. `selftest_rename` aliases the type and
renames the gadget id; law verdict stays PASS. No `POW` opcode.

## Network full Triad

`network_saturation_triad_0` declares two competing load-class emitters,
admits PALMA + Gu-Yang through Bind, enters
`run::initialize_with_admitted_field_sweeps`, and observes born stall plus
contest/border/chokepoint columns. Generic thresholds are not used.

## Evidence

| Command / proof | Result |
|---|---|
| `cargo test -p simthing-embedder --test finance_toy_0 --test network_saturation_triad_0 --test vendor_door_0 --test vendor_door_triad_surface_0 -- --test-threads=1` | PASS 1 + 2 + 6 + 2 |
| `embedder_guide_exemplars_check.sh --check` | PASS |
| `embedder_guide_exemplars_check.sh --selftest` | live_shape / staircase / door_import / guide_path / rename PASS |
| DOC-BUDGET (`docs/embedders_guide.md` 115 / 120) | PASS |
| inventory / detachability / orientation / digest | PASS |
| `lifecycle_schema_pr_gate.sh` vs `0bf9810d` | PASS |
| `agent_scan.sh` at `cf48009d` vs `0bf9810d` | PASS `delta_inspect=0` |

## Scope disposition

Return **PROBATION / proof-present / DA-review-pending**. Coding does not invoke
`/clearance` or `/relay-lint`, merge, graduate, move the pointer, or begin 11.3.
