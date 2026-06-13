# SimThing 0.0.8.1 — PALMA Pathfinding Integration Guide

> **Status: DESIGN GUIDE / READY FOR REVIEW (2026-06-11, PALMA-PATH-0).** This document is the
> 0.0.8.1 integration guide for **semiring relaxation fields** over Location-owned gridcell SimThings.
> It is **not** a pathfinding engine specification, **not** constitutional authority, and **not** an
> implementation. PALMA-PATH-1 adds a tiny CPU oracle proof; PALMA-PATH-2+ are gated separately.
>
> **Authority order (binding):**
> 1. [`agents.md`](agents.md)
> 2. [`simthing_core_design.md`](simthing_core_design.md)
> 3. [`invariants.md`](invariants.md)
> 4. [`design_0_0_8_1.md`](design_0_0_8_1.md) §0 and §2
> 5. [`design_0_0_8_1_clausething_production_track.md`](design_0_0_8_1_clausething_production_track.md) when ClauseThing is in scope
> 6. Resource Flow / RegionField / StructuredFieldStencilOp docs and tests
> 7. PALMA paper sections listed in §3 — **inspiration for semiring algebra only**

**Paper (inspiration, not constitution):**
[PALMA: A Lightweight Tropical Algebra Library for ARM-Based Embedded Systems](https://arxiv.org/html/2601.17028v1)
— SIMD-Accelerated Semiring Linear Algebra with Spectral Analysis and Embedded Case Studies.

If the paper conflicts with `simthing_core_design.md` or `design_0_0_8_1.md`, **SimThing docs win**.

---

## 1. Motivation

Grand-strategy and logistics scenarios need **traversal-cost awareness** over Location grids —
where convoys can go cheaply, where pirate pressure blocks corridors, where fuel shortage raises
effective distance — without introducing a **privileged pathfinding engine** beside the SimThing tree.

The PALMA paper shows that **single-source shortest-path relaxation** is a **min-plus semiring
linear-algebra** problem: repeated sparse matrix–vector multiply under the tropical `(min, +)` algebra
converges to least-cost potentials. SimThing already computes **Location-owned heatmap fields every
tick** via Resource Flow pressure, overlays, and `RegionField` / `StructuredFieldStencilOp` bands.

**The SimThing-native trick:** ride min-plus **D** (traversal-cost / movement-potential) relaxation on
the **same grid pass** that already maintains **W** (local impedance) from ordinary SimThing state.
Traversal-cost evaluation becomes a **stowaway** on a Location heatmap band the engine already wants —
not a separate graph service.

This does **not** make pathfinding literally free. It makes **field-based cost evaluation** cheap
relative to per-mover CPU route queries at scale. CPU pathfinding remains the **oracle/fallback**
for small mover counts, static maps, and proof gates.

---

## 2. Binding SimThing authority

| Doctrine | Application |
|---|---|
| Everything is a SimThing | Locations and gridcells are SimThings; movables are SimThings |
| No pathfinding engine | No route planner, graph manager, or A* service |
| No movement engine | Movement commitment stays threshold / BoundaryRequest over fields |
| No CPU planner | Decisions emerge from threshold crossings, not search |
| Resource Flow opt-in | Arena pressure feeds **W** only when explicitly admitted |
| RegionField opt-in | Stencil bands explicit; presence alone inactive |
| Scenario proof | Real reduction path + CPU oracle; standalone math is oracle-only |

Related accepted surfaces: [`adr/resource_flow_substrate.md`](adr/resource_flow_substrate.md),
[`clausething/ct_3b_4a_movement_front_heatmap_memo.md`](clausething/ct_3b_4a_movement_front_heatmap_memo.md)
(RF-fed heatmap spine — complementary, not superseded).

---

## 3. Paper link and relevant sections

**Read for this guide:**

| Section | Borrow |
|---|---|
| §2.2 Tropical Semirings | `(min, +)` min-plus algebra definition |
| §2.3 Tropical Linear Algebra | Matrix–vector multiply as relaxation |
| §2.6 Connection to Classical Graph Algorithms | Shortest-path as tropical closure |
| §3.2.4 Sparse Matrix Structure (CSR) | Bounded adjacency on grids |
| §4.3.2 Sparse Matrix-Vector Multiplication | One relaxation step |
| §5.2 Single-Source Paths | Single-source seeding and iteration |
| §6.6 Experiment 5: Algorithm Comparison | CPU field vs per-query cost intuition |

**Explicitly excluded from design authority:**

- §4.2 ARM NEON SIMD Optimization
- Raspberry Pi / ARM benchmark claims
- Embedded C implementation details
- PALMA API naming and hardware layout

**Borrow the algebraic trick, not the ARM implementation.**

---

## 4. “Borrow algebra, not ARM implementation”

PALMA’s value for SimThing is **semiring mode selection** and **relaxation iteration count** over
**admitted sparse adjacency** — not its NEON kernels or embedded deployment story.

SimThing will implement relaxation through:

- existing **Location/gridcell property columns** (CPU oracle first);
- later **JIT EML / WGSL** over the same buffers as `StructuredFieldStencilOp` when parity exists;
- **never** a PALMA C library or ARM-specific layout as constitutional substrate.

---

## 5. Everything is a SimThing

| Concept | SimThing reading |
|---|---|
| Location | Node-parent SimThing; owns grid mapping role and local flow/pressure context |
| Gridcell | Child SimThing under Location; carries **W**, **D**, pressure, blockade, reachability columns |
| Terran convoy | Ordinary mobile SimThing; samples **D** locally later — not a planner client |
| Pirate fleet | Mobile SimThing whose presence raises **W** on nearby gridcells via overlays/RF pressure |
| Blockade | Overlay or pressure column raising **W** in corridor cells — not a “blockade engine” |
| Fuel shortage | Property/overlay raising **W** distally — numeric field only |

No global graph registry. Adjacency is **Location-owned slot-range stencil topology**, admitted at spec time.

---

## 6. Location / gridcell ownership of fields

**Location SimThings:**

- are **node-parents** containing gridcell children;
- own **RegionField** / stencil band registrations for the local lattice;
- evaluate **local property stacks** (Resource Flow aggregates, overlays, masks);
- produce **heatmap-related columns** (suppression/disruption pressure per CT-3b+4a) and **impedance W**.

**Gridcell SimThings:**

- are **child SimThings** of a Location;
- carry scalar (or small vector) columns such as:
  - `W` — local impedance / traversal cost multiplier;
  - `D` — traversal-cost / movement-potential field (min-plus relaxation output);
  - pressure columns (hostile, blockade, fuel shortage) that **compose into W** upstream;
  - optional reachability (Boolean semiring mode);
- participate in **Location-owned stencil bands** — not in a detached graph structure.

---

## 7. Movable SimThings as field samplers, not planner clients

Movable SimThings (convoys, fleets, leaders, agents):

- remain **ordinary SimThings** with location, position `(x, y)`, destination `(x, y)`, movement-intent properties;
- may carry children and later benefit from table/LIFO/subtree move optimization;
- **do not** call a pathfinding API;
- **sample** local **D** (and optionally local gradient of **D** or desirability fronts) when a
  **generic movement/commitment path** already exists;
- fire **BoundaryRequest** only through existing threshold/reparenting mechanics — not through route objects.

If no generic movement commitment path exists for a fixture, **stop and ledger the blocker** — do not invent one (PALMA-PATH-3 rule).

---

## 8. Min-plus recurrence (binding convention)

**Chosen convention — cell-entry (node-weight) form:**

```text
D_next[cell] = W[cell] + min_{neighbor ∈ N4(cell)} D_current[neighbor]
```

**Definitions:**

- **D** — traversal-cost / movement-potential scalar field (cost from destination backward, or
  toward destination depending on seed — PALMA-PATH-1 uses **destination seed** `D[dest] = 0`).
- **W** — local impedance field on each gridcell; finite non-negative in normal cells; `+∞` sentinel
  for blocked/ unreachable cells if authored.
- **N4** — four-neighbor adjacency on the admitted square grid (8-neighbor / hex deferred).

**Boundary:** destination cell holds `D[dest] = 0` each iteration (single-source seed per PALMA §5.2).

**Alternate convention (not used in v1 proof):**

```text
D_next[cell] = min_{neighbor ∈ N4(cell)} (D_current[neighbor] + W[neighbor])
```

(edge-weight on neighbor entry). Pick one convention per band; do not mix within a session.

The min-plus update sees **only numeric fields and neighbor indices**. It does not branch on “fleet,”
“pirate,” “blockade,” or “fuel.”

---

## 9. W impedance composition from Resource Flow / overlays / hostile pressure

**W** is composed from **ordinary SimThing state** before the min-plus band runs:

| Source (authoring/runtime column) | Effect on W |
|---|---|
| Base terrain impedance | Property column on gridcell |
| Resource Flow congestion / arena pressure | Reduced RF column bound to cell (CT-3b+4a spine) |
| Hostile fleet pressure | Overlay or pressure column → Add/Mult on W |
| Blockade overlay | Raises W in corridor cells |
| Fuel / supply shortage | Raises W in affected cells (distal overlay sweep) |
| Masks | Zero or gate W contribution |

Composition uses **existing overlay OrderBands and EML gadgets** — not semantic `if pirate then …`
branches in the relaxation kernel.

Example composition (illustrative, not a new opcode):

```text
W = terrain_base + blockade_overlay + hostile_pressure_scale * hostile_pressure + rf_congestion_term
```

(clamps and SELECT guards via EML where needed).

---

## 10. Stowaway advantage (honest)

| Claim | Honest reading |
|---|---|
| “Free pathfinding” | **False** — relaxation still costs iterations × cells |
| “Stowaway on heatmap pass” | **True** — **W** already maintained for pressure/suppression; **D** band adds one min-plus stencil pass over the same buffers |
| “Scales to many movers” | **Often true** — one field update serves all samplers; per-mover CPU A* does not |
| “Always beats CPU routing” | **False** — small mover counts + static maps may favor CPU oracle/fallback (PALMA §6.6 intuition) |
| “Replaces FIELD_POLICY gradient movement” | **False** — complementary; gradient sit/step and min-plus **D** can coexist on different columns |

---

## 11. What this is not

- Not a **pathfinding engine** or route planner
- Not a **movement engine** or CPU assignment planner
- Not a **combat**, **economy**, or **semantic SEAD** engine
- Not **ClauseScript runtime** interpretation
- Not **presentation-only** heatmap state
- Not **all-pairs shortest paths** every tick on large maps (forbidden without explicit bounded consumer)
- Not **exact full-map shortest path** required every tick — fixed iteration count + dirty regions are allowed
- Not a **detached graph manager** — adjacency lives in Location/gridcell admission only

---

## 12. Semiring mode table

| Mode | Algebra | SimThing use |
|---|---|---|
| **Min-plus** `(min, +)` | Tropical shortest path | **D** traversal cost / least resistance (PALMA-PATH v1) |
| **Max-plus** `(max, +)` | Dominance propagation | Influence / strongest accumulated pressure front |
| **Max-min** `(max, min)` | Bottleneck | Corridor capacity / max-min cut intuition |
| **Boolean** `(or, and)` | Reachability | Legal adjacency / can-reach-without-block mask |
| **Min-max** `(min, max)` | Worst-link minimization | Risk-aware corridor (deferred) |

PALMA-PATH-1 implements **min-plus only**. Other modes are documented for future bands; each requires
its own consumer and CPU oracle.

---

## 13. GPU / JIT / WGSL policy

**JIT EML / WGSL is allowed and expected later** for bounded field arithmetic — same posture as
CT-3b+4a ORIENT-0 amendment.

**Allowed later (PALMA-PATH-2+):**

- min-plus neighbor relaxation over admitted Location/gridcell buffers;
- impedance **W** composition kernels (numeric only);
- copy/scatter/gather between property columns and stencil buffers;
- fixed-iteration bands;
- bounded dirty-region updates;
- CPU oracle parity tests before proof gates close.

**Forbidden:**

- ClauseScript interpretation on GPU;
- semantic category / noun dispatch;
- participant discovery or hidden enrollment;
- movement policy or route planning in WGSL;
- pathfinding engine semantics;
- semantic SEAD engine.

Spec/admission/install defines buffers and bounds; GPU code **computes**, never **decides meaning**.

---

## 14. Exactness / sqrt policy

**Min-plus relaxation does not require sqrt or magnitude.**

If future **W** composition or movement sampling uses distance, gradient norm, or Euclidean magnitude
and claims **exactness** or closes a **bit-exact parity gate**, route through `m_jit_sqrt_f_exact`
(Candidate F). Native WGSL `sqrt` is `ApproximateJitOnly` / diagnostic only and cannot close exact
proof gates.

PALMA-PATH-0/1 do not exercise sqrt paths.

---

## 15. Quick PR proof ladder

| Rung | ID | Scope | Status |
|---|---|---|---|
| 1 | **PALMA-PATH-0** | Integration guide | **ACCEPTED / GUIDE** |
| 2 | **PALMA-PATH-1R** | Hardened CPU oracle: detour/gap bend, INF cut, scalar field only | **IMPLEMENTED / PASS** — [`tests/palma_path_1_cpu_oracle_results.md`](tests/palma_path_1_cpu_oracle_results.md) |
| 3 | **PALMA-PATH-2** | GPU/JIT min-plus stencil; CPU parity | **IMPLEMENTED / PASS** — [`tests/palma_path_2_gpu_min_plus_results.md`](tests/palma_path_2_gpu_min_plus_results.md) |
| 4 | **PALMA-PATH-3** | Terran convoy / pirate fleet field sampling (numeric + GPU) | **PARTIAL / NUMERIC+GPU FIXTURE PASS** — live SimThing tree deferred to PATH-3R — [`tests/palma_path_3_terran_pirate_fixture_results.md`](tests/palma_path_3_terran_pirate_fixture_results.md) |
| 5 | **PALMA-PATH-3R** | Admitted Location/gridcell/convoy tree + generic Reparent | **IMPLEMENTED / PASS** — [`tests/palma_path_3r_simthing_tree_fixture_results.md`](tests/palma_path_3r_simthing_tree_fixture_results.md) |
| 6 | **PALMA-PATH-4** | Toy-axis benchmark (32–128 grids) | **IMPLEMENTED / PASS** — [`tests/palma_path_4_benchmark_results.md`](tests/palma_path_4_benchmark_results.md) |
| 7 | **PALMA-PATH-4S** | Stellaris-scale 180×180 / 150-fleet representative workload | **IMPLEMENTED / METRICS REMEDIAL PASS** — [`tests/palma_path_4_stellaris_scale_benchmark_results.md`](tests/palma_path_4_stellaris_scale_benchmark_results.md) |
| 8 | **PALMA-PATH-5** | Admitted Location/gridcell property-column integration | **IMPLEMENTED / PASS** — [`tests/palma_path_5_install_session_property_results.md`](tests/palma_path_5_install_session_property_results.md) |
| 9 | **PALMA-PATH-6** | Opt-in session/RegionField min-plus band over W/D columns | **PARTIAL / TEST-PROFILE PASS** — [`tests/palma_path_6_session_regionfield_results.md`](tests/palma_path_6_session_regionfield_results.md) (default `SimSession` tick not wired) |
| 10 | **PALMA-PATH-7** | Production GPU traversal utility seating | **IMPLEMENTED / PASS** — [`tests/palma_path_7_gpu_traversal_utility_results.md`](tests/palma_path_7_gpu_traversal_utility_results.md) |
| 11 | **PALMA-PATH-8** | GPU-native W input / D output field graph connection | **IMPLEMENTED / PASS** — [`tests/palma_path_8_gpu_native_field_graph_results.md`](tests/palma_path_8_gpu_native_field_graph_results.md) |
| 12 | **PALMA-PATH-8R** | Remove public `tick()` scaffold; explicit GPU dispatch | **IMPLEMENTED / PASS** — [`tests/palma_path_8r_remove_tick_scaffold_results.md`](tests/palma_path_8r_remove_tick_scaffold_results.md) |
| 13 | **PALMA-PATH-8R-CLEAN** | Remove public PALMA legacy field-band aliases | **IMPLEMENTED / PASS** — [`tests/palma_path_8r_cleanup_results.md`](tests/palma_path_8r_cleanup_results.md) |
| 14 | **PALMA-PATH-9** | Downstream GPU probe consumes resident D (compact readback only) | **IMPLEMENTED / PASS** — [`tests/palma_path_9_downstream_gpu_consumer_results.md`](tests/palma_path_9_downstream_gpu_consumer_results.md) |

One rung per PR. Codex/Cursor must not attempt the full ladder at once.

---

## 15a. Production seating (PATH-7 — generic GPU utility)

PALMA names the **semiring algebra provenance** in docs. Runtime code uses the generic GPU traversal field utility:

```text
W impedance (property/buffer)
  → MinPlusTraversalFieldOp / TraversalFieldBandSession
  → D traversal potential (GPU-resident by default)
  → downstream GPU EML / threshold / field consumers (future)
  → CPU only via explicit diagnostic readback or committed BoundaryRequests
```

**Default production mode:** explicit `dispatch_gpu_resident` with `TraversalFieldGpuInput` — no CPU readback, no shadow/property D scatter, **no public `tick()` scaffold**.

**Diagnostic modes:** `dispatch_diagnostic_readback`, `dispatch_shadow_column_compatibility`, `dispatch_oracle_verification_*` — explicit only.

**Removed (PATH-8R):** public `tick()` / `tick_with_input()` — CPU-shadow gather is not reachable via a friendly default wrapper.

**Not landed:** pathfinding engine, movement policy, route object, predecessor table, mandatory per-tick CPU D readback, default `SimSession` band scheduling.

**Fable handoff:** use `MinPlusTraversalFieldOp::dispatch_traversal_from_input` or `TraversalFieldBandSession::dispatch_gpu_resident`; do not retread PATH-1–6 proof sequence unless changing algebra or admission.

---

## 15b. GPU-native field graph connection (PATH-8)

PATH-8 connects the seated utility to upstream GPU field buffers without mandatory CPU W gather:

```text
GPU W impedance buffer (flat or interleaved w_col)
  → IndexedScatter prepare + MinPlusTraversalFieldOp
  → GPU-resident D (`MinPlusTraversalGpuOutputHandle` / `resident_d_output()`)
  → downstream GPU field / EML / threshold consumers (deferred wiring)
```

**W input modes:**

| Mode | API | Role |
|---|---|---|
| `TraversalFieldGpuInput::FlatW` / `InterleavedW` | `dispatch_gpu_resident`, `dispatch_diagnostic_readback`, `dispatch_oracle_verification_gpu` | **Production** — GPU W from upstream field pass |
| `TraversalFieldShadowColumnCompatInput` | `dispatch_shadow_column_compatibility`, `dispatch_oracle_verification_shadow_compat` | **Diagnostic/compatibility only** — explicit CPU shadow gather |

**D output:** `MinPlusTraversalGpuOutputHandle` / `TraversalFieldBandSession::resident_d_output()` exposes the resident ping-pong buffer. No CPU readback in `dispatch_gpu_resident`.

**Removed (PATH-8R):** `tick()`, `tick_with_input()`, and generic `TraversalFieldInput` — no default CPU-shadow path.

**Not landed:** automatic `SimSession` / RegionField pass-graph wiring, downstream GPU threshold consumer on D, pathfinding engine, movement policy.

---

## 15c. Tick scaffold removal (PATH-8R)

Public `tick()` implied a runtime subsystem and let tests pass via CPU-shadow compatibility without proving GPU-native production shape.

Production callers must use explicit dispatch:

```text
TraversalFieldGpuInput
  → dispatch_gpu_resident
  → resident_d_output()
```

Diagnostic/compatibility callers must name their mode explicitly (`dispatch_shadow_column_compatibility`, `dispatch_oracle_verification_*`, `dispatch_diagnostic_readback`).

---

## 15d. Legacy PALMA alias removal (PATH-8R-CLEAN)

Public `palma_min_plus_field_band` re-exports (`PalmaMinPlusFieldBandSession`, `PALMA_MIN_PLUS_*`, `TraversalFieldBandTickReport`) were removed.

**Runtime API:** `simthing_driver::min_plus_traversal_field` only — `TraversalFieldBandSession`, `TraversalFieldDispatchReport`, `TRAVERSAL_FIELD_*` constants.

**PALMA** remains algebraic provenance in docs and test fixture names only — not a production subsystem noun.

---

## 15e. Downstream GPU consumer smoke (PATH-9)

PATH-9 proves resident traversal D can feed a downstream GPU consumer without full-D CPU readback:

```text
GPU W impedance buffer
  → dispatch_gpu_resident / MinPlusTraversalFieldOp
  → GPU-resident D (`MinPlusTraversalGpuOutputHandle` / `resident_d_output()`)
  → MinPlusTraversalDProbe (gather D at candidate cell indices + min reduction)
  → compact probe buffer readback (test assertion / diagnostic only)
```

**Production path:** `MinPlusTraversalDProbeOp::probe_resident_d` binds the resident interleaved values buffer directly — no full D field readback.

**Probe output:** `MinPlusTraversalDProbeResult { gathered, min_d }` — one f32 per candidate plus min D across the set (cap `TRAVERSAL_D_PROBE_MAX_CANDIDATES = 64`).

**CPU oracle:** `cpu_probe_d_at_candidates` compares against CPU-computed flat D at the same indices — oracle verification only; not the production hot path.

**Not landed:** pathfinding engine, movement policy, route object, predecessor table, automatic SimSession pass-graph wiring, semantic GPU interpretation of D.

---

## 16. Terran convoy / pirate fleet example (numeric W only in PATH-1)

**Scenario narrative (comments and fixture names):**

1. **Location** SimThing owns a 5×5 (or 8×8) grid of gridcell children.
2. **Destination** gridcell (starport) seeds `D = 0`.
3. **Uniform** `W = 1` except:
   - **Pirate fleet pressure** raises `W` around pirate-adjacent cells;
   - **Blockade overlay** raises `W` in a corridor band;
   - **Fuel shortage** raises `W` in distant cells (optional in PATH-1).
4. Min-plus iterations propagate **D**; convoy at `(x, y)` would read `D[x, y]` — **not implemented in PATH-1**.
5. When blockade **W** clears, **D** on affected cells drops on subsequent iterations — proves field reactivity without routes.

PATH-1 represents pirate/blockade/fuel as **numeric W arrays only** — no convoy movement, no ClauseThing.

---

## 17. Benchmark plan (PALMA-PATH-4 — not run in PATH-0/1)

Compare (honest, no overclaim) — **run completed PALMA-PATH-4**; see [`tests/palma_path_4_benchmark_results.md`](tests/palma_path_4_benchmark_results.md).

| Axis | Values |
|---|---|
| Grid | 64×64, 128×128, maybe 256×256 |
| Movers | 10, 100, 1k, 10k |
| W churn per tick | 0%, 1%, 5%, 20% |
| Relaxation iterations | 1, 2, 4, 8 |

Metrics: wall time per tick for (a) CPU per-mover route query oracle, (b) CPU min-plus field update,
(c) GPU min-plus field update when PATH-2 exists.

**Expected outcome:** CPU per-mover may win at small mover counts and static **W**; field update may win
at high mover counts or high **W** churn. Report both; do not declare universal GPU victory.

---

## 18. Stop conditions

Stop and escalate to design authority if:

1. A **pathfinding engine** or graph manager detached from Location/gridcell SimThings is proposed
2. **Movable SimThings** cease to be ordinary SimThings or gain planner-client APIs
3. **W** is computed by semantic branches instead of property/overlay/flow columns
4. **GPU code** interprets game semantics (pirate/fleet/blockade literals)
5. **All-pairs closure** on large maps every tick is required without bounded consumer
6. **Exact full-map shortest path** every tick is mandated (vs fixed iterations)
7. **simthing-sim** semantic changes are required
8. **ARM/NEON PALMA sections** are treated as constitutional authority
9. Design contradicts `simthing_core_design.md` §7 or `design_0_0_8_1.md` §0 anti-planner doctrine
10. **`cargo test --workspace`** is requested for PATH-0/1 narrow proofs

---

## Closure checklist (PALMA-PATH-0)

- [x] Authority order stated
- [x] Paper link + relevant sections named
- [x] ARM/NEON sections excluded from design authority
- [x] Everything-is-a-SimThing framing
- [x] Location/gridcell ownership
- [x] Movable SimThings remain ordinary field samplers
- [x] Min-plus recurrence stated (cell-entry convention)
- [x] W composition explained
- [x] Stowaway advantage stated honestly
- [x] CPU pathfinding as oracle/fallback
- [x] GPU/JIT/WGSL allowed later for bounded arithmetic only
- [x] No semantic GPU code
- [x] Exact sqrt rule included
- [x] Terran/pirate scenario proposed
- [x] PR ladder proposed
- [x] No runtime implementation in PATH-0

**0.0.8.2 closeout addendum (2026-06-13, PR5).** ClauseThing scenario containers now author one
`palma_feedstock` block lowering to `HydratedScenarioPalmaFeedstock`: `w_source` binds to a scenario
`field_operator` id; `w_output_col` and `d_output_col` are generic column bindings for the existing
BH-2C `GpuInterleavedW` → min-plus `d_col`/`w_col` bridge. This is W/D **feedstock authoring
only** — not pathfinding, movement, routes, predecessors, or a PALMA runtime service. Driver
install and GPU exercise remain PR8; FIELD_POLICY unification remains PR6.

**Report:** [`tests/palma_path_0_design_results.md`](tests/palma_path_0_design_results.md)
