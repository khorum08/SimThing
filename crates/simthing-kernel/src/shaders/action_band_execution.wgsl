struct ActionBandTemplateGpu {
    target_kind: u32,
    channel_start: u32,
    channel_count: u32,
    target_data_start: u32,
    projection_width: u32,
    band_start: u32,
    band_count: u32,
    membership_range: u32,
    projection_range: u32,
    velocity_current_channel: u32,
    velocity_previous_channel: u32,
}

struct ActionBandInstanceGpu {
    slot: u32,
    template_index: u32,
    projection_start: u32,
    generation: u32,
    param0: f32,
    param1: f32,
    param2: f32,
    param3: f32,
}

struct ActionBandStateGpu {
    satisfied: u32,
    generation: u32,
    projection_start: u32,
    projection_len: u32,
    distance: f32,
    velocity: f32,
    reserved0: u32,
    reserved1: u32,
}

struct ActionBandBandGpu {
    threshold_registration: u32,
    program_range: u32,
    binding_start: u32,
    binding_count: u32,
}

struct ActionBandEmissionBindingGpu {
    destination_kind: u32,
    destination_index: u32,
    auxiliary0: u32,
    auxiliary1: u32,
}

struct ActionBandCrossingInputGpu {
    instance_row: u32,
    band_index: u32,
    output_start: u32,
    output_count: u32,
    post_value: f32,
    threshold: f32,
    crossing_col: u32,
    reserved: u32,
}

