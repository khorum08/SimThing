# Embedder Guide

Cold-reader surface for the Vendor Door graduated at 11.1. Five verbs, in order: **Derive, Populate, Overlay, Bind, Run**. No engine edits. No scenario-side wiring. `POW` is not an admitted opcode; a power law is `exp(k * ln x)` from admitted `EXP` and `LN`.

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
                overlay::TransformOp::set(110.0),
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

Need, corridor, front, and chokepoint are ordinary Bind thresholds over values the tree and overlay produce. They are not hand-fed. From `crates/simthing-embedder/tests/network_saturation_triad_0.rs`:

```rust
    let mut scenario = run::Scenario::map_light("network-saturation".into(), 1, 2, 1.0, 5);
```

```rust
    bind::velocity_threshold(
        &mut session,
        bind::VelocityAlertRegistration {
            sim_thing_id: origin.id,
            property_id: pid,
            sub_field: populate::SubFieldRole::Velocity,
            threshold: 0.0,
            direction: populate::Direction::Rising,
            cost_band: cost_band.clone(),
        },
    );
```

## Authored law: power as EXP and LN

Volume-delay is `1 + 0.15 * (v/c)^4`, composed as `exp(4 * ln ratio)`. From `crates/simthing-embedder/tests/network_saturation_triad_0.rs`:

```rust
    let delay = 1.0 + 0.15 * populate::eml_exp_pinned_f32(4.0 * populate::eml_ln_pinned_f32(ratio));
```

A staircase or piecewise ladder is the rival; it must disagree. Do not mint `POW`.
