use anyhow::Result;
use serde::{Deserialize, Deserializer};
use std::ops::Deref;

/// The global configuration
#[derive(Deserialize)]
pub struct Config {
    pub tasks: Frozen<Vec<TaskConfig>>,
}

/// The configuration of a specific task
#[derive(Deserialize)]
pub struct TaskConfig {
    pub name: Frozen<String>,
}

impl Config {
    /// Loads the configuration from the user and project config files
    pub fn load() -> Result<Config> {
        let config = Config {
            tasks: Frozen::new(Vec::new()),
        };

        Ok(config)
    }
}

/// A value which may only be read, never written
pub struct Frozen<T>(T);

impl<T> Frozen<T> {
    /// Create a new frozen value
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

/// Allow frozen values to be referenced
impl<T> Deref for Frozen<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// Allow frozen values to be deserialized
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Frozen<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        T::deserialize(deserializer).map(|value| Frozen::new(value))
    }
}