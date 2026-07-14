# HD-POINTER-LIFECYCLE-GATE-0 — results

**Status:** PROBATION (implementer). DA authors the graduation stamp at merge (HD ruling 6).
**PR / branch / merge:** branch `cursor/hd-pointer-lifecycle-gate-0`; PR [#1344](https://github.com/khorum08/SimThing/pull/1344); merge <pending, DA>.
**Clearance:** `CLEARANCE-VERDICT: DA-RESERVE(gate-wiring)` · `body_sha: evidence-tail` · `DA-TREEVERIFY-PROFILE: DEEP-TREE` · `REQUIRED-ANCHORS: orientation-harness-core`.
**HD-RECEIPT:** caab38a976d2 · **ORIENT-RECEIPT:** ada87881548c (coding)
**tested_code_sha:** `TESTEDSHA` · **coverage_basis:** PASS · **ci_green:** PASS

## What changed (gate-wiring; no crates/**, no dispatch/lint logic, no new tables/verdict lexicon)
- `gen_orientation.sh --open`: **refuses** the pointer flip (`FAIL(outgoing-track-open)`, no writes) unless the
  OUTGOING track doc's status header declares CLOSED/PARKED — reads only the state token after `Status:` (not
  trailing prose). Fires only on a real move off a live workplan; `none`/missing/same-doc realign unaffected.
- **Coherent-root enforcement (remand 1):** the active-pointer file, orientation, and owner-directives table must
  all resolve under `ORIENTATION_REPO_ROOT`, else `FAIL(incoherent-root)` before any write. Closes the cross-root
  env-seam bypass (fake PARKED root cannot authorize a victim write). No new bypass flag.
- **Forced open is ONE transactional operation (remand 2 + follow-ups):** a zero-write preflight **plans the
  complete post-op `owner_directives.tsv` bytes** with full semantic validation — 4 fields; status exactly
  active/retired; nonempty directive/scope/set_by; no duplicate active `(directive, scope)`; normalized planned
  row — plus every mutation target's type/writability. Commit writes **exactly those admitted bytes** (no
  reconstruct) and **rolls back all four surfaces to original bytes/existence on any failure**. Pointer transition
  and Owner row commit together. `--force-owner` without `--open` rejected. Normal `--open` unchanged.
- `clearance.yml`: board sync also fires on **push to master** (board-json + SimThing Board issue); PR-ingress
  skipped on push. `current_handoff: none` (#1342) already supported.
- Selftest harness: sandbox seeds + overrides `owner_directives.tsv` (`ORIENTATION_OWNER_DIRECTIVES`) so force fixtures never touch the real file.

## Load-bearing proofs (+ what each catches)
- `gen_orientation.sh --selftest` → PASS (23), 10 gate fixtures incl.: `gate_open_refused` (no write),
  `gate_parked_closed_allowed`, `gate_force_owner_records`, `gate_incoherent_root` (remand 1), `..._invalid_target`,
  `..._requires_open`, `..._unwritable_directive`, `..._invalid_status_row` + `..._duplicate_active_pair` (planned-
  bytes semantics reject before writes), and **`..._rollback_after_writes`** — a deterministic selftest-only fault
  (`ORIENTATION_FORCE_FAULT_AFTER_WRITES=1`, fail-only, no bypass) fires on the real `commit_forced_open` path
  AFTER the pointer+orientation writes → `FAIL(forced-transition-aborted)` + all four surfaces byte/existence-equal.
- Falsifier bite (rot): disabling rollback leaves the post-write fault half-applied (pointer MUTATED, orientation
  WRITTEN); disabling `assert_coherent_open_root` lets the cross-root attack mutate the victim. Both are load-bearing.
- Live gate: outgoing `docs/design_0_0_8_4_8_4_hd_board.md` (OPEN) → `--open <other>` FAILs, `active_track.txt`
  unchanged; same-doc realign still `OPENED`. Board `current_handoff: none` renders (exit 0).
- `gen_orientation.sh --check`, `agent_scan` (delta_inspect=0), `doc_budget_check --check`, `relay_lint --selftest` (36): all PASS.

## Scope Ledger
Classification: gate-wiring. Specified = implemented. No `crates/**`, Studio/UI, dispatch/lint logic, new tables,
or new verdict lexicon (new details: `FAIL(outgoing-track-open|incoherent-root|forced-preflight|forced-transition-aborted)`).
Stop-conditions clear: no stale-orient-receipt (`ORIENT-RECEIPT: ada87881548c` valid on master), no scope-widening,
no gate-weakening (`--force-owner` is the only bypass, coherent-root-bound + transactional; fixtures + rot prove it).

## Conformance
Anchor `orientation-harness-core` ACKed (`@8a365d1c0864`). Owner directive HD-STACK-CADENCE-0 honored (one
automated rung). Board freshness reuses existing board-sync commands.

## Known gaps / next
Live post-merge push board re-render is proven only once merged (pre-merge: command-level proof). Next: DA deep
pass → DA-authored stamp on the HD-6 cell → merge; then HD-7 (Owner review between rungs, cadence).
