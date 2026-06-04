pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_ID: &str = "ATLAS-BATCH-0-PACK";
pub const DRESS_REHEARSAL_ATLAS_BATCH_0_PACK_STATUS_PASS: &str =
    "IMPLEMENTED / PASS - EC-A2a pack plan + G=0 CPU oracle + VRAM report; EC-A2b GPU dispatch deferred to PACK-GPU";

/// Matches `simthing_spec::V78_ATLAS_DEFAULT_VRAM_BUDGET_BYTES` (1.5 GiB default profile).
pub const V78_ATLAS_VRAM_BUDGET_BYTES: u64 = 1_610_612_736;

pub const PACKING_STRATEGY: &str =
    "row-major tile placement per homogeneous class into the smallest enclosing atlas rectangle";

pub const CLASS_GALACTIC_20X20: &str = "Galactic20x20";
pub const CLASS_STAR_SYSTEM_10X10: &str = "StarSystem10x10";
pub const CLASS_PLANET_SURFACE_10X10: &str = "PlanetSurface10x10";

#[path = "dress_rehearsal_atlas_batch_0_loc.rs"]
mod loc;

pub use loc::{
    channel_set_has_kind, ChannelKind, ChannelSet, LocationId, LocationMaterialization,
    LocationRole, Owner,
};

