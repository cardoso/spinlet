use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Child {
    #[serde(default = "default_name")]
    name: String
}

impl Child {
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn default_name() -> String {
    "shell".to_string()
}

impl Default for Child {
    fn default() -> Self {
        Self {
            name: default_name(),
        }
    }
}