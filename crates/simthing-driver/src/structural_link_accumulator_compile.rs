//! Compile `SimThingScenarioSpec` structural links into AccumulatorOp Sum-over-INPUT_LIST plans.

use std::collections::BTreeMap;

use simthing_core::{
    AccumulatorOp, ColumnIndex, CombineFn, CompiledAccumulatorOpPlan, ConsumeMode, GateSpec,
    InputSpec, ScaleSpec, SlotIndex, SourceSpec, StructuralScalarChannel,
};
use simthing_spec::{
    validate_scenario_links, validate_stead_mapping_consistency, ScenarioLinkError,
    SimThingScenarioSpec, SteadMappingError,
};

#[derive(Debug, thiserror::Error)]
pub enum DriverCompileError {
    #[error(transparent)]
    SteadMapping(#[from] SteadMappingError),
    #[error(transparent)]
    LinkValidation(#[from] ScenarioLinkError),
    #[error("structural link endpoint not found in placements: from={from}, to={to}")]
    InvalidLinkEndpoint { from: String, to: String },
    #[error("structural self-link at system_id={system_id}")]
    SelfLink { system_id: String },
    #[error("duplicate structural link: from={from}, to={to}")]
    DuplicateLink { from: String, to: String },
    #[error("reversed duplicate structural link: from={from}, to={to}")]
    ReversedDuplicateLink { from: String, to: String },
    #[error("input and output channels must differ")]
    IdenticalChannels,
    #[error("no structural locations to compile")]
    EmptyLocationSet,
}

struct DenseProjection {
    location_count: u32,
    adjacency: Vec<Vec<u32>>,
}

fn build_dense_projection(
    spec: &SimThingScenarioSpec,
) -> Result<DenseProjection, DriverCompileError> {
    validate_stead_mapping_consistency(spec)?;
    validate_scenario_links(spec)?;

    let mut placements: Vec<_> = spec.structural_grid.placements.iter().collect();
    placements.sort_by(|left, right| {
        left.row
            .cmp(&right.row)
            .then_with(|| left.col.cmp(&right.col))
            .then_with(|| left.system_id.cmp(&right.system_id))
            .then_with(|| left.simthing_id_raw.cmp(&right.simthing_id_raw))
    });

    if placements.is_empty() {
        return Err(DriverCompileError::EmptyLocationSet);
    }

    let location_count = placements.len() as u32;
    let system_to_dense: BTreeMap<String, u32> = placements
        .iter()
        .enumerate()
        .map(|(dense, placement)| (placement.system_id.to_string(), dense as u32))
        .collect();

    let mut adjacency = vec![Vec::new(); location_count as usize];
    let mut seen_edges = std::collections::BTreeSet::new();

    for link in &spec.links {
        let from_dense = *system_to_dense.get(&link.from_system_id).ok_or_else(|| {
            DriverCompileError::InvalidLinkEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            }
        })?;
        let to_dense = *system_to_dense.get(&link.to_system_id).ok_or_else(|| {
            DriverCompileError::InvalidLinkEndpoint {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            }
        })?;
        if from_dense == to_dense {
            return Err(DriverCompileError::SelfLink {
                system_id: link.from_system_id.clone(),
            });
        }
        let (min_dense, max_dense) = if from_dense < to_dense {
            (from_dense, to_dense)
        } else {
            (to_dense, from_dense)
        };
        if !seen_edges.insert((min_dense, max_dense)) {
            return Err(DriverCompileError::DuplicateLink {
                from: link.from_system_id.clone(),
                to: link.to_system_id.clone(),
            });
        }
        adjacency[min_dense as usize].push(max_dense);
        adjacency[max_dense as usize].push(min_dense);
    }

    for neighbors in &mut adjacency {
        neighbors.sort_unstable();
        neighbors.dedup();
    }

    Ok(DenseProjection {
        location_count,
        adjacency,
    })
}

/// Compile canonical structural link neighbor-sum into AccumulatorOp Sum-over-INPUT_LIST ops.
pub fn compile_structural_link_neighbor_sum_plan(
    scenario: &SimThingScenarioSpec,
    input_channel: StructuralScalarChannel,
    output_channel: StructuralScalarChannel,
) -> Result<CompiledAccumulatorOpPlan, DriverCompileError> {
    if input_channel == output_channel {
        return Err(DriverCompileError::IdenticalChannels);
    }

    let projection = build_dense_projection(scenario)?;
    let input_col = input_channel.0;
    let output_col = output_channel.0;
    let n_dims = input_col.max(output_col) + 1;

    let mut ops = Vec::new();
    for (target_slot, neighbors) in projection.adjacency.iter().enumerate() {
        if neighbors.is_empty() {
            continue;
        }
        let inputs: Vec<InputSpec> = neighbors
            .iter()
            .map(|&neighbor_slot| InputSpec {
                slot: SlotIndex::new(neighbor_slot),
                col: ColumnIndex::new(input_col as usize),
                unit_cost: 1.0,
            })
            .collect();
        ops.push(AccumulatorOp {
            source: SourceSpec::ConjunctiveCrossing { inputs },
            combine: CombineFn::Sum,
            gate: GateSpec::Always,
            scale: ScaleSpec::Identity,
            consume: ConsumeMode::AddToTarget,
            targets: vec![(
                SlotIndex::new(target_slot as u32),
                ColumnIndex::new(output_col as usize),
            )],
        });
    }

    Ok(CompiledAccumulatorOpPlan {
        slot_count: projection.location_count,
        n_dims,
        input_channel,
        output_channel,
        ops,
    })
}
