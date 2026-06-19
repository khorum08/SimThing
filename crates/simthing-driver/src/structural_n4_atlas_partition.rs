//! Structural N4 atlas partition/admission for oversized frames.
//!
//! Partitions oversized structural layouts into bounded execution theaters without
//! shrinking scenario authority. Cross-partition N4 halo exchange is deferred.

use simthing_spec::{
    MappingExecutionProfile, SimThingScenarioSpec, REGION_FIELD_MAX_CELL_COUNT,
    REGION_FIELD_STANDARD_MAX_GRID,
};

use crate::structural_n4_theater_compile::{
    build_theater_geometry, derive_n4_edges, AtlasDeferralReason, CompiledStructuralN4Theater,
    CompiledStructuralPlacement, StructuralGridCoordinate, StructuralTheaterCompileError,
};

/// Origin of a partition theater within the original structural frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StructuralTheaterOrigin {
    pub col: u32,
    pub row: u32,
}

/// One bounded partition theater with global provenance metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionedStructuralN4Theater {
    pub theater: CompiledStructuralN4Theater,
    pub origin: StructuralTheaterOrigin,
    pub partition_index: u32,
}

/// Cross-partition N4 adjacency deferred from first-slice bounded execution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeferredCrossPartitionN4Edge {
    pub global_a: StructuralGridCoordinate,
    pub global_b: StructuralGridCoordinate,
    pub partition_index_a: u32,
    pub partition_index_b: u32,
}

/// Bounded partition tile sizing for atlas admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StructuralAtlasPartitionProfile {
    pub max_theater_width: u32,
    pub max_theater_height: u32,
    pub include_overlap_halo: bool,
}

impl Default for StructuralAtlasPartitionProfile {
    fn default() -> Self {
        Self {
            max_theater_width: REGION_FIELD_STANDARD_MAX_GRID,
            max_theater_height: REGION_FIELD_STANDARD_MAX_GRID,
            include_overlap_halo: false,
        }
    }
}

/// Partitioned structural atlas preserving original frame metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompiledStructuralN4Atlas {
    pub original_frame_width: u32,
    pub original_frame_height: u32,
    pub theaters: Vec<PartitionedStructuralN4Theater>,
    pub partition_profile: StructuralAtlasPartitionProfile,
    pub deferred_cross_partition_edges: Vec<DeferredCrossPartitionN4Edge>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StructuralAtlasAdmission {
    Single(CompiledStructuralN4Theater),
    Partitioned(CompiledStructuralN4Atlas),
    Deferred {
        frame_width: u32,
        frame_height: u32,
        occupied_cells: u64,
        reason: AtlasDeferralReason,
    },
}

fn fits_single_bounded_theater(
    theater: &CompiledStructuralN4Theater,
    profile: &StructuralAtlasPartitionProfile,
) -> bool {
    let cells = theater.frame_width.saturating_mul(theater.frame_height);
    theater.frame_width <= profile.max_theater_width
        && theater.frame_height <= profile.max_theater_height
        && cells <= REGION_FIELD_MAX_CELL_COUNT
}

fn tiles_per_axis(span: u32, max_tile: u32) -> u32 {
    if max_tile == 0 {
        return 0;
    }
    (span + max_tile - 1) / max_tile
}

