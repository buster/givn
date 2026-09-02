//! `givn run` — start the product declared in the run block.
//!
//! The library entry point [`execute`] returns the exit code to the caller;
//! only the binary wrapper in `main.rs` converts it with
//! `std::process::exit`. This seam keeps the executor unit-testable
//! in-process (a `process::exit` would kill the test runner).

use crate::cli::run_block::{self, RunFindingKind};
use crate::config::manifest::{Manifest, ProductType, Run};
use crate::config::overlay::load_effective;

/// The resolved, ready-to-execute start plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartPlan {
    /// The start command, executed via `sh -c`.
    pub command: String,
    /// The declared product type.
    pub product_type: ProductType,
    /// Port hand-off: env var name and the resolved port (caller-provided
    /// or freshly allocated).
    pub port_env: Option<(String, u16)>,
    /// HTTP path probed for a 2xx readiness response.
    pub ready_path: Option<String>,
    /// Seconds to wait for readiness.
    pub ready_timeout_s: u64,
}

/// Allocate one ephemeral port by binding `127.0.0.1:0` and dropping the
/// listener. The bind-then-use race is accepted and documented (design §13).
pub fn ephemeral_port() -> u16 {
    let listener =
        std::net::TcpListener::bind(("127.0.0.1", 0)).expect("bind ephemeral port on 127.0.0.1:0");
    listener.local_addr().expect("local addr").port()
}

/// Resolve the start plan from the manifest and the caller's `--port`.
///
/// Preflight findings are returned as `Err` — the caller prints them to
/// stderr (stdout is the executor's machine contract) and exits 1.
pub fn resolve_start_plan(
    manifest: &Manifest,
    caller_port: Option<u16>,
) -> Result<StartPlan, Vec<run_block::RunFinding>> {
    let findings = run_block::check_run(manifest);
    if !findings.is_empty() {
        return Err(findings);
    }
    let run: &Run = &manifest.run;
    let product_type = run
        .product_type
        .expect("check_run rejects an unset product type");

    // A caller port is handed to the product through `run.port_env`; without
    // a hand-off there is nothing to inject it into.
    if caller_port.is_some() && run.port_env.is_none() {
        return Err(vec![run_block::RunFinding {
            kind: RunFindingKind::MissingField,
            message: "--port requires run.port_env — the fixed run.port cannot be overridden"
                .to_string(),
        }]);
    }

    let port_env = match (caller_port, run.port_env.as_deref()) {
        (Some(port), Some(name)) => Some((name.to_string(), port)),
        (None, Some(name)) => Some((name.to_string(), ephemeral_port())),
        _ => None,
    };

    Ok(StartPlan {
        command: run.command.clone(),
        product_type,
        port_env,
        ready_path: run.ready_path.clone(),
        ready_timeout_s: run.ready_timeout_s,
    })
}

/// The line `givn run --print` emits: one sh-eval-able start line.
pub fn print_line(plan: &StartPlan) -> String {
    match &plan.port_env {
        Some((name, port)) => format!("{name}={port} {}", plan.command),
        None => plan.command.clone(),
    }
}

/// The explanation a library product prints: nothing to start.
pub const LIBRARY_LINE: &str = "This project is a library; there is nothing to start.";

/// Library entry point: run the declared product. Returns the exit code.
pub fn execute(caller_port: Option<u16>, print: bool) -> i32 {
    let manifest = match load_effective() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };

    // Preflight: the declaration must be complete before anything starts.
    let preflight = run_block::check_run(&manifest);
    if !preflight.is_empty() {
        for f in &preflight {
            eprintln!("  [{}] {}", run_block::label(f.kind), f.message);
        }
        return 1;
    }

    let run = &manifest.run;
    if run.product_type == Some(ProductType::Library) {
        let sentinel = run.command.trim() == Run::SENTINEL;
        if print && !sentinel {
            println!("{}", run.command);
        } else {
            println!("{LIBRARY_LINE}");
        }
        return 0;
    }

    let plan = match resolve_start_plan(&manifest, caller_port) {
        Ok(plan) => plan,
        Err(findings) => {
            for f in &findings {
                eprintln!("  [{}] {}", run_block::label(f.kind), f.message);
            }
            return 1;
        }
    };

    if print {
        println!("{}", print_line(&plan));
        return 0;
    }

    start_product(&plan)
}

/// Spawn the product, wait for readiness, forward signals. Implemented by
/// the spawn/probe task (T6); the plan-resolution scenarios above never
/// reach it.
fn start_product(plan: &StartPlan) -> i32 {
    let _ = plan;
    unimplemented!("spawn/probe loop lands with the executor loop task")
}
