pub mod httpapi;
pub mod metaapi;
pub mod rsfdata;

pub static API_VERSION_STRING: &'static str = std::env!("CARGO_PKG_VERSION");