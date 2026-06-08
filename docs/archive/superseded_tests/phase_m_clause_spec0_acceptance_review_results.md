# CLAUSE-SPEC-0 (L2) — Design-Authority Acceptance Review

**Reviewer:** Opus 4.8 (design authority — Mapping/FIELD_POLICY + v7.8 track). **Date:** 2026-05-30.
**Decision:** **ACCEPT (Option A).** L2 / CLAUSE-SPEC-0 satisfies the designer-authored FrontierV2
scenario-admission gate. ClauseThing and ClauseScript remain **parked** pending separate product
authorization. No new prooflet, hygiene ladder, or remediation is required.

## What was reviewed — code, not only the report

Per completion criterion 2, I read the implementation, not just the test report:

- `crates/simthing-spec/src/designer_admission/clause_spec.rs` (543 lines) — the scenario shape,
  the admission function, the lowering, and the guardrail routing.
- `crates/simthing-spec/src/designer_admission/diagnostic.rs` — the new diagnostic codes.
- `crates/simthing-spec/src/designer_admission/preview.rs` — L1 preflight reuse + nit fix.
- `crates/simthing-driver/tests/phase_m_clause_spec0_frontier_v2_compile.rs` — the compile smoke.
- Verified `crates/simthing-sim/src/**` carries **no** FrontierV2/ClauseThing/ClauseScript/FIELD_POLICY/
  RegionCell/ArenaRegistry/proposal/ResourceFlow awareness (only the pre-existing generic
  `BoundaryRequest` core enum appears, which is unrelated).

## Review answers

| # | Question | Finding |
|---|---|---|
| 1 | Minimum FrontierV2 scenario shape? | **Yes** — `ClauseSpecFrontierV2Scenario` carries the happy-path fields (id/profile/grid/ticks/factions/resource_flow/mapping/movement/structural/artifact_targets) plus `#[serde(default)] = false` guardrail-probe booleans used only to drive negative admission. |
| 2 | Happy-path RON admits? | **Yes** — 25/25 spec admission tests + the driver compile smoke pass; `accepted == diagnostics.is_empty()`. |
| 3 | Reuses L1 preflight, not bypassed? | **Yes** — `admit_*` calls `preview_designer_admission_preflight(&manifest)` and inherits its diagnostics before adding CLAUSE-SPEC field validation. |
| 4 | Lowers only to accepted FrontierV2 artifact metadata? | **Yes** — `accepted_artifact_targets` filtered through `resolve_frontier_artifact_target_id`; `lowering_summary.metadata_only == true`; targets are `AcceptedFrontierArtifactTarget::{ResourceFlowAllocatorRoute, FrontierV2OwnColumnShadow, FrontierV2BoundaryRequestShadow}`; no production runtime object created. |
| 5 | Driver smoke proves metadata→accepted fixture, no runtime? | **Yes** — asserts the lowering points at `FrontierV2CombinedFeedbackFixture` + accepted route/movement/structural, `metadata_only`, validates the existing fixture skeleton, asserts default-off; no `run_`/dispatch/`SimSession` wiring. |
| 6 | All L0/L1 guardrails enforced at admission? | **Yes** — `to_preflight_manifest` routes every guardrail-request field into L1 rejection (`requested_guardrail_overrides`/`requested_runtime_features`/`requested_mapping_features`/`requested_resource_flow_features`/`requested_authoring_frontend`), and field validation enforces FrontierV2 profile, grid ≤ 32, ticks ≥ 2, ≥ 2 factions, `FlatStarOptIn`/`FlatStarResourceFlow`/`SparseRegionFieldV1`, required targets present, `depth_cap > 2 → nested_e11b` reject. This is the v7.8 §2.1 relocate-to-admission doctrine, realized. |
| 7 | Diagnostic nit resolved with `MalformedManifest` + `UnknownArtifactTarget`? | **Yes** — both codes exist (`L1-0-MALFORMED-MANIFEST-REJECTED`, `L1-0-UNKNOWN-ARTIFACT-TARGET-REJECTED`); `preview.rs` routes empty `manifest_id`/`profile_name` → MalformedManifest and unknown targets → UnknownArtifactTarget, no longer mislabeled `SimthingSimSemanticState`. |
| 8 | Avoided `simthing-sim` semantic awareness? | **Yes** — verified by scan; only the pre-existing generic `BoundaryRequest` core type appears. |
| 9 | Avoided ClauseThing/ClauseScript/default-wiring/scheduler/WGSL/CPU-planner/RF-bypass/cross-entity/production-writes/shared-pool? | **Yes** — each is a *rejected request boolean*, not an implementation; admission rejects them. |
| 10 | Accept, or one remediation? | **ACCEPT** — no remediation. |
| 11 | Next gate? | Doc-only L2 acceptance recording (this pass). **L3 ClauseThing / ClauseScript remain parked pending separate product authorization.** |

## Guardrail confirmations (no authorization)

ClauseThing runtime, ClauseScript parser, production `SimSession` wiring, scheduler/cache, semantic
WGSL, CPU planner/urgency/commitment, Resource-Flow bypass, cross-entity/production movement writes,
production commitment emission, shared-pool tick writes, `simthing-sim` semantic awareness,
FrontierV2-5, ACT-5/EVENT-3/OBS-5/PIPE-1, and Lines A/B/C activation **remain unauthorized** — each
is rejected at admission (when requested) or simply not implemented.

## Commands

| Command | Result |
|---|---|
| `cargo check --workspace` | **PASS** (pre-existing warnings only: `simthing-driver` unused import, deprecated `EmlTreeMeta`) |
| `cargo test -p simthing-spec --test clause_spec0_frontier_v2_admission` (implementer) | 25/25 |
| `cargo test -p simthing-driver --test phase_m_clause_spec0_frontier_v2_compile` (implementer) | 1/1 |

No code changes were made by this review (doc-only acceptance); implementer test evidence stands.

## Scans

- `simthing-sim` semantic-awareness scan: clean (only pre-existing `BoundaryRequest`).
- `FrontierV2-5 / ACT-5 / EVENT-3 / OBS-5 / PIPE-1`: rejection/parked references only.
- `ClauseThing / ClauseScript`: parked/rejected references only — no parser or runtime.
- No implementer self-acceptance phrasing; the implementation report said "pending design-authority
  review," and this memo is the acceptance of record.

## Ruling

**ACCEPT — CLAUSE-SPEC-0 satisfies L2 for designer-authored FrontierV2 scenario admission through
`simthing-spec`.** The RON-first scenario admits, reuses L1 diagnostics, lowers metadata-only to
accepted FrontierV2 fixture artifact targets, and preserves all v7.8 guardrails by relocating them
to spec admission (the §2.1 doctrine). **L2 is accepted by design authority. L3 (ClauseThing /
ClauseScript) remains parked pending separate product authorization** — do not start the ClauseScript
parser or ClauseThing runtime, FrontierV2-5, ACT/EVENT/OBS/PIPE, or any Line A/B/C activation without
a named scenario + gate. v7.8 constitution / production-track split intact.
