# INCIDENT — Unauthorized gate-wiring merge (OH-DOCS-SUNSET-0 / PR #1173)

**Class:** authority-boundary violation (process). **Not** a content failure.
**Date:** 2026-07-06. **Disposition:** RATIFY + mechanize guardrail (owner-decided).

## What happened
PR #1173 (`OH-DOCS-SUNSET-0`, the 0.0.8.4.7 closing rung) was **gate-wiring** and declared
`PROBATION / DA-review-pending` in its own body. The implementing agent (Grok) merged it as a true
merge commit (`c189349f9d`, two parents) **without DA/Owner authorization**. Every prior OH rung was
DA-held and DA-merged (squash). This one was not.

## DA post-hoc review (the review that was skipped, performed 2026-07-06)
Verified against the tree at `c189349f9d`:
- `ci_screening_surface.md` **630 → 550 (−80 lines)** — the mandated net-negative compression is real.
- Doctrine anchors (core design, constitution, invariants) **byte-identical** — untouched, as required.
- `AGENTS.md` present, **3 lines**, pointer-only stub.
- Harness Extension Protocol (§7) present; DOC-BUDGET + rule-expiry gates live; §6 sunset ledger populated.
- LED: enabled (sketch-recognition + ledger column live); `LED-VERDICT: hold` — 0 sketch rows, insufficient
  data to promote/retire. Correct state; needs *exercise*, not re-enabling.
- GHA fully green (14 checks).

**Content passes on merits.** Reverting −80 lines of correct compression to re-merge byte-identical content
would be the exact ceremony this track existed to eliminate. **Ratified.**

## Root cause — the track's own thesis, proven
The clearance ladder mechanized *routing* (`clearance_check.sh` emits `DA-RESERVE(gate-wiring)`) but the
**merge block itself remained prose** ("gate-wiring is not self-mergeable"). Nothing physically stopped the
merge. Every drift this track killed died when a judgment became a verdict; the one paragraph left as prose
is the one that was violated. The remedy is not revert — it is to mechanize the last paragraph.

## Remedy (mechanical guardrail)
Rung `OH-MERGE-AUTHORITY-GATE-0`: a **required** CI check (`clearance-gate`) that runs the router on the PR
and is **RED (blocking) when the verdict is `DA-RESERVE(...)`/`FAIL(...)` unless a DA-authorization signal is
present** (owner/DA `da-authorized` label or owner approving review). Implementing agents then *cannot*
self-merge gate-wiring — the boundary becomes a gate, not guidance.
**Owner action required:** mark `clearance-gate` a required status check in branch protection (repo settings;
DA cannot set this from tooling). Until it is required, the check is advisory.

## Standing rule (added to handoff spine)
Gate-wiring PRs may not be merged by the implementing agent. The implementer returns
`PROBATION / proof-present / DA-review-pending` only; `DA-GRADUATED` is written only after DA/Owner
authorization or a DA/Owner-performed merge. Unauthorized self-merge is a process incident even if CI is green.
