# BAND-QUANTIZED-DRAW-0 results

Rung: `BAND-QUANTIZED-DRAW-0` (6.1b)
Handoff: Board comment `5170209783`
DA amended ruling: Board comment `5170146070`
Orientation-freshness remand: Board comment `5171604399`
HD-RECEIPT: `d7cf7f107500`
ORIENT-RECEIPT: `6af1884543b0`
orientation_rule_stamp: `5554b2613f8907ff`
Branch: `grok/band-quantized-draw-0`
Code head measured: `348d6efaaf649a7e780d1f3a907c28fea1692671`
Status: **PROBATION / proof-present / DA-review-pending — amended generation-level STOP fired**

The accepted bounded code and measurement delta was re-reviewed under the
current branch orientation. No semantic change is required; this remand changes
governance identity only and preserves the exact generation-level STOP evidence.

## Bounded remand disposition

`TransformOp::apply_with_params` remains the singular caller-visible execution
entry. Under that entry, the admitted one-node Set/LITERAL, Add, and Multiply
programs now use derived arithmetic specializations. The stored, admitted, and
wire form remains the same opaque EML node vector; there is no tag, caller mode,
second representation, or alternate admission path.

The obsolete production interpreter-entry counter was removed because the DA
ruling explicitly permits derived specialization under the singular entry, and
the counter itself added hot-path cost. Set, Add, and Multiply specialization
results are compared bit-for-bit with `eval_overlay_eml` over the same admitted
node programs.

Everything outside this bounded change remains frozen. No 6.2 work, clearance,
merge, pointer movement, or dispatch was performed.

## Binding generation-level measurement

### Exact comparison

- Prejoin direct-path SHA: `7d9766299be96e4b35da02e678c88b985307b176`.
- Specialized SHA: `348d6efaaf649a7e780d1f3a907c28fea1692671`.
- Byte-identical benchmark source at both SHAs (LF-canonical SHA-256):
  `adb90c780f60c88e1b8c12ba3fe59e2ec34ff319249d72ee435b691ad3ccd52c`.
- Build: Cargo release profile, `rustc 1.95.0 (59807616e 2026-04-14)`,
  LLVM 22.1.2, host `x86_64-pc-windows-msvc`, `cargo 1.95.0`.
- Machine: Windows Home 25H2 build 26200.8973; Intel Core i9-13980HX,
  24 physical cores / 32 logical processors.

### Representative workload and rationale

The benchmark uses the ordinary `Evaluator`, a generic tree of 40,000 Cohort
participants with one property each, and one inherited Governance overlay
containing Set, Add, and Multiply on Amount. This produces exactly 120,000
overlay applications per generation. The 40,000-participant scale corresponds
to the canonical 200 x 200-small structural population scale while keeping the
benchmark itself mechanism-generic. Each process performs three warmup
generations followed by 15 measured generations.

Both variants deserialize the historical Set/Add/Multiply wire form through the
same benchmark source. Every warmup and measured generation asserts the final
Amount result is bit-identical to `0.9375f32`.

### Raw repeated samples

All values are wall-clock nanoseconds per complete generation. `p10` and `p90`
use the benchmark's fixed order-statistic indices over each sorted 15-sample
run.

| variant | run | samples (ns) | median | p10 | p90 |
|---|---:|---|---:|---:|---:|
| prejoin `7d976629` | 1 | `[11567700, 11560300, 11381900, 11543300, 11834500, 11930300, 11560500, 11854900, 11413200, 11376300, 11495900, 11662100, 11700600, 11847800, 11466500]` | 11560500 | 11381900 | 11854900 |
| prejoin `7d976629` | 2 | `[12117000, 11461200, 11407700, 11693100, 11725600, 11635800, 11815900, 11815100, 11573100, 12105300, 11412200, 12285900, 12094700, 12182200, 11426200]` | 11725600 | 11412200 | 12182200 |
| prejoin `7d976629` | 3 | `[11266500, 11823400, 11313800, 11392000, 11268100, 11331100, 11097700, 11570300, 11564400, 11154100, 11809200, 11712500, 11647500, 11867200, 11351300]` | 11392000 | 11154100 | 11823400 |
| specialized `348d6efa` | 1 | `[15027700, 15168900, 15435300, 15461500, 15248100, 15256200, 14655400, 14966800, 15097400, 14949000, 14892700, 14855100, 14432200, 14739400, 15051400]` | 15027700 | 14655400 | 15435300 |
| specialized `348d6efa` | 2 | `[14689200, 14615900, 14817200, 14898200, 15052800, 14946100, 14602300, 14993600, 15201700, 15439700, 15130000, 14816200, 14915000, 15394500, 14906900]` | 14915000 | 14615900 | 15394500 |
| specialized `348d6efa` | 3 | `[15844500, 15655000, 15695200, 15548500, 15050400, 15299200, 15265200, 15157000, 15432100, 15421100, 15310800, 15147200, 15818500, 15402200, 15549100]` | 15421100 | 15147200 | 15818500 |

Median of the three run medians:

- prejoin: **11,560,500 ns/generation**;
- specialized: **15,027,700 ns/generation**;
- delta: **+3,467,200 ns/generation**;
- ratio: **1.2999178x**, or **+29.9918%**.

The specialized distribution is cleanly separated from the prejoin result and
the generation-level regression is measurable. Under the amended DA ruling,
the required **STOP fires**. No optimization, representation clone, scale
redefinition, or architectural workaround was attempted after this finding.

### Residual per-op secondary measurement

The exact specialized code head reports:

```text
specialized_med=0.665ns/op prejoin_med=0.193ns/op ratio=3.45
samples=7 iters=500000
```

This is secondary diagnostic evidence only; the binding disposition above is
the representative per-generation wall-clock comparison.

## Focused validation

- `cargo check -p simthing-core`: PASS (inherited warnings only).
- `cargo test -p simthing-core --lib -q`: PASS, 25/25.
- `cargo test -p simthing-core --test band_quantized_draw_0 -q`: PASS, 9/9.
- `cargo test -p simthing-core --doc -q`: PASS, 27/27.
- `cargo test -p simthing-sim --test band_quantized_draw_production_0 -q`: PASS, 7/7.
- `test_inventory_drift_check.sh`: PASS, 1,061 discovered / 1,061 ledgered.
- `detachability_check.sh`: PASS, production coupling 0 / proof coupling 0.
- `test_lifecycle_expiry_check.sh --schema`: PASS.
- `scenario_residue_check.sh`: INSPECT with scenario 0 / domain 0 and 58
  inherited dead-export advisories outside this bounded diff.
- The repository-wide inventory policy check retains inherited failures outside
  this remand; the two changed oracle-parity rows satisfy KEEP/permanent-residue
  policy and introduce no new inventory-policy finding.

## Frozen proof inventory retained from the accepted PR

- `VelocityAlertRegistration` / `AggregateAlertRegistration` carry
  `CostBandSemantic`; `ThresholdBuilder` admits the stored recipe throttle.
- `TransformOp` is an opaque admitted EML program struct, and compile-fail
  proofs prevent enum variants, external matching, a computed form, or private
  node forging.
- Production boundary resolution, off-by-one N oracle, conservation, one-node
  Set shape, and runtime N-dependent behavior proofs remain green.
- Zero WGSL diff; zero `ThresholdRegistration` layout change; no allowlist
  widening; Stage 2 recipe/output-scale re-expression remains deferred.
