use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct BaseLanguage {
    pub name: String,
    pub def: HashMap<String, String>,
}

impl BaseLanguage {
    pub fn get(&self, k: &str) -> &String {
        self.def.get(k).expect(&*format!("Invalid Language, `{}` not defined", k))
    }
}
