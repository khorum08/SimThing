//! Reusable facility-local GPU Current/Next plane discipline (7.6a).
//!
//! One [`FacilityPlaneGenerationBoundary`] may advance any number of planes,
//! but every plane keeps distinct buffers and an unforgeable owner capability.
//! The boundary is the sole swap authority: facilities can bind their own
//! Current read-only and Next read-write surfaces, never another facility's.

use std::sync::atomic::{AtomicU64, Ordering};

use bytemuck::Pod;
use thiserror::Error;
use wgpu::util::DeviceExt;

use crate::GpuContext;

static NEXT_BOUNDARY_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_OWNER_ID: AtomicU64 = AtomicU64::new(1);

/// Opaque capability for exactly one facility's resident plane.
///
/// It is intentionally neither `Clone` nor constructible outside this module.
pub struct FacilityPlaneOwner {
    boundary_id: u64,
    owner_id: u64,
}

/// The one generation-boundary authority for a set of facility-local planes.
pub struct FacilityPlaneGenerationBoundary {
    boundary_id: u64,
    generation: u32,
}

impl FacilityPlaneGenerationBoundary {
    pub fn new() -> Self {
        Self {
            boundary_id: NEXT_BOUNDARY_ID.fetch_add(1, Ordering::Relaxed),
            generation: 0,
        }
    }

    /// Admit one facility under this boundary. The returned owner capability
    /// cannot authorize access to any sibling facility plane.
    pub fn admit_facility(&self) -> FacilityPlaneOwner {
        FacilityPlaneOwner {
            boundary_id: self.boundary_id,
            owner_id: NEXT_OWNER_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Advance all supplied facility planes at one generation boundary.
    ///
    /// A plane from another boundary and duplicate presentation of one plane
    /// both fail before any swap occurs. This makes a second swap authority or
    /// double-swap attempt observationally inert rather than partially applied.
    pub fn advance(
        &mut self,
        planes: &mut [(&FacilityPlaneOwner, &mut FacilityResidentPlane)],
    ) -> Result<u32, FacilityPlaneError> {
        for (index, (owner, plane)) in planes.iter().enumerate() {
            plane.validate_owner(owner)?;
            if plane.boundary_id != self.boundary_id {
                return Err(FacilityPlaneError::ForeignSwapAuthority);
            }
            if planes[..index]
                .iter()
                .any(|(_, prior)| prior.owner_id == plane.owner_id)
            {
                return Err(FacilityPlaneError::DuplicatePlaneAdvance);
            }
        }
        let next = self
            .generation
            .checked_add(1)
            .ok_or(FacilityPlaneError::GenerationOverflow)?;
        for (_, plane) in planes.iter_mut() {
            std::mem::swap(&mut plane.current, &mut plane.next);
            plane.generation = next;
        }
        self.generation = next;
        Ok(next)
    }
}

impl Default for FacilityPlaneGenerationBoundary {
    fn default() -> Self {
        Self::new()
    }
}

/// One facility's physically distinct GPU-resident Current/Next plane.
pub struct FacilityResidentPlane {
    boundary_id: u64,
    owner_id: u64,
    current: wgpu::Buffer,
    next: wgpu::Buffer,
    rows: usize,
    row_bytes: usize,
    generation: u32,
}

impl FacilityResidentPlane {
    pub fn from_rows<T: Pod>(
        ctx: &GpuContext,
        label: &str,
        boundary: &FacilityPlaneGenerationBoundary,
        owner: &FacilityPlaneOwner,
        rows: &[T],
    ) -> Result<Self, FacilityPlaneError> {
        if owner.boundary_id != boundary.boundary_id {
            return Err(FacilityPlaneError::ForeignSwapAuthority);
        }
        if rows.is_empty() {
            return Err(FacilityPlaneError::EmptyPlane);
        }
        let bytes = bytemuck::cast_slice(rows);
        let usage = wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::COPY_DST;
        let make = |suffix: &str| {
            ctx.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{label}_{suffix}")),
                    contents: bytes,
                    usage,
                })
        };
        Ok(Self {
            boundary_id: boundary.boundary_id,
            owner_id: owner.owner_id,
            current: make("current"),
            next: make("next"),
            rows: rows.len(),
            row_bytes: std::mem::size_of::<T>(),
            generation: boundary.generation,
        })
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn bytes_per_plane(&self) -> usize {
        self.rows * self.row_bytes
    }

    pub fn carry_bytes(&self) -> usize {
        self.bytes_per_plane()
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn validate_owner(&self, owner: &FacilityPlaneOwner) -> Result<(), FacilityPlaneError> {
        if owner.boundary_id != self.boundary_id || owner.owner_id != self.owner_id {
            return Err(FacilityPlaneError::ForeignPlaneWrite);
        }
        Ok(())
    }

    pub(crate) fn current_for(
        &self,
        owner: &FacilityPlaneOwner,
    ) -> Result<&wgpu::Buffer, FacilityPlaneError> {
        self.validate_owner(owner)?;
        Ok(&self.current)
    }

    pub(crate) fn next_for(
        &self,
        owner: &FacilityPlaneOwner,
    ) -> Result<&wgpu::Buffer, FacilityPlaneError> {
        self.validate_owner(owner)?;
        Ok(&self.next)
    }

    /// Encode the whole-plane Current → Next carry used at the ordinary
    /// generation boundary. No gather, comparator, or semantic evaluation is
    /// introduced by this primitive.
    pub(crate) fn encode_carry(
        &self,
        owner: &FacilityPlaneOwner,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), FacilityPlaneError> {
        let current = self.current_for(owner)?;
        let next = self.next_for(owner)?;
        encoder.copy_buffer_to_buffer(current, 0, next, 0, self.carry_bytes() as u64);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum FacilityPlaneError {
    #[error("facility attempted to write or bind another facility's resident plane")]
    ForeignPlaneWrite,
    #[error("a second generation-boundary authority attempted to swap a resident plane")]
    ForeignSwapAuthority,
    #[error("the same resident plane was presented twice at one generation boundary")]
    DuplicatePlaneAdvance,
    #[error("facility resident-plane generation overflow")]
    GenerationOverflow,
    #[error("facility resident plane requires at least one admitted row")]
    EmptyPlane,
}
