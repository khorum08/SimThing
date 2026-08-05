# EML-LN-PRIMITIVE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.12)
- Status: **STOP** — certified toolchain cannot match pinned LN CPU semantics bit-for-bit under exhaustive admitted-domain replay (handoff `stop_conditions[0]`)
- Branch: `grok/eml-ln-primitive-0`
- ORIENT-RECEIPT: `d950cd858719`
- HD-RECEIPT: `a28f739a7fdb`
- Exact implementation base: `822de227c5666a15a5f7a0d6f80273c90ef2ec9c`
- Expected route: `DA-RESERVE(gate-wiring)` — coding does not `/clearance`, merge, or flip pointers

## ANCHOR-ACK

`admission-ladder-necessity-test@4bedf826f6f7, core-gpu-residency@8db4198cbc29,
core-overlays@54df7604a49d, core-property-value-model@17cd41a567b7,
core-rf-arenas@d171614211e9, eml-admission-shapes@d3c9259c4244,
eml-extension-ladder@7755bc72ffbe, eml-integration-plan@8eba54b02320,
eml-triad-integration@dada7d680557, exact-numeric-candidate-f@6938a2efadb5,
field-policy-time-decisions@993c7d0560e8, field-sweep-preservation@acc521a5a361,
founding-ontology-invariants@46802793fba7, orientation-harness-core@8a365d1c0864,
rf-arena-substrate@17b5f1e5c2ba, scanner-selftest-delta-gate@34fb2662baae,
seal-residue-cross-crate@49ee7c4ba6f4, simthing-0087-binding-laws@a59203b79425,
simthing-0087-pillars@61487cba1f9e, stead-events-are-rf@525388344ef2,
stead-rejected-shapes@3752549ff106, stead-shared-surface-ledger@87eaa1e7bb9c,
stead-spatial-contract-core@8585db4ac631, stemthing-binding-laws@6787a118c3ca,
stemthing-lane-not-leg@a9e9caa27a0f, stemthing-slot-identity-ruling@02c87b9126e1,
structural-execution-convergence@6b4cedec482b, workshop-candidate-homing@3e584f0ad175` — ACK.

## STOP — exact gap

**Certified tuple:** NVIDIA GeForce RTX 4080 Laptop GPU / Vulkan / NVIDIA 595.79 /
`rustc 1.95.0` + `wgpu 22.1.0` / `naga 22.1.0` (Cargo.lock).

**Constraint set that remains binding:** f32-only; no `div`; no f64; no vendor
`log`; identical CPU / interpreted-WGSL / SSA-JIT sequence; full positive-normal
domain `0x00800000..=0x7F7FFFFF` (2_130_706_432 patterns); do not drop the
toolchain or weaken the bit-exact referee.

**Best-measured candidate retained in-tree (not qualified):** tag **LN1C**
(`0x4C4E3143`), algorithm identity **`0x108443cfaeeaadfe`** —
classic two-iteration Newton reciprocal + fdlibm `Lg*` `ln1p` (independent of
`k`) + `fma(k, LN2, ln1p)` with `LN2` bits `0x3F317218`.

| Measurement | Result |
|---|---|
| Three-way probe (~6414 values, incl. prior magnets) | **bit-exact** (jit + interpreted) |
| Characterization vs f64 `ln` | **max_ulp=1**, max_rel≈8.3e-8, nonfinite=0, monotone spot checks green |
| Exhaustive CPU digest | computable (e.g. LN1C prior run produced a live digest; **not pinned** — GPU replay RED) |
| Exhaustive interpreted / JIT | **RED** — first divergence at `0x008dcb6b` (1 ULP; `got=0xc2ae7839` vs `want=0xc2ae7838` shape; both arms) |

**Candidate shapes tried and rejected (same toolchain, same fences):**

1. Geometric `1/(1+u)` nest + single `fma(k,LN2,ln1p)` — probe green; characterization ~14 ULP (unsuitable); exhaustive RED near floor (`0x0095db87`).
2. Fused-Newton recip + hi/lo two-fma reconstruction — probe RED at large positive `k` (`0x7f33786c`).
3. Fused-Newton + single `fma(k,LN2,ln1p)` — probe green; char ≤1 ULP; exhaustive RED near floor (`0x008fc4b9`).
4. Classic-Newton + single `fma` (**LN1C**, retained) — probe green; char ≤1 ULP; exhaustive RED near floor (`0x008dcb6b`).
5. Classic-Newton + separately-rounded `k*LN2+ln1p` — exhaustive RED even earlier (`0x00800009`); suggests the shader compiler does **not** simply expand outer `fma` into mul+add in a way CPU can mirror.

**Ruling requested:** bit-exact exhaustive LN over the full positive-normal domain
appears unreachable on this certified compiler/backend/driver under the current
allowed op set. Do **not** weaken the referee, shrink the domain, admit a
vendor `log`, or drop the roster row. Options for Owner/DA: new allowed
primitive (e.g. correctly-rounded `div` / table form), domain policy change, or
a measured ULP-band admission law (would be a design change, not coding
discretion).

## What did land (implementation progress, not admission)

- `LN = 27` closed-opcode widening; 5.10 call-site shapes; CPU/GPU/JIT wiring to
  `eml_ln_pinned` / `eml_ln_pinned_f32`.
- Library gadgets: `PowerLawGadget` / POW, `EmlOperatorGadget` (`eml()`),
  `EntropyTermGadget`, `LogAccumulateMapGadget` — ordinary EML only; no POW opcode.
- Mutation referees, CI freshness scaffold (`scripts/ci/eml_ln_qualification_check.sh`),
  cost-evidence harness (`#[ignore]`), results/orientation stubs.
- EXP sequence/identity untouched.

## Pinned artifacts

Admitted-domain size **2130706432**. Exhaustive reference digest: **`0x0`**
(intentionally unpinned — GPU replay does not match). Algorithm identity of the
retained LN1C candidate: **`0x108443cfaeeaadfe`**.

Coding returns **STOP / proof-present**. No clearance, merge, or pointer flip.
