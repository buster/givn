//! Shared harness for the run-execution capability: port reservation,
//! ready-fixture location, fixture-declaration writers, canned-response
//! listeners, and (with the signal task) the spawn-and-interrupt helpers.
//! Step definitions live in `run_execution_steps.rs` /
//! `run_execution_e2e_steps.rs`.

use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use super::GivnWorld;

/// Reserve one free port (bind 127.0.0.1:0, read, drop). The bind-then-use
/// race is the accepted, documented mitigation (design §13).
pub(crate) fn reserve_free_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("reserve free port");
    listener.local_addr().expect("local addr").port()
}

/// Locate (or build) the harness-free `ready_fixture` test binary. The
/// build-on-demand mirrors the `release_givn_binary` precedent.
pub(crate) fn ready_fixture_path() -> PathBuf {
    let deps = std::env::current_exe()
        .expect("current exe")
        .parent()
        .expect("exe parent")
        .to_path_buf();
    if let Some(path) = scan_for_fixture(&deps) {
        return path;
    }
    let status = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
        .args(["build", "--test", "ready_fixture"])
        .status()
        .expect("build ready_fixture");
    assert!(status.success(), "ready_fixture build failed");
    scan_for_fixture(&deps).expect("ready_fixture binary after build")
}

fn scan_for_fixture(dir: &std::path::Path) -> Option<PathBuf> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("ready_fixture"))
                && p.metadata()
                    .map(|m| m.permissions().mode() & 0o111 != 0)
                    .unwrap_or(false)
        })
}

/// The fixture command for a server that binds the port givn hands it
/// (through `port_env`) and answers 2xx on the ready path.
pub(crate) fn serve_command(ready_path: &str, extra: &[&str]) -> String {
    let mut parts = vec![
        ready_fixture_path().to_string_lossy().into_owned(),
        "serve".to_string(),
        "--ready-path".to_string(),
        ready_path.to_string(),
    ];
    parts.extend(extra.iter().map(|s| s.to_string()));
    parts.join(" ")
}

/// Write a server run declaration into the fixture project.
pub(crate) fn write_server_declaration(
    world: &mut GivnWorld,
    command: &str,
    port_env: Option<&str>,
    fixed_port: Option<u16>,
    ready_path: &str,
    ready_timeout_s: u64,
) {
    let mut declaration = format!("run:\n  command: \"{command}\"\n  product_type: server\n");
    if let Some(env_name) = port_env {
        declaration.push_str(&format!("  port_env: \"{env_name}\"\n"));
    }
    if let Some(port) = fixed_port {
        declaration.push_str(&format!("  port: {port}\n"));
    }
    declaration.push_str(&format!("  ready_path: \"{ready_path}\"\n"));
    declaration.push_str(&format!("  ready_timeout_s: {ready_timeout_s}\n"));
    let env = world.env.get_or_insert_with(crate::common::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::common::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command.to_string());
    world.run_fixed_port = fixed_port;
}

/// Write a library run declaration carrying a real (non-sentinel) command.
pub(crate) fn write_library_declaration(world: &mut GivnWorld, command: &str) {
    let declaration = format!("run:\n  command: \"{command}\"\n  product_type: library\n");
    let env = world.env.get_or_insert_with(crate::common::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::common::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command.to_string());
    world.run_fixed_port = None;
}

/// A canned HTTP responder on 127.0.0.1: the answer status can be flipped
/// while the probe is polling.
pub(crate) struct Responder {
    stop: Arc<AtomicBool>,
    status: Arc<AtomicU16>,
    connections: Arc<AtomicU32>,
    handle: Option<JoinHandle<()>>,
}

impl std::fmt::Debug for Responder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Responder")
            .field("stopped", &self.stop.load(Ordering::SeqCst))
            .field("status", &self.status.load(Ordering::SeqCst))
            .finish()
    }
}

impl Responder {
    pub(crate) fn set_status(&self, status: u16) {
        self.status.store(status, Ordering::SeqCst);
    }

    /// Block until the responder has served `n` connections, so a status
    /// flip cannot race the probe's first attempt.
    pub(crate) fn wait_for_connections(&self, n: u32, timeout: Duration) {
        let deadline = std::time::Instant::now() + timeout;
        while self.connections.load(Ordering::SeqCst) < n {
            assert!(
                std::time::Instant::now() < deadline,
                "responder never served {n} connection(s)"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    pub(crate) fn stop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for Responder {
    fn drop(&mut self) {
        self.stop();
    }
}

pub(crate) fn spawn_responder(port: u16, status: u16) -> Responder {
    let stop = Arc::new(AtomicBool::new(false));
    let status = Arc::new(AtomicU16::new(status));
    let connections = Arc::new(AtomicU32::new(0));
    let stop_thread = Arc::clone(&stop);
    let status_thread = Arc::clone(&status);
    let connections_thread = Arc::clone(&connections);
    let handle = std::thread::spawn(move || {
        let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) else {
            return;
        };
        let _ = listener.set_nonblocking(true);
        while !stop_thread.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    connections_thread.fetch_add(1, Ordering::SeqCst);
                    let mut buf = [0u8; 2048];
                    let n = std::io::Read::read(&mut stream, &mut buf).unwrap_or(0);
                    let head = String::from_utf8_lossy(&buf[..n]);
                    let _ = std::io::Write::write_all(
                        &mut stream,
                        format!(
                            "HTTP/1.0 {} X\r\nContent-Length: 0\r\n\r\n",
                            status_thread.load(Ordering::SeqCst)
                        )
                        .as_bytes(),
                    );
                    let _ = head;
                }
                Err(_) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
    });
    Responder {
        stop,
        status,
        connections,
        handle: Some(handle),
    }
}

/// An in-process `givn run` execution: the library entry point runs on a
/// background thread with the fixture project as the process cwd (restored
/// by the asserting steps).
#[derive(Debug)]
pub(crate) struct InProcStart {
    pub port: u16,
    pub marker: std::path::PathBuf,
    pub prev_cwd: std::path::PathBuf,
    pub handle: Option<std::thread::JoinHandle<i32>>,
    pub joined: Option<i32>,
}

impl Drop for InProcStart {
    fn drop(&mut self) {
        // Never leak a scenario cwd switch, even on failure paths.
        let _ = std::env::set_current_dir(&self.prev_cwd);
    }
}
