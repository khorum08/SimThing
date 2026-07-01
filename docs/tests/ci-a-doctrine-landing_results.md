# CI-A-DOCTRINE-LANDING-0 Results

## Status

**PROBATION** — docs-only three-altitude landing of the CI doctrine-scan contract. Not COMPLETE; DA/Owner closes the rung.

## PR / branch / merge

- Branch: `ci-a-doctrine-landing-0`
- PR: [#1043](https://github.com/khorum08/SimThing/pull/1043)
- Merge: `a1ef8334ff` (master)
- PR run: [28494374601](https://github.com/khorum08/SimThing/actions/runs/28494374601) — success (56s)
- Master push run: [28494467422](https://github.com/khorum08/SimThing/actions/runs/28494467422) — success (53s)

## Recipient Agent

Cursor (docs-focused pass; template specified Haiku/Sonnet, executed under the session's active agent per owner dispatch)

## Orchestrator / §1A Triage Agent

Codex

## Executive DA

Opus / Owner

## What changed

Docs-only. No runtime code, crates, scanners, allowlists, fixtures, or workflow files touched.

- `docs/simthing_core_design.md` — added a new paragraph after the cross-crate seal law and before "Residue-as-tripwire" in §1.2.1: **"CI doctrine-scan — the automated rung-3 guard layer (`0.0.8.4.6`)."** States the CI scan is the executable form of the admission ladder's guard-scan rung, DA-equivalent when clean, INSPECT-routes-to-triage, and itself residue (retire on promotion).
- `docs/design_0_0_8_3.md` — added point 5 to the §0.9 carry-forward list: **DA-equivalence contract, §1A triage contract, allowlist contract, scan-retirement obligation, merge-hold rule, verification rule** (verify the tree, not the relayed report). Kept compact — one point, no history log.
- `docs/handoff_template.md` — added two spine bullets (CI doctrine-scan contract; merge-hold + verify-the-tree rule) to the non-negotiable context-spine block, and in §1 Identity: expanded `seal-residue-risk` with `authority boundary touched` / `scan-retirement candidate` values, added a new `ci-doctrine-scan:` field (expected commands, RELIABLE/HEURISTIC/INSPECT paths, allowlist-edit expectation), a **Retirement obligation** bullet, and a **Merge-hold rule** bullet.
- `docs/design_0_0_8_4_6_ci_scaffolding.md` — `CI-A-DOCTRINE-LANDING-0` row: `AUTHORIZED — dispatch` → **PROBATION**, DoD cell records what landed where.
- `docs/tests/ci-a-selftest-inspect-repair-0_results.md` — GitHub Actions proof section clarified: recorded all three genuine runs (first PR run `28492389014`/`84451616254`; second PR run `28492477649`/`84451883130`, the one Opus's ruling cites; actual master push run `28492521172`/`84452014679`), and corrected the Opus-relayed label — `28492477649` is tree-verified (`gh run view`) as a `pull_request` run on the PR branch, not a master push, per the "verify the tree" rule this very rung lands.
- `docs/tests/current_evidence_index.md` — `CI-A-SELFTEST-INSPECT-REPAIR-0` row updated to **DA-CLEARED / COMPLETE** with all three run IDs; new `CI-A-DOCTRINE-LANDING-0` row added.
- Verified `.tmp_*` / `scripts/ci/.tmp_*` gitignore guard from #1042 is present (`.gitignore` lines 24–25) — not reworked, per instruction.

## Three-altitude landing

| Altitude | File | Section |
|---|---|---|
| Core design | `docs/simthing_core_design.md` | §1.2.1, new paragraph before "Residue-as-tripwire" |
| Constitution / standing doctrine | `docs/design_0_0_8_3.md` | §0.9, new point 5 |
| Handoff template | `docs/handoff_template.md` | context spine (2 new bullets) + §1 Identity (4 additions) |

## CI contract landed

- CI doctrine-scan is the automated rung-3 guard layer standing in for a type/admission boundary.
- Clean **RELIABLE** = DA-equivalent, trusted without re-verification.
- **FAIL** = HOLD.
- **INSPECT** = triage/look — never silent pass, never automatic block.

## §1A triage contract landed

- PR author pays first with a one-line structured justification per INSPECT flag.
- Bounded loop + greppable spam-bounds (excess INSPECT volume; symbol-walking across HEURISTIC scan-ids; INSPECT rising while a RELIABLE FAIL stays open) force hill-climbing to escalate-as-FAIL.
- DA spot-audit remains the backstop over triage clearances.

## Allowlist / sanctioned-door contract landed

- An `allow/*.txt` entry is a **typed admission record**, not a babysat list.
- A new entry is a **deliberate, reviewed widening of a sanctioned door** (door-class grammar + rationale + promotion-blocker).
- Never edit scanner logic to dodge a valid finding.

## Scan-retirement obligation landed

- A scan is itself residue (rung 3 of the admission ladder).
- When its guarded invariant promotes to a type boundary or admission hard-error, the promoting rung **retires (narrows/deletes) the now-redundant scan in the same PR**.
- A RELIABLE scan with no promotion-blocker is a flagged anomaly (already true in `scans.tsv` discipline; now also stated at core/constitution/template altitude).

## seal-residue-risk field landed

- Handoff template §1 Identity: `seal-residue-risk:` now enumerates `none` (default) | `<B#…>` | `authority boundary touched` | `scan-retirement candidate`.
- New `ci-doctrine-scan:` field alongside it: expected commands, RELIABLE/HEURISTIC/INSPECT involvement, whether an allowlist edit is expected.

## Merge-hold rule landed

- Constitution §0.9.5 and handoff template both state: no rung touching PROBATION / authority / gate-state semantics merges before DA/Owner clearance.
- A truthful corrective self-report of a breach may be accepted on its merits — never precedent for skipping clearance again.
- Paired verification rule: **verify the tree, not the relayed report** — landed at both altitudes, and applied in this very PR's evidence-cleanup section (the Opus-relayed run-ID label was corrected against `gh run view`, not merely copied forward).

## Evidence cleanup

- Final Actions run ID `28492477649` / job `84451883130` recorded in `docs/tests/ci-a-selftest-inspect-repair-0_results.md` and `docs/tests/current_evidence_index.md`, alongside the earlier `28492389014`/`84451616254` run (not erased) and the actual master push run `28492521172`/`84452014679`.
- Tree-verification correction: `28492477649` is a `pull_request` run, not the "final post-merge/master run" label Opus's summary used — both facts now recorded so a future reader sees the genuine event type.
- `.tmp_*` / `scripts/ci/.tmp_*` `.gitignore` guard from #1042 confirmed present; not reworked (already handled).

## Load-bearing greps

```
rg -n "DA-equivalence|DA-equivalent" docs
  -> docs/handoff_template.md, docs/design_0_0_8_3.md, docs/simthing_core_design.md,
     docs/design_0_0_8_4_6_ci_scaffolding.md

rg -n "DOCTRINE-SCAN|doctrine-scan|CI doctrine-scan" docs
  -> lands in docs/design_0_0_8_4_6_ci_scaffolding.md, docs/simthing_core_design.md,
     docs/handoff_template.md, docs/design_0_0_8_3.md (+ existing CI evidence docs)

rg -n "INSPECT|triage|spam-bound|spam bounds" docs
  -> lands in docs/design_0_0_8_3.md, docs/handoff_template.md,
     docs/design_0_0_8_4_6_ci_scaffolding.md, docs/tests/current_evidence_index.md
     (+ existing CI evidence docs)

rg -n "allowlist.*reviewed|sanctioned door|typed admission" docs
  -> docs/design_0_0_8_3.md, docs/simthing_core_design.md,
     docs/design_0_0_8_4_6_ci_scaffolding.md, docs/handoff_template.md

rg -n "seal-residue-risk|seal residue risk" docs
  -> docs/design_0_0_8_3.md, docs/simthing_core_design.md,
     docs/design_0_0_8_4_6_ci_scaffolding.md, docs/handoff_template.md

rg -n "retire|retirement|promotion target" docs
  -> lands in docs/handoff_template.md, docs/design_0_0_8_4_5_simthing_kernel.md,
     docs/simthing_core_design.md, docs/design_0_0_8_3.md,
     docs/design_0_0_8_4_6_ci_scaffolding.md (+ many pre-existing hits)

rg -n "No rung.*DA clearance|before DA clearance|Do not trust relayed proof" docs
  -> docs/handoff_template.md, docs/design_0_0_8_4_6_ci_scaffolding.md
```

## Load-bearing command transcripts

```
bash scripts/ci/doctrine_selftest.sh
DOCTRINE-SELFTEST-VERDICT: PASS   (positive control PASS; 16 known-bad PASS; 4 heuristic PASS; 6 traps PASS; rot test PASS)

bash scripts/ci/doctrine_scan.sh
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED

bash scripts/ci/doctrine_pr_scan.sh --prove-delta
PR-delta proof: PASS   (all 4 cases PASS, including the load-bearing pre-existing-heuristic-outside-delta case)

bash scripts/ci/inspect_spam_check.sh --prove
INSPECT-SPAM-PROOF: PASS   (all 5 cases: single-gray-zone OK, symbol-walking SPAM, >3-inspect SPAM,
                             branch-name-alias-only OK, rising-while-reliable SPAM)

python scripts/ci/verify_kernel_surface.py
lib.rs exports: 195
kernel_surface.txt: 195
missing: []
extra: []
```

`git status` after all five proof runs showed only the three intended doc edits — no proof-junk (`gray.rs`, `s.rs`, `f1.rs`-`f5.rs`, `bad.rs`, `i1.rs`-`i3.rs`) leaked into the tracked tree, confirming `inspect_spam_check.sh --prove`'s temp-repo isolation holds.

## Scope ledger

Touched:

- `docs/simthing_core_design.md`
- `docs/design_0_0_8_3.md`
- `docs/handoff_template.md`
- `docs/design_0_0_8_4_6_ci_scaffolding.md`
- `docs/tests/ci-a-doctrine-landing_results.md` (new)
- `docs/tests/ci-a-selftest-inspect-repair-0_results.md` (Actions run ID section only)
- `docs/tests/current_evidence_index.md`

Untouched (forbidden, confirmed clean):

- `crates/**`
- `scripts/ci/scans.tsv`, `scripts/ci/scan_allowlists.py`, `scripts/ci/allow/**`, `scripts/ci/fixtures/**`
- `scripts/ci/doctrine_scan.sh`, `scripts/ci/doctrine_pr_scan.sh`, `scripts/ci/doctrine_selftest.sh`, `scripts/ci/inspect_spam_check.sh`
- `.github/workflows/**`
- `.gitignore` (already correct from #1042; not reworked)
- Track B / Track C runtime artifacts, Studio / mapeditor files
- `SEALED_TYPES` hard-coding in `scan_allowlists.py` (closeout debt, explicitly deferred to `CI-A-CLOSEOUT-0`)

## Known gaps / next

- `scan_allowlists.py` still hard-codes `SEALED_TYPES` — migrate to data before `CI-A-CLOSEOUT-0` (standing debt, not this rung's scope).
- `doctrine_selftest.sh` runtime remains ~7 minutes (process-spawn bound) — non-blocking debt, optimize when convenient.
- `CI-A-WORKFLOW-0` / `CI-A-WORKFLOW-0R` remain PROBATION (unaffected by this docs-only rung).
- Next rung after DA verification: `CI-A-CLOSEOUT-0` (Opus/Owner) — record the DA-equivalence + retirement contracts as closed doctrine, verify all three altitudes, publish the reliability legend, close the track.
