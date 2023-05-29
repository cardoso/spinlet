

use schemars::JsonSchema;
pub use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Current {
    #[serde(default = "default_name")]
    name: String
}

impl Current {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn default_name() -> String {
    "let".to_string()
}

impl Default for Current {
    fn default() -> Self {
        Self {
            name: default_name(),
        }
    }
}
