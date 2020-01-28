#[macro_use] extern crate log;
pub mod view;

#[cfg(target_os="linux")]
pub mod gl;

#[cfg(target_arch="wasm32")]
pub mod webgl;

pub use view::*;