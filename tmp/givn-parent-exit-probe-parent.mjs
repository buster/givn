import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawn } from "node:child_process";
import net from "node:net";

const node = process.env.GIVN_OPENCODE_NODE;
const supervisor = process.env.GIVN_OPENCODE_SUPERVISOR;
if (!node || !supervisor) throw new Error("prepared Node and supervisor paths are required");
const root = await mkdtemp(join(tmpdir(), "givn-opencode-parent-exit-"));
const driver = join(process.cwd(), "tmp/givn-parent-exit-probe-driver.mjs");
const sentinel = net.createServer((socket) => socket.end("sentinel"));
await new Promise((resolve) => sentinel.listen(0, "127.0.0.1", resolve));
const sentinelPort = sentinel.address().port;
const child = spawn(supervisor, ["--timeout-seconds", "20", "--root", root, "--", node, driver], {
  stdio: ["pipe", "pipe", "pipe"],
});
let stdout = "";
let stderr = "";
child.stdout.setEncoding("utf8");
child.stderr.setEncoding("utf8");
child.stdout.on("data", (chunk) => { stdout += chunk; });
child.stderr.on("data", (chunk) => { stderr += chunk; });
const ready = await new Promise((resolve, reject) => {
  const timer = setTimeout(() => reject(new Error(`readiness timed out; stdout=${stdout}; stderr=${stderr}`)), 10000);
  const poll = setInterval(() => {
    const line = stdout.split("\n").find((candidate) => candidate.includes('"type":"ready"'));
    if (line) {
      clearTimeout(timer);
      clearInterval(poll);
      resolve(JSON.parse(line));
    }
    if (child.exitCode !== null) reject(new Error(`supervisor exited early ${child.exitCode}; stdout=${stdout}; stderr=${stderr}`));
  }, 20);
});
child.stdin.end();
const [code, signal] = await new Promise((resolve) => child.once("exit", (code, signal) => resolve([code, signal])));
const probe = (port) => new Promise((resolve) => {
  const socket = net.createConnection({ host: "127.0.0.1", port });
  socket.setTimeout(500);
  socket.once("connect", () => { socket.destroy(); resolve("reachable"); });
  socket.once("error", () => resolve("closed"));
  socket.once("timeout", () => { socket.destroy(); resolve("timeout"); });
});
const owned = await Promise.all([ready.providerPort, ready.proxyPort, ready.descendantPort].map(probe));
const sentinelResponse = await new Promise((resolve, reject) => {
  const socket = net.createConnection({ host: "127.0.0.1", port: sentinelPort });
  let response = "";
  socket.setEncoding("utf8");
  socket.on("data", (chunk) => { response += chunk; });
  socket.once("end", () => resolve(response));
  socket.once("error", reject);
});
sentinel.close();
console.log(JSON.stringify({ ready, supervisorExit: { code, signal }, ownedPortResults: owned, sentinelResponse, stdout, stderr, ownedRootRemoved: !(await import("node:fs")).existsSync(root) }));
if (code === 0 || owned.some((result) => result !== "closed") || sentinelResponse !== "sentinel" || (await import("node:fs")).existsSync(root)) process.exitCode = 1;
