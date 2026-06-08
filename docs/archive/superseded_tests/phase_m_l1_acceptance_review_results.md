# L1-ACCEPT-0 — simthing-spec Buildout Closure and L2 Gate Decision (Design-Authority Review)

## Base HEAD

`dde0daf23e5e56a10bbb97682e7c994c31dc078e` (post V7.8-CLEAN-0 merge, PR #350)

## Reviewer

Design authority (Opus), 2026-05-30.

## Files reviewed

| File | Role |
|---|---|
| `docs/design_v7_8.md` | constitution |
| `docs/design_v7_8_production_track.md` | PR ladder home |
| `docs/tests/phase_m_l1_0_designer_admission_substrate_results.md` | L1-0 evidence |
| `docs/tests/phase_m_l1_1_designer_preflight_manifest_results.md` | L1-1 evidence |
| `docs/tests/phase_m_v7_8_cleanup_track_prune_results.md` | V7.8-CLEAN-0 evidence |
| `docs/workshop/field_policy_track.md` | FIELD_POLICY/Frontier charter §10–§11 |
| `docs/workshop/mapping_current_guidance.md` | status table |
| `docs/invariants.md` | binding constraints (read; unchanged) |
| `docs/adr/mapping_sparse_regioncell.md` | governing Mapping ADR (firewall pattern) |
| `docs/adr/resource_flow_substrate.md` | governing Resource Flow ADR (firewall pattern) |
| `docs/worklog.md` | append-only history |
| `crates/simthing-spec/src/designer_admission/{mod,diagnostic,artifact_target,preflight,preview,manifest}.rs` | **L1 substrate code (verified, not just reports)** |

Archive consulted as reference only; not treated as active authority.

## Review answers

| # | Question | Finding |
|---|---|---|
| 1 | Did L1-0 provide the required designer-facing guardrail diagnostic vocabulary? | **Yes.** `DesignerFacingGuardrailClass` (14 buckets), `DesignerAdmissionRejectionKind` (24 kinds), `DesignerAdmissionDiagnosticCode` (24 stable `L1-0-*` codes), `DesignerAdmissionDiagnostic` (message + hint), and `evaluate_designer_admission_request` preflight surface. Code verified. 12 tests pass. |
| 2 | Did L1-1 provide a sufficient RON-first preflight manifest + diagnostic preview surface? | **Yes.** `DesignerAdmissionPreflightManifest` (shallow posture fields, default-off) with RON roundtrip in `ron.rs`; `preview_designer_admission_preflight` composes L1-0 diagnostics from designer-authored feature tokens into `DesignerAdmissionPreviewReport` (accepted targets, diagnostics, `rejected` flag, summary lines). 13 tests pass. |
| 3 | Does the current L1 substrate give L2 enough structure to admit a designer-authored FrontierV2 scenario? | **Yes — for the preflight/admission contract.** The shared vocabulary + RON input surface + preview report are exactly the import-time firewall L2 needs. L2 adds the full scenario graph and compile-to-artifacts path on top; the rejection contract it must enforce already exists. |
| 4 | Are the accepted FrontierV2 artifact target identifiers sufficient for L2 lowering? | **Yes.** Five stable targets (`AcceptedFrontierV2FixtureArtifacts`, `FrontierV2CombinedFeedbackFixture`, `FrontierV2OwnColumnShadow`, `FrontierV2BoundaryRequestShadow`, `ResourceFlowAllocatorRoute`) with stable string ids + `resolve_frontier_artifact_target_id`. These name the same accepted L0 V1-5→V2-0..4 fixture artifacts L2 lowers to. |
| 5 | Are guardrails now correctly represented at designer/spec admission? | **Yes.** Every hard-coded L0 fixture protection (default-on, RF bypass, cross-entity/production movement writes, production commitment, shared-pool tick writes, CPU planner/urgency/commitment, semantic WGSL, scheduler/cache, simthing-sim semantic leakage, atlas/mask/perception/source without gate, nested E-11B/E-11B-5/D-2a without named scenario, ClauseScript/ClauseThing parked, FrontierV2-5 rejected, ACT-5/EVENT-3/OBS-5/PIPE-1 reopen rejected) maps to a stable rejection code. Matches the Mapping/Resource-Flow ADR firewall pattern. |
| 6 | Did V7.8-CLEAN-0 remove clutter without losing authoritative evidence? | **Yes.** L0 (V1-5→V2-0..4), L1-0/L1-1, FIELD_POLICY V1, E-phase/E11, M-JIT retained evidence all remain active in `docs/tests/`. Only closed/superseded design/workshop/production docs archived and 13 scratch `.log` files deleted. No SHA/fingerprint reconciliation. |
| 7 | Is the AccumulatorOp v2 production plan correctly closed/stubbed/archived? | **Yes.** Active path is a 20-line CLOSED stub pointing to `design_v7_8_production_track.md`; full plan at `docs/archive/closed_production/`. |
| 8 | Is any L1 gap concrete enough to require L1-2? | **No.** One non-blocking quality nit exists (see below) but no substrate gap blocks L2 admission. |
| 9 | Should L2 / `CLAUSE-SPEC-0` open now? | **Yes.** |
| 10 | What exact status should be recorded in the production track? | L1 **ACCEPTED**; L2 / `CLAUSE-SPEC-0` **OPEN — next implementation gate**; L3 ClauseThing/ClauseScript parked; Lines A/B/C parked. |

## Non-blocking observation (not an L1-2 gap)

`preview.rs` reuses `DesignerAdmissionDiagnosticCode::SimthingSimSemanticStateRequestRejected` for
structural manifest validation (empty `manifest_id`/`profile_name`, unknown artifact target). These
are real rejections with correct messages, but the *code* is semantically mislabeled. This is a
quality nit for L2 to refine (it may want a dedicated `MalformedManifest`/`UnknownArtifactTarget`
code) — it does **not** block L2 admission and does **not** require a narrow L1-2 slice. Recorded
here so L2 can address it inline rather than as a separate hygiene pass.

## Guardrail confirmation

No authorization exists for any of:
FrontierV2-5; ACT-5/EVENT-3/OBS-5/PIPE-1; ClauseThing runtime; ClauseScript parser; production
SimSession wiring; scheduler/cache; semantic WGSL; CPU planner/urgency/commitment; Resource Flow
bypass; cross-entity/production movement writes; production commitment emission; shared-pool tick
writes; simthing-sim semantic awareness; Lines A/B/C activation without named scenario + gate.

All appear only as **rejection/guardrail vocabulary** in `simthing-spec/src/designer_admission/` and
as negative/parked references in docs.

## Decision

**ACCEPT (Option A).** L1 simthing-spec buildout is sufficient for the next gate. The L1-0
designer-admission diagnostic vocabulary, L1-1 RON preflight manifest + diagnostic preview surface,
and accepted FrontierV2 artifact targets are an adequate designer/spec admission substrate. **L2 /
`CLAUSE-SPEC-0` is opened** as the next implementation gate: admit a designer-authored FrontierV2
scenario through `simthing-spec` and lower it to the same accepted FrontierV2 fixture artifacts. No
L1-2 implementation required. ClauseThing and ClauseScript remain parked.

## Doc updates

| File | Update |
|---|---|
| `docs/design_v7_8_production_track.md` | L1 status → ACCEPTED; L2 status → **open/next gate**; L1-ACCEPT-0 row added |
| `docs/workshop/mapping_current_guidance.md` | L1-ACCEPT-0 row; forward horizon → L2 open |
| `docs/workshop/field_policy_track.md` | L1-ACCEPT-0 ruling note under §11 |
| `docs/worklog.md` | append-only L1-ACCEPT-0 line |

## Scans run

| Scan | Expected | Result |
|---|---|---|
| `rg "L1-ACCEPT-0\|CLAUSE-SPEC-0\|L1 simthing-spec\|L2 CLAUSE-SPEC" <active docs + this report>` | references present, consistent | PASS |
| `rg "FrontierV2-5\|ACT-5\|EVENT-3\|OBS-5\|PIPE-1" crates docs` | no authorization; negative/guardrail only | PASS — only rejection vocab in `designer_admission/` + historical stop refs |
| `rg "ClauseThing\|ClauseScript" <active docs + this report>` | parked only | PASS |
| `find docs/tests … *.log/*tmp*/*scratch*` | none / delete | PASS — 0 found (V7.8-CLEAN-0 already removed; nothing regenerated) |

## Commands run

| Command | Result |
|---|---|
| `cargo check --workspace` | **PASS** (pre-existing warnings only) |

Doc-only review; no `simthing-spec` code changed, so the per-test reruns were not required. The L1-0
(12) and L1-1 (13) suites were last green at L1-1 landing (`385f2f6`); no code touched since.

## Transient cleanup result

No scratch/tmp/log artifacts present. None deleted (V7.8-CLEAN-0 already cleared them).

## Next gate status

| Gate | Status |
|---|---|
| L0 Frontier consumer | landed + ACCEPTED |
| L1 simthing-spec buildout | **ACCEPTED (L1-ACCEPT-0)** — L1-0 + L1-1 sufficient |
| **L2 / CLAUSE-SPEC-0** | **OPEN — next implementation gate** (designer-authored FrontierV2 scenario admission through simthing-spec) |
| L3 ClauseThing / ClauseScript | parked |
| Lines A/B/C | parked behind named scenarios |
| FrontierV2-5 | rejected |
| ACT-5 / EVENT-3 / OBS-5 / PIPE-1 | unauthorized |

## Final verdict

**ACCEPT** — L1-ACCEPT-0 accepted the simthing-spec buildout as sufficient to open L2 /
`CLAUSE-SPEC-0`, based on L1-0 diagnostic vocabulary, L1-1 RON preflight manifest/preview, and
V7.8-CLEAN-0 context cleanup. L2 is now the next gate for designer-authored FrontierV2 scenario
admission through simthing-spec. ClauseThing and ClauseScript remain parked; FrontierV2-5 and
ACT/EVENT/OBS/PIPE ladder expansion remain unauthorized; no cleanup or SHA hygiene loop was started.
