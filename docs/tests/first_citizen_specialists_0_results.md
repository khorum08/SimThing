# FIRST-CITIZEN-SPECIALISTS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 3.2)
- Status: **PROBATION / proof-present / DA-review-pending**
- HD-RECEIPT: `11ddf57fcac0`
- ORIENT-RECEIPT: `16b366e49528`

## What landed

- Authored `specialization = <profile>` on location and entity (`child`) blocks in `hydrate_scenario.rs`, threaded as `DeclaredSpecialization{profile, span_token}` (same owner path).
- NEW referees in `first_citizen_specialists_0.rs`: Location+`spatial` admits when placed / spans `StructurallyPlaced` when unplaced; Fleet+`owner-seat` spans `Kind(Owner)` unmet with exact token.
- HEURISTIC `OWNER-POLICY-WEIGHT-AUTHORITY-MINT` scan over authored scenario sources minting `8_300_318`; known_bad + selftest + reach-log wire; clean tree quiet.
- Citizen observability: `SpecializationReport::citizen_counts()` → `scripts/ci/specialization_citizen_counts.tsv` → board/orientation generators (`spatial=1500 owner-seat=2 session-root=1`).
- Ladder stamp: 3.2 PROBATION; Active open rung → `ROW-SLOT-OBJECT-SEMANTICS-0`; orientation regenerated.

## Fences held

- Zero new `SpecializationRequirement` variants
- Zero runtime/tick profile consultation
- Zero edits to existing tests
- Zero kernel/GPU/WGSL edits
