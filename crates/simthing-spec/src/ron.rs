use crate::error::CapabilityTreeError;
use crate::spec::capability::CapabilityTreeSpec;

pub fn deserialize_capability_tree_ron(text: &str) -> Result<CapabilityTreeSpec, CapabilityTreeError> {
    ron::from_str(text).map_err(|e| CapabilityTreeError::RonParse(e.to_string()))
}
