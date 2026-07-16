---
rung: HC-AUDIT-POLISH-0
kind: rung
track: 0.0.8.4.8.4.1
base_sha: 02cac01007c7a7a60f62a4b0807a5f3184a65412
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Fully-automated: coder = Grok CLI (grok-4.5), DA-driven. Three minor Fable-audit corrections; none change scan semantics beyond consistency. Ignore benign tzutil errors. Owner carries progression."
surfaces: ["scripts/ci/gen_orientation.sh", ".github/workflows/clearance.yml", "scripts/ci/scans.tsv", "docs/ci_screening_surface.md"]
forbidden: ["crates/**", "Studio/UI", "regex widening of GUARD-KABUKI-TRIPWIRE", "clearance router logic", "new tables"]
required_checks: ["doctrine-selftest", "gen-orientation-selftest", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening"]
---
## BUILD
1. Single-source the horizon staleness window: gen_orientation.sh lifecycle assessment hard-codes
   HORIZON_ENTRY_STALE_DAYS=90 (~line 1671) while doctrine_scan.sh honors the env — make the
   lifecycle path honor the SAME env override (default 90). Falsifier: an env override changes
   BOTH scan-exemption and lifecycle-assessment behavior consistently; pre-fix they diverge.
2. Board push-sync live-handoff render: on push-to-master the clearance.yml sync step skips
   handoff resolution even when a live dispatch exists. Resolve via the graceful --current-handoff
   (#1342 path) on push events; render 'none' only when genuinely none. Falsifier: sync with a
   live .hd resolves it (fixture or selftest-level proof of the resolution branch); pre-fix none.
3. Document the accepted GUARD-KABUKI-TRIPWIRE evasion residue (PRIVATE fn source-scanner or
   var-bound include_str! evades the pub-fn-anchored arms) in the scans.tsv note column + one
   ci_screening_surface.md line. Do NOT widen the regex.
Also report (verify-only, no action): anchor_reach_log prune readiness + leased-.hd-set coherence.
## FENCES
- No crates, no router logic, no tripwire regex change, no new tables. Workflow edit limited to the
  sync step's resolution branch. 0.0.8.6 --park/--unpark stays byte-exact (§3a).
## EXIT-PROOF
- Falsifiers 1+2 bite pre-fix (prove divergence / none-render before, consistency / resolution
  after). doctrine-selftest + gen_orientation --selftest + agent-scan + --check + doc-budget green;
  live 0.0.8.6 --unpark re-proved (19e0e85c8d3f).
- PROBATION LEADS the HC-8 cell in-diff; orientation regenerated; DA stamps graduation at merge.
