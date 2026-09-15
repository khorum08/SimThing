# Independent-resource binding law — child-share formula column-agnosticity + TENTH E8 roll

DA increment per Orchestration relay `5688590364` (Astra STOP `5688400035` accepted as
truthful). **Third true substrate gap of the 0.0.8.8 rehearsal.** Model 1 remains
UNDECIDED; Model 2 remains CLOSED; a binding repair is evidence for neither. Draft #2062
stays OPEN/UNMERGED and returns to Astra unchanged in semantic expectation after this
lands.

## The defect (cause chain verified in source before repair)

`compile_child_share_formula_nodes` baked `SLOT_VALUE(cols.weight_col)` into the ONE
globally-registered child-share tree (`CHILD_SHARE_FORMULA_TREE_ID`, register-once), and
`arena_allocation_sync` registered per-arena while `disburse_op` evaluated the same tree
for every arena — so whichever arena registered first supplied the absolute weight column
consumed by ALL arenas. Astra's executed matrix (#2062, preserved as the falsifier): A
alone `.75/.25` GREEN, B alone `.25/.75` GREEN, joint A-first both `.75/.25`, joint
B-first both `.25/.75`, ordinary `open_from_spec + step_once` wrong on B — order-dependent
aliasing exactly as the source predicts.

## The repair (the relay's preferred minimal shape, admitted)

Verified against the kernel's EXISTING EvalEML admission before implementation
(`encode.rs`: input-list PARAM binding admits at most four inputs; exactly one write
target; per-input slots are native) — **no kernel change; containment preserved.**

- Formula is column-agnostic: `SLOT_VALUE(weight_col)` → `PARAM(3)`; the tree owns no
  absolute arena column.
- Each `disburse_op` supplies the TARGET CHILD'S OWN currently-resolved `weight_col` at
  the child's slot as the fourth admitted input (declared input order is the PARAM order).
- `register_child_share_formula(registry)` is one generic registration; per-arena column
  resolution remains as layout validation. ONE EML authority; no per-resource namespace,
  parallel registry, host replay, CPU correction, or alternate allocator.
- Rebind/layout coupling (proof floor 5): structurally eliminated — the shared formula no
  longer references any column, so layout changes cannot stale it; the operation supplies
  the currently-resolved child weight each plan.

## Witnesses (falsifier-proven)

- `child_share_formula_is_column_agnostic_with_child_weight_as_param_3` (formula level):
  no `SLOT_VALUE` node anywhere in the tree; child weight arrives as `PARAM(3)` exactly.
- `independent_resource_layouts_bind_their_own_child_weight_in_both_orders` (planner
  level): two DISTINCT resource column layouts, BOTH planning orders — every disbursement
  op carries exactly four inputs with `input[3] = (child slot, that arena's own
  weight_col)`.
- **Mutant RED (witness bite):** with the fourth input removed (the defect shape), the
  planner witness FAILS at `child weight must ride as the fourth input`; restored, all
  driver lib tests 11/11 GREEN.
- Prior law preserved: interior-policy overlay witnesses 3/3, post-RF observation 1/1,
  accumulator convergence seal 2/2 — all GREEN on the repaired source.
- #2062's executed RED matrix is preserved untouched as the historical falsifier; its
  GREEN rerun after rebase is Astra's, through the full §5.3 Model-1 discriminator.

## TENTH E8 roll (mechanically required: child_share_eml / plan / sync are bundle components)

- **Old-pin refusal (RED, required first), repaired source, reference tuple** (RTX 4080
  Laptop / Vulkan / NVIDIA 595.79; rustc 1.95.0; `simthing-gpu/eml-resource-profiling`):
  `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: d5fcaf92eda4f724`; parity referee FAILED
  `left: 15419392269796833060` (observed `0xd5fc_af92_eda4_f724`) vs
  `right: 8061962344363647220` (pinned `0x6fe1_d809_c05e_e0f4`).
- **Roll:** exactly the two existing literals, identical independently observed value; no
  third site; no comparator/component-list/`build.rs`/Cargo change:
  `QUALIFIED_RESIDENT_CLEARING_FINGERPRINT` and `QUALIFIED_RECORD_FINGERPRINT`
  `0x6fe1_d809_c05e_e0f4` → `0xd5fc_af92_eda4_f724`.
- Pin chain: `0xb295…` → `0x42a7…` → `0x6f28…` → `0x6fe1…` → `0xd5fc_af92_eda4_f724`.
- **Post-roll battery (floor = ninth roll, all reference machine):**
  - parity `resident_clearing_parity_0`:
    `RESIDENT-CLEARING-QUALIFICATION-FINGERPRINT: d5fcaf92eda4f724`; 1 passed
    (fingerprint equality + full mutant matrix inside the referee).
  - `resident_clearing_score_and_bands_0`: 3 passed — including
    `allocated_flow_is_direct_recursive_evaleml_input_and_gpu_self_consumes`
    STRENGTHENED: its expected input list now also pins PARAM(3) = the target child's own
    resolved weight, fencing this increment's defect permanently (the pre-strengthening
    referee correctly RED-flagged the four-input shape, proving it watches the list).
  - `simthing-gpu resident_clearing_runtime`: 4 passed.

## Clean-checkout proof

CLEANCHECKOUT_PLACEHOLDER
