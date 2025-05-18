#[cfg(target_os = "windows")]
extern crate winres;

fn main() {
	#[cfg(target_os = "windows")]
	if cfg!(target_os = "windows") {
		let res = winres::WindowsResource::new();
		res.compile().unwrap();
	}
}