//! C-8a persistent GPU EML node/range program table.

use std::collections::HashMap;

use simthing_core::{
    eml_nodes::execution_class_to_u32, EmlExecutionClass, EmlFormulaMeta, EmlNodeGpu, EmlTreeId,
};
use wgpu::{Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor};

use crate::context::GpuContext;

use super::types::EmlTreeRangeGpu;

pub const DEFAULT_EML_TREE_CAPACITY: u32 = 64;
pub const DEFAULT_EML_NODE_CAPACITY: u32 = 2048;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum EmlUploadError {
    #[error("EML node capacity exceeded: need {need}, have {have}")]
    NodeCapacityExceeded { need: u32, have: u32 },
    #[error("EML tree capacity exceeded: need {need}, have {have}")]
    TreeCapacityExceeded { need: u32, have: u32 },
    #[error("EML formula {tree_id:?} is CpuOracleOnly and cannot upload to GPU")]
    CpuOracleOnly { tree_id: EmlTreeId },
    #[error("EML formula {tree_id:?} meta node_count {meta_count} does not match nodes.len() {actual_count}")]
    NodeCountMismatch {
        tree_id: EmlTreeId,
        meta_count: u32,
        actual_count: u32,
    },
}

/// Persistent GPU-resident EML program storage (shared across AccumulatorOp sessions).
pub struct EmlGpuProgramTable {
    node_buffer: Buffer,
    range_buffer: Buffer,
    pub generation: u64,
    pub ranges: Vec<EmlTreeRangeGpu>,
    pub node_capacity: u32,
    pub range_capacity: u32,
    pub node_used: u32,
    pub range_used: u32,
    pub node_upload_count: u64,
    pub range_upload_count: u64,
    /// Registry generation last reflected in this GPU table (boundary-sync skip gate).
    pub uploaded_registry_generation: Option<u64>,
}

impl EmlGpuProgramTable {
    pub fn new(ctx: &GpuContext, node_capacity: u32, range_capacity: u32) -> Self {
        let node_buffer = mk_storage_buffer(
            &ctx.device,
            "eml_node_buffer",
            node_capacity,
            std::mem::size_of::<EmlNodeGpu>() as u64,
        );
        let range_buffer = mk_storage_buffer(
            &ctx.device,
            "eml_range_buffer",
            range_capacity,
            std::mem::size_of::<EmlTreeRangeGpu>() as u64,
        );
        Self {
            node_buffer,
            range_buffer,
            generation: 1,
            ranges: Vec::new(),
            node_capacity,
            range_capacity,
            node_used: 0,
            range_used: 0,
            node_upload_count: 0,
            range_upload_count: 0,
            uploaded_registry_generation: None,
        }
    }

    pub(crate) fn bind_buffers(&self) -> (&Buffer, &Buffer) {
        (&self.node_buffer, &self.range_buffer)
    }

    /// Total GPU upload operations performed on this table (node buffer writes).
    pub fn upload_count(&self) -> u64 {
        self.node_upload_count
    }

    pub fn ensure_capacity(
        &mut self,
        ctx: &GpuContext,
        required_nodes: u32,
        required_ranges: u32,
    ) -> Result<(), EmlUploadError> {
        let mut new_node_cap = self.node_capacity;
        let mut new_range_cap = self.range_capacity;
        while new_node_cap < required_nodes {
            new_node_cap = new_node_cap.saturating_mul(2);
        }
        while new_range_cap < required_ranges {
            new_range_cap = new_range_cap.saturating_mul(2);
        }
        if new_node_cap == self.node_capacity && new_range_cap == self.range_capacity {
            return Ok(());
        }
        self.grow_buffers(ctx, new_node_cap, new_range_cap)?;
        Ok(())
    }

