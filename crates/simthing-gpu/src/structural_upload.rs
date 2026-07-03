//! GPU-resident structural upload buffers.
//!
//! These buffers are projection/cache over scenario-derived structural rows — not model authority.
//!
//! Public upload consumes only [`PackedUpload`]; free row bundles and typed indices must not cross
//! the upload seam (see module doc compile_fail proofs).
//!
//! ```compile_fail
//! use simthing_gpu::PackedUpload;
//!
//! fn packed_upload_fields_private_compile_fail() {
//!     let _ = PackedUpload {
//!         frame: Default::default(),
//!         locations: vec![],
//!         links: vec![],
//!     };
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_gpu::{
//!     upload_structural_rows_to_gpu, StructuralFrameGpuRow, StructuralLinkGpuRow,
//!     StructuralLocationGpuRow,
//! };
//!
//! fn upload_rejects_free_structural_rows_compile_fail(
//!     device: &wgpu::Device,
//!     queue: &wgpu::Queue,
//!     frame: StructuralFrameGpuRow,
//!     locations: &[StructuralLocationGpuRow],
//!     links: &[StructuralLinkGpuRow],
//! ) {
//!     let _ = upload_structural_rows_to_gpu(device, queue, frame, locations, links);
//! }
//! ```
//!
//! ```compile_fail
//! use simthing_core::{ColumnIndex, SlotIndex};
//! use simthing_gpu::upload_structural_rows_to_gpu;
//!
//! fn upload_rejects_semantic_slot_column_arguments_compile_fail(
//!     device: &wgpu::Device,
//!     queue: &wgpu::Queue,
//!     slot: SlotIndex,
//!     col: ColumnIndex,
//! ) {
//!     let _ = upload_structural_rows_to_gpu(device, queue, slot, col);
//! }
//! ```

use bytemuck::{Pod, Zeroable};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device, Queue};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Pod, Zeroable)]
pub struct StructuralFrameGpuRow {
    pub width: u32,
    pub height: u32,
    pub occupied_cells: u32,
    pub location_count: u32,
    pub link_count: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct StructuralLocationGpuRow {
    pub dense_index: u32,
    pub simthing_id_raw: u32,
    pub system_id: u32,
    pub row: u32,
    pub col: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct StructuralLinkGpuRow {
    pub from_dense_index: u32,
    pub to_dense_index: u32,
    pub reserved0: u32,
    pub reserved1: u32,
}

/// Pre-pack row bundle for validation paths; upload itself requires [`PackedUpload`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralUploadRows {
    pub frame: StructuralFrameGpuRow,
    pub locations: Vec<StructuralLocationGpuRow>,
    pub links: Vec<StructuralLinkGpuRow>,
}

/// Validated, byte-ready structural upload packet. Only this type crosses the public GPU upload seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedUpload {
    frame: StructuralFrameGpuRow,
    locations: Vec<StructuralLocationGpuRow>,
    links: Vec<StructuralLinkGpuRow>,
}

impl PackedUpload {
    pub fn new(
        frame: StructuralFrameGpuRow,
        locations: Vec<StructuralLocationGpuRow>,
        links: Vec<StructuralLinkGpuRow>,
    ) -> Result<Self, StructuralUploadError> {
        if locations.is_empty() {
            return Err(StructuralUploadError::EmptyLocationRows);
        }
        if locations.len() != frame.location_count as usize {
            return Err(StructuralUploadError::LocationCountMismatch);
        }
        if links.len() != frame.link_count as usize {
            return Err(StructuralUploadError::LinkCountMismatch);
        }
        Ok(Self {
            frame,
            locations,
            links,
        })
    }

    pub fn frame(&self) -> StructuralFrameGpuRow {
        self.frame
    }

    pub fn locations(&self) -> &[StructuralLocationGpuRow] {
        &self.locations
    }

    pub fn links(&self) -> &[StructuralLinkGpuRow] {
        &self.links
    }
}

impl TryFrom<StructuralUploadRows> for PackedUpload {
    type Error = StructuralUploadError;

    fn try_from(rows: StructuralUploadRows) -> Result<Self, Self::Error> {
        Self::new(rows.frame, rows.locations, rows.links)
    }
}

impl TryFrom<&StructuralUploadRows> for PackedUpload {
    type Error = StructuralUploadError;

    fn try_from(rows: &StructuralUploadRows) -> Result<Self, Self::Error> {
        Self::new(rows.frame, rows.locations.clone(), rows.links.clone())
    }
}

