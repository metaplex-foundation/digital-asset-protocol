extern crate core;

mod api;
mod entrypoint;
mod interfaces;
mod lifecycle;
mod generated;
mod modules;
pub mod blob;
pub mod module;

#[macro_use]
pub mod validation;

pub use solana_program;

solana_program::declare_id!("assetbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
