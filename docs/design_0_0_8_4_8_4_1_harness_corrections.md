# 0.0.8.4.8.4.1 — Harness Corrections

> **Status: OPEN / Owner-directed (2026-07-16).** Short corrective track off the HD Board substrate
> (0.0.8.4.8.4, CLOSED). Opened via `gen_orientation.sh --open` (verdict OPENED, entry rung HC-1);
> 0.0.8.6 is PARKED (receipt `19e0e85c8d3f`) and resumes via `--unpark` when this track parks/closes.
>
> **Why.** Five harness defects surfaced in the first post-HD production rung
> (`STUDIO-FLEET-PRESENCE-READOUT-0`, #1355) and in the HD closeout. Each one lets a **false green**
> or **silent cruft** through gates whose entire promise is that green means proven. All four are
> mechanical, share two surfaces (`scans.tsv` + `gen_orientation.sh`/`track_closeout.sh`), and were
> each caught by a human/DA read rather than by the harness — which is the defect.
>
> **Roles:** DA = Fable (or step-in). Orchestrator = webchat slot. Coder = Owner-assigned slot.
> (Ruling 7: roles are slots, models are data — no rung may condition on a vendor.)
> **Mode:** manual-progression (HD-STACK-CADENCE-0) — one rung at a time, Owner review between.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base:** `simthing_core_design.md` §1.2 (admission ladder: type > admission hard-error >
guard scan > prose) · `design_0_0_8_3.md` §0 · this file · `ci_screening_surface.md` ·
`docs/handoff_template.md` §H (anti-kabuki) · `track_closeout_protocol.md` · `owner_authoring_guide.md`.

**Held decisions — do NOT re-derive:**

- **Green must mean proven.** A gate an implementer can silence without review is not a gate.
- **Growth without retirement = regression**; TSV growth is the primary fail state.
- **Doctrine as data, not prose** — a rule only a DA read can catch belongs one rung lower on the
  admission ladder.
- No new standalone tables; compose existing readers/writers. No clearance-router lexicon change.
- **Net scan ledger ≤ 0 — with one carve-out (DA, 2026-07-16):** promoting a *prose* rule down to a
  guard scan is the admission ladder's intended direction and retires nothing, so HC-2 may add its
  scan without a pairing. The obligation it does carry: the prose rule stops being the enforcement
  and becomes a pointer to the scan. Ledger growth by any other means still needs retirement.
- Every mutation transactional (HD-6 preflight/staged/rollback pattern) with a rollback fixture.

**Binding conditions (record at open):**

| rung | condition | status |
|---|---|---|
| HC-TRACK-OPEN-0 | blocked-until-owner-directs-open-and-0.0.8.6-parked | discharged (Owner 2026-07-16; 0.0.8.6 parked receipt 19e0e85c8d3f) |
| HC-CLOSEOUT-0 | every rung lands a falsifier that FAILS on the pre-fix tree (prove-the-guard-bites) | active |

---

## 1. Root cause this track closes

The rustification moved verification from prose to machines. These five holes let the machine layer
report green on unproven work — the exact class the harness exists to kill.

| # | Defect | How it produced false state | Evidence |
|---|---|---|---|
| 1 | Generic in-code scan-exclusion token | `role-resolution-exclude-site` voids a `SPEC-LOWERER-KIND-READ` HEURISTIC finding with no DA review, no ledger row, no trace | #1355: 2 findings silenced; stripping the 2 comments flips `agent_scan` `PASS delta_inspect=0` → `INSPECT delta_inspect=2`. Named exclusions beside it (`planet_non_grid_child_kind_label`…) require a gate-wiring `scans.tsv` edit — the asymmetry is the hole |
| 2 | Anti-kabuki §H rule 2 is prose-only | A source-scanning guard `pub fn` shipped in a crate's **public API**, self-evading via `format!("{}{}", "TP_FLEET_", …)`, inspecting only its own file — passed every gate green | #1355: caught by DA read alone; no scan detects the shape |
| 3 | Closeout never reaps discharged binding rows | Closed tracks' conditions accrete forever | Post-HD-C: **10/10** rows in `binding_conditions.tsv` are `discharged` from CLOSED tracks (TP×4, HU×2, OC×2, HD×2) — the table is 100% dead rows |
| 5 | Ladder rows silently column-shift | `parse_rungs()` bounds too-few columns but never too-many; an escaped or bare pipe in any cell shifts `parts[3]` off the Exit proof, so stamps never register and the pointer sticks — `--check` stays green | Hit 3× by the DA (HD-1 `body_sha` cell, OC ladder, and this very workplan while documenting the `Active open rung` row); caught by human memory each time, never by a gate |
| 4 | No pointer-divergence lint | The authoritative `Active open rung` row can name a rung whose exit-proof is `DA-GRADUATED`; `--check` passes | 12.4 (#1355): ladder stamped DA-GRADUATED while the pointer row still named it — `gen_orientation --check: PASS`. Two pointer sources (authoritative row vs ladder scan) with no agreement check |

---

## 2. PR ladder (all gate-wiring / DA-reserve; Std tier)

| Rung | ID | Scope | Exit proof | Tier |
|---|---|---|---|---|
| HC-1 | `HC-EXCLUSION-REVIEW-GATE-0` | **Close the self-service suppression hole (defect 1). Owner ruling 2026-07-16: DELETE the generic token outright — no pairing option, no either/or.** Remove `role-resolution-exclude-site` from the `SPEC-LOWERER-KIND-READ` exclusion column in `scans.tsv`; exclusions become DA-authored named symbols only, so every exclusion costs a reviewed gate-wiring edit. Sweep every `scans.tsv` exclusion column for other generic (non-symbol) tokens and delete those too, reporting the census. The two `fleet_presence.rs` kind reads stay as accounted INSPECTs (already justified via #1355) — deletion must not silently re-suppress them. Falsifier: a fixture site bearing the token is scanned, not excluded. | **DA-GRADUATED / merged [#1363](https://github.com/khorum08/SimThing/pull/1363)** — merged 2026-07-16T03:0xZ (dispatched_at 00:50Z; roles: orch=webchat-slot coder=Owner-assigned da=Fable; mode=manual-progression). DA deep pass on `f67a5bf2`: **falsifier proven to BITE by the DA, not relayed** — selftest green at head; restoring the pre-fix `scans.tsv` yields `SPEC-LOWERER-KIND-READ (role_resolution_exclude_site_kind_param_match) FAIL (count=0)` + `DOCTRINE-SELFTEST-VERDICT: FAIL`, i.e. the token silences the site on the old tree exactly as charged. Generic token deleted; census covers every exclusion column; the 2 `fleet_presence.rs` INSPECTs stay accounted and unsuppressed; trap identity preserved in place (M, not rename) with the sample generated in the selftest sandbox — zero durable ledger growth, `test_inventory.tsv` absent from the diff; no crates/gen_orientation/router touched; live 0.0.8.6 `--unpark` re-proved by the DA (receipt `19e0e85c8d3f`, 1 row + 1 handoff restored). Evidence `docs/tests/hc_exclusion_review_gate_0_results.md`. | Std |
| HC-2 | `HC-GUARD-KABUKI-TRIPWIRE-0` | **Mechanize anti-kabuki rule 2 (defect 2).** Add a HEURISTIC scan (`scans.tsv` row + selftest) catching bespoke source-scanning guards: production `pub fn` taking `source: &str`/`&Path` that string-scans, and `include_str!("../src/` in tests. Routes to INSPECT + triage (not FAIL) — legitimate cases exist and must be justifiable, not silenced. Prose rule stays; the tripwire is its mechanized rung-3. | **DA-GRADUATED / merged [#1365](https://github.com/khorum08/SimThing/pull/1365)** — merged 2026-07-16T13:3xZ (dispatched_at 03:17Z; roles: orch=webchat-slot coder=Owner-assigned da=Fable; mode=manual-progression). DA deep pass on `7199eda3`: **falsifier proven to BITE by the DA** — removing the scan row FAILs all 3 controls (scan=MISSING), restored PASS; not green-both-ways. HEURISTIC/INSPECT-only, never FAIL; zero durable ledger growth (controls sandbox-generated, `test_inventory.tsv` absent); prose §H rule 2 demoted to a scan pointer; live 0.0.8.6 `--unpark` re-proved (`19e0e85c8d3f`). **The scan immediately earned its keep — it caught 2 pre-existing kabuki guards** (`simthing-gpu` dead uncalled API; `simthing-mapeditor` the identical #1355 self-scan shape). DA correction: coder marked both 'intentional/keep'; re-labeled as REMEDIATION CANDIDATES in triage — surfaced, not blessed. Remediation deferred (crates/** is out of HC-2's detection-only scope) → follow-up rung proposed. Evidence `docs/tests/hc_guard_kabuki_tripwire_0_results.md`. | Std |
| HC-3 | `HC-CLOSEOUT-BINDING-REAP-0` | **Reap discharged rows at close (defect 3).** `track_closeout.sh` removes the closing track's discharged `binding_conditions.tsv` rows as part of `--apply`, reported in the CLOSEOUT-RECEIPT. Retire the 10 existing dead rows from closed tracks in the same PR (evidence: the table is currently 100% discharged). Must not touch rows of open/parked tracks — the 0.0.8.6 park block round-trip (`--unpark`) stays byte-exact. | **DA-GRADUATED / merged [#1368](https://github.com/khorum08/SimThing/pull/1368)** — merged 2026-07-16T15:0xZ (dispatched_at 14:24Z; roles: orch=Codex coder=Grok-CLI(grok-4.5) da=Fable; mode=FULLY-AUTOMATED — first HC rung with the Grok CLI coder surface, DA-driven headless in a persistent session). DA deep pass on `9d985107`, all reproduced by the DA: binding_conditions.tsv 12→2 exact (removed precisely TP×4/HU×2/OC×2/HD×2; survivors HC-TRACK-OPEN-0 + active HC-CLOSEOUT-0); `--prove` fixtures all bite — **falsifier proven (pre-fix path LEAVES the closed row, fixed reaps it — not green-both-ways)**, open-track negative control + active row spared, HD-6 rollback restores byte-exact, CLOSEOUT-RECEIPT carries `binding_reaped`; full battery green; live 0.0.8.6 `--unpark` restores receipt 19e0e85c8d3f (1 row + 1 handoff); forbidden surfaces untouched. Evidence `docs/tests/hc_closeout_binding_reap_0_results.md`. | Std |
| HC-4 | `HC-POINTER-DIVERGENCE-LINT-0` | **One truth for the pointer (defect 4).** `gen_orientation.sh --check` FAILs when the authoritative `Active open rung` row names a rung whose exit-proof cell already carries a graduation/finished stamp (the same `is_completed_exit` markers), or names a rung absent from the ladder. (Meta: this very cell must state that WITHOUT tripping the pointer parser — an early lesson HC-4 mechanizes.) Fixtures: graduated-rung-named-as-pointer FAILs; unknown-rung FAILs; legitimate not-yet-dispatched next rung passes; `none`-form passes. Document the two-source rule in `owner_authoring_guide.md` (stamping a cell does not move an authoritative pointer). | **DA-GRADUATED / merged [#1370](https://github.com/khorum08/SimThing/pull/1370)** — 2026-07-16 (orch=Codex coder=Grok-CLI(grok-4.5) da=Fable; fully-automated). DA deep pass on `6a3f0994`: all 27 selftest fixtures pass incl. the 5 divergence falsifiers + park-refuses-divergent + scope-cell-not-false-complete; root-cause fix verified — `next_rung_pointer` drops the `or is_completed_exit(deliv)` that skipped HC-3's pointer; battery green; live 0.0.8.6 --unpark 19e0e85c8d3f. Two-source rule documented. Evidence `docs/tests/hc_pointer_divergence_lint_0_results.md`. | Std |
| HC-5 | `HC-LADDER-COLUMN-INTEGRITY-0` | **Assert the parser's own invariant (defect 5: silent column shift).** `parse_rungs()` splits ladder rows on the pipe character (even when backtick-wrapped), skips rows with *too few* columns, and **never bounds too many** — so any cell containing an escaped pipe (the only way markdown renders a literal pipe) or a bare pipe silently shifts every column right: `parts[3]` reads the Scope tail while the real Exit proof lands at `parts[4]` and is discarded. Stamps then never register and the pointer sticks on finished work, with `--check` green. **Fix inside the existing pass — no new script, no new workflow (Owner, 2026-07-16):** `is_ladder_header` already identifies each table's shape; capture its declared column count and FAIL in `gen_orientation.sh --check` when any data row of that table does not match it exactly, naming the row and the remedy (say it without a pipe — backticks do not help; a bare pipe splits too). Same surface as HC-4. Falsifier (ruling 3): a fixture ladder row bearing an escaped pipe in its Scope cell parses to the WRONG exit proof with `--check` PASS on the pre-fix tree, and FAILs after. | **DA-GRADUATED / merged [#1372](https://github.com/khorum08/SimThing/pull/1372)** — 2026-07-16 (orch=Codex coder=Grok-CLI(grok-4.5) da=Fable; fully-automated). DA deep pass on `9c37f50a`: 28 selftest fixtures incl. the bite-proof `pre_fix_misreads_exit` (pre-fix reads exit='still scope' from shifted cols) + escaped-pipe/bare-pipe FAIL + clean-row pass; asserts at the parse_rungs choke point only (no repo-wide scan; closed-track ladders unreddened); battery green; live 0.0.8.6 --unpark 19e0e85c8d3f. Evidence `docs/tests/hc_ladder_column_integrity_0_results.md`. | Std |
| HC-6 | `HC-HORIZON-ENTRY-CONVENTION-0` | **Distinguish future-facing parked API from kabuki (Owner-directed 2026-07-16).** The Owner lays down consumerless API ahead of a consumer (horizon entry points); these may self-reference and look like kabuki until the consumer arrives, and must NOT be deleted. Define a greppable, DATED, self-declaring marker — `HORIZON-ENTRY(<iso-date>): <intended consumer / design ref>` — that a symbol carries to affirm future intent. The `GUARD-KABUKI-TRIPWIRE` scan (and dead-code sense) EXEMPTS a symbol bearing a well-formed FRESH marker, so laying down future API no longer trips the tripwire — but the exemption is greppable, dated and lifecycle-assessed, NOT the silent self-service door HC-1 deleted. Wire `--park` / `--unpark` / `track_closeout` (and the librarian staleness gauge) to assess markers: one older than the staleness window with still no consumer FLAGS to INSPECT (stale or superseded → deletion candidate; a human decides, never auto-delete). Falsifier (ruling 3): a fresh-marked consumerless fn is exempt; the same fn unmarked, or marked with a stale date, is flagged — proven to differ on the pre-fix tree. | **PROBATION / proof-present / DA-review-pending** — implementer proof on branch `coder/hc-horizon-entry-convention-0` (`HD-RECEIPT: 9661c3391deb`; roles: orch=Codex coder=Grok-CLI(grok-4.5) da=Fable; mode=fully-automated). Evidence `docs/tests/hc_horizon_entry_convention_0_results.md`. DA stamps graduation at merge. | Std |
| HC-7 | `HC-KABUKI-GUARD-REMEDIATE-0` | **Delete the two flagged kabuki guards (Owner-directed 2026-07-16; DA-assessed as kabuki, not horizon — neither carries an HC-6 marker).** `simthing-gpu::scan_for_forbidden_validation_tokens` is DEAD (zero callers); `simthing-mapeditor::observe_module_source_forbids_workshop_residue` is the identical #1355 self-scan shape (test feeds it `include_str!` of its own source). Delete both fns + their `lib.rs` re-exports + the Studio test's self-scan assertion (keep the rest of that test); if either invariant is worth keeping it returns as a real admission-type/test, never a self-scan. RETIRE the two `GUARD-KABUKI-TRIPWIRE` triage + justification rows HC-2 added (net ledger −4 — retirement). Falsifier: build green after removal and the tripwire's live-hit count drops 2 → 0. Unlike HC-1/HC-2, `crates/**` edits are IN scope here — this is the remediation rung. | NOT STARTED | Std |
| HC-C | `HC-CLOSEOUT-0` | Measured close: each rung's falsifier demonstrated to FAIL on the pre-fix tree; `binding_conditions.tsv` row count strictly decreased; no new tables; net scan ledger ≤ 0 with retirement pairing; discharge bindings; close via `track_closeout.sh`. | NOT STARTED | DA |

**Sequencing:** HC-1 → HC-2 (∥ HC-3) → HC-4 → HC-5 → HC-6 → HC-7 → HC-C. HC-1/HC-2 share `scans.tsv`;
HC-3/HC-4/HC-5 share the lifecycle scripts (HC-4 and HC-5 both extend `gen_orientation.sh --check`, so
they sequence). HC-6 (the horizon marker) precedes HC-7 (the remediation) so the deletion's criterion —
'no horizon marker present' — already exists, and HC-7 doubles as HC-6's live proof (the two guards are
correctly NOT exempted). Each rung is independently valuable and may re-park.

---

## 3. Standing DA rulings

1. **A gate an implementer can silence without review is not a gate.** (Owner, 2026-07-16.) Exclusions
   are **DA-authored named symbols in `scans.tsv`** — one path, always gate-wired. There is no
   self-service exclusion door and no in-code token that voids a finding; an implementer who believes
   a site is legitimate accounts for it as an INSPECT and lets the DA rule.
2. **Suppression is not accounting.** HEURISTIC findings route through triage; the finding stays
   visible so the scan's retire-condition can observe the site.
3. **Prove the guard bites.** Every rung lands a falsifier that FAILS on the pre-fix tree; a fixture
   that passes before and after proves nothing (HC-CLOSEOUT-0 binding).
4. Rules that only a DA read can catch belong one rung lower on the admission ladder — mechanize or
   consciously accept the prose tier and say so.
5. This track adds no tables and no lexicon; it composes existing surfaces.
6. **Kabuki vs horizon (Owner, 2026-07-16).** Hunt and delete kabuki aggressively — but future-facing
   parked API laid down ahead of a consumer is NOT kabuki; it is legitimate and may self-reference until
   the consumer arrives. The difference is an AFFIRMATIVE, DATED, self-declared HORIZON-ENTRY marker
   (HC-6): kabuki is unmarked self-referential scaffolding that proves nothing; a horizon entry names its
   intended consumer and is periodically re-assessed for staleness/supersession. Unmarked + consumerless
   = kabuki, delete. Marked + fresh = exempt. Marked + stale = INSPECT for a human decision. The marker is
   greppable and dated so it can never become HC-1's silent forever-pass.

## 3a. Cascade contract (binding — each rung owns its own blast radius)

Verified before dispatch (2026-07-16): `parse_rungs()` is the single choke point, consumed by exactly
three paths — active-track generation/`--check`, `--park`, and `--unpark`. It is **not** a repo-wide doc
walk. Consequences, established so no rung re-derives them:

- **HC-4/HC-5 land green on history.** The one known pre-existing ladder violation
  (`design_0_0_8_4_6_ci_scaffolding.md`, escaped pipe in a ladder row) belongs to a CLOSED track that is
  never active/parked/unparked. `design_0_0_8_6_studio_live_ops.md` is pipe-clean, so the unpark that
  resumes Phase 12 stays safe. **Assert at the `parse_rungs` choke point** so all three consumers are
  covered at once; do **not** widen to a repo-wide scan (that reddens closed history for no gain).
- **HC-4 must close the park round-trip.** `--park` stores pointer state and `--unpark` restores it, so a
  track parked while its authoritative row names a graduated rung would resurrect divergence and fail
  `--check` after unpark. Guard: **`--park` refuses a divergent pointer** (same family as its existing
  open-PR refusal), so unpark can never restore one. Round-trip fixture required.
- **HC-2 must ledger its own fixtures.** New scan ⇒ new selftest fixture files ⇒ `test_inventory.tsv` rows
  in the same PR, or the drift gate crashes `doctrine_scan` outright (observed live 2026-07-15, cost a CI
  cycle). Any `ci_screening_surface.md` row lands within the existing cap. Net-ledger carve-out per §0.
- **HC-3 owns the receipt shape.** Reaping changes CLOSEOUT-RECEIPT content; update the receipt fixtures in
  the same PR. Parked tracks' rows live in their block and are out of scope — `--unpark` byte-exactness is
  the falsifier.
- **HC-1 is inert on landing.** Zero live users of the token remain (#1355's remand removed both), so
  deletion changes no verdict; the census is for *other* generic tokens, not this one.
- **Every rung** re-proves the 0.0.8.6 park round-trip (`--unpark` in a sandbox) before relay — the parked
  track is live state and no harness change may strand it.

## 4. Non-goals

No pointer change at authoring · no clearance-router/class changes · no new tables · no Studio/UI
or crates work (0.0.8.6 stays PARKED, receipt `19e0e85c8d3f`) · no re-litigation of #1355's merits
(that rung graduated; only its harness lessons are in scope) · no HD substrate redesign.

## 5. References

`STUDIO-FLEET-PRESENCE-READOUT-0` (#1355) DA remand + clearance — defects 1, 2, 4 ·
HD-C closeout (#1353) — defect 3 · `scans.tsv` `SPEC-LOWERER-KIND-READ` row ·
`handoff_template.md` §H rules 2/11 · `gen_orientation.sh` `authoritative_active_pointer()` +
fixture `orientation_authoritative_park_pointer` · `track_closeout_protocol.md`.
