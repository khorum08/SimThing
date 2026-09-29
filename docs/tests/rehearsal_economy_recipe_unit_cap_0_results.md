# Authoritative per-generation recipe-unit cap + SIXTEENTH E8 roll

**Status: the LAW stands; its MECHANISM was re-expressed as a budget and the kernel field removed
(DA intervention 3(a) of the 0.0.8.8 drift audit, live Board `5879126789`; see the last section).**

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

## Re-expressed as a budget: TWENTY-FIRST E8 roll

**Why.** The drift audit judged the kernel field against the 0.0.8.7 foundation, and the field
fails it. "A cap is a budget: exceeding it does not fail a check — the draw floors to zero because
`V` is exhausted" (StemThing law 3), and "more behavior means more admitted data" (the SimThing
Principle). The containment rebuttal above answered whether the kernel could hold the cap without
new domain vocabulary. It never asked whether the cap belonged in the kernel at all. The `TODO(E-3R+)`
contract it cited is met by data: a budget is GPU-resident, and it bounds the credit and every debit
through the one count.

**The mechanism now.** ClauseThing lowers `production_building.max_units_per_generation = N` into
three pieces of ordinary data at the building's location:
- a capacity quantity, `{location}_{building}_capacity_units`, whose Amount opens at `N`;
- a standing `Set(N)` policy that refills it;
- one more recipe input, spending that capacity at unit cost 1.

Each tick the recipe executes `floor(min(input_i / unit_cost_i))` over its inputs, the capacity among
them, and the overlay pass then restores `N`. The count floors once the budget is spent. A refill is
a Set, never an Add, so unused budget is never banked.

The spec, the driver, core and the kernel carry no cap at all:
- `CombineFn::MinAcrossInputs` is a unit variant again;
- `encode.rs`, the WGSL shader, the kernel CPU oracle and `transfer_accumulator.rs` are
  byte-identical to their pre-#2078 bytes;
- the fixed-transfer refusal is gone with the field it guarded.

**The law's rules, held by arithmetic:**
- The one count debits every input, the budget included, and credits the target (rules 1, 2, 5).
- A Set refill gives one generation's budget and never a carry (rule 3).
- Legacy recipes gain no input (rule 4).
- Nothing is host-side (rule 6).
- Native zero, negative, fractional or repeated authority still refuses with a span (rule 8).
- A recipe interchange carrying the retired `max_units_per_generation` field refuses as unknown.

**Witnesses:**
- `recipe_unit_cap_0` (rewritten): the lowered budget for caps 1 and 2, the hint not promoted, no
  budget on uncapped recipes, and a whole-spec interchange round trip.
- `native_recipe_inputs_session_0` (unchanged): cap 1 gives `[1, 1]` and cap 2 gives `[2, 2]` in
  each of 3 generations, with the exact per-generation accounting.
- `native_structural_products_session_0` (unchanged): 2.2's funded births under cap 1.
- The kernel-field matrix test `c8c_conjunctive_recipe_unit_cap_is_atomic_and_per_generation` is
  deleted under `authorized_deletions.tsv` (`5879126789-DA`), since the kernel no longer has the
  field.

**Evidence** (NVIDIA 616.92, `--features simthing-gpu/eml-resource-profiling`):

| step | result |
|---|---|
| old pin RED | the referee refused `0x2308_1112_d5c9_8f38`, observing `490e902413acc670` |
| roll | both literals `0x490e_9024_13ac_c670` |
| battery | parity 1/1 (`490e902413acc670`), score and bands 3/3, runtime 4/4 |
| cap witnesses | workshop 8/8: uncapped G1 `[10, 8]`, cap 1 `[1, 1]` ×3, cap 2 `[2, 2]` ×3, alloy `[[0, 4], [2, 4]]`, structural products 5/5; clausething 2/2; c8c 2/2 |
| full suites | core 32, kernel 55, spec 16, sim 25, gpu 7, driver 58 (1 ignored), workshop 16, clausething 47 (2 ignored), mapeditor 26 (run alone: after clausething in one batch it hits a local cache artifact before any test runs); 0 failed |
| clean checkout | PASS: a fresh GitHub clone at the roll commit `eb0077e6` (autocrlf `false`, 0 dirty paths, 0 CRLF files) reproduces `490e902413acc670` |

Pin chain: `…0xef92_b0f2_866c_ef5d` → `0x2308_1112_d5c9_8f38` (#2098) → `0x490e_9024_13ac_c670`.
