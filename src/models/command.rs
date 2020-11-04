use serde::{Deserialize, Serialize};
use crate::{traits::ModuleComponent, util::deserialize_file};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Command {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub description: String,
    pub command: String,
}

impl ModuleComponent for Command {
    fn load(path: &str) -> Command {
        deserialize_file::<Command>(path).expect(format!("Invalid command file: `{}`", path).as_str())
    }
}
