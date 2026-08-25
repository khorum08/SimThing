# STEMTHING-B-VRAM-RESIDENCY-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2b
- Status: **PROBATION / proof-present / DA-review-pending**
- Implementation base: `78e1cfe5be507e60c1a523cd24c50f9164b8347d`
- Branch: `codex/stemthing-b-vram-residency-0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- HD-RECEIPT: `0942e8fc2761`
- Handoff: Board comment `5406500478`
- Expected route: `DA-RESERVE(gate-wiring)`
- Pointer movement: none
- Structural certificate: owed at graduation

## Pre-edit authority map

| Required link | Exact landed attachment |
|---|---|
| cleared entitlement | graduated 11.2a `MarketGrantRecord`, minted only from `ConstrainedGrant` |
| owning residency boundary | kernel-owned `SlotAllocator` and its admitted direct-parent relation table |
| physical extent/oracle | authorized A1 addition `ResidencyExtent` plus private, stateless level-local `ResidencyPlacementOracle` |
| canonical record | existing 6.1 `IntegrationSchedule`, extended only by typed row kinds |
| relocation identity/history | existing `SlotAllocator::epoch_rebind`, `AnchorRemapSection`, and `resolve_slot_through_chain` |
| ordinary session generation | existing `DispatchCoordinator::day_index`; no second clock or generation authority |

The map found exactly one missing node: physical extent adjudication inside the
owning kernel boundary. No second clearing, allocator, schedule, remap chain,
history, telemetry, retry, or convergence surface was needed.

## Signal matrix

| Signal | Observed result |
|---|---|
| two-stage handoff | driver validates admitted offering/resource provenance and converts the graduated grant to a private-field provisional entitlement; only the kernel boundary commits geometry |
| recursive locality | root checks its direct granter child against `[0,16)`; that child's containing extent is its committed `[0,8)` placement and only its own direct children are compared |
| ordinary infeasibility RED | proposed sibling `[2,4)` overlaps committed `[0,3)`; typed `Overlap` refusal, no geometry commit, retained U=`2`, canonical refusal row, revalue generation=`5`, no crash |
| later revaluation | the same cleared grant is explicitly reconsidered at generation 5 and commits at `[3,5)`; generation 4 performs no retry, convergence, or re-clear |
| unchanged placement | identical grant/quantity/extent returns `Unchanged` and appends no schedule row or global re-proof |
| relocation | worker moves from `[0,3)` to `[5,8)` only after the existing epoch-rebind accepts the full binding-table assignment; stable placement identity follows the canonical remap chain |
| committed corruption RED | planted overlap and planted out-of-bounds committed state each record `ResidencyCommittedCorruption` before a typed, extent-rich `SessionTerminated`; the placement boundary remains unusable afterward |
| ordinary session wiring | `SimSession` owns the one integration schedule, uses its existing day generation for market placement, and refuses hot-path execution after recorded placement termination |

## Standing falsifiers

| Test | Biting failure |
|---|---|
| `cleared_entitlement_places_locally_refuses_to_u_then_revalues_and_relocates` | clearing/placement collapse, global extent scan, crash-on-infeasibility, lost U, same-generation retry, schedule side log, unchanged re-proof, or second relocation history |
| `committed_residency_corruption_records_then_hard_faults_for_exact_reason` | overlap/out-of-bounds corruption downgraded to U/warning/panic, missing granter/extents, fault recorded after shutdown, or reusable terminal boundary |

Focused commands:

```text
cargo test -p simthing-driver --test stemthing_b_vram_residency_0
cargo test -p simthing-kernel committed_residency_corruption_records_then_hard_faults_for_exact_reason
```

Local result: both standing witnesses pass. Exact tested commit and workflow run
IDs are carried in the PR and Board return after the evidence commit.

## Fences retained

- No 11.2c allocator-policy retirement, 11.2d facade work, later convergence,
  or pointer movement.
- Existing allocator free-list machinery is untouched and remains downstream;
  no ordering policy is attributed to it by this rung.
- Physical infeasibility does not re-clear, retry, converge, or terminate the
  session. Committed corruption is never represented as U or a recoverable warning.
- No second history, telemetry plane, replay recorder, generation authority,
  placement manager, or global extent scan.

