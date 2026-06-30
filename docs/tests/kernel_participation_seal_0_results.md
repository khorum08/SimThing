# KERNEL-PARTICIPATION-SEAL-0 Results

## Status

**PROBATION** — spatial arena participation requires a sealed `PlacedParticipant` proof minted by structural/boundary validation; external crates cannot struct-literal or named-constructor forge the proof. DA re-review required before DONE.

## PR / branch / merge

- Branch: `codex/kernel-participation-seal-0`
- PR: (pending)
- Merge: (pending)

## What changed

- Added sealed `PlacedParticipant` in `simthing-core` with private fields, accessors, and `pub(crate)` minter `from_validated_spatial_binding`.
- Added structural binding validators: `validate_location_ids_have_structural_placements` and `validate_and_mint_placed_participants_by_location_id` (validates against real grid metadata, then mints proofs).
- Added spatial arena enrollment surface in `simthing-spec`: `ExplicitParticipantSpec::spatial`, `spatial_nested`, and `spatial_arena_explicit_participants` — all require `PlacedParticipant`.
- Non-spatial RF channel identity unchanged: `ExplicitParticipantSpec::flat` / `nested` remain the non-spatial path.
- MapGen suppression arena (spatially-bound gridcell `Location`s) enrolls via `mint_spatial_arena_participants` → validated proofs → `spatial_arena_explicit_participants`. Deposit arena (non-gridcell feedstock) still uses `flat()`.

## Sealed participation surfaces

| Type / API | Public fields | External struct literal | Sanctioned minters / enrollment |
|---|---|---|---|
| `PlacedParticipant` | None (accessors only) | `compile_fail` | `from_validated_spatial_binding` (`pub(crate)`); public `validate_and_mint_placed_participants_by_location_id` validates real structural grid table |
| `ExplicitParticipantSpec::spatial` | N/A | N/A | Requires `PlacedParticipant` argument |
| `spatial_arena_explicit_participants` | N/A | N/A | Requires `&[(u32, PlacedParticipant)]` |
| `ExplicitParticipantSpec::flat` | Public serde fields | N/A (non-spatial channel identity) | Preserved for non-spatial RF |

## Constructor / minter audit

| Type / path | Struct literal? | Public named constructor with forgeable input? | Validates real state? | Verdict |
|---|---|---|---|---|
| `PlacedParticipant` | Forbidden (`compile_fail`) | No public `new` / `from_coord` / `from_raw` | Mint only via validator or `pub(crate)` minter after validation | OK |
| `from_validated_spatial_binding` | N/A | `pub(crate)` only | Caller must have validated | OK — not external |
| `validate_and_mint_placed_participants_by_location_id` | N/A | Public | Yes — checks structural binding table | OK |
| `validate_location_ids_have_structural_placements` | N/A | Public | Yes — location-id presence in table | OK |
| `ExplicitParticipantSpec::spatial` | N/A | Takes sealed `PlacedParticipant` only | Indirect — proof required | OK |
| `ExplicitParticipantSpec::flat` | Public | `slot + subtree_root_id` only | Non-spatial channel; documented not for spatial gridcell arenas | OK — non-spatial path |

## Sanctioned channels preserved

**Spatial:** structural tree / `grid_metadata` placements → `validate_and_mint_placed_participants_by_location_id` → `PlacedParticipant` → `ExplicitParticipantSpec::spatial` / `spatial_arena_explicit_participants` → RF arena enrollment → accumulator projection.

**Non-spatial RF:** `ExplicitParticipantSpec::flat(slot, subtree_root_id)` channel identity path unchanged (driver E-10/E-11 tests, deposit arena, cohort arenas).

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `PlacedParticipant` struct-literal `compile_fail` (`placed_participant.rs`) | External forge via private fields |
| `PlacedParticipant::from_validated_spatial_binding` named `compile_fail` | External raw minter reach |
| `ExplicitParticipantSpec::from_spatial_channel_identity` `compile_fail` (`resource_flow.rs`) | Spatial enrollment without sealed proof (forbidden API) |
| `cargo test -p simthing-core placed_participant` | Validator mint + reject missing/duplicate placements |
| `cargo test -p simthing-clausething --test mapgen_rf_stead_binding` | STEAD spatial binding behavior preserved |
| `cargo test -p simthing-clausething --test mapgen_resource_flow` | MapGen RF enrollment parity |
| `cargo test -p simthing-sim as_sim_semantic_free_public_surface_audit --lib` | No semantic surface regression |

## Value parity

No resolved value, event, or participant ordering change intended. Spatial suppression arena still enrolls the same gridcell participants with the same slot/subtree_root_id pairs; only the admission path now carries a compile-time placement proof. MapGen RF/STEAD binding tests green.

## Performance parity

Zero-cost by construction: `PlacedParticipant` is a transparent newtype record (private fields, no heap, no runtime branch on hot path). Enrollment-time validation runs at MapGen compile/admission (unchanged locus vs pre-seal `validate_spatial_binding`). No new hot-path runtime check in accumulator tick.

## Scope Ledger

| File / area | Why touched |
|---|---|
| `crates/simthing-core/src/placed_participant.rs` | Sealed proof type + validators + `compile_fail` |
| `crates/simthing-core/src/lib.rs` | Export |
| `crates/simthing-spec/src/spec/resource_flow.rs` | `spatial()` enrollment + `compile_fail` |
| `crates/simthing-spec/src/lib.rs` | Re-export `spatial_arena_explicit_participants` |
| `crates/simthing-clausething/src/mapgen_resource_flow.rs` | Wire suppression arena to proof mint path |
| `docs/tests/kernel_participation_seal_0_results.md` | This ledger |
| `docs/tests/current_evidence_index.md` | Index row |
| `docs/design_0_0_8_4_5_simthing_kernel.md` | Rung 5 OPEN → PROBATION |

**Not touched:** write/emission seals, `simthing-kernel` crate extraction, `deny.toml`, `design_0_0_8_5`, new hot-path runtime checks, broad test battery.

## Known gaps / next

- DA re-review: PROBATION → DONE.
- **`KERNEL-CRATE-EXTRACT-0`** — dependency-enforce seals in extracted kernel crate.
- Future spatial enrollment outside MapGen should route through the same `PlacedParticipant` mint path; additional call-site migration only when new spatial-bound arenas land.
