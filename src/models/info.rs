use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub dependencies: Vec<String>,
    pub actions_path: Option<String>,
    pub languages_path: Option<String>,
}