struct ThresholdEmissionGpu {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

struct ActionBandDispatchParams {
    n_dims: u32,
    instance_count: u32,
    crossing_start: u32,
    crossing_count: u32,
}

@group(0) @binding(0) var<storage, read> action_templates: array<ActionBandTemplateGpu>;
@group(0) @binding(1) var<storage, read> action_target_channels: array<u32>;
@group(0) @binding(2) var<storage, read> action_target_data: array<f32>;
@group(0) @binding(3) var<storage, read> action_instances: array<ActionBandInstanceGpu>;
@group(0) @binding(4) var<storage, read> action_state_current: array<ActionBandStateGpu>;
@group(0) @binding(5) var<storage, read_write> action_state_next: array<ActionBandStateGpu>;
@group(0) @binding(6) var<storage, read_write> action_projection_next: array<f32>;
@group(0) @binding(7) var<storage, read> values: array<atomic<i32>>;
@group(0) @binding(8) var<uniform> tick_params: ActionBandDispatchParams;
@group(0) @binding(9) var<storage, read> action_bands: array<ActionBandBandGpu>;
@group(0) @binding(10) var<storage, read> action_band_binding_indices: array<u32>;
@group(0) @binding(11) var<storage, read> action_emission_bindings: array<ActionBandEmissionBindingGpu>;
@group(0) @binding(12) var<storage, read> action_crossings: array<ActionBandCrossingInputGpu>;
@group(0) @binding(13) var<storage, read_write> action_consequences: array<ThresholdEmissionGpu>;
@group(0) @binding(14) var<storage, read> eml_nodes: array<EmlNodeGpu>;
@group(0) @binding(15) var<storage, read> eml_tree_ranges: array<EmlTreeRangeGpu>;

const ACTION_TARGET_POINT: u32 = 0u;
const ACTION_TARGET_SCALAR_AT_LEAST: u32 = 1u;
const ACTION_TARGET_SCALAR_AT_MOST: u32 = 2u;
const ACTION_TARGET_INTERVAL: u32 = 3u;
const ACTION_TARGET_AABB: u32 = 4u;
const ACTION_TARGET_LOCUS_RADIUS: u32 = 5u;
const ACTION_TARGET_PALMA_REACHABLE: u32 = 6u;
const ACTION_TARGET_EML_PROJECTED: u32 = 7u;
const ACTION_NO_PROGRAM: u32 = 0xFFFFFFFFu;
fn action_value(slot: u32, col: u32) -> f32 {
    return atomic_read_f32_at(slot * tick_params.n_dims + col);
}

@compute @workgroup_size(64)
fn actionband_evaluate(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row = gid.x;
    if (row >= tick_params.instance_count) {
        return;
    }

    let instance = action_instances[row];
    let descriptor = action_templates[instance.template_index];
    let prior = action_state_current[row];
    var satisfied = 1u;
    var distance = 0.0;
    var velocity = 0.0;
    if (descriptor.velocity_current_channel != ACTION_NO_PROGRAM) {
        velocity = action_value(instance.slot, descriptor.velocity_current_channel)
            - action_value(instance.slot, descriptor.velocity_previous_channel);
    }

    if (descriptor.target_kind == ACTION_TARGET_POINT) {
        for (var i = 0u; i < descriptor.projection_width; i = i + 1u) {
            let current = action_value(instance.slot, action_target_channels[descriptor.channel_start + i]);
            let projected = action_target_data[descriptor.target_data_start + i] - current;
            action_projection_next[instance.projection_start + i] = projected;
            distance = distance + abs(projected);
            if (projected != 0.0) { satisfied = 0u; }
        }
    } else if (descriptor.target_kind == ACTION_TARGET_SCALAR_AT_LEAST || descriptor.target_kind == ACTION_TARGET_SCALAR_AT_MOST) {
        let current = action_value(instance.slot, action_target_channels[descriptor.channel_start]);
        let projected = action_target_data[descriptor.target_data_start] - current;
        let ok = select(current <= action_target_data[descriptor.target_data_start], current >= action_target_data[descriptor.target_data_start], descriptor.target_kind == ACTION_TARGET_SCALAR_AT_LEAST);
        action_projection_next[instance.projection_start] = select(projected, 0.0, ok);
        distance = select(projected, 0.0, ok);
        satisfied = select(0u, 1u, ok);
    } else if (descriptor.target_kind == ACTION_TARGET_INTERVAL) {
        let current = action_value(instance.slot, action_target_channels[descriptor.channel_start]);
        let lo = action_target_data[descriptor.target_data_start];
        let hi = action_target_data[descriptor.target_data_start + 1u];
        let projected = select(select(hi - current, 0.0, current <= hi), lo - current, current < lo);
        action_projection_next[instance.projection_start] = projected;
        distance = projected;
        satisfied = select(0u, 1u, projected == 0.0);
    } else if (descriptor.target_kind == ACTION_TARGET_AABB) {
        for (var i = 0u; i < descriptor.projection_width; i = i + 1u) {
            let current = action_value(instance.slot, action_target_channels[descriptor.channel_start + i]);
            let lo = action_target_data[descriptor.target_data_start + i];
            let hi = action_target_data[descriptor.target_data_start + descriptor.projection_width + i];
            let projected = clamp(current, lo, hi) - current;
            action_projection_next[instance.projection_start + i] = projected;
            distance = distance + abs(projected);
            if (projected != 0.0) { satisfied = 0u; }
        }
    } else if (descriptor.target_kind == ACTION_TARGET_LOCUS_RADIUS || descriptor.target_kind == ACTION_TARGET_PALMA_REACHABLE) {
        let current = action_value(instance.slot, action_target_channels[descriptor.channel_start]);
        let limit = action_target_data[descriptor.target_data_start];
        let projected = max(current - limit, 0.0);
        action_projection_next[instance.projection_start] = projected;
        distance = projected;
        satisfied = select(0u, 1u, projected == 0.0);
    } else if (descriptor.target_kind == ACTION_TARGET_EML_PROJECTED) {
        let membership = eml_eval(EmlEvalCtx(descriptor.membership_range, instance.slot, instance.param0, instance.param1, instance.param2, instance.param3));
        satisfied = select(0u, 1u, membership != 0.0);
        for (var i = 0u; i < descriptor.projection_width; i = i + 1u) {
            let projected = eml_eval(EmlEvalCtx(descriptor.projection_range, instance.slot, f32(i), instance.param1, instance.param2, instance.param3));
            action_projection_next[instance.projection_start + i] = projected;
            distance = distance + abs(projected);
        }
        if (satisfied == 1u) { distance = 0.0; }
    }

    action_state_next[row] = ActionBandStateGpu(
        satisfied,
        prior.generation + 1u,
        instance.projection_start,
        descriptor.projection_width,
        distance,
        velocity,
        0u,
        0u,
    );
}

fn actionband_evaluate_depth1_crossing(
    crossing: ActionBandCrossingInputGpu,
    instance: ActionBandInstanceGpu,
    descriptor: ActionBandTemplateGpu,
) -> ActionBandStateGpu {
    let current = crossing.post_value;
    var satisfied = 0u;
    var distance = 0.0;
    var projected = 0.0;

    if (descriptor.target_kind == ACTION_TARGET_POINT) {
        projected = action_target_data[descriptor.target_data_start] - current;
        distance = abs(projected);
        satisfied = select(0u, 1u, projected == 0.0);
    } else if (descriptor.target_kind == ACTION_TARGET_SCALAR_AT_LEAST || descriptor.target_kind == ACTION_TARGET_SCALAR_AT_MOST) {
        projected = action_target_data[descriptor.target_data_start] - current;
        let ok = select(current <= action_target_data[descriptor.target_data_start], current >= action_target_data[descriptor.target_data_start], descriptor.target_kind == ACTION_TARGET_SCALAR_AT_LEAST);
        projected = select(projected, 0.0, ok);
        distance = projected;
        satisfied = select(0u, 1u, ok);
    } else if (descriptor.target_kind == ACTION_TARGET_INTERVAL) {
        let lo = action_target_data[descriptor.target_data_start];
        let hi = action_target_data[descriptor.target_data_start + 1u];
        projected = select(select(hi - current, 0.0, current <= hi), lo - current, current < lo);
        distance = projected;
        satisfied = select(0u, 1u, projected == 0.0);
    } else if (descriptor.target_kind == ACTION_TARGET_AABB) {
        let lo = action_target_data[descriptor.target_data_start];
        let hi = action_target_data[descriptor.target_data_start + 1u];
        projected = clamp(current, lo, hi) - current;
        distance = abs(projected);
        satisfied = select(0u, 1u, projected == 0.0);
    } else if (descriptor.target_kind == ACTION_TARGET_LOCUS_RADIUS || descriptor.target_kind == ACTION_TARGET_PALMA_REACHABLE) {
        projected = max(current - action_target_data[descriptor.target_data_start], 0.0);
        distance = projected;
        satisfied = select(0u, 1u, projected == 0.0);
    }

    action_projection_next[instance.projection_start] = projected;
    let prior = action_state_current[crossing.instance_row];
    let next = ActionBandStateGpu(
        satisfied,
        prior.generation + 1u,
        instance.projection_start,
        1u,
        distance,
        0.0,
        0u,
        0u,
    );
    action_state_next[crossing.instance_row] = next;
    return next;
}

@compute @workgroup_size(64)
fn actionband_emit_depth1(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= tick_params.crossing_count) {
        return;
    }
    let crossing_row = tick_params.crossing_start + gid.x;
    let crossing = action_crossings[crossing_row];
    let band = action_bands[crossing.band_index];
    let instance = action_instances[crossing.instance_row];
    let descriptor = action_templates[instance.template_index];
    let state = actionband_evaluate_depth1_crossing(crossing, instance, descriptor);
    var payload = crossing.post_value;
    if (band.program_range != ACTION_NO_PROGRAM) {
        payload = eml_eval(EmlEvalCtx(band.program_range, instance.slot, crossing.post_value, crossing.threshold, state.distance, state.velocity));
    }
    for (var i = 0u; i < crossing.output_count; i = i + 1u) {
        action_consequences[crossing.output_start + i] = ThresholdEmissionGpu(
            band.threshold_registration,
            instance.slot,
            crossing.crossing_col,
            payload,
        );
    }
}

@compute @workgroup_size(64)
fn actionband_emit(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= tick_params.crossing_count) {
        return;
    }
    let crossing_row = tick_params.crossing_start + gid.x;
    let crossing = action_crossings[crossing_row];
    let band = action_bands[crossing.band_index];
    let state = action_state_next[crossing.instance_row];
    let instance = action_instances[crossing.instance_row];
    var payload = crossing.post_value;
    if (band.program_range != ACTION_NO_PROGRAM) {
        payload = eml_eval(EmlEvalCtx(band.program_range, instance.slot, crossing.post_value, crossing.threshold, state.distance, state.velocity));
    }
    for (var i = 0u; i < crossing.output_count; i = i + 1u) {
        action_consequences[crossing.output_start + i] = ThresholdEmissionGpu(
            band.threshold_registration,
            instance.slot,
            crossing.crossing_col,
            payload,
        );
    }
}