pub struct StructuralUploadGpuBuffers {
    pub frame_buffer: Buffer,
    pub location_buffer: Buffer,
    pub link_buffer: Buffer,
    pub location_count: u32,
    pub link_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralUploadGpuReport {
    pub frame_bytes: u64,
    pub location_bytes: u64,
    pub link_bytes: u64,
    pub location_count: u32,
    pub link_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralUploadReadback {
    pub frame_bytes: Vec<u8>,
    pub location_bytes: Vec<u8>,
    pub link_bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StructuralUploadError {
    #[error("count overflow: {field}={value}")]
    CountOverflow { field: &'static str, value: u64 },
    #[error("structural upload requires at least one location row")]
    EmptyLocationRows,
    #[error("byte size overflow: {field}={bytes}")]
    ByteSizeOverflow { field: &'static str, bytes: u64 },
    #[error("frame location_count does not match location row slice")]
    LocationCountMismatch,
    #[error("frame link_count does not match link row slice")]
    LinkCountMismatch,
    #[error("GPU buffer map_async failed")]
    MapAsyncFailed,
    #[error("GPU readback failed for {buffer}: {reason}")]
    ReadbackFailed {
        buffer: &'static str,
        reason: String,
    },
}

pub const FRAME_ROW_BYTES: u64 = std::mem::size_of::<StructuralFrameGpuRow>() as u64;
pub const LOCATION_ROW_BYTES: u64 = std::mem::size_of::<StructuralLocationGpuRow>() as u64;
pub const LINK_ROW_BYTES: u64 = std::mem::size_of::<StructuralLinkGpuRow>() as u64;

fn checked_row_bytes(
    count: usize,
    row_bytes: u64,
    field: &'static str,
) -> Result<u64, StructuralUploadError> {
    let count_u64 = u64::try_from(count).map_err(|_| StructuralUploadError::CountOverflow {
        field,
        value: count as u64,
    })?;
    count_u64
        .checked_mul(row_bytes)
        .ok_or(StructuralUploadError::ByteSizeOverflow {
            field,
            bytes: count_u64.saturating_mul(row_bytes),
        })
}

fn buffer_size_for_rows(
    count: usize,
    row_bytes: u64,
    field: &'static str,
) -> Result<u64, StructuralUploadError> {
    if count == 0 {
        Ok(row_bytes)
    } else {
        checked_row_bytes(count, row_bytes, field)
    }
}

pub fn upload_structural_rows_to_gpu(
    device: &Device,
    queue: &Queue,
    upload: &PackedUpload,
) -> Result<(StructuralUploadGpuBuffers, StructuralUploadGpuReport), StructuralUploadError> {
    let frame = upload.frame();
    let locations = upload.locations();
    let links = upload.links();

    let frame_bytes = FRAME_ROW_BYTES;
    let location_bytes = checked_row_bytes(locations.len(), LOCATION_ROW_BYTES, "location_bytes")?;
    let link_bytes = buffer_size_for_rows(links.len(), LINK_ROW_BYTES, "link_bytes")?;

    let frame_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_upload_frame"),
        contents: bytemuck::bytes_of(&frame),
        usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });

    let location_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("structural_upload_locations"),
        contents: bytemuck::cast_slice(locations),
        usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
    });

    let link_buffer = if links.is_empty() {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("structural_upload_links_empty"),
            size: link_bytes,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
            mapped_at_creation: false,
        })
    } else {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("structural_upload_links"),
            contents: bytemuck::cast_slice(links),
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        })
    };

    let _ = queue;

    Ok((
        StructuralUploadGpuBuffers {
            frame_buffer,
            location_buffer,
            link_buffer,
            location_count: frame.location_count,
            link_count: frame.link_count,
        },
        StructuralUploadGpuReport {
            frame_bytes,
            location_bytes,
            link_bytes,
            location_count: frame.location_count,
            link_count: frame.link_count,
        },
    ))
}

