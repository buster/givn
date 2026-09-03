//! @e2e step definitions for the run-execution capability.
//!
//! Real subprocess invocation of the givn binary in fixture projects; the
//! declared products are the in-tree ready fixture.

use cucumber::given;

use super::GivnWorld;
use crate::steps::run_execution_support as support;

#[given("a project with a server run declaration that becomes ready and exits cleanly")]
fn server_becomes_ready_exits_cleanly(world: &mut GivnWorld) {
    let command = support::serve_command("/healthz", &["--exit-after", "1"]);
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 60);
}

#[given("a project with a server run declaration with a port hand-off")]
fn server_declaration_with_port_handoff(world: &mut GivnWorld) {
    let command = support::serve_command("/healthz", &[]);
    support::write_server_declaration(world, &command, Some("PORT"), None, "/healthz", 60);
}
