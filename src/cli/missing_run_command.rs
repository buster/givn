/// Sentinel default `run.command`.
///
/// givn never executes the start command as part of validation, but the
/// sentinel string lands in `givn/commands.yaml` and in the README run
/// block — someone will copy-execute it. This prints setup guidance and
/// exits non-zero so a copied sentinel gives direction instead of a bare
/// `command not found`.
pub fn run() -> anyhow::Result<()> {
    eprintln!(
        r#"givn: no start command configured (run.command is unset).

The `run:` block in givn/commands.yaml declares how the built product is
started: the start command, the product type (server, cli, or library), and
— for servers — the port hand-off and readiness path. givn validates the
declaration, documents it in the README, and blocks `givn archive` while it
is unconfigured. `givn run` and automation around givn (CI, a preview host)
read it and start the product.

Set the run block in givn/commands.yaml, e.g.:

  # A server: exactly one of `port` or `port_env`, plus `ready_path`.
  run:
    command: "./run.sh"
    product_type: server
    port_env: "PORT"
    ready_path: "/healthz"
    ready_timeout_s: 60

  # A one-shot CLI tool:
  # run:
  #   command: "cargo run --release -- --help"
  #   product_type: cli

After editing givn/commands.yaml, re-run the command that invoked this
sentinel. See givn/commands.yaml (key: run) and the README run block."#
    );
    std::process::exit(1);
}
