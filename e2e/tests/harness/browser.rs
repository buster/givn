//! The Rust side of the `@e2e` browser tier.
//!
//! One [`BrowserDriver`] owns the JSON-line driver process
//! (`tests/e2e/browser-driver.mjs`) for the scenario's lifetime: it spawns it,
//! sends one command at a time, and reads one reply. A non-zero driver exit, a
//! timeout, or an `{"ok": false, "error": …}` reply fails the step — it is
//! never turned into a skipped one — and the process is killed on drop, so a
//! stray browser cannot survive a failing scenario. The scenario's `TempDir`
//! removal is what deletes the report the browser read.

use std::fmt;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

/// How long one driver command may take before the step fails. Playwright's own
/// action timeouts are shorter; this one covers a driver that never answers at
/// all, which is the failure that must never become a skip.
const COMMAND_TIMEOUT: Duration = Duration::from_secs(60);

/// The viewports the change's Visual Design Contract declares (`design.md`
/// §Visual Design Contract): `mobile (360x780)` and `desktop (1920x1080)`.
/// Every `@e2e` scenario commits its final state at both.
pub const VIEWPORTS: [(&str, u32, u32); 2] = [("mobile", 360, 780), ("desktop", 1920, 1080)];

/// The change whose visual evidence the browser tier captures.
const CHANGE_DIR: &str = "givn/changes/development-timeline";

/// The `@e2e` tier's browser driver: one `node` process, one browser, one
/// context, one page per scenario.
#[allow(dead_code)] // Task E2–E9 drive it; Task E1 creates and proves it.
pub struct BrowserDriver {
    /// The `node` process running `tests/e2e/browser-driver.mjs`.
    child: Child,
    /// The driver's stdin: one JSON command per line.
    stdin: ChildStdin,
    /// The driver's replies, one JSON line each, in order.
    replies: Receiver<String>,
    /// The next command's id, so a reply can be matched to its request.
    next_id: u64,
}

