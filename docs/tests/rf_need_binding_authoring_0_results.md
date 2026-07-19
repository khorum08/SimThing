# RF-NEED-BINDING-AUTHORING-0 (RF-5A) Results

## Status

**PROBATION / second-remand-remediated / clearance-pending** — 2026-07-19 (coder=Codex).

Remander `5015916255`, following remander `5014412811`, is implemented on the existing draft PR #1416 branch. RF-5/#1414 remains **BLOCKED-PAUSED**. No merge, DA relay, OVL, Studio production change, canonical TP change, recipe-authority expansion, or hosted GPU/desktop proof is claimed.

## Identity

| Field | Value |
|---|---|
| Rung | `RF-NEED-BINDING-AUTHORING-0` (RF-5A) |
| PR | #1416 (draft) |
| Base | `cecde41f7ed73287f309020eec6db8f57776feb9` |
| Pre-second-remand head | `f7d2b89cdd7674944a11085b6d3d8f6e8ea76387` |
| HD-RECEIPT | `5844fdbd66f4` |
| DA ruling | `5014178941` Modified Option A |
| Prior remander | `5014412811` |
| Current remander | `5015916255` |
| ORIENT-RECEIPT | `2c9fde39d1d6` |
| implementation_sha / tested_code_sha | `d53d5253aad066e0fe8b187e1b2dc21ed7279e7c` |
| evidence_sha | `79e5ed4c666522b5fb97c50c6b381e3b89ea78e4` |
| coverage_basis | RF-5A workshop 24/24 plus targeted RF/RF-4/OrderBand/adapter regressions; GPU/desktop rows are owner-local only |

## Remander remedies

| # | Finding | Remediation and falsifier |
|---|---|---|
| 1 | LIVE mutated registrations after open | LIVE uses an authored 0.25 transfer and authored install defaults before `SimSession::open`; no post-open registration mutation. |
| 2 | Host-qualified materialization could select first DFS owner | Transfer source/target, emission, and threshold host identities survive hydrate → compile → session materialization. Qualified rows resolve the named install target and its property cell directly. Conflicting explicit economy hosts fail closed. Recipes remain unqualified/legacy and are outside this claim. |
| 3 | Host failures lacked actual source spans | Location/owner host spans survive hydration. Missing, ambiguous, conflicting, and named-host-without-property tests assert the actual authored span token. |
| 4 | Same-participant bindings could collide on one need target despite disjoint staging | Resolution tracks `(participant_slot, need_col)` claims and fails closed at the authored participant span when a second binding claims the cell. No new need cell or combine order is invented. |
| 5 | Need staging could collide with gated-rate bands | Additive dependency map is producer `0` → stage `1` → need EvalEML `2` → arena `3`; the coexistence test asserts this order. |
| 6 | Three-control proof was relational | LIVE, DISCONNECT, and STATIC assert the exact t1/t2 closed forms below. |
| 7 | Disconnect switch was production-visible | Disconnect resync exists only behind the workshop-enabled `rf-test-harness` feature; the internal options path is crate-private. |
| 8 | Evidence chain was stale | Implementation and evidence SHAs are rebound by ordered commits after all gates pass. |
| 9 | Ordinary threshold proof tolerated a retry and 1–2 events | The authored `0.25` drip deterministically crosses `0.1` on named tick 1. The proof now requires ordinary kind 77 exactly once and need kind 91 exactly once after the post-RF append scan. |
| 10 | Post-RF append scan duplicated/misdecoded ordinary events | Root cause was twofold: a stale full-scan `n_ops` uniform executed a retained op beyond the shorter need packet, and a local need packet index aliased the restored full sidecar. The append scan now refreshes the uniform without clearing prior events, while need registrations occupy the stable full/append prefix. |

## Authoritative cell path

```text
authored source cell on named economy host
  -- ordinary transfer, producer band 0 --> current source value
  -- staged identity projection, band 1 --> participant-private stage cell
  -- need EvalEML, band 2 --> participant need cell
  -- arena execution begins at band 3
```

Property identity selects a column; the authored economy host selects the row. An unrelated overlay-held copy of the same property may coexist, but two explicit economy placements for the same property on different hosts are rejected.

This host-authority claim covers transfers, emissions, and threshold emits only. Recipe registrations remain unqualified/first-DFS legacy behavior and are neither changed nor claimed by RF-5A.

## Exact three-control measurements

Budgeted authored transfer: `0.25` per tick. Initial source/stage/need: `3.00/3.00/3.00`.

