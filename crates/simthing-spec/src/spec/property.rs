use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertySpec {
    pub id:           String,
    pub namespace:    String,
    pub name:         String,
    #[serde(default)]
    pub display_name: String,
}
