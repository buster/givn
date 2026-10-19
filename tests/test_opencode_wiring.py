from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from givn_harness.driver import DriverSpec, _command_for
from givn_harness.runner import (
    RunConfig,
    RunError,
    _sandbox_policy,
    _validate_config,
    prepare_opencode_workspace,
)


class OpenCodeWiringTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.workspace = self.root / "workspace"
        self.workspace.mkdir()
        self.stage_dir = self.root / "stage"
        self.stage_dir.mkdir()

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_private_config_uses_only_deepseek_and_private_xdg_roots(self) -> None:
        env = prepare_opencode_workspace(
            self.workspace, self.stage_dir, "deepseek/deepseek-flash#max"
        )
        config_path = (
            Path(env["XDG_CONFIG_HOME"]) / "opencode" / "opencode.json"
        )
        config = json.loads(config_path.read_text())
        self.assertEqual(config["model"], "deepseek/deepseek-flash#max")
        self.assertIs(config["snapshots"], False)
        policies = config["policies"]
        self.assertEqual(policies[0]["resource"], "*")
        self.assertEqual(policies[0]["effect"], "deny")
        self.assertEqual(policies[1]["resource"], "deepseek")
        self.assertEqual(policies[1]["effect"], "allow")
        for key in ("XDG_CONFIG_HOME", "XDG_DATA_HOME", "XDG_CACHE_HOME", "XDG_STATE_HOME"):
            self.assertTrue(Path(env[key]).is_relative_to(self.workspace))
        self.assertTrue((self.stage_dir / "opencode.json").is_file())

    def test_sandbox_mounts_runtime_and_pins_givn_without_host_state(self) -> None:
        env = prepare_opencode_workspace(
            self.workspace, self.stage_dir, "deepseek/deepseek-flash#max"
        )
        givn_binary = self.root / "bin" / "givn"
        givn_binary.parent.mkdir()
        givn_binary.write_text("#!/bin/sh\n")
        givn_binary.chmod(0o755)
        config = RunConfig(
            benchmark=None,  # type: ignore[arg-type]
            output=self.root,
            driver=DriverSpec(
                kind="opencode",
                model="deepseek/deepseek-flash#max",
                extra_env=env,
            ),
            givn_binary=givn_binary,
            network="host",
            allow_unverified_network=True,
        )
        policy = _sandbox_policy(config, self.workspace, env_override=env)
        mounted = {str(path) for path in policy.extra_ro_binds}
        self.assertIn(str(givn_binary.resolve()), mounted)
        self.assertTrue(any(path.name == ".opencode" for path in policy.extra_ro_binds))
        self.assertFalse(any("config/opencode" in item for item in mounted))
        self.assertFalse(any("opencode.db" in item for item in mounted))
        self.assertIn(str(givn_binary.parent.resolve()), policy.env["PATH"])
        self.assertEqual(policy.network, "host")

    def test_secret_env_moves_key_out_of_argv(self) -> None:
        from givn_harness.runner import prepare_secret_env
        from givn_harness.sandbox import SandboxPolicy, build_command

        env = prepare_secret_env(
            self.workspace, {"DEEPSEEK_API_KEY": "super-secret", "PATH": "/usr/bin"}
        )
        self.assertNotIn("DEEPSEEK_API_KEY", env)
        self.assertIn("HARNESS_SECRETS_FILE", env)
        secret_path = Path(env["HARNESS_SECRETS_FILE"])
        self.assertEqual(secret_path.stat().st_mode & 0o777, 0o600)
        self.assertIn("super-secret", secret_path.read_text())
        command = build_command(
            SandboxPolicy(
                mode="required",
                network="none",
                workspace=self.workspace,
                env=env,
            ),
            ["true"],
        )
        self.assertNotIn("super-secret", " ".join(command))
        self.assertIn("HARNESS_SECRETS_FILE", command)
        self.assertEqual(command[-1], "true")
        self.assertIn("/bin/sh", command)

    def test_opencode_command_uses_absolute_executable(self) -> None:
        command = _command_for(
            DriverSpec(kind="opencode", model="deepseek/deepseek-flash#max"),
            workspace=self.workspace,
            prompt_path=self.workspace / ".harness" / "prompt.md",
            stage_id="initial",
            condition="test",
        )
        self.assertIn("--standalone", command)
        self.assertTrue(command[0].endswith("opencode"))
        self.assertTrue(command[0].startswith("/"))

    def _config(self, **overrides) -> RunConfig:
        givn_binary = self.root / "bin" / "givn"
        givn_binary.parent.mkdir(exist_ok=True)
        givn_binary.write_text("#!/bin/sh\n")
        givn_binary.chmod(0o755)
        prices = self.root / "prices.toml"
        prices.write_text(
            """
schema = 1
currency = "USD"
source = "test"
date = "2026-01-01"
[per_million_tokens]
input = 1.0
output = 2.0
""",
            encoding="utf-8",
        )
        values = dict(
            benchmark=None,
            output=self.root,
            driver=DriverSpec(
                kind="opencode",
                model="deepseek/deepseek-flash#max",
            ),
            givn_binary=givn_binary,
            network="host",
            allow_unverified_network=True,
            budget_usd=5.0,
            price_table=prices,
            secret_env={"DEEPSEEK_API_KEY": "test-key"},
        )
        values.update(overrides)
        return RunConfig(**values)  # type: ignore[arg-type]

    def test_host_network_requires_explicit_exploratory_flag(self) -> None:
        config = self._config(allow_unverified_network=False)
        with self.assertRaises(RunError):
            _validate_config(config)

    def test_opencode_requires_api_key(self) -> None:
        config = self._config(
            driver=DriverSpec(
                kind="opencode", model="deepseek/deepseek-flash#max"
            ),
            secret_env={},
        )
        with self.assertRaises(RunError):
            _validate_config(config)

    def test_exploratory_class_is_recorded(self) -> None:
        config = self._config()
        _validate_config(config)
        sandbox = config.driver_extra_sandbox()
        self.assertEqual(sandbox["execution_class"], "exploratory")
        self.assertEqual(sandbox["network_allowlist"], "unverified")

    def test_cli_refuses_opencode_without_key_before_dispatch(self) -> None:
        from click.testing import CliRunner

        from givn_harness.cli import main

        self._config()
        result = CliRunner().invoke(
            main,
            [
                "run",
                "--benchmark",
                str(Path(__file__).resolve().parents[1] / "benchmarks" / "lending"),
                "--driver",
                "opencode",
                "--output",
                str(self.root / "runs"),
                "--sandbox",
                "required",
                "--budget-usd",
                "5",
                "--price-table",
                str(self.root / "prices.toml"),
            ],
        )
        self.assertNotEqual(result.exit_code, 0)
        self.assertIn("deepseek-api-key-file", result.output)


if __name__ == "__main__":
    unittest.main()
