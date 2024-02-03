use std::path::Path;
use std::{env, path::PathBuf};

fn main() {
	let library_name = "RBRHacker";
	let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	println!("cargo:rustc-link-lib=static={}", library_name);
	println!("cargo:rustc-link-search=native={}", Path::new(&root).join("rbr").display());
}