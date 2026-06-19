//! Structural N4 atlas partition/admission for oversized frames.
//!
//! Partitions oversized structural layouts into bounded execution theaters without
//! shrinking scenario authority. Optional one-cell structural N4 halo admission
//! represents cross-partition adjacency in bounded theater inputs.

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

/// Padding applied to theater-local coordinates so halo cells remain non-negative.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StructuralTheaterCoordPadding {
    pub west: u32,
    pub north: u32,
}

/// Owned partition tile cell vs one-cell structural N4 halo neighbor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructuralTheaterCellRole {
    Owned,
    Halo,
}

/// One admitted halo cell with global provenance and GPU-local coordinates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralTheaterHaloCell {
    pub global_coord: StructuralGridCoordinate,
    pub local_coord: StructuralGridCoordinate,
    pub source_partition_index: u32,
}

/// Provenance record that a deferred cross-partition edge is covered by halo metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CrossPartitionHaloCoverage {
    pub edge: DeferredCrossPartitionN4Edge,
    pub partition_a_has_halo: bool,
    pub partition_b_has_halo: bool,
}

/// One bounded partition theater with global provenance metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartitionedStructuralN4Theater {
    pub theater: CompiledStructuralN4Theater,
    pub origin: StructuralTheaterOrigin,
    pub partition_index: u32,
    pub coord_padding: StructuralTheaterCoordPadding,
    pub halo_cells: Vec<StructuralTheaterHaloCell>,
}

impl PartitionedStructuralN4Theater {
    /// Recover global coordinates for a theater-local cell (after coord padding).
    pub fn global_from_local(&self, local: StructuralGridCoordinate) -> StructuralGridCoordinate {
        StructuralGridCoordinate {
            col: self
                .origin
                .col
                .saturating_add(local.col)
                .saturating_sub(self.coord_padding.west),
            row: self
                .origin
                .row
                .saturating_add(local.row)
                .saturating_sub(self.coord_padding.north),
        }
    }

