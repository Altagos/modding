use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::traits::ModuleComponent;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Paths {
    pub info: String,
    pub languages: HashMap<String, String>,
    pub commands: HashMap<String, String>,
}

impl ModuleComponent for Paths {
    fn load(path: &str) -> Self where for<'de> Self: Deserialize<'de> {
        crate::util::deserialize_file::<Self>(path).expect("Invalid paths.ron file")
    }
}
