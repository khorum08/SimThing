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

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::SimThingKind;

    #[test]
    fn paths_cover_every_node() {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let child = SimThing::new(SimThingKind::Location, 0);
        let grand = SimThing::new(SimThingKind::Cohort, 0);
        root.add_child(child);
        root.children[0].add_child(grand);

        let paths = build_node_paths(&root);
        assert_eq!(paths.len(), 3);
        assert!(paths[&root.id].is_empty());
        assert_eq!(paths[&root.children[0].id], vec![0]);
        assert_eq!(paths[&root.children[0].children[0].id], vec![0, 0]);
    }

    #[test]
    fn detach_at_path_removes_subtree() {
        let mut root = SimThing::new(SimThingKind::World, 0);
        let child_id = {
            let child = SimThing::new(SimThingKind::Location, 0);
            let id = child.id;
            root.add_child(child);
            id
        };
        let paths = build_node_paths(&root);
        let detached = detach_at_path(&mut root, &paths[&child_id]).expect("detach");
        assert_eq!(detached.id, child_id);
        assert!(root.children.is_empty());
    }
}
