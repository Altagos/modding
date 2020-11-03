use crate::models::info::Info;
use crate::models::language::Language;
use crate::models::paths::Paths;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct Mod {
    pub info: Info,
    pub paths: Paths,
    pub languages: HashMap<String, Language>,
}

impl Mod {
    pub fn load(path: &Path) -> Self {
        let mut module = Mod::default();
        
        module.info = Info::load(format!("{}/info.ron", path.to_str().unwrap()).as_str());
        module.build_paths(path);

        module
    }

    pub fn build_paths(&mut self, path: &Path) {
        let path = path.to_str().unwrap();
        let info = self.info.clone();
        let mut paths = Paths::default();
        
        paths.info = "/info.ron".to_string();

        if info.languages_path.is_some() {
            let languages_path = info.languages_path.unwrap().clone();
            for entry in WalkDir::new(format!("{}/{}", path, languages_path))
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    println!("Language: {}", entry.path().display());

                    let lang_file = fs::File::open(entry.path())
                        .expect("Only execute this command in a mod folder");
                    let lang: Language = match from_reader(lang_file) {
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
        }

        self.paths = paths;
    }
}
