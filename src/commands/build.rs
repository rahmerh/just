use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, bail};
use clap::Args;

use crate::config::Config;

#[derive(Args)]
pub struct Build;

impl Build {
    pub fn run(&self, run_in: &PathBuf, config: Config) -> anyhow::Result<()> {
        let src_dir = run_in.join(config.src_dir);
        let out_dir = run_in.join(config.out_dir);

        fs::create_dir_all(&out_dir).with_context(|| "Failed to create output directory")?;

        let sources = Self::find_java_sources(&src_dir)?;

        if sources.is_empty() {
            bail!("No Java source files found in {:?}", src_dir);
        }

        let status = Command::new("javac")
            .arg("-d")
            .arg(&out_dir)
            .args(&sources)
            .status()
            .context("Failed to spawn javac")?;

        if !status.success() {
            anyhow::bail!("javac exited with non-zero status: {}", status);
        }

        println!(
            "Build complete for {} in {}",
            config.project.name,
            out_dir.display()
        );

        Ok(())
    }

    fn find_java_sources(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut files = vec![];

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                files.extend(Self::find_java_sources(&path)?);
            } else if path.extension().and_then(|e| e.to_str()) == Some("java") {
                files.push(path);
            }
        }

        Ok(files)
    }
}
