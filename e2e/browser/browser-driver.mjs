// The @e2e browser tier's driver: one Chromium and one context per scenario,
// driven by JSON lines on stdin. The Rust step helper (`tests/common/browser.rs`)
// spawns this process once per scenario, owns it for the scenario's lifetime,
// and no step opens a second context.
//
// Requests: {"id": <n>, "command": "<name>", ...fields}
// Replies:  {"id": <n>, "ok": true,  "value": <any>}
//           {"id": <n>, "ok": false, "error": "<command>: <message>"}
//
// Every navigation is a `file://` URL: the report under test is one
// self-contained file with its script inline, so there is no server, no port,
// no module resolution, no CORS, and no `fetch`. The context emulates
// `prefers-reduced-motion: reduce`, and every command that reads or captures
// the page first awaits `document.fonts.ready` and one idle tick, so a capture
// never depends on timing. The process exits on stdin EOF and on `quit`.
import { createInterface } from "node:readline";
import { chromium } from "playwright";

/** The browser, its context, and the page — started once, lazily. */
let browser = null;
let context = null;
let page = null;

/** The one browser and the one context of this scenario. */
async function ensurePage() {
  if (page) return page;
  browser = await chromium.launch();
  context = await browser.newContext({
    reducedMotion: "reduce",
    viewport: { width: 1280, height: 900 },
  });
  page = await context.newPage();
  return page;
}

/** Wait until fonts are ready and the page has been idle: the deterministic
    capture contract — reduced motion is emulated, the report renders from
    embedded timestamps, and nothing is timed. */
async function settled() {
  const target = await ensurePage();
  await target.evaluate(async () => {
    if (document.fonts && document.fonts.ready) await document.fonts.ready;
  });
  await target.evaluate(async () => {
    await new Promise((resolve) => {
      if (typeof requestIdleCallback === "function") {
        requestIdleCallback(() => resolve(), { timeout: 500 });
      } else {
        setTimeout(resolve, 0);
      }
    });
  });
}

/** The commands the Rust helper may send. A command that throws becomes an
    `ok: false` reply, which the helper turns into a failed step. */
const commands = {
  async goto(message) {
    const target = await ensurePage();
    await target.goto(message.url, { waitUntil: "load" });
    await settled();
  },
  async viewport(message) {
    const target = await ensurePage();
    await target.setViewportSize({ width: message.width, height: message.height });
    await settled();
  },
  async click(message) {
    const target = await ensurePage();
    await target.click(message.selector, { timeout: message.timeout ?? 10_000 });
    await settled();
  },
  async press(message) {
    const target = await ensurePage();
    if (message.selector) await target.focus(message.selector);
    await target.keyboard.press(message.key);
    await settled();
  },
  async text(message) {
    const target = await ensurePage();
    return await target.innerText(message.selector, {
      timeout: message.timeout ?? 10_000,
    });
  },
  async count(message) {
    const target = await ensurePage();
    return await target.locator(message.selector).count();
  },
  async attribute(message) {
    const target = await ensurePage();
    return await target.getAttribute(message.selector, message.name);
  },
  async evaluate(message) {
    const target = await ensurePage();
    return await target.evaluate(message.script);
  },
  async screenshot(message) {
    await settled();
    const target = await ensurePage();
    await target.screenshot({
      path: message.path,
      animations: "disabled",
      fullPage: message.fullPage ?? false,
    });
  },
  async quit() {
    await shutdown();
    return null;
  },
};

/** Close the browser: the driver's own half of "a stray browser cannot survive
    a failure" (the Rust helper kills the process on drop as well). */
async function shutdown() {
  if (!browser) return;
  const closing = browser;
  browser = null;
  context = null;
  page = null;
  await closing.close().catch(() => {});
}

/** One request line: run its command, reply once. */
async function handle(line) {
  const trimmed = line.trim();
  if (trimmed === "") return;
  let message;
  try {
    message = JSON.parse(trimmed);
  } catch (error) {
    process.stdout.write(
      `${JSON.stringify({ id: null, ok: false, error: `unreadable command: ${error.message}` })}\n`,
    );
    return;
  }
  const command = commands[message.command];
  if (typeof command !== "function") {
    process.stdout.write(
      `${JSON.stringify({ id: message.id, ok: false, error: `unknown command: ${message.command}` })}\n`,
    );
    return;
  }
  try {
    const value = await command(message);
    process.stdout.write(
      `${JSON.stringify({ id: message.id, ok: true, value: value === undefined ? null : value })}\n`,
    );
    if (message.command === "quit") process.exit(0);
  } catch (error) {
    process.stdout.write(
      `${JSON.stringify({ id: message.id, ok: false, error: `${message.command}: ${error?.message ?? error}` })}\n`,
    );
  }
}

// One command at a time, in the order the helper sent them: the page's state
// (zoom level, open panel) survives across the steps of one scenario.
let chain = Promise.resolve();
const lines = createInterface({ input: process.stdin });
lines.on("line", (line) => {
  chain = chain.then(() => handle(line));
});
lines.on("close", () => {
  chain.then(shutdown).then(() => process.exit(0));
});
