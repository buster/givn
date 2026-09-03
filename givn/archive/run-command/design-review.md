# Design Review: run-command

## 1. Method

Two review passes with fresh contexts:

- **Grilling subagent** — stress-tested proposal, specs, design, arc42
  impact, and ADR-067 one question at a time; produced decisions D1–D3 and
  findings F1–F13.
- **Hardening subagent** — independent context; applied the decisions and
  findings to the plan artifacts (specs, design, ADR, arc42 chapters) and
  produced this document. No implementation code, no tasks.md, no git
  commands.

## 2. Questions and decisions

**Q1 — How does the permanent suite keep passing once the archive/review
gates sharpen, and how does givn's own repository avoid being blocked by its
own gate mid-change?**

Options considered: (a) migrate the permanent specs via one shared fixture
step before the gate tasks; (b) scope the gates to this change's specs only;
(c) leave permanent fixtures as they are and accept breakage.

User's answer (D1): one shared fixture Given writes a complete run
declaration into fixture projects; the design specifies it concretely
(`run_declaration_fixtures_support.rs`, §4.2) and adds an explicit §15 task
migrating the ~125 archive/review invocations across 14 permanent spec files
before the gate tasks sharpen. The dogfood fill of givn's own
`givn/commands.yaml` (`command: "cargo run -- instructions"`,
`product_type: cli`) moves before the gate-sharpening tasks, so interleaved
repo work is never blocked (F13).

**Q2 — What does a library's README run block render when a real start
command is declared?**

Options considered: (a) keep §6.5's line-only render and leave §14 R10's
"rendered into the README" contradictory; (b) render the command only, drop
the library line; (c) render the library line plus the command under one
design-wide render predicate.

User's answer (D2): the library command is rendered. The design-wide
predicate: the README shows the todo placeholder exactly when `check_run`
reports findings, and the real render otherwise. Consequences written in:
library + sentinel → library line, no placeholder (protects "a library
without a start command blocks nothing" — a placeholder would block archive
via the README completeness gate); library + real command → library line +
the command; half-configured server → placeholder, never a partial render.
§6.5/§14 contradiction resolved accordingly; specs adjusted.

**Q3 — How does the in-process SIGTERM-143 scenario observe the exit code?**

Options considered: (a) run the real binary as a child of the cucumber
runner; (b) split signal handling out of the executor for testability;
(c) the executor's library entry point returns the exit code and only the
binary main wrapper converts it with `process::exit`.

User's answer (D3): the library seam returns the exit code. Stated
explicitly in design.md (§7 seam paragraph and §7.4): the in-process
SIGTERM-143 scenario calls the library entry point and asserts the returned
code; `process::exit` never runs inside the cucumber runner.

## 3. Findings register

| ID | Finding | Resolution |
|---|---|---|
| F1 | Task list must carry the new migration work and keep every task independently green. | Applied — design.md §15 rebuilt as a 16-task sequence; each task lands green in order. |
| F2 | Sentinel subcommand had no inventory entry and no scenario. | Applied — `givn missing-run-command` added to the run-command inventory with one @e2e scenario (`The sentinel start command prints setup guidance`) and a §12 matrix row. |
| F3 | Grilling closed with no issue found on this branch; nothing to apply. | No issue found — no artifact change. |
| F4 | Grilling closed with no issue found on this branch; nothing to apply. | No issue found — no artifact change. |
| F5 | `--print` edges uncovered; fresh-port behaviour undocumented. | Applied — two regular scenarios in run-execution.feature (library `--print` resolves the declared command, exit 0; unconfigured `--print` exits 1 with preflight findings and no sentinel on stdout); design.md §7.2 states each `--print` call resolves a fresh ephemeral port (eval-then-run yields a different port) — stated, deliberate. |
| F6 | `run:` in `givn/config.yaml` was silent. | Applied — design.md §3 documents that `OverlayDoc` is shared, `config.yaml` accepts `run:`, and `commands.yaml` wins by apply order, identical to `verify:`/`coverage:`; no gate, no scenario. |
| F7 | Grilling closed with no issue found on this branch; nothing to apply. | No issue found — no artifact change. |
| F8 | "Sufficient teardown" overstated for compound commands. | Applied — design.md §7.4 softened: forwarding reaches the direct `sh -c` child only; grandchildren of compound commands may leak; mitigation: start commands should use `exec`; accepted as unix-boring tradeoff. Risk R-44 added to arc42 ch.11. |
| F9 | ADR-067 below the MADR bar. | Applied — ADR-067 gains `## Qualification` (`ADR-QUALIFICATION: PASS`, structured verdict with all five mandatory gates PASS plus checklist and dimension evidence), `### Confirmation` naming the @e2e ready-line/exit-code scenarios and the R1–R10 `check_run` suite as fitness evidence, and `## Pros and Cons of the Options` comparing declaration-only, execution-only-elsewhere, and declare-and-execute. |
| F10 | Compatibility obligation unmapped to risk. | Applied — ch.11 risk R-45: ready line format, exit codes, and signal behaviour become durable public contracts once `givn run` ships; breaking them breaks kiekste and scripts; pinned by executable specs. |
| F11 | Backfill commit behaviour unobserved. | Applied — two regular scenarios in run-command.feature: the upgrade commit includes the backfilled run declaration (clean tree, `givn/commands.yaml` in the latest commit) and a repeated upgrade creates no additional commit. |
| F12 | Vocabulary drift ("run block" vs canonical "run declaration"). | Applied — user-facing message texts aligned ("run declaration incomplete", "declaration check found N finding(s)", "✓ Run gate passed", "✓ run declaration passed"); spec fixture steps and the archive-blocked assertion use "run declaration"; glossary verified to cover run declaration, start command, product type, sentinel, readiness probe, readiness path, backfill (Backfill scaffold), declaration gate (Run gate), executor; added the missing **Readiness timeout** entry. Code-internal identifiers (`run_block.rs`, `check_run`, `refresh_run_block`) unchanged. |
| F13 | Dogfood fill ordered too late; interleaved repo work would be blocked. | Applied — design.md §15 task 12 (dogfood fill) precedes the gate tasks 13–14; order constraints documented. |

