# StemThing §3.1 census — session digest for the incoming DA

Companion to `handoffs/STEMTHING-CENSUS-TIER2-SESSION.hd.md`. The handoff is the contract; this is
the evidence and the reasoning, so the next session starts from findings rather than from scratch.

- Authored at master `380e8630` (Phase 6 complete, 7/7)
- Pointer: **`none`** — parked deliberately; the constitutional flip is an Owner act
- Owner pre-approval (2026-08-03): core design + constitution already updated for EXP/LN and the
  StemThing unification; **any further doc/constitution changes the amendment needs are approved**

## 1. Where the track stands

Phase 6 closed 7/7: 6.0, 6.0b, 6.1, 6.1b, 6.2, 6.2b, 6.3 — all `DA-GRADUATED`. 5.10
(`EML-PRIMITIVE-DOMAIN-0`) graduated out of sequence under an Owner repoint. Remaining Phase 5:
5.11 `EML-EXP-PRIMITIVE-0`, 5.12 `EML-LN-PRIMITIVE-0`.

**Parallelism is Owner-ruled, not a judgment call.** `stead_stemthing_unification.md` §10: rungs
5.10–5.12 dispatch in the **same Phase 6 → 7 gap, in parallel with StemThing-A** — disjoint files
(kernel EML surfaces vs census/Tier-2 docs), disjoint lanes, no shared gate. Neither waits on the
other. The primitives are **not** StemThing components; the dependency runs the other way and
later — StemThing-B's derivation-pricing curves are horizon consumers of `EXP`/`LN`/`POW`.

Phase 7 waits behind StemThing-A. Do not dispatch 7.1 from pointer movement.

## 2. What §3.1 asks

Core-design registry discipline says slots recycle through tombstone free-lists, are never
compacted mid-session, and slot/column indices stay stable for the GPU. Epoch compaction with
physical relocation contradicts that, and **the core design wins until amended — silent
reinterpretation is inadmissible**. Two lawful shapes:

- **(a)** `SlotIndex` becomes stable *logical* identity; physical binding happens at boundary
  upload; compaction re-derives bindings at an epoch boundary with a **recorded remap**; zero
  per-access indirection between epochs. Requires the Tier-2 amendment **plus an enumeration of
  every slot-bearing artifact and its rebind path**.
- **(b)** No mid-session compaction; reclamation stays tombstone-recycled.

**One `BLOCKER` verdict forces shape (b).** Do not force (a).

## 3. Findings already established — re-verifiable, not assumed

### 3.1 The recorded-remap mechanism shape (a) requires ALREADY EXISTS

`crates/simthing-core/src/anchor_remap.rs`, landed at 5.2 `WRITE-DOOR-BAND-DELTA-0`:

```rust
pub struct AnchorLocusRemap {
    pub sim_thing_id: SimThingId,
    pub property_id:  SimPropertyId,
    pub from_slot: Option<SlotIndex>,   // None = birth
    pub to_slot:   Option<SlotIndex>,   // None = retire
    pub from_col:  Option<ColumnIndex>,
    pub to_col:    Option<ColumnIndex>,
}
```

`AnchorRemapOperation` covers `Fission`, `Fusion`, `Remove`, `AddChild`, `AddDimension`,
**`SlotCapacityGrow`** (genuinely slot-moving), `Reparent` (relation-only), and a consolidated
`BoundaryFlush`. Endpoints are validated by `validate_exact_anchor_remap_endpoints`
(`crates/simthing-sim/src/anchor_remap_encode.rs`), derived from authoritative pre/post snapshots,
**never fabricated**; stable-slot reparent carries an explicit empty/not-required witness.

**This is exactly "compaction re-derives bindings at an epoch boundary with a recorded remap."**
§3.1's central prerequisite is an existing, enforced door — the amendment should cite it, not
propose a replacement.

### 3.2 The replay recorder is slot-free

The 6.1/6.2 `IntegrationSchedule` (`crates/simthing-core/src/generation_stamp.rs`) carries
`parent_generation`, `child_generation`, `product_key`, `kind` — **no slot**. It is the single
replay authority (6.2 extended it with a row kind rather than minting a second log).

One replay surface does mention a slot: `SpecDelta::ScriptedInstanceSlotChanged { current_slot: u32 }`
in `crates/simthing-driver/src/spec_replay.rs:121`. It is a **raw-`u32` evader** the type-walk
misses and the grep cross-check catches — exactly the case the methodology's step 4 exists for.

Its verdict is **`REPLAY-RECORD`, not `BLOCKER`**, on the strength of Replay v3's own governing
invariant (module header):

> Every cross-reference uses an authored string id (`tree_id`, `event_id`) or a logical compound
> key. Raw `OverlayId`s are never serialized for spec state — they are process-local atomic ids
> that change every install.

