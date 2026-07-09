# HU-DOCS-CONSOLIDATION-0 Results

**PROOF-PRESENT / PROBATION** — docs-only entry-point + screening catch-up.
Gate-wiring review surface; not self-merged (Fable).

ORIENT-RECEIPT: `258b76525e5c` · stamp `ba76e34e03a0bbd2`.

## Changes

| Doc | Before | After | Justification |
|---|---|---|---|
| `docs/agent_onboarding.md` | 105 | 122 | 4-step coding loop + observation section (session ritual changed) |
| `docs/ci_screening_surface.md` | 548 | 525 | agent_scan + class_predicates + treeverify fold; paid by §11 compress |
| `docs/agents.md` | 188 | 190 | one-liners: agent_scan + track_closeout |
| **Net (three)** | **841** | **837** | **−4** (~zero growth) |

DOC-BUDGET re-pointed to exact post-refresh counts (screening stays ≤548).

## Grep proof

| Query | Location |
|---|---|
| `agent_scan` | screening engines + §6/§7; onboarding loop; agents.md |
| `class_predicates` | screening engines + merge-authority |
| `DA-TREEVERIFY-PROFILE` | clearance_check row + §5A merge authority |
| stale `test_lifecycle_boundary_rows` / `boundary_check` | **0** in screening |

## Exit proof

```text
doc_budget_check.sh --check -> PASS
gen_orientation.sh --check -> PASS (untouched)
agent_scan.sh -> PASS footer
```

tested_code_sha: 389c3445d26153ac5e80397522eaf437dcbbbd50
coverage_basis: PASS - docs catch-up + budget + agent_scan
