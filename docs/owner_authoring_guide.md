# Owner Authoring Guide

Plain-language path for creating or changing a SimThing workplan; machinery names are included so an agent can execute the request.

## Author A Workplan
1. Name the track and set a status header: `OPEN`, `PARKED`, or `CLOSED`.
2. Add ladder rows with `Rung`, `ID`, `Scope`, `Exit proof`, and `Tier`.
3. Stamp the **start** of the `Exit proof` cell only: `NOT STARTED`, `HANDOFF DISPATCHED`, `PROBATION`, `DA-GRADUATED`. Scope narrative must not carry completion stamps.
4. Never put escaped pipes in table cells; use commas or semicolons inside a cell.
5. Record binding conditions at open: owner directives, closure blockers, and net-prose promises.
Ask an orchestrator to scribe the track doc, regenerate orientation, and show the diff before pushing.

## Revise A Workplan
Use plain verbs: `approve`, `hold`, `status`, or `amend: <change>` (also `/handoff approve|hold|status|amend`). Humans do not hand-edit `handoffs/<RUNG>.hd.md` or generated projections. Ruling 6: implementer proof stamps Exit-proof as `PROBATION`; DA graduation stamps are DA-authored at merge.

## Two-Source Pointer Rule
Stamping a ladder Exit-proof cell does **not** move an authoritative `Active open rung` row. At graduation update **both** the Exit-proof stamp and the Active open rung (or `none`), then `bash scripts/ci/gen_orientation.sh`. `--check` FAILs when Active open rung names a graduated or absent rung.

## Open, Park, Or Close
- Open/realign: `bash scripts/ci/gen_orientation.sh --open <track-doc>`.
- Park: `bash scripts/ci/gen_orientation.sh --park <track-doc>` (moves track-scoped rows + in-flight `.hd` into one receipt-stamped EOF block). Refuses a divergent Active open rung (same family as open-PR refusal).
- Reopen: `bash scripts/ci/gen_orientation.sh --unpark <track-doc>` (receipt-validate, restore, remove block, flip pointer).
- `PARKED`/`CLOSED` is the normal admission state for `--open`; HD-6 refuses while outgoing status is `OPEN` and points to `--park`.
- `--force-owner "<directive>"` only for deliberate override; directive is recorded. Closing uses `track_closeout.sh` (unpark first).

## Regenerate The Library
After ladder edits: `bash scripts/ci/gen_orientation.sh`. After anchor edits: `bash scripts/ci/anchor_check.sh --resync`. Catalog: `bash scripts/ci/librarian.sh --catalog --role <coding|orchestrator|da>`. Staleness: `bash scripts/ci/librarian.sh --staleness` (culls need `--confirm`).

## Progression Modes
Manual: Owner checks the SimThing Board, prompts one agent at a time. Automated: DA/orchestrator stack dispatches, routes, remands, and advances with less Owner prompting.

## Browser Stack Protocol
Forward by pointer, not pasted payload: "Check the SimThing Board issue and execute if the latest handoff is yours." Orchestrator writes coder block with rung, receipt, branch, PR title; verify the board still names the same pointer/receipt before sending.

## Coder Surfaces
Use role slots first; products are examples. Coding may run in Cursor cloud, a Grok CLI worktree, or a Claude sonnet-class coding surface. The handoff owns the branch, receipt, checks, and stop conditions.
