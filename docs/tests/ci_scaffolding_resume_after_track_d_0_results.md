# CI-SCAFFOLDING-RESUME-AFTER-TRACK-D-0 Results

## Status

**PROBATION / ORCHESTRATION REVIEW**

This rung resumes CI scaffolding after Track D closeout. It does not reopen Track D, delete tests, modify product code, edit workflows, or normalize full-workspace proof as routine.

Branch: `ci-scaffolding-resume-after-track-d-0` (pending PR).

## Current baseline

Verified on live `master` at `ef9f6f007afd1ad5aefafc3b55756d552a8db014` (merge of #1124):

| Metric | Value |
|---|---|
| Corpus inventory | **731 rows** (`scripts/ci/test_inventory.tsv`) |
| Runnable KEEP survivors | 592 unit/integration KEEP rows |
| Ledger-only markers | 137 `cfg_test_mod` marker rows |
| Dependency-floor helpers | 2 non-runnable fixture rows |
| Track D | **CLOSED** |
| Active closure anchors | **D-SWEEP** (`TEST-NECESSITY-SWEEP-0`), **D-CLOSE** (`TRACK-D-CLOSEOUT-0`) |
| Collapsed provenance | D0–D2z → historical only |

## Track D closure verification

| PR | Merged | Head | Merge commit | Verified |
|---|---|---|---|---|
| #1122 `TEST-NECESSITY-SWEEP-0` | yes | `e3c39a5af0b4b202d2c39867f2ad648708da3043` | `3ef232506f0acc3dea6810847ebdffa3fabd92ec` | yes |
| #1123 `TRACK-D-CLOSEOUT-0` | yes | `0a9764f7d7a5d825fe03ff640e4324bddbaf35df` | `742be60ea068afaf0e95ee8baffd4f4dc55667c1` | yes |
| #1124 Track-D residue correction | yes | `56e22bae00a1580bc0f111c6468e86235e9304e6` | `ef9f6f007afd1ad5aefafc3b55756d552a8db014` | yes |

#1122 deleted **3,478** tests (inventory 4,070 → 731 after 0R2 dependency-floor rows). DA independently re-ran `cargo check --workspace --all-targets` green at sweep closeout.

## Rustified Test Lifecycle summary

Every future test is born as a scoped borrow assumed deleted at its birth track's closure unless it promotes into a kernel type/seal or EML opcode-stack, is a TIER7 terminal proof with a catches: note, or is a non-runnable dependency-floor helper.

Standing law is live in:

- `docs/design_0_0_8_4_6_ci_scaffolding.md` §4.1
- `docs/design_0_0_8_3.md` §0.9
- `docs/simthing_core_design.md` §1.2
- `docs/handoff_template.md` §6
- `docs/ci_screening_surface.md` (Necessity Test + retired one-representative premise)

There is no fourth keep class. These are not keep reasons: might be useful later, legacy coverage, one representative per boundary, behavior-regression without escaped-bug proof, AUDIT, selected representative, promotion-target residue, admission-adjacent enumeration.

## Closure certificate rule

`cargo test --workspace --all-targets` is a one-time PR-ladder closure certificate or DA-deferred closure proof, not routine Doctrine Exec, scheduled, default, or comment-triggered proof.

Routine proof remains: `doctrine_scan.sh`, `gen_digest.sh --check`, `doctrine_exec_profile_lint.sh`, `doctrine_exec_profile_lint.sh --prove-gha-proof-seal`, inventory/drift/boundary gates where relevant, `cargo check` floors for touched crates, and targeted CPU-safe representatives only when profile-safe.

## Current CI scaffolding open items

1. **Track B (OPEN)** — webchat GH-CPU rungs 3–6 remain PROBATION; smoke default is `ci-b-webchat-smoke`; owner-deep quarantined.
2. **Stale Track-D Doctrine Exec profiles** — `test-pare-*` rows in `doctrine_exec_profiles.tsv` still executable but reference pre-sweep corpus binaries; must not masquerade as current proof.
3. **CI-layer residue** — Track-D-era scripts/TSVs with `test_pare_*` naming still in active gates; several authorize spent deletion waves only.
4. **PROBATION on closed Track-D ledger rungs** — evidence index still marks `TEST-PARE-STANDARD-DA-0` and related D0-era entries PROBATION (historical table; not active law).
5. **`TEST-PARE-CADENCE-DF`** — FUTURE row in design ledger; post-731 cadence not yet specified beyond §4.1 lifecycle for new tracks.

## Stale Track D residue check

### Docs

| Check | Result |
|---|---|
| `one representative per boundary` as active law | **Retired** — present only as historical/negated context in design §4, ci_screening_surface, handoff §6 |
| D0–D2z as active sequence | **Collapsed** — design §4 table marks COLLAPSED/SUPERSEDED |
| Track D OPEN / PROBATION as active track state | **Closed** — design §4 header: CLOSED 2026-07-04 |
| `material reduction not landed` | **Stale in historical results docs only** — design §4 operator bullets and D-CLOSE/DF rows use post-reduction 731-row framing |
| `full battery may graduate to scheduled sentinel` | **Qualified** — DF FUTURE row allows cadence discussion post-reduction; not wired as routine proof |

No contradictory active-law edits required in optional files for this rung.

### Profiles

See profile map below. **Gap confirmed:** `test-pare-spec` lists 70+ spec integration binaries from the pre-#1122 corpus; many are deleted. Profiles remain **executable** in Doctrine Exec if invoked — they are not inert provenance references.

### CI scripts / TSVs

See retention table below.

### test_edit_scope

`scripts/ci/test_edit_scope.tsv` has 21 rows (including header). All rows authorize **closed** Track-D or necessity-sweep edit waves (`test-pare-*`, `test-necessity-sweep-0/0r` profiles). Rationales still say "Retire when Track D paring ends" — Track D has ended. **Conclusion:** DELETE_CANDIDATE for active CI enforcement; any future lifecycle work needs birth-track-scoped edit authorization, not spent wave replay.

### PROBATION entries

Stale PROBATION on **closed Track-D ledger construction rungs** in `current_evidence_index.md` (e.g. `TEST-PARE-STANDARD-DA-0`, `TEST-PARE-INVENTORY-0`, `TEST-ADMISSION-REGIME-0`) — historical provenance only; not reopened here (broader historical-table cleanup deferred).

Track B rungs (CI-B-*) PROBATION in design §3B table remains **live** — correct for OPEN Track B.

## Track-D-era CI artifact retention/deletion audit

Track-D-era CI artifacts are retained only if they enforce the Rustified Test Lifecycle or Necessity Test; otherwise they are deletion/retirement debt and must leave the active CI layer.

| artifact | current_role | track_d_origin_or_current_origin | lifecycle_enforcement_bearing | classification | delete_or_keep_recommendation | required_followup |
|---|---|---|---|---|---|---|
| `scripts/ci/test_inventory.tsv` | Authoritative 731-row survivor ledger | TEST-PARE-INVENTORY-0 → maintained through D-SWEEP | Enforces scoped-borrow inventory truth; KEEP ownership; dependency-floor rows | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | Rename framing in CI-LIFECYCLE-RESIDUE-DELETE-0 optional |
| `scripts/ci/test_inventory_check.sh` | Validates ledger schema, KEEP notes, residue classes, audit parity | Track D gates → inherited by sweep | Enforces TIER7 `catches:` discipline, illegal KEEP, promotion-target hygiene | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/test_inventory_drift_check.sh` | Detects unledgered runnable tests + stale rows | Track D drift gate | **Critical:** catches unledgered runnable tests; dependency-floor stale exception is narrow | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/test_residue_classes.tsv` | Legal `permanent-residue:*` / promotion classes | Track D → +`dependency-floor` at 0R2 | Defines lawful survivor classes under lifecycle regime | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/test_pare_boundary_check.sh` | Validates boundary ownership for KEEP rows | TEST-PARE-STANDARD-DA-0 | Enforces KEEP rows have superseding boundary + tier disposition | RENAME_OR_REFRAME_CANDIDATE | **KEEP** (reframe) | Rename to lifecycle-boundary-check in CI-LIFECYCLE-RESIDUE-DELETE-0 |
| `scripts/ci/test_pare_boundaries.tsv` | Boundary ID / tier / retirement policy registry | TEST-PARE-STANDARD-DA-0 | Defines B-T7 terminal-proof and dependency-floor boundaries | RENAME_OR_REFRAME_CANDIDATE | **KEEP** (reframe) | Reframe naming; add lifecycle-native boundary vocabulary |
| `scripts/ci/test_pare_boundary_rows.tsv` | Per-row boundary disposition map | Track D collapse waves → frozen at 731 | Maps survivors to boundaries; NEVER_PARE / KEEP enforcement | RENAME_OR_REFRAME_CANDIDATE | **KEEP** (reframe) | Prune spent collapse candidates in gate-state follow-up |
| `scripts/ci/test_pare_audit.tsv` | Historical PARED-row provenance + audit parity | D0–D2z audit waves | Feeds inventory_check audit report; mostly spent PARED history | HISTORICAL_PROVENANCE_ONLY | **DELETE_CANDIDATE** (from active CI) | Archive to `docs/tests/` or delete in CI-LIFECYCLE-RESIDUE-DELETE-0 after inventory_check decoupling |
| `scripts/ci/test_edit_scope.tsv` | Authorizes test/src edit paths per wave | TEST-EDIT-SCOPE-GATE-0 | Only authorizes **closed** Track-D deletion waves | DELETE_CANDIDATE | **DELETE** from CI layer | Replace with birth-track edit scope in gate-state follow-up |
| `scripts/ci/test_edit_scope_check.sh` | Enforces edit_scope.tsv on diffs | TEST-EDIT-SCOPE-GATE-0 | Same — spent wave authorization only | DELETE_CANDIDATE | **DELETE** with TSV | CI-LIFECYCLE-RESIDUE-DELETE-0 |
| `scripts/ci/doctrine_exec_profiles.tsv` | Profile → command map | Track B + Track D profiles | Mixed: smoke/floor live; `test-pare-*` stale | GATE_STATE_FOLLOWUP_REQUIRED | **KEEP** table; retire stale rows | CI-PROOF-PROFILE-TAXONOMY-0 |
| `scripts/ci/doctrine_exec_profile_lint.sh` | Forbids forbidden tokens / profile classes | Track B | Enforces proof-mode taxonomy + GHA seal rules | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/doctrine_exec_gha_proof_seal.sh` | Proves non-owner-deep profiles lack GPU/desktop | GHA-PROOF-SEAL-0 | Enforces sparse routine proof | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/doctrine_scan.sh` | Allowlist doctrine scan | Track A/C | Unrelated to Track D; standing gate | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |
| `scripts/ci/gen_digest.sh` | Digest generation/check | Track C | Standing gate | LIVE_LIFECYCLE_ENFORCEMENT | **KEEP** | — |

### Classification rollup

| Class | Artifacts |
|---|---|
| LIVE_LIFECYCLE_ENFORCEMENT | `test_inventory.tsv`, `test_inventory_check.sh`, `test_inventory_drift_check.sh`, `test_residue_classes.tsv`, `doctrine_exec_profile_lint.sh`, `doctrine_exec_gha_proof_seal.sh`, `doctrine_scan.sh`, `gen_digest.sh` |
| RENAME_OR_REFRAME_CANDIDATE | `test_pare_boundary_check.sh`, `test_pare_boundaries.tsv`, `test_pare_boundary_rows.tsv` |
| HISTORICAL_PROVENANCE_ONLY | `test_pare_audit.tsv` (active CI coupling only) |
| DELETE_CANDIDATE | `test_edit_scope.tsv`, `test_edit_scope_check.sh`, `test_pare_audit.tsv` (after decouple) |
| GATE_STATE_FOLLOWUP_REQUIRED | `doctrine_exec_profiles.tsv` (stale `test-pare-*` executable rows) |

## Profile/proof-mode reconciliation

**Question:** Are historical Track-D profiles retained only as provenance/inert references, or are they still executable Doctrine Exec profiles?

**Answer:** They are **still executable** if `/seal-proof profile=test-pare-*` or plan lint references them. They are **not** inert. Several reference deleted test binaries and must be reconciled or retired before any agent treats a green run as current corpus proof.

| profile_id | profile_class | risk_class | status_after_track_d | current_concern | recommended_followup |
|---|---|---|---|---|---|
| `ci-b-webchat-smoke` | smoke | webchat-orchestration | active-current | PR default; mechanics-only | **KEEP** — routine proof |
| `seal-residue` | targeted | seal-residue | active-current | Doc/seal proof | **KEEP** — targeted |
| `data-deliverable` | targeted | data-deliverable | active-current | Named TP deliverable tests | **KEEP** — verify survivors still exist |
| `tests-compile-floor-non-bevy` | targeted | gate-state/tests-compile-floor | active-current | Standing compile floor | **KEEP** — routine gate-state |
| `test-pare-clausething` | targeted | test-deletion-clausething | historical-stale | Track-D deletion wave; wave spent | **needs-narrowing** or retire — CI-PROOF-PROFILE-TAXONOMY-0 |
| `test-pare-spec` | targeted | test-deletion-spec | historical-stale | 70+ binaries; many deleted in #1122 | **needs-retirement** — CI-PROOF-PROFILE-TAXONOMY-0 |
| `test-pare-tier2-cpu-admission-collapse` | targeted | test-deletion-tier2-admission-collapse | historical-stale | Spent admission-collapse wave | **needs-retirement** |
| `test-pare-mapgenerator-admission-collapse` | targeted | test-deletion-tier2-admission-collapse | historical-stale | Spent mapgenerator wave | **needs-retirement** |
| `test-pare-cpu-safe-boundary-sweep` | targeted | test-deletion-cpu-safe-boundary-sweep | historical-stale | Spent sweep | **needs-retirement** |
| `test-pare-broken-clausething-admission-residue` | targeted | test-deletion-broken-clausething-admission-residue | historical-stale | Spent residue wave | **needs-retirement** |
| `test-pare-src-unit-fossil-residue` | targeted | test-deletion-src-unit-fossil-residue | historical-stale | Crate-check only; wave spent | **needs-retirement** |
| `test-pare-gpu-bevy-residue` | targeted | test-deletion-gpu-bevy-residue | historical-stale | Partial GPU-named tests; wave spent | **needs-retirement** |
| `test-pare-protected-pare-delete` | targeted | test-deletion-protected-pare | historical-stale | Crate-check shell; wave spent | **needs-retirement** |
| `test-pare-conservative-survivor-delete` | targeted | test-deletion-conservative-survivor | historical-stale | Crate-check shell; wave spent | **needs-retirement** |
| `test-consolidate-classifier-families` | targeted | test-deletion-classifier-consolidation | historical-stale | Hygiene table test deleted in #1122 | **needs-retirement** |
| `owner-deep-full-cpu-quarantined` | owner-deep | general-owner-deep | owner-deep-quarantined | Dispatch-only artillery | **KEEP** — owner-deep |
| `timeout-finalize-proof` | probe | timeout-finalize | active-current | Probe infrastructure | **KEEP** — probe |

### Proof-mode taxonomy (post-Track-D)

| Mode | Current posture |
|---|---|
| routine | `doctrine_scan.sh`, `gen_digest.sh --check`, profile lint + GHA proof-seal, `ci-b-webchat-smoke` on PR |
| targeted | Named profiles with exact binaries; `tests-compile-floor-non-bevy` for gate-state |
| owner-deep | `owner-deep-full-cpu-quarantined` — workflow_dispatch only |
| closure-certificate | `cargo test --workspace --all-targets` — one-time only; used at D-SWEEP/D-CLOSE; not in profiles table |

## Handoff template check

`docs/handoff_template.md` §6 aligns with standing law:

- Necessity Test / zero floor for rejection-class coverage
- Rustified lifecycle (assumed deleted at track closure)
- Three lawful survivor paths (promote-to-kernel, TIER7+catches, dependency-floor)
- Closure certificate rule for `cargo test --workspace`

**Gap:** Template does not yet force explicit **birth-track** + **deletion-story** fields for new tests in a dedicated subsection — optional follow-on `CI-HANDOFF-LIFECYCLE-ENFORCEMENT-0` if orchestration wants mechanical enforcement beyond §6 prose.

## CI screening surface check

`docs/ci_screening_surface.md` correctly states Necessity Test supersedes one-representative-per-boundary. Track D note block still references `test_pare_boundaries.tsv` machinery — accurate as **enforcement machinery** but Track-D-named. No edit required this rung; reframe in CI-LIFECYCLE-RESIDUE-DELETE-0.

## Required follow-on PRs

| Priority | ID | Scope |
|---|---|---|
| 1 | `CI-LIFECYCLE-RESIDUE-DELETE-0` | Remove/reframe Track-D-era CI scripts/TSVs without live lifecycle bearing; decouple `test_pare_audit.tsv`; delete `test_edit_scope*`; rename boundary machinery |
| 2 | `CI-PROOF-PROFILE-TAXONOMY-0` | Retire/narrow stale `test-pare-*` profiles; mark historical profiles inert; prevent deleted-test binaries masquerading as current proof |
| 3 | `CI-COMMAND-ERGONOMICS-0` | Publish copy-paste proof blocks: routine PR, gate-state PR, test-addition PR, closure certificate |
| 4 | `CI-HANDOFF-LIFECYCLE-ENFORCEMENT-0` | Only if birth-track/deletion-story fields need template hardening |

## Validation

Docs-only rung — validation at PR head:

| Gate | Result |
|---|---|
| `doctrine_scan.sh` | **PASS** (failures=0 inspect=0; commit ef9f6f007a at scan time) |
| `gen_digest.sh --check` | **PASS** |
| `doctrine_exec_profile_lint.sh` | **PASS** (profiles=17 default=ci-b-webchat-smoke) |
| `doctrine_exec_profile_lint.sh --prove-gha-proof-seal` | **PASS** (prove) |
| `git diff --check origin/master...HEAD` | **PASS** |
| `cargo test --workspace --all-targets` | **not run** (forbidden for this rung) |

Inventory/boundary/edit-scope gates **not run** — no script/ledger edits in this PR.

## Scope Ledger

| Bucket | Items |
|---|---|
| specified | Results doc; evidence index row; design ledger row; Track-D artifact audit; profile map; stale residue check |
| implemented | All specified docs deliverables |
| proxied | Live tree verification via git + GitHub connector (not relay-only) |
| deferred | Profile/script deletion; PROBATION cleanup on historical D0 entries; optional ci_screening_surface reframe |

## Graduation routing

| Field | Value |
|---|---|
| Risk class | docs-only CI scaffolding verification |
| CI verdict | PROBATION / ORCHESTRATION REVIEW |
| Falsification check | Orchestrator confirms audit table matches live `master`; no Track D reopen language introduced; stale `test-pare-*` profiles explicitly classified executable-not-inert |
| Recommended posture | Light-review eligible if docs-only and gates green; follow-ons route to gate-state/DA for CI script/profile deletion |