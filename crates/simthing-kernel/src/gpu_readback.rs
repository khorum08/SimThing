//! Kernel-owned GPU readback buffers — sealed records minted in-crate only (KERNEL-CRATE-EXTRACT-0R2).

use bytemuck::Pod;
use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Device, Maintain, MapMode,
    Queue,
};

use crate::sealed::{
    EmissionRecord, EmissionRecordGpu, ThresholdEmission, ThresholdEmissionGpu, ThresholdEvent,
    ThresholdEventGpu,
};

#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum KernelReadbackError {
    #[error("emission buffer overflow: count={count}, capacity={capacity}")]
    EmissionOverflow { count: u32, capacity: u32 },
    #[error("threshold emission buffer overflow: count={count}, capacity={capacity}")]
    ThresholdEmissionOverflow { count: u32, capacity: u32 },
}

fn read_buffer_bytes_range(
    device: &Device,
    queue: &Queue,
    buf: &Buffer,
    offset: u64,
    size: u64,
) -> Vec<u8> {
    let staging = device.create_buffer(&BufferDescriptor {
        label: Some("kernel_readback_staging"),
        size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("kernel_readback_encoder"),
    });
    encoder.copy_buffer_to_buffer(buf, offset, &staging, 0, size);
    queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    slice.map_async(MapMode::Read, |_| {});
    device.poll(Maintain::Wait);
    let mapped = slice.get_mapped_range();
    mapped.to_vec()
}

