//! C-8c persistent GPU input-list table for conjunctive transfer.

use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor};

use crate::context::GpuContext;

use super::types::AccumulatorInputGpu;

pub const DEFAULT_INPUT_LIST_CAPACITY: u32 = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputListRange {
    pub offset: u32,
    pub count: u32,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum InputListUploadError {
    #[error("input-list capacity exceeded: need {need}, have {have}")]
    CapacityExceeded { need: u32, have: u32 },
}

/// Persistent GPU-resident input-list storage for MinAcrossInputs transfer.
pub struct AccumulatorInputListTable {
    pub buffer: Buffer,
    pub entries: Vec<AccumulatorInputGpu>,
    pub generation: u64,
    pub uploaded_generation: Option<u64>,
    pub uploaded_source_generation: Option<u64>,
    pub capacity: u32,
    pub used: u32,
    pub upload_count: u64,
}

impl AccumulatorInputListTable {
    pub fn new(ctx: &GpuContext, capacity: u32) -> Self {
        let buffer = mk_storage_buffer(&ctx.device, "input_list_buffer", capacity);
        Self {
            buffer,
            entries: Vec::new(),
            generation: 1,
            uploaded_generation: None,
            uploaded_source_generation: None,
            capacity,
            used: 0,
            upload_count: 0,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn ensure_capacity(
        &mut self,
        ctx: &GpuContext,
        required_entries: u32,
    ) -> Result<(), InputListUploadError> {
        if required_entries <= self.capacity {
            return Ok(());
        }
        let mut new_cap = self.capacity.max(1);
        while new_cap < required_entries {
            new_cap = new_cap.saturating_mul(2);
        }
        let old_used = self.used;
        let old_buffer = std::mem::replace(
            &mut self.buffer,
            mk_storage_buffer(&ctx.device, "input_list_buffer", new_cap),
        );
        self.capacity = new_cap;
        if old_used > 0 {
            let mut encoder = ctx
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("input_list_grow_copy"),
                });
            let bytes = (old_used as u64) * std::mem::size_of::<AccumulatorInputGpu>() as u64;
            encoder.copy_buffer_to_buffer(&old_buffer, 0, &self.buffer, 0, bytes);
            ctx.queue.submit(Some(encoder.finish()));
        }
        Ok(())
    }

    pub fn upload_lists(
        &mut self,
        ctx: &GpuContext,
        lists: &[Vec<AccumulatorInputGpu>],
        source_generation: u64,
    ) -> Result<Vec<InputListRange>, InputListUploadError> {
        let flat_len: u32 = lists.iter().map(|l| l.len() as u32).sum();
        if flat_len == 0 {
            if self.used != 0 || !self.entries.is_empty() {
                self.generation = self.generation.wrapping_add(1);
            }
            self.entries.clear();
            self.used = 0;
            self.uploaded_generation = Some(self.generation);
            self.uploaded_source_generation = Some(source_generation);
            return Ok(Vec::new());
        }

        let mut flat = Vec::with_capacity(flat_len as usize);
        let mut ranges = Vec::with_capacity(lists.len());
        let mut offset = 0u32;
        for list in lists {
            let count = list.len() as u32;
            ranges.push(InputListRange { offset, count });
            flat.extend_from_slice(list);
            offset += count;
        }

        if self.uploaded_generation == Some(self.generation)
            && self.uploaded_source_generation == Some(source_generation)
            && self.entries == flat
        {
            return Ok(ranges);
        }

        self.ensure_capacity(ctx, flat_len)?;
        ctx.queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&flat));
        self.entries = flat;
        self.used = flat_len;
        self.generation = self.generation.wrapping_add(1);
        self.uploaded_generation = Some(self.generation);
        self.uploaded_source_generation = Some(source_generation);
        self.upload_count += 1;
        Ok(ranges)
    }
}

fn mk_storage_buffer(device: &wgpu::Device, label: &str, capacity: u32) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: (capacity as u64) * std::mem::size_of::<AccumulatorInputGpu>() as u64,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(slot: u32, col: u32, unit_cost: f32) -> AccumulatorInputGpu {
        AccumulatorInputGpu {
            slot,
            col,
            unit_cost_bits: unit_cost.to_bits(),
            flags: 0,
        }
    }

    #[test]
    fn c8c_input_list_table_upload_roundtrip() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = AccumulatorInputListTable::new(&ctx, 16);
        let lists = vec![vec![input(0, 1, 5.0), input(0, 2, 3.0)]];
        let ranges = table.upload_lists(&ctx, &lists, 1).unwrap();
        assert_eq!(
            ranges,
            vec![InputListRange {
                offset: 0,
                count: 2
            }]
        );
        assert_eq!(table.used, 2);
        assert_eq!(table.upload_count, 1);
    }

    #[test]
    fn c8c_input_list_table_skips_unchanged_upload() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = AccumulatorInputListTable::new(&ctx, 16);
        let lists = vec![vec![input(1, 0, 2.0)]];
        table.upload_lists(&ctx, &lists, 1).unwrap();
        let uploads = table.upload_count;
        table.upload_lists(&ctx, &lists, 1).unwrap();
        assert_eq!(table.upload_count, uploads);
    }

    #[test]
    fn c8c_input_list_table_growth_preserves_entries() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = AccumulatorInputListTable::new(&ctx, 2);
        let lists = vec![vec![input(0, 0, 1.0), input(0, 1, 1.0), input(0, 2, 1.0)]];
        table.upload_lists(&ctx, &lists, 1).unwrap();
        assert!(table.capacity >= 3);
        assert_eq!(table.entries.len(), 3);
    }

    #[test]
    fn c8c_input_list_empty_upload_after_nonempty_bumps_generation() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = AccumulatorInputListTable::new(&ctx, 16);
        let lists = vec![vec![input(0, 0, 1.0)]];
        table.upload_lists(&ctx, &lists, 1).unwrap();
        let gen_after_nonempty = table.generation;
        table.upload_lists(&ctx, &[], 2).unwrap();
        assert!(table.generation > gen_after_nonempty);
        assert!(table.entries.is_empty());
        assert_eq!(table.used, 0);
    }
}
