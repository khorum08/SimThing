//! 0088-BASELINE-0 observer-owned timestamps around existing queue seams.
//! Resolve only after the measured causal chain has been submitted. No economic data
//! enters this observer and no synchronization is introduced between measured legs.

use anyhow::{ensure, Context, Result};

pub struct GpuTimeline {
    queries: wgpu::QuerySet,
    resolve: wgpu::Buffer,
    readback: wgpu::Buffer,
    count: u32,
}

impl GpuTimeline {
    pub fn new(device: &wgpu::Device, count: u32) -> Result<Self> {
        ensure!(
            device.features().contains(
                wgpu::Features::TIMESTAMP_QUERY | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS
            ),
            "TIMESTAMP-SEAM-UNAVAILABLE: enclosing GPU intervals cannot be captured"
        );
        Ok(Self {
            queries: device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("rehearsal timestamps"),
                ty: wgpu::QueryType::Timestamp,
                count,
            }),
            resolve: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("rehearsal resolve"),
                size: count as u64 * 8,
                usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("rehearsal timestamp egress"),
                size: count as u64 * 8,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            count,
        })
    }

    pub fn encode_mark(&self, encoder: &mut wgpu::CommandEncoder, index: u32) {
        encoder.write_timestamp(&self.queries, index);
    }

    /// A queue marker bracketing a public API that owns its own encoder.
    /// Its interval includes queue gaps and is not labelled kernel-only time.
    pub fn mark(&self, device: &wgpu::Device, queue: &wgpu::Queue, index: u32) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("rehearsal seam"),
        });
        self.encode_mark(&mut encoder, index);
        queue.submit(Some(encoder.finish()));
    }

    pub fn read_ns(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Vec<f64>> {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("rehearsal observer egress"),
        });
        encoder.resolve_query_set(&self.queries, 0..self.count, &self.resolve, 0);
        encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.readback, 0, self.count as u64 * 8);
        let submission = queue.submit(Some(encoder.finish()));
        let (tx, rx) = std::sync::mpsc::channel();
        self.readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| {
                let _ = tx.send(r);
            });
        device.poll(wgpu::Maintain::WaitForSubmissionIndex(submission));
        rx.recv().context("timestamp callback")??;
        let data = self.readback.slice(..).get_mapped_range();
        let ticks: &[u64] = bytemuck::cast_slice(&data);
        let origin = ticks[0];
        let ns = ticks
            .iter()
            .map(|t| t.wrapping_sub(origin) as f64 * queue.get_timestamp_period() as f64)
            .collect();
        drop(data);
        self.readback.unmap();
        Ok(ns)
    }
}
