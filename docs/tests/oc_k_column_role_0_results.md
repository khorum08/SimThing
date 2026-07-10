# OC-K-COLUMN-ROLE-0 Results

## Status
**DONE / PROBATION** — door + lit path + scan retirement; DA deep audit/merge.

## Changed files
simthing-core registry/property; kernel/sim/spec/driver/feeder ColumnIndex call sites; scans.tsv (−AS5, −RAW-DATA-INDEX); selftest; core §3 pathway; design ladder; this doc.

## Owner valve
OC-KERNEL-LANE discharged (#1272). dmission-amendment-request: allowed.

## Forgeability
Pre-seal illegal forms: 	ype ColumnIndex = usize, .data[N]. Sealed: private PropertyValue.data, private-field ColumnIndex/RoleOffset, compile_fail docs; col_for_role → ColumnIndex only.

## Scan hits
| scan | pre | post | action |
|---|---|---|---|
| RAW-DATA-INDEX | 0 | deleted | retired |
| AS5-COLUMN-ALIAS | 0 | deleted | retired |

## Door
ColumnIndex newtype; SubFieldRole → offset_of → RoleOffset → role accessors; col_for_role → ColumnIndex. No bare .data[N] on sealed paths.

## Lit pathway
Core §3 role-vocabulary example under property-value-rf-overlays anchor payload.

## CPU-oracle parity
Bit-exact 32::to_bits(): role vs layout-lane Amount/Velocity; overlay role transform vs RoleOffset write; col_for_role global bits = start+lane.

## Scan deletion
Both rows removed from scans.tsv; selftest cases removed. known_bad fixtures retained as seal-proof history.

## seal-residue-risk
**B3 residual:** ColumnIndex::new/.raw() at GPU packing; 
aw_lanes serialization hatch — greppable. B1–B2,B4–B8 none.

## Falsification
No K1/K3/K4/closeout/0.0.8.6; no semantic words below spec; no scan-only substitute.

## Commands
See PR body for proof outputs; sticky Clearance after auto-post.
