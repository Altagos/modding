use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::traits::ModuleComponent;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct BaseLanguage {
    pub name: String,
    pub def: HashMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Language {
    pub name: String,
    pub def: HashMap<String, String>,
}

impl BaseLanguage {
    pub fn get(&self, k: &str) -> &String {
        self.def.get(k).expect(&*format!("Base Language not complete, `{}` not defined", k))
    }
}

impl Language {
    pub fn get(&self, k: &str) -> Option<&String> {
        self.def.get(k)
    }
}

impl ModuleComponent for BaseLanguage {
    fn load(path: &str) -> Self where for<'de> Self: Deserialize<'de> {
        crate::util::deserialize_file::<Self>(path).expect(format!("Invalid base language file: `{}`", path).as_str())
    }
}

impl ModuleComponent for Language {
    fn load(path: &str) -> Self where for<'de> Self: Deserialize<'de> {
        crate::util::deserialize_file::<Self>(path).expect(format!("Invalid language file: `{}`", path).as_str())
    }
}