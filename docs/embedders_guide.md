# Embedder Guide

Cold-reader surface for the Vendor Door. Five verbs, in order: **Derive, Populate, Overlay, Bind, Run**. No engine edits. No scenario-side wiring. The only ingress is this door: unified authoring → RF Triad resolution → domain contention → unified SimThing execution. `POW` is not an opcode. Authored power is `EmlGadgetInstanceSpec::PowerLaw` with a positive `input_floor`.

Every rust block below is copied from an exemplar that runs. Do not paraphrase the blocks; if they drift, the admission gate fails.

## Import the door

From `crates/simthing-embedder/tests/finance_toy_0.rs`:

```rust
use simthing_embedder::{bind, derive, overlay, populate, run};
```

## Derive, then Populate

Author an owner seat, then bind only the ownership crossing. Descendants inherit by absence. From `crates/simthing-embedder/tests/finance_toy_0.rs`:

```rust
    let seat = derive::owner_seat("alpha", "Alpha Desk", "desk").expect("owner seat");
```

```rust
    let owner = populate::OwnerRef::try_new_authored("alpha").expect("owner");
    populate::owner(root, &owner);
    populate::ownership(root).expect("one crossing");
```

Stand up a running tree with the door's `Scenario`, not a game scenario pack:

```rust
    let mut scenario = run::Scenario::map_light("finance-toy".into(), 1, 2, 1.0, 3);
```

## Overlay

An overlay needs an in-tree origin and a non-empty horizon. From `crates/simthing-embedder/tests/finance_toy_0.rs`:

```rust
    overlay::authored(
        root,
        origin,
        overlay::OverlayKind::Instruction,
        overlay::OverlaySource::System,
        vec![origin.id],
        overlay::PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![(
                populate::SubFieldRole::Amount,
                populate::TransformOp::set(110.0),
            )],
        },
        vec![overlay::DissolveCondition::AtSessionEnd],
    )
    .expect("overlay")
```

## Bind and Run

Initialize, start, tick, observe, serialize. From `crates/simthing-embedder/tests/finance_toy_0.rs`:

```rust
    let mut session = run::initialize(scenario, &run::GameModeSpec::default()).expect("init");
    run::start(&mut session, run::ExecutionPosture::Paced).expect("start");
    run::tick(&mut session).expect("tick");
    let shadow = bind::shadow(&session);
```

```rust
    let summary = run::serialize(&mut session, replay.path(), 1).expect("serialize");
```

## Network saturation (full Triad)

Declare competing emitters, admit PALMA and Gu-Yang through Bind, and observe born comparative outputs. Generic thresholds are not a substitute. From `crates/simthing-embedder/tests/network_saturation_triad_0.rs`:

```rust
        derive::ComparativeEmitterClass {
            authored_order: 0,
            class_id: 0.0,
            value_col: col(1, authored_bound),
        },
```

```rust
            let palma = bind::compile_palma_n4_field_sweep(bind::PalmaN4FieldSweepSpec {
                width: 2,
                height: 2,
                n_dims,
                d_col: col(10, authored_bound),
                w_col: col(13, authored_bound),
                destination_slot: bind::SlotIndex::new(0),
                inf_sentinel: f32::MAX,
            })?;
```

```rust
    let observed = bind::observe_gu_yang_stall(&session).expect("born Gu-Yang stall");
```

## Authored law: PowerLaw gadget

Volume-delay is the admitted gadget, not a staircase and not a hand-rolled `exp(k * ln x)` call. From `crates/simthing-embedder/tests/network_saturation_triad_0.rs`:

```rust
        gadgets: vec![derive::EmlGadgetInstanceSpec::PowerLaw {
            id: "volume-delay".into(),
            input_col: 0,
            output_col: Some(1),
            exponent: 4.0,
            input_floor: 0.25,
        }],
```

A staircase or piecewise ladder is the rival. Do not mint `POW`.
