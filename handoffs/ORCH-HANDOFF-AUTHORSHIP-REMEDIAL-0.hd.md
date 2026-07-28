---
rung: ORCH-HANDOFF-AUTHORSHIP-REMEDIAL-0
kind: remedial
track: 0.0.8.7
base_sha: e5d2df7f6c2eb4e84c5bff0808c0d9e497d8bc49
audience: orchestrator
model_tier: std
owner_approved: true
expected_route: ORCHESTRATOR-CLEARABLE
owner_notes: "Owner-directed remedial. The orchestrator waited on a DA-authored 4.2 handoff (board 5107391066). That inverts the written scribe division. Revert to normal behavior and draft the 4.2 handoff yourself."
surfaces: ["handoffs", "docs/orchestrator_orientation.md"]
forbidden: ["waiting on DA-composed handoff bodies", "coder dispatch before the merged 4.2 handoff + fresh HD-RECEIPT are board-visible", "any protocol-doc edit (division is held in operating context per Owner)"]
required_checks: ["clearance", "doctrine-scan"]
stop_conditions: ["stale-orient-receipt", "scope-widening"]
---
## BUILD
- Correction: the written protocol (agent_onboarding, Prompt protocol) assigns
  `.hd` authorship to the orchestrator-scribe. DA authorship of the 3.1/4.1
  canonicals was exceptional and Owner-ruled NOT precedent. Waiting for a
  DA-authored handoff is a protocol inversion; do not repeat it.
- Re-orient: re-read `docs/orchestrator_orientation.md` at current head (it
  regenerated at the 4.1 graduation, master `e5d2df7f`) and carry its embedded
  receipt before drafting.
- Draft `handoffs/PLAN-STRUCT-TYPING-0.hd.md` yourself from the SS3b ladder row
  and the 4.1-established boundary: `ColumnIndex` end-to-end through plan
  structs; raw `u32` collapse only at the governed `wgsl_encode` boundary; no
  absorption of the 9.2 legacy-mint sweep; corpus/referee gates per track law;
  committed lane Std - Grok.
- Authoring budget: the coding render cap is 60 physical lines; keep authored
  content (body + stop/checks/forbidden items) near 45 lines. The Owner's
  cap raise to 90 lands in Phase 10.1, not now.
- Open the draft as a PR and route it through the clearance ladder. NO DA
  draft-review relay on Std-lane precedented rungs: on a clearable verdict,
  self-merge, refresh the Board with the canonical HD-RECEIPT, and issue the
  pointer-only Grok dispatch "Implement handoff PLAN-STRUCT-TYPING-0" without
  waiting on the DA. Relay a draft to the DA only when the router returns
  DA-RESERVE or the rung's committed lane is frontier/MAX.
## FENCES
- The DA does not compose handoff bodies (Owner ruling, token economy). The
  DA's pass on your draft is review-and-issue: targeted corrections, receipt.
- `owner_approved` rides the Owner's standing word for precedented rung
  dispatches; DA-issued corrections are binding before merge.
## EXIT-PROOF
- Orchestrator acknowledgment carrying a fresh orientation receipt; the
  `PLAN-STRUCT-TYPING-0.hd.md` handoff is merged, board-visible with its
  receipt, and the Grok dispatch is issued — no DA draft relay occurred.
