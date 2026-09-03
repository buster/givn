//! Timestamped subprocess execution for the long archive gates.
//!
//! `sh_streamed` is the single execution point for shell commands that can
//! run for minutes (gate hooks, verify, coverage). In verbose mode it pipes
//! the child's stdout/stderr and forwards every line with a local-time
//! prefix, so a hang is visible at the exact line it stopped on. Otherwise
//! it inherits stdio, preserving today's unfiltered output.

use std::io::{BufRead, BufReader};
use std::process::{Command, ExitStatus, Stdio};

/// Verbose flag for the current CLI invocation (single-shot process).
static VERBOSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Enable timestamped output forwarding for this invocation.
pub fn set_verbose(verbose: bool) {
    VERBOSE.store(verbose, std::sync::atomic::Ordering::SeqCst);
}

/// Current local time as `HH:MM:SS` (unix), or `??:??:??` elsewhere.
pub fn local_timestamp() -> String {
    #[cfg(unix)]
    {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as libc::time_t)
            .unwrap_or(0);
        let mut tm: libc::tm = unsafe { std::mem::zeroed() };
        let ok = unsafe { libc::localtime_r(&secs, &mut tm) };
        if ok.is_null() {
            return "??:??:??".into();
        }
        format!("{:02}:{:02}:{:02}", tm.tm_hour, tm.tm_min, tm.tm_sec)
    }
    #[cfg(not(unix))]
    {
        "??:??:??".into()
    }
}

/// Run `sh -c <command>` with extra environment entries and an optional
/// working directory. Verbose mode pipes and forwards the child's output
/// with `[HH:MM:SS]` prefixes; otherwise stdio is inherited unchanged.
pub fn sh_streamed(
    command: &str,
    envs: &[(&str, &str)],
    current_dir: Option<&std::path::Path>,
    label: &str,
) -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(command);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    if let Some(dir) = current_dir {
        cmd.current_dir(dir);
    }

    if !VERBOSE.load(std::sync::atomic::Ordering::SeqCst) {
        return cmd.status();
    }

    println!("[{}] === {label} started: {command}", local_timestamp());
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn()?;
    let stdout = child.stdout.take().map(BufReader::new);
    let stderr = child.stderr.take().map(BufReader::new);
    let out_thread = std::thread::spawn(move || forward(stdout));
    let err_thread = std::thread::spawn(move || forward(stderr));
    let status = child.wait()?;
    let _ = out_thread.join();
    let _ = err_thread.join();
    println!(
        "[{}] === {label} finished (exit {})",
        local_timestamp(),
        status.code().unwrap_or(-1)
    );
    Ok(status)
}

fn forward<R: std::io::Read>(reader: Option<BufReader<R>>) {
    let Some(mut reader) = reader else {
        return;
    };
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => println!("[{}] {}", local_timestamp(), line.trim_end_matches('\n')),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_has_clock_shape() {
        let ts = local_timestamp();
        let parts: Vec<&str> = ts.split(':').collect();
        assert_eq!(parts.len(), 3, "{ts}");
        for part in &parts {
            assert_eq!(part.len(), 2, "{ts}");
            assert!(part.chars().all(|c| c.is_ascii_digit()), "{ts}");
        }
    }

    #[test]
    fn non_verbose_passes_status_through() {
        let status = sh_streamed("exit 0", &[], None, "test").expect("run");
        assert!(status.success());
        let status = sh_streamed("exit 3", &[], None, "test").expect("run");
        assert_eq!(status.code(), Some(3));
    }

    #[test]
    fn verbose_mode_forwards_env_and_cwd() {
        set_verbose(true);
        let dir = std::env::temp_dir();
        let status = sh_streamed(
            "test \"$RUNNER_TEST_VAR\" = \"1\" -a -d .",
            &[("RUNNER_TEST_VAR", "1")],
            Some(&dir),
            "test",
        )
        .expect("run");
        set_verbose(false);
        assert!(status.success());
    }
}
