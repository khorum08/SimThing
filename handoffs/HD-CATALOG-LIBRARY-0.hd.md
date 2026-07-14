---
rung: HD-CATALOG-LIBRARY-0
kind: rung
track: 0.0.8.4.8.4
base_sha: baac36c90a9e8ffbe412f8695a8c63f8d6a14223
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed 2026-07-14, manual-progression stack: each agent acts when the Owner prompts it to check the board; the coder is a fresh cold-start agent (any capable coder, ruling 7). One rung at a time."
surfaces: ["scripts/ci/librarian.sh", "scripts/ci/anchor_check.sh", "scripts/ci/anchor_query.sh"]
forbidden: ["crates/**", "Studio/UI", "new tables", "second trigger engine", "gen_orientation gate changes", "handoff_dispatch logic changes"]
required_checks: ["librarian-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "reach-log-mutation-on-read"]
---
## BUILD
- `librarian.sh --catalog [--role coding|orchestrator|da]` v2: enumerate the LIBRARY per role —
  every anchor reachable by that role (id + trigger domains, from `doctrine_anchors.tsv` x
  `anchor_triggers.tsv`), the always-on spine, and the payload sections that role receives in
  projections — NOT bound to the current handoff. Roles must render meaningfully differently.
- `--staleness` gains one gauge line: count of `harness-fixture` birth_track rows in
  `scripts/ci/test_inventory.tsv` (the one monotonically growing permanent set).
## FENCES
- Compose EXISTING data (anchors TSV, triggers TSV, orientation, dispatch payload map); no new
  tables; no second trigger matcher — resolve through the anchor_check/anchor_query machinery.
- Catalog stays read-only: temp `ANCHOR_REACH_LOG_PATH` isolation (HD-4 pattern); live and
  fixture reach-log bytes unchanged by any catalog invocation.
- Reports <=60 lines per role, complete-or-fail (never truncate); do not weaken cull/staleness.
## EXIT-PROOF
- Fixtures bite: per-role-catalogs-differ (coding vs orchestrator vs da content assertions);
  catalog-cap complete-or-fail; catalog-read-only (reach-log byte equality); fixture-count gauge
  present in staleness output.
- Battery green: librarian selftest, agent_scan, gen_orientation --check, doc_budget --check.
- PROBATION stamp LEADS the HD-7 exit-proof cell in the same diff; orientation regenerated.
- DA authors the graduation stamp at merge (ruling 6); relay carries this HD-RECEIPT.
