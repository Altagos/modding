use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::paths::{BasePaths};
use crate::models::language::BaseLanguage;
use std::path::Path;
use ron::de::from_reader;
use std::fs;
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct BaseMod {
    pub paths: BasePaths,
    pub languages: HashMap<String, BaseLanguage>,
}

impl BaseMod {
    pub fn load(path: &Path) -> Self {
        let mut base = Self::default();
        base.languages = Default::default();

        base.build_paths(path);

        base
    }

    pub fn language_default(&self) -> &BaseLanguage {
        &self.languages.get("en").expect("Expecting the English language to be defined!")
    }

    pub fn build_paths(&mut self, path: &Path)  {
        let path = path.to_str().unwrap();
        let mut paths = BasePaths::default();

        for entry in WalkDir::new(format!("{}/languages", path))
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                println!("Language: {}", entry.path().display());

                let lang_file = fs::File::open(entry.path())
                    .expect("Only execute this command in a mod folder");
                let lang: BaseLanguage = match from_reader(lang_file) {
                    Ok(x) => x,
                    Err(e) => {
                        println!(
                            "Failed to load language ({}): {}",
                            entry.path().display(),
                            e
                        );

                        std::process::exit(0);
                    }
                };

                paths.languages.insert(
                    lang.name.clone(),
                    String::from(
                        &entry
                            .path()
                            .to_str()
                            .unwrap()
                            .to_owned()
                            .replace(&path, "")
                            .replace("\\", "/"),
                    ),
                );

                self.languages.insert(lang.name.clone(), lang.clone());
            }
        }

        self.paths = paths;
    }
}