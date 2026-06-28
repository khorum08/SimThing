# 0.0.8.4 — SimThing Admission Substrate: Doctrine as Type

> **Status: OPEN (production track, opened 2026-06-28, owner-directed; promoted ahead of the
> ClauseScript Terran-Pirate track, which resequences to 0.0.8.5).** Sits *beneath* the permanent
> paradigm [`simthing_core_design.md`](simthing_core_design.md) — and is the **first consumer of its
> §1.2 (the admission substrate)** — and *beneath* the constitution [`design_0_0_8_3.md`](design_0_0_8_3.md).
>
> This is a **refactor track that deletes enforcement surface.** Its deliverable is not a feature; it is
> the migration of named invariants from prose / guard-scan **up** to type boundaries, so that whole
> classes of drift become **uncompilable rather than merely forbidden** — and the now-redundant guard or
> prose detector is **retired.** Success is measured in *guards retired and prose shortened*, not artifacts
> added.

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2** — the admission substrate + the admission ladder (the doctrine this track executes); plus the antecedents it cites: **§2** (kind never branched at runtime), **§3** (role→column resolution; no `data[N]`), **§4** (semantic-free sim; compile-away), **§5.1** (channel identity).
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 — the constitution (anti-flattening §0.6, harness §0.5).
3. **This file** — the 0.0.8.4 canonical design file.
4. [`handoff_template.md`](handoff_template.md) — the binding handoff skeleton (§H anti-kabuki rules + the context spine, which now carries §1.2). Every rung handoff uses it.

**Established decisions — do NOT re-derive:**

- This is a **pure refactor**: each rung changes *types*, never *behavior*. CPU-oracle parity stays bit-exact; existing tests stay green. A rung that changes a resolved value is wrong.
- The proof that an illegal state is now unrepresentable is **one** `compile_fail` / `trybuild` negative test — the minimal load-bearing proof (handoff template §6 / D4). **No batteries.**
- Every rung **retires** the lower-rung enforcement it replaces (a guard test, a source scan, or a prose detector). Net enforcement surface goes **down**. A rung that adds a type *and* keeps the old guard has not finished.
- `SimThingKind` legitimately exists at the **spec / authoring / topology** layer (it is a convenience label there). The target is never "delete `SimThingKind`"; it is "make it **unreadable in the simulation tick path**" (core §2).
- Raw integer access to a column buffer is not banned outright — **serialization byte-lanes are legitimate** (e.g. the lossless metadata seed in `scenario.rs`). The target is: the *default* accessor is role-keyed; raw access requires an **explicit, named, greppable** escape hatch (`raw_lane()`), never an anonymous `data[3]`.

---

## 1. Objective & per-rung definition of done

**Objective.** Promote the highest-value prose/guard-enforced SimThing invariants to type boundaries,
closing the gaps where the type system currently *permits what the constitution forbids* (audit findings,
§3 below). Each promotion deletes a guard.

**Per-rung definition of done (the rung contract):**

1. **The illegal state is now uncompilable** — proven by exactly one `compile_fail` doc-test or `trybuild`
   UI test (the canonical violation no longer builds).
2. **The redundant enforcement is retired** — the guard test / source scan / prose detector it replaces is
   deleted or narrowed to its true residue, and the Scope Ledger names it.
3. **Pure refactor, parity preserved** — the rung's targeted tests + the relevant CPU-oracle parity tests
   are bit-exact green; no resolved value changes. `cargo test --workspace` is never run.
4. **One results doc** with a Scope Ledger: *invariant promoted · rung climbed from→to · guard retired ·
   parity evidence*. Nothing else.

---

## 2. The ladder

Ordered by independence and impact; each rung is small and self-contained. Rungs may split if an audit
shows the migration surface is larger than one focused PR.

