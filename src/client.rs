use crate::models::module::{BaseMod};
use walkdir::{WalkDir, DirEntry};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct Client {
    pub mod_dir: String,
    pub base: BaseMod,
}

impl Client {
    pub fn load(mod_dir: &str) -> Self {
        let mut client = Self::default();

        client.mod_dir = mod_dir.to_string();
        let mut contains_base_mod = false;
        for entry in WalkDir::new(&mod_dir)
            .into_iter()
            .filter_map(|e| e.ok()) {
            // println!("{}", entry.path().to_str().unwrap());
            if Client::is_base(&entry, &mod_dir) {
                contains_base_mod = true;
                client.base = BaseMod::load(entry.path());
            }
        }

        if !contains_base_mod {
            eprintln!("Error: Base mod does not exists");
            std::process::exit(0);
        }
        client
    }

    fn is_base(entry: &DirEntry, mod_dir: &str) -> bool {
        entry.path().is_dir() && entry.path() == Path::new(&format!("{}/base", mod_dir))
    }
}