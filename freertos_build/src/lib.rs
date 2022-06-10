use anyhow::{anyhow, Result};
use config::Config;
use std::{
    env,
    path::{Path, PathBuf},
};
mod config;
pub mod ports;

pub struct Port {
    target_triplet: String,
    directory: String,
    sources: Vec<String>,
}

fn kernel_sources() -> Vec<String> {
    vec![
        "croutine.c".to_string(),
        "event_groups.c".to_string(),
        "list.c".to_string(),
        "queue.c".to_string(),
        "stream_buffer.c".to_string(),
        "tasks.c".to_string(),
        "timers.c".to_string(),
    ]
}

fn get_workspace_dir() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dir = manifest_dir
        .parent()
        .ok_or_else(|| anyhow!("Invalid build file tree."))?;
    Ok(workspace_dir.to_owned())
}

fn get_source_paths(port: &Port) -> Result<Vec<PathBuf>> {
    let mut all_sources = kernel_sources();
    all_sources.extend_from_slice(&port.sources);

    let mut kernel_dir = get_workspace_dir()?;
    kernel_dir.push("FreeRTOS-Kernel");

    let ret = all_sources
        .drain(..)
        .map(|x| {
            let mut source_file = PathBuf::from(&kernel_dir);
            source_file.push(x);
            source_file
        })
        .collect();
    Ok(ret)
}

fn build_freertos(port: &Port, config: &Config, target_dir: &Path) -> Result<()> {
    let sources = get_source_paths(port);

    let mut target_dir = PathBuf::from(target_dir);
    target_dir.push("freertos");
    std::fs::create_dir_all(&target_dir).unwrap();
    target_dir.push("FreeRTOSConfig.h");
    std::fs::write(&target_dir, config.render()?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{config::ConfigBuilder, ports::cortex_m4f};

    use super::*;

    #[test]
    fn test_build() {
        let config = Config::new(100_000_000).build();
        let mut tmp_dir = get_workspace_dir().unwrap();
        tmp_dir.push("tmp");
        std::fs::create_dir_all(&tmp_dir).unwrap();
        build_freertos(&cortex_m4f(), &config, &tmp_dir).unwrap();
    }
}
