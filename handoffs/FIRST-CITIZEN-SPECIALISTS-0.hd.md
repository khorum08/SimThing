---
rung: FIRST-CITIZEN-SPECIALISTS-0
kind: rung
track: 0.0.8.7
base_sha: 2c5969731fd1841045962e35d734700d982a971b
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(unclassified-scope)
owner_notes: "Rung 3.2, closes Phase 3. Std lane (Grok, pin -m grok-4.5). Builds ONLY on the 3.1 protocol as landed — profiles stay data; new requirement KINDS remain DA-gated (none are authorized by this handoff); enrichment is authoring reach + guards + observability, not new machinery."
surfaces: ["crates/simthing-clausething/src/hydrate_scenario.rs", "crates/simthing-clausething/tests", "crates/simthing-core/src/specialization.rs", "crates/simthing-driver/tests", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["new SpecializationRequirement variants (DA-gated; NOT authorized here)", "runtime/tick profile consultation", "editing ANY existing test", "kernel/GPU/WGSL edits", "new ColumnIndex mints outside the 0.1 doors", "widening beyond the named hydrate_scenario.rs surface (3.1 admission ruling covers exactly it)"]
required_checks: ["cargo build --workspace", "full simthing-driver battery on live GPU", "doctrine-scan", "doctrine-selftest (scans.tsv changes)", "orientation-check", "doc-budget", "clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "an-enrichment-requires-a-new-requirement-kind (DA-route; do not improvise)", "guard-false-fires-on-legitimate-hydration-output"]
---
## BUILD
- Make the three citizens fully AUTHORABLE (ANCHOR-ACK core-0087): extend the `specialization = <profile>` clause key from owner blocks to location and entity blocks in `hydrate_scenario.rs` (same scalar-token span capture; same `DeclaredSpecialization{profile, span_token}` threading). Referee: an authored Location declaring `spatial` admits when placed and spans correctly when unplaced; an entity declaring `owner-seat` spans (Kind requirement unmet) with the exact token.
- **Out-of-band stamp guard (3.1 graduation fence):** the typed `OWNER_POLICY_WEIGHT_AUTHORITY` stamp may be written ONLY by hydration's field-economy derivation. Add a HEURISTIC scans.tsv tripwire flagging authored scenario sources (clause / scenario JSON fixtures) that mint property id `8_300_318` directly, + known_bad fixture + selftest; INSPECT-only, reach-log wired (0.3 pattern). Legitimate hydration output must stay quiet (delta-scoped, net-new only).
- **Citizen observability (Consumer Law):** surface per-profile conformance counts (spatial / owner-seat / session-root totals from `SpecSessionState.specialization`) through the existing board/orientation generation path (0.2 execution-status pattern — generator source, never hand-edited mirrors). Regression coverage for the render.
- Doctrine-CI co-evolution rides the PR; stamp 3.2 + advance posture row to `ROW-SLOT-OBJECT-SEMANTICS-0` (Phase 4) in-diff; regen orientation.
## FENCES
- Profiles-as-data only: zero new requirement kinds, zero runtime consultation, zero existing-test edits (full battery UNMODIFIED is the falsifier). The guard is INSPECT-only.
## EXIT-PROOF
- Authored-declaration referees green (location + entity, positive + spanned negatives); guard fixture fires + clean tree quiet + reach-log append proven; board/orientation render the citizen counts with regression coverage; workspace build + full driver battery green UNMODIFIED on live GPU. Stamp + posture advance in-diff per the ritual.
