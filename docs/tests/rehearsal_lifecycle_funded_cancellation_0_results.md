# Canonical funded cancellation — placement retirement + bound-session shrink + THIRTEENTH E8 roll

DA increment per Orchestration relay `5705632907` (Astra return `5705551059` accepted).
**Model 1 remains UNDECIDED; Model 2 remains CLOSED.** Seventh admitted substrate gap:
the two ownership halves of one canonical cancellation boundary — placement never retired
on canonical `Remove`, and the #2067 acceptance law had (correctly) no shrink clause.

## A. Exact placement retirement on canonical Remove

`apply_remove` retired slots/relations/tombstones but never the acquired committed
residency placement — `committed_residency_placement` kept returning the generation-3
quantity-2 grant after the subtree was gone. Repair, inside the SAME canonical boundary
lifecycle: the allocator retires exactly the commitments owned by the removed subtree —
every commitment whose grantee is a removed identity (freeing its extent inside the
surviving granter's level), every committed level whose granter is removed, and every
root extent a removed granter declared. Identity-keyed (grant identity carried in the
commitment), never slot-number inference, never broad reset; parent/sibling commitments
untouched; no refund, no synthetic stock credit, no second ledger — the existing
grant/residency lifecycle authority expresses retirement (relay rules A1–A6).
**Reusability (A7) proven**: after retirement the freed extent admits a NEW grantee and
the surviving level audits clean against its containing extent.

## B. Bound-session canonical SHRINK successor (#2067 extended symmetrically)

Baseline succession stays ONLY in the post-canonical-boundary acceptance point;
pre-hot-cycle whole-shape equality unchanged (B1). The pure law gains removal evidence —
`outcome.maintainer.tombstoned`, the canonical maintainer's OWN typed record of this
exact boundary's removals (existing artifact; no new evidence surface, no table-diff
inference): a missing accepted row is lawful only with same-boundary tombstone evidence
for that identity (B2/B5); a bound-footprint identity stays typed `BoundIdentityRemapped`
even WITH removal evidence (B3); every surviving row must be identically mapped (B4);
registry/dimension/resident fences exact (B6); mixed add+remove needs BOTH proof classes
(B7); no enrollment mutation, no reinstall, fail-stop and single ownership untouched
(B8/B9).

## Proof floor executed (reference machine)

- **Pure ActionBand law battery 9/9** (5 prior witnesses re-proven under the new
  signature + 4 shrink witnesses, all ledgered): removal evidence admits unbound shrink /
  absence is stale; bound removal stays typed even with evidence; surviving remap stays
  stale even with evidence; mixed add+remove lawful only with both classes; all #2067
  additive-growth cases unchanged.
- **Pure placement battery** (ledgered): removed subtree retires exactly its placements
  (as grantee AND as granter, root extents included), sibling survives, freed extent
  reusable and audit-clean; unrelated removal retires nothing.
- **#2062's UNCHANGED suites at exact tested head `1cd598bd`: recipe 5/5 + WIP 3/3
  GREEN** — the funded-cancellation witness passes all 8 permutations: G3 funded birth,
  G4 canonical Remove retires subtree + relations + EXACT placement, binding table
  returns to the pre-birth table and is ACCEPTED (shrink law), G5/G6 continue healthy;
  consumed materials stay consumed, residual WIP stays owned; external AddChild controls
  isolate the seams.
- Embedder ingress plants green; driver lib witnesses green.

## THIRTEENTH E8 roll (`session.rs` in bundle; `boundary.rs` untouched)

- **Old-pin refusal (RED, required first), repaired source, reference tuple:** ordinary
  ingress `UnqualifiedAdapter { required: 7493919273263963470, observed:
  17030971329449224085 }`; parity referee FAILED with
  `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ec5a2a30afaee795` — same observed value
  derived independently twice.
- **Roll:** both literals atomically `0x67ff_bfdf_e25a_1d4e` → `0xec5a_2a30_afae_e795`;
  no third site; no comparator/component-list/`build.rs`/Cargo change.
- Pin chain: `…0xf65d…` → `0x67ff…` → `0xec5a_2a30_afae_e795`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `c10e2e9a` (the exact thirteenth-roll commit)
- command: fresh `git clone` at that exact commit (canonical LF checkout), sibling
  directory `simthing-e8-roll13-verify`, never %TEMP%;
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ec5a2a30afaee795`; referee
  `1 passed; 0 failed` — pinned fingerprint reproduced from the fresh canonical-LF clone.
