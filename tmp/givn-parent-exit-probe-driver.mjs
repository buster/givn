import { spawn } from "node:child_process";
import { createServer } from "node:net";
import { createInterface } from "node:readline";

const listen = (server) => new Promise((resolve, reject) => {
  server.once("error", reject);
  server.listen(0, "127.0.0.1", () => {
    server.off("error", reject);
    resolve(server.address().port);
  });
});
const provider = createServer((socket) => socket.end("provider"));
const proxy = createServer((socket) => socket.end("proxy"));
const providerPort = await listen(provider);
const proxyPort = await listen(proxy);
const childSource = `import {createServer} from 'node:net'; const s=createServer(x=>x.end('descendant')); s.listen(0,'127.0.0.1',()=>console.log(s.address().port)); process.on('SIGTERM',()=>s.close(()=>process.exit(0))); setInterval(()=>{},1000);`;
const descendant = spawn(process.execPath, ["--input-type=module", "-e", childSource], { stdio: ["ignore", "pipe", "inherit"] });
const lines = createInterface({ input: descendant.stdout });
const descendantPort = await new Promise((resolve, reject) => {
  lines.once("line", (line) => resolve(Number(line)));
  descendant.once("error", reject);
  descendant.once("exit", (code) => reject(new Error(`descendant exited early: ${code}`)));
});
console.log(JSON.stringify({ type: "ready", providerPort, proxyPort, descendantPort, driverPid: process.pid, descendantPid: descendant.pid }));

process.on("SIGTERM", async () => {
  descendant.kill("SIGTERM");
  await Promise.all([provider, proxy].map((server) => new Promise((resolve) => server.close(resolve))));
  await new Promise((resolve) => descendant.once("exit", resolve));
  process.exit(0);
});
setInterval(() => {}, 1000);
