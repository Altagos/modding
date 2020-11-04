use serde::Deserialize;
use crate::util::deserialize_file;

pub trait ModuleComponent {
    fn load(path: &str) -> Self where for<'de> Self: Deserialize<'de> {
        deserialize_file::<Self>(path).expect("Invalid .ron file")
    }
}