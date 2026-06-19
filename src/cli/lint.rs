use anyhow::Result;

/// Static check of .feature files (syntax, @wip, delta tags). Runs NO tests.
///
/// Exit codes (set via process::exit after this returns):
///   0 = clean
///   1 = parse error
///   2 = @wip / PENDING
pub fn run(paths: Vec<String>) -> Result<()> {
    println!("givn lint: not implemented yet");
    let _ = paths;
    Ok(())
}
