use crate::{models::language::BaseLanguage, traits::ModuleComponent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

use super::{command::Command, info::Info, paths::Paths};

#[derive(Clone, Debug, Deserialize, Default, Serialize)]
pub struct BaseMod {
    mod_dir: String,
    pub info: Info,
    pub paths: Paths,
    pub languages: HashMap<String, BaseLanguage>,
    pub commands: HashMap<String, Command>,
}

impl BaseMod {
    pub fn load(path: &Path) -> Self {
        let mut base = Self::default();
        tracing::info!("Loading mod `{}`...", path.display());

        base.mod_dir = path.to_str().unwrap().to_string();
        base.info = Info::load(format!("{}/info.ron", path.to_str().unwrap()).as_str());
        tracing::info!("Building paths for mod: `{}`", base.info.name);
        base.build_paths(path);
        tracing::info!("Loading languages from mod: `{}`", base.info.name);
        base.load_languages();
        tracing::info!("Loading commands from mod: `{}`", base.info.name);
        base.load_commands();

        tracing::info!("Loaded mod `{}`...", base.info.name);

        base
    }

    pub fn language_default(&self) -> &BaseLanguage {
        &self
            .languages
            .get("en")
            .expect("Expecting the English language to be defined!")
    }

    pub fn build_paths(&mut self, path: &Path) {
        let path = path.to_str().unwrap();
        let info = self.info.clone();
        let mut paths = Paths::default();

        paths.info = "/info.ron".to_string();

        tracing::debug!("Building laguage paths...");

        if info.languages_path.is_some() {
            let languages_path = info.languages_path.unwrap();
            for entry in WalkDir::new(format!("{}/{}", path, languages_path))
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    let c_path = entry.path().to_str().unwrap();
                    if c_path.ends_with(".ron") {
                        tracing::debug!("Language: {}", entry.path().display());
                        let lang = BaseLanguage::load(entry.path().to_str().unwrap());

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
                    }

                    // self.languages.insert(lang.name.clone(), lang);
                }
            }
        }

        tracing::debug!("Finished building laguage paths");
        tracing::debug!("Building command paths...");

        if info.commands_path.is_some() {
            let commands_path = info.commands_path.unwrap();
            for entry in WalkDir::new(format!("{}/{}", path, commands_path))
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    let c_path = entry.path().to_str().unwrap();
                    if c_path.ends_with(".ron") {
                        tracing::debug!("Command: {}", entry.path().display());
                        let command = Command::load(entry.path().to_str().unwrap());

                        paths.commands.insert(
                            command.name.clone(),
                            String::from(c_path.replace(&path, "").replace("\\", "/")),
                        );
                    }

                    // self.commands.insert(command.name.clone(), command);
                }
            }
        }

        tracing::debug!("Finished building command paths");

        self.paths = paths;
    }

    pub fn load_languages(&mut self) {
        let paths = self.paths.languages.clone();
        for (name, path) in paths {
            tracing::debug!("Loading language: `{}`", name);
            self.languages.insert(
                name,
                BaseLanguage::load(format!("{}{}", &self.mod_dir, path).as_str()),
            );
        }
    }

    pub fn load_commands(&mut self) {
        let paths = self.paths.commands.clone();
        for (name, path) in paths {
            tracing::debug!("Loading command: `{}`", name);
            self.commands.insert(
                name,
                Command::load(format!("{}{}", &self.mod_dir, path).as_str()),
            );
        }
    }
}