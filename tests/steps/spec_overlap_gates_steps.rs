use crate::steps::{GivnWorld, output};
use cucumber::then;

#[then("the command exits 3")]
fn exits_3(world: &mut GivnWorld) {
    assert_eq!(
        output(world).status.code(),
        Some(3),
        "expected exit code 3, stderr: {}",
        String::from_utf8_lossy(&output(world).stderr)
    );
}

#[then("stdout is valid JSON")]
fn stdout_valid_json(world: &mut GivnWorld) {
    let stdout = String::from_utf8_lossy(&output(world).stdout);
    serde_json::from_str::<serde_json::Value>(&stdout)
        .unwrap_or_else(|e| panic!("stdout is not valid JSON: {e}\nstdout: {stdout}"));
}
