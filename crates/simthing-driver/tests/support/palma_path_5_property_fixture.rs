//! PALMA-PATH-5 — admitted Location/gridcell W/D property columns → GPU MinPlusStencilOp.
//!
//! Seeds numeric `W` on gridcell SimProperty columns, gathers into the stencil input format,
//! runs the existing GPU min-plus band, writebacks `D` to property columns, and samples through
//! generic property access. No pathfinding engine, session RegionField scheduling, or movement policy.

use simthing_core::{
    ClampBehavior, DimensionRegistry, PropertyValue, SimPropertyId, SimThing, SimThingId,
    SubFieldRole, SubFieldSpec,
};
use simthing_gpu::{
    cpu_min_plus_d_from_w, max_d_field_error, pack_w_and_initial_d, project_tree_to_values,
    MinPlusStencilConfig, MinPlusStencilError,
};
use simthing_spec::{compile_property, PropertySpec};

use super::palma_min_plus_oracle::{cell_index, INF};
use super::palma_terran_pirate_fixture::{
    build_location_w_field, gridcell_simthing_id, sample_lowest_d_neighbor, GridCoord,
    LocationImpedanceField, CONVOY_START, DESTINATION, FIXTURE_HEIGHT, FIXTURE_ITERATIONS,
    FIXTURE_WIDTH,
};
use super::palma_terran_pirate_tree::{find_node, location_simthing_id, PalmaAdmittedTree};

pub const GRID_TRAVERSAL_NAMESPACE: &str = "palma";
pub const GRID_TRAVERSAL_NAME: &str = "grid_traversal";
pub const GRID_TRAVERSAL_W_ROLE: &str = "w";
pub const GRID_TRAVERSAL_D_ROLE: &str = "d";

/// Admitted Location/gridcell tree with per-gridcell `grid_traversal` W/D property columns.
pub struct PalmaPath5PropertyTree {
    pub inner: PalmaAdmittedTree,
    pub grid_traversal_id: SimPropertyId,
    pub w_local_offset: usize,
    pub d_local_offset: usize,
    pub w_global_col: usize,
    pub d_global_col: usize,
}

