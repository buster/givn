//! Step definitions for the `instruction-extension` capability.
//!
//! Step bindings are added scenario by scenario at RED; this module exists so
//! the capability owns exactly one step file (never one shared file). Only
//! mutation, validation, and rejection steps live here; the shared serving
//! assertions live in `serve_extended_rules_steps.rs`.

use crate::harness as common;
use crate::steps::GivnWorld;
use std::fs;

fn env(world: &GivnWorld) -> &common::TestEnv {
    world
        .env
        .as_ref()
        .expect("a project root is required first")
}

pub(crate) fn write_extensions(env: &common::TestEnv, body: &str) {
    let path = env.path("givn/config.yaml");
    let mut config = fs::read_to_string(&path).expect("read givn/config.yaml");
    if !config.ends_with('\n') {
        config.push('\n');
    }
    config.push_str("\nextensions:\n");
    config.push_str(body);
    fs::write(&path, config).expect("write givn/config.yaml");
}

fn remove_extensions(env: &common::TestEnv) {
    let path = env.path("givn/config.yaml");
    let config = fs::read_to_string(&path).expect("read givn/config.yaml");
    let lines: Vec<&str> = config.lines().collect();
    let start = lines
        .iter()
        .rposition(|line| *line == "extensions:")
        .expect("the fixture records an extensions block");
    let mut kept = lines[..start].join("\n");
    kept.push('\n');
    fs::write(&path, kept).expect("write givn/config.yaml");
}

pub(crate) fn replace_extensions(env: &common::TestEnv, body: &str) {
    remove_extensions(env);
    write_extensions(env, body);
}
