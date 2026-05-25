//! GPU device/queue lifecycle. One `GpuContext` per session.

use thiserror::Error;
use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Features, Instance,
           InstanceDescriptor, Limits, MemoryHints, PowerPreference, Queue,
           RequestAdapterOptions};

#[derive(Debug, Error)]
pub enum GpuInitError {
    #[error("no suitable GPU adapter found")]
    NoAdapter,
    #[error("device request failed: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}

pub struct GpuContext {
    pub instance: Instance,
    pub adapter:  Adapter,
    pub device:   Device,
    pub queue:    Queue,
    timestamp_supported: bool,
    timestamp_period_ns: f32,
}

impl GpuContext {
    /// Blocking init — for tests and one-shot setup. Uses pollster.
    pub fn new_blocking() -> Result<Self, GpuInitError> {
        pollster::block_on(Self::new())
    }

    pub async fn new() -> Result<Self, GpuInitError> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference:       PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface:     None,
            })
            .await
            .ok_or(GpuInitError::NoAdapter)?;

        let timestamp_supported = adapter.features().contains(Features::TIMESTAMP_QUERY);
        let required_features = if timestamp_supported {
            Features::TIMESTAMP_QUERY
        } else {
            Features::empty()
        };

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label:             Some("simthing-gpu device"),
                    required_features,
                    required_limits:   Limits::default(),
                    memory_hints:      MemoryHints::default(),
                },
                None,
            )
            .await?;

        let timestamp_period_ns = queue.get_timestamp_period();

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            timestamp_supported,
            timestamp_period_ns,
        })
    }

    pub fn timestamp_supported(&self) -> bool {
        self.timestamp_supported
    }

    pub fn timestamp_period_ns(&self) -> f32 {
        self.timestamp_period_ns
    }
}
