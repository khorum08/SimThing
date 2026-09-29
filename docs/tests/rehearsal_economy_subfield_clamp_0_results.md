# Native sub-field clamp projection (authored finite bounds)

**Status: the projection stands. A bound on a CONSERVED RF cell now refuses at admission (DA
intervention 3(b) of the 0.0.8.8 drift audit, live Board `5879126789`; see the last section). The
fence below became law.**

DA increment per Orchestration relay `5745626815` (Astra return `5745459860` accepted as a truthful
authoring STOP on 2.2 `0088-ECONOMY-FLEET-0`; #2075 remains OPEN/DRAFT). No 2.2 semantic verdict.
Model 2 remains CLOSED. Parser admission is NOT a bounded-storage proof: saturation, headroom and
recovery accounting remain #2075's obligation.

## Ruling

**No existing native representation admits a finite bound on an authored cell.** Independent
archaeology confirms Astra's:

- `ClampBehavior::Bounded { min, max }` and `Floored { min }` have existed in core since the
  property model landed (`simthing-core/src/property.rs`);
- the ONLY ClauseThing producers of `Bounded` are hydrator-internal generated cells
  (`hydrate_field_economy.rs`: field-economy velocity `-1..1` and intensity `0..1`), which no
  author can address;
- `rehearsal_ingress_fields.rs::parse_subfield` admitted `role`, `default`, `governed_by` and
  `accumulator`, refused everything else, and hardcoded `ClampBehavior::Unbounded`.

**The runtime law already exists and is already executed.** This is the decisive fact:
`governed_pairs_for_property` emits one `GovernedPair` per sub-field carrying `governed_by`, with
`clamp_kind`, `clamp_min` and `clamp_max`; the kernel's governed integration
(`accumulator_op.wgsl::integrate_clamp_at_slots`) applies `apply_amount_clamp` to every integrated
value and truncates the governing velocity at both bounds (`amount_at_floor` → clamp negative
velocity to zero; `amount_at_ceiling` → clamp positive velocity to zero). A bounded governed cell
therefore saturates with headroom under the ONE registry-wide governed integration.

So the gap was authoring only, and the admitted increment is a direct projection of existing data
onto the existing parser. No new type, no second mechanism, no runtime law, no E8 pin
(`rehearsal_ingress_fields.rs` is not a `build.rs` component; all 31 components and `Cargo.lock`
are untouched).

## The law

```
sub_field = { role = balance governed_by = balance_rate accumulator = Balance
              clamp = Bounded { min = 0 max = 24 } }
sub_field = { role = Amount clamp = Floored { min = 0 } }
sub_field = { role = Velocity clamp = Unbounded }
```

- **Whole enum, not a subset.** All three existing arms project. A `Bounded`-only admission would
  have guaranteed a second relay the first time a stock needs a floor with no ceiling, and the
  third arm costs one match arm.
- **Omission is unchanged.** No `clamp` field means `Unbounded`, exactly as before, so every
  historical document keeps its meaning.
- **Bounds are finite and ordered.** `min <= max`, equality allowed (a degenerate pinned cell);
  non-finite values refuse.
- **Fails closed with a span**, before activation, on: a reversed bound; `Bounded` missing `min` or
  `max`; `Floored` carrying `max`; an unknown clamp form (scalar or header); an unknown bound key;
  a duplicated bound; a duplicated `clamp` field (existing duplicate-field law).

## Proof floor executed (reference machine)

**Hydration** (`simthing-clausething`, `native_subfield_clamp_0`, 2/2):
- `Bounded { min = 0 max = 24 }`, `Bounded { min = 3 max = 3 }`, `Floored { min = -2.5 }` and
  `Unbounded` project to exactly those core values;
- omission leaves every cell `Unbounded`, the roles are identical either way, and a declared bound
  changes only the cell that declares it;
- eleven malformed forms refuse before activation, each with its token: `min <= max`; missing
  `max`; missing `min`; `Floored` with `max`; an empty block (which degenerates to a scalar at the
  parse layer and refuses there); `Ceiling`; a bare `Sealed`; `max = 1e40`; bound key `lo`;
  duplicated `min`; duplicated `clamp`.

**Ordinary native session** (`simthing-workshop`,
`native_subfield_clamp_session_0::an_authored_bound_survives_the_cache_and_saturates_in_execution`):
the shipped two-faction bundle is re-authored in a temp copy, with the governed energy balance
bounded to `0..1` and an otherwise identical unbounded control.
- The live registry carries `Bounded { min: 0.0, max: 1.0 }`; the control carries `Unbounded`.
- A session rebuilt from the derived canonical cache carries the same bound: the cache is not a
  second authority.
- Across five generations no row ever exceeds the ceiling.
- Eight rows that the control carries PAST the ceiling by RISING (each reaching `2.5`) sit at
  exactly `1.0` under the bound. Rows that merely start high are excluded from that set, so this is
  saturation, not a static clamp.

**Mutants (4/4 RED, each restored):**
- the authored clamp ignored → hydration and session witnesses RED;
- bound ordering unchecked → hydration RED;
- an unknown bound key silently ignored → hydration RED;
- `Floored` accepting `max` → hydration RED.

**Regression** (0 failures): clausething 47 (45 + this law's 2), spec 16, core 32, workshop 16
(15 + this law's 1), mapeditor 25. One mapeditor attempt aborted on a corrupted incremental
state left by concurrent builds (`only metadata stub found for rlib dependency core`); rerun
alone it passes 25/0. No E8 roll: `rehearsal_ingress_fields.rs` is not a `build.rs` component.

## Fence carried to #2075 (not settled here)

A bounded cell inside a CONSERVED arena is a sink at its ceiling and a source at its floor: the
governed integration truncates velocity there, so allocation that the arena disbursed can stop
arriving. RF-1's `balance_governed` fact (#2076) assumes a governed leaf settles its allocation
exactly. Authoring a bound therefore does NOT by itself make bounded storage lawful in a conserved
path. #2075 must either bound a cell outside the conserved path, or carry explicit
saturation/overflow/recovery accounting that reconciles the difference. That accounting is the 2.2
obligation this increment unblocks, not one it discharges.

## The fence became law: conserved RF cells refuse a bound

The drift audit judged the fence above against the 0.0.8.7 foundation. The Invariant Set says
conservation holds "for any input", and StemThing law 3 says a cap is a budget. A bound on a cell
that carries an arena's conserved quantity truncates the governed integration and destroys the flow
the arena settled. So it is not bounded storage; it is a leak. Admission now refuses it at the one RF
door, `compile_resource_flow_admission`, which every authored and derived arena passes through:
`SpecError::AdmissionRefused { law_id: "rf-conserved-cell-unclamped", element_path }`.

**The conserved cells.** A property belongs to an arena when it is the arena's flow or balance
property, or when it binds `AllocatedFlow` to that arena. In such a property the conserved cells are:
- the arena's `AllocatedFlow`;
- every `Balance`, which settles it;
- each Balance's governing rate.

An `IntrinsicFlow` bound only shapes the authored source term, and an `AllocatorWeight` bound touches
no conserved quantity, so both stay admitted. The built-in residency-capacity arena and every shipped
scenario author no bound, so nothing shipped changes. Storage bounds for 2.2 are budgets (intervention
5).

**The witness inverts.** `native_subfield_clamp_session_0` re-authors the shipped two-faction bundle
once per case.

| case | outcome |
|---|---|
| balance `Bounded { 0, 1 }` | refuses typed at ordinary admission |
| balance `Floored { 0 }` | refuses typed at ordinary admission |
| balance_rate `Bounded { -1, 1 }` | refuses typed at ordinary admission |
| allocated `Floored { 0 }` | refuses typed at ordinary admission |
| weight `Bounded { 0, 5 }` | admitted; the live registry and the cache rebind hold it; a generation runs |
| flow `Floored { 0 }` | admitted; the live registry and the cache rebind hold it; a generation runs |
| balance `Unbounded` (explicit) | admitted; a generation runs |
| omission | admitted; a generation runs |

The saturation test it replaces is deleted under `authorized_deletions.tsv` (`5879126789-DA`). The
hydration projection (`native_subfield_clamp_0`) is unchanged.

**Evidence** (NVIDIA 616.92, `--features simthing-gpu/eml-resource-profiling`):
- **Witness 1/1:** 4 refusals and 4 admissions, with cache rebind.
- **Seal untouched:** no sealed component changes. The parity referee passes 1/1 against the current pin, observing `490e902413acc670`, so there is no roll.
- **Full suites, 0 failed:** spec 16, driver 58 (1 ignored), workshop 16, mapeditor 25, clausething 47 (2 ignored).
