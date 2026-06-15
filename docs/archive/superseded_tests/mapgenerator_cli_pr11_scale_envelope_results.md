# MapGeneratorCLI PR11 — 1000-Star Scale Envelope Results

> **Artifact lifecycle: CURRENT_EVIDENCE** (DA-approved 2026-06-15 after independent audit + a DA heap remediation; promoted from PROBATION).

## Verdict

**PASS — DA-APPROVED (2026-06-15, executive design authority, with a DA min-heap remediation — see sign-off)** — 1000-star elliptical producer generation completes under bounded algorithms; lattice
capacity overflow is fail-closed via u64 math; occupancy relocation no longer rescans the full lattice per insert;
topology/special-route/partition/cluster pair enumeration is capped (`PRODUCER_PAIR_CANDIDATE_CAP` = 65536) with
spatial hyperlane windows where applicable. Generated output parses and lowers lattice for 1000 systems. RF
enrollment/admit/install at 1000 stars remains **blocked** by closed lowerer caps — honestly recorded, not widened.
PR10 tiny-fixture real-adapter GPU compact evidence harness remains live. **Zero** closed `src/` edits
(`simthing-clausething`, `simthing-sim`, `simthing-gpu`, `simthing-driver`, `simthing-spec`).

## PR10 lifecycle correction (Part A)