fn read_u32_counter(device: &Device, queue: &Queue, count_buf: &Buffer) -> u32 {
    let bytes = read_buffer_bytes_range(device, queue, count_buf, 0, 4);
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn mint_from_gpu_pod<T: Pod, S>(
    device: &Device,
    queue: &Queue,
    records_buf: &Buffer,
    count: u32,
    record_size: u64,
    mint: impl Fn(&T) -> S,
) -> Vec<S> {
    if count == 0 {
        return Vec::new();
    }
    let used = (count as u64) * record_size;
    let bytes = read_buffer_bytes_range(device, queue, records_buf, 0, used);
    let gpu: &[T] = bytemuck::cast_slice(&bytes);
    gpu.iter().map(mint).collect()
}

/// Kernel-owned EmitEvent compact record buffer + counter (B-2).
pub struct EmissionRecordReadback {
    records: Buffer,
    count: Buffer,
    capacity: u32,
}

impl EmissionRecordReadback {
    pub fn new(device: &Device, capacity: u32) -> Self {
        let record_len = (capacity as u64) * std::mem::size_of::<EmissionRecordGpu>() as u64;
        let records = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_emission_records"),
            size: record_len.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let count = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_emission_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            records,
            count,
            capacity,
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn total_buffer_bytes(&self) -> u64 {
        self.records.size() + self.count.size()
    }

    #[doc(hidden)]
    pub fn records_binding(&self) -> &Buffer {
        &self.records
    }

    #[doc(hidden)]
    pub fn count_binding(&self) -> &Buffer {
        &self.count
    }

    pub fn reset_count(&self, queue: &Queue) {
        queue.write_buffer(&self.count, 0, &0u32.to_le_bytes());
    }

    pub fn read_count(&self, device: &Device, queue: &Queue) -> u32 {
        read_u32_counter(device, queue, &self.count)
    }

    pub fn read_records(
        &self,
        device: &Device,
        queue: &Queue,
    ) -> Result<Vec<EmissionRecord>, KernelReadbackError> {
        let count = self.read_count(device, queue);
        if count == 0 {
            return Ok(Vec::new());
        }
        if count > self.capacity {
            return Err(KernelReadbackError::EmissionOverflow {
                count,
                capacity: self.capacity,
            });
        }
        Ok(mint_from_gpu_pod::<EmissionRecordGpu, _>(
            device,
            queue,
            &self.records,
            count,
            std::mem::size_of::<EmissionRecordGpu>() as u64,
            EmissionRecord::from_gpu_readback,
        ))
    }

    pub fn read_records_capped(
        &self,
        device: &Device,
        queue: &Queue,
    ) -> Result<(u32, Vec<EmissionRecord>), KernelReadbackError> {
        let count = self.read_count(device, queue);
        if count == 0 {
            return Ok((0, Vec::new()));
        }
        let read_count = count.min(self.capacity);
        let records = mint_from_gpu_pod::<EmissionRecordGpu, _>(
            device,
            queue,
            &self.records,
            read_count,
            std::mem::size_of::<EmissionRecordGpu>() as u64,
            EmissionRecord::from_gpu_readback,
        );
        Ok((count, records))
    }
}

/// Kernel-owned C-1 threshold emission buffer + counter.
pub struct ThresholdEmissionReadback {
    records: Buffer,
    count: Buffer,
    capacity: u32,
}

impl ThresholdEmissionReadback {
    pub fn new(device: &Device, capacity: u32) -> Self {
        let record_len = (capacity as u64) * std::mem::size_of::<ThresholdEmissionGpu>() as u64;
        let records = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_threshold_emissions"),
            size: record_len.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let count = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_threshold_emission_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self {
            records,
            count,
            capacity,
        }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn total_buffer_bytes(&self) -> u64 {
        self.records.size() + self.count.size()
    }

    pub fn ensure_capacity(&mut self, device: &Device, capacity: u32) {
        if capacity <= self.capacity {
            return;
        }
        let record_len = (capacity as u64) * std::mem::size_of::<ThresholdEmissionGpu>() as u64;
        self.records = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_threshold_emissions"),
            size: record_len.max(4),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        self.capacity = capacity;
    }

    #[doc(hidden)]
    pub fn records_binding(&self) -> &Buffer {
        &self.records
    }

    #[doc(hidden)]
    pub fn count_binding(&self) -> &Buffer {
        &self.count
    }

    pub fn reset_count(&self, queue: &Queue) {
        queue.write_buffer(&self.count, 0, &0u32.to_le_bytes());
    }

    pub fn read_count(&self, device: &Device, queue: &Queue) -> u32 {
        read_u32_counter(device, queue, &self.count)
    }

    pub fn read_threshold_emissions(
        &self,
        device: &Device,
        queue: &Queue,
    ) -> Result<Vec<ThresholdEmission>, KernelReadbackError> {
        let count = self.read_count(device, queue);
        if count == 0 {
            return Ok(Vec::new());
        }
        if count > self.capacity {
            return Err(KernelReadbackError::ThresholdEmissionOverflow {
                count,
                capacity: self.capacity,
            });
        }
        Ok(mint_from_gpu_pod::<ThresholdEmissionGpu, _>(
            device,
            queue,
            &self.records,
            count,
            std::mem::size_of::<ThresholdEmissionGpu>() as u64,
            ThresholdEmission::from_gpu_readback,
        ))
    }

    /// Reconstruct Pass 7 `ThresholdEvent`s from compact threshold emissions.
    pub fn read_threshold_events(
        &self,
        device: &Device,
        queue: &Queue,
        event_kinds: &[u32],
    ) -> Result<Vec<ThresholdEvent>, KernelReadbackError> {
        let emissions = self.read_threshold_emissions(device, queue)?;
        Ok(emissions
            .into_iter()
            .map(|e| {
                ThresholdEvent::from_kernel_pass7_readback(
                    e.slot(),
                    e.col(),
                    e.value(),
                    event_kinds[e.reg_idx() as usize],
                )
            })
            .collect())
    }
}

/// Kernel-owned Pass 7 `event_candidates` buffer + counter.
pub struct ThresholdEventCandidatesReadback {
    candidates: Buffer,
    count: Buffer,
}

impl ThresholdEventCandidatesReadback {
    pub fn new(device: &Device, candidate_bytes: u64) -> Self {
        let candidates = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_event_candidates"),
            size: candidate_bytes.max(std::mem::size_of::<ThresholdEventGpu>() as u64),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let count = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_event_count"),
            size: 4,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        Self { candidates, count }
    }

    pub fn total_buffer_bytes(&self) -> u64 {
        self.candidates.size() + self.count.size()
    }

    pub fn candidates_size(&self) -> u64 {
        self.candidates.size()
    }

    pub fn ensure_candidates_bytes(&mut self, device: &Device, byte_size: u64) {
        if byte_size <= self.candidates.size() {
            return;
        }
        self.candidates = device.create_buffer(&BufferDescriptor {
            label: Some("kernel_event_candidates"),
            size: byte_size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
    }

    #[doc(hidden)]
    pub fn candidates_binding(&self) -> &Buffer {
        &self.candidates
    }

    #[doc(hidden)]
    pub fn count_binding(&self) -> &Buffer {
        &self.count
    }

    pub fn reset_count(&self, queue: &Queue) {
        queue.write_buffer(&self.count, 0, &0u32.to_le_bytes());
    }

    pub fn read_count(&self, device: &Device, queue: &Queue) -> u32 {
        read_u32_counter(device, queue, &self.count)
    }

    /// Read back exactly `n` threshold events (caller caps at registration count).
    pub fn read_events(
        &self,
        device: &Device,
        queue: &Queue,
        n_thresholds: u32,
        n: u32,
    ) -> Vec<ThresholdEvent> {
        let n = n.min(n_thresholds);
        if n == 0 {
            return Vec::new();
        }
        mint_from_gpu_pod::<ThresholdEventGpu, _>(
            device,
            queue,
            &self.candidates,
            n,
            std::mem::size_of::<ThresholdEventGpu>() as u64,
            ThresholdEvent::from_gpu_readback,
        )
    }
}
