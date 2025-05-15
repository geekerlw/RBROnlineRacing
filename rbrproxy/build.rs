use std::path::Path;
use std::{env, path::PathBuf};

#[cfg(target_os = "windows")]
fn main() {
	let library_name = "RBRProxy";
	let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	println!("cargo:rustc-link-lib=static={}", library_name);
	println!("cargo:rustc-link-search=native={}", Path::new(&root).display());
}

#[cfg(not(target_os = "windows"))]
fn main() {

}