#[allow(dead_code)] // Task E2–E9 drive it; Task E1 creates and proves it.
impl BrowserDriver {
    /// Start the driver for one scenario: the repository's driver script, run
    /// with `node` resolved from `PATH` (the design does not pin Node).
    pub fn start() -> BrowserDriver {
        let package_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let script = package_dir.join("browser").join("browser-driver.mjs");
        let mut child = Command::new("node")
            .arg(&script)
            .current_dir(package_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap_or_else(|error| {
                panic!(
                    "start the browser driver {} (is `node` on PATH?): {error}",
                    script.display()
                )
            });
        let stdin = child.stdin.take().expect("the driver's stdin is piped");
        let stdout = child.stdout.take().expect("the driver's stdout is piped");
        let (sender, replies) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                match line {
                    Ok(line) => {
                        if sender.send(line).is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
        BrowserDriver {
            child,
            stdin,
            replies,
            next_id: 1,
        }
    }

    /// One command, one reply: returns the reply's `value`. A reply that is not
    /// `ok`, an answer that never comes, or a driver that died before answering
    /// panics — a failing step, never a skipped one.
    fn command(&mut self, command: &str, fields: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        let mut request = json!({ "id": id, "command": command });
        if let Value::Object(fields) = fields {
            for (key, value) in fields {
                request[key] = value;
            }
        }
        writeln!(self.stdin, "{request}")
            .unwrap_or_else(|error| panic!("send `{command}` to the browser driver: {error}"));
        self.stdin
            .flush()
            .unwrap_or_else(|error| panic!("send `{command}` to the browser driver: {error}"));
        let line = match self.replies.recv_timeout(COMMAND_TIMEOUT) {
            Ok(line) => line,
            Err(RecvTimeoutError::Timeout) => panic!(
                "the browser driver did not answer `{command}` within {}s",
                COMMAND_TIMEOUT.as_secs()
            ),
            Err(RecvTimeoutError::Disconnected) => panic!(
                "the browser driver exited before answering `{command}` ({}): {}",
                self.child
                    .try_wait()
                    .map(|status| status.map_or_else(|| "running".to_string(), |s| s.to_string()))
                    .unwrap_or_else(|error| error.to_string()),
                self.stderr_hint(),
            ),
        };
        let reply: Value = serde_json::from_str(&line).unwrap_or_else(|error| {
            panic!("the browser driver's reply is not JSON ({error}): {line}")
        });
        assert_eq!(
            reply["id"].as_u64(),
            Some(id),
            "the browser driver answered another command: {line}"
        );
        if reply["ok"] != json!(true) {
            panic!(
                "the browser driver failed `{command}`: {}",
                reply["error"].as_str().unwrap_or("no error message")
            );
        }
        reply["value"].clone()
    }

    /// The driver's own last words, for a failure message: the child's stderr is
    /// inherited, so the tail is already on the test output.
    fn stderr_hint(&self) -> &'static str {
        "(the driver's stderr is part of this output)"
    }

    /// Navigate to `url` — always a `file://` URL of the generated report.
    pub fn goto(&mut self, url: &str) {
        self.command("goto", json!({ "url": url }));
    }

    /// Resize the one context's page to a contract viewport. Never a second
    /// context: the scenario's state (zoom, open panel) survives.
    pub fn viewport(&mut self, width: u32, height: u32) {
        self.command("viewport", json!({ "width": width, "height": height }));
    }

    /// Click the element `selector` names — a real browser click.
    pub fn click(&mut self, selector: &str) {
        self.command("click", json!({ "selector": selector }));
    }

    /// Press `key`, optionally focusing `selector` first — the keyboard path
    /// (`Tab` reachability, `Enter`/`Space` activation, `Escape` closing).
    pub fn press(&mut self, selector: Option<&str>, key: &str) {
        self.command("press", json!({ "selector": selector, "key": key }));
    }

    /// The rendered text of the element `selector` names.
    pub fn text(&mut self, selector: &str) -> String {
        self.command("text", json!({ "selector": selector }))
            .as_str()
            .unwrap_or_else(|| panic!("the browser returned no text for {selector}"))
            .to_string()
    }

    /// How many elements `selector` matches — the DOM-level absence check.
    pub fn count(&mut self, selector: &str) -> u64 {
        self.command("count", json!({ "selector": selector }))
            .as_u64()
            .unwrap_or_else(|| panic!("the browser returned no count for {selector}"))
    }

    /// One attribute of the element `selector` names.
    pub fn attribute(&mut self, selector: &str, name: &str) -> Option<String> {
        self.command("attribute", json!({ "selector": selector, "name": name }))
            .as_str()
            .map(str::to_string)
    }

    /// Run `script` in the page and return its value — reading the state the
    /// page's own script produced (the visible range), never a fetch.
    pub fn evaluate(&mut self, script: &str) -> Value {
        self.command("evaluate", json!({ "script": script }))
    }

    /// Capture `path`: the driver settles the page first and disables
    /// animations, so the capture never depends on timing.
    pub fn screenshot(&mut self, path: &Path) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|error| panic!("create {}: {error}", parent.display()));
        }
        self.command("screenshot", json!({ "path": path.display().to_string() }));
        let bytes = std::fs::metadata(path)
            .unwrap_or_else(|error| panic!("read the capture {}: {error}", path.display()))
            .len();
        assert!(
            bytes > 0,
            "the browser wrote an empty capture: {}",
            path.display()
        );
    }

    /// Bring `selector` into view: the report's own scrolling, so a state that
    /// sits below the fold is the state a capture then shows.
    pub fn scroll_into_view(&mut self, selector: &str) {
        self.command(
            "evaluate",
            json!({
                "script": format!(
                    "document.querySelector({selector:?}).scrollIntoView({{ block: 'start' }})"
                )
            }),
        );
    }

    /// Whether evidence capture is enabled. The release workflow sets
    /// `GIVN_EVIDENCE_CAPTURE=0` while it verifies a release: captures carry
    /// measured durations and would otherwise dirty the tested tree with
    /// changes the release commit must not carry.
    fn capture_enabled() -> bool {
        std::env::var("GIVN_EVIDENCE_CAPTURE").ok().as_deref() != Some("0")
    }

    /// Capture one scenario's evidence pair at both contract viewports, under
    /// `evidence/visual/<scenario-slug>/` — the layout the change's visual gate
    /// reads.
    pub fn capture_evidence(&mut self, scenario: &str) {
        self.capture_evidence_from(scenario, "#report-title");
    }

    /// Capture one scenario's evidence pair with the page scrolled to `anchor`
    /// at each viewport first: the state the capture must show is the state the
    /// scenario asserted, even where it sits below the fold at the mobile
    /// viewport.
    pub fn capture_evidence_from(&mut self, scenario: &str, anchor: &str) {
        if !Self::capture_enabled() {
            return;
        }
        for (viewport, width, height) in VIEWPORTS {
            self.viewport(width, height);
            self.scroll_into_view(anchor);
            self.screenshot(&evidence_file(scenario, viewport));
        }
    }

    /// Capture the declared state's pair (`<viewport>--<state>.png`) beside the
    /// final-state pair, from the same open report.
    pub fn capture_state_evidence(&mut self, scenario: &str, state: &str) {
        if !Self::capture_enabled() {
            return;
        }
        for (viewport, width, height) in VIEWPORTS {
            self.viewport(width, height);
            self.screenshot(&state_evidence_file(scenario, viewport, state));
        }
    }

    /// Ask the driver to close its browser and exit.
    pub fn quit(&mut self) {
        let _ = writeln!(
            self.stdin,
            "{}",
            json!({ "id": self.next_id, "command": "quit" })
        );
        let _ = self.stdin.flush();
        let _ = self.child.wait();
    }
}

