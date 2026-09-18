# Authoritative per-generation recipe-unit cap + SIXTEENTH E8 roll

DA increment per Orchestration relay `5735839909` (Astra return `5735702463` accepted as a
truthful substrate STOP on 2.2 `0088-ECONOMY-FLEET-0`; #2075 retained RED at `3d00def2`). Model 1
remains UNDECIDED; Model 2 remains CLOSED; no fleet semantics. **DA disclosure:** my own #2076
probe showed a refinery executing two batches in one generation (the scenario comment calls
the throttle "a hint"). I did not flag it as 2.2's next STOP.

## Ruling: a kernel change, and why containment does not bar it

The cap lives in the kernel's generic conjunctive primitive. The kernel-containment law presumes
an app-motivated kernel change is wrong. The presumption is rebutted here:

- The core builder already carried a standing contract, `TODO(E-3R+)`, reserving exactly this
  shape: GPU-resident, affecting credit AND debit, conservation-exact, never `ScaleSpec` alone.
- No domain vocabulary enters the kernel. The change is one optional unit ceiling on an existing
  combine.
- No composition of admitted ops can express the cap without a synthetic permit ledger or source
  starvation. The relay correctly rules both out, because both change the economy.

## The law

- **Units, atomically:** `executed = min(floor(min_i(input_i / unit_cost_i)), cap)`. That ONE
  count debits every input by `executed × unit_cost_i` and credits the target by
  `executed × output_coefficient` (rules 1, 2, 5).
- **Per generation, no carry:** the op executes once per generation in its band, and unused
  capacity is never banked (rule 3).
- **Optional; legacy unchanged:** `CombineFn::MinAcrossInputs { max_units: Option<NonZeroU32> }`
  is encoded in `combine_a`, where 0 means uncapped. Every legacy conjunctive op therefore encodes
  byte-identically (rule 4).
- **A distinct authoritative field:** `max_units_per_generation`. `throttle_hint_max_per_tick`
  stays a hint and is not promoted. The cap is refused on a single-source fixed transfer
  (`max_transfer`, E-2A untouched), so the two laws never share a representation (rules 4, 9).
- **GPU-resident:** WGSL clamps the count in `gather_min_across_inputs`, and the kernel CPU oracle
  mirrors it. There is no host policy, ledger, token bucket, or permit resource (rule 6).
- **Order and contention unchanged:** no reordering and no second lane. #2077's host-qualified
  contention stands (rule 7).
- **Fails closed:** `NonZeroU32` by type. Native zero, negative, fractional or repeated authority
  refuses with a span. The canonical interchange refuses zero, negative and fractional values at
  deserialization (rule 8).

**Wiring:** `production_building.max_units_per_generation` → `ResourceRecipeSpec` →
`CompiledResourceRecipe` → `ConjunctiveRecipeRegistration` → `TransferRegistration` →
`MinAcrossInputs { max_units }` → `combine_a` → WGSL.

## Proof floor executed (reference machine)

- **Kernel matrix** (GPU == CPU oracle bit-exact in every case):
  - cap 1 on a recipe affording 10 units → exactly 1 unit, both inputs debited once;
  - cap 2 → exactly 2 units;
  - uncapped → all 10 units;
  - input order reversed → identical;
  - three inputs with cap 2 → 2 units;
  - affordable below the cap → the limiting input rules (3);
  - coefficient 1.5 → the credit is scaled, the debit is not;
  - two capped recipes in one band → each respects its own cap;
  - three generations → one unit per generation;
  - two starved generations followed by funding → 1 unit, not 3 (no carry);
  - a cap on a fixed transfer → `UnitCapOnFixedTransfer`.
- **Authoring 2/2:**
  - absent → `None`, and the hint stays 3;
  - authored → `Some(1)`, and the hint is still 3;
  - an uncapped recipe's canonical JSON is unchanged, and the interchange round-trips the cap;
  - native `0`, `-1`, `1.5` and a repeated cap refuse with spans;
  - interchange `0`, `-1` and `1.5` refuse.
- **Ordinary two-faction session** (shipped bundle re-authored in a temp copy, abundant minerals,
  cache rebind identical):
  - uncapped G1 = `[10, 8]` units, matching the relay's falsifier shape;
  - cap 1 → `[1, 1]` in each of 3 generations;
  - cap 2 → `[2, 2]` in each of 3 generations;
  - every generation, for both factions: Δminerals = accrual − 2·units and
    Δenergy = settled − units.
- **Mutants, all RED:**
  - the shader ignores the cap → it executes 10 units while the oracle executes 1;
  - the cap is applied to the credit only (the TODO's warned shape) → all inputs are debited but 1
    is credited;
  - materialization drops the authored cap → G1 executes 10 and 8.
- **Legacy unchanged:** the existing C-8c single-source and conjunctive tests are GREEN, as are the
  2.1 suites and the full clausething, spec and mapeditor suites.

**Informational:** the ordinary base-scenario session is RF-only. By its documented posture
("RF-only sessions need not author a residency-capacity market") it never admits resident
clearing, so it is not seal-gated. The E8 old-pin refusal is shown on the driver resident path
below.

## SIXTEENTH E8 roll

Sealed components moved: `accumulator_op.rs`, `accumulator_op_builder.rs`,
`accumulator_op/encode.rs`, `accumulator_op.wgsl`.

- **Old-pin refusal (RED, required first)** on the repaired source: driver resident ingress
  `UnqualifiedAdapter { required: 11810271022427289334, observed: 11066379952262892183 }`; parity
  referee FAILED with `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 9993adef34224a97`. The two
  derivations agree.
- **Roll:** both literals atomically `0xa3e6_82eb_e0de_36f6` → `0x9993_adef_3422_4a97`. No third
  site. No comparator, component-list, `build.rs`, or Cargo change.
- Pin chain: `…0xee0e…` → `0xa3e6…` → `0x9993_adef_3422_4a97`.
- **Battery at floor:** parity 1/1 + mutant matrix, score-and-bands 3/3, runtime 4/4.

## Clean-checkout proof

- commit: `964894c3` (the exact sixteenth-roll commit)
- command: fresh `git clone` with `core.autocrlf=false` (canonical LF checkout) at that exact commit,
  in sibling directory `simthing-e8-roll16-verify` (not under %TEMP%);
  `cargo test -p simthing-workshop --features simthing-gpu/eml-resource-profiling --test resident_clearing_parity_0 -- --nocapture --test-threads=1`
- observed: `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: 9993adef34224a97`; referee
  `1 passed; 0 failed`. The fresh canonical-LF clone reproduces the pinned fingerprint.