## 4. Branch verdicts

| Branch | Verdict |
|---|---|
| Scope | PASS grilling unchanged. D2 slightly amends the library README contract — design/spec territory; proposal.md untouched, no scope mismatch. |
| Tech choices | PASS grilling unchanged. libc decision remains lockfile-verified; no new crates; D3 changes no technology choice. |
| Missing scenarios | PASS after hardening — F2 (sentinel @e2e), F5 (two `--print` variants), F11 (two upgrade-commit scenarios), D2 (library real-command render, half-configured placeholder) added. |
| Testability | PASS after hardening — D3 library seam makes the 143 code assertable in-process; D1 shared fixture step (§4.2) makes the permanent-suite migration mechanical. |
| E2E fidelity | PASS grilling unchanged; the one new @e2e scenario (F2) follows the canonical one-e2e-per-inventory-action rule. |
| Interaction coverage | PASS grilling unchanged; inventory remains exhaustive after F2's addition (F5/F11 scenarios are variants of existing inventory actions). |
| Risk | PASS after hardening — R-44 (compound-command teardown residual) and R-45 (machine-contract compatibility obligation) added to ch.11; ADR links updated. |
| ADR qualification | PASS after hardening — ADR-067 at full MADR compliance (F9). |
| Arc42 | PASS grilling unchanged; hardening only appended risk rows and one glossary term, and mirrored them in the change-local arc42.md assessment. |
| Ubiquitous language | PASS after hardening — F12 alignment applied; glossary covers every term the design uses. |

## 5. Hardening changes applied

- `givn/changes/run-command/specs/run-command.feature` — inventory gains the
  sentinel action; new @e2e sentinel-guidance scenario; new library
  real-command render and half-configured placeholder render scenarios;
  library sentinel render sharpened with "no start command"; two
  upgrade-commit scenarios; "run block" → "run declaration" in fixture steps
  and the archive-blocked assertion.
- `givn/changes/run-command/specs/run-execution.feature` — new `--print`
  library scenario and `--print` unconfigured-preflight scenario.
- `givn/changes/run-command/design.md` — §3 config.yaml `run:` continuity
  note; §4 table row and new §4.2 shared fixture step specification; §6.3/§6.4
  canonical message texts and gate prose; §6.5 render predicate with library
  consequences and archive-consistency note; §7 executor seam paragraph,
  §7.1 library row, §7.2 `--print` fresh-port/sentinel/preflight notes, §7.4
  teardown residual and returned-code seam; §9 step rows and registration;
  §12 sentinel matrix row and de-duplicated fixture description; §15
  16-task sequence with order constraints; §16 test-economy updates.
- `docs/arc42/adr/ADR-067-givn-declares-and-executes-the-product-start.md` —
  Qualification section (verdict, checklist evidence, dimension evidence),
  Confirmation section, Pros and Cons of the Options, links updated to
  R-41..R-45.
- `docs/arc42/11-risks-and-technical-debt.md` — R-44 and R-45 rows.
- `docs/arc42/12-glossary.md` — Readiness timeout entry.
- `givn/changes/run-command/arc42.md` — assessment rows 11 and 12 updated.

`proposal.md` untouched (no fundamental scope mismatch). `tasks.md` not
written. No git commands.

## 6. Lint result

`givn lint --change run-command` → exit **2** (WIP findings only; both delta
files parse cleanly; 48 scenarios across 2 files). Policy requires exit 0 or
2 — satisfied.

Spec scenario counts: run-command.feature 24 → 29; run-execution.feature 17
→ 19; total 41 → 48.

DESIGN-REVIEW: PASS
