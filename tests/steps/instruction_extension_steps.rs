//! Step definitions for the `instruction-extension` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). Only
//! mutation, validation, and rejection steps live here; the shared serving
//! assertions live in `serve_extended_rules_steps.rs`.
