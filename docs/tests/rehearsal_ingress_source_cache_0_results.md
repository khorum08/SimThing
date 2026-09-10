# 0088 ingress native source/cache leaf

PROBATION / proof-present / clearance-pending / OPEN / UNMERGED. This is a bounded
leaf of 0088-INGRESS-FIDELITY-0, stacked on the bridge/UI leaf. It does not close
the rung or claim the legacy census is ready for transition.

Dispatch: Board 5619313604. HD-RECEIPT: 70df16e1babd.
ORIENT-RECEIPT: 28f56884d309; 48 required anchors ACKed at 5612017867.
Base: 64524a81fc5eb5bce1c9688c36dc1a4048335242 (bridge/UI PR #2032).
The leaf's exact tested head and hosted verdicts travel in its PR/Board return.

## Behavior and authority

Native path ingestion now uses the existing parse/expand/hydrate/rebind chain,
including document-local variables. The picker writes a reproducible JSON source
cache and reloads through that same chain. Studio's JSON path adapter recognizes
the cache and builds its authored live profile from the admitted native result.
Plain ScenarioSpec JSON remains the existing structural-record format; it cannot
recover authored programs already discarded by an old export.

The cache uses the existing RawDocument type, original source identity, relative
native path, and content identities for declared dependencies. It is not another
hydrated program or executable authority: load re-reads and re-admits native source,
compares the raw document and dependency manifest, and refuses mismatches. JSON edits
cannot replace the native economics. Source and dependency identities use FNV-1a64
plus byte length as stable freshness checks, not authentication or signatures.

The committed specimen declares its embedded base by relative source_json path and
pins its bytes in the sibling .dependencies.json file. Existing explicit source_json
or include_json declarations are normalized into the same cache manifest without
requiring author rewrites. An optional sibling manifest must match all declared
dependencies exactly; its own identity is also retained in the cache. Cache paths
are relative to the cache, dependency and compatibility resolver paths to the native
source directory. The shipped specimen needs no resolver token. Native hydration
checks dependency identities both before and after rebind.

Save Scenario retains native-cache provenance. If the canonical document or authored
program/tree/targets differ from the loaded source, saving refuses before writing;
edit the native source and reload to regenerate the cache. Source/dependency drift
also refuses. Save Candidate explicitly refuses native authored sessions because
that legacy runtime-candidate writer currently omits their program. This is a
visible limitation, not a continuation-fidelity claim.

## Owning proof

`crates/simthing-mapeditor/tests/rehearsal_ingress_source_cache.rs` owns three
behavior-regression AUDIT / ledger-only rows, birth 0.0.8.8, DSU 0:

1. `rehearsal_ingress_native_cache_preserves_born_values_and_policy_changes`
   installs the committed native bundle and its generated JSON cache in ordinary
   Studio live sessions, ticks each, and compares canonical hosted observations.
   Ten child quantities and two owner stockpiles agree. Zero-valued PRESENT children
   remain observable. Baseline alpha/beta stockpiles are 12/13; each owner's child
   quantities are 10, 12, 14, 16, 0. Independently changing beta_3's authored amount
   from 8 to 20 yields 40 at beta_3; changing beta's authored policy from 3 to 7
   yields stockpile 17. Both routes agree in each variant. Cache regeneration is
   byte-reproducible, source drift and raw-document tampering refuse, and edited
   documents/programs cannot be silently replaced by an old cache on save.
2. `rehearsal_ingress_bundle_relocates_and_refuses_dependency_drift` copies the
   source, base, manifest and cache to another directory, then opens them in a child
   process whose working directory is a third directory. It subsequently changes
   base bytes and witnesses refusal through native and JSON routes.
3. `rehearsal_ingress_source_refusals_retain_file_element_and_span` table-checks
   an unsupported modifier field and unsupported scenario recipe. Each refuses at
   native ingestion with the rung law, source file, offending element and token span,
   before any Studio session is activated. The original valid source still loads.

The specimen is `scenarios/rehearsal_ingress_cache.clause` with its adjacent base
and dependency manifest. Its field economy is an intermediate cache/portability
witness; it is not the complete unequal-depth RF discriminator.

## Validation and remaining work

Run full touched-package tests with `cargo test -p simthing-mapeditor --no-fail-fast
-- --test-threads=1`; exact-head results, lifecycle schema/drift and scan findings are
reported on the PR and Board. No penned referee is consumed or renewed by this leaf.
The retained post-RF publication witness and two World-root historical records are
included in the package run without contract changes.

Still required for the rung: one committed native/cache source with the full
two-owner, unequal-depth RF graph and policy-sensitive born outputs; historical
stage successors; hydration/generation compatibility conformance; and evidence for
the census transitions, including literal-install and vertical-seed retirement.
No engine, spec, gate, Cargo or census edits occur here. Orchestration owns every
scan-id INSPECT triage and the ordinary O* ritual; no direct DA relay or self-merge.
