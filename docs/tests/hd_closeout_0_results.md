# HD-CLOSEOUT-0 — Closeout report (0.0.8.4.8.4 HD Board)

**Status: COMPLETE** — DA-closed 2026-07-15 (Fable). Track closed via manual DA closeout
(no ripe inventory rows; the track's fixtures are permanent `harness-fixture` class).

## Cruft reaped
- 8 graduated `.hd` operational artifacts (276 lines): all HD-* handoffs (§1.5 lease-on-graduation → reap at close).
- 2 expired prior-track closeout manifests (orphaned; both source tracks CLOSED):
  `0.0.8.5-terran-pirate_closeout_manifest.tsv` (6d, TP CLOSED 07-09), `oc_closeout_0_manifest.tsv` (5d, OC CLOSED 07-10).
- 10 lease rows removed from `closeout_artifacts.tsv` (now header-only; 0 live leases).
- Ladder cell history folded: 12,250 → 3,058 chars (deep-pass/remand narratives → stamp+evidence).

## Prose-delta (net corpus DECREASE — HD-CLOSEOUT-0 satisfied)
Agent-facing doctrine/onboarding surface, track base (`2738650d`, pre-#1325) → close:
| Doc | base | close | Δ |
|---|---|---|---|
| handoff_template.md | 360 | 112 | −248 |
| agent_onboarding.md | 122 | 148 | +26 |
| owner_authoring_guide.md | 0 | 40 | +40 |
| agents.md | 179 | 189 | +10 |
| ci_screening_surface.md | 523 | 525 | +2 |
| **subtotal** |  |  | **−170** |
Plus reaped this close: −276 `.hd` lines, −9,192 chars ladder history, −71KB prior manifests.
Binding form ("template + reading-list deletions ≥ additions"): −248 template alone ≥ +78 additions. **Discharged.**

## Bindings discharged
- HD-TRACK-OPEN-0 (discharged at open, 2026-07-12).
- HD-CLOSEOUT-0 (this close): net-corpus-prose-must-decrease — proven above.

## Lease sweep
`track_closeout.sh --artifact-expiry` pre-close flagged 3 cruft leases; all reaped. Post-close: 0 leases.

## Survivors (justified)
- 9 graduated design-doc rungs (compressed historical record).
- `harness-fixture` selftest corpus (permanent-residue, seal-proof class) — the substrate's own proofs.
- All HD machinery live in `scripts/ci/` (gen_orientation park/unpark, librarian, handoff_dispatch, clearance router).
