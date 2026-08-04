# EML-EXP-PRIMITIVE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.11)
- Status: **PROBATION / proof-present / DA-review-pending**
- Dispatch base: `7ebf8884679840b3c2670ad0a4e7a55304bb3e28` (handoff merge #1631)
- Branch: `fable/eml-exp-primitive-0`
- ORIENT-RECEIPT: `d950cd858719` (orientation_rule_stamp `2d131557973b6050`)
- HD-RECEIPT: `eba8e17a4526`
- Dispatch: Board comment `5183411832`
- Expected route: `DA-RESERVE(gate-wiring)`
- Scope: **5.11 only** — no LN/POW/eml()/LogAccumulate, no StemThing-A, no movement/contention, no 5.12 work

## ANCHOR-ACK

`admission-ladder-necessity-test, core-gpu-residency, core-overlays, core-property-value-model,
core-rf-arenas, eml-admission-shapes, eml-extension-ladder, eml-integration-plan,
eml-triad-integration, exact-numeric-candidate-f, field-policy-time-decisions,
field-sweep-preservation, founding-ontology-invariants, orientation-harness-core,
rf-arena-substrate, scanner-selftest-delta-gate, seal-residue-cross-crate,
simthing-0087-binding-laws, simthing-0087-pillars, stead-events-are-rf, stead-rejected-shapes,
stead-shared-surface-ledger, stead-spatial-contract-core, stemthing-binding-laws,
stemthing-lane-not-leg, stemthing-slot-identity-ruling, structural-execution-convergence,
workshop-candidate-homing` — ACK; ingested via the coding projection before edits.

## The admitted primitive

Full-domain `EXP` over `[-87.33, +88.72]` (endpoint bits `0xC2AEA8F6` / `0x42B170A4`,
finite-only policy) — the FIRST admitted exact primitive through the landed 5.10
`ExactPrimitiveAdmissionDoor`, ending the transcendental deferral C-8 left conditional.
`CLOSED_OPCODES` widened by exactly one opcode (`EXP = 26`; roster 23 → 24). Every output over
the admitted domain is a positive normal finite f32 (no subnormal, no overflow reachable — the
endpoints are chosen for exactly that, and the sequence is flush-to-zero-robust: subnormal
inputs collapse to `1.0` under both FTZ and gradual underflow).

### Pinned algorithm-as-spec (the sequence IS the bit law)

`crates/simthing-core/src/eml_exp.rs` (CPU twin, canonical) — one product, one
round-ties-even, eight **explicit fused multiply-adds** (`f32::mul_add` / WGSL `fma`, both
single-rounding IEEE), one add, exact split power-of-two scale via exponent-field assembly.
No `div`, no f64, no vendor transcendental, no implementation-defined op anywhere.
Algorithm identity (FNV-1a-64 over version + order tag + every constant's exact bits):
**`0x29765ea9251c2ae1`** — pinned in `crates/simthing-kernel/src/eml_exp_qualification.rs`;
semantics are append-only (any change is a NEW primitive name and REDs the freshness tripwire).

**Why fused/intrinsic ops rather than separate mul+add (measured, DA attention):** the first
candidate sequence used the classic magic-shifter RNE round and separate mul/add steps. On the
certified toolchain the shader compiler **algebraically eliminated `(a + 1.5·2^23) − 1.5·2^23`
and contracted mul+add chains — including ACROSS `bitcast` round-trip fences** (micro-diagnosed
step-by-step: `kf` collapsed to `a`, k off by one at the domain floor, probe referee RED at
`0xC2AEA8F6`). Exactly the §6 hazard: fences are not a reliable barrier. The remedy pins the
semantics the hardware actually executes — `round()` (RNE intrinsic) and explicit `fma()` —
which are fully-IEEE-specified single-rounding operations available identically on CPU
(`round_ties_even`/`mul_add`). Zero fences retained (measured unnecessary under v2; the doc's
"ship the faster naked arithmetic" ruling). The 2^32 digest referees the whole question.

### Execution arms (one definition per surface, no bespoke shader)

- Interpreted WGSL arm: `fn eml_exp_pinned` in `field_sweep.wgsl` (placed OUTSIDE the JIT
  excision markers) + byte-identical copy in `accumulator_op.wgsl`; `OP_EXP`/`EML_OP_EXP` cases.
- SSA-JIT arm: `emit_program` lowers `EXP` as a call to the SAME surviving helper — one pinned
  definition serves both arms in the field pipeline (referee:
  `eml_exp_primitive_0_jit_lowering_calls_the_single_pinned_helper`).
- Copy-drift referee: `eml_exp_primitive_0_wgsl_helper_copies_and_pinned_constants_agree`
  (byte-equality across shader homes + constants vs the Rust twin's exact bits + op census).
- CPU twins: `eval_field_eml_cpu`, `accumulator_op::eval_eml_cpu`, `eval_overlay_eml`,
  spec `eval_eml_postfix`, `intensity_eml` inline — all call `eml_exp_pinned_f32`.

### Call-site law (5.10 shapes wired to production admission)

`admit_exp_call_sites` (kernel door) runs at field-program validation and GPU program-table
upload; `validate_exp_call_sites` mirrors it in the core registry (the `validate_div_node`
precedent). Admissible: shape 2 — immediately-preceding `CLAMP_BOUNDED` whose authored bounds
lie inside the domain (the guard IS the authored saturated semantics); shape 1 —
immediately-preceding in-domain `LITERAL_F32` (`ExactPrimitiveRangeEvidence::LiteralInRange`,
the §4 "literal in range" certificate). Anything else is the spanned
`UnguardedExactPrimitiveCallSite` error (span = node indices). The naive unstabilized
softmax form `EXP(β·zᵢ)` is mechanically rejected by this gate.

## Exhaustive 2^32 qualification (LOCAL phase-boundary act — never CI)

Exact command:

```text
cargo test -p simthing-workshop --release --test eml_exp_primitive_0_qualification -- --ignored --nocapture --test-threads 1
```

Enumeration law: ascending u32 bit order over the admitted domain — positive bits
`0x00000000..=0x42B170A4`, then negative bits `0x80000000..=0xC2AEA8F6`. Digest: FNV-1a-64 over
each output's little-endian bit bytes in that order.

```text
EML_EXP_QUALIFY arm=cpu-reference tested=2237667740 digest=0x7875a45ba919d588 algorithm=0x29765ea9251c2ae1
EML_EXP_QUALIFY arm=interpreted   tested=2237667740 digest=0x7875a45ba919d588
EML_EXP_QUALIFY arm=jit           tested=2237667740 digest=0x7875a45ba919d588
```

**Every admitted-domain input, all three arms bit-identical** (2,237,667,740 values; per-chunk
first-divergence assertion never fired). Wall clock ≈ 230 s total.

### Certified toolchain roster (trust chain pinned)

| Adapter | Backend | Driver | Compiler chain | interpreted digest | jit digest | Qualified |
|---|---|---|---|---|---|---|
| NVIDIA GeForce RTX 4080 Laptop GPU | Vulkan | NVIDIA 595.79 | rustc 1.95.0 + wgpu 22.1.0 / naga 22.1.0 | `0x7875a45ba919d588` | `0x7875a45ba919d588` | 2026-08-04 |

The host also carries an Intel UHD iGPU; it is NOT a certified tuple — `GpuContext`
deterministically prefers the discrete adapter (`context.rs` enumerate-first-discrete), so no
production dispatch reaches it. Certifying it (or any new tuple) is one local sweep + one
roster row.

### Freshness wiring (presence/freshness only; drift invalidates)

- Compile/test-time tripwire: `eml_exp_primitive_0_qualified_artifacts_bind_to_the_live_algorithm_identity`
  (live algorithm identity ↔ pinned identity; opcode value; replay digests ↔ reference).
- Static CI watcher: `scripts/ci/eml_exp_qualification_check.sh --check` — pinned SHA-256 over
  the CPU-twin sequence region and the WGSL helper block (copies must also be byte-equal),
  digest/domain presence in the qualification module and this doc, recorded wgpu version vs
  `Cargo.lock`. `--selftest` plants a one-ULP constant drift in the twin and in one WGSL home;
  both must FAIL. **The 2^32 sweep is never re-executed by CI.**
- Residual (named, per stop-condition honesty): the *driver* link of the chain cannot be
  statically watched from CI (no GPU); it is pinned in the roster and re-verified by every
  local GPU referee/qualification run. This is the certified-substrate law working as designed,
  not a silent gap.

## Independent numerical characterization (approximation quality, NOT a rounding claim)

Against the host f64 `exp` as higher-precision reference
(`eml_exp_primitive_0_numerical_characterization`; boundaries ±4096 ULP neighborhoods,
2×2^22 stratified, 2^22 LCG-random, monotone spot sweeps):

```text
EML_EXP_CHARACTERIZE checked=12591106 max_ulp=1 at_bits=0xc2aea812 max_rel=8.243e-8 nonfinite=0 nonpositive=0 monotonicity_violations=0
```

Observed envelope: **max 1 ULP**, max relative error 8.243e-8, positive-finite everywhere,
zero monotonicity violations in dense adjacent-input sweeps. The pinned sequence remains the
bit law; correct rounding is neither claimed nor required (a consumer needing it routes to a
future `EXP_CR` name).

## Cost gate (unweakened key, real gadget baseline)

`eml_exp_primitive_0_cost_gate_beats_the_pinned_gadget_baseline` — driver-originated
`VK_KHR_pipeline_executable_properties` statistics on the certified adapter. Baseline: the
pinned 21-node ordinary `ADD`/`MUL` Horner tree (`LegacyFixed32`) on the canonical interpreter.
Candidate: the guarded `EXP` program JIT-lowered at `CompactStack4`. Raw driver values verbatim:

| Pipeline | Registers | Binary bytes | Raw NVIDIA "Local Memory Size" |
|---|---|---|---|
| canonical interpreter (Legacy32, gadget baseline) | 28 | 16,000 | 68,719,476,864 |
| SSA-JIT EXP block (Compact4) | 17 | 2,560 | 68,719,476,736 |

No regression + strict improvement on all three dimensions (the 128-byte delta in the raw
local-memory stat is the interpreter's 32-slot stack array; the 2^36 base is the same NVIDIA
reporting quirk the 5.7 census recorded verbatim). `verify_cost` mints the key; no new cost
dimension, no baseline substitution.

## Consumers (authored EML data through existing doors)

- **Sign-stable Logistic CostBand steering** — `logistic_steering_eml_nodes` (simthing-core,
  beside the staircase builder): 31 nodes, `SELECT`-on-sign over
  `EXP(CLAMP(-|k·N - k·x0| - …))`-folded halves, argument ≤ 0 by construction, clamp-guarded
  (shape 2). Bit-exact builder/interpreter/oracle parity across the N sweep; monotone; bounded
  `[lo, hi]`; C1 seam at `x0`. Consumer necessity MEASURED:
  `EML_EXP_CONSUMER steering staircase worst_abs=1.112682 excess_bps=2967` — the admission
  ritual's `measured_threshold_excess_bps`.
- **Exponential STEAD falloff law** — `compile_stead_exponential_falloff_field_sweep`
  (driver): `Σ u_j · EXP(-λ·d_j)` with distance on the existing `EDGE_SCALAR` seam, replacing
  per-hop-band weight tables; one ordinary registration, `CompactStack4`. Interpreted arm
  bit-exact vs CPU twin; per-edge weight equals the pinned exponential exactly.
- **Stabilized SoftmaxWeight** — `SoftmaxWeightGadget` (kernel, beside
  `SoftStepPolicyConditional`): `EXP(β·(zᵢ - max z))` with `max z` from the existing `MAX`
  reduction band and normalization on the existing `Sum` band; 8 nodes, `CompactStack4`;
  oracle parity referee. The naive `EXP(β·zᵢ)` form is not the canonical softmax and cannot
  pass call-site admission (mechanized rejection, not prose).

## Mutation referees (planted defects RED)

| Planted defect | Referee | Result |
|---|---|---|
| split-ln2 reduction fused to single constant (compiler constant-folding shape) | `…planted_ln2_fusion_mutant_reds_the_digest` | digest ≠ reference — RED |
| scale reassociation `y·(s1·s2)` (overflows 2^128 at the domain ceiling) | `…planted_scale_reassociation_mutant_reds_the_digest` | non-finite at ceiling + digest ≠ reference — RED |
| unguarded-call-site admission bypass | `…call_sites_admit_only_the_two_shapes_and_span_the_rest` | forged Ok ≠ gate verdict — RED |
| one-ULP pinned-constant drift (twin and WGSL, separately) | `eml_exp_qualification_check.sh --selftest` | FAIL both — RED |
| algorithm-identity insensitivity | `…algorithm_identity_moves_with_any_pinned_constant` | identity moves — RED |

## Known gaps / next

- **Generic JIT fold-seam FMA contraction (pre-existing, EXP-free, triaged):** on the
  certified toolchain the SSA-JIT contracts `accumulator + (u·m)` across the generated
  map/fold seam for MUL-ending maps, drifting ≤1 ULP vs the CPU twin. Witness with ZERO `EXP`
  in the program: `eml_exp_primitive_0_jit_fold_seam_witness_is_exp_free` (3/32 cells,
  `NEIGHBOR·0.33333334` map + Sum fold). The landed three-way census never sees it because its
  authored values are dyadic (every product exact). The falloff consumer's JIT arm is bounded
  ≤1 ULP in-test with commentary; the primitive itself carries zero drift on the same arm
  (2^32 proof). Routed to triage as a generic-lowering finding with candidate remedies (fold
  seam specified as fused in twin+JIT, matching this rung's philosophy; or NoContraction
  emission). Not a 5.11 surface: fixing it moves the generic fold semantics for every program.
- 5.12 `EML-LN-PRIMITIVE-0` inherits the v2 template: intrinsic/fused pinned ops, exhaustive
  digest, cost-vs-own-gadget, guarded call sites, certified-roster freshness.

## Verification

```text
cargo check -p simthing-core -p simthing-kernel -p simthing-spec -p simthing-driver -p simthing-workshop: PASS
cargo test -p simthing-core --lib eml_exp: 4 passed (constants bits, endpoints, identity tripwire)
cargo test -p simthing-core --lib eml_registry: 1 passed (registry call-site mirror, spanned naive rejection)
cargo test -p simthing-core --test eml_exp_consumer_0: 2 passed (logistic parity + measured staircase excess)
cargo test -p simthing-kernel --lib eml_exp: 7 passed (vocabulary census, call-site shapes,
  softmax gadget parity, JIT single-helper lowering, WGSL copy referee, artifact freshness
  tripwire, one-EXP admission ritual)
cargo test -p simthing-workshop --test eml_exp_primitive_0_qualification: 4 passed
  (three-way probe parity, STEAD falloff three-way, 2 digest mutants); +5 ignored local acts
  all run and PASS (2^32 CPU/interpreted/JIT with pinned-digest asserts, characterization,
  seam witness)
cargo test -p simthing-workshop --test eml_exp_primitive_0_cost_evidence -- --ignored: 1 passed
cargo test -p simthing-workshop --test eml_resource_class_jit_parity_0: 1 passed (inherited census)
bash scripts/ci/eml_exp_qualification_check.sh --check: PASS
bash scripts/ci/eml_exp_qualification_check.sh --selftest: PASS (both planted drifts bite)
bash scripts/ci/agent_scan.sh: see relay
bash scripts/ci/test_inventory_drift_check.sh: see relay
```

## Posture

No clearance, merge, pointer movement, 5.12/LN work, StemThing-A work, or successor-rung stub
is claimed. `EXP` semantics are append-only from this landing; the door instance admitting it
is exercised in tests — production vocabulary remains the closed set plus the one admitted
opcode, and every production call site carries a 5.10 shape.
