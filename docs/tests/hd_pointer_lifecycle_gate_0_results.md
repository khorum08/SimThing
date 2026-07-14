# HD-POINTER-LIFECYCLE-GATE-0 — results

**Status:** PROBATION (implementer). DA authors the graduation stamp at merge (HD ruling 6).
**PR / branch / merge:** branch `cursor/hd-pointer-lifecycle-gate-0`; PR [#1344](https://github.com/khorum08/SimThing/pull/1344); merge <pending, DA>.
**Clearance:** `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` · `body_sha: evidence-tail` · `DA-TREEVERIFY-PROFILE: DEEP-TREE` · `REQUIRED-ANCHORS: orientation-harness-core`.
**HD-RECEIPT:** caab38a976d2 · **ORIENT-RECEIPT:** ada87881548c (coding)
**tested_code_sha:** `fb1a71665a422e633b3d01c2803517075e962a09` · **coverage_basis:** PASS · **ci_green:** PASS

## What changed (gate-wiring; no crates/**, no dispatch/lint logic changes, no new tables/verdict lexicon)
- `gen_orientation.sh --open`: **refuses** the pointer flip (`ORIENTATION-OPEN-VERDICT: FAIL(outgoing-track-open)`,
  no writes) when the OUTGOING active track doc's status header does not declare CLOSED/PARKED. The check reads
  only the status **state token** after `Status:` (up to the first delimiter), so trailing prose that mentions
  another track being "parked" (e.g. the HD board line) does not falsely pass. Fires only on a real pointer
  move off a live workplan; `none`/missing/same-doc realign are unaffected (existing selftests intact).
- **Coherent-root enforcement (remand 1):** `--open` requires the active-pointer file, generated orientation,
  and owner-directives table to all resolve under `ORIENTATION_REPO_ROOT` — else `FAIL(incoherent-root)` before
  any write. Closes the cross-root env-seam bypass (a fake root's PARKED outgoing must not authorize a write to
  a victim checkout's pointer/orientation). No new bypass flag; sandbox paths stay coherent under their root.
- `--force-owner "<directive>"`: **two-phase (remand 2)** — the outgoing/target/classification are admitted with
  ZERO writes first; only after the pointer+orientation transition succeeds is the `owner_directives.tsv` row
  (`<directive>` / outgoing track id / active / Owner-<date>) recorded (`ORIENTATION-OPEN-FORCE-OWNER: recorded`).
  A forced open that fails classification leaves no stray directive. `--force-owner` without `--open` is rejected.
- `clearance.yml`: board sync also fires on **push to master** (renders board-json + updates the SimThing Board
  issue); PR-ingress steps (clearance_check, sticky, ingress) are skipped on push. Kills the post-merge stale
  window. Board render with `current_handoff: none` is the #1342 class (already supported).
- Selftest harness: sandbox now seeds + overrides `owner_directives.tsv` (`ORIENTATION_OWNER_DIRECTIVES`) so the
  force-owner fixture never touches the real file.

## Load-bearing proofs (+ what each catches)
- `gen_orientation.sh --selftest` → PASS (19) incl. 6 gate fixtures: `gate_open_refused` (flip-from-OPEN FAILs,
  no write), `gate_parked_closed_allowed` (PARKED+CLOSED proceed — catches over-blocking), `gate_force_owner_records`
  (override proceeds AND records the row), `gate_incoherent_root` (cross-root attack FAILs, victim untouched —
  remand 1), `gate_force_owner_invalid_target` (forced open to a non-workplan target FAILs with zero writes /
  no stray directive — remand 2), `gate_force_owner_requires_open` (force without `--open` rejected).
- Falsifiers bite (rot test): with the coherence guard removed, the cross-root attack succeeds (victim pointer
  MUTATED, orientation WRITTEN, `CREATED`) — the guard is load-bearing.
- Live gate: outgoing = `docs/design_0_0_8_4_8_4_hd_board.md` (OPEN) → `--open <other>` FAILs, `active_track.txt`
  unchanged; same-doc realign still `OPENED`.
- Board render `current_handoff: none`: `--board-json` (no handoff) → `--render-board` exit 0 (post-merge push
  re-render is the live proof).
- `gen_orientation.sh --check` PASS (generate path unchanged); `agent_scan`, `doc_budget_check --check`,
  `relay_lint.sh --selftest` (36) all PASS.

## Scope Ledger
Classification: gate-wiring. Specified = implemented. No `crates/**`, Studio/UI, `handoff_dispatch.sh` logic
(board-sync reused only), new tables, or new verdict lexicon (the one new detail is the `--open` gate's
`FAIL(outgoing-track-open)`). Stop-conditions clear: no stale-orient-receipt (`ORIENT-RECEIPT: ada87881548c`
still valid on master), no scope-widening, no gate-weakening (`--force-owner` is the only bypass; fixtures prove refusal).

## Conformance
Anchor `orientation-harness-core` ACKed (`@8a365d1c0864`). Owner directive HD-STACK-CADENCE-0 honored: one
automated rung; no other rungs started. Board freshness reuses existing board-sync commands.

## Known gaps / next
Live post-merge push board re-render is proven only once merged (pre-merge: command-level proof). Next: DA deep
pass → DA-authored graduation stamp on the HD-6 cell → merge; then HD-7 (Owner review between rungs, cadence).
