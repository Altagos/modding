use std::fs;

use ron::de::from_reader;
use serde::de::DeserializeOwned;

pub fn deserialize_file<T: DeserializeOwned>(path: &str) -> Option<T> {
    return match fs::File::open(path) {
        Ok(x) => match from_reader(x) {
            Ok(y) => Some(y),
            Err(_) => None
        },
        Err(_) => None,
    };
}
