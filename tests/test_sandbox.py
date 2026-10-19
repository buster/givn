from __future__ import annotations

import tempfile
import unittest
from pathlib import Path

from givn_harness.sandbox import (
    SandboxPolicy,
    SandboxUnavailable,
    build_command,
    bubblewrap_available,
    probe,
    require_isolated,
)


class SandboxCommandTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.workspace = Path(self.temp.name) / "workspace"
        self.workspace.mkdir()

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_command_unshares_network_and_private_home_by_default(self) -> None:
        command = build_command(
            SandboxPolicy(mode="required", network="none", workspace=self.workspace),
            ["true"],
        )
        self.assertIn("--unshare-all", command)
        self.assertNotIn("--share-net", command)
        self.assertIn("--tmpfs", command)
        self.assertIn("--die-with-parent", command)
        self.assertIn("--clearenv", command)
        self.assertEqual(command[-1], "true")

    def test_host_network_is_explicit_and_recorded(self) -> None:
        policy = SandboxPolicy(mode="required", network="host", workspace=self.workspace)
        command = build_command(policy, ["true"])
        self.assertIn("--share-net", command)
        self.assertEqual(policy.describe()["network_allowlist"], "unavailable")

    def test_unconfined_mode_is_explicit(self) -> None:
        command = build_command(
            SandboxPolicy(mode="none", network="none", workspace=self.workspace), ["true"]
        )
        self.assertEqual(command, ["true"])

    def test_credential_paths_are_opt_in(self) -> None:
        credential = Path(self.temp.name) / "auth.json"
        credential.write_text("{}")
        command = build_command(
            SandboxPolicy(
                mode="required",
                network="none",
                workspace=self.workspace,
                credential_binds=[credential],
            ),
            ["true"],
        )
        self.assertIn(str(credential), command)

    def test_require_isolated_refuses_none(self) -> None:
        with self.assertRaises(SandboxUnavailable):
            require_isolated(SandboxPolicy(mode="none", workspace=self.workspace))


@unittest.skipUnless(bubblewrap_available(), "bubblewrap is unavailable")
class SandboxProbeTests(unittest.TestCase):
    def test_probe_proves_workspace_host_and_network_boundaries(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            workspace = Path(temporary) / "workspace"
            workspace.mkdir()
            result = probe(SandboxPolicy(mode="required", network="none", workspace=workspace))
        self.assertTrue(result["ok"], result)
        names = {check["name"] for check in result["checks"]}
        self.assertEqual(names, {"workspace-writable", "host-state-hidden", "network-denied"})


if __name__ == "__main__":
    unittest.main()
