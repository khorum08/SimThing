# EML-LN-PRIMITIVE-0 results

- Track: 0.0.8.7 RF arena modernization (rung 5.12)
- Status: **COMPLETE / DA-GRADUATED — merged #1637 @ 97b5b399** (LND4; DA independently verified the grid arithmetic and planted a 1-ULP grid violation on LN2_HI — RED)
- Dispatch base: `4d998226d69113c537e42e9fdc97f6643d1895bc` (#1636); handoff base `7c6a5401`
- Branch: `fable/eml-ln-primitive-0`
- ORIENT-RECEIPT: `d950cd858719` (rule stamp `2d131557973b6050`, re-taken at base)
- HD-RECEIPT: `c864588e38e5`
- Dispatch: Board `5186770915`; DA algorithm order `5186693435`
- Scope: 5.12 only — no POW opcode, no trig, no StemThing-A/6.4, no 9.1

## ANCHOR-ACK + Candidate-F archaeology ACK

All 28 projected anchors ACK (roster identical to 5.11's, listed in the coding
projection). **Explicit rung-local archaeology ACK:** `docs/workshop/sqrt_candidates.md`
§§3–8 + §11 read and carried — the approximate hardware seed was lawful for sqrt ONLY
because `x−y²` is an exactly computable deciding residual (bitmask Dekker split, no exact
ties); the discipline carried is standalone-frozen-artifact → edge-first falsification →
exhaustive-before-promotion → separate promotion, NOT the sqrt residual formula, and NOT
the rejected LN EXP-residual analogue.

## The admitted primitive — candidate LND4

`LN` over positive finite normals `[2^-126, f32::MAX]` (bits `0x00800000..=0x7F7FFFFF`,
2,130,706,432 patterns) through the 5.10 door; `CLOSED_OPCODES` widens by exactly `LN = 27`
(roster 24 → 25). DA 5186693435 route — **no seed, no vendor transcendental, no f64, no
DIV, no loop, no data-dependent branch** — with two measured refinements (DA attention):

1. **Exact-residual reduction (Sol formulation):** `s` is carried as an exact
   double-single pair (`p_hi = m·inv_c` rounded; `p_err = fma(m, inv_c, −p_hi)` exact;
   `s = p_hi − 1` Sterbenz-exact). No reduction bit is ever discarded.
2. **Grid-exact reconstruction replacing runtime two-sum:** micro-diagnosis proved the
   certified tuple's compiler **collapses the Knuth two-sum error idiom to zero** (every
   error lane `0x00000000` on GPU; same eliminator class as 5.11's `(a+C)−C`). The fix
   moves exactness into AUTHORED CONSTANTS: `LN2_HI` and every `ln_c_hi` sit on a shared
   2^-16 grid so `k·LN2_HI` and `k·LN2_HI + ln_c_hi` are **exactly representable**
   (integer-grid arithmetic disguised as f32 — reassociation-immune by exactness; max
   grid integer 5,837,775 < 2^23, proven at authoring). 40-bit tails ride explicit `fma`
   only. A third measured refinement: the cubic term rides INSIDE an fma
   (`fma(z, s·poly, ·)`) so no intermediate can go subnormal (FTZ-immune by construction —
   the Candidate-F lesson).

Sequence: √2-folded exact bit decomposition (`t = bits − 0x3F330000`; identity cells
j=76/77 bracket 1.0 → `ln(1.0) = +0.0` exactly and the near-1 neighborhood is
pure-polynomial with a Sterbenz-exact argument); 128-cell const table (every `inv_c`
exactly representable ≤9 significant bits; per-cell |s| ≤ 2^-7.00 proven by exact rational
arithmetic at authoring); degree-5 `ln(1+s)`; ONE final f32 add rounds `(t_hi, g1)`.
Algorithm identity (version + tag `LND4` + constants + all 384 table words + domain):
**`0xc32ceb9f9807c0ca`**. Twin: `crates/simthing-core/src/eml_ln.rs`; frozen artifact:
`crates/simthing-driver/tests/wgsl/eml_ln_ds_candidate.wgsl`; helper + table byte-identical
across both production shader homes.

## Edge-first gate (ran BEFORE any sweep — dispatch standing order)

1,795 rows bit-exact GPU-vs-twin: `1.0 → +0.0` **exact**; `min_normal → 0xC2AEAC50`
(the exact input that killed LNCF at 2 ULP — now 0 ULP); `f32::MAX → 0x42B17218`; 1±ulp;
all 254 binade boundaries (both ends); all 128 table seams × 3 binades; 512 near-1 rows.
Two bring-up defects were caught BY the edge gate and fixed pre-sweep: (a) three
contractible mul-into-add sites → pinned as explicit `fma` (the 5.11 law); (b) the
two-sum collapse above → grid-exact v4 redesign. Full-domain CPU prototype validation
after each revision: max 1 ULP vs f64, zero over-1, monotone.

## Exhaustive admitted-domain qualification (LOCAL act — never CI)

```text
EML_LN_QUALIFY arm=cpu-reference  tested=2130706432 digest=0x196aced82d03f378 algorithm=0xc32ceb9f9807c0ca
EML_LN_QUALIFY arm=standalone-gpu tested=2130706432 digest=0x196aced82d03f378
EML_LN_QUALIFY arm=interpreted    tested=2130706432 digest=0x196aced82d03f378
EML_LN_QUALIFY arm=jit            tested=2130706432 digest=0x196aced82d03f378
```

**Every admitted pattern, all four arms bit-identical** (per-element first-divergence
asserts never fired; promotion commit followed standalone green, per the execution order).
Characterization (33.3M stratified ascending rows vs f64 `ln`): **max 1 ULP, zero over-1,
zero monotonicity violations**; no correct-rounding claim — the pinned sequence is the bit
law. Certified tuple: NVIDIA RTX 4080 Laptop / Vulkan / 595.79 / rustc 1.95.0 + wgpu
22.1.0 (roster + digests pinned in `crates/simthing-kernel/src/eml_ln_qualification.rs`;
live-tuple comparator gates every GPU referee path).

## Cost gate (unweakened key, real gadget baseline)

VK pipeline statistics, certified adapter — 21-node ordinary Horner-LN gadget baseline on
the canonical interpreter (Legacy32) vs the SSA-JIT guarded-LN block (Compact4), raw
driver values verbatim: baseline 32 reg / 17,664 B / 68,719,476,864; candidate **18 reg /
2,688 B / 68,719,476,736** — no regression + strict improvement on all three;
`verify_cost` minted.

## Consumers (authored data; 5.10 shape law; no new mechanism)

`LnConsumerGadgets` (kernel, guarded shape-2 call sites throughout): `PowerLaw(x;a) =
EXP(a·LN(x))` (no third opcode); `eml(x,y) = SUB(EXP(x), LN(y))` — **Anchor B becomes
literal**; entropy term `−p·LN(p)` with the p=0 case authored via SELECT; `LogAccumulate`
map (`LN` before the existing Sum lane — a new authored law, never a Product
substitution; no fold reordering). Naive/unguarded LN is a spanned admission error at
registry, field-sweep, and upload gates (mechanized, tested).

## Mutation referees (planted defects RED)

table-drift (one-ULP inv_c) → digest RED; single-f32 reconstruction collapse (the failed
pre-5.12 family shape) → digest RED; final-rounding lo-lane truncation → observable on
261,939/262,144 probe rows; planted freshness drifts (grid constant in twin; WGSL helper)
→ `eml_ln_qualification_check.sh --selftest` both FAIL; algorithm identity moves with any
table word.

## Failure archaeology (retained verbatim — do not erase, never resurrect)

| Tag | Identity | STOP |
|---|---|---|
| LN1C | `0x108443cfaeeaadfe` | exhaustive RED at `0x008dcb6b` (1 ULP, both GPU arms) — single-f32 reconstruction family |
| LNCF | `0xbc2f8faa558bb920` | probe RED at `0x00800000` (min_normal, 2 ULP; 1,478 mismatches) — vendor-log seed + EXP-residual ±1 ULP snap; the seed-accuracy assumption LND4 removes entirely |

PR #1635 remains closed/unmerged archaeology; nothing was branched or cherry-picked from it.

## Verification

```text
cargo test -p simthing-core --lib: 36 passed (incl. 4 LN twin pins)
cargo test -p simthing-kernel --lib eml_: 13 passed (incl. LN ritual, roster census 25,
  call-site battery, freshness tripwire)
cargo test -p simthing-workshop --test eml_ln_primitive_0_qualification: 4 passed
  (edge battery 1795 rows, 3 mutants); +5 ignored local acts all PASS (exhaustive
  CPU/standalone/interpreted/JIT + characterization)
cargo test -p simthing-workshop --test eml_ln_primitive_0_cost_evidence -- --ignored: 1 passed
cargo test -p simthing-workshop --test eml_exp_primitive_0_qualification: 4 passed (inherited 5.11)
bash scripts/ci/eml_exp_qualification_check.sh --check: PASS (5.11 freshness intact)
bash scripts/ci/eml_ln_qualification_check.sh --check + --selftest: PASS
agent_scan / inventory / hosted runs: see relay
```

## Posture

No `/clearance`, merge, pointer movement, StemThing-A/6.4, 9.1, POW-opcode, trig, or
successor work. LN semantics are append-only from this landing; promotion followed
exhaustive green strictly.
