use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverlaySpec {
    pub id: String,
    #[serde(default)]
    pub display_name: String,
}
