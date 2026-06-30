//! Generic single-encoder batching for W impedance compose + PALMA min-plus prep/relaxation.
//!
//! MapGen PR8 / BH-2C measurement spike only — numeric scheduling over existing ops; no movement semantics.

use wgpu::CommandEncoderDescriptor;

use crate::min_plus_stencil::{MinPlusStencilError, MinPlusStencilOp};
use crate::w_impedance_compose::{
    WImpedanceComposeConfig, WImpedanceComposeError, WImpedanceComposeOp,
};
use crate::GpuContext;

/// Compact evidence from a serial or scheduled W→PALMA GPU chain.
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledWPalmaChainEvidence {
    pub queue_submits: u32,
    pub scheduled_single_encoder: bool,
    pub gpu_resident: bool,
    pub probe_d: f32,
    pub elapsed_ms: u128,
}

/// Serial baseline: separate queue submits per existing op (W compose, scatters, min-plus iterations).
pub fn dispatch_serial_w_palma_chain(
    ctx: &GpuContext,
    w_op: &WImpedanceComposeOp,
    w_config: &WImpedanceComposeConfig,
    interleaved_buffer: &wgpu::Buffer,
    stencil: &MinPlusStencilOp,
    iterations: u32,
) -> Result<u32, ScheduledWPalmaChainError> {
    w_op.compose_resident_field(ctx, interleaved_buffer, w_config)?;
    stencil.prepare_input_from_gpu_interleaved_w(ctx, interleaved_buffer)?;
    stencil.dispatch_ping_pong(ctx, iterations)?;
    Ok(MinPlusStencilOp::serial_w_palma_queue_submit_count(
        iterations,
    ))
}

/// Scheduled-concurrency path: W compose + PALMA prep + ping-pong in one command encoder / one submit.
pub fn dispatch_scheduled_w_palma_chain(
    ctx: &GpuContext,
    w_op: &WImpedanceComposeOp,
    w_config: &WImpedanceComposeConfig,
    interleaved_buffer: &wgpu::Buffer,
    stencil: &MinPlusStencilOp,
    iterations: u32,
) -> Result<(), ScheduledWPalmaChainError> {
    let w_bg = w_op
        .compose_bind_group(ctx, interleaved_buffer, w_config)
        .map_err(ScheduledWPalmaChainError::WCompose)?;
    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("scheduled_w_palma_chain_enc"),
        });
    w_op.record_compose_pass(&mut encoder, &w_bg, w_config);
    stencil
        .record_prepare_from_gpu_interleaved_w(ctx, &mut encoder, interleaved_buffer)
        .map_err(ScheduledWPalmaChainError::MinPlus)?;
    stencil
        .record_ping_pong(&ctx.device, &mut encoder, iterations)
        .map_err(ScheduledWPalmaChainError::MinPlus)?;
    ctx.queue.submit(Some(encoder.finish()));
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ScheduledWPalmaChainError {
    #[error(transparent)]
    WCompose(#[from] WImpedanceComposeError),
    #[error(transparent)]
    MinPlus(#[from] MinPlusStencilError),
}
