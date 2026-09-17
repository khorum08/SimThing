# Expired pen: bounded consumer renewal and carried funded cancellation

Status: PROBATION / proof-present / DA-review-pending. No merge or graduation.

## Authority and scope

Ingress: Board/PR comment `5706525735`, binding DA dispatch `5706504123`.
Base: `6dbab071be7afd3967c63663badcb9067e94ac22`. The isolated branch
`codex/0088-expired-pen-disposal` carries the exact #2068 head
`8a4165cb6747f7e3c9172822704d7c38f6dc8599` through local integration commit
`639e26ef09ba003149acde02b2178cc5d8dfbb91`.

The Owner amended the disposal instructions in this coding session on 2026-09-17 UTC:

> do not reap anything that is still used or may be used

The explicit fixture disposition was:

> retain fixtures but just renew their lifecycles so that they are up for reaping in 5 wall clock days or the next lawful closeout (rung or workplan)

This supersedes the original expected-zero-renewals disposal instruction. The actual
364-row pen contains **318 Rust test identities and 46 harness fixtures**, not 364 Rust
functions. The 46 fixtures have existing gate/selftest consumers. The Rust proofs are
retained conservatively as potential consumers of the remaining 0.0.8.8 substrate work;
this is not a claim that every proof has already been consumed by the construction rung.
The initial uncommitted removal pass was completely reversed before final validation.

## Implemented behavior

All 364 identities remain at their existing source paths, byte-identical to the carried
#2068 tree. Original `parked_at`, `birth_track`, and closeout provenance are preserved.
Each row gains one `dsu_survivals` increment, a named current/potential consumer, and an
explicit bounded renewal in the existing `park_reason` field. There is no second live
ledger and no broad rewrite of birth tracks.

The existing `track_closeout.sh` now recognizes this opt-in record:

- `renewed_on: 2026-09-17`, `expires_on: 2026-09-22` (UTC date boundaries; no later than five days).
- `closeout: next-rung-or-workplan`, plus nonempty `consumer` and `authority`.
- The earlier event wins: either closeout door requires explicit disposition before
  proceeding, even before September 22. A failed closeout does not delete files.
- The expiry gate and reaper share the same deadline predicate. Malformed, overlong,
  future-dated, consumerless or uncounted renewals fail closed before deletion.
- Ordinary seven-day leases and the existing shared-source/live-survivor protections
  remain unchanged. A deadline makes an asset due for disposition, not permission to
  blindly delete a still-consumed fixture or shared file.

The gate-code companion is necessary to enforce the Owner's shorter, event-bounded
renewal. It is DA-reserved and remains unmerged. No gate has been weakened or suppressed.

The first hosted gate run exposed an existing orientation-selftest pipe failure:
`printf | grep -q` under `pipefail` reported a false missing drift anchor after the
reader exited early. The selftest now consumes the complete stream; assertions and
receipt derivation are unchanged. Hosted run `35168228505` records the original
Linux failure; Windows did not reproduce it. The corrected local selftest passes.

## Scope and preservation

| Account | Result |
|---|---|
| Original expired pen | 364 retained with finite renewal; zero source deletions |
| Current fixture consumers | 46 retained; gate/selftest command named per identity |
| Potential Rust proof consumers | 318 retained; exact Cargo target/filter named per identity |
| Live inventory | 1,018 rows unchanged from the carried #2068 tree |
| Original file lease | One row unchanged |
| Source and qualification bundle | Byte-identical to #2068; no additional E8 roll |
| Other agents' work | Separate worktree, branch and Cargo target directory; no edits to their checkouts |

Per-identity decisions and source SHA-256 values are in
`expired_pen_disposal_0_checkpoint/retention_decisions.tsv`. The immutable carried commit
above is the original ledger/source snapshot. The funded-cancellation production repair
and thirteenth E8 pin remain exactly #2068's content. Model 1 remains UNDECIDED;
Model 2 remains CLOSED. #2068 must not merge separately.

## Proof

- Nine affected/carried crates: `cargo check --locked ... --tests` PASS after full restoration.
- `track_closeout.sh --prove`: **148 checks PASS**, including the disposable
  build/resolve/check/apply rehearsal and new deadline/closeout/refusal cases.
- Current artifact expiry: **PASS expired=0 cruft=0 malformed=0**.
- Current reaper dry run: **reaped=0 files=0 manual=0**.
- September 22 controlled-clock falsifier: **FAIL expired=364 cruft=1 malformed=0**,
  exit 1 as required. The one unrelated historical file lease is the CRUFT row.
- Lifecycle schema and scheduled checks: PASS; existing 54 closed-track audit rows unchanged.
- Scenario residue: existing 47 advisory DEAD-EXPORT entries; no DEAD-TARGET or new residue.
- `git diff --check`: PASS. No source delta after the carried integration commit.
- Carried cancellation acceptance law: 9/9 PASS; exact placement battery: 3/3 PASS;
  profiled qualification mutation battery: 4/4 PASS.

Further SHA-bound execution and hosted gate evidence are recorded at PR/Board return.
Historical #2068 GPU evidence is not represented as a new execution of this branch.

## Routing and receipts

Risk class: gate-wiring, carrying the previously reviewed production repair.
Return to Fable for DEEP-TREE review; coding does not merge.

ORIENT-RECEIPT: 28f56884d309
role: coding
orientation_rule_stamp: 73e54d6b56b7266b
ANCHOR-ACK: orientation-harness-core@8a365d1c0864
ANCHOR-ACK: scanner-selftest-delta-gate@34fb2662baae
ANCHOR-ACK: rehearsal-0088-routing@ac4e7d2c5a61
ANCHOR-ACK: rehearsal-0088-construction-contract@bdd51c7c4ae2
ANCHOR-ACK: seal-residue-cross-crate@c61c33d90efc

The disposal ingress is the named comment/DA ruling; no disposal `.hd.md` or
HD-RECEIPT was issued. No handoff receipt is fabricated.
