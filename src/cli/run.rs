//! `givn run` — start the product declared in the run block.
//!
//! The library entry point [`execute`] returns the exit code to the caller;
//! only the binary wrapper in `main.rs` converts it with
//! `std::process::exit`. This seam keeps the executor unit-testable
//! in-process (a `process::exit` would kill the test runner).

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Command;
use std::sync::atomic::{AtomicI32, Ordering};
use std::time::{Duration, Instant};

use crate::cli::run_block::{self, RunFindingKind};
use crate::config::manifest::{Manifest, ProductType, Run};
use crate::config::overlay::load_effective;

/// The signal caught by the executor's handler, 0 while none arrived.
/// The handler only stores the number (async-signal-safe).
static SIGNAL: AtomicI32 = AtomicI32::new(0);

/// Catch SIGTERM so it can be forwarded to the started product; without
/// this, an interrupt would kill givn and orphan the product.
#[cfg(unix)]
fn install_signal_handlers() {
    extern "C" fn handler(sig: i32) {
        SIGNAL.store(sig, Ordering::SeqCst);
    }
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = handler as *const () as usize;
    action.sa_flags = libc::SA_RESTART;
    unsafe {
        libc::sigaction(libc::SIGINT, &action, std::ptr::null_mut());
        libc::sigaction(libc::SIGTERM, &action, std::ptr::null_mut());
    }
}

#[cfg(not(unix))]
fn install_signal_handlers() {}

/// The resolved, ready-to-execute start plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartPlan {
    /// The start command, executed via `sh -c`.
    pub command: String,
    /// The declared product type.
    pub product_type: ProductType,
    /// The resolved port the product listens on (declared fixed port or the
    /// injected hand-off port). `None` for products without a port.
    pub port: Option<u16>,
    /// Port hand-off: env var name and the resolved port (caller-provided
    /// or freshly allocated).
    pub port_env: Option<(String, u16)>,
    /// HTTP path probed for a 2xx readiness response.
    pub ready_path: Option<String>,
    /// Seconds to wait for readiness.
    pub ready_timeout_s: u64,
}

/// One readiness-probe attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeOutcome {
    /// The ready path answered 2xx on the Nth attempt (1-based).
    Ready { attempts: u32 },
    /// The deadline elapsed without a 2xx response.
    TimedOut,
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
/// stderr (stdout is the executor's machine contract) and exits 1. The
/// caller (`execute`) always preflights before resolving, so no redundant
/// validation happens here.
pub fn resolve_start_plan(
    manifest: &Manifest,
    caller_port: Option<u16>,
) -> Result<StartPlan, Vec<run_block::RunFinding>> {
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
    let port = match (&port_env, run.port) {
        (Some((_, port)), _) => Some(*port),
        (None, Some(port)) => Some(port),
        (None, None) => None,
    };

    Ok(StartPlan {
        command: run.command.clone(),
        product_type,
        port,
        port_env,
        ready_path: run.ready_path.clone(),
        ready_timeout_s: run.ready_timeout_s,
    })
}

/// One readiness probe attempt against `127.0.0.1:port`: an HTTP/1.0 GET on
/// `ready_path`. 2xx accepts; anything else — other statuses, malformed
/// responses, connection refused or reset — is a failed attempt.
pub fn probe_once(port: u16, ready_path: &str) -> bool {
    let Ok(mut stream) = TcpStream::connect_timeout(
        &std::net::SocketAddr::from(([127, 0, 0, 1], port)),
        Duration::from_millis(500),
    ) else {
        return false;
    };
    let request = format!("GET {ready_path} HTTP/1.0\r\nHost: 127.0.0.1:{port}\r\n\r\n");
    if stream.write_all(request.as_bytes()).is_err() {
        return false;
    }
    let mut head = [0u8; 8 * 1024];
    let Ok(n) = stream.read(&mut head) else {
        return false;
    };
    let head = String::from_utf8_lossy(&head[..n]);
    let Some(status) = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|code| code.parse::<u16>().ok())
    else {
        return false;
    };
    (200..300).contains(&status)
}

