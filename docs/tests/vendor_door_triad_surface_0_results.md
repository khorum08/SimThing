# VENDOR-DOOR-TRIAD-SURFACE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 11.1c)
- Status: **PROBATION — proof present / DA review pending**
- Handoff baseline: `9e81ef6aa12725d185372eed3ab8c7c8bffc9cb8`
- Reconciled execution base: `8cbdbd04cf1660c452cca0ea14a7b10d10a3009d`
- Implementation / tested_code_sha: `502653b0e30b200d4a436ecb4d2a46dfac8b7e7a`
- HD-RECEIPT: `622933c70c88`
- ORIENT-RECEIPT: `a5dc59920dd4`
- orientation_rule_stamp: `61818ff7d4adda84`
- orientation_digest: `8bb010feac86eaa7346b7ce75c97f415afd3c58bb3220783a301b1da57825aea`
- expected_route: `DA-RESERVE(gate-wiring)`
- Homing boundary: `engine-native`

## Product

The leaf `simthing-embedder` facade now reaches the graduated full-Triad
session seam through its five verbs. Derive exposes competing emitter classes
and authored EML gadget data; Bind exposes bounded authored columns, admitted
PALMA/Gu-Yang compilation, bands, and read-only stall observation; Run delegates
to `SimSession::open_from_spec_with_admitted_field_sweeps` with a deferred
compiler that receives the seam's live finalized registry width.

The pre-graduation caller workaround was removed: Bind no longer projects a
registry width, the witness does not rewrite authored `RegionFieldSpec`
dimensions, and Run does not precompile registrations. The one graduated
dimension-finalization seam now owns that operation.

The integration witness imports only `simthing_embedder` and its five verb
namespaces. Existing authored fixture shapes are re-exported through Populate,
Overlay, and Bind, so the proof has no direct engine-crate bypass.

## Delegation archaeology

| Door entry | Graduated production surface |
|---|---|
| Derive competing emitter declaration | `simthing_driver::ComparativeEmitterClass` |
| Derive authored EML data/compiler | `simthing_spec::{EmlGadgetStackSpec, EmlGadgetInstanceSpec, compile_eml_gadget_stack}` |
| Populate authored scenario/tree/RF shapes | existing `simthing_core`, `simthing_gpu`, and `simthing_spec` data types by direct re-export |
| Bind PALMA admission | `simthing_driver::{PalmaN4FieldSweepSpec, compile_palma_n4_field_sweep}` |
| Bind Gu-Yang admission | `simthing_driver::{GuYangN4FieldSweepSpec, compile_gu_yang_n4_field_sweeps}` |
| Bind projection bands/output shape | `simthing_driver::{ComparativeProjectionBands, GuYangStallOutputs}` |
| Bind authored Triad column | `simthing_core::ColumnIndex::try_from_admitted_authored` with the caller's bound |
| Bind read-only observation | admitted `SpecSessionState.comparative_projection.stall_outputs` plus the ordinary mapping's `readback_canonical_field` |
| Run initialization | `simthing_driver::SimSession::open_from_spec_with_admitted_field_sweeps` |

## Exit proofs

| Proof | Result |
|---|---|
| Five-verb production witness | PASS — authored EML reaches `compile_eml_gadget_stack`; competing emitter declarations match the admitted field plan; the seam supplies the finalized width to PALMA and Gu-Yang compilers; their registrations open through the ordinary seam and run one production tick. |
| Five-verb-only surface | PASS — the witness contains no direct `simthing_core`, `simthing_driver`, `simthing_gpu`, `simthing_sim`, or `simthing_spec` reference. |
| Dimension-finalization authority | PASS — authored field dimensions are unchanged, the compiler receives the live post-admission registry width, and no facade preview/prediction helper remains. |
| Read-only observation | PASS — every returned bit is compared with the existing mapping's canonical GPU readback at the admitted `GuYangStallOutputs` columns. No observation feeds a CPU decision. |
| Fabricated-observable mutant | EXPECTED RED — replacing the observed stall lane with `1.0` fails for `VENDOR-DOOR-TRIAD-FABRICATED-OBSERVABLE`. Mutation restored. |
| Raw-column mutant | EXPECTED RED — replacing bounded admission with `ColumnIndex::from_gpu_round_trip(raw)` fails for `VENDOR-DOOR-TRIAD-RAW-COLUMN-MINT`. Mutation restored. |
| Direct tuple construction | COMPILE-FAIL — the facade re-export preserves `ColumnIndex`'s private field (`E0423`). |
| Born-observable census | PASS — no public facade function named chokepoint, corridor, front, or dominance. |
| Facade state/arrow | PASS — no static, Mutex, Arc, OnceCell, RefCell, cache, or registry; no workspace crate depends on `simthing-embedder`. |
| Engine diff | PASS — zero engine-crate source edits. |

## ActionBand handoff to 11.1d

`action_band_commitments` remains a facade re-export of
`compile_crossing_consequence_session`. The production driver source contains
only the function definition and no caller. This rung neither wires nor
withdraws that entry; 11.1d owns the disposition.

## Local evidence

| Command | Result |
|---|---|
| `cargo check -p simthing-embedder` | PASS |
| `cargo test -p simthing-embedder --test vendor_door_triad_surface_0` with adapter match required | PASS — 2 integration tests; `NVIDIA GeForce RTX 4080 Laptop GPU` / Vulkan |
| `cargo test -p simthing-embedder` with adapter match required | PASS — 8 integration tests and 2 compile-fail doctests |
| fabricated-observable planted mutation | EXPECTED RED — `VENDOR-DOOR-TRIAD-FABRICATED-OBSERVABLE` on the live production readback path; mutation restored |
| raw-column planted mutation | EXPECTED RED — `VENDOR-DOOR-TRIAD-RAW-COLUMN-MINT` on the bounded authored door; mutation restored |
| inventory drift prove / lifecycle schema + scheduled | PASS — inventory has 1,309 rows; zero expired candidates; implementation bound to `502653b0` |
| detachability / DOC-BUDGET | PASS — zero production/proof upward coupling; prose within budget |
| generated orientation / sanctioned digest | PASS |
| Agent scan at `502653b0` | PASS — zero hard failures and zero inspect flags |

Required anchors acknowledged: `orientation-harness-core@8a365d1c0864` and
`scanner-selftest-delta-gate@34fb2662baae`.

## Scope disposition

Return **PROBATION / proof-present / DA-review-pending**. Coding does not invoke
clearance or relay lint, merge, move the pointer, begin the 11.2 guide, or start
successor work.
