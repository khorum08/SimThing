//! Compile admitted structural theater + mapping operator specs into generic sim plans.
//!
//! Does not deserialize scenario authority, instantiate GPU operators, or run ticks.
//! Structural theater admission remains in [`crate::structural_n4_theater_compile`].

use simthing_sim::{CompiledMappingPlan, CompiledMappingStep};
use simthing_spec::{CompiledRegionFieldPreview, CompiledWImpedanceCompose};
use thiserror::Error;

use crate::first_slice_mapping_runtime::compiled_stencil_to_gpu_config;
use crate::structural_n4_theater_compile::CompiledStructuralN4Theater;
use crate::w_impedance_compose_bridge::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config,
};
use simthing_core::StructuralCoord;

/// Admitted mapping operator inputs for generic sim plan assembly.
#[derive(Clone, Debug)]
pub struct MappingPlanCompileSpec {
    pub structured_field: CompiledRegionFieldPreview,
    pub structured_hops: u32,
    pub structured_to_interleaved_writes: Vec<(u32, u32)>,
    pub w_compose: CompiledWImpedanceCompose,
    pub min_plus_profile_index: usize,
    pub min_plus_dest: StructuralCoord,
    pub min_plus_d_col: u32,
    pub min_plus_iterations: u32,
    pub min_plus_inf: f32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MappingPlanCompileError {
    #[error("structured field grid {field_grid} does not match theater frame {frame_width}x{frame_height}")]
    StructuredFieldTheaterMismatch {
        field_grid: u32,
        frame_width: u32,
        frame_height: u32,
    },
    #[error("w compose grid {w_width}x{w_height} does not match theater frame {frame_width}x{frame_height}")]
    WComposeTheaterMismatch {
        w_width: u32,
        w_height: u32,
        frame_width: u32,
        frame_height: u32,
    },
    #[error("min-plus destination ({col},{row}) is outside theater frame {width}x{height}")]
    MinPlusDestOutOfFrame {
        col: u32,
        row: u32,
        width: u32,
        height: u32,
    },
    #[error("w compose profile index {index} out of range (profiles={profiles})")]
    InvalidWComposeProfileIndex { index: usize, profiles: usize },
    #[error("structured_hops must be > 0")]
    ZeroStructuredHops,
    #[error("min_plus_iterations must be > 0")]
    ZeroMinPlusIterations,
    #[error("interleaved_n_dims required when structured_to_interleaved_writes is non-empty")]
    MissingInterleavedDims,
}

/// Compile a generic sim mapping plan from an admitted structural theater and admitted specs.
pub fn compile_mapping_plan_from_admitted_theater(
    theater: &CompiledStructuralN4Theater,
    spec: MappingPlanCompileSpec,
) -> Result<CompiledMappingPlan, MappingPlanCompileError> {
    validate_mapping_plan_compile_spec(theater, &spec)?;

    let structured_config = compiled_stencil_to_gpu_config(&spec.structured_field.stencil);
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&spec.w_compose);
    let min_plus_config = composed_w_min_plus_stencil_config(
        &w_gpu,
        spec.min_plus_profile_index,
        spec.min_plus_d_col,
        (spec.min_plus_dest.col(), spec.min_plus_dest.row()),
        spec.min_plus_inf,
    );

    Ok(CompiledMappingPlan {
        interleaved_width: theater.frame_width,
        interleaved_height: theater.frame_height,
        interleaved_n_dims: spec.w_compose.n_dims,
        steps: vec![
            CompiledMappingStep::StructuredFieldStencil {
                config: structured_config,
                hops: spec.structured_hops,
                interleaved_column_writes: spec.structured_to_interleaved_writes,
            },
            CompiledMappingStep::WImpedanceCompose { config: w_gpu },
            CompiledMappingStep::MinPlusStencil {
                config: min_plus_config,
                iterations: spec.min_plus_iterations,
            },
        ],
    })
}

/// Compile a structured-field-only generic sim mapping plan.
pub fn compile_structured_field_mapping_plan(
    theater: &CompiledStructuralN4Theater,
    structured_field: &CompiledRegionFieldPreview,
    structured_hops: u32,
    interleaved_column_writes: Vec<(u32, u32)>,
    interleaved_n_dims: u32,
) -> Result<CompiledMappingPlan, MappingPlanCompileError> {
    if structured_hops == 0 {
        return Err(MappingPlanCompileError::ZeroStructuredHops);
    }
    validate_structured_field_theater(theater, structured_field)?;
    if !interleaved_column_writes.is_empty() && interleaved_n_dims == 0 {
        return Err(MappingPlanCompileError::MissingInterleavedDims);
    }

    Ok(CompiledMappingPlan {
        interleaved_width: if interleaved_column_writes.is_empty() {
            0
        } else {
            theater.frame_width
        },
        interleaved_height: if interleaved_column_writes.is_empty() {
            0
        } else {
            theater.frame_height
        },
        interleaved_n_dims,
        steps: vec![CompiledMappingStep::StructuredFieldStencil {
            config: compiled_stencil_to_gpu_config(&structured_field.stencil),
            hops: structured_hops,
            interleaved_column_writes,
        }],
    })
}

fn validate_mapping_plan_compile_spec(
    theater: &CompiledStructuralN4Theater,
    spec: &MappingPlanCompileSpec,
) -> Result<(), MappingPlanCompileError> {
    if spec.structured_hops == 0 {
        return Err(MappingPlanCompileError::ZeroStructuredHops);
    }
    if spec.min_plus_iterations == 0 {
        return Err(MappingPlanCompileError::ZeroMinPlusIterations);
    }
    validate_structured_field_theater(theater, &spec.structured_field)?;
    if spec.w_compose.width != theater.frame_width || spec.w_compose.height != theater.frame_height
    {
        return Err(MappingPlanCompileError::WComposeTheaterMismatch {
            w_width: spec.w_compose.width,
            w_height: spec.w_compose.height,
            frame_width: theater.frame_width,
            frame_height: theater.frame_height,
        });
    }
    if spec.min_plus_profile_index >= spec.w_compose.profiles.len() {
        return Err(MappingPlanCompileError::InvalidWComposeProfileIndex {
            index: spec.min_plus_profile_index,
            profiles: spec.w_compose.profiles.len(),
        });
    }
    if spec.min_plus_dest.col() >= theater.frame_width
        || spec.min_plus_dest.row() >= theater.frame_height
    {
        return Err(MappingPlanCompileError::MinPlusDestOutOfFrame {
            col: spec.min_plus_dest.col(),
            row: spec.min_plus_dest.row(),
            width: theater.frame_width,
            height: theater.frame_height,
        });
    }
    Ok(())
}

fn validate_structured_field_theater(
    theater: &CompiledStructuralN4Theater,
    structured_field: &CompiledRegionFieldPreview,
) -> Result<(), MappingPlanCompileError> {
    let grid = structured_field.grid_size;
    if grid != theater.frame_width || grid != theater.frame_height {
        return Err(MappingPlanCompileError::StructuredFieldTheaterMismatch {
            field_grid: grid,
            frame_width: theater.frame_width,
            frame_height: theater.frame_height,
        });
    }
    Ok(())
}
