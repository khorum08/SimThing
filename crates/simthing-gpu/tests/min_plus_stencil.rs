//! PALMA-PATH-2 — GPU min-plus stencil compile and parity smoke tests.

use simthing_gpu::{
    cpu_min_plus_d_from_w, extract_d_flat, max_d_field_error, pack_w_and_initial_d, GpuContext,
    MinPlusStencilConfig, MinPlusStencilOp, MIN_PLUS_INF,
};
use std::sync::Mutex;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn with_gpu<F: FnOnce(&GpuContext)>(f: F) {
    let ctx = GpuContext::new_blocking().expect("GPU required for min-plus stencil tests");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    f(&ctx);
}

fn config_5x5() -> MinPlusStencilConfig {
    MinPlusStencilConfig {
        width: 5,
        height: 5,
        n_dims: 2,
        d_col: 0,
        w_col: 1,
        dest_x: 0,
        dest_y: 0,
        inf_sentinel: MIN_PLUS_INF,
    }
}

#[test]
fn min_plus_stencil_compiles_and_matches_cpu_uniform() {
    with_gpu(|ctx| {
        let config = config_5x5();
        let w = vec![1.0f32; 25];
        let iterations = 16u32;
        let cpu_d = cpu_min_plus_d_from_w(&w, &config, iterations).unwrap();
        let values = pack_w_and_initial_d(&w, &config).unwrap();
        let op = MinPlusStencilOp::new(ctx, config).unwrap();
        op.upload_values(ctx, &values).unwrap();
        let gpu_values = op.run_ping_pong(ctx, iterations).unwrap();
        let gpu_d = extract_d_flat(&gpu_values, op.config()).unwrap();
        assert!(max_d_field_error(&cpu_d, &gpu_d) < 1e-4);
    });
}
