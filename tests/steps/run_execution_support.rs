//! Shared harness for the run-execution capability: port reservation and
//! fixture-declaration writers. Step definitions live in
//! `run_execution_steps.rs` / `run_execution_e2e_steps.rs`.

use std::net::TcpListener;

use super::GivnWorld;

/// Reserve one free port (bind 127.0.0.1:0, read, drop). The bind-then-use
/// race is the accepted, documented mitigation (design §13).
pub(crate) fn reserve_free_port() -> u16 {
    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("reserve free port");
    listener.local_addr().expect("local addr").port()
}

/// Replace the fixture project's run declaration with a fixed-port server
/// declaration whose port was reserved immediately before the write.
pub(crate) fn write_fixed_port_server_declaration(world: &mut GivnWorld, command: &str) {
    let port = reserve_free_port();
    let declaration = format!(
        "run:\n  command: \"{command}\"\n  product_type: server\n  port: {port}\n  ready_path: \"/healthz\"\n  ready_timeout_s: 60\n"
    );
    let env = world.env.get_or_insert_with(crate::common::make_init_root);
    let path = env.path("givn/commands.yaml");
    let content = std::fs::read_to_string(&path).expect("read commands.yaml");
    let updated = crate::common::replace_top_level_key(&content, "run", &declaration);
    std::fs::write(&path, updated).expect("write commands.yaml");
    world.run_declared_command = Some(command.to_string());
    world.run_fixed_port = Some(port);
}

/// Replace the fixture project's run declaration with a library declaration
/// carrying a real (non-sentinel) start command.
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
