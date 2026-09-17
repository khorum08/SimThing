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

/// Detach the subtree at `path`. Returns `None` for the root path or a stale
/// out-of-bounds index — never panics (STRUCTURAL IDENTITY LAW, relay
/// 5707155261: paths are hints; a shifted index must not abort the boundary).
pub fn detach_at_path(root: &mut SimThing, path: &[usize]) -> Option<SimThing> {
    if path.is_empty() {
        return None;
    }
    let (parent_path, idx) = path.split_at(path.len().checked_sub(1)?);
    let idx = *idx.first()?;
    let parent = node_at_path_mut(root, parent_path)?;
    if idx >= parent.children.len() {
        return None;
    }
    Some(parent.children.remove(idx))
}

/// STRUCTURAL IDENTITY LAW (DA admission, relay 5707155261): `SimThingId` is
/// the ONLY mutation authority; a cached child-index path is an acceleration
/// hint. The hint is honored only when the node it currently reaches still
/// carries the requested identity; otherwise resolution falls back to the
/// CURRENT tree by identity. A stale hint therefore never detaches a
/// different identity and never panics on a shifted index.
pub fn detach_by_identity(
    root: &mut SimThing,
    target: SimThingId,
    hint: Option<&[usize]>,
) -> Option<SimThing> {
    if let Some(path) = hint {
        if !path.is_empty() {
            let (parent_path, idx) = path.split_at(path.len() - 1);
            let idx = idx[0];
            if let Some(parent) = node_at_path_mut(root, parent_path) {
                if parent.children.get(idx).map(|child| child.id) == Some(target) {
                    return Some(parent.children.remove(idx));
                }
            }
        }
        // Stale hint: fall through to identity resolution below.
    }
    detach_subtree_by_identity(root, target)
}

/// Identity-walk detach against the CURRENT tree; the authoritative slow path.
pub fn detach_subtree_by_identity(root: &mut SimThing, target: SimThingId) -> Option<SimThing> {
    if let Some(pos) = root.children.iter().position(|child| child.id == target) {
        return Some(root.children.remove(pos));
    }
    for child in &mut root.children {
        if let Some(found) = detach_subtree_by_identity(child, target) {
            return Some(found);
        }
    }
    None
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
