# Native recipe-input locus law + host-qualified consumed-cell contention

DA increment per Orchestration relay `5730468245` (Astra return `5725853236` accepted as a truthful
substrate STOP on 2.2 `0088-ECONOMY-FLEET-0`; #2075 retained RED). Model 1 remains UNDECIDED;
Model 2 remains CLOSED. No fleet semantics. The first consumer remains #2075. **No sealed byte
moves.** `hydrate_field_economy.rs`, `hydrate_scenario.rs`, `rehearsal_ingress_fields.rs` and
`compile/resource_economy.rs` are not in `build.rs::COMPONENTS`, and `Cargo.lock` is untouched. The
pin stays at the fifteenth (`0xa3e6_82eb_e0de_36f6`, #2076).

## A. Native authoring fidelity (rules 1–3, 5–8)

- **Collection, not scalar.** Repeated `input` fields in `production_building` accumulate into the
  complete conjunction and lower to the already-generic `Vec<RecipeInputSpec>`. There is no
  last-wins, no first-wins, and no reduction to one cost.
- **Two locus forms, exactly one per cost:**
  - `{ resource = minerals amount = 2 }` is the historical location-local material shorthand. It
    lowers byte-identically to before, and only this form registers a derived quantity property.
  - `{ entity = terran property = "meridian::energy" role = balance amount = 1 }` is a canonical
    binding to an EXISTING property/role/host. It uses the native property-declaration role
    vocabulary (the shared parser, now crate-visible, not a copy). It never invents a property: an
    unknown property, role or host refuses at ordinary admission, before activation.
- **Order carries no economics.** The lowered conjunction is canonically ordered by locus. A
  cost's source span is provenance and is kept for refusal attribution only.
- **Duplicate-field law.** A repeated authored field is either a declared collection or a typed,
  spanned error, never a silent last-wins. Every production-building and input scalar refuses a
  repeat. Two costs lowering to the same `(property, role, host)` locus are refused rather than
  merged; this includes a canonical cost that aliases a shorthand's own derived locus.

## B. Consumed-cell contention keyed by host (found by the ordinary proof)

The spec-level contention pre-check keyed consumed cells by `(band, property, column)` with no
host. It predated host-qualified economy rows. Terran's and Pirate's refineries (both in band 0)
each spending their OWN `meridian::energy` balance were therefore refused as one contended cell
(mutant below). #2075 would have hit this next.

**Repair:** the key is the authored cell `(band, property, column, host)`, where `None` means the
property's unique live host. The kernel planner's resolved `(band, slot, column)` check remains the
final word, so aliasing hosts still refuse there. The same host still contends.

## Proof floor executed (reference machine, stacked on #2076's fifteenth pin)

- **Hydration battery 4/4:**
  - minerals+energy and energy+minerals lower to the same conjunction;
  - a 3-cost building is preserved in all six orders;
  - the single-cost building lowers exactly as before;
  - 7 malformed or repeated-field shapes refuse with spans;
  - 3 duplicate-locus shapes refuse with spans.
- **Contention unit witness:** distinct hosts compile; the same host still refuses.
- **Ordinary two-faction session** (shipped `stellaristhing_base` bundle re-authored in a temp
  copy; shipped bytes untouched; native parse → expand → hydrate → profile → `SimSession`; canonical
  cache re-loaded):
  - Both refineries lower to `{2 × A1/E1 minerals (local), 1 × meridian::energy balance at the
    owner}`. There is no fabricated energy ledger.
  - The cache rebind carries the same conjunction, and trajectories are bit-identical in both
    authored orders.
  - Alloys produced, [withheld 3 gens, restored 4 gens] × [terran, pirate] = `[[0, 4], [2, 4]]`:
    - Terran refines nothing while its energy is withheld, even with minerals available.
    - Pirate refines in the same band from its own energy and stops when its 8 energy is spent.
    - Terran resumes once its income is restored through the ordinary overlay door (owner-own
      intrinsic, which #2076's ownership law settles).
  - Every generation, for both factions: Δminerals = accrual − 2·alloys and
    Δenergy = settled − alloys, exactly. Energy is never overdrawn.
- **Pre-activation refusals (ordinary path):**
  - unknown property → `AdmissionRefused { law_id: "resource-economy-property-registered" }`;
  - unknown role → `InvalidResourceEconomyRole { role: "Named(reserve)" }`;
  - unknown host → `NeedBindingInvalid { "economy host entity `nobody` is not in install_targets",
    span_token: Some(423) }`, which points at the authored cost.
- **Mutants:**
  - host-blind contention key → the ordinary session refuses the two-faction conjunction
    (`ResourceEconomyConsumedInputContention`, terran vs pirate) → RED;
  - last-wins inputs (the original defect) → hydration 2/4 RED, ordinary witness RED.
- Existing law GREEN: `field_economy_grammar_0` 4/4; the full clausething, spec and mapeditor
  suites; 2.1 suites (via #2076).

## Parallel finding (named, not landed): crate-wide last-wins census

The duplicate-field law is enacted here for `production_building` and its inputs only. The census
pattern `=> field = Some(…)` still finds 185 single-line last-wins match arms (188 before this
change), and multi-line arms add more:

| hydrator | arms |
|---|---|
| `hydrate_category_economy` | 46 |
| `hydrate_scenario` | 35 |
| `hydrate_field_economy` (outside this block) | 35 |
| `hydrate_field_operator` | 20 |
| `hydrate` | 19 |
| `hydrate_shipsize_decoder` (sealed) | 11 |
| `hydrate_scenario_commitment` | 8 |
| `hydrate_resource_flow` | 7 |
| `hydrate_palma_feedstock` | 3 |
| `rehearsal_ingress_fields` | 1 |

Before refusal can be enabled crate-wide, shipped assets need a census for repeated scalars.
Routed as a Clause-B census item.
