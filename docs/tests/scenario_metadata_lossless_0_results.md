# SCENARIO-METADATA-LOSSLESS-0 — lossless Scenario-root generator seed metadata

> **Lifecycle: PROBATION** — Lossless u64 seed encoding, roundtrip/rejection tests, sidecar sync proof, and corpus fixture update landed. GameSession child enforcement deferred. Pending owner DA approval.

**Date:** 2026-06-19  
**PR:** #778 — SCENARIO-METADATA-LOSSLESS-0  
**Merge SHA:** `a7522960`  
**Base:** `master` after PR #777 / SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0

## Defect summary

PR #777 correctly made `SimThingKind::Scenario` the canonical serializable file root and placed scenario metadata on Scenario-root properties. However, `scenario_metadata_seed_value` encoded a `u64` as two `f32` 32-bit halves:

```rust
// OLD (lossy)
data: vec![(seed & 0xFFFF_FFFF) as f32, (seed >> 32) as f32]
```

`f32` cannot exactly represent arbitrary 32-bit integer halves. Arbitrary `u64` generator seeds could be corrupted by canonical Scenario metadata serialization — blocking the next rung (SCENARIO-GAMESESSION-CHILD-0).

## Encoding choice

**Chosen: four u16 chunks stored as f32 values** (preferred option; least churn, compatible with existing `PropertyValue.data` vector convention and structural integer validation patterns).

Alternative (decimal UTF-8 string via string metadata encoding) was not chosen — it would require a different property shape and more serde churn.

## New lossless encoding

```rust
// NEW (lossless)
data: vec![
    (seed & 0xFFFF) as f32,
    ((seed >> 16) & 0xFFFF) as f32,
    ((seed >> 32) & 0xFFFF) as f32,
    ((seed >> 48) & 0xFFFF) as f32,
]
```

Decoding uses `checked_u16_f32`: rejects non-finite, fractional, or out-of-range (0..=65535) chunks; requires exactly four elements.

## Exact seed patterns tested

| Pattern | Value | Purpose |
|---|---|---|
| Zero | `0` | Backward-compatible seed 0 |
| Max | `u64::MAX` | All bits set; all chunks 65535 |
| High-bit | `0x8000_0000_0000_0001` | MSB + LSB set |
| Mixed | `0x1234_5678_9ABC_DEF0` | Corpus fixture seed; varied chunk values |

## Malformed metadata rejection

| Case | Behavior |
|---|---|
| `data.len() != 4` | `scenario_metadata_seed` returns `None`; canonical validation rejects |
| Fractional chunk (e.g. `1.5`) | `checked_u16_f32` rejects |
| Out-of-range chunk (e.g. `65536.0`) | `checked_u16_f32` rejects |
| Sidecar/root seed mismatch (non-zero sidecar) | `ScenarioMetadataMismatch { field: "generator_seed" }` |

## Sidecar sync proof

| Direction | Proof |
|---|---|
| Root → sidecar | `sync_sidecar_from_root_metadata` copies exact `u64::MAX` from root property to `provenance.generator_seed` |
| Sidecar → root | `sync_root_metadata_from_sidecar` recreates exact `0x8000_0000_0000_0001` in root property |
| Serialize | `serialize_scenario_authority` syncs sidecar from root before write; roundtrip preserves exact seed |

## Corpus fixture update

`scenarios/corpus/minimal_scenario_root.simthing-scenario.json`:

- Seed property `8300204`: `[43981.0, 26505.0, 9029.0, 1.0]` (JSON f64-safe non-zero seed `0x0001_2345_6789_ABCD`)
- Sidecar `generator_seed`: `320255973501901` (decimal mirror; must stay within JSON f64 integer precision)
- Full mixed pattern `0x1234_5678_9ABC_DEF0` and `u64::MAX` tested in code only (canonical root property encoding is lossless; JSON number sidecar mirrors cannot represent all u64 values exactly)
- Minimal structure unchanged; no GameSession child

## Specified-vs-implemented ledger

| Specified | Implemented | Status |
|---|---|---|
| Replace lossy two-f32 encoding | Four u16 chunks as f32 | PASS |
| Roundtrip arbitrary u64 exactly | Zero, MAX, high-bit, mixed tests | PASS |
| Reject malformed seed metadata | Length, fractional, out-of-range tests + validation | PASS |
| Sidecar sync exact | Root↔sidecar sync tests | PASS |
| Corpus fixture new encoding | `minimal_scenario_root.simthing-scenario.json` | PASS |
| Legacy World-root unchanged | Terran Pirate compatibility test | PASS |
| e10 regression guards | No lossy patterns; u64::MAX + mixed tests required | PASS |
| GameSession enforcement | Not implemented (deferred) | SKIP |
| GPU / Studio runtime / MapGenerator / ClauseThing | Not touched | PASS |

## Validation commands

| Command | Status |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo check -p simthing-core` | PASS |
| `cargo test -p simthing-core` | PASS |
| `cargo check -p simthing-spec` | PASS |
| `cargo test -p simthing-spec` | PASS |
| `cargo test -p simthing-spec --test scenario_serializable_simthing_root` | PASS |
| `cargo test -p simthing-spec --test e10_resource_flow_admission` | PASS |
| `cargo test -p simthing-mapeditor --test terran_pirate_skeleton` | PASS |
| `git diff --check` | PASS |
| `git diff --name-only master...HEAD` | See files changed |

## Files changed

| Path | Change |
|---|---|
| `crates/simthing-spec/src/spec/scenario.rs` | Lossless seed encode/decode; `checked_u16_f32`; canonical seed validation |
| `crates/simthing-spec/tests/scenario_serializable_simthing_root.rs` | Seed roundtrip/rejection/sync tests |
| `crates/simthing-spec/tests/e10_resource_flow_admission.rs` | Lossy-encoding regression guards |
| `scenarios/corpus/minimal_scenario_root.simthing-scenario.json` | Four-chunk seed encoding |
| `docs/tests/scenario_metadata_lossless_0_results.md` | This report |
| `docs/tests/current_evidence_index.md` | New row + qualify SCENARIO-SERIALIZABLE row |
| `docs/0.8.3 Simthing Studio Production.md` | SCENARIO-METADATA-LOSSLESS-0 section |

## Deleted/archived artifacts

None. No scratch logs, temp fixtures, or superseded reports removed.

## Deferred next rung

**SCENARIO-GAMESESSION-CHILD-0** — require exactly one GameSession child under Scenario. Must not build on lossy seed metadata; this PR clears that blocker.

## DA status

**PROBATION** — pending owner/DA approval. SCENARIO-SERIALIZABLE-SIMTHING-ROOT-0 (PR #777) remains PROBATION but is now qualified by this precision fix.