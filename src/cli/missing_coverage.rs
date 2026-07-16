/// Sentinel for `coverage.command` — prints setup guidance and exits non-zero.
///
/// This is the default value of `coverage.command` in the manifest. When the
/// user hasn't configured a real coverage merge script, running a coverage
/// hook will trigger this sentinel, ensuring coverage never falsely reports
/// GREEN.
use anyhow::{Result, bail};

pub fn run() -> Result<()> {
    eprintln!();
    eprintln!("  ⚠  No coverage merge script configured.");
    eprintln!();
    eprintln!("  The coverage lifecycle artifact requires a real script to merge");
    eprintln!("  test coverage data from verify (+ verify-e2e, if configured).");
    eprintln!();
    eprintln!("  To configure it, edit givn/config.yaml:");
    eprintln!();
    eprintln!("      coverage:");
    eprintln!("        command: \"<your coverage merge command>\"");
    eprintln!();
    eprintln!("  The script receives GIVN_FEATURES and should:");
    eprintln!("    1. Locate coverage data produced by verify");
    eprintln!("    2. Compare against the project coverage baseline");
    eprintln!("    3. Print a unified report to stdout");
    eprintln!("    4. Exit 0 if coverage is OK, non-zero on regression");
    eprintln!();
    eprintln!("  Examples:");
    eprintln!("    scripts/merge-coverage.sh");
    eprintln!("    cargo llvm-cov report --lcov-path coverage/lcov.info");
    eprintln!("    nyc report --reporter=text-summary");
    eprintln!();
    bail!("coverage.command not configured")
}
