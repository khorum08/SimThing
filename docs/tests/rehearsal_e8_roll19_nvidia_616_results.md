# NINETEENTH E8 qualification roll — NVIDIA 616.92

DA increment per Orchestration relay `5755177528` (Astra return `5747826768`; 2.2
`0088-ECONOMY-FLEET-0`, consumer #2075 OPEN/DRAFT and truthfully incomplete). This is a runtime
qualification increment only. No 2.2 semantic verdict, no merge or graduation of #2075, no Model 2
reopening. `coverage_basis` on #2075 stays FAIL and is not overridden.

## What happened

The reference machine's driver moved **NVIDIA 595.79 → 616.92**. The resident-clearing
qualification tuple hashes `driver_runtime`, so the record's fingerprint moved and every product
session refused typed at ordinary open:

```text
ResidentClearing(LiveHead(UnqualifiedAdapter {
    required: 17165209339348674868,  // ee3712f2ef186934  (eighteenth pin)
    observed: 17263054877348392797   // ef92b0f2866cef5d  (live tuple)
}))
```

That refusal is the law working, not a regression: *"A changed tuple must be separately qualified;
it never inherits production authority by merely being able to create a device."*

## Independent capture (DA, on exact master `d3b58b75`)

A temporary probe (not committed) captured the live record through the ordinary
`ResidentClearingQualification::capture` door. Every field matches the relay:

| field | value |
|---|---|
| backend / adapter | `Vulkan` / `NVIDIA GeForce RTX 4080 Laptop GPU` |
| vendor / device / class | 4318 / 10144 / `DiscreteGpu` |
| driver_runtime | **`NVIDIA 616.92`** |
| compiler | rustc 1.95.0 (59807616e 2026-04-14), LLVM 22.1.2 |
| cargo features | `EML_RESOURCE_PROFILING` |
| shader compiler | wgpu 22.1.0 / naga 22.1.0 |
| cargo lock hash | `0x1bd6ab6097f443d7` |
| semantic kernel bundle | `0xca5fca9690ccccd8` |
| workgroups / subgroup / ABI | `[32, 64]` / subgroup-independent / 1 |
| **fingerprint** | **`0xef92_b0f2_866c_ef5d`** |

**The driver string is the ONLY delta.** A second probe cloned the live record, replaced
`driver_runtime` alone with `NVIDIA 595.79`, and recomputed: `0xee3712f2ef186934` — the eighteenth
pin exactly, with the two records otherwise field-for-field equal. So no sealed byte, ABI, feature
set, compiler, lock, bundle, workgroup or adapter identity moved.

**Sealed-byte isolation.** All 31 `build.rs` components and `Cargo.lock` are byte-identical to
master; the roll edits only the two pin literals. Note that
`crates/simthing-gpu/src/resident_clearing_runtime.rs`, which carries the production literal, is
deliberately NOT a hashed component — otherwise rolling the pin would move the bundle hash and the
fingerprint would chase its own tail.

## Old-pin negative, captured first and twice

- driver resident path: `UnqualifiedAdapter { required: 0xee3712f2ef186934, observed: 0xef92b0f2866cef5d }`;
- parity referee: FAILED, printing `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ef92b0f2866cef5d`.

The two derivations agree, and they agree with Astra's independent reproduction.

## Qualification battery under NVIDIA 616.92 (required = observed)

The successor is qualified by RUNNING the battery on the new environment, never by inheritance:

- **parity referee 1/1** — the terminal CPU-oracle/GPU bit-exactness referee, printing
  `ef92b0f2866cef5d`;
- **score-and-bands 3/3** — including the canonical-order/atomic-arrival mutant matrix;
- **runtime 4/4** — the three independently-bound semantics plus
  `changed_qualification_tuple_fails_typed_before_execution`, so the guard still refuses a changed
  tuple. No comparator, bypass or wildcard was introduced.
- **Ordinary-session ingress and regression**, all on the rolled pin, 0 failures: driver 54
  (settlement ownership, structural RF enrollment, the 2.1 construction floor — every one an
  ordinary product session that refused before the roll), workshop 16 (native structural
  products, recipe inputs, clamp session, parity, score-and-bands), gpu 7, kernel 55,
  clausething 47, mapeditor 25. Two suites first aborted on a corrupted incremental build
  state (`only metadata stub found for rlib dependency core`), a local artifact of pruning
  `target/debug/incremental`; rerun alone they pass 47/0 and 25/0.

## Clean-checkout reproduction

- commit: `14586116` (the exact nineteenth-roll commit);
- fresh `git clone` with `core.autocrlf=false` (canonical LF checkout) into sibling
  `simthing-e8-roll19-verify`, checked out at that exact sha;
- status clean before and after the run (0 modified entries), both literals read
  `0xef92_b0f2_866c_ef5d`;
- `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0`
  observed `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: ef92b0f2866cef5d`, referee 1 passed / 0 failed.

The clone was deleted after the proof: the evidence is the reproduced fingerprint, not the directory.

## Ambient wall-clock debt paid by this PR (disclosed)

Unrelated to the roll: the 7-day lease on `docs/workshop/archive/field_policy/README.md` expired on
2026-09-23, so `ARTIFACT-EXPIRY` fails for EVERY PR until someone pays it, and the roll PR is the
one that must be green. The safe reaper refuses `docs/` paths by design, so the disposition is by
hand and it is a deletion: the archive is superseded probe residue whose own README names the live
generic `structured_field_stencil` code that replaced it, its instrument has been compile-broken
since 2026-08-30, its track 0.0.8.7 closed on 2026-09-08, and nothing references the path. The
directory and its lease row are reaped together (only the README carried a row, so the three files
it indexed go with it). Leases are now 0; the deletion guard passes.

## Ruling: ONE exact fingerprint stays the law

The relay asked explicitly whether 595.79 and 616.92 should both remain admitted. **They should
not.** The pin is retired and replaced, not widened:

- the certification evidence is produced by running the battery on ONE live environment; an
  allowlist entry for 595.79 would assert a certification that can no longer be reproduced on this
  machine, which is an unfalsifiable claim and therefore decoration;
- CI runs no cargo tests by standing Owner ruling, so no second environment needs simultaneous
  admission; the fleet is one reference machine;
- a genuine multi-environment need (a second machine) is a qualification-POLICY change and must
  arrive as one: a per-environment certification record, a falsifier proving an uncertified tuple
  still refuses, and a retirement rule for stale entries. It is not inferred from a driver update.

Pin chain: `…0x9993…` → `0x17a0…` (17th) → `0xee37_12f2_ef18_6934` (18th) →
**`0xef92_b0f2_866c_ef5d` (19th)**.

## Consumer note for #2075 (a rebase IS required)

The qualification constant is COMPILED INTO the code under test. A branch that does not carry this
commit still compiles `0xee37…` and will keep refusing at ordinary session open, regardless of
routing. There is no environment override, and adding one would be exactly the guard bypass this
law forbids. So #2075 needs exactly ONE further synchronization onto the master that carries this
roll before Astra resumes the ordered gates.
