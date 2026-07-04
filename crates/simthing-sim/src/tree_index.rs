//! One-pass SimThing tree path index for O(1) id → node lookup during boundary work.

use simthing_core::{SimThing, SimThingId};
use std::collections::HashMap;

/// Child-index path from root to each node (`root` has an empty path).
pub fn build_node_paths(root: &SimThing) -> HashMap<SimThingId, Vec<usize>> {
    let mut paths = HashMap::new();
    collect_node_paths(root, &mut Vec::new(), &mut paths);
    paths
}

fn collect_node_paths(
    node: &SimThing,
    path: &mut Vec<usize>,
    paths: &mut HashMap<SimThingId, Vec<usize>>,
) {
    paths.insert(node.id, path.clone());
    for (idx, child) in node.children.iter().enumerate() {
        path.push(idx);
        collect_node_paths(child, path, paths);
        path.pop();
    }
}

pub fn node_at_path<'a>(root: &'a SimThing, path: &[usize]) -> Option<&'a SimThing> {
    let mut node = root;
    for &idx in path {
        node = node.children.get(idx)?;
    }
    Some(node)
}

pub fn node_at_path_mut<'a>(root: &'a mut SimThing, path: &[usize]) -> Option<&'a mut SimThing> {
    let mut node = root;
    for &idx in path {
        node = node.children.get_mut(idx)?;
    }
    Some(node)
}

/// Detach the subtree at `path`. Returns `None` for the root path.
pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
    if path.is_empty() {
        return None;
    }
    let (parent_path, idx) = path.split_at(path.len().checked_sub(1)?);
    let idx = *idx.first()?;
    let parent = node_at_path_mut(root, parent_path)?;
    Some(parent.children.remove(idx))
}

/// Child-index paths in depth-first pre-order (shorter paths before deeper ones).
pub fn paths_preorder(paths: &HashMap<SimThingId, Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ordered: Vec<Vec<usize>> = paths.values().cloned().collect();
    ordered.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    ordered
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::SimThingKind;

}
