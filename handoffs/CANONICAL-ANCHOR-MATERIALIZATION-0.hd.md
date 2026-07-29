---
rung: CANONICAL-ANCHOR-MATERIALIZATION-0
kind: rung
track: 0.0.8.7
base_sha: 1c2b6613172ef19adc5282bc85d7fa2857ebe3fd
audience: coding
model_tier: std
owner_approved: true
expected_route: DA-RESERVE(novelty)
owner_notes: "DA-authorized Route-3 successor to rung 5.3 under rulings 5121471257 and 5122416859. Std lane: Grok CLI (grok-4.5 pinned), but the derivation law in this draft requires DA review and issuance before coding dispatch. Build canonical host materialization only; 5.3 GPU-table semantics remain fixed and 5.4+ Field Sweep remains fenced."
surfaces: ["crates/simthing-core/src", "crates/simthing-driver/src", "crates/simthing-driver/tests", "crates/simthing-sim/src", "scripts/ci", "docs/design_0_0_8_7_rf_arena_modernization.md", "docs/tests"]
forbidden: ["new authored property-host field, hosting DSL, or ClauseScript/ScenarioSpec syntax", "kind/name-prefix/root-default host inference", "changes to anchor-table POD/schema, WGSL, fused writer, remap shader, or typed readback", "second observation table, CPU shadow authority, consumer-specific cache, or raw-value consumer path", "FIELD-SWEEP rungs 5.4-5.8", "Phase 6 event transport, async queues, rings, or backpressure redesign", "wire/replay persistence of derived host or anchor-table state", "new raw SlotIndex/ColumnIndex/RoleOffset doors"]
required_checks: ["cargo build --workspace", "cargo test -p simthing-core", "cargo test -p simthing-sim", "adapter-pinned full simthing-driver battery", "canonical anchor-materialization admission referees", "anchor_table_surface_0 and Studio observation referees unmodified", "RF-1 and replay determinism", "agent-scan and doctrine-scan", "observation-bypass census, inventory, orientation, doc-budget, anchors, clearance"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "canonical 25 hosts require new authoring syntax or scenario mutation", "a missing Anchored property has zero or conflicting lawful host candidates", "materialization requires kind/name/root fallback", "5.3 table/kernel/consumer semantics must change", "wire or replay bytes change"]
---
## BUILD
- Add one typed admission-time host-materialization pass after existing install structures are resolved and before final property-admission reporting / initial GPU sync. Existing live `(SimThingId, SimPropertyId)` loci remain authoritative; only Anchored properties with zero live loci enter derivation.
- Derive candidates on canonical property identity from EXISTING admitted evidence only: Resource Flow derivation/parent edges, owner policy-weight authority, and install-resolved threshold, need/economy, overlay, or hosted-observation registrations. Do not consult SimThing kind, property spelling, display metadata, or a default root.
- A missing Anchored property materializes `PropertyValue::from_layout` only when the admitted evidence converges on exactly one existing SimThing host. Zero candidates, multiple conflicting candidates, or evidence pointing outside the admitted tree hard-error with property identity plus available span/provenance; never guess or silently skip. Preserve every pre-existing value and locus byte-for-byte.
- Attach the pass to the ordinary `compile_and_install` path so `preview_install`, `install_atomic`, `open_from_spec`, and Studio preview/accept all receive the same materialized tree. The existing `snapshot_anchored_loci` → typed GPU anchor-table admission door must observe the result without a special upload or caller enrollment seam.
- Replace the 5.3 canonical-zero tripwire with an unmodified full canonical TP install referee: hydrate the repository ClauseScript pack, use its complete admitted game mode / authority structure, manufacture no hosts, and prove `25 Anchored / 0 Unobserved`, `25` live loci covering `25` unique property identities, and `25` typed GPU table rows. Re-run the 5.3 hosted-observation and Studio authority referees unchanged; add only load-bearing negatives for missing/conflicting evidence, existing-value preservation, and Unobserved exclusion.
- Land one signal-only results doc; stamp 5.3b PROBATION and regenerate orientation, but keep the authoritative active pointer on 5.3b. Under the standing Two-Source Pointer Rule, only the DA graduation stamp advances to `FIELD-SWEEP-IR-PROBE-0`.
## FENCES
- Derivation-by-admission only: no new hosting DSL, authored host field, scenario fixture edit, kind/name heuristic, or root catch-all. Existing admitted structures are the complete vocabulary.
- Materialization changes tree property presence only. It must not change property disposition, values, topology, slots, RF math, threshold semantics, anchor-table schema/writers/readback, Studio consumer authority, or wire/replay bytes.
- No second host registry or observation authority. Any evidence/report used to explain admission is diagnostic only and cannot become runtime lookup state.
- Fulcrum successor only: no Field Sweep 5.4-5.8 and no Phase 6 transport work.
## EXIT-PROOF
- The unmodified canonical TP install derives exactly one live host for each of its 25 Anchored properties, zero Unobserved/dark loci, and exactly 25 rows in the existing GPU anchor table; no caller enrollment, authored host syntax, or manufactured fixture state appears in the diff.
- Missing/ambiguous evidence fails closed; already-hosted loci and values remain exact; 5.3 consumer/bypass/remap/generation proofs remain green and unedited.
- Workspace/core/sim plus adapter-pinned full driver and Studio observation corpus, RF-1, replay, scans, censuses, inventory, orientation, doc-budget, anchors, and clearance are green. PR remains PROBATION for DA deep-tree graduation; no 5.4 dispatch.
