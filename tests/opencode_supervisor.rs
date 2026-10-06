//! Harness-free owner for an OpenCode fixture process group.
//!
//! The test runner keeps this process's stdin open as a lease. The runtime
//! driver and all of its descendants occupy a separate process group, so a
//! runner panic, exit, or signal cannot leave fixture services behind.

use std::io;
use std::process::{Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

static SIGNAL: AtomicI32 = AtomicI32::new(0);

fn main() -> ExitCode {
    if std::env::args().any(|arg| arg == "--list") {
        println!("opencode_supervisor: test");
        return ExitCode::SUCCESS;
    }
    if let Err(error) = install_handlers() {
        eprintln!("OpenCode supervisor setup failure: installing signal handlers: {error}");
        return ExitCode::FAILURE;
    }
    if let Err(error) = run() {
        eprintln!("OpenCode supervisor failure: {error:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> anyhow::Result<()> {
    use anyhow::{Context, bail};

    let mut args = std::env::args().skip(1);
    let mut deadline = Duration::from_secs(60);
    let mut root = None;
    let mut command = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--timeout-seconds" => {
                let value = args.next().context("--timeout-seconds needs a value")?;
                deadline = Duration::from_secs(value.parse().context("invalid timeout")?);
            }
            "--root" => root = args.next().map(std::path::PathBuf::from),
            "--" => {
                command.extend(args);
                break;
            }
            other => bail!("unknown option {other:?}"),
        }
    }
    let Some(program) = command.first() else {
        bail!("expected -- <fixture driver argv...>");
    };
    let root = root.context("--root is required")?;
    if !root.is_absolute()
        || !root
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("givn-opencode-"))
    {
        bail!("refusing non-owned fixture root {}", root.display());
    }

    #[cfg(target_os = "linux")]
    enable_subreaper()?;

    let mut child_command = Command::new(program);
    child_command
        .args(&command[1..])
        .current_dir(&root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        child_command.process_group(0);
    }
    let mut child = child_command.spawn().context("spawning fixture driver")?;
    let pid = child.id() as libc::pid_t;
    let pgid = pid;
    eprintln!("fixture process group started: leader_pid={pid} pgid={pgid} role=driver");

    let started = Instant::now();
    let mut status = None;
    let mut reason = None;
    loop {
        if let Some(exit) = child.try_wait().context("checking fixture driver")? {
            status = Some(exit);
            break;
        }
        if SIGNAL.load(Ordering::SeqCst) != 0 {
            reason = Some(format!("received signal {}", SIGNAL.load(Ordering::SeqCst)));
            break;
        }
        if started.elapsed() >= deadline {
            reason = Some(format!(
                "fixture deadline of {}s expired",
                deadline.as_secs()
            ));
            break;
        }
        if lease_reached_eof()? {
            reason = Some("parent lease closed".to_string());
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    if let Some(ref reason) = reason {
        eprintln!("stopping fixture process group: {reason}; pgid={pgid} signal=SIGTERM");
        signal_group(pgid, libc::SIGTERM)?;
        let term_started = Instant::now();
        while term_started.elapsed() < Duration::from_secs(3) {
            if status.is_none() {
                status = child.try_wait().context("waiting after SIGTERM")?;
            }
            if !group_exists(pgid)? {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        if group_exists(pgid)? {
            eprintln!("fixture process group escalation: pgid={pgid} signal=SIGKILL");
            signal_group(pgid, libc::SIGKILL)?;
        }
    }

    if status.is_none() {
        status = Some(child.wait().context("reaping fixture driver")?);
    }
    #[cfg(target_os = "linux")]
    reap_adopted()?;

    let gone_started = Instant::now();
    while group_exists(pgid)? && gone_started.elapsed() < Duration::from_secs(5) {
        #[cfg(target_os = "linux")]
        reap_adopted()?;
        std::thread::sleep(Duration::from_millis(50));
    }
    if group_exists(pgid)? {
        bail!("owned fixture process group {pgid} survived teardown");
    }
    eprintln!("fixture process group reaped: leader_pid={pid} pgid={pgid}");

    if let Some(reason) = reason {
        let _ = std::fs::remove_dir_all(&root);
        bail!("{reason}");
    }
    let status = status.expect("child status set");
    if !status.success() {
        let _ = std::fs::remove_dir_all(&root);
        bail!("fixture driver exited with {status}");
    }
    std::fs::remove_dir_all(&root).context("removing owned fixture root")?;
    Ok(())
}

fn install_handlers() -> io::Result<()> {
    extern "C" fn handler(signal: libc::c_int) {
        SIGNAL.store(signal, Ordering::SeqCst);
    }
    unsafe {
        let mut action: libc::sigaction = std::mem::zeroed();
        action.sa_sigaction = handler as *const () as usize;
        action.sa_flags = 0;
        libc::sigemptyset(&mut action.sa_mask);
        if libc::sigaction(libc::SIGINT, &action, std::ptr::null_mut()) != 0
            || libc::sigaction(libc::SIGTERM, &action, std::ptr::null_mut()) != 0
        {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn lease_reached_eof() -> io::Result<bool> {
    let mut pollfd = libc::pollfd {
        fd: libc::STDIN_FILENO,
        events: libc::POLLIN | libc::POLLHUP,
        revents: 0,
    };
    let result = unsafe { libc::poll(&mut pollfd, 1, 0) };
    if result < 0 {
        return Err(io::Error::last_os_error());
    }
    if result == 0 || pollfd.revents & libc::POLLIN == 0 && pollfd.revents & libc::POLLHUP == 0 {
        return Ok(false);
    }
    let mut byte = 0_u8;
    let count = unsafe { libc::read(libc::STDIN_FILENO, (&mut byte as *mut u8).cast(), 1) };
    if count < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(count == 0)
}

fn signal_group(pgid: libc::pid_t, signal: libc::c_int) -> io::Result<()> {
    if unsafe { libc::kill(-pgid, signal) } == 0 {
        return Ok(());
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        Ok(())
    } else {
        Err(error)
    }
}

fn group_exists(pgid: libc::pid_t) -> io::Result<bool> {
    if unsafe { libc::kill(-pgid, 0) } == 0 {
        return Ok(true);
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        Ok(false)
    } else if error.raw_os_error() == Some(libc::EPERM) {
        Ok(true)
    } else {
        Err(error)
    }
}

#[cfg(target_os = "linux")]
fn enable_subreaper() -> anyhow::Result<()> {
    if unsafe { libc::prctl(libc::PR_SET_CHILD_SUBREAPER, 1, 0, 0, 0) } != 0 {
        return Err(anyhow::anyhow!(
            "PR_SET_CHILD_SUBREAPER: {}",
            io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn reap_adopted() -> anyhow::Result<()> {
    loop {
        let mut status = 0;
        let pid = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
        if pid > 0 {
            eprintln!("reaped adopted fixture descendant pid={pid}");
            continue;
        }
        if pid == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ECHILD) {
            return Ok(());
        }
        if error.kind() == io::ErrorKind::Interrupted {
            continue;
        }
        return Err(anyhow::anyhow!("waitpid adopted descendants: {error}"));
    }
}