`mapgenerator_cli_pr10_gpu_compact_evidence_results.md` artifact table row promoted to **CURRENT_EVIDENCE**
(DA-APPROVED & MERGED #690). PR10 scope unchanged: tiny generated admit/install + real-adapter GPU compact evidence only.

## Artifact lifecycle audit

| Artifact | Classification | Action |
|---|---|---|
| `docs/tests/mapgenerator_cli_pr1_params_results.md` through `pr10` | CURRENT_EVIDENCE | Preserved |
| `docs/tests/mapgenerator_cli_pr11_scale_envelope_results.md` | CURRENT_EVIDENCE (DA-approved) | New (this report) |
| `docs/tests/mapgen_pr*_results.md` | CURRENT_EVIDENCE | Preserved baseline |
| `docs/clausething/mapgen_corpus_manifest.md` | PRESERVED BASELINE / CURRENT_EVIDENCE | Unchanged |
| `crates/simthing-clausething/tests/fixtures/mapgen/` | PRESERVED BASELINE | Unchanged |

## Files changed

| Area | Path |
|---|---|
| Lattice u64 capacity | `crates/simthing-mapgenerator/src/lattice.rs` |
| Occupancy free-list | `crates/simthing-mapgenerator/src/occupancy.rs` |
| Bounded pair enumeration | `crates/simthing-mapgenerator/src/pair_candidates.rs` |
| Hyperlane spatial candidates | `crates/simthing-mapgenerator/src/topology.rs` |
| Special-route farthest-cap | `crates/simthing-mapgenerator/src/special_routes.rs` |
| Partition/cluster bounded bridges | `crates/simthing-mapgenerator/src/partition.rs`, `cluster.rs` |
| Producer scale tests | `crates/simthing-mapgenerator/tests/scale_envelope.rs` |
| Integration tests | `crates/simthing-clausething/tests/mapgenerator_cli_pr11_scale_envelope.rs` |
| PR10 lifecycle table | `docs/tests/mapgenerator_cli_pr10_gpu_compact_evidence_results.md` |
| Ladder / production / reference | `docs/design_0_0_8_6_mapgenerator_cli_ladder.md`, `docs/design_0_0_8_1_clausething_production_track.md`, `docs/clausething/MapGeneratorCLI.md` |

## Scale hardening summary

| Risk (carried) | Mitigation |
|---|---|
| `SquareLattice::cell_count` u32 edge² wrap | `cell_count_u64()` + `try_cell_count()` fail-closed |
| `fixture_lattice_edge_for_system_count` overflow | Returns `HyperlaneError::FixtureLatticeOverflow` |
| `OccupancyGrid::insert_relocated` O(cells) rebuild | Precomputed placeable cells + free index list |
| O(N²) hyperlane candidates | `collect_pairs_within_chebyshev` — O(N·(2D+1)²) |
| O(N²) long-range / bridge pairs | `collect_farthest_pairs_with_filter` — **min-heap**, cap 65536 farthest rows, **O(log cap) per pair after cap** (DA remediation — see sign-off) |
| Partition BFS/DFS adjacency | Spatial pairs within Chebyshev 2 |

## 1000-star producer generation proof

- Shape: `elliptical`, procedural, seed `11000`, `num_stars=1000`, `lattice_size=50`
- Topology: hyperlanes, 1 wormhole pair, partition bridges, cluster bridges, 1 nebula
- Emits `static_galaxy_scenario` with 1000 `system` blocks, bounded `add_hyperlane` feedstock, no forbidden terms
- One-system-per-cell and core-mask preserved

## Parse / lattice / link proof status

| Stage | Status |
|---|---|
| `parse_mapgen_neutral_document` | **PASS** — 1000-star text |
| `generate_mapgen_lattice_hierarchy` | **PASS** — 1000 gridcells |
| Hyperlane feedstock extraction | **PASS** — bounded declaration count |
| `generate_mapgen_resource_flow_enrollment` (default) | **BLOCKED** — closed cap / missing deposit feedstock on elliptical initializer refs |
| `generate_mapgen_links` full lower | **DEFERRED** — requires RF enrollment |
| `install_atomic` at 1000 | **NOT CLAIMED** |

## Admission / install / GPU status

| Path | Status |
|---|---|
| 1000-star RF/admit/install | **Blocked** — closed lowerer RF slot cap; not widened in this PR |
| PR10 tiny fixture GPU compact evidence | **Live** — `mapgenerator_cli_pr10_gpu_compact_evidence` unchanged; adapter-required |
| 1000-star GPU compact evidence | **Not executed** — no galaxy-scale install under closed caps |

**Deferred closed-track amendment candidate:** raise RF participant/slot caps or add scalable deposit initializer emission so 1000-star generated packs can admit without bypass — requires DA-authorized 0.0.8.2+ amendment, not producer-only PR11.

## Closed-source / source-change gate

**PASS (producer-only)** — `git diff --name-only master...HEAD` excludes forbidden closed runtime `src/` paths.
Changes are confined to `simthing-mapgenerator/src/` (allowed producer hardening), tests, and docs.

## Dependency boundary

- `simthing-mapgenerator` has no forbidden runtime crate dependencies
- `simthing-clausething` dev-depends on `simthing-mapgenerator` for integration tests only

## Commands run

```text
cargo fmt --all -- --check
cargo test -p simthing-mapgenerator
cargo test -p simthing-clausething --test mapgenerator_cli_pr11_scale_envelope
cargo test -p simthing-clausething --test mapgenerator_cli_pr10_gpu_compact_evidence
cargo test -p simthing-clausething --test mapgen_neutral_ast_parse
cargo test -p simthing-clausething --test mapgen_lattice_hierarchy
cargo test -p simthing-clausething --test mapgen_links
cargo test -p simthing-clausething --test mapgen_resource_flow
cargo test -p simthing-clausething --test mapgen_constitution_guards
git diff --check
git diff --name-only master...HEAD
```

## Test results

| Suite | Result |
|---|---|
| `cargo test -p simthing-mapgenerator` | PASS (includes `scale_envelope` 15/15) |
| `mapgenerator_cli_pr11_scale_envelope` | 7 passed |
| `mapgenerator_cli_pr10_gpu_compact_evidence` | PASS (see validation run) |
| MapGen guard batteries | PASS (see validation run) |

## DA sign-off status

**DA-APPROVED — 2026-06-15, executive design authority (with one DA remediation, below).** Independent
branch-source audit + battery rerun. The three carried scale notes are addressed:
1. **`cell_count` u32 overflow → CLOSED.** `cell_count_u64()` (saturating) + `try_cell_count()` →
   `CapacityOverflow` fail-closed; `cell_count()` saturates instead of panicking; `from_index` honors it.
2. **O(cells)-per-insert relocation → CLOSED.** Occupancy precomputes placeable cells once + a free-index list;
   relocation probes the free list (no full lattice rebuild); `placeable_full_scan_count()` stays 0 (asserted).
3. **O(N²) topology/bridge enumeration → CLOSED (after DA remediation).** Hyperlanes use a windowed position
   index (`collect_pairs_within_chebyshev`, ~O(N·(2D+1)²)). The long-range passes (special routes / partition /
   cluster bridges via `collect_farthest_pairs_with_filter`) examine O(N²) pairs but with **O(1)/O(log cap) per
   pair**.

**DA REMEDIATION applied before sign-off:** as received, `collect_farthest_pairs_with_filter` did an **O(cap)
linear `min_by_key` scan per pair once the 65536 cap was reached** → O(N²·cap) time; at 1000 stars this dominated
and made the scale suite take **~43s** (and was the cliff that hung the 2000-system helper). The DA replaced the
linear min-scan with a **`BinaryHeap` min-heap** keyed on `(distance, left, right)` (evict-smallest-if-larger,
O(log cap) per pair; result sorted for determinism). This is **output-identical** — downstream passes re-sort by
distance-descending and select only the farthest few, and both retain the cap-largest distances; the full
`simthing-mapgenerator` suite (incl. determinism + special-route/cluster/partition unit tests) passes unchanged.
**scale_envelope dropped 43.39s → 1.00s.** This genuinely closes the O(N²) note in *time*, not just memory.

**Honest scope (accepted, not overclaimed):** 1000-star **producer generation** + **parse + lattice lower**
(≥1000 gridcells) are proven; **RF enrollment / admit / install / GPU at 1000 stars remain BLOCKED** by closed
lowerer RF slot caps and are **correctly NOT widened** (that would be a DA-authorized 0.0.8.2.5 amendment, not a
producer PR — flagged as a candidate). The PR10 tiny-fixture real-adapter GPU compact evidence remains the live
guardrail. **Zero `crates/simthing-clausething/src/` changes; no `simthing-*` dep in producer.**

Battery rerun on the branch (post-remediation): `cargo fmt --check` clean; `cargo test -p simthing-mapgenerator`
zero failures; `scale_envelope` 15 (1.00s); `mapgenerator_cli_pr11_scale_envelope` 7; `mapgenerator_cli_pr10_gpu_compact_evidence`
12 (real adapter); `mapgen_constitution_guards` 21; `mapgen_links` 19; `mapgen_resource_flow` 16; `git diff --check` clean.

## Whether 0.0.8.6 can close or needs PR12

**PR12 delivered (PROBATION).** Docs-only closeout ledger, UI handoff, and extensibility note in
[`mapgenerator_cli_pr12_closeout_results.md`](mapgenerator_cli_pr12_closeout_results.md). **0.0.8.6 MapGeneratorCLI
is CLOSED after PR12 DA approval.**
