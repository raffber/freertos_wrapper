use anyhow::{anyhow, Result};
use config::Config;
use std::{
    env,
    path::{Path, PathBuf},
};
mod config;
pub mod ports;

pub struct Port {
    pub target_triplet: String,
    pub directory: PathBuf,
    pub sources: Vec<PathBuf>,
}

fn kernel_sources() -> Vec<PathBuf> {
    vec![
        "croutine.c".into(),
        "event_groups.c".into(),
        "list.c".into(),
        "queue.c".into(),
        "stream_buffer.c".into(),
        "tasks.c".into(),
        "timers.c".into(),
    ]
}

fn get_workspace_dir() -> Result<PathBuf> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dir = manifest_dir
        .parent()
        .ok_or_else(|| anyhow!("Invalid build file tree."))?;
    Ok(workspace_dir.to_owned())
}

struct BuildFiles {
    sources: Vec<PathBuf>,
    includes: Vec<PathBuf>,
}

fn get_source_paths(port: &Port) -> Result<BuildFiles> {
    let mut kernel_dir = get_workspace_dir()?;
    kernel_dir.push("FreeRTOS-Kernel");

    let kernel_sources: Vec<_> = kernel_sources()
        .drain(..)
        .map(|x| {
            let mut source_file = PathBuf::from(&kernel_dir);
            source_file.push(x);
            source_file
        })
        .collect();

    let mut port_directory = kernel_dir.clone();
    port_directory.push("portable");
    port_directory.push("GCC");
    port_directory.push(port.directory.clone());
    let port_sources: Vec<_> = port
        .sources
        .iter()
        .map(|x| {
            let mut source_file = PathBuf::from(&port_directory);
            source_file.push(x);
            source_file
        })
        .collect();

    let mut sources = kernel_sources;
    sources.extend(port_sources);

    let mut kernel_include_dir = kernel_dir.clone();
    kernel_include_dir.push("include");

    let ret = BuildFiles {
        sources,
        includes: vec![kernel_dir, kernel_include_dir, port_directory],
    };
    Ok(ret)
}

fn build_freertos(port: &Port, config: &Config, target_dir: &Path) -> Result<()> {
    let sources = get_source_paths(port)?;

    let mut target_dir = PathBuf::from(target_dir);
    target_dir.push("freertos");
    std::fs::create_dir_all(&target_dir).unwrap();
    let mut config_file = target_dir.clone();
    config_file.push("FreeRTOSConfig.h");
    std::fs::write(&config_file, config.render()?)?;

    let mut build = cc::Build::new();
    println!("Sources: {:?}", sources.sources);
    println!("Includes: {:?}", sources.includes);
    build.files(&sources.sources);
    build.includes(&sources.includes);
    build.include(&target_dir);
    build.target(&port.target_triplet);
    build.out_dir(target_dir);
    println!("HIA!!!!");
    build.compile("freertos");

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
