use freertos_build::{build_freertos, config::Config, ports::cortex_m4f};
use std::{env, fs::File, io::Write, path::PathBuf};

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

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");
}