pub fn readback_buffer_bytes_blocking(
    device: &Device,
    queue: &Queue,
    src: &Buffer,
    byte_len: u64,
    buffer: &'static str,
) -> Result<Vec<u8>, StructuralUploadError> {
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("structural_upload_readback_staging"),
        size: byte_len,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("structural_upload_readback_encoder"),
    });
    encoder.copy_buffer_to_buffer(src, 0, &staging, 0, byte_len);
    queue.submit(Some(encoder.finish()));
    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device.poll(wgpu::Maintain::Wait);
    match rx.recv() {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            return Err(StructuralUploadError::ReadbackFailed {
                buffer,
                reason: format!("map_async: {err:?}"),
            });
        }
        Err(_) => return Err(StructuralUploadError::MapAsyncFailed),
    }
    let data = slice.get_mapped_range();
    let out = data.to_vec();
    drop(data);
    staging.unmap();
    Ok(out)
}

pub fn readback_pod_blocking<T: Pod>(
    device: &Device,
    queue: &Queue,
    src: &Buffer,
    buffer: &'static str,
) -> Result<T, StructuralUploadError> {
    let bytes = readback_buffer_bytes_blocking(
        device,
        queue,
        src,
        std::mem::size_of::<T>() as u64,
        buffer,
    )?;
    if bytes.len() != std::mem::size_of::<T>() {
        return Err(StructuralUploadError::ReadbackFailed {
            buffer,
            reason: format!(
                "expected {} bytes, got {}",
                std::mem::size_of::<T>(),
                bytes.len()
            ),
        });
    }
    Ok(*bytemuck::from_bytes::<T>(&bytes))
}

pub fn readback_structural_upload_blocking(
    device: &Device,
    queue: &Queue,
    buffers: &StructuralUploadGpuBuffers,
    report: &StructuralUploadGpuReport,
) -> Result<StructuralUploadReadback, StructuralUploadError> {
    Ok(StructuralUploadReadback {
        frame_bytes: readback_buffer_bytes_blocking(
            device,
            queue,
            &buffers.frame_buffer,
            report.frame_bytes,
            "frame_buffer",
        )?,
        location_bytes: readback_buffer_bytes_blocking(
            device,
            queue,
            &buffers.location_buffer,
            report.location_bytes,
            "location_buffer",
        )?,
        link_bytes: readback_buffer_bytes_blocking(
            device,
            queue,
            &buffers.link_buffer,
            report.link_bytes,
            "link_buffer",
        )?,
    })
}

pub fn source_row_bytes<T: Pod>(rows: &[T]) -> Vec<u8> {
    bytemuck::cast_slice(rows).to_vec()
}