| Rung | ID | Promote | Type move (illustrative — implementer owns the form) | Retires |
|---|---|---|---|---|
| 0 | `AS-TRACK-OPEN-0` | — | This doc + evidence-index row. Docs only. | — |
| 1 | `AS-COLUMN-ACCESS-0` | §3 "no hardcoded `data[N]`" | Encapsulate `PropertyValue.data`; access only via a `RoleOffset`/`ColumnIndex` resolved from `PropertyLayout` (`offset_of`/`col_for_role` already exist in `simthing-core`). Raw access becomes an explicit `raw_lane()` for serialization byte-lanes only (the `scenario.rs` metadata seed). | The "no hardcoded `data[N]`" prose detector; any `data[` source scan. `compile_fail`: `value.data[0]` does not build. |
| 2 | `AS-CHANNEL-NEWTYPES-0` | §5.1 channel identity | `OwnerRef`, `ResourceKey`, `ScopeId`, `ParentLocationId` as distinct newtypes (over the current `String`/id). Transposition and bare-string passing become uncompilable. | Arg-order / mis-binding validation + any "owner vs resource" runtime check. `compile_fail`: passing a `ResourceKey` where `OwnerRef` is expected. |
| 3 | `AS-KIND-OUT-OF-TICK-0` | §2 "behavior never branches on kind at runtime" | Introduce the tick/runtime view type that carries **no `kind`** (resolved columns/slots only); drive the production kind-reads (audit `simthing-sim`, e.g. `is_capability_container(&child.kind, …)` at `boundary.rs:1293`) from a resolved flag/column instead of a `kind` match. | Core §9 "no `match kind`" detector → promoted to type. `compile_fail`: a tick-path fn cannot access `.kind`. **May split** (audit production vs test kind-reads first). |
| 4 | `AS-SIM-SEMANTIC-FREE-0` | §4 "`simthing-sim` never learns the words" | Seal the `simthing-sim` public surface so it cannot **name** a game concept — no semantic `SimThingKind` variant (`Faction`/`Cohort`) or semantic `String` category/faction crosses the crate boundary; the sim sees columns, indices, and opaque registration handles. Composes on AS-3. | The scattered semantic-free source scans → narrowed to true residue (e.g. WGSL text, which Rust can't see — stays a scan). `compile_fail`: a faction/category type at the `simthing-sim` boundary. |
| 5 | `AS-INDEX-NEWTYPES-0` *(horizon)* | §3/§4 stable indices | `SlotIndex`, `ColumnIndex`, and audit `OrderBand` (already `OrderBand(u32)`) as distinct newtypes so a slot index can't be used as a column index; owner-vs-spatial-parent distinction so "reparent to an owner" (core §2 law 2) is uncompilable. | Mixing-index bugs; the "owner is never a spatial parent" prose detector (partial). |
| F | `AS-CLOSEOUT-0` | — | Scope Ledger across rungs: invariants promoted, **guards/prose retired (the success metric)**, parity intact. DA review. | — |

---

## 3. Audit findings (why this is closing real gaps, not gilding)

The grounding pass (2026-06-28) found the type system currently **permits what the constitution forbids**:

- `simthing-sim` carries `SimThingKind` with semantic variants and at least one production kind-read
  (`is_capability_container(&child.kind, …)`, `boundary.rs:1293`) — against §2/§4. (AS-3/AS-4.)
- Raw `data[0]` / `data[1..]` indexing in `scenario.rs` despite §3's "no hardcoded `data[N]`, ever"; the
  role-keyed accessors exist but are optional. (AS-1.)
- Channel identity is `owner_ref/resource_key/scope_id: Option<String>` — freely transposable, against
  §5.1's structured key. (AS-2.)

These are not stylistic; each is a place where an agent can satisfy the compiler while violating the
ontology. Promoting them is the §1.2 directive applied to the engine's own current state.

## 4. Anti-kabuki specifics for this track

This track is uniquely resistant to ceremony because **its output is deletion.** Reinforced bindings:

- **Success is net-negative enforcement surface.** A rung's Scope Ledger must show a guard/prose detector
  *removed*. Adding a type while keeping the old scan is an incomplete rung, not a careful one.
- **One `compile_fail` per promoted invariant.** It proves the boundary holds; it is the *whole* negative
  suite. Do not enumerate variants of the now-impossible state.
- **No new runtime behavior, no new feature, no new doc beyond the one results ledger.** If a rung is
  tempted to add observability, a registry, or a migration report, that is the noun-for-verb reflex (D8) —
  the migration *is* the type change.

## 5. Honest residue (what stays prose + admission — by nature, not neglect)

Types cannot reach everything; this track deliberately stops where they stop, concentrating prose/DA
judgment on the true residue (core §1.2):

- **no-CPU-planner** and **no-flattening** (§0.6) — properties of *behavior over time*, not of a value's
  type; stay constitutional + reviewed.
- **The GPU/WGSL trust domain** — Rust's checker cannot see into shader text; CPU-oracle parity and the
  semantic-free *shader* scan remain the only admission the shader has (§4). AS-4 narrows the scan to
  exactly this residue and no wider.
- **Live ontological conformance** — "is this still one accumulate→reduce→threshold loop?" remains DA work.

## 6. References

- The doctrine: [`simthing_core_design.md`](simthing_core_design.md) §1.2 (+ §2, §3, §4, §5.1, §9 "Promotion over repetition").
- Constitution: [`design_0_0_8_3.md`](design_0_0_8_3.md) §0.
- Handoff discipline: [`handoff_template.md`](handoff_template.md) (§H + the context spine).
- Deferred successor: the ClauseScript Terran-Pirate galaxy track (resequenced to **0.0.8.5**) —
  [`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md);
  it inherits every invariant this track promotes for free.
