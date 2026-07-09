# HU-DELTA-SCAN-0 Results

## Status

**PROOF-PRESENT / PROBATION** — delta-first coding screen. Gate-wiring; not self-merged.

## Deliverable

`scripts/ci/agent_scan.sh` — thin wrapper over `doctrine_scan.sh --pr-delta` (no new scan logic).

| Mode | Behavior |
|---|---|
| RELIABLE | Whole-tree hard FAIL via existing scanner (unchanged) |
| HEURISTIC | Changed files/lines only (`--pr-delta`) |
| Footer | `AGENT-SCAN-VERDICT: PASS\|FAIL\|INSPECT delta_inspect=N elapsed=Ns` |
| Drift gates | Skipped locally (`DOCTRINE_SCAN_SKIP_DRIFT=1`); remain CI/maintainer whole-tree |

## Proofs

```text
bash scripts/ci/agent_scan.sh --selftest
→ AGENT-SCAN-SELFTEST: PASS (3 fixtures)
  known-bad in delta → FAIL (elapsed=2s)
  heuristic outside delta → PASS delta_inspect=0 (elapsed=2s)
  footer grammar stable → PASS

bash scripts/ci/agent_scan.sh  (light dirty tree vs origin/master)
→ AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=10s

p50 light sample: 2s (sandbox selftest clean/FAIL cases); live light ≤10.7s
bash scripts/ci/gen_orientation.sh --check → PASS
Scanner surface (scans.tsv / allow/*) untouched → doctrine_selftest not required (§4B)
```

## Orientation / handoff

- Coding inner loop ≤4 steps: orient-once → `cargo check -p` → `agent_scan` → focused test
- `handoff_template.md` routine proof points at `agent_scan` for coding diffs

## Non-goals held

No RELIABLE weaken · no allowlist/scans.tsv edit · no gen_orientation refactor · whole-tree scan remains CI/maintainer
