#[cfg(target_os = "windows")]
extern crate winres;

use std::path::Path;
use std::{env, path::PathBuf};

fn main() {
	let library_name = "RBRHacker";
	let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	println!("cargo:rustc-link-lib=static={}", library_name);
	println!("cargo:rustc-link-search=native={}", Path::new(&root).join("rbr").display());

	#[cfg(target_os = "windows")]
	if cfg!(target_os = "windows") {
		let res = winres::WindowsResource::new();
		res.compile().unwrap();
	}
}