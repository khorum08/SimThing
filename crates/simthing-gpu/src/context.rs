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

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label:            Some("simthing-gpu device"),
                    required_features: Features::empty(),
                    required_limits:   Limits::default(),
                    memory_hints:      MemoryHints::default(),
                },
                None,
            )
            .await?;

        Ok(Self { instance, adapter, device, queue })
    }
}
