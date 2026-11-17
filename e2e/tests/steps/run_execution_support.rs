//! Shared harness for the run-execution capability: port reservation,
//! ready-fixture location, fixture-declaration writers, canned-response
//! listeners, and (with the signal task) the spawn-and-interrupt helpers.
//! Step definitions live in `run_execution_steps.rs` /
//! `run_execution_e2e_steps.rs`.

use super::GivnWorld;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

pub(crate) fn reserve_free_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("reserve free port");
    listener.local_addr().expect("local addr").port()
}

pub(crate) fn wait_for_listener(port: u16) {
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while std::time::Instant::now() < deadline {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("responder listener never came up on port {port}");
}

/// The built ``ready_fixture`` binary the project runner exported.
pub(crate) fn ready_fixture_path() -> PathBuf {
    crate::harness::binary::ready_fixture()
}

fn scan_for_fixture(dir: &std::path::Path) -> Option<PathBuf> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::read_dir(dir)
        .ok()?
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("ready_fixture"))
                && p.metadata()
                    .map(|m| m.permissions().mode() & 0o111 != 0)
                    .unwrap_or(false)
        })
        .max_by_key(|p| p.metadata().and_then(|m| m.modified()).ok())
}

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
    let env = world.env.get_or_insert_with(crate::harness::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::harness::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command.to_string());
    world.run_fixed_port = fixed_port;
}

pub(crate) fn write_library_declaration(world: &mut GivnWorld, command: &str) {
    let declaration = format!("run:\n  command: \"{command}\"\n  product_type: library\n");
    let env = world.env.get_or_insert_with(crate::harness::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::harness::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command.to_string());
    world.run_fixed_port = None;
}

pub(crate) struct Responder {
    stop: Arc<AtomicBool>,
    status: Arc<AtomicU16>,
    connections: Arc<AtomicU32>,
    handle: Option<JoinHandle<()>>,
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

#[derive(Debug)]
pub(crate) struct InProcStart {
    pub port: u16,
    pub marker: std::path::PathBuf,
    pub prev_cwd: std::path::PathBuf,
    pub handle: Option<std::thread::JoinHandle<i32>>,
    pub joined: Option<i32>,
}
