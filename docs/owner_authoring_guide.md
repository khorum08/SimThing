# Owner Authoring Guide

Plain-language path for creating or changing a SimThing workplan; machinery names are included so an agent can execute the request.

## Author A Workplan
1. Name the track and set a status header: `OPEN`, `PARKED`, or `CLOSED`.
2. Add ladder rows with `Rung`, `ID`, `Scope`, `Exit proof`, and `Tier`.
3. Put the stamp at the start of the `Exit proof` cell: `NOT STARTED`, `HANDOFF DISPATCHED`, `PROBATION`, `DA-GRADUATED`.
4. Never put escaped pipes in table cells; use commas or semicolons inside a cell.
5. Record binding conditions at open: owner directives, closure blockers, and net-prose promises.
Ask an orchestrator to act as scribe, edit the track doc, regenerate orientation, and show the diff before pushing.

## Revise A Workplan
Use plain verbs: `approve`, `hold`, `status`, or `amend: <change>`. After HD-3, the same verbs work as `/handoff approve|hold|status|amend` comments.
Nobody hand-edits generated projections or asks an implementer to invent a handoff from chat prose.
Ruling 6: amendments and implementer proof stamp the exit-proof cell as `PROBATION`; DA graduation stamps are DA-authored at merge.

## Open, Park, Or Close
- Open or realign with `bash scripts/ci/gen_orientation.sh --open <track-doc>`.
- HD-6 blocks a new active pointer while the outgoing track is still `OPEN`.
- A forced escape uses `--force-owner "<directive>"` and records the owner directive.
- Closing uses `track_closeout.sh`; it leases handoffs and result docs before reaping.

## Regenerate The Library
- After ladder edits: `bash scripts/ci/gen_orientation.sh`.
- After anchored doctrine edits: `bash scripts/ci/anchor_check.sh --resync`.
- Reachable doctrine: `bash scripts/ci/librarian.sh --catalog --role <coding|orchestrator|da>`.
- Aging/dead rows: `bash scripts/ci/librarian.sh --staleness`; culls dry-run unless `--confirm` is explicit.

## Progression Modes
Manual mode: the Owner checks the SimThing Board, then prompts one agent at a time to render and execute the active handoff.
Automated mode: the DA/orchestrator stack dispatches, routes, remands, and advances between rungs with less Owner prompting.

## Browser Stack Protocol
Forward by pointer, not pasted payload: "Check the SimThing Board issue and execute if the latest handoff is yours." The orchestrator writes the coder block with rung, receipt, branch, and PR title; before sending, verify the board still names the same pointer and receipt.

## Coder Surfaces
Use role slots first; products are examples. Coding may run in Cursor cloud, a Grok CLI worktree, or a Claude sonnet-class coding surface. The handoff owns the branch, receipt, checks, and stop conditions.
