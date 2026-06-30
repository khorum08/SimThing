# 0.0.8.4.5 — SimThing Kernel: Constitution-as-Admission-Substrate

> **Status: OPEN — GREENLIT for orchestration + production (owner-approved 2026-06-29).** Opening sequence:
> the cheap pair `KERNEL-FORBID-UNSAFE-0` + `KERNEL-DEP-BUDGET-0` (parallel, Cursor/Grok), then the seals
> (write-seal gates 0.0.8.5's live-STEAD-decision phase), then the crate extraction. Per-rung PROBATION →
> DA re-review → DONE, same loop as the closed 0.0.8.4 track. Sits *beneath* the permanent
> paradigm [`simthing_core_design.md`](simthing_core_design.md) (esp. **§1.2 the admission substrate**) and
> *beneath* the constitution [`design_0_0_8_3.md`](design_0_0_8_3.md). It is the **keystone follow-on to the
> closed 0.0.8.4 Admission Substrate** ([`design_0_0_8_4_admission_substrate.md`](design_0_0_8_4_admission_substrate.md))
> and is sequenced **between** it and the 0.0.8.5 Terran-Pirate track
> ([`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md)).
>
> **Thesis — Constitution-as-admission-substrate.** 0.0.8.4 promoted nine invariants from prose to types.
> This track promotes the *spine itself*: it makes **"the accumulate→reduce→mask→threshold sweep is the only
> authoritative path to mutate resolved state or emit a decision"** a **type-and-dependency fact**, enforced
> by a minimal-dependency `simthing-kernel` crate — so the most load-bearing line of the constitution stops
> being a directive an agent must hold and becomes a wall an agent cannot pass. Like 0.0.8.4, its success
> metric is **net-negative enforcement surface**, and every rung is a **pure refactor** (CPU-oracle parity
> bit-exact).

---

## 0. Track harness header (constitution §0.5 Rule 1)

**Fixed base (durable; hold every rung):**

1. [`simthing_core_design.md`](simthing_core_design.md) **§1.2** (admission ladder), **§0.0/§5** (one authoritative loop), **§4** (semantic-free sim; compile-away), **§7** (arena-pressure projection → heatmap).
2. [`design_0_0_8_3.md`](design_0_0_8_3.md) §0 (constitution; anti-flattening §0.6).
3. **This file** — the 0.0.8.4.5 canonical design file.
4. [`design_0_0_8_4_admission_substrate.md`](design_0_0_8_4_admission_substrate.md) — the closed substrate this builds on (`SimulationFabric`, `ColumnIndex`/`SlotIndex`, `StructuralCoord`, channel newtypes, `PackedUpload`) + its §2.1 exit-state taxonomy.
5. [`handoff_template.md`](handoff_template.md) — binding handoff skeleton (§H anti-kabuki + the context spine, incl. the "one authoritative path" directive this track upgrades to a type-fact).

**Established decisions — do NOT re-derive:**

- **Pure refactor:** types/crate structure change, resolved values do not. CPU-oracle parity bit-exact; existing tests green. A rung that changes a resolved value is wrong.
- **Seals before extraction.** The write/emission/participation seals land *within existing crates first* (the write-seal is a 0.0.8.5 precondition — below); the `simthing-kernel` crate extraction is the **keystone last**, making the seals dependency-enforced rather than convention.
- **Ship every seal with its sanctioned channel (§2.1).** A seal that blocks the direct path without a visible conformant path (EML gadget / RF arena / `BoundaryProtocol`) *pushes capable models toward sidecars*. Each seal rung names the sanctioned alternative; the goal is *channeling*, not just denial.
- **Deny escape *primitives* at the highest admission rung** — `#![forbid(unsafe_code)]` (compiler) + minimal `Cargo.toml` deps (dependency graph), **never** a grep token-firewall (that is the D8 noun-for-verb regression).
- **The kernel crate IS the authoritative-runtime admission surface.** It is the sole owner of authoritative state and the sole minter of authoritative effects (mutations + emissions). Every other crate either depends on its **read-only view** (to observe) or produces **registrations/EML data** (which it executes). It is the *runtime-authority* layer; it **composes with, does not replace,** the content-admission layer (hydration/spec), which compiles *down* to kernel registrations. Document them as two composed layers, never conflated.
- **Performance is paramount — the seals MUST be zero-cost.** This crate is the per-tick hot path for the 2000-star / billion-pop benchmark. Every seal is a *compile-time* construct: capability tokens are **ZSTs** (zero-sized, compile to nothing), newtypes are `#[repr(transparent)]`, views are borrows — **no runtime check, no indirection, no dynamic dispatch, no allocation** on the hot path. The crate boundary must **preserve hot-path inlining** (`#[inline]`/`#[inline(always)]` where measured, LTO verified). A seal that adds runtime cost is wrong even if it compiles. (This is *additional to* value parity — see §2 DoD.)
- **Consumer/corpus streamlining is consumer-pulled, NOT bundled here.** This track *enables* ClauseThing-admission and consumer simplification; it does not perform them (constitution consumer-pulled discipline).

---

## 1. Objective, the STEAD precondition, and the productization payoff

**Objective.** Make the accumulator sweep the *only typed/dependency-enforced* path to authoritative mutation
and decision emission, housed in a minimal-dependency `simthing-kernel` crate.

**Why now (the STEAD precondition — Grok).** 0.0.8.5's central mandate is *"all decisions made entirely by
STEAD."* STEAD decides on the Movement-Front; the front is only *complete* if **every** effect flows through
the accumulator → arena-pressure projection → grid-cell heatmap. A bypass mutation is **invisible to the
front**, so STEAD would decide on a front that lies. **The write-seal is the necessary precondition that
makes the front complete-able** (the projection bindings make it actual — §5). Therefore the write-seal must
land **before 0.0.8.5's live-STEAD-decision phase** (it does *not* block 0.0.8.5's earlier galaxy/ownership/
fleet phases — those run in parallel).

**Why a crate (the productization boundary — owner's point).** A zero-dependency kernel with a small sealed
surface is the artifact core §1's *second mission* ships: consumers (Studio, the LEWM/field-movie corpus
tooling, future modders, any "code meant to use with SimThings") depend on the kernel's **read-only view**,
not on internal mutability. The extraction also yields a **reusable template** — minimal-dep core + sealed
mutation + read-only view — applicable later to other authorities (spec, scenario). This is permanent
downstream leverage, not internal hygiene.

**The payoff at closeout.** The handoff-spine "one authoritative path" line **upgrades from directive to
type-fact**, and "all conflict is resource flow" becomes *uncompilable to bypass* — the constitution's spine,
enforced by the dependency graph.

### 1.1 Standing ruling — the kernel is *the* SimThing admission-substrate authority (owner-directed, 2026-06-29, permanent)

`simthing-kernel` is **the SimThing rustification embodied as a crate**: the single home of authoritative
state and effects, where Doctrine-as-Type is enforced by *ownership*, not convention. The crate-extract
review proved why this is not optional — **a seal cannot hold across a crate boundary** (cross-crate `&Buffer`
/ `pub` minter / shared `ctx` is capability-for-everyone; Rust has no friend visibility). Therefore:

1. **Authoritative state and the code that mutates it live in the same crate — the kernel.** The GPU
   dispatch, encode, and readback that bind authoritative buffers belong **inside** the kernel; `&Buffer`,
   the `Queue`/`Device` `ctx`, and any raw write path **never cross the kernel boundary.** Consumers get
   high-level sealed entry points (`dispatch_tick()`, `read_*() -> Vec<Sealed>`) and a read-only view.
2. **When sealing is in tension with crate convenience, Doctrine-as-Type wins, and the code moves into the
   kernel** — never the other way (never weaken a seal to keep code outside). Size is not a reason to leave
   authority unsealed (owner directive 2026-06-29: "I don't care how big the slice is").
3. **Other authority-bearing services are readjusted the same way over time.** Whatever currently holds
   authoritative state or mints authoritative effects outside the kernel (today: GPU session/pipeline
   orchestration; later: candidates in spec/scenario) migrates **into** the kernel as it is next touched —
   consumer-pulled, but the direction is fixed: **authority gravitates to the kernel.** This compounds for
   every project built on SimThings — they depend on one sealed authority crate, not a federation of
   convention-protected seams.

This ruling is a **mandatory closeout landing** (§2A): it lands in the Core Design Doc and Constitution §0
so it propagates as permanent, cross-version doctrine.

### 1.1.1 Owner ruling — runtime authority lives with dispatch (KERNEL-DISPATCH-INCRATE-0)

Rustification is permanent doctrine: constitution-and-doctrine-as-type architecture is law. For runtime
authority, `simthing-kernel` is the home of that law.

A sealed authoritative type must be minted in the same crate that owns the private source of truth it is
minted from. A crate boundary cannot be sealed by a public capability token because Rust has no friend-crate
visibility: anything callable by `simthing-gpu` is callable by any dependent sidecar crate.

Therefore, authoritative buffers and the code that encodes, binds, dispatches, reads back, and mints from
them belong inside `simthing-kernel`. `simthing-gpu` may provide broad GPU utilities and thin caller/adaptor
surfaces, but it must not own or expose authoritative write/bind/readback paths.

---

## 2. The ladder

Each rung: one `compile_fail` (or a `cargo`-deny / dependency check) proving the illegal state no longer
builds; retire the prose/guard it replaces; pure refactor; one results ledger. Recipient per the handoff
routing (coding → Cursor/Grok; closeout → Opus/Owner).

**Per-rung definition of done — two parities, both gated:** (1) **value parity** — CPU-oracle bit-exact, no
resolved value changes; (2) **performance parity** — the seal is a zero-cost construct (token is a ZST /
newtype is `#[repr(transparent)]` / view is a borrow) **and** a microbenchmark (or the existing resident
tick over a representative slot count) shows **no regression** vs the pre-seal baseline. The crate-extraction
rung additionally proves hot-path **inlining is preserved across the boundary** (LTO on; `#[inline]` where
measured). A rung that adds runtime cost fails DoD even if it compiles and parity holds.

| Rung | ID | Promote | Type/dependency move | Retires | Recipient | State |
|---|---|---|---|---|---|---|
| 0 | `KERNEL-TRACK-OPEN-0` | — | This doc + evidence-index row. | — | Haiku/Sonnet | OPEN |
| 1 | `KERNEL-FORBID-UNSAFE-0` | the `unsafe`/raw-pointer sidecar primitive | Relocate any `unsafe` behind the GPU boundary; add `#![forbid(unsafe_code)]` to the semantic-free crates (`simthing-sim`, later `simthing-kernel`). **Compiler-enforced**, strictly stronger than a grep firewall. | Any "no `unsafe`" prose/scan; denies `transmute`/`*mut` sidecars. **All-projects, durable.** | Cursor/Grok | **DONE — DA-APPROVED** (2026-06-29) — `#![forbid(unsafe_code)]` on `simthing-sim`; zero residual `unsafe` (compilation = proof) (`docs/tests/kernel_forbid_unsafe_0_results.md`) |
| 2 | `KERNEL-DEP-BUDGET-0` | the import-a-sidecar-tool vector | Pin/minimize `simthing-sim` `Cargo.toml`; a new dependency requires DA sign-off (a `cargo`-deny/`deny.toml` check). Precursor to the extraction. | Implicit "don't add heavy deps" prose. | Cursor/Grok | **DONE — DA-APPROVED** (2026-06-29) — bidirectional allowlist lib-gate (adversarially proven; runs in `cargo test`); `tempfile` removed. **DA note (revised 2026-06-29):** the lib-test is the **sole real gate** (accepted as the handoff's "equivalent repo-native check"). The inert `deny.toml` stub was **removed** — a compliance-shaped artifact that enforces nothing is a handwave vector (a future agent could cite its existence as "dependency compliance" without checking the real gate). If `cargo-deny` is ever adopted, its config is created **fresh-and-wired** at that point, never as an unenforcing stub (`docs/tests/kernel_dep_budget_0_results.md`) |
| 3 | `KERNEL-WRITE-SEAL-0` | §0.0/§5 — only the sweep mutates resolved state | Resolved column buffer mutable **only** via kernel accumulator passes + the narrow `BoundaryProtocol`; no public setter, no mutation even via `ColumnIndex` except an explicit greppable `raw_lane()` for serialization. Enforced by a kernel-minted capability token (within-crate) — the precursor to crate ownership (rung 6). **Sanctioned channel: EML gadget / RF arena / BoundaryProtocol (§2.1).** | The "one authoritative path" *directive* → type. `compile_fail`: external code mutating a resolved column. **The 0.0.8.5 STEAD precondition.** | Cursor/Grok | **DONE — DA-APPROVED** (2026-06-29) — sealed `pub(crate)` `ResolvedGpuBuffers` (no `&Buffer` exposed); `install_resolved_*_at_boundary` + dispatch wrappers are the sanctioned channel; `compile_fail` on external `state.resolved.values`. **`ResolvedWriteAuthority(())` is a ZST (zero-cost) with a private field + private `boundary_install()` minter — genuinely unforgeable.** Adoption complete (zero residual GPU `write_*`); perf parity by construction (ZST, no runtime branch) + `c1_threshold_perf`. **This clears the gate for 0.0.8.5's live-STEAD-decision phase.** (`docs/tests/kernel_write_seal_0_results.md`) |
| 4 | `KERNEL-EMISSION-SEAL-0` | §8 — decisions are threshold crossings | `EmissionRecord` / decision types get **private constructors**; only the threshold-crossing logic (kernel or its CPU-oracle twin) can mint one. **Sanctioned channel: `Threshold`→`EmitEvent`→`BoundaryRequest`.** | "decisions are GPU-resident threshold crossings" prose → type. `compile_fail`: forging an `EmissionRecord`. | Cursor/Grok | **DONE — DA-APPROVED** (2026-06-29, after 0R) — the named-constructor forge vector is closed: **`from_boundary_delivery` deleted** (gone, not deprecated; `compile_fail` now targets the *named-constructor* vector + the struct-literal one). The only public emission producer is **`cpu_oracle_threshold_events`, the sanctioned CPU-oracle twin** — `event_kind` comes from the registration, events only on genuine crossings (real threshold logic), **not** a caller-picked output; no `pub fn new → SealedType` raw minter remains. Adoption complete (all callers were test-only, migrated to oracle crossings). **Soft point / future tightening:** the public oracle twin is the looser surface; crate extraction (rung 6) may move it `pub(crate)`/behind the dependency boundary. With rungs 3+4 done, **both halves the 0.0.8.5 decision phase needs are in place.** (`docs/tests/kernel_emission_seal_0_results.md`) |
| 5 | `KERNEL-PARTICIPATION-SEAL-0` | §5.2/§7 — spatial arenas need placed participants | Arena registration accepts only typed participants; a **spatial** arena requires a `StructuralCoord` placement proof (`validate_spatial_binding` as a type, not a runtime check). | The "property possession never admits" / spatial-binding runtime guards → type. `compile_fail`: an unplaced participant entering a spatial arena. | Cursor/Grok | **DONE — DA-APPROVED** (2026-06-29) — `PlacedParticipant` private fields + `_private` marker; **both `compile_fail`s present (struct-literal AND named-constructor `from_validated_spatial_binding`)** — the emission lesson applied unprompted. `pub(crate)` minter; **no public raw `from_coord`/`from_raw`/`new`**; the public `validate_and_mint_…` takes a binding table and the coord comes from the table, not the caller — no raw forge. `ExplicitParticipantSpec::spatial` requires the proof; `compile_fail` on unplaced spatial enrollment. **Deposit-arena-`flat()` confirmed correct** (spatially-neutral feedstock, no Movement-Front projection; `mapgen_rf_stead_binding` green would catch a mis-classified spatial-arena-on-flat). Parity preserved; zero-cost newtype. *Forward watch: if the deposit arena ever gains a heatmap projection it must migrate to `spatial()`.* (`docs/tests/kernel_participation_seal_0_results.md`) |
| 6 | `KERNEL-CRATE-EXTRACT-0` | §4 — semantic-free core as a dependency-graph fact | **The keystone.** Carve the already-identified semantic-free core (the `AccumulatorOp` passes behind `SimulationFabric` + the sealed column buffer) into **`simthing-kernel`**: minimal deps (ideally only `simthing-eml` + the fabric/index types), `#![forbid(unsafe_code)]`, owns the only mutable column buffer; every other crate depends on its **read-only view**. The seals (3–5) become dependency-enforced, not convention. **May split.** | The semantic-free *source scans* → narrowed to shader-text only; the kernel's dep graph is now the firewall. `compile_fail`/dep-check: a consumer crate reaching kernel-internal mutation. | Cursor/Grok | **PROBATION** — `KERNEL-DISPATCH-INCRATE-0` completed the seal: GPU dispatch/encode/readback + authoritative buffers live in `simthing-kernel`; `pub(crate)` buffer accessors; no public POD bridge or authority minter; 20 doc `compile_fail`s; parity green (`docs/tests/kernel_dispatch_incrate_0_results.md`). DA re-review required before DONE. |
| F | `KERNEL-CLOSEOUT-0` | — | Scope Ledger + perf-parity ledger; consolidate sub-rung docs (one ledger each); **the four mandatory documentation landings below (§2A)**. | — | Opus/Owner (DA) | OPEN |

### 2A. Closeout documentation guarantees (binding — the track does not close without all four)

`KERNEL-CLOSEOUT-0` is **not** done until each of these lands; they are gating, not optional:

1. **SimThing Core Design Doc** — a new section documenting `simthing-kernel` as the authoritative-runtime admission surface: its role (sole owner of authoritative state + sole minter of effects), the two-layer relationship with content-admission, the read-only-view contract for consumers, and the **zero-cost / hot-path performance mandate** (so future agents never reach for a runtime-checked seal). This is the permanent paradigm record.
2. **Constitution §0 (transient carry-forward)** — a short clause naming the kernel crate as the runtime authoritative path, added **by addition** to §0 so it **propagates verbatim to every future constitution version** (the prominent, cross-version home). It states: authoritative state/effects exist only behind the kernel surface; bypass is a constitutional violation *and* a compile error.
3. **Handoff template** — the context spine's "one authoritative path" line **upgrades from directive to type-fact**, AND the template gains a short **"the kernel door"** pointer: *to affect authoritative state, observe via the kernel read-only view and produce a registration / EML / `BoundaryProtocol` effect — here is the entry point; do not grope for sealed paths.* This converts the seal from a wall agents discover by compile-error into a signpost they read first — the token-economy point, applied to the seal itself.
4. **STEAD-completeness statement** — recorded in the closeout ledger: write-seal (no bypass) + projection bindings (visibility) ⇒ the Movement-Front is complete by construction, so 0.0.8.5's STEAD-only decisions are sound.

5. **The Admission-Substrate Amendment Valve (§3A)** documented at the same three altitudes — Core Design Doc, Constitution §0 (carry-forward), and the handoff template (with the `admission-amendment-request` request-permission gate). The sanctioned escape must be as well-known as the seals it governs.

Guarantee (1)+(2)+(3)+(5) means the kernel's role **and its owner-gated escape** are documented at all three altitudes — permanent paradigm, cross-version constitution, and the agent-facing handoff — so no agent burns tokens blindly feeling through sealed paths, and no agent invents a sidecar when a legitimate amendment is the answer.

---

## 3. Sequencing with 0.0.8.5 (parallel, with one gate)

This track does **not** serialize in front of 0.0.8.5. They overlap:

- **0.0.8.5 early phases** (galaxy generation, ownership, planets/factories/cohorts, fleets, hydration) inherit the AS-1–8B boundaries and need **nothing** from this track — proceed in parallel.
- **`KERNEL-WRITE-SEAL-0` (rung 3) is the one gate:** it must land **before 0.0.8.5's live-STEAD-decision phase**, because that phase's soundness depends on a complete front. The emission-seal (4) gates the same phase.
- **`KERNEL-CRATE-EXTRACT-0` (rung 6)** can land before *or shortly after* 0.0.8.5's decision phase — the seals already deliver the behavior; the crate makes them dependency-enforced. It must not be rushed into 0.0.8.5's critical path.

So the flagship 2000-star demo is **not delayed**: the cheap hardening (1–2) and the seals (3–5) are small and land alongside 0.0.8.5's early work; only the decision phase waits on the write-seal, which it needed anyway.

## 3A. The Admission-Substrate Amendment Valve (owner-gated escape — binding)

The seals are deliberately rigid; that rigidity is the point. But a genuinely new feature or refactor may
someday need to **add** an authoritative kernel entry, **repair** a seal, or **temporarily suspend** a
restriction. This valve is the **only sanctioned way** to do so — it exists so a legitimate need routes
through a loud, recorded, owner-signed door instead of a sidecar (the §2.1 principle applied to the seal
itself: the sanctioned channel for changing a seal *is* this valve). It applies to **all** admission-substrate
seals — the closed AS-1–8B boundaries and this track's kernel seals.

**Three gates, all required, default-denied:**

1. **Request-permission (handoff-level).** By default an agent may **not** request the valve. An orchestrator
   may, in a specific handoff, explicitly grant permission to *request* it
   (`admission-amendment-request: allowed`, handoff §1). Without that line, the agent **escalates the blocker
   to the DA** — it does not propose a seal change, and it does not work around the seal.
2. **Owner / Exec-DA approval, interrogation-backed.** Even with request-permission, the valve opens **only**
   on the Owner's approval or a direct Owner→DA direction, **after the Owner interrogates the need**: why is
   this required? why can it not be a registration / EML gadget / overlay within the existing seal? is it
   add, repair, or suspend? what is the blast radius? Only the Owner or Exec-DA writes the grant; it is never
   pre-filed; the agent **never self-grants.**
3. **Recorded amendment + loud, temporary suspensions.** Every grant is a written Deviation/Amendment Record
   (§0.6 discipline): what changed, why, add/repair/suspend, scope — and for a **suspension**, a **re-seal
   plan + expiry.** The amendment carries a **greppable named marker** in code; a suspended restriction is
   never an invisible hole — it is a clearly-named, DA-gated, audit-flagged escape whose default is re-seal.

This preserves the seal's integrity (default-denied, owner-gated, recorded, loud) while guaranteeing
legitimate future needs a sanctioned path — which is itself the anti-sidecar mechanism: when the only way to
change a seal is this loud owner-signed door, even a frontier model routes to the *request*, not a hidden
bypass.

## 4. What this opens — recorded, NOT bundled (consumer-pulled)

The kernel boundary *enables* these; per consumer-pulled discipline they open only when a consumer names them:

- **ClauseThing admission streamlining.** Once the kernel's sealed surface is the admission contract, many ClauseThing hydration checks collapse to "does this lower to a valid kernel registration?" — which the type system answers. The clausescript transpiler in **0.0.8.5** is the natural consumer that pulls this.
- **Consumer migration to the read-only view** (Studio, exporters, corpus tooling) — pulled per consumer.
- **The reusable authority template** (minimal-dep core + sealed mutation + read-only view) applied to the spec/scenario authorities — a later track, when those authorities are next touched.

## 5. Honest residue (stays prose + admission, by nature)

- **Projection-binding completeness.** The write-seal makes the front *complete-able* (necessary); the front is *actually* complete only where `ArenaPressureBindingSpec` projections exist (sufficient). Whether every gameplay arena has its projection is a **scenario-authoring** obligation (0.0.8.5), DA-reviewed — not a kernel type fact.
- **`no_std` is not pursued** — the GPU/`wgpu` path needs `std`; the kernel minimizes deps, it does not go bare-metal.
- **Live ontological conformance** ("is this still one accumulate→reduce→threshold loop?") remains DA judgment — the kernel makes *bypass* uncompilable, not *good modeling* automatic.
- **WGSL shader text** — the final residue Rust cannot see (unchanged).

### 5.1 Where the seal bottoms out (the terminus map — so no leak surfaces reactively)

The sealing recursion has a definite floor. `KERNEL-DISPATCH-INCRATE-0` closes the **last cross-crate
*visibility* leak** (the `&Buffer` / `ctx` path); after it, **no further crate-boundary path out exists in
safe Rust.** The adjacent forge vectors were checked against the tree (2026-06-29) and are **already closed**,
and the dispatch move must *preserve* them (DoD invariants, do not regress):

- **Transmute-from-bytes is closed:** the sealed types derive only `Clone/Copy/Debug/PartialEq(/Eq/Hash)` —
  **not `bytemuck::Pod`/`Zeroable`** — so `cast`-from-bytes cannot forge them. (Their `*Gpu` mirrors are Pod;
  the sealed types must never become Pod.)
- **Serde forge is closed:** the sealed types **do not derive `Deserialize`** (serde's derive is a backdoor
  constructor for private-field types). Any save/load of authority must go through a controlled, validated
  path, never `from_*` straight into a sealed type.
- **External-shader injection is closed:** the kernel accepts **no external `ShaderModule`/`Pipeline`** for
  authoritative buffers — the compute shaders are kernel-internal. The dispatch move keeps them so.

Below those, **two structural floors remain — not leaks, the acknowledged residue where type-enforcement
ends by nature:**

1. **WGSL shader text** (above) — a correct in-crate, semantic-free shader + CPU-oracle parity is the
   admission; Rust cannot type-check the shader.
2. **`unsafe` in a *consumer* crate** — the true terminus. `#![forbid(unsafe_code)]` governs the kernel and
   the crates we apply it to (e.g. `simthing-sim`), **not** arbitrary dependents. A consumer that writes
   `unsafe { mem::transmute }` can forge any sealed type by replicating its layout; no crate seal can stop a
   consumer's own `unsafe`. **So the seal is a type-*fact* within the kernel + forbid-unsafe crates, and an
   honest *directive* (+ the amendment valve, + license/social boundary) at the ecosystem edge** (third-party
   modders/researchers). That edge is unenforceable by types **by design**, and that is acceptable — it is
   where the productization boundary meets the open world.

**So the answer to "is dispatch-in-crate the end?": yes for the visibility recursion.** The residue
afterward is exactly `{ in-crate-WGSL-correctness, consumer-side unsafe }` — both already named, neither a
new crate-boundary path. Extending the seal as far as it *can* go = forbid-unsafe on every crate we own
(the `KERNEL-FORBID-UNSAFE` rung, generalized to `simthing-gpu` once the orchestration lands there too).

### 5.2 Bypass-state catalogue — the residue is a red-flag scan, not a passive gap (binding)

The residue is acceptable **only because routing through it is a *deliberate* act we can detect.** In safe
Rust within our crates you **cannot** forge a sealed authority type or reach an authoritative buffer by
accident — every route requires an unmistakable, **greppable** construct. The finite set of those constructs
is the **bypass-state catalogue**: each is a *deliberate-circumvention signal*, and a hit on any is a **red
flag** requiring DA sign-off (or a recorded amendment-valve grant). The scan is the highest admission rung
available for a surface types cannot reach (§1.2 rung-3 used *correctly* — over genuine residue, not as a
substitute for a type that should exist).

| # | Bypass state (deliberate-circumvention signal) | Detect |
|---|---|---|
| B1 | **`unsafe`** in any crate that can name a sealed authority type or an authoritative buffer (the `mem::transmute` forge route) | `forbid(unsafe_code)` on the crate (compiler); a new `unsafe` is a red flag |
| B2 | a sealed authority type deriving **`Pod`/`Zeroable`/`Deserialize`/`Serialize`/`Default`** (opens transmute / serde / default forge) | scan derives on sealed types; only `Clone/Copy/Debug/PartialEq(/Eq/Hash)` allowed |
| B3 | a `pub`/`#[doc(hidden)] pub fn` returning **`&Buffer`/`&mut Buffer`/`BindingResource`** for an authoritative buffer (the visibility leak) | boundary scan of the kernel public API |
| B4 | a `pub fn … -> SealedType` (or `Vec<…>`) taking **forgeable input** (`*Gpu` POD / raw `u32`/`f32` / `&[u8]`) — the bridge-launder | scan public producers of sealed types |
| B5 | accepting an **external `ShaderModule`/`ComputePipeline`/WGSL string** for authoritative buffers (custom-compute route) | scan kernel public API for shader/pipeline params |
| B6 | exposing **`ctx`/`Queue`/`Device`** so it can be paired with an authoritative buffer handle | boundary scan |
| B7 | **`mem::transmute` / `bytemuck::cast*`** whose target is a sealed authority type | grep (covered by B1 if unsafe-forbidden, but `bytemuck` cast is safe — B2 closes it by keeping sealed types non-Pod) |
| B8 | a new **dependency** on the kernel (or any crate that names sealed types) that pulls in `unsafe`/serde-for-authority | the dependency-budget gate |

**These are red flags, not auto-rejections.** A legitimate need routes through the **amendment valve** (§3A,
owner-gated); an illegitimate one is exactly the deliberate circumvention the catalogue exists to make loud.
Either way the route is *visible*. The orchestrator runs the scan on any rung that touches the kernel
authority surface; the handoff template flags the risk up front (handoff §1).

## 6. References

- The doctrine: [`simthing_core_design.md`](simthing_core_design.md) §1.2, §0.0, §4, §5, §7, §8.
- The substrate this builds on: [`design_0_0_8_4_admission_substrate.md`](design_0_0_8_4_admission_substrate.md) (§2.1 exit states; the AS-9 "sealed-authority" cluster this track realizes).
- Handoff discipline: [`handoff_template.md`](handoff_template.md) (§H; the spine line this track upgrades).
- Consumer: [`design_0_0_8_5_clausescript_terran_pirate_galaxy.md`](design_0_0_8_5_clausescript_terran_pirate_galaxy.md) (the STEAD-decision phase the write-seal gates).
