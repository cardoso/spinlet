use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Parent {
    #[serde(default = "default_name")]
    name: String,
    #[serde(default)]
    alias: HashMap<String, String>,
}

impl Parent {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn alias(&self, name: &str) -> Option<&str> {
        self.alias.get(name).map(|s| s.as_str())
    }
}

pub fn default_name() -> String {
    "spin".to_string()
}

impl Default for Parent {
    fn default() -> Self {
        Self {
            name: default_name(),
            alias: HashMap::new(),
        }
    }
}