pub fn readback_matches_source(readback: &StructuralUploadReadback, upload: &PackedUpload) -> bool {
    readback.frame_bytes == bytemuck::bytes_of(&upload.frame()).to_vec()
        && readback.location_bytes == source_row_bytes(upload.locations())
        && (upload.links().is_empty() || readback.link_bytes == source_row_bytes(upload.links()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GpuContext;

    #[test]
    fn structural_gpu_rows_are_repr_c_stable() {
        assert_eq!(std::mem::align_of::<StructuralFrameGpuRow>(), 4);
        assert_eq!(std::mem::align_of::<StructuralLocationGpuRow>(), 4);
        assert_eq!(std::mem::align_of::<StructuralLinkGpuRow>(), 4);
    }

    #[test]
    fn structural_gpu_frame_row_size_is_32() {
        assert_eq!(std::mem::size_of::<StructuralFrameGpuRow>(), 32);
    }

    #[test]
    fn structural_gpu_location_row_size_is_32() {
        assert_eq!(std::mem::size_of::<StructuralLocationGpuRow>(), 32);
    }

    #[test]
    fn structural_gpu_link_row_size_is_16() {
        assert_eq!(std::mem::size_of::<StructuralLinkGpuRow>(), 16);
    }

    fn sample_rows() -> (
        StructuralFrameGpuRow,
        Vec<StructuralLocationGpuRow>,
        Vec<StructuralLinkGpuRow>,
    ) {
        let frame = StructuralFrameGpuRow {
            width: 8,
            height: 8,
            occupied_cells: 2,
            location_count: 2,
            link_count: 1,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
        };
        let locations = vec![
            StructuralLocationGpuRow {
                dense_index: 0,
                simthing_id_raw: 10,
                system_id: 1,
                row: 2,
                col: 3,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
            StructuralLocationGpuRow {
                dense_index: 1,
                simthing_id_raw: 11,
                system_id: 2,
                row: 2,
                col: 4,
                reserved0: 0,
                reserved1: 0,
                reserved2: 0,
            },
        ];
        let links = vec![StructuralLinkGpuRow {
            from_dense_index: 0,
            to_dense_index: 1,
            reserved0: 0,
            reserved1: 0,
        }];
        (frame, locations, links)
    }

    fn sample_packed_upload() -> PackedUpload {
        let (frame, locations, links) = sample_rows();
        PackedUpload::new(frame, locations, links).expect("sample packet")
    }

    #[test]
    fn packed_upload_rejects_link_count_mismatch() {
        let (frame, locations, links) = sample_rows();
        let bad_frame = StructuralFrameGpuRow {
            link_count: 99,
            ..frame
        };
        let err = PackedUpload::new(bad_frame, locations, links).expect_err("mismatch");
        assert!(matches!(err, StructuralUploadError::LinkCountMismatch));
    }

    #[test]
    fn structural_upload_rejects_count_overflow() {
        let frame = StructuralFrameGpuRow {
            width: 1,
            height: 1,
            occupied_cells: 1,
            location_count: u32::MAX,
            link_count: 0,
            reserved0: 0,
            reserved1: 0,
            reserved2: 0,
        };
        let err = checked_row_bytes(usize::MAX, LOCATION_ROW_BYTES, "location_bytes")
            .expect_err("overflow");
        assert!(matches!(
            err,
            StructuralUploadError::CountOverflow { .. }
                | StructuralUploadError::ByteSizeOverflow { .. }
        ));
        let _ = frame;
    }

    #[test]
    fn packed_upload_public_api_preserves_prior_bytes() {
        let upload = sample_packed_upload();
        assert_eq!(upload.frame().location_count, 2);
        assert_eq!(upload.locations().len(), 2);
        assert_eq!(upload.links().len(), 1);
        assert_eq!(
            source_row_bytes(upload.locations()),
            source_row_bytes(&sample_rows().1)
        );
    }

    #[test]
    fn structural_upload_allocates_gpu_buffers_from_packed_upload() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let upload = sample_packed_upload();
        let (buffers, report) =
            upload_structural_rows_to_gpu(&ctx.device, &ctx.queue, &upload).expect("upload");
        assert_eq!(buffers.location_count, 2);
        assert_eq!(buffers.link_count, 1);
        assert_eq!(report.frame_bytes, FRAME_ROW_BYTES);
        assert_eq!(report.location_bytes, 2 * LOCATION_ROW_BYTES);
        assert_eq!(report.link_bytes, LINK_ROW_BYTES);
    }

    #[test]
    fn structural_upload_reports_expected_byte_sizes_from_packed_upload() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let upload = sample_packed_upload();
        let (_, report) =
            upload_structural_rows_to_gpu(&ctx.device, &ctx.queue, &upload).expect("upload");
        assert_eq!(report.frame_bytes, 32);
        assert_eq!(report.location_bytes, 64);
        assert_eq!(report.link_bytes, 16);
    }

    #[test]
    fn structural_upload_readback_matches_packed_upload_source() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let upload = sample_packed_upload();
        let (buffers, report) =
            upload_structural_rows_to_gpu(&ctx.device, &ctx.queue, &upload).expect("upload");
        let readback =
            readback_structural_upload_blocking(&ctx.device, &ctx.queue, &buffers, &report)
                .expect("readback");
        assert!(readback_matches_source(&readback, &upload));
        assert_eq!(
            readback.frame_bytes,
            bytemuck::bytes_of(&upload.frame()).to_vec()
        );
        assert_eq!(
            readback.location_bytes,
            source_row_bytes(upload.locations())
        );
    }

    #[test]
    fn structural_upload_readback_returns_result() {
        let Some(ctx) = GpuContext::new_blocking().ok() else {
            eprintln!("skipping: no GPU");
            return;
        };
        let upload = sample_packed_upload();
        let (buffers, report) =
            upload_structural_rows_to_gpu(&ctx.device, &ctx.queue, &upload).expect("upload");
        assert!(
            readback_structural_upload_blocking(&ctx.device, &ctx.queue, &buffers, &report).is_ok()
        );
    }

    #[test]
    fn structural_upload_readback_reports_map_failure_if_simulated_or_unit_testable() {
        let err = StructuralUploadError::MapAsyncFailed;
        assert!(matches!(err, StructuralUploadError::MapAsyncFailed));
        let err = StructuralUploadError::ReadbackFailed {
            buffer: "frame_buffer",
            reason: "map_async: simulated".to_string(),
        };
        assert!(matches!(
            err,
            StructuralUploadError::ReadbackFailed {
                buffer: "frame_buffer",
                ..
            }
        ));
    }
}
