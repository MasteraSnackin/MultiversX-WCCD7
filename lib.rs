elrond_wasm::imports!();

pub mod contract;
pub mod storage;
pub mod endpoints;
pub mod utils;

elrond_wasm::derive_imports!();