pub fn register_grid_traversal_property(
    registry: &mut DimensionRegistry,
) -> Result<(SimPropertyId, usize, usize), simthing_spec::SpecError> {
    let spec = PropertySpec {
        id: GRID_TRAVERSAL_NAME.into(),
        namespace: GRID_TRAVERSAL_NAMESPACE.into(),
        name: GRID_TRAVERSAL_NAME.into(),
        display_name: String::new(),
        description: String::new(),
        sub_fields: vec![
            SubFieldSpec {
                role: SubFieldRole::Named(GRID_TRAVERSAL_W_ROLE.into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: 1.0,
                display_name: GRID_TRAVERSAL_W_ROLE.into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
            SubFieldSpec {
                role: SubFieldRole::Named(GRID_TRAVERSAL_D_ROLE.into()),
                width: 1,
                clamp: ClampBehavior::Unbounded,
                velocity_max: None,
                default: INF,
                display_name: GRID_TRAVERSAL_D_ROLE.into(),
                display_range: None,
                governed_by: None,
                reduction_override: None,
                soft_aggregate_guard: None,
                accumulator_spec: None,
            },
        ],
    };
    let (id, _diag) = compile_property(&spec, registry)?;
    let layout = registry.property(id).layout.clone();
    let w_off = layout
        .offset_of(&SubFieldRole::Named(GRID_TRAVERSAL_W_ROLE.into()))
        .expect("w sub-field");
    let d_off = layout
        .offset_of(&SubFieldRole::Named(GRID_TRAVERSAL_D_ROLE.into()))
        .expect("d sub-field");
    Ok((id, w_off, d_off))
}

impl PalmaPath5PropertyTree {
    pub fn build_with_field(field: &LocationImpedanceField) -> Self {
        let mut inner = PalmaAdmittedTree::build();
        let (grid_traversal_id, w_local_offset, d_local_offset) =
            register_grid_traversal_property(&mut inner.reg)
                .expect("register grid_traversal property");
        inner.n_dims = inner.reg.total_columns.max(1);

        let layout = inner.reg.property(grid_traversal_id).layout.clone();
        let w_range = inner.reg.column_range(grid_traversal_id);
        let w_global_col = w_range.start + w_local_offset;
        let d_global_col = w_range.start + d_local_offset;

        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        for y in 0..height {
            for x in 0..width {
                let cell_id = gridcell_simthing_id(x, y);
                let cell = find_node_mut(&mut inner.root, cell_id).expect("gridcell");
                let mut pv = PropertyValue::from_layout(&layout);
                pv.data[w_local_offset] = field.w[cell_index(x, y, width)];
                pv.data[d_local_offset] = INF;
                cell.add_property(grid_traversal_id, pv);
            }
        }

        let dest = GridCoord {
            x: DESTINATION.0 as usize,
            y: DESTINATION.1 as usize,
        };
        if let Some(dest_cell) =
            find_node_mut(&mut inner.root, gridcell_simthing_id(dest.x, dest.y))
        {
            dest_cell
                .properties
                .get_mut(&grid_traversal_id)
                .unwrap()
                .data[d_local_offset] = 0.0;
        }

        inner.shadow = vec![0.0f32; inner.alloc.capacity() * inner.n_dims];
        let mut tree = Self {
            inner,
            grid_traversal_id,
            w_local_offset,
            d_local_offset,
            w_global_col,
            d_global_col,
        };
        tree.sync_shadow_from_tree();
        tree
    }

    pub fn build_default() -> Self {
        Self::build_with_field(&build_location_w_field(true, None, false))
    }

    pub fn min_plus_config(&self) -> MinPlusStencilConfig {
        super::palma_min_plus_oracle::test_config(FIXTURE_WIDTH, FIXTURE_HEIGHT, DESTINATION)
    }

    pub fn sync_shadow_from_tree(&mut self) {
        project_tree_to_values(
            &self.inner.root,
            &self.inner.reg,
            &self.inner.alloc,
            self.inner.n_dims,
            &mut self.inner.shadow,
        );
    }

    pub fn gridcell_id_at(&self, coord: GridCoord) -> SimThingId {
        gridcell_simthing_id(coord.x, coord.y)
    }

    pub fn read_w_from_property(&self, coord: GridCoord) -> f32 {
        let id = self.gridcell_id_at(coord);
        let cell = find_node(&self.inner.root, id).expect("gridcell");
        cell.properties[&self.grid_traversal_id].data[self.w_local_offset]
    }

    pub fn read_d_from_property(&self, coord: GridCoord) -> f32 {
        let id = self.gridcell_id_at(coord);
        let cell = find_node(&self.inner.root, id).expect("gridcell");
        cell.properties[&self.grid_traversal_id].data[self.d_local_offset]
    }

    /// Gather flat `W` in row-major grid order from admitted gridcell property columns.
    pub fn gather_w_flat_from_properties(&self) -> Vec<f32> {
        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        let mut w = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                w.push(self.read_w_from_property(GridCoord { x, y }));
            }
        }
        w
    }

    /// Cross-check: flat `W` from projected session shadow buffer at registry column offsets.
    pub fn gather_w_flat_from_shadow(&self) -> Vec<f32> {
        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        let n_dims = self.inner.n_dims;
        let mut w = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let id = gridcell_simthing_id(x, y);
                let slot = self.inner.alloc.slot_of(id).expect("gridcell slot") as usize;
                w.push(self.inner.shadow[slot * n_dims + self.w_global_col]);
            }
        }
        w
    }

    /// Write flat `D` back to gridcell property columns and resync shadow.
    pub fn write_d_flat_to_properties(&mut self, d: &[f32]) -> Result<(), MinPlusStencilError> {
        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        let cells = width * height;
        if d.len() != cells {
            return Err(MinPlusStencilError::BufferTooShort {
                actual: d.len(),
                required: cells,
            });
        }
        for y in 0..height {
            for x in 0..width {
                let idx = cell_index(x, y, width);
                let cell_id = gridcell_simthing_id(x, y);
                let cell = find_node_mut(&mut self.inner.root, cell_id).expect("gridcell");
                cell.properties
                    .get_mut(&self.grid_traversal_id)
                    .expect("grid_traversal property")
                    .data[self.d_local_offset] = d[idx];
            }
        }
        self.sync_shadow_from_tree();
        Ok(())
    }

    pub fn gather_d_flat_from_properties(&self) -> Vec<f32> {
        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        let mut d = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                d.push(self.read_d_from_property(GridCoord { x, y }));
            }
        }
        d
    }

    pub fn cpu_oracle_d_from_property_w(&self) -> Result<Vec<f32>, MinPlusStencilError> {
        let w = self.gather_w_flat_from_properties();
        cpu_min_plus_d_from_w(&w, &self.min_plus_config(), FIXTURE_ITERATIONS)
    }

    /// Sample lowest-D neighbor using property-column reads only (movable sampling path).
    pub fn sample_lowest_d_neighbor_from_properties(
        &self,
        from: GridCoord,
    ) -> Option<super::palma_terran_pirate_fixture::FieldSampleStep> {
        let width = FIXTURE_WIDTH as usize;
        let height = FIXTURE_HEIGHT as usize;
        let d = self.gather_d_flat_from_properties();
        sample_lowest_d_neighbor(&d, width, height, from)
    }

    pub fn pack_stencil_values_from_properties(&self) -> Result<Vec<f32>, MinPlusStencilError> {
        let w = self.gather_w_flat_from_properties();
        pack_w_and_initial_d(&w, &self.min_plus_config())
    }

    pub fn location_id(&self) -> SimThingId {
        location_simthing_id()
    }

    pub fn convoy_id(&self) -> SimThingId {
        self.inner.convoy_id
    }

    pub fn convoy_coord(&self) -> GridCoord {
        GridCoord {
            x: CONVOY_START.0,
            y: CONVOY_START.1,
        }
    }
}

pub fn find_node_mut<'a>(node: &'a mut SimThing, id: SimThingId) -> Option<&'a mut SimThing> {
    if node.id == id {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_mut(child, id) {
            return Some(found);
        }
    }
    None
}

/// Blocker ledger for PATH-5 — honest limits of this rung.
pub struct PalmaPath5BlockerLedger {
    pub session_region_field_min_plus_scheduling: bool,
    pub simsession_default_tick_wiring: bool,
    pub install_round_trip_required: bool,
}

impl PalmaPath5BlockerLedger {
    pub fn current() -> Self {
        Self {
            session_region_field_min_plus_scheduling: true,
            simsession_default_tick_wiring: true,
            install_round_trip_required: false,
        }
    }
}

pub fn max_d_field_error_public(cpu_d: &[f32], gpu_d: &[f32]) -> f32 {
    max_d_field_error(cpu_d, gpu_d)
}
