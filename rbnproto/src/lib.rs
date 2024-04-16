use serde::{Deserialize, Serialize};

pub mod httpapi;
pub mod metaapi;
pub mod rsfdata;

pub static API_VERSION_STRING: &'static str = std::env!("CARGO_PKG_VERSION");


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct D3DMATRIX {
    pub m: [[f32; 4]; 4],
}