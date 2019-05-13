use openssl_src;
use std::path::PathBuf;

pub fn get_openssl(_target: &str) -> (PathBuf, PathBuf) {
    let artifacts = openssl_src::Build::new().build();
    println!("cargo:vendored=1");
    (
        artifacts.lib_dir().to_path_buf(),
        artifacts.include_dir().to_path_buf(),
    )
}