/// Poll the readiness probe every 100 ms until it accepts or the deadline
/// elapses. Returns the attempt count for the retry scenarios.
pub fn probe_until_ready(port: u16, ready_path: &str, timeout: Duration) -> ProbeOutcome {
    let deadline = Instant::now() + timeout;
    let mut attempts = 0;
    loop {
        attempts += 1;
        if probe_once(port, ready_path) {
            return ProbeOutcome::Ready { attempts };
        }
        if Instant::now() >= deadline {
            return ProbeOutcome::TimedOut;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
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

/// Spawn the product via `sh -c`, wait for readiness (server products),
/// and forward the child's exit code.
fn start_product(plan: &StartPlan) -> i32 {
    #[cfg(unix)]
    use std::os::unix::process::CommandExt;

    install_signal_handlers();
    let mut command = Command::new("sh");
    command.arg("-c").arg(&plan.command);
    // Own process group: the declared command is a shell line, so killing
    // `sh` alone would orphan its grandchildren (servers keep the inherited
    // stdio pipes open forever). The group id equals the child's pid.
    #[cfg(unix)]
    unsafe {
        command.pre_exec(|| {
            libc::setpgid(0, 0);
            Ok(())
        });
    }
    if let Some((name, port)) = &plan.port_env {
        command.env(name, port.to_string());
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            eprintln!("givn run: failed to start {}: {e}", plan.command);
            return 1;
        }
    };

    let is_server = plan.product_type == ProductType::Server;
    let deadline = Instant::now() + Duration::from_secs(plan.ready_timeout_s);
    let mut ready = false;
    loop {
        // A caught signal is forwarded before anything else is inspected,
        // so givn always answers with 128+sig even if the product dies
        // from the very same signal.
        let caught = SIGNAL.load(Ordering::SeqCst);
        if caught != 0 {
            // The declared command runs under `sh`, which does not always
            // exec it in place — forward to the child's whole process
            // group so the product itself receives the signal.
            #[cfg(unix)]
            unsafe {
                libc::kill(-(child.id() as i32), caught);
            }
            #[cfg(not(unix))]
            let _ = child.kill();
            // Reaping is bounded: a product that ignores the signal is
            // killed after the grace period, so givn never waits forever.
            let grace = Instant::now() + Duration::from_secs(2);
            loop {
                if matches!(child.try_wait(), Ok(Some(_))) {
                    break;
                }
                if Instant::now() >= grace {
                    #[cfg(unix)]
                    unsafe {
                        libc::kill(-(child.id() as i32), libc::SIGKILL);
                    }
                    #[cfg(not(unix))]
                    let _ = child.kill();
                    let _ = child.wait();
                    break;
                }
                std::thread::sleep(Duration::from_millis(20));
            }
            return 128 + caught;
        }
        if let Ok(Some(status)) = child.try_wait() {
            // The child's exit code is forwarded exactly — early exits
            // before readiness and cli exits alike.
            return status.code().unwrap_or(1);
        }
        if is_server && !ready {
            let port = plan.port.expect("a server plan carries a resolved port");
            let ready_path = plan.ready_path.as_deref().expect("R3 requires ready_path");
            // One probe attempt per tick: a zero deadline degenerates
            // probe_until_ready to a single attempt.
            if matches!(
                probe_until_ready(port, ready_path, Duration::ZERO),
                ProbeOutcome::Ready { .. }
            ) {
                ready = true;
                println!("givn run: ready at http://127.0.0.1:{port}{ready_path}");
            } else if !ready && Instant::now() >= deadline {
                // Kill the whole group: `sh` plus everything it started.
                #[cfg(unix)]
                unsafe {
                    libc::kill(-(child.id() as i32), libc::SIGKILL);
                }
                #[cfg(not(unix))]
                let _ = child.kill();
                let _ = child.wait();
                return 2;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
