# OC-K-COLUMN-ROLE-0 Results

## Status
**GRADUATED / PR #1273** — door + lit path + scan retirement; remedial B3 declaration + COLUMN-INDEX-MINT tripwire.

## Changed files
`simthing-core` registry/property; kernel/sim/spec/driver/feeder ColumnIndex call sites; `scans.tsv` (−AS5, −RAW-DATA-INDEX, +COLUMN-INDEX-MINT); selftest fixture; core §3 pathway; design ladder; this doc.

## Owner valve
`OC-KERNEL-LANE` discharged (#1272). `admission-amendment-request: allowed`.

## Forgeability
Pre-seal illegal forms: `type ColumnIndex = usize`, `.data[N]`. Sealed: private `PropertyValue.data`, private-field `ColumnIndex`/`RoleOffset`, compile_fail docs; `col_for_role` → `ColumnIndex` only.

## Scan ledger
| scan | action |
|---|---|
| RAW-DATA-INDEX | retired (−1) |
| AS5-COLUMN-ALIAS | retired (−1) |
| COLUMN-INDEX-MINT | added HEURISTIC (+1) |
| **Net** | **−1** (growth doctrine held) |

## Door
`ColumnIndex` newtype; `SubFieldRole` → `offset_of` → `RoleOffset` → role accessors; `col_for_role` → `ColumnIndex`. No bare `.data[N]` on sealed paths.

## Lit pathway
Core §3 role-vocabulary example under `property-value-rf-overlays`.

## CPU-oracle parity
Bit-exact `f32::to_bits()`: role vs layout-lane Amount/Velocity; overlay role transform; `col_for_role` = start+lane.

## seal_residue_risk
B3 residual = `ColumnIndex::new(usize)` remains fully public, with 153 sites / 33 files across driver runtime lattice, kernel accumulators, and spec admission, plus `.raw()`/`.raw_u32()` at packing boundaries. Residue is intentionally tracked by COLUMN-INDEX-MINT and promoted to OC-K2.1 backlog for admission-gated constructors; B1-B2 and B4-B8 none. **Not baseline-zero.**

## Falsification
No K1/K3/K4/closeout/0.0.8.6; no semantic words below spec; no scan-only substitute for the door.

## Commands
See PR remedial proof comment after amended head.
