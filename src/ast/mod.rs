use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circuit {
    pub name: String,
    // TODO: Add circuit components and connections
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub component_type: String,
    // TODO: Add component properties
}