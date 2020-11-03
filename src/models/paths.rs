use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Paths {
    pub info: String,
    pub languages: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct BasePaths {
    pub languages: HashMap<String, String>,
}

