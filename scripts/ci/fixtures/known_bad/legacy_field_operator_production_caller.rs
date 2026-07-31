use simthing_gpu::{GpuContext, StructuredFieldStencilConfig, StructuredFieldStencilOp};

pub fn planted_production_caller(
    ctx: &GpuContext,
    config: StructuredFieldStencilConfig,
) -> StructuredFieldStencilOp {
    StructuredFieldStencilOp::new(ctx, config).expect("planted legacy production caller")
}
