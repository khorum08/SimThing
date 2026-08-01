# GUYANG-COMPARATIVE-PROJECTIONS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8)
- Status: **PROBATION / graduation-ready / DA-review-pending**
- HD-RECEIPT: `b8f9a2e4ef61`
- DA semantic ruling: Board `5150877754`
- DA seam amendment + TP void + 5.8b deferral: Board `5151136145`
- Closeout remand: Board `5151157568`
- Orchestrator TP-clause correction (prompted by Owner concern): `5150987561` — **TP witness RATIFIED VOID**
- Adapter: `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan`

Exact tested head is bound on the PR as bare `tested_code_sha:` (this file cannot self-hash).

## Amended 5.8 contract (DA `5151136145`)

| Clause | Binding |
|---|---|
| Proof shape | **Scenario-neutral only** — no corpus load of any kind |
| TP witness | **VOID** — not a 5.8 obligation |
| Birth | **Explicit and fail-closed** — `admit_comparative_projections(...)` is the 5.8 production posture |
| Default-derived birth | **Out of scope for 5.8** — owned by rung **5.8b `COMPARATIVE-DEFAULT-BIRTH-0`** |
| Topology / role invent | Forbidden under 5.8; seams deferred to 5.8b without STOP-as-blocker on 5.8 graduation |

## Load-bearing evidence

| Surface | Result |
|---|---|
| Margin = exact `top1 − top2` | PASS |
| Border = winner-identity change | PASS |
| Stall = gross − \|net\| (second authored EML registration) | PASS |
| Contest = stall under both-strong @ small-margin | PASS |
| Comparative census = **3** (dominance, margin, contest) | PASS — border/chokepoint are band readouts (2), not comparative columns |
| Authored-order tie-break | PASS — vector reverse invariant; planted authored_order flip red |
| Grid + LinkGraph CPU/oracle **and GPU** bit parity | PASS (RTX 4080/Vulkan) |
| Threshold plan band compatibility | PASS — ordinary `EmitOnThreshold` for front-formed / front-hardened / chokepoint; controls suppress |
| Explicit admit dispositions | PASS — Insufficient at 1 emitter; Born at ≥2 with fixed column count; `Result` fail-closed |
| Install does not invent topology/string birth | PASS — keeps `comparative_projection = None` (honest; default birth is 5.8b) |
| Scope | PASS — driver-only consumer; no allowlist/kernel/GPU public doors |

## Production posture (5.8)

```text
admit_comparative_projections(
  registry,
  adjacency,          // already-admitted FieldAdjacency from the caller
  neighbor_slots,     // sealed from public grid offsets or link rows
  emitters,           // explicit ComparativeEmitterClass {authored_order, class_id, value_col}
  palma_d_col, guyang_value_col, guyang_conductance_col,
  bands,
  authored_opt_out_reason,
) -> Result<ComparativeProjectionAdmission, ComparativeProjectionError>
```

No namespace scan. No topology invent. Compile errors propagate. One-emitter and authored opt-out remain explicit dispositions.

## Focused proof

```text
guyang_comparative_projections_0: 5 passed; 0 failed
GUYANG-COMPARATIVE-PROJECTIONS grid adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
GUYANG-COMPARATIVE-PROJECTIONS link adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
```

1. `install_does_not_invent_topology_or_string_default_birth`
2. `explicit_admit_dispositions_and_fixed_comparative_column_count`
3. `authored_order_tie_break_invariant_under_registration_vector_reversal`
4. `grid_and_link_graph_cpu_oracle_and_gpu_parity`
5. `front_formed_hardened_and_chokepoint_threshold_plan_compatible`

## Posture

Return **PROBATION / graduation-ready**. No pointer move, no 5.8b implementation, no 5.9, no tiled-gather, no TP resurrection. Coding does not invoke `/clearance` — orchestration settles exact-head clearance and relays to DA.
