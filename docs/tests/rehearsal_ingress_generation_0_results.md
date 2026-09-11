# 0088 generation submission and admitted hydration

PROBATION / proof-present / clearance-pending / OPEN / UNMERGED.
Implementation dispatch Board 5627721196; revalidation Board 5628679514;
HD-RECEIPT 70df16e1babd;
ORIENT-RECEIPT 28f56884d309; 48-anchor ACK Board 5612017867.
Current base 9f09f24036c26cf7de6214682a3e98ca5ac3489c includes merged #2037.
The exact tested commit, hosted results and current Clearance verdict are
recorded together in PR #2039 and its Board return as `tested_code_sha`. This
packet describes only the generation/hydration leaf; #2038 is separate.

## Parameter convergence

`GenerationProfile::to_map_generator_params` now owns refreshing the current
editable shape values before registry-scoped parameter selection. The execution
path consumes that same conversion, without a separate refresh step. Presets
remain authoring data; MapGeneratorParams, ShapeRegistry and the existing
producer own validation and generation. No new preset registry or generator.

RED commit 40bc0952eb065e98e5f8f9d43ea1bbb19c5e982a demonstrates the concrete
divergence: after stored jitter 2 is edited to 0.25, direct parameter conversion
still returns 2 while the execution path refreshes it. The owning test fails at
that assertion. The RED is preserved after rebase as 568642092ae8d49298005cad60643857ab965ded.
The unchanged test passes after the conversion is centralized.
All four active presets are exercised with bounded 16-system requests; direct
submitted shape parameters, shape, seed and star count match the producer report.

## Hydration convergence and compatibility

All six existing generation/hydration entry points already meet in the single
`hydrate_mapgen_result_into_simthing_spec_with_star_names` conversion. That
conversion now consumes `load_scenario_spec_from_json_str` and honors
`ingestion_ready` before returning a generated authority object. The admitted
spec loader owns validation/classification; the UI projections remain derived.
The handoff's existing representations are reused, with no new authority or IR.

Generated topology without authored ownership remains a legacy World structural
producer record under the spec loader's admitted compatibility profile. The
adapter does not fabricate an Owner or relabel that record as authored live
economics. Authored native scenarios retain the canonical route demonstrated by
#2032–#2034. Session source variants continue to carry provenance only.

`rehearsal_ingress_generation_adapters_preserve_admitted_structural_records`
compares every generated system's id/row/column and every hyperlane against the
producer result through four authority wrappers, admitted serialization/reload
and Studio reconstruction. Both projection wrappers preserve those cells/links;
the named-star wrappers preserve authored display names. Compatibility admission
and absence of an invented authored live profile are explicit assertions.

Owning tests: crates/simthing-mapeditor/tests/rehearsal_ingress_generation.rs.
Reproduce: `cargo test -p simthing-mapeditor --test rehearsal_ingress_generation
-- --test-threads=1`. The accepted implementation at
`d1f348856d037b578d0458fcb062cbfb5400d4da` passed both owning tests and the full
mapeditor package (12 passed, 0 failed, 0 ignored), plus local and hosted scans.
Those earlier runs do not establish clearance for the current master.

## Current-base revalidation

The branch was rebased onto #2037's merge above. The only conflict was the test
inventory append; both JSON rows and both generation rows were preserved. The
production files and owning generation tests are byte-identical to the accepted
implementation. Inventory drift passes with 971 active / 1540 discovered / 569
parked, no stale or unledgered tests; lifecycle schema passes. This leaf retains
two new behavior-regression/AUDIT/ledger-only rows, current-track birth, DSU 0;
there is no pen renewal or change to existing lifecycle dispositions.

Current clearance requires cargo check, the local delta scan, owning generation/
hydration tests, the full mapeditor package including the merged JSON tests, and
the exact-head hosted Doctrine Scan. The PR/Board return binds their final results
to the final committed `tested_code_sha` and records the freshly evaluated
Clearance verdict. Hosted scan evidence includes its artifact and executed-step
conclusions; a green job with a skipped delta scan is not proof. Orchestration
owns review and merge after these current-head checks pass.

## Conditional transition evidence

For LEGACY-STUDIO-MAPGEN-HYDRATION, the leaf provides one conversion through the
admitted loader, with all public wrappers preserving producer geometry, links,
naming and compatibility. For LEGACY-STUDIO-GENERATION-PRESETS, direct and executed
requests share one MapGeneratorParams conversion and producer registry validation.
Existing preset names, profiles, shape storage and generation modes remain
compatible. The prior census-transition signals remain conditional on this leaf
and the coupled census increment landing; this packet does not advance the census.

This evidence makes no all-B1 readiness, rung-complete or graduation claim. Other
leaf proofs remain separate. No new authority, IR, registry, protected table,
retirement/deletion, historical fixture, engine/spec, Cargo, gate, triage or
graduation edit is made here. Ordinary revalidation returns to Orchestration.
