use serde::{Deserialize, Serialize};
use crate::{traits::ModuleComponent, util::deserialize_file};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    // pub dependencies: Vec<String>,
    // pub actions_path: Option<String>,
    pub languages_path: Option<String>,
    pub commands_path: Option<String>
}

impl ModuleComponent for Info {
    fn load(path: &str) -> Info {
        deserialize_file::<Info>(path).expect("No info.ron file present or invalid info.ron file")
    }
}
