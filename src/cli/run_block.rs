//! Validation of the run declaration (`givn/commands.yaml`, key `run`).
//!
//! `check_run` is deterministic: every finding is a provable property of the
//! declaration itself — never a heuristic about the product it starts.

use crate::config::manifest::{Manifest, ProductType, Run};

/// The kind of a run-declaration finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunFindingKind {
    /// The declaration was never filled in (sentinel/absent fields).
    Unconfigured,
    /// A required field for the declared product type is missing.
    MissingField,
    /// A field is set that the declared product type forbids.
    ForbiddenField,
    /// The start command provably does nothing.
    NoOpCommand,
}

/// One deterministic problem with the run declaration.
#[derive(Debug, Clone)]
pub struct RunFinding {
    pub kind: RunFindingKind,
    pub message: String,
}

/// Two-letter-kind label used in reports, matching the finding-report style
/// of the README check (`"OPEN "`, `"REMN "`).
pub fn label(kind: RunFindingKind) -> &'static str {
    match kind {
        RunFindingKind::Unconfigured => "UNCF ",
        RunFindingKind::MissingField => "MISS ",
        RunFindingKind::ForbiddenField => "FORB ",
        RunFindingKind::NoOpCommand => "NOOP ",
    }
}

/// First tokens of start commands that provably start nothing.
const NO_OP_COMMANDS: &[&str] = &["true", ":", "echo", "printf", "sleep", "cat", "test", "["];

/// Validate the run declaration. An empty result means the declaration is
/// complete and internally consistent.
pub fn check_run(manifest: &Manifest) -> Vec<RunFinding> {
    let run = &manifest.run;
    let mut findings = Vec::new();

    let is_library = run.product_type == Some(ProductType::Library);

    // R1 — command unset. A library may keep the sentinel (R10 exception).
    if !is_library && (run.command.trim().is_empty() || run.command.trim() == Run::SENTINEL) {
        findings.push(RunFinding {
            kind: RunFindingKind::Unconfigured,
            message: "run.command is unset — set the command that starts the product".to_string(),
        });
    }

    // R9 — a provably no-op start command. Runs regardless of the product
    // type, so it sits before the type-dependent block below.
    {
        let trimmed = run.command.trim();
        let first_token = trimmed.split_whitespace().next().unwrap_or(trimmed);
        if !trimmed.is_empty() && (NO_OP_COMMANDS.contains(&first_token) || trimmed == "exit 0") {
            findings.push(RunFinding {
                kind: RunFindingKind::NoOpCommand,
                message: format!(
                    "run.command '{trimmed}' does nothing — it cannot start the product"
                ),
            });
        }
    }

    // R2 — product type unset; type-dependent checks are skipped without one.
    if run.product_type.is_none() {
        findings.push(RunFinding {
            kind: RunFindingKind::Unconfigured,
            message: "run.product_type is unset — one of server, cli, library".to_string(),
        });
        return findings;
    }

    // R3 — a server must declare where readiness is reported.
    if run.product_type == Some(ProductType::Server) && run.ready_path.is_none() {
        findings.push(RunFinding {
            kind: RunFindingKind::MissingField,
            message: "product_type server requires run.ready_path".to_string(),
        });
    }

    // R4 — a server needs exactly one port source.
    if run.product_type == Some(ProductType::Server) && run.port.is_none() && run.port_env.is_none()
    {
        findings.push(RunFinding {
            kind: RunFindingKind::MissingField,
            message: "product_type server requires exactly one of run.port or run.port_env"
                .to_string(),
        });
    }

    // R5 — the two port sources are mutually exclusive.
    if run.product_type == Some(ProductType::Server) && run.port.is_some() && run.port_env.is_some()
    {
        findings.push(RunFinding {
            kind: RunFindingKind::ForbiddenField,
            message: "run.port and run.port_env are mutually exclusive".to_string(),
        });
    }

    // R6 — server-only fields are forbidden for cli and library products.
    if matches!(
        run.product_type,
        Some(ProductType::Cli) | Some(ProductType::Library)
    ) {
        for (set, field) in [
            (run.port.is_some(), "run.port"),
            (run.port_env.is_some(), "run.port_env"),
            (run.ready_path.is_some(), "run.ready_path"),
        ] {
            if set {
                findings.push(RunFinding {
                    kind: RunFindingKind::ForbiddenField,
                    message: format!("{field} applies to product_type server only"),
                });
            }
        }
    }

    // R7 — a server's readiness wait must be positive.
    if run.product_type == Some(ProductType::Server) && run.ready_timeout_s == 0 {
        findings.push(RunFinding {
            kind: RunFindingKind::MissingField,
            message: "run.ready_timeout_s must be greater than zero".to_string(),
        });
    }

    // R8 — the readiness path must be absolute so callers can probe it.
    if run.product_type == Some(ProductType::Server)
        && run
            .ready_path
            .as_deref()
            .is_some_and(|p| !p.starts_with('/'))
    {
        findings.push(RunFinding {
            kind: RunFindingKind::MissingField,
            message: "run.ready_path must be an absolute path, e.g. /healthz".to_string(),
        });
    }

    findings
}
