use crate::filesystem::{Filesystem, FS_LOCAL};
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILENAME: &'static str = "dx.toml";

/// The global configuration
#[derive(Deserialize)]
pub struct Config {
    /// Project configuration, if present
    pub project: Option<ProjectConfig>,

    /// Task configuration
    pub tasks: Vec<TaskConfig>,
}

/// The configuration of the project
#[derive(Deserialize)]
pub struct ProjectConfig {
    pub path: PathBuf,
}

/// The configuration of a specific task
#[derive(Deserialize)]
pub struct TaskConfig {
    /// Name of this task
    pub name: String,
}

impl Config {
    /// Loads the configuration from the user and project config files
    pub fn load() -> Result<Config> {
        let mut config = Config {
            project: None,
            tasks: Vec::new(),
        };

        // read project config
        if let Some(project_config_path) = FS_LOCAL.find_file_ascending(
            &fs::canonicalize(".")?,
            &fs::canonicalize("/")?,
            &CONFIG_FILENAME,
        ) {
            let project_config = ProjectConfig {
                path: project_config_path,
            };

            config.project = Some(project_config);
        }

        Ok(config)
    }
}
