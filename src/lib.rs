//! givn library crate — exported for integration test access.
//!
//! Modules mirror `main.rs` but with `pub` visibility so that integration
//! tests (especially the Gherkin runner) can import and test internal
//! subsystems at module boundaries (e.g. `config::overlay::apply`).

pub mod cli;
pub mod cobertura;
pub mod config;
pub mod error;
pub mod gherkin;
pub mod hooks;
pub mod integrity_hook;
pub mod overlap;
pub mod skills;
pub mod spec;
pub mod state;
