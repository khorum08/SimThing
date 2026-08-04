# StemThing §3.1 Slot-Bearing-Artifact Census — Methodology

> **Status: PREPARATORY WORK PRODUCT (DA, Fable; Owner-directed 2026-08-03 "do 1 now").** This is
> the **method**, not the census. The census itself runs **after 6.3's graduation stamp** (Owner
> ruling, single pass, recorded in `stead_stemthing_unification.md` §10) — running it earlier
> would rot against the surfaces 6.2b/6.3 are still adding. This document exists so the
> 6.3-stamp trigger executes in hours, not a session. It binds nothing; the census's output feeds
> the §3.1 Tier-2 slot-identity amendment, and Sol's review standard governs: the enumeration
> must be **tree-derived and exhaustive**, not illustrative.

---

## 1. The question the census answers

Shape (a) of the §3.1 ruling (stable **logical** `SlotIndex`; physical row binding at boundary
upload; epoch compaction rebinding with a recorded remap) is admissible **iff no authoritative
identity depends on physical-row permanence**. The census proves or refutes that, artifact by
artifact. One irreparable dependency ⇒ shape (b) wins and the elastic-memory prize shrinks. The
census is therefore a falsifiable proof obligation, not inventory hygiene.

**Definition — slot-bearing artifact:** any type, buffer, record, table, or persisted structure
that stores, transmits, or derives meaning from a slot index / row offset — directly (a `SlotIndex`
or raw `u32` row), or indirectly (an ordering, range, or address computed from one).

## 2. The leverage point: start from the mint doors, not from grep

4.1 (`ROW-SLOT-OBJECT-SEMANTICS-0`) made typed object-issued residency "the sole production
structural row door," and 4.2 (`PLAN-STRUCT-TYPING-0`) collapsed raw `u32` to the single WGSL
encode/decode boundary. **The census walks outward from those doors** — enumerate the
constructors/mint sites of the slot-bearing types, then walk every consumer transitively. Grep is
the *cross-check* for evaders (raw arithmetic, serialized forms), never the primary method. This
is the admission substrate paying for itself: the Phase 4 typing work is what makes an exhaustive
census tractable.

Sweep order:

1. **Type-walk:** from `SlotIndex` / slot-typed residency types → all constructors → all
   consumers (compiler-assisted: temporarily seal a constructor, harvest the error list — the
   compile-fail census pattern; do this in a throwaway branch, never landed).
2. **Boundary-walk:** the WGSL encode/decode boundary module — every field that crosses as a raw
   `u32` row, on both CPU and shader sides.
3. **Persistence-walk:** everything serialized (replay records, checkpoints, scenario dumps,
   TSVs) that could embed a row.
4. **Grep cross-check:** `slot`, `row`, `linear_idx`, `* n_dims`, `SlotRange`, `INPUT_LIST`
   payloads, `base + k *` arithmetic — flag anything the type-walk missed; each hit is either
   added to the census or recorded as provably non-slot (with reason).

## 3. Artifact classes to enumerate

Known-member examples are seeds, **not** the census. Every class gets a full sweep.

| # | Class | Known members (seeds) | Primary risk |
|---|---|---|---|
| K1 | Kernel dispatch/registration PODs | `AccumulatorOpGpu` source/target slots; packed session upload; threshold registrations; `EmlEvalCtx.eval_slot` | rebind = re-encode at boundary (already re-uploaded on structural change — verify unconditionally true) |
| K2 | Gather/adjacency tables | `INPUT_LIST` entries; `LinkGraph` neighbor lists; `SlotRange` plans; sparse child input lists | **ORDER** — see H1 |
| K3 | Field-sweep surfaces | `FieldSweepRegistration` adjacency; conductance certificates (per-node keying); grid-cell ↔ slot mapping for spatial ranges | order + spatial-contiguity assumptions |
| K4 | Emission/readback records | `EmissionRecordGpu`, `ThresholdEmissionGpu {slot, col}`; sealed `ThresholdEvent` reconstruction | slot crosses into *sealed decision ingress* — does any sealed token persist a row? |
| C1 | CPU session structures | `SlotAllocator` free-list; shadow (row-major); `slot_of` / id→slot map; `ArenaRegistry` participants `(ArenaIdx, SlotId)` | the id→slot map is the natural home of the logical/physical split — it becomes the *binding table* |
| C2 | Lifecycle/records | `FissionLineageRecord`; boundary maintainer outcomes; 6.x additions: stamped products, `OwnerChannelRfBucket` keys, `CommandDeficit` routing, 6.2 queue buckets | do any key by slot rather than id? |
| P1 | Replay/persistence | `BoundaryDeltaEntry`; `shadow_values` checkpoints; 6.1 recorded integration schedule; grant/remap records (future) | **H2** — old replays must survive a remap |
| J1 | JIT artifacts | 5.7 kernel cache (keyed by class + program identity); compiled straight-line WGSL | do compiled kernels bake slot constants, or receive them as uniforms? cache-key contamination |
| S1 | Spec/driver compile products | structural link compile resolution; arena pressure bindings `(target_id, row, col)` — is `row` a slot or a grid coordinate?; region-field buffers (`y*width+x` cell-indexed — separate from slot space, verify the mapping table) | conflation of cell-index space with slot space |

