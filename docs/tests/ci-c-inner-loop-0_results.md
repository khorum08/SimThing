# CI-C-INNER-LOOP-0 Results

## Status

**PROBATION** — pending DA review. Track C rung; `CI-C-DIGEST-0` / `CI-C-TRACK-ADDENDUM-0` / `CI-C-CLOSEOUT-0` remain held.

## PR / branch / merge

- Branch: `ci-c-inner-loop-0`
- PR: (recorded after push)
- Merge: (recorded after push)

## What changed

**Convention landing (docs-only, one line each):**

- `docs/handoff_template.md` — one new context-spine bullet, immediately after the existing CI doctrine-scan bullet:
  > *"Inner loop (Track C §3A): run `cargo check -p <touched-crate>` and `bash scripts/ci/doctrine_scan.sh` after small edits, not only at PR time — the FAIL-with-remedy is a steering signal during generation, not just an after-the-fact gate; full contract: [`ci_screening_surface.md`](ci_screening_surface.md)."*
- `docs/agents.md` — one new bullet inside the existing "CI doctrine-scan" section (no second CI section created):
  > *"Inner loop (Track C): run `cargo check -p <touched-crate>` and `bash scripts/ci/doctrine_scan.sh` after small edits, before PR-time cleanup — the same scanner, consulted earlier, so a doomed path is pruned in your own loop instead of at CI/triage/DA."*
- `docs/design_0_0_8_4_6_ci_scaffolding.md` — `CI-C-INNER-LOOP-0` lifecycle: `DEFERRED` → **PROBATION**, DoD cell records the landing + the real demo below.
- No edit to `docs/ci_screening_surface.md` — its existing §6 "Inner-loop self-scan" bullet already states the contract; both new pointers above delegate to it rather than restating.

**Real substrate-touching inner-loop demo (`crates/simthing-sim/src/threshold_registry.rs`):**

Added `impl ThresholdSemantic { pub fn debug_kind(&self) -> &'static str }` — a small, genuinely useful, non-inert addition: a stable, display-only tag per `ThresholdSemantic` variant for observability/log lines (the module has no existing label/describe helper; callers currently have no cheap way to print a stable event-kind tag). Backed by one new regression test (`debug_kind_tags_are_stable_and_display_only`) asserting the tag for three representative variants.

**The real inner-loop catch:** the first draft of the `CapabilityUnlock` arm read:

```rust
ThresholdSemantic::CapabilityUnlock { .. } => "faction_capability_unlock",
```

— a genuine slip: the surrounding doc comment on this very variant (line 121) still reads *"the faction's `CapabilityTreeInstance`"*, a pre-existing leftover of the project's retired "faction" vocabulary (constitution: "Terminology correction — owner, not faction"), and it was the natural (wrong) word to reach for. Running the inner loop caught it for real — see transcript below — before it ever reached a PR. Fixed to:

```rust
ThresholdSemantic::CapabilityUnlock { .. } => "capability_unlock",
```

which also matches the sibling tags' naming convention (no redundant subject prefix — `fission_trigger`, `property_expiry`, etc. carry no owner/faction prefix either).

No `scripts/ci/scans.tsv` or `scripts/ci/allow/*.txt` edit. No digest/addendum/dashboard/metrics work.

## Inner-loop transcript

**1. `cargo check -p simthing-sim` — first draft (naive `"faction_capability_unlock"` tag), real output:**

```
$ cargo check -p simthing-sim
    Checking simthing-sim v0.1.0 (C:\Users\mvorm\SimThing\crates\simthing-sim)
warning: methods `replace`, `access`, and `access_mut` are never used
   --> crates\simthing-sim\src\sim_runtime_tree.rs:118:19
    = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default
warning: `simthing-sim` (lib) generated 1 warning
    Finished `dev` profile [optimized + debuginfo] target(s) in 16.77s
```

Compiles clean (pre-existing, unrelated dead-code warning only — not introduced by this change).

**2. `bash scripts/ci/doctrine_scan.sh` — first draft, real output (the real catch):**

```
DOCTRINE SCAN REPORT  (commit 47781ea11b, 2026-07-01T06:55:26Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  INSPECT  1  design §5 semantic words below spec .\crates\simthing-sim\src\threshold_registry.rs:151:            ThresholdSemantic::CapabilityUnlock { .. } => "faction_capability_unlock",
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 1   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: INSPECT  failures=0 inspect=1 selftest=SKIPPED
```

