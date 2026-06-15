//! Single-source vanilla shape registry entries (PR8).

use std::collections::BTreeMap;

use crate::shape_registry::{
    RegisteredShapeName, ShapeParameterDescriptor, ShapeStrategyDescriptor, ShapeStrategyEntry,
};
use crate::strategy::ShapeStrategy;

use super::bar::BarStrategy;
use super::cartwheel::CartwheelStrategy;
use super::elliptical::EllipticalStrategy;
use super::ring::RingStrategy;
use super::spiral::SpiralStrategy;
use super::spoked::SpokedStrategy;
use super::starburst::StarburstStrategy;
use super::static_arbitrary::StaticArbitraryStrategy;

static ELLIPTICAL: EllipticalStrategy = EllipticalStrategy;
static STATIC: StaticArbitraryStrategy = StaticArbitraryStrategy;
static SPIRAL_2: SpiralStrategy = SpiralStrategy { arms: 2 };
static SPIRAL_3: SpiralStrategy = SpiralStrategy { arms: 3 };
static SPIRAL_4: SpiralStrategy = SpiralStrategy { arms: 4 };
static SPIRAL_6: SpiralStrategy = SpiralStrategy { arms: 6 };
static RING: RingStrategy = RingStrategy;
static BAR: BarStrategy = BarStrategy;
static STARBURST: StarburstStrategy = StarburstStrategy;
static CARTWHEEL: CartwheelStrategy = CartwheelStrategy;
static SPOKED: SpokedStrategy = SpokedStrategy;

/// Build the vanilla registry map — single source for descriptors and executable dispatch.
pub fn vanilla_entries() -> BTreeMap<String, ShapeStrategyEntry> {
    let entries: Vec<ShapeStrategyEntry> = vec![
        entry(static_descriptor(), Some(&STATIC)),
        entry(arbitrary_static_descriptor(), Some(&STATIC)),
        entry(elliptical_descriptor(), Some(&ELLIPTICAL)),
        entry(spiral_descriptor(2, "Two-arm spiral"), Some(&SPIRAL_2)),
        entry(spiral_descriptor(3, "Three-arm spiral"), Some(&SPIRAL_3)),
        entry(spiral_descriptor(4, "Four-arm spiral"), Some(&SPIRAL_4)),
        entry(spiral_descriptor(6, "Six-arm spiral"), Some(&SPIRAL_6)),
        entry(ring_descriptor(), Some(&RING)),
        entry(bar_descriptor(), Some(&BAR)),
        entry(starburst_descriptor(), Some(&STARBURST)),
        entry(cartwheel_descriptor(), Some(&CARTWHEEL)),
        entry(spoked_descriptor(), Some(&SPOKED)),
    ];
    entries
        .into_iter()
        .map(|e| (e.descriptor.name.0.clone(), e))
        .collect()
}

fn entry(
    descriptor: ShapeStrategyDescriptor,
    strategy: Option<&'static dyn ShapeStrategy>,
) -> ShapeStrategyEntry {
    ShapeStrategyEntry::new(descriptor, strategy)
}

fn param(key: &str, description: &str, required: bool) -> ShapeParameterDescriptor {
    ShapeParameterDescriptor {
        key: key.into(),
        description: description.into(),
        required,
    }
}

fn shared_procedural_params() -> Vec<ShapeParameterDescriptor> {
    vec![
        param(
            "core_radius",
            "Core void radius scale (producer-side)",
            false,
        ),
        param("jitter", "Placement jitter scale", false),
    ]
}

fn spiral_params() -> Vec<ShapeParameterDescriptor> {
    let mut params = shared_procedural_params();
    params.extend([
        param(
            "num_arms",
            "Arm count override (must match registered name when set)",
            false,
        ),
        param("arm_tightness", "Arm tightness scale", false),
        param("arm_width", "Perpendicular arm width", false),
    ]);
    params
}

fn static_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("static".into()),
        display_name: "Static".into(),
        description: "Explicit integer lattice cells (in-memory passthrough).".into(),
        parameters: vec![param(
            "coordinate_transform",
            "Optional coordinate transform label",
            false,
        )],
        executable: true,
        requires_explicit_cells: true,
    }
}

fn arbitrary_static_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("arbitrary_static".into()),
        display_name: "Arbitrary / static".into(),
        description: "Explicit point-cloud + graph admission (static_galaxy_scenario form).".into(),
        parameters: vec![param(
            "coordinate_transform",
            "Optional coordinate transform label",
            false,
        )],
        executable: true,
        requires_explicit_cells: true,
    }
}

fn elliptical_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("elliptical".into()),
        display_name: "Elliptical".into(),
        description: "Elliptical disc sampling over the square lattice.".into(),
        parameters: shared_procedural_params(),
        executable: true,
        requires_explicit_cells: false,
    }
}

fn spiral_descriptor(arms: u32, label: &str) -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName(format!("spiral_{arms}")),
        display_name: label.into(),
        description: format!("{arms}-arm spiral curve quantized to lattice cells."),
        parameters: spiral_params(),
        executable: true,
        requires_explicit_cells: false,
    }
}

fn ring_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("ring".into()),
        display_name: "Ring".into(),
        description: "Annular band with central void.".into(),
        parameters: {
            let mut params = shared_procedural_params();
            params.push(param("ring_radius", "Ring band center radius", false));
            params.push(param("arm_width", "Ring band width scale", false));
            params.push(param(
                "band_width",
                "Legacy alias for ring band width",
                false,
            ));
            params
        },
        executable: true,
        requires_explicit_cells: false,
    }
}

fn bar_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("bar".into()),
        display_name: "Bar".into(),
        description: "Elongated central bar with bounded jitter.".into(),
        parameters: {
            let mut params = shared_procedural_params();
            params.push(param("bar_length", "Bar length scale", false));
            params.push(param("bar_width", "Bar width scale", false));
            params
        },
        executable: true,
        requires_explicit_cells: false,
    }
}

fn starburst_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("starburst".into()),
        display_name: "Starburst".into(),
        description: "Radial burst / spoke distribution.".into(),
        parameters: {
            let mut params = shared_procedural_params();
            params.push(param("num_arms", "Spoke count", false));
            params
        },
        executable: true,
        requires_explicit_cells: false,
    }
}

fn cartwheel_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("cartwheel".into()),
        display_name: "Cartwheel".into(),
        description: "Ring band plus radial spokes / hub distribution.".into(),
        parameters: {
            let mut params = shared_procedural_params();
            params.push(param("ring_radius", "Ring band center radius", false));
            params.push(param("num_arms", "Spoke count", false));
            params
        },
        executable: true,
        requires_explicit_cells: false,
    }
}

fn spoked_descriptor() -> ShapeStrategyDescriptor {
    ShapeStrategyDescriptor {
        name: RegisteredShapeName("spoked".into()),
        display_name: "Spoked".into(),
        description: "Hub plus radial spoke sampling.".into(),
        parameters: {
            let mut params = shared_procedural_params();
            params.push(param("num_arms", "Spoke count", false));
            params
        },
        executable: true,
        requires_explicit_cells: false,
    }
}
