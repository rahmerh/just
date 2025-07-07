use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project: Project,
    pub out_dir: PathBuf,
    pub src_dir: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let file_content = fs::read_to_string("just.toml")
            .with_context(|| "No just.toml file found in project root.")?;

        let config: Config =
            toml::from_str(&file_content).with_context(|| "Could not deserialize config file.")?;

        Ok(config)
    }

    pub fn initialize_in(root_dir: &PathBuf) -> anyhow::Result<Config> {
        let project_name = match root_dir.file_name() {
            Some(f) => f.display().to_string(),
            None => "Just fill this in yourself".to_string(),
        };

        let project = Project { name: project_name };
        let config = Config {
            project,
            out_dir: PathBuf::from("out"),
            src_dir: PathBuf::from("src"),
        };

        Self::save(root_dir.join("just.toml"), &config)?;

        Ok(config)
    }

    fn save(config_file: PathBuf, config: &Config) -> anyhow::Result<()> {
        let contents = toml::to_string_pretty(config)?;
        fs::write(config_file, contents)?;

        Ok(())
    }
}