**3. Fix applied:** `"faction_capability_unlock"` → `"capability_unlock"`.

**4. `cargo check -p simthing-sim` — after fix, real output:** clean, same pre-existing warning only, `Finished` exit 0.

**5. `bash scripts/ci/doctrine_scan.sh` — after fix, real output (final, clean):**

```
DOCTRINE SCAN REPORT  (commit 47781ea11b, 2026-07-01T07:01:36Z)
  scanner self-test: SKIPPED
  scan mode: whole-tree
  reliable scope: whole-tree
  heuristic scope: whole-tree
  --- results ---
  B3-BUFFER-ESCAPE  PASS  0  design §5 B3 buffer escape
  FORGE-MINTERS  PASS  0  design §5 forge minters
  UNSAFE-FN  PASS  0  design §5 unsafe fn
  UNSAFE-ALLOW-ATTR  PASS  0  design §5 allow unsafe attr
  UNSAFE-FORBID-ATTR  PASS  0  design §5 forbid unsafe attr
  AS5-COLUMN-ALIAS  PASS  0  design §5 AS-5 ColumnIndex alias
  DENY-TOML-STUB  PASS  0  design §0.6.6 deny.toml stub
  RAW-DATA-INDEX  PASS  0  design §5 raw data[N] index
  SIM-KIND-READ  PASS  0  design §5 sim .kind read
  SEMANTIC-WORDS  PASS  0  design §5 semantic words below spec
  SPEC-STRING-CHANNEL  PASS  0  design §5 stringly channel identity
  ALLOW-SEALED-PRODUCERS  PASS  0  design §5 sealed producer allowlist
  ALLOW-BUFFER-HANDLES  PASS  0  design §5 buffer handle allowlist
  ALLOW-KERNEL-SURFACE  PASS  0  design §5 kernel surface allowlist
  --- summary ---
  hard failures: 0   inspect flags: 0   reliability: RELIABLE=hard FAIL; HEURISTIC=INSPECT only
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
```

**6. `cargo test -p simthing-sim --lib threshold_registry::tests::debug_kind_tags_are_stable_and_display_only` — real output:**

```
running 1 test
test threshold_registry::tests::debug_kind_tags_are_stable_and_display_only ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
```

## INSPECT justification(s)

Author-pays-first structured justification (`scan-id | location-or-symbol | rationale | promotion-blocker`), kept in this results doc per the handoff (not added to `scripts/ci/inspect_justifications.tsv`, since the flag was resolved by a real code fix before PR and would be stale the instant it landed):

```
SEMANTIC-WORDS | crates/simthing-sim/src/threshold_registry.rs:151 (ThresholdSemantic::debug_kind, CapabilityUnlock arm) | inner-loop first draft used the retired "faction" vocabulary in a new debug-tag string literal for a display-only observability tag (not a runtime kind branch); violates the constitution's owner-not-faction terminology correction | promote when a lint/type forbids the literal token "faction" in non-doc production source outside SimThingKindTag::Faction's serde-compat variant; today this is inner-loop discipline (Track C), not yet type-enforced
```

