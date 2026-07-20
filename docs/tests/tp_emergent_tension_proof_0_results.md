# TP-EMERGENT-TENSION-PROOF-0 — evidence

## Claim

Canonical multi-tick GPU runs on `scenarios/terran_pirate_galaxy.clause` through ordinary `open_from_spec + step_once` accrete production from an authored output coefficient, suppress local flow through an authored `flow_coupling` on distinct location hosts, and diverge sealed **construction** crossings (manufacturing need on produced hulls, `event_kind = 92`) when only owner policy-weight silo scalars change. Clause-source coefficient=`0` and coupling-omitted controls bite. Unrelated disruption threshold `event_kind = 71` is proven sealed while construction gauges stay zero. RF-5 expansion/minerals regressions remain green via source variants. Owner OVL is **pending**.

## Identity SHAs

| role | full SHA |
| --- | --- |
| Branch | `coder/tp-emergent-tension-proof-0` |
| PR | https://github.com/khorum08/SimThing/pull/1418 |
| Accepted first landing (governance only) | `cbf3b81f5659bbfdefafa1001276713178f54dab` |
| Remand 1 reviewed head | `38c0185da63faa90812bcd6b8a3b550728c1d3ba` |
| Remand 1 tested code | `25083f0d32209e00f65e140293e9972e34e1f207` |
| Remand 2 reviewed head | `4c4170f682ce0f15e9d057c4bb543149ce31372b` |
| Remand 2 load-bearing tested_code_sha (falsifier tip) | `7f6560e038123d74332fdd26f8c20d412cdebcc8` |
| evidence_sha | `deac6f51edd9a974f40b0c052d7ed1056b6de8bd` |
| final_head_sha | `59ae931f3afd1d673d72c1a99c4c4c0ee87b8c51` |

Load-bearing GPU falsifier tip remains `7f6560e038123d74332fdd26f8c20d412cdebcc8`. PR `tested_code_sha` / `current_pr_head_sha` equal the pushed tip for `body_sha: fresh`.

## Load-bearing proof

`cargo test -p simthing-workshop --test tp_field_session_elevate_0 -- --nocapture --test-threads=1`

Result at tested tip `7f6560e038123d74332fdd26f8c20d412cdebcc8`: **ok. 8 passed; 0 failed** (9.52s).

Representative paired print (local Windows GPU):

```
TP12_10_PAIRED ticks=8 path=simthing_driver::SimSession::open_from_spec + step_once
  productive={production:318 disruption:70 suppression:2 crossings:1}
  tension={production:280 disruption:32 suppression:40 crossings:0}
  divergence={production:38 suppression:38 crossings:1}
TP12_10_COUPLING_NEGATIVE … production=320 disruption=72 suppression=0
TP12_10_COEFFICIENT_NEGATIVE … production_open=0 production_after=0 crossings=0
TP12_10_GAUGE_FALSIFIER sealed_71=1 sealed_92=0 cum_decision=1 cum_construction=0
```

Distinct hosts proven per run: `shipyard != outpost != session_root`.

Predeclared minima: production divergence ≥ 6, suppression divergence ≥ 6, crossing divergence ≥ 1.

## Remand 2 corrections

1. **Unrelated-event gauge falsifier.** `studio_macro_gauges_use_exact_property_and_construction_event_kind` now runs a Clause-source case (`coefficient = 0.0`, disruption `amount = 1` below Rising threshold `3`) so sealed readback proves `event_kind = 71` fires while `event_kind = 92` cannot. Asserts `need_threshold_event_count == 0`, `cumulative_construction_crossings == 0`, nonzero global/other decision evidence, and exact hulls vs disrupted_hulls property keys. No manual event injection.
2. **Concrete evidence identity.** This file records full tested / evidence / final-head SHAs with no circular “see PR body” placeholder.
3. **Windows Studio + exact adapter.** Recorded below.
4. **Fresh clearance.** PR body `tested_code_sha` equals the final metadata head so `/clearance` emits `body_sha: fresh`.

## Remand 1 corrections (retained)

1. Construction need = `terran_manufacturing_need` on `terran_shipyard_hulls_quantity` / `event_kind = 92` (threshold 5.0). RF-5 minerals/expansion (`event_kind = 91`) kept via Clause-source variants.
2. Negatives are Clause-source only (`coefficient = 0.0`; omit `flow_coupling`). Coupling optional when production exists.
3. Studio gauges use exact property keys; cumulative construction crossings are event_kind-filtered.
4. `compose_recursive_rf_profile` grafts authored Location hosts under GameSession.

## Surfaces

- Generic coefficient + optional `flow_coupling` authoring/hydration
- Spec/kernel admit `output_coefficient` / `output_scale` ≥ 0 (0 = neutralization)
- Canonical clause + Unbounded Amount layout
- Studio_ops Telemetry exact-identity macro gauges
- Location-host graft in recursive RF compose

## Coverage battery (local Windows, owner GPU)

```
cargo test -p simthing-workshop --test tp_field_session_elevate_0 -- --nocapture --test-threads=1
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.52s

cargo test -p simthing-mapeditor --test studio_field_session_elevate_0 -- --nocapture
test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.27s

cargo test -p simthing-clausething --test field_economy_grammar_0 -- --nocapture
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

cargo build -p simthing-mapeditor --bin simthing-studio
Finished `dev` profile [optimized + debuginfo] target(s) in 0.48s

cargo test -p simthing-mapeditor --lib studio_gpu_adapter_policy::tests::exact_adapter_policy_accepts_non_dx12_backends_and_rejects_identity_or_dx12 -- --nocapture
test studio_gpu_adapter_policy::tests::exact_adapter_policy_accepts_non_dx12_backends_and_rejects_identity_or_dx12 ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s

bash scripts/ci/agent_scan.sh
DOCTRINE-SCAN-VERDICT: PASS  failures=0 inspect=0 selftest=SKIPPED
AGENT-SCAN-VERDICT: PASS delta_inspect=0 elapsed=47s
```

GPU/desktop proofs are owner-local by design; GitHub Actions has no GPU or desktop context.
Exact NVIDIA GeForce RTX 4080 Laptop GPU identity policy and owner-authorized non-DX12 backend support (DX12 blocked; no fallback adapter path) remain green.

## Regressions

- RF-5 high/low need-event pair (minerals source variant): green
- RF-4 `15/10` marginal `5` + governed-Balance disconnect: green
- `field_economy_grammar_0` (10): green
- agent_scan / inventory drift: green

## Owner OVL

**Pending** — paired Studio_ops Telemetry screenshots under the two policy-weight authorings remain Owner-closed after orch code acceptance. Do not self-merge or mark ready.