| Control | t1 source/stage/need | t2 source/stage/need | Required bite |
|---|---|---|---|
| LIVE | `2.75/2.75/2.75` | `2.50/2.50/2.50` | Stage and need follow both ordinary GPU transfer steps. |
| DISCONNECT | `2.75/2.75/2.75` | `2.50/2.75/2.75` | Actual runtime resync omits only staged projections; source continues while stage/need freeze. |
| STATIC | `3.00/3.00/3.00` | `3.00/3.00/3.00` | Pre-open disabled economy holds every cell. |

Observed focused output:

```text
running 24 tests
DISCONNECT exact: t1 source/stage/need=2.75/2.75/2.75; t2=2.50/2.75/2.75
STATIC exact: t1 source/stage/need=3.00/3.00/3.00; t2=3.00/3.00/3.00
LIVE exact: t1 source/stage/need=2.75/2.75/2.75; t2=2.50/2.50/2.50
test result: ok. 24 passed; 0 failed; 0 ignored
```

## Exact threshold and duplicate-target falsifiers

Observed from the actual owner-local GPU execution at implementation SHA `d53d5253aad066e0fe8b187e1b2dc21ed7279e7c`:

```text
RF-5A duplicate need target rejected at span 84: install: need_binding `second_same_participant` invalid: need target collision: participant slot 7 column 2 is already claimed by binding `expansion_need` (span_token=Some(84))
RF-5A crossing tick 1 raw threshold reg_idx=[1, 0]
RF-5A crossing tick 1 exact counts: ordinary kind 77=1; need kind 91=1
test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The raw indices show one retained full-scan ordinary record and one distinct need-only append record. The reconstructed sealed events prove kind 77 remains exactly one after the append rescan and kind 91 emits exactly once. The duplicate-target falsifier is an admission failure at the authored participant span, not a staged-lane or post-readout surrogate.

## Targeted verification

| Command | Result |
|---|---|
| `cargo check -p simthing-spec` | PASS |
| `cargo check -p simthing-driver` | PASS |
| `cargo check -p simthing-workshop --tests` | PASS |
| `cargo test -p simthing-workshop --test rf_need_binding_authoring_0 -- --nocapture` | PASS, 24/24 with exact values above |
| `cargo test -p simthing-driver --lib gated_rate_and_need_binding_bands_are_dependency_ordered -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-driver --lib governed_adapter_preserves_authored_targets_and_orderband -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-kernel --lib governed_integration_executes_only_on_its_authored_orderband -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-workshop --test rf_execute_recursive_default_0 -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-driver --test rf_conservation_oracle_0 -- --nocapture` | PASS, 2/2 |
| `cargo test -p simthing-driver rf_conservation_oracle::tests --lib -- --nocapture` | PASS, 3/3 |
| `cargo test -p simthing-workshop --test tp_rf_reduce_up_golden_0 -- --nocapture` | PASS, 2/2 |
| `cargo test -p simthing-spec --test runtime_rf_tick -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-spec --test runtime_tick_shell -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-spec --test runtime_tick_history -- --nocapture` | PASS, 1/1 |
| `cargo test -p simthing-mapeditor --lib studio_gpu_adapter_policy::tests -- --nocapture` | PASS, 3/3; CPU policy proof accepts supported non-DX12 backends and blocks DX12 |
| `cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture` | PASS, 8/8 owner-local GPU proof; not a GHA claim |

## SHA-bound governance

`bash scripts/ci/doctrine_pr_scan.sh cecde41f7ed73287f309020eec6db8f57776feb9 d53d5253aad066e0fe8b187e1b2dc21ed7279e7c`:

```text
TEST-BUDGET  INSPECT  1
WORKSHOP-HOMING-DETECTION  PASS  0
TEST-INVENTORY-DRIFT  PASS  0
DOC-BUDGET  PASS  0
hard failures: 0   inspect flags: 1
```

The single heuristic TEST-BUDGET inspection is discharged by additive `inspect_justifications.tsv` and `triage_log.tsv` rows bound to implementation SHA `d53d5253aad066e0fe8b187e1b2dc21ed7279e7c`. The 24 named proofs cover distinct load-bearing remand rows: authored LIVE/DISCONNECT/STATIC execution, host/span admission failures, duplicate need-target rejection, cross-row projection, exact single event counts, and authority/non-invention boundaries.

Hosted CI may verify CPU/build/doctrine rows, but GitHub Actions has no GPU or desktop context. The GPU/Windows observations in this ledger are local owner-machine evidence and are not represented as hosted proofs.

## Holds

Draft #1416 only · RF-5/#1414 blocked-paused · no merge · no DA relay · no OVL/Studio/TP scope expansion.
