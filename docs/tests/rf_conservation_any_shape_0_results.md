# rf_conservation_any_shape_0 — RF conservation for ANY tree shape

**Status: COMPLETE (DA intervention 1 of the 0.0.8.8 drift audit, live Board `5879126789`; rung
`0088-ECONOMY-FLEET-0`, which stays on HOLD).**

## The law it proves

The Invariant Set holds "for any input, so each is provable over inline-constructed input"
(0.0.8.7 binding laws). RF conservation had only been witnessed on depth-1 and depth-2 fixtures, and
two settlement defects hid beyond them:

- the historical interior path destroyed mass;
- #2065 created it below depth 1.

Both were repaired by #2076. Rung 1.2's graduation packet recorded the loss: a site's +2 per
generation surplus vanished while the owner balances stayed static.

## The witness

`crates/simthing-driver/tests/rf_conservation_any_shape_0.rs`, run through the ORDINARY resident
session (`SimSession::open_from_spec` + `step_once`) with derived RF participation:

- **`conservation_holds_for_every_generated_tree_shape`**. A deterministic splitmix64 generator
  builds 32 trees of depth 1..=5, one depth per seed modulo 5. The generator buys coverage, never
  validity. Each tree has:
  - intrinsic sources and sinks on the root, interiors and leaves (+2, +1, +0.5, 0, −0.5, −1);
  - zero and non-zero weights;
  - standing Set/Add/Multiply weight policies on interiors.

  Coverage: depth histogram [7, 7, 6, 6, 6], 80 interiors owning intrinsic flow, 77 sinks, 68
  interior weight policies, 3 generations each.
- **`conservation_holds_for_the_shipped_owner_economy_shape`**. This is the 1.2 shape: two owner
  interiors with ×1 and ×1.5 Multiply policies, one site each, and four cohorts per site (two +2
  generators, two −1 upkeeps). It runs with root flow 0 and with root flow 3.

Every generation is judged by the independent RF-1 structural check (`check_arena_structural`).
Every participant's Balance is governed, so Σ intrinsic flow = Σ ΔBalance within the oracle's
O(ε·n) bound. **Mutant:** the same snapshot with 0.25 of mass removed from one participant must fail
that check, in every generation of every shape, which proves the bound is not vacuous and the
observation is live.

## Result on master `9fdc4b22`

```
cargo test -p simthing-driver --features simthing-gpu/eml-resource-profiling --test rf_conservation_any_shape_0 -- --nocapture --test-threads=1
coverage: 32 trees, depth histogram [7, 7, 6, 6, 6], 80 interiors owning intrinsic flow, 77 sinks, 68 interior weight policies, 3 generations each
test conservation_holds_for_every_generated_tree_shape ... ok
test conservation_holds_for_the_shipped_owner_economy_shape ... ok
test result: ok. 2 passed; 0 failed
```

**GREEN: master conserves on every generated shape.** #2076's depth law holds beyond the D≤2
fixtures that could not falsify it.

**Sensitivity to the defects it targets (reasoned, not executed).** The E8 seal refuses any mutated
sealed settlement file before economics, by design, so the historical laws cannot be replayed through
this ordinary path. They violated conservation by whole units per generation:

- #2076 measured 8 energy settled against 4 produced at depth 3;
- 1.2 lost the full +2 surplus.

Both are orders of magnitude above the bound this witness enforces on depth 3–5 shapes with interior
and leaf flows.

## Lifecycle

Both tests are ledgered `invariant-required` / `KEEP` (never-pare) under `0088-ECONOMY-FLEET-0`.
The Invariant Set law is their consumer.
