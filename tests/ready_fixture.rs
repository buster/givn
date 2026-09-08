//! A hermetic HTTP-ready fixture process for run-execution tests.
//!
//! Built as a harness-free test target: never installed, never shipped.
//! With no arguments (how `cargo test` / cargo-nextest invoke it) it exits
//! 0 immediately, so it never hangs a test harness.
//!
//! Modes:
//! - `serve [--port N | $PORT] [--ready-path P] [--exit-after N]
//!   [--never-ready] [--trap-marker PATH]` — bind 127.0.0.1, answer 2xx on
//!   the ready path (404 otherwise or with --never-ready), optionally exit
//!   cleanly after N successful readiness responses, and optionally write a
//!   marker file when SIGINT/SIGTERM arrives (async-signal-safe handler:
//!   the handler only stores the signal number; the serve loop observes it
//!   and writes the marker).
//! - `fail --code N` — exit N immediately.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::time::Duration;

static SIGNUM: AtomicI32 = AtomicI32::new(0);

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--list") {
        // cargo-nextest discovery: single-unit harness, nothing ignored.
        if !args.iter().any(|a| a == "--ignored") {
            println!("ready_fixture: test");
        }
        return;
    }
    if args.is_empty() {
        return;
    }
    match args[0].as_str() {
        "serve" => serve(&args[1..]),
        "fail" => fail(&args[1..]),
        _ => {}
    }
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn fail(args: &[String]) -> ! {
    let code: i32 = arg_value(args, "--code")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    std::process::exit(code);
}

#[cfg(unix)]
fn install_trap() {
    extern "C" fn handler(sig: i32) {
        // Async-signal-safe: store the number, touch nothing else.
        SIGNUM.store(sig, Ordering::SeqCst);
    }
    let handler: extern "C" fn(i32) = handler;
    let mut action: libc::sigaction = unsafe { std::mem::zeroed() };
    action.sa_sigaction = handler as usize;
    action.sa_flags = libc::SA_RESTART;
    unsafe {
        libc::sigaction(libc::SIGINT, &action, std::ptr::null_mut());
        libc::sigaction(libc::SIGTERM, &action, std::ptr::null_mut());
    }
}

#[cfg(not(unix))]
fn install_trap() {}

fn serve(args: &[String]) {
    let port: u16 = arg_value(args, "--port")
        .and_then(|v| v.parse().ok())
        .or_else(|| std::env::var("PORT").ok().and_then(|v| v.parse().ok()))
        .unwrap_or(8080);
    let ready_path = arg_value(args, "--ready-path").unwrap_or_else(|| "/healthz".to_string());
    let exit_after: u32 = arg_value(args, "--exit-after")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let never_ready = args.iter().any(|a| a == "--never-ready");
    let trap_marker = arg_value(args, "--trap-marker");

    install_trap();
    let listener = TcpListener::bind(("127.0.0.1", port)).expect("fixture bind");
    listener
        .set_nonblocking(true)
        .expect("fixture nonblocking listener");
    let successes = Arc::new(AtomicU32::new(0));

    loop {
        if SIGNUM.load(Ordering::SeqCst) != 0 {
            if let Some(marker) = &trap_marker {
                let _ = std::fs::write(marker, SIGNUM.load(Ordering::SeqCst).to_string());
            }
            std::process::exit(0);
        }
        match listener.accept() {
            Ok((stream, _)) => {
                if never_ready {
                    respond(&stream, 404);
                    continue;
                }
                if request_path(&stream) == ready_path {
                    respond(&stream, 200);
                    let count = successes.fetch_add(1, Ordering::SeqCst) + 1;
                    if exit_after > 0 && count >= exit_after {
                        std::process::exit(0);
                    }
                } else {
                    respond(&stream, 404);
                }
            }
            Err(_) => std::thread::sleep(Duration::from_millis(20)),
        }
    }
}

fn request_path(stream: &TcpStream) -> String {
    let mut buf = [0u8; 2048];
    let mut stream = stream;
    let n = stream.read(&mut buf).unwrap_or(0);
    let head = String::from_utf8_lossy(&buf[..n]);
    head.split_whitespace().nth(1).unwrap_or("").to_string()
}

fn respond(mut stream: &TcpStream, status: u16) {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        _ => "Not Found",
    };
    let _ = stream
        .write_all(format!("HTTP/1.0 {status} {reason}\r\nContent-Length: 0\r\n\r\n").as_bytes());
    let _ = stream.flush();
}