    /// Whether a theater-local coordinate is an admitted halo cell.
    pub fn cell_role(&self, local: StructuralGridCoordinate) -> StructuralTheaterCellRole {
        if self.halo_cells.iter().any(|halo| halo.local_coord == local) {
            StructuralTheaterCellRole::Halo
        } else {
            StructuralTheaterCellRole::Owned
        }
    }
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
    pub halo_coverage: Vec<CrossPartitionHaloCoverage>,
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

fn owned_tile_span(max_tile: u32, include_overlap_halo: bool) -> u32 {
    if include_overlap_halo {
        max_tile.saturating_sub(1).max(1)
    } else {
        max_tile
    }
}

fn partition_tile_index(
    coord: StructuralGridCoordinate,
    frame_width: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> u32 {
    let tile_width = owned_tile_span(profile.max_theater_width, profile.include_overlap_halo);
    let tile_height = owned_tile_span(profile.max_theater_height, profile.include_overlap_halo);
    let tile_col = coord.col / tile_width;
    let tile_row = coord.row / tile_height;
    let tiles_per_row = tiles_per_axis(frame_width, tile_width);
    tile_row * tiles_per_row + tile_col
}

fn tile_origin_for_index(
    partition_index: u32,
    frame_width: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> StructuralTheaterOrigin {
    let tile_width = owned_tile_span(profile.max_theater_width, profile.include_overlap_halo);
    let tile_height = owned_tile_span(profile.max_theater_height, profile.include_overlap_halo);
    let tiles_per_row = tiles_per_axis(frame_width, tile_width);
    let tile_col = partition_index % tiles_per_row;
    let tile_row = partition_index / tiles_per_row;
    StructuralTheaterOrigin {
        col: tile_col * tile_width,
        row: tile_row * tile_height,
    }
}

fn tile_span_for_origin(
    origin: StructuralTheaterOrigin,
    frame_width: u32,
    frame_height: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> (u32, u32) {
    let owned_width = owned_tile_span(profile.max_theater_width, profile.include_overlap_halo);
    let owned_height = owned_tile_span(profile.max_theater_height, profile.include_overlap_halo);
    let tile_width = (frame_width - origin.col).min(owned_width);
    let tile_height = (frame_height - origin.row).min(owned_height);
    (tile_width, tile_height)
}

fn cell_in_tile(
    global: StructuralGridCoordinate,
    origin: StructuralTheaterOrigin,
    tile_width: u32,
    tile_height: u32,
) -> bool {
    global.col >= origin.col
        && global.col < origin.col.saturating_add(tile_width)
        && global.row >= origin.row
        && global.row < origin.row.saturating_add(tile_height)
}

fn collect_owned_partition_cells(
    full: &CompiledStructuralN4Theater,
    origin: StructuralTheaterOrigin,
    tile_width: u32,
    tile_height: u32,
) -> (
    Vec<StructuralGridCoordinate>,
    Vec<CompiledStructuralPlacement>,
) {
    let mut occupied_cells = Vec::new();
    let mut system_placements = Vec::new();

    for placement in &full.system_placements {
        let global = StructuralGridCoordinate {
            col: placement.col,
            row: placement.row,
        };
        if !cell_in_tile(global, origin, tile_width, tile_height) {
            continue;
        }
        let local_col = placement.col - origin.col;
        let local_row = placement.row - origin.row;
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

    occupied_cells.sort_unstable();
    system_placements.sort_by_key(|placement| placement.system_id);
    (occupied_cells, system_placements)
}

fn tentative_local(
    global: StructuralGridCoordinate,
    origin: StructuralTheaterOrigin,
) -> (i64, i64) {
    (
        i64::from(global.col) - i64::from(origin.col),
        i64::from(global.row) - i64::from(origin.row),
    )
}

fn normalize_theater_geometry(
    origin: StructuralTheaterOrigin,
    owned_locals: &[StructuralGridCoordinate],
    halo_globals: &[(StructuralGridCoordinate, u32)],
    execution_profile: MappingExecutionProfile,
    system_placements: Vec<CompiledStructuralPlacement>,
) -> (
    CompiledStructuralN4Theater,
    StructuralTheaterCoordPadding,
    Vec<StructuralTheaterHaloCell>,
) {
    let mut tentative_coords: Vec<(i64, i64)> = owned_locals
        .iter()
        .map(|coord| (i64::from(coord.col), i64::from(coord.row)))
        .collect();
    for (global, _) in halo_globals {
        tentative_coords.push(tentative_local(*global, origin));
    }

    let min_col = tentative_coords
        .iter()
        .map(|(col, _)| *col)
        .min()
        .unwrap_or(0);
    let min_row = tentative_coords
        .iter()
        .map(|(_, row)| *row)
        .min()
        .unwrap_or(0);
    let padding = StructuralTheaterCoordPadding {
        west: min_col.min(0).unsigned_abs() as u32,
        north: min_row.min(0).unsigned_abs() as u32,
    };

    let shift_tentative = |(col, row): (i64, i64)| StructuralGridCoordinate {
        col: (col + i64::from(padding.west)) as u32,
        row: (row + i64::from(padding.north)) as u32,
    };

    let mut occupied_cells: Vec<StructuralGridCoordinate> = owned_locals
        .iter()
        .map(|coord| shift_tentative((i64::from(coord.col), i64::from(coord.row))))
        .collect();
    for (global, _) in halo_globals {
        let shifted = shift_tentative(tentative_local(*global, origin));
        if !occupied_cells.contains(&shifted) {
            occupied_cells.push(shifted);
        }
    }
    occupied_cells.sort_unstable();

    let max_col = occupied_cells
        .iter()
        .map(|coord| coord.col)
        .max()
        .unwrap_or(0);
    let max_row = occupied_cells
        .iter()
        .map(|coord| coord.row)
        .max()
        .unwrap_or(0);
    let frame_width = max_col.saturating_add(1);
    let frame_height = max_row.saturating_add(1);

    let halo_cells = halo_globals
        .iter()
        .map(
            |(global, source_partition_index)| StructuralTheaterHaloCell {
                global_coord: *global,
                local_coord: shift_tentative(tentative_local(*global, origin)),
                source_partition_index: *source_partition_index,
            },
        )
        .collect();

    let shifted_placements: Vec<CompiledStructuralPlacement> = system_placements
        .into_iter()
        .map(|mut placement| {
            let shifted = shift_tentative((i64::from(placement.col), i64::from(placement.row)));
            placement.col = shifted.col;
            placement.row = shifted.row;
            placement
        })
        .collect();

    let theater = CompiledStructuralN4Theater {
        frame_width,
        frame_height,
        occupied_cells: occupied_cells.clone(),
        n4_edges: derive_n4_edges(&occupied_cells),
        system_placements: shifted_placements,
        execution_profile,
    };

    (theater, padding, halo_cells)
}

fn build_partition_theater(
    full: &CompiledStructuralN4Theater,
    origin_col: u32,
    origin_row: u32,
    tile_width: u32,
    tile_height: u32,
    partition_index: u32,
) -> PartitionedStructuralN4Theater {
    let origin = StructuralTheaterOrigin {
        col: origin_col,
        row: origin_row,
    };
    let (owned_locals, system_placements) =
        collect_owned_partition_cells(full, origin, tile_width, tile_height);
    let (theater, padding, halo_cells) = normalize_theater_geometry(
        origin,
        &owned_locals,
        &[],
        full.execution_profile,
        system_placements,
    );

    PartitionedStructuralN4Theater {
        theater,
        origin,
        partition_index,
        coord_padding: padding,
        halo_cells,
    }
}

fn validate_theater_caps(
    theater: &CompiledStructuralN4Theater,
    partition_index: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> Result<(), StructuralTheaterCompileError> {
    let cells = theater
        .frame_width
        .checked_mul(theater.frame_height)
        .ok_or(StructuralTheaterCompileError::FrameDimensionOverflow)?;
    if theater.frame_width > profile.max_theater_width
        || theater.frame_height > profile.max_theater_height
        || cells > REGION_FIELD_MAX_CELL_COUNT
    {
        return Err(StructuralTheaterCompileError::HaloExceedsTheaterCap {
            partition_index,
            frame_width: theater.frame_width,
            frame_height: theater.frame_height,
            max_width: profile.max_theater_width,
            max_height: profile.max_theater_height,
        });
    }
    Ok(())
}

fn apply_halo_admission(
    theaters: &mut [PartitionedStructuralN4Theater],
    full: &CompiledStructuralN4Theater,
    deferred_edges: &[DeferredCrossPartitionN4Edge],
    frame_width: u32,
    frame_height: u32,
    profile: &StructuralAtlasPartitionProfile,
) -> Result<Vec<CrossPartitionHaloCoverage>, StructuralTheaterCompileError> {
    let mut halo_globals_by_partition: Vec<Vec<(StructuralGridCoordinate, u32)>> =
        vec![Vec::new(); theaters.len()];

    let mut halo_coverage = Vec::with_capacity(deferred_edges.len());
    for edge in deferred_edges {
        let origin_a = tile_origin_for_index(edge.partition_index_a, frame_width, profile);
        let origin_b = tile_origin_for_index(edge.partition_index_b, frame_width, profile);
        let (tile_w_a, tile_h_a) =
            tile_span_for_origin(origin_a, frame_width, frame_height, profile);
        let (tile_w_b, tile_h_b) =
            tile_span_for_origin(origin_b, frame_width, frame_height, profile);

        let partition_a_needs_b = !cell_in_tile(edge.global_b, origin_a, tile_w_a, tile_h_a);
        let partition_b_needs_a = !cell_in_tile(edge.global_a, origin_b, tile_w_b, tile_h_b);

        if partition_a_needs_b {
            push_unique_halo(
                &mut halo_globals_by_partition[edge.partition_index_a as usize],
                edge.global_b,
                edge.partition_index_b,
            );
        }
        if partition_b_needs_a {
            push_unique_halo(
                &mut halo_globals_by_partition[edge.partition_index_b as usize],
                edge.global_a,
                edge.partition_index_a,
            );
        }

        halo_coverage.push(CrossPartitionHaloCoverage {
            edge: edge.clone(),
            partition_a_has_halo: partition_a_needs_b,
            partition_b_has_halo: partition_b_needs_a,
        });
    }

    for entry in theaters.iter_mut() {
        let partition_index = entry.partition_index as usize;
        if halo_globals_by_partition[partition_index].is_empty() {
            continue;
        }
        let (tile_width, tile_height) =
            tile_span_for_origin(entry.origin, frame_width, frame_height, profile);
        let (owned_locals, system_placements) =
            collect_owned_partition_cells(full, entry.origin, tile_width, tile_height);
        let (theater, padding, halo_cells) = normalize_theater_geometry(
            entry.origin,
            &owned_locals,
            &halo_globals_by_partition[partition_index],
            full.execution_profile,
            system_placements,
        );
        validate_theater_caps(&theater, entry.partition_index, profile)?;
        entry.theater = theater;
        entry.coord_padding = padding;
        entry.halo_cells = halo_cells;
    }

    Ok(halo_coverage)
}

fn push_unique_halo(
    halos: &mut Vec<(StructuralGridCoordinate, u32)>,
    global: StructuralGridCoordinate,
    source_partition_index: u32,
) {
    if halos.iter().any(|(existing, _)| *existing == global) {
        return;
    }
    halos.push((global, source_partition_index));
}

fn partition_oversize_theater(
    full: CompiledStructuralN4Theater,
    partition_profile: StructuralAtlasPartitionProfile,
) -> Result<CompiledStructuralN4Atlas, StructuralTheaterCompileError> {
    if partition_profile.max_theater_width == 0 || partition_profile.max_theater_height == 0 {
        return Err(StructuralTheaterCompileError::FrameDimensionOverflow);
    }

    let original_frame_width = full.frame_width;
    let original_frame_height = full.frame_height;
    let owned_width = owned_tile_span(
        partition_profile.max_theater_width,
        partition_profile.include_overlap_halo,
    );
    let owned_height = owned_tile_span(
        partition_profile.max_theater_height,
        partition_profile.include_overlap_halo,
    );
    let tiles_col = tiles_per_axis(original_frame_width, owned_width);
    let tiles_row = tiles_per_axis(original_frame_height, owned_height);

    let mut theaters = Vec::with_capacity((tiles_col * tiles_row) as usize);
    let mut partition_index = 0u32;
    let mut origin_row = 0u32;
    while origin_row < original_frame_height {
        let tile_height = (original_frame_height - origin_row).min(owned_height);
        let mut origin_col = 0u32;
        while origin_col < original_frame_width {
            let tile_width = (original_frame_width - origin_col).min(owned_width);
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
            origin_col = origin_col.saturating_add(owned_width);
        }
        origin_row = origin_row.saturating_add(owned_height);
    }

    let mut deferred_cross_partition_edges = Vec::new();
    for (global_a, global_b) in &full.n4_edges {
        let idx_a = partition_tile_index(*global_a, original_frame_width, &partition_profile);
        let idx_b = partition_tile_index(*global_b, original_frame_width, &partition_profile);
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

    let halo_coverage = if partition_profile.include_overlap_halo {
        apply_halo_admission(
            &mut theaters,
            &full,
            &deferred_cross_partition_edges,
            original_frame_width,
            original_frame_height,
            &partition_profile,
        )?
    } else {
        Vec::new()
    };

    Ok(CompiledStructuralN4Atlas {
        original_frame_width,
        original_frame_height,
        theaters,
        partition_profile,
        deferred_cross_partition_edges,
        halo_coverage,
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