It **records that a slot moved**; it does not **use a slot to resolve identity**. A historical
diff stating a slot value remains true of its epoch, and `AnchorLocusRemap` is what makes it
interpretable across one. Confirm this reading against `docs/adr/spec_session_state_replay.md`
§`OverlayId stability` before finalising the row.

### 3.3 The one slot-keyed ordering is not production

`crates/simthing-sim/src/legacy_oracle.rs:96-97` sorts on `(slot, col, event_kind)`. Its header:
*"Migration oracle harness (C-INF-2)... Runtime tick paths must not depend on this module — it
exists so migration PRs compare AccumulatorOp against legacy in one place."*

Parity harness, not authoritative order. **Not an `ORDER-PIN` blocker** — but §10 pre-flags
canonical fold order as a hard case, so sweep for *other* fold orders (notably the RF reduce-up
bucket sort, where `OwnerRef` string ordering is already known to permute bucket order — see 9.2's
row, which rules that persistent column layout must key on a stable interned owner id).

## 4. Provisional verdict

**No `BLOCKER` found; shape (a) is supported by existing mechanism.** Type-walk universe is
**47 consumer files** across six crates (`driver` 19, `kernel` 12, `core` 8, `sim` 3, `gpu` 3,
`spec` 2). Persistence, ordering and boundary walks each resolve to `REBIND-AT-BOUNDARY` or
`REPLAY-RECORD`.

**This is provisional and must not be published as exhaustive.**

## 5. What remains — the exhaustiveness proof

1. **Compile-fail type-walk** (methodology §2 step 1): in a throwaway branch, seal the `SlotIndex`
   constructor (`slot_index.rs:65 pub fn new`) and harvest the error list. Never land the branch.
   This is the step that converts "47 files reference it" into "N consumers, enumerated."
2. **Per-artifact rows** for all 47, in the §4 schema: `artifact | crate:file | representation |
   lifetime | order-bearing? | replay-bearing? | rebind path under (a) | verdict`.
3. **The reconciliation line** — row count vs type-walk consumer count vs grep residue, with every
   grep hit either a row or recorded provably-non-slot **with reason**. The methodology is explicit
   that this line *"is what makes it tree-derived exhaustive rather than curated."*
4. Grep cross-check terms per §2 step 4: `slot`, `row`, `linear_idx`, `* n_dims`, `SlotRange`,
   `INPUT_LIST` payloads, `base + k *`.

## 6. Then the amendment

- **Tier-2 core-design amendment**: restate the stability law as *stable within an epoch;
  rebindable only at a recorded boundary remap*, with the census enumeration as its evidence.
  Amend `docs/simthing_core_design.md` — the registry-discipline text is the thing that currently
  conflicts. Editing anchored sections breaks `anchor_check`; that failure is the mechanism that
  forces the re-stamp, so run `anchor_check.sh --resync` and re-stamp deliberately.
- **Mint the StemThing rungs** in `docs/design_0_0_8_7_rf_arena_modernization.md` §3b. §10 blocks
  minting until the Tier-2 ruling lands — so rule first, mint second. Give each row an exit-proof
  that covers its scope; `exit_proof_coverage_check.sh` will flag any that drift.
- **Anchor the governing docs.** Both `full_eml_unification.md` and `stead_stemthing_unification.md`
  are declared governing but have **zero rows** in `doctrine_anchors.tsv` and `anchor_triggers.tsv`
  — no content hash, unreachable via `anchor_query.sh`. Anchor them section-wise, as
  the now-retired temporary automata anchor was (3 anchor rows + 1 trigger row).

## 7. Traps this session paid for

- **A proof that exercises only the ordinary path cannot see the defect the rung exists to
  prevent.** Every Phase 6 remand had this shape: 6.1's egress had callers but no liveness witness;
  6.1b measured a ratio against a sub-nanosecond denominator and hid an allocation regression
  behind an interpretation story; 6.2 coalesced only in ascending order; 6.3 kept a second
  staleness truth. Ask it of every proof.
- **Plant the defect in production, not in the seam the author provided.** A referee that only
  catches its own `#[cfg(test)]` mutation is decoration.
- **`--lint` PASS ≠ dispatchable**: it does not check the coding-projection cap.
- **Prose is the weakest admission tier.** Types > admission error > guard > prose. Do not write a
  §4 law where a one-form type already enforces it.
- **The gates apply to the DA.** `gen_orientation` rejected a pointer I set to an invented rung id
  at the 6.3 stamp. That was a correct catch.

## 8. Standing delegation (do not re-litigate)

Handoff authorship is **orchestration's** for any rung `exit_proof_coverage_check.sh` does not
flag (`docs/handoff_template.md` §3, landed #1610). DA reviews pre-dispatch for scope/proof/fence
alignment, never authorship. Three consecutive delegated handoffs (6.2, 6.2b, 6.3) needed zero
corrections. Coverage currently flags **one** row: 11.2 `EMBEDDER-GUIDE-EXEMPLARS-0`, which is
DA-authored and still outstanding.
