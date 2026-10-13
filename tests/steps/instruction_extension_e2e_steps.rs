//! E2E step definitions for the `instruction-extension` capability.
//!
//! E2E bindings invoke the compiled Givn CLI as a real subprocess: the
//! extension is project-owned configuration, so the fixture writes it and the
//! request path serves it. Step bindings are added scenario by scenario at
//! RED; this module exists so the capability owns exactly one e2e step file
//! (never one shared file).
