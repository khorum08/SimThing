// Spike-only: iterative evaluator over a fixed topologically sorted node array (max 32 nodes).

struct IntensityInput {
    velocity: f32,
    intensity: f32,
}

struct IntensityOutput {
    value: f32,
}

struct EmlNode {
    op: u32,
    a: u32,
    b: u32,
    c: u32,
    value: f32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

struct EmlParams {
    n_slots: u32,
    root_node: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<storage, read> inputs: array<IntensityInput>;
@group(0) @binding(1) var<storage, read> nodes: array<EmlNode>;
@group(0) @binding(2) var<storage, read_write> outputs: array<IntensityOutput>;
@group(0) @binding(3) var<uniform> params: EmlParams;

const MAX_NODES: u32 = 32u;

fn eval_tree(slot: u32) -> f32 {
    var scratch: array<f32, MAX_NODES>;

    for (var i: u32 = 0u; i <= params.root_node; i = i + 1u) {
        let node = nodes[i];

        switch node.op {
            case 0u: {
                scratch[i] = node.value;
            }
            case 1u: {
                scratch[i] = inputs[slot].velocity;
            }
            case 2u: {
                scratch[i] = inputs[slot].intensity;
            }
            case 3u: {
                scratch[i] = abs(scratch[node.a]);
            }
            case 4u: {
                scratch[i] = select(0.0, 1.0, scratch[node.a] > scratch[node.b]);
            }
            case 5u: {
                scratch[i] = scratch[node.a] * scratch[node.b];
            }
            case 6u: {
                scratch[i] = scratch[node.a] + scratch[node.b];
            }
            case 7u: {
                scratch[i] = scratch[node.a] - scratch[node.b];
            }
            case 8u: {
                scratch[i] = select(scratch[node.c], scratch[node.b], scratch[node.a] != 0.0);
            }
            case 9u: {
                scratch[i] = clamp(scratch[node.a], 0.0, 1.0);
            }
            default: {
                scratch[i] = 0.0;
            }
        }
    }

    return scratch[params.root_node];
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= params.n_slots) {
        return;
    }
    outputs[slot].value = eval_tree(slot);
}