## 4. Census output schema

One TSV row per artifact (goes in the amendment's evidence, `scripts/ci/` conventions):

```
artifact | crate:file | representation (typed SlotIndex / raw u32 / derived offset / serialized)
| lifetime (tick-transient / session-resident / persisted)
| order-bearing? (does any fold/iteration order derive from it)
| replay-bearing? (does any persisted record embed it)
| rebind path under shape (a)
| verdict
```

**Verdict classes (closed set):**

- `REBIND-FREE` — transient; never survives a boundary; no action.
- `REBIND-AT-BOUNDARY` — re-derived/re-uploaded at boundary already; shape (a) is free here,
  *cite the code path that re-derives it*.
- `ORDER-PIN` — an ordering derives from physical rows; must be re-pinned to logical identity
  (authored order) before shape (a); enumerate the re-pin.
- `REPLAY-RECORD` — persisted; needs the remap record to keep old artifacts valid; specify the
  record extension (the canonical 6.1 history surface, never a second mechanism).
- `BLOCKER` — authoritative identity depends on physical permanence with no lawful rebind path.
  **One of these ⇒ shape (b).**

Exhaustiveness proof: the census row count must reconcile against the type-walk consumer count
and the grep cross-check residue (every grep hit either in the census or justified out). That
reconciliation line is what makes it "tree-derived exhaustive" rather than curated.

## 5. The two pre-flagged hard cases (expanded procedures)

**H1 — canonical fold order.** `CanonicalOrderProof` seals neighbor-list order; the link
compiler's basis is "sorted + deduped" lists. **Sorted by what key?** If by slot index →
fold order is *physical* → epoch rebinding reorders folds → bit-exactness breaks — the one thing
WHERE-not-ORDER forbids. Procedure: read the sort key in the link-compiler basis and every
`CanonicalOrderProof` mint; verdict is `ORDER-PIN` unless order is already derived from logical
id or authored order (5.8b's `authored_order` precedent is the target shape). Also check grid
generators: `GridOffsets` order is authored table order (safe by construction — verify no
slot-sort creeps in at materialization).

**H2 — replay keying.** Procedure: enumerate every field of `BoundaryDeltaEntry` and the
`shadow_values` checkpoint format; classify each as id-keyed (safe) or row-keyed
(`REPLAY-RECORD`). Then the 6.1 integration schedule: confirm it keys by (id, generation), not
row. If any row-keyed record exists, specify its remap-record coverage such that **a pre-remap
replay replays bit-exactly against post-remap state** — that sentence is the acceptance test.

## 6. Post-6.3 delta list (why the census waits)

Surfaces Phase 6 is still adding, to be swept as first-class members once landed: 6.2's
coalescing queue buckets and ring egress; 6.2b's resolution-site dispatch surfaces (slot-space
identity is its *central* claim — its "riskiest assumption" artifacts are census rows by
definition); 6.3's `OwnerChannelRfSteadSurface` retained rows.

## 7. What the census feeds

The §3.1 Tier-2 amendment to `simthing_core_design.md`: restate registry stability as *stable
logical identity; physical binding per epoch; rebind only at a recorded boundary remap*, with the
census TSV as the enumeration Sol required and the `ORDER-PIN`/`REPLAY-RECORD` rows as the
amendment's work list. Zero `BLOCKER` rows is the admissibility condition for shape (a). The
amendment's merge lifts the HARD HOLD; StemThing-A rows mint immediately behind it.
