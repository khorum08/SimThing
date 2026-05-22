use crate::error::SpecError;
use crate::spec::capability::CapabilityTreeSpec;
use crate::spec::game_mode::GameModeSpec;

pub fn deserialize_game_mode_ron(text: &str) -> Result<GameModeSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}

pub fn deserialize_capability_tree_ron(text: &str) -> Result<CapabilityTreeSpec, SpecError> {
    ron::from_str(text).map_err(|e| SpecError::RonParse(e.to_string()))
}
