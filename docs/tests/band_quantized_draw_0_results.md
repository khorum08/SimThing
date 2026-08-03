# BAND-QUANTIZED-DRAW-0 results

Rung: `BAND-QUANTIZED-DRAW-0` (6.1b)
Canonical handoff: Board comment `5170209783`
Allocation/stack-clone remand: Board comment `5172156294`
DA diagnosis/ruling: Board comment `5171898023`
HD-RECEIPT: `d7cf7f107500`
ORIENT-RECEIPT: `6af1884543b0`
orientation_rule_stamp: `5554b2613f8907ff`
Branch: `grok/band-quantized-draw-0`
Implementation SHA measured: `78a4ad8e6a65aaf47597daaf2608ccadd05179c0`
Status: **COMPLETE / DA-GRADUATED — merged #1602 @ 63c8ed51**

## Bounded remand disposition

The repair uses the authorized inline-small-program representation. The single
private `TransformOp.nodes` field is now a `SmallVec<[EmlNode; 3]>`: Set uses
one inline node, Add and Multiply use three inline nodes, and longer admitted
programs spill to heap. This was chosen over the GPU program-table alternative
because `simthing-core` owns the CPU value/admission boundary and cannot reuse
the kernel-owned `EmlGpuProgramTable` without reversing the crate dependency or
creating a second authority.

This remains exactly one opaque admitted EML value form. There is no static or
computed discriminant, tag, alternate wire form, caller-selected mode, or
second admission rule. Historical wire compatibility is unchanged, the same
private cap validator governs both the public admission helper and
`TransformOp` construction, and the compile-fail seals remain load-bearing.

`TransformStack` is now a persistent `Arc`-shared tail. Cloning a traversal
state clones one pointer; pushing clones only the new delta and points to the
existing tail. Root-first traversal preserves the previous ancestor-to-leaf
semantics while removing the accumulated-stack deep clone and its O(depth²)
multiplier.

Mechanical allocation/clone proof:

- `degenerate_programs_stay_inline_and_larger_programs_spill` proves Set, Add,
  Multiply, and their clones do not spill, while a four-node program does.
- `transform_stack_push_shares_history_and_preserves_order` proves both clone
  and push retain pointer-identical history, then proves Set-at-root followed by
  Add-at-leaf still produces bit-identical `5.0f32`.

Everything else remains frozen: CostBand production semantics, per-program
cap, opcode set, WGSL, `ThresholdRegistration` layout, allowlists, Stage 2, and
6.2+ were not changed.

## Binding generation-level measurement

### Exact comparison

- Prejoin direct-path SHA: `7d9766299be96e4b35da02e678c88b985307b176`.
- Repaired implementation SHA: `78a4ad8e6a65aaf47597daaf2608ccadd05179c0`.
- Byte-identical benchmark source relative to the accepted STOP evidence
  (SHA-256): `adb90c780f60c88e1b8c12ba3fe59e2ec34ff319249d72ee435b691ad3ccd52c`.
- Build: Cargo release profile, `rustc 1.95.0 (59807616e 2026-04-14)`,
  LLVM 22.1.2, host `x86_64-pc-windows-msvc`, `cargo 1.95.0`.
- Machine: Windows Home 25H2 build 26200.8973; Intel Core i9-13980HX,
  24 physical cores / 32 logical processors.

The benchmark still uses the ordinary `Evaluator`, 40,000 generic Cohort
participants with one property each, and one inherited Governance overlay
containing Set, Add, and Multiply on Amount: exactly 120,000 applications per
generation. Each process performs three warmups and 15 measured generations.
Every warmup and measurement asserts a bit-identical final `0.9375f32` amount.

### Raw repeated samples

All values are wall-clock nanoseconds per complete generation. The three
prejoin rows are the accepted byte-identical baseline; the repaired rows were
rerun at the exact implementation tree above.

