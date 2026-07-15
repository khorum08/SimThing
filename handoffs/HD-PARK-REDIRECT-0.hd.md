---
rung: HD-PARK-REDIRECT-0
kind: rung
track: 0.0.8.4.8.4
base_sha: d0d0a8233a6afc4aee4ab0b83f6b26d43eb0b882
audience: coding
model_tier: std
expected_route: DA-RESERVE(gate-wiring)
owner_approved: true
owner_notes: "Owner-directed corrective before HD-C, manual-progression. Goals: no orphaned cruft at redirection; keep lifecycling/deleting cruft as a key CI goal; ONE state home per workplan (absolute EOF block). Owner rulings: rows MOVE out of live TSVs and restore on reopen; in-flight .hd folds into the block (open rung PRs block parking); migrate 0.0.8.6 now."
surfaces: ["scripts/ci/gen_orientation.sh", "scripts/ci/track_closeout.sh", "docs/owner_authoring_guide.md", "docs/design_0_0_8_6_studio_live_ops.md"]
forbidden: ["crates/**", "Studio/UI", "new standalone tables", "handoff_dispatch logic changes", "clearance router changes"]
required_checks: ["gen-orientation-selftest", "track-closeout-prove", "agent-scan", "orientation-check", "doc-budget"]
stop_conditions: ["stale-orient-receipt", "scope-widening", "non-transactional-mutation", "parked-rows-visible-to-live-agents"]
---
## BUILD
- `gen_orientation.sh --park <track.md>` (transactional, HD-6 preflight/staged/rollback pattern):
  inventory the track's scoped rows (binding_conditions, owner_directives, closeout_artifacts
  leases) + its live `handoffs/*.hd.md`; REMOVE them from live TSVs/handoffs and write ONE fenced
  machine block (JSON + PARK-RECEIPT 12-hex + parked_at + parked_from_head + pointer) at absolute
  EOF, past addenda/changelog, marked "agents: read only when executing --unpark"; set header
  PARKED; regen. Re-park = staleness pass (drop superseded entries), overwrite, restamp.
- `--unpark <track.md>`: validate receipt; staleness-check entries (vanished referents -> report
  + drop); re-insert rows, restore .hd files, DELETE the block, set OPEN, flip pointer, regen.
  Virgin target (no block) -> identical to --open today. Open rung PRs for the track BLOCK --park.
- `track_closeout.sh`: refuse closeout while a parked block exists (unpark-first invariant);
  closing deletes any parked section. HD-6 refusal message now points to --park as the path.
- Migrate `design_0_0_8_6_studio_live_ops.md`: real parked block; its track-scoped live rows
  (incl. STUDIO-OWNER-CLOSURE-0) move into it. Owner guide lifecycle section updated (park/unpark).
## FENCES
- Every mutation transactional with rollback fixture proof; single-block-at-EOF enforced by lint;
  no second inventory mechanism — compose existing TSV readers/writers; net scan ledger <= 0.
## EXIT-PROOF
- Fixtures bite: park-moves-rows-out (live TSVs shrink), unpark-restores-rows-byte-exact,
  hd-fold-and-restore, open-PR-blocks-park, re-park-supersede, closeout-refuses-while-parked,
  closeout-deletes-block, virgin-open-unchanged, tampered-receipt-FAILs, post-write rollback.
- 0.0.8.6 migration live on this PR; battery green; PROBATION LEADS the HD-9 cell in-diff;
  orientation regenerated; DA stamps graduation at merge (ruling 6).
