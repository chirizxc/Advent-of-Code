use std::{convert::AsRef, path::Path};

use anyhow::Context;

pub fn read_txt<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let dir = Path::new(file!())
        .parent()
        .context("Failed to get parent directory")?;

    std::fs::read_to_string(dir.join(&path))
        .with_context(|| format!("Cannot read file {}", path.as_ref().display()))
}