    fn grow_buffers(
        &mut self,
        ctx: &GpuContext,
        new_node_cap: u32,
        new_range_cap: u32,
    ) -> Result<(), EmlUploadError> {
        let old_node_used = self.node_used;
        let old_range_used = self.range_used;
        let old_nodes = std::mem::replace(
            &mut self.node_buffer,
            mk_storage_buffer(
                &ctx.device,
                "eml_node_buffer",
                new_node_cap,
                std::mem::size_of::<EmlNodeGpu>() as u64,
            ),
        );
        let old_ranges = std::mem::replace(
            &mut self.range_buffer,
            mk_storage_buffer(
                &ctx.device,
                "eml_range_buffer",
                new_range_cap,
                std::mem::size_of::<EmlTreeRangeGpu>() as u64,
            ),
        );
        self.node_capacity = new_node_cap;
        self.range_capacity = new_range_cap;
        self.generation = self.generation.wrapping_add(1);

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("eml_grow_copy"),
            });
        if old_node_used > 0 {
            encoder.copy_buffer_to_buffer(
                &old_nodes,
                0,
                &self.node_buffer,
                0,
                old_node_used as u64 * std::mem::size_of::<EmlNodeGpu>() as u64,
            );
        }
        if old_range_used > 0 {
            encoder.copy_buffer_to_buffer(
                &old_ranges,
                0,
                &self.range_buffer,
                0,
                old_range_used as u64 * std::mem::size_of::<EmlTreeRangeGpu>() as u64,
            );
        }
        ctx.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    /// Upload formula node programs. Returns stable `tree_id → range_index` mapping.
    pub fn upload_trees(
        &mut self,
        ctx: &GpuContext,
        trees: &[(EmlTreeId, EmlFormulaMeta, Vec<EmlNodeGpu>)],
    ) -> Result<HashMap<EmlTreeId, u32>, EmlUploadError> {
        if trees.is_empty() {
            if self.node_used != 0 || self.range_used != 0 || !self.ranges.is_empty() {
                self.generation = self.generation.wrapping_add(1);
            }
            self.ranges.clear();
            self.node_used = 0;
            self.range_used = 0;
            return Ok(HashMap::new());
        }

        let total_nodes: u32 = trees.iter().map(|(_, _, nodes)| nodes.len() as u32).sum();
        let total_ranges = trees.len() as u32;
        self.ensure_capacity(ctx, total_nodes, total_ranges)?;

        let mut flat_nodes = Vec::with_capacity(total_nodes as usize);
        let mut ranges = Vec::with_capacity(trees.len());
        let mut mapping = HashMap::new();

        let mut node_offset = 0u32;
        for (range_index, (tree_id, meta, nodes)) in trees.iter().enumerate() {
            if meta.execution_class == EmlExecutionClass::CpuOracleOnly {
                return Err(EmlUploadError::CpuOracleOnly { tree_id: *tree_id });
            }
            if meta.node_count != nodes.len() as u32 {
                return Err(EmlUploadError::NodeCountMismatch {
                    tree_id: *tree_id,
                    meta_count: meta.node_count,
                    actual_count: nodes.len() as u32,
                });
            }
            let range = EmlTreeRangeGpu {
                node_offset,
                node_count: nodes.len() as u32,
                execution_class: execution_class_to_u32(meta.execution_class),
                flags: 0,
            };
            flat_nodes.extend_from_slice(nodes);
            node_offset += nodes.len() as u32;
            ranges.push(range);
            mapping.insert(*tree_id, range_index as u32);
        }

        ctx.queue
            .write_buffer(&self.node_buffer, 0, bytemuck::cast_slice(&flat_nodes));
        ctx.queue
            .write_buffer(&self.range_buffer, 0, bytemuck::cast_slice(&ranges));

        self.node_upload_count += 1;
        self.range_upload_count += 1;
        self.node_used = flat_nodes.len() as u32;
        self.range_used = total_ranges;
        self.ranges = ranges;
        self.generation = self.generation.wrapping_add(1);
        Ok(mapping)
    }
}

fn mk_storage_buffer(device: &wgpu::Device, label: &str, count: u32, elem_size: u64) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some(label),
        size: (count as u64 * elem_size).max(elem_size),
        usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{eml_opcode, EmlExecutionClass};

    fn literal(v: f32) -> EmlNodeGpu {
        EmlNodeGpu {
            opcode: eml_opcode::LITERAL_F32,
            flags: 0,
            a: v.to_bits(),
            b: 0,
            c: 0,
            d: 0,
        }
    }

    fn meta(id: u32, n: u32) -> EmlFormulaMeta {
        EmlFormulaMeta {
            tree_id: EmlTreeId(id),
            execution_class: EmlExecutionClass::ExactDeterministic,
            allowed_consumers: Default::default(),
            max_abs_error: None,
            deterministic_gpu: true,
            requires_guard_for_hard_threshold: false,
            node_count: n,
            max_stack_depth: 1,
            has_loops: false,
            has_recursion: false,
            display_name: "test".into(),
        }
    }

    #[test]
    fn upload_roundtrip_mapping() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = EmlGpuProgramTable::new(&ctx, 32, 8);
        let trees = vec![(
            EmlTreeId(1),
            meta(1, 3),
            vec![
                literal(2.0),
                literal(3.0),
                EmlNodeGpu {
                    opcode: eml_opcode::ADD,
                    flags: 0,
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
            ],
        )];
        let map = table.upload_trees(&ctx, &trees).unwrap();
        assert_eq!(map.get(&EmlTreeId(1)), Some(&0));
        assert_eq!(table.range_used, 1);
        assert_eq!(table.node_used, 3);
    }

    #[test]
    fn c8a_empty_upload_after_nonempty_table_bumps_generation() {
        let ctx = GpuContext::new_blocking().expect("gpu");
        let mut table = EmlGpuProgramTable::new(&ctx, 32, 8);
        let trees = vec![(
            EmlTreeId(1),
            meta(1, 2),
            vec![
                literal(1.0),
                EmlNodeGpu {
                    opcode: eml_opcode::RETURN_TOP,
                    flags: 0,
                    a: 0,
                    b: 0,
                    c: 0,
                    d: 0,
                },
            ],
        )];
        table.upload_trees(&ctx, &trees).unwrap();
        let gen_after_upload = table.generation;
        table.upload_trees(&ctx, &[]).unwrap();
        assert!(table.generation > gen_after_upload);
        assert_eq!(table.node_used, 0);
        assert_eq!(table.range_used, 0);
    }
}
