# Codex/Cursor Handoff 6 (REMEDIAL) — `ATLAS-BATCH-0-STORE` fix

**Recipient model: Cursor** · **Role: production implementation agent**
**From:** Opus (design authority) · **Date:** 2026-06-03 · Diagnosed by running the suite directly.

> **It is NOT a crash.** Your command `cargo test ... 2>&1` in **PowerShell 5.1** wraps cargo's stderr
> as `NativeCommandError` and the non-zero exit reads as a "crash," hiding the real compiler output.
> **First fix the capture**, then you'll see the two real defects below:
> `cargo test -p simthing-driver --test dress_rehearsal_atlas_batch_0_store -- --nocapture *>&1 | Tee-Object docs/tests/scenario_0080_2_atlas_batch_0_store_cargo_test_2026_06_03.txt`
> (use `*>&1` in PS, not `2>&1`; or run under Bash). When green this is the raw-evidence log.

## What I observed (running it directly, Bash)

- Before fixes: **compile error** (defect A) → build fails → looks like a crash under `2>&1`.
- After adding the missing function: **10 passed, 1 FAILED** — `owner_indexed_entries_do_not_blind_sum_by_position` (defect B).

## Defect A — missing function (compile error)

`crates/simthing-driver/src/dress_rehearsal_atlas_batch_0_store.rs:222` calls `store_key_owner_rank(...)`,
but only `store_key_channel_rank` is defined. Add the analogue (right after `store_key_channel_rank`):

```rust
fn store_key_owner_rank(owner: Owner) -> u8 {
    match owner { Owner::Terran => 0, Owner::Pirate => 1 }
}
```
Also clear the 2 `unused import` warnings (`ChannelSet`) flagged in src:19 and test:14.

## Defect B — `FleetStrength(Pirate)` not emitted for constructed pirate (logic + design smell)

`owner_indexed_entries_do_not_blind_sum_by_position` adds a constructed patrol + pirate at system
`LocationId(1)` cell `(3,3)` and expects **both** `FleetStrength(Terran)` and `FleetStrength(Pirate)`
entries. The pirate one is missing because `occupant_kind_name` matches inconsistently:

```rust
} else if source_id.contains("patrol")        { Some("patrol")   // substring — matches "constructed-patrol-…"
} else if source_id.starts_with("pirate-ship"){ Some("pirate")   // PREFIX — FAILS for "constructed-pirate-ship-…"
```

`register_constructed_co_location_occupants` re-ids the occupants with a `constructed-` prefix, so the
constructed **pirate** fails the `starts_with("pirate-ship")` check → returns `None` → the
`Some("pirate") => push FleetStrength(Pirate)` arm never runs → the entry is absent.

**Root cause / smell:** deriving occupant kind by **parsing the source-id string** is fragile and is a
`match kind`-by-string-parse smell. **Preferred fix (do this):** derive the channel set — including
`FleetStrength(owner)` and `Patrol/PiratePresence` — from the **LOC occupant's structured data** (its
`ChannelSet` + its `owner`), not from the id string. `channels_for_source` already reads
`occupant.channels` first; make the LOC channel set (or an `owner`-keyed rule on it) the single source
of truth so a pirate occupant deterministically emits `PiratePresence` + `FleetStrength(Pirate)`
regardless of its id, and delete the id-string `occupant_kind_name` augmentation. **Minimum acceptable
fallback** (if you keep string parsing for now): make the matching consistent (anchor pirate the same
way as patrol) AND ensure the constructed pirate emits `PiratePresence` + `FleetStrength(Pirate)` — but
the structured-data fix is what removes the smell and is preferred.

## Requirements

- Keep STORE **CPU-only / fixture-only** (contract handoff 5 unchanged): no GPU, no `AccumulatorOp`/
  `EvalEML`, no `simthing-gpu`/`-core`/`-sim` edits, no `lib.rs` export, generic seeded values (no
  recipes). Do not edit GEN/LOC/PACK/PACK-GPU sources, constitution, or invariants.
- **Run to 11/11 green** with a clean capture (above); save the raw log; only then write the report/
  status row and update the production doc + worklog to PASS. "Execution pending" / a masked crash is
  not acceptance.
- Cite the fixed base harness + the §0.5 self-check on handoff back.

## Stop conditions (unchanged from handoff 5)

If the fix appears to need a live masked reduction / `AccumulatorOp` / GPU / new WGSL / runtime
`match kind` on a real `SimThingKind` / economy / FIELD_POLICY / movement / combat — STOP and escalate. The
correct fix is structured channel derivation from LOC data, which needs none of those.
