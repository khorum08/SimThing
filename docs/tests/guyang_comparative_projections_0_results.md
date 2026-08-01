# GUYANG-COMPARATIVE-PROJECTIONS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8)
- Status: **PROBATION / proof-present / DA-review-pending** (Remand 2 discharge)
- HD-RECEIPT: `b8f9a2e4ef61`
- DA ruling: Board `5150877754`
- Remand 2: Board `5150890712`
- Adapter: `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan`

Exact tested/pushed head is bound in the PR/board relay (this file cannot self-hash).

## DA-amended law (implemented)

| Law | Implementation |
|---|---|
| Margin | exact `top1 − top2` (non-negative magnitude) |
| Border | **winner-identity change** `argmax(target) ≠ argmax(neighbor)` with authored-order tie-break |
| Stall | authored second field-sweep: map `abs((c_i+c_j)/2·(u_j−u_i))`, fold `(+,0)` → gross; stall = gross − \|net\| |
| Contest | admitted stall under both-strong @ small-margin |
| Scope | **driver-only consumer**; no kernel/GPU public doors; no allowlist widening |

## Contract discharge

| Requirement | Result |
|---|---|
| Winner-identity border | PASS — fires at multi-emitter fronts on Grid N4 and LinkGraph |
| Argmax-tie falsifier | PASS — exact tie uses authored order; reverse order flips identity and border partner |
| Gu-Yang stall truthful | PASS — net/gross/stall columns born on generic field-sweep EML; contest consumes stall |
| Default-derived Anchored mint | PASS — `derive_comparative_projections_at_admission` mints fixed comparative+stall properties when ≥2 Anchored emitters |
| Fixed column count vs N | PASS — 2 and 3 emitters yield same `COMPARATIVE_DERIVED_COLUMN_COUNT` |
| Unmodified TP witness | PASS — loads `terran_pirate_skeleton` via deserialize/validate/compile theater; default-born projections; front + chokepoint thresholds; controls suppress without contested-border or PALMA-low-D |
| CPU/GPU parity | PASS — full chain bit-exact on Vulkan |
| Scope envelope | PASS — no `allow/*.txt`, kernel, or GPU production edits in this discharge |

## Focused proof

```text
guyang_comparative_projections_0: 6 passed; 0 failed
GUYANG-COMPARATIVE-PROJECTIONS adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
```

## Posture

Return **PROBATION**. No pointer move, no 5.9, no tiled-gather, no oracle retirement. Orchestrator owns settled-body exact-head clearance (coding does not invoke `/clearance`).
