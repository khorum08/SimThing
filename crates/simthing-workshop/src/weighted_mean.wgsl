// WeightedMean gather/combine/scatter spike — one invocation per parent, canonical child order.

struct WeightedChild {
    value: f32,
    weight: f32,
}

struct ParentRange {
    offset: u32,
    len: u32,
}

struct WeightedMeanOutput {
    value: f32,
}

struct WeightedMeanParams {
    n_parents: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<storage, read> children: array<WeightedChild>;
@group(0) @binding(1) var<storage, read> ranges: array<ParentRange>;
@group(0) @binding(2) var<storage, read_write> outputs: array<WeightedMeanOutput>;
@group(0) @binding(3) var<uniform> params: WeightedMeanParams;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let parent = gid.x;
    if (parent >= params.n_parents) {
        return;
    }

    let range = ranges[parent];

    var weighted_sum: f32 = 0.0;
    var weight_sum: f32 = 0.0;

    if (range.len > 0u) {
        let first = children[range.offset];
        weighted_sum = first.value * first.weight;
        weight_sum = first.weight;

        for (var i: u32 = 1u; i < range.len; i = i + 1u) {
            let child = children[range.offset + i];
            let scaled = child.value * child.weight;
            weighted_sum = weighted_sum + scaled;
            weight_sum = weight_sum + child.weight;
        }
    }

    var out: f32 = 0.0;
    if (weight_sum != 0.0) {
        out = weighted_sum / weight_sum;
    }

    outputs[parent].value = out;
}
