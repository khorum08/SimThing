# Artifact-expiry repair: manual-debt checkpoint

This is the checkpoint required by Board handoff **5673501235**, carrying DA ruling
**5673485508**. Manual dispositions remain **PROPOSED**. None has been applied.

The disposal branch `codex/artifact-expiry-repair` started at exact master
`c26f02270ed16749170c780c4d9cadb856f51a2b` and merged the canonical repair commit
`cecb22588ad9d5c8663eff0429f22a2b09bbcb82` through merge commit
`51bd51078919f88b927b4174d0b64f48aba6e3c9`. The repair is unchanged. PR #2060 remains
its canonical open record; this branch carries it toward the final debt-paying PR.
PR #2059, master, and the active workplan/orientation pointer were not changed.

Coding ORIENT-RECEIPT: `28f56884d309`; rule stamp `73e54d6b56b7266b`.
Authority: `docs/track_closeout_protocol.md`; lifecycle doctrine queried through
`anchor_query.sh`, including admission-ladder-necessity-test and
workshop-candidate-homing. Current workplan obligations were checked against
`docs/0_0_8_8_Integrated_SimThing_Rehearsal.md`, the committed exposure matrix, and
the opening research inventory. This checkpoint creates no new doctrine or gate.

## Executed safe reap

| Account | Exact master | After safe reap | Delta |
| --- | ---: | ---: | ---: |
| Live test inventory | 995 | 995 | 0 |
| Parked test rows | 568 | 559 | -9 |
| File leases | 90 | 85 | -5 |
| Expired items | 293 | 279 | -14 |
| Non-expired CRUFT | 364 | 364 | 0 |

`track_closeout.sh --prove` passed, including all ten carried reaper-safety
falsifiers. Dry and apply both selected **11 unique safe files / 9 parked rows**.
The full output and deduplicated removal list are retained alongside this file.
Actual apply used the UTC wall clock, no override and no `--all`.

Immediately after apply, `--artifact-expiry` returned
`FAIL expired=279 cruft=364 malformed=0` (exit 1). That is the required manual-debt
checkpoint, not completion or a request for a red merge. All 365 non-expired rows
(364 CRUFT plus one not-yet-CRUFT file lease) and all 162 existing files referenced
by those rows were checked unchanged. The 13 mixed test files and shared support
were preserved. The protected repair still matches its DA commit.

`cargo check --locked -p simthing-core -p simthing-driver -p simthing-embedder
-p simthing-workshop --tests` passed in 40.39 seconds with warnings. This compiles
the affected test targets; it does not claim GPU/runtime execution. Full cargo
output is retained locally with the agent's proof files. The required reaper proof
is committed here in full. Hosted Doctrine and clearance belong after approved
manual disposal reaches expired=0; no cleanup PR or clearance request is opened
at this intermediate checkpoint.

## Proposed manual dispositions

Read [the complete Markdown table](manual_dispositions.md) or its identical
[machine-readable TSV](manual_dispositions.tsv). There is one row for every
remaining expired identity, with its originating ledger/date, exact reaper
refusal, observed committed consumers, specific owner/referee or destination,
committed workplan obligation, and exactly one proposed disposition.

| Item type | DELETE proposed | ELEVATE proposed | Total |
| --- | ---: | ---: | ---: |
| Exact parked test identities | 71 | 124 | 195 |
| Whole-file artifact leases | 17 | 67 | 84 |
| Total | 88 | 191 | 279 |

The proposal audit checked exact coverage/uniqueness against both ledgers and the
post-apply expiry output. All named owner paths and explicit referee functions
exist; no explicitly named retained referee is itself proposed for deletion.
Source/import inspection is evidence of committed consumers, not an execution
claim or a measured performance baseline.

Review these distinctions when deciding the table:

- Known escaped-regression proofs have explicit current-track obligations. Their
  proposed lifecycle is finite `until-closeout:*`, without resetting a lease.
- Mixed test-file leases are distinct from the test identities inside them.
  Promoting a container/support file changes no non-expired pen identity or date.
- Canonical production sources stay at their current homes with source/E8 bytes
  unchanged. E9a/E9b, sparse-weight and carry measurement debt stays explicitly
  with the current research inventory and application-performance rung.
- The six named M13 workshop families are proposed as finite sets of non-runnable
  historical evidence in `docs/archive/0087-workshop-research/`, with their live
  exports removed together. That does not validate or execute an old instrument.
- The exact old A1 refinement harness, its referenced shader and notes retain
  historical reference ownership. Other obsolete stencil variants have the
  archive README's concrete structured-stencil successor.
- The font is embedded by Studio and two simthing-tools modules; its proposal
  moves identical bytes to the tools asset owner and repoints those consumers.
- Two ignored `generator_cli` tests are real implementations invoked by generator
  scripts. They propose elevation into named owning-crate binaries and explicit
  script caller updates. This protected companion work needs approval; it has
  not been performed or hidden as a test deletion.

Orchestration must approve the table before any manual mutation. Resume this same
branch with the approved dispositions, then require expired=0, local gates,
hosted Doctrine and fresh clearance. The carried protected repair routes the
completed disposal PR back to the DA as ruled. No red merge or separate repair-free
cleanup PR is permitted.

## Evidence files

- [Dry run](decommission_dry_run.txt) and [actual apply](decommission_apply.txt)
- [Unique safe deletions](safe_deletions.txt)
- [Immediate expiry output](artifact_expiry.txt)
- [Full reaper proof](prove.txt)
- [Preservation check](preservation_verification.txt)

These files record this one disposal checkpoint. They are review evidence, not a
new standing inventory or automatic disposition authority. The approved final
disposal return must reconcile actual outcomes with this proposal set.