Note: `bash scripts/ci/doctrine_scan.sh`'s own justification-file summary line ("justifications file present with 1 entries") is a pre-existing quirk unrelated to this INSPECT — `scripts/ci/inspect_justifications.tsv` contains only its header row, which the current parser counts as one entry keyed on the literal string `scan-id`. It never matched or resolved this rung's real `SEMANTIC-WORDS` flag (key mismatch). `scripts/ci/inspect_justifications.tsv` is forbidden-path for this rung (not `scripts/ci/scan_allowlists.py`/`doctrine_scan.sh`, but out of this rung's touch set), so it is reported here, not fixed here.

## Triage outcome(s)

**GREEN** — the author (this rung's implementer) fixed the underlying issue before PR. Verified *why* it is now legitimate, not merely that the scanner went quiet: the replacement tag (`"capability_unlock"`) carries no legacy-vocabulary term, and its shape now matches every sibling tag's naming convention (`fission_trigger`, `fusion_trigger`, `property_expiry`, `velocity_alert`, `aggregate_alert`, `scripted_event_trigger` — none carry a subject prefix). Confirmed by the final whole-tree scan (`SEMANTIC-WORDS PASS 0`) and by re-reading the diff, not by scanner silence alone.

Logged to `scripts/ci/triage_log.tsv`:

```
SEMANTIC-WORDS | ci-c-inner-loop-0 | green | inner-loop demo: new ThresholdSemantic::debug_kind() first draft tagged CapabilityUnlock as "faction_capability_unlock" (legacy vocabulary, echoing the variant's own doc comment); doctrine_scan.sh caught it as a real INSPECT before PR; renamed to "capability_unlock" (drops legacy term, matches sibling tag naming with no subject prefix); final whole-tree scan PASS 0; transcript in docs/tests/ci-c-inner-loop-0_results.md | <commit>
```

(`<commit>` filled with the real branch-head SHA in a same-branch follow-up commit once the initial commit exists — see Scope Ledger.)

## Load-bearing proofs

| Proof | Catches |
|---|---|
| `cargo check -p simthing-sim` (before + after fix) | the substrate-touching final edit still compiles; no new warnings introduced |
| `bash scripts/ci/doctrine_scan.sh` (before fix) | the inner-loop scan actually emits a real HEURISTIC/INSPECT on a real substrate touch — not asserted, not fabricated |
| author justification row (above) | §1A cost-symmetry requirement met before triage |
| triage GREEN resolution + `scripts/ci/triage_log.tsv` row | green-with-INSPECT did not silently pass; the fix's legitimacy was verified, not merely "scanner went quiet" |
| `bash scripts/ci/doctrine_scan.sh` (after fix) | final tree is clean — `DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0` |
| `cargo test -p simthing-sim --lib threshold_registry::tests::debug_kind_tags_are_stable_and_display_only` | the new public API's tag mapping is pinned by a real regression test, not just eyeballed |
| `git diff --name-only master...HEAD` | scope stayed inside the rung-local touch set; no C2/C3/global-list/tooling metastasis |
| GitHub Doctrine Scan workflow on PR | authoritative CI path agrees with the local proof above |

## Scope Ledger

| Path | Touched | Note |
|---|---|---|
| `crates/simthing-sim/src/threshold_registry.rs` | yes | new `debug_kind()` + one regression test; the substrate-touching demo |
| `docs/handoff_template.md` | yes | one inner-loop convention line |
| `docs/agents.md` | yes | one inner-loop convention line (existing CI section) |
| `docs/design_0_0_8_4_6_ci_scaffolding.md` | yes | `CI-C-INNER-LOOP-0` row → PROBATION |
| `docs/tests/ci-c-inner-loop-0_results.md` | yes (new) | this doc |
| `docs/tests/current_evidence_index.md` | yes | one line |
| `scripts/ci/triage_log.tsv` | yes | one row, the real INSPECT above |
| `scripts/ci/scans.tsv`, `scripts/ci/allow/**`, `scripts/ci/doctrine_scan.sh`, `scripts/ci/doctrine_pr_scan.sh`, `scripts/ci/doctrine_selftest.sh`, `scripts/ci/scan_allowlists.py`, `scripts/ci/inspect_spam_check.sh`, `docs/sanctioned_surface.md`, `scripts/ci/gen_digest.sh`, `scripts/ci/inspect_justifications.tsv`, any dashboard/metrics artifact | **no** | forbidden / not needed |
| C2 digest generation, C3 addendum machinery | **no** | explicit non-goal |

## Known gaps / next

- `CI-C-DIGEST-0` (sanctioned-surface digest) remains held/DEFERRED.
- `CI-C-TRACK-ADDENDUM-0` remains held/DEFERRED.
- `CI-C-CLOSEOUT-0` will read the accumulated `triage_log.tsv` corpus (this rung's row is the first real entry) and classify chronically-firing HEURISTICs as retirement/promotion candidates, or record "corpus thin."
- The `threshold_registry.rs` doc comment at the `CapabilityUnlock` variant still literally reads "the faction's `CapabilityTreeInstance`" (a `///` comment, correctly excluded by the scanner's own comment filter — this is why it never fired on its own). A genuine, tiny follow-up terminology cleanup, out of this rung's scope (comments are not scanned by design; not a doctrine violation).
- PR remains PROBATION pending DA review, per the merge-hold rule this track's own doctrine landing (`CI-A-DOCTRINE-LANDING-0`) established.
