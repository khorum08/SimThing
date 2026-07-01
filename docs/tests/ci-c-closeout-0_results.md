# CI-C-CLOSEOUT-0 — results

**Status:** DA-CLOSED (Executive DA: Opus/Owner, 2026-07-01)
**Track:** 0.0.8.4.6 Track C (the carrot) — **CLOSED**

## DA decision
Track C **CLOSED**. C1 COMPLETE; C2 recovered after 0R and COMPLETE; C3 accepted COMPLETE (guarantees proven; packaging irregularity ruled cosmetic). All proofs run against the tree by the DA, not relayed.

## Track C rung audit
- **C1 `CI-C-INNER-LOOP-0` — COMPLETE.** Convention landed one-line in the handoff spine + agents.md (delegating to `ci_screening_surface.md`). `debug_kind()` verified display-only (zero non-test call sites, no new variant). The `SEMANTIC-WORDS` slip was real, caught pre-PR by the inner loop, fixed, logged GREEN.
- **C2 `CI-C-DIGEST-0` (after `CI-C-DIGEST-0R`) — COMPLETE.** GATE 1 HOLD (ungated freshness) is resolved: `gen_digest.sh --check` is wired into `.github/workflows/doctrine-scan.yml` (step "Sanctioned surface digest freshness", `set -o pipefail` before `… | tee` — exit not masked). Content faithful: 5/5 embedded sha256 match live sources; sealed-producer door set diffs exact. **Drift genuinely bites** — DA perturbed the digest (injected a phantom door) → `--check` exit 1; restored.
- **C3 `CI-C-TRACK-ADDENDUM-0` — COMPLETE.** `--prove-addendum` PASS with **substantive** assertions: opt-in (no marker → addendum not loaded), auto-detach (only the named track's addendum loads; another's does not), additive-only (an addendum redefining a global scan-id → nonzero + "redefines global scan-id"), digest scope (track digest = global + that track only). Global `scripts/ci/scans.tsv` + `allow/**` **byte-unchanged** by Track C (last touch is Track A's `f47369bf97`). Implementation landed under its own properly-titled commit `bc2e23ecc7`.
  - **Packaging irregularity (recorded, ruled cosmetic):** C3's *results doc* rode in on the C2-HOLD PR #1060 merge rather than its own PR. The *implementation* commit is clean and properly attributed, the tree carries the correct code, and the guarantees are proven — so the substance is accepted. Non-blocking process note: evidence should ride with its own PR (§7).

## Three-position scanner map
- **before generation** — `docs/sanctioned_surface.md` digest (C2), now freshness-gated.
- **during generation** — inner-loop self-scan `cargo check` + `doctrine_scan.sh` (C1).
- **after generation** — GitHub Actions doctrine-scan gate (Track A) + digest-freshness step (C2 0R).

## Triage corpus review
`scripts/ci/triage_log.tsv`: **1 entry** — `SEMANTIC-WORDS | ci-c-inner-loop-0 | green` (the C1 inner-loop demo; a true positive caught pre-PR, resolved GREEN, commit `fd11b746`). C2, 0R, and C3 raised **no** INSPECT (verified). Classification: 1 true-positive, 0 false-positive, 0 gray. **No chronically-firing HEURISTIC** — corpus is **too thin for promotion/retirement conclusions**; no scan promoted, retired, or tightened. The thinness is the honest finding: a CI-infra track barely touches the sim HEURISTIC surface, and the scanner is well-tuned (no noise). First real exercise of the §1A telemetry→maintenance cadence: reviewed, no action.

## Load-bearing proofs (DA local run, 2026-07-01)
```
gen_digest.sh --check                       -> gen_digest --check: PASS (exit 0)
gen_digest.sh --check (digest perturbed)    -> exit 1  (drift bites; restored)
doctrine_scan.sh                            -> DOCTRINE-SCAN-VERDICT: PASS failures=0 inspect=0
doctrine_scan.sh --prove-addendum           -> doctrine_scan --prove-addendum: PASS (exit 0)
git log -1 -- scripts/ci/scans.tsv          -> f47369bf97 (Track A) — no Track C change
workflow "digest freshness" step            -> set -o pipefail; gen_digest --check (exit not masked)
```

## INSPECT / triage
None raised by closeout.

## Scope Ledger
| Element | State |
|---|---|
| C1 accepted COMPLETE | implemented |
| C2 accepted COMPLETE (post-0R) | implemented |
| C3 accepted COMPLETE (packaging cosmetic) | implemented |
| Three-position scanner recorded | implemented |
| Corpus reviewed | implemented (1 row, no action) |
| Global scans/allowlists byte-unchanged | verified |
| No repair rungs open | confirmed |

## Known gaps / next
- Non-blocking: `gen_digest.sh`/`doctrine_scan.sh` call `python3`, which hits the Windows app-alias stub locally; CI ubuntu `python3` is real (authoritative run is GitHub-side). Non-blocking process note on C3 evidence packaging (above).
- Track C CLOSED. Deferred-but-scoped: Track B (executable seal-proof harness) opens when a consumer needs per-change seal-proof; the per-track addendum mechanism (C3) is available for future production tracks (Terran-Pirate, MapGen) opt-in.
