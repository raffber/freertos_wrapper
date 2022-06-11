use freertos_build::{build_freertos, config::Config, ports::cortex_m4f};
use std::{env, path::PathBuf};

fn main() {
    let config = Config::new(100_000_000).build();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    std::fs::create_dir_all(&out_dir).unwrap();
    build_freertos(&cortex_m4f(), &config, &out_dir).unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=freertos");
}
