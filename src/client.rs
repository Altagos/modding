use std::collections::HashMap;
use crate::models::module::Mod;
use crate::models::base_mod::BaseMod;
use walkdir::{WalkDir, DirEntry};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct Client {
    pub mod_dir: String,
    pub base: BaseMod,
    pub mods: HashMap<String, Mod>,
}

impl Client {
    pub fn load(mod_dir: &str) -> Self {
        let mut client = Self::default();

        client.mod_dir = mod_dir.to_string();
        let mut contains_base_mod = false;
        for entry in WalkDir::new(&mod_dir).min_depth(1).max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok()) {
            // println!("{}", entry.path().to_str().unwrap());
            if Client::is_base(&entry, &mod_dir) {
                contains_base_mod = true;
                client.base = BaseMod::load(entry.path());
            } else if entry.path().is_dir() {
                let module = Mod::load(entry.path());
                let key = entry.path().to_str().unwrap().replace(mod_dir, "").replace("\\", "").replace("/", "");
                client.mods.insert(key, module);
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

    /// Get the translation of a certain key from a mod. If translation is not found, the translation from the base mod will be provided.
    /// The Language is defined as following `<mod>/<language>`.
    pub fn get_translation(&self, language: &str, k: &str) -> &String {
        let split: Vec<&str> = language.split('/').collect();
        let (module_name, lang_name) = (split[0], split[1]);
        let module = self.mods.get(module_name);
        match module {
            Some(x) => {
                let lang = x.languages.get(lang_name);
                match lang {
                    Some(y) => {
                        let translation = y.get(k);
                        match translation {
                            Some(z) => return z,
                            None => {}
                        }
                    }
                    None => {}
                };
            }
            None => {}
        };
        self.base.language_default().get(k)
    }
}