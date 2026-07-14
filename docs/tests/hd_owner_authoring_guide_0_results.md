# HD-OWNER-AUTHORING-GUIDE-0 Results

Status: PROBATION / proof-present / DA-review-pending

## Identity
- PR: #1349; branch `coder/hd-owner-authoring-guide-0`; handoff `handoffs/HD-OWNER-AUTHORING-GUIDE-0.hd.md`.
- `HD-RECEIPT: 91fa0e4a034b`; `ORIENT-RECEIPT: ada87881548c`.
- `tested_code_sha`: final PR head is mirrored in the PR body after push; this committed file cannot contain its own hash.
- `coverage_basis: PASS`; `ci_green: PASS` for local required battery.

## What Changed
- Added capped Owner guide: author/revise/park/reopen/close, regenerate, progression modes, board-pointer browser flow, coder surfaces.
- Role-slot sweep: onboarding headings and template routing are role/capability first; vendor/tool names are examples only.
- Added Cursor cloud `python-is-python3` caveat with compression; HD-8 PROBATION stamp + orientation regen + evidence row.

## Cold-Reader Walkthrough
| step | from guide | live cross-check |
|---|---|---|
| author | set `OPEN/PARKED/CLOSED`; ladder cells lead with stamps; no escaped pipes; bindings at open | HD board row uses lead stamp; table cells avoid escaped pipes |
| revise | use `approve/hold/status/amend` or `/handoff ...`; orchestrator scribes `.hd`; no projection hand edits | handoff_dispatch renders projections; owner verbs exist from HD-3 |
| park/reopen | scribe sets outgoing status `PARKED` or `CLOSED`; regenerate; run `gen_orientation.sh --open <next>` | HD-6 gate admits PARKED/CLOSED and refuses OPEN unless forced |
| force | `--force-owner "<directive>"` only for deliberate override | owner_directives.tsv records the escape |
| regenerate/library | `gen_orientation.sh`; `anchor_check.sh --resync`; `librarian.sh --catalog/--staleness` | HD-7 catalog + staleness machinery green |
| progress | manual = Owner prompts from board; automated = DA/orchestrator advances unattended | board pointer/receipt prompt used for this rung |

## Role-First Audit
| surface | evidence |
|---|---|
| onboarding headings | `Coding role`, `Orchestration role`, `DA role` |
| template routing | `coding role`, `docs-capable role`, `DA role`; no vendor-conditioned recipient |
| search hits | `rg "Grok|Cursor|Codex|Fable|Claude|vendor|model|worktree|python-is-python3"`: examples/caveats only |
| dispositions | coder surface examples retained; no workflow, script, doc heading, or `.hd` field conditions on a vendor |

## Prose And Cap Accounting
| file | add | del | net | cap before | cap after | compression |
|---|---:|---:|---:|---:|---:|---|
| `docs/owner_authoring_guide.md` | 39 | 0 | +39 | new | 40 | capped at birth |
| `docs/agent_onboarding.md` | 9 | 10 | -1 | 150 | 150 | examples folded to one line |
| `docs/handoff_template.md` | 3 | 3 | 0 | 112 | 112 | routing bullet folded |
| `docs/agents.md` | 2 | 2 | 0 | 190 | 190 | VM caveats merged |
| board/orientation/index/budget | 5 | 3 | +2 | live | live | generated/status rows |
| this result | 56 | 0 | +56 | none | <=60 | compact tables |

Net +96 including evidence; justified by the requested Owner guide and capped at birth.

## Battery
- `bash scripts/ci/agent_scan.sh` PASS.
- `bash scripts/ci/gen_orientation.sh --check` PASS.
- `bash scripts/ci/doc_budget_check.sh --check` PASS.
- `bash scripts/ci/relay_lint.sh --selftest` PASS.

## Scope Ledger
Classification: gate-wiring docs. Docs + `doc_budget_baseline.tsv` only. No `crates/**`, Studio/UI, script logic, workflows, new tables, or doctrine restatement.

## Known Residue
- DA authors any graduation stamp at merge; this PR remains draft, PROBATION, and unmerged.
