# GUYANG-COMPARATIVE-PROJECTIONS-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.8)
- Status: **PROBATION / proof-present / DA-review-pending** — no pointer move, graduation, tiled-gather work, or oracle retirement claimed.
- HD-RECEIPT: `b8f9a2e4ef61`
- ORIENT-RECEIPT: `e563c4399d73` (rule stamp `5319c193d38da6ce`)
- Dispatch: Board comment `5148926910`
- Expected route: `DA-RESERVE(gate-wiring)`
- Adapter (GPU parity): `NVIDIA GeForce RTX 4080 Laptop GPU` / `Vulkan`

The exact tested head is bound in the PR/board relay because this file cannot self-hash.

## Contract discharge

| Requirement | Result |
|---|---|
| Sealed scenario-neutral authority | PASS — `admit_comparative_projections` over co-located field-sweep outputs; no scenario grammar/switch |
| Default-derived ≥2 emitters | PASS — 1 class → `InsufficientEmitters`; ≥2 → `Born` with fixed `COMPARATIVE_DERIVED_COLUMN_COUNT=5`; authored opt-out reason visible |
| Bounded columns independent of N | PASS — census for 2/3/5/8/12 emitters always yields the same derived column count |
| Dominance + margin | PASS — argmax with exact `top1-top2`; authored-order tie-break; reversal falsifier bites |
| Contest / border / chokepoint | PASS — both-strong@small-margin contest; near-zero / adjacency margin border band; chokepoint = contested-border ∧ PALMA-low-D with paired suppressions |
| Field-EML only (no new field kernel) | PASS — ordinary `FieldSweepRegistration` chain; kernel-private top1 transient; no hand-written semantic WGSL |
| CPU oracle ↔ field-EML parity | PASS — full derived columns bit-exact vs independent oracle |
| CPU ↔ GPU parity | PASS — `dispatch_chain` bit-exact on RTX 4080/Vulkan |
| TP zero wiring + event witness | PASS — `terran_pirate_galaxy.clause` has zero projection wiring tokens; front-formed + exactly one chokepoint-emerged via ordinary threshold path |
| STEAD co-evolution | PASS — §10 comparative-projection binding note landed with the rung |
| Fences held | PASS — no border service, no TP production branch, no tiling/performance work, no Phase 6+, seven oracles untouched |

## Focused proof

```text
guyang_comparative_projections_0: 7 passed; 0 failed
GUYANG-COMPARATIVE-PROJECTIONS adapter=NVIDIA GeForce RTX 4080 Laptop GPU backend=Vulkan
```

## Review posture

Return **PROBATION**. Coding does not move the active pointer, open 5.9, retire oracles, or touch Gu-Yang tiled-gather debt (`FIELD-SWEEP-TILED-GATHER-0`).