fn partition_tile_index(
    coord: StructuralGridCoordinate,
    frame_width: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> u32 {
    let tile_col = coord.col / profile.max_theater_width;
    let tile_row = coord.row / profile.max_theater_height;
    let tiles_per_row = tiles_per_axis(frame_width, profile.max_theater_width);
    tile_row * tiles_per_row + tile_col
}

fn build_partition_theater(
    full: &CompiledStructuralN4Theater,
    origin_col: u32,
    origin_row: u32,
    tile_width: u32,
    tile_height: u32,
    partition_index: u32,
) -> PartitionedStructuralN4Theater {
    let mut occupied_cells = Vec::new();
    let mut system_placements = Vec::new();

    for placement in &full.system_placements {
        if placement.col >= origin_col
            && placement.col < origin_col.saturating_add(tile_width)
            && placement.row >= origin_row
            && placement.row < origin_row.saturating_add(tile_height)
        {
            let local_col = placement.col - origin_col;
            let local_row = placement.row - origin_row;
            occupied_cells.push(StructuralGridCoordinate {
                col: local_col,
                row: local_row,
            });
            system_placements.push(CompiledStructuralPlacement {
                system_id: placement.system_id,
                col: local_col,
                row: local_row,
                location_id: placement.location_id.clone(),
                simthing_id_raw: placement.simthing_id_raw,
            });
        }
    }

    occupied_cells.sort_unstable();
    system_placements.sort_by_key(|placement| placement.system_id);

    let theater = CompiledStructuralN4Theater {
        frame_width: tile_width,
        frame_height: tile_height,
        occupied_cells: occupied_cells.clone(),
        n4_edges: derive_n4_edges(&occupied_cells),
        system_placements,
        execution_profile: full.execution_profile,
    };

    PartitionedStructuralN4Theater {
        theater,
        origin: StructuralTheaterOrigin {
            col: origin_col,
            row: origin_row,
        },
        partition_index,
    }
}

fn partition_oversize_theater(
    full: CompiledStructuralN4Theater,
    partition_profile: StructuralAtlasPartitionProfile,
) -> Result<CompiledStructuralN4Atlas, StructuralTheaterCompileError> {
    if partition_profile.max_theater_width == 0 || partition_profile.max_theater_height == 0 {
        return Err(StructuralTheaterCompileError::FrameDimensionOverflow);
    }
    if partition_profile.include_overlap_halo {
        return Err(StructuralTheaterCompileError::FrameDimensionOverflow);
    }

    let original_frame_width = full.frame_width;
    let original_frame_height = full.frame_height;
    let tiles_col = tiles_per_axis(original_frame_width, partition_profile.max_theater_width);
    let tiles_row = tiles_per_axis(original_frame_height, partition_profile.max_theater_height);

    let mut theaters = Vec::with_capacity((tiles_col * tiles_row) as usize);
    let mut partition_index = 0u32;
    let mut origin_row = 0u32;
    while origin_row < original_frame_height {
        let tile_height =
            (original_frame_height - origin_row).min(partition_profile.max_theater_height);
        let mut origin_col = 0u32;
        while origin_col < original_frame_width {
            let tile_width =
                (original_frame_width - origin_col).min(partition_profile.max_theater_width);
            let tile_cells = tile_width
                .checked_mul(tile_height)
                .ok_or(StructuralTheaterCompileError::FrameDimensionOverflow)?;
            if tile_cells > REGION_FIELD_MAX_CELL_COUNT {
                return Err(StructuralTheaterCompileError::FrameDimensionOverflow);
            }
            theaters.push(build_partition_theater(
                &full,
                origin_col,
                origin_row,
                tile_width,
                tile_height,
                partition_index,
            ));
            partition_index += 1;
            origin_col = origin_col.saturating_add(partition_profile.max_theater_width);
        }
        origin_row = origin_row.saturating_add(partition_profile.max_theater_height);
    }

    let mut deferred_cross_partition_edges = Vec::new();
    for (global_a, global_b) in &full.n4_edges {
        let idx_a =
            partition_tile_index(*global_a, original_frame_width, &partition_profile);
        let idx_b =
            partition_tile_index(*global_b, original_frame_width, &partition_profile);
        if idx_a != idx_b {
            deferred_cross_partition_edges.push(DeferredCrossPartitionN4Edge {
                global_a: *global_a,
                global_b: *global_b,
                partition_index_a: idx_a,
                partition_index_b: idx_b,
            });
        }
    }
    deferred_cross_partition_edges.sort_by(|left, right| {
        (
            left.partition_index_a,
            left.partition_index_b,
            left.global_a,
            left.global_b,
        )
            .cmp(&(
                right.partition_index_a,
                right.partition_index_b,
                right.global_a,
                right.global_b,
            ))
    });

    Ok(CompiledStructuralN4Atlas {
        original_frame_width,
        original_frame_height,
        theaters,
        partition_profile,
        deferred_cross_partition_edges,
    })
}

/// Compile structural N4 atlas admission: single bounded theater or partitioned atlas.
///
/// Reads scenario structural grid authority only. Does not invoke GPU operators,
/// sim scheduler, or shrink the scenario frame.
pub fn compile_structural_n4_atlas(
    scenario: &SimThingScenarioSpec,
    profile: MappingExecutionProfile,
    partition_profile: StructuralAtlasPartitionProfile,
) -> Result<StructuralAtlasAdmission, StructuralTheaterCompileError> {
    let theater = build_theater_geometry(scenario, profile)?;

    if fits_single_bounded_theater(&theater, &partition_profile) {
        return Ok(StructuralAtlasAdmission::Single(theater));
    }

    Ok(StructuralAtlasAdmission::Partitioned(
        partition_oversize_theater(theater, partition_profile)?,
    ))
}