| variant | run | samples (ns) | median | p10 | p90 |
|---|---:|---|---:|---:|---:|
| prejoin `7d976629` | 1 | `[11567700, 11560300, 11381900, 11543300, 11834500, 11930300, 11560500, 11854900, 11413200, 11376300, 11495900, 11662100, 11700600, 11847800, 11466500]` | 11560500 | 11381900 | 11854900 |
| prejoin `7d976629` | 2 | `[12117000, 11461200, 11407700, 11693100, 11725600, 11635800, 11815900, 11815100, 11573100, 12105300, 11412200, 12285900, 12094700, 12182200, 11426200]` | 11725600 | 11412200 | 12182200 |
| prejoin `7d976629` | 3 | `[11266500, 11823400, 11313800, 11392000, 11268100, 11331100, 11097700, 11570300, 11564400, 11154100, 11809200, 11712500, 11647500, 11867200, 11351300]` | 11392000 | 11154100 | 11823400 |
| repaired `78a4ad8e` | 1 | `[9417500, 9675400, 10126900, 9748800, 9530700, 9997400, 9903700, 9386300, 9647000, 9645000, 9401100, 9375100, 9512500, 9548800, 9295200]` | 9548800 | 9375100 | 9997400 |
| repaired `78a4ad8e` | 2 | `[9230300, 9103000, 9387200, 9098200, 9063700, 9140000, 9252400, 9404200, 9732300, 9457900, 10423400, 9535100, 10493800, 9399500, 9128300]` | 9387200 | 9098200 | 10423400 |
| repaired `78a4ad8e` | 3 | `[9578800, 9384700, 9578200, 10032200, 9653000, 9045200, 9385500, 9909900, 9428900, 9177000, 9600100, 9672300, 10236500, 9574900, 10612600]` | 9578800 | 9177000 | 10236500 |

Median of the three run medians:

- prejoin: **11,560,500 ns/generation**;
- repaired: **9,548,800 ns/generation**;
- delta: **-2,011,700 ns/generation**;
- ratio: **0.8259850x**, or **-17.4015%**;
- generation delta divided by 120,000 applications: **-16.7642
  ns/application**.

The repaired distribution is faster than the accepted prejoin baseline, so
there is no measurable generation-level regression and no positive residual to
waive. Relative to the remanded 15,027,700 ns median, the repair removes
5,478,900 ns/generation (-36.4587%).

### Residual per-op secondary measurement

The secondary one-node Set diagnostic at the implementation tree reports:

```text
specialized_med=0.771ns/op prejoin_med=0.193ns/op ratio=3.99
samples=7 iters=500000
```

The observed isolated interpretation residual is `+0.578 ns/op`. It remains a
secondary diagnostic only; the binding ordinary-Evaluator result above is
faster than prejoin and retains the bit-identity assertion.

## Focused validation

- `cargo check -p simthing-core`: PASS (inherited warnings only).
- `cargo test -p simthing-core --lib --no-fail-fast`: PASS, 27/27.
- `cargo test -p simthing-core --test band_quantized_draw_0 -q`: PASS, 9/9.
- `cargo test -p simthing-core --doc -q`: PASS, 27/27.
- `cargo test -p simthing-sim --test band_quantized_draw_production_0 -q`: PASS, 7/7.
- `test_inventory_drift_check.sh`: PASS, 1,063 discovered / 1,063 ledgered.
- `doctrine_scan.sh`: INSPECT with 0 hard failures and 671 inherited heuristic
  findings; scanner surface unchanged, so scanner selftest is not required.
- `detachability_check.sh`: PASS, production coupling 0 / proof coupling 0.
- `test_lifecycle_expiry_check.sh --schema`: PASS.
- `scenario_residue_check.sh`: INSPECT with scenario 0 / domain 0 and 58
  inherited dead-export advisories outside this bounded diff.
- Repository-wide `test_inventory_check.sh` retains its inherited failures;
  the two new AUDIT rows are present in the drift-clean single inventory and
  introduce no new policy finding.

## Orientation and current ACKs

The carried coding identity is `ORIENT-RECEIPT 6af1884543b0`, rule stamp
`5554b2613f8907ff`, orientation digest
`dbad9a1a65d1783adfea26de1ef1c8f9dee1ca64f89c219336a168ba74c8e48c`.
The remand path query acknowledged the current anchors:

- `core-overlays@54df7604a49d`
- `core-property-value-model@04338b307bf8`
- `core-rf-arenas@d171614211e9`
- `exact-numeric-candidate-f@6938a2efadb5`
- `rf-arena-substrate@17b5f1e5c2ba`
- `simthing-0087-binding-laws@a59203b79425`
- `simthing-0087-pillars@61487cba1f9e`
- `stead-events-are-rf@525388344ef2`
- `stead-rejected-shapes@3752549ff106`
- `stead-shared-surface-ledger@87eaa1e7bb9c`
- `stead-spatial-contract-core@8585db4ac631`
- `workshop-candidate-homing@3e584f0ad175`

No clearance, merge, pointer movement, or 6.2 work was performed.
