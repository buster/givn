# Design Review: llm-attribution

## Scope

PASS. The proposal asks for best-effort LLM attribution visible in the Git
committer identity and commit message, shared Git plumbing, and OpenCode and
Claude Code integrations. The feature scenarios cover initialization,
complete context, partial context, no context, model/harness changes, and
existing-hook compatibility. Complete provenance, server enforcement, and
model selection remain explicitly out of scope.

## Grilling Decisions

### Existing Git hooks

Decision: preserve and chain an existing `commit-msg` hook.

The local activation launcher runs the existing hook first and preserves its
non-zero result. It moves an unknown existing hook to one `.givn-original`
backup only when that backup does not already exist and never overwrites a
backup. Givn stamping runs only after the existing hook succeeds and any Givn
stamping error is non-blocking. Repeated initialization is idempotent.

### Checked-in versus active hook

The real hook source is checked in at `.githooks/commit-msg`. The local launcher
under Git's resolved hooks directory is machine-local and is installed by
`givn init` or `givn skills sync`. Git does not automatically activate a
checked-in hook after clone; that local activation boundary is documented and
is not a security guarantee.

### Committer identity

The generic hook does not mutate the current Git process's identity. Harness
integrations provide `GIT_COMMITTER_NAME` before invoking Git when model data is
available. The attributed-commit scenario drives the real Git command with the
runtime identity and verifies the resulting committer. If an adapter cannot
provide the identity, the commit remains valid and the message records partial
or unavailable attribution.

### OpenCode metadata

The generated OpenCode plugin uses `shell.env` for shell variables and the
`chat.message` event for provider, model, and variant values. The variant is
the reasoning-mode value when present. Missing session metadata produces an
explicit unavailable field and never blocks the shell or commit.

### Claude Code metadata

The generated Claude Code PreToolUse Bash hook scopes environment variables to
commands containing a Git commit. It uses model and effort values from the
hook payload or environment when available. The best-effort fallback identifies
`claude-code` as both the available LLM identity and harness, with reasoning
marked unavailable. No unsupported assumption about a default model variable
is required.

### Human interaction

The commit message is stamped by `commit-msg`, after the editor has finished.
When no attribution context is present, the hook leaves the message byte-for-
byte unchanged and exits successfully. Attribution is never a commit gate.

### Partial and failure behavior

The behavior is explicit:

| Context | Result |
|---|---|
| No active context | Commit succeeds; message and human identity remain unchanged. |
| Complete context | Current values and `complete` trailers are appended; supplied committer name is visible. |
| Partial context | Missing fields are `unavailable`, status is `partial`, and commit succeeds. |
| Missing `givn` binary | Hook exits successfully without changing the message. |
| Malformed hook payload | Agent command remains unchanged; commit behavior is unaffected. |
| Existing hook failure | Existing failure is returned and Givn stamping does not run. |
| Repeated stamping | Prior managed trailers are replaced, not duplicated. |
| `--no-verify` or disabled hooks | Normal Git bypass semantics apply; local hooks are not a security boundary. |

Values are normalized as single-line trailer values. Newlines and malformed
values are treated as unavailable rather than interpreted as message content.

## Missing Scenarios Resolved

The feature delta now includes partial attribution and existing-hook chaining.
The attributed commit scenario is tagged `@e2e`, while the human, partial,
model-change, and chaining cases remain non-E2E variants with distinct
invariants. The durable `support/group.md` interaction inventory contains both
normalized E2E actions, and the design matrix maps each to a real CLI/Git
subprocess driver.

## Testability

Every scenario can fail before production implementation:

- initialization assertions fail when the hook or integrations are absent;
- commit scenarios fail when the hook does not stamp, preserves duplicates,
  changes a human message, or blocks on missing data;
- identity assertions inspect the actual Git commit object;
- chaining assertions inspect the existing hook's marker and exit behavior.

The Cucumber runner already uses `.fail_on_skipped()`. New RED step bodies use
`unimplemented!()` and are registered in one integration step file and one E2E
step file for this capability.

## E2E Fidelity

The two inventory entries have matching `@e2e` scenarios:

| Inventory entry | Scenario | Interface | Driver |
|---|---|---|---|
| Initialize a Givn project with OpenCode and Claude Code targets | Consumer initializes a project and receives shared attribution plumbing | CLI and project filesystem | Compiled `givn init --agents opencode,claude` subprocess plus filesystem and Git-hook inspection |
| Create a commit while attribution context is available | Consumer creates an attributed commit and inspects it with Git | Git CLI | Real `git add`/`git commit`/`git show` subprocesses with controlled runtime environment |

No external service or digital twin is required. Temporary repositories provide
isolated Git state.

## Architecture Documentation

All twelve Arc42 chapters were assessed as affected and updated. The change
marker contains the complete twelve-row assessment and `STATUS: DONE`. The
glossary defines Attribution context, Commit attribution, Harness, Reasoning
mode, Attribution trailer, Attribution status, Committer identity,
Commit-attribution hook, and Hook activation launcher.

The architectural choice is not ADR-qualified. The canonical owners are the
feature specification for observable behavior and `design.md` for the complete
technical rationale. No new ADR was created.

## Risk Mitigation

The highest-risk failure is damaging an existing Git hook. The local dispatcher
preserves the original hook and its failure result, never overwrites an
existing backup, avoids changing `core.hooksPath`, and treats unresolved custom
paths as a non-blocking activation warning. Harness API drift is mitigated by
best-effort extraction, explicit unavailable values, and non-blocking hooks.

`givn lint --change llm-attribution` reports the expected WIP state and no
feature parse failure; implementation will remove `@wip` only after the
scenario-by-scenario RED/GREEN/REFACTOR loop.

DESIGN-REVIEW: PASS
