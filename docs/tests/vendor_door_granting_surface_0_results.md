# VENDOR-DOOR-GRANTING-SURFACE-0 results

- Track: 0.0.8.7 RF arena modernization, rung 11.2e
- Status: **COMPLETE — DA-GRADUATED / merged #1847 @ `3c623991`** (Fable deep review, graduation ruling on Board #1332; StemThing-B predecessor arc complete)
- Exact implementation base: `e6cb3be1adbbe5b833f856eecf928d0c16c748ee`
- Branch: `codex/vendor-door-granting-surface-0`
- tested_code_sha: `1df854ef921421651982e22b3ef7c3e44b3910ca`
- Handoff: Board #1332 comment `5432172782`
- Governing DA exit/unblock: Board #1332 comment `5432021207`
- HD-RECEIPT: `732b0263cdd0`
- ORIENT-RECEIPT: `1a6a00162374`
- orientation_rule_stamp: `9ee3f7649d1fc790`
- orientation_digest_sha: `cef3c3639369fcc18a75ce08f4f602d8240b1bc090af09ae23b73c0c2cb00c39`
- Expected route: `DA-RESERVE(gate-wiring)`
- Structural-certificate baseline inherited for DA: 120 suites / 460 passed / 3 failed / 14 ignored

## Pre-edit delegation map and disposition

The facade-only map closed without a STOP. Populate can directly re-export the
graduated declaration-shaped `ConstrainedSupply`, owner/resource/scope keys,
runtime demand bucket, generation stamp, and 11.2b `ResidencyExtent`. Derive can
directly re-export the existing specialization flow-market offering, price,
sealed Draw, admission, weight, grant, and provenance vocabulary. Run can
directly re-export 11.2a constrained clearing and delegate exactly to the
existing 11.2c session install and 11.2b session placement methods.

No engine capability was missing. Bind and Overlay retain their meanings, all
dependency arrows remain embedder -> engine, and no engine crate depends on
`simthing-embedder`.

## Product

The Vendor Door remains exactly `bind`, `derive`, `overlay`, `populate`, and
`run`. Populate now exposes arbitrary conserved granter budgets and the already
accepted residency extent vocabulary. Derive exposes the graduated strict
offering and sealed Draw declarations. Run exposes the graduated constrained
clear and two policy-free session delegations:

- `install_growth_entitlement_market` terminates at
  `SimSession::install_growth_entitlement_market`;
- `realize_market_grant_residency` terminates at
  `SimSession::realize_market_grant_residency`.

The facade adds no market object, allocator, clearing loop, placement oracle,
provenance type, retry/convergence loop, generation authority, observation,
history, telemetry, or replay surface. Production changes are confined to the
leaf `simthing-embedder` facade; engine source changes are zero.

## Load-bearing proofs

| Proof | Result |
|---|---|
| Generic non-residency | PASS — a vendor-authored `vendor-compute-cycles` lane admits through Derive, clears through Run, quantizes through the existing scalar CostBand, and its accepted quantity becomes the intrinsic-flow input to the standing RF + PALMA + Gu-Yang session witness. Bind observes only admitted live Triad columns and Run writes the existing canonical replay. |
| Strict references / sealed Draw | PASS — an unknown offering reference fails existing market admission; offering construction has compile-fail REDs for both `per_type_delta` and `shadow_tier`. |
| Recursive child granter | PASS — root budget 8 grants 6 to a child, then the accepted sealed quantity is the child's only descendant budget; the same Draw/claim/clear/record grammar grants 4 and preserves `6 = 4 + 2`. No recursive helper API or manager exists. |
| Five verbs / leaf / zero state | PASS — the standing facade-shape witness requires the exact five public modules, scans zero owned-state shapes, verifies no reverse engine dependency, and checks both new Run wrappers terminate at existing session methods. |
| Sixth-verb/API RED | COMPILE-FAIL — `simthing_embedder::grant` is unresolved (`E0432`). The witness continues to contain zero direct engine-crate references. |
| Residency coexistence | PASS — `ResidencyExtent` is authored through Populate and the Run residency function has the exact admitted-market + sealed-grant + caller-extent session signature. There is no allocation verb or facade placement policy. |
| Authority census | PASS — zero new state-bearing facade structs, managers, registries, allocators, clearing/provenance/retry/history/telemetry/observation authorities; zero engine semantic/source edits. |

## Focused evidence

| Command | Result |
|---|---|
| `cargo check -p simthing-embedder` | PASS |
| `cargo test -p simthing-embedder -- --test-threads=1` | PASS — 13 integration tests plus 5 compile-fail doctests; the load-bearing Triad proof ran on the local NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan adapter. |
| `cargo test -p simthing-driver --test stemthing_b_flow_market_germ_0 --test stemthing_b_vram_residency_0 --test stemthing_b_growth_entitlement_seam_0 -- --test-threads=1` | PASS — 10/10 frozen 11.2a/b/c tests, including generic non-residency full grammar, provenance/refusal/revalue, ordinary growth, and replay. |
| `cargo test -p simthing-spec --test stemthing_b_allocator_retirement_0` | PASS — 2/2 11.2d retirement/order seals. |
| `cargo test -p simthing-driver --test determinism_matrix_0 -- --test-threads=1` | PASS — 2/2, including the planted order-authority defects. |
| `cargo fmt --all -- --check` / `git diff --check` | PASS |
| inventory check / drift check | PASS — exact 1,343/1,343; zero unledgered, stale, or parked rows. |
| lifecycle `--schema` / `--scheduled` | PASS — zero expired candidates. |
| anchor check / generated orientation check | PASS; required anchors acknowledged. |
| inventory drift / lifecycle prove selftests | PASS — all synthetic and live falsifier cases bit for their named reasons. |
| Agent/Doctrine scan at `1df854ef` | PASS — exact base-to-checkpoint delta; zero hard failures and zero inspect flags. |

Coverage basis: PASS — facade-only source diff, one non-residency integrated
market/RF/CostBand/Triad/replay witness, recursive-child conservation witness,
compile/API seals, all facade standing tests, and frozen 11.2a–d batteries.

## Fences retained

- No engine semantic/source change in core, spec, sim, driver, kernel, GPU, or WGSL.
- No sixth facade operation and no duplicate offering, clearing, placement, provenance, observation, or replay grammar.
- No per-type delta, shadow tier, free-list/order grant policy, direct allocation/write bypass, or same-generation retry.
- No 11.3 implementation, 11.4, 12.x, Vector CostBand, or ClauseThing-red work.
- Pointer remains 11.3. Coding does not clear, merge, graduate, or move it.
