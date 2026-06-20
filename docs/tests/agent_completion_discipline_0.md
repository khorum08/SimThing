# AGENT-COMPLETION-DISCIPLINE-0 — prevent cargo-output hangs and require durable final reporting

> **Operational handoff — prepend to every SimThing implementation/remediation prompt.**

## Problem

Recent agent turns have completed and merged PRs, but the interactive session has ended or appeared to hang inside cargo/test terminal output instead of producing a final natural-language summation.

This creates ambiguity for the owner/orchestrator:

- Was the implementation complete?
- Was the PR merged?
- Which tests passed?
- Which tests failed, skipped, or were narrowed?
- What files changed?
- Was evidence lifecycle updated?
- Did any docs remain stale?
- Did cargo actually hang, or did the model just consume the turn with terminal output?

This must stop.

## Rule 0 — Do not let terminal output be the final artifact

Never end the turn with raw cargo output, a stack trace, a long compiler stream, or an unfinished command prompt.

The final visible message must always include a concise completion summary, even if validation failed.

Required final message structure:

```text
Status:
PR:
Merge:
Implemented:
Validation:
Evidence/docs:
Known gaps:
Next recommended action:
```

If a command is still running, stop and report the last known status rather than leaving the user watching terminal output.

## Rule 1 — Write the result report skeleton before running validation

Before running long validation commands, create or update the result report skeleton in `docs/tests/`.

Example: `docs/tests/<rung_name>_results.md`

The report must be created early with these sections:

- Status
- PR / branch / merge
- Current defect or mission
- Implemented changes
- Boundary / constitution checks
- Validation commands
- Files changed
- Evidence lifecycle
- Known gaps
- Deferred next rung
- DA status

Then fill in PASS/FAIL/SKIP/PARTIAL as validation proceeds.

Do not wait until the end of a long cargo run to create the report.

## Rule 2 — Draft the PR summary before validation

Before validation, draft the intended PR summary in the PR body or local notes.

Required PR summary shape:

```text
## Summary
- ...

## Boundary commitments
- No GPU primitive/shader/WGSL changes.
- No sim runtime tick ownership changes.
- No Studio GPU dispatch.
- No new privileged engines.
- No Terran Pirate fixture edits unless explicitly scoped.

## Validation
- PENDING: ...

## Evidence
- docs/tests/<rung_name>_results.md
```

After validation, update `PENDING` entries to PASS/FAIL/SKIP/PARTIAL.

## Rule 3 — Use focused validation first

Run the focused validation matrix first. Do not begin with broad workspace or package-wide tests unless the handoff explicitly requires them.

Preferred order:

```text
cargo fmt --all -- --check
cargo check -p <touched-crate>
cargo test -p <touched-crate> --test <focused_test>
cargo test -p <other-touched-crate> --test <focused_test>
git diff --check
git diff --name-only master...HEAD
```

Only after focused tests pass should broader package tests be run.

If broad tests are requested but likely to be noisy or slow, run them after the focused proof and record their status honestly.

## Rule 4 — Avoid verbose cargo output

Do not run commands with unnecessary verbose flags.

Do not use `--nocapture` unless debugging a specific failing test.

Do not paste long terminal logs into the final response.

For validation reporting, record concise status:

```text
PASS — cargo test -p simthing-spec --test planet_child_location_admission
FAIL — cargo test -p simthing-driver --test owner_silo_gpu_tick
SKIP — cargo test -p simthing-mapeditor --test x; test not added in this rung
PARTIAL — cargo test -p simthing-spec; stopped after focused suite due known environment linker instability
```

If a command fails, capture the shortest relevant error excerpt in the result report.

## Rule 5 — Add time/termination discipline around cargo

If a cargo command appears to hang, do not let the whole turn disappear into it.

- Prefer focused test binaries over full package tests.
- Stop broad test expansion once required proof is satisfied.
- If a command runs abnormally long, terminate it and report PARTIAL with reason.
- Record the last completed command and the command that hung.
- Do not merge if required focused validation is still unknown.

When using an environment that supports shell timeouts, wrap broad commands with a reasonable timeout and record timeout as PARTIAL/FAIL depending on whether that command was required.

Example status language:

```text
PARTIAL — cargo test -p simthing-spec did not complete before timeout after focused planet/local-grid tests passed. No failing assertion was observed before termination. Full package test remains deferred.
```

Do not claim PASS for a timed-out command.

## Rule 6 — Keep the evidence lifecycle clean

Before opening or merging a PR, classify touched evidence:

| Path | Role |
|------|------|
| `docs/tests/current_evidence_index.md` | LIVE_LEDGER |
| `docs/tests/<new_results>.md` | PROBATION or PARTIAL |
| `docs/0.8.3 Simthing Studio Production.md` | living production synthesis |
| `scenarios/corpus/*.simthing-scenario.json` | durable corpus fixtures |
| scratch logs/temp files | delete unless intentionally promoted |

Required actions:

- Save new result reports in `docs/tests/` only if needed for visibility.
- Delete scratch logs, temporary scenario files, failed-load samples, temp configs, generated sample files, and screenshots not used as evidence.
- Do not leave contradictory duplicate reports.
- Do not delete live ledger files.
- Do not delete DA ruling files.
- Do not DA-promote without owner approval.
- Update production synthesis in the same PR when the rung changes doctrine or product status.
- Update `current_evidence_index.md` in the same PR.

## Rule 7 — PR body must be updated before merge

Before merging, update the PR body with final validation status.

Required:

- PR number if known.
- Evidence report path.
- Validation command list with PASS/FAIL/SKIP/PARTIAL.
- Known deferred items.
- Explicit boundary commitments.

Do not merge a PR whose body still says only "TBD," "PENDING," "branch evidence," or placeholder merge information.

If merge SHA is unknown until after merge, update `current_evidence_index.md` and result report in a follow-up doc commit, but explicitly record that it remains pending until filled.

## Rule 8 — Final response must be produced after merge or failure

After merge, final response must include:

```text
Status: merged / open / failed / partial
PR: #...
Merge: <sha>
Implemented:
Validation:
Docs/evidence:
Known gaps:
Next recommended action:
```

If not merged:

```text
Status: not merged
Branch:
Blocking issue:
Last completed validation:
Next action:
```

## Rule 9 — Never hide uncertainty

Allowed:

- "Focused tests passed; full package test was not run."
- "Full package test timed out; no failure observed before timeout."
- "PR merged but merge SHA still needs evidence-index cleanup."
- "I did not run GPU adapter tests because this rung did not touch GPU surfaces."

Forbidden:

- Claiming full validation passed when only focused tests ran.
- Treating timeout as PASS.
- Treating cargo output ending the session as a final report.
- Saying a PR is clean if evidence docs still contain branch placeholders.

## Rule 10 — Apply on every implementation handoff

Before implementation, create the result report skeleton and draft PR summary. Run focused validation first. Keep cargo output concise. If cargo hangs, terminate and record PARTIAL honestly. Before merge, update the PR body, production doc (if doctrine changed), evidence index, and result report. After merge or failure, produce a final structured summary; do not end the turn in terminal output.

## Completion checklist

Before ending the turn, confirm:

- [ ] Result report exists or an explicit reason is recorded for not creating one.
- [ ] PR body has final validation status.
- [ ] Production doc was updated if product/constitution status changed.
- [ ] Evidence index was updated.
- [ ] Scratch/temp artifacts were deleted.
- [ ] Validation commands are recorded with PASS/FAIL/SKIP/PARTIAL.
- [ ] Known gaps are listed.
- [ ] Final response is a human-readable summary, not raw cargo output.