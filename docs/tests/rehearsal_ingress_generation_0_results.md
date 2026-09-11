# 0088 generation submission and admitted hydration

PROBATION / proof-present / clearance-pending / OPEN / UNMERGED.
Dispatch Board 5627721196; HD-RECEIPT 70df16e1babd;
ORIENT-RECEIPT 28f56884d309; 48-anchor ACK Board 5612017867.
Base 9d2f5d9538442c011cd1f3663c1400502e42307a. Exact tested head and hosted
results are in the PR/Board return. Independent of pending #2037 and #2038.

## Parameter convergence

`GenerationProfile::to_map_generator_params` now owns refreshing the current
editable shape values before registry-scoped parameter selection. The execution
path consumes that same conversion, without a separate refresh step. Presets
remain authoring data; MapGeneratorParams, ShapeRegistry and the existing
producer own validation and generation. No new preset registry or generator.

RED commit 40bc0952eb065e98e5f8f9d43ea1bbb19c5e982a demonstrates the concrete
divergence: after stored jitter 2 is edited to 0.25, direct parameter conversion
still returns 2 while the execution path refreshes it. The owning test fails at
that assertion. The unchanged test passes after the conversion is centralized.
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
-- --test-threads=1`. Both tests PASS. Cargo check, local delta agent scan
(failures=0 / INSPECT=0), inventory drift (969 active / 1538 discovered / 569
parked; no stale/unledgered), lifecycle schema and diff check PASS. Exact final
package and hosted checks are in the PR/Board return. Two new behavior-regression/
AUDIT/ledger-only rows, current-track birth, DSU 0; no pen renewal.

## B1 readiness

LEGACY-STUDIO-MAPGEN-HYDRATION is READY for its pre-authorized internalization
transition once this leaf lands: a single conversion joins the admitted loader;
all public wrappers preserve producer geometry, links, naming and compatibility.
LEGACY-STUDIO-GENERATION-PRESETS is READY: direct and executed requests share one
MapGeneratorParams conversion and the producer's registry validation. Existing
preset names, profiles, shape storage and generation modes remain compatible.

Together with merged #2032/#2033/#2034 (full authored bridge, canonical projection
plus declared dependencies), pending #2037 (plain JSON) and #2038 (literal-install
and vertical-seed successors), this supplies the remaining B1 family evidence.
Orchestration owns review/merge and relays any ready census increment to the DA.
No protected table, coupled retirement, historical fixture, engine/spec, Cargo,
gate, triage or graduation edit is made here. This is not a rung-closeout stamp.