impl Drop for BrowserDriver {
    /// The scenario owns its driver: a failing step must never leave a browser
    /// behind, so the child is killed when the state that holds it is dropped.
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl fmt::Debug for BrowserDriver {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BrowserDriver")
            .field("driver", &"tests/e2e/browser-driver.mjs")
            .field("pid", &self.child.id())
            .finish()
    }
}

/// The `file://` URL of one report inside the scenario's temporary project
/// root: the report is one self-contained file, so the browser needs no server.
#[allow(dead_code)] // Task E6–E9 navigate through it.
pub fn file_url(path: &Path) -> String {
    let absolute = path
        .canonicalize()
        .unwrap_or_else(|error| panic!("resolve {} for a file:// URL: {error}", path.display()));
    format!("file://{}", absolute.display())
}

/// The evidence file of one `@e2e` scenario's capture:
/// `<change>/evidence/visual/<scenario-slug>/<viewport>.png`, exactly the path
/// `src/integrity_hook.rs::check_visual_evidence` reads. The slug is the hook's
/// own rule (`src/integrity_hook.rs::scenario_slug`), never a second convention.
#[allow(dead_code)] // Task E2–E9 commit their captures here.
pub fn evidence_file(scenario: &str, viewport: &str) -> PathBuf {
    evidence_root(scenario).join(format!("{viewport}.png"))
}

/// The declared state's evidence file of one `@e2e` scenario:
/// `<viewport>--<state>.png` beside the final-state pair — the naming the
/// visual gate enforces for the contract's `empty` state.
#[allow(dead_code)] // Task E9 commits the `empty` pair here.
pub fn state_evidence_file(scenario: &str, viewport: &str, state: &str) -> PathBuf {
    evidence_root(scenario).join(format!("{viewport}--{state}.png"))
}

/// The change's directory: the active change while it exists, otherwise the
/// archived copy. The archive gate runs the e2e scopes after it has moved the
/// change, so captures must follow the change instead of recreating the old
/// `givn/changes/<id>` path (which would dirty the tested tree outside the
/// declared evidence and block the archive).
fn change_dir(root: &Path) -> PathBuf {
    let active = root.join(CHANGE_DIR);
    if active.is_dir() {
        return active;
    }
    let id = active
        .file_name()
        .expect("CHANGE_DIR always names a change directory");
    root.join("givn").join("archive").join(id)
}

/// The scenario's evidence directory under the change's visual evidence root.
#[allow(dead_code)] // Reached through the two evidence-file helpers above.
fn evidence_root(scenario: &str) -> PathBuf {
    change_dir(&super::binary::product_root())
        .join("evidence")
        .join("visual")
        .join(super::timing::scenario_slug(scenario))
}
