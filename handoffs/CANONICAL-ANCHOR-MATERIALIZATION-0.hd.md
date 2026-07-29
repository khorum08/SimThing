---
rung: CANONICAL-ANCHOR-MATERIALIZATION-0
kind: rung
track: 0.0.8.7
base_sha: 1c2b6613172ef19adc5282bc85d7fa2857ebe3fd
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(novelty)
owner_notes: "Route-3 successor authorized by DA rulings 5121471257 and 5122416859. Std Grok lane, but DA must review and issue this derivation law before dispatch. Build 5.3b only; 5.3 table semantics and 5.4+ remain fenced. DA-amended at issuance: census-before-implementation, observation-only materialization, determinism-vs-goldens treatment."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-sim/src", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["new hosting DSL, authored host field, or ClauseScript/ScenarioSpec syntax", "kind, property-name, display-name, or default-root host inference", "changes to the 5.3 table schema, WGSL, writers, remaps, or typed readback", "second host/observation authority or raw-value consumer path", "Field Sweep 5.4-5.8, Phase 6 transport, wire/replay persistence, or new raw index doors"]
required_checks: ["cargo build --workspace; core and sim batteries", "adapter-pinned full driver battery", "canonical materialization referees plus unmodified 5.3 and Studio referees", "RF-1 and replay determinism", "agent/doctrine scans; bypass census; inventory/orientation/doc-budget/anchors/clearance"]
stop_conditions: ["stale-orient-receipt or scope-widening", "canonical hosts require new authoring or scenario mutation", "a missing Anchored property has zero or conflicting lawful candidates", "derivation requires kind/name/root fallback or a 5.3 semantic change"]
---
## BUILD
- Census RULED (#1495). RESIDENCY = value-PLACING relations ONLY (economy
  emission/transfer/recipe host_entity, need_binding locus, threshold host,
  hosted_observation location, RF edge); governance instruments (owner-policy
  overlays, policy-weight authority) corroborate but NEVER elect. Re-run it.
- Add one typed install-time pass after existing admitted structures resolve and before final admission reporting / initial GPU sync. Existing loci remain authoritative; only Anchored properties with zero live loci enter derivation.
- Derive candidates on canonical property identity from EXISTING admitted RF parent edges, owner policy-weight authority, and install-resolved threshold, need/economy, overlay, or hosted-observation registrations. Never consult kind, names, display metadata, or a default root.
- Materialize `PropertyValue::from_layout` only when evidence converges on exactly one existing SimThing. Zero, conflicting, or out-of-tree candidates hard-error with identity and provenance. Preserve all existing values and loci exactly.
- Attach to ordinary `compile_and_install`, so preview, atomic install, open-from-spec, and Studio share the same tree. The existing `snapshot_anchored_loci` → typed GPU-table door consumes the result without special upload or enrollment.
- The 7 zero-candidate properties (`tp::hull`, `tp::weapon_damage`, `tp::upkeep`, 4x `tp::combat_*`) become AUTHORED `Unobserved{reason}` in `scenarios/terran_pirate_galaxy.clause` (the 5.1 door: declare the absence of a host, never fabricate one; reasons name the uninstantiated entity class and Phase 8 as successor) — DA-authorized, and the ONLY authoring permitted. Then replace the canonical-zero tripwire with the unmodified-topology full TP install: prove `18 Anchored / 7 Unobserved`, 18 live loci over 18 properties, 18 GPU rows, 7 dark cells; regenerate `property_admission_inventory.tsv` (25/0 -> 18/7); keep 5.3 hosted/Studio proofs unedited; add only missing/conflicting-evidence, value-preservation, and Unobserved negatives.
- Land one results doc and stamp 5.3b PROBATION while keeping the active pointer on 5.3b. DA alone advances to `FIELD-SWEEP-IR-PROBE-0` at graduation.
## FENCES
- Derivation-by-admission only; existing admitted structures are the complete vocabulary.
- Change tree property presence only: no disposition, value, topology, slot, RF, threshold, table, consumer, or wire/replay semantic change.
- Materialized loci are observation hosts only: layout-default values; no RF
  participation, accumulator-plan, or topology change; RF-1 sums identical.
- Determinism required: same install -> same loci in canonical
  SimPropertyId order. Byte-identity with pre-5.3b goldens is NOT expected
  where property presence legitimately changes — enumerate and justify each
  re-derived golden in the results doc; never silently rebase.
- No second host registry or observation authority; provenance is diagnostic only.
## EXIT-PROOF
- Unmodified canonical TP derives exactly one host per 25 Anchored properties, zero dark loci, and 25 rows in the existing GPU table, with no new authoring or fixture mutation.
- Missing/ambiguous evidence fails closed; existing loci/values and all unmodified 5.3 proofs remain exact.
- Required batteries and gates are green. PR stays PROBATION for DA graduation; no 5.4 dispatch.
