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

| Rung | ID | Promote | Type move (illustrative — implementer owns the form) | Retires | Recipient | State |
|---|---|---|---|---|---|---|
| 0 | `AS-TRACK-OPEN-0` | — | This doc + evidence-index row. Docs only. | — | Haiku/Sonnet (docs) | DONE |
| 1 | `AS-COLUMN-ACCESS-0` | §3 "no hardcoded `data[N]`" | Encapsulate `PropertyValue.data`; access only via a `RoleOffset`/`ColumnIndex` resolved from `PropertyLayout` (`offset_of`/`col_for_role` already exist in `simthing-core`). Raw access becomes an explicit `raw_lane()` for serialization byte-lanes only (the `scenario.rs` metadata seed). | The "no hardcoded `data[N]`" prose detector; any `data[` source scan. `compile_fail`: `value.data[0]` does not build. | Cursor/Grok | **DONE — DA-APPROVED** |
| 2 | `AS-CHANNEL-NEWTYPES-0` | §5.1 channel identity | `OwnerRef`, `ResourceKey`, `ScopeId`, `ParentLocationId` as distinct newtypes (over the current `String`/id). Transposition and bare-string passing become uncompilable. | Arg-order / mis-binding validation + any "owner vs resource" runtime check. `compile_fail`: passing a `ResourceKey` where `OwnerRef` is expected. **0R:** production/report surfaces adopt newtypes; `as_channel_newtypes_production_adoption`. | Cursor/Grok | **DONE — DA-APPROVED** |
| 3 | `AS-KIND-OUT-OF-TICK-0` | §2 "behavior never branches on kind at runtime" | Introduce the tick/runtime view type that carries **no `kind`** (resolved columns/slots only); drive the production kind-reads (audit `simthing-sim`, e.g. `is_capability_container(&child.kind, …)` at `boundary.rs:1293`) from a resolved column instead of a `kind` match (static → resolve-away; dynamic predicate → **EML gadget**, §2.1). | Core §9 "no `match kind`" detector → promoted to type. `compile_fail`: a tick-path fn cannot access `.kind`. **May split** (audit production vs test kind-reads first). | Cursor/Grok | **DONE — DA-APPROVED** |
| 4 | `AS-SIM-SEMANTIC-FREE-0` | §4 "`simthing-sim` never learns the words" | Seal the `simthing-sim` public surface so it cannot **name** a game concept — no semantic `SimThingKind` variant (`Faction`/`Cohort`) or semantic `String` category/faction crosses the crate boundary; the sim sees columns, indices, and opaque registration handles. Composes on AS-3. | The scattered semantic-free source scans → narrowed to true residue (e.g. WGSL text, which Rust can't see — stays a scan). `compile_fail`: a faction/category type at the `simthing-sim` boundary. | Cursor/Grok | **DONE — DA-APPROVED** (public surface; `pub(crate)` internal seams = named follow-on) |
| 5 | `AS-INDEX-NEWTYPES-0` | §3/§4 stable indices | `SlotIndex`, `ColumnIndex`, and audit `OrderBand` (already `OrderBand(u32)`) as distinct newtypes so a slot index can't be used as a column index; owner-vs-spatial-parent distinction so "reparent to an owner" (core §2 law 2) is uncompilable. Entity ids are copyable integer-wrapped newtypes (`SystemId(u32)`, `OwnerId`, `LocationId`) — this subsumes the only load-bearing part of the "flat-arena / no reference cycles" idea (Gemini review); owned `Vec` children already preclude cycles, so no separate cycle-prevention rung is warranted. | Mixing-index bugs; the "owner is never a spatial parent" prose detector (partial). | Cursor/Grok | **DONE — DA-APPROVED** |
| 6 | `AS-STRUCTURAL-COORD-0` | STEAD: render coords cannot leak into structural logic (core §0/§7; `stead_spatial_contract.md`) | `StructuralCoord { col, row }` integer newtype that **cannot be constructed from render `f32`** except via an explicit, named conversion; the lowerer / Movement-Front / RF-binding structural paths accept only `StructuralCoord`, never a bare float pair. | Driver `StructuralGridCoordinate` public-field literal seam → private-field `StructuralCoord` + `compile_fail`. `compile_fail`: building a structural coord from render floats. **Now-rung (independent of AS-5).** | Cursor/Grok | **DONE — DA-APPROVED** |
| 7 | `AS-TICK-FABRIC-BOUNDARY-0` | the **hot-path slice** of no-CPU-planner (core §8) | The per-tick fn accepts only `&mut SimulationFabric` — resolved numeric columns/slots, **no** semantic / strategic / `kind` / overlay-authoring state. Planning inputs are structurally inaccessible *inside* the tick. Capstones AS-3 + AS-4 (they remove `kind` and semantic names; this removes *all* non-resolved state from the tick view). | Session-loop direct GPU dispatch → `run_simulation_fabric_hot_cycle`. `compile_fail`: fabric field access to proto/scenario/tree; mapping hot path cannot reach boundary effects. **0A:** ordinary tick. **0B:** RF + mapping hot dispatch. **0C:** pre-tick feeder enqueue sealed in hot cycle. **Depends on AS-3 + AS-4.** | Cursor/Grok | **DONE — DA-APPROVED** |
| 8 | `AS-PACKED-UPLOAD-BOUNDARY-0` | the **structural upload slice** of semantic-free-GPU (§4) | The **structural** GPU upload utility consumes only a `PackedUpload` (`[f32; N]` columns + primitive indices); no rich/semantic struct crosses the structural upload seam. **Delivered scope = the structural upload path.** Session-upload paths sealed by **`AS-8B`**. | Old free-row upload signature **removed**; `compile_fail`: a semantic struct / typed index at the structural upload boundary. **Depends on AS-1.** | Cursor/Grok | **DONE — DA-APPROVED** (structural seam) |
| 8B | `AS-8B` | the **session upload slice** of semantic-free-GPU (§4) | `AccumulatorOpSession` upload consumes only `PackedAccumulatorUpload` / `PackedThresholdUpload` / `PackedIntentUpload`; semantic ops and free GPU slices pack before the upload seam. | Old free-slice session upload signatures **removed**; 6 `compile_fail` proofs on session boundary. §5 GPU residue → *shader-text only* (**DA-APPROVED**). **Depends on AS-8.** | Cursor/Grok | **DONE — DA-APPROVED** |
| F | `AS-CLOSEOUT-0` | — | Scope Ledger across rungs: invariants promoted, **guards/prose retired (the success metric)**, parity intact. DA review. **Consolidation completed at DA review** (10 sub-rung slice docs expunged; one ledger per split rung). | — | Haiku/Sonnet (docs) | **DONE — DA-APPROVED** — TRACK CLOSED |

> **DA graduation log (executive DA, 2026-06-29).** Rungs **6 `AS-STRUCTURAL-COORD-0`** and
> **7 `AS-TICK-FABRIC-BOUNDARY-0`** → **DONE / DA-APPROVED** after read-only code review. Both pass the full
> rung contract with **no remediation required**: real type boundary (private-field `StructuralCoord` /
> `SimulationFabric`, each with `compile_fail` proofs that the illegal state no longer builds), **adopted in
> production** (mapping-plan / N4-atlas paths consume `StructuralCoord`; the session hot loop runs through
> `SimulationFabric`), the old seam **deleted** (driver `StructuralGridCoordinate` removed; direct
> session-loop dispatch replaced) — net-negative enforcement surface — and parity preserved. This is the
> clean pattern AS-2 lacked (defined-but-not-adopted). **0R remediation (AS-CHANNEL-NEWTYPES-0R)** adopts
> newtypes across production/report RF surfaces; DA re-review pending. Rung **5 `AS-INDEX-NEWTYPES-0`** →
> **PROBATION** after PR #969 merge (`docs/tests/as_index_newtypes_0_results.md`): `SlotIndex` on `SlotAllocator`
> public boundary + production adoption; **0B** (`AS-INDEX-NEWTYPES-0B`) typed AccumulatorOp/builder slot/column
> axes + GPU encoding parity — CPU-side typed-index residue closed; upload-boundary raw fields remain by design;
> DA re-review pending before DONE — DA-APPROVED. Rungs **1–4**
> graduate under the separate AS-2 remediation pass (AS-1 cleared; AS-2 held for adoption; AS-3/AS-4 on
> doc-consolidation + net-negative confirmation).
>
> **DA graduation log (executive DA, 2026-06-29, re-review after PR #966 remediation).** Rungs **1–4** →
> **DONE / DA-APPROVED**. **AS-1**: encapsulation + role-keyed access verified. **AS-2**: remediation
> satisfied — production channel sites adopt `OwnerRef`/`ResourceKey`/`ScopeId`/`ParentLocationId(u32)`;
> the raw-string scan returns only its own pattern-literals (production fields gone); transposition
> compile_fails present — the defined-but-not-adopted gap is closed. **AS-3**: kind-free tick view verified;
> core §9 detector 1 now annotated as type-enforced for the tick path (this PR). **AS-4**: public surface
> sealed (its stated scope); shader-text is the legitimate remaining residue (§5); the `pub(crate)`
> internal seams are a **named follow-on rung**, not an AS-4 gap. Net: rungs **1, 2, 3, 4, 6, 7 are
> DA-APPROVED**; **5** DA-APPROVED (0A/0B); **8** pending; **F** closeout opens once 8 lands
> (and should consolidate the AS-3 0A–0E / AS-4 0A–0C / AS-5 0A–0B sub-rung docs into one ledger each).
>
> **DA graduation log (executive DA, 2026-06-29, AS-5 re-review after PR #969/#970).** Rung
> **5 `AS-INDEX-NEWTYPES-0` (0A/0B)** → **DONE / DA-APPROVED**. Full handoff contract met, no remediation:
> `SlotIndex(u32)` + `ColumnIndex(usize)` private-field newtypes (old `type ColumnIndex = usize` alias
> **deleted** — net-negative); AccumulatorOp/builder slot/column axes **adopted** (the only bare-integer hit
> is the rejection `compile_fail`'s own pattern); GPU keeps raw `u32` only at the packing boundary via
> explicit `.raw()`/`.raw_u32()` (encoding parity proven). Both lighter handoff items discharged: **OrderBand**
> audited as *no-change* (already a distinct enum variant), and **owner-vs-spatial-parent** verified **and**
> given a `compile_fail` (`owner_ref_rejects_spatial_parent`) — stronger than required. Net: rungs **1–7 are
> DA-APPROVED**; **8 `AS-PACKED-UPLOAD-BOUNDARY-0`** → **PROBATION** after landing
> (`docs/tests/as_packed_upload_boundary_0_results.md`): structural upload sealed behind `PackedUpload`;
> accumulator session upload residue documented; **F** closeout after DA approves AS-8.
>
> **DA graduation log (executive DA, 2026-06-29, AS-8 re-review after PR #973).** Rung
> **8 `AS-PACKED-UPLOAD-BOUNDARY-0`** → **DONE / DA-APPROVED for its delivered scope (the structural upload
> seam)**. Verified: `PackedUpload` private fields + `new()`/`TryFrom` bridges + 7 `compile_fail`; the
> structural `upload_*` fn takes **only** `&PackedUpload` (old free-row signature **removed** — net-negative);
> **adopted** at both production callers (`structural_validation.rs`, mapeditor `scenario_projection.rs` pack
> before upload); byte parity proven. **DA correction (the one over-claim):** my earlier AS-8 row/§5 said the
> GPU residue shrinks to *shader-text only* — that is **premature**. The session-upload paths
> (`upload_ops`/`upload_threshold_ops`/`upload_intent_ops`/EML `upload_*_ops`) still cross semantic — Codex
> recorded this honestly (no AS-2-style over-claim). Sealing them is **`AS-8B` (named follow-on)**; only after
> it lands is the GPU residue shader-text-only. The handoff-spine "one authoritative path" line therefore
> stays a **directive**, not yet a type-fact, for uploads. Net: rungs **1–8 DA-APPROVED** (8 for its delivered
> scope); remaining build work = **`AS-8B`** session-upload packed packets; then **F** closeout (consolidate
> AS-3/AS-4/AS-5/AS-8 sub-rung docs into one ledger each).
>
> **DA graduation log (executive DA, 2026-06-29, AS-8B re-review after PR #975).** Rung **8B `AS-8B`** →
> **DONE / DA-APPROVED. Brief NOT exceeded** — the 37-file diff (13 driver · 5 gpu · 4 sim · ~13 existing
> test files · 2 docs) is the *legitimate adoption surface* of sealing five session-upload fns: every call
> site across three crates + their tests rewired to pack-before-upload (`PackedAccumulatorUpload`/
> `PackedThresholdUpload`/`PackedIntentUpload`). **Zero creep** — no AS-F / 0.0.8.5 / channel_key / coord /
> fabric files touched (only the ladder doc, 10 lines). This is the *opposite* of AS-2's failure: AS-2 was
> held for too *few* files (defined-but-not-adopted); AS-8B has many *because* it adopted fully. Seal real
> (session API consumes packed packets only; semantic slices pack CPU-side **above** the seam — correct per
> §4), 6 `compile_fail`, parity preserved (83+14 tests). **The GPU residue is now genuinely shader-text-only.**
> Net: **rungs 1–8B all DA-APPROVED**; the last build rung is done. **`AS-F` closeout is now cleared to open**
> (consolidate sub-rung docs + final Scope Ledger). The handoff-spine "one authoritative path" line stays a
> **directive** — the *upload* seal is complete, but the write-seal / emission-seal that make the full
> invariant a type-fact are the separate AS-9+ cluster.

> **AS-F closeout log (2026-06-29).** AS-F opened after AS-8B DA approval. Build ladder rungs 1–8B are DONE / DA-APPROVED. This closeout consolidates sub-rung evidence, final Scope Ledger, net-negative enforcement ledger, and honest residue ledger (`docs/tests/as_closeout_0_results.md`). No source code changes; no AS-9 or 0.0.8.5 scope. GPU upload residue is shader-text only. DA final review required before AS-F → DONE — DA-APPROVED.

*Recipient* follows the handoff-template routing (Type → agent). *State:* `DONE` · `OPEN` (queued) · set to `IN PROGRESS` / `PROBATION` / `DA-APPROVED` as a rung lands.

### 2.1 Exit states for a violation — where the behavior goes

The type boundary makes a violating *form* uncompilable; the **behavior that form encoded still needs a
conformant home.** A rung therefore has more than one response, chosen by what the behavior **is**, not by
convenience. **Deletion and EML reduction are siblings, not a choice between bloat and loss:** deletion is
for *redundant* code; EML reduction is for *useful computation currently expressed as host control flow.*

| The violating code is… | Exit state | Destination |
|---|---|---|
| dead / redundant / already computed elsewhere | **Delete** | — (the default for redundancy) |
| a branch over **static authoring metadata** | **Resolve-away** | a flag/column resolved at the CPU prep pass; the runtime sees the resolved value (§4 "compile away before upload") |
| a **dynamic computation** — predicate, formula, weight, urgency, threshold over runtime column values | **Reduce to EML** | an EML gadget tree / opcode stack over the one `EvalEML` interpreter (Anchor B, core §4.1); the host branch dies, the behavior becomes GPU-resident data |
| **flow / reduction / allocation** | **Refactor to RF** | an AccumulatorOp arena (reduce-up / disburse-down) — *not* EML (EML is expression eval; reduction is the sweep) |
| a **modifier** | **Refactor to overlay** | a `PropertyTransformDelta` on a weight/column |
| genuine CPU-boundary work types cannot reach | **Escalate to residue** | stays prose + admission + DA judgment (§5) |

The canonical violation — a `match kind` in the tick path — is almost never *delete*: its intent is a
predicate, so it reduces to a `SELECT`/`CMP` EML gadget over a resolved column, which is **both**
uncompilable-as-a-host-branch (this track's goal) **and** GPU-resident data (Anchor B's goal). This track
and the EML path are the **same move from opposite ends**: AS removes the host-side semantic branch by
making it unrepresentable; EML supplies the destination for what that branch meant.

**The fence — EML reduction may not launder a subsystem.** A "reduction" does **not** keep a violating
system alive; it re-expresses the behavior as conformant data so it *stops* violating. It is a valid exit
**only** if the result is (a) **generic / semantic-free** — the opcode stack names no game concept, only
floats and resolved indices; (b) **bounded** — the bounded-feedback contract (finite decay < 1, explicit
clamp; `ExactDeterministic` ≤32 nodes); and (c) **CPU-oracle bit-exact** — the rung stays a *refactor*
(same resolved value, new substrate), never a behavior change. If a "reduction" needs a new **semantic**
opcode, or cannot meet the admission contract, it is the violation wearing EML clothing — the exit reverts
to delete or escalate (core §9 detector 9).

**GPU lowering & JIT are downstream of this, not part of it.** AS-1/AS-2 harden the CPU→GPU **resolution
boundary**: the `ColumnIndex` / channel newtypes are exactly the resolved descriptors the GPU upload
consumes, so "wrong column binding" — a WGSL-trust-domain bug Rust otherwise cannot see (§5) — becomes
partly catchable on the host (a `ColumnIndex` is unconstructable except via layout resolution). The EML
exit (AS-3/AS-4) is the **behavior** lowering: a reduced gadget runs in the unified kernel. **JIT
EML→WGSL compilation of a hot reduced gadget is a separate, later optimization tier** (pinned artifact +
CPU-oracle parity — the Candidate-F precedent), never bundled into an AS refactor rung, which must stay
behavior-preserving and small. **AS reduces to the interpreter; the JIT, if a gadget proves hot, is a
perf track's job.**

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

> **The residue is smaller than first written (revised 2026-06-28).** AS-7 and AS-8 reclassified two items
> originally called unreachable: a *slice* of each turned out to be type-reachable — the hot-path slice of
> no-CPU-planner via the tick-input type (AS-7), and the upload slice of the GPU boundary via the
> packed-payload type (AS-8). §1.2 applied to this section's own residue (Gemini review). What remains:

- **no-CPU-planner — *boundary-time only*** (the hot-path slice is now AS-7). Planning *across* ticks at a
  boundary is behavior-over-time; types can't reach it — stays constitutional + reviewed.
- **no-flattening** (§0.6) — specified-vs-implemented recursive structure; types cannot prove a tier was
  not silently collapsed. Stays constitutional + reviewed.
- **The GPU trust domain — *shader-text only*** (structural upload sealed by AS-8; session upload sealed by
  AS-8B). Rust cannot see into WGSL text; CPU-oracle parity + the semantic-free *shader*
  scan remain the only admission the shader has (§4).
- **Live ontological conformance** — "is this still one accumulate→reduce→threshold loop?" remains DA work.

## 5A. Horizon addenda — tabled type moves (explored later, not opened)

Candidates surfaced (Grok review, 2026-06-28) that are elegant but not yet sharp enough to open as rungs;
recorded so they are not re-derived. Each opens only when a rung names it.

- **Phantom resolution state** — **PROMOTED to rung AS-8** (`AS-PACKED-UPLOAD-BOUNDARY-0`, 2026-06-28):
  the CPU-prep → GPU-upload seam type-enforcement is now a ladder rung, not a tabled idea.
- **Overlay-lifecycle typestate** — active vs `Suspended` as typestate so a suspended overlay cannot be
  mutated/reduced. Medium value; risks heaviness against the band-applied kernel.
- **Arena settlement-phase typestate** — local-settled vs not, in recursive RF. Likely too heavy for the
  current design; revisit only if a settlement-order bug recurs.
- **OrderBand dependency ordering at the type level** — encode band-before-band as types. Ambitious and
  elegant, but high risk of killing the data-driven flexibility OrderBands exist for. Research only.
- **Const-generic bounded capacities** — only where a cap is *truly* static; most caps are admission-checked
  data and must stay data (genericity). Narrow applicability.

**Eliminated (leave as data / already done) — do not open:** `ReductionRule` / `ClampBehavior` /
`FissionPolicy` stay **data** (typing them costs genericity for no safety; already resolved at prep, never
branched in hot paths); `CompiledMappingPlan` / `CompiledAccumulatorOpPlan` are already construction-gated
opaque handles; `Event` / `BoundaryRequest` are CPU-boundary **residue** by design (§5). The
sealed-trait / private-module technique for `simthing-sim`'s surface is **not a separate rung — it is the
mechanism of AS-4.**

## 6. References

- The doctrine: [`simthing_core_design.md`](simthing_core_design.md) §1.2 (+ §2, §3, §4, §5.1, §9 "Promotion over repetition").
- Constitution: [`design_0_0_8_3.md`](design_0_0_8_3.md) §0.
- Handoff discipline: [`handoff_template.md`](handoff_template.md) (§H + the context spine).
- Deferred successor: the ClauseScript Terran-Pirate galaxy track (resequenced to **0.0.8.5**) —
  [`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md);
  it inherits every invariant this track promotes for free.
