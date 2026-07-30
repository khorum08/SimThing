---
rung: CANONICAL-ANCHOR-MATERIALIZATION-0
kind: rung
track: 0.0.8.7
base_sha: 1c2b6613172ef19adc5282bc85d7fa2857ebe3fd
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(novelty)
owner_notes: "Route-3 successor authorized by DA rulings 5121471257 and 5122416859. Std Grok lane, but DA must review and issue this derivation law before dispatch. Build 5.3b only; 5.3 table semantics and 5.4+ remain fenced. DA-amended at issuance: census-before-implementation, observation-only materialization, determinism-vs-goldens treatment. UPHOLDING THE EML/N4 EXPANSION (Owner directive): nothing in 5.3b narrows rungs 5.4-5.8 — the field-sweep preservation invariants (anchor `field-sweep-preservation`) stay binding in full, and this rung's totality-over-cardinality and no-carve-outs reasoning REINFORCES them; payload-conditional admission governs whether a dimension exists at install, never the sweep's per-session column stability."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-sim/src", "crates/simthing-clausething/src", "crates/simthing-mapeditor/src", "scenarios/terran_pirate_galaxy.clause", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["new hosting DSL, authored host field, or ClauseScript/ScenarioSpec syntax; deleting, retaining-out, or clearing domain packs / overlays / any admitted structure in ANY proof", "kind, property-name, display-name, or default-root host inference; id/name substring tests to class evidence provenance", "changes to the 5.3 table schema, WGSL, writers, remaps, or typed readback", "second host/observation authority or raw-value consumer path", "Field Sweep 5.4-5.8, Phase 6 transport, wire/replay persistence, or new raw index doors"]
required_checks: ["cargo build --workspace; core and sim batteries", "adapter-pinned full driver battery", "canonical materialization referees plus unmodified 5.3 and Studio referees", "RF-1 and replay determinism", "agent/doctrine scans; bypass census; inventory/orientation/doc-budget/anchors/clearance"]
stop_conditions: ["stale-orient-receipt or scope-widening", "any authoring, admission, or disposition change beyond the typed repairs authorized by ruling 5124757579", "zero or conflicting lawful candidates for any property in the derivation set (zero live loci after admitted structures and the lawful admission/disposition repairs resolve)", "derivation requires kind/name/root fallback or a 5.3 semantic change"]
---
## BUILD
- Census RULED (#1495). RESIDENCY = value-PLACING relations ONLY (economy
  emission/transfer/recipe host_entity, need_binding locus, threshold host,
  hosted_observation location — reaching production ONLY as the lowered presence-emission's typed `host_entity`, never an id/name substring test);
  governance instruments (owner-policy overlays, policy-weight authority) corroborate, NEVER elect.
- Add one typed install-time pass after existing admitted structures resolve and before final admission reporting / initial GPU sync. Existing loci remain authoritative — INCLUDING overlay effect-host stores, which are lawful residency under 2.1 effect-host admission, not governance electing residency. The DERIVATION SET is exactly the Anchored properties with ZERO live loci after admitted structures and the lawful admission/disposition repairs resolve.
- Derive candidates on canonical property identity from those value-placing relations ONLY. Governance evidence may corroborate an elected host, never elect one. Never consult kind, names, display metadata, or a default root.
- Materialize `PropertyValue::from_layout` only when evidence converges on exactly one existing SimThing. Zero, conflicting, or out-of-tree candidates hard-error with identity and provenance. Preserve all existing values and loci exactly.
- Attach to ordinary `compile_and_install`, so preview, atomic install, open-from-spec, and Studio share the same tree. The existing `snapshot_anchored_loci` → typed GPU-table door consumes the result without special upload or enrollment.
- ADMISSION GOVERNS EXISTENCE (one rule, no carve-outs): a resource-bearing dimension is registered IFF an authored structure admits it. `tp::{hull,weapon_damage,upkeep}` follow the fleet/ship payload — registering them with no payload is a pre-existing defect this rung exposes, and the payload-conditional fix is ADMITTED here (typed on payload presence, never on names). Whatever remains ADMITTED and genuinely hostless declares authored `Unobserved{reason}` via the 5.1 door (reasons name the uninstantiated class and Phase 8 as successor); that set is DERIVED from the corpus, not fixed at 7. There is NO derivation-set exception: registry-only placeholders such as `_studio_live_bridge::seed` are either not registered or declare `Unobserved` at their synthesis site — a property may NEVER sit Anchored with zero hosts. Then replace the canonical-zero tripwire with the ORDINARY unmutated install — domain packs and overlays ENABLED, nothing retained-out or cleared — and prove TOTALITY, not cardinality: ZERO Anchored properties with zero live loci, one GPU row per live locus, no `(thing, property)` pair twice; a property lawfully residing on several things is native to `AnchoredLocusMap` and is not duplication; regenerate `property_admission_inventory.tsv` and PUBLISH its derived counts in the results doc.
- Land one results doc and stamp 5.3b PROBATION while keeping the active pointer on 5.3b. DA alone advances to `FIELD-SWEEP-IR-PROBE-0` at graduation.
## FENCES
- Derivation-by-admission only; existing admitted structures are the complete vocabulary.
- Change tree property presence only: no value, topology, slot, RF, threshold, table, consumer, or wire/replay semantic change. Admission and disposition changes are limited to the typed repairs authorized by ruling 5124757579 — payload-conditional admission, authored `Unobserved{reason}` for admitted-and-hostless, and synthesis-site `Unobserved` or non-registration for programmatic placeholders — no fixed count, and nothing else.
- Materialized loci are observation hosts only: layout-default values; no RF
  participation, accumulator-plan, or topology change; RF-1 sums identical.
- Determinism required: same install -> same loci in canonical
  SimPropertyId order. Byte-identity with pre-5.3b goldens is NOT expected
  where property presence legitimately changes — enumerate and justify each
  re-derived golden in the results doc; never silently rebase.
- No second host registry or observation authority; provenance is diagnostic only.
## EXIT-PROOF
- ORDINARY canonical install (overlays and domain packs ENABLED, unmutated) proves: ZERO Anchored properties with zero live loci; one GPU row per live locus with no repeated `(thing, property)`; `exact / zero=0 / conflict=0` over the derivation set; and the regenerated inventory's Anchored/Unobserved counts PUBLISHED as derived results (no asserted target). Authoring is limited to the ruling-authorized typed repairs; the canonical TP `Unobserved{reason}` edits remain enumerated evidence where still applicable, never a global cap. No other fixture mutation.
- Missing/ambiguous evidence fails closed; existing loci/values and all unmodified 5.3 proofs remain exact.
- Required batteries and gates are green. PR stays PROBATION for DA graduation; no 5.4 dispatch.