const GALAXY_SIDE: u32 = 20;
const SYSTEM_SIDE: u32 = 10;
const PLANET_SURFACE_SIDE: u32 = 10;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileClassDescriptor {
    pub class_id: String,
    pub role: LocationRole,
    pub tile_width: u32,
    pub tile_height: u32,
    pub channels: ChannelSet,
    pub source_location_ids: Vec<LocationId>,
    pub atlas_width: u32,
    pub atlas_height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackedTile {
    pub source_location_id: LocationId,
    pub source_role: LocationRole,
    pub class_id: String,
    pub atlas_origin: (u32, u32),
    pub tile_dims: (u32, u32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TileMaskBounds {
    pub class_id: String,
    pub atlas_origin: (u32, u32),
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GZeroMaskDescriptor {
    pub rule: &'static str,
    pub tile_bounds: Vec<TileMaskBounds>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VramReport {
    pub unpacked_cell_count: u64,
    pub packed_cell_count: u64,
    pub mask_or_gutter_overhead_cells: u64,
    pub bytes_per_cell_assumption: u64,
    pub unpacked_bytes_estimate: u64,
    pub packed_bytes_estimate: u64,
    pub vram_multiplier: f64,
    pub budget_name: &'static str,
    pub budget_pass: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AtlasBatchPlan {
    pub classes: Vec<TileClassDescriptor>,
    pub tiles: Vec<PackedTile>,
    pub mask: GZeroMaskDescriptor,
    pub vram: VramReport,
    pub total_packed_cells: u64,
}

impl AtlasBatchPlan {
    pub fn canonical() -> Self {
        Self::from_materialization(&LocationMaterialization::canonical())
    }

    pub fn from_materialization(materialization: &LocationMaterialization) -> Self {
        let mut classes = Vec::new();
        let mut tiles = Vec::new();

        let galactic_locations: Vec<_> = materialization
            .locations
            .iter()
            .filter(|location| location.role == LocationRole::Galactic)
            .collect();
        let system_locations: Vec<_> = materialization
            .locations
            .iter()
            .filter(|location| location.role == LocationRole::StarSystem)
            .collect();
        let surface_locations: Vec<_> = materialization
            .locations
            .iter()
            .filter(|location| location.role == LocationRole::PlanetSurface)
            .collect();

        pack_class_row_major(
            CLASS_GALACTIC_20X20,
            LocationRole::Galactic,
            GALAXY_SIDE,
            GALAXY_SIDE,
            &galactic_locations,
            &mut classes,
            &mut tiles,
        );
        pack_class_row_major(
            CLASS_STAR_SYSTEM_10X10,
            LocationRole::StarSystem,
            SYSTEM_SIDE,
            SYSTEM_SIDE,
            &system_locations,
            &mut classes,
            &mut tiles,
        );
        pack_class_row_major(
            CLASS_PLANET_SURFACE_10X10,
            LocationRole::PlanetSurface,
            PLANET_SURFACE_SIDE,
            PLANET_SURFACE_SIDE,
            &surface_locations,
            &mut classes,
            &mut tiles,
        );

        let mask = build_g_zero_mask(&classes, &tiles);
        let vram = build_vram_report(materialization, &classes);
        let total_packed_cells: u64 = classes
            .iter()
            .map(|class| u64::from(class.atlas_width) * u64::from(class.atlas_height))
            .sum();

        Self {
            classes,
            tiles,
            mask,
            vram,
            total_packed_cells,
        }
    }

    pub fn class(&self, class_id: &str) -> Option<&TileClassDescriptor> {
        self.classes.iter().find(|class| class.class_id == class_id)
    }

    pub fn tiles_in_class(&self, class_id: &str) -> Vec<&PackedTile> {
        self.tiles
            .iter()
            .filter(|tile| tile.class_id == class_id)
            .collect()
    }
}

fn pack_class_row_major(
    class_id: &str,
    role: LocationRole,
    tile_width: u32,
    tile_height: u32,
    locations: &[&loc::LocationGridDescriptor],
    classes: &mut Vec<TileClassDescriptor>,
    tiles: &mut Vec<PackedTile>,
) {
    let tile_count = locations.len() as u32;
    let atlas_width = tile_width * tile_count;
    let atlas_height = tile_height;
    let channels = locations
        .first()
        .map(|location| location.channels.clone())
        .unwrap_or_else(|| ChannelSet {
            channels: Vec::new(),
        });

    let source_location_ids: Vec<_> = locations.iter().map(|location| location.id).collect();

    for (index, location) in locations.iter().enumerate() {
        tiles.push(PackedTile {
            source_location_id: location.id,
            source_role: location.role,
            class_id: class_id.to_string(),
            atlas_origin: (tile_width * index as u32, 0),
            tile_dims: (tile_width, tile_height),
        });
    }

    classes.push(TileClassDescriptor {
        class_id: class_id.to_string(),
        role,
        tile_width,
        tile_height,
        channels,
        source_location_ids,
        atlas_width,
        atlas_height,
    });
}

fn build_g_zero_mask(classes: &[TileClassDescriptor], tiles: &[PackedTile]) -> GZeroMaskDescriptor {
    let tile_bounds = tiles
        .iter()
        .map(|tile| TileMaskBounds {
            class_id: tile.class_id.clone(),
            atlas_origin: tile.atlas_origin,
            width: tile.tile_dims.0,
            height: tile.tile_dims.1,
        })
        .collect();

    let _ = classes;
    GZeroMaskDescriptor {
        rule: "algebraic tile-local G=0: stencil samples crossing a tile boundary resolve to 0",
        tile_bounds,
    }
}

fn build_vram_report(
    materialization: &LocationMaterialization,
    classes: &[TileClassDescriptor],
) -> VramReport {
    let mut unpacked_cell_count = 0u64;
    let mut unpacked_bytes_estimate = 0u64;

    for location in &materialization.locations {
        let cells = u64::from(location.width) * u64::from(location.height);
        let bytes_per_cell = bytes_per_cell_for_channels(&location.channels);
        unpacked_cell_count += cells;
        unpacked_bytes_estimate += cells * bytes_per_cell;
    }

    let mut packed_cell_count = 0u64;
    let mut packed_bytes_estimate = 0u64;

    for class in classes {
        let cells = u64::from(class.atlas_width) * u64::from(class.atlas_height);
        let bytes_per_cell = bytes_per_cell_for_channels(&class.channels);
        packed_cell_count += cells;
        packed_bytes_estimate += cells * bytes_per_cell;
    }

    let mask_or_gutter_overhead_cells = packed_cell_count.saturating_sub(unpacked_cell_count);
    let vram_multiplier = if unpacked_bytes_estimate == 0 {
        1.0
    } else {
        packed_bytes_estimate as f64 / unpacked_bytes_estimate as f64
    };

    VramReport {
        unpacked_cell_count,
        packed_cell_count,
        mask_or_gutter_overhead_cells,
        bytes_per_cell_assumption: 4,
        unpacked_bytes_estimate,
        packed_bytes_estimate,
        vram_multiplier,
        budget_name: "V78AtlasVramBudget",
        budget_pass: packed_bytes_estimate <= V78_ATLAS_VRAM_BUDGET_BYTES,
    }
}

fn bytes_per_cell_for_channels(channels: &ChannelSet) -> u64 {
    channels.channels.len() as u64 * 4
}

pub fn pack_coord(
    plan: &AtlasBatchPlan,
    location_id: LocationId,
    x: u32,
    y: u32,
) -> Option<(u32, u32)> {
    let tile = plan
        .tiles
        .iter()
        .find(|tile| tile.source_location_id == location_id)?;
    if x >= tile.tile_dims.0 || y >= tile.tile_dims.1 {
        return None;
    }
    Some((tile.atlas_origin.0 + x, tile.atlas_origin.1 + y))
}

pub fn unpack_coord(
    plan: &AtlasBatchPlan,
    class_id: &str,
    ax: u32,
    ay: u32,
) -> Option<(LocationId, u32, u32)> {
    let class = plan.class(class_id)?;
    if ax >= class.atlas_width || ay >= class.atlas_height {
        return None;
    }
    for tile in plan.tiles_in_class(class_id) {
        let (ox, oy) = tile.atlas_origin;
        let (tw, th) = tile.tile_dims;
        if ax >= ox && ax < ox + tw && ay >= oy && ay < oy + th {
            return Some((tile.source_location_id, ax - ox, ay - oy));
        }
    }
    None
}

fn tile_source_at_atlas(
    plan: &AtlasBatchPlan,
    class_id: &str,
    ax: u32,
    ay: u32,
) -> Option<LocationId> {
    let class = plan.class(class_id)?;
    if ax >= class.atlas_width || ay >= class.atlas_height {
        return None;
    }
    plan.tiles_in_class(class_id)
        .into_iter()
        .find(|tile| {
            let (ox, oy) = tile.atlas_origin;
            let (tw, th) = tile.tile_dims;
            ax >= ox && ax < ox + tw && ay >= oy && ay < oy + th
        })
        .map(|tile| tile.source_location_id)
}

/// CPU oracle: in-tile neighbor samples pass through `field`; crossing a tile boundary → 0.
pub fn g_zero_sample(
    plan: &AtlasBatchPlan,
    class_id: &str,
    ax: u32,
    ay: u32,
    neighbor: (u32, u32),
    field: &[f32],
) -> f32 {
    let (nax, nay) = neighbor;
    let center_source = match tile_source_at_atlas(plan, class_id, ax, ay) {
        Some(source) => source,
        None => return 0.0,
    };
    let neighbor_source = match tile_source_at_atlas(plan, class_id, nax, nay) {
        Some(source) => source,
        None => return 0.0,
    };
    if center_source != neighbor_source {
        return 0.0;
    }
    let class = plan
        .class(class_id)
        .expect("class_id validated by tile lookup");
    let Some(index) = atlas_linear_index(class.atlas_width, nax, nay) else {
        return 0.0;
    };
    field.get(index).copied().unwrap_or(0.0)
}

fn atlas_linear_index(atlas_width: u32, ax: u32, ay: u32) -> Option<usize> {
    Some((ay as usize) * (atlas_width as usize) + (ax as usize))
}

pub fn channel_set_matches(lhs: &ChannelSet, rhs: &ChannelSet) -> bool {
    lhs.channels == rhs.channels
}

pub fn channel_set_has_owner_indexed(set: &ChannelSet) -> bool {
    set.channels
        .iter()
        .any(|channel| matches!(channel.kind, ChannelKind::FleetStrength(_)))
}
