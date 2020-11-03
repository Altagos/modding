use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Info {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    // pub dependencies: Vec<String>,
    // pub actions_path: Option<String>,
    pub languages_path: Option<String>,
}

impl Info {
    pub fn load(path: &str) -> Self {
        let file = fs::File::open(path).expect("No info.ron file present");
        from_reader(file).expect("Invalid info.ron file")
    }
}
