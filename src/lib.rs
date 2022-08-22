extern crate core;

mod api;
pub mod blob;
mod entrypoint;
mod generated;
mod interfaces;
mod lifecycle;
pub mod module;
mod modules;

#[macro_use]
pub mod validation;

pub use solana_program;

solana_program::declare_id!